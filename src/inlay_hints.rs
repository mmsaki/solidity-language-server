use crate::goto::{CHILD_KEYS, CachedBuild, bytes_to_pos};
use crate::hover::find_node_by_id;
use serde_json::Value;
use std::collections::HashSet;
use tower_lsp::lsp_types::*;

pub fn inlay_hints(
    build: &CachedBuild,
    uri: &Url,
    range: Range,
    source: &[u8],
) -> Option<Vec<InlayHint>> {
    let sources = build.ast.get("sources")?;
    let path_str = uri.to_file_path().ok()?.to_str()?.to_string();
    let abs = build
        .path_to_abs
        .iter()
        .find(|(k, _)| path_str.ends_with(k.as_str()))?
        .1
        .clone();
    let file_ast = find_file_ast(sources, &abs)?;
    let (r0, r1) = (
        crate::goto::pos_to_bytes(source, range.start),
        crate::goto::pos_to_bytes(source, range.end),
    );
    let mut hints = Vec::new();
    let mut seen: HashSet<(u32, u32)> = HashSet::new();
    let mut stack: Vec<&Value> = vec![file_ast];
    while let Some(node) = stack.pop() {
        if let Some(src) = node.get("src").and_then(|v| v.as_str()) {
            let (off, len) = parse_src(src);
            if off + len < r0 || off > r1 {
                continue;
            }
            if let Some((args, decl_id, names, skip)) = resolve_params(node, sources) {
                for (i, arg) in args.iter().enumerate() {
                    let pi = i + skip;
                    if pi >= names.len() || names[pi].is_empty() {
                        continue;
                    }
                    if let Some(s) = arg.get("src").and_then(|v| v.as_str())
                        && let Some(pos) = bytes_to_pos(source, parse_src(s).0)
                    {
                        hints.push(InlayHint {
                            position: pos,
                            kind: Some(InlayHintKind::PARAMETER),
                            label: InlayHintLabel::String(format!("{}:", names[pi])),
                            text_edits: None,
                            tooltip: None,
                            padding_left: None,
                            padding_right: Some(true),
                            data: Some(serde_json::json!({"decl_id": decl_id, "param_index": pi})),
                        });
                    }
                }
            }
        }
        for key in CHILD_KEYS {
            match node.get(key) {
                Some(Value::Array(a)) => stack.extend(a.iter()),
                Some(v @ Value::Object(_)) => stack.push(v),
                _ => {}
            }
        }
    }
    hints.retain(|h| seen.insert((h.position.line, h.position.character)));
    Some(hints)
}

fn resolve_params<'a>(
    node: &'a Value,
    sources: &'a Value,
) -> Option<(&'a Vec<Value>, u64, Vec<String>, usize)> {
    let nt = node.get("nodeType").and_then(|v| v.as_str())?;
    let (call, is_emit) = match nt {
        "EmitStatement" => (node.get("eventCall")?, true),
        "FunctionCall" => (node, false),
        _ => return None,
    };
    let args = call
        .get("arguments")
        .and_then(|v| v.as_array())
        .filter(|a| !a.is_empty())?;
    let decl_id = call
        .get("expression")?
        .get("referencedDeclaration")
        .and_then(|v| v.as_u64())
        .filter(|&id| id < 0x8000_0000_0000_0000)?;
    let decl = find_node_by_id(sources, decl_id)?;
    let kind = call.get("kind").and_then(|v| v.as_str()).unwrap_or("");
    if !is_emit && kind != "functionCall" && kind != "structConstructorCall" {
        return None;
    }
    if kind == "structConstructorCall" {
        if call
            .get("names")
            .and_then(|v| v.as_array())
            .is_some_and(|a| !a.is_empty())
        {
            return None;
        }
        let members = decl.get("members").and_then(|v| v.as_array())?;
        let names: Vec<String> = members
            .iter()
            .map(|m| m.get("name").and_then(|v| v.as_str()).unwrap_or("").into())
            .collect();
        return Some((args, decl_id, names, 0));
    }
    let params = decl
        .get("parameters")?
        .get("parameters")
        .and_then(|v| v.as_array())?;
    let names: Vec<String> = params
        .iter()
        .map(|p| p.get("name").and_then(|v| v.as_str()).unwrap_or("").into())
        .collect();
    let skip = if names.len() == args.len() + 1 { 1 } else { 0 };
    Some((args, decl_id, names, skip))
}

pub fn resolve_inlay_hint(build: &CachedBuild, mut hint: InlayHint) -> InlayHint {
    let data = match hint.data.take() {
        Some(d) => d,
        None => return hint,
    };
    let decl_id = match data.get("decl_id").and_then(|v| v.as_u64()) {
        Some(id) => id,
        None => return hint,
    };
    let pi = data
        .get("param_index")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;
    let decl = match build
        .ast
        .get("sources")
        .and_then(|s| find_node_by_id(s, decl_id))
    {
        Some(n) => n,
        None => return hint,
    };
    let nt = decl.get("nodeType").and_then(|v| v.as_str()).unwrap_or("");
    let param = match nt {
        "StructDefinition" => decl
            .get("members")
            .and_then(|v| v.as_array())
            .and_then(|a| a.get(pi)),
        _ => decl
            .get("parameters")
            .and_then(|p| p.get("parameters"))
            .and_then(|v| v.as_array())
            .and_then(|a| a.get(pi)),
    };
    if let Some(loc) = param
        .and_then(|p| p.get("src"))
        .and_then(|v| v.as_str())
        .and_then(|s| crate::goto::src_to_location(s, &build.id_to_path_map))
    {
        let text = match &hint.label {
            InlayHintLabel::String(s) => s.clone(),
            InlayHintLabel::LabelParts(p) => p.iter().map(|p| p.value.as_str()).collect(),
        };
        hint.label = InlayHintLabel::LabelParts(vec![InlayHintLabelPart {
            value: text,
            tooltip: None,
            location: Some(loc),
            command: None,
        }]);
    }
    hint
}

pub(crate) fn find_file_ast<'a>(sources: &'a Value, abs_path: &str) -> Option<&'a Value> {
    for (_, contents) in sources.as_object()? {
        let ast = contents
            .as_array()?
            .first()?
            .get("source_file")?
            .get("ast")?;
        if ast.get("absolutePath").and_then(|v| v.as_str()) == Some(abs_path) {
            return Some(ast);
        }
    }
    None
}

pub(crate) fn parse_src(src: &str) -> (usize, usize) {
    let p: Vec<&str> = src.split(':').collect();
    if p.len() >= 2 {
        (p[0].parse().unwrap_or(0), p[1].parse().unwrap_or(0))
    } else {
        (0, 0)
    }
}

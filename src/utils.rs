use std::sync::OnceLock;
use tower_lsp::lsp_types::{Position, PositionEncodingKind};

// ---------------------------------------------------------------------------
// Position Encoding
// ---------------------------------------------------------------------------

static ENCODING: OnceLock<PositionEncoding> = OnceLock::new();

/// Store the negotiated encoding.  Called exactly once from the LSP
/// `initialize` handler.  Subsequent calls are silently ignored.
pub fn set_encoding(enc: PositionEncoding) {
    let _ = ENCODING.set(enc);
}

/// Read the negotiated encoding (falls back to UTF-16 if never set).
pub fn encoding() -> PositionEncoding {
    ENCODING.get().copied().unwrap_or_default()
}

/// How the LSP client counts column offsets within a line.
///
/// Set once during `initialize()` via [`set_encoding`] and read implicitly by
/// [`byte_offset_to_position`] and [`position_to_byte_offset`].  All other
/// modules are encoding-agnostic — they never need to know or pass this value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PositionEncoding {
    /// Column = number of bytes from the start of the line (UTF-8 code units).
    Utf8,
    /// Column = number of UTF-16 code units from the start of the line.
    /// This is the **mandatory default** per the LSP specification.
    #[default]
    Utf16,
}

impl PositionEncoding {
    /// Pick the best encoding from the set the client advertises.
    ///
    /// Preference: UTF-8 if supported, otherwise UTF-16 (the mandatory fallback).
    pub fn negotiate(client_encodings: Option<&[PositionEncodingKind]>) -> Self {
        let Some(encodings) = client_encodings else {
            return Self::default();
        };
        if encodings.contains(&PositionEncodingKind::UTF8) {
            PositionEncoding::Utf8
        } else {
            PositionEncoding::Utf16
        }
    }
}

impl From<PositionEncoding> for PositionEncodingKind {
    fn from(value: PositionEncoding) -> Self {
        match value {
            PositionEncoding::Utf8 => PositionEncodingKind::UTF8,
            PositionEncoding::Utf16 => PositionEncodingKind::UTF16,
        }
    }
}

// ---------------------------------------------------------------------------
// Byte-offset to LSP Position conversion
// ---------------------------------------------------------------------------

/// Convert a byte offset in `source` to a [`Position`] whose column unit depends
/// on the negotiated [`PositionEncoding`].
pub fn byte_offset_to_position(source: &str, byte_offset: usize) -> Position {
    let enc = encoding();
    let mut line: u32 = 0;
    let mut col: u32 = 0;

    for (i, ch) in source.char_indices() {
        if i >= byte_offset {
            break;
        }
        match (ch, enc) {
            ('\n', _) => {
                line += 1;
                col = 0;
            }
            ('\r', _) => {}
            (ch, PositionEncoding::Utf8) => {
                col += ch.len_utf8() as u32;
            }
            (ch, PositionEncoding::Utf16) => {
                col += ch.len_utf16() as u32;
            }
        }
    }
    Position {
        line,
        character: col,
    }
}

/// Convert an LSP [`Position`] position back to a byte offset, where
/// `character` is interpreted according to the negotiated [`PositionEncoding`].
pub fn position_to_byte_offset(source: &str, pos: Position) -> usize {
    let enc = encoding();
    let mut current_line: u32 = 0;
    let mut current_col: u32 = 0;

    for (i, ch) in source.char_indices() {
        if current_line >= pos.line && current_col >= pos.character {
            return i;
        }

        match (ch, enc) {
            ('\n', _) => {
                if current_line == pos.line {
                    return i; // clamp to end of line
                }
                current_line += 1;
                current_col = 0;
            }
            (ch, PositionEncoding::Utf8) => {
                current_col += ch.len_utf8() as u32;
            }
            (ch, PositionEncoding::Utf16) => {
                current_col += ch.len_utf16() as u32;
            }
        }
    }
    source.len() // position not found, we default to the end of the content
}

pub fn is_valid_solidity_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !first.is_ascii_alphabetic() && first != '_' && first != '$' {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$')
}

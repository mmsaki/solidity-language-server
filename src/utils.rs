use std::{
    path::{Path, PathBuf},
    sync::OnceLock,
};
use tower_lsp::lsp_types::PositionEncodingKind;

/// How the LSP client counts column offsets within a line.
///
/// Set once during `initialize()` via [`set_encoding`] and read implicitly by
/// [`byte_offset_to_position`] and [`position_to_byte_offset`].  All other
/// modules are encoding-agnostic — they never need to know or pass this value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionEncoding {
    /// Column = number of bytes from the start of the line (UTF-8 code units).
    Utf8,
    /// Column = number of UTF-16 code units from the start of the line.
    /// This is the **mandatory default** per the LSP specification.
    Utf16,
}

impl PositionEncoding {
    /// The mandatory LSP fallback encoding.
    pub const DEFAULT: Self = PositionEncoding::Utf16;

    /// Pick the best encoding from the set the client advertises.
    ///
    /// Preference: UTF-8 if supported, otherwise UTF-16 (the mandatory fallback).
    pub fn negotiate(client_encodings: Option<&[PositionEncodingKind]>) -> Self {
        let Some(encodings) = client_encodings else {
            return Self::DEFAULT;
        };
        if encodings.contains(&PositionEncodingKind::UTF8) {
            PositionEncoding::Utf8
        } else {
            PositionEncoding::Utf16
        }
    }

    /// Convert to the LSP wire type.
    pub fn to_encoding_kind(self) -> PositionEncodingKind {
        match self {
            PositionEncoding::Utf8 => PositionEncodingKind::UTF8,
            PositionEncoding::Utf16 => PositionEncodingKind::UTF16,
        }
    }
}

// ---------------------------------------------------------------------------
// Global encoding state — written once in `initialize`, read everywhere.
// ---------------------------------------------------------------------------

static ENCODING: OnceLock<PositionEncoding> = OnceLock::new();

/// Store the negotiated encoding.  Called exactly once from the LSP
/// `initialize` handler.  Subsequent calls are silently ignored.
pub fn set_encoding(enc: PositionEncoding) {
    let _ = ENCODING.set(enc);
}

/// Read the negotiated encoding (falls back to UTF-16 if never set).
pub fn encoding() -> PositionEncoding {
    ENCODING.get().copied().unwrap_or(PositionEncoding::DEFAULT)
}

// ---------------------------------------------------------------------------
// Byte-offset ↔ LSP-position conversion
// ---------------------------------------------------------------------------

/// Convert a byte offset in `source` to an `(line, column)` pair whose column
/// unit depends on the negotiated [`PositionEncoding`].
pub fn byte_offset_to_position(source: &str, byte_offset: usize) -> (u32, u32) {
    let enc = encoding();
    let mut line: u32 = 0;
    let mut col: u32 = 0;
    let bytes = source.as_bytes();
    let mut i = 0;

    while i < byte_offset && i < bytes.len() {
        match bytes[i] {
            b'\n' => {
                line += 1;
                col = 0;
                i += 1;
            }
            b'\r' if i + 1 < bytes.len() && bytes[i + 1] == b'\n' => {
                line += 1;
                col = 0;
                i += 2;
            }
            _ => {
                match enc {
                    PositionEncoding::Utf8 => {
                        // One byte = one UTF-8 code unit.
                        col += 1;
                        i += 1;
                    }
                    PositionEncoding::Utf16 => {
                        // Advance by the full character, count UTF-16 code units.
                        let ch_len = utf8_char_len(bytes[i]);
                        let ch = &source[i..i + ch_len];
                        col += ch.chars().next().map(|c| c.len_utf16() as u32).unwrap_or(1);
                        i += ch_len;
                    }
                }
            }
        }
    }

    (line, col)
}

/// Convert an LSP `(line, character)` position back to a byte offset, where
/// `character` is interpreted according to the negotiated [`PositionEncoding`].
pub fn position_to_byte_offset(source: &str, line: u32, character: u32) -> usize {
    let enc = encoding();
    let mut current_line: u32 = 0;
    let mut current_col: u32 = 0;

    for (i, ch) in source.char_indices() {
        if current_line == line && current_col == character {
            return i;
        }

        match ch {
            '\n' => {
                if current_line == line {
                    return i; // clamp to end of line
                }
                current_line += 1;
                current_col = 0;
            }
            _ => {
                current_col += match enc {
                    PositionEncoding::Utf8 => ch.len_utf8() as u32,
                    PositionEncoding::Utf16 => ch.len_utf16() as u32,
                };
            }
        }
    }

    source.len()
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Number of bytes in a UTF-8 character given its leading byte.
fn utf8_char_len(lead: u8) -> usize {
    match lead {
        0x00..=0x7F => 1,
        0xC0..=0xDF => 2,
        0xE0..=0xEF => 3,
        0xF0..=0xF7 => 4,
        _ => 1, // continuation byte — shouldn't happen at a char boundary
    }
}

pub fn is_valid_solidity_identifier(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    let chars: Vec<char> = name.chars().collect();
    let first = chars[0];
    if !first.is_ascii_alphabetic() && first != '_' {
        return false;
    }
    for &c in &chars {
        if !c.is_ascii_alphanumeric() && c != '_' {
            return false;
        }
    }
    true
}

/// Returns the path of the top-level directory of the working git tree.
pub fn find_git_root(path: impl AsRef<Path>) -> Option<PathBuf> {
    path.as_ref()
        .ancestors()
        .find(|p| p.join(".git").exists())
        .map(Path::to_path_buf)
}

/// Finds the foundry project root by walking up from `path` looking for `foundry.toml`,
/// bounded by the git root. Falls back to the git root if no `foundry.toml` is found.
pub fn find_project_root(path: impl AsRef<Path>) -> Option<PathBuf> {
    let path = path.as_ref();
    let boundary = find_git_root(path);
    let found = path
        .ancestors()
        .take_while(|p| {
            if let Some(boundary) = &boundary {
                p.starts_with(boundary)
            } else {
                true
            }
        })
        .find(|p| p.join("foundry.toml").is_file())
        .map(Path::to_path_buf);
    found.or(boundary)
}

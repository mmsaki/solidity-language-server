pub fn byte_offset_to_position(source: &str, byte_offset: usize) -> (u32, u32) {
    let mut line = 0;
    let mut col = 0;
    let mut i = 0;

    let bytes = source.as_bytes();
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
                col += 1;
                i += 1;
            }
        }
    }

    (line, col)
}

pub fn position_to_byte_offset(source: &str, line: u32, character: u32) -> usize {
    let mut current_line = 0;
    let mut current_col = 0;

    for (i, ch) in source.char_indices() {
        if current_line == line && current_col == character {
            return i;
        }

        match ch {
            '\n' => {
                if current_line == line && current_col < character {
                    return i; // clamp to end of line
                }
                current_line += 1;
                current_col = 0;
            }
            _ => {
                current_col += 1;
            }
        }
    }

    source.len()
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

use solidity_language_server::utils::{
    byte_offset_to_position, is_valid_solidity_identifier, position_to_byte_offset,
};

#[test]
fn test_byte_offset_to_position_unix_newlines() {
    let source = "line1\nline2\nline3\n";
    assert_eq!(byte_offset_to_position(source, 0), (0, 0)); // 'l' in line1
    assert_eq!(byte_offset_to_position(source, 5), (0, 5)); // '\n'
    assert_eq!(byte_offset_to_position(source, 6), (1, 0)); // 'l' in line2
    assert_eq!(byte_offset_to_position(source, 11), (1, 5)); // '\n'
    assert_eq!(byte_offset_to_position(source, 12), (2, 0)); // 'l' in line3
}

#[test]
fn test_byte_offset_to_position_windows_newlines() {
    let source = "line1\r\nline2\r\nline3\r\n";
    assert_eq!(byte_offset_to_position(source, 0), (0, 0));
    assert_eq!(byte_offset_to_position(source, 5), (0, 5));
    assert_eq!(byte_offset_to_position(source, 7), (1, 0)); // skips \r\n
    assert_eq!(byte_offset_to_position(source, 12), (1, 5));
    assert_eq!(byte_offset_to_position(source, 14), (2, 0));
}

#[test]
fn test_byte_offset_to_position_no_newlines() {
    let source = "justoneline";
    assert_eq!(byte_offset_to_position(source, 0), (0, 0));
    assert_eq!(byte_offset_to_position(source, 5), (0, 5));
    assert_eq!(byte_offset_to_position(source, 11), (0, 11));
}

#[test]
fn test_byte_offset_to_position_offset_out_of_bounds() {
    let source = "short\nfile";
    let offset = source.len() + 10;
    assert_eq!(byte_offset_to_position(source, offset), (1, 4));
}

#[test]
fn test_byte_offset_to_position_empty_source() {
    let source = "";
    assert_eq!(byte_offset_to_position(source, 0), (0, 0));
    assert_eq!(byte_offset_to_position(source, 10), (0, 0));
}

#[test]
fn test_position_to_byte_offset_basic() {
    let source = "line1\nline2\nline3\n";
    assert_eq!(position_to_byte_offset(source, 0, 0), 0); // 'l'
    assert_eq!(position_to_byte_offset(source, 0, 5), 5); // '\n'
    assert_eq!(position_to_byte_offset(source, 1, 0), 6); // 'l' in line2
    assert_eq!(position_to_byte_offset(source, 1, 3), 9); // 'e' in line2
    assert_eq!(position_to_byte_offset(source, 2, 0), 12); // 'l' in line3
}

#[test]
fn test_position_to_byte_offset_out_of_bounds() {
    let source = "line1\nline2\n";
    assert_eq!(position_to_byte_offset(source, 10, 10), source.len());
}

#[test]
fn test_position_to_byte_offset_empty() {
    let source = "";
    assert_eq!(position_to_byte_offset(source, 0, 0), 0);
}

#[test]
fn test_is_valid_solidity_identifier() {
    assert!(is_valid_solidity_identifier("validName"));
    assert!(is_valid_solidity_identifier("_valid"));
    assert!(is_valid_solidity_identifier("a"));
    assert!(is_valid_solidity_identifier("name123"));
    assert!(is_valid_solidity_identifier("_"));
    assert!(is_valid_solidity_identifier("a_b_c"));

    assert!(!is_valid_solidity_identifier(""));
    assert!(!is_valid_solidity_identifier("123invalid"));
    assert!(!is_valid_solidity_identifier("invalid-name"));
    assert!(!is_valid_solidity_identifier("invalid name"));
    assert!(!is_valid_solidity_identifier("invalid.name"));
}

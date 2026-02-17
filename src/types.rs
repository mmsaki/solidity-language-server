/// Type wrapper for AST node IDs.
///
/// Every node in the Solidity compiler's JSON AST has a unique numeric `id`.
/// Wrapping it prevents accidental mixups with [`FileId`] or plain integers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub u64);

/// Newtype wrapper for source file IDs.
///
/// The compiler assigns each input file a numeric ID that appears in `src`
/// strings (`"offset:length:fileId"`). Wrapping it prevents accidental
/// mixups with [`NodeId`] or plain integers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileId(pub u64);

/// A parsed `"offset:length:fileId"` source location from the AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceLoc {
    /// Byte offset in the source file.
    pub offset: usize,
    /// Byte length of the source range.
    pub length: usize,
    /// ID of the source file this location belongs to.
    pub file_id: FileId,
}

impl SourceLoc {
    /// Parse a `"offset:length:fileId"` string.
    ///
    /// Returns `None` if the format is invalid or any component fails to parse.
    pub fn parse(src: &str) -> Option<Self> {
        let mut parts = src.split(':');
        let offset = parts.next()?.parse::<usize>().ok()?;
        let length = parts.next()?.parse::<usize>().ok()?;
        let file_id = parts.next()?.parse::<u64>().ok()?;
        // Reject if there are extra parts
        if parts.next().is_some() {
            return None;
        }
        Some(Self {
            offset,
            length,
            file_id: FileId(file_id),
        })
    }

    /// End byte offset (`offset + length`).
    pub fn end(&self) -> usize {
        self.offset + self.length
    }

    /// The file ID as a string, for use as a HashMap key when interacting
    /// with the `source_id_to_path` map (which uses string keys).
    pub fn file_id_str(&self) -> String {
        self.file_id.0.to_string()
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for FileId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_loc_parse_valid() {
        let loc = SourceLoc::parse("100:50:3").unwrap();
        assert_eq!(loc.offset, 100);
        assert_eq!(loc.length, 50);
        assert_eq!(loc.file_id, FileId(3));
        assert_eq!(loc.end(), 150);
        assert_eq!(loc.file_id_str(), "3");
    }

    #[test]
    fn test_source_loc_parse_zero() {
        let loc = SourceLoc::parse("0:0:0").unwrap();
        assert_eq!(loc.offset, 0);
        assert_eq!(loc.length, 0);
        assert_eq!(loc.file_id, FileId(0));
    }

    #[test]
    fn test_source_loc_parse_invalid_format() {
        assert!(SourceLoc::parse("").is_none());
        assert!(SourceLoc::parse("100").is_none());
        assert!(SourceLoc::parse("100:50").is_none());
        assert!(SourceLoc::parse("abc:50:3").is_none());
        assert!(SourceLoc::parse("100:abc:3").is_none());
        assert!(SourceLoc::parse("100:50:abc").is_none());
    }

    #[test]
    fn test_source_loc_parse_rejects_extra_parts() {
        assert!(SourceLoc::parse("100:50:3:extra").is_none());
    }

    #[test]
    fn test_node_id_equality() {
        assert_eq!(NodeId(42), NodeId(42));
        assert_ne!(NodeId(42), NodeId(43));
    }

    #[test]
    fn test_file_id_equality() {
        assert_eq!(FileId(1), FileId(1));
        assert_ne!(FileId(1), FileId(2));
    }

    #[test]
    fn test_node_id_file_id_are_different_types() {
        // This test documents the compile-time guarantee.
        // NodeId(1) and FileId(1) are different types — they cannot be
        // compared or used interchangeably.
        let _n: NodeId = NodeId(1);
        let _f: FileId = FileId(1);
        // If you uncomment the following line, it won't compile:
        // assert_ne!(_n, _f);
    }
}

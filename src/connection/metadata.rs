use super::Command;

/// Extract the metadata of a request
///
/// The metadata is one byte in the form:
/// first 4 bits: version as u4
/// and the left over 4 bits: command as u4
pub(super) fn extract_metadata(metadata: &u8) -> Result<(u8, Command), anyhow::Error> {
    let version = metadata >> 4; // example 0b0001_0000 (v1) -> 0b0000_0001 == 1

    // only look at the last 4 bits
    let command = metadata & 0b0000_1111;

    match command {
        0 => Ok((version, Command::Get)),
        1 => Ok((version, Command::Set)),
        2 => Ok((version, Command::Insert)),
        3 => Ok((version, Command::Remove)),
        _ => Err(anyhow::anyhow!("unknown command {}", command))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_metadata_get() {
        let metadata = 0b0001_0000;
        let (version, command) = extract_metadata(&metadata).unwrap();
        assert_eq!(version, 1);
        assert_eq!(command, Command::Get);
    }

    #[test]
    fn extract_metadata_set() {
        let metadata = 0b0001_0001;
        let (version, command) = extract_metadata(&metadata).unwrap();
        assert_eq!(version, 1);
        assert_eq!(command, Command::Set);
    }

    #[test]
    fn extract_metadata_insert() {
        let metadata = 0b0001_0010;
        let (version, command) = extract_metadata(&metadata).unwrap();
        assert_eq!(version, 1);
        assert_eq!(command, Command::Insert);
    }

    #[test]
    fn extract_metadata_remove() {
        let metadata = 0b0001_0011;
        let (version, command) = extract_metadata(&metadata).unwrap();
        assert_eq!(version, 1);
        assert_eq!(command, Command::Remove);
    }

    #[test]
    fn extract_metadata_unknown_command() {
        let metadata = 0b0001_1111;
        let res = extract_metadata(&metadata);
        assert!(res.is_err());
        assert_eq!(res.err().unwrap().to_string(), "unknown command 15");
    }
}
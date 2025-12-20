use anyhow::Context;
use tokio::io::{AsyncReadExt};
use crate::types::Command;

#[derive(Debug, PartialEq)]
pub(super) struct HeaderData {
    pub(super) version: u8,
    pub(super) command: Command,
    pub(super) content_length: u16,
}

pub(super) async fn read_header<R>(reader: &mut R) -> Result<HeaderData, anyhow::Error>
where
    R: AsyncReadExt + Unpin,
{
    // u8 version + u8 command + u16 content length
    let mut header = [0u8; 4];
    reader.read_exact(&mut header).await
        .context("header too short")?;

    let version = header[0];
    let command = header[1].try_into()
        .context("invalid command")?;
    let content_length = u16::from_be_bytes([header[2], header[3]]);

    Ok(HeaderData {
        version,
        command,
        content_length
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[tokio::test]
    async fn test_read_header() {
        let data = [
            /* version 1 */ 0x01,
            /* command get */ 0x00,
            /* content_length 255 */ 0x00, 0xFF];
        let mut cursor = Cursor::new(data);

        let result = read_header(&mut cursor).await.unwrap();
        assert_eq!(result, HeaderData {
            version: 1,
            command: Command::Get,
            content_length: 255
        });
    }

    #[tokio::test]
    async fn test_read_header_too_short() {
        let data = [
            /* version 1 */ 0x01,
            /* command get */ 0x00,
            /* content_length 255, missing one byte */ 0x00];
        let mut cursor = Cursor::new(data);

        let err = read_header(&mut cursor).await.unwrap_err();
        assert!(err.to_string().contains("header too short"));
    }

    #[tokio::test]
    async fn test_read_header_with_invalid_command() {
        let data = [
            /* version 1 */ 0x01,
            /* command invalid */ 0x10,
            /* content_length 255 */ 0x00, 0xFF];
        let mut cursor = Cursor::new(data);

        let err = read_header(&mut cursor).await.unwrap_err();
        assert!(err.to_string().contains("invalid command"));
    }
}
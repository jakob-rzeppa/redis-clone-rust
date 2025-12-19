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
    // u8 version
    let version = reader.read_u8().await
        .context("failed to read version")?;

    // u8 command (GET, SET, INSERT, REMOVE)
    let command: Command = reader
        .read_u8().await.context("failed to read command")?
        .try_into().context("failed to read command")?;

    // u16 content length
    let content_length = reader.read_u16().await
        .context("failed to read content_length")?;

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
    async fn test_read_header_with_missing_byte_content_length() {
        let data = [
            /* version 1 */ 0x01,
            /* command get */ 0x00,
            /* content_length 255, missing one byte */ 0x00];
        let mut cursor = Cursor::new(data);

        let err = read_header(&mut cursor).await.unwrap_err();
        assert!(err.to_string().contains("failed to read content_length"));
    }

    #[tokio::test]
    async fn test_read_header_with_invalid_command() {
        let data = [
            /* version 1 */ 0x01,
            /* command invalid */ 0x10,
            /* content_length 255 */ 0x00, 0xFF];
        let mut cursor = Cursor::new(data);

        let err = read_header(&mut cursor).await.unwrap_err();
        assert!(err.to_string().contains("failed to read command"));
    }

    #[tokio::test]
    async fn test_read_header_with_missing_command() {
        let data = [
            /* version 1 */ 0x01
            /* missing command */];
        let mut cursor = Cursor::new(data);

        let err = read_header(&mut cursor).await.unwrap_err();
        assert!(err.to_string().contains("failed to read command"));
    }

    #[tokio::test]
    async fn test_read_header_with_missing_version() {
        let data = [];
        let mut cursor = Cursor::new(data);

        let err = read_header(&mut cursor).await.unwrap_err();
        assert!(err.to_string().contains("failed to read version"));
    }
}
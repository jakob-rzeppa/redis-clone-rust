use anyhow::Context;
use tokio::io::{AsyncReadExt};
use crate::connection::metadata::Command;
use crate::connection::metadata::extract_metadata;

#[derive(Debug, PartialEq)]
pub(super) enum HeaderData {
    Get { id: u32 },
    Set { id: u32, content_length: u16 },
    Insert { content_length: u16 },
    Remove { id: u32 },
}

pub(super) async fn read_header<S>(stream: &mut S) -> Result<HeaderData, anyhow::Error>
where
    S: AsyncReadExt + Unpin,
{
    // get metadata from the stream
    let metadata = stream.read_u8().await
        .context("read metadata failed")?;

    // extract the version and command from the metadata
    let (version, command) = extract_metadata(&metadata)
        .context("parse metadata failed")?;

    // only version 1 is implemented
    if version != 1 {
        return Err(anyhow::Error::msg("wrong version"))
    }

    // extract metadata for each command
    match command {
        Command::Get => {
            let id = stream.read_u32().await
                .context("read id failed")?;

            Ok(HeaderData::Get { id })
        }
        Command::Set => {
            let id = stream.read_u32().await
                .context("read id failed")?;
            let content_length = stream.read_u16().await
                .context("read content_length failed")?;

            Ok(HeaderData::Set { id, content_length })
        }
        Command::Insert => {
            let content_length = stream.read_u16().await
                .context("read content_length failed")?;

            Ok(HeaderData::Insert { content_length })
        }
        Command::Remove => {
            let id = stream.read_u32().await
                .context("read id failed")?;

            Ok(HeaderData::Remove { id })
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[tokio::test]
    async fn test_get() {
        let data = [0b0001_0000,
            /* id 9 in u32 */ 0x00, 0x00, 0x00, 0x09];
        let mut cursor = Cursor::new(data);

        let result = read_header(&mut cursor).await.unwrap();
        assert_eq!(result, HeaderData::Get { id: 0x09 });
    }

    #[tokio::test]
    async fn test_get_invalid() {
        let data = [0b0001_0000,
            /* id 0 in u32 (one byte missing) */ 0x00, 0x00, 0x00];
        let mut cursor = Cursor::new(data);

        let err = read_header(&mut cursor).await.unwrap_err();
        assert!(err.to_string().contains("read id failed"));
    }

    #[tokio::test]
    async fn test_set() {
        let data = [0b0001_0001,
            /* id 9 in u32 */ 0x00, 0x00, 0x00, 0x09,
            /* content_length 30 in u16 */ 0x00, 30];
        let mut cursor = Cursor::new(data);

        let result = read_header(&mut cursor).await.unwrap();
        assert_eq!(result, HeaderData::Set { id: 0x09, content_length: 30 });
    }

    #[tokio::test]
    async fn test_set_invalid_content_length() {
        let data = [0b0001_0001,
            /* id 9 in u32 */ 0x00, 0x00, 0x00, 0x09,
            /* content_length 30 in u16 (one byte missing) */ 0x00];
        let mut cursor = Cursor::new(data);

        let err = read_header(&mut cursor).await.unwrap_err();
        assert!(err.to_string().contains("read content_length failed"));
    }

    #[tokio::test]
    async fn test_set_invalid_id() {
        let data = [0b0001_0001,
            /* id 9 in u32 (one byte missing) */ 0x00, 0x00, 0x00
            /* content_length missing */];
        let mut cursor = Cursor::new(data);

        let err = read_header(&mut cursor).await.unwrap_err();
        assert!(err.to_string().contains("read id failed"));
    }

    #[tokio::test]
    async fn test_insert() {
        let data = [0b0001_0010,
            /* content_length 30 in u16 */ 0x00, 30];
        let mut cursor = Cursor::new(data);

        let result = read_header(&mut cursor).await.unwrap();
        assert_eq!(result, HeaderData::Insert { content_length: 30 });
    }

    #[tokio::test]
    async fn test_insert_invalid() {
        let data = [0b0001_0010,
            /* content_length 30 in u16 (missing one byte) */ 0x00];
        let mut cursor = Cursor::new(data);

        let err = read_header(&mut cursor).await.unwrap_err();
        assert!(err.to_string().contains("read content_length failed"));
    }

    #[tokio::test]
    async fn test_remove() {
        let data = [0b0001_0011,
            /* id 9 in u32 */ 0x00, 0x00, 0x00, 0x09];
        let mut cursor = Cursor::new(data);

        let result = read_header(&mut cursor).await.unwrap();
        assert_eq!(result, HeaderData::Remove { id: 9 });
    }

    #[tokio::test]
    async fn test_remove_invalid() {
        let data = [0b0001_0011,
            /* id 9 in u32 (missing three byte) */ 0x00];
        let mut cursor = Cursor::new(data);

        let err = read_header(&mut cursor).await.unwrap_err();
        assert!(err.to_string().contains("read id failed"));
    }

    #[tokio::test]
    async fn test_empty() {
        let data = [];
        let mut cursor = Cursor::new(data);

        let err = read_header(&mut cursor).await.unwrap_err();
        assert!(err.to_string().contains("read metadata failed"));
    }

    #[tokio::test]
    async fn test_invalid_version() {
        let data = [0b0011_0000,
            /* id 9 in u32 */ 0x00, 0x00, 0x00, 0x09];
        let mut cursor = Cursor::new(data);

        let err = read_header(&mut cursor).await.unwrap_err();
        assert!(err.to_string().contains("wrong version"));
    }

    #[tokio::test]
    async fn test_invalid_command() {
        let data = [0b0001_1000]; // invalid command
        let mut cursor = Cursor::new(data);

        let err = read_header(&mut cursor).await.unwrap_err();
        assert!(err.to_string().contains("parse metadata failed"));
    }
}
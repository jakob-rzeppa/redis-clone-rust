use anyhow::Context;
use tokio::io::{AsyncRead, AsyncReadExt};

pub(super) async fn read_content<S>(stream: &mut S, content_length: usize) -> Result<Vec<u8>, anyhow::Error>
where
    S: AsyncRead + Unpin,
{
    let mut content = vec![0; content_length];

    stream.read_exact(&mut content).await
        .context("read content failed")?;

    // if read_u8 works not all bytes from the stream are read => the content_length was to short
    if stream.read_u8().await.is_ok() {
        return Err(anyhow::anyhow!("read content failed: stream contains more bytes as expected"));
    }

    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[tokio::test]
    async fn test_read_content() {
        let data = b"Hello, world!";
        let mut cursor = Cursor::new(data);

        let result = read_content(&mut cursor, data.len()).await.unwrap();
        assert_eq!(result, data);
    }

    #[tokio::test]
    async fn test_read_content_too_short() {
        let data = b"Short";
        let mut cursor = Cursor::new(data);

        let err = read_content(&mut cursor, data.len() + 5).await.unwrap_err();
        assert!(err.to_string().contains("read content failed"));
    }

    #[tokio::test]
    async fn test_read_content_too_long() {
        let data = b"Loooooooooooooooooooong";
        let mut cursor = Cursor::new(data);

        let err = read_content(&mut cursor, data.len() - 5).await.unwrap_err();
        assert!(err.to_string().contains("read content failed"));
    }
}
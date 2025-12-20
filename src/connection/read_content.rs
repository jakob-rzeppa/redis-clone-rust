use anyhow::Context;
use tokio::io::{AsyncRead, AsyncReadExt};

pub(super) async fn read_content<S>(stream: &mut S, content_length: usize) -> Result<Vec<u8>, anyhow::Error>
where
    S: AsyncRead + Unpin,
{
    let mut content = vec![0; content_length];

    stream.read_exact(&mut content).await
        .context("content smaller than expected")?;

    if content.len() < content_length {
        return Err(anyhow::anyhow!("content smaller than expected"))
    }

    let eot_byte = stream.read_u8().await
        .context("read eot character failed")?;

    if eot_byte != 0x04 {
        return Err(anyhow::anyhow!("eot character not found"))
    }

    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[tokio::test]
    async fn test_read_content() {
        let data = b"Hello, world!\x04";
        let mut cursor = Cursor::new(data);

        let result = read_content(&mut cursor, data.len() - 1).await.unwrap();
        assert_eq!(result, data[0..data.len()-1]);
    }

    #[tokio::test]
    async fn test_read_content_no_eot_character() {
        let data = b"Hello, world!";
        let mut cursor = Cursor::new(data);

        let err = read_content(&mut cursor, data.len()).await.unwrap_err();
        assert!(err.to_string().contains("read eot character failed"));
    }

    #[tokio::test]
    async fn test_read_content_too_short() {
        let data = b"Short\x04";
        let mut cursor = Cursor::new(data);

        let err = read_content(&mut cursor, data.len() + 5).await.unwrap_err();
        assert!(err.to_string().contains("content smaller than expected"));
    }

    #[tokio::test]
    async fn test_read_content_too_long() {
        let data = b"Loooooooooooooooooooong\x04";
        let mut cursor = Cursor::new(data);

        let err = read_content(&mut cursor, data.len() - 5).await.unwrap_err();
        assert!(err.to_string().contains("eot character not found"));
    }
}
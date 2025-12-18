use anyhow::Context;
use tokio::io::{AsyncRead, AsyncReadExt};

pub(super) async fn read_content<S>(stream: &mut S, content_length: usize) -> Result<Vec<u8>, anyhow::Error>
where
    S: AsyncRead + Unpin,
{
    let mut content = vec![0; content_length];

    stream.read_exact(&mut content).await
        .context("read content failed")?;

    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_read_content() {

    }
}
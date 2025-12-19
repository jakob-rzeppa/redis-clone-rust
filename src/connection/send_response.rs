use anyhow::Context;
use tokio::io::{AsyncWriteExt};
use crate::types::{Response};

pub(super) async fn send_response<W>(response: Response, mut writer: W) -> Result<(), anyhow::Error>
where
    W: AsyncWriteExt + Unpin,
{
    writer.write_u8(response.version).await
        .context("failed to send version")?;
    writer.write_u8(response.command.into()).await
        .context("failed to send version")?;
    writer.write_u16(response.status_code.into()).await
        .context("failed to send status code")?;
    writer.write_u16(response.content_length).await
        .context("failed to send content_length")?;

    if let Some(content) = response.content {
        writer.write_all(&content).await
            .context("failed to send content")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use crate::types::{Command, StatusCode};

    #[tokio::test]
    async fn test_send_response() {
        let content = b"hello";

        let response = Response {
            version: 1,
            command: Command::Get,
            status_code: StatusCode::Ok,
            content_length: content.len() as u16,
            content: Some(content.to_vec()),
        };

        let buffer = Vec::new();
        let mut cursor = Cursor::new(buffer);

        send_response(response, &mut cursor)
            .await
            .expect("send_response failed");

        let written = cursor.into_inner();

        let expected = vec![
            1,                // version
            0,                // command
            0, 200,            // status_code (u16, big-endian)
            0, 5,              // content_length (u16)
            b'h', b'e', b'l', b'l', b'o',
        ];

        assert_eq!(written, expected);
    }

    #[tokio::test]
    async fn test_send_response_no_content() {
        let response = Response {
            version: 1,
            command: Command::Get,
            status_code: StatusCode::Ok,
            content_length: 0u16,
            content: None,
        };

        let buffer = Vec::new();
        let mut cursor = Cursor::new(buffer);

        send_response(response, &mut cursor)
            .await
            .expect("send_response failed");

        let written = cursor.into_inner();

        let expected = vec![
            1,                // version
            0,                // command
            0, 200,            // status_code (u16, big-endian)
            0, 0,              // content_length (u16)
        ];

        assert_eq!(written, expected);
    }
}
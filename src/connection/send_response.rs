use anyhow::Context;
use tokio::io::{AsyncWriteExt};
use crate::types::{Response};
use crate::util::big_endian;

pub(super) async fn send_response<W>(response: Response, mut writer: W) -> Result<(), anyhow::Error>
where
    W: AsyncWriteExt + Unpin,
{
    let mut msg: Vec<u8> = Vec::with_capacity(6 /* Header */ + response.content_length as usize);

    msg.push(response.version);
    msg.push(response.command as u8);
    msg.append(&mut big_endian::u16_to_vec(&(response.status_code as u16)));
    msg.append(&mut big_endian::u16_to_vec(&response.content_length));

    if let Some(mut content) = response.content {
        msg.append(&mut content);
    }

    // EOT character
    msg.push(0x04);

    writer.write_all(&msg).await
        .context("failed to send response")?;

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
            4 // eot
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
            4 // eot
        ];

        assert_eq!(written, expected);
    }
}
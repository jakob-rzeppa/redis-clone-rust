use anyhow::Context;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::TcpStream;

pub(super) async fn read_content(reader: &mut BufReader<TcpStream>, content_length: usize) -> Result<Vec<u8>, anyhow::Error> {
    let mut content = Vec::with_capacity(content_length);

    reader.read_exact(&mut content).await
        .context("read content failed")?;

    Ok(content)
}
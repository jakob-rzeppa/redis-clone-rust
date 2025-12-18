use anyhow::Context;
use tokio::io::{AsyncReadExt};
use crate::connection::Command;
use crate::connection::metadata::extract_metadata;

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
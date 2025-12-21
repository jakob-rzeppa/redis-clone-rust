mod get;
mod set;
mod remove;

use get::handle_get_request;
use remove::handle_remove_request;
use set::handle_set_request;
use crate::types::{Command, Request, Response, StatusCode};

pub(crate) async fn route_request(request: Request) -> Response {
    match request.command {
        Command::Get => handle_get_request(request).await,
        Command::Set => handle_set_request(request).await,
        Command::Remove => handle_remove_request(request).await,
        Command::Invalid => Response {
            version: request.version,
            command: request.command,
            status_code: StatusCode::NotFound,
            content_length: 0,
            content: None,
        }
    }
}
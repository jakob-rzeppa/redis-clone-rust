mod get;
mod set;
mod remove;
mod insert;

use get::handle_get_request;
use remove::handle_remove_request;
use set::handle_set_request;
use crate::controller::insert::handle_insert_request;
use crate::repository::SharedRepository;
use crate::types::{Command, Request, Response, StatusCode};

pub(crate) async fn route_request(request: Request, db: SharedRepository) -> Response {
    match request.command {
        Command::Get => handle_get_request(request, db).await,
        Command::Set => handle_set_request(request, db).await,
        Command::Insert => handle_insert_request(request, db).await,
        Command::Remove => handle_remove_request(request, db).await,
        Command::Invalid => Response {
            version: request.version,
            command: request.command,
            status_code: StatusCode::NotFound,
            content_length: 0,
            content: None,
        }
    }
}
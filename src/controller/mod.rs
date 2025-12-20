mod get;
mod set;
mod insert;
mod remove;

use get::handle_get_request;
use insert::handle_insert_request;
use remove::handle_remove_request;
use set::handle_set_request;
use crate::types::{Command, Request, Response};

pub(crate) async fn route_request(request: Request) -> Response {
    match request.command {
        Command::Get => handle_get_request(request).await,
        Command::Set => handle_set_request(request).await,
        Command::Insert => handle_insert_request(request).await,
        Command::Remove => handle_remove_request(request).await,
    }
}
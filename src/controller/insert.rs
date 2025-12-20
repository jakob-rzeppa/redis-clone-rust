use crate::types::{Request, Response, StatusCode};

pub(super) async fn handle_insert_request(request: Request) -> Response {
    Response {
        version: request.version,
        command: request.command,
        status_code: StatusCode::NotImplemented,
        content_length: 0,
        content: Some(Vec::new()),
    }
}
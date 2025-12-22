use crate::repository::error::DatabaseError;
use crate::repository::SharedRepository;
use crate::types::{Request, Response, StatusCode};

/// REMOVE REQUEST
///
/// Request Body:
/// 4 bytes u32 id to read
///
/// Responses:
/// 200 ok
/// 400 invalid request
/// 404 not found
/// 500 internal server error
pub(super) async fn handle_remove_request(request: Request, db: SharedRepository) -> Response {
    if request.content_length != 4 {
        return Response {
            version: request.version,
            command: request.command,
            status_code: StatusCode::InvalidRequest,
            content_length: 0,
            content: None,
        }
    }

    let content = match request.content {
        Some(content) => content,
        None => return Response {
            version: request.version,
            command: request.command,
            status_code: StatusCode::InvalidRequest,
            content_length: 0,
            content: None,
        }
    };

    // it is safe to assume, that all 4 bytes are here, since content_length is 4
    let id = u32::from_be_bytes([content[0], content[1], content[2], content[3]]);

    match db.remove(id).await {
        Ok(()) => {
            Response {
                version: request.version,
                command: request.command,
                status_code: StatusCode::Ok,
                content_length: 0,
                content: None,
            }
        }
        Err(err) => {
            match err {
                DatabaseError::NotFound(_) => Response {
                    version: request.version,
                    command: request.command,
                    status_code: StatusCode::NotFound,
                    content_length: 0,
                    content: None,
                },
                _ => Response {
                    version: request.version,
                    command: request.command,
                    status_code: StatusCode::InternalServerError,
                    content_length: 0,
                    content: None,
                }
            }

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::controller::set::handle_set_request;
    use crate::repository::MockRepository;
    use crate::types::{Command, Request};

    fn make_request(content_length: u16, content: Option<Vec<u8>>) -> Request {
        Request {
            version: 1,
            command: Command::Get,
            content_length,
            content,
        }
    }

    // ---- TESTS ----

    #[tokio::test]
    async fn invalid_request_when_content_length_not_4() {
        let mut mock = MockRepository::new();

        mock.expect_remove().never();

        let mock = Arc::new(mock);

        let request = make_request(3, Some(vec![0, 0, 0]));
        let response = handle_remove_request(request, mock).await;

        assert_eq!(response.status_code, StatusCode::InvalidRequest);
    }

    #[tokio::test]
    async fn invalid_request_when_content_missing() {
        let mut mock = MockRepository::new();

        mock.expect_remove().never();

        let mock = Arc::new(mock);

        let request = make_request(4, None);
        let response = handle_remove_request(request, mock).await;

        assert_eq!(response.status_code, StatusCode::InvalidRequest);
    }

    #[tokio::test]
    async fn not_found_when_entry_missing() {
        let mut mock = MockRepository::new();

        mock.expect_remove()
            .with(mockall::predicate::eq(42u32))
            .times(1)
            .returning(|_| Err(DatabaseError::NotFound(42)));

        let mock = Arc::new(mock);

        let request = make_request(4, Some(42u32.to_be_bytes().to_vec()));
        let response = handle_remove_request(request, mock).await;

        assert_eq!(response.status_code, StatusCode::NotFound);
    }

    #[tokio::test]
    async fn unknown_db_error()  {
        let mut mock = MockRepository::new();

        mock.expect_remove()
            .with(mockall::predicate::eq(42u32))
            .times(1)
            .returning(|_| Err(DatabaseError::AlreadyExists(42))); // Already Exists should not be returned

        let mock = Arc::new(mock);

        let request = make_request(4, Some(42u32.to_be_bytes().to_vec()));
        let response = handle_remove_request(request, mock).await;

        assert_eq!(StatusCode::InternalServerError, response.status_code);
    }

    #[tokio::test]
    async fn valid_request() {
        let mut mock = MockRepository::new();

        mock.expect_remove()
            .with(mockall::predicate::eq(42u32))
            .times(1)
            .returning(|_| Ok(()));

        let mock = Arc::new(mock);

        let request = make_request(4, Some(42u32.to_be_bytes().to_vec()));
        let response = handle_remove_request(request, mock).await;

        assert_eq!(response.status_code, StatusCode::Ok);
    }
}
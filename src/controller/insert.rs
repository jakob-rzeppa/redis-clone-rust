use crate::repository::error::DatabaseError;
use crate::repository::SharedRepository;
use crate::types::{Request, Response, StatusCode};

/// SET REQUEST
///
/// Request Body:
/// 4 bytes u32 id to read
/// content (at least 1 byte)
///
/// Responses:
/// 200 with content as body
/// 400 invalid request
/// 409 conflict: entry with id already exists
pub(super) async fn handle_insert_request(request: Request, db: SharedRepository) -> Response {
    // if no id or content empty (smaller than 1 byte)
    if request.content_length <= 5 {
        return Response {
            version: request.version,
            command: request.command,
            status_code: StatusCode::InvalidRequest,
            content_length: 0,
            content: None,
        };
    }

    let mut content = match request.content {
        Some(content) => content,
        None => return Response {
            version: request.version,
            command: request.command,
            status_code: StatusCode::InvalidRequest,
            content_length: 0,
            content: None,
        }
    };

    // since content_length is > 5 we don't need to check the length of content
    let id = u32::from_be_bytes([content[0], content[1], content[2], content[3]]);

    // remove the first 4 bytes from the content (id)
    for _ in 0..4 {
        content.remove(0);
    }

    match db.insert(id, content).await {
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
                DatabaseError::AlreadyExists(_) => Response {
                    version: request.version,
                    command: request.command,
                    status_code: StatusCode::Conflict,
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
    use crate::repository::error::DatabaseError;
    use crate::repository::{MockRepository};
    use crate::types::{Command, Request};

    fn make_request(id: u32, data: Vec<u8>) -> Request {
        let mut content = id.to_be_bytes().to_vec();
        content.extend_from_slice(&data);

        Request {
            version: 1,
            command: Command::Insert,
            content_length: content.len() as u16,
            content: Some(content),
        }
    }

    // ---- TESTS ----

    #[tokio::test]
    async fn invalid_request_content_length_smaller_5()  {
        let mut mock = MockRepository::new();

        mock.expect_insert().never();

        let mock = Arc::new(mock);

        let request = Request {
            version: 1,
            command: Command::Insert,
            content_length: 2,
            content: Some(vec![0,0]),
        };
        let response = handle_insert_request(request, mock).await;

        assert_eq!(StatusCode::InvalidRequest, response.status_code);
    }

    #[tokio::test]
    async fn invalid_request_content_missing()  {
        let mut mock = MockRepository::new();

        mock.expect_insert().never();

        let mock = Arc::new(mock);

        let request = Request {
            version: 1,
            command: Command::Insert,
            content_length: 6,
            content: None,
        };
        let response = handle_insert_request(request, mock).await;

        assert_eq!(StatusCode::InvalidRequest, response.status_code);
    }

    #[tokio::test]
    async fn entry_already_exists()  {
        let mut mock = MockRepository::new();

        mock.expect_insert()
            .with(mockall::predicate::eq(42u32), mockall::predicate::eq(b"hello".to_vec()))
            .times(1)
            .returning(|_, _| Err(DatabaseError::AlreadyExists(42)));

        let mock = Arc::new(mock);

        let request = make_request(42, b"hello".to_vec());
        let response = handle_insert_request(request, mock).await;

        assert_eq!(StatusCode::Conflict, response.status_code);
    }

    #[tokio::test]
    async fn unknown_db_error()  {
        let mut mock = MockRepository::new();

        mock.expect_insert()
            .with(mockall::predicate::eq(42u32), mockall::predicate::eq(b"hello".to_vec()))
            .times(1)
            .returning(|_, _| Err(DatabaseError::NotFound(42))); // Not Found should not be returned

        let mock = Arc::new(mock);

        let request = make_request(42, b"hello".to_vec());
        let response = handle_insert_request(request, mock).await;

        assert_eq!(StatusCode::InternalServerError, response.status_code);
    }

    #[tokio::test]
    async fn valid_request()  {
        let mut mock = MockRepository::new();

        mock.expect_insert()
            .with(mockall::predicate::eq(42u32), mockall::predicate::eq(b"hello".to_vec()))
            .times(1)
            .returning(|_, _| Ok(()));

        let mock = Arc::new(mock);

        let request = make_request(42, b"hello".to_vec());
        let response = handle_insert_request(request, mock).await;

        assert_eq!(response.status_code, StatusCode::Ok);
        assert_eq!(0, response.content_length);
    }
}
use crate::repository::SharedRepository;
use crate::types::{Request, Response, StatusCode};

/// GET REQUEST
///
/// Request Body:
/// 4 bytes u32 id to read
///
/// Responses:
/// 200 with content as body
/// 400 invalid request
/// 404 not found
pub(super) async fn handle_get_request(request: Request, db: SharedRepository) -> Response {
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

    // it is safe to assume, that all 4 bytes are here
    let id = u32::from_be_bytes([content[0], content[1], content[2], content[3]]);

    match db.get(id).await {
        Some(value) => {
            Response {
                version: request.version,
                command: request.command,
                status_code: StatusCode::Ok,
                content_length: value.len() as u16,
                content: Some(value),
            }
        }
        None => {
            Response {
                version: request.version,
                command: request.command,
                status_code: StatusCode::NotFound,
                content_length: 0,
                content: None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use mockall::mock;
    use crate::repository::error::DatabaseError;
    use crate::repository::RepositoryApi;
    use crate::types::{Command, Request};

    mock! {
        Repository {}

        #[async_trait::async_trait]
        impl RepositoryApi for Repository {
            async fn get(&self, id: u32) -> Option<Vec<u8>>;
            async fn set(&self, id: u32, data: Vec<u8>) -> Result<(), DatabaseError>;
            async fn insert(&self, id: u32, data: Vec<u8>) -> Result<(), DatabaseError>;
            async fn remove(&self, id: u32) -> Result<(), DatabaseError>;
        }
    }

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

        mock.expect_get().never();

        let mock = Arc::new(mock);

        let request = make_request(3, Some(vec![0, 0, 0]));
        let response = handle_get_request(request, mock).await;

        assert_eq!(response.status_code, StatusCode::InvalidRequest);
    }

    #[tokio::test]
    async fn invalid_request_when_content_missing() {
        let mut mock = MockRepository::new();

        mock.expect_get().never();

        let mock = Arc::new(mock);

        let request = make_request(4, None);
        let response = handle_get_request(request, mock).await;

        assert_eq!(response.status_code, StatusCode::InvalidRequest);
    }

    #[tokio::test]
    async fn not_found_when_entry_missing() {
        let mut mock = MockRepository::new();

        mock.expect_get()
            .with(mockall::predicate::eq(42u32))
            .times(1)
            .returning(|_| None);

        let mock = Arc::new(mock);

        let request = make_request(4, Some(42u32.to_be_bytes().to_vec()));
        let response = handle_get_request(request, mock).await;

        assert_eq!(response.status_code, StatusCode::NotFound);
    }

    #[tokio::test]
    async fn valid_request() {
        let mut mock = MockRepository::new();

        mock.expect_get()
            .with(mockall::predicate::eq(42u32))
            .times(1)
            .returning(|_| Some(b"hello".to_vec()));

        let mock = Arc::new(mock);

        let request = make_request(4, Some(vec![0,0,0,42]));
        let response = handle_get_request(request, mock).await;

        assert_eq!(response.status_code, StatusCode::Ok);
        assert_eq!(response.content_length, 5);
        assert_eq!(response.content, Some(b"hello".to_vec()));
    }
}
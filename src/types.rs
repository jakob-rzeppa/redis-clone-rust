/// Example Request Structure
///
/// u8 version
/// u8 command
/// u16 content length
/// content of specified length
pub(crate) struct Request<'a> {
    pub(crate) version: u8,
    pub(crate) command: Command,
    pub(crate) content_length: u16,
    pub(crate) content: &'a [u8],
}

/// Example Response Structure
///
/// u8 version
/// u8 command
/// u16 status code
/// u16 content length
/// content of specified length
pub(crate) struct Response<'a> {
    pub(crate) version: u8,
    pub(crate) command: Command,
    pub(crate) status_code: StatusCode,
    pub(crate) content_length: u16,
    pub(crate) content: &'a [u8],
}

// u8 in request / response
pub(crate) enum Command {
    Get = 0,
    Set = 1,
    Insert = 2,
    Remove = 3,
}

impl Into<u8> for Command {
    fn into(self) -> u8 {
        self as u8
    }
}

// u16 in request / response
pub(crate) enum StatusCode {
    Ok = 200,
    InvalidRequest = 400,
    NotFound = 404,
    InternalServerError = 500
}

impl Into<u16> for StatusCode {
    fn into(self) -> u16 {
        self as u16
    }
}
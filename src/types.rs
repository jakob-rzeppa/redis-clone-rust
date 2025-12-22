/// Example Request Structure
///
/// u8 version
/// u8 command
/// u16 content length
/// content of specified length
pub(crate) struct Request {
    pub(crate) version: u8,
    pub(crate) command: Command,
    pub(crate) content_length: u16,
    pub(crate) content: Option<Vec<u8>>,
}

/// Example Response Structure
///
/// u8 version
/// u8 command
/// u16 status code
/// u16 content length
/// content of specified length
pub(crate) struct Response {
    pub(crate) version: u8,
    pub(crate) command: Command,
    pub(crate) status_code: StatusCode,
    pub(crate) content_length: u16,
    pub(crate) content: Option<Vec<u8>>,
}

// u8 in request / response
#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum Command {
    Get = 0,
    Set = 1,
    Remove = 2,
    Invalid = 0xFF // the invalid command is given to the router if the command doesn't exist
}

impl From<u8> for Command {
    fn from(i: u8) -> Self {
        match i {
            0 => Command::Get,
            1 => Command::Set,
            2 => Command::Remove,
            _ => Command::Invalid,
        }
    }
}

// u16 in request / response
#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum StatusCode {
    Ok = 200,
    InvalidRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
    NotImplemented = 501,
    WriteBlocked = 520 // someone else is currently writing
}
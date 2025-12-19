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
    Insert = 2,
    Remove = 3,
}

impl Into<u8> for Command {
    fn into(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for Command {
    type Error = anyhow::Error;

    fn try_from(i: u8) -> Result<Self, Self::Error> {
        match i {
            0 => Ok(Command::Get),
            1 => Ok(Command::Set),
            2 => Ok(Command::Insert),
            3 => Ok(Command::Remove),
            _ => Err(anyhow::anyhow!("invalid command")),
        }
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
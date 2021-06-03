use rocket::response::*;
use rocket::Request;

#[derive(Debug)]
pub enum BackendError {
    CalamineError(calamine::Error),
    DeserializeError(calamine::DeError),
    SpreadsheetError(calamine::XlsxError),
    DatabaseError(diesel::result::Error),
}

impl From<calamine::Error> for BackendError {
    fn from(error: calamine::Error) -> Self {
        BackendError::CalamineError(error)
    }
}

impl From<calamine::DeError> for BackendError {
    fn from(error: calamine::DeError) -> Self {
        BackendError::DeserializeError(error)
    }
}

impl From<calamine::XlsxError> for BackendError {
    fn from(error: calamine::XlsxError) -> Self {
        BackendError::SpreadsheetError(error)
    }
}

impl From<diesel::result::Error> for BackendError {
    fn from(error: diesel::result::Error) -> Self {
        BackendError::DatabaseError(error)
    }
}

impl<'a> Responder<'a> for BackendError {
    fn respond_to(self, _request: &Request) -> Result<'a> {
        use rocket::http::Status;

        let status = match self {
            //TODO: implement for each error type
            _ => Status::InternalServerError,
        };

        Response::build().status(status).ok()
    }
}

use http::StatusCode;

use serde::Serialize;

use crate::Response;

#[derive(Debug, Clone, Serialize)]
pub struct HTTPException {
    pub detail: String,

    pub status_code: u16,
}

impl HTTPException {
    pub fn new(status_code: u16, detail: impl Into<String>) -> Self {
        Self { status_code, detail: detail.into() }
    }

    pub fn response(self) -> Response {
        let status =
            StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        Response::json_with_status(status, self)
    }
}

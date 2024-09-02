use std::fmt::Display;

use axum::{body::Body, response::{IntoResponse, Response}};
use http::StatusCode;
use serde::{ser::SerializeStruct, Serialize, Serializer};


pub struct AppError
{
    pub err_msg: String,
    pub status_code: StatusCode,
}

// TODO : This makes sense?
unsafe impl Send for AppError {}
unsafe impl Sync for AppError {}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("AppError", 2)?;
        state.serialize_field("err_msg", &self.err_msg)?;
        state.serialize_field("status_code", &self.status_code.as_u16())?;
        state.end()
    }
}

impl Display for AppError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         write!(f, "error: {}, code: {}", self.err_msg, self.status_code)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {

        let json_data = serde_json::to_string(&self).unwrap();

        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(Body::from(json_data))
            .unwrap()
    }
}


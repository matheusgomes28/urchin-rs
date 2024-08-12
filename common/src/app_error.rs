use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde::{ser::SerializeStruct, Serialize, Serializer};


pub struct AppError
{
    pub err_msg: String,
    pub status_code: StatusCode,
}


impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status_code, format!("Something went wrong: {}", self.err_msg))
            .into_response()
    }
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("AppError", 3)?;
        state.serialize_field("err_msg", &self.err_msg)?;
        state.serialize_field("status_code", &self.status_code.as_u16())?;
        state.end()
    }
}

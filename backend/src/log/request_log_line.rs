use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::json;
use serde_with::skip_serializing_none;
use tracing::debug;
use uuid::Uuid;

use crate::error::{Result, Error};

pub fn log_request(
    uuid: Uuid,
    method: Method,
    uri: Uri,
    error: Option<&Error>,
) -> Result<()> {

    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        method: method.to_string(),
        uri: uri.to_string(),
        error: error.map(|e| e.to_string()),
    };

    debug!("REQUEST LOG LINE: {}", json!(&log_line));

    Ok(())
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,
    method: String,
    uri: String,
    error: Option<String>
}
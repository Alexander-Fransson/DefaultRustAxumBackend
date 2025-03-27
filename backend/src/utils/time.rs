use time::{
    format_description::well_known::Rfc3339, 
    OffsetDateTime, 
    Duration
};
use super::{Result, Error};

pub fn now_utc_plus_sec_str(sec: f64) -> Result<String> {
    let now = OffsetDateTime::now_utc();
    let future = now + Duration::seconds_f64(sec);
    future.format(&Rfc3339)
    .map_err(|e| Error::FailedToFormtOffsetDateTime(e.to_string()))
}
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

pub fn time_str_to_offset_date_time(time_str: &str) -> Result<OffsetDateTime> {
    let parsed_time = OffsetDateTime::parse(time_str, &Rfc3339)
    .map_err(|e| Error::FailedToParseOffsetDateTime(e.to_string()))?;

    Ok(parsed_time)
}
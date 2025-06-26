use std::error::Error;

use time::OffsetDateTime;
#[cfg(feature = "tzdb")]
use time::UtcOffset;
use time::format_description::well_known::Rfc2822;

pub(crate) const DEFINE_SOURCE_DATE_EPOCH: &str = "SOURCE_DATE_EPOCH";
pub enum DateTime {
    Local(OffsetDateTime),
    Utc(OffsetDateTime),
}

impl DateTime {
    #[cfg(not(feature = "tzdb"))]
    pub fn local_now() -> Result<Self, Box<dyn Error>> {
        // Warning: This attempts to create a new OffsetDateTime with the current date and time in the local offset, which may fail.
        // Currently, it always fails on MacOS.
        // This issue does not exist with the "tzdb" feature (see below), which should be used instead.
        OffsetDateTime::now_local()
            .map(DateTime::Local)
            .map_err(|e| e.into())
    }

    #[cfg(feature = "tzdb")]
    pub fn local_now() -> Result<Self, Box<dyn Error>> {
        let local_time = tzdb::now::local()?;
        let time_zone_offset =
            UtcOffset::from_whole_seconds(local_time.local_time_type().ut_offset())?;
        let local_date_time = OffsetDateTime::from_unix_timestamp(local_time.unix_time())?
            .to_offset(time_zone_offset);
        Ok(DateTime::Local(local_date_time))
    }

    pub fn now() -> Self {
        Self::local_now().unwrap_or_else(|_| DateTime::Utc(OffsetDateTime::now_utc()))
    }

    pub fn to_rfc2822(&self) -> String {
        match self {
            DateTime::Local(dt) | DateTime::Utc(dt) => dt.format(&Rfc2822).unwrap_or_default(),
        }
    }
}

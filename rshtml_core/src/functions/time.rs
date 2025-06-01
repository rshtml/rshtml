use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;

pub fn time<T: Display + ?Sized>(value: &T) -> RsDateTime {
    let value_str = value.to_string();
    let default_format = "%Y-%m-%d %H:%M:%S".to_string();

    match DateTime::parse_from_rfc3339(&value_str) {
        Ok(dt) => return RsDateTime(dt.with_timezone(&Utc), default_format),
        Err(err) => eprintln!("DEBUG: Time error: {}", err),
    };

    match NaiveDateTime::from_str(&value_str) {
        Ok(ndt) => return RsDateTime(DateTime::<Utc>::from_naive_utc_and_offset(ndt, Utc), default_format),
        Err(err) => eprintln!("DEBUG: Time error: {}", err),
    };

    match NaiveDate::from_str(&value_str) {
        Ok(nd) => {
            let naive_datetime = nd.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
            return RsDateTime(DateTime::<Utc>::from_naive_utc_and_offset(naive_datetime, Utc), default_format);
        }
        Err(err) => eprintln!("DEBUG: Time error: {}", err),
    };

    RsDateTime(DateTime::default(), default_format)
}

pub struct RsDateTime(DateTime<Utc>, String);

impl RsDateTime {
    pub fn date_time(&self) -> DateTime<Utc> {
        self.0
    }

    pub fn date(&self) -> NaiveDate {
        self.0.date_naive()
    }

    pub fn pretty(&mut self) -> &Self {
        self.1 = "%b %d, %Y".to_string();
        self
    }
}

impl Display for RsDateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format(self.1.as_str()))
    }
}

impl Deref for RsDateTime {
    type Target = DateTime<Utc>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

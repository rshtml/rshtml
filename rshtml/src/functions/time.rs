use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

pub fn time<T: Display + ?Sized>(value: &T) -> RsDateTime {
    let value_str = value.to_string();
    let value_str = value_str.trim();
    let default_format = "%Y-%m-%d %H:%M:%S".to_string();
    let mut err_str = String::new();

    let dt_result: Result<DateTime<Utc>, _> = value_str.parse();
    match dt_result {
        Ok(dt) => return RsDateTime(dt.with_timezone(&Utc), default_format),
        Err(err) => err_str.push_str(&format!("{err}")),
    }

    let ndt_result: Result<NaiveDateTime, _> = value_str.parse();
    if let Ok(ndt) = ndt_result {
        return RsDateTime(
            DateTime::<Utc>::from_naive_utc_and_offset(ndt, Utc),
            default_format,
        );
    }

    let nd_result: Result<NaiveDate, _> = value_str.parse();
    if let Ok(nd) = nd_result {
        let naive_datetime = nd.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        return RsDateTime(
            DateTime::<Utc>::from_naive_utc_and_offset(naive_datetime, Utc),
            default_format,
        );
    }

    eprintln!("DEBUG: Time Error: {err_str}");

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

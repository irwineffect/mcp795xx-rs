
#[derive(Debug,Clone,Copy)]
pub struct DateTime {
    pub seconds: u8,
    pub minutes: u8,
    pub hours: u8,

    pub weekday: u8,
    pub date: u8,
    pub month: u8,
    pub year: u16,
}

#[cfg(feature = "chrono")]
use chrono::{Datelike, Timelike};

#[cfg(feature = "chrono")]
impl From<chrono::naive::NaiveDateTime> for DateTime {
    fn from(datetime: chrono::naive::NaiveDateTime) -> Self {
        Self {
            year: datetime.year() as u16,
            month: datetime.month() as u8,
            date: datetime.day() as u8,
            weekday: datetime.weekday().number_from_sunday() as u8,
            hours: datetime.hour() as u8,
            minutes: datetime.minute() as u8,
            seconds: datetime.second() as u8,
        }
    }
}

#[cfg(feature = "chrono")]
impl From<DateTime> for chrono::naive::NaiveDateTime {
    fn from(datetime: DateTime) -> Self {
        chrono::naive::NaiveDate::from_ymd(
            datetime.year as i32,
            datetime.month as u32,
            datetime.date as u32
            ).and_hms(
            datetime.hours as u32,
            datetime.minutes as u32,
            datetime.seconds as u32
            )
    }
}

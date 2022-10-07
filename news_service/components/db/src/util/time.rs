use time::Duration;

use super::convert::datetime_from_unix_timestamp;
use super::convert::now_utc_datetime;

pub fn datetime_minutes_diff(date_timestamp: i64) -> i64 {
    let date = datetime_from_unix_timestamp(date_timestamp);
    let diff: Duration = now_utc_datetime() - date;
    diff.whole_minutes()
}            
use db::util::convert::now_utc_datetime;
use time::ext::NumericalDuration;
use time::OffsetDateTime;

pub fn past_365_days() -> OffsetDateTime {
    //TODO now_utc_timestamp().checked_add((-365).days()).unwrap()
    now_utc_timestamp().checked_add((-7).days()).unwrap()
}

pub fn past_7_days() -> OffsetDateTime {
    now_utc_timestamp().checked_add((-7).days()).unwrap()
}

fn now_utc_timestamp() -> OffsetDateTime {
    let now_utc_timestamp = now_utc_datetime();
    // Remove nanoseconds to avoid Twitter API error
    now_utc_timestamp
        .checked_add(-time::Duration::nanoseconds(
            now_utc_timestamp.nanosecond() as i64
        ))
        .unwrap()
}

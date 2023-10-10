use time::PrimitiveDateTime as DateTime;
use time::Duration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let gigaseconds = Duration::seconds(1_000_000_000);
    start + gigaseconds
}

fn main() {
    println!("helloworlds");
}

/// Create a datetime from the given numeric point in time.
///
/// Panics if any field is invalid.
fn dt(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> DateTime {
    use time::{Date, Time};
    DateTime::new(
        Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
        Time::from_hms(hour, minute, second).unwrap(),
    )
}
#[test]
fn date() {
    let start_date = dt(2011, 4, 25, 0, 0, 0);
    assert_eq!(after(start_date), dt(2043, 1, 1, 1, 46, 40));
}

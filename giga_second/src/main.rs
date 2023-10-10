use time::PrimitiveDateTime as DateTime;
use time::Duration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let gigaseconds = Duration::seconds(1_000_000_000);
    start + gigaseconds
}

fn main() {
    let first_giga = after();
}

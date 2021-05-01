use chrono::NaiveTime;
use chrono::Duration;

fn main() {
    let time1 = NaiveTime::parse_from_str("11:58:10 PM", "%I:%M:%S %p").unwrap();
    let time2 = NaiveTime::parse_from_str("12:02:15 AM", "%I:%M:%S %p").unwrap();
    let mut duration = time2 - time1;
    if duration < Duration::zero() {
        duration = duration + Duration::days(1);
    }
    println!("{} mins {} secs.", duration.num_minutes(), duration.num_seconds() % 60);
}

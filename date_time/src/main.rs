use chrono::offset;
use chrono::{DateTime, FixedOffset, Local, SecondsFormat, Utc};

fn main() {
    _basic();
    // geral();
}

fn _basic() {
    let fixed_zero: DateTime<offset::FixedOffset> =
        DateTime::parse_from_rfc3339("0000-01-10T00:00:00-00:00").unwrap();
    let fixed_sp = fixed_zero.with_timezone(&FixedOffset::west(3 * 3600));

    let utc_now = Utc::now();
    let fixed_now = utc_now.with_timezone(&FixedOffset::west(3 * 3600));

    // let t1: DateTime<Local> = Local.datetime_from_str("0000-01-01T00:00:00-00:00");
    println!("fixed_zero: {}", fixed_zero.to_rfc3339());
    println!("fixed_sp:   {}", fixed_sp.to_rfc3339());
    println!("utc_now:    {}", utc_now.to_rfc3339());
    println!("fixed_now:  {}", fixed_now.to_rfc3339());
    println!(
        "fixed_now:  {}",
        fixed_now.to_rfc3339_opts(SecondsFormat::Secs, true)
    );
    println!(
        "fixed_now:  {}",
        fixed_now.to_rfc3339_opts(SecondsFormat::Secs, false)
    );
}

fn _geral() {
    // let t1 = chrono::Local::now();
    let fixed_zero: DateTime<offset::FixedOffset> =
        DateTime::parse_from_rfc3339("0000-01-10T00:00:00-03:00").unwrap();
    let utc: DateTime<offset::Utc> = fixed_zero.with_timezone(&Utc);
    let now = Utc::now();
    let fixed_from_now = now.with_timezone(&FixedOffset::west(3 * 3600));
    let local = Local::now();
    let local_from_fixed = fixed_zero.with_timezone(&Local);
    let test = fixed_zero.with_timezone(&FixedOffset::west(3 * 3600));
    // let t1: DateTime<Local> = Local.datetime_from_str("0000-01-01T00:00:00-00:00");
    println!("fixed_zero: {}", fixed_zero.to_rfc3339());
    println!("utc: {}", utc.to_rfc3339());
    println!("now: {}", now.to_rfc3339());
    println!("local: {}", local.to_rfc3339());
    println!("local_from_fixed: {}", local_from_fixed.to_rfc3339());
    println!("test: {}", test.to_rfc3339());
    println!("fixed_from_now: {}", fixed_from_now.to_rfc3339());
}

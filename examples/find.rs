use chrono::{Local, TimeZone};
use rust_erajp;

fn main() {
    let date_time = Local.with_ymd_and_hms(1984, 1, 1, 0, 0, 0).unwrap();
    println!("{}", rust_erajp::find(date_time).unwrap().name);
}

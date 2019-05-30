use std::io::{self, BufRead};
use chrono::{NaiveDate, Local};

fn main() {
    let mut last_date = "".to_string();

    for line in io::stdin().lock().lines() {
        let l = line.unwrap();
        if l.starts_with("## ") {
            last_date = l.chars().skip(3).take(10).collect();
        }
        println!("{}", l)
    }

    let d = NaiveDate::parse_from_str(&last_date, "%Y-%m-%d").expect("Not a date");
    let today = Local::today().naive_local();
    if d != today {
        println!("\n{}", today.format("## %Y-%m-%d").to_string());
    }
}

use std::collections::HashSet;
use std::io::{self, BufRead};
use chrono::{NaiveDate, Local};


fn main() {
    let mut last_date = String::from("");
    let mut items: HashSet<String> = HashSet::new();

    for line in io::stdin().lock().lines() {
        let l = line.unwrap();
        println!("{}", l);

        match l.chars().next() {
            Some('#') => {
                last_date = l.chars().skip(3).take(10).collect();
            },
            Some('*') | Some('>') => {
                items.insert(l.chars().skip(2).collect());
            },
            Some('x') | Some('-') => {
                let item: String = l.chars().skip(2).collect();
                items.remove(&item);
            },
            _ => (),
        }
    }

    let d = NaiveDate::parse_from_str(&last_date, "%Y-%m-%d").expect("Not a date");
    let today = Local::today().naive_local();
    if d != today {
        println!("\n## {}", today.format("%Y-%m-%d").to_string());
        for item in items {
            println!("> {}", item);
        }
    }
}

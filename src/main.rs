use std::collections::HashSet;
use std::io::{self, BufRead};
use chrono::{NaiveDate, Local};


fn main() {
    let mut last_date = String::from("");
    let mut items: HashSet<String> = HashSet::new();

    for line in io::stdin().lock().lines() {
        let l = line.unwrap();
        println!("{}", l);

        let mut chars = l.chars();
        match chars.next() {
            Some('#') => {
                last_date = chars.skip(2).take(10).collect();
            },
            Some('*') | Some('>') => {
                items.insert(chars.skip(1).collect());
            },
            Some('x') | Some('-') => {
                let item: String = chars.skip(1).collect();
                items.remove(&item);
            },
            _ => (),
        }
    }

    let d = NaiveDate::parse_from_str(&last_date, "%Y-%m-%d").expect("Not a date");
    let now = Local::now().naive_local();
    if d != now.into() {
        println!("\n## {}", now.format("%Y-%m-%d").to_string());
        for item in items {
            println!("> {}", item);
        }
    }
}

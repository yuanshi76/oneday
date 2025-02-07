extern crate arboard;
extern crate chrono;
extern crate tokio;
extern crate systray;
extern crate sled;
extern crate pulldown_cmark;

use arboard::Clipboard;
use chrono::{Local, DateTime};
use tokio::time::{self, Duration};
use systray::{Systray, MenuItem};
use sled::Db;
use pulldown_cmark::{Parser, Options, Event};

#[tokio::main]
async fn main() {
    let mut clipboard = Clipboard::new().unwrap();
    let db = Db::open("clipboard_data").unwrap();

    let mut items = vec![];

    loop {
        let text = clipboard.get_text().unwrap();
        if !items.contains(&text) {
            items.push(text.clone());
            db.insert(text.clone(), b"").unwrap();
        }

        let now: DateTime<Local> = Local::now();
        if now.hour() == 0 && now.minute() == 0 && now.second() == 0 {
            summarize_and_store(&db);
        }

        time::sleep(Duration::from_secs(1)).await;
    }
}

fn summarize_and_store(db: &Db) {
    let mut summary = String::new();
    for item in db.iter() {
        let (_, _) = item.unwrap();
        summary += "TODO: Summarize item\n";
    }

    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("daily_summary.md")
        .unwrap();
    file.write_all(summary.as_bytes()).unwrap();
}

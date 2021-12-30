extern crate chrono;

use chrono::{TimeZone, Weekday, ParseResult};
use chrono::prelude::{DateTime, Utc, Local, Datelike, Timelike};
use chrono::offset::FixedOffset;

// UTC で現在の日付、時刻を取得する。
//let utc: DateTime<Utc> = Utc::now();
fn test () {
    let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = Local::now();
    println!("UTC:{} LOCAL:{}", utc, local);
}

enum TicketStatus {
    NEW,
    ONGOING,
    DONE,
    FEEDBACK,
    COMPLETED
}

enum TicketPriority {
    LOW,
    HIGH,
    NOW,
    ASAP
}
pub struct Ticket {
    id: i32,
    title: Option<String>,
    detail: Option<String>,
    status: Option<TicketStatus>,
    priority: Option<TicketPriority>,
    deadline: Option<DateTime<Local>>,
    staff: Option<String>,
}

impl Ticket {
    fn new() -> Self {
        Self {
            id: 0,
            title: None,
            detail: None,
            status: None,
            priority: None,
            deadline: None,
            staff: None,
        }
    }
}
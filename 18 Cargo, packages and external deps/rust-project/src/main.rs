use chrono::{Local, Utc};

fn main() {
    // Get the current date and time in UTC
    let now = Utc::now();
    println!("Current date and time in UTC: {}", now);

    // Format the date and time
    let formatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("Formatted date and time: {}", formatted);

    // Get local time
    let local = Local::now();
    println!("Current date and time in local: {}", local);
}

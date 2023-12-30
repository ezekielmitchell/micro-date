#![allow(unused)]

use std::{io, thread::sleep};
use chrono::{Local, DateTime};
use std::time::Duration;


fn main() {
    loop { 
        let current: DateTime<Local> = Local::now();
        println!("{}", current);
        sleep(Duration::new(3, 0));
    }
}

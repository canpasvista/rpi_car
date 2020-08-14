use tokio::time;
use chrono::{Utc, Local, DateTime, Date};

async fn task_that_takes_a_second() {

    //println!("hello");
    println!("{}", Utc::now());
    time::delay_for(time::Duration::from_millis(10)).await;
    println!("{}", Utc::now());
    //println!("---")
}

#[tokio::main]
async fn main() {

    let mut interval = time::interval(time::Duration::from_millis(100));
    for _i in 0..5 {
        interval.tick().await;
        //println!("{}", Utc::now());
        task_that_takes_a_second().await;
    }
}
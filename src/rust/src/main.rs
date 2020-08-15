//use chrono::{Date, DateTime, Local};
use chrono::Utc;
use std::error::Error;
use tokio::time;
//use rppal::gpio::Gpio;
use libc::c_uint;
use std::convert::TryInto;
mod udp;

use json_flex;
use json_flex::JFObject;

use std::net::UdpSocket;

//const GPIO_L: u8 = 14;
//const GPIO_R: u8 = 15;

/*
async fn task_that_takes_a_second() {
    //println!("hello");
    println!("{}", Utc::now());
    time::delay_for(time::Duration::from_millis(10)).await;
    println!("{}", Utc::now());
    //println!("---")
}
*/

fn l_motor(speed: c_uint) -> Result<(), Box<dyn Error>> {
    /*let mut l_pin = Gpio::new()?.get(GPIO_L)?.into_output();
    l_pin.set_high();
    unsafe { libc::usleep(speed) };
    l_pin.set_low();
    Ok(())*/
    Ok(())
}
fn r_motor(speed: c_uint) -> Result<(), Box<dyn Error>> {
    /*let mut pin = Gpio::new()?.get(GPIO_R)?.into_output();
    pin.set_high();
    unsafe{ libc::usleep(speed) };
    pin.set_low();
    Ok(())*/
    Ok(())
}

async fn setSpeed(l_speed: i64, r_speed: i64) -> Result<(), Box<dyn Error>> {
    /*let mut interval = time::interval(time::Duration::from_millis(20));
    let mut interval2 = time::interval(time::Duration::from_millis(20));
    let mut n : i64 = l_speed;
    let mut n2 : i64 = 0;
    for _i in 0..10 {
        interval.tick().await;
    let r : u32 = (1350+l_speed-n2).try_into().unwrap();
    r_motor(r);
        interval2.tick().await;
        let l : u32 = (1450-r_speed+n2).try_into().unwrap();
        l_motor(l);
    println!(" : {}   {}",l_speed,r_speed);
    };
    Ok(())
    */
    Ok(())
}

#[tokio::main]
async fn main() {
    let socket = UdpSocket::bind("0.0.0.0:5000").expect("Could not bind socket");
    socket.set_nonblocking(true).unwrap();
    let mut l = 0;
    let mut r = 0;
    loop {
        let array = udp::start(&socket);
        match array {
            Option::None => {
                if (l == 0 && r == 0) {
                    setSpeed(l.try_into().unwrap(), r.try_into().unwrap()).await;
                }
            }
            Option::Some(array) => {
                if *(array["stop"].unwrap_i64()) == 1 {
                    println!("stop");
                    setSpeed(0, 0).await;
                } else {
                    println!(
                        "{} - {} ",
                        *(array["R"].unwrap_i64()),
                        *(array["L"].unwrap_i64())
                    );
                    l = *(array["L"].unwrap_i64());
                    r = *(array["R"].unwrap_i64());

                    setSpeed(l.try_into().unwrap(), r.try_into().unwrap()).await;
                }
            } //setSpeed(400,400).await;
        }
    }
}

use std::{thread::sleep, time};

fn main() {
    println!("hello world");
    loop {
        sleep(time::Duration::from_secs(10));
        println!("step");
    }
}

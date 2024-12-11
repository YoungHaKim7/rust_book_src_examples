use std::{thread, time::Duration};

fn main() {
    // let y = 3;
    // let while_a = false;
    println!("로켓트 카운드 타운 시작: ");

    let mut while_x = 10;
    while while_x != 1 {
        // You can add your logic here
        while_x -= 1; // Decrement the variable to exit the loop
                      // let mut while_x = 10; // Initialize a variable to control the while loop
                      // println!("Inside the while loop");
        thread::sleep(Duration::from_secs(1));
        println!("{} 초", while_x);
    }
    thread::sleep(Duration::from_secs(1));
    println!("0 초 ~~~ 발사 ~~~~")
}

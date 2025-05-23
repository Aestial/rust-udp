use std::net::UdpSocket;
use std::time::{Duration, Instant};
use rand::Rng;

fn main() -> std::io::Result<()> {
    // Bind to a socket
    let socket = UdpSocket::bind("0.0.0.0:34254")?;
    println!("UDP server running on port 34254");
    
    // Set the destination address (change this to your Python client's address)
    let dest_addr = "192.168.0.204:9999";
    
    // Create a random number generator
    let mut rng = rand::thread_rng();
    
    // Calculate the interval for 100Hz (10ms between sends)
    let interval = Duration::from_millis(10);
    let mut next_time = Instant::now() + interval;
    
    loop {
        // Generate random number 0-10
        let num: u8 = rng.gen_range(0..=10);
        
        // Send the number
        socket.send_to(&[num], dest_addr)?;
        
        // Sleep until the next send time
        let now = Instant::now();
        if now < next_time {
            std::thread::sleep(next_time - now);
        }
        next_time += interval;
    }
}

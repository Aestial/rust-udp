use std::net::UdpSocket;
use std::time::SystemTime;

fn main() -> std::io::Result<()> {
    // Bind to the port we want to receive on
    let socket = UdpSocket::bind("0.0.0.0:34255")?;
    println!("UDP receiver running on port 34255");
    
    let mut buf = [0; 1]; // Buffer for 1 byte (u8)
    
    loop {
        // Receive data
        let (amt, _src) = socket.recv_from(&mut buf)?;
        
        if amt == 1 {
            let num = buf[0];
            let timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64();
            
            println!("{:.6}: Received number: {}", timestamp, num);
        }
    }
}
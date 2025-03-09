use std::io::prelude::*;
use std::io::Result;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

pub fn sock_connect(ip: &str, port: &str) -> Result<()> {
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port).as_str())?;
    let mut count = 0;
    loop {
        if count == 100 {
            break;
        }
        let msg = String::from("[FROM CLIENT1] Spamminnnnnng\n");
        stream.write(msg.as_bytes())?;
        let mut buffer = [0; 128];
        let raw = stream.read(&mut buffer)?;
        let ss = String::from_utf8_lossy(&buffer[..raw]);
        println!("Received: {:?}\n", ss);
        count += 1;
        thread::sleep(Duration::from_millis(500));
    }
    Ok(())
}

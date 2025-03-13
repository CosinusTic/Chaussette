use std::io::prelude::*;
use std::io::Result;
use std::net::TcpStream;

pub fn connect(ip: &str, port: &str) -> Result<TcpStream> {
    let stream = TcpStream::connect(format!("{}:{}", ip, port));

    Ok(stream?)
}

pub fn write(stream: &mut TcpStream, msg: &str) -> std::io::Result<usize> {
    stream.write(msg.as_bytes())
}

pub fn read(stream: &mut TcpStream, buf: &mut [u8; 128]) -> String {
    let raw = stream.read(buf).unwrap();
    String::from_utf8_lossy(&buf[..raw]).to_string()
}
/*
pub fn connectt(ip: &str, port: &str) -> Result<()> {
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
*/

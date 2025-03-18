use client::net;
use proto::proto;
use std::{
    io::{Read, Write},
    thread,
    time::Duration,
};

fn main() -> std::io::Result<()> {
    let ip = "127.0.0.1";
    let port = "7878";

    let mut buffer = [0; 1024];
    if let Ok(mut stream) = net::connect(ip, port) {
        stream.read(&mut buffer)?;
        if let Some(msg) = proto::Message::decode(&buffer) {
            println!("Recieved: {:?}", msg);
        }
    }

    Ok(())
}

/*
fn main() -> std::io::Result<()> {
    let ip = "127.0.0.1";
    let port = "7878";
    let mut name_buff = String::new();

    println!("Enter your name: \n");
    std::io::stdin().read_line(&mut name_buff)?;
    let name = name_buff.trim_end();

    if let Ok(mut stream) = net::connect(ip, port) {
        stream.write(String::from(format!("[{}] connected\n", name)).as_bytes())?;
        println!("Connection to {}:{} established", ip, port);
        let mut count = 0;
        loop {
            if count == 100 {
                break;
            }
            let msg = String::from(format!("[{}] Spamminnnnnng\n", name));
            net::write(&mut stream, msg.as_str())?;
            let mut buffer = [0; 128];
            let s: String = net::read(&mut stream, &mut buffer);
            println!("Recieved: {}", s);
            count += 1;
            thread::sleep(Duration::from_millis(500));
        }
    } else {
        eprintln!("[Warning] {} unreachable on port {}.", ip, port);
    }

    Ok(())
}
*/

use std::io::{BufRead, BufReader, Result, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> Result<()> {
    let reader = BufReader::new(stream.try_clone()?);
    let mut n = 0;
    for line in reader.lines() {
        let l = line?;
        println!("[SERVER] Received: {}\n", l);
        let s = format!("[FROM SERVER] Message No. {}\n", n);
        stream.write_all(s.as_bytes())?;
        stream.flush()?;
        n += 1;
    }
    Ok(())
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on 127.0.0.1:7878...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Error handling client {}", e);
                    }
                });
            }
            Err(e) => eprintln!("Failed to accent client {}", e),
        }
    }

    Ok(())
}

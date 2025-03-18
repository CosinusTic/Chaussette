use proto::proto;
use server::api;
use server::api::map_poke;
use server::api::models::Pokemon;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use threadpool::ThreadPool;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let msg = proto::Message::Text(String::from("hello"));
    stream.write_all(&msg.encode())?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let name = "Pikachu";
    let json = api::get_poke(name).await?;
    let p: Pokemon = api::map_poke(&json)?;
    api::dl_poke_png(&p).await?;
    println!(
        "Pokemon name: {}, id: {}, weight: {}, png: {}",
        p.name, p.id, p.weight, p.sprites.front_default
    );

    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let pool = ThreadPool::new(4);
    let listener = Arc::new(listener);

    println!("Server listening on 127.0.0.1:7878...");

    for stream in listener.incoming() {
        println!("Incoming request");
        match stream {
            Ok(stream) => {
                let stream = Arc::new(stream);
                pool.execute(move || {
                    if let Err(e) = handle_client((*stream).try_clone().unwrap()) {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("Failed to accept client: {}", e),
        }
    }

    Ok(())
}

/*
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
    let pool = ThreadPool::new(4);
    let listener = Arc::new(listener);

    println!("Server listening on 127.0.0.1:7878...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream = Arc::new(stream);
                pool.execute(move || {
                    if let Err(e) = handle_client((*stream).try_clone().unwrap()) {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("Failed to accept client: {}", e),
        }
    }

    Ok(())
}
*/

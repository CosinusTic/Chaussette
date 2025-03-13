use client::net;
use std::{io::Write, thread, time::Duration};

fn main() {
    let ip = "127.0.0.1";
    let port = "7878";
    let client_name = "Nathan";
    if let Ok(mut stream) = net::connect(ip, port) {
        stream
            .write(String::from(format!("[{}] connected\n", client_name)).as_bytes())
            .unwrap();
        println!("Connection to {}:{} established", ip, port);
        let mut count = 0;
        loop {
            if count == 100 {
                break;
            }
            let msg = String::from(format!("[{}] Spamminnnnnng\n", client_name));
            net::write(&mut stream, msg.as_str()).unwrap();
            let mut buffer = [0; 128];
            let s: String = net::read(&mut stream, &mut buffer);
            println!("Recieved: {}", s);
            count += 1;
            thread::sleep(Duration::from_millis(500));
        }
    } else {
        eprintln!("[Warning] {} unreachable on port {}.", ip, port);
    }
}
/*
use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
*/

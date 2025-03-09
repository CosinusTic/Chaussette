use client::net::sock_connect;
use std::io::Result;

fn main() -> Result<()> {
    let ip = "127.0.0.1";
    let port = "7878";
    sock_connect(ip, port)
}

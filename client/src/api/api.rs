use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::api::models::Pokemon;
use http::{Request, Response, Uri};
use reqwest;

static BASE_URL: &str = "https://pokeapi.co/api/v2";

pub fn get_pokemon(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path: String = format!("{}/pokemon", BASE_URL);
    let mut stream = TcpStream::connect(format!("{}/pokemon/{}", BASE_URL, name))?;
    let req = format!(
        "GET {} HTTP/1.1\r\nHost: pokeapi.co\r\nConnection: close\r\n\r\n",
        path.as_str()
    );

    stream.write_all(req.as_bytes())?;
    stream.flush()?;

    let mut response = String::new();
    let resp = stream.read_to_string(&mut response)?;

    println!("Obtained response: {}", resp);
    Ok(())
}

pub async fn get_poke(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path: String = format!("{}/pokemon/{}", BASE_URL, name);
    let body: String = reqwest::get(path.as_str()).await?.text().await?;

    println!("body: {}", body);

    Ok(body)
}

pub fn map_poke(json: &str) -> Result<Pokemon, Box<dyn std::error::Error>> {
    let p: Pokemon = serde_json::from_str(json)?;
    println!("Pokemon name: {}, id: {}", p.name, p.id);
    Ok(p)
}

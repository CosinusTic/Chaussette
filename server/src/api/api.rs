use crate::api::models::Pokemon;
use reqwest;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

lazy_static::lazy_static! {
    static ref BASE_DLPATH: PathBuf = {
        let path = PathBuf::from("server/src/img");
        fs::create_dir_all(&path).expect("Failed to create img directory");
        path
    };
}
static BASE_URL: &'static str = "https://pokeapi.co/api/v2";
pub async fn get_poke(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path: String = format!("{}/pokemon/{}", BASE_URL, name);
    let body: String = reqwest::get(path.as_str()).await?.text().await?;

    Ok(body)
}

pub fn map_poke(json: &str) -> Result<Pokemon, Box<dyn std::error::Error>> {
    let p: Pokemon = serde_json::from_str(json)?;
    Ok(p)
}

pub async fn dl_poke_png(p: &Pokemon) -> Result<(), Box<dyn std::error::Error>> {
    let r = reqwest::get(p.sprites.front_default.as_str()).await?;
    let bytes = r.bytes().await?;
    let file_path = BASE_DLPATH.join(format!("{}.png", p.name));
    let mut f = File::create(&file_path)?;
    f.write_all(&bytes)?;
    println!("Saved: {}", file_path.to_str().unwrap());

    Ok(())
}

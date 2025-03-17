use crate::api::models::Pokemon;
use reqwest;
use std::fs::File;

static BASE_URL: &'static str = "https://pokeapi.co/api/v2";
static BASE_DLPATH: &'static str = "src/img/";

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
    let mut f: File = File::create(format!("{}/{}.png", BASE_DLPATH, p.name.as_str()))?;
    let r = reqwest::get(p.sprites.front_default.as_str()).await?;
    let bytes = r.bytes().await?;

    std::io::copy(&mut bytes.as_ref(), &mut f)?;
    println!("Saved: {}{}.png", BASE_DLPATH, p.name.as_str());

    Ok(())
}

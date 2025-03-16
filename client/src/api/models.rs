use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Sprites {
    pub back_default: String,
    pub front_default: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pokemon {
    pub name: String,
    pub id: u32,
    pub sprites: Sprites,
    pub weight: u8,
}

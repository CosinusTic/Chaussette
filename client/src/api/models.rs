use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pokemon {
    pub name: String,
    pub id: u32,
}

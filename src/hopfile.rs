use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ModEntry {}

#[derive(Debug, Deserialize)]
pub struct Hopfile {
    pub version: String,
    pub mods: HashMap<String, ModEntry>,
}

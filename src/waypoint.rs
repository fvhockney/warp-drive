use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Waypoint {
    pub name: String,
    pub path: PathBuf,
    pub called: u32,
}

impl Waypoint {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
            called: 0,
        }
    }
}

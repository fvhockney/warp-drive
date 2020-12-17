use crate::waypoint::Waypoint;
use crate::WarpError;
use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;
use toml;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub waypoints: Vec<Waypoint>,
}

impl Default for Config {
    fn default() -> Self {
        Self { waypoints: vec![] }
    }
}

impl Config {
    pub fn add(&mut self, name: String, path: PathBuf) -> Result<(), WarpError> {
        self.waypoints.push(Waypoint::new(name, path));
        self.save()?;
        Ok(())
    }

    pub fn update(&mut self, name: String, path: PathBuf) -> Result<(), WarpError> {
        let index = self
            .waypoints
            .iter()
            .position(|e| e.name == name)
            .ok_or_else(move || WarpError::NoWaypointWithName(name))?;
        let mut waypoint = self.waypoints.swap_remove(index);
        waypoint.path = path;
        self.waypoints.push(waypoint);
        self.save()?;
        Ok(())
    }

    pub fn load() -> Result<Config, WarpError> {
        let config = confy::load("warp_drive")?;
        Ok(config)
    }

    fn save(&self) -> Result<(), WarpError> {
        confy::store("warp_drive", self)?;
        Ok(())
    }

    pub fn remove(&mut self, name: String) -> Result<(), WarpError> {
        let new_waypoints: Vec<Waypoint> = self
            .waypoints
            .drain(..)
            .filter(|e| e.name != name)
            .collect();
        self.waypoints = new_waypoints;
        self.save()?;
        Ok(())
    }

    pub fn path() -> Result<PathBuf, WarpError> {
        let config_path = confy::get_configuration_file_path("warp_drive")?;
        Ok(config_path)
    }

    pub fn get(name: String) -> Result<Waypoint, WarpError> {
        let config = Self::load()?;
        let waypoint = config.waypoints.iter().find(|e| e.name == name);
        let res_waypoint = waypoint.ok_or_else(|| WarpError::NoWaypointWithName(name))?;
        Ok(res_waypoint.clone())
    }

    pub fn dump() -> Result<String, WarpError> {
        let config = Self::load()?;
        Ok(toml::to_string(&config)?)
    }

    pub fn list_all(verbosity: ListVerbosity) -> Result<String, WarpError> {
        let space = String::from(" ");
        let config = Self::load()?;
        let mut stringified = String::from("");
        let longest_str = config.waypoints.iter().fold(0, |acc, x| {
            if x.name.len() > acc {
                x.name.len()
            } else {
                acc
            }
        });
        for point in config.waypoints.iter() {
            stringified.push_str(&point.name);
            if verbosity == ListVerbosity::Long {
                stringified.push_str(&space.repeat(longest_str - point.name.len() + 2));
                stringified.push_str(&point.path.display().to_string());
            }
            stringified.push_str("\n");
        }
        Ok(stringified)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum ListVerbosity {
    Long,
    Short,
}

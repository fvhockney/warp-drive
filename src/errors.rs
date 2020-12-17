use confy::ConfyError;
use std::error::Error;
use std::fmt;

pub enum WarpError {
    NoWaypointWithName(String),
    ConfyError(ConfyError),
    SerError(toml::ser::Error),
    CWDError,
    ClapError,
}

impl fmt::Display for WarpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WarpError::NoWaypointWithName(name) => {
                write!(f, "No waypoint named \"{}\" found!", name)
            }
            WarpError::ConfyError(err) => write!(f, "{}", err),
            WarpError::SerError(err) => write!(f, "{}", err),
            WarpError::CWDError => write!(f, "Either the directory doesn't exist or you don't have appropriate permissions for it"),
            WarpError::ClapError => write!(f, "Clap parsing error"),
        }
    }
}

impl fmt::Debug for WarpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WarpError::NoWaypointWithName(name) => {
                write!(f, "No waypoint named \"{}\" found!", name)
            }
            WarpError::ConfyError(err) => write!(f, "{}", err),
            WarpError::SerError(err) => write!(f, "{}", err),
            WarpError::CWDError => write!(f, "Either the directory doesn't exist or you don't have appropriate permissions for it"),
            WarpError::ClapError => write!(f, "Clap parsing error"),
        }
    }
}

impl From<toml::ser::Error> for WarpError {
    fn from(err: toml::ser::Error) -> WarpError {
        WarpError::SerError(err)
    }
}

impl From<ConfyError> for WarpError {
    fn from(err: ConfyError) -> WarpError {
        WarpError::ConfyError(err)
    }
}

impl Error for WarpError {}

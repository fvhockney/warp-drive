use std::env;
use std::process::exit;

mod cli;
use cli::cli;

mod waypoint;

mod errors;
use errors::WarpError;

mod config;
use config::{Config, ListVerbosity};

fn add_entry(name: String) -> Result<String, WarpError> {
    let path = env::current_dir().map_err(|_err| WarpError::CWDError)?;
    let n_name = name.clone();
    let n_path = path.clone();
    let mut config = Config::load()?;
    config.add(name, path)?;
    Ok(format!(
        "printf \"Added warp-point {} for {}\\n\"",
        n_name,
        n_path.as_path().display(),
    ))
}

fn update_entry(name: String) -> Result<String, WarpError> {
    let path = env::current_dir().map_err(|_err| WarpError::CWDError)?;
    let n_name = name.clone();
    let n_path = path.clone();
    let mut config = Config::load()?;
    config
        .update(name.clone(), path.clone())
        .or_else(move |_| config.add(name, path))?;
    Ok(format!(
        "printf \"updated warppoint {} to {}\\n\"",
        n_name,
        n_path.as_path().display(),
    ))
}

fn delete_entry(name: &str) -> Result<String, WarpError> {
    let mut config = Config::load()?;
    config.remove(name.into())?;
    Ok(format!("printf \"Deleted warp-point {}\\n\"", name))
}

fn edit_config() -> Result<String, WarpError> {
    let path = Config::path()?;
    Ok(format!("$EDITOR {}", path.as_path().display()))
}

fn goto_entry(name: &str) -> Result<String, WarpError> {
    let waypoint = Config::get(name.into())?;
    Ok(format!("cd {}", waypoint.path.as_path().display()))
}

fn dump() -> Result<String, WarpError> {
    let config = Config::dump()?;
    Ok(format!("printf \"%b\" {:?}", config))
}

fn list_all(verbosity: ListVerbosity) -> Result<String, WarpError> {
    let paths = Config::list_all(verbosity)?;
    Ok(format!("printf \"{}\\n\"", paths))
}

fn init() -> Result<String, WarpError> {
    Ok(format!(
        "
warp () {{
    __warp_output=$(warp-drive-bin \"$@\")
    __warp_exit_code=$?
    if [ $__warp_exit_code -eq 0 ]; then
        if eval \"$__warp_output\"; then
            return 0
        else
            printf \"%s\\nIs the parking break on?\n\" \"$__warp_output\" >&2
            return 1
        fi
    else
        printf \"%s\\n\" \"$__warp_output\" >&2
        return $__warp_exit_code
    fi
}}
"
    ))
}

fn print_and_exit(output: Result<String, WarpError>) {
    match output {
        Ok(x) => {
            println!("{}", x);
            exit(0)
        }
        Err(x) => {
            eprintln!("{}", x);
            exit(127)
        }
    }
}

fn main() {
    let args = cli().get_matches_safe();
    match args {
        Ok(matches) => {
            if let Some(waypoint) = matches.value_of("waypoint") {
                let output = goto_entry(waypoint);
                print_and_exit(output)
            }
            let output = match matches.subcommand() {
                ("dump", _) => dump(),
                ("edit", _) => edit_config(),
                ("init", _) => init(),
                ("add", Some(matches)) => {
                    let name = matches.value_of("name").expect("clap error");
                    add_entry(name.into())
                }
                ("to", Some(matches)) => {
                    let name = matches.value_of("name").expect("clap error");
                    goto_entry(name)
                }
                ("eject", Some(matches)) => {
                    let name = matches.value_of("name").expect("clap error");
                    delete_entry(name)
                }
                ("update", Some(matches)) => {
                    let name = matches.value_of("name").expect("clap error");
                    update_entry(name.into())
                }
                ("ls", Some(matches)) => {
                    let verbosity = match matches.is_present("long") {
                        true => ListVerbosity::Long,
                        false => ListVerbosity::Short,
                    };
                    list_all(verbosity)
                }
                _ => Err(WarpError::ClapError),
            };
            print_and_exit(output)
        }
        Err(e) => match e {
            clap::Error {
                kind: clap::ErrorKind::HelpDisplayed,
                message,
                ..
            } => {
                println!("{}", message);
                exit(1)
            }
            clap::Error {
                kind: clap::ErrorKind::VersionDisplayed,
                message,
                ..
            } => {
                println!("{}", message);
                exit(1)
            }
            x => {
                println!("{}", x.message);
                exit(1)
            }
        },
    }
}

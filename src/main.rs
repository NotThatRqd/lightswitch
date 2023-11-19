use std::{fs, io::{self, ErrorKind}, process::ExitCode};
use lightswitch::{start_lightswitch, Config};

#[tokio::main]
async fn main() -> ExitCode {
    let config = match load_config() {
        Ok(config) => config,
        Err(_) => return ExitCode::FAILURE,
    };
    start_lightswitch(config).await;
    ExitCode::SUCCESS
}

fn load_config() -> Result<Config, ()> {
    let contents = match fs::read_to_string("lightswitch_config.toml") {
        Ok(content) => content,
        Err(e) => {
            handle_io_error(e);
            return Err(());
        }
    };
    let config = match toml::from_str(&contents) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("There was a problem parsing the config: {}", e.message());
            return Err(());
        }
    };
    Ok(config)
}

fn handle_io_error(error: io::Error) {
    eprintln!("There was a problem reading the config: {error}");
    if error.kind() == ErrorKind::NotFound {
        eprintln!("NOTE: You might be missing a `lightswitch_config.toml` file.");
    }
}

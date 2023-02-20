use append::append_media;
use args::{Arguments, Commands, ConfigCommands};
use clap::Parser;
use config::Config;
use confy;

mod ankiconnect;
mod append;
mod args;
mod config;
mod error;
use error::TimmError;

#[tokio::main]
async fn main() -> Result<(), TimmError> {
    // Get configuration for Picture field and SentenceAudio field
    let cfg = match confy::load::<config::Config>("timm", "card-opt") {
        Ok(v) => v,
        Err(e) => return Err(TimmError::ConfigError(e)),
    };

    // Get the arguments given by ShareX
    let args = Arguments::parse();

    match args.command {
        Commands::Append { media, path } => append_media(cfg, media, path).await?,
        Commands::Config(config) => match config.command {
            ConfigCommands::Init => match confy::store("timm", "card-opt", Config::default()) {
                Ok(_) => print!("Successfully created default configuration file"),
                Err(e) => return Err(TimmError::ConfigError(e)),
            },
            ConfigCommands::Path => {
                let path = confy::get_configuration_file_path("timm", "card-opt")
                    .expect("Could not get file path");
                println!("{}", path.display());
            }
        },
    }

    Ok(())
}

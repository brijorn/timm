use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
pub enum MediaType {
    Picture,
    Audio,
}

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "timm")]
#[command(about = "Append media to Anki cards", long_about = None)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    #[command(
        about = "The main function, adds the given audio or picture to the latest anki card"
    )]
    Append {
        /// The type of media being appended
        #[arg(required = true)]
        media: MediaType,

        /// The path to the media
        #[arg(required = true)]
        path: String,
    },
    #[command(about = "Configuration options", long_about = None)]
    Config(ConfigArgs),
}

#[derive(Debug, Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommands,
}
#[derive(Debug, Subcommand)]
pub enum ConfigCommands {
    #[command(about = "Create/Set the configuration to the default values")]
    Init,
    Path,
}

use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
pub enum ActionType {
    Picture,
    Audio
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
pub struct Args {
   /// Name of the person to greet
   #[arg(short, long, value_enum)]
   pub action_type: ActionType,

   /// Number of times to greet
   #[arg(short, long)]
   pub file_path: String,
}

#[derive(Debug)]
pub enum TimmError {
    ConfigError(confy::ConfyError),
    ArgumentError(clap::error::Error),
    HttpError(reqwest::Error),
    DeserializeError(String),
    ResponseError(String)
}
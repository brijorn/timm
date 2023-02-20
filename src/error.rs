#[derive(Debug)]
pub enum TimmError {
    ConfigError(confy::ConfyError),
    HttpError(reqwest::Error),
    DeserializeError(String),
    ResponseError(String),
}

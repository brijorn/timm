use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub audio_field: String,
    pub picture_field: String,
}

/// `Config` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            audio_field: "SentenceAudio".to_string(),
            picture_field: "Picture".to_string(),
        }
    }
}

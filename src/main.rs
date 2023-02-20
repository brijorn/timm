use std::{path::Path, collections::HashMap};

use ankiconnect::{AnkiConnect, AnkiConnectBodyParams, AnkiConnectNoteParams, AnkiConnectMedia};
use args::ActionType;
use clap::Parser;
use config::Config;
use confy;

mod ankiconnect;
mod args;
mod config;
mod error;
use error::TimmError;



#[tokio::main]
async fn main() -> Result<(), TimmError> {
    // Get the arguments given by ShareX
    let args = match args::Args::try_parse() {
        Ok(v) => v,
        Err(e) => return Err(TimmError::ArgumentError(e)),
    };

    // Get configuration for Picture field and SentenceAudio field
    let cfg = match confy::load::<config::Config>("timm", "card-options") {
        Ok(v) => v,
        Err(e) => return Err(TimmError::ConfigError(e)),
    };

    println!("{:?}", confy::get_configuration_file_path("timm", None));
    let path = Path::new(&args.file_path);
    
    let process = match args.action_type {
        ActionType::Audio => update_audio(path, cfg).await,
        ActionType::Picture => update_picture(path, cfg).await
    };

    match process {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

async fn update_audio(path: &Path, cfg: Config) -> Result<(), TimmError>  {
    let last_note_id = match AnkiConnect::default().get_last_added_note_id().await {
        Ok(v) => v,
        Err(e) => return Err(e)
    };

    println!("{:?}", last_note_id);
    let mut fields = HashMap::new();

    fields.insert(cfg.audio_field.clone(), "".to_string());

    let params = AnkiConnectBodyParams {
        deck: None,
        query: None,
        note: Some(AnkiConnectNoteParams {
            id: last_note_id,
            fields,
            audio: Some([AnkiConnectMedia {
                path: path.to_str().unwrap().to_string(),
                filename: path.file_name().unwrap().to_str().unwrap().to_string(),
                fields: [cfg.audio_field]
            }]),
            picture: None
        })
    };

    let apple = serde_json::to_string(&params);


    println!("{:#?}", &apple);

    AnkiConnect::default().update_note_fields(params).await
}

async fn update_picture(path: &Path, cfg: Config) -> Result<(), TimmError> {
    let last_note_id = match AnkiConnect::default().get_last_added_note_id().await {
        Ok(v) => v,
        Err(e) => return Err(e)
    };

    println!("{:?}", last_note_id);
    let mut fields = HashMap::new();

    fields.insert(cfg.picture_field.clone(), "".to_string());

    let params = AnkiConnectBodyParams {
        deck: None,
        query: None,
        note: Some(AnkiConnectNoteParams {
            id: last_note_id,
            fields,
            picture: Some([AnkiConnectMedia {
                path: path.to_str().unwrap().to_string(),
                filename: path.file_name().unwrap().to_str().unwrap().to_string(),
                fields: [cfg.picture_field]
            }]),
            audio: None
        })
    };

    let apple = serde_json::to_string(&params);


    println!("{:#?}", &apple);

    AnkiConnect::default().update_note_fields(params).await
}
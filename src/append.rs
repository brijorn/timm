use std::{collections::HashMap, path::Path};

use crate::{
    ankiconnect::{AnkiConnect, AnkiConnectBodyParams, AnkiConnectMedia, AnkiConnectNoteParams},
    args::MediaType,
    config::Config,
    error::TimmError,
};

pub async fn append_media(cfg: Config, media: MediaType, path: String) -> Result<(), TimmError> {
    let path = Path::new(&path);

    let process = match media {
        MediaType::Audio => update_audio(path, cfg).await,
        MediaType::Picture => update_picture(path, cfg).await,
    };

    match process {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

async fn update_audio(path: &Path, cfg: Config) -> Result<(), TimmError> {
    let last_note_id = match AnkiConnect::default().get_last_added_note_id().await {
        Ok(v) => v,
        Err(e) => return Err(e),
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
                fields: [cfg.audio_field],
            }]),
            picture: None,
        }),
    };

    let apple = serde_json::to_string(&params);

    println!("{:#?}", &apple);

    AnkiConnect::default().update_note_fields(params).await
}

async fn update_picture(path: &Path, cfg: Config) -> Result<(), TimmError> {
    let last_note_id = match AnkiConnect::default().get_last_added_note_id().await {
        Ok(v) => v,
        Err(e) => return Err(e),
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
                fields: [cfg.picture_field],
            }]),
            audio: None,
        }),
    };

    AnkiConnect::default().update_note_fields(params).await
}

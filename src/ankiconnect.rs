use std::{collections::HashMap, fmt::Display};

use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};

use crate::error::TimmError;
pub struct AnkiConnect {
    client: Client,
    base_path: String,
}

impl Default for AnkiConnect {
    fn default() -> Self {
        Self {
            client: Client::new(),
            base_path: "http://127.0.0.1:8765".to_string(),
        }
    }
}

enum Method {
    Get,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
enum AnkiConnectAction {
    FindNotes,
    UpdateNoteFields,
}

impl Display for AnkiConnectAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnkiConnectAction::FindNotes => write!(f, "findNotes"),
            AnkiConnectAction::UpdateNoteFields => write!(f, "updateNoteFields"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AnkiConnectBodyParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deck: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<AnkiConnectNoteParams>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AnkiConnectNoteParams {
    pub id: i64,
    pub fields: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<[AnkiConnectMedia; 1]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<[AnkiConnectMedia; 1]>,
}

#[derive(Deserialize, Serialize)]
pub struct AnkiConnectNoteFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<[String; 1]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<[String; 1]>,
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct AnkiConnectMedia {
    pub path: String,
    pub filename: String,
    pub fields: [String; 1],
}

#[derive(Deserialize, Serialize)]
struct AnkiConnectBody {
    action: AnkiConnectAction,
    version: u8,
    params: AnkiConnectBodyParams,
}

impl AnkiConnectBody {
    pub fn new(action: AnkiConnectAction, version: u8, params: AnkiConnectBodyParams) -> Self {
        Self {
            action,
            version,
            params,
        }
    }
}

impl AnkiConnect {
    async fn request(
        self,
        method: Method,
        action: AnkiConnectAction,
        params: AnkiConnectBodyParams,
    ) -> Result<Response, Error> {
        let req = match method {
            Method::Get => self
                .client
                .get(self.base_path)
                .json(&AnkiConnectBody::new(action, 6, params)),
        };

        req.send().await
    }
    pub async fn update_note_fields(self, params: AnkiConnectBodyParams) -> Result<(), TimmError> {
        let res = match self
            .request(Method::Get, AnkiConnectAction::UpdateNoteFields, params)
            .await
        {
            Ok(v) => v,
            Err(e) => return Err(TimmError::HttpError(e)),
        };

        match res.json::<serde_json::Value>().await {
            Ok(_) => Ok(()),
            Err(e) => return Err(TimmError::ResponseError(e.to_string())),
        }
    }

    pub async fn get_last_added_note_id(self) -> Result<i64, TimmError> {
        let params = AnkiConnectBodyParams {
            deck: None,
            query: Some("added:1".to_string()),
            note: None,
        };

        let res = match self
            .request(Method::Get, AnkiConnectAction::FindNotes, params)
            .await
        {
            Ok(v) => v,
            Err(e) => return Err(TimmError::HttpError(e)),
        };

        let body = match res.json::<AnkiConnectResponse<Vec<i64>>>().await {
            Ok(v) => v,
            Err(e) => {
                println!("{:#?}", e);
                return Err(TimmError::DeserializeError(
                    "Could not Deserialize AnkiConnect Response".to_string(),
                ));
            }
        };

        match body.result.iter().max() {
            Some(v) => Ok(v.to_owned()),
            None => Err(TimmError::ResponseError("No Value Found".to_string())),
        }
    }

    pub async fn gui_browse() {
        todo!();
    }
}

#[derive(Deserialize, Serialize)]
struct AnkiConnectResponse<T> {
    result: T,
    error: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::ankiconnect::AnkiConnect;

    #[tokio::test]
    async fn get_last_added_note() {
        let anki_connect = AnkiConnect::default();

        let result = match anki_connect.get_last_added_note_id().await {
            Ok(v) => v,
            Err(e) => return println!("{:#?}", e),
        };
        assert_eq!(result, 1676847645233);
    }
}

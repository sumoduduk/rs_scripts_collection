use serde::{Deserialize, Serialize};

use super::read_json;

#[derive(Debug, Deserialize, Serialize)]
pub enum JsonEnum {
    FromFile,
    FromJsonImage,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonFile {
    pub original_filename: String,
    pub hash_filename: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonImage {
    pub asset_id: String,
    pub url: String,
    pub original_filename: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ValueReturned {
    FileValue(Vec<JsonFile>),
    JsonValue(Vec<JsonImage>),
}

use ValueReturned::*;

impl ValueReturned {
    pub fn get_arr_file(self) -> Option<Vec<JsonFile>> {
        match self {
            FileValue(data) => Some(data),
            _ => None,
        }
    }

    pub fn get_arr_json(self) -> Option<Vec<JsonImage>> {
        match self {
            JsonValue(data) => Some(data),
            _ => None,
        }
    }
}

pub fn load(str_data: &str, choice: JsonEnum) -> eyre::Result<ValueReturned> {
    let str_data = read_json::read_file(str_data)?;
    match choice {
        JsonEnum::FromFile => {
            let json_struct = serde_json::from_str::<Vec<JsonFile>>(&str_data)?;
            Ok(ValueReturned::FileValue(json_struct))
        }
        JsonEnum::FromJsonImage => {
            let json_struct = serde_json::from_str::<Vec<JsonImage>>(&str_data)?;
            Ok(ValueReturned::JsonValue(json_struct))
        }
    }
}


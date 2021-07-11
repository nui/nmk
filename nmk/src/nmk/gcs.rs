use std::fs;
use std::io::Read;
use std::path::Path;

use serde::{Deserialize, Serialize};

const GET_OBJECT_BASE_URL: &str = "https://www.googleapis.com/storage/v1/b/nmk.nuimk.com/o";

#[derive(Deserialize)]
pub struct ListObjectResponse {
    pub kind: String,
    pub items: Vec<ObjectMeta>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectMeta {
    pub id: String,
    pub self_link: String,
    pub media_link: String,
    pub name: String,
    pub generation: String,
    pub size: String,
    pub md5_hash: String,
    pub etag: String,
}

impl ObjectMeta {
    pub fn write_to_file(&self, path: &Path) {
        let json_data =
            serde_json::to_string_pretty(self).expect("failed to serialize ObjectMeta to json");
        fs::write(path, json_data).expect("failed to write ObjectMeta to file");
    }

    pub fn read_from_file(path: &Path) -> Self {
        let json_data = fs::read(path).expect("failed to read ObjectMeta from file");
        serde_json::from_slice(&json_data).expect("failed to deserialize ObjectMeta")
    }
}

pub fn download_file(media_link: &str) -> Result<impl Read + Send, ureq::Error> {
    Ok(ureq::get(media_link).call()?.into_reader())
}

pub fn get_object_meta(url: &str) -> crate::Result<ObjectMeta> {
    let response = ureq::get(url).call()?;
    Ok(response.into_json()?)
}

pub fn get_object_meta_url(path: &str) -> String {
    format!("{}/{}", GET_OBJECT_BASE_URL, path)
}

pub fn list_objects(url: &str) -> crate::Result<Vec<ObjectMeta>> {
    let response = ureq::get(url).call()?;
    let list_result: ListObjectResponse = response.into_json()?;
    Ok(list_result.items)
}

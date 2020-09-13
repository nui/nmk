use std::fmt::{self, Debug, Display};
use std::fs;
use std::path::Path;

use bytes::{Buf, Bytes};
use reqwest::Client;
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
            serde_json::to_string_pretty(self).expect("Unable to serialize ObjectMeta to json");
        fs::write(path, json_data).expect("Unable to write ObjectMeta to file");
    }

    pub fn read_from_file(path: &Path) -> Self {
        let json_data = fs::read(path).expect("Unable to read ObjectMeta from file");
        serde_json::from_slice(&json_data).expect("Unable to deserialize ObjectMeta")
    }
}

#[derive(Debug)]
enum GcsError {
    HttpError { status: u16, url: String },
}

impl Display for GcsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // This Debug is intentional
        Debug::fmt(self, f)
    }
}

impl std::error::Error for GcsError {}

pub async fn download_file(client: &Client, media_link: &str) -> crate::Result<Bytes> {
    let response = client.get(media_link).send().await?;
    let status = response.status();
    if !status.is_success() {
        return Err(GcsError::HttpError {
            status: status.as_u16(),
            url: media_link.to_string(),
        }
        .into());
    }
    Ok(response.bytes().await?)
}

pub async fn get_object_meta(client: &Client, url: &str) -> crate::Result<ObjectMeta> {
    let response = client.get(url).send().await?;
    let status = response.status();
    if !status.is_success() {
        return Err(GcsError::HttpError {
            status: status.as_u16(),
            url: url.to_string(),
        }
        .into());
    }
    let data = response.bytes().await?;
    let meta = serde_json::from_slice(data.bytes())?;
    Ok(meta)
}

pub fn get_object_meta_url(path: &str) -> String {
    format!("{}/{}", GET_OBJECT_BASE_URL, path)
}

pub async fn list_objects(client: &Client, url: &str) -> crate::Result<Vec<ObjectMeta>> {
    let response = client.get(url).send().await?;
    let status = response.status();
    if !status.is_success() {
        Err(GcsError::HttpError {
            status: status.as_u16(),
            url: url.to_string(),
        })?
    }
    let data = response.bytes().await?;
    let list_result = serde_json::from_slice::<ListObjectResponse>(data.bytes())?;
    Ok(list_result.items)
}

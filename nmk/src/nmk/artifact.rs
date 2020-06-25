use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::str::FromStr;

use bytes::Bytes;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Metadata {
    etag: String,
}

impl Metadata {
    pub fn etag(&self) -> &str {
        &self.etag
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string_pretty(self).expect("To json string should not fail")
    }

    pub fn write_to_file(&self, dst: impl AsRef<Path>) -> std::io::Result<()> {
        std::fs::write(dst.as_ref(), self.to_string())
    }

    pub fn read_from_file(src: impl AsRef<Path>) -> Option<Self> {
        let data = std::fs::read(src.as_ref()).ok()?;
        serde_json::from_slice(&data).ok()
    }
}

impl FromStr for Metadata {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

#[derive(Debug)]
enum DownloadFileError {
    EtagNotFound,
    HttpError { status: u16, url: String },
}

impl Display for DownloadFileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DownloadFileError {}

pub async fn download_file_metadata(
    client: &Client,
    url: impl AsRef<str>,
) -> Result<Metadata, Box<dyn std::error::Error>> {
    let url = url.as_ref();
    let response = client.head(url).send().await?;
    let status = response.status();
    if status.is_success() {
        let etag = response
            .headers()
            .get("ETag")
            .ok_or(DownloadFileError::EtagNotFound)?
            .to_str()?
            .trim_matches('"')
            .to_owned();
        Ok(Metadata { etag })
    } else {
        Err(DownloadFileError::HttpError {
            status: status.as_u16(),
            url: url.to_string(),
        })?
    }
}

pub async fn download_file(
    client: &Client,
    url: impl AsRef<str>,
) -> Result<Bytes, Box<dyn std::error::Error>> {
    let url = url.as_ref();
    let response = client.get(url).send().await?;
    let status = response.status();
    if status.is_success() {
        Ok(response.bytes().await?)
    } else {
        Err(DownloadFileError::HttpError {
            status: status.as_u16(),
            url: url.to_string(),
        })?
    }
}

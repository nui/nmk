use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use hyper::body::HttpBody;
use hyper::client::connect::dns::GaiResolver;
use hyper::client::HttpConnector;
use hyper::{Body, Client, Uri};
use hyper_rustls::HttpsConnector;

use tempfile::tempfile;

use crate::nmkup::BoxError;

type HttpsClient = Client<HttpsConnector<HttpConnector<GaiResolver>>, Body>;

pub struct SecureClient(HttpsClient);

impl SecureClient {
    pub fn new() -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        Self(client)
    }

    #[allow(dead_code)]
    pub fn into_client(self) -> HttpsClient {
        self.0
    }

    pub async fn download_as_file(&self, uri: Uri) -> Result<File, BoxError> {
        let mut response = self.0.get(uri).await?;

        let mut file = tempfile()?;

        while let Some(chunk) = response.body_mut().data().await {
            let buf = chunk?;
            file.write(&buf)?;
        }
        // seek to the beginning of file
        file.seek(SeekFrom::Start(0))?;
        Ok(file)
    }

    pub async fn get_bytes(&self, uri: Uri) -> Result<Bytes, BoxError> {
        let mut response = self.0.get(uri).await?;
        let mut buf = BytesMut::new();

        while let Some(chunk) = response.body_mut().data().await {
            buf.put(chunk?);
        }
        Ok(buf.to_bytes())
    }
}

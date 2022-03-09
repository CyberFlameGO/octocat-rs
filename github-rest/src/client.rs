//! This module contains a pre-built client you can use instead of making your
//! own client. you can still do this though by implementing the [`Requester`]
//! trait

use async_trait::async_trait;
use github_api::end_points::{EndPoints, Methods};
use reqwest::header;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Display;

use crate::{GithubRestError, Requester};

/// A default implementation of the [`Requester`] trait.
pub struct DefaultRequester {
    client: reqwest::Client,
}

impl DefaultRequester {
    pub fn new<T>(auth: T) -> Self
    where
        T: Into<String> + Display,
    {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str("tricked.pro/v2").unwrap(),
        );
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_str("application/vnd.github.v3+json").unwrap(),
        );
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(auth.to_string().as_str()).unwrap(),
        );
        let client = reqwest::Client::builder().default_headers(headers).build().unwrap();
        DefaultRequester { client }
    }

    pub fn new_none() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str("tricked.pro/v2").unwrap(),
        );
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_str("application/vnd.github.v3+json").unwrap(),
        );

        let client = reqwest::Client::builder().default_headers(headers).build().unwrap();
        DefaultRequester { client }
    }
}

#[async_trait]
impl Requester for DefaultRequester {
    async fn raw_req<T, V>(&self, url: EndPoints, query: Option<&T>, body: Option<V>) -> Result<String, GithubRestError>
    where
        T: Serialize + ?Sized + std::marker::Send + std::marker::Sync,
        V: Into<Self::Body> + std::marker::Send,
    {
        let path = format!("https://api.github.com{}", url.path());

        let mut req = match url.method() {
            Methods::Get => self.client.get(path),
            Methods::Post => self.client.post(path),
            Methods::Put => self.client.put(path),
            Methods::Patch => self.client.patch(path),
            Methods::Delete => self.client.delete(path),
        };

        if let Some(query) = query {
            req = req.query(query)
        }

        if let Some(body) = body {
            req = req.body(body)
        }

        let res = req.send().await?;

        match res.status().as_u16() {
            200..=299 => {}
            _ => {
                return Err(GithubRestError::ResponseError(res.status(), res.text().await?));
            }
        }
        let txt = res.text().await?;

        Ok(txt)
    }

    async fn req<T, V, A: DeserializeOwned>(
        &self,
        url: EndPoints,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<A, GithubRestError>
    where
        T: Serialize + ?Sized + std::marker::Send + std::marker::Sync,
        V: Into<Self::Body> + std::marker::Send,
    {
        let r = self.raw_req(url, query, body).await?;
        Ok(serde_json::from_str(&r)?)
    }
}

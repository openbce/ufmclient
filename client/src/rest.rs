/*
 * SPDX-FileCopyrightText: Copyright (c) 2021-2023 NVIDIA CORPORATION & AFFILIATES. All rights reserved.
 * SPDX-License-Identifier: LicenseRef-NvidiaProprietary
 *
 * NVIDIA CORPORATION, its affiliates and licensors retain all intellectual
 * property and proprietary rights in and to this material, related
 * documentation and any modifications thereto. Any use, reproduction,
 * disclosure or distribution of this material and related documentation
 * without an express license agreement from NVIDIA CORPORATION or
 * its affiliates is strictly prohibited.
 */

use std::fmt;
use std::fmt::{Display, Formatter};

use hyper::client::HttpConnector;
use hyper::header::{AUTHORIZATION, CONTENT_TYPE};
use hyper::{Body, Client, Method, Uri};
use hyper_tls::HttpsConnector;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RestError {
    #[error("{0}")]
    Unknown(String),
    #[error("'{0}' not found")]
    NotFound(String),
    #[error("failed to auth '{0}'")]
    AuthFailure(String),
    #[error("invalid configuration '{0}'")]
    InvalidConfig(String),
}

impl From<hyper::Error> for RestError {
    fn from(value: hyper::Error) -> Self {
        if value.is_user() {
            return RestError::AuthFailure(value.message().to_string());
        }

        RestError::Unknown(value.message().to_string())
    }
}

#[derive(Clone, Debug)]
pub enum RestScheme {
    Http,
    Https,
}

impl From<String> for RestScheme {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "HTTP" => RestScheme::Http,
            "HTTPS" => RestScheme::Https,
            _ => RestScheme::Http,
        }
    }
}

impl Display for RestScheme {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RestScheme::Http => write!(f, "http"),
            RestScheme::Https => write!(f, "https"),
        }
    }
}

pub struct RestClientConfig {
    pub username: String,
    pub password: String,
    pub address: String,
    pub port: Option<u16>,
    pub scheme: RestScheme,
}

pub struct RestClient {
    base_url: String,
    auth_info: String,
    scheme: RestScheme,
    http_client: hyper::Client<HttpConnector>,
    https_client: hyper::Client<HttpsConnector<HttpConnector>>,
}

impl RestClient {
    pub fn new(conf: &RestClientConfig) -> Result<RestClient, RestError> {
        // TODO(k82cn): also support credential auth.
        let auth = format!("{}:{}", conf.username, conf.password);
        let auth_info = format!("Basic {}", base64::encode(auth));

        let base_url = match &conf.port {
            None => format!("{}://{}", conf.scheme, conf.address),
            Some(p) => format!("{}://{}:{}", conf.scheme, conf.address, p),
        };
        let _ = base_url
            .parse::<Uri>()
            .map_err(|_| RestError::InvalidConfig("invalid rest address".to_string()))?;

        Ok(Self {
            base_url,
            auth_info,
            scheme: conf.scheme.clone(),
            // TODO(k82cn): Add timout for the clients.
            http_client: Client::new(),
            https_client: Client::builder().build::<_, hyper::Body>(HttpsConnector::new()),
        })
    }

    pub async fn get<'a, T: serde::de::DeserializeOwned>(
        &'a self,
        path: &'a String,
    ) -> Result<T, RestError> {
        let resp = self.execute_request(Method::GET, path, None).await?;
        let data = serde_json::from_str(&resp)
            .map_err(|_| RestError::InvalidConfig("invalid response".to_string()))?;

        Ok(data)
    }

    pub async fn post(&self, path: &String, data: String) -> Result<(), RestError> {
        self.execute_request(Method::POST, path, Some(data)).await?;

        Ok(())
    }

    pub async fn put(&self, path: &String, data: String) -> Result<(), RestError> {
        self.execute_request(Method::PUT, path, Some(data)).await?;

        Ok(())
    }

    pub async fn delete(&self, path: &String) -> Result<(), RestError> {
        self.execute_request(Method::DELETE, path, None).await?;

        Ok(())
    }

    async fn execute_request(
        &self,
        method: Method,
        path: &String,
        data: Option<String>,
    ) -> Result<String, RestError> {
        let url = format!("{}/{}", self.base_url, path);
        let uri = url
            .parse::<Uri>()
            .map_err(|_| RestError::InvalidConfig("invalid path".to_string()))?;

        let body = data.unwrap_or(String::new());

        let req = hyper::Request::builder()
            .method(method)
            .uri(uri)
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, self.auth_info.to_string())
            .body(Body::from(body))
            .map_err(|_| RestError::InvalidConfig("invalid rest request".to_string()))?;

        let body = match &self.scheme {
            RestScheme::Http => self.http_client.request(req).await?,
            RestScheme::Https => self.https_client.request(req).await?,
        };

        let chunk = hyper::body::to_bytes(body.into_body()).await?;
        let data = String::from_utf8(chunk.to_vec()).unwrap();

        Ok(data)
    }
}

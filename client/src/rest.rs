use std::env;

use base64::{engine::general_purpose, Engine as _};

use hyper::header::{AUTHORIZATION, CONTENT_TYPE};
use hyper::{Body, Client, Method, Response, Uri};
use hyper_tls::HttpsConnector;

use crate::types::RestError;
use crate::types::RestSchema;

pub struct RestClient {
    username: String,
    password: String,
    address: String,
    port: String,
    schema: RestSchema,
}

impl RestClient {
    pub fn new() -> Result<RestClient, RestError> {
        let username = env::var("UFM_USERNAME")?;
        let password = env::var("UFM_PASSWORD")?;
        let address = env::var("UFM_ADDRESS")?;
        let port = env::var("UFM_PORT")?;
        let schema = env::var("UFM_HTTP_SCHEMA")?;

        Ok(Self {
            username,
            password,
            address,
            port,
            schema: RestSchema::from(schema),
        })
    }

    fn build_auth(&self) -> String {
        let auth = format!("{}:{}", self.username, self.password);
        // TODO(k82cn): also support credential auth.
        format!("Basic {}", general_purpose::STANDARD_NO_PAD.encode(auth))
    }

    async fn execute_request(
        &self,
        method: Method,
        path: &String,
        data: Option<String>,
    ) -> hyper::Result<Response<Body>> {
        let url = format!("{}://{}:{}/{}", self.schema, self.address, self.port, path);
        let uri = url.parse::<Uri>().unwrap();

        let body = data.unwrap_or("".to_string());

        let req = hyper::Request::builder()
            .method(method)
            .uri(uri)
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, self.build_auth())
            .body(Body::from(body))
            .unwrap();

        match &self.schema {
            RestSchema::Http => {
                let client = Client::new();
                client.request(req).await
            }
            RestSchema::Https => {
                let https = HttpsConnector::new();
                let client = Client::builder().build::<_, hyper::Body>(https);
                client.request(req).await
            }
        }
    }

    pub async fn get(&self, path: &String) -> Result<String, RestError> {
        let body = self.execute_request(Method::GET, path, None).await?;
        let chunk = hyper::body::to_bytes(body.into_body()).await?;
        let data = String::from_utf8(chunk.to_vec()).unwrap();

        Ok(data)
    }

    pub fn post(&self, path: String, data: String) {}

    pub fn put(&self, path: String, data: String) {}

    pub fn delete(&self, path: String) {}
}

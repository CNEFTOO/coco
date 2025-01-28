use reqwest::{Client, header::HeaderMap};
use crate::rule::Poc;

pub struct HttpClient<'a> {
    client: &'a Client,
}

impl<'a> HttpClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        HttpClient { client }
    }

    pub async fn send_request(&self, url: &str, poc: &Poc) -> Result<(u16, HeaderMap, String), reqwest::Error> {
        let method = if poc.method.is_empty() { "GET" } else { &poc.method };
        let mut request = self.client.request(method.parse().unwrap(), url);
        
        // 添加请求头
        for (key, value) in &poc.headers {
            request = request.header(key, value);
        }
        
        // 添加请求体
        if let Some(body) = &poc.body {
            request = request.body(body.clone());
        }
        
        let response = request.send().await?;
        let status = response.status().as_u16();
        let headers = response.headers().clone();
        let body = response.text().await?;
        
        Ok((status, headers, body))
    }
}
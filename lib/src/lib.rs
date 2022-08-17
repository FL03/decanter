/*
    Appellation: rs-crypto-sdk <library>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Endpoint {
    Http(String),
    Https(String),
    WebSocket(String)
}

impl Endpoint {
    pub fn new(endpoint: String) -> Self {
        if endpoint.contains("https") {
            Self::Https(endpoint)
        }
        else if endpoint.contains("http") {
            Self::Http(endpoint)
        } else if endpoint.contains("wss") {
            Self::WebSocket(endpoint)
        } else {
            panic!("Failed to match input to an option")
        }
    }
    pub fn from<T: std::string::ToString>(endpoint: T) -> Self {
        Self::new(endpoint.to_string())
    }
}

impl Default for Endpoint {
    fn default() -> Self {
        Self::from("https://")
    }
}



#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ApiClient {
    pub url: String
}

impl ApiClient {
    pub fn new(url: String) -> Self {
        Self { url }
    }
    pub fn from<T: std::string::ToString>(url: T) -> Self {
        Self::new(url.to_string())
    }
    pub fn new_client(&self) -> reqwest::Client {
        reqwest::Client::new()
    }
    pub fn session(&self) -> scsys::BoxResult<reqwest::Client> {
        let client = self.new_client();
        Ok(client)
    }
    pub async fn get(&self) -> scsys::BoxResult<reqwest::Response> {
        let response = self.session().ok().unwrap().get(self.url.as_str()).send().await.expect("Request Error");
        Ok(response)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint() {
        let actual = Endpoint::new("https://google.com".to_string());
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}
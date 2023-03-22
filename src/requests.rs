use std::collections::HashMap;

use crate::openai::OpenAI;

pub type Json = serde_json::Value;
pub type ApiResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ApiError(String),
    RequestError(String),
}

pub trait Requests {
    fn post(&self, url: &str, body: Json) -> ApiResult<Json>;
    fn get(&self, url: &str) -> ApiResult<Json>;
}

impl Requests for OpenAI {
    fn post(&self, sub_url: &str, body: Json) -> ApiResult<Json> {
        let mut headers = HashMap::new();
        headers.insert("Authorization", &format!("Bearer {}", self.auth.api_key));

        log::info!("=== 🚀 Post url: {:?}, body: {body}", sub_url);
        println!("=== 🚀 Post url: {:?}, body: {body}", sub_url);

        let response = self
            .agent
            .post(&(self.api_url.clone() + sub_url))
            .set("Content-Type", "application/json")
            .set(
                "OpenAI-Organization",
                &self.auth.organization.clone().unwrap_or_default(),
            )
            .set("Authorization", &format!("Bearer {}", self.auth.api_key))
            .send_json(body);

        match response {
            Ok(resp) => Ok(resp.into_json::<Json>().unwrap()),
            Err(err) => Err(Error::RequestError(err.to_string())),
        }
    }

    fn get(&self, sub_url: &str) -> ApiResult<Json> {
        let mut headers = HashMap::new();
        headers.insert("Authorization", &format!("Bearer {}", self.auth.api_key));

        log::info!("=== 🚀 Get url: {:?}", sub_url);

        let response = self
            .agent
            .get(&(self.api_url.clone() + sub_url))
            .set("Content-Type", "application/json")
            .set(
                "OpenAI-Organization",
                &self.auth.organization.clone().unwrap_or_default(),
            )
            .set("Authorization", &format!("Bearer {}", self.auth.api_key))
            .call();

        match response {
            Ok(resp) => Ok(resp.into_json::<Json>().unwrap()),
            Err(err) => Err(Error::RequestError(err.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::openai;
    use super::*;
    use ureq::json;
    
    #[test]
    fn test_post() {
        let openai = openai::new_test_openai();
        let body = json!({
            "model": "gpt-3.5-turbo",
            "messages": [{"role": "user", "content": "Say this is a test!"}],
            "temperature": 0.7
        });
        let sub_url = "chat/completions";
        let result = openai.post(sub_url, body);
        assert_eq!(result.unwrap().to_string().contains("This is a test"), true);
    }

    #[test]
    fn test_get() {
        let openai = openai::new_test_openai();
        let resp = openai.get("models");
        assert_eq!(resp.unwrap().to_string().contains("babbage"), true);
    }
}

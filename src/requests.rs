use crate::openai::OpenAI;
use crate::*;

#[cfg(not(test))]
use log::{debug, error};

#[cfg(test)]
use std::{eprintln as error, println as debug};

pub trait Requests {
	fn post(&self, sub_url: &str, body: Json) -> ApiResult<Json>;
	fn get(&self, sub_url: &str) -> ApiResult<Json>;
}

impl Requests for OpenAI {
	fn post(&self, sub_url: &str, body: Json) -> ApiResult<Json> {
		let path = if self.api_version.is_empty() {
			self.api_url.clone() + sub_url
		} else {
			// azure openai:
			// api_url/chat/completions?api-version=2023-03-15-preview
			self.api_url.clone() + sub_url + "?api-version=" + &*self.api_version.clone()
		};
		let response = self
			.agent
			.post(&path)
			.set("Content-Type", "application/json")
			.set("OpenAI-Organization", &self.auth.organization.clone().unwrap_or_default())
			.set("Authorization", &format!("Bearer {}", self.auth.api_key))
			.send_json(body);

		deal_response(response, sub_url)
	}

	fn get(&self, sub_url: &str) -> ApiResult<Json> {
		let path = if self.api_version.is_empty() {
			self.api_url.clone() + sub_url
		} else {
			// azure openai:
			// api_url/chat/completions?api-version=2023-03-15-preview
			self.api_url.clone() + sub_url + "?api-version=" + &*self.api_version.clone()
		};
		let response = self
			.agent
			.get(&path)
			.set("Content-Type", "application/json")
			.set("OpenAI-Organization", &self.auth.organization.clone().unwrap_or_default())
			.set("Authorization", &format!("Bearer {}", self.auth.api_key))
			.call();

		deal_response(response, sub_url)
	}
}

fn deal_response(response: Result<ureq::Response, ureq::Error>, sub_url: &str) -> ApiResult<Json> {
	match response {
		Ok(resp) => {
			let json = resp.into_json::<Json>().unwrap();
			debug!("<== ✔️\n\tDone api: {sub_url}, resp: {json}");
			Ok(json)
		},
		Err(err) => match err {
			ureq::Error::Status(status, response) => {
				let error_msg = response.into_json::<Json>().unwrap();
				error!("<== ❌\n\tError api: {sub_url}, status: {status}, error: {error_msg}");
				return Err(Error::ApiError(format!("{error_msg}")));
			},
			ureq::Error::Transport(e) => {
				error!("<== ❌\n\tError api: {sub_url}, error: {:?}", e.to_string());
				Err(Error::RequestError(e.to_string()))
			},
		},
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::openai;
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
		let result = openai.post(sub_url, body).unwrap();
		assert!(result.to_string().contains("This is a test"));
	}

	#[test]
	fn test_get() {
		let openai = openai::new_test_openai();
		let resp = openai.get("models").unwrap();
		assert!(resp.to_string().contains("babbage"));
	}
}

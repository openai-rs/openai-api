// Given a prompt and an instruction, the model will return an edited version of the prompt.

//! Edits API

use crate::requests::Requests;
use crate::*;
use serde::{Deserialize, Serialize};

use super::{completions::Completion, EDIT_CREATE};

#[derive(Debug, Serialize, Deserialize)]
pub struct EditsBody {
	pub model: String,
	pub instruction: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub n: Option<i32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f32>,
}

pub trait EditsApi {
	/// Creates a new edit for the provided input, instruction, and parameters.
	fn edit_create(&self, chat_body: &EditsBody) -> ApiResult<Completion>;
}

impl EditsApi for OpenAI {
	fn edit_create(&self, chat_body: &EditsBody) -> ApiResult<Completion> {
		let request_body = serde_json::to_value(chat_body).unwrap();
		let res = self.post(EDIT_CREATE, request_body)?;
		let completion: Completion = serde_json::from_value(res).unwrap();
		Ok(completion)
	}
}

#[cfg(test)]
mod tests {
	use crate::{
		apis::edits::{EditsApi, EditsBody},
		openai::new_test_openai,
	};

	#[test]
	fn test_edit_create() {
		let openai = new_test_openai();
		let body = EditsBody {
			model: "text-davinci-edit-001".to_string(),
			temperature: None,
			top_p: None,
			n: Some(2),
			instruction: "Fix the spelling mistakes".to_string(),
			input: Some("What day of the wek is it?".to_string()),
		};
		let rs = openai.edit_create(&body);
		let choice = rs.unwrap().choices;
		let text = &choice[0].text.as_ref().unwrap();
		assert!(text.contains("week"));
	}
}

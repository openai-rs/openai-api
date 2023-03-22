use crate::{
    openai::OpenAI,
    requests::{ApiResult, Error, Json, Requests},
};
use serde::{Deserialize, Serialize};

use super::MODELS_LIST;
use super::MODELS_RETRIEVE;

/// List and describe the various models available in the API.
/// You can refer to the [Models](https://platform.openai.com/docs/models) documentation
/// to understand what models are available and the differences between them.
#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub object: Option<String>,
    pub owned_by: Option<String>,
    pub permission: Vec<Permission>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    pub id: String,
    pub object: Option<String>,
    pub created: u64,
    pub allow_create_engine: bool,
    pub allow_sampling: bool,
    pub allow_logprobs: bool,
    pub allow_search_indices: bool,
    pub allow_view: bool,
    pub allow_fine_tuning: bool,
    pub organization: Option<String>,
    pub group: Option<String>,
    pub is_blocking: bool,
}

pub trait ModelApi {
    fn models_list(&self) -> Result<Vec<Model>, Error>;
    fn models_retrieve(&self, model_id: &str) -> ApiResult<Model>;
}

impl ModelApi for OpenAI {
    fn models_list(&self) -> ApiResult<Vec<Model>> {
        let resp = self.get(MODELS_LIST);
        if let Err(e) = resp {
            return Err(e);
        }
        let res: Json = resp.unwrap();
        let data = res.as_object().unwrap().get("data");
        if let Some(data) = data {
            let models: Vec<Model> = serde_json::from_value(data.clone()).unwrap();
            return Ok(models);
        }
        Err(Error::ApiError("No data".to_string()))
    }

    fn models_retrieve(&self, model_id: &str) -> ApiResult<Model> {
        let resp = self.get(&(MODELS_RETRIEVE.to_owned() + model_id));
        if let Err(e) = resp {
            return Err(e);
        }
        let res: Json = resp.unwrap();
        let model: Model = serde_json::from_value(res.clone()).unwrap();
        Ok(model)
    }
}

#[cfg(test)]
mod tests {
    use crate::{apis::models::ModelApi, openai::new_test_openai};

    #[test]
    fn test_models() {
        let openai = new_test_openai();
        let result = openai.models_list();
        assert_eq!(result.unwrap().len() > 0, true);
    }

    #[test]
    fn test_get_model() {
        let openai = new_test_openai();
        let result = openai.models_retrieve("babbage");
        assert_eq!("babbage", result.unwrap().id);
    }
}

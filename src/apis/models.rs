use crate::{
    openai::OpenAI,
    requests::{ApiResult, Error, Json, Requests},
};
use serde::{Deserialize, Serialize};

use super::MODELS_API;

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    id: String,
    object: Option<String>,
    owned_by: Option<String>,
    permission: Vec<Permission>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Permission {
    id: String,
    object: Option<String>,
    created: u64,
    allow_create_engine: bool,
    allow_sampling: bool,
    allow_logprobs: bool,
    allow_search_indices: bool,
    allow_view: bool,
    allow_fine_tuning: bool,
    organization: Option<String>,
    group: Option<String>,
    is_blocking: bool,
}

pub trait ModelApi {
    fn models_list(&self) -> Result<Vec<Model>, Error>;
    fn models_retrieve(&self, model_id: &str) -> ApiResult<Model>;
}

impl ModelApi for OpenAI {
    fn models_list(&self) -> ApiResult<Vec<Model>> {
        let resp = self.get(MODELS_API);
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
        let resp = self.get(&(MODELS_API.to_owned() + "/" + model_id));
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

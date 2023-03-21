use ureq::{Agent, AgentBuilder};

/// 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
/// org-Vrhcw5F7a7zBBsSbvrqUpQFL
/// https://api.openai.com/v1/
pub struct Auth {
    pub api_key: String,
    pub organization: Option<String>,
}
#[allow(dead_code)]
impl Auth {
    pub fn new(api_key: &str) -> Auth {
        Auth {
            api_key: api_key.to_string(),
            organization: None,
        }
    }
}

pub struct OpenAI {
    pub auth: Auth,
    pub api_url: String,
    pub agent: Agent,
}

#[allow(dead_code)]
impl OpenAI {
    pub fn new(auth: Auth, api_url: &str) -> OpenAI {
        OpenAI {
            auth,
            api_url: api_url.to_string(),
            agent: AgentBuilder::new().build(),
        }
    }

    pub fn set_proxy(mut self, proxy: &str) -> OpenAI {
        let proxy = ureq::Proxy::new(proxy).unwrap();
        self.agent = ureq::AgentBuilder::new().proxy(proxy).build();
        self
    }
}

#[cfg(test)]
pub fn new_test_openai() -> OpenAI {
    let api_key = "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";
    let auth = Auth::new(api_key);
    OpenAI::new(auth, "https://api.openai.com/v1/").set_proxy("http://192.168.3.10:10808")
}

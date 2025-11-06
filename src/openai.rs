use serde::{Deserialize, Serialize};
use ureq::{Agent, AgentBuilder};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Auth {
	pub api_key: String,
	pub organization: Option<String>,
}

impl Auth {
	pub fn new(api_key: &str) -> Auth {
		Auth { api_key: api_key.to_string(), organization: None }
	}

	pub fn with_organization(mut self, organization: &str) -> Self {
		self.organization = Some(organization.to_string());
		self
	}

	pub fn from_env() -> Result<Self, String> {
		let api_key =
			std::env::var("OPENAI_API_KEY").map_err(|_| "Missing OPENAI_API_KEY".to_string())?;
		let organization = std::env::var("OPENAI_ORGANIZATION").ok();

		Ok(Self { api_key, organization })
	}
}

/// Container for API credentials and URL configuration
#[derive(Debug, Clone)]
pub struct Credentials {
	pub auth: Auth,
	pub api_url: String,
}

impl Credentials {
	/// Create new credentials with the specified API key and URL
	pub fn new(api_key: &str, api_url: &str) -> Self {
		Self { auth: Auth::new(api_key), api_url: api_url.to_string() }
	}

	/// Add organization ID to these credentials
	pub fn with_organization(mut self, organization: &str) -> Self {
		self.auth.organization = Some(organization.to_string());
		self
	}

	/// Load credentials from environment variables:
	/// - OPENAI_API_KEY: Required - your OpenAI API key
	/// - OPENAI_API_URL: Optional - defaults to "https://api.openai.com/v1/"
	/// - OPENAI_ORGANIZATION: Optional - your organization ID
	pub fn from_env() -> Result<Self, String> {
		let api_key =
			std::env::var("OPENAI_API_KEY").map_err(|_| "Missing OPENAI_API_KEY".to_string())?;

		let api_url = std::env::var("OPENAI_API_URL")
			.unwrap_or_else(|_| "https://api.openai.com/v1/".to_string());

		Ok(Self {
			auth: Auth { api_key, organization: std::env::var("OPENAI_ORGANIZATION").ok() },
			api_url,
		})
	}
}

#[derive(Debug, Clone)]
pub struct OpenAI {
	pub auth: Auth,
	pub api_url: String,
	pub(crate) agent: Agent,
}

impl OpenAI {
	pub fn new(auth: Auth, api_url: &str) -> Self {
		Self { auth, api_url: api_url.to_string(), agent: AgentBuilder::new().build() }
	}

	/// Initialize client from environment variables
	/// A convenient shorthand for Credentials::from_env() + from_credentials()
	pub fn from_env() -> Result<Self, String> {
		let credentials = Credentials::from_env()?;
		Ok(Self::from_credentials(credentials))
	}

	/// Initialize client using a Credentials object
	pub fn from_credentials(credentials: Credentials) -> Self {
		Self {
			auth: credentials.auth,
			api_url: credentials.api_url,
			agent: AgentBuilder::new().build(),
		}
	}

	pub fn builder() -> OpenAIBuilder {
		OpenAIBuilder::new()
	}

	pub fn set_proxy(mut self, proxy: &str) -> Self {
		let proxy = ureq::Proxy::new(proxy).map_err(|e| format!("Invalid proxy: {}", e)).unwrap();

		self.agent = ureq::AgentBuilder::new().proxy(proxy).build();
		self
	}

	pub fn use_env_proxy(mut self) -> Self {
		let proxy = match (std::env::var("http_proxy"), std::env::var("https_proxy")) {
			(Ok(http_proxy), _) => Some(http_proxy),
			(_, Ok(https_proxy)) => Some(https_proxy),
			_ => {
				log::warn!("Missing http_proxy or https_proxy");
				None
			},
		};

		if let Some(proxy) = proxy {
			if let Ok(proxy) = ureq::Proxy::new(&proxy) {
				self.agent = ureq::AgentBuilder::new().proxy(proxy).build();
			}
		}
		self
	}
}

pub struct OpenAIBuilder {
	api_key: Option<String>,
	api_url: Option<String>,
	organization: Option<String>,
	use_proxy: bool,
	proxy: Option<String>,
}

impl OpenAIBuilder {
	fn new() -> Self {
		Self { api_key: None, api_url: None, organization: None, use_proxy: false, proxy: None }
	}

	pub fn api_key(mut self, api_key: &str) -> Self {
		self.api_key = Some(api_key.to_string());
		self
	}

	pub fn api_url(mut self, api_url: &str) -> Self {
		self.api_url = Some(api_url.to_string());
		self
	}

	pub fn organization(mut self, organization: &str) -> Self {
		self.organization = Some(organization.to_string());
		self
	}

	pub fn use_env_proxy(mut self) -> Self {
		self.use_proxy = true;
		self
	}

	pub fn proxy(mut self, proxy: &str) -> Self {
		self.proxy = Some(proxy.to_string());
		self
	}

	pub fn from_env(mut self) -> Self {
		if self.api_key.is_none() {
			self.api_key = std::env::var("OPENAI_API_KEY").ok();
		}

		if self.api_url.is_none() {
			self.api_url = std::env::var("OPENAI_API_URL").ok();
		}

		if self.organization.is_none() {
			self.organization = std::env::var("OPENAI_ORGANIZATION").ok();
		}

		self
	}

	pub fn build(self) -> Result<OpenAI, String> {
		let api_key = self.api_key.ok_or_else(|| "API key is required".to_string())?;

		let api_url = self.api_url.unwrap_or_else(|| "https://api.openai.com/v1/".to_string());

		let mut auth = Auth::new(&api_key);
		if let Some(org) = self.organization {
			auth.organization = Some(org);
		}

		let mut client = OpenAI::new(auth, &api_url);

		if let Some(proxy) = self.proxy {
			client = client.set_proxy(&proxy);
		} else if self.use_proxy {
			client = client.use_env_proxy();
		}

		Ok(client)
	}
}

#[cfg(test)]
pub fn new_test_openai() -> OpenAI {
	let auth = Auth::from_env().unwrap();
	OpenAI::new(auth, "https://api.openai.com/v1/").use_env_proxy()
}

#[cfg(test)]
pub fn new_test_openai_with_credentials() -> OpenAI {
	let credentials = Credentials::from_env().unwrap();
	OpenAI::from_credentials(credentials).use_env_proxy()
}

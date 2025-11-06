use openai_api_rust::openai::{Credentials, OpenAI};
use std::env;

#[test]
fn test_credentials_from_env() {
	// Save original env vars to restore them after test
	let original_api_key = env::var("OPENAI_API_KEY").ok();
	let original_api_url = env::var("OPENAI_API_URL").ok();
	let original_organization = env::var("OPENAI_ORGANIZATION").ok();

	// Set test values
	env::set_var("OPENAI_API_KEY", "test_key");
	env::set_var("OPENAI_API_URL", "https://test.api.com/v1/");
	env::set_var("OPENAI_ORGANIZATION", "test_org");

	// Test credentials creation
	let credentials = Credentials::from_env().unwrap();
	assert_eq!(credentials.auth.api_key, "test_key");
	assert_eq!(credentials.api_url, "https://test.api.com/v1/");
	assert_eq!(credentials.auth.organization, Some("test_org".to_string()));

	// Test OpenAI client creation with credentials
	let client = OpenAI::from_credentials(credentials);
	assert_eq!(client.auth.api_key, "test_key");
	assert_eq!(client.api_url, "https://test.api.com/v1/");
	assert_eq!(client.auth.organization, Some("test_org".to_string()));

	// Test direct creation from env
	let client = OpenAI::from_env().unwrap();
	assert_eq!(client.auth.api_key, "test_key");
	assert_eq!(client.api_url, "https://test.api.com/v1/");
	assert_eq!(client.auth.organization, Some("test_org".to_string()));

	// Test with default URL
	env::remove_var("OPENAI_API_URL");
	let credentials = Credentials::from_env().unwrap();
	assert_eq!(credentials.api_url, "https://api.openai.com/v1/");

	// Restore original env vars
	match original_api_key {
		Some(val) => env::set_var("OPENAI_API_KEY", val),
		None => env::remove_var("OPENAI_API_KEY"),
	}

	match original_api_url {
		Some(val) => env::set_var("OPENAI_API_URL", val),
		None => env::remove_var("OPENAI_API_URL"),
	}

	match original_organization {
		Some(val) => env::set_var("OPENAI_ORGANIZATION", val),
		None => env::remove_var("OPENAI_ORGANIZATION"),
	}
}

#[test]
fn test_credentials_builder() {
	// Create credentials directly
	let credentials = Credentials::new("manual_key", "https://manual.api.com/v1/")
		.with_organization("manual_org");

	assert_eq!(credentials.auth.api_key, "manual_key");
	assert_eq!(credentials.api_url, "https://manual.api.com/v1/");
	assert_eq!(credentials.auth.organization, Some("manual_org".to_string()));

	// Create OpenAI client using the builder pattern
	let client = OpenAI::builder()
		.api_key("builder_key")
		.api_url("https://builder.api.com/v1/")
		.organization("builder_org")
		.build()
		.unwrap();

	assert_eq!(client.auth.api_key, "builder_key");
	assert_eq!(client.api_url, "https://builder.api.com/v1/");
	assert_eq!(client.auth.organization, Some("builder_org".to_string()));
}

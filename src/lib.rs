#![warn(unused_crate_dependencies)]

pub mod apis;
pub use apis::*;
pub mod openai;
pub use openai::*;
mod requests;

use log as _;

pub type Json = serde_json::Value;
pub type ApiResult<T> = Result<T, Error>;

pub use openai::*;

#[derive(Debug)]
pub enum Error {
	/// An Error returned by the API
	ApiError(String),
	/// An Error not related to the API
	RequestError(String),
}

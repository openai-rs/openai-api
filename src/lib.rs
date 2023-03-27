#![warn(unused_crate_dependencies)]

pub mod apis;
use std::fmt::{Display, Formatter, self};

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

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::ApiError(msg) => write!(f, "API error: {}", msg),
            Error::RequestError(msg) => write!(f, "Request error: {}", msg),
        }
    }
}
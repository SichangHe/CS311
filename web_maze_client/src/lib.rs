use std::error::Error;

pub type ResultDyn<T> = Result<T, Box<dyn Error>>;

pub mod request;
pub mod response;
pub mod stats;

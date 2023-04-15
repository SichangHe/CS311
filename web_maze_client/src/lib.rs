use std::error::Error;

pub type ResultDyn<T> = Result<T, Box<dyn Error>>;

pub mod response;

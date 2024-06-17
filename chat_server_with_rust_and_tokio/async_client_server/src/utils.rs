use std::error::Error;

// result and error types we'll use throughout the application
pub type ChatError = Box<dyn Error + Send + Sync + 'static>;
pub type ChatResult<T> = Result<T, ChatError>;
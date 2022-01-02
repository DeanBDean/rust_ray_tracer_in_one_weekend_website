use std::error::Error;

pub type LocalError = Box<dyn Error + Send + Sync + 'static>;

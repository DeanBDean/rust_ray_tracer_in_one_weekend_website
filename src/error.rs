use std::error::Error;

#[allow(clippy::module_name_repetitions)]
pub type LocalError = Box<dyn Error + Send + Sync + 'static>;

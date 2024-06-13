use std::error::Error;

use crate::utils::{get_config::Config, registry::RegistryBaseColor};

#[derive(Debug)]
pub struct TransformOptions<'a> {
    pub filename: String,
    pub raw: String,
    pub config: &'a Config,
    pub base_color: Option<&'a RegistryBaseColor>,
}

pub async fn transform(options: TransformOptions<'_>) -> Result<String, Box<dyn Error>> {
    Ok(options.raw)
}

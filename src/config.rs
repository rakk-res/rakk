use serde::Deserialize;
use config::{Config as Loader, File, Environment};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub frame_id: String,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let s = Loader::builder()
            .set_default("frame_id", "local-rack-01")?
            .set_default("log_level", "info")?
            .add_source(File::with_name("Rakk").required(false))
            .add_source(Environment::with_prefix("RAKK"))
            .build()?;
        
        Ok(s.try_deserialize()?)
    }
}

use crate::core::blade::Blade;
use tracing::{warn, info};

pub struct CoolingSystem {
    max_load: f64,
    temp_threshold: f64,
}

impl CoolingSystem {
    pub fn new() -> Self {
        Self { 
            max_load: 1000.0,
            temp_threshold: 80.0,
        }
    }

    pub fn check_metrics(&self, blade: &Blade) -> anyhow::Result<()> {
        if blade.size > self.max_load {
            warn!("Overheat warning: Blade size {} exceeds cooling capacity", blade.size);
            return Err(anyhow::anyhow!("Thermal throttling engaged: Load too high"));
        }
        info!("Cooling system nominal. Blade accepted.");
        Ok(())
    }
}

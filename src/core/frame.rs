use std::sync::Arc;
use crate::config::Config;
use crate::core::blade::Blade;
use crate::core::cooling::CoolingSystem;
use crate::slots::Registry;
use tracing::{info, instrument};

pub struct Frame {
    config: Config,
    cooling: CoolingSystem,
    slots: Registry,
}

impl Frame {
    pub fn boot(config: Config) -> Self {
        info!("Booting Rakk Frame System (ID: {})...", config.frame_id);
        Self {
            config,
            cooling: CoolingSystem::new(),
            slots: Registry::init(),
        }
    }

    #[instrument(skip(self))]
    pub async fn mount(&self, blade: Blade) -> anyhow::Result<String> {
        info!("Mounting Blade: {} [{}]", blade.id, blade.asset);
        
        // 1. Cooling Check (Risk Management)
        self.cooling.check_metrics(&blade)?;
        
        // 2. Slot Lookup
        let slot = self.slots.get(&blade.target_slot)
            .ok_or_else(|| anyhow::anyhow!("Target slot '{}' not installed", blade.target_slot))?;
            
        // 3. Execution
        let result = slot.execute(blade).await?;
        info!("Execution successful: {}", result);
        
        Ok(result)
    }
}

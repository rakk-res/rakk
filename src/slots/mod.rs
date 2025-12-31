pub mod solana;
pub mod polymarket;
pub mod kalshi;

use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use crate::core::blade::Blade;

#[async_trait]
pub trait SlotInterface: Send + Sync {
    async fn execute(&self, blade: Blade) -> anyhow::Result<String>;
}

#[derive(Clone)]
pub enum Slot {
    Solana,
    Polymarket,
    Kalshi,
}

pub struct Registry {
    slots: HashMap<String, Arc<dyn SlotInterface>>,
}

impl Registry {
    pub fn init() -> Self {
        let mut slots = HashMap::new();
        // In a real app, we would load enabled slots from config
        slots.insert("solana".to_string(), Arc::new(solana::SolanaSlot::new("mock_rpc")) as Arc<dyn SlotInterface>);
        
        Self { slots }
    }
    
    pub fn get(&self, name: &str) -> Option<Arc<dyn SlotInterface>> {
        self.slots.get(name).cloned()
    }
}

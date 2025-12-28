use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blade {
    pub id: Uuid,
    pub asset: String,
    pub size: f64,
    pub side: Side,
    pub target_slot: String,
    pub timestamp: i64,
}

impl Blade {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            asset: "UNKNOWN".into(),
            size: 0.0,
            side: Side::Buy,
            target_slot: "solana".into(),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
    
    pub fn target(mut self, slot: &str) -> Self {
        self.target_slot = slot.to_string();
        self
    }
    
    pub fn asset(mut self, asset: &str) -> Self {
        self.asset = asset.to_string();
        self
    }

    pub fn size(mut self, size: f64) -> Self {
        self.size = size;
        self
    }

    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }
}

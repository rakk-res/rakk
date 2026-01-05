pub mod core;
pub mod slots;
pub mod utils;
pub mod config;

pub mod prelude {
    pub use crate::core::frame::Frame;
    pub use crate::core::blade::{Blade, Side};
    pub use crate::slots::Slot;
    pub use crate::config::Config;
    pub use anyhow::Result;
}

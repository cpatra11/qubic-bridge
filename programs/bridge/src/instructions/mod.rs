// src/solana/programs/bridge/src/instructions/mod.rs
use anchor_lang::prelude::*;

pub mod initialize_bridge;
pub mod add_validator;
pub mod remove_validator;
pub mod update_bridge_state;
pub mod emergency_pause;
pub mod emergency_unpause;
pub mod lock_assets;
pub mod unlock_assets;
pub mod validate_bridge;
pub mod update_config;
pub mod query;

pub use initialize_bridge::*;
pub use add_validator::*;
pub use remove_validator::*;
pub use update_bridge_state::*;
pub use emergency_pause::*;
pub use emergency_unpause::*;
pub use lock_assets::*;
pub use unlock_assets::*;
pub use validate_bridge::*;
pub use update_config::*;
pub use query::*;
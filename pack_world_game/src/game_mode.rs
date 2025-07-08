pub mod game_mode_inventory;
pub mod game_mode_shop;
pub mod game_mode_world;

pub use game_mode_inventory::*;
pub use game_mode_shop::*;
pub use game_mode_world::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GameMode {
    World,
    Shop,
    Inventory,
}

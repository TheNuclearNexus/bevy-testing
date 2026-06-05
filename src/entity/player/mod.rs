mod components;
#[cfg(feature = "dev")]
mod resources;
mod systems;

pub use components::*;
#[cfg(feature = "dev")]
pub use resources::*;
pub use systems::*;

pub const IDENT: &str = "Player";

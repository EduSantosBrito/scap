//! Cross Platform, Performant and High Quality screen recordings

pub mod capturer;
pub mod frame;
mod targets;
mod utils;

// Helper Methods
pub use targets::get_all_targets;
pub use targets::get_main_display;
pub use targets::get_target_dimensions;
pub use targets::Target;
pub use utils::has_permission;
pub use utils::is_supported;
pub use utils::request_permission;

//! Code shared between the WGSL front and back ends.

mod diagnostics;
mod to_wgsl;
mod types;

pub use diagnostics::DisplayFilterableTriggeringRule;
pub use to_wgsl::{ToWgsl, TryToWgsl, address_space_str};
pub use types::TypeContext;

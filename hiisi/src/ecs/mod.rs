extern crate specs;

mod plugin;

pub use self::specs::prelude::*;
pub use self::specs::*;
pub use self::specs::rayon as parallel;

pub use self::plugin::*;

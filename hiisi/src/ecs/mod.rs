extern crate specs;

pub use self::specs::prelude::*;
pub use self::specs::*;
pub use self::specs::rayon as parallel;

pub mod events {
    pub use super::specs::shrev::*;
}

extern crate specs;
extern crate specs_derive;

pub use self::specs::prelude::*;
pub use self::specs::*;
pub use self::specs::rayon as parallel;
pub use self::specs_derive::*;

pub mod events {
    pub use super::specs::shrev::*;
}

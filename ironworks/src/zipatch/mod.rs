//! Adapters to allow working with game data directly out of ZiPatch files.

pub mod lookup;
pub mod repository;
pub mod utility;
pub mod view;
pub mod zipatch;

pub use {
    repository::{Patch, PatchRepository},
    view::View,
    zipatch::ZiPatch,
};

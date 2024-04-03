//! Structs and utilities for parsing .mdl files.

pub mod container;
pub mod mesh;
pub mod model;
pub mod structs;

pub use {
    container::ModelContainer,
    mesh::{Mesh, VertexAttribute, VertexValues},
    model::{Lod, Model},
    structs::VertexAttributeKind,
};

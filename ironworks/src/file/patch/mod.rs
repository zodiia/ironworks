// TEMP
#![allow(missing_docs, dead_code)]

mod chunk;
mod lazy;
mod zipatch;

pub use {
	chunk::{AddDirectoryChunk, ApplyChunk, Chunk, DeleteDirectoryChunk, OptionKind},
	lazy::LazyStreamReader,
	zipatch::{ChunkIterator, ZiPatch},
};

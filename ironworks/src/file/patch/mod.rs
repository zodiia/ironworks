//! Structs and utilities for parsing ZiPatch .patch files.

pub mod chunk;
pub mod command;
pub mod zipatch;

pub use {
    chunk::{
        AddDirectoryChunk, ApplyChunk, Chunk, DeleteDirectoryChunk, FileHeaderChunk, FileHeaderV3,
        OptionKind, SqPackChunk,
    },
    command::{
        AddCommand, BlockHeader, DeleteCommand, ExpandCommand, FileOperation, FileOperationCommand,
        HeaderFileKind, HeaderKind, HeaderUpdateCommand, IndexUpdateCommand, IndexUpdateKind,
        PatchInfoCommand, SqPackFile, TargetInfoCommand, TargetPlatform, TargetRegion,
    },
    zipatch::{ChunkIterator, ZiPatch},
};

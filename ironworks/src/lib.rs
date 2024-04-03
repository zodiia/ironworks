//! Modular toolkit for working with FFXIV data.

// Lint config
#![allow(clippy::module_inception)]
#![warn(missing_debug_implementations, missing_docs)]
// Doc config
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]

pub mod error;
pub mod ironworks;
pub mod utility;

#[cfg(feature = "excel")]
pub mod excel;
pub mod file;
#[cfg(feature = "sestring")]
pub mod sestring;
#[cfg(feature = "sqpack")]
pub mod sqpack;
#[cfg(feature = "zipatch")]
pub mod zipatch;

pub use {
	crate::ironworks::{FileStream, Ironworks, Resource},
	error::{Error, ErrorValue},
};

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_send() {
		fn assert_send<T: Send>() {}
		assert_send::<Ironworks>();
		assert_send::<Error>();
		assert_send::<ErrorValue>();
	}

	#[test]
	fn test_sync() {
		fn assert_sync<T: Sync>() {}
		assert_sync::<Ironworks>();
		assert_sync::<Error>();
		assert_sync::<ErrorValue>();
	}
}

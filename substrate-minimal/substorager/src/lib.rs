//! Minimal implementation of Substrate storage.

#![deny(missing_docs)]

#[cfg(test)] mod test;

// std
use std::{
	fmt::{Display, Formatter, Result as FmtResult},
	ops::Deref,
};
// crates.io
#[cfg(feature = "codec")] use parity_scale_codec::{Decode, Encode};

/// Storage key.
///
/// Substrate reference(s):
/// - <https://github.com/paritytech/substrate/blob/c4d36065764ee23aeb3ccd181c4b6ecea8d2447a/primitives/storage/src/lib.rs#L35-L43>
#[derive(Debug, Default)]
pub struct StorageKey(pub Vec<u8>);
impl StorageKey {
	/// Create an empty [`StorageKey`].
	pub fn new() -> Self {
		Default::default()
	}
}
impl AsRef<[u8]> for StorageKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
impl Deref for StorageKey {
	type Target = [u8];

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl Display for StorageKey {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "{}", array_bytes::bytes2hex("0x", &self.0))
	}
}
impl From<Vec<u8>> for StorageKey {
	fn from(v: Vec<u8>) -> Self {
		Self(v)
	}
}
impl<const N: usize> From<[u8; N]> for StorageKey {
	fn from(a: [u8; N]) -> Self {
		Self(a.to_vec())
	}
}
impl From<&[u8]> for StorageKey {
	fn from(a: &[u8]) -> Self {
		Self(a.to_vec())
	}
}

/// Storage hasher.
///
/// Substrate reference(s):
/// - <https://github.com/paritytech/substrate/blob/c4d36065764ee23aeb3ccd181c4b6ecea8d2447a/frame/support/src/hash.rs#L25-L34>
#[derive(Debug)]
#[cfg_attr(feature = "codec", derive(Encode, Decode))]
pub enum StorageHasher {
	#[allow(missing_docs)]
	Blake2_128,
	#[allow(missing_docs)]
	Blake2_256,
	#[allow(missing_docs)]
	Blake2_128Concat,
	#[allow(missing_docs)]
	Twox128,
	#[allow(missing_docs)]
	Twox256,
	#[allow(missing_docs)]
	Twox64Concat,
}
impl StorageHasher {
	/// Hash the data and make it into a [`StorageKey`].
	pub fn hash(&self, data: &[u8]) -> StorageKey {
		match self {
			StorageHasher::Blake2_128 => subhasher::blake2_128(data).into(),
			StorageHasher::Blake2_256 => subhasher::blake2_256(data).into(),
			StorageHasher::Blake2_128Concat => subhasher::blake2_128_concat(data).into(),
			StorageHasher::Twox128 => subhasher::twox128(data).into(),
			StorageHasher::Twox256 => subhasher::twox256(data).into(),
			StorageHasher::Twox64Concat => subhasher::twox64_concat(data).into(),
		}
	}
}

/// Calculate the storage key of a pallet `StorageValue` item.
pub fn storage_key(pallet: &[u8], item: &[u8]) -> StorageKey {
	let mut storage_key = Vec::new();

	storage_key.extend_from_slice(&subhasher::twox128(pallet));
	storage_key.extend_from_slice(&subhasher::twox128(item));

	storage_key.into()
}

/// Calculate the storage key of a pallet `StorageMap` item.
pub fn storage_map_key(pallet: &[u8], item: &[u8], key: (&StorageHasher, &[u8])) -> StorageKey {
	let mut storage_map_key = storage_key(pallet, item);

	storage_map_key.0.extend_from_slice(&key.0.hash(key.1));

	storage_map_key
}

/// Calculate the storage key of a pallet `StorageDoubleMap` item.
pub fn storage_double_map_key(
	pallet: &[u8],
	item: &[u8],
	key1: (StorageHasher, &[u8]),
	key2: (StorageHasher, &[u8]),
) -> StorageKey {
	let mut storage_double_map_key = storage_key(pallet, item);

	storage_double_map_key.0.extend_from_slice(&key1.0.hash(key1.1));
	storage_double_map_key.0.extend_from_slice(&key2.0.hash(key2.1));

	storage_double_map_key
}

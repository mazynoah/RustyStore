//! # Rusty Store
//!
//! RustyStore is a Rust library for managing and storing serialized data using RON (Rusty Object Notation).
//!
//! ## Overview
//!
//! The library offers a set of utilities for reading, writing, and managing serialized data with RON. The primary components are:
//!
//!   - `Store`: A store is any struct which implements the Storing trait.
//!   - `Storage`: Manages file system paths for cache, data, and configuration storage.
//!   - `StoreHandle`: Represents a handle to a specific store, allowing access and modification of the data.
//!   - `StoreManager`: Provides an abstraction for managing and modifying store data, including options for committing or deferring changes.
//!
//! ## Examples
//!
//! ### `examples/minimal`
//!
//! ```rust
//! use rusty_store::{StoreManager, Storage, Storing};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize, Default, Storing)]
//! pub struct MyStore {
//!     pub count: u32,
//! }
//!
//! pub trait MyStoreTrait {
//!     fn increment_count(&mut self) -> Result<(), rusty_store::StoreError>;
//! }
//!
//! impl MyStoreTrait for StoreManager<MyStore> {
//!     fn increment_count(&mut self) -> Result<(), rusty_store::StoreError> {
//!         self.modify_store(|store| store.count += 1)
//!     }
//! }
//!
//!
//! // Initialize the Storage and create a new manager
//! let mut counter: StoreManager<MyStore> = Storage::new("com.github.mazynoah.storage")
//!     .new_manager("manager")
//!     .expect("Failed to create StoreManager");
//!
//! counter
//!     .increment_count()
//!     .expect("Could not increment count");
//!
//! println!("Count: {}", counter.get_store().count);
//!
//! ```
//!
//! ## Traits
//!
//! - **`Storing`**: This trait must be implemented by any type that needs to be stored. It requires the type to be serializable and deserializable using RON, and provides a method to define the type of storage (`Cache`, `Data`, `Config`).
//!

extern crate rustystore_macros;
pub use rustystore_macros::Storing;
mod manager;
mod storage;

pub use manager::StoreManager;
pub use storage::*;

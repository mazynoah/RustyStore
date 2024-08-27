//! # Rusty Store
//!
//! This library provides functionality for managing and storing application data using RON (Rusty Object Notation) format. It includes mechanisms for reading from and writing to the file system, handling data in different storage types, and managing store handles. The main components are `Storage`, `StoreHandle`, and `Storing` traits, with additional functionality provided through the `StorageManager` struct in the `manager` module.
//!
//! ## Components
//!
//! - `Storage`: Manages paths for cache, data, and configuration directories. It provides methods to read from and write to these paths.
//! - `StoreHandle`: Wraps a store and its identifier. It provides methods to access and modify the stored data.
//! - `Storing`: A trait that must be implemented by any type that is to be stored. It defines the type of storage and enforces serialization and deserialization requirements.
//! - `StoreError`: An enumeration of potential errors that may occur during file operations or data processing.
//!
//! ## Examples
//!
//! ### Reading and Writing Data
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use storage::{Storage, StoreHandle, Storing, StoringType};
//!
//! #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
//! pub struct MyStore {
//!     pub count: u32,
//! }
//!
//! impl Storing for MyStore {
//!     fn store_type() -> StoringType {
//!         StoringType::Data
//!     }
//! }
//!
//! // Initialize the Storage with the defaults
//! let storage = Storage::new("com.github.mazynoah.storage".to_owned());
//!
//! // Create a handle for managing the store data.
//! let mut handle = StoreHandle::<MyStore>::new("handle");
//!
//! // Read existing store from storage
//! storage
//!     .read(&mut handle)
//!     .expect("Failed to read from storage");
//!
//! // Modify the store data
//! let counter = handle.get_store_mut();
//!
//! counter.increment_count();
//! counter.increment_count();
//! counter.increment_count();
//!
//! // Write changes to disk
//! storage
//!     .write(&mut handle)
//!     .expect("Failed to write to storage");
//!
//! let counter = handle.get_store();
//!
//! println!("Count: {}", counter.count);
//! ```
//!
//! ## Error Handling
//!
//! The library uses the `StoreError` enum to represent errors that may occur during file operations or data parsing. Each variant provides detailed error information, including source errors for troubleshooting.
//!
//! ## Traits
//!
//! - **`Storing`**: This trait must be implemented by any type that needs to be stored. It requires the type to be serializable and deserializable using RON, and provides a method to define the type of storage (`Cache`, `Data`, `Config`).
//!
//! ## `Storage` Struct
//!
//! The `Storage` struct handles file system paths for different types of data. It provides methods to read from and write to these paths, including the ability to handle non-existent files by creating default values.
//!
//! ### Methods
//!
//! - `new(app_id: String) -> Self`: Creates a new `Storage` instance with paths derived from the provided application ID.
//! - `from(cache_dir: PathBuf, data_dir: PathBuf, config_dir: PathBuf) -> Self`: Creates a `Storage` instance with specified paths for cache, data, and configuration directories.
//! - `read<T: Storing>(&self, handle: &mut StoreHandle<T>) -> Result<(), StoreError>`: Reads the store data from a file and updates the provided `StoreHandle`.
//! - `write<T: Storing>(&self, handle: &mut StoreHandle<T>) -> Result<(), StoreError>`: Writes the current store data to a file from the provided `StoreHandle`.
//!
//! ## `StoreHandle` Struct
//!
//! The `StoreHandle` struct wraps a store and its identifier, providing methods to access and modify the stored data. It supports serialization and deserialization of the stored data.
//!
//! ### Methods
//!
//! - `new(store_id: &str) -> Self`: Creates a new `StoreHandle` with the given identifier and default store data.
//! - `get_store(&self) -> &T`: Returns a reference to the stored data.
//! - `get_store_mut(&mut self) -> &mut T`: Returns a mutable reference to the stored data.
//! - `store_id(&self) -> &str`: Returns the identifier of the store.
//! - `set_store(&mut self, store: T)`: Sets the store data.
//!

extern crate rustystore_macros;
pub use rustystore_macros::Storing;
mod manager;
mod storage;

pub use manager::StoreManager;
pub use storage::*;

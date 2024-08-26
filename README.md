# Storage Management in Rust

This crate provides a Rust library for managing and storing serialized data using RON (Rusty Object Notation). It includes functionality for handling various types of stores and managing their persistence.

## Overview

The library offers a set of utilities for reading, writing, and managing serialized data with RON. The primary components are:

- **`Storage`**: Manages file system paths for cache, data, and configuration storage.
- **`StoreHandle`**: Represents a handle to a specific store, allowing access and modification of the data.
- **`StorageManager`**: Provides an abstraction for managing and modifying store data, including options for committing or deferring changes.
- **`Store`**: A store is any kind of struct which implements the `Storing` trait.

## Usage

1. Add the library to your Cargo.toml:

```toml
    [dependencies]
    storage = "1"
```
2. Use the provided examples and components to manage your store data as demonstrated.

## Examples

### `examples/handle.rs`

Demonstrates basic usage of `StoreHandle` to read, modify, and write data to storage.

```rust
use serde::{Deserialize, Serialize};
use storage::{Storage, StoreHandle, Storing, StoringType};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct MyStore {
    pub count: u32,
}

impl MyStore {
    fn increment_count(&mut self) {
        self.count += 1;
    }
}

impl Storing for MyStore {
    fn store_type() -> StoringType {
        StoringType::Data
    }
}

fn main() {
    // Initialize the Storage
    let storage = Storage::new("com.github.mazynoah.storage".to_owned());

    // Create a handle for managing the store data.
    let mut handle = StoreHandle::<MyStore>::new("handle");

    // Read existing store from storage
    storage
        .read(&mut handle)
        .expect("Failed to read from storage");

    // Modify the store data
    let counter = handle.get_store_mut();

    counter.increment_count();
    counter.increment_count();
    counter.increment_count();

    // Write changes to disk
    storage
        .write(&mut handle)
        .expect("Failed to write to storage");

    let counter = handle.get_store();

    println!("Count: {}", counter.count);
}

```

### `examples/manager_uncommitted.rs`

Shows how to use StorageManager to manage data with uncommitted changes and then save them.

```rust
use serde::{Deserialize, Serialize};
use storage::{manager::StorageManager, Storage, StoreHandle, Storing, StoringType};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct MyStore {
    pub count: u32,
}

impl Storing for MyStore {
    fn store_type() -> StoringType {
        StoringType::Data
    }
}

pub trait MyStoreTrait {
    fn increment_count(&mut self);
}

impl MyStoreTrait for StorageManager<MyStore> {
    fn increment_count(&mut self) {
        self.modify_store_uncommitted(|store| store.count += 1)
    }
}

fn main() {
    // Initialize the Storage with the defaults
    let storage = Storage::new("com.github.mazynoah.storage".to_owned());

    // Create a handle for managing the store data.
    let handle = StoreHandle::<MyStore>::new("manager_uncommitted");

    // Use `StorageManager` to manage the handle's change.
    let mut manager =
        StorageManager::new(&storage, handle).expect("Failed to create StorageManager");

    // Modify the data without saving the changes to disk.
    manager.increment_count();
    manager.increment_count();
    manager.increment_count();

    // Save the data to the storage
    manager.save().expect("Failed to save count to storage");

    let counter = manager.get_store();

    println!("Count: {}", counter.count);
}
```
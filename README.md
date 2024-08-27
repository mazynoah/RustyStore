# Storage Management in Rust
[![Documentation](https://docs.rs/rusty-store/badge.svg)](https://docs.rs/rusty-store)
[![version](https://img.shields.io/crates/v/rusty-store.svg)](https://crates.io/crates/rusty-store)

RustyStore is a Rust library for managing and storing serialized data using RON (Rusty Object Notation). It includes functionality for handling various types of stores and managing their persistence.

## Overview

The library offers a set of utilities for reading, writing, and managing serialized data with RON. The primary components are:

- **`Storage`**: Manages file system paths for cache, data, and configuration storage.
- **`StoreHandle`**: Represents a handle to a specific store, allowing access and modification of the data.
- **`StoreManager`**: Provides an abstraction for managing and modifying store data, including options for committing or deferring changes.
- **`Store`**: A store is any kind of struct which implements the `Storing` trait.

## Usage

1. Add the library to your Cargo.toml:

```toml
    [dependencies]
    rusty-store = "0.1.0"
```
2. Use the provided examples and components to manage your store data as demonstrated.

## Examples

### `examples/minimal.rs`

The recommended minimal setup

```rust
use rusty_store::{StoreManager, Storage, Storing};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Storing)]
pub struct MyStore {
    pub count: u32,
}

pub trait MyStoreTrait {
    fn increment_count(&mut self) -> Result<(), rusty_store::StoreError>;
}

impl MyStoreTrait for StoreManager<MyStore> {
    fn increment_count(&mut self) -> Result<(), rusty_store::StoreError> {
        self.modify_store(|store| store.count += 1)
    }
}

fn main() {
    // Initialize the Storage and create a new manager
    let mut counter: StoreManager<MyStore> = Storage::new("com.github.mazynoah.storage")
        .new_manager("manager")
        .expect("Failed to create StoreManager");

    counter
        .increment_count()
        .expect("Could not increment count");

    println!("Count: {}", counter.get_store().count);
}

```

### `examples/handle.rs`

Demonstrates basic usage of `StoreHandle` to read, modify, and write data to storage.

```rust
use rusty_store::{Storage, StoreHandle, Storing};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Storing)]
pub struct MyStore {
    pub count: u32,
}

impl MyStore {
    fn increment_count(&mut self) {
        self.count += 1;
    }
}

fn main() {
    // Initialize the Storage with the defaults
    let storage = Storage::new("com.github.mazynoah.storage");

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


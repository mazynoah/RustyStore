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

fn main() {
    // Initialize the Storage with the defaults
    let storage = Storage::new("com.github.mazynoah.storage".to_owned());

    // Create a handle for managing the store data.
    let handle = StoreHandle::<MyStore>::new("manager");

    // Use `StorageManager` to manage the handle's change.
    let mut manager =
        StorageManager::new(&storage, handle).expect("Failed to create StorageManager");

    // Get a mutable reference to the store
    let counter = manager.get_store_mut();

    counter.count = 75;

    // Save the data to the storage
    manager.save().expect("Failed to save count to storage");

    let counter = manager.get_store();

    println!("Count: {}", counter.count);
}

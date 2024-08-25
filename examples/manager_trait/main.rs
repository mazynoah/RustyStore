use serde::{Deserialize, Serialize};
use storage::{self, manager::StorageManager, Storage, StoreHandle, Storing, StoringType};

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
    fn increment_count(&mut self) -> Result<(), storage::StoreError>;
}

impl MyStoreTrait for StorageManager<MyStore> {
    fn increment_count(&mut self) -> Result<(), storage::StoreError> {
        self.modify_store(|store| store.count += 1)
    }
}

fn main() {
    // Initialize the Storage with the defaults
    let storage = Storage::new("com.github.mazynoah.storage".to_owned());

    // Create a handle for managing the store data.
    let handle = StoreHandle::<MyStore>::new("manager_trait");

    // Use `StorageManager` to manage the store.
    let mut manager =
        StorageManager::new(&storage, handle).expect("Failed to create StorageManager");

    // Modify and save the data using `StorageManager`.
    manager
        .increment_count()
        .expect("Failed to increment count");

    let counter = manager.get_store();

    println!("Count: {}", counter.count);
}

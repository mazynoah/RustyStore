use rusty_store::{Storage, StoreManager, Storing};
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
    // Initialize the Storage with the defaults
    let storage = Storage::new("com.github.mazynoah.storage");

    // Use `StoreManager` to manage the store.
    let mut manager =
        StoreManager::new(&storage, "manager_trait").expect("Failed to create StoreManager");

    // Modify and save the data using `StoreManager`.
    manager
        .increment_count()
        .expect("Failed to increment count");

    let counter = manager.get_store();

    println!("Count: {}", counter.count);
}

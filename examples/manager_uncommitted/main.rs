use rusty_store::{Storage, StoreManager, Storing};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Storing)]
pub struct MyStore {
    pub count: u32,
}

pub trait MyStoreTrait {
    fn increment_count(&mut self);
}

impl MyStoreTrait for StoreManager<MyStore> {
    fn increment_count(&mut self) {
        self.modify_store_uncommitted(|store| store.count += 1)
    }
}

fn main() {
    // Initialize the Storage with the defaults
    let storage = Storage::new("com.github.mazynoah.storage");

    // Use `StoreManager` to manage the store.
    let mut manager =
        StoreManager::new(&storage, "manager_uncommitted").expect("Failed to create StoreManager");

    // Modify the data without saving the changes to disk.
    manager.increment_count();
    manager.increment_count();
    manager.increment_count();

    // Save the data to the storage
    manager.save().expect("Failed to save count to storage");

    let counter = manager.get_store();

    println!("Count: {}", counter.count);
}

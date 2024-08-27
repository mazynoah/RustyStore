use rusty_store::{Storage, StoreManager, Storing};
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

    // Create a StoreManager for managing the store data
    let mut counter_manager = storage
        .new_manager::<MyStore>("manager")
        .expect("Failed to create StoreManager");

    // Alternatively:
    let mut counter_manager =
        StoreManager::<MyStore>::new(&storage, "handle").expect("Failed to create StoreManager");

    // Get a mutable reference to the store
    let counter = counter_manager.get_store_mut();

    counter.count = 75;
    counter.increment_count();

    // Save the data to the storage
    counter_manager
        .save()
        .expect("Failed to save count to storage");

    let counter = counter_manager.get_store();

    println!("Count: {}", counter.count);
}

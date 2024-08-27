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

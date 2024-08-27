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

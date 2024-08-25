use super::{Storage, StoreError, StoreHandle, Storing};

/// Used to manage store handles
///
/// # Example
///
///```
/// use serde::{Deserialize, Serialize};
/// use storage::{manager::StorageManager, Storage, StoreHandle, Storing, StoringType};
///
/// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
/// pub struct MyStore {
///    pub count: u32,
/// }
///
/// impl Storing for MyStore {
///     fn store_type() -> StoringType {
///            StoringType::Data
///      }
/// }
///
/// // Initialize the Storage with the defaults
/// let storage = Storage::new("com.github.mazynoah.storage".to_owned());
///
/// // Create a handle for managing the store data.
/// let handle = StoreHandle::<MyStore>::new("manager");
///
/// // Use `StorageManager` to manage the handle's change.
/// let mut manager =
///    StorageManager::new(&storage, handle).expect("Failed to create StorageManager");
///
/// // Get a mutable reference to the store
/// let counter = manager.get_store_mut();
///
/// counter.count = 75;
///
/// // Save the data to the storage
/// manager.save().expect("Failed to save count to storage");
///
/// let counter = manager.get_store();
///
/// println!("Count: {}", counter.count);
///
///```
#[derive(Debug, Clone)]
pub struct StorageManager<T: Storing> {
    store: Storage,
    handle: StoreHandle<T>,
}

impl<T: Storing> StorageManager<T> {
    /// Creates a new `StorageManager` by reading the store data from the provided `Storage` into the `StoreHandle`.
    ///
    /// This function attempts to read the store data from the storage.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// let storage = Storage::new(&app);
    /// let handle = StoreHandle::<MyStore>::new("my_store_id");
    /// let manager = StorageManager::new(&storage, handle).expect("Failed to create StorageManager");
    /// ```
    pub fn new(storage: &Storage, mut handle: StoreHandle<T>) -> Result<Self, StoreError> {
        storage.read(&mut handle)?;

        Ok(Self {
            store: storage.clone(),
            handle,
        })
    }

    /// Returns a reference to the stored data.
    pub fn get_store(&self) -> &T {
        self.handle.get_store()
    }

    /// Returns a reference to the stored data.
    pub fn get_store_mut(&mut self) -> &mut T {
        self.handle.get_store_mut()
    }

    /// Reads the stored data from the storage.
    /// This allows to get changes external to the application
    pub fn get_store_alive(&mut self) -> Result<&T, StoreError> {
        self.store.read(&mut self.handle)?;
        Ok(self.handle.get_store())
    }

    /// This method is used to modify the store.
    ///
    /// # Example
    ///
    /// ```
    /// let storage = Storage::new(&app);
    /// let handle = StoreHandle::<MyStore>::new("my_store_id");
    /// let manager = StorageManager::new(&storage, handle).expect("Failed to create StorageManager");
    ///
    /// manager.modify_store(|store| store.some_field = 25).expect("Failed to write store modifications");
    /// ```
    pub fn modify_store<F>(&mut self, mut change: F) -> Result<(), StoreError>
    where
        F: FnMut(&mut T),
    {
        let store = self.handle.get_store_mut();
        change(store);
        self.save()
    }

    /// This method is used to modify the store without committing changes to disk.
    ///
    /// # Example
    ///
    /// ```
    /// let storage = Storage::new(&app);
    /// let handle = StoreHandle::<MyStore>::new("my_store_id");
    /// let manager = StorageManager::new(&storage, handle).expect("Failed to create StorageManager");
    ///
    /// manager.modify_store_uncommitted(|store| store.some_field = 25);
    ///
    /// manager.save().expect("Failed to save modifications");
    /// ```
    pub fn modify_store_uncommitted<F>(&mut self, mut change: F)
    where
        F: FnMut(&mut T),
    {
        let store = self.handle.get_store_mut();
        change(store);
    }

    /// This method writes the current state of the store to the storage.
    pub fn save(&mut self) -> Result<(), StoreError> {
        self.store.write(&mut self.handle)
    }
}

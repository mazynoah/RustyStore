use crate::storage::{Storage, StoreError, StoreHandle, Storing};

/// `StoreManager` manages the lifecycle of a store within a specified `Storage` backend. It handles reading and writing store data, as well as providing mutable access to the store's contents.
///
/// The `StoreManager` is designed to work with any type that implements the `Storing` trait, allowing it to manage different kinds of store data structures. It abstracts away the complexity of directly interacting with the storage backend, providing an easy-to-use API for managing and persisting store data.
///
///
///
/// ## Example
///
/// ```rust
/// use serde::{Deserialize, Serialize};
/// use storage::{StoreManager, Storing};
///
/// #[derive(Serialize, Deserialize, Default, Storing)]
/// pub struct MyStore {
///     pub count: u32,
/// }
///
/// let storage = Storage::new("APP_ID");
///
/// // Create a StoreManager for managing the store data
/// let mut manager = StoreManager::<MyStore>::new(&storage, "my_store_id")
///        .expect("Failed to create StoreManager");
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
/// ```
#[derive(Debug, Clone)]
pub struct StoreManager<T: Storing> {
    store: Storage,
    handle: StoreHandle<T>,
}

impl<T: Storing> StoreManager<T> {
    /// Creates a new `StoreManager` by reading the store data from the provided `Storage` into the `StoreHandle`.
    ///
    /// This function attempts to read the store data from the storage.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// let storage = Storage::new(app_id);
    /// let handle = StoreHandle::<MyStore>::new("my_store_id");
    /// let manager = StoreManager::from_handle(&storage, handle).expect("Failed to create StoreManager");
    /// ```
    pub fn from_handle(storage: &Storage, mut handle: StoreHandle<T>) -> Result<Self, StoreError> {
        storage.read(&mut handle)?;

        Ok(Self {
            store: storage.clone(),
            handle,
        })
    }

    /// Creates a new `StoreManager` by reading the store data from the provided `Storage` into the `StoreHandle`.
    ///
    /// This function attempts to read the store data from the storage.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// let storage = Storage::new("APP_ID");
    /// let manager = StoreManager::new(&storage, "my_store_id").expect("Failed to create StoreManager");
    /// ```
    pub fn new(storage: &Storage, store_id: &str) -> Result<Self, StoreError> {
        let mut handle = StoreHandle::<T>::new(store_id);
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

    /// Modifies the store and commits the changes to the storage
    ///
    /// # Example
    ///
    /// ```
    /// let storage = Storage::new("APP_ID");
    /// let mut manager = StoreManager::<MyStore>::new(&storage, "my_store_id")
    ///        .expect("Failed to create StoreManager");
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
    /// let storage = Storage::new("APP_ID");
    /// let mut manager = StoreManager::<MyStore>::new(&storage, "my_store_id")
    ///        .expect("Failed to create StoreManager");
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

use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use thiserror::Error;

use log::debug;
use log::info;
use log::log;
use log::warn;

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("RON parsing error: {0}")]
    RonParse(#[source] ron::error::SpannedError),

    #[error("RON error: {0}")]
    Ron(#[source] ron::error::Error),

    #[error("Failed to open file: {0}")]
    FileOpen(#[source] std::io::Error),

    #[error("Failed to read from file: {0}")]
    Read(#[source] std::io::Error),

    #[error("Failed to create directory: {0}")]
    CreateDir(#[source] std::io::Error),

    #[error("Failed to write to file: {0}")]
    Write(#[source] std::io::Error),
}

#[derive(Debug, Default)]
pub enum StoringType {
    Cache,
    #[default]
    Data,
    Config,
}

pub trait Storing: Clone + Serialize + for<'de> Deserialize<'de> + Debug + Default {
    fn store_type() -> StoringType;
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct StoreHandle<T> {
    store_id: String,
    store: T,
}

impl<T: Storing> StoreHandle<T> {
    pub fn new(store_id: &str) -> Self {
        Self {
            store: T::default(),
            store_id: store_id.to_owned(),
        }
    }

    fn set_store(&mut self, store: T) {
        debug!("Setting store with id: {}", self.store_id);
        self.store = store;
    }

    /// Returns a mutable reference to the stored data.
    pub fn get_store_mut(&mut self) -> &mut T {
        &mut self.store
    }

    /// Returns a reference to the stored data.
    pub fn get_store(&self) -> &T {
        &self.store
    }

    pub fn store_id(&self) -> &str {
        &self.store_id
    }
}

/// Handles file system paths for reading from and writing to data storage.
///
/// The `Storage` struct provides a way to manage file paths used for storing data in different locations.
/// It simplifies the process of accessing and modifying data by providing methods for these operations.
///
/// # Example
///
/// ```
/// use serde::{Deserialize, Serialize};
/// use storage::{Storage, StoreHandle, Storing, StoringType};
///
/// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
/// pub struct MyStore {
///     pub count: u32,
/// }
///
/// impl MyStore {
///     fn increment_count(&mut self) {
///         self.count += 1;
///     }
/// }
///
/// impl Storing for MyStore {
///     fn store_type() -> StoringType {
///         StoringType::Data
///     }
/// }
///
/// // Initialize the Storage with the defaults
/// let storage = Storage::new("com.github.mazynoah.storage".to_owned());
///
/// // Create a handle for managing the store data.
/// let mut handle = StoreHandle::<MyStore>::new("handle");
///
/// // Read existing store from storage
/// storage
///     .read(&mut handle)
///     .expect("Failed to read from storage");
///
/// // Modify the store data
/// let counter = handle.get_store_mut();
///
/// counter.increment_count();
/// counter.increment_count();
/// counter.increment_count();
///
/// // Write changes to disk
/// storage
///     .write(&mut handle)
///     .expect("Failed to write to storage");
///
/// let counter = handle.get_store();
///
/// println!("Count: {}", counter.count);
///
///
/// ```
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Storage {
    cache_dir: PathBuf,
    data_dir: PathBuf,
    config_dir: PathBuf,
}

impl Storage {
    /// Creates a new `Storage` instance by obtaining the paths for cache, data, and configuration directories.
    ///
    /// # Panics
    ///
    /// - Panics if the cache directory, data directory, or configuration directory path cannot be determined.
    pub fn new(app_id: String) -> Self {
        Self {
            data_dir: dirs::data_dir()
                .expect("Failed to determine cache directory path")
                .join(&app_id),
            config_dir: dirs::config_dir()
                .expect("Failed to determine data directory path")
                .join(&app_id),
            cache_dir: dirs::cache_dir()
                .expect("Failed to determine configuration directory path")
                .join(&app_id),
        }
    }

    /// Creates a new `Storage` instance by obtaining the paths for cache, data, and configuration directories.
    pub fn from(cache_dir: PathBuf, data_dir: PathBuf, config_dir: PathBuf) -> Self {
        Self {
            cache_dir,
            data_dir,
            config_dir,
        }
    }

    /// Reads the store from a file and updates the provided `StoreHandle`.
    /// If the file does not exist, it creates a default store if a default is available.
    ///
    /// # Example
    ///
    /// ```
    /// let storage = Storage::new(app);
    /// let handle: StoreHandle<MyStore> = StoreHandle::new("my_store_id");
    ///
    /// storage.read(&mut handle).expect("Failed to read store");
    ///
    /// ```
    pub fn read<T: Storing>(&self, handle: &mut StoreHandle<T>) -> Result<(), StoreError> {
        debug!("Reading store with id: {}", handle.store_id());
        self.open_file::<T, _>(
            |file, handle| {
                let store = Self::read_string(file).map_err(StoreError::Read)?;
                let store_data: T = ron::from_str(&store).map_err(StoreError::RonParse)?;

                handle.set_store(store_data);

                info!("Successfully read store with id: {}", handle.store_id());
                Ok(())
            },
            handle,
        )
    }

    /// Writes the current store `T` from the provided `StoreHandle` to a file.
    /// If the file does not exist, it creates a default store if a default is available.
    ///
    /// # Example
    ///
    /// ```
    /// let storage = Storage::new(app);
    /// let handle: StoreHandle<MyStore> = StoreHandle::new("my_store_id");
    ///
    /// storage.write(&mut handle).expect("Failed to read store");
    ///
    /// ```
    pub fn write<T: Storing>(&self, handle: &mut StoreHandle<T>) -> Result<(), StoreError> {
        debug!("Writing store with id: {}", handle.store_id());
        self.open_file::<T, _>(
            |file: &mut File, handle| {
                let store = handle.get_store_mut();

                let str =
                    ron::ser::to_string_pretty(&store, PrettyConfig::new().compact_arrays(true))
                        .map_err(StoreError::Ron)?;

                file.write(str.as_bytes()).map_err(StoreError::Write)?;

                info!("Successfully wrote store with id: {}", handle.store_id());
                Ok(())
            },
            handle,
        )
    }

    /// Opens the file for reading or writing. If the file does not exist, it attempts
    /// to create a default store if a default is provided.
    ///
    /// # Example
    ///
    /// ```rust
    /// storage.open_file::<MyStore, _>(|file, handle| {
    ///     // Perform file operations
    ///     Ok(())
    /// }, &mut handle);
    /// ```
    fn open_file<T, F>(
        &self,
        mut operation: F,
        handle: &mut StoreHandle<T>,
    ) -> Result<(), StoreError>
    where
        T: Storing,
        F: FnMut(&mut File, &mut StoreHandle<T>) -> Result<(), StoreError>,
    {
        let mut dir_path = self.dir_path::<T>();
        dir_path.push(handle.store_id()); // i don't like this

        debug!("Opening file at path: {:?}", dir_path);

        match OpenOptions::new()
            .read(true)
            .write(true)
            .create(false)
            .truncate(false)
            .open(&dir_path)
        {
            Ok(mut config) => {
                debug!("File opened successfully at path: {:?}", dir_path);
                operation(&mut config, handle)
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                warn!(
                    "File not found at path: {:?}, creating default store",
                    dir_path
                );
                self.store_default::<T>(dir_path)?;
                self.open_file(operation, handle)
            }
            Err(err) => {
                warn!(
                    "Failed to open file at path: {:?}, error: {:?}",
                    dir_path, err
                );
                Err(StoreError::FileOpen(err))
            }
        }
    }

    fn store_default<T: Storing>(&self, path: PathBuf) -> Result<(), StoreError> {
        debug!("Storing default configuration at path: {:?}", path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(StoreError::CreateDir)?;
            info!("Created directory for path: {:?}", parent);
        }

        let default_store = T::default();
        let str = ron::ser::to_string_pretty(&default_store, PrettyConfig::new())
            .map_err(StoreError::Ron)?;
        fs::write(&path, str).map_err(StoreError::Write)?;
        info!("Default store written at path: {:?}", &path);

        Ok(())
    }

    fn dir_path<T: Storing>(&self) -> PathBuf {
        let path = match T::store_type() {
            StoringType::Cache => self.cache_dir.clone(),
            StoringType::Data => self.data_dir.clone(),
            StoringType::Config => self.config_dir.clone(),
        };
        debug!(
            "Resolved directory path for store type: {:?} to path: {:?}",
            T::store_type(),
            path
        );
        path
    }

    fn read_string(mut file: &File) -> Result<String, std::io::Error> {
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        debug!("Read string from file, length: {}", buf.len());
        Ok(buf)
    }
}

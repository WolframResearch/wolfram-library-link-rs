use static_assertions::assert_not_impl_any;

use crate::sys::{self, mint};


/// Storage for heterogenous data.
///
/// # Example
///
/// ```no_run
/// use wl_library_link::DataStore;
///
/// let mut data = DataStore::new();
///
/// data.add_i64(1);
/// ```
#[derive(Debug)]
pub struct DataStore(sys::DataStore);

// TODO: Implement Clone for this type using the appropriate RTL callback.
assert_not_impl_any!(DataStore: Copy, Clone);

impl DataStore {
    /// Create an empty [`DataStore`].
    ///
    /// *LibraryLink C Function:* [`createDataStore`][sys::st_WolframIOLibrary_Functions::createDataStore].
    pub fn new() -> Self {
        let io_funcs = unsafe { *crate::get_library_data().ioLibraryFunctions };

        let create_data_store: unsafe extern "C" fn() -> sys::DataStore = io_funcs
            .createDataStore
            .expect("createDataStore callback is NULL");

        let ds: sys::DataStore = unsafe { create_data_store() };

        DataStore(ds)
    }

    /// Add an `i64` value to this `DataStore`.
    ///
    /// *LibraryLink C Function:* [`DataStore_addInteger`][sys::st_WolframIOLibrary_Functions::DataStore_addInteger].
    pub fn add_i64(&mut self, value: i64) {
        let DataStore(ds) = *self;

        let io_funcs = unsafe { *crate::get_library_data().ioLibraryFunctions };

        let data_store_add_integer: unsafe extern "C" fn(sys::DataStore, mint) = io_funcs
            .DataStore_addInteger
            .expect("DataStore_addInteger callback is NULL");

        unsafe { data_store_add_integer(ds, value) }
    }

    /// Convert this `DataStore` into a raw [`wl_library_link_sys::DataStore`] pointer.
    pub fn into_ptr(self) -> sys::DataStore {
        let DataStore(ds) = self;
        ds
    }
}

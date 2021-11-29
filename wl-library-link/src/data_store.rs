use static_assertions::assert_not_impl_any;

use crate::{rtl, sys};


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
        let ds: sys::DataStore = unsafe { rtl::createDataStore() };

        DataStore(ds)
    }

    /// Convert this `DataStore` into a raw [`wl_library_link_sys::DataStore`] pointer.
    pub fn into_raw(self) -> sys::DataStore {
        let DataStore(ds) = self;
        ds
    }

    /// Add an `i64` value to this `DataStore`.
    ///
    /// *LibraryLink C Function:* [`DataStore_addInteger`][sys::st_WolframIOLibrary_Functions::DataStore_addInteger].
    pub fn add_i64(&mut self, value: i64) {
        let DataStore(ds) = *self;

        unsafe { rtl::DataStore_addInteger(ds, value) }
    }
}

use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};


use static_assertions::assert_not_impl_any;

use crate::{rtl, sys, NumericArray};


/// Storage for heterogenous expression-like data.
///
/// `DataStore` can be used to pass expression-like structures via *LibraryLink* functions.
///
/// `DataStore` can be used as an argument or return type in a *LibraryLink* function
/// exposed via [`export![]`][crate::export].
///
/// # Example
///
/// The following `DataStore` expression:
///
/// ```wolfram
/// Developer`DataStore[1, "hello", False]
/// ```
///
/// can be created using the Rust code:
///
/// ```no_run
/// use wolfram_library_link::DataStore;
///
/// let mut data = DataStore::new();
///
/// data.add_i64(1);
/// data.add_str("hello");
/// data.add_bool(false);
/// ```
#[derive(Debug)]
pub struct DataStore(sys::DataStore);

assert_not_impl_any!(DataStore: Copy);

impl DataStore {
    /// Create an empty [`DataStore`].
    ///
    /// *LibraryLink C Function:* [`createDataStore`][rtl::createDataStore].
    pub fn new() -> Self {
        let ds: sys::DataStore = unsafe { rtl::createDataStore() };

        if ds.is_null() {
            panic!("sys::DataStore is NULL");
        }

        DataStore(ds)
    }

    /// Returns the number of elements in this data store.
    ///
    /// *LibraryLink C Function:* [`DataStore_getLength`][rtl::DataStore_getLength].
    pub fn len(&self) -> usize {
        let DataStore(ds) = *self;

        let len: i64 = unsafe { rtl::DataStore_getLength(ds) };

        usize::try_from(len).expect("DataStore i64 length overflows usize")
    }

    /// Construct a `DataStore` from a raw [`wolfram_library_link_sys::DataStore`] pointer.
    pub unsafe fn from_raw(raw: sys::DataStore) -> Self {
        DataStore(raw)
    }

    /// Convert this `DataStore` into a raw [`wolfram_library_link_sys::DataStore`] pointer.
    pub fn into_raw(self) -> sys::DataStore {
        let DataStore(ds) = self;

        // Don't run Drop on `self`; ownership of this value is being given to the caller.
        std::mem::forget(self);

        ds
    }

    //==================================
    // Unnamed data
    //==================================

    /// Add a `bool` value to this `DataStore`.
    ///
    /// *LibraryLink C Function:* [`DataStore_addBoolean`][rtl::DataStore_addBoolean].
    pub fn add_bool(&mut self, value: bool) {
        let DataStore(ds) = *self;

        unsafe { rtl::DataStore_addBoolean(ds, sys::mbool::from(value)) }
    }

    /// Add an `i64` value to this `DataStore`.
    ///
    /// *LibraryLink C Function:* [`DataStore_addInteger`][rtl::DataStore_addBoolean].
    pub fn add_i64(&mut self, value: i64) {
        let DataStore(ds) = *self;

        unsafe { rtl::DataStore_addInteger(ds, value) }
    }

    /// Add an `f64` value to this `DataStore`.
    ///
    /// *LibraryLink C Function:* [`DataStore_addReal`][rtl::DataStore_addReal].
    pub fn add_f64(&mut self, value: f64) {
        let DataStore(ds) = *self;

        unsafe { rtl::DataStore_addReal(ds, value) }
    }

    /// Add an [`mcomplex`][sys::mcomplex] value to this `DataStore`.
    ///
    /// *LibraryLink C Function:* [`DataStore_addComplex`][rtl::DataStore_addComplex].
    pub fn add_complex_f64(&mut self, value: sys::mcomplex) {
        let DataStore(ds) = *self;

        unsafe { rtl::DataStore_addComplex(ds, value) }
    }

    /// Add a [`str`] value to this `DataStore`.
    ///
    /// See also: [`DataStore::add_c_str()`].
    ///
    /// *LibraryLink C Function:* [`DataStore_addString`][rtl::DataStore_addString].
    pub fn add_str(&mut self, value: &str) {
        let DataStore(ds) = *self;

        let value = CString::new(value).expect("could not convert &str to CString");

        unsafe { rtl::DataStore_addString(ds, value.as_ptr() as *mut c_char) }
    }

    /// Add a [`CStr`] value to this `DataStore`.
    ///
    /// See also: [`DataStore::add_str()`].
    ///
    /// *LibraryLink C Function:* [`DataStore_addString`][rtl::DataStore_addString].
    pub fn add_c_str(&mut self, value: &CStr) {
        let DataStore(ds) = *self;

        unsafe { rtl::DataStore_addString(ds, value.as_ptr() as *mut c_char) }
    }

    /// Add a `DataStore` value to this `DataStore`.
    ///
    /// *LibraryLink C Function:* [`DataStore_addDataStore`][rtl::DataStore_addDataStore].
    ///
    /// # Example
    ///
    /// The `DataStore` value constructed by the following code:
    ///
    /// ```no_run
    /// use wolfram_library_link::DataStore;
    ///
    /// let mut inner = DataStore::new();
    /// inner.add_i64(0);
    /// let mut outer = DataStore::new();
    /// outer.add_data_store(inner);
    /// ```
    ///
    /// will have this representation when passed via LibraryLink into Wolfram Language:
    ///
    /// ```wolfram
    /// Developer`DataStore[Developer`DataStore[0]]
    /// ```
    pub fn add_data_store(&mut self, ds: DataStore) {
        let DataStore(this_ds) = *self;
        // Use into_raw() to avoid running Drop on `ds`.
        let other_ds = ds.into_raw();

        unsafe { rtl::DataStore_addDataStore(this_ds, other_ds) }
    }

    /// Add a [`NumericArray`] value to this `DataStore`.
    ///
    /// *LibraryLink C Function:* [`DataStore_addMNumericArray`][rtl::DataStore_addMNumericArray].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use wolfram_library_link::{DataStore, NumericArray};
    ///
    /// let array: NumericArray<i64> = NumericArray::from_slice(&[1, 2, 3]);
    ///
    /// let mut store = DataStore::new();
    ///
    /// // Erase the NumericArray data type.
    /// store.add_numeric_array(array.into_generic());
    /// ```
    ///
    /// See also: [`NumericArray::into_generic()`].
    pub fn add_numeric_array(&mut self, array: NumericArray) {
        let DataStore(ds) = *self;
        let array = unsafe { array.into_raw() };

        unsafe { rtl::DataStore_addMNumericArray(ds, array) }
    }

    //==================================
    // Named data
    //==================================

    /// Add a `bool` value to this `DataStore`.
    ///
    /// *LibraryLink C Function:* [`DataStore_addNamedBoolean`][rtl::DataStore_addNamedBoolean].
    pub fn add_named_bool(&mut self, name: &str, value: bool) {
        let DataStore(ds) = *self;

        let name = CString::new(name).expect("could not convert &str to CString");

        unsafe {
            rtl::DataStore_addNamedBoolean(
                ds,
                name.as_ptr() as *mut c_char,
                sys::mbool::from(value),
            )
        }
    }

    /// Add an `i64` value to this `DataStore`.
    ///
    /// *LibraryLink C Function:* [`DataStore_addNamedInteger`][rtl::DataStore_addNamedBoolean].
    pub fn add_named_i64(&mut self, name: &str, value: i64) {
        let DataStore(ds) = *self;

        let name = CString::new(name).expect("could not convert &str to CString");

        unsafe { rtl::DataStore_addNamedInteger(ds, name.as_ptr() as *mut c_char, value) }
    }

    /// Add an `f64` value to this `DataStore`.
    ///
    /// *LibraryLink C Function:* [`DataStore_addNamedReal`][rtl::DataStore_addNamedReal].
    pub fn add_named_f64(&mut self, name: &str, value: f64) {
        let DataStore(ds) = *self;

        let name = CString::new(name).expect("could not convert &str to CString");

        unsafe { rtl::DataStore_addNamedReal(ds, name.as_ptr() as *mut c_char, value) }
    }

    /// Add an [`mcomplex`][sys::mcomplex] value to this `DataStore`.
    ///
    /// *LibraryLink C Function:* [`DataStore_addNamedComplex`][rtl::DataStore_addNamedComplex].
    pub fn add_named_complex_f64(&mut self, name: &str, value: sys::mcomplex) {
        let DataStore(ds) = *self;

        let name = CString::new(name).expect("could not convert &str to CString");

        unsafe { rtl::DataStore_addNamedComplex(ds, name.as_ptr() as *mut c_char, value) }
    }

    /// Add a [`str`] value to this `DataStore`.
    ///
    /// See also: [`DataStore::add_c_str()`].
    ///
    /// *LibraryLink C Function:* [`DataStore_addNamedString`][rtl::DataStore_addNamedString].
    pub fn add_named_str(&mut self, name: &str, value: &str) {
        let DataStore(ds) = *self;

        let name = CString::new(name).expect("could not convert &str to CString");
        let value = CString::new(value).expect("could not convert &str to CString");

        unsafe {
            rtl::DataStore_addNamedString(
                ds,
                name.as_ptr() as *mut c_char,
                value.as_ptr() as *mut c_char,
            )
        }
    }

    /// Add a [`CStr`] value to this `DataStore`.
    ///
    /// See also: [`DataStore::add_str()`].
    ///
    /// *LibraryLink C Function:* [`DataStore_addNamedString`][rtl::DataStore_addNamedString].
    pub fn add_named_c_str(&mut self, name: &str, value: &CStr) {
        let DataStore(ds) = *self;

        let name = CString::new(name).expect("could not convert &str to CString");

        unsafe {
            rtl::DataStore_addNamedString(
                ds,
                name.as_ptr() as *mut c_char,
                value.as_ptr() as *mut c_char,
            )
        }
    }

    /// Add a `DataStore` value to this `DataStore`.
    ///
    /// *LibraryLink C Function:* [`DataStore_addNamedDataStore`][rtl::DataStore_addNamedDataStore].
    ///
    /// # Example
    ///
    /// The `DataStore` value constructed by the following code:
    ///
    /// ```no_run
    /// use wolfram_library_link::DataStore;
    ///
    /// let mut inner = DataStore::new();
    /// inner.add_i64(0);
    /// let mut outer = DataStore::new();
    /// outer.add_named_data_store("inner", inner);
    /// ```
    ///
    /// will have this representation when passed via LibraryLink into Wolfram Language:
    ///
    /// ```wolfram
    /// Developer`DataStore["inner" -> Developer`DataStore[0]]
    /// ```
    pub fn add_named_data_store(&mut self, name: &str, ds: DataStore) {
        let DataStore(this_ds) = *self;
        // Use into_raw() to avoid running Drop on `ds`.
        let other_ds = ds.into_raw();

        let name = CString::new(name).expect("could not convert &str to CString");

        unsafe {
            rtl::DataStore_addNamedDataStore(
                this_ds,
                name.as_ptr() as *mut c_char,
                other_ds,
            )
        }
    }

    /// Add a [`NumericArray`] value to this `DataStore`.
    ///
    /// See also [`DataStore::add_numeric_array()`].
    ///
    /// *LibraryLink C Function:* [`DataStore_addNamedMNumericArray`][rtl::DataStore_addNamedMNumericArray].
    pub fn add_named_numeric_array(&mut self, name: &str, array: NumericArray) {
        let DataStore(ds) = *self;
        let array = unsafe { array.into_raw() };

        let name = CString::new(name).expect("could not convert &str to CString");

        unsafe {
            rtl::DataStore_addNamedMNumericArray(ds, name.as_ptr() as *mut c_char, array)
        }
    }
}

//======================================
// Clone and Drop Impls
//======================================

impl Clone for DataStore {
    fn clone(&self) -> DataStore {
        let DataStore(ds) = *self;

        let duplicate = unsafe { rtl::copyDataStore(ds) };

        DataStore(duplicate)
    }
}

impl Drop for DataStore {
    fn drop(&mut self) {
        let DataStore(ds) = *self;
        let ds: sys::DataStore = ds;

        unsafe { rtl::deleteDataStore(ds) }
    }
}

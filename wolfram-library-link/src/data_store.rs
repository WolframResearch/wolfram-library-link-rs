use std::{
    ffi::{CStr, CString},
    fmt,
    marker::PhantomData,
    os::raw::c_char,
};


use once_cell::sync::OnceCell;
use static_assertions::assert_not_impl_any;

use crate::{
    rtl,
    sys::{self, mcomplex, mint, mreal},
    FromArg, Image, NumericArray,
};


/// Storage for heterogenous expression-like data.
///
/// `DataStore` can be used to pass expression-like structures via *LibraryLink* functions.
///
/// `DataStore` can be used as an argument or return type in a *LibraryLink* function
/// exposed via [`#[export]`][crate::export].
///
/// Use [`DataStore::nodes()`] to get an iterator over the [`DataStoreNode`]s contained
/// in this `DataStore`.
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
// TODO: Provide better Debug formatting for this type.
#[derive(Debug)]
#[derive(ref_cast::RefCast)]
#[repr(transparent)]
pub struct DataStore(sys::DataStore);

assert_not_impl_any!(DataStore: Copy);

/// Element borrowed from the linked list of nodes that make up a [`DataStore`].
///
/// # Lifetime `'store`
///
/// A `DataStoreNode` is borrowed from a [`DataStore`], and cannot outlive the `DataStore`
/// it was borrowed from:
///
/// ```compile_fail
/// # use wolfram_library_link::DataStore;
/// let mut store = DataStore::new();
/// store.add_named_i64("value", 5);
///
/// let node = store.first_node().unwrap();
///
/// drop(store);
///
/// // Error: `node` cannot outlive `store`.
/// println!("node name: {:?}", node.name());
/// ```
pub struct DataStoreNode<'store> {
    raw: sys::DataStoreNode,
    marker: PhantomData<&'store DataStore>,
    /// Private.
    ///
    /// This field is a cache that allows us to return a `&MArgument` reference that has
    /// the same lifetime as the `self` node, which is ultimately needed as the lifetime
    /// for references stored in [`DataStoreNodeValue`].
    ///
    /// We'd prefer to be able to directly return a reference to the "inner" field stored
    /// on the `sys::DataStoreNode` opaque type, but we're not able to access that field
    /// directly. Instead, we just cache the value of that field (accessed using
    /// [`rtl::DataStoreNode_getData`]). Caching the value is valid because `DataStoreNode`
    /// values are immutable.
    data: OnceCell<sys::MArgument>,
}

/// Value of a [`DataStoreNode`].
///
/// Instances of this type are returned by [`DataStoreNode::value()`].
///
/// [`DataStoreNode`]s can contain any value that can be stored in an
/// [`MArgument`][sys::MArgument].
///
// TODO: Rename this to `ArgValue`, as this is based on `MArgument`?
#[allow(missing_docs)]
pub enum DataStoreNodeValue<'node> {
    Boolean(bool),
    Integer(mint),
    Real(mreal),
    Complex(mcomplex),
    Str(&'node str),
    NumericArray(&'node NumericArray),
    Image(&'node Image),
    DataStore(&'node DataStore),
}

/// Iterator over the [`DataStoreNode`]s stored in a [`DataStore`].
///
/// Instances of this type are returned by [`DataStore::nodes()`].
///
/// # Example
///
/// ```no_run
/// # use wolfram_library_link::DataStore;
/// let mut store = DataStore::new();
///
/// store.add_i64(5);
/// store.add_named_bool("condition", true);
/// store.add_str("Hello, World!");
///
/// for node in store.nodes() {
///     println!("node: {:?}", node);
/// }
/// ```
///
/// prints:
///
/// ```text
/// node: DataStoreNode { name: None, value: 5 }
/// node: DataStoreNode { name: Some("condition"), value: true }
/// node: DataStoreNode { name: None, value: "Hello, World!" }
/// ```
pub struct Nodes<'s> {
    node: Option<DataStoreNode<'s>>,
}

//======================================
// Impls
//======================================

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

    /// Returns an iterator over the [`DataStoreNode`]s of this `DataStore`.
    ///
    /// A [`DataStore`] is made up of a linked list of [`DataStoreNode`]s. The [`Nodes`]
    /// iterator will repeatedly call the [`DataStoreNode::next_node()`] method to iterate
    /// over the nodes.
    pub fn nodes<'s>(&'s self) -> Nodes<'s> {
        Nodes {
            node: self.first_node(),
        }
    }

    /// Get the first [`DataStoreNode`] of this `DataStore`.
    pub fn first_node<'s>(&'s self) -> Option<DataStoreNode<'s>> {
        let DataStore(raw) = *self;

        let node = unsafe { rtl::DataStore_getFirstNode(raw) };

        if node.is_null() {
            return None;
        }

        Some(DataStoreNode {
            raw: node,
            marker: PhantomData,
            data: OnceCell::new(),
        })
    }

    // Note: No `last_node()` method is provided to wrap the DataStoreNode_getLastNode()
    //       function, because it would have no purpose, since there is no way to get the
    //       previous node.
    // pub fn last_node(&self) -> DataStoreNode { ... }
}

//--------------
// DataStoreNode
//--------------

impl<'store> DataStoreNode<'store> {
    /// Get the name associated with this node, if any.
    ///
    /// *LibraryLink C Function:* [`DataStoreNode_getName`][rtl::DataStoreNode_getName].
    pub fn name(&self) -> Option<String> {
        // TODO: Do we need to free this string, or does getName() just return a
        //       borrwed reference?
        let mut raw_c_str: *mut c_char = std::ptr::null_mut();

        let err_code: sys::errcode_t =
            unsafe { rtl::DataStoreNode_getName(self.raw, &mut raw_c_str) };

        if err_code != 0 || raw_c_str.is_null() {
            return None;
        }

        let c_str = unsafe { CStr::from_ptr(raw_c_str) };

        let str: &str = c_str.to_str().ok()?;

        Some(str.to_owned())
    }

    /// Get the value stored in this `DataStoreNode`.
    ///
    /// This is a safe wrapper around [`DataStoreNode::data_raw()`].
    pub fn value<'node>(&'node self) -> DataStoreNodeValue<'node> {
        use DataStoreNodeValue as V;

        let data_raw: &'node sys::MArgument = unsafe { self.data_raw() };

        unsafe {
            match self.data_type_raw() as u32 {
                sys::MType_Undef => panic!("unexpected DataStoreNode Undef data type"),
                sys::MType_Boolean => V::Boolean(bool::from_arg(data_raw)),
                sys::MType_Integer => V::Integer(mint::from_arg(data_raw)),
                sys::MType_Real => V::Real(mreal::from_arg(data_raw)),
                sys::MType_Complex => V::Complex(mcomplex::from_arg(data_raw)),
                sys::MType_UTF8String => V::Str(<&str>::from_arg(data_raw)),
                sys::MType_Tensor => {
                    unimplemented!("unhandled DataStoreNode Tensor data type")
                },
                sys::MType_SparseArray => {
                    unimplemented!("unhandled DataStoreNode SparseArray data type")
                },
                sys::MType_NumericArray => {
                    V::NumericArray(<&NumericArray>::from_arg(data_raw))
                },
                sys::MType_Image => V::Image(<&Image>::from_arg(data_raw)),
                sys::MType_DataStore => V::DataStore(<&DataStore>::from_arg(data_raw)),
                type_ => {
                    panic!("unexpected DataStoreNode::data_type_raw() value: {}", type_)
                },
            }
        }
    }

    /// Get the next node in this linked list of `DataStoreNode`'s.
    ///
    /// *LibraryLink C Function:* [`DataStoreNode_getNextNode`][rtl::DataStoreNode_getNextNode].
    pub fn next_node(&self) -> Option<DataStoreNode<'store>> {
        let raw_next: sys::DataStoreNode =
            unsafe { rtl::DataStoreNode_getNextNode(self.raw) };

        if raw_next.is_null() {
            return None;
        }

        Some(DataStoreNode {
            raw: raw_next,
            marker: PhantomData,
            data: OnceCell::new(),
        })
    }

    /// *LibraryLink C Function:* [`DataStoreNode_getDataType`][rtl::DataStoreNode_getDataType].
    pub fn data_type_raw(&self) -> sys::type_t {
        unsafe { rtl::DataStoreNode_getDataType(self.raw) }
    }

    /// *LibraryLink C Function:* [`DataStoreNode_getData`][rtl::DataStoreNode_getData].
    pub unsafe fn data_raw(&self) -> &sys::MArgument {
        match self.try_data_raw() {
            Ok(value) => value,
            Err(code) => panic!(
                "DataStoreNode::data_raw: failed to get data (error code: {})",
                code
            ),
        }
    }

    /// *LibraryLink C Function:* [`DataStoreNode_getData`][rtl::DataStoreNode_getData].
    pub unsafe fn try_data_raw<'node>(
        &'node self,
    ) -> Result<&'node sys::MArgument, sys::errcode_t> {
        self.data
            .get_or_try_init(|| -> Result<sys::MArgument, sys::errcode_t> {
                let mut arg: sys::MArgument = sys::MArgument {
                    integer: std::ptr::null_mut(),
                };

                let err_code: sys::errcode_t =
                    rtl::DataStoreNode_getData(self.raw, &mut arg);

                if err_code != 0 {
                    return Err(err_code);
                }

                Ok(arg)
            })
    }
}

//---------------
// DataStoreAdd trait
//---------------

pub trait DataStoreAdd {
    fn add_to_datastore(&self, ds: &mut DataStore);
}

impl DataStoreAdd for bool {
    fn add_to_datastore(&self, ds: &mut DataStore) {
        ds.add_bool(*self)
    }
}

impl DataStoreAdd for &str {
    fn add_to_datastore(&self, ds: &mut DataStore) {
        ds.add_str(*self)
    }
}

impl DataStoreAdd for char {
    fn add_to_datastore(&self, ds: &mut DataStore) {
        String::from(*self).as_str().add_to_datastore(ds)
    }
}

impl DataStoreAdd for u16 {
    fn add_to_datastore(&self, ds: &mut DataStore) {
        ds.add_i64(*self as i64)
    }
}

impl DataStoreAdd for DataStore {
    fn add_to_datastore(&self, ds: &mut DataStore) {
        ds.add_data_store(self.clone())
    }
}

impl<T: DataStoreAdd> DataStoreAdd for Vec<T> {
    fn add_to_datastore(&self, ds: &mut DataStore) {
        let mut inner = DataStore::new();
        self.into_iter().for_each(|item| item.add_to_datastore(&mut inner));
        ds.add_data_store(inner)
    }
}

//---------------
// From DataStoreNodeValue
//---------------

impl From<DataStoreNodeValue<'_>> for bool {
    fn from(value: DataStoreNodeValue) -> bool {
        match value {
            DataStoreNodeValue::Boolean(val) => val,
            _ => panic!("expected DataStoreNodeValue::Boolean"),
        }
    }
}

impl From<DataStoreNodeValue<'_>> for mint {
    fn from(value: DataStoreNodeValue) -> mint {
        match value {
            DataStoreNodeValue::Integer(val) => val,
            _ => panic!("expected DataStoreNodeValue::Integer"),
        }
    }
}

impl From<DataStoreNodeValue<'_>> for mreal {
    fn from(value: DataStoreNodeValue) -> mreal {
        match value {
            DataStoreNodeValue::Real(val) => val,
            _ => panic!("expected DataStoreNodeValue::Real"),
        }
    }
}

impl From<DataStoreNodeValue<'_>> for f32 {
    fn from(value: DataStoreNodeValue) -> f32 {
        match value {
            DataStoreNodeValue::Real(val) => val as f32,
            _ => panic!("expected DataStoreNodeValue::Real"),
        }
    }
}

impl From<DataStoreNodeValue<'_>> for mcomplex {
    fn from(value: DataStoreNodeValue) -> mcomplex {
        match value {
            DataStoreNodeValue::Complex(val) => val,
            _ => panic!("expected DataStoreNodeValue::Complex"),
        }
    }
}

impl<'node> From<DataStoreNodeValue<'node>> for &'node str {
    fn from(value: DataStoreNodeValue<'node>) -> &'node str {
        match value {
            DataStoreNodeValue::Str(val) => val,
            _ => panic!("expected DataStoreNodeValue::Str"),
        }
    }
}

impl<'node> From<DataStoreNodeValue<'node>> for String {
    fn from(value: DataStoreNodeValue<'node>) -> String {
        match value {
            DataStoreNodeValue::Str(val) => String::from(val),
            _ => panic!("expected DataStoreNodeValue::Str"),
        }
    }
}

impl From<DataStoreNodeValue<'_>> for char {
    fn from(value: DataStoreNodeValue) -> char {
        match value {
            DataStoreNodeValue::Str(val) => String::from(val).chars().next().unwrap(),
            _ => panic!("expected DataStoreNodeValue::Str"),
        }
    }
}

impl<'node> From<DataStoreNodeValue<'node>> for &'node NumericArray {
    fn from(value: DataStoreNodeValue<'node>) -> &'node NumericArray {
        match value {
            DataStoreNodeValue::NumericArray(val) => val,
            _ => panic!("expected DataStoreNodeValue::NumericArray"),
        }
    }
}

impl<'node> From<DataStoreNodeValue<'node>> for &'node Image {
    fn from(value: DataStoreNodeValue<'node>) -> &'node Image {
        match value {
            DataStoreNodeValue::Image(val) => val,
            _ => panic!("expected DataStoreNodeValue::Image"),
        }
    }
}

impl<'node> From<DataStoreNodeValue<'node>> for &'node DataStore {
    fn from(value: DataStoreNodeValue<'node>) -> &'node DataStore {
        match value {
            DataStoreNodeValue::DataStore(val) => val,
            _ => panic!("expected DataStoreNodeValue::DataStore"),
        }
    }
}

impl<'node, T: for<'a> From<DataStoreNodeValue<'a>>> From<DataStoreNodeValue<'node>> for Vec<T> {
    fn from(value: DataStoreNodeValue<'node>) -> Vec<T> {
        match value {
            DataStoreNodeValue::DataStore(val) => val.nodes().map(|n| n.value().into()).collect(),
            _ => panic!("expected DataStoreNodeValue::DataStore"),
        }
    }
}

//---------------
// Nodes iterator
//---------------

impl<'store> Iterator for Nodes<'store> {
    type Item = DataStoreNode<'store>;

    fn next(&mut self) -> Option<Self::Item> {
        let Nodes { node } = self;

        let curr = node.take()?;

        *node = curr.next_node();

        Some(curr)
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

//======================================
// Formatting Impls
//======================================

impl<'store> fmt::Debug for DataStoreNode<'store> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DataStoreNode")
            .field("name", &self.name())
            .field("value", &self.value())
            // TODO: Add an enum to wrap the raw data type and use that here instead.
            // .field("data_type_raw", &self.data_type_raw())
            .finish()
    }
}

impl<'node> fmt::Debug for DataStoreNodeValue<'node> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DataStoreNodeValue as V;

        match self {
            V::Boolean(val) => val.fmt(f),
            V::Integer(val) => val.fmt(f),
            V::Real(val) => val.fmt(f),
            V::Complex(val) => val.fmt(f),
            V::Str(val) => val.fmt(f),
            V::NumericArray(val) => val.fmt(f),
            V::Image(val) => val.fmt(f),
            V::DataStore(val) => val.fmt(f),
        }
    }
}

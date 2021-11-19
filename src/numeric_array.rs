use std::ffi::c_void;
use std::mem::MaybeUninit;

use crate::sys;

/// Extremely basic wrapper around raw MNumericArray. Basically only suitable for working
/// with ByteArray[]'s (for serializing/deserializing WXF).
pub struct NumericArray(sys::MNumericArray);

impl NumericArray {
    pub unsafe fn from_raw(array: sys::MNumericArray) -> Self {
        NumericArray(array)
    }

    pub unsafe fn into_raw(self) -> sys::MNumericArray {
        self.0
    }

    /// *LibraryLink C API Documentation:* [`MNumericArray_getData`](https://reference.wolfram.com/language/LibraryLink/ref/callback/MNumericArray_getData.html)
    pub fn data_ptr(&self) -> *mut c_void {
        let NumericArray(numeric_array) = *self;

        unsafe {
            let getter: unsafe extern "C" fn(*mut sys::st_MNumericArray) -> *mut c_void =
                (*crate::get_library_data().numericarrayLibraryFunctions)
                    .MNumericArray_getData
                    .expect("MNumericArray_getData callback is NULL");

            getter(numeric_array)
        }
    }

    /// # Safety
    ///
    /// This method must only be called when it's assured that the data contained by this
    /// NumericArray has been initialized.
    pub unsafe fn data_bytes(&self) -> &[u8] {
        let data_ptr: *mut c_void = self.data_ptr();
        let data_ptr = data_ptr as *mut u8;

        std::slice::from_raw_parts(data_ptr, self.length_in_bytes())
    }

    pub unsafe fn data_bytes_mut(&mut self) -> &mut [MaybeUninit<u8>] {
        let data_ptr: *mut c_void = self.data_ptr();
        let data_ptr = data_ptr as *mut MaybeUninit<u8>;

        std::slice::from_raw_parts_mut(data_ptr, self.length_in_bytes())
    }

    pub fn length_in_bytes(&self) -> usize {
        let dims = self.dimensions();

        let length: i64 = dims.iter().product();
        let length =
            usize::try_from(length).expect("NumericArray length overflows usize");

        // FIXME: This should multiple `length` by the size-in-bytes of
        //        st_MNumericArray.tensor_property_type.
        length
    }

    /// *LibraryLink C API Documentation:* [`MNumericArray_getRank`](https://reference.wolfram.com/language/LibraryLink/ref/callback/MNumericArray_getRank.html)
    pub fn rank(&self) -> usize {
        let NumericArray(numeric_array) = *self;

        let rank: sys::mint = unsafe {
            let getter: unsafe extern "C" fn(*mut sys::st_MNumericArray) -> sys::mint =
                (*crate::get_library_data().numericarrayLibraryFunctions)
                    .MNumericArray_getRank
                    .expect("MNumericArray_getRank callback is NULL");

            getter(numeric_array)
        };

        let rank = usize::try_from(rank).expect("NumericArray rank overflows usize");

        rank
    }

    fn dimensions(&self) -> &[crate::sys::mint] {
        let NumericArray(numeric_array) = *self;

        let rank = self.rank();

        debug_assert!(rank != 0);

        let dims: *const crate::sys::mint = unsafe {
            let getter: unsafe extern "C" fn(
                *mut sys::st_MNumericArray,
            ) -> *const sys::mint = (*crate::get_library_data()
                .numericarrayLibraryFunctions)
                .MNumericArray_getDimensions
                .expect("MNumericArray_getDimensions callback is NULL");

            getter(numeric_array)
        };

        debug_assert!(!dims.is_null());

        unsafe { std::slice::from_raw_parts(dims, rank) }
    }
}

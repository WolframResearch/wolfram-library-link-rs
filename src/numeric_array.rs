use std::mem::MaybeUninit;

/// Extremely basic wrapper around raw MNumericArray. Basically only suitable for working
/// with ByteArray[]'s (for serializing/deserializing WXF).
pub struct NumericArray(pub(crate) crate::sys::MNumericArray);

impl NumericArray {
    /// # Safety
    ///
    /// This method must only be called when it's assured that the data contained by this
    /// NumericArray has been initialized.
    pub unsafe fn data_bytes(&self) -> &[u8] {
        let NumericArray(numeric_array) = *self;

        let data_ptr: *mut std::ffi::c_void = (*numeric_array).data;
        let data_ptr = data_ptr as *mut u8;

        std::slice::from_raw_parts(data_ptr, self.length_in_bytes())
    }

    pub unsafe fn data_bytes_mut(&mut self) -> &mut [MaybeUninit<u8>] {
        let NumericArray(numeric_array) = self;

        let data_ptr: *mut std::ffi::c_void = (**numeric_array).data;
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

    fn dimensions(&self) -> &[crate::sys::mint] {
        let numeric_array: crate::sys::st_MNumericArray = unsafe { *self.0 };

        let rank = usize::try_from(numeric_array.rank)
            .expect("NumericArray rank overflows usize");

        let dims: *mut crate::sys::mint = numeric_array.dims;

        debug_assert!(rank != 0);
        debug_assert!(!dims.is_null());

        unsafe { std::slice::from_raw_parts(dims, rank) }
    }
}
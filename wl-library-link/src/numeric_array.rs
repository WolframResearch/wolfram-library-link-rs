use std::ffi::c_void;
use std::fmt;
use std::marker::PhantomData;
use std::mem::MaybeUninit;

use static_assertions::{assert_eq_align, assert_eq_size};

use crate::{rtl, sys};

#[rustfmt::skip]
use crate::sys::MNumericArray_Data_Type::{
    MNumericArray_Type_Bit8 as BIT8_TYPE,
    MNumericArray_Type_Bit16 as BIT16_TYPE,
    MNumericArray_Type_Bit32 as BIT32_TYPE,
    MNumericArray_Type_Bit64 as BIT64_TYPE,

    MNumericArray_Type_UBit8 as UBIT8_TYPE,
    MNumericArray_Type_UBit16 as UBIT16_TYPE,
    MNumericArray_Type_UBit32 as UBIT32_TYPE,
    MNumericArray_Type_UBit64 as UBIT64_TYPE,

    MNumericArray_Type_Real32 as REAL32_TYPE,
    MNumericArray_Type_Real64 as REAL64_TYPE,

    MNumericArray_Type_Complex_Real32 as COMPLEX_REAL32_TYPE,
    MNumericArray_Type_Complex_Real64 as COMPLEX_REAL64_TYPE,
};

/// This type is an ABI-compatible wrapper around [`wl_library_link_sys::MNumericArray`].
///
/// A [`NumericArray`] can contain any type `T` which satisfies the trait
/// [`NumericArrayType`].
///
/// Use [`NumericArray::kind()`] to dynamically resolve a `NumericArray` with unknown
/// element type into a `NumericArray<T>` with explicit element type.
///
/// Use [`UninitNumericArray`] to construct a [`NumericArray`] without requiring an
/// intermediate allocation to copy the elements from.
#[repr(transparent)]
pub struct NumericArray<T = ()>(sys::MNumericArray, PhantomData<T>);

/// Represents a [`NumericArray`] which has been allocated, but whose elements have not
/// yet been initialized.
///
/// Use [`as_slice_mut()`][`UninitNumericArray::as_slice_mut()`] to initialize the
/// elements of this [`UninitNumericArray`].
pub struct UninitNumericArray<T: NumericArrayType>(sys::MNumericArray, PhantomData<T>);

//======================================
// Traits
//======================================

/// Marker trait to denote the types which can be stored in a [`NumericArray`].
///
/// Those types are:
///
///   * [`u8`], [`u16`], [`u32`], [`u64`]
///   * [`i8`], [`i16`], [`i32`], [`i64`]
///   * [`f32`], [`f64`]
///   * [`mcomplex`][sys::mcomplex]
///
/// [`NumericArrayDataType`] is an enumeration of all the types which satisfy this trait.
pub trait NumericArrayType: private::Sealed {
    /// The [`NumericArrayDataType`] which dynamically represents the type which this
    /// trait is implemented for.
    const TYPE: NumericArrayDataType;
}

mod private {
    use crate::sys;

    pub trait Sealed {}

    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}

    impl Sealed for i8 {}
    impl Sealed for i16 {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}

    impl Sealed for f32 {}
    impl Sealed for f64 {}

    // impl Sealed for sys::complexreal32 {}
    impl Sealed for sys::mcomplex {}
}

impl NumericArrayType for i8 {
    const TYPE: NumericArrayDataType = NumericArrayDataType::Bit8;
}
impl NumericArrayType for i16 {
    const TYPE: NumericArrayDataType = NumericArrayDataType::Bit16;
}
impl NumericArrayType for i32 {
    const TYPE: NumericArrayDataType = NumericArrayDataType::Bit32;
}
impl NumericArrayType for i64 {
    const TYPE: NumericArrayDataType = NumericArrayDataType::Bit64;
}

impl NumericArrayType for u8 {
    const TYPE: NumericArrayDataType = NumericArrayDataType::UBit8;
}
impl NumericArrayType for u16 {
    const TYPE: NumericArrayDataType = NumericArrayDataType::UBit16;
}
impl NumericArrayType for u32 {
    const TYPE: NumericArrayDataType = NumericArrayDataType::UBit32;
}
impl NumericArrayType for u64 {
    const TYPE: NumericArrayDataType = NumericArrayDataType::UBit64;
}

impl NumericArrayType for f32 {
    const TYPE: NumericArrayDataType = NumericArrayDataType::Real32;
}
impl NumericArrayType for f64 {
    const TYPE: NumericArrayDataType = NumericArrayDataType::Real64;
}

// TODO: Why is there no WolframLibrary.h typedef for 32-bit complex reals?
// impl NumericArrayType for sys::complexreal32 {
//     const TYPE: NumericArrayDataType = NumericArrayDataType::ComplexReal32;
// }
impl NumericArrayType for sys::mcomplex {
    const TYPE: NumericArrayDataType = NumericArrayDataType::ComplexReal64;
}

//======================================
// Enums
//======================================

/// The type of the data being stored in a [`NumericArray`].
///
/// This is an enumeration of all the types which satisfy [`NumericArrayType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
#[allow(missing_docs)]
pub enum NumericArrayDataType {
    Bit8 = BIT8_TYPE as u32,
    Bit16 = BIT16_TYPE as u32,
    Bit32 = BIT32_TYPE as u32,
    Bit64 = BIT64_TYPE as u32,

    UBit8 = UBIT8_TYPE as u32,
    UBit16 = UBIT16_TYPE as u32,
    UBit32 = UBIT32_TYPE as u32,
    UBit64 = UBIT64_TYPE as u32,

    Real32 = REAL32_TYPE as u32,
    Real64 = REAL64_TYPE as u32,

    ComplexReal32 = COMPLEX_REAL32_TYPE as u32,
    ComplexReal64 = COMPLEX_REAL64_TYPE as u32,
}

/// Data array borrowed from a [`NumericArray`].
///
/// Use [`NumericArray::kind()`] to get an instance of this type.
#[allow(missing_docs)]
pub enum NumericArrayKind<'e> {
    //
    // Signed integer types
    //
    Bit8(&'e NumericArray<i8>),
    Bit16(&'e NumericArray<i16>),
    Bit32(&'e NumericArray<i32>),
    Bit64(&'e NumericArray<i64>),

    //
    // Unsigned integer types
    //
    UBit8(&'e NumericArray<u8>),
    UBit16(&'e NumericArray<u16>),
    UBit32(&'e NumericArray<u32>),
    UBit64(&'e NumericArray<u64>),

    //
    // Real types
    //
    Real32(&'e NumericArray<f32>),
    Real64(&'e NumericArray<f64>),

    //
    // Complex types
    //
    // ComplexReal32(&'e NumericArray<sys::complexreal32>),
    ComplexReal64(&'e NumericArray<sys::mcomplex>),
}

// Assert that `sys::mcomplex` is the 64-bit complex real type and not a 32-bit complex
// real type.
assert_eq_size!(sys::mcomplex, [f64; 2]);
assert_eq_align!(sys::mcomplex, f64);

//======================================
// Impls
//======================================

impl NumericArray {
    /// Dynamically resolve a `NumericArray` of unknown element type into a
    /// `NumericArray<T>` with explicit element type.
    ///
    /// # Example
    ///
    /// Implement a function which returns the sum of an integral `NumericArray`
    ///
    /// ```no_run
    /// use wl_library_link::{NumericArray, NumericArrayKind};
    ///
    /// fn sum(array: NumericArray) -> i64 {
    ///     match array.kind() {
    ///         NumericArrayKind::Bit8(na) => na.as_slice().into_iter().copied().map(i64::from).sum(),
    ///         NumericArrayKind::Bit16(na) => na.as_slice().into_iter().copied().map(i64::from).sum(),
    ///         NumericArrayKind::Bit32(na) => na.as_slice().into_iter().copied().map(i64::from).sum(),
    ///         NumericArrayKind::Bit64(na) => na.as_slice().into_iter().sum(),
    ///         NumericArrayKind::UBit8(na) => na.as_slice().into_iter().copied().map(i64::from).sum(),
    ///         NumericArrayKind::UBit16(na) => na.as_slice().into_iter().copied().map(i64::from).sum(),
    ///         NumericArrayKind::UBit32(na) => na.as_slice().into_iter().copied().map(i64::from).sum(),
    ///         NumericArrayKind::UBit64(na) => {
    ///             match i64::try_from(na.as_slice().into_iter().sum::<u64>()) {
    ///                 Ok(sum) => sum,
    ///                 Err(_) => panic!("overflows i64"),
    ///             }
    ///         },
    ///         NumericArrayKind::Real32(_)
    ///         | NumericArrayKind::Real64(_)
    ///         | NumericArrayKind::ComplexReal64(_) => panic!("bad type"),
    ///     }
    /// }
    /// ```
    pub fn kind(&self) -> NumericArrayKind {
        /// The purpose of this intermediate function is to limit the scope of the call to
        /// transmute(). `transmute()` is a *very* unsafe function, so it seems prudent to
        /// future-proof this code against accidental changes which alter the inferrence
        /// of the transmute() target type.
        unsafe fn trans<T: NumericArrayType>(array: &NumericArray) -> &NumericArray<T> {
            std::mem::transmute(array)
        }

        unsafe {
            use NumericArrayDataType::*;

            match self.data_type() {
                Bit8 => NumericArrayKind::Bit8(trans(self)),
                Bit16 => NumericArrayKind::Bit16(trans(self)),
                Bit32 => NumericArrayKind::Bit32(trans(self)),
                Bit64 => NumericArrayKind::Bit64(trans(self)),

                UBit8 => NumericArrayKind::UBit8(trans(self)),
                UBit16 => NumericArrayKind::UBit16(trans(self)),
                UBit32 => NumericArrayKind::UBit32(trans(self)),
                UBit64 => NumericArrayKind::UBit64(trans(self)),

                Real32 => NumericArrayKind::Real32(trans(self)),
                Real64 => NumericArrayKind::Real64(trans(self)),

                // TODO: Handle this case? Is there a 32 bit complex real typedef?
                ComplexReal32 => unimplemented!(
                    "NumericArray::kind(): NumericArray of ComplexReal32 is not currently supported."
                ),
                // ComplexReal32 => NumericArrayKind::ComplexReal32(trans(self)),
                ComplexReal64 => NumericArrayKind::ComplexReal64(trans(self)),
            }
        }
    }

    /// Attempt to resolve this `NumericArray` into a `&NumericArray<T>` of the specified
    /// element type.
    ///
    /// If the element type of this array does not match `T`, an error will be returned.
    ///
    /// # Example
    ///
    /// Implement a function which unwraps the `&[u8]` data in a `NumericArray` of 8-bit
    /// integers.
    ///
    /// ```no_run
    /// use wl_library_link::NumericArray;
    ///
    /// fn bytes(array: &NumericArray) -> &[u8] {
    ///     let byte_array: &NumericArray<u8> = match array.try_kind::<u8>() {
    ///         Ok(array) => array,
    ///         Err(_) => panic!("expected NumericArray of UnsignedInteger8")
    ///     };
    ///
    ///     byte_array.as_slice()
    /// }
    /// ```
    pub fn try_kind<T>(&self) -> Result<&NumericArray<T>, ()>
    where
        T: NumericArrayType,
    {
        /// The purpose of this intermediate function is to limit the scope of the call to
        /// transmute(). `transmute()` is a *very* unsafe function, so it seems prudent to
        /// future-proof this code against accidental changes which alter the inferrence
        /// of the transmute() target type.
        unsafe fn trans<T: NumericArrayType>(array: &NumericArray) -> &NumericArray<T> {
            std::mem::transmute(array)
        }

        if self.data_type() == T::TYPE {
            return Ok(unsafe { trans(self) });
        }

        Err(())
    }

    /// Attempt to resolve this `NumericArray` into a `NumericArray<T>` of the specified
    /// element type.
    ///
    /// If the element type of this array does not match `T`, the original untyped array
    /// will be returned as the error value.
    pub fn try_into_kind<T>(self) -> Result<NumericArray<T>, NumericArray>
    where
        T: NumericArrayType,
    {
        /// The purpose of this intermediate function is to limit the scope of the call to
        /// transmute(). `transmute()` is a *very* unsafe function, so it seems prudent to
        /// future-proof this code against accidental changes which alter the inferrence
        /// of the transmute() target type.
        unsafe fn trans<T: NumericArrayType>(array: NumericArray) -> NumericArray<T> {
            std::mem::transmute(array)
        }

        if self.data_type() == T::TYPE {
            return Ok(unsafe { trans(self) });
        }

        Err(self)
    }
}

impl<T: NumericArrayType> NumericArray<T> {
    /// Construct a new one-dimensional [`NumericArray`] from a slice.
    ///
    /// Use [`NumericArray::from_array()`] to construct multidimensional numeric arrays.
    ///
    /// # Panics
    ///
    /// This function will panic if [`NumericArray::try_from_array()`] returns
    /// an error.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use wl_library_link::NumericArray;
    /// let array = NumericArray::from_slice(&[1, 2, 3, 4, 5]);
    /// ```
    ///
    /// # Alternatives
    ///
    /// [`UninitNumericArray`] can be used to allocate a mutable numeric array,
    /// eliminating the need for an intermediate allocation.
    pub fn from_slice(data: &[T]) -> NumericArray<T> {
        NumericArray::<T>::try_from_slice(data)
            .expect("failed to create NumericArray from slice")
    }

    /// Fallible alternative to [`NumericArray::from_slice()`].
    pub fn try_from_slice(data: &[T]) -> Result<NumericArray<T>, ()> {
        let dim1 = data.len();

        NumericArray::try_from_array(&[dim1], data)
    }

    /// Construct a new multidimensional [`NumericArray`] from a list of dimensions and the
    /// flat slice of data.
    ///
    /// # Panics
    ///
    /// This function will panic if [`NumericArray::try_from_array()`] returns
    /// an error.
    ///
    /// # Example
    ///
    /// Construct the 2x2 [`NumericArray`] `{{1, 2}, {3, 4}}` from a list of dimensions and
    /// a flat buffer.
    ///
    /// ```no_run
    /// # use wl_library_link::NumericArray;
    /// let array = NumericArray::from_array(&[2, 2], &[1, 2, 3, 4]);
    /// ```
    pub fn from_array(dimensions: &[usize], data: &[T]) -> NumericArray<T> {
        NumericArray::<T>::try_from_array(dimensions, data)
            .expect("failed to create NumericArray from array")
    }

    /// Fallible alternative to [`NumericArray::from_array()`].
    ///
    /// This function will return an error if:
    ///
    /// * `dimensions` is empty
    /// * the product of `dimensions` is 0
    /// * `data.len()` is not equal to the product of `dimensions`
    pub fn try_from_array(
        dimensions: &[usize],
        data: &[T],
    ) -> Result<NumericArray<T>, ()> {
        let uninit = UninitNumericArray::try_from_dimensions(dimensions)?;

        Ok(uninit.init_from_slice(data))
    }
}

impl<T> NumericArray<T> {
    /// Erase the concrete `T` data type associated with this `NumericArray`.
    ///
    /// Use [`NumericArray::try_into_kind()`] to convert back into a `NumericArray<T>`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use wl_library_link::NumericArray;
    /// let array: NumericArray<i64> = NumericArray::from_slice(&[1, 2, 3]);
    ///
    /// let array: NumericArray = array.into_generic();
    /// ```
    pub fn into_generic(self) -> NumericArray {
        let NumericArray(na, PhantomData) = self;
        NumericArray(na, PhantomData)
    }

    /// Construct a `NumericArray<T>` from a raw [`MNumericArray`][sys::MNumericArray].
    ///
    /// # Safety
    ///
    /// The following conditions must be met for safe usage of this function:
    ///
    /// * `array` must be a fully initialized and valid numeric array object
    /// * `T` must either:
    ///   - be `()`, representing an array with dynamic element type, or
    ///   - `T` must satisfy [`NumericArrayType`], and the element type of `array` must
    ///     be the same as `T`.
    // TODO: Add something about the reference count in the above list?
    pub unsafe fn from_raw(array: sys::MNumericArray) -> NumericArray<T> {
        NumericArray(array, PhantomData)
    }

    /// Convert this `NumericArray` into a raw [`MNumericArray`][sys::MNumericArray]
    /// object.
    pub unsafe fn into_raw(self) -> sys::MNumericArray {
        self.0
    }

    /// *LibraryLink C API Documentation:* [`MNumericArray_getData`](https://reference.wolfram.com/language/LibraryLink/ref/callback/MNumericArray_getData.html)
    pub fn data_ptr(&self) -> *mut c_void {
        let NumericArray(numeric_array, _) = *self;

        unsafe { data_ptr(numeric_array) }
    }

    /// Access the elements stored in this [`NumericArray`] as a flat buffer.
    pub fn as_slice(&self) -> &[T] {
        let ptr: *mut c_void = self.data_ptr();

        debug_assert!(!ptr.is_null());

        // Assert that `ptr` is aligned to `T`.
        debug_assert!(ptr as usize % std::mem::size_of::<T>() == 0);

        let ptr = ptr as *const T;

        unsafe { std::slice::from_raw_parts(ptr, self.flattened_length()) }
    }

    /// Access the elements stored in this [`NumericArray`] as a mutable flat buffer.
    ///
    /// # Safety
    ///
    /// `NumericArray` is an immutable shared data structure. There is no robust, easy way
    /// to determine whether mutation of a `NumericArray` is safe. Prefer to use
    /// [`UninitNumericArray`] to create and initialize a numeric array value instead of
    /// mutating an existing `NumericArray`.
    pub unsafe fn as_slice_mut(&mut self) -> &mut [T] {
        let ptr: *mut c_void = self.data_ptr();

        debug_assert!(!ptr.is_null());

        // Assert that `ptr` is aligned to `T`.
        debug_assert!(ptr as usize % std::mem::size_of::<T>() == 0);

        let ptr = ptr as *mut T;

        std::slice::from_raw_parts_mut(ptr, self.flattened_length())
    }

    fn data_type(&self) -> NumericArrayDataType {
        let value: u32 = u32::try_from(self.tensor_property_type())
            .expect("NumericArray tensor property type value overflows u32");

        NumericArrayDataType::try_from(value)
            .expect("NumericArray tensor property type is value is not a known NumericArrayDataType variant")
    }

    fn tensor_property_type(&self) -> u32 {
        let NumericArray(numeric_array, _) = *self;

        unsafe { rtl::MNumericArray_getType(numeric_array) }
    }

    /// The number of elements in the underlying flat data array.
    ///
    /// This is the product of the dimension lengths of this [`NumericArray`].
    ///
    /// This is *not* the number of bytes.
    ///
    /// *LibraryLink C API Documentation:* [`MNumericArray_getFlattenedLength`](https://reference.wolfram.com/language/LibraryLink/ref/callback/MNumericArray_getFlattenedLength.html)
    pub fn flattened_length(&self) -> usize {
        let NumericArray(numeric_array, _) = *self;

        let len = unsafe { flattened_length(numeric_array) };

        // Check that the stored length matches the length computed from the dimensions.
        debug_assert!(len == self.dimensions().iter().copied().product::<usize>());

        len
    }

    /// *LibraryLink C API Documentation:* [`MNumericArray_getRank`](https://reference.wolfram.com/language/LibraryLink/ref/callback/MNumericArray_getRank.html)
    pub fn rank(&self) -> usize {
        let NumericArray(numeric_array, _) = *self;

        let rank: sys::mint = unsafe { rtl::MNumericArray_getRank(numeric_array) };

        let rank = usize::try_from(rank).expect("NumericArray rank overflows usize");

        rank
    }

    /// Get the dimensions of this `NumericArray`.
    ///
    /// *LibraryLink C API Documentation:* [`MNumericArray_getDimensions`](https://reference.wolfram.com/language/LibraryLink/ref/callback/MNumericArray_getDimensions.html)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use wl_library_link::NumericArray;
    /// let array = NumericArray::from_array(&[2, 2], &[1, 2, 3, 4]);
    ///
    /// assert_eq!(array.dimensions(), &[2, 2]);
    /// assert_eq!(array.rank(), array.dimensions().len());
    /// ```
    pub fn dimensions(&self) -> &[usize] {
        let NumericArray(numeric_array, _) = *self;

        let rank = self.rank();

        debug_assert!(rank != 0);

        let dims: *const crate::sys::mint =
            unsafe { rtl::MNumericArray_getDimensions(numeric_array) };

        assert_eq_size!(sys::mint, usize);
        let dims: *mut usize = dims as *mut usize;

        debug_assert!(!dims.is_null());

        unsafe { std::slice::from_raw_parts(dims, rank) }
    }
}

unsafe fn data_ptr(numeric_array: sys::MNumericArray) -> *mut c_void {
    rtl::MNumericArray_getData(numeric_array)
}

unsafe fn flattened_length(numeric_array: sys::MNumericArray) -> usize {
    let len: sys::mint = rtl::MNumericArray_getFlattenedLength(numeric_array);

    let len = usize::try_from(len).expect("i64 overflows usize");

    len
}

//======================================
// UninitNumericArray
//======================================

impl<T: NumericArrayType> UninitNumericArray<T> {
    /// Construct a new uninitialized NumericArray with the specified dimensions.
    ///
    /// # Panics
    ///
    /// This function will panic if [`UninitNumericArray::try_from_dimensions()`] returns
    /// an error.
    pub fn from_dimensions(dimensions: &[usize]) -> UninitNumericArray<T> {
        UninitNumericArray::try_from_dimensions(dimensions)
            .expect("failed to create UninitNumericArray from dimensions")
    }

    /// Try to construct a new uninitialized NumericArray with the specified dimensions.
    ///
    /// This function will return an error if:
    ///
    /// * `dimensions` is empty.
    /// * the product of `dimensions` is equal to 0.
    /// * the underlying allocation function returns `NULL`.
    pub fn try_from_dimensions(
        dimensions: &[usize],
    ) -> Result<UninitNumericArray<T>, ()> {
        assert!(!dimensions.is_empty());

        let kind: NumericArrayDataType = <T as NumericArrayType>::TYPE;
        let rank = dimensions.len();
        debug_assert!(rank > 0);

        unsafe {
            let mut numeric_array: sys::MNumericArray = std::ptr::null_mut();

            let err_code: sys::errcode_t = rtl::MNumericArray_new(
                kind as u32,
                i64::try_from(rank).expect("usize overflows i64"),
                dimensions.as_ptr() as *mut sys::mint,
                &mut numeric_array,
            );

            if err_code != 0 || numeric_array.is_null() {
                return Err(());
            }

            Ok(UninitNumericArray(numeric_array, PhantomData))
        }
    }

    /// # Panics
    ///
    /// This function will panic if `source` does not have the same length as
    /// this array's [`as_slice_mut()`][UninitNumericArray::as_slice_mut] slice.
    pub fn init_from_slice(mut self, source: &[T]) -> NumericArray<T> {
        let data = self.as_slice_mut();

        // Safety: copy_from_slice_uninit() unconditionally asserts that `data` and
        //         `source` have the same number of elements, so if it succeeds we're
        //         certain that every element of the NumericArray has been initialized.
        copy_from_slice_uninit(source, data);

        unsafe { self.assume_init() }
    }

    /// Mutable access to the elements of this [`UninitNumericArray`].
    ///
    /// This function returns a mutable slice of [`std::mem::MaybeUninit<T>`]. This is done
    /// because it is undefined behavior in Rust to construct a `&` (or `&mut`) reference
    /// to a value which has not been initialized. Note that it is undefined behavior even
    /// if the reference is never read from. The `MaybeUninit` type explicitly makes the
    /// compiler aware that the `T` value might not be initialized.
    ///
    /// # Example
    ///
    /// Construct the numeric array `{1, 2, 3, 4, 5}`.
    ///
    /// ```no_run
    /// use wl_library_link::{NumericArray, UninitNumericArray};
    ///
    /// // Construct a `1x5` numeric array with elements of type `f64`.
    /// let mut uninit = UninitNumericArray::<f64>::from_dimensions(&[5]);
    ///
    /// for (index, elem) in uninit.as_slice_mut().into_iter().enumerate() {
    ///     elem.write(index as f64 + 1.0);
    /// }
    ///
    /// // Now that we've taken responsibility for initializing every
    /// // element of the UninitNumericArray, we've upheld the
    /// // invariant necessary to make a call to `assume_init()` safe.
    /// let array: NumericArray<f64> = unsafe { uninit.assume_init() };
    /// ```
    ///
    /// See [`assume_init()`][UninitNumericArray::assume_init].
    pub fn as_slice_mut(&mut self) -> &mut [MaybeUninit<T>] {
        let UninitNumericArray(numeric_array, PhantomData) = *self;

        unsafe {
            let len = flattened_length(numeric_array);

            let ptr: *mut c_void = data_ptr(numeric_array);
            let ptr = ptr as *mut MaybeUninit<T>;

            std::slice::from_raw_parts_mut(ptr, len)
        }
    }

    /// Assume that this NumericArray's elements have been initialized.
    ///
    /// Use [`as_slice_mut()`][UninitNumericArray::as_slice_mut] to initialize the values
    /// in this array.
    ///
    /// # Safety
    ///
    /// This function must only be called once all elements of this NumericArray have
    /// been initialized. It is undefined behavior to construct a [`NumericArray`] without
    /// first initializing the data array.
    pub unsafe fn assume_init(self) -> NumericArray<T> {
        let UninitNumericArray(expr, PhantomData) = self;

        NumericArray(expr, PhantomData)
    }
}

/// This function is modeled after after the `copy_from_slice()` method on the primitive
/// `slice` type. This can be used to initialize an [`UninitNumericArray`] from a slice of
/// data.
fn copy_from_slice_uninit<T>(src: &[T], dest: &mut [MaybeUninit<T>]) {
    assert_eq!(
        src.len(),
        dest.len(),
        "destination and source slices have different lengths"
    );

    unsafe {
        std::ptr::copy_nonoverlapping(
            src.as_ptr(),
            dest.as_mut_ptr() as *mut T,
            dest.len(),
        )
    }
}

//======================================
// Formatting Impls
//======================================

impl<T> fmt::Debug for NumericArray<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("NumericArray")
            .field("raw", &self.0)
            .field("data_type", &self.data_type())
            .finish()
    }
}

//======================================
// Conversion Impls
//======================================

impl TryFrom<u32> for NumericArrayDataType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        // debug_assert!(u32::try_from(self.tensor_property_type()).is_ok());

        #[rustfmt::skip]
        let ok = match value {
            _ if value == BIT8_TYPE as u32 => NumericArrayDataType::Bit8,
            _ if value == BIT16_TYPE as u32 => NumericArrayDataType::Bit16,
            _ if value == BIT32_TYPE as u32 => NumericArrayDataType::Bit32,
            _ if value == BIT64_TYPE as u32 => NumericArrayDataType::Bit64,

            _ if value == UBIT8_TYPE as u32 => NumericArrayDataType::UBit8,
            _ if value == UBIT16_TYPE as u32 => NumericArrayDataType::UBit16,
            _ if value == UBIT32_TYPE as u32 => NumericArrayDataType::UBit32,
            _ if value == UBIT64_TYPE as u32 => NumericArrayDataType::UBit64,

            _ if value == REAL32_TYPE as u32 => NumericArrayDataType::Real32,
            _ if value == REAL64_TYPE as u32 => NumericArrayDataType::Real64,

            _ if value == COMPLEX_REAL32_TYPE as u32 => NumericArrayDataType::ComplexReal32,
            _ if value == COMPLEX_REAL64_TYPE as u32 => NumericArrayDataType::ComplexReal64,

            _ => return Err(()),
        };

        Ok(ok)
    }
}

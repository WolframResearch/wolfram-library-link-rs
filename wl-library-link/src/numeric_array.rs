use std::ffi::c_void;
use std::marker::PhantomData;
use std::mem::MaybeUninit;

use static_assertions::{assert_eq_align, assert_eq_size};

use crate::sys;

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

/// This type is an ABI-compatible wrapper around [`wl_kernel_sys::st_MNumericArray`].
///
/// A [`NumericArray`] can contain any type `T` which satisfies the trait
/// [`NumericArrayType`].
///
/// This type should not be confused with the [`ENumericArray`] type, which represents a
/// Raw-type expression containing a `NumericArray`.
///
/// Use [`UninitNumericArray`] to construct a [`NumericArray`] without requiring an
/// intermediate allocation to copy the elements from.
#[repr(transparent)]
pub struct NumericArray<T = ()>(sys::MNumericArray, PhantomData<T>);

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
///   * [`complexreal32`][sys::complexreal32], [`complexreal64`][sys::complexreal64]
///
/// [`NumericArrayDataType`] is an enumeration of all the types which satisfy this trait.
pub trait NumericArrayType: private::Sealed {
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
    pub unsafe fn from_raw(array: sys::MNumericArray) -> Self {
        NumericArray(array, PhantomData)
    }

    pub unsafe fn into_raw(self) -> sys::MNumericArray {
        self.0
    }

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

impl<T> NumericArray<T> {
    /// *LibraryLink C API Documentation:* [`MNumericArray_getData`](https://reference.wolfram.com/language/LibraryLink/ref/callback/MNumericArray_getData.html)
    pub fn data_ptr(&self) -> *mut c_void {
        let NumericArray(numeric_array, _) = *self;

        unsafe {
            let getter: unsafe extern "C" fn(*mut sys::st_MNumericArray) -> *mut c_void =
                (*crate::get_library_data().numericarrayLibraryFunctions)
                    .MNumericArray_getData
                    .expect("MNumericArray_getData callback is NULL");

            getter(numeric_array)
        }
    }

    /// Access the elements stored in this [`NumericArray`] as a flat buffer.
    pub fn data(&self) -> &[T] {
        let ptr: *mut c_void = self.data_ptr();

        debug_assert!(!ptr.is_null());

        // Assert that `ptr` is aligned to `T`.
        debug_assert!(ptr as usize % std::mem::size_of::<T>() == 0);

        let ptr = ptr as *const T;

        unsafe { std::slice::from_raw_parts(ptr, self.flattened_length()) }
    }

    fn data_type(&self) -> NumericArrayDataType {
        let value: u32 = u32::try_from(self.tensor_property_type())
            .expect("NumericArray tensor property type value overflows u32");

        NumericArrayDataType::try_from(value)
            .expect("NumericArray tensor property type is value is not a known NumericArrayDataType variant")
    }

    fn tensor_property_type(&self) -> u32 {
        let NumericArray(numeric_array, _) = *self;

        unsafe {
            let getter: unsafe extern "C" fn(
                *mut sys::st_MNumericArray,
            )
                -> sys::MNumericArray_Data_Type::Type = (*crate::get_library_data()
                .numericarrayLibraryFunctions)
                .MNumericArray_getType
                .expect("MNumericArray_getType callback is NULL");

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

        let length: usize = dims.iter().product();

        // FIXME: This should multiple `length` by the size-in-bytes of
        //        st_MNumericArray.tensor_property_type.
        length
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

        let len: sys::mint = unsafe {
            let getter: unsafe extern "C" fn(*mut sys::st_MNumericArray) -> sys::mint =
                (*crate::get_library_data().numericarrayLibraryFunctions)
                    .MNumericArray_getFlattenedLength
                    .expect("MNumericArray_getFlattenedLength callback is NULL");

            getter(numeric_array)
        };

        let len = usize::try_from(len).expect("i64 overflows usize");

        // Check that the stored length matches the length computed from the dimensions.
        debug_assert!(len == self.dimensions().iter().copied().product::<usize>());

        len
    }

    /// *LibraryLink C API Documentation:* [`MNumericArray_getRank`](https://reference.wolfram.com/language/LibraryLink/ref/callback/MNumericArray_getRank.html)
    pub fn rank(&self) -> usize {
        let NumericArray(numeric_array, _) = *self;

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

    fn dimensions(&self) -> &[usize] {
        let NumericArray(numeric_array, _) = *self;

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

        assert_eq_size!(sys::mint, usize);
        let dims: *mut usize = dims as *mut usize;

        debug_assert!(!dims.is_null());

        unsafe { std::slice::from_raw_parts(dims, rank) }
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

//! Traits for working with data types that can be passed natively via LibraryLink
//! [`MArgument`]s.

use std::{
    cell::RefCell,
    ffi::{CStr, CString},
    os::raw::c_char,
};

use ref_cast::RefCast;

use crate::{
    rtl,
    sys::{self, mint, mreal, MArgument},
    DataStore, Image, NumericArray,
};

/// Trait implemented for types that may be passed via an [`MArgument`].
pub trait FromArg<'a> {
    #[allow(missing_docs)]
    unsafe fn from_arg(arg: &'a MArgument) -> Self;
}

/// Trait implemented for that that may be returned via an [`MArgument`].
///
/// The [`MArgument`] that this trait is used to modify must be the return value of a
/// LibraryLink function. It is not valid to modify [`MArgument`]s that contain
/// LibraryLink function arguments.
pub trait IntoArg {
    /// Move `self` into `arg`.
    ///
    /// # Safety
    ///
    /// `arg` must be an uninitialized [`MArgument`] that is used to store the return
    /// value of a LibraryLink function. The return type of that function must match
    /// the type of `self.`
    ///
    /// This function must only be called immediately before returning from a LibraryLink
    /// function. Each native LibraryLink function must perform at most one call to this
    /// method per invocation.
    //
    // Private implementation note:
    //   the "at most one call to this method per invocation" constraint is necessary to
    //   maintain the safety invariants of `impl IntoArg for CString`.
    unsafe fn into_arg(self, arg: MArgument);
}

/// Trait implemented for any function whose parameters and return type are native
/// LibraryLink [`MArgument`] types.
///
/// [`export!`][crate::export] may only be used with functions that implement this trait.
///
/// A function implements this trait if all of its parameters implement [`FromArg`] and
/// its return type implements [`IntoArg`].
///
/// Functions that pass their arguments and return value using a [`wstp::Link`] do not
/// implement this trait.
pub trait NativeFunction<'a> {
    /// Call the function using the raw LibraryLink [`MArgument`] fields.
    unsafe fn call(&self, args: &'a [MArgument], ret: MArgument);
}

//======================================
// FromArg Impls
//======================================

impl FromArg<'_> for bool {
    unsafe fn from_arg(arg: &MArgument) -> Self {
        crate::bool_from_mbool(*arg.boolean)
    }
}

impl FromArg<'_> for mint {
    unsafe fn from_arg(arg: &MArgument) -> Self {
        *arg.integer
    }
}

impl FromArg<'_> for mreal {
    unsafe fn from_arg(arg: &MArgument) -> Self {
        *arg.real
    }
}

impl FromArg<'_> for sys::mcomplex {
    unsafe fn from_arg(arg: &MArgument) -> Self {
        *arg.cmplex
    }
}

//--------------------------------------
// Strings
//--------------------------------------

unsafe fn c_str_from_arg<'a>(arg: &'a MArgument) -> &'a CStr {
    let cstr: *mut i8 = *arg.utf8string;
    CStr::from_ptr(cstr)
}

impl<'a> FromArg<'a> for CString {
    unsafe fn from_arg(arg: &'a MArgument) -> CString {
        let owned = {
            let cstr: &'a CStr = c_str_from_arg(arg);
            CString::from(cstr)
        };

        // Now that we own our own copy of the string, disown the Kernel's copy.
        rtl::UTF8String_disown(*arg.utf8string);

        owned
    }
}

/// # Panics
///
/// This conversion will panic if the [`MArgument::utf8string`] field is not valid UTF-8.
impl<'a> FromArg<'a> for String {
    unsafe fn from_arg(arg: &'a MArgument) -> String {
        let owned = {
            let cstr: &'a CStr = c_str_from_arg(arg);
            let str: &'a str = cstr
                .to_str()
                .expect("FromArg for &str: string was not valid UTF-8");
            str.to_owned()
        };

        // Now that we own our own copy of the string, disown the Kernel's copy.
        rtl::UTF8String_disown(*arg.utf8string);

        owned
    }
}

// TODO: Supported borrowed &CStr and &str's using some kind of wrapper that ensures we
//       disown the Kernel string.

/// # Safety
///
/// The lifetime of the returned `&CStr` must be the same as the lifetime of `arg`.
///
/// # Warning
///
/// Using `&CStr` as the parameter type of a *LibraryLink* function will result in a
/// memory leak. Use [`String`] or [`CString`] instead.
impl<'a> FromArg<'a> for &'a CStr {
    unsafe fn from_arg(arg: &'a MArgument) -> &'a CStr {
        c_str_from_arg(arg)
    }
}

/// # Panics
///
/// This conversion will panic if the [`MArgument::utf8string`] field is not valid UTF-8.
///
/// # Safety
///
/// The lifetime of the returned `&str` must be the same as the lifetime of `arg`.
///
/// # Warning
///
/// Using `&str` as the parameter type of a *LibraryLink* function will result in a
/// memory leak. Use [`String`] or [`CString`] instead.
impl<'a> FromArg<'a> for &'a str {
    unsafe fn from_arg(arg: &'a MArgument) -> &'a str {
        let cstr: &'a CStr = FromArg::<'a>::from_arg(arg);
        cstr.to_str()
            .expect("FromArg for &str: string was not valid UTF-8")
    }
}

//--------------------------------------
// NumericArray
//--------------------------------------

// TODO: Add FromArg for NumericArray which just clones the numeric array? Or disclaims
//       ownership in another way?


/// # Safety
///
/// `FromArg for NumericArray<T>` MUST be constrained by `T: NumericArrayType` to prevent
/// accidental creation of invalid `NumericArray` conversions. Without this constraint,
/// it would be possible to write code like:
///
/// ```compile_fail
/// # mod scope {
/// # use wolfram_library_link::{export, NumericArray};
/// fn and(bools: NumericArray<bool>) -> bool {
///     // ...
/// #   todo!()
/// }
///
/// // Unsafe!
/// export![and(_)];
/// # }
/// ```
///
/// which is not valid because `bool` is not a valid numeric array type.
impl<'a, T: crate::NumericArrayType> FromArg<'a> for &'a NumericArray<T> {
    unsafe fn from_arg(arg: &'a MArgument) -> &'a NumericArray<T> {
        NumericArray::ref_cast(&*arg.numeric)
    }
}

impl<'a, T: crate::NumericArrayType> FromArg<'a> for NumericArray<T> {
    unsafe fn from_arg(arg: &'a MArgument) -> NumericArray<T> {
        NumericArray::from_raw(*arg.numeric)
    }
}

impl<'a> FromArg<'a> for &'a NumericArray<()> {
    unsafe fn from_arg(arg: &'a MArgument) -> &'a NumericArray<()> {
        NumericArray::ref_cast(&*arg.numeric)
    }
}

impl<'a> FromArg<'a> for NumericArray<()> {
    unsafe fn from_arg(arg: &'a MArgument) -> NumericArray<()> {
        NumericArray::from_raw(*arg.numeric)
    }
}

//--------------------------------------
// Image
//--------------------------------------

impl<'a, T: crate::ImageData> FromArg<'a> for &'a Image<T> {
    unsafe fn from_arg(arg: &'a MArgument) -> &'a Image<T> {
        Image::ref_cast(&*arg.image)
    }
}

impl<'a, T: crate::ImageData> FromArg<'a> for Image<T> {
    unsafe fn from_arg(arg: &'a MArgument) -> Image<T> {
        Image::from_raw(*arg.image)
    }
}

impl<'a> FromArg<'a> for &'a Image<()> {
    unsafe fn from_arg(arg: &'a MArgument) -> &'a Image<()> {
        Image::ref_cast(&*arg.image)
    }
}

impl<'a> FromArg<'a> for Image<()> {
    unsafe fn from_arg(arg: &'a MArgument) -> Image<()> {
        Image::from_raw(*arg.image)
    }
}

//--------------------------------------
// DataStore
//--------------------------------------

impl FromArg<'_> for DataStore {
    unsafe fn from_arg(arg: &MArgument) -> DataStore {
        DataStore::from_raw(*arg.tensor as sys::DataStore)
    }
}

impl<'a> FromArg<'a> for &'a DataStore {
    unsafe fn from_arg(arg: &MArgument) -> &'a DataStore {
        DataStore::ref_cast(&*(arg.tensor as *mut sys::DataStore))
    }
}

//======================================
// impl IntoArg
//======================================

impl IntoArg for () {
    unsafe fn into_arg(self, _arg: MArgument) {
        // Do nothing.
    }
}

//---------------------
// Primitive data types
//---------------------

impl IntoArg for bool {
    unsafe fn into_arg(self, arg: MArgument) {
        let boole: u32 = if self { sys::True } else { sys::False };
        *arg.boolean = boole as sys::mbool;
    }
}

impl IntoArg for mint {
    unsafe fn into_arg(self, arg: MArgument) {
        *arg.integer = self;
    }
}

impl IntoArg for mreal {
    unsafe fn into_arg(self, arg: MArgument) {
        *arg.real = self;
    }
}

impl IntoArg for sys::mcomplex {
    unsafe fn into_arg(self, arg: MArgument) {
        *arg.cmplex = self;
    }
}

//--------------------------------------------------
// Convenience conversions for narrow integer sizes.
//--------------------------------------------------

impl IntoArg for i8 {
    unsafe fn into_arg(self, arg: MArgument) {
        *arg.integer = mint::from(self);
    }
}

impl IntoArg for i16 {
    unsafe fn into_arg(self, arg: MArgument) {
        *arg.integer = mint::from(self);
    }
}

impl IntoArg for i32 {
    unsafe fn into_arg(self, arg: MArgument) {
        *arg.integer = mint::from(self);
    }
}

impl IntoArg for u8 {
    unsafe fn into_arg(self, arg: MArgument) {
        *arg.integer = mint::from(self);
    }
}

impl IntoArg for u16 {
    unsafe fn into_arg(self, arg: MArgument) {
        *arg.integer = mint::from(self);
    }
}

// If we're on a 32 bit platform, mint might be an alias for i32. Avoid providing this
// conversion on those platforms.
#[cfg(target_pointer_width = "64")]
impl IntoArg for u32 {
    unsafe fn into_arg(self, arg: MArgument) {
        *arg.integer = mint::from(self);
    }
}

//--------------------
// Strings
//--------------------

thread_local! {
    /// See [`<CString as IntoArg>::into_arg()`] for information about how this static is
    /// used.
    static RETURNED_STRING: RefCell<Option<CString>> = RefCell::new(None);
}

impl IntoArg for CString {
    unsafe fn into_arg(self, arg: MArgument) {
        // Extend the lifetime of `self.as_ptr()` by storing `self` in `RETURNED_STRING`.
        //
        // This will keep `raw` valid past the point that the current LibraryLink
        // function returns, at which point it will be copied by the Kernel and is no
        // longer used. We'll drop `self` the next time this function is called.
        //
        // This implementation limits the maximum number of "leaked" strings to just one.
        //
        // For more information on management of string memory when passed via
        // LibraryLink, see:
        //
        // <https://reference.wolfram.com/language/LibraryLink/tutorial/InteractionWithWolframLanguage.html#262826222>
        let raw: *const c_char = RETURNED_STRING.with(|stored| {
            // Drop the previously returned string, if any.
            if let Some(prev) = stored.replace(None) {
                drop(prev);
            }

            let raw: *const c_char = self.as_ptr();

            *stored.borrow_mut() = Some(self);

            raw
        });

        // Return `raw` via this MArgument.
        *arg.utf8string = raw as *mut c_char;
    }
}

impl IntoArg for String {
    /// # Panics
    ///
    /// This function will panic if `self` cannot be converted into a [`CString`].
    unsafe fn into_arg(self, arg: MArgument) {
        let cstring = CString::new(self)
            .expect("IntoArg for String: could not convert String to CString");

        <CString as IntoArg>::into_arg(cstring, arg)
    }
}

//---------------------------------------
// NumericArray, Image, DataStore
//---------------------------------------

impl<T> IntoArg for NumericArray<T> {
    unsafe fn into_arg(self, arg: MArgument) {
        *arg.numeric = self.into_raw();
    }
}

impl<T> IntoArg for Image<T> {
    unsafe fn into_arg(self, arg: MArgument) {
        *arg.image = self.into_raw();
    }
}

impl IntoArg for DataStore {
    unsafe fn into_arg(self, arg: MArgument) {
        *arg.tensor = self.into_raw() as *mut _;
    }
}

//======================================
// impl NativeFunction
//======================================

/// Implement `NativeFunction` for functions that use raw [`MArgument`]s for their
/// arguments and return value.
///
/// # Example
///
/// ```
/// # mod scope {
/// use wolfram_library_link::{self as wll, sys::MArgument, FromArg};
///
/// wll::export![raw_add2(_, _)];
///
/// fn raw_add2(args: &[MArgument], ret: MArgument) {
///     let x = unsafe { i64::from_arg(&args[0]) };
///     let y = unsafe { i64::from_arg(&args[1]) };
///
///     unsafe {
///         *ret.integer = x + y;
///     }
/// }
/// # }
/// ```
///
/// ```wolfram
/// LibraryFunctionLoad["...", "raw_add2", {Integer, Integer}, Integer]
/// ```
impl<'a: 'b, 'b> NativeFunction<'a> for &dyn Fn(&'b [MArgument], MArgument) {
    unsafe fn call(&self, args: &'a [MArgument], ret: MArgument) {
        self(args, ret)
    }
}

//--------------------
// impl NativeFunction
//--------------------

macro_rules! impl_NativeFunction {
    ($($type:ident),*) => {
        impl<'a, $($type: FromArg<'a>,)* R: IntoArg> NativeFunction<'a> for &dyn Fn($($type),*) -> R {
            unsafe fn call(&self, args: &'a [MArgument], ret: MArgument) {
                // Re-use the $type name as the local variable names. E.g.
                //     let A1 = A1::from_arg(..);
                // This works because types and variable names are different namespaces.
                #[allow(non_snake_case)]
                let [$($type,)*] = match args {
                    [$($type,)*] => [$($type,)*],
                    _ => panic!(
                        "LibraryLink function number of arguments ({}) does not match \
                        number of parameters",
                        args.len()
                    ),
                };

                $(
                    #[allow(non_snake_case)]
                    let $type: $type = $type::from_arg($type);
                )*

                let result: R = self($($type,)*);

                result.into_arg(ret);
            }
        }
    }
}

// Handle the zero-arguments case specially.
impl<'a, R> NativeFunction<'a> for &dyn Fn() -> R
where
    R: IntoArg,
{
    unsafe fn call(&self, args: &[MArgument], ret: MArgument) {
        if args.len() != 0 {
            panic!(
                "LibraryLink function number of arguments ({}) does not match number of \
                parameters",
                args.len()
            );
        }

        let result = self();

        result.into_arg(ret);
    }
}

impl_NativeFunction!(A1);
impl_NativeFunction!(A1, A2);
impl_NativeFunction!(A1, A2, A3);
impl_NativeFunction!(A1, A2, A3, A4);
impl_NativeFunction!(A1, A2, A3, A4, A5);
impl_NativeFunction!(A1, A2, A3, A4, A5, A6);
impl_NativeFunction!(A1, A2, A3, A4, A5, A6, A7);
impl_NativeFunction!(A1, A2, A3, A4, A5, A6, A7, A8);
impl_NativeFunction!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_NativeFunction!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);
impl_NativeFunction!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11);
impl_NativeFunction!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12);

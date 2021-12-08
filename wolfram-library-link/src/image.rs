use std::{ffi::c_void, marker::PhantomData, os::raw::c_int};

use static_assertions::assert_type_eq_all;

use crate::{
    rtl,
    sys::{self, mbool, mint, MImage_CS_Type::*, MImage_Data_Type::*},
};

/// Native Wolfram [`Image`][ref/Image]<sub>WL</sub> or
/// [`Image3D`][ref/Image3D]<sub>WL</sub>.
///
// TODO: This represents a 2-dimensional image.
///
/// Use [`UninitImage::new_2d()`] to construct a new 2-dimensional image.
///
/// [ref/Image]: https://reference.wolfram.com/language/ref/Image.html
/// [ref/Image3D]: https://reference.wolfram.com/language/ref/Image3D.html
pub struct Image<T = ()>(sys::MImage, PhantomData<T>);

/// Represents an allocated [`Image`] whose image data has not yet been initialized.
pub struct UninitImage(sys::MImage);

/// Type of data stored in an [`Image`].
#[repr(i32)]
#[allow(missing_docs)]
pub enum ImageType {
    Bit = MImage_Type_Bit,
    Bit8 = MImage_Type_Bit8,
    Bit16 = MImage_Type_Bit16,
    Real32 = MImage_Type_Real32,
    Real64 = MImage_Type_Real,
}

/// Color space used by an [`Image`].
#[repr(i32)]
#[allow(missing_docs)]
pub enum ColorSpace {
    Automatic = MImage_CS_Automatic,
    CMYK = MImage_CS_CMYK,
    Gray = MImage_CS_Gray,
    HSB = MImage_CS_HSB,
    LAB = MImage_CS_LAB,
    LCH = MImage_CS_LCH,
    LUV = MImage_CS_LUV,
    RGB = MImage_CS_RGB,
    XYZ = MImage_CS_XYZ,
}

/// Position of a pixel position in an [`Image`].
#[derive(Copy, Clone)]
pub enum Pixel {
    /// Index in a 2-dimensional image.
    ///
    /// Fields are `[row, column]`.
    D2([usize; 2]),
    /// Index in a 3-dimensional image.
    ///
    /// Fields are `[slice, row, column]`.
    D3([usize; 3]),
}

impl Pixel {
    /// Construct a pixel position from a slice of indices.
    ///
    /// # Panics
    ///
    /// `pos` must contain exactly 2 or 3 elements, respectively indicating a 2-dimensional
    /// or 3-dimensional position. This function will panic if another length is found.
    pub fn from_slice(pos: &[usize]) -> Self {
        match *pos {
            [row, column] => Pixel::D2([row, column]),
            [slice, row, column] => Pixel::D3([slice, row, column]),
            _ => panic!(
                "Pixel::from_slice: index should have 2 or 3 elements; got {} elements",
                pos.len()
            ),
        }
    }

    fn as_slice(&self) -> &[usize] {
        match self {
            Pixel::D2(array) => array,
            Pixel::D3(array) => array,
        }
    }
}

//======================================
// Traits
//======================================

/// Trait implemented for types that can *logically* be stored in an [`Image`].
///
/// The `STORAGE` associated type represents the data that is *physically* stored in the
/// [`Image`] buffer.
///
/// The following types can be used in an image:
///
/// * [`bool`]
/// * [`u8`], [`u16`]
/// * [`f32`], [`f64`]
///
/// # Safety
///
/// This trait is already implemented for all types that can legally be stored in an
/// [`Image`] implementing this trait for other types may lead to undefined behavior.
pub unsafe trait ImageData: Copy {
    /// The type of the data that is *physically* stored in the [`Image`] buffer.
    ///
    /// In practice, this type is equal to `Self` for every logicaly type except `bool`,
    /// which represents a bitmapped image that logically stores a single bit of boolean
    /// data, but physically allocate one byte for each pixel/channel.
    type STORAGE: Copy;

    #[allow(missing_docs)]
    unsafe fn get_raw(
        image: &Image<Self>,
        pos: *const mint,
        channel: mint,
    ) -> (Self, c_int);

    // TODO: This has the same restrictions as NumericArray::as_slice_mut(), based on
    //       the share_count().
    // fn set(image: &mut Image, pos: &[usize], value: Self);
}

//--------------------------------------
// ImageData Impls
//--------------------------------------

assert_type_eq_all!(i8, sys::raw_t_bit);
assert_type_eq_all!(u8, sys::raw_t_ubit8);
assert_type_eq_all!(u16, sys::raw_t_ubit16);
assert_type_eq_all!(f32, sys::raw_t_real32);
assert_type_eq_all!(f64, sys::raw_t_real64);

unsafe impl ImageData for bool {
    type STORAGE = i8; // sys::raw_t_bit

    unsafe fn get_raw(
        image: &Image<Self>,
        pos: *const mint,
        channel: mint,
    ) -> (Self, c_int) {
        let mut value: i8 = 0;

        let err_code: c_int =
            rtl::MImage_getBit(image.as_raw(), pos as *mut mint, channel, &mut value);

        // FIXME: Is this meant to be non-negative vs negative, or zero vs non-zero?
        //        This currently assumes zero vs non-zero.
        let boole: bool = value != 0;

        (boole, err_code)
    }
}

unsafe impl ImageData for u8 {
    type STORAGE = Self; // sys::raw_t_ubit8

    unsafe fn get_raw(
        image: &Image<Self>,
        pos: *const mint,
        channel: mint,
    ) -> (Self, c_int) {
        let mut value: u8 = 0;

        let err_code: c_int =
            rtl::MImage_getByte(image.as_raw(), pos as *mut mint, channel, &mut value);

        (value, err_code)
    }
}

unsafe impl ImageData for u16 {
    type STORAGE = Self; // sys::raw_t_ubit16

    unsafe fn get_raw(
        image: &Image<Self>,
        pos: *const mint,
        channel: mint,
    ) -> (Self, c_int) {
        let mut value: u16 = 0;

        let err_code: c_int =
            rtl::MImage_getBit16(image.as_raw(), pos as *mut mint, channel, &mut value);

        (value, err_code)
    }
}

unsafe impl ImageData for f32 {
    type STORAGE = Self; // sys::raw_t_real32

    unsafe fn get_raw(
        image: &Image<Self>,
        pos: *const mint,
        channel: mint,
    ) -> (Self, c_int) {
        let mut value: f32 = 0.0;

        let err_code: c_int =
            rtl::MImage_getReal32(image.as_raw(), pos as *mut mint, channel, &mut value);

        (value, err_code)
    }
}

unsafe impl ImageData for f64 {
    type STORAGE = Self; // sys::raw_t_real64

    unsafe fn get_raw(
        image: &Image<Self>,
        pos: *const mint,
        channel: mint,
    ) -> (Self, c_int) {
        let mut value: f64 = 0.0;

        let err_code: c_int =
            rtl::MImage_getReal(image.as_raw(), pos as *mut mint, channel, &mut value);

        (value, err_code)
    }
}

//======================================
// Impls
//======================================

impl<T: ImageData> Image<T> {
    /// Access the data in this [`Image`] as a flat buffer.
    ///
    /// The returned slice will have a length equal to
    /// [`flattened_length()`][Image::flattened_length].
    pub fn as_slice(&self) -> &[T::STORAGE] {
        let raw: *mut c_void = unsafe { self.raw_data() };
        let len: usize = self.flattened_length();

        // Safety: The documentation for `MImage_getRawData` states that the number of
        //         elements is equal to the value obtained by `MImage_getFlattenedLength`.
        unsafe { std::slice::from_raw_parts(raw as *mut T::STORAGE, len) }
    }

    /// Get the value of the specified pixel and channel.
    pub fn get(&self, pixel: Pixel, channel: usize) -> Option<T> {
        let pixel_pos: &[usize] = pixel.as_slice();

        // This is necessary for the `unsafe` call to be valid, otherwise the raw pixel
        // getter function may read an uninitialized value if this is a 3D image but we
        // only provided a 2D index.
        assert_eq!(pixel_pos.len(), self.rank());

        let (value, err_code): (T, c_int) = unsafe {
            <T as ImageData>::get_raw(
                self,
                pixel_pos.as_ptr() as *mut mint,
                channel as mint,
            )
        };

        if err_code != 0 {
            // TODO: Return the error code?
            return None;
        }

        Some(value)
    }
}

impl<T> Image<T> {
    //
    // Properties
    //

    /// The number of elements in the underlying flat data buffer.
    ///
    // TODO: This is the product of ...? See ref/ page.
    ///
    /// *LibraryLink C API Documentation:* [`MImage_getFlattenedLength`](https://reference.wolfram.com/language/LibraryLink/ref/callback/MImage_getFlattenedLength.html)
    pub fn flattened_length(&self) -> usize {
        let len: sys::mint = unsafe { rtl::MImage_getFlattenedLength(self.as_raw()) };

        usize::try_from(len).expect("Image flattened length overflows usize")
    }

    /// *LibraryLink C API Documentation:* [`MImage_getChannels`](https://reference.wolfram.com/language/LibraryLink/ref/callback/MImage_getChannels.html)
    pub fn channels(&self) -> usize {
        let channels: sys::mint = unsafe { rtl::MImage_getChannels(self.as_raw()) };

        usize::try_from(channels).expect("Image channels count overflows usize")
    }

    /// *LibraryLink C API Documentation:* [`MImage_getRank`](https://reference.wolfram.com/language/LibraryLink/ref/callback/MImage_getRank.html)
    pub fn rank(&self) -> usize {
        let rank: sys::mint = unsafe { rtl::MImage_getRank(self.as_raw()) };

        usize::try_from(rank).expect("Image rank overflows usize")
    }

    /// *LibraryLink C API Documentation:* [`MImage_getRowCount`](https://reference.wolfram.com/language/LibraryLink/ref/callback/MImage_getRowCount.html)
    pub fn row_count(&self) -> usize {
        let count: sys::mint = unsafe { rtl::MImage_getRowCount(self.as_raw()) };

        usize::try_from(count).expect("Image row count overflows usize")
    }

    /// *LibraryLink C API Documentation:* [`MImage_getColumnCount`](https://reference.wolfram.com/language/LibraryLink/ref/callback/MImage_getColumnCount.html)
    pub fn column_count(&self) -> usize {
        let count: sys::mint = unsafe { rtl::MImage_getColumnCount(self.as_raw()) };

        usize::try_from(count).expect("Image column count overflows usize")
    }

    /// *LibraryLink C API Documentation:* [`MImage_getSliceCount`](https://reference.wolfram.com/language/LibraryLink/ref/callback/MImage_getSliceCount.html)
    pub fn slice_count(&self) -> usize {
        let count: sys::mint = unsafe { rtl::MImage_getSliceCount(self.as_raw()) };

        usize::try_from(count).expect("Image slice count overflows usize")
    }

    /// Get the color space of this image.
    ///
    /// *LibraryLink C API Documentation:* [`MImage_getColorSpace`](https://reference.wolfram.com/language/LibraryLink/ref/callback/MImage_getColorSpace.html)
    pub fn color_space(&self) -> ColorSpace {
        ColorSpace::try_from(self.color_space_raw())
            .expect("Image color space is not a known ColorSpace variant")
    }

    /// *LibraryLink C API Documentation:* [`MImage_getColorSpace`](https://reference.wolfram.com/language/LibraryLink/ref/callback/MImage_getColorSpace.html)
    pub fn color_space_raw(&self) -> sys::colorspace_t {
        unsafe { rtl::MImage_getColorSpace(self.as_raw()) }
    }

    /// Get the data type of this image.
    pub fn data_type(&self) -> ImageType {
        ImageType::try_from(self.data_type_raw())
            .expect("Image data type is not a known ImageType variant")
    }

    /// *LibraryLink C API Documentation:* [`MImage_getDataType`](https://reference.wolfram.com/language/LibraryLink/ref/callback/MImage_getDataType.html)
    pub fn data_type_raw(&self) -> sys::imagedata_t {
        unsafe { rtl::MImage_getDataType(self.as_raw()) }
    }

    /// *LibraryLink C API Documentation:* [`MImage_alphaChannelQ`](https://reference.wolfram.com/language/LibraryLink/ref/callback/MImage_alphaChannelQ.html)
    pub fn has_alpha_channel(&self) -> bool {
        let boole: mbool = unsafe { rtl::MImage_alphaChannelQ(self.as_raw()) };

        crate::bool_from_mbool(boole)
    }

    /// *LibraryLink C API Documentation:* [`MImage_interleavedQ`](https://reference.wolfram.com/language/LibraryLink/ref/callback/MImage_interleavedQ.html)
    pub fn is_interleaved(&self) -> bool {
        let boole: mbool = unsafe { rtl::MImage_interleavedQ(self.as_raw()) };

        crate::bool_from_mbool(boole)
    }

    /// Returns the share count of this `Image`.
    ///
    /// If this `Image` is not shared, the share count is 0.
    ///
    /// If this `Image` was passed into the current library "by reference" due to
    /// use of the `Automatic` or `"Constant"` memory management strategy, that reference
    /// is not reflected in the `share_count()`.
    ///
    /// *LibraryLink C API Documentation:* [`MImage_shareCount`](https://reference.wolfram.com/language/LibraryLink/ref/callback/MImage_shareCount.html)
    pub fn share_count(&self) -> usize {
        let count: sys::mint = unsafe { rtl::MImage_shareCount(self.as_raw()) };

        usize::try_from(count).expect("Image share count mint overflows usize")
    }

    //
    // Raw Image's
    //

    /// Construct an `Image` from a raw [`MImage`][sys::MImage].
    pub unsafe fn from_raw(raw: sys::MImage) -> Image<T> {
        Image(raw, PhantomData)
    }

    /// Extract the raw [`MImage`][sys::MImage] instance from this `Image`.
    pub unsafe fn into_raw(self) -> sys::MImage {
        let raw = self.as_raw();

        // Don't run Drop on `self`; ownership of this value is being given to the
        // caller.
        std::mem::forget(self);

        raw
    }

    #[allow(missing_docs)]
    #[inline]
    pub unsafe fn as_raw(&self) -> sys::MImage {
        let Image(raw, PhantomData) = *self;

        raw
    }

    /// *LibraryLink C API Documentation:* [`MImage_getRawData`](https://reference.wolfram.com/language/LibraryLink/ref/callback/MImage_getRawData.html)
    pub unsafe fn raw_data(&self) -> *mut c_void {
        rtl::MImage_getRawData(self.as_raw())
    }
}

impl UninitImage {
    /// Construct a new uninitialized `Image` with the specified properties.
    ///
    /// # Panics
    ///
    /// This function will panic if [`UninitImage::try_new_2d()`] returns an error.
    pub fn new_2d(
        width: usize,
        height: usize,
        channels: usize,
        ty: ImageType,
        space: ColorSpace,
        interleaving: bool,
    ) -> UninitImage {
        UninitImage::try_new_2d(width, height, channels, ty, space, interleaving)
            .expect("UninitImage::new_2d: failed to create image")
    }

    /// Construct a new uninitialized 2D image.
    // TODO: Use a better error type than i64.
    pub fn try_new_2d(
        width: usize,
        height: usize,
        channels: usize,
        ty: ImageType,
        space: ColorSpace,
        interleaving: bool,
    ) -> Result<UninitImage, i64> {
        let width = mint::try_from(width).expect("image width overflows `mint`");
        let height = mint::try_from(height).expect("image height overflows `mint`");
        let channels =
            mint::try_from(channels).expect("image channels count overflows `mint`");

        let mut new_raw: sys::MImage = std::ptr::null_mut();

        let err_code: c_int = unsafe {
            rtl::MImage_new2D(
                width,
                height,
                channels,
                ty.as_raw(),
                space.as_raw(),
                mbool::from(interleaving),
                &mut new_raw,
            )
        };

        if err_code != 0 || new_raw.is_null() {
            return Err(i64::from(err_code));
        }

        Ok(UninitImage(new_raw))
    }

    /// Assume that the data in this image has been initialized.
    pub unsafe fn assume_init(self) -> Image {
        let UninitImage(raw) = self;

        // Don't run Drop on `self`; ownership of this value is being given to the caller.
        std::mem::forget(self);

        Image::from_raw(raw)
    }
}

impl ImageType {
    #[allow(missing_docs)]
    pub fn as_raw(self) -> sys::imagedata_t {
        self as i32
    }
}

impl ColorSpace {
    #[allow(missing_docs)]
    pub fn as_raw(self) -> sys::colorspace_t {
        self as i32
    }
}

//======================================
// Trait Impls
//======================================

impl TryFrom<sys::imagedata_t> for ImageType {
    type Error = ();

    fn try_from(value: sys::colorspace_t) -> Result<Self, Self::Error> {
        #[allow(non_upper_case_globals)]
        let ok = match value {
            MImage_Type_Bit => ImageType::Bit,
            MImage_Type_Bit8 => ImageType::Bit8,
            MImage_Type_Bit16 => ImageType::Bit16,
            MImage_Type_Real32 => ImageType::Real32,
            MImage_Type_Real => ImageType::Real64,
            _ => return Err(()),
        };

        Ok(ok)
    }
}

impl TryFrom<sys::colorspace_t> for ColorSpace {
    type Error = ();

    fn try_from(value: sys::colorspace_t) -> Result<Self, Self::Error> {
        use ColorSpace::*;

        #[allow(non_upper_case_globals)]
        let ok = match value {
            MImage_CS_Automatic => Automatic,
            MImage_CS_CMYK => CMYK,
            MImage_CS_Gray => Gray,
            MImage_CS_HSB => HSB,
            MImage_CS_LAB => LAB,
            MImage_CS_LCH => LCH,
            MImage_CS_LUV => LUV,
            MImage_CS_RGB => RGB,
            MImage_CS_XYZ => XYZ,
            _ => return Err(()),
        };

        Ok(ok)
    }
}
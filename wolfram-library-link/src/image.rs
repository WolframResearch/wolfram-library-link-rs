use std::{ffi::c_void, os::raw::c_int};

use crate::{
    rtl,
    sys::{self, mbool, mint, MImage_CS_Type::*, MImage_Data_Type::*},
};

/// Native Wolfram [`Image`][ref/Image]<sub>WL</sub>.
///
// TODO: This represents a 2-dimensional image.
///
/// Use [`UninitImage::new_2d()`] to construct a new 2-dimensional image.
///
/// [ref/Image]: https://reference.wolfram.com/language/ref/Image.html
pub struct Image(sys::MImage);


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

//======================================
// Impls
//======================================

impl Image {
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
    pub unsafe fn from_raw(raw: sys::MImage) -> Image {
        Image(raw)
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

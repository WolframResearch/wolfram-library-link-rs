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
// TODO: Provide better Debug formatting for this type.
#[derive(Debug)]
#[derive(ref_cast::RefCast)]
#[repr(transparent)]
pub struct Image<T = ()>(sys::MImage, PhantomData<T>);

/// Represents an allocated [`Image`] whose image data has not yet been initialized.
pub struct UninitImage<T: ImageData>(sys::MImage, PhantomData<T>);

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

#[allow(missing_docs)]
type Getter<T> = unsafe extern "C" fn(sys::MImage, *mut mint, mint, *mut T) -> c_int;
type Setter<T> = unsafe extern "C" fn(sys::MImage, *mut mint, mint, T) -> c_int;

/// Trait implemented for types that can *logically* be stored in an [`Image`].
///
/// The `STORAGE` associated type represents the data that is *physically* stored in the
/// [`Image`] buffer.
///
/// The following logical types can be used in an image:
///
/// * [`bool`]
/// * [`u8`], [`u16`]
/// * [`f32`], [`f64`]
///
/// # Safety
///
/// This trait is already implemented for all types that can legally be stored in an
/// [`Image`]. Implementing this trait for other types may lead to undefined behavior.
pub unsafe trait ImageData: Copy + Default {
    /// The type of the data that is *physically* stored in the [`Image`] buffer.
    ///
    /// In practice, this type is equal to `Self` for every logicaly type except `bool`,
    /// which represents a bitmapped image that logically stores a single bit of boolean
    /// data, but physically allocate one byte for each pixel/channel.
    type STORAGE: Copy;

    /// The [`ImageType`] variant represented by `Self`.
    const TYPE: ImageType;

    #[allow(missing_docs)]
    fn getter() -> Getter<Self>;

    #[allow(missing_docs)]
    fn setter() -> Setter<Self>;

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
    const TYPE: ImageType = ImageType::Bit;

    fn getter() -> Getter<Self> {
        extern "C" fn bool_getter(
            image: sys::MImage,
            pos: *mut mint,
            channel: mint,
            value: *mut bool,
        ) -> c_int {
            let mut storage: <bool as ImageData>::STORAGE = 0;

            let err_code =
                unsafe { rtl::MImage_getBit(image, pos, channel, &mut storage) };

            if err_code == 0 {
                // FIXME: Is this meant to be non-negative vs negative, or zero vs non-zero?
                //        This currently assumes zero vs non-zero.
                let boole: bool = storage != 0;
                unsafe { *value = boole };
            }

            err_code
        }

        bool_getter
    }

    fn setter() -> Setter<Self> {
        extern "C" fn bool_setter(
            image: sys::MImage,
            pos: *mut mint,
            channel: mint,
            value: bool,
        ) -> c_int {
            unsafe { rtl::MImage_setBit(image, pos, channel, i8::from(value)) }
        }

        bool_setter
    }
}

unsafe impl ImageData for u8 {
    type STORAGE = Self; // sys::raw_t_ubit8
    const TYPE: ImageType = ImageType::Bit8;

    fn getter() -> Getter<Self> {
        *rtl::MImage_getByte
    }

    fn setter() -> Setter<Self> {
        *rtl::MImage_setByte
    }
}

unsafe impl ImageData for u16 {
    type STORAGE = Self; // sys::raw_t_ubit16
    const TYPE: ImageType = ImageType::Bit16;

    fn getter() -> Getter<Self> {
        *rtl::MImage_getBit16
    }

    fn setter() -> Setter<Self> {
        *rtl::MImage_setBit16
    }
}

unsafe impl ImageData for f32 {
    type STORAGE = Self; // sys::raw_t_real32
    const TYPE: ImageType = ImageType::Real32;

    fn getter() -> Getter<Self> {
        *rtl::MImage_getReal32
    }

    fn setter() -> Setter<Self> {
        *rtl::MImage_setReal32
    }
}

unsafe impl ImageData for f64 {
    type STORAGE = Self; // sys::raw_t_real64
    const TYPE: ImageType = ImageType::Real64;

    fn getter() -> Getter<Self> {
        *rtl::MImage_getReal
    }

    fn setter() -> Setter<Self> {
        *rtl::MImage_setReal
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
    ///
    /// # Example
    ///
    /// Get the value of the second channel of the top-left pixel in an image.
    ///
    /// ```no_run
    /// # use wolfram_library_link::{Image, Pixel};
    /// # let image: Image<u8> = todo!();
    /// // let image: Image<u8> = ...
    ///
    /// let value: u8 = image.get(Pixel::D2([0, 0]), 2).unwrap();
    /// ```
    ///
    /// In an [`RGB`][ColorSpace::RGB] image, this is the value of the green channel for
    /// this pixel.
    ///
    /// In an [`HSB`][ColorSpace::HSB] image, this is the value of the saturation for this
    /// pixel.
    pub fn get(&self, pixel: Pixel, channel: usize) -> Option<T> {
        let pixel_pos: &[usize] = pixel.as_slice();

        // This is necessary for the `unsafe` call to be valid, otherwise the raw pixel
        // getter function may read an uninitialized value if this is a 3D image but we
        // only provided a 2D index.
        assert_eq!(pixel_pos.len(), self.rank());

        let getter: unsafe extern "C" fn(_, _, _, _) -> c_int = T::getter();

        let mut value: T = T::default();

        let err_code: c_int = unsafe {
            getter(
                self.as_raw(),
                pixel_pos.as_ptr() as *mut mint,
                channel as mint,
                &mut value,
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

impl<T: ImageData> UninitImage<T> {
    /// Construct a new uninitialized `Image` with the specified properties.
    ///
    /// # Panics
    ///
    /// This function will panic if [`UninitImage::try_new_2d()`] returns an error.
    pub fn new_2d(
        width: usize,
        height: usize,
        channels: usize,
        space: ColorSpace,
        interleaving: bool,
    ) -> UninitImage<T> {
        UninitImage::try_new_2d(width, height, channels, space, interleaving)
            .expect("UninitImage::new_2d: failed to create image")
    }

    /// Construct a new uninitialized 2D image.
    // TODO: Use a better error type than i64.
    pub fn try_new_2d(
        width: usize,
        height: usize,
        channels: usize,
        space: ColorSpace,
        interleaving: bool,
    ) -> Result<UninitImage<T>, i64> {
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
                T::TYPE.as_raw(),
                space.as_raw(),
                mbool::from(interleaving),
                &mut new_raw,
            )
        };

        if err_code != 0 || new_raw.is_null() {
            return Err(i64::from(err_code));
        }

        Ok(UninitImage(new_raw, PhantomData))
    }

    /// Efficiently set every pixel value in this image to zero.
    ///
    /// This fully initializes this image, albeit to a black image.
    pub fn zero(&mut self) {
        let UninitImage(raw, PhantomData) = *self;

        let data_ptr: *mut c_void = unsafe { rtl::MImage_getRawData(raw) };
        let data_ptr = data_ptr as *mut T::STORAGE;
        let len: mint = unsafe { rtl::MImage_getFlattenedLength(raw) };
        let len =
            usize::try_from(len).expect("UninitImage flattened length overflows usize");

        unsafe { std::ptr::write_bytes(data_ptr, 0, len) }
    }

    /// Set the value of the specified pixel and channel.
    ///
    /// # Panics
    ///
    /// This function will panic if the underlying call to [`ImageData::setter()`] fails.
    /// This can happen if the specified `pixel` or `channel` does not exist.
    pub fn set(&mut self, pixel: Pixel, channel: usize, value: T) {
        let pixel_pos: &[usize] = pixel.as_slice();

        let rank = unsafe { rtl::MImage_getRank(self.0) };

        // Assert that we have two indices if this is a 2D image, and three indices if
        // this is a 3D image.
        assert_eq!(pixel_pos.len(), rank as usize);

        let setter: unsafe extern "C" fn(_, _, _, T) -> c_int = T::setter();

        let err_code: c_int = unsafe {
            setter(
                self.0,
                pixel_pos.as_ptr() as *mut mint,
                channel as mint,
                value,
            )
        };

        if err_code != 0 {
            // TODO: Return the error code?
            panic!("Image pixel set() failed with error code {}", err_code);
        }
    }

    /// Assume that the data in this image has been initialized.
    ///
    /// Use [`UninitImage::zero()`] to quickly ensure that every pixel value has been
    /// initialized.
    ///
    /// Use [`UninitImage::set()`] to set individual pixel/channel values.
    ///
    /// # Safety
    ///
    /// This function must only be called once all elements of this image have been
    /// initialized. It is undefined behavior to construct an [`Image`] without first
    /// initializing the data array. In practice, uninitialized values will be essentially
    /// random, causing the resulting image to appear different each time it is created.
    pub unsafe fn assume_init(self) -> Image<T> {
        let UninitImage(raw, PhantomData) = self;

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

    /// Get the string name of this type, suitable for use in
    /// [`Image`][ref/Image]<code>[<i>data</i>, &quot;<i>type</i>&quot;]</code>.
    ///
    /// [ref/Image]: https://reference.wolfram.com/language/ref/Image.html
    pub fn name(&self) -> &'static str {
        match self {
            ImageType::Bit => "Bit",
            // TODO: Is "Bit8" supported by LibraryLink? The C enum name uses Bit8, but
            //       the ref/Image docs and the LibraryLink User Guide both say "Byte" is
            //       the WL name for this type.
            //
            //       There is a similar inconsistence with Real vs Real64: the User Guide
            //       lists "Real32" and "Real", but ref/Image uses "Real32" and "Real64".
            ImageType::Bit8 => "Byte",
            ImageType::Bit16 => "Bit16",
            ImageType::Real32 => "Real32",
            ImageType::Real64 => "Real64",
        }
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

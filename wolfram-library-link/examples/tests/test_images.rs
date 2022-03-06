use wolfram_library_link::{
    self as wll, ColorSpace, Image, NumericArray, Pixel, UninitImage, UninitNumericArray,
};


#[wll::export]
fn test_image_arg(image: &Image<bool>) -> NumericArray<i8> {
    let mut array = UninitNumericArray::from_dimensions(&[image.flattened_length()]);

    for pair in image
        .as_slice()
        .iter()
        .zip(array.as_slice_mut().into_iter())
    {
        let (pixel, elem): (&i8, &mut std::mem::MaybeUninit<i8>) = pair;
        elem.write(*pixel);
    }

    // Safety: We iterated over every element in `array` and initialized it with the
    //         corresponding pixel value from `image`.
    unsafe { array.assume_init() }
}

// fn test_image_manual_arg(image: Image<bool>) -> NumericArray<bool> {
//     todo!("")
// }

#[wll::export]
fn test_create_bitmap_image() -> Image<bool> {
    let width = 2;
    let height = 2;
    let channels = 1;

    let mut image = UninitImage::<bool>::new_2d(
        width,
        height,
        channels,
        ColorSpace::Automatic,
        false,
    );

    image.set(Pixel::D2([1, 1]), 1, false);
    image.set(Pixel::D2([1, 2]), 1, true);
    image.set(Pixel::D2([2, 1]), 1, true);
    image.set(Pixel::D2([2, 2]), 1, false);

    unsafe { image.assume_init() }
}

/// Create an image with four pixels, where the top left image is red, the top right
/// pixel is green, the bottom left pixel is blue, and the bottom right pixel is light
/// gray.
#[wll::export]
fn test_create_color_rgb_u8_image() -> Image<u8> {
    let width = 2;
    let height = 2;
    let channels = 3; // Red, green, and blue.

    let mut image: UninitImage<u8> =
        UninitImage::new_2d(width, height, channels, ColorSpace::RGB, false);

    // Red, green, and blue channels indices.
    const R: usize = 1;
    const G: usize = 2;
    const B: usize = 3;

    // Set every pixel value to black. The image data is otherwise completely
    // uninitialized memory, and can contain arbitrary values.
    image.zero();

    // Set the top left, top right, and bottom left pixels on only one color channel.
    image.set(Pixel::D2([1, 1]), R, u8::MAX);
    image.set(Pixel::D2([1, 2]), G, u8::MAX);
    image.set(Pixel::D2([2, 1]), B, u8::MAX);

    // Make this pixel white, by setting R, G, and B channels to ~80%.
    image.set(Pixel::D2([2, 2]), R, 200);
    image.set(Pixel::D2([2, 2]), G, 200);
    image.set(Pixel::D2([2, 2]), B, 200);

    unsafe { image.assume_init() }
}

/// Create an image with four pixels, where the top left image is red, the top right
/// pixel is green, the bottom left pixel is blue, and the bottom right pixel is light
/// gray.
#[wll::export]
fn test_create_color_rgb_f32_image() -> Image<f32> {
    let width = 2;
    let height = 2;
    let channels = 3; // Red, green, and blue.

    let mut image: UninitImage<f32> =
        UninitImage::new_2d(width, height, channels, ColorSpace::RGB, false);

    // Red, green, and blue channels indices.
    const R: usize = 1;
    const G: usize = 2;
    const B: usize = 3;

    // Set every pixel value to black. The image data is otherwise completely
    // uninitialized memory, and can contain arbitrary values.
    image.zero();

    // Set the top left, top right, and bottom left pixels on only one color channel.
    image.set(Pixel::D2([1, 1]), R, 1.0);
    image.set(Pixel::D2([1, 2]), G, 1.0);
    image.set(Pixel::D2([2, 1]), B, 1.0);

    // Make this pixel white, by setting R, G, and B channels to 80%.
    image.set(Pixel::D2([2, 2]), R, 0.8);
    image.set(Pixel::D2([2, 2]), G, 0.8);
    image.set(Pixel::D2([2, 2]), B, 0.8);

    unsafe { image.assume_init() }
}

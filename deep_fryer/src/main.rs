extern crate image;
use image::GenericImageView;
use image::jpeg::{JPEGDecoder, JPEGEncoder};
use std::fs::File;
use image::ImageDecoder;
use image::DecodingResult;
use image::ColorType;

// Compiles and runs, but produces nonsense output
fn main() {
    let f = File::open("test.jpg").unwrap();
    let mut decoder = JPEGDecoder::new(f);
    let dims = decoder.dimensions().unwrap();
    println!("ct: {:?}", decoder.colortype());
    let img = decoder.read_image().unwrap();

    match img {
        DecodingResult::U8(to_encode) => {
            println!("Processing vec of u8s");
            let mut f_out = File::create("test_encoded.jpg").unwrap();
            let mut encoder = JPEGEncoder::new_with_quality(&mut f_out, 100);
            encoder.encode(
                to_encode.as_slice(),
                decoder.dimensions().unwrap().0,
                decoder.dimensions().unwrap().1,
                ColorType::RGB(8)
            );
        },
        DecodingResult::U16(vec_of_u16s) => {
            println!("Got a u16, sorry");
        },
    }
}

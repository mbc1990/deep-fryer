extern crate image;
use image::GenericImageView;
use image::jpeg::{JPEGDecoder, JPEGEncoder};
use std::fs::File;
use image::ImageDecoder;
use image::DecodingResult;
use image::ColorType;

// Compiles and runs, but produces nonsense output
fn main() {
    let img = image::open("test.jpg").unwrap();
    let mut out: Vec<u8> = Vec::new();
    let mut encoder = JPEGEncoder::new_with_quality(&mut out, 10);
    encoder.encode(
        img.as_rgb8().unwrap(),
        img.width(),
        img.height(),
        ColorType::RGB(8)
    );
    println!("Out: {:?}", out);
    image::save_buffer("test_processed.jpg", &out[..], img.width(), img.height(), image::RGB(8)).unwrap()
}

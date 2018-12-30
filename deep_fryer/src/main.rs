extern crate image;
extern crate rand;
use image::jpeg::{JPEGDecoder, JPEGEncoder};
use std::fs::File;
use image::ImageDecoder;
use image::DecodingResult;
use image::ColorType;
use rand::Rng;


fn main() {

    // TODO: Do this with reusable buffers, not reading/writing to filesystem each time
    let mut rng = rand::thread_rng();
    for i in 0..500 {
        let f = File::open("test.jpg").unwrap();
        let mut decoder = JPEGDecoder::new(f);
        let dims = decoder.dimensions().unwrap();
        let img = decoder.read_image().unwrap();

        match img {
            DecodingResult::U8(to_encode) => {
                let mut f_out = File::create("test.jpg").unwrap();
                let quality = rng.gen_range(70, 80);
                println!("Iteration: {} with quality: {}", i, quality);
                let mut encoder = JPEGEncoder::new_with_quality(&mut f_out, quality);
                let res = encoder.encode(
                    to_encode.as_slice(),
                    dims.0,
                    dims.1,
                    ColorType::RGB(8)
                );
                match res {
                    Ok(_) => {},
                    Err(err) => {
                        println!("Err: {}", err);
                    }
                }
            },
            DecodingResult::U16(_to_encode) => {
                println!("Got a u16, sorry");
            },
        }
    }
}

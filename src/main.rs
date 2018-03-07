extern crate image;

use std::io::Read;
use std::fs::File;

fn main() {
    let mut f = File::open("dump.bin").unwrap();

    let mut data = Vec::new();
    f.read_to_end(&mut data).unwrap();


    {
        let mut writer = File::create("dump.jpg").unwrap();
        let mut encoder = image::jpeg::JPEGEncoder::new_with_quality(&mut writer, 100);
        encoder.encode(
            &data,
            512,
            512,
            image::ColorType::Gray(8)).expect("TODO: encoding");
    }
    {
        let mut writer = File::create("dump.png").unwrap();
        let mut encoder = image::png::PNGEncoder::new(&mut writer);
        encoder.encode(
            &data,
            512,
            512,
            image::ColorType::Gray(8)).expect("TODO: encoding");
    }
}

extern crate image;
extern crate stb_image;

use std::fs;
use std::fs::File;
use std::io::{prelude::*, Cursor};

use std::time::Instant;

use image::ImageFormat;

fn main() {
    let paths = fs::read_dir(r"D:\Projects\Hebron\TestImages").unwrap();

    let mut loadTime1: u128 = 0;
    let mut loadTime2: u128 = 0;

    for path2 in paths {
        let path = path2.unwrap().path();

        if path.extension().is_none() {
            continue;
        }

        let extStr = path.extension().unwrap().to_str().unwrap();
        if extStr != "png"
            && extStr != "jpg"
            && extStr != "bmp"
            && extStr != "gif"
            && extStr != "psd"
            && extStr != "tga"
        {
            continue;
        }

        println!("Name: {}", path.display());

        // Load file into memory
        let mut f = File::open(path).expect("file not found");

        let mut contents = vec![];
        f.read_to_end(&mut contents);

        // Load image
        let now = Instant::now();
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut comp: i32 = 0;

        let mut buffer: Vec<u8>;
        unsafe {
            let img = stb_image::stbi_load_from_memory(
                contents.as_mut_ptr(),
                contents.len() as i32,
                &mut x,
                &mut y,
                &mut comp,
                stb_image::STBI_rgb_alpha,
            );
            stb_image::c_runtime::free(img);
        }

        loadTime1 += now.elapsed().as_millis();

        let now2 = Instant::now();

        let img2 = image::load_from_memory(&contents.as_mut());

        loadTime2 += now2.elapsed().as_millis();

        println!(
            "Width: {}, Height: {}, Comp: {}, Load Time: {} ms, Load Time 2: {}",
            x, y, comp, loadTime1, loadTime2
        );
    }
}

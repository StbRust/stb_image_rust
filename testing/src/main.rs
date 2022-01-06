extern crate image;
extern crate stb_image;
extern crate stb_image_rust;

use std::fs;
use std::fs::File;
use std::io::{prelude::*, Cursor};

use std::time::Instant;

use image::ImageFormat;
use stb_image::image::LoadResult;

fn main() {
    let paths = fs::read_dir(r"D:\Projects\Hebron\TestImages").unwrap();

    let mut loadTime1: u128 = 0;
    let mut loadTime2: u128 = 0;
    let mut loadTime3: u128 = 0;

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

        let img: *mut u8;
        unsafe {
            img = stb_image_rust::stbi_load_from_memory(
                contents.as_ptr(),
                contents.len() as i32,
                &mut x,
                &mut y,
                &mut comp,
                stb_image_rust::STBI_rgb_alpha,
            );
        }

        loadTime1 += now.elapsed().as_millis();

        let now2 = Instant::now();

        let img2Wrapped =
            stb_image::image::load_from_memory_with_depth(&contents.as_mut(), 4, true);

        if let LoadResult::ImageU8(img2) = img2Wrapped {
            'outer: loop {
                if (x != img2.width as i32) {
                    println!("width mismatch: {} {}", x, img2.width);
                    break;
                }

                if (y != img2.height as i32) {
                    println!("height mismatch: {} {}", y, img2.height);
                    break;
                }

                let len = x * y * 4;
                if (len != img2.data.len() as i32) {
                    println!("len mismatch: {} {}", len, img2.data.len());
                    break;
                }

                for i in 0..len - 1 {
                    unsafe {
                        if (*img.offset(i as isize) != img2.data[i as usize]) {
                            println!(
                                "data mismatch: {} {} {}",
                                i,
                                *img.offset(i as isize),
                                img2.data[i as usize]
                            );
                            break 'outer;
                        }
                    }
                }

                break;
            }
        } else {
            println!("image isnt ImageU8");
        }

        loadTime2 += now2.elapsed().as_millis();

        unsafe {
            stb_image_rust::c_runtime::free(img);
        }

        let now3 = Instant::now();

        let img3 = image::load_from_memory(&contents.as_mut());

        loadTime3 += now3.elapsed().as_millis();

        println!(
            "Width: {}, Height: {}, Comp: {}, Load Time: {} ms, Load Time 2: {}, Load Time 3: {}",
            x, y, comp, loadTime1, loadTime2, loadTime3
        );
    }
}

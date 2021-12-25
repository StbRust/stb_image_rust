extern crate piston_window;
extern crate image;
extern crate stb_image;

use std::env;
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::prelude::*;
use piston_window::*;
use stb_image::*;

fn main() {
/*    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
            .exit_on_esc(true).build().unwrap();

    // Load file into memory
    let mut f = File::open("D:\\Temp\\pipeline.png").expect("file not found");

    let mut contents = vec![];
    f.read_to_end(&mut contents);

    // Load image
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut comp: i32 = 0;

    let mut buffer: Vec<u8>;
    unsafe {
        let img = stb_image::stb_image::stbi_load_from_memory(contents.as_mut_ptr(), contents.len() as i32,
                                                     &mut x, &mut y, &mut comp, stb_image::stb_image::STBI_rgb_alpha);
        let size = x * y * 4;
        buffer = Vec::<u8>::with_capacity(size as usize);
        for i in 0..size {
            buffer.push(*img.offset(i as isize));
        }

        stb_image::c_runtime::free(img);
    }

    println!("X: {}, Y: {}, COMP: {}", x, y, comp);

    let image_buffer = image::RgbaImage::from_raw(x as u32, y as u32, buffer).unwrap();

    let texture = Texture::from_image(&mut window.factory,
                                             &image_buffer,
                                             &TextureSettings::new()
    ).unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([0.0; 4], graphics);
            image(&texture, context.transform, graphics);
        });
    }*/

    let mut g: stbi__gif = Default::default();

    g.codes[2].prefix = 100;


    println!("Hello, world!");
}
### About
stb_image_rust is Rust port of the famous C framework: [https://github.com/nothings/stb](https://github.com/nothings/stb)

It is important to note, that this project is **port**, not **wrapper**. Original C code had been ported to Rust. Therefore stb_image_rust doesnt require any native binaries.

The porting hasn't been done by hand, but using [Hebron](https://github.com/rds1983/Hebron).

### Crate
https://crates.io/crates/stb_image_rust

### Sample Code
```
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut comp: i32 = 0;

        let img: *mut u8;
		
	// Load the image
        unsafe {
            img = stb_image_rust::stbi_load_from_memory(
                contents.as_mut_ptr(),
                contents.len() as i32,
                &mut x,
                &mut y,
                &mut comp,
                stb_image_rust::STBI_rgb_alpha,
            );
        }
		
	// Do something with it
	...
		
	// Free the allocated memory
        unsafe {
            stb_image_rust::c_runtime::free(img);
        }		
```

### License
Public Domain

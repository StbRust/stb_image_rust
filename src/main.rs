pub mod c_runtime;
mod stb_image;

struct internal_struct
{
    a:i32,
    b:i32
}

struct my_struct
{
    data:[internal_struct;10],
    r:i32
}

fn test(ms:&mut my_struct)
{
    ms.data[0].a = 5;
}

fn main() {
//    let mut ms:my_struct = unsafe {std::mem::zeroed()};
/*    let ms:*mut my_struct = std::ptr::null_mut();

    if ms == std::ptr::null_mut()
    {

    }*/

    let mut data:[i32;10] = unsafe {std::mem::uninitialized()};

    data[0] = 5;
//    println!("OUTPUT: {}", ms.data[0].a);
}

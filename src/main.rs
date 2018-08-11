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
    r:i32,
    f:fn(i32) -> i32
}

fn fff(v: i32) -> i32
{
    return v * v;
}

unsafe fn test(ms:*mut my_struct)
{
    (*ms).f = fff;
    (*ms).data[0].a = 15;
}

fn main() {
    let mut ms:my_struct = unsafe {std::mem::uninitialized()};
/*    let ms:*mut my_struct = std::ptr::null_mut();

    if ms == std::ptr::null_mut()
    {

    }*/

/*    let mut d:Vec<i32> = vec![1, 2, 3, 4, 5];
    let data:*mut i32 = d.as_mut_ptr();

    unsafe {
        *(data.offset(3)) = 10;

        println!("OUTPUT: {}", *data.offset(3));
    }*/

    unsafe {
        test(&mut ms);

        ms.data[0].a = (ms.f)(ms.data[0].a);

        println!("OUTPUT: {}", ms.data[0].a);
    }
}

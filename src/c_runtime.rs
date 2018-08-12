use std;
use std::alloc;
use std::mem;

pub unsafe fn memcpy(src:*mut u8, dest:*mut u8, count: u64) {
    for i in 0..count {
        *src.offset(i as isize) = *dest.offset(i as isize);
    }
}

pub unsafe fn memset(src:*mut u8, value:i32, count: u64) {
    for i in 0..count {
        *src.offset(i as isize) = value as u8;
    }
}

pub unsafe fn malloc(count: u64) -> *mut u8 {
    let layout = std::alloc::Layout::from_size_align(count as usize, 1)
        .expect("Bad layout");

    return std::alloc::alloc(layout);
}

pub unsafe fn realloc<T>(data:*mut T, count: u64) -> *mut u8 {
    if (data == std::ptr::null_mut()) {
        return malloc(count);
    }

    let layout = std::alloc::Layout::from_size_align(count as usize, 1)
        .expect("Bad layout");

    return std::alloc::realloc(data as *mut u8, layout, count as usize);
}

pub unsafe fn free<T>(data:*mut T) {
    let layout = std::alloc::Layout::from_size_align(1, 1)
        .expect("Bad layout");

    std::alloc::dealloc(data as *mut u8, layout);
}

pub fn _lrotl(x:u32, y:i32) -> u32
{
    return (x << y) | (x >> (32 - y));
}

pub fn abs(x:i32) -> i32
{
    return i32::abs(x);
}
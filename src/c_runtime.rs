use std;

pub fn memcpy(src:*mut u8, dest:*mut u8, count: u64) {
}

pub fn memset(src:*mut u8, value:i32, len: u64) {
}

pub fn malloc(count: u64) -> *mut u8 {
    return std::ptr::null_mut();
}

pub fn realloc<T>(data:*mut T, count: u64) -> *mut u8 {
    return std::ptr::null_mut();
}

pub fn free<T>(data:*mut T) {
}

pub fn _lrotl(x:u32, y:i32) -> u32
{
    return (x << y) | (x >> (32 - y));
}

pub fn abs(x:i32) -> i32
{
    return 0;
}
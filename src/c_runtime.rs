pub fn memcpy(src:*mut u8, dest:*mut u8, count: u64) {
}

pub fn memset(src:*mut u8, value:i32, len: u64) {
}

pub fn malloc(count: u64) -> *mut u8 {
}

pub fn realloc(data:*mut u8, count: u64) -> *mut u8 {
}

pub fn free(data:*mut u8) {
}

pub fn _lrotl(x:u32, y:i32) -> u32
{
    (x << y) | (x >> (32 - y));
}

pub fn abs(x:i32) -> i32
{
    0;
}
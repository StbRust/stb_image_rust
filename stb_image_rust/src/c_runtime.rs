use std;
use std::alloc;
use std::mem;

pub trait One {
    fn one() -> Self;
}

impl One for i32 {
    fn one() -> Self {
        1
    }
}

impl One for u32 {
    fn one() -> Self {
        1
    }
}

pub unsafe fn postInc<T: std::ops::AddAssign + One + Copy>(mut a: *mut T) -> T {
    let mut result: T = *a;
    *a += One::one();
    return result;
}

pub unsafe fn preInc<T: std::ops::AddAssign + One + Copy>(mut a: *mut T) -> T {
    *a += One::one();
    return *a;
}

pub unsafe fn postDec<T: std::ops::SubAssign + One + Copy>(mut a: *mut T) -> T {
    let mut result: T = *a;
    *a -= One::one();
    return result;
}

pub unsafe fn preDec<T: std::ops::SubAssign + One + Copy>(mut a: *mut T) -> T {
    *a -= One::one();
    return *a;
}

pub unsafe fn preIncPtr<T>(mut a: *mut *mut T) -> *mut T {
    *a = (*a).offset(1);
    return *a;
}

pub unsafe fn preDecPtr<T>(mut a: *mut *mut T) -> *mut T {
    *a = (*a).offset(-1);
    return *a;
}

pub unsafe fn postIncPtr<T>(mut a: *mut *mut T) -> *mut T {
    let mut result: *mut T = *a;
    *a = (*a).offset(1);
    return result;
}

pub unsafe fn postDecPtr<T>(mut a: *mut *mut T) -> *mut T {
    let mut result: *mut T = *a;
    *a = (*a).offset(-1);
    return result;
}

pub unsafe fn memcpy(src: *mut u8, dest: *mut u8, count: u64) {
    std::ptr::copy_nonoverlapping(dest, src, count as usize);
}

pub unsafe fn memset(src: *mut u8, value: i32, count: u64) {
    std::ptr::write_bytes(src, value as u8, count as usize);
}

pub unsafe fn malloc(count: u64) -> *mut u8 {
    let layout = std::alloc::Layout::from_size_align(count as usize, 1).expect("Bad layout");

    return std::alloc::alloc(layout);
}

pub unsafe fn realloc<T>(data: *mut T, count: u64) -> *mut u8 {
    if (data == std::ptr::null_mut()) {
        return malloc(count);
    }

    let layout = std::alloc::Layout::from_size_align(count as usize, 1).expect("Bad layout");

    return std::alloc::realloc(data as *mut u8, layout, count as usize);
}

pub unsafe fn free<T>(data: *mut T) {
    let layout = std::alloc::Layout::from_size_align(1, 1).expect("Bad layout");

    std::alloc::dealloc(data as *mut u8, layout);
}

pub fn _lrotl(x: u32, y: i32) -> u32 {
    return (x << y) | (x >> (32 - y));
}

pub fn abs(x: i32) -> i32 {
    return i32::abs(x);
}

pub fn pow(x: f64, p: f64) -> f64 {
    return 0.0f64;
}

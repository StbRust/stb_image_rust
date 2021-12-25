static stbi__g_failure_reason: &'static str = "";

#[derive(Clone, Copy)]
pub struct stbi_io_callbacks {
	read: Option<fn(*mut u8, *mut i8, i32) -> i32>,
	skip: fn(*mut u8, i32),
	eof: fn(*mut u8) -> i32,
}

struct stbi__resample {
	resample: unsafe fn(*mut u8, *mut u8, *mut u8, i32, i32) -> *mut u8,
	line0: *mut u8,
	line1: *mut u8,
	hs: i32,
	vs: i32,
	w_lores: i32,
	ystep: i32,
	ypos: i32,
}

struct stbi__jpeg {
/*    s: *mut stbi__context,
    huff_dc: [stbi__huffman; 4],
    huff_ac: [stbi__huffman; 4],
    dequant: [[u16; 64]; 4],
    fast_ac: [[i16; 512]; 4],
    img_h_max: i32,
    img_v_max: i32,
    img_mcu_x: i32,
    img_mcu_y: i32,
    img_mcu_w: i32,
    img_mcu_h: i32,
    img_comp: [img_comp; 4],
    code_buffer: u32,
    code_bits: i32,
    marker: u8,
    nomore: i32,
    progressive: i32,
    spec_start: i32,
    spec_end: i32,
    succ_high: i32,
    succ_low: i32,
    eob_run: i32,
    jfif: i32,
    app14_color_transform: i32,
    rgb: i32,
    scan_n: i32,
    order: [i32; 4],
    restart_interval: i32,
    todo: i32,
    idct_block_kernel: unsafe fn(*mut u8, i32, *mut i16),
    YCbCr_to_RGB_kernel: unsafe fn(*mut u8, *mut u8, *mut u8, *mut u8, i32, i32),
    resample_row_hv_2_kernel: unsafe fn(*mut u8, *mut u8, *mut u8, i32, i32)*/
}

unsafe fn stbi__tga_test(s: *mut stbi__context) -> i32 {
/*    let res: i32 = (i32)(0);
    let sz: i32;
    let tga_color_type: i32;
    stbi__get8(s);
    tga_color_type = (i32)(stbi__get8(s));
    if tga_color_type > 1 { goto errorEnd; }
    sz = (i32)(stbi__get8(s));
    if tga_color_type == 1 {
        if sz != 1 && sz != 9 { goto errorEnd; }
        stbi__skip(s, 4);
        sz = (i32)(stbi__get8(s));
        if sz != 8 && sz != 15 && sz != 16 && sz != 24 && sz != 32 { goto errorEnd; }
        stbi__skip(s, 4);
    } else {
        if sz != 2 && sz != 3 && sz != 10 && sz != 11 { goto errorEnd; }
        stbi__skip(s, 9);
    }
    if stbi__get16le(s) < 1 { goto errorEnd; }
    if stbi__get16le(s) < 1 { goto errorEnd; }
    sz = (i32)(stbi__get8(s));
    if tga_color_type == 1 && sz != 8 && sz != 16 { goto errorEnd; }
    if sz != 8 && sz != 15 && sz != 16 && sz != 24 && sz != 32 { goto errorEnd; }
    res = (i32)(1);
    errorEnd: ;
    stbi__rewind(s);
    return (i32)(res);*/
    return 0;
}

pub fn stbi__err(s: &str) -> i32 {
//    stbi__g_failure_reason = s;
    return 0;
}

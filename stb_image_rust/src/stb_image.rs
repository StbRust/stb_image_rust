pub static mut stbi__g_failure_reason: &str = "";

pub unsafe fn stbi__err(s: &str) -> i32 {
    //    stbi__g_failure_reason = s;
    return 0;
}

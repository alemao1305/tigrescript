func print(s: string) {
    @extern("libc", "puts")
    native func puts(ptr: *u8) -> i32
    
    unsafe { puts(s.to_cstr()) }
}
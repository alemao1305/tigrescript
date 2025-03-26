@extern("libc", "printf")
native func printf(fmt: *u8, ...) -> i32

@extern("libc", "scanf")
native func scanf(fmt: *u8, ...) -> i32

func print(s: string) {
    unsafe { printf("%s\0".as_ptr(), s.as_ptr()) }
}

func read_line() -> string {
    let mut buf = [0u8; 256];
    unsafe { scanf("%255s\0".as_ptr(), buf.as_mut_ptr()) };
    buf.to_string()
}
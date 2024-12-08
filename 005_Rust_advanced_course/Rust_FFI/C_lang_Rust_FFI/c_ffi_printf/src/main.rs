use libc::printf;

fn main() {
    unsafe {
        let print_x = (r#"hello c__FFI(Rust Lang)"#.as_ptr()) as *const i8;
        printf(print_x);
    }
}

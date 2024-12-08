// C 함수를 선언합니다.
extern "C" {
    fn c_function_name(arg1: i32, arg2: i32) -> i32;
}

// 러스트에서 C 함수를 호출합니다.
fn call_c_function() {
    unsafe {
        c_function_name(1, 2);
    }
}

#[no_mangle]
pub extern "C" fn rust_function_name(arg1: i32, arg2: i32) -> i32 {
    // 함수 본문
    arg1 + arg2
}

fn main() {
    // C에서 메모리 할당
    let arr = std::ptr::null_mut();
    // 메모리 해제
    // Note: You don't need to manually free the memory in Rust.
    // Rust's garbage collector will handle it for you.
    // However, if you want to manually free the memory, you can use the following code:
    unsafe {
        libc::free(arr as *mut libc::c_void);
    }
    println!("C fn : arr: {:?}", arr);

    // C funcfion
    // call_c_function();
}

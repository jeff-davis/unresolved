extern "C" {
    static unresolved: i32;
}

#[no_mangle]
extern "C" fn myfunc() {
    unsafe {
        println!("{}", unresolved);
    }
}

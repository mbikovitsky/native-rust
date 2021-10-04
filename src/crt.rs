// https://github.com/rust-lang/rust/pull/64990/files

#[no_mangle]
#[used]
static mut _fltused: i32 = 0x9875;  // See: crt\src\vcruntime\fltused.cpp

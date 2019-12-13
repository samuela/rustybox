use libc;

#[no_mangle]
pub static mut bb_common_bufsiz1: [libc::c_char; 1024] = [0; 1024];

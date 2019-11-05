use libc;

// "TRAILER!!!"
#[no_mangle]
pub static mut cpio_TRAILER: [libc::c_char; 11] = [84, 82, 65, 73, 76, 69, 82, 33, 33, 33, 0];

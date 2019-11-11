
use libc;
#[no_mangle]
pub unsafe extern "C" fn true_main(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int {
  0
}

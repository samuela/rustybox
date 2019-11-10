use libc;
use libc::access;
use libc::atoi;
use libc::fclose;
use libc::fprintf;
use libc::lstat;
use libc::printf;
use libc::puts;
use libc::rename;
use libc::rmdir;
use libc::sprintf;
use libc::strchr;
use libc::strcmp;
use libc::strrchr;
use libc::strstr;
use libc::system;





#[no_mangle]
pub unsafe extern "C" fn true_main(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int {
  0
}

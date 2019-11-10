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





extern "C" {


  #[no_mangle]
  fn bb_show_usage() -> !;
}

#[no_mangle]
pub unsafe extern "C" fn single_argv(mut argv: *mut *mut libc::c_char) -> *mut libc::c_char {
  if !(*argv.offset(1)).is_null()
    && strcmp(
      *argv.offset(1),
      b"--\x00" as *const u8 as *const libc::c_char,
    ) == 0
  {
    argv = argv.offset(1)
  }
  if (*argv.offset(1)).is_null() || !(*argv.offset(2)).is_null() {
    bb_show_usage();
  }
  return *argv.offset(1);
}

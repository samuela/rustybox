
use crate::libbb::appletlib::applet_name;
use libc;
use libc::strcmp;
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

use crate::libbb::ptr_to_globals::bb_errno;
use libc;
use libc::close;
use libc::off_t;
extern "C" {

  #[no_mangle]
  fn bb_copyfd_eof(fd1: libc::c_int, fd2: libc::c_int) -> off_t;

  #[no_mangle]
  fn open_or_warn_stdin(pathname: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  static bb_argv_dash: [*const libc::c_char; 0];
}

#[no_mangle]
pub unsafe extern "C" fn bb_cat(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut fd: libc::c_int = 0;
  let mut retval: libc::c_int = 0i32;
  if (*argv).is_null() {
    argv = &bb_argv_dash as *const [*const libc::c_char; 0] as *mut *mut libc::c_char
  }
  let mut current_block_5: u64;
  loop {
    fd = open_or_warn_stdin(*argv);
    if fd >= 0i32 {
      /* This is not a xfunc - never exits */
      let mut r: off_t = bb_copyfd_eof(fd, 1i32);
      if fd != 0i32 {
        close(fd);
      }
      if r >= 0 {
        current_block_5 = 16658872821858055392;
      } else {
        current_block_5 = 3640593987805443782;
      }
    } else {
      current_block_5 = 3640593987805443782;
    }
    match current_block_5 {
      3640593987805443782 => retval = 1i32,
      _ => {}
    }
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  return retval;
}

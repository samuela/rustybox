use libc;
extern "C" {
  #[no_mangle]
  fn time(__timer: *mut time_t) -> time_t;
  /* Note: does not use stdio, writes to fd 2 directly */
  #[no_mangle]
  fn bb_putchar_stderr(ch: libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  // NB: will return short write on error, not -1,
  // if some data was written before error occurred
  #[no_mangle]
  fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
}

use crate::librb::size_t;
use crate::librb::ssize_t;
use libc::time_t;
/* Read a block of data from stdin, write it to stdout.
 * Activity is indicated by a '.' to stderr
 */
#[no_mangle]
pub unsafe extern "C" fn pipe_progress_main(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut buf: [libc::c_char; 4096] = [0; 4096];
  let mut t: time_t = time(0 as *mut time_t);
  let mut len: libc::c_int = 0;
  loop {
    len = safe_read(
      0i32,
      buf.as_mut_ptr() as *mut libc::c_void,
      4096i32 as size_t,
    ) as libc::c_int;
    if !(len > 0i32) {
      break;
    }
    let mut new_time: time_t = time(0 as *mut time_t);
    if new_time != t {
      t = new_time;
      bb_putchar_stderr('.' as i32 as libc::c_char);
    }
    full_write(1i32, buf.as_mut_ptr() as *const libc::c_void, len as size_t);
  }
  bb_putchar_stderr('\n' as i32 as libc::c_char);
  return 0i32;
}

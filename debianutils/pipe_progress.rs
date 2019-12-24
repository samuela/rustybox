use crate::librb::size_t;
use libc;
use libc::time;
use libc::time_t;
extern "C" {

  /* Note: does not use stdio, writes to fd 2 directly */

  // NB: will return short write on error, not -1,
  // if some data was written before error occurred

}

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
    len = crate::libbb::read::safe_read(0, buf.as_mut_ptr() as *mut libc::c_void, 4096i32 as size_t)
      as libc::c_int;
    if !(len > 0) {
      break;
    }
    let mut new_time: time_t = time(0 as *mut time_t);
    if new_time != t {
      t = new_time;
      crate::libbb::xfuncs::bb_putchar_stderr('.' as i32 as libc::c_char);
    }
    crate::libbb::full_write::full_write(
      1i32,
      buf.as_mut_ptr() as *const libc::c_void,
      len as size_t,
    );
  }
  crate::libbb::xfuncs::bb_putchar_stderr('\n' as i32 as libc::c_char);
  return 0;
}

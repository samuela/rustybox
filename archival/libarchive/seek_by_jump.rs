use libc;
extern "C" {
  #[no_mangle]
  fn lseek(__fd: libc::c_int, __offset: __off64_t, __whence: libc::c_int) -> __off64_t;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn seek_by_read(fd: libc::c_int, amount: off_t);
}
pub type __off64_t = libc::c_long;
use crate::librb::off_t;

/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn seek_by_jump(mut fd: libc::c_int, mut amount: off_t) {
  if amount != 0 && lseek(fd, amount, 1i32) == -1i32 as off_t {
    if *bb_errno == 29i32 {
      seek_by_read(fd, amount);
    } else {
      bb_simple_perror_msg_and_die(b"seek failure\x00" as *const u8 as *const libc::c_char);
    }
  };
}

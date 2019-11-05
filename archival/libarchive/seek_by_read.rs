use libc;
extern "C" {
  #[no_mangle]
  fn bb_copyfd_exact_size(fd1: libc::c_int, fd2: libc::c_int, size: off_t);
}
pub type __off64_t = libc::c_long;
use crate::librb::off_t;
/* vi: set sw=4 ts=4: */
/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/*  If we are reading through a pipe, or from stdin then we can't lseek,
 *  we must read and discard the data to skip over it.
 */
#[no_mangle]
pub unsafe extern "C" fn seek_by_read(mut fd: libc::c_int, mut amount: off_t) {
  if amount != 0 {
    bb_copyfd_exact_size(fd, -1i32, amount);
  };
}

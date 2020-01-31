use libc;
use libc::off_t;

/*  If we are reading through a pipe, or from stdin then we can't lseek,
 *  we must read and discard the data to skip over it.
 */
pub unsafe fn seek_by_read(mut fd: libc::c_int, mut amount: off_t) {
  if amount != 0 {
    crate::libbb::copyfd::bb_copyfd_exact_size(fd, -1i32, amount);
  };
}

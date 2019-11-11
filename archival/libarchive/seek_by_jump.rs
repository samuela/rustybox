use crate::libbb::ptr_to_globals::bb_errno;

use libc;
extern "C" {
  #[no_mangle]
  fn lseek(__fd: libc::c_int, __offset: off64_t, __whence: libc::c_int) -> off64_t;



  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;

  #[no_mangle]
  fn seek_by_read(fd: libc::c_int, amount: off_t);
}

use libc::off64_t;
use libc::off_t;
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

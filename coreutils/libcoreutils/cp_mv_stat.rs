use crate::libbb::ptr_to_globals::bb_errno;
use libc;

pub unsafe fn cp_mv_stat2(
  fn_0: *const libc::c_char,
  fn_stat: *mut libc::stat,
  sf: unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::stat) -> libc::c_int,
) -> libc::c_int {
  if sf(fn_0, fn_stat) < 0 {
    if *bb_errno != 2 {
      crate::libbb::perror_msg::bb_perror_msg(
        b"can\'t stat \'%s\'\x00" as *const u8 as *const libc::c_char,
        fn_0,
      );
      return -1;
    }
    return 0;
  }
  if (*fn_stat).st_mode & 0o170000 as libc::c_uint == 0o40000 as libc::c_uint {
    return 3;
  }
  return 1;
}
pub unsafe fn cp_mv_stat(fn_0: *const libc::c_char, fn_stat: *mut libc::stat) -> libc::c_int {
  return cp_mv_stat2(fn_0, fn_stat, libc::stat);
}

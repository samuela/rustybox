use libc;
extern "C" {
  #[no_mangle]
  fn isatty(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn fchmod(__fd: libc::c_int, __mode: __mode_t) -> libc::c_int;
  #[no_mangle]
  fn xfstat(fd: libc::c_int, buf: *mut stat, errmsg: *const libc::c_char);
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_perror_nomsg_and_die() -> !;
}

use crate::librb::__mode_t;

use crate::librb::mode_t;
use crate::librb::stat;
use crate::librb::timespec;
#[no_mangle]
pub unsafe extern "C" fn mesg_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut sb: stat = stat {
    st_dev: 0,
    st_ino: 0,
    st_nlink: 0,
    st_mode: 0,
    st_uid: 0,
    st_gid: 0,
    __pad0: 0,
    st_rdev: 0,
    st_size: 0,
    st_blksize: 0,
    st_blocks: 0,
    st_atim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_mtim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_ctim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    __glibc_reserved: [0; 3],
  };
  let mut m: mode_t = 0;
  let mut c: libc::c_char = 0i32 as libc::c_char;
  argv = argv.offset(1);
  if !(*argv.offset(0)).is_null()
    && (!(*argv.offset(1)).is_null() || {
      c = *(*argv.offset(0)).offset(0);
      (c as libc::c_int != 'y' as i32) && c as libc::c_int != 'n' as i32
    })
  {
    bb_show_usage();
  }
  /* We are a NOFORK applet.
   * (Not that it's very useful, but code is trivially NOFORK-safe).
   * Play nice. Do not leak anything.
   */
  if isatty(0i32) == 0 {
    bb_simple_error_msg_and_die(b"not a tty\x00" as *const u8 as *const libc::c_char);
  }
  xfstat(
    0i32,
    &mut sb,
    b"stdin\x00" as *const u8 as *const libc::c_char,
  );
  if c as libc::c_int == 0i32 {
    puts(
      if sb.st_mode & (0o200i32 >> 3i32 | 0o200i32 >> 3i32 >> 3i32) as libc::c_uint != 0 {
        b"is y\x00" as *const u8 as *const libc::c_char
      } else {
        b"is n\x00" as *const u8 as *const libc::c_char
      },
    );
    return 0i32;
  }
  m = if c as libc::c_int == 'y' as i32 {
    (sb.st_mode) | (0o200i32 >> 3i32) as libc::c_uint
  } else {
    (sb.st_mode) & !(0o200i32 >> 3i32 | 0o200i32 >> 3i32 >> 3i32) as libc::c_uint
  };
  if fchmod(0i32, m) != 0i32 {
    bb_perror_nomsg_and_die();
  }
  return 0i32;
}

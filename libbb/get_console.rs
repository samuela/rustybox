use crate::libbb::ptr_to_globals::bb_errno;

use libc;
use libc::close;
use libc::ioctl;
use libc::open;
use libc::ptrdiff_t;
extern "C" {


  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_xioctl(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
}

pub const KDGKBTYPE: C2RustUnnamed = 19251;
/* make vt active */
pub const VT_WAITACTIVE: C2RustUnnamed_0 = 22023;
pub const VT_ACTIVATE: C2RustUnnamed_0 = 22022;

/*
 * Utility routines.
 *
 * Copyright (C) many different people.  If you wrote this, please
 * acknowledge your work.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* From <linux/kd.h> */
pub type C2RustUnnamed = libc::c_uint;
/* From <linux/vt.h> */
pub type C2RustUnnamed_0 = libc::c_uint;
/* wait for vt active */
/* get keyboard type */
unsafe extern "C" fn open_a_console(mut fnam: *const libc::c_char) -> libc::c_int {
  /* try read-write */
  let mut fd: libc::c_int = open(fnam, 0o2);

  /* if failed, try read-only */
  if fd < 0 && *bb_errno == 13 {
    fd = open(fnam, 0)
  }

  /* if failed, try write-only */
  if fd < 0 && *bb_errno == 13 {
    fd = open(fnam, 0o1)
  }

  return fd;
}
/*
 * Get an fd for use with kbd/console ioctls.
 * We try several things because opening /dev/console will fail
 * if someone else used X (which does a chown on /dev/console).
 */
#[no_mangle]
pub unsafe extern "C" fn get_console_fd_or_die() -> libc::c_int {
  static mut console_names: [*const libc::c_char; 3] = [
    b"/dev/console\x00" as *const u8 as *const libc::c_char,
    b"/dev/tty0\x00" as *const u8 as *const libc::c_char,
    b"/dev/tty\x00" as *const u8 as *const libc::c_char,
  ];
  let mut fd: libc::c_int = 0;
  fd = 2;
  while fd >= 0 {
    let mut fd4name: libc::c_int = 0;
    let mut choice_fd: libc::c_int = 0;
    let mut arg: libc::c_char = 0;
    fd4name = open_a_console(console_names[fd as usize]);
    loop {
      choice_fd = if fd4name >= 0 { fd4name } else { fd };
      arg = 0 as libc::c_char;
      if ioctl(
        choice_fd,
        KDGKBTYPE as libc::c_int as libc::c_ulong,
        &mut arg as *mut libc::c_char,
      ) == 0
      {
        return choice_fd;
      }
      if !(fd4name >= 0) {
        break;
      }
      close(fd4name);
      fd4name = -1
    }
    fd -= 1
  }
  bb_simple_error_msg_and_die(b"can\'t open console\x00" as *const u8 as *const libc::c_char);
}

#[no_mangle]
pub unsafe extern "C" fn console_make_active(mut fd: libc::c_int, vt_num: libc::c_int) {
  bb_xioctl(
    fd,
    VT_ACTIVATE as libc::c_int as libc::c_uint,
    vt_num as ptrdiff_t as *mut libc::c_void,
    b"VT_ACTIVATE\x00" as *const u8 as *const libc::c_char,
  );
  bb_xioctl(
    fd,
    VT_WAITACTIVE as libc::c_int as libc::c_uint,
    vt_num as ptrdiff_t as *mut libc::c_void,
    b"VT_WAITACTIVE\x00" as *const u8 as *const libc::c_char,
  );
}

use crate::librb::size_t;
use libc;
use libc::close;
use libc::free;
use libc::getgid;
use libc::getuid;
use libc::gid_t;
use libc::setutxent;
use libc::uid_t;
use libc::utmpx;
extern "C" {
  #[no_mangle]
  fn getutxent() -> *mut utmpx;
  #[no_mangle]
  fn xopen_as_uid_gid(
    pathname: *const libc::c_char,
    flags: libc::c_int,
    u: uid_t,
    g: gid_t,
  ) -> libc::c_int;
  #[no_mangle]
  fn xmalloc_read(fd: libc::c_int, maxsz_p: *mut size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xopen_xwrite_close(file: *const libc::c_char, str: *const libc::c_char);
  #[no_mangle]
  fn concat_path_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __exit_status {
  pub e_termination: libc::c_short,
  pub e_exit: libc::c_short,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
  pub tv_sec: i32,
  pub tv_usec: i32,
}

/*
 * wall - write a message to all logged-in users
 * Copyright (c) 2009 Bernhard Reutner-Fischer
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config WALL
//config:	bool "wall (2.6 kb)"
//config:	default y
//config:	depends on FEATURE_UTMP
//config:	help
//config:	Write a message to all users that are logged in.
/* Needs to be run by root or be suid root - needs to write to /dev/TTY: */
//applet:IF_WALL(APPLET(wall, BB_DIR_USR_BIN, SUID_REQUIRE))
//kbuild:lib-$(CONFIG_WALL) += wall.o
//usage:#define wall_trivial_usage
//usage:	"[FILE]"
//usage:#define wall_full_usage "\n\n"
//usage:	"Write content of FILE or stdin to all logged-in users"
//usage:
//usage:#define wall_sample_usage
//usage:	"echo foo | wall\n"
//usage:	"wall ./mymessage"
#[no_mangle]
pub unsafe extern "C" fn wall_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut ut: *mut utmpx = std::ptr::null_mut();
  let mut msg: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut fd: libc::c_int = 0;
  fd = 0;
  if !(*argv.offset(1)).is_null() {
    /* The applet is setuid.
     * Access to the file must be under user's uid/gid.
     */
    fd = xopen_as_uid_gid(*argv.offset(1), 0, getuid(), getgid())
  }
  msg = xmalloc_read(fd, std::ptr::null_mut::<size_t>()) as *mut libc::c_char;
  if false && !(*argv.offset(1)).is_null() {
    close(fd);
  }
  setutxent();
  loop {
    ut = getutxent();
    if ut.is_null() {
      break;
    }
    let mut line: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    if (*ut).ut_type as libc::c_int != 7i32 {
      continue;
    }
    line = concat_path_file(
      b"/dev\x00" as *const u8 as *const libc::c_char,
      (*ut).ut_line.as_mut_ptr(),
    );
    xopen_xwrite_close(line, msg);
    free(line as *mut libc::c_void);
  }
  return 0;
}

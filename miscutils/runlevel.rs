use libc;
use libc::printf;
use libc::puts;
use libc::setutxent;
extern "C" {

  #[no_mangle]
  fn getutxent() -> *mut utmpx;
  #[no_mangle]
  fn utmpxname(__file: *const libc::c_char) -> libc::c_int;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __exit_status {
  pub e_termination: libc::c_short,
  pub e_exit: libc::c_short,
}
use libc::utmpx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
  pub tv_sec: i32,
  pub tv_usec: i32,
}

/*
 * Prints out the previous and the current runlevel.
 *
 * Version: @(#)runlevel  1.20  16-Apr-1997  MvS
 *
 * This file is part of the sysvinit suite,
 * Copyright 1991-1997 Miquel van Smoorenburg.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 *
 * initially busyboxified by Bernhard Reutner-Fischer
 */
//config:config RUNLEVEL
//config:	bool "runlevel (559 bytes)"
//config:	default y
//config:	depends on FEATURE_UTMP
//config:	help
//config:	Find the current and previous system runlevel.
//config:
//config:	This applet uses utmp but does not rely on busybox supporing
//config:	utmp on purpose. It is used by e.g. emdebian via /etc/init.d/rc.
//applet:IF_RUNLEVEL(APPLET_NOEXEC(runlevel, runlevel, BB_DIR_SBIN, SUID_DROP, runlevel))
//kbuild:lib-$(CONFIG_RUNLEVEL) += runlevel.o
//usage:#define runlevel_trivial_usage
//usage:       "[FILE]"
//usage:#define runlevel_full_usage "\n\n"
//usage:       "Find the current and previous system runlevel\n"
//usage:       "\n"
//usage:       "If no utmp FILE exists or if no runlevel record can be found,\n"
//usage:       "print \"unknown\""
//usage:
//usage:#define runlevel_example_usage
//usage:       "$ runlevel /var/run/utmp\n"
//usage:       "N 2"
#[no_mangle]
pub unsafe extern "C" fn runlevel_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut ut: *mut utmpx = std::ptr::null_mut();
  let mut prev: libc::c_char = 0;
  if !(*argv.offset(1)).is_null() {
    utmpxname(*argv.offset(1));
  }
  setutxent();
  loop {
    ut = getutxent();
    if ut.is_null() {
      break;
    }
    if (*ut).ut_type as libc::c_int == 1i32 {
      prev = ((*ut).ut_pid / 256i32) as libc::c_char;
      if prev as libc::c_int == 0 {
        prev = 'N' as i32 as libc::c_char
      }
      printf(
        b"%c %c\n\x00" as *const u8 as *const libc::c_char,
        prev as libc::c_int,
        (*ut).ut_pid % 256i32,
      );
      return 0;
    }
  }
  puts(b"unknown\x00" as *const u8 as *const libc::c_char);
  return 1i32;
}

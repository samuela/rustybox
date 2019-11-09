use libc;

extern "C" {
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn getlogin_r(__name: *mut libc::c_char, __name_len: size_t) -> libc::c_int;
}
use crate::librb::size_t;

/*
 * Mini logname implementation for busybox
 *
 * Copyright (C) 2000  Edward Betts <edward@debian.org>.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Mar 16, 2003      Manuel Novoa III   (mjn3@codepoet.org)
 *
 * SUSv3 specifies the string used is that returned from getlogin().
 * The previous implementation used getpwuid() for geteuid(), which
 * is _not_ the same.  Erik apparently made this change almost 3 years
 * ago to avoid failing when no utmp was available.  However, the
 * correct course of action wrt SUSv3 for a failing getlogin() is
 * a diagnostic message and an error return.
 */
//config:config LOGNAME
//config:	bool "logname (1.1 kb)"
//config:	default y
//config:	help
//config:	logname is used to print the current user's login name.
//applet:IF_LOGNAME(APPLET_NOFORK(logname, logname, BB_DIR_USR_BIN, BB_SUID_DROP, logname))
//kbuild:lib-$(CONFIG_LOGNAME) += logname.o
/* BB_AUDIT SUSv3 compliant */
/* http://www.opengroup.org/onlinepubs/007904975/utilities/logname.html */
//usage:#define logname_trivial_usage
//usage:       ""
//usage:#define logname_full_usage "\n\n"
//usage:       "Print the name of the current user"
//usage:
//usage:#define logname_example_usage
//usage:       "$ logname\n"
//usage:       "root\n"
/* This is a NOFORK applet. Be very careful! */
#[no_mangle]
pub unsafe extern "C" fn logname_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut buf: [libc::c_char; 64] = [0; 64];
  if !(*argv.offset(1)).is_null() {
    bb_show_usage();
  }
  /* Using _r function - avoid pulling in static buffer from libc */
  if getlogin_r(
    buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
  ) == 0i32
  {
    puts(buf.as_mut_ptr());
    return fflush_all();
  }
  bb_simple_perror_msg_and_die(b"getlogin\x00" as *const u8 as *const libc::c_char);
}

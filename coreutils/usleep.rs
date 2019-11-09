use libc;




extern "C" {
  #[no_mangle]
  fn xatou(str: *const libc::c_char) -> libc::c_uint;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn usleep(__useconds: useconds_t) -> libc::c_int;
}
use libc::useconds_t;

/*
 * usleep implementation for busybox
 *
 * Copyright (C) 2003  Manuel Novoa III  <mjn3@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config USLEEP
//config:	bool "usleep (1.3 kb)"
//config:	default y
//config:	help
//config:	usleep is used to pause for a specified number of microseconds.
//applet:IF_USLEEP(APPLET_NOFORK(usleep, usleep, BB_DIR_BIN, BB_SUID_DROP, usleep))
//kbuild:lib-$(CONFIG_USLEEP) += usleep.o
/* BB_AUDIT SUSv3 N/A -- Apparently a busybox extension. */
//usage:#define usleep_trivial_usage
//usage:       "N"
//usage:#define usleep_full_usage "\n\n"
//usage:       "Pause for N microseconds"
//usage:
//usage:#define usleep_example_usage
//usage:       "$ usleep 1000000\n"
//usage:       "[pauses for 1 second]\n"
/* This is a NOFORK applet. Be very careful! */
#[no_mangle]
pub unsafe extern "C" fn usleep_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  if (*argv.offset(1)).is_null() {
    bb_show_usage();
  }
  /* Safe wrt NOFORK? (noforks are not allowed to run for
   * a long time). Try "usleep 99999999" + ^C + "echo $?"
   * in hush with FEATURE_SH_NOFORK=y.
   * At least on uclibc, usleep() thanslates to nanosleep()
   * which returns early on any signal (even caught one),
   * and uclibc does not loop back on EINTR.
   */
  usleep(xatou(*argv.offset(1)));
  return 0i32;
}

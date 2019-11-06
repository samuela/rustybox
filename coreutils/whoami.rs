use libc;
extern "C" {
  #[no_mangle]
  fn geteuid() -> uid_t;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn xuid2uname(uid: uid_t) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_show_usage() -> !;
}

use libc::uid_t;

/*
 * Mini whoami implementation for busybox
 *
 * Copyright (C) 2000  Edward Betts <edward@debian.org>.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config WHOAMI
//config:	bool "whoami (3.2 kb)"
//config:	default y
//config:	help
//config:	whoami is used to print the username of the current
//config:	user id (same as id -un).
//applet:IF_WHOAMI(APPLET_NOFORK(whoami, whoami, BB_DIR_USR_BIN, BB_SUID_DROP, whoami))
//kbuild:lib-$(CONFIG_WHOAMI) += whoami.o
/* BB_AUDIT SUSv3 N/A -- Matches GNU behavior. */
//usage:#define whoami_trivial_usage
//usage:       ""
//usage:#define whoami_full_usage "\n\n"
//usage:       "Print the user name associated with the current effective user id"
#[no_mangle]
pub unsafe extern "C" fn whoami_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  if !(*argv.offset(1)).is_null() {
    bb_show_usage();
  }
  /* Will complain and die if username not found */
  puts(xuid2uname(geteuid()));
  return fflush_all();
}

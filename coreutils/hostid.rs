use libc;
extern "C" {
  #[no_mangle]
  fn gethostid() -> libc::c_long;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn bb_show_usage() -> !;
}

use crate::librb::uint32_t;

/*
 * Mini hostid implementation for busybox
 *
 * Copyright (C) 2000  Edward Betts <edward@debian.org>.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config HOSTID
//config:	bool "hostid (286 bytes)"
//config:	default y
//config:	help
//config:	hostid prints the numeric identifier (in hexadecimal) for
//config:	the current host.
//applet:IF_HOSTID(APPLET_NOFORK(hostid, hostid, BB_DIR_USR_BIN, BB_SUID_DROP, hostid))
//kbuild:lib-$(CONFIG_HOSTID) += hostid.o
/* BB_AUDIT SUSv3 N/A -- Matches GNU behavior. */
//usage:#define hostid_trivial_usage
//usage:       ""
//usage:#define hostid_full_usage "\n\n"
//usage:       "Print out a unique 32-bit identifier for the machine"
/* This is a NOFORK applet. Be very careful! */
#[no_mangle]
pub unsafe extern "C" fn hostid_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  if !(*argv.offset(1)).is_null() {
    bb_show_usage();
  }
  /* POSIX says gethostid returns a "32-bit identifier" */
  printf(
    b"%08x\n\x00" as *const u8 as *const libc::c_char,
    gethostid() as uint32_t,
  );
  return fflush_all();
}

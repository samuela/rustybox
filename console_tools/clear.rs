use libc;
use libc::access;
use libc::atoi;
use libc::fclose;
use libc::fprintf;
use libc::lstat;
use libc::printf;
use libc::puts;
use libc::rename;
use libc::rmdir;
use libc::sprintf;
use libc::strchr;
use libc::strcmp;
use libc::strrchr;
use libc::strstr;
use libc::system;




use libc::ssize_t;

extern "C" {
  #[no_mangle]
  fn full_write1_str(str: *const libc::c_char) -> ssize_t;
}

/*
 * Mini clear implementation for busybox
 *
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config CLEAR
//config:	bool "clear (tiny)"
//config:	default y
//config:	help
//config:	This program clears the terminal screen.
//applet:IF_CLEAR(APPLET_NOFORK(clear, clear, BB_DIR_USR_BIN, BB_SUID_DROP, clear))
//kbuild:lib-$(CONFIG_CLEAR) += clear.o
//usage:#define clear_trivial_usage
//usage:       ""
//usage:#define clear_full_usage "\n\n"
//usage:       "Clear screen"

#[no_mangle]
pub unsafe extern "C" fn clear_main(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  /* home; clear to the end of screen */
  return (full_write1_str(b"\x1b[H\x1b[J\x00" as *const u8 as *const libc::c_char) != 6)
    as libc::c_int;
}

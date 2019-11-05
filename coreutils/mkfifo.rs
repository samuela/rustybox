use libc;
extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn mkfifo(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn getopt_mk_fifo_nod(argv: *mut *mut libc::c_char) -> mode_t;
}
pub type __mode_t = libc::c_uint;
use crate::librb::mode_t;

/*
 * mkfifo implementation for busybox
 *
 * Copyright (C) 2003  Manuel Novoa III  <mjn3@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config MKFIFO
//config:	bool "mkfifo (3.8 kb)"
//config:	default y
//config:	help
//config:	mkfifo is used to create FIFOs (named pipes).
//config:	The 'mknod' program can also create FIFOs.
//applet:IF_MKFIFO(APPLET_NOEXEC(mkfifo, mkfifo, BB_DIR_USR_BIN, BB_SUID_DROP, mkfifo))
//kbuild:lib-$(CONFIG_MKFIFO) += mkfifo.o
/* BB_AUDIT SUSv3 compliant */
/* http://www.opengroup.org/onlinepubs/007904975/utilities/mkfifo.html */
//usage:#define mkfifo_trivial_usage
//usage:       "[-m MODE] " IF_SELINUX("[-Z] ") "NAME"
//usage:#define mkfifo_full_usage "\n\n"
//usage:       "Create named pipe\n"
//usage:     "\n	-m MODE	Mode (default a=rw)"
//usage:	IF_SELINUX(
//usage:     "\n	-Z	Set security context"
//usage:	)
/* This is a NOEXEC applet. Be very careful! */
#[no_mangle]
pub unsafe extern "C" fn mkfifo_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut mode: mode_t = 0; /* Avoid multibyte problems. */
  let mut retval: libc::c_int = 0i32;
  mode = getopt_mk_fifo_nod(argv);
  argv = argv.offset(optind as isize);
  if (*argv).is_null() {
    bb_show_usage();
  }
  loop {
    if mkfifo(*argv, mode) < 0i32 {
      bb_simple_perror_msg(*argv);
      retval = 1i32
    }
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  return retval;
}

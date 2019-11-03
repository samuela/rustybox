use libc;
extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  static mut xfunc_error_retval: uint8_t;
  #[no_mangle]
  fn bb_warn_ignoring_args(arg: *mut libc::c_char);
  #[no_mangle]
  fn xmalloc_ttyname(fd: libc::c_int) -> *mut libc::c_char;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
/* vi: set sw=4 ts=4: */
/*
 * tty implementation for busybox
 *
 * Copyright (C) 2003  Manuel Novoa III  <mjn3@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config TTY
//config:	bool "tty (3.6 kb)"
//config:	default y
//config:	help
//config:	tty is used to print the name of the current terminal to
//config:	standard output.
//applet:IF_TTY(APPLET_NOFORK(tty, tty, BB_DIR_USR_BIN, BB_SUID_DROP, tty))
//kbuild:lib-$(CONFIG_TTY) += tty.o
/* BB_AUDIT SUSv4 compliant */
/* http://www.opengroup.org/onlinepubs/9699919799/utilities/tty.html */
//usage:#define tty_trivial_usage
//usage:       ""
//usage:#define tty_full_usage "\n\n"
//usage:       "Print file name of stdin's terminal"
//usage:	IF_INCLUDE_SUSv2( "\n"
//usage:     "\n	-s	Print nothing, only return exit status"
//usage:	)
//usage:
//usage:#define tty_example_usage
//usage:       "$ tty\n"
//usage:       "/dev/tty2\n"
#[no_mangle]
pub unsafe extern "C" fn tty_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut s: *const libc::c_char = 0 as *const libc::c_char; /* Note: No longer relevant in SUSv3. */
  let mut silent: libc::c_int = 0; /* SUSv3 requires > 1 for error. */
  let mut retval: libc::c_int = 0;
  xfunc_error_retval = 2i32 as uint8_t;
  silent = getopt32(argv, b"s\x00" as *const u8 as *const libc::c_char) as libc::c_int;
  argv = argv.offset(optind as isize);
  /* gnu tty outputs a warning that it is ignoring all args. */
  bb_warn_ignoring_args(*argv.offset(0));
  retval = 0i32;
  s = xmalloc_ttyname(0i32);
  if s.is_null() {
    /* According to SUSv3, ttyname can fail with EBADF or ENOTTY.
     * We know the file descriptor is good, so failure means not a tty. */
    s = b"not a tty\x00" as *const u8 as *const libc::c_char;
    retval = 1i32
  }
  if silent == 0 {
    puts(s);
  }
  fflush_stdout_and_exit(retval);
}

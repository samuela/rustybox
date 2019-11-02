use libc;
extern "C" {
  #[no_mangle]
  fn get_console_fd_or_die() -> libc::c_int;
  #[no_mangle]
  fn console_make_active(fd: libc::c_int, vt_num: libc::c_int);
  #[no_mangle]
  fn xatou_range(str: *const libc::c_char, l: libc::c_uint, u: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn single_argv(argv: *mut *mut libc::c_char) -> *mut libc::c_char;
}
/* vi: set sw=4 ts=4: */
/*
 * Mini chvt implementation for busybox
 *
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config CHVT
//config:	bool "chvt (2 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	This program is used to change to another terminal.
//config:	Example: chvt 4 (change to terminal /dev/tty4)
//applet:IF_CHVT(APPLET_NOEXEC(chvt, chvt, BB_DIR_USR_BIN, BB_SUID_DROP, chvt))
//kbuild:lib-$(CONFIG_CHVT) += chvt.o
//usage:#define chvt_trivial_usage
//usage:       "N"
//usage:#define chvt_full_usage "\n\n"
//usage:       "Change the foreground virtual terminal to /dev/ttyN"
#[no_mangle]
pub unsafe extern "C" fn chvt_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut num: libc::c_int = xatou_range(
    single_argv(argv),
    1i32 as libc::c_uint,
    63i32 as libc::c_uint,
  ) as libc::c_int;
  console_make_active(get_console_fd_or_die(), num);
  return 0i32;
}

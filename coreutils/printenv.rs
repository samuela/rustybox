use libc;
extern "C" {
  #[no_mangle]
  fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  static mut environ: *mut *mut libc::c_char;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
}
/* vi: set sw=4 ts=4: */
/*
 * printenv implementation for busybox
 *
 * Copyright (C) 2005 by Erik Andersen <andersen@codepoet.org>
 * Copyright (C) 2005 by Mike Frysinger <vapier@gentoo.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config PRINTENV
//config:	bool "printenv (1.3 kb)"
//config:	default y
//config:	help
//config:	printenv is used to print all or part of environment.
//applet:IF_PRINTENV(APPLET_NOFORK(printenv, printenv, BB_DIR_BIN, BB_SUID_DROP, printenv))
//kbuild:lib-$(CONFIG_PRINTENV) += printenv.o
//usage:#define printenv_trivial_usage
//usage:       "[VARIABLE]..."
//usage:#define printenv_full_usage "\n\n"
//usage:       "Print environment VARIABLEs.\n"
//usage:       "If no VARIABLE specified, print all."
/* This is a NOFORK applet. Be very careful! */
#[no_mangle]
pub unsafe extern "C" fn printenv_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut exit_code: libc::c_int = 0i32;
  /* no variables specified, show whole env */
  if (*argv.offset(1)).is_null() {
    let mut e: *mut *mut libc::c_char = environ;
    /* environ can be NULL! (for example, after clearenv())
     * Check for that:
     */
    if !e.is_null() {
      while !(*e).is_null() {
        let fresh0 = e;
        e = e.offset(1);
        puts(*fresh0);
      }
    }
  } else {
    /* search for specified variables and print them out if found */
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut env: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
      argv = argv.offset(1);
      arg = *argv;
      if arg.is_null() {
        break;
      }
      env = getenv(arg);
      if !env.is_null() {
        puts(env);
      } else {
        exit_code = 1i32
      }
    }
  }
  fflush_stdout_and_exit(exit_code);
}

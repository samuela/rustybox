use libc;
use libc::printf;
extern "C" {

  #[no_mangle]
  fn getpriority(__which: __priority_which_t, __who: id_t) -> libc::c_int;
  #[no_mangle]
  fn setpriority(__which: __priority_which_t, __who: id_t, __prio: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
  #[no_mangle]
  fn xatoi_range(str: *const libc::c_char, l: libc::c_int, u: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn BB_EXECVP_or_die(argv: *mut *mut libc::c_char) -> !;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
}
pub type __id_t = libc::c_uint;
pub type id_t = __id_t;
pub type __priority_which = libc::c_uint;
pub const PRIO_USER: __priority_which = 2;
pub const PRIO_PGRP: __priority_which = 1;
pub const PRIO_PROCESS: __priority_which = 0;
pub type __priority_which_t = __priority_which;

/*
 * nice implementation for busybox
 *
 * Copyright (C) 2005  Manuel Novoa III  <mjn3@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config NICE
//config:	bool "nice (2.1 kb)"
//config:	default y
//config:	help
//config:	nice runs a program with modified scheduling priority.
//applet:IF_NICE(APPLET_NOEXEC(nice, nice, BB_DIR_BIN, SUID_DROP, nice))
//kbuild:lib-$(CONFIG_NICE) += nice.o
//usage:#define nice_trivial_usage
//usage:       "[-n ADJUST] [PROG ARGS]"
//usage:#define nice_full_usage "\n\n"
//usage:       "Change scheduling priority, run PROG\n"
//usage:     "\n	-n ADJUST	Adjust priority by ADJUST"
#[no_mangle]
pub unsafe extern "C" fn nice_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut old_priority: libc::c_int = 0;
  let mut adjustment: libc::c_int = 0;
  old_priority = getpriority(PRIO_PROCESS, 0 as id_t);
  argv = argv.offset(1);
  if (*argv).is_null() {
    /* No args, so (GNU) output current nice value. */
    printf(
      b"%d\n\x00" as *const u8 as *const libc::c_char,
      old_priority,
    ); /* Set default adjustment. */
    fflush_stdout_and_exit(0i32);
  }
  adjustment = 10i32;
  if *(*argv.offset(0)).offset(0) as libc::c_int == '-' as i32 {
    let mut nnn: *mut libc::c_char = (*argv.offset(0)).offset(1);
    if *nnn.offset(0) as libc::c_int == 'n' as i32 {
      /* -n */
      nnn = nnn.offset(1);
      if *nnn.offset(0) == 0 {
        /* else: "-nNNN" (w/o space) */
        /* "-n NNN" */
        argv = argv.offset(1);
        nnn = *argv
      }
    }
    /* else: "-NNN" (NNN may be negative) - same as "-n NNN" */
    if nnn.is_null() || (*argv.offset(1)).is_null() {
      /* Missing priority or PROG! */
      bb_show_usage();
    }
    adjustment = xatoi_range(nnn, (-2147483647i32 - 1i32) / 2i32, 2147483647i32 / 2i32);
    argv = argv.offset(1)
  }
  /* Set our priority. */
  let mut prio: libc::c_int = old_priority + adjustment;
  if setpriority(PRIO_PROCESS, 0 as id_t, prio) < 0 {
    bb_perror_msg_and_die(
      b"setpriority(%d)\x00" as *const u8 as *const libc::c_char,
      prio,
    );
  }
  BB_EXECVP_or_die(argv);
}

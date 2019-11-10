use libc;











































extern "C" {
  #[no_mangle]
  static mut stdout: *mut FILE;
  #[no_mangle]
  fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn bb_perror_nomsg_and_die() -> !;
}

use libc::FILE;

/*
 * yes implementation for busybox
 *
 * Copyright (C) 2003  Manuel Novoa III  <mjn3@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Mar 16, 2003      Manuel Novoa III   (mjn3@codepoet.org)
 *
 * Size reductions and removed redundant applet name prefix from error messages.
 */
//config:config YES
//config:	bool "yes (1.2 kb)"
//config:	default y
//config:	help
//config:	yes is used to repeatedly output a specific string, or
//config:	the default string 'y'.
//applet:IF_YES(APPLET_NOEXEC(yes, yes, BB_DIR_USR_BIN, BB_SUID_DROP, yes))
/* was NOFORK, but then yes can't be ^C'ed if run by hush */
//kbuild:lib-$(CONFIG_YES) += yes.o
/* BB_AUDIT SUSv3 N/A -- Matches GNU behavior. */
//usage:#define yes_trivial_usage
//usage:       "[STRING]"
//usage:#define yes_full_usage "\n\n"
//usage:       "Repeatedly output a line with STRING, or 'y'"
#[no_mangle]
pub unsafe extern "C" fn yes_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut pp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let ref mut fresh0 = *argv.offset(0);
  *fresh0 = b"y\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
  if !(*argv.offset(1)).is_null() {
    argv = argv.offset(1)
  }
  loop {
    pp = argv;
    loop {
      fputs_unlocked(*pp, stdout);
      pp = pp.offset(1);
      if (*pp).is_null() {
        break;
      }
      putchar_unlocked(' ' as i32);
    }
    if !(putchar_unlocked('\n' as i32) != -1i32) {
      break;
    }
  }
  bb_perror_nomsg_and_die();
}

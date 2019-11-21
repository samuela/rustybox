use crate::libbb::perror_nomsg_and_die::bb_perror_nomsg_and_die;
use libc;
use libc::c_char;
use libc::c_int;
use std::ffi::CStr;
use std::io;
use std::io::Write;
use std::slice;

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
pub unsafe extern "C" fn yes_main(mut argc: c_int, mut argv: *const *const c_char) -> c_int {
  let line = if argc > 1 {
    slice::from_raw_parts(argv, argc as usize)
      .iter()
      .map(|c_char_arg| CStr::from_ptr(*c_char_arg).to_str().unwrap())
      .collect::<Vec<&str>>()
      .join(" ")
  } else {
    "y".to_string()
  };
  let mut stdout = io::stdout();
  loop {
    print!("{}", line);
    if let Err(_) = stdout.write_all(b"\n") {
      break;
    }
  }
  bb_perror_nomsg_and_die();
  0
}

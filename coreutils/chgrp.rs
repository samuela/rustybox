use libc;
use libc::close;

extern "C" {
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn chown_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;
}

/*
 * Mini chgrp implementation for busybox
 *
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config CHGRP
//config:	bool "chgrp (7.6 kb)"
//config:	default y
//config:	help
//config:	chgrp is used to change the group ownership of files.
//applet:IF_CHGRP(APPLET_NOEXEC(chgrp, chgrp, BB_DIR_BIN, BB_SUID_DROP, chgrp))
//kbuild:lib-$(CONFIG_CHGRP) += chgrp.o chown.o
/* BB_AUDIT SUSv3 defects - none? */
/* BB_AUDIT GNU defects - unsupported long options. */
/* http://www.opengroup.org/onlinepubs/007904975/utilities/chgrp.html */
//usage:#define chgrp_trivial_usage
//usage:       "[-RhLHP"IF_DESKTOP("cvf")"]... GROUP FILE..."
//usage:#define chgrp_full_usage "\n\n"
//usage:       "Change the group membership of each FILE to GROUP\n"
//usage:     "\n	-R	Recurse"
//usage:     "\n	-h	Affect symlinks instead of symlink targets"
//usage:     "\n	-L	Traverse all symlinks to directories"
//usage:     "\n	-H	Traverse symlinks on command line only"
//usage:     "\n	-P	Don't traverse symlinks (default)"
//usage:	IF_DESKTOP(
//usage:     "\n	-c	List changed files"
//usage:     "\n	-v	Verbose"
//usage:     "\n	-f	Hide errors"
//usage:	)
//usage:
//usage:#define chgrp_example_usage
//usage:       "$ ls -l /tmp/foo\n"
//usage:       "-r--r--r--    1 andersen andersen        0 Apr 12 18:25 /tmp/foo\n"
//usage:       "$ chgrp root /tmp/foo\n"
//usage:       "$ ls -l /tmp/foo\n"
//usage:       "-r--r--r--    1 andersen root            0 Apr 12 18:25 /tmp/foo\n"
/* This is a NOEXEC applet. Be very careful! */
#[no_mangle]
pub unsafe extern "C" fn chgrp_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  /* "chgrp [opts] abc file(s)" == "chown [opts] :abc file(s)" */
  let mut p: *mut *mut libc::c_char = argv;
  loop {
    p = p.offset(1);
    if (*p).is_null() {
      break;
    }
    if !(*(*p.offset(0)).offset(0) as libc::c_int != '-' as i32) {
      continue;
    }
    let ref mut fresh0 = *p.offset(0);
    *fresh0 = xasprintf(b":%s\x00" as *const u8 as *const libc::c_char, *p.offset(0));
    break;
  }
  return chown_main(argc, argv);
}

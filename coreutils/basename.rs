use crate::librb::size_t;
use libc;
use libc::ssize_t;
use libc::strcmp;
extern "C" {

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn bb_get_last_path_component_strip(path: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn bb_show_usage() -> !;
}

/*
 * Mini basename implementation for busybox
 *
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Mar 16, 2003      Manuel Novoa III   (mjn3@codepoet.org)
 *
 * Changes:
 * 1) Now checks for too many args.  Need at least one and at most two.
 * 2) Don't check for options, as per SUSv3.
 * 3) Save some space by using strcmp().  Calling strncmp() here was silly.
 */
//config:config BASENAME
//config:	bool "basename (438 bytes)"
//config:	default y
//config:	help
//config:	basename is used to strip the directory and suffix from filenames,
//config:	leaving just the filename itself. Enable this option if you wish
//config:	to enable the 'basename' utility.
//applet:IF_BASENAME(APPLET_NOFORK(basename, basename, BB_DIR_USR_BIN, SUID_DROP, basename))
//kbuild:lib-$(CONFIG_BASENAME) += basename.o
/* BB_AUDIT SUSv3 compliant */
/* http://www.opengroup.org/onlinepubs/007904975/utilities/basename.html */
//usage:#define basename_trivial_usage
//usage:       "FILE [SUFFIX]"
//usage:#define basename_full_usage "\n\n"
//usage:       "Strip directory path and .SUFFIX from FILE"
//usage:
//usage:#define basename_example_usage
//usage:       "$ basename /usr/local/bin/foo\n"
//usage:       "foo\n"
//usage:       "$ basename /usr/local/bin/\n"
//usage:       "bin\n"
//usage:       "$ basename /foo/bar.txt .txt\n"
//usage:       "bar"
/* This is a NOFORK applet. Be very careful! */
#[no_mangle]
pub unsafe extern "C" fn basename_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut m: size_t = 0;
  let mut n: size_t = 0;
  let mut s: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  if !(*argv.offset(1)).is_null()
    && strcmp(
      *argv.offset(1),
      b"--\x00" as *const u8 as *const libc::c_char,
    ) == 0
  {
    argv = argv.offset(1)
  }
  if (*argv.offset(1)).is_null() {
    bb_show_usage();
  }
  /* It should strip slash: /abc/def/ -> def */
  argv = argv.offset(1);
  s = bb_get_last_path_component_strip(*argv);
  m = strlen(s);
  argv = argv.offset(1);
  if !(*argv).is_null() {
    if !(*argv.offset(1)).is_null() {
      bb_show_usage();
    }
    n = strlen(*argv);
    if m > n && strcmp(s.offset(m as isize).offset(-(n as isize)), *argv) == 0 {
      m = (m as libc::c_ulong).wrapping_sub(n) as size_t as size_t
      /*s[m] = '\0'; - redundant */
    }
  }
  /* puts(s) will do, but we can do without stdio this way: */
  let fresh0 = m;
  m = m.wrapping_add(1);
  *s.offset(fresh0 as isize) = '\n' as i32 as libc::c_char;
  /* NB: != is correct here: */
  return (full_write(1, s as *const libc::c_void, m) != m as ssize_t) as libc::c_int;
}

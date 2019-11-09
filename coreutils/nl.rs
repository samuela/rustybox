use libc;

extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> u32;
  #[no_mangle]
  fn print_numbered_lines(ns: *mut number_state, filename: *const libc::c_char) -> libc::c_int;
}

use crate::librb::size_t;
use crate::librb::smallint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct number_state {
  pub width: libc::c_uint,
  pub start: libc::c_uint,
  pub inc: libc::c_uint,
  pub sep: *const libc::c_char,
  pub empty_str: *const libc::c_char,
  pub all: smallint,
  pub nonempty: smallint,
}
pub type C2RustUnnamed = libc::c_uint;
pub const OPT_p: C2RustUnnamed = 1;

/*
 * Copyright (C) 2017 Denys Vlasenko <vda.linux@googlemail.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config NL
//config:	bool "nl (4.6 kb)"
//config:	default y
//config:	help
//config:	nl is used to number lines of files.
//applet:IF_NL(APPLET(nl, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_NL) += nl.o
//usage:#define nl_trivial_usage
//usage:       "[OPTIONS] [FILE]..."
//usage:#define nl_full_usage "\n\n"
//usage:       "Write FILEs to standard output with line numbers added\n"
//usage:     "\n	-b STYLE	Which lines to number - a: all, t: nonempty, n: none"
//^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^TODO: support "pBRE": number only lines thatmatch regexp BRE"
// //usage:     "\n	-f STYLE	footer lines"
// //usage:     "\n	-h STYLE	header lines"
// //usage:     "\n	-d CC		use CC for separating logical pages"
//usage:     "\n	-i N		Line number increment"
// //usage:     "\n	-l NUMBER	group of NUMBER empty lines counted as one"
// //usage:     "\n	-n FORMAT	lneft justified, no leading zeros; rn or rz"
// //usage:     "\n	-p 		do not reset line numbers at logical pages (huh?)"
//usage:     "\n	-s STRING	Use STRING as line number separator"
//usage:     "\n	-v N		Start from N"
//usage:     "\n	-w N		Width of line numbers"
/* By default, selects -v1 -i1 -l1 -sTAB -w6 -nrn -hn -bt -fn */
#[no_mangle]
pub unsafe extern "C" fn nl_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut ns: number_state = number_state {
    width: 0,
    start: 0,
    inc: 0,
    sep: 0 as *const libc::c_char,
    empty_str: 0 as *const libc::c_char,
    all: 0,
    nonempty: 0,
  };
  let mut opt_b: *const libc::c_char = b"t\x00" as *const u8 as *const libc::c_char;
  static mut nl_longopts: [libc::c_char; 106] = [
    98, 111, 100, 121, 45, 110, 117, 109, 98, 101, 114, 105, 110, 103, 0, 1, 98, 108, 105, 110,
    101, 45, 105, 110, 99, 114, 101, 109, 101, 110, 116, 0, 1, 105, 110, 111, 45, 114, 101, 110,
    117, 109, 98, 101, 114, 0, 0, 112, 110, 117, 109, 98, 101, 114, 45, 115, 101, 112, 97, 114, 97,
    116, 111, 114, 0, 1, 115, 115, 116, 97, 114, 116, 105, 110, 103, 45, 108, 105, 110, 101, 45,
    110, 117, 109, 98, 101, 114, 0, 1, 118, 110, 117, 109, 98, 101, 114, 45, 119, 105, 100, 116,
    104, 0, 1, 119, 0,
  ];
  let mut exitcode: libc::c_int = 0;
  ns.width = 6i32 as libc::c_uint;
  ns.start = 1i32 as libc::c_uint;
  ns.inc = 1i32 as libc::c_uint;
  ns.sep = b"\t\x00" as *const u8 as *const libc::c_char;
  getopt32long(
    argv,
    b"pw:+s:v:+i:+b:\x00" as *const u8 as *const libc::c_char,
    nl_longopts.as_ptr(),
    &mut ns.width as *mut libc::c_uint,
    &mut ns.sep as *mut *const libc::c_char,
    &mut ns.start as *mut libc::c_uint,
    &mut ns.inc as *mut libc::c_uint,
    &mut opt_b as *mut *const libc::c_char,
  );
  ns.all = (*opt_b.offset(0) as libc::c_int == 'a' as i32) as libc::c_int as smallint;
  ns.nonempty = (*opt_b.offset(0) as libc::c_int == 't' as i32) as libc::c_int as smallint;
  ns.empty_str = xasprintf(
    b"%*s\n\x00" as *const u8 as *const libc::c_char,
    ns.width
      .wrapping_add(strlen(ns.sep) as libc::c_int as libc::c_uint),
    b"\x00" as *const u8 as *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  if (*argv).is_null() {
    argv = argv.offset(-1);
    *argv = b"-\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }
  exitcode = 0i32;
  loop {
    exitcode |= print_numbered_lines(&mut ns, *argv);
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  fflush_stdout_and_exit(exitcode);
}

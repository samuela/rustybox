use libc;


extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut stdout: *mut FILE;
  #[no_mangle]
  static mut stderr: *mut FILE;
  #[no_mangle]
  fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn die_if_ferror(file: *mut FILE, msg: *const libc::c_char);
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
  #[no_mangle]
  fn xfopen_stdin(filename: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn xatoull_range(
    str: *const libc::c_char,
    l: libc::c_ulonglong,
    u: libc::c_ulonglong,
  ) -> libc::c_ulonglong;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  static mut logmode: smallint;
  #[no_mangle]
  static mut xfunc_error_retval: u8;
}

use crate::librb::smallint;



use libc::off_t;

use libc::FILE;
pub type C2RustUnnamed = libc::c_uint;
pub const LOGMODE_BOTH: C2RustUnnamed = 3;
pub const LOGMODE_SYSLOG: C2RustUnnamed = 2;
pub const LOGMODE_STDIO: C2RustUnnamed = 1;
pub const LOGMODE_NONE: C2RustUnnamed = 0;
#[inline(always)]
unsafe extern "C" fn xatoul_range(
  mut str: *const libc::c_char,
  mut l: libc::c_ulong,
  mut u: libc::c_ulong,
) -> libc::c_ulong {
  return xatoull_range(str, l as libc::c_ulonglong, u as libc::c_ulonglong) as libc::c_ulong;
}

/*
 * Mini cmp implementation for busybox
 *
 * Copyright (C) 2000,2001 by Matt Kraai <kraai@alumni.carnegiemellon.edu>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config CMP
//config:	bool "cmp (4.9 kb)"
//config:	default y
//config:	help
//config:	cmp is used to compare two files and returns the result
//config:	to standard output.
//applet:IF_CMP(APPLET(cmp, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_CMP) += cmp.o
//usage:#define cmp_trivial_usage
//usage:       "[-l] [-s] FILE1 [FILE2" IF_DESKTOP(" [SKIP1 [SKIP2]]") "]"
//usage:#define cmp_full_usage "\n\n"
//usage:       "Compare FILE1 with FILE2 (or stdin)\n"
//usage:     "\n	-l	Write the byte numbers (decimal) and values (octal)"
//usage:     "\n		for all differing bytes"
//usage:     "\n	-s	Quiet"
/* BB_AUDIT SUSv3 (virtually) compliant -- uses nicer GNU format for -l. */
/* http://www.opengroup.org/onlinepubs/007904975/utilities/cmp.html */
static mut fmt_eof: [libc::c_char; 16] = [
  99, 109, 112, 58, 32, 69, 79, 70, 32, 111, 110, 32, 37, 115, 10, 0,
];
static mut fmt_differ: [libc::c_char; 33] = [
  37, 115, 32, 37, 115, 32, 100, 105, 102, 102, 101, 114, 58, 32, 99, 104, 97, 114, 32, 37, 108,
  117, 44, 32, 108, 105, 110, 101, 32, 37, 117, 10, 0,
];
// This fmt_l_opt uses gnu-isms.  SUSv3 would be "%.0s%.0s%"OFF_FMT"u %o %o\n"
static mut fmt_l_opt: [libc::c_char; 21] = [
  37, 46, 48, 115, 37, 46, 48, 115, 37, 108, 117, 32, 37, 51, 111, 32, 37, 51, 111, 10, 0,
];
#[no_mangle]
pub unsafe extern "C" fn cmp_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut fp1: *mut FILE = 0 as *mut FILE; /* Hopefully won't overflow... */
  let mut fp2: *mut FILE = 0 as *mut FILE; /* missing file results in exitcode 2 */
  let mut outfile: *mut FILE = stdout; /* -s suppresses open error messages */
  let mut filename1: *const libc::c_char = 0 as *const libc::c_char;
  let mut filename2: *const libc::c_char = b"-\x00" as *const u8 as *const libc::c_char;
  let mut skip1: off_t = 0i32 as off_t;
  let mut skip2: off_t = 0i32 as off_t;
  let mut char_pos: off_t = 0i32 as off_t;
  let mut line_pos: libc::c_int = 1i32;
  let mut fmt: *const libc::c_char = 0 as *const libc::c_char;
  let mut c1: libc::c_int = 0;
  let mut c2: libc::c_int = 0;
  let mut opt: libc::c_uint = 0;
  let mut retval: libc::c_int = 0i32;
  opt = getopt32(
    argv,
    b"^sl\x00-1:?4:l--s:s--l\x00" as *const u8 as *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  filename1 = *argv;
  argv = argv.offset(1);
  if !(*argv).is_null() {
    filename2 = *argv;
    if 1i32 != 0 && {
      argv = argv.offset(1);
      !(*argv).is_null()
    } {
      skip1 = xatoul_range(
        *argv,
        0i32 as libc::c_ulong,
        9223372036854775807i64 as libc::c_ulong,
      ) as off_t;
      argv = argv.offset(1);
      if !(*argv).is_null() {
        skip2 = xatoul_range(
          *argv,
          0i32 as libc::c_ulong,
          9223372036854775807i64 as libc::c_ulong,
        ) as off_t
      }
    }
  }
  xfunc_error_retval = 2i32 as u8;
  if opt & (1i32 << 0i32) as libc::c_uint != 0 {
    logmode = 0i32 as smallint
  }
  fp1 = xfopen_stdin(filename1);
  fp2 = xfopen_stdin(filename2);
  if fp1 == fp2 {
    /* Paranoia check... stdin == stdin? */
    /* Note that we don't bother reading stdin.  Neither does gnu wc.
     * But perhaps we should, so that other apps down the chain don't
     * get the input.  Consider 'echo hello | (cmp - - && cat -)'.
     */
    return 0i32;
  }
  logmode = LOGMODE_STDIO as libc::c_int as smallint;
  if opt & (1i32 << 1i32) as libc::c_uint != 0 {
    fmt = fmt_l_opt.as_ptr()
  } else {
    fmt = fmt_differ.as_ptr()
  }
  while skip1 != 0 {
    getc_unlocked(fp1);
    skip1 -= 1
  }
  while skip2 != 0 {
    getc_unlocked(fp2);
    skip2 -= 1
  }
  loop {
    c1 = getc_unlocked(fp1);
    c2 = getc_unlocked(fp2);
    char_pos += 1;
    if c1 != c2 {
      /* Remember: a read error may have occurred. */
      retval = 1i32; /* But assume the files are different for now. */
      if c2 == -1i32 {
        /* We know that fp1 isn't at EOF or in an error state.  But to
         * save space below, things are setup to expect an EOF in fp1
         * if an EOF occurred.  So, swap things around.
         */
        fp1 = fp2; /* Well, no error, so it must really be EOF. */
        filename1 = filename2;
        c1 = c2
      }
      if c1 == -1i32 {
        die_if_ferror(fp1, filename1);
        fmt = fmt_eof.as_ptr();
        outfile = stderr;
        /* There may have been output to stdout (option -l), so
         * make sure we fflush before writing to stderr. */
        fflush_all();
      }
      if !(opt & (1i32 << 0i32) as libc::c_uint == 0) {
        break;
      }
      if opt & (1i32 << 1i32) as libc::c_uint != 0 {
        line_pos = c1
        /* line_pos is unused in the -l case. */
      }
      fprintf(outfile, fmt, filename1, filename2, char_pos, line_pos, c2);
      if !(opt != 0) {
        break;
      }
    } else if c1 == '\n' as i32 {
      line_pos += 1
    }
    /* This must be -l since not -s. */
    /* If we encountered an EOF,
     * the while check will catch it. */
    if !(c1 != -1i32) {
      break;
    }
  }
  die_if_ferror(fp1, filename1);
  die_if_ferror(fp2, filename2);
  fflush_stdout_and_exit(retval);
}

use libc;

extern "C" {
  #[no_mangle]
  fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn mktemp(__template: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn mkdtemp(__template: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_perror_nomsg_and_die() -> !;
  #[no_mangle]
  fn concat_path_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;
}


pub const OPT_q: C2RustUnnamed = 2;
pub const OPT_d: C2RustUnnamed = 1;
pub const OPT_u: C2RustUnnamed = 16;
pub const OPT_p: C2RustUnnamed = 8;
pub const OPT_t: C2RustUnnamed = 4;
pub type C2RustUnnamed = libc::c_uint;

/*
 * Mini mktemp implementation for busybox
 *
 * Copyright (C) 2000 by Daniel Jacobowitz
 * Written by Daniel Jacobowitz <dan@debian.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Coreutils 6.12 man page says:
 *        mktemp [OPTION]... [TEMPLATE]
 * Create a temporary file or directory, safely, and print its name. If
 * TEMPLATE is not specified, use tmp.XXXXXXXXXX.
 * -d, --directory
 *        create a directory, not a file
 * -q, --quiet
 *        suppress diagnostics about file/dir-creation failure
 * -u, --dry-run
 *        do not create anything; merely print a name (unsafe)
 * --tmpdir[=DIR]
 *        interpret TEMPLATE relative to DIR. If DIR is not specified,
 *        use  $TMPDIR if set, else /tmp.  With this option, TEMPLATE must
 *        not be an absolute name. Unlike with -t, TEMPLATE may contain
 *        slashes, but even here, mktemp still creates only the final com-
 *        ponent.
 * -p DIR use DIR as a prefix; implies -t [deprecated]
 * -t     interpret TEMPLATE as a single file name component, relative  to
 *        a  directory:  $TMPDIR, if set; else the directory specified via
 *        -p; else /tmp [deprecated]
 */
//config:config MKTEMP
//config:	bool "mktemp (4.2 kb)"
//config:	default y
//config:	help
//config:	mktemp is used to create unique temporary files
//applet:IF_MKTEMP(APPLET_NOEXEC(mktemp, mktemp, BB_DIR_BIN, BB_SUID_DROP, mktemp))
//kbuild:lib-$(CONFIG_MKTEMP) += mktemp.o
//usage:#define mktemp_trivial_usage
//usage:       "[-dt] [-p DIR] [TEMPLATE]"
//usage:#define mktemp_full_usage "\n\n"
//usage:       "Create a temporary file with name based on TEMPLATE and print its name.\n"
//usage:       "TEMPLATE must end with XXXXXX (e.g. [/dir/]nameXXXXXX).\n"
//usage:       "Without TEMPLATE, -t tmp.XXXXXX is assumed.\n"
//usage:     "\n	-d	Make directory, not file"
//usage:     "\n	-q	Fail silently on errors"
//usage:     "\n	-t	Prepend base directory name to TEMPLATE"
//usage:     "\n	-p DIR	Use DIR as a base directory (implies -t)"
//usage:     "\n	-u	Do not create anything; print a name"
//usage:     "\n"
//usage:     "\nBase directory is: -p DIR, else $TMPDIR, else /tmp"
//usage:
//usage:#define mktemp_example_usage
//usage:       "$ mktemp /tmp/temp.XXXXXX\n"
//usage:       "/tmp/temp.mWiLjM\n"
//usage:       "$ ls -la /tmp/temp.mWiLjM\n"
//usage:       "-rw-------    1 andersen andersen        0 Apr 25 17:10 /tmp/temp.mWiLjM\n"
#[no_mangle]
pub unsafe extern "C" fn mktemp_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut path: *const libc::c_char = 0 as *const libc::c_char;
  let mut chp: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opts: libc::c_uint = 0;
  path = getenv(b"TMPDIR\x00" as *const u8 as *const libc::c_char);
  if path.is_null() || *path.offset(0) as libc::c_int == '\u{0}' as i32 {
    path = b"/tmp\x00" as *const u8 as *const libc::c_char
  }
  opts = getopt32(
    argv,
    b"^dqtp:u\x00?1\x00" as *const u8 as *const libc::c_char,
    &mut path as *mut *const libc::c_char,
  );
  chp = *argv.offset(optind as isize);
  if chp.is_null() {
    /* GNU coreutils 8.4:
     * bare "mktemp" -> "mktemp -t tmp.XXXXXX"
     */
    chp = xstrdup(b"tmp.XXXXXX\x00" as *const u8 as *const libc::c_char);
    opts |= OPT_t as libc::c_int as libc::c_uint
  }
  if opts & (OPT_t as libc::c_int | OPT_p as libc::c_int) as libc::c_uint != 0 {
    chp = concat_path_file(path, chp)
  }
  if opts & OPT_u as libc::c_int as libc::c_uint != 0 {
    chp = mktemp(chp);
    if *chp.offset(0) as libc::c_int == '\u{0}' as i32 {
      current_block = 11708925467940708064;
    } else {
      current_block = 4808432441040389987;
    }
  } else if opts & OPT_d as libc::c_int as libc::c_uint != 0 {
    if mkdtemp(chp).is_null() {
      current_block = 11708925467940708064;
    } else {
      current_block = 4808432441040389987;
    }
  } else if mkstemp(chp) < 0i32 {
    current_block = 11708925467940708064;
  } else {
    current_block = 4808432441040389987;
  }
  match current_block {
    11708925467940708064 => {
      if opts & OPT_q as libc::c_int as libc::c_uint != 0 {
        return 1i32;
      }
      /* don't use chp as it gets mangled in case of error */
      bb_perror_nomsg_and_die();
    }
    _ => {
      puts(chp);
      return 0i32;
    }
  };
}

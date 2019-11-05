use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn xmalloc_realpath_coreutils(path: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc_readlink_or_warn(path: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  static mut logmode: smallint;
}
pub type __uint32_t = libc::c_uint;
use crate::librb::uint32_t;
use crate::librb::smallint;
pub type C2RustUnnamed = libc::c_uint;
pub const LOGMODE_BOTH: C2RustUnnamed = 3;
pub const LOGMODE_SYSLOG: C2RustUnnamed = 2;
pub const LOGMODE_STDIO: C2RustUnnamed = 1;
pub const LOGMODE_NONE: C2RustUnnamed = 0;

/*
 * Mini readlink implementation for busybox
 *
 * Copyright (C) 2000,2001 Matt Kraai <kraai@alumni.carnegiemellon.edu>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config READLINK
//config:	bool "readlink (4 kb)"
//config:	default y
//config:	help
//config:	This program reads a symbolic link and returns the name
//config:	of the file it points to
//config:
//config:config FEATURE_READLINK_FOLLOW
//config:	bool "Enable canonicalization by following all symlinks (-f)"
//config:	default y
//config:	depends on READLINK
//config:	help
//config:	Enable the readlink option (-f).
//applet:IF_READLINK(APPLET_NOFORK(readlink, readlink, BB_DIR_USR_BIN, BB_SUID_DROP, readlink))
//kbuild:lib-$(CONFIG_READLINK) += readlink.o
//usage:#define readlink_trivial_usage
//usage:	IF_FEATURE_READLINK_FOLLOW("[-fnv] ") "FILE"
//usage:#define readlink_full_usage "\n\n"
//usage:       "Display the value of a symlink"
//usage:	IF_FEATURE_READLINK_FOLLOW( "\n"
//usage:     "\n	-f	Canonicalize by following all symlinks"
//usage:     "\n	-n	Don't add newline"
//usage:     "\n	-v	Verbose"
//usage:	)
/*
 * # readlink --version
 * readlink (GNU coreutils) 6.10
 * # readlink --help
 *   -f, --canonicalize
 *      canonicalize by following every symlink in
 *      every component of the given name recursively;
 *      all but the last component must exist
 *   -e, --canonicalize-existing
 *      canonicalize by following every symlink in
 *      every component of the given name recursively,
 *      all components must exist
 *   -m, --canonicalize-missing
 *      canonicalize by following every symlink in
 *      every component of the given name recursively,
 *      without requirements on components existence
 *   -n, --no-newline              do not output the trailing newline
 *   -q, --quiet, -s, --silent     suppress most error messages
 *   -v, --verbose                 report error messages
 *
 * bbox supports: -f (partially) -n -v (fully), -q -s (accepts but ignores)
 * Note: we export the -f flag, but our -f behaves like coreutils' -e.
 * Unfortunately, there isn't a C lib function we can leverage to get this
 * behavior which means we'd have to implement the full stack ourselves :(.
 */
#[no_mangle]
pub unsafe extern "C" fn readlink_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opt: libc::c_uint = 0;
  /* We need exactly one non-option argument.  */
  opt = getopt32(
    argv,
    b"^fnvsq\x00=1\x00" as *const u8 as *const libc::c_char,
  );
  fname = *argv.offset(optind as isize);
  /* compat: coreutils readlink reports errors silently via exit code */
  if opt & 4i32 as libc::c_uint == 0 {
    /* not -v */
    logmode = LOGMODE_NONE as libc::c_int as smallint
  }
  /* NOFORK: only one alloc is allowed; must free */
  if opt & 1i32 as libc::c_uint != 0 {
    /* -f */
    buf = xmalloc_realpath_coreutils(fname)
  } else {
    buf = xmalloc_readlink_or_warn(fname)
  }
  if buf.is_null() {
    return 1i32;
  }
  printf(
    if opt & 2i32 as libc::c_uint != 0 {
      b"%s\x00" as *const u8 as *const libc::c_char
    } else {
      b"%s\n\x00" as *const u8 as *const libc::c_char
    },
    buf,
  );
  free(buf as *mut libc::c_void);
  fflush_stdout_and_exit(0i32);
}

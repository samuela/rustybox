use libc;
extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> uint32_t;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_parse_mode(s: *const libc::c_char, cur_mode: libc::c_uint) -> libc::c_int;
  #[no_mangle]
  fn bb_make_directory(
    path: *mut libc::c_char,
    mode: libc::c_long,
    flags: libc::c_int,
  ) -> libc::c_int;
}
use crate::librb::__uint32_t;
pub type __mode_t = libc::c_uint;
use crate::librb::uint32_t;
use crate::librb::mode_t;
pub type C2RustUnnamed = libc::c_int;
pub const FILEUTILS_IGNORE_CHMOD_ERR: C2RustUnnamed = -2147483648;
pub const FILEUTILS_REFLINK_ALWAYS: C2RustUnnamed = 262144;
pub const FILEUTILS_REFLINK: C2RustUnnamed = 131072;
pub const FILEUTILS_RMDEST: C2RustUnnamed = 32768;
pub const FILEUTILS_NO_TARGET_DIR: C2RustUnnamed = 16384;
pub const FILEUTILS_UPDATE: C2RustUnnamed = 8192;
pub const FILEUTILS_VERBOSE: C2RustUnnamed = 4096;
pub const FILEUTILS_DEREFERENCE_L0: C2RustUnnamed = 256;
pub const FILEUTILS_DEREF_SOFTLINK: C2RustUnnamed = 128;
pub const FILEUTILS_MAKE_SOFTLINK: C2RustUnnamed = 64;
pub const FILEUTILS_MAKE_HARDLINK: C2RustUnnamed = 32;
pub const FILEUTILS_INTERACTIVE: C2RustUnnamed = 16;
pub const FILEUTILS_FORCE: C2RustUnnamed = 8;
pub const FILEUTILS_RECUR: C2RustUnnamed = 4;
pub const FILEUTILS_DEREFERENCE: C2RustUnnamed = 2;
pub const FILEUTILS_PRESERVE_STATUS: C2RustUnnamed = 1;

/*
 * Mini mkdir implementation for busybox
 *
 * Copyright (C) 2001 Matt Kraai <kraai@alumni.carnegiemellon.edu>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Mar 16, 2003      Manuel Novoa III   (mjn3@codepoet.org)
 *
 * Fixed broken permission setting when -p was used; especially in
 * conjunction with -m.
 */
/* Nov 28, 2006      Yoshinori Sato <ysato@users.sourceforge.jp>: Add SELinux Support.
 */
//config:config MKDIR
//config:	bool "mkdir (4.5 kb)"
//config:	default y
//config:	help
//config:	mkdir is used to create directories with the specified names.
//applet:IF_MKDIR(APPLET_NOFORK(mkdir, mkdir, BB_DIR_BIN, BB_SUID_DROP, mkdir))
//kbuild:lib-$(CONFIG_MKDIR) += mkdir.o
/* BB_AUDIT SUSv3 compliant */
/* http://www.opengroup.org/onlinepubs/007904975/utilities/mkdir.html */
//usage:#define mkdir_trivial_usage
//usage:       "[OPTIONS] DIRECTORY..."
//usage:#define mkdir_full_usage "\n\n"
//usage:       "Create DIRECTORY\n"
//usage:     "\n	-m MODE	Mode"
//usage:     "\n	-p	No error if exists; make parent directories as needed"
//usage:	IF_SELINUX(
//usage:     "\n	-Z	Set security context"
//usage:	)
//usage:
//usage:#define mkdir_example_usage
//usage:       "$ mkdir /tmp/foo\n"
//usage:       "$ mkdir /tmp/foo\n"
//usage:       "/tmp/foo: File exists\n"
//usage:       "$ mkdir /tmp/foo/bar/baz\n"
//usage:       "/tmp/foo/bar/baz: No such file or directory\n"
//usage:       "$ mkdir -p /tmp/foo/bar/baz\n"
/* This is a NOFORK applet. Be very careful! */
#[no_mangle]
pub unsafe extern "C" fn mkdir_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut mode: libc::c_long = -1i32 as libc::c_long;
  let mut status: libc::c_int = 0i32;
  let mut flags: libc::c_int = 0i32;
  let mut opt: libc::c_uint = 0;
  let mut smode: *mut libc::c_char = 0 as *mut libc::c_char;
  opt = getopt32long(
    argv,
    b"m:pv\x00" as *const u8 as *const libc::c_char,
    b"mode\x00\x01mparents\x00\x00pverbose\x00\x00v\x00" as *const u8 as *const libc::c_char,
    &mut smode as *mut *mut libc::c_char,
  );
  if opt & 1i32 as libc::c_uint != 0 {
    let mut mmode: mode_t = bb_parse_mode(smode, 0o777i32 as libc::c_uint) as mode_t;
    if mmode == -1i32 as mode_t {
      bb_error_msg_and_die(
        b"invalid mode \'%s\'\x00" as *const u8 as *const libc::c_char,
        smode,
      );
    }
    mode = mmode as libc::c_long
  }
  if opt & 2i32 as libc::c_uint != 0 {
    flags |= FILEUTILS_RECUR as libc::c_int
  }
  if opt & 4i32 as libc::c_uint != 0 && FILEUTILS_VERBOSE as libc::c_int != 0 {
    flags |= FILEUTILS_VERBOSE as libc::c_int
  }
  argv = argv.offset(optind as isize);
  if (*argv.offset(0)).is_null() {
    bb_show_usage();
  }
  loop {
    if bb_make_directory(*argv, mode, flags) != 0 {
      status = 1i32
    }
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  return status;
}

use libc;




extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn remove_file(path: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn bb_get_last_path_component_strip(path: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
}


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
 * Mini rm implementation for busybox
 *
 * Copyright (C) 2001 Matt Kraai <kraai@alumni.carnegiemellon.edu>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Mar 16, 2003      Manuel Novoa III   (mjn3@codepoet.org)
 *
 * Size reduction.
 */
//config:config RM
//config:	bool "rm (5.4 kb)"
//config:	default y
//config:	help
//config:	rm is used to remove files or directories.
//applet:IF_RM(APPLET_NOEXEC(rm, rm, BB_DIR_BIN, BB_SUID_DROP, rm))
/* was NOFORK, but then "rm -i FILE" can't be ^C'ed if run by hush */
//kbuild:lib-$(CONFIG_RM) += rm.o
/* BB_AUDIT SUSv3 compliant */
/* http://www.opengroup.org/onlinepubs/007904975/utilities/rm.html */
//usage:#define rm_trivial_usage
//usage:       "[-irf] FILE..."
//usage:#define rm_full_usage "\n\n"
//usage:       "Remove (unlink) FILEs\n"
//usage:     "\n	-i	Always prompt before removing"
//usage:     "\n	-f	Never prompt"
//usage:     "\n	-R,-r	Recurse"
//usage:
//usage:#define rm_example_usage
//usage:       "$ rm -rf /tmp/foo\n"
/* This is a NOEXEC applet. Be very careful! */
#[no_mangle]
pub unsafe extern "C" fn rm_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut status: libc::c_int = 0i32;
  let mut flags: libc::c_int = 0i32;
  let mut opt: libc::c_uint = 0;
  opt = getopt32(
    argv,
    b"^fiRrv\x00f-i:i-f\x00" as *const u8 as *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  if opt & 1i32 as libc::c_uint != 0 {
    flags |= FILEUTILS_FORCE as libc::c_int
  }
  if opt & 2i32 as libc::c_uint != 0 {
    flags |= FILEUTILS_INTERACTIVE as libc::c_int
  }
  if opt & (8i32 | 4i32) as libc::c_uint != 0 {
    flags |= FILEUTILS_RECUR as libc::c_int
  }
  if opt & 16i32 as libc::c_uint != 0 && FILEUTILS_VERBOSE as libc::c_int != 0 {
    flags |= FILEUTILS_VERBOSE as libc::c_int
  }
  if !(*argv).is_null() {
    let mut current_block_12: u64;
    loop {
      let mut base: *const libc::c_char = bb_get_last_path_component_strip(*argv);
      if *base.offset(0) as libc::c_int == '.' as i32
        && (*base.offset(1) == 0
          || *base.offset(1) as libc::c_int == '.' as i32 && *base.offset(2) == 0)
      {
        bb_simple_error_msg(
          b"can\'t remove \'.\' or \'..\'\x00" as *const u8 as *const libc::c_char,
        );
        current_block_12 = 15652330335145281839;
      } else if remove_file(*argv, flags) >= 0i32 {
        current_block_12 = 7651349459974463963;
      } else {
        current_block_12 = 15652330335145281839;
      }
      match current_block_12 {
        15652330335145281839 => status = 1i32,
        _ => {}
      }
      argv = argv.offset(1);
      if (*argv).is_null() {
        break;
      }
    }
  } else if flags & FILEUTILS_FORCE as libc::c_int == 0 {
    bb_show_usage();
  }
  return status;
}

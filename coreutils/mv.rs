use crate::libbb::ptr_to_globals::bb_errno;
use libc;
use libc::access;
use libc::fprintf;
use libc::free;
use libc::isatty;
use libc::lstat;
use libc::printf;
use libc::rename;
use libc::stat;
use libc::unlink;
use libc::FILE;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut stderr: *mut FILE;

  #[no_mangle]
  fn remove_file(path: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn copy_file(
    source: *const libc::c_char,
    dest: *const libc::c_char,
    flags: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn bb_get_last_path_component_strip(path: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> u32;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_ask_y_confirmation() -> libc::c_int;
  #[no_mangle]
  fn concat_path_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;

  #[no_mangle]
  fn cp_mv_stat2(fn_0: *const libc::c_char, fn_stat: *mut stat, sf: stat_func) -> libc::c_int;
  #[no_mangle]
  fn cp_mv_stat(fn_0: *const libc::c_char, fn_stat: *mut stat) -> libc::c_int;
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
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
pub type stat_func =
  Option<unsafe extern "C" fn(_: *const libc::c_char, _: *mut stat) -> libc::c_int>;

/*
 * Mini mv implementation for busybox
 *
 * Copyright (C) 2000 by Matt Kraai <kraai@alumni.carnegiemellon.edu>
 * SELinux support by Yuichi Nakamura <ynakam@hitachisoft.jp>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Mar 16, 2003      Manuel Novoa III   (mjn3@codepoet.org)
 *
 * Size reduction and improved error checking.
 */
//config:config MV
//config:	bool "mv (10 kb)"
//config:	default y
//config:	help
//config:	mv is used to move or rename files or directories.
//applet:IF_MV(APPLET_NOEXEC(mv, mv, BB_DIR_BIN, SUID_DROP, mv))
/* NOEXEC despite cases when it can be a "runner" (mv LARGE_DIR OTHER_FS) */
//kbuild:lib-$(CONFIG_MV) += mv.o
//usage:#define mv_trivial_usage
//usage:       "[-fin] SOURCE DEST\n"
//usage:       "or: mv [-fin] SOURCE... DIRECTORY"
//usage:#define mv_full_usage "\n\n"
//usage:       "Rename SOURCE to DEST, or move SOURCE(s) to DIRECTORY\n"
//usage:     "\n	-f	Don't prompt before overwriting"
//usage:     "\n	-i	Interactive, prompt before overwrite"
//usage:     "\n	-n	Don't overwrite an existing file"
//usage:
//usage:#define mv_example_usage
//usage:       "$ mv /tmp/foo /bin/bar\n"
#[no_mangle]
pub unsafe extern "C" fn mv_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut source_stat: stat = std::mem::zeroed();
  let mut source_exists: libc::c_int = 0;
  let mut current_block: u64;
  let mut dest_stat: stat = std::mem::zeroed();
  let mut last: *const libc::c_char = 0 as *const libc::c_char;
  let mut dest: *const libc::c_char = 0 as *const libc::c_char;
  let mut flags: libc::c_uint = 0;
  let mut dest_exists: libc::c_int = 0;
  let mut status: libc::c_int = 0i32;
  let mut copy_flag: libc::c_int = 0i32;
  /* Need at least two arguments.
   * If more than one of -f, -i, -n is specified , only the final one
   * takes effect (it unsets previous options).
   */
  flags = getopt32long(
    argv,
    b"^finv\x00-2:f-in:i-fn:n-fi\x00" as *const u8 as *const libc::c_char,
    b"interactive\x00\x00iforce\x00\x00fno-clobber\x00\x00nverbose\x00\x00v\x00" as *const u8
      as *const libc::c_char,
  );
  argc -= optind;
  argv = argv.offset(optind as isize);
  last = *argv.offset((argc - 1i32) as isize);
  if argc == 2i32 {
    dest_exists = cp_mv_stat(last, &mut dest_stat);
    if dest_exists < 0i32 {
      return 1i32;
    }
    if dest_exists & 2i32 == 0 {
      /* last is not a directory */
      dest = last;
      current_block = 4372395669998863707;
    } else {
      current_block = 17407779659766490442;
    }
  } else {
    current_block = 17407779659766490442;
  }
  loop {
    match current_block {
      17407779659766490442 => {
        dest = concat_path_file(last, bb_get_last_path_component_strip(*argv));
        dest_exists = cp_mv_stat(dest, &mut dest_stat);
        if !(dest_exists < 0i32) {
          current_block = 4372395669998863707;
          continue;
        }
        current_block = 6059157660367733168;
      }
      _ => {
        if dest_exists != 0 {
          if flags & (1i32 << 2i32) as libc::c_uint != 0 {
            current_block = 11386481267603146021;
          } else if flags & (1i32 << 0i32) as libc::c_uint == 0
            && (access(dest, 2i32) < 0i32 && isatty(0i32) != 0
              || flags & (1i32 << 1i32) as libc::c_uint != 0)
          {
            if fprintf(
              stderr,
              b"mv: overwrite \'%s\'? \x00" as *const u8 as *const libc::c_char,
              dest,
            ) < 0i32
            {
              current_block = 6059157660367733168;
            } else if bb_ask_y_confirmation() == 0 {
              current_block = 11386481267603146021;
            } else {
              current_block = 14763689060501151050;
            }
          } else {
            current_block = 14763689060501151050;
          }
        } else {
          current_block = 14763689060501151050;
        }
        match current_block {
          11386481267603146021 => {}
          6059157660367733168 => {}
          _ => {
            if rename(*argv, dest) < 0i32 {
              source_stat = std::mem::zeroed();
              source_exists = 0;
              if *bb_errno != 18i32 || {
                source_exists = cp_mv_stat2(
                  *argv,
                  &mut source_stat,
                  Some(
                    lstat
                      as unsafe extern "C" fn(_: *const libc::c_char, _: *mut stat) -> libc::c_int,
                  ),
                );
                (source_exists) < 1i32
              } {
                bb_perror_msg(
                  b"can\'t rename \'%s\'\x00" as *const u8 as *const libc::c_char,
                  *argv,
                );
                current_block = 6059157660367733168;
              } else {
                static mut fmt: [libc::c_char; 45] = [
                  99, 97, 110, 39, 116, 32, 111, 118, 101, 114, 119, 114, 105, 116, 101, 32, 37,
                  115, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 119, 105, 116, 104, 32, 37,
                  115, 100, 105, 114, 101, 99, 116, 111, 114, 121, 0,
                ];
                if dest_exists != 0 {
                  if dest_exists == 3i32 {
                    if source_exists != 3i32 {
                      bb_error_msg(
                        fmt.as_ptr(),
                        b"\x00" as *const u8 as *const libc::c_char,
                        b"non-\x00" as *const u8 as *const libc::c_char,
                      );
                      current_block = 6059157660367733168;
                    } else {
                      current_block = 10891380440665537214;
                    }
                  } else if source_exists == 3i32 {
                    bb_error_msg(
                      fmt.as_ptr(),
                      b"non-\x00" as *const u8 as *const libc::c_char,
                      b"\x00" as *const u8 as *const libc::c_char,
                    );
                    current_block = 6059157660367733168;
                  } else {
                    current_block = 10891380440665537214;
                  }
                  match current_block {
                    6059157660367733168 => {}
                    _ => {
                      if unlink(dest) < 0i32 {
                        bb_perror_msg(
                          b"can\'t remove \'%s\'\x00" as *const u8 as *const libc::c_char,
                          dest,
                        );
                        current_block = 6059157660367733168;
                      } else {
                        current_block = 1356832168064818221;
                      }
                    }
                  }
                } else {
                  current_block = 1356832168064818221;
                }
                match current_block {
                  6059157660367733168 => {}
                  _ => {
                    /* FILEUTILS_RECUR also prevents nasties like
                     * "read from device and write contents to dst"
                     * instead of "create same device node" */
                    copy_flag =
                      FILEUTILS_RECUR as libc::c_int | FILEUTILS_PRESERVE_STATUS as libc::c_int;
                    if copy_file(*argv, dest, copy_flag) >= 0i32
                      && remove_file(
                        *argv,
                        FILEUTILS_RECUR as libc::c_int | FILEUTILS_FORCE as libc::c_int,
                      ) >= 0i32
                    {
                      current_block = 11386481267603146021;
                    } else {
                      current_block = 6059157660367733168;
                    }
                  }
                }
              }
            } else {
              current_block = 11386481267603146021;
            }
          }
        }
      }
    }
    match current_block {
      6059157660367733168 => {
        status = 1i32
        /* Ouch! fprintf failed! */
      }
      _ => {}
    }
    if flags & ((1i32 << 3i32) * 1i32) as libc::c_uint != 0 {
      printf(
        b"\'%s\' -> \'%s\'\n\x00" as *const u8 as *const libc::c_char,
        *argv,
        dest,
      );
    }
    if dest != last {
      free(dest as *mut libc::c_void);
    }
    argv = argv.offset(1);
    if *argv != last as *mut libc::c_char {
      current_block = 17407779659766490442;
    } else {
      break;
    }
  }
  return status;
}

use libc;
extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn open_or_warn_stdin(pathname: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf_0: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
use crate::librb::ssize_t;
use crate::librb::uint32_t;
use crate::librb::size_t;
pub type C2RustUnnamed = libc::c_uint;
pub const COMMON_BUFSIZE: C2RustUnnamed = 1024;
/* vi: set sw=4 ts=4: */
/*
 * sum -- checksum and count the blocks in a file
 *     Like BSD sum or SysV sum -r, except like SysV sum if -s option is given.
 *
 * Copyright (C) 86, 89, 91, 1995-2002, 2004 Free Software Foundation, Inc.
 * Copyright (C) 2005 by Erik Andersen <andersen@codepoet.org>
 * Copyright (C) 2005 by Mike Frysinger <vapier@gentoo.org>
 *
 * Written by Kayvan Aghaiepour and David MacKenzie
 * Taken from coreutils and turned into a busybox applet by Mike Frysinger
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config SUM
//config:	bool "sum (4 kb)"
//config:	default y
//config:	help
//config:	checksum and count the blocks in a file
//applet:IF_SUM(APPLET(sum, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_SUM) += sum.o
//usage:#define sum_trivial_usage
//usage:       "[-rs] [FILE]..."
//usage:#define sum_full_usage "\n\n"
//usage:       "Checksum and count the blocks in a file\n"
//usage:     "\n	-r	Use BSD sum algorithm (1K blocks)"
//usage:     "\n	-s	Use System V sum algorithm (512byte blocks)"
pub type C2RustUnnamed_0 = libc::c_uint;
pub const SUM_SYSV: C2RustUnnamed_0 = 2;
pub const PRINT_NAME: C2RustUnnamed_0 = 1;
pub const SUM_BSD: C2RustUnnamed_0 = 0;
/* BSD: calculate and print the rotated checksum and the size in 1K blocks
The checksum varies depending on sizeof (int). */
/* SYSV: calculate and print the checksum and the size in 512-byte blocks */
/* Return 1 if successful.  */
unsafe extern "C" fn sum_file(
  mut file: *const libc::c_char,
  mut type_0: libc::c_uint,
) -> libc::c_uint {
  let mut total_bytes: libc::c_ulonglong = 0i32 as libc::c_ulonglong;
  let mut fd: libc::c_int = 0;
  let mut r: libc::c_int = 0;
  /* The sum of all the input bytes, modulo (UINT_MAX + 1).  */
  let mut s: libc::c_uint = 0i32 as libc::c_uint;
  fd = open_or_warn_stdin(file);
  if fd == -1i32 {
    return 0i32 as libc::c_uint;
  }
  loop {
    let mut bytes_read: size_t = safe_read(
      fd,
      bb_common_bufsiz1.as_mut_ptr() as *mut libc::c_void,
      COMMON_BUFSIZE as libc::c_int as size_t,
    ) as size_t;
    if bytes_read as ssize_t <= 0i32 as libc::c_long {
      r = (fd != 0 && close(fd) != 0i32) as libc::c_int;
      if bytes_read == 0 && r == 0 {
        break;
      }
      bb_simple_perror_msg(file);
      return 0i32 as libc::c_uint;
    } else {
      total_bytes = total_bytes.wrapping_add(bytes_read as libc::c_ulonglong);
      if type_0 >= SUM_SYSV as libc::c_int as libc::c_uint {
        loop {
          bytes_read = bytes_read.wrapping_sub(1);
          s = s.wrapping_add(
            *bb_common_bufsiz1.as_mut_ptr().offset(bytes_read as isize) as libc::c_uint
          );
          if !(bytes_read != 0) {
            break;
          }
        }
      } else {
        r = 0i32;
        loop {
          s = (s >> 1i32).wrapping_add((s & 1i32 as libc::c_uint) << 15i32);
          let fresh0 = r;
          r = r + 1;
          s =
            s.wrapping_add(*bb_common_bufsiz1.as_mut_ptr().offset(fresh0 as isize) as libc::c_uint);
          s &= 0xffffi32 as libc::c_uint;
          bytes_read = bytes_read.wrapping_sub(1);
          if !(bytes_read != 0) {
            break;
          }
          /* Keep it within bounds. */
        }
      }
    }
  }
  if type_0 < PRINT_NAME as libc::c_int as libc::c_uint {
    file = b"\x00" as *const u8 as *const libc::c_char
  }
  if type_0 >= SUM_SYSV as libc::c_int as libc::c_uint {
    r = (s & 0xffffi32 as libc::c_uint).wrapping_add((s & 0xffffffffu32) >> 16i32) as libc::c_int;
    s = ((r & 0xffffi32) + (r >> 16i32)) as libc::c_uint;
    printf(
      b"%u %llu %s\n\x00" as *const u8 as *const libc::c_char,
      s,
      total_bytes
        .wrapping_add(511i32 as libc::c_ulonglong)
        .wrapping_div(512i32 as libc::c_ulonglong),
      file,
    );
  } else {
    printf(
      b"%05u %5llu %s\n\x00" as *const u8 as *const libc::c_char,
      s,
      total_bytes
        .wrapping_add(1023i32 as libc::c_ulonglong)
        .wrapping_div(1024i32 as libc::c_ulonglong),
      file,
    );
  }
  return 1i32 as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn sum_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut n: libc::c_uint = 0;
  let mut type_0: libc::c_uint = SUM_BSD as libc::c_int as libc::c_uint;
  n = getopt32(argv, b"sr\x00" as *const u8 as *const libc::c_char);
  argv = argv.offset(optind as isize);
  if n & 1i32 as libc::c_uint != 0 {
    type_0 = SUM_SYSV as libc::c_int as libc::c_uint
  }
  /* give the bsd priority over sysv func */
  if n & 2i32 as libc::c_uint != 0 {
    type_0 = SUM_BSD as libc::c_int as libc::c_uint
  }
  if (*argv.offset(0)).is_null() {
    /* Do not print the name */
    n = sum_file(b"-\x00" as *const u8 as *const libc::c_char, type_0)
  } else {
    /* Need to print the name if either
     * - more than one file given
     * - doing sysv */
    type_0 = type_0.wrapping_add(
      (!(*argv.offset(1)).is_null() || type_0 == SUM_SYSV as libc::c_int as libc::c_uint)
        as libc::c_int as libc::c_uint,
    );
    n = 1i32 as libc::c_uint;
    loop {
      n &= sum_file(*argv, type_0);
      argv = argv.offset(1);
      if (*argv).is_null() {
        break;
      }
    }
  }
  return (n == 0) as libc::c_int;
}

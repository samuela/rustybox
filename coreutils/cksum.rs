use crate::librb::size_t;
use crate::librb::uoff_t;
use libc;
use libc::close;
use libc::off_t;
use libc::printf;
extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static bb_msg_standard_input: [libc::c_char; 0];

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

pub type C2RustUnnamed = libc::c_uint;
pub const COMMON_BUFSIZE: C2RustUnnamed = 1024;

/*
 * cksum - calculate the CRC32 checksum of a file
 *
 * Copyright (C) 2006 by Rob Sullivan, with ideas from code by Walter Harms
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config CKSUM
//config:	bool "cksum (4.1 kb)"
//config:	default y
//config:	help
//config:	cksum is used to calculate the CRC32 checksum of a file.
//applet:IF_CKSUM(APPLET_NOEXEC(cksum, cksum, BB_DIR_USR_BIN, SUID_DROP, cksum))
/* bb_common_bufsiz1 usage here is safe wrt NOEXEC: not expecting it to be zeroed. */
//kbuild:lib-$(CONFIG_CKSUM) += cksum.o
//usage:#define cksum_trivial_usage
//usage:       "FILE..."
//usage:#define cksum_full_usage "\n\n"
//usage:       "Calculate the CRC32 checksums of FILEs"
/* This is a NOEXEC applet. Be very careful! */
pub unsafe fn cksum_main(mut _argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut crc32_table: *mut u32 = crate::libbb::crc32::crc32_filltable(0 as *mut u32, 1i32); /* coreutils 6.9 compat */
  let mut exit_code: libc::c_int = 0;
  crate::libbb::getopt32::getopt32(argv, b"\x00" as *const u8 as *const libc::c_char);
  argv = argv.offset(optind as isize);
  loop {
    let mut crc: u32 = 0;
    let mut filesize: off_t = 0;
    let mut fd: libc::c_int =
      crate::libbb::wfopen_input::open_or_warn_stdin(if !(*argv).is_null() {
        *argv
      } else {
        bb_msg_standard_input.as_ptr()
      });
    if fd < 0 {
      exit_code = 1i32
    } else {
      crc = 0 as u32;
      filesize = 0 as off_t;
      loop {
        let mut t: uoff_t = 0;
        let mut bytes_read: libc::c_int = crate::libbb::read::safe_read(
          fd,
          bb_common_bufsiz1.as_mut_ptr() as *mut libc::c_void,
          COMMON_BUFSIZE as libc::c_int as size_t,
        ) as libc::c_int;
        if bytes_read > 0 {
          filesize += bytes_read as libc::c_long
        } else {
          /* Checksum filesize bytes, LSB first, and exit */
          close(fd); /* break flag */
          fd = -1i32;
          t = filesize as uoff_t;
          bytes_read = 0;
          while t != 0 as libc::c_ulong {
            let fresh0 = bytes_read;
            bytes_read = bytes_read + 1;
            *bb_common_bufsiz1.as_mut_ptr().offset(fresh0 as isize) = t as u8 as libc::c_char;
            t >>= 8i32
          }
        }
        crc = crate::libbb::crc32::crc32_block_endian1(
          crc,
          bb_common_bufsiz1.as_mut_ptr() as *const libc::c_void,
          bytes_read as libc::c_uint,
          crc32_table,
        );
        if fd < 0 {
          break;
        }
      }
      crc = !crc;
      printf(
        if !(*argv).is_null() {
          b"%u %lu %s\n\x00" as *const u8 as *const libc::c_char
        } else {
          b"%u %lu\n\x00" as *const u8 as *const libc::c_char
        },
        crc,
        filesize,
        *argv,
      );
    }
    if !(!(*argv).is_null() && {
      argv = argv.offset(1);
      !(*argv).is_null()
    }) {
      break;
    }
  }
  crate::libbb::fflush_stdout_and_exit::fflush_stdout_and_exit(exit_code);
}

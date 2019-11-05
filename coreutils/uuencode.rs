use libc;
extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut stdout: *mut _IO_FILE;
  #[no_mangle]
  fn fflush(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn umask(__mask: __mode_t) -> __mode_t;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  /* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
   * But potentially slow, don't use in one-billion-times loops */
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  // NB: will return short read on error, not -1,
  // if some data was read before error occurred
  #[no_mangle]
  fn full_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn xwrite(fd: libc::c_int, buf: *const libc::c_void, count: size_t);
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  static bb_uuenc_tbl_base64: [libc::c_char; 0];
  #[no_mangle]
  static bb_uuenc_tbl_std: [libc::c_char; 0];
  #[no_mangle]
  fn bb_uuencode(
    store: *mut libc::c_char,
    s: *const libc::c_void,
    length: libc::c_int,
    tbl: *const libc::c_char,
  );
}
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type mode_t = __mode_t;
use crate::librb::stat;
use crate::librb::timespec;
use crate::librb::_IO_FILE;
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
  pub _next: *mut _IO_marker,
  pub _sbuf: *mut _IO_FILE,
  pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
/* vi: set sw=4 ts=4: */
/*
 * Copyright (C) 2000 by Glenn McGrath
 *
 * based on the function base64_encode from http.c in wget v1.6
 * Copyright (C) 1995, 1996, 1997, 1998, 2000 Free Software Foundation, Inc.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config UUENCODE
//config:	bool "uuencode (4.4 kb)"
//config:	default y
//config:	help
//config:	uuencode is used to uuencode a file.
//applet:IF_UUENCODE(APPLET(uuencode, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_UUENCODE) += uuencode.o
//usage:#define uuencode_trivial_usage
//usage:       "[-m] [FILE] STORED_FILENAME"
//usage:#define uuencode_full_usage "\n\n"
//usage:       "Uuencode FILE (or stdin) to stdout\n"
//usage:     "\n	-m	Use base64 encoding per RFC1521"
//usage:
//usage:#define uuencode_example_usage
//usage:       "$ uuencode busybox busybox\n"
//usage:       "begin 755 busybox\n"
//usage:       "<encoded file snipped>\n"
//usage:       "$ uudecode busybox busybox > busybox.uu\n"
//usage:       "$\n"
pub type C2RustUnnamed = libc::c_uint;
/* This *MUST* be a multiple of 3 */
pub const DST_BUF_SIZE: C2RustUnnamed = 60;
pub const SRC_BUF_SIZE: C2RustUnnamed = 45;
#[no_mangle]
pub unsafe extern "C" fn uuencode_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut stat_buf: stat = stat {
    st_dev: 0,
    st_ino: 0,
    st_nlink: 0,
    st_mode: 0,
    st_uid: 0,
    st_gid: 0,
    __pad0: 0,
    st_rdev: 0,
    st_size: 0,
    st_blksize: 0,
    st_blocks: 0,
    st_atim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_mtim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_ctim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    __glibc_reserved: [0; 3],
  };
  let mut src_fd: libc::c_int = 0i32;
  let mut tbl: *const libc::c_char = 0 as *const libc::c_char;
  let mut mode: mode_t = 0;
  let mut src_buf: [libc::c_char; 45] = [0; 45];
  let mut dst_buf: [libc::c_char; 61] = [0; 61];
  tbl = bb_uuenc_tbl_std.as_ptr();
  mode = 0o666i32 as libc::c_uint & !umask(0o666i32 as __mode_t);
  if getopt32(argv, b"^m\x00-1:?2\x00" as *const u8 as *const libc::c_char) != 0 {
    tbl = bb_uuenc_tbl_base64.as_ptr()
  }
  argv = argv.offset(optind as isize);
  if !(*argv.offset(1)).is_null() {
    src_fd = xopen(*argv.offset(0), 0i32);
    fstat(src_fd, &mut stat_buf);
    mode = stat_buf.st_mode
      & (0o400i32
        | 0o200i32
        | 0o100i32
        | (0o400i32 | 0o200i32 | 0o100i32) >> 3i32
        | (0o400i32 | 0o200i32 | 0o100i32) >> 3i32 >> 3i32) as libc::c_uint;
    argv = argv.offset(1)
  }
  printf(
    b"begin%s %o %s\x00" as *const u8 as *const libc::c_char,
    if tbl == bb_uuenc_tbl_std.as_ptr() {
      b"\x00" as *const u8 as *const libc::c_char
    } else {
      b"-base64\x00" as *const u8 as *const libc::c_char
    },
    mode,
    *argv,
  );
  loop {
    let mut size: size_t = full_read(
      src_fd,
      src_buf.as_mut_ptr() as *mut libc::c_void,
      SRC_BUF_SIZE as libc::c_int as size_t,
    ) as size_t;
    if size == 0 {
      break;
    }
    if (size as ssize_t) < 0i32 as libc::c_long {
      bb_simple_perror_msg_and_die(b"read error\x00" as *const u8 as *const libc::c_char);
    }
    /* Encode the buffer we just read in */
    bb_uuencode(
      dst_buf.as_mut_ptr(),
      src_buf.as_mut_ptr() as *const libc::c_void,
      size as libc::c_int,
      tbl,
    );
    bb_putchar('\n' as i32);
    if tbl == bb_uuenc_tbl_std.as_ptr() {
      bb_putchar(*tbl.offset(size as isize) as libc::c_int);
    }
    fflush(stdout);
    xwrite(
      1i32,
      dst_buf.as_mut_ptr() as *const libc::c_void,
      (4i32 as libc::c_ulong).wrapping_mul(
        size
          .wrapping_add(2i32 as libc::c_ulong)
          .wrapping_div(3i32 as libc::c_ulong),
      ),
    );
  }
  printf(if tbl == bb_uuenc_tbl_std.as_ptr() {
    b"\n`\nend\n\x00" as *const u8 as *const libc::c_char
  } else {
    b"\n====\n\x00" as *const u8 as *const libc::c_char
  });
  fflush_stdout_and_exit(0i32);
}

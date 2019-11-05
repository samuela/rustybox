use libc;
extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn ftruncate(__fd: libc::c_int, __length: __off64_t) -> libc::c_int;
  #[no_mangle]
  fn fdatasync(__fildes: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
  #[no_mangle]
  fn bb_copyfd_size(fd1: libc::c_int, fd2: libc::c_int, size: off_t) -> off_t;
  #[no_mangle]
  fn xunlink(pathname: *const libc::c_char);
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xlseek(fd: libc::c_int, offset: off_t, whence: libc::c_int) -> off_t;
  #[no_mangle]
  fn xclose(fd: libc::c_int);
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_show_usage() -> !;
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
pub type __syscall_slong_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type off_t = __off64_t;
use crate::librb::timespec;
use crate::librb::stat;
pub const OPT_u: C2RustUnnamed = 2;
pub const OPT_z: C2RustUnnamed = 4;
pub const OPT_f: C2RustUnnamed = 1;
pub type C2RustUnnamed = libc::c_uint;
pub const OPT_x: C2RustUnnamed = 32;
pub const OPT_v: C2RustUnnamed = 16;
pub const OPT_n: C2RustUnnamed = 8;
/* vi: set sw=4 ts=4: */
/*
 * Copyright (C) 2017 Denys Vlasenko <vda.linux@googlemail.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config SHRED
//config:	bool "shred (4.9 kb)"
//config:	default y
//config:	help
//config:	Overwrite a file to hide its contents, and optionally delete it
//applet:IF_SHRED(APPLET(shred, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_SHRED) += shred.o
//usage:#define shred_trivial_usage
//usage:       "FILE..."
//usage:#define shred_full_usage "\n\n"
//usage:       "Overwrite/delete FILEs\n"
//usage:     "\n	-f	Chmod to ensure writability"
//usage:     "\n	-n N	Overwrite N times (default 3)"
//usage:     "\n	-z	Final overwrite with zeros"
//usage:     "\n	-u	Remove file"
//-x and -v are accepted but have no effect
/* shred (GNU coreutils) 8.25:
-f, --force		change permissions to allow writing if necessary
-u			truncate and remove file after overwriting
-z, --zero		add a final overwrite with zeros to hide shredding
-n, --iterations=N	overwrite N times instead of the default (3)
-v, --verbose		show progress
-x, --exact		do not round file sizes up to the next full block; this is the default for non-regular files
--random-source=FILE	get random bytes from FILE
-s, --size=N		shred this many bytes (suffixes like K, M, G accepted)
--remove[=HOW]		like -u but give control on HOW to delete;  See below
*/
#[no_mangle]
pub unsafe extern "C" fn shred_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut rand_fd: libc::c_int = 0; /* for compiler */
  rand_fd = rand_fd;
  let mut zero_fd: libc::c_int = 0;
  let mut num_iter: libc::c_uint = 3i32 as libc::c_uint;
  let mut opt: libc::c_uint = 0;
  opt = getopt32(
    argv,
    b"fuzn:+vx\x00" as *const u8 as *const libc::c_char,
    &mut num_iter as *mut libc::c_uint,
  );
  argv = argv.offset(optind as isize);
  zero_fd = xopen(b"/dev/zero\x00" as *const u8 as *const libc::c_char, 0i32);
  if num_iter != 0i32 as libc::c_uint {
    rand_fd = xopen(
      b"/dev/urandom\x00" as *const u8 as *const libc::c_char,
      0i32,
    )
  }
  if (*argv).is_null() {
    bb_show_usage();
  }
  loop {
    let mut sb: stat = stat {
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
    let mut fname: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_uint = 0;
    let mut fd: libc::c_int = 0;
    let fresh0 = argv;
    argv = argv.offset(1);
    fname = *fresh0;
    if fname.is_null() {
      break;
    }
    fd = -1i32;
    if opt & OPT_f as libc::c_int as libc::c_uint != 0 {
      fd = open(fname, 0o1i32);
      if fd < 0i32 {
        chmod(fname, 0o666i32 as __mode_t);
      }
    }
    if fd < 0i32 {
      fd = xopen(fname, 0o1i32)
    }
    if fstat(fd, &mut sb) == 0i32 && sb.st_size > 0i32 as libc::c_long {
      let mut size: off_t = sb.st_size;
      i = 0i32 as libc::c_uint;
      while i < num_iter {
        bb_copyfd_size(rand_fd, fd, size);
        fdatasync(fd);
        xlseek(fd, 0i32 as off_t, 0i32);
        i = i.wrapping_add(1)
      }
      if opt & OPT_z as libc::c_int as libc::c_uint != 0 {
        bb_copyfd_size(zero_fd, fd, size);
        fdatasync(fd);
      }
      if opt & OPT_u as libc::c_int as libc::c_uint != 0 {
        ftruncate(fd, 0i32 as __off64_t);
        xunlink(fname);
      }
      xclose(fd);
    }
  }
  return 0i32;
}

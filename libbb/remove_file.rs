use libc;
extern "C" {
  pub type __dirstream;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn isatty(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn unlink(__name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn rmdir(__path: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn opendir(__name: *const libc::c_char) -> *mut DIR;
  #[no_mangle]
  fn closedir(__dirp: *mut DIR) -> libc::c_int;
  #[no_mangle]
  fn readdir(__dirp: *mut DIR) -> *mut dirent;
  #[no_mangle]
  static mut stderr: *mut FILE;
  #[no_mangle]
  fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_ask_y_confirmation() -> libc::c_int;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  fn concat_subpath_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __ino64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
use crate::librb::size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
  pub d_ino: __ino64_t,
  pub d_off: __off64_t,
  pub d_reclen: libc::c_ushort,
  pub d_type: libc::c_uchar,
  pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
use crate::librb::stat;
use crate::librb::timespec;



use crate::librb::FILE;
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
/* vi: set sw=4 ts=4: */
/*
 * Busybox main internal header file
 *
 * Based in part on code from sash, Copyright (c) 1999 by David I. Bell
 * Permission has been granted to redistribute this code under GPL.
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* TODO: and glibc? */
/* There are two incompatible basename's, let's not use them! */
/* See the dirname/basename man page for details */
/* dirname,basename */
/* Don't do this here:
 * #include <sys/sysinfo.h>
 * Some linux/ includes pull in conflicting definition
 * of struct sysinfo (only in some toolchanins), which breaks build.
 * Include sys/sysinfo.h only in those files which need it.
 */
/* Just in case libc doesn't define some of these... */
//This breaks on bionic:
//# if !defined(__socklen_t_defined) && !defined(_SOCKLEN_T_DECLARED)
// /* We #define socklen_t *after* includes, otherwise we get
// * typedef redefinition errors from system headers
// * (in case "is it defined already" detection above failed)
// */
//#  define socklen_t bb_socklen_t
//   typedef unsigned socklen_t;
//# endif
//if this is still needed, add a fix along the lines of
//  ifdef SPECIFIC_BROKEN_LIBC_CHECK / typedef socklen_t / endif
//in platform.h instead!
/*
 * Use '%m' to append error string on platforms that support it,
 * '%s' and strerror() on those that don't.
 */
/*nothing*/
/* Some libc's forget to declare these, do it ourself */
/* klogctl is in libc's klog.h, but we cheat and not #include that */
/* Busybox does not use threads, we can speed up stdio. */
/* Above functions are required by POSIX.1-2008, below ones are extensions */
/* musl <= 1.1.15 does not support fflush_unlocked(NULL) */
//# undef  fflush
//# define fflush(stream) fflush_unlocked(stream)
/* Make all declarations hidden (-fvisibility flag only affects definitions) */
/* (don't include system headers after this until corresponding pop!) */
/* Tested to work correctly with all int types (IIRC :]) */
/* Large file support */
/* Note that CONFIG_LFS=y forces bbox to be built with all common ops
 * (stat, lseek etc) mapped to "largefile" variants by libc.
 * Practically it means that open() automatically has O_LARGEFILE added
 * and all filesize/file_offset parameters and struct members are "large"
 * (in today's world - signed 64bit). For full support of large files,
 * we need a few helper #defines (below) and careful use of off_t
 * instead of int/ssize_t. No lseek64(), O_LARGEFILE etc necessary */
/* CONFIG_LFS is on */
/* "long" is long enough on this system */
/* usage: sz = BB_STRTOOFF(s, NULL, 10); if (errno || sz < 0) die(); */
/* usage: printf("size: %"OFF_FMT"d (%"OFF_FMT"x)\n", sz, sz); */
/* scary. better ideas? (but do *test* them first!) */
/* Users report bionic to use 32-bit off_t even if LARGEFILE support is requested.
 * We misdetected that. Don't let it build:
 */
/* Some useful definitions */
/* Macros for min/max.  */
/* buffer allocation schemes */
/* glibc uses __errno_location() to get a ptr to errno */
/* We can just memorize it once - no multithreading in busybox :) */
/* dmalloc will redefine these to it's own implementation. It is safe
 * to have the prototypes here unconditionally.  */
/* After v = xrealloc_vector(v, SHIFT, idx) it's ok to use
 * at least v[idx] and v[idx+1], for all idx values.
 * SHIFT specifies how many new elements are added (1:2, 2:4, ..., 8:256...)
 * when all elements are used up. New elements are zeroed out.
 * xrealloc_vector(v, SHIFT, idx) *MUST* be called with consecutive IDXs -
 * skipping an index is a bad bug - it may miss a realloc!
 */
//TODO: supply a pointer to char[11] buffer (avoid statics)?
/* cp.c, mv.c, install.c depend on these values. CAREFUL when changing them! */
/* -p */
/* !-d */
/* -R */
/* -f */
/* -i */
/* -l */
/* -s */
/* -L */
/* -H */
/* -a = -pdR (mapped in cp.c) */
/* -r = -dR  (mapped in cp.c) */
/* -P = -d   (mapped in cp.c) */
/* -v */
/* -u */
/* -T */
/* --remove-destination */
/* bit 17 skipped for "cp --parents" */
/* cp --reflink=auto */
/* cp --reflink[=always] */
/*
 * Hole. cp may have some bits set here,
 * they should not affect remove_file()/copy_file()
 */
/* vi: set sw=4 ts=4: */
/*
 * Mini remove_file implementation for busybox
 *
 * Copyright (C) 2001 Matt Kraai <kraai@alumni.carnegiemellon.edu>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Used from NOFORK applets. Must not allocate anything */
#[no_mangle]
pub unsafe extern "C" fn remove_file(
  mut path: *const libc::c_char,
  mut flags: libc::c_int,
) -> libc::c_int {
  let mut path_stat: stat = stat {
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
  if lstat(path, &mut path_stat) < 0i32 {
    if *bb_errno != 2i32 {
      bb_perror_msg(
        b"can\'t stat \'%s\'\x00" as *const u8 as *const libc::c_char,
        path,
      );
      return -1i32;
    }
    if flags & FILEUTILS_FORCE as libc::c_int == 0 {
      bb_perror_msg(
        b"can\'t remove \'%s\'\x00" as *const u8 as *const libc::c_char,
        path,
      );
      return -1i32;
    }
    return 0i32;
  }
  if path_stat.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint {
    let mut dp: *mut DIR = 0 as *mut DIR;
    let mut d: *mut dirent = 0 as *mut dirent;
    let mut status: libc::c_int = 0i32;
    if flags & FILEUTILS_RECUR as libc::c_int == 0 {
      bb_error_msg(
        b"\'%s\' is a directory\x00" as *const u8 as *const libc::c_char,
        path,
      );
      return -1i32;
    }
    if flags & FILEUTILS_FORCE as libc::c_int == 0 && access(path, 2i32) < 0i32 && isatty(0i32) != 0
      || flags & FILEUTILS_INTERACTIVE as libc::c_int != 0
    {
      fprintf(
        stderr,
        b"%s: descend into directory \'%s\'? \x00" as *const u8 as *const libc::c_char,
        applet_name,
        path,
      );
      if bb_ask_y_confirmation() == 0 {
        return 0i32;
      }
    }
    dp = opendir(path);
    if dp.is_null() {
      return -1i32;
    }
    loop {
      d = readdir(dp);
      if d.is_null() {
        break;
      }
      let mut new_path: *mut libc::c_char = 0 as *mut libc::c_char;
      new_path = concat_subpath_file(path, (*d).d_name.as_mut_ptr());
      if new_path.is_null() {
        continue;
      }
      if remove_file(new_path, flags) < 0i32 {
        status = -1i32
      }
      free(new_path as *mut libc::c_void);
    }
    if closedir(dp) < 0i32 {
      bb_perror_msg(
        b"can\'t close \'%s\'\x00" as *const u8 as *const libc::c_char,
        path,
      );
      return -1i32;
    }
    if flags & FILEUTILS_INTERACTIVE as libc::c_int != 0 {
      fprintf(
        stderr,
        b"%s: remove directory \'%s\'? \x00" as *const u8 as *const libc::c_char,
        applet_name,
        path,
      );
      if bb_ask_y_confirmation() == 0 {
        return status;
      }
    }
    if status == 0i32 && rmdir(path) < 0i32 {
      bb_perror_msg(
        b"can\'t remove \'%s\'\x00" as *const u8 as *const libc::c_char,
        path,
      );
      return -1i32;
    }
    if flags & FILEUTILS_VERBOSE as libc::c_int != 0 {
      printf(
        b"removed directory: \'%s\'\n\x00" as *const u8 as *const libc::c_char,
        path,
      );
    }
    return status;
  }
  /* !ISDIR */
  if flags & FILEUTILS_FORCE as libc::c_int == 0
    && access(path, 2i32) < 0i32
    && !(path_stat.st_mode & 0o170000i32 as libc::c_uint == 0o120000i32 as libc::c_uint)
    && isatty(0i32) != 0
    || flags & FILEUTILS_INTERACTIVE as libc::c_int != 0
  {
    fprintf(
      stderr,
      b"%s: remove \'%s\'? \x00" as *const u8 as *const libc::c_char,
      applet_name,
      path,
    );
    if bb_ask_y_confirmation() == 0 {
      return 0i32;
    }
  }
  if unlink(path) < 0i32 {
    bb_perror_msg(
      b"can\'t remove \'%s\'\x00" as *const u8 as *const libc::c_char,
      path,
    );
    return -1i32;
  }
  if flags & FILEUTILS_VERBOSE as libc::c_int != 0 {
    printf(
      b"removed \'%s\'\n\x00" as *const u8 as *const libc::c_char,
      path,
    );
  }
  return 0i32;
}

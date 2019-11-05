use libc;
extern "C" {
  pub type __dirstream;
  #[no_mangle]
  fn closedir(__dirp: *mut DIR) -> libc::c_int;
  #[no_mangle]
  fn readdir(__dirp: *mut DIR) -> *mut dirent;
  #[no_mangle]
  fn opendir(__name: *const libc::c_char) -> *mut DIR;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn safe_strncpy(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    size: size_t,
  ) -> *mut libc::c_char;
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
use crate::librb::dev_t;
use crate::librb::stat;
use crate::librb::timespec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arena {
  pub st: stat,
  pub dev: dev_t,
  pub devpath: [libc::c_char; 256],
}
pub const DEVNAME_MAX: C2RustUnnamed = 256;
/* vi: set sw=4 ts=4: */
/*
 * Utility routines.
 *
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Find block device /dev/XXX which contains specified file
 * We handle /dev/dir/dir/dir too, at a cost of ~80 more bytes code */
/* Do not reallocate all this stuff on each recursion */
pub type C2RustUnnamed = libc::c_uint;
unsafe extern "C" fn find_block_device_in_dir(mut ap: *mut arena) -> *mut libc::c_char {
  let mut dir: *mut DIR = 0 as *mut DIR;
  let mut entry: *mut dirent = 0 as *mut dirent;
  let mut retpath: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut len: libc::c_int = 0;
  let mut rem: libc::c_int = 0;
  len = strlen((*ap).devpath.as_mut_ptr()) as libc::c_int;
  rem = DEVNAME_MAX as libc::c_int - 2i32 - len;
  if rem <= 0i32 {
    return 0 as *mut libc::c_char;
  }
  dir = opendir((*ap).devpath.as_mut_ptr());
  if dir.is_null() {
    return 0 as *mut libc::c_char;
  }
  let fresh0 = len;
  len = len + 1;
  (*ap).devpath[fresh0 as usize] = '/' as i32 as libc::c_char;
  loop {
    entry = readdir(dir);
    if entry.is_null() {
      break;
    }
    safe_strncpy(
      (*ap).devpath.as_mut_ptr().offset(len as isize),
      (*entry).d_name.as_mut_ptr(),
      rem as size_t,
    );
    /* lstat: do not follow links */
    if lstat((*ap).devpath.as_mut_ptr(), &mut (*ap).st) != 0i32 {
      continue;
    }
    if (*ap).st.st_mode & 0o170000i32 as libc::c_uint == 0o60000i32 as libc::c_uint
      && (*ap).st.st_rdev == (*ap).dev
    {
      retpath = xstrdup((*ap).devpath.as_mut_ptr());
      break;
    } else {
      if !((*ap).st.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint) {
        continue;
      }
      /* Do not recurse for '.' and '..' */
      if (*entry).d_name[0] as libc::c_int == '.' as i32
        && ((*entry).d_name[1] == 0
          || (*entry).d_name[1] as libc::c_int == '.' as i32 && (*entry).d_name[2] == 0)
      {
        continue;
      }
      retpath = find_block_device_in_dir(ap);
      if !retpath.is_null() {
        break;
      }
    }
  }
  closedir(dir);
  return retpath;
}
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
/* NB: without FILEUTILS_RECUR in flags, it will basically "cat"
 * the source, not copy (unless "source" is a directory).
 * This makes "cp /dev/null file" and "install /dev/null file" (!!!)
 * work coreutils-compatibly. */
/*ACTION_REVERSE      = (1 << 4), - unused */
/* more than enough for "/dev/ttyXXX" */
#[no_mangle]
pub unsafe extern "C" fn find_block_device(mut path: *const libc::c_char) -> *mut libc::c_char {
  let mut a: arena = arena {
    st: stat {
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
    },
    dev: 0,
    devpath: [0; 256],
  };
  if stat(path, &mut a.st) != 0i32 {
    return 0 as *mut libc::c_char;
  }
  a.dev = if a.st.st_mode & 0o170000i32 as libc::c_uint == 0o60000i32 as libc::c_uint {
    a.st.st_rdev
  } else {
    a.st.st_dev
  };
  strcpy(
    a.devpath.as_mut_ptr(),
    b"/dev\x00" as *const u8 as *const libc::c_char,
  );
  return find_block_device_in_dir(&mut a);
}

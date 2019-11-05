use libc;
extern "C" {
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_xioctl(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
}
pub type ptrdiff_t = libc::c_long;
pub const KDGKBTYPE: C2RustUnnamed = 19251;
/* make vt active */
pub const VT_WAITACTIVE: C2RustUnnamed_0 = 22023;
pub const VT_ACTIVATE: C2RustUnnamed_0 = 22022;

/*
 * Utility routines.
 *
 * Copyright (C) many different people.  If you wrote this, please
 * acknowledge your work.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* From <linux/kd.h> */
pub type C2RustUnnamed = libc::c_uint;
/* From <linux/vt.h> */
pub type C2RustUnnamed_0 = libc::c_uint;
/* wait for vt active */
/* get keyboard type */
unsafe extern "C" fn open_a_console(mut fnam: *const libc::c_char) -> libc::c_int {
  let mut fd: libc::c_int = 0;
  /* try read-write */
  fd = open(fnam, 0o2i32);
  /* if failed, try read-only */
  if fd < 0i32 && *bb_errno == 13i32 {
    fd = open(fnam, 0i32)
  }
  /* if failed, try write-only */
  if fd < 0i32 && *bb_errno == 13i32 {
    fd = open(fnam, 0o1i32)
  }
  return fd;
}
/*
 * Get an fd for use with kbd/console ioctls.
 * We try several things because opening /dev/console will fail
 * if someone else used X (which does a chown on /dev/console).
 */
#[no_mangle]
pub unsafe extern "C" fn get_console_fd_or_die() -> libc::c_int {
  static mut console_names: [*const libc::c_char; 3] = [
    b"/dev/console\x00" as *const u8 as *const libc::c_char,
    b"/dev/tty0\x00" as *const u8 as *const libc::c_char,
    b"/dev/tty\x00" as *const u8 as *const libc::c_char,
  ];
  let mut fd: libc::c_int = 0;
  fd = 2i32;
  while fd >= 0i32 {
    let mut fd4name: libc::c_int = 0;
    let mut choice_fd: libc::c_int = 0;
    let mut arg: libc::c_char = 0;
    fd4name = open_a_console(console_names[fd as usize]);
    loop {
      choice_fd = if fd4name >= 0i32 { fd4name } else { fd };
      arg = 0i32 as libc::c_char;
      if ioctl(
        choice_fd,
        KDGKBTYPE as libc::c_int as libc::c_ulong,
        &mut arg as *mut libc::c_char,
      ) == 0i32
      {
        return choice_fd;
      }
      if !(fd4name >= 0i32) {
        break;
      }
      close(fd4name);
      fd4name = -1i32
    }
    fd -= 1
  }
  bb_simple_error_msg_and_die(b"can\'t open console\x00" as *const u8 as *const libc::c_char);
}

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
pub unsafe extern "C" fn console_make_active(mut fd: libc::c_int, vt_num: libc::c_int) {
  bb_xioctl(
    fd,
    VT_ACTIVATE as libc::c_int as libc::c_uint,
    vt_num as ptrdiff_t as *mut libc::c_void,
    b"VT_ACTIVATE\x00" as *const u8 as *const libc::c_char,
  );
  bb_xioctl(
    fd,
    VT_WAITACTIVE as libc::c_int as libc::c_uint,
    vt_num as ptrdiff_t as *mut libc::c_void,
    b"VT_WAITACTIVE\x00" as *const u8 as *const libc::c_char,
  );
}

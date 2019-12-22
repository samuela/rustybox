use libc;

/*
 * skip_whitespace implementation for busybox
 *
 * Copyright (C) 2003  Manuel Novoa III  <mjn3@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn skip_whitespace(mut s: *const libc::c_char) -> *mut libc::c_char {
  /* In POSIX/C locale (the only locale we care about: do we REALLY want
   * to allow Unicode whitespace in, say, .conf files? nuts!)
   * isspace is only these chars: "\t\n\v\f\r" and space.
   * "\t\n\v\f\r" happen to have ASCII codes 9,10,11,12,13.
   * Use that.
   */
  while *s as libc::c_int == ' ' as i32
    || (*s as libc::c_int - 9i32) as libc::c_uchar as libc::c_int <= 13i32 - 9i32
  {
    s = s.offset(1)
  }
  return s as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn skip_non_whitespace(mut s: *const libc::c_char) -> *mut libc::c_char {
  while *s as libc::c_int != '\u{0}' as i32
    && *s as libc::c_int != ' ' as i32
    && (*s as libc::c_int - 9i32) as libc::c_uchar as libc::c_int > 13i32 - 9i32
  {
    s = s.offset(1)
  }
  return s as *mut libc::c_char;
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
#[no_mangle]
pub unsafe extern "C" fn skip_dev_pfx(mut tty_name: *const libc::c_char) -> *mut libc::c_char {
  let mut unprefixed: *mut libc::c_char = crate::libbb::compare_string_array::is_prefixed_with(
    tty_name,
    b"/dev/\x00" as *const u8 as *const libc::c_char,
  );
  return if !unprefixed.is_null() {
    unprefixed
  } else {
    tty_name as *mut libc::c_char
  };
}

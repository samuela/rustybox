use libc;
pub type __mode_t = libc::c_uint;
pub type mode_t = __mode_t;
/* vi: set sw=4 ts=4: */
/*
 * mode_string implementation for busybox
 *
 * Copyright (C) 2003  Manuel Novoa III  <mjn3@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* The previous version used "0pcCd?bB-?l?s???".  However, the '0', 'C',
 * and 'B' types don't appear to be available on linux.  So I removed them. */
static mut type_chars: [libc::c_char; 16] = [
  63, 112, 99, 63, 100, 63, 98, 63, 45, 63, 108, 63, 115, 63, 63, 63,
];
/* ********************************* 0123456789abcdef */
static mut mode_chars: [libc::c_char; 7] = [114, 119, 120, 83, 84, 115, 116];
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
#[no_mangle]
pub unsafe extern "C" fn bb_mode_string(mut mode: mode_t) -> *const libc::c_char {
  static mut buf: [libc::c_char; 12] = [0; 12];
  let mut p: *mut libc::c_char = buf.as_mut_ptr();
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  let mut k: libc::c_int = 0;
  let mut m: libc::c_int = 0;
  *p = type_chars[(mode >> 12i32 & 0xfi32 as libc::c_uint) as usize];
  i = 0i32;
  m = 0o400i32;
  loop {
    k = 0i32;
    j = k;
    loop {
      p = p.offset(1);
      *p = '-' as i32 as libc::c_char;
      if mode & m as libc::c_uint != 0 {
        *p = mode_chars[j as usize];
        k = j
      }
      m >>= 1i32;
      j += 1;
      if !(j < 3i32) {
        break;
      }
    }
    i += 1;
    if mode & (0o10000i32 >> i) as libc::c_uint != 0 {
      *p = mode_chars[(3i32 + (k & 2i32) + (i == 3i32) as libc::c_int) as usize]
    }
    if !(i < 3i32) {
      break;
    }
  }
  /* Note: We don't bother with nul termination because bss initialization
   * should have taken care of that for us.  If the user scribbled in buf
   * memory, they deserve whatever happens.  But we'll at least assert. */
  return buf.as_mut_ptr();
}

use libc;
use libc::chdir;
use libc::chmod;
use libc::chown;
use libc::closelog;
use libc::dup2;
use libc::fstat;
use libc::getenv;
use libc::geteuid;
use libc::getopt;
use libc::getpid;
use libc::isatty;
use libc::kill;
use libc::openlog;
use libc::sigaddset;
use libc::sigemptyset;
use libc::sigprocmask;
use libc::sleep;
use libc::sscanf;
use libc::strcasecmp;
use libc::strcpy;
use libc::symlink;
use libc::syscall;
use libc::syslog;
use libc::time;
use libc::access;
use libc::atoi;
use libc::fclose;
use libc::fprintf;
use libc::lstat;
use libc::printf;
use libc::puts;
use libc::rename;
use libc::rmdir;
use libc::sprintf;
use libc::strchr;
use libc::strcmp;
use libc::strrchr;
use libc::strstr;
use libc::system;




/* Usual "this only works for ascii compatible encodings" disclaimer. */
#[no_mangle]
pub unsafe extern "C" fn bb_process_escape_sequence(
  mut ptr: *mut *const libc::c_char,
) -> libc::c_char {
  let mut q: *const libc::c_char = 0 as *const libc::c_char;
  let mut num_digits: libc::c_uint = 0;
  let mut n: libc::c_uint = 0;
  let mut base: libc::c_uint = 0;
  n = 0i32 as libc::c_uint;
  num_digits = n;
  base = 8i32 as libc::c_uint;
  q = *ptr;
  if 1i32 != 0 && *q as libc::c_int == 'x' as i32 {
    q = q.offset(1);
    base = 16i32 as libc::c_uint;
    num_digits = num_digits.wrapping_add(1)
  }
  loop
  /* bash requires leading 0 in octal escapes:
   * \02 works, \2 does not (prints \ and 2).
   * We treat \2 as a valid octal escape sequence. */
  {
    let mut r: libc::c_uint = 0;
    let mut d: libc::c_uint = (*q as libc::c_uchar as libc::c_int - '0' as i32) as libc::c_uint;
    if d >= 10i32 as libc::c_uint {
      d = ((*q as libc::c_int | 0x20i32 as libc::c_char as libc::c_int) as libc::c_uchar
        as libc::c_int
        - 'a' as i32) as libc::c_uint;
      //d += 10;
      /* The above would map 'A'-'F' and 'a'-'f' to 10-15,
       * however, some chars like '@' would map to 9 < base.
       * Do not allow that, map invalid chars to N > base:
       */
      if d as libc::c_int >= 0i32 {
        d = d.wrapping_add(10i32 as libc::c_uint)
      }
    }
    if d >= base {
      if 1i32 != 0 && base == 16i32 as libc::c_uint {
        num_digits = num_digits.wrapping_sub(1);
        if num_digits == 0i32 as libc::c_uint {
          /* \x<bad_char>: return '\',
           * leave ptr pointing to x */
          return '\\' as i32 as libc::c_char;
        }
      }
      break;
    } else {
      r = n.wrapping_mul(base).wrapping_add(d);
      if r > (127i32 * 2i32 + 1i32) as libc::c_uint {
        break;
      }
      n = r;
      q = q.offset(1);
      num_digits = num_digits.wrapping_add(1);
      if !(num_digits < 3i32 as libc::c_uint) {
        break;
      }
    }
  }
  if num_digits == 0i32 as libc::c_uint {
    /* Not octal or hex escape sequence.
     * Is it one-letter one? */
    /* bash builtin "echo -e '\ec'" interprets \e as ESC,
     * but coreutils "/bin/echo -e '\ec'" does not.
     * Manpages tend to support coreutils way.
     * Update: coreutils added support for \e on 28 Oct 2009. */
    static mut charmap: [libc::c_char; 20] = [
      'a' as i32 as libc::c_char,
      'b' as i32 as libc::c_char,
      'e' as i32 as libc::c_char,
      'f' as i32 as libc::c_char,
      'n' as i32 as libc::c_char,
      'r' as i32 as libc::c_char,
      't' as i32 as libc::c_char,
      'v' as i32 as libc::c_char,
      '\\' as i32 as libc::c_char,
      '\u{0}' as i32 as libc::c_char,
      '\u{7}' as i32 as libc::c_char,
      '\u{8}' as i32 as libc::c_char,
      27i32 as libc::c_char,
      '\u{c}' as i32 as libc::c_char,
      '\n' as i32 as libc::c_char,
      '\r' as i32 as libc::c_char,
      '\t' as i32 as libc::c_char,
      '\u{b}' as i32 as libc::c_char,
      '\\' as i32 as libc::c_char,
      '\\' as i32 as libc::c_char,
    ];
    let mut p: *const libc::c_char = charmap.as_ptr();
    loop {
      if *p as libc::c_int == *q as libc::c_int {
        q = q.offset(1);
        break;
      } else {
        p = p.offset(1);
        if !(*p as libc::c_int != '\u{0}' as i32) {
          break;
        }
      }
    }
    /* p points to found escape char or NUL,
     * advance it and find what it translates to.
     * Note that \NUL and unrecognized sequence \z return '\'
     * and leave ptr pointing to NUL or z. */
    n = *p.offset(
      (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
        .wrapping_div(2i32 as libc::c_ulong) as isize,
    ) as libc::c_uint
  }
  *ptr = q;
  return n as libc::c_char;
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
/* bb_copyfd_XX print read/write errors and return -1 if they occur */
/* "short" copy can be detected by return value < size */
/* this helper yells "short read!" if param is not -1 */
#[no_mangle]
pub unsafe extern "C" fn strcpy_and_process_escape_sequences(
  mut dst: *mut libc::c_char,
  mut src: *const libc::c_char,
) -> *mut libc::c_char {
  loop {
    let mut c: libc::c_char = 0;
    let mut c1: libc::c_char = 0;
    let fresh0 = src;
    src = src.offset(1);
    c1 = *fresh0;
    c = c1;
    if c1 as libc::c_int == '\\' as i32 {
      c1 = bb_process_escape_sequence(&mut src)
    }
    *dst = c1;
    if c as libc::c_int == '\u{0}' as i32 {
      return dst;
    }
    dst = dst.offset(1)
  }
}

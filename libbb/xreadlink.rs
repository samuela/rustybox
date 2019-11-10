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



use libc::free;
extern "C" {

  #[no_mangle]
  fn readlink(__path: *const libc::c_char, __buf: *mut libc::c_char, __len: size_t) -> ssize_t;
  #[no_mangle]
  fn realpath(__name: *const libc::c_char, __resolved: *mut libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn strerror(_: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_get_last_path_component_strip(path: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn concat_path_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn xrealloc_getcwd_or_warn(cwd: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);

}

use crate::librb::size_t;
use libc::ssize_t;
pub const GROWBY: C2RustUnnamed = 80;
pub type C2RustUnnamed = libc::c_uint;

/*
 * xreadlink.c - safe implementation of readlink.
 * Returns a NULL on failure.
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* Some systems (eg Hurd) do not have MAXSYMLINKS definition,
 * set it to some reasonable value if it isn't defined */
/*
 * NOTE: This function returns a malloced char* that you will have to free
 * yourself.
 */
#[no_mangle]
pub unsafe extern "C" fn xmalloc_readlink(mut path: *const libc::c_char) -> *mut libc::c_char {
  let mut buf: *mut libc::c_char = 0 as *mut libc::c_char; /* how large we will grow strings by */
  let mut bufsize: libc::c_int = 0i32;
  let mut readsize: libc::c_int = 0i32;
  loop {
    bufsize += GROWBY as libc::c_int;
    buf = xrealloc(buf as *mut libc::c_void, bufsize as size_t) as *mut libc::c_char;
    readsize = readlink(path, buf, bufsize as size_t) as libc::c_int;
    if readsize == -1i32 {
      free(buf as *mut libc::c_void);
      return 0 as *mut libc::c_char;
    }
    if !(bufsize < readsize + 1i32) {
      break;
    }
  }
  *buf.offset(readsize as isize) = '\u{0}' as i32 as libc::c_char;
  return buf;
}
/* !RETURNS_MALLOC: it's a realloc-like function */
/*
 * This routine is not the same as realpath(), which
 * canonicalizes the given path completely. This routine only
 * follows trailing symlinks until a real file is reached and
 * returns its name. If the path ends in a dangling link or if
 * the target doesn't exist, the path is returned in any case.
 * Intermediate symlinks in the path are not expanded -- only
 * those at the tail.
 * A malloced char* is returned, which must be freed by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmalloc_follow_symlinks(
  mut path: *const libc::c_char,
) -> *mut libc::c_char {
  let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut lpc: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut linkpath: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut bufsize: libc::c_int = 0;
  let mut looping: libc::c_int = 20i32 + 1i32;
  buf = xstrdup(path);
  'c_6598: loop {
    bufsize = strlen(buf).wrapping_add(1i32 as libc::c_ulong) as libc::c_int;
    loop {
      linkpath = xmalloc_readlink(buf);
      if linkpath.is_null() {
        /* not a symlink, or doesn't exist */
        if *bb_errno == 22i32 || *bb_errno == 2i32 {
          return buf;
        }
        break;
      } else {
        looping -= 1;
        if looping == 0 {
          free(linkpath as *mut libc::c_void);
          break;
        } else if *linkpath as libc::c_int != '/' as i32 {
          bufsize =
            (bufsize as libc::c_ulong).wrapping_add(strlen(linkpath)) as libc::c_int as libc::c_int;
          buf = xrealloc(buf as *mut libc::c_void, bufsize as size_t) as *mut libc::c_char;
          lpc = bb_get_last_path_component_strip(buf);
          strcpy(lpc, linkpath);
          free(linkpath as *mut libc::c_void);
        } else {
          free(buf as *mut libc::c_void);
          buf = linkpath;
          continue 'c_6598;
        }
      }
    }
    free(buf as *mut libc::c_void);
    return 0 as *mut libc::c_char;
  }
}
#[no_mangle]
pub unsafe extern "C" fn xmalloc_readlink_or_warn(
  mut path: *const libc::c_char,
) -> *mut libc::c_char {
  let mut buf: *mut libc::c_char = xmalloc_readlink(path);
  if buf.is_null() {
    /* EINVAL => "file: Invalid argument" => puzzled user */
    let mut errmsg: *const libc::c_char = b"not a symlink\x00" as *const u8 as *const libc::c_char;
    let mut err: libc::c_int = *bb_errno;
    if err != 22i32 {
      errmsg = strerror(err)
    }
    bb_error_msg(
      b"%s: cannot read link: %s\x00" as *const u8 as *const libc::c_char,
      path,
      errmsg,
    );
  }
  return buf;
}
#[no_mangle]
pub unsafe extern "C" fn xmalloc_realpath(mut path: *const libc::c_char) -> *mut libc::c_char {
  /* NB: uclibc also defines __GLIBC__
   * Therefore the test "if glibc, or uclibc >= 0.9.31" looks a bit weird:
   */
  /* glibc provides a non-standard extension */
  /* new: POSIX.1-2008 specifies this behavior as well */
  return realpath(path, 0 as *mut libc::c_char);
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
/* xxxx_strip version can modify its parameter:
 * "/"        -> "/"
 * "abc"      -> "abc"
 * "abc/def"  -> "def"
 * "abc/def/" -> "def" !!
 */
/* "abc/def/" -> "" and it never modifies 'path' */
/* Simpler version: does not special case "/" string */
/* NB: can violate const-ness (similarly to strchr) */
#[no_mangle]
pub unsafe extern "C" fn xmalloc_realpath_coreutils(
  mut path: *const libc::c_char,
) -> *mut libc::c_char {
  let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
  *bb_errno = 0i32;
  buf = xmalloc_realpath(path);
  /*
   * There is one case when "readlink -f" and
   * "realpath" from coreutils succeed,
   * even though file does not exist, such as:
   *     /tmp/file_does_not_exist
   * (the directory must exist).
   */
  if buf.is_null() && *bb_errno == 2i32 {
    let mut last_slash: *mut libc::c_char = strrchr(path, '/' as i32);
    if !last_slash.is_null() {
      let fresh0 = last_slash;
      last_slash = last_slash.offset(1);
      *fresh0 = '\u{0}' as i32 as libc::c_char;
      buf = xmalloc_realpath(path);
      if !buf.is_null() {
        let mut len: libc::c_uint = strlen(buf) as libc::c_uint;
        buf = xrealloc(
          buf as *mut libc::c_void,
          (len as libc::c_ulong)
            .wrapping_add(strlen(last_slash))
            .wrapping_add(2i32 as libc::c_ulong),
        ) as *mut libc::c_char;
        let fresh1 = len;
        len = len.wrapping_add(1);
        *buf.offset(fresh1 as isize) = '/' as i32 as libc::c_char;
        strcpy(buf.offset(len as isize), last_slash);
      }
    } else {
      let mut target: *mut libc::c_char = xmalloc_readlink(path);
      if !target.is_null() {
        let mut cwd: *mut libc::c_char = 0 as *mut libc::c_char;
        if *target.offset(0) as libc::c_int == '/' as i32 {
          /*
           * $ ln -s /bin/qwe symlink  # note: /bin is a link to /usr/bin
           * $ readlink -f symlink
           * /usr/bin/qwe/target_does_not_exist
           * $ realpath symlink
           * /usr/bin/qwe/target_does_not_exist
           */
          buf = xmalloc_realpath_coreutils(target);
          free(target as *mut libc::c_void);
          return buf;
        }
        /*
         * $ ln -s target_does_not_exist symlink
         * $ readlink -f symlink
         * /CURDIR/target_does_not_exist
         * $ realpath symlink
         * /CURDIR/target_does_not_exist
         */
        cwd = xrealloc_getcwd_or_warn(0 as *mut libc::c_char);
        buf = concat_path_file(cwd, target);
        free(cwd as *mut libc::c_void);
        free(target as *mut libc::c_void);
        return buf;
      }
    }
  }
  return buf;
}

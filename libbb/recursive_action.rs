use libc;
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
  fn opendir(__name: *const libc::c_char) -> *mut DIR;
  #[no_mangle]
  fn readdir(__dirp: *mut DIR) -> *mut dirent;
  #[no_mangle]
  fn closedir(__dirp: *mut DIR) -> libc::c_int;


  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn concat_subpath_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;
}





use libc::dirent;
use libc::DIR;
use libc::stat;

pub type C2RustUnnamed = libc::c_uint;
pub const ACTION_DANGLING_OK: C2RustUnnamed = 64;
pub const ACTION_QUIET: C2RustUnnamed = 32;
pub const ACTION_DEPTHFIRST: C2RustUnnamed = 8;
pub const ACTION_FOLLOWLINKS_L0: C2RustUnnamed = 4;
pub const ACTION_FOLLOWLINKS: C2RustUnnamed = 2;
pub const ACTION_RECURSE: C2RustUnnamed = 1;

/*
 * Utility routines.
 *
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/*
 * Walk down all the directories under the specified
 * location, and do something (something specified
 * by the fileAction and dirAction function pointers).
 *
 * Unfortunately, while nftw(3) could replace this and reduce
 * code size a bit, nftw() wasn't supported before GNU libc 2.1,
 * and so isn't sufficiently portable to take over since glibc2.1
 * is so stinking huge.
 */
unsafe extern "C" fn true_action(
  mut _fileName: *const libc::c_char,
  mut _statbuf: *mut stat,
  mut _userData: *mut libc::c_void,
  mut _depth: libc::c_int,
) -> libc::c_int {
  return 1i32;
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
/* fileName is (l)stat'ed (depending on ACTION_FOLLOWLINKS[_L0]).
 *
 * If it is a file: fileAction in run on it, its return value is returned.
 *
 * In case we are in a recursive invocation (see below):
 * normally, fileAction should return 1 (TRUE) to indicate that
 * everything is okay and processing should continue.
 * fileAction return value of 0 (FALSE) on any file in directory will make
 * recursive_action() also return 0, but it doesn't stop directory traversal
 * (fileAction/dirAction will be called on each file).
 *
 * [TODO: maybe introduce -1 to mean "stop traversal NOW and return"]
 *
 * If it is a directory:
 *
 * If !ACTION_RECURSE, dirAction is called and its
 * return value is returned from recursive_action(). No recursion.
 *
 * If ACTION_RECURSE, directory is opened, and recursive_action() is called
 * on each file/subdirectory.
 * If any one of these calls returns 0, current recursive_action() returns 0.
 *
 * If !ACTION_DEPTHFIRST, dirAction is called before recurse.
 * Return value of 0 (FALSE) is an error: prevents recursion,
 * the warning is printed (unless ACTION_QUIET) and recursive_action() returns 0.
 * Return value of 2 (SKIP) prevents recursion, instead recursive_action()
 * returns 1 (TRUE, no error).
 *
 * If ACTION_DEPTHFIRST, dirAction is called after recurse.
 * If it returns 0, the warning is printed and recursive_action() returns 0.
 *
 * ACTION_FOLLOWLINKS mainly controls handling of links to dirs.
 * 0: lstat(statbuf). Calls fileAction on link name even if points to dir.
 * 1: stat(statbuf). Calls dirAction and optionally recurse on link to dir.
 */
#[no_mangle]
pub unsafe extern "C" fn recursive_action(
  mut fileName: *const libc::c_char,
  mut flags: libc::c_uint,
  mut fileAction: Option<
    unsafe extern "C" fn(
      _: *const libc::c_char,
      _: *mut stat,
      _: *mut libc::c_void,
      _: libc::c_int,
    ) -> libc::c_int,
  >,
  mut dirAction: Option<
    unsafe extern "C" fn(
      _: *const libc::c_char,
      _: *mut stat,
      _: *mut libc::c_void,
      _: libc::c_int,
    ) -> libc::c_int,
  >,
  mut userData: *mut libc::c_void,
  mut depth: libc::c_uint,
) -> libc::c_int {
  let mut current_block: u64;
  let mut statbuf: stat = std::mem::zeroed();
  let mut follow: libc::c_uint = 0;
  let mut status: libc::c_int = 0;
  let mut dir: *mut DIR = 0 as *mut DIR;
  let mut next: *mut dirent = 0 as *mut dirent;
  if fileAction.is_none() {
    fileAction = Some(
      true_action
        as unsafe extern "C" fn(
          _: *const libc::c_char,
          _: *mut stat,
          _: *mut libc::c_void,
          _: libc::c_int,
        ) -> libc::c_int,
    )
  }
  if dirAction.is_none() {
    dirAction = Some(
      true_action
        as unsafe extern "C" fn(
          _: *const libc::c_char,
          _: *mut stat,
          _: *mut libc::c_void,
          _: libc::c_int,
        ) -> libc::c_int,
    )
  }
  follow = ACTION_FOLLOWLINKS as libc::c_int as libc::c_uint;
  if depth == 0i32 as libc::c_uint {
    follow =
      (ACTION_FOLLOWLINKS as libc::c_int | ACTION_FOLLOWLINKS_L0 as libc::c_int) as libc::c_uint
  }
  follow &= flags;
  status = if follow != 0 {
    Some(stat as unsafe extern "C" fn(_: *const libc::c_char, _: *mut stat) -> libc::c_int)
  } else {
    Some(lstat as unsafe extern "C" fn(_: *const libc::c_char, _: *mut stat) -> libc::c_int)
  }
  .expect("non-null function pointer")(fileName, &mut statbuf);
  if status < 0i32 {
    if flags & ACTION_DANGLING_OK as libc::c_int as libc::c_uint != 0
      && *bb_errno == 2i32
      && lstat(fileName, &mut statbuf) == 0i32
    {
      /* Dangling link */
      return fileAction.expect("non-null function pointer")(
        fileName,
        &mut statbuf,
        userData,
        depth as libc::c_int,
      );
    }
  } else {
    /* If S_ISLNK(m), then we know that !S_ISDIR(m).
     * Then we can skip checking first part: if it is true, then
     * (!dir) is also true! */
    if !(statbuf.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint) {
      return fileAction.expect("non-null function pointer")(
        fileName,
        &mut statbuf,
        userData,
        depth as libc::c_int,
      );
    }
    /* It's a directory (or a link to one, and followLinks is set) */
    if flags & ACTION_RECURSE as libc::c_int as libc::c_uint == 0 {
      return dirAction.expect("non-null function pointer")(
        fileName,
        &mut statbuf,
        userData,
        depth as libc::c_int,
      );
    }
    if flags & ACTION_DEPTHFIRST as libc::c_int as libc::c_uint == 0 {
      status = dirAction.expect("non-null function pointer")(
        fileName,
        &mut statbuf,
        userData,
        depth as libc::c_int,
      );
      if status == 0i32 {
        current_block = 8207725251379991669;
      } else {
        if status == 2i32 {
          return 1i32;
        }
        current_block = 2719512138335094285;
      }
    } else {
      current_block = 2719512138335094285;
    }
    match current_block {
      8207725251379991669 => {}
      _ => {
        dir = opendir(fileName);
        if !dir.is_null() {
          status = 1i32;
          loop {
            next = readdir(dir);
            if next.is_null() {
              break;
            }
            let mut nextFile: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut s: libc::c_int = 0;
            nextFile = concat_subpath_file(fileName, (*next).d_name.as_mut_ptr());
            if nextFile.is_null() {
              continue;
            }
            //#define RECURSE_RESULT_ABORT -1
            //		if (s == RECURSE_RESULT_ABORT) {
            //			closedir(dir);
            //			return s;
            //		}
            /* process every file (NB: ACTION_RECURSE is set in flags) */
            s = recursive_action(
              nextFile,
              flags,
              fileAction,
              dirAction,
              userData,
              depth.wrapping_add(1i32 as libc::c_uint),
            );
            if s == 0i32 {
              status = 0i32
            }
            free(nextFile as *mut libc::c_void);
          }
          closedir(dir);
          if flags & ACTION_DEPTHFIRST as libc::c_int as libc::c_uint != 0 {
            if dirAction.expect("non-null function pointer")(
              fileName,
              &mut statbuf,
              userData,
              depth as libc::c_int,
            ) == 0
            {
              current_block = 8207725251379991669;
            } else {
              current_block = 7226443171521532240;
            }
          } else {
            current_block = 7226443171521532240;
          }
          match current_block {
            8207725251379991669 => {}
            _ => return status,
          }
        }
      }
    }
  }
  /* findutils-4.1.20 reports this */
  /* (i.e. it doesn't silently return with exit code 1) */
  /* To trigger: "find -exec rm -rf {} \;" */
  if flags & ACTION_QUIET as libc::c_int as libc::c_uint == 0 {
    bb_simple_perror_msg(fileName);
  }
  return 0i32;
}

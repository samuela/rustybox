use crate::librb::size_t;
use crate::libbb::ptr_to_globals::bb_errno;
use libc;
use libc::off64_t;
use libc::off_t;
use libc::ssize_t;
extern "C" {
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn xfunc_die() -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn sendfile(
    __out_fd: libc::c_int,
    __in_fd: libc::c_int,
    __offset: *mut off64_t,
    __count: size_t,
  ) -> ssize_t;
}

pub const buffer_size: C2RustUnnamed = 4096;
pub type C2RustUnnamed = libc::c_uint;

/* Used by NOFORK applets (e.g. cat) - must not use xmalloc.
 * size < 0 means "ignore write errors", used by tar --to-command
 * size = 0 means "copy till EOF"
 */
unsafe extern "C" fn bb_full_fd_action(
  mut src_fd: libc::c_int,
  mut dst_fd: libc::c_int,
  mut size: off_t,
) -> off_t {
  let mut status: libc::c_int = -1i32;
  let mut total: off_t = 0i32 as off_t;
  let mut continue_on_write_error: bool = 0i32 != 0;
  let mut sendfile_sz: ssize_t = 0;
  let mut buffer: [libc::c_char; 4096] = [0; 4096];
  if size < 0 {
    size = -size;
    continue_on_write_error = 1i32 != 0
  }
  if !(src_fd < 0i32) {
    sendfile_sz = if 1i32 == 0 {
      0i32
    } else {
      (16i32 * 1024i32) * 1024i32
    } as ssize_t;
    if size == 0 {
      size = (16i32 * 1024i32 * 1024i32) as off_t;
      status = 1i32
      /* copy until eof */
    }
    let mut current_block_18: u64;
    loop {
      let mut rd: ssize_t = 0;
      if sendfile_sz != 0 {
        /* dst_fd == -1 is a fake, else... */
        if dst_fd >= 0i32 {
          rd = sendfile(
            dst_fd,
            src_fd,
            0 as *mut off64_t,
            if size > sendfile_sz as i64 {
              sendfile_sz as size_t
            } else {
              size as size_t
            },
          );
          if rd >= 0 {
            current_block_18 = 1688160156426625504;
          } else {
            current_block_18 = 15904375183555213903;
          }
        } else {
          current_block_18 = 15904375183555213903;
        }
        match current_block_18 {
          1688160156426625504 => {}
          _ => {
            sendfile_sz = 0i32 as ssize_t;
            current_block_18 = 4808432441040389987;
          }
        }
      /* do not try sendfile anymore */
      } else {
        current_block_18 = 4808432441040389987;
      }
      match current_block_18 {
        4808432441040389987 => {
          rd = safe_read(
            src_fd,
            buffer.as_mut_ptr() as *mut libc::c_void,
            if size > buffer_size as libc::c_int as libc::c_long {
              buffer_size as libc::c_int as libc::c_long
            } else {
              size
            } as size_t,
          );
          if rd < 0 {
            bb_simple_perror_msg(b"read error\x00" as *const u8 as *const libc::c_char);
            break;
          }
        }
        _ => {}
      }
      if rd == 0 {
        /* eof - all done */
        status = 0i32;
        break;
      } else {
        /* dst_fd == -1 is a fake, else... */
        if dst_fd >= 0i32 && sendfile_sz == 0 {
          let mut wr: ssize_t = full_write(
            dst_fd,
            buffer.as_mut_ptr() as *const libc::c_void,
            rd as size_t,
          );
          if wr < rd {
            if !continue_on_write_error {
              bb_simple_perror_msg(b"write error\x00" as *const u8 as *const libc::c_char);
              break;
            } else {
              dst_fd = -1i32
            }
          }
        }
        total += rd as i64;
        if !(status < 0i32) {
          continue;
        }
        /* if we aren't copying till EOF... */
        size -= rd as i64;
        if !(size == 0) {
          continue;
        }
        /* 'size' bytes copied - all done */
        status = 0i32;
        break;
      }
    }
  }
  /* some environments don't have munmap(), hide it in #if */
  return if status != 0 {
    -1i32 as libc::c_long
  } else {
    total
  };
}
#[no_mangle]
pub unsafe extern "C" fn bb_copyfd_size(
  mut fd1: libc::c_int,
  mut fd2: libc::c_int,
  mut size: off_t,
) -> off_t {
  if size != 0 {
    return bb_full_fd_action(fd1, fd2, size);
  }
  return 0i32 as off_t;
}
#[no_mangle]
pub unsafe extern "C" fn bb_copyfd_exact_size(
  mut fd1: libc::c_int,
  mut fd2: libc::c_int,
  mut size: off_t,
) {
  let mut sz: off_t = bb_copyfd_size(fd1, fd2, size);
  if sz == (if size >= 0 { size } else { -size }) {
    return;
  }
  if sz != -1i32 as libc::c_long {
    bb_simple_error_msg_and_die(b"short read\x00" as *const u8 as *const libc::c_char);
  }
  /* if sz == -1, bb_copyfd_XX already complained */
  xfunc_die();
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
#[no_mangle]
pub unsafe extern "C" fn bb_copyfd_eof(mut fd1: libc::c_int, mut fd2: libc::c_int) -> off_t {
  return bb_full_fd_action(fd1, fd2, 0i32 as off_t);
}

use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn vfork() -> libc::c_int;
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn _exit(_: libc::c_int) -> !;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn wait(__stat_loc: *mut libc::c_int) -> __pid_t;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xmemdup(s: *const libc::c_void, n: libc::c_int) -> *mut libc::c_void;
  #[no_mangle]
  fn is_suffixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  static mut bb_got_signal: smallint;
  #[no_mangle]
  fn xlseek(fd: libc::c_int, offset: off_t, whence: libc::c_int) -> off_t;
  #[no_mangle]
  fn xpipe(filedes: *mut libc::c_int);
  #[no_mangle]
  fn full_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn xread(fd: libc::c_int, buf: *mut libc::c_void, count: size_t);
  #[no_mangle]
  fn xmalloc_read_with_initial_buf(
    fd: libc::c_int,
    maxsz_p: *mut size_t,
    buf: *mut libc::c_char,
    total: size_t,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn xfork() -> pid_t;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn wait_any_nohang(wstat: *mut libc::c_int) -> pid_t;
  #[no_mangle]
  fn xfunc_die() -> !;
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn unpack_gz_stream(xstate: *mut transformer_state_t) -> libc::c_longlong;
  #[no_mangle]
  fn unpack_bz2_stream(xstate: *mut transformer_state_t) -> libc::c_longlong;
  #[no_mangle]
  fn unpack_lzma_stream(xstate: *mut transformer_state_t) -> libc::c_longlong;
  #[no_mangle]
  fn unpack_xz_stream(xstate: *mut transformer_state_t) -> libc::c_longlong;
}

use crate::librb::__pid_t;




pub type bb__aliased_u32 = u32;
use crate::librb::fd_pair;
use crate::librb::off_t;
use crate::librb::pid_t;
use crate::librb::size_t;
use crate::librb::smallint;
use crate::librb::ssize_t;
use libc::time_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct transformer_state_t {
  pub signature_skipped: smallint,
  pub xformer: Option<unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong>,
  pub src_fd: libc::c_int,
  pub dst_fd: libc::c_int,
  pub mem_output_size_max: size_t,
  pub mem_output_size: size_t,
  pub mem_output_buf: *mut libc::c_char,
  pub bytes_out: off_t,
  pub bytes_in: off_t,
  pub crc32: u32,
  pub mtime: time_t,
  pub magic: C2RustUnnamed,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub b: [u8; 8],
  pub b16: [u16; 4],
  pub b32: [u32; 2],
}
pub const XZ_MAGIC2: C2RustUnnamed_0 = 5920890;
pub const XZ_MAGIC1: C2RustUnnamed_0 = 14333;
pub const BZIP2_MAGIC: C2RustUnnamed_0 = 23106;
pub const COMPRESS_MAGIC: C2RustUnnamed_0 = 40223;
pub const GZIP_MAGIC: C2RustUnnamed_0 = 35615;

pub type C2RustUnnamed_0 = libc::c_uint;
pub const XZ_MAGIC2a: C2RustUnnamed_0 = 90;
pub const XZ_MAGIC1a: C2RustUnnamed_0 = 1484404733;

/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn init_transformer_state(mut xstate: *mut transformer_state_t) {
  memset(
    xstate as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<transformer_state_t>() as libc::c_ulong,
  );
}
#[no_mangle]
pub unsafe extern "C" fn check_signature16(
  mut xstate: *mut transformer_state_t,
  mut magic16: libc::c_uint,
) -> libc::c_int {
  if (*xstate).signature_skipped == 0 {
    let mut magic2: u16 = 0;
    if full_read(
      (*xstate).src_fd,
      &mut magic2 as *mut u16 as *mut libc::c_void,
      2i32 as size_t,
    ) != 2i32 as libc::c_long
      || magic2 as libc::c_uint != magic16
    {
      bb_simple_error_msg(b"invalid magic\x00" as *const u8 as *const libc::c_char);
      return -1i32;
    }
    (*xstate).signature_skipped = 2i32 as smallint
  }
  return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn transformer_write(
  mut xstate: *mut transformer_state_t,
  mut buf: *const libc::c_void,
  mut bufsize: size_t,
) -> ssize_t {
  let mut nwrote: ssize_t = 0;
  if (*xstate).mem_output_size_max != 0i32 as libc::c_ulong {
    let mut pos: size_t = (*xstate).mem_output_size;
    let mut size: size_t = 0;
    (*xstate).mem_output_size =
      ((*xstate).mem_output_size as libc::c_ulong).wrapping_add(bufsize) as size_t as size_t;
    size = (*xstate).mem_output_size;
    if size > (*xstate).mem_output_size_max {
      free((*xstate).mem_output_buf as *mut libc::c_void);
      (*xstate).mem_output_buf = 0 as *mut libc::c_char;
      bb_perror_msg(
        b"buffer %u too small\x00" as *const u8 as *const libc::c_char,
        (*xstate).mem_output_size_max as libc::c_uint,
      );
      nwrote = -1i32 as ssize_t
    } else {
      (*xstate).mem_output_buf = xrealloc(
        (*xstate).mem_output_buf as *mut libc::c_void,
        size.wrapping_add(1i32 as libc::c_ulong),
      ) as *mut libc::c_char;
      memcpy(
        (*xstate).mem_output_buf.offset(pos as isize) as *mut libc::c_void,
        buf,
        bufsize,
      );
      *(*xstate).mem_output_buf.offset(size as isize) = '\u{0}' as i32 as libc::c_char;
      nwrote = bufsize as ssize_t
    }
  } else {
    nwrote = full_write((*xstate).dst_fd, buf, bufsize);
    if nwrote != bufsize as ssize_t {
      bb_simple_perror_msg(b"write\x00" as *const u8 as *const libc::c_char);
      nwrote = -1i32 as ssize_t
    }
  }
  return nwrote;
}
#[no_mangle]
pub unsafe extern "C" fn xtransformer_write(
  mut xstate: *mut transformer_state_t,
  mut buf: *const libc::c_void,
  mut bufsize: size_t,
) -> ssize_t {
  let mut nwrote: ssize_t = transformer_write(xstate, buf, bufsize);
  if nwrote != bufsize as ssize_t {
    xfunc_die();
  }
  return nwrote;
}
#[no_mangle]
pub unsafe extern "C" fn check_errors_in_children(mut signo: libc::c_int) {
  let mut current_block: u64;
  let mut status: libc::c_int = 0;
  if signo == 0 {
    /* block waiting for any child */
    if wait(&mut status) < 0i32 {
      /* probably there are no children */
      //FIXME: check EINTR?
      return;
    }
    current_block = 12949536109325556800;
  } else {
    /* Wait for any child without blocking */
    current_block = 7095457783677275021;
  }
  loop {
    match current_block {
      7095457783677275021 =>
      /* this child exited with 0 */
      {
        if wait_any_nohang(&mut status) < 0i32 {
          //FIXME: check EINTR?
          /* wait failed?! I'm confused... */
          return;
        }
        current_block = 12949536109325556800;
      }
      _ =>
      /*if (WIFEXITED(status) && WEXITSTATUS(status) == 0)*/
      /* On Linux, the above can be checked simply as: */
      {
        if status == 0i32 {
          current_block = 7095457783677275021;
          continue;
        }
        /* Cannot happen:
        if (!WIFSIGNALED(status) && !WIFEXITED(status)) ???;
         */
        bb_got_signal = 1i32 as smallint;
        current_block = 7095457783677275021;
      }
    }
  }
}
/* transformer(), more than meets the eye */
#[no_mangle]
pub unsafe extern "C" fn fork_transformer(
  mut fd: libc::c_int,
  mut signature_skipped: libc::c_int,
  mut transformer: Option<unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong>,
) {
  let mut fd_pipe: fd_pair = fd_pair { rd: 0, wr: 0 };
  let mut pid: libc::c_int = 0;
  xpipe(&mut fd_pipe.rd);
  pid = if 1i32 != 0 {
    xfork()
  } else {
    ({
      let mut bb__xvfork_pid: pid_t = vfork();
      if bb__xvfork_pid < 0i32 {
        bb_simple_perror_msg_and_die(b"vfork\x00" as *const u8 as *const libc::c_char);
      }
      bb__xvfork_pid
    })
  };
  if pid == 0i32 {
    /* Child */
    close(fd_pipe.rd); /* we don't want to read from the parent */
    /* notreached */
    let mut r: libc::c_longlong = 0;
    let mut xstate: transformer_state_t = transformer_state_t {
      signature_skipped: 0,
      xformer: None,
      src_fd: 0,
      dst_fd: 0,
      mem_output_size_max: 0,
      mem_output_size: 0,
      mem_output_buf: 0 as *mut libc::c_char,
      bytes_out: 0,
      bytes_in: 0,
      crc32: 0,
      mtime: 0,
      magic: C2RustUnnamed { b: [0; 8] },
    };
    init_transformer_state(&mut xstate);
    xstate.signature_skipped = signature_skipped as smallint;
    xstate.src_fd = fd;
    xstate.dst_fd = fd_pipe.wr;
    r = transformer.expect("non-null function pointer")(&mut xstate);
    _exit((r < 0i32 as libc::c_longlong) as libc::c_int);
  }
  // FIXME: error check?
  /* must be _exit! bug was actually seen here */
  /* parent process */
  close(fd_pipe.wr); /* don't want to write to the child */
  xmove_fd(fd_pipe.rd, fd);
}
/* Used by e.g. rpm which gives us a fd without filename,
 * thus we can't guess the format from filename's extension.
 */
unsafe extern "C" fn setup_transformer_on_fd(
  mut fd: libc::c_int,
  mut fail_if_not_compressed: libc::c_int,
) -> *mut transformer_state_t {
  let mut current_block: u64;
  let mut xstate: *mut transformer_state_t = 0 as *mut transformer_state_t;
  xstate = xzalloc(::std::mem::size_of::<transformer_state_t>() as libc::c_ulong)
    as *mut transformer_state_t;
  (*xstate).src_fd = fd;
  /* .gz and .bz2 both have 2-byte signature, and their
   * unpack_XXX_stream wants this header skipped. */
  (*xstate).signature_skipped = 2i32 as smallint;
  xread(
    fd,
    (*xstate).magic.b16.as_mut_ptr() as *mut libc::c_void,
    2i32 as size_t,
  );
  if 1i32 != 0 && (*xstate).magic.b16[0] as libc::c_int == GZIP_MAGIC as libc::c_int {
    (*xstate).xformer = Some(
      unpack_gz_stream as unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong,
    )
  } else if false && (*xstate).magic.b16[0] as libc::c_int == COMPRESS_MAGIC as libc::c_int {
    // Branch conditional was originally
    //    if (ENABLE_FEATURE_SEAMLESS_Z && xstate->magic.b16[0] == COMPRESS_MAGIC

    // (*xstate).xformer =
    //   Some(unpack_Z_stream as unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong)
  } else if 1i32 != 0 && (*xstate).magic.b16[0] as libc::c_int == BZIP2_MAGIC as libc::c_int {
    (*xstate).xformer = Some(
      unpack_bz2_stream as unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong,
    )
  } else {
    if 1i32 != 0 && (*xstate).magic.b16[0] as libc::c_int == XZ_MAGIC1 as libc::c_int {
      let mut v32: u32 = 0;
      (*xstate).signature_skipped = 6i32 as smallint;
      xread(
        fd,
        &mut *(*xstate).magic.b16.as_mut_ptr().offset(1) as *mut u16 as *mut libc::c_void,
        4i32 as size_t,
      );
      v32 = *(&mut *(*xstate).magic.b16.as_mut_ptr().offset(1) as *mut u16
        as *mut bb__aliased_u32);
      if v32 == XZ_MAGIC2 as libc::c_int as libc::c_uint {
        (*xstate).xformer = Some(
          unpack_xz_stream as unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong,
        );
        current_block = 4138974559695082291;
      } else {
        current_block = 15904375183555213903;
      }
    } else {
      current_block = 15904375183555213903;
    }
    match current_block {
      4138974559695082291 => {}
      _ => {
        /* No known magic seen */
        if fail_if_not_compressed != 0 {
          bb_simple_error_msg_and_die(
            b"no gzip/bzip2/xz magic\x00" as *const u8 as *const libc::c_char,
          );
        }
      }
    }
  }
  /* Some callers expect this function to "consume" fd
   * even if data is not compressed. In this case,
   * we return a state with trivial transformer.
   */
  //	USE_FOR_MMU(xstate->xformer = copy_stream;)
  //	USE_FOR_NOMMU(xstate->xformer_prog = "cat";)
  return xstate;
}
unsafe extern "C" fn fork_transformer_and_free(mut xstate: *mut transformer_state_t) {
  fork_transformer((*xstate).src_fd, 1i32, (*xstate).xformer);
  free(xstate as *mut libc::c_void);
}
/* Used by e.g. rpm which gives us a fd without filename,
 * thus we can't guess the format from filename's extension.
 */
#[no_mangle]
pub unsafe extern "C" fn setup_unzip_on_fd(
  mut fd: libc::c_int,
  mut fail_if_not_compressed: libc::c_int,
) -> libc::c_int {
  let mut xstate: *mut transformer_state_t = setup_transformer_on_fd(fd, fail_if_not_compressed);
  if (*xstate).xformer.is_none() {
    free(xstate as *mut libc::c_void);
    return 1i32;
  }
  fork_transformer_and_free(xstate);
  return 0i32;
}
/* lzma has no signature, need a little helper. NB: exist only for ENABLE_FEATURE_SEAMLESS_LZMA=y */
/* ...and custom version for LZMA */
#[no_mangle]
pub unsafe extern "C" fn setup_lzma_on_fd(mut fd: libc::c_int) {
  let mut xstate: *mut transformer_state_t =
    xzalloc(::std::mem::size_of::<transformer_state_t>() as libc::c_ulong)
      as *mut transformer_state_t;
  (*xstate).src_fd = fd;
  (*xstate).xformer = Some(
    unpack_lzma_stream as unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong,
  );
  fork_transformer_and_free(xstate);
}
unsafe extern "C" fn open_transformer(
  mut fname: *const libc::c_char,
  mut fail_if_not_compressed: libc::c_int,
) -> *mut transformer_state_t {
  let mut xstate: *mut transformer_state_t = 0 as *mut transformer_state_t;
  let mut fd: libc::c_int = 0;
  fd = open(fname, 0i32);
  if fd < 0i32 {
    return 0 as *mut transformer_state_t;
  }
  /* .lzma has no header/signature, can only detect it by extension */
  if !is_suffixed_with(fname, b".lzma\x00" as *const u8 as *const libc::c_char).is_null() {
    xstate = xzalloc(::std::mem::size_of::<transformer_state_t>() as libc::c_ulong)
      as *mut transformer_state_t;
    (*xstate).src_fd = fd;
    (*xstate).xformer = Some(
      unpack_lzma_stream as unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong,
    );
    return xstate;
  }
  xstate = setup_transformer_on_fd(fd, fail_if_not_compressed);
  return xstate;
}
#[no_mangle]
pub unsafe extern "C" fn open_zipped(
  mut fname: *const libc::c_char,
  mut fail_if_not_compressed: libc::c_int,
) -> libc::c_int {
  let mut fd: libc::c_int = 0;
  let mut xstate: *mut transformer_state_t = 0 as *mut transformer_state_t;
  xstate = open_transformer(fname, fail_if_not_compressed);
  if xstate.is_null() {
    return -1i32;
  }
  fd = (*xstate).src_fd;
  if (*xstate).xformer.is_some() {
    fork_transformer(fd, 1i32, (*xstate).xformer);
  } else {
    /* the file is not compressed */
    xlseek(
      fd,
      -((*xstate).signature_skipped as libc::c_int) as off_t,
      1i32,
    );
    (*xstate).signature_skipped = 0i32 as smallint
  }
  free(xstate as *mut libc::c_void);
  return fd;
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
/* !RETURNS_MALLOC: it's a realloc-like function */
/* bb_signals(BB_FATAL_SIGS, handler) catches all signals which
 * otherwise would kill us, except for those resulting from bugs:
 * SIGSEGV, SIGILL, SIGFPE.
 * Other fatal signals not included (TODO?):
 * SIGBUS   Bus error (bad memory access)
 * SIGPOLL  Pollable event. Synonym of SIGIO
 * SIGPROF  Profiling timer expired
 * SIGSYS   Bad argument to routine
 * SIGTRAP  Trace/breakpoint trap
 *
 * The only known arch with some of these sigs not fitting
 * into 32 bits is parisc (SIGXCPU=33, SIGXFSZ=34, SIGSTKFLT=36).
 * Dance around with long long to guard against that...
 */
// Write to pipe with no readers
// Quit from keyboard
// Abort signal from abort(3)
// Timer signal from alarm(2)
// Virtual alarm clock
// CPU time limit exceeded
// File size limit exceeded
// Yes kids, these are also fatal!
/* Unlike signal() and bb_signals, sets handler with sigaction()
 * and in a way that while signal handler is run, no other signals
 * will be blocked; syscalls will not be restarted: */
/* syscalls like read() will be interrupted with EINTR: */
/* syscalls like read() won't be interrupted (though select/poll will be): */
/* Will do sigaction(signum, act, NULL): */
/* SIG_BLOCK/SIG_UNBLOCK all signals: */
/* Return old set in the same set: */
/* Standard handler which just records signo */
/* not FAST_FUNC! */
/* In this form code with pipes is much more readable */
/* Useful for having small structure members/global variables */
/* | AF_DECnet */
/* | AF_IPX */
/* SO_REUSEADDR allows a server to rebind to an address that is already
 * "in use" by old connections to e.g. previous server instance which is
 * killed or crashed. Without it bind will fail until all such connections
 * time out. Linux does not allow multiple live binds on same ip:port
 * regardless of SO_REUSEADDR (unlike some other flavors of Unix).
 * Turn it on before you call bind(). */
/* On Linux this never fails. */
/* NB: returns port in host byte order */
/* Create stream socket, and allocate suitable lsa.
 * (lsa of correct size and lsa->sa.sa_family (AF_INET/AF_INET6))
 * af == AF_UNSPEC will result in trying to create IPv6 socket,
 * and if kernel doesn't support it, fall back to IPv4.
 * This is useful if you plan to bind to resulting local lsa.
 */
/* Create server socket bound to bindaddr:port. bindaddr can be NULL,
 * numeric IP ("N.N.N.N") or numeric IPv6 address,
 * and can have ":PORT" suffix (for IPv6 use "[X:X:...:X]:PORT").
 * Only if there is no suffix, port argument is used */
/* NB: these set SO_REUSEADDR before bind */
/* Create client TCP socket connected to peer:port. Peer cannot be NULL.
 * Peer can be numeric IP ("N.N.N.N"), numeric IPv6 address or hostname,
 * and can have ":PORT" suffix (for IPv6 use "[X:X:...:X]:PORT").
 * If there is no suffix, port argument is used */
/* Connect to peer identified by lsa */
/* Get local address of bound or accepted socket */
/* Get remote address of connected or accepted socket */
/* Return malloc'ed len_and_sockaddr with socket address of host:port
 * Currently will return IPv4 or IPv6 sockaddrs only
 * (depending on host), but in theory nothing prevents e.g.
 * UNIX socket address being returned, IPX sockaddr etc...
 * On error does bb_error_msg and returns NULL */
/* Version which dies on error */
/* Same, useful if you want to force family (e.g. IPv6) */
/* Assign sin[6]_port member if the socket is an AF_INET[6] one,
 * otherwise no-op. Useful for ftp.
 * NB: does NOT do htons() internally, just direct assignment. */
/* Retrieve sin[6]_port or return -1 for non-INET[6] lsa's */
/* Reverse DNS. Returns NULL on failure. */
/* This one doesn't append :PORTNUM */
/* This one also doesn't fall back to dotted IP (returns NULL) */
/* inet_[ap]ton on steroids */
// "old" (ipv4 only) API
// users: traceroute.c hostname.c - use _list_ of all IPs
// Also mount.c and inetd.c are using gethostbyname(),
// + inet_common.c has additional IPv4-only stuff
/* opaque */
// RFC 5246
// sequence number
//   Each connection state contains a sequence number, which is
//   maintained separately for read and write states.  The sequence
//   number MUST be set to zero whenever a connection state is made the
//   active state.  Sequence numbers are of type uint64 and may not
//   exceed 2^64-1.
/*uint64_t read_seq64_be;*/
/*u8 *server_write_MAC_key;*/
//used by AES_GCM
/* 0 if argv[0] is NULL: */
/* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
 * But potentially slow, don't use in one-billion-times loops */
/* Note: does not use stdio, writes to fd 2 directly */
// gcc-4.1.1 still isn't good enough at optimizing it
// (+200 bytes compared to macro)
//static ALWAYS_INLINE
//int LONE_DASH(const char *s) { return s[0] == '-' && !s[1]; }
//static ALWAYS_INLINE
//int NOT_LONE_DASH(const char *s) { return s[0] != '-' || s[1]; }
/* Returns a string with unprintable chars replaced by '?' or
 * SUBST_WCHAR. This function is unicode-aware. */
/* Prints unprintable char ch as ^C or M-c to file
 * (M-c is used only if ch is ORed with PRINTABLE_META),
 * else it is printed as-is (except for ch = 0x9b) */
/* Return a string that is the printable representation of character ch.
 * Buffer must hold at least four characters. */
// NB: will return short read on error, not -1,
// if some data was read before error occurred
// Reads one line a-la fgets (but doesn't save terminating '\n').
// Reads byte-by-byte. Useful when it is important to not read ahead.
// Bytes are appended to pfx (which must be malloced, or NULL).
/* Reads block up to *maxsz_p (default: INT_MAX - 4095) */
/* Returns NULL if file can't be opened (default max size: INT_MAX - 4095) */
/* Never returns NULL */
/* Else use variable one (a bit more expensive) */
/* Autodetects gzip/bzip2 formats. fd may be in the middle of the file! */
/* Autodetects .gz etc */
#[no_mangle]
pub unsafe extern "C" fn xmalloc_open_zipped_read_close(
  mut fname: *const libc::c_char,
  mut maxsz_p: *mut size_t,
) -> *mut libc::c_void {
  let mut xstate: *mut transformer_state_t = 0 as *mut transformer_state_t;
  let mut image: *mut libc::c_char = 0 as *mut libc::c_char;
  xstate = open_transformer(fname, 0i32);
  if xstate.is_null() {
    /* file open error */
    return 0 as *mut libc::c_void;
  }
  image = 0 as *mut libc::c_char;
  if (*xstate).xformer.is_some() {
    /* In-memory decompression */
    (*xstate).mem_output_size_max = if !maxsz_p.is_null() {
      *maxsz_p
    } else {
      (2147483647i32 - 4095i32) as size_t
    };
    (*xstate).xformer.expect("non-null function pointer")(xstate);
    if !(*xstate).mem_output_buf.is_null() {
      image = (*xstate).mem_output_buf;
      if !maxsz_p.is_null() {
        *maxsz_p = (*xstate).mem_output_size
      }
    }
  } else {
    /* File is not compressed.
     * We already read first few bytes, account for that.
     * Example where it happens:
     * "modinfo MODULE.ko" (not compressed)
     *   open("MODULE.ko", O_RDONLY|O_LARGEFILE) = 4
     *   read(4, "\177E", 2)                     = 2
     *   fstat64(4, ...)
     *   mmap(...)
     *   read(4, "LF\2\1\1\0\0\0\0"...
     * ...and we avoided seeking on the fd! :)
     */
    image = xmalloc_read_with_initial_buf(
      (*xstate).src_fd,
      maxsz_p,
      xmemdup(
        &mut (*xstate).magic as *mut C2RustUnnamed as *const libc::c_void,
        (*xstate).signature_skipped as libc::c_int,
      ) as *mut libc::c_char,
      (*xstate).signature_skipped as size_t,
    ) as *mut libc::c_char;
    (*xstate).signature_skipped = 0i32 as smallint
  }
  if image.is_null() {
    bb_perror_msg(
      b"read error from \'%s\'\x00" as *const u8 as *const libc::c_char,
      fname,
    );
  }
  close((*xstate).src_fd);
  free(xstate as *mut libc::c_void);
  return image as *mut libc::c_void;
}
/* SEAMLESS_COMPRESSION */

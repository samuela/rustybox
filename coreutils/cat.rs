use libc;
extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut stdout: *mut _IO_FILE;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn open_or_warn_stdin(pathname: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn visible(ch: libc::c_uint, buf: *mut libc::c_char, flags: libc::c_int);
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  /* Applets which are useful from another applets */
  #[no_mangle]
  fn bb_cat(argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn print_numbered_lines(ns: *mut number_state, filename: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type smallint = libc::c_schar;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
use crate::librb::_IO_FILE;
pub type _IO_lock_t = ();
use crate::librb::_IO_marker;
use crate::librb::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct number_state {
  pub width: libc::c_uint,
  pub start: libc::c_uint,
  pub inc: libc::c_uint,
  pub sep: *const libc::c_char,
  pub empty_str: *const libc::c_char,
  pub all: smallint,
  pub nonempty: smallint,
}
pub type C2RustUnnamed = libc::c_uint;
pub const COMMON_BUFSIZE: C2RustUnnamed = 1024;
unsafe extern "C" fn catv(mut opts: libc::c_uint, mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut retval: libc::c_int = 0i32;
  let mut fd: libc::c_int = 0;
  let mut eol_seen: bool =
    opts & ((1i32 << 4i32) * 1i32 | (1i32 << 5i32) * 1i32) as libc::c_uint != 0;
  let mut eol_char: libc::c_uint = if eol_seen as libc::c_int != 0 {
    '\n' as i32
  } else {
    0x100i32
  } as libc::c_uint;
  let mut skip_num_on: libc::c_uint = if opts & ((1i32 << 5i32) * 1i32) as libc::c_uint != 0 {
    '\n' as i32
  } else {
    0x100i32
  } as libc::c_uint;
  let mut lineno: libc::c_uint = 0i32 as libc::c_uint;
  /* These consts match, we can just pass "opts" to visible() */
  loop {
    fd = open_or_warn_stdin(*argv);
    if fd < 0i32 {
      retval = 1i32
    } else {
      loop {
        let mut i: libc::c_int = 0;
        let mut res: libc::c_int = 0;
        res = read(
          fd,
          bb_common_bufsiz1.as_mut_ptr() as *mut libc::c_void,
          COMMON_BUFSIZE as libc::c_int as size_t,
        ) as libc::c_int;
        if res < 0i32 {
          retval = 1i32
        }
        if res <= 0i32 {
          break;
        }
        i = 0i32;
        while i < res {
          let mut c: libc::c_uchar =
            *bb_common_bufsiz1.as_mut_ptr().offset(i as isize) as libc::c_uchar;
          let mut buf: [libc::c_char; 5] = [0; 5];
          if eol_seen as libc::c_int != 0 && c as libc::c_uint != skip_num_on {
            lineno = lineno.wrapping_add(1);
            printf(b"%6u  \x00" as *const u8 as *const libc::c_char, lineno);
          }
          eol_seen = c as libc::c_uint == eol_char;
          visible(c as libc::c_uint, buf.as_mut_ptr(), opts as libc::c_int);
          fputs_unlocked(buf.as_mut_ptr(), stdout);
          i += 1
        }
      }
      if 0i32 != 0 && fd != 0 {
        close(fd);
      }
    }
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  fflush_stdout_and_exit(retval);
}
#[no_mangle]
pub unsafe extern "C" fn cat_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opts: libc::c_uint = 0;
  opts = getopt32(
    argv,
    b"^etvAnbu\x00Aetv\x00" as *const u8 as *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  /* Read from stdin if there's nothing else to do. */
  if (*argv.offset(0)).is_null() {
    argv = argv.offset(-1);
    *argv = b"-\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }
  if opts & 7i32 as libc::c_uint != 0 {
    return catv(opts, argv);
  }
  opts >>= 4i32;
  if opts & (1i32 << 0i32 | 1i32 << 1i32) as libc::c_uint != 0 {
    /* -n or -b */
    let mut ns: number_state = number_state {
      width: 0,
      start: 0,
      inc: 0,
      sep: 0 as *const libc::c_char,
      empty_str: 0 as *const libc::c_char,
      all: 0,
      nonempty: 0,
    }; /* -n without -b */
    let mut exitcode: libc::c_int = 0; /* -b (with or without -n) */
    ns.width = 6i32 as libc::c_uint;
    ns.start = 1i32 as libc::c_uint;
    ns.inc = 1i32 as libc::c_uint;
    ns.sep = b"\t\x00" as *const u8 as *const libc::c_char;
    ns.empty_str = b"\n\x00" as *const u8 as *const libc::c_char;
    ns.all = (opts & (1i32 << 1i32) as libc::c_uint == 0) as libc::c_int as smallint;
    ns.nonempty = (opts & (1i32 << 1i32) as libc::c_uint) as smallint;
    exitcode = 0i32;
    loop {
      exitcode |= print_numbered_lines(&mut ns, *argv);
      argv = argv.offset(1);
      if (*argv).is_null() {
        break;
      }
    }
    fflush_stdout_and_exit(exitcode);
  }
  /*opts >>= 2;*/
  return bb_cat(argv);
}

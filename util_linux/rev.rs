use crate::librb::size_t;

use libc;
use libc::fclose;
use libc::free;
use libc::FILE;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut stdout: *mut FILE;

  #[no_mangle]
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut FILE,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn feof_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fopen_or_warn_stdin(filename: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
  #[no_mangle]
  static bb_argv_dash: [*const libc::c_char; 0];
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_mbstowcs(dest: *mut wchar_t, src: *const libc::c_char, n: size_t) -> size_t;
  #[no_mangle]
  fn bb_wcstombs(dest: *mut libc::c_char, src: *const wchar_t, n: size_t) -> size_t;
}

pub type wchar_t = libc::c_int;
/*
 * rev implementation for busybox
 *
 * Copyright (C) 2010 Marek Polacek <mmpolacek@gmail.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config REV
//config:	bool "rev (4.4 kb)"
//config:	default y
//config:	help
//config:	Reverse lines of a file or files.
//applet:IF_REV(APPLET(rev, BB_DIR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_REV) += rev.o
//usage:#define rev_trivial_usage
//usage:	"[FILE]..."
//usage:#define rev_full_usage "\n\n"
//usage:	"Reverse lines of FILE"
/* In-place invert */
unsafe extern "C" fn strrev(mut s: *mut wchar_t, mut len: libc::c_int) {
  let mut i: libc::c_int = 0;
  if len != 0i32 {
    len -= 1;
    if len != 0i32 && *s.offset(len as isize) == '\n' as i32 {
      len -= 1
    }
  }
  i = 0i32;
  while i < len {
    let mut c: wchar_t = *s.offset(i as isize);
    *s.offset(i as isize) = *s.offset(len as isize);
    *s.offset(len as isize) = c;
    i += 1;
    len -= 1
  }
}
#[no_mangle]
pub unsafe extern "C" fn rev_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut retval: libc::c_int = 0;
  let mut bufsize: size_t = 0;
  let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
  getopt32(argv, b"\x00" as *const u8 as *const libc::c_char);
  argv = argv.offset(optind as isize);
  if (*argv.offset(0)).is_null() {
    argv = &bb_argv_dash as *const [*const libc::c_char; 0] as *mut *mut libc::c_char
  }
  retval = 0i32;
  bufsize = 256i32 as size_t;
  buf = xmalloc(bufsize) as *mut libc::c_char;
  loop {
    let mut pos: size_t = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let fresh0 = argv;
    argv = argv.offset(1);
    fp = fopen_or_warn_stdin(*fresh0);
    if fp.is_null() {
      retval = 1i32
    } else {
      pos = 0i32 as size_t;
      loop {
        /* Read one line */
        *buf.offset(bufsize.wrapping_sub(1i32 as libc::c_ulong) as isize) = 1i32 as libc::c_char; /* not 0 */
        if fgets_unlocked(
          buf.offset(pos as isize),
          bufsize.wrapping_sub(pos) as libc::c_int,
          fp,
        )
        .is_null()
        {
          break; /* EOF/error */
        }
        if *buf.offset(bufsize.wrapping_sub(1i32 as libc::c_ulong) as isize) as libc::c_int
          == '\u{0}' as i32
          && *buf.offset(bufsize.wrapping_sub(2i32 as libc::c_ulong) as isize) as libc::c_int
            != '\n' as i32
          && feof_unlocked(fp) == 0
        {
          /* Line is too long, extend buffer */
          pos = bufsize.wrapping_sub(1i32 as libc::c_ulong);
          bufsize = (bufsize as libc::c_ulong).wrapping_add(
            (64i32 as libc::c_ulong).wrapping_add(bufsize.wrapping_div(8i32 as libc::c_ulong)),
          ) as size_t as size_t;
          buf = xrealloc(buf as *mut libc::c_void, bufsize) as *mut libc::c_char
        } else {
          /* Process and print it */
          let mut tmp: *mut wchar_t =
            xmalloc(bufsize.wrapping_mul(::std::mem::size_of::<wchar_t>() as libc::c_ulong))
              as *mut wchar_t;
          /* Convert to wchar_t (might error out!) */
          let mut len: libc::c_int = bb_mbstowcs(tmp, buf, bufsize) as libc::c_int;
          if len >= 0i32 {
            strrev(tmp, len);
            /* Convert back to char */
            bb_wcstombs(buf, tmp, bufsize);
          }
          free(tmp as *mut libc::c_void);
          fputs_unlocked(buf, stdout);
        }
      }
      fclose(fp);
    }
    if (*argv).is_null() {
      break;
    }
  }
  fflush_stdout_and_exit(retval);
}

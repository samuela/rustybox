use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xwrite(fd: libc::c_int, buf: *const libc::c_void, count: size_t);
  #[no_mangle]
  fn fopen_or_warn_stdin(filename: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn llist_add_to(old_head: *mut *mut llist_t, data: *mut libc::c_void);
  #[no_mangle]
  fn llist_pop(elm: *mut *mut llist_t) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
}
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
  pub _flags: libc::c_int,
  pub _IO_read_ptr: *mut libc::c_char,
  pub _IO_read_end: *mut libc::c_char,
  pub _IO_read_base: *mut libc::c_char,
  pub _IO_write_base: *mut libc::c_char,
  pub _IO_write_ptr: *mut libc::c_char,
  pub _IO_write_end: *mut libc::c_char,
  pub _IO_buf_base: *mut libc::c_char,
  pub _IO_buf_end: *mut libc::c_char,
  pub _IO_save_base: *mut libc::c_char,
  pub _IO_backup_base: *mut libc::c_char,
  pub _IO_save_end: *mut libc::c_char,
  pub _markers: *mut _IO_marker,
  pub _chain: *mut _IO_FILE,
  pub _fileno: libc::c_int,
  pub _flags2: libc::c_int,
  pub _old_offset: __off_t,
  pub _cur_column: libc::c_ushort,
  pub _vtable_offset: libc::c_schar,
  pub _shortbuf: [libc::c_char; 1],
  pub _lock: *mut libc::c_void,
  pub _offset: __off64_t,
  pub __pad1: *mut libc::c_void,
  pub __pad2: *mut libc::c_void,
  pub __pad3: *mut libc::c_void,
  pub __pad4: *mut libc::c_void,
  pub __pad5: size_t,
  pub _mode: libc::c_int,
  pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
  pub _next: *mut _IO_marker,
  pub _sbuf: *mut _IO_FILE,
  pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct llist_t {
  pub link: *mut llist_t,
  pub data: *mut libc::c_char,
}
/* vi: set sw=4 ts=4: */
/*
 * tac implementation for busybox
 * tac - concatenate and print files in reverse
 *
 * Copyright (C) 2003  Yang Xiaopeng  <yxp at hanwang.com.cn>
 * Copyright (C) 2007  Natanael Copa  <natanael.copa@gmail.com>
 * Copyright (C) 2007  Tito Ragusa    <farmatito@tiscali.it>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* Based on Yang Xiaopeng's (yxp at hanwang.com.cn) patch
 * http://www.uclibc.org/lists/busybox/2003-July/008813.html
 */
//config:config TAC
//config:	bool "tac (3.9 kb)"
//config:	default y
//config:	help
//config:	tac is used to concatenate and print files in reverse.
//applet:IF_TAC(APPLET_NOEXEC(tac, tac, BB_DIR_USR_BIN, BB_SUID_DROP, tac))
//kbuild:lib-$(CONFIG_TAC) += tac.o
//usage:#define tac_trivial_usage
//usage:	"[FILE]..."
//usage:#define tac_full_usage "\n\n"
//usage:	"Concatenate FILEs and print them in reverse"
/* This is a NOEXEC applet. Be very careful! */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lstring {
  pub size: libc::c_int,
  pub buf: [libc::c_char; 1],
}
#[no_mangle]
pub unsafe extern "C" fn tac_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut name: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut f: *mut FILE = 0 as *mut FILE;
  let mut line: *mut lstring = 0 as *mut lstring;
  let mut list: *mut llist_t = 0 as *mut llist_t;
  let mut retval: libc::c_int = 0i32;
  /* tac from coreutils 6.9 supports:
         -b, --before
                attach the separator before instead of after
         -r, --regex
                interpret the separator as a regular expression
         -s, --separator=STRING
                use STRING as the separator instead of newline
  We support none, but at least we will complain or handle "--":
  */
  getopt32(argv, b"\x00" as *const u8 as *const libc::c_char);
  argv = argv.offset(optind as isize);
  if (*argv).is_null() {
    argv = argv.offset(-1);
    *argv = b"-\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }
  /* We will read from last file to first */
  name = argv;
  while !(*name).is_null() {
    name = name.offset(1)
  }
  loop {
    let mut ch: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    name = name.offset(-1);
    f = fopen_or_warn_stdin(*name);
    if f.is_null() {
      /* error message is printed by fopen_or_warn_stdin */
      retval = 1i32
    } else {
      i = 0i32;
      *bb_errno = i;
      loop {
        ch = getc_unlocked(f);
        if ch != -1i32 {
          if i & 0x7fi32 == 0 {
            /* Grow on every 128th char */
            line = xrealloc(
              line as *mut libc::c_void,
              ((i + 0x7fi32) as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_add(1i32 as libc::c_ulong),
            ) as *mut lstring
          }
          let fresh0 = i;
          i = i + 1;
          *(*line).buf.as_mut_ptr().offset(fresh0 as isize) = ch as libc::c_char
        }
        if ch == '\n' as i32 || ch == -1i32 && i != 0i32 {
          line = xrealloc(
            line as *mut libc::c_void,
            (i as libc::c_ulong)
              .wrapping_add(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
          ) as *mut lstring;
          (*line).size = i;
          llist_add_to(&mut list, line as *mut libc::c_void);
          line = 0 as *mut lstring;
          i = 0i32
        }
        if !(ch != -1i32) {
          break;
        }
      }
      /* fgetc sets errno to ENOENT on EOF, we don't want
       * to warn on this non-error! */
      if *bb_errno != 0 && *bb_errno != 2i32 {
        bb_simple_perror_msg(*name);
        retval = 1i32
      }
    }
    if !(name != argv) {
      break;
    }
  }
  while !list.is_null() {
    line = (*list).data as *mut lstring;
    xwrite(
      1i32,
      (*line).buf.as_mut_ptr() as *const libc::c_void,
      (*line).size as size_t,
    );
    list = (*list).link
  }
  return retval;
}

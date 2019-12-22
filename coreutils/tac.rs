use crate::libbb::llist::llist_t;
use crate::libbb::ptr_to_globals::bb_errno;
use crate::librb::size_t;
use libc;
use libc::FILE;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;

}

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
//applet:IF_TAC(APPLET_NOEXEC(tac, tac, BB_DIR_USR_BIN, SUID_DROP, tac))
//kbuild:lib-$(CONFIG_TAC) += tac.o
//usage:#define tac_trivial_usage
//usage:	"[FILE]..."
//usage:#define tac_full_usage "\n\n"
//usage:	"Concatenate FILEs and print them in reverse"
/* This is a NOEXEC applet. Be very careful! */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lstring {
  pub size: libc::c_int,
  pub buf: [libc::c_char; 1],
}
#[no_mangle]
pub unsafe extern "C" fn tac_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut name: *mut *mut libc::c_char = std::ptr::null_mut();
  let mut f: *mut FILE = std::ptr::null_mut();
  let mut line: *mut lstring = std::ptr::null_mut();
  let mut list: *mut llist_t = std::ptr::null_mut();
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
  crate::libbb::getopt32::getopt32(argv, b"\x00" as *const u8 as *const libc::c_char);
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
    f = crate::libbb::wfopen_input::fopen_or_warn_stdin(*name);
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
            line = crate::libbb::xfuncs_printf::xrealloc(
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
          line = crate::libbb::xfuncs_printf::xrealloc(
            line as *mut libc::c_void,
            (i as libc::c_ulong)
              .wrapping_add(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
          ) as *mut lstring;
          (*line).size = i;
          crate::libbb::llist::llist_add_to(&mut list, line as *mut libc::c_void);
          line = std::ptr::null_mut();
          i = 0i32
        }
        if !(ch != -1i32) {
          break;
        }
      }
      /* fgetc sets errno to ENOENT on EOF, we don't want
       * to warn on this non-error! */
      if *bb_errno != 0 && *bb_errno != 2i32 {
        crate::libbb::perror_msg::bb_simple_perror_msg(*name);
        retval = 1i32
      }
    }
    if !(name != argv) {
      break;
    }
  }
  while !list.is_null() {
    line = (*list).data as *mut lstring;
    crate::libbb::xfuncs_printf::xwrite(
      1i32,
      (*line).buf.as_mut_ptr() as *const libc::c_void,
      (*line).size as size_t,
    );
    list = (*list).link
  }
  return retval;
}

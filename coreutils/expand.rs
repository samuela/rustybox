use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut stdin: *mut FILE;
  #[no_mangle]
  static mut stdout: *mut FILE;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn clearerr(__stream: *mut FILE);
  #[no_mangle]
  fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
  #[no_mangle]
  fn xmalloc_fgets(file: *mut FILE) -> *mut libc::c_char;
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
  #[no_mangle]
  fn fclose_if_not_stdin(file: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fopen_or_warn_stdin(filename: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn xatou_range(str: *const libc::c_char, l: libc::c_uint, u: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> u32;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  static bb_msg_standard_input: [libc::c_char; 0];
  /* Width on terminal */
  #[no_mangle]
  fn unicode_strwidth(string: *const libc::c_char) -> size_t;
}

use crate::librb::size_t;


use libc::FILE;
/* expand - convert tabs to spaces
 * unexpand - convert spaces to tabs
 *
 * Copyright (C) 89, 91, 1995-2006 Free Software Foundation, Inc.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 *
 * David MacKenzie <djm@gnu.ai.mit.edu>
 *
 * Options for expand:
 * -t num  --tabs NUM      Convert tabs to num spaces (default 8 spaces).
 * -i      --initial       Only convert initial tabs on each line to spaces.
 *
 * Options for unexpand:
 * -a      --all           Convert all blanks, instead of just initial blanks.
 * -f      --first-only    Convert only leading sequences of blanks (default).
 * -t num  --tabs NUM      Have tabs num characters apart instead of 8.
 *
 *  Busybox version (C) 2007 by Tito Ragusa <farmatito@tiscali.it>
 *
 *  Caveat: this versions of expand and unexpand don't accept tab lists.
 */
//config:config EXPAND
//config:	bool "expand (5.1 kb)"
//config:	default y
//config:	help
//config:	By default, convert all tabs to spaces.
//config:
//config:config UNEXPAND
//config:	bool "unexpand (5.3 kb)"
//config:	default y
//config:	help
//config:	By default, convert only leading sequences of blanks to tabs.
//applet:IF_EXPAND(APPLET(expand, BB_DIR_USR_BIN, BB_SUID_DROP))
//                   APPLET_ODDNAME:name      main    location        suid_type     help
//applet:IF_UNEXPAND(APPLET_ODDNAME(unexpand, expand, BB_DIR_USR_BIN, BB_SUID_DROP, unexpand))
//kbuild:lib-$(CONFIG_EXPAND) += expand.o
//kbuild:lib-$(CONFIG_UNEXPAND) += expand.o
//usage:#define expand_trivial_usage
//usage:       "[-i] [-t N] [FILE]..."
//usage:#define expand_full_usage "\n\n"
//usage:       "Convert tabs to spaces, writing to stdout\n"
//usage:     "\n	-i	Don't convert tabs after non blanks"
//usage:     "\n	-t	Tabstops every N chars"
//usage:#define unexpand_trivial_usage
//usage:       "[-fa][-t N] [FILE]..."
//usage:#define unexpand_full_usage "\n\n"
//usage:       "Convert spaces to tabs, writing to stdout\n"
//usage:     "\n	-a	Convert all blanks"
//usage:     "\n	-f	Convert only leading blanks"
//usage:     "\n	-t N	Tabstops every N chars"
pub type C2RustUnnamed = libc::c_uint;
pub const OPT_ALL: C2RustUnnamed = 4;
pub const OPT_TABS: C2RustUnnamed = 2;
pub const OPT_INITIAL: C2RustUnnamed = 1;
//FIXME: does not work properly with input containing NULs
//coreutils 8.30 preserves NULs but treats them as chars of width zero:
//AB<nul><tab>C will expand <tab> to 6 spaces, not 5.
unsafe extern "C" fn expand(
  mut file: *mut FILE,
  mut tab_size: libc::c_uint,
  mut opt: libc::c_uint,
) {
  loop {
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr_strbeg: *mut libc::c_char = 0 as *mut libc::c_char;
    //commented-out code handles NULs, +90 bytes of code, not tested much
    //		size_t linelen;
    //		unsigned len = 0;
    //		linelen = 1024 * 1024;
    //		line = xmalloc_fgets_str_len(file, "\n", &linelen);
    line = xmalloc_fgets(file); //
    if line.is_null() {
      break;
    }
    ptr_strbeg = line;
    ptr = ptr_strbeg;
    loop {
      let mut c: libc::c_uchar = *ptr as libc::c_uchar;
      if c as libc::c_int == '\u{0}' as i32 {
        break;
      }
      if opt & OPT_INITIAL as libc::c_int as libc::c_uint != 0
        && ({
          let mut bb__isblank: libc::c_uchar = c;
          (bb__isblank as libc::c_int == ' ' as i32 || bb__isblank as libc::c_int == '\t' as i32)
            as libc::c_int
        }) == 0
      {
        /* not space or tab */
        break; //
      } else {
        if c as libc::c_int == '\t' as i32 {
          let mut len: libc::c_uint = 0i32 as libc::c_uint;
          *ptr = '\u{0}' as i32 as libc::c_char;
          len = (len as libc::c_ulong).wrapping_add(unicode_strwidth(ptr_strbeg)) as libc::c_uint
            as libc::c_uint;
          len = tab_size.wrapping_sub(len.wrapping_rem(tab_size));
          /*while (ptr[1] == '\t') { ptr++; len += tab_size; } - can handle many tabs at once */
          printf(
            b"%s%*s\x00" as *const u8 as *const libc::c_char,
            ptr_strbeg,
            len,
            b"\x00" as *const u8 as *const libc::c_char,
          );
          //				len = 0;
          ptr_strbeg = ptr.offset(1)
        }
        ptr = ptr.offset(1)
      }
    }
    fputs_unlocked(ptr_strbeg, stdout);
    free(line as *mut libc::c_void);
  }
}
unsafe extern "C" fn unexpand(
  mut file: *mut FILE,
  mut tab_size: libc::c_uint,
  mut opt: libc::c_uint,
) {
  let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
  loop {
    line = xmalloc_fgets(file);
    if line.is_null() {
      break;
    }
    let mut ptr: *mut libc::c_char = line;
    let mut column: libc::c_uint = 0i32 as libc::c_uint;
    while *ptr != 0 {
      let mut n: libc::c_uint = 0;
      let mut len: libc::c_uint = 0i32 as libc::c_uint;
      while *ptr as libc::c_int == ' ' as i32 {
        ptr = ptr.offset(1);
        len = len.wrapping_add(1)
      }
      column = column.wrapping_add(len);
      if *ptr as libc::c_int == '\t' as i32 {
        column = column.wrapping_add(tab_size.wrapping_sub(column.wrapping_rem(tab_size)));
        ptr = ptr.offset(1)
      } else {
        n = column.wrapping_div(tab_size);
        if n != 0 {
          column = column.wrapping_rem(tab_size);
          len = column;
          loop {
            let fresh0 = n;
            n = n.wrapping_sub(1);
            if !(fresh0 != 0) {
              break;
            }
            putchar_unlocked('\t' as i32);
          }
        }
        if opt & OPT_INITIAL as libc::c_int as libc::c_uint != 0 && ptr != line {
          printf(
            b"%*s%s\x00" as *const u8 as *const libc::c_char,
            len,
            b"\x00" as *const u8 as *const libc::c_char,
            ptr,
          );
          break;
        } else {
          n = strcspn(ptr, b"\t \x00" as *const u8 as *const libc::c_char) as libc::c_uint;
          printf(
            b"%*s%.*s\x00" as *const u8 as *const libc::c_char,
            len,
            b"\x00" as *const u8 as *const libc::c_char,
            n,
            ptr,
          );
          let mut c: libc::c_char = *ptr.offset(n as isize);
          *ptr.offset(n as isize) = '\u{0}' as i32 as libc::c_char;
          len = unicode_strwidth(ptr) as libc::c_uint;
          *ptr.offset(n as isize) = c;
          ptr = ptr.offset(n as isize);
          column = column.wrapping_add(len).wrapping_rem(tab_size)
        }
      }
    }
    free(line as *mut libc::c_void);
  }
}
#[no_mangle]
pub unsafe extern "C" fn expand_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  /* Default 8 spaces for 1 tab */
  let mut opt_t: *const libc::c_char = b"8\x00" as *const u8 as *const libc::c_char;
  let mut file: *mut FILE = 0 as *mut FILE;
  let mut tab_size: libc::c_uint = 0;
  let mut opt: libc::c_uint = 0;
  let mut exit_status: libc::c_int = 0i32;
  if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(0) as libc::c_int == 'e' as i32) {
    opt = getopt32long(
      argv,
      b"it:\x00" as *const u8 as *const libc::c_char,
      b"initial\x00\x00itabs\x00\x01t\x00" as *const u8 as *const libc::c_char,
      &mut opt_t as *mut *const libc::c_char,
    )
  } else {
    opt = getopt32long(
      argv,
      b"^ft:a\x00ta\x00" as *const u8 as *const libc::c_char,
      b"first-only\x00\x00itabs\x00\x01tall\x00\x00a\x00" as *const u8 as *const libc::c_char,
      &mut opt_t as *mut *const libc::c_char,
    );
    /* -f --first-only is the default */
    if opt & OPT_ALL as libc::c_int as libc::c_uint == 0 {
      opt |= OPT_INITIAL as libc::c_int as libc::c_uint
    }
  }
  tab_size = xatou_range(
    opt_t,
    1i32 as libc::c_uint,
    (2147483647i32 as libc::c_uint)
      .wrapping_mul(2u32)
      .wrapping_add(1u32),
  );
  argv = argv.offset(optind as isize);
  if (*argv).is_null() {
    argv = argv.offset(-1);
    *argv = bb_msg_standard_input.as_ptr() as *mut libc::c_char
  }
  loop {
    file = fopen_or_warn_stdin(*argv);
    if file.is_null() {
      exit_status = 1i32
    } else {
      if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(0) as libc::c_int == 'e' as i32) {
        expand(file, tab_size, opt);
      } else {
        unexpand(file, tab_size, opt);
      }
      /* Check and close the file */
      if fclose_if_not_stdin(file) != 0 {
        bb_simple_perror_msg(*argv);
        exit_status = 1i32
      }
      /* If stdin also clear EOF */
      if file == stdin {
        clearerr(file);
      }
    }
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  /* Now close stdin also */
  /* (if we didn't read from it, it's a no-op) */
  if fclose(stdin) != 0 {
    bb_simple_perror_msg_and_die(bb_msg_standard_input.as_ptr());
  }
  fflush_stdout_and_exit(exit_status);
}

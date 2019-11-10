use libc;
use libc::putenv;
use libc::umask;
use libc::mknod;
use libc::fchmod;
use libc::fscanf;
use libc::alarm;
use libc::sync;
use libc::setsid;
use libc::ioctl;
use libc::statfs;
use libc::mount;
use libc::prctl;
use libc::opendir;
use libc::closedir;
use libc::readdir;
use libc::strtok;
use libc::putchar_unlocked;
use libc::endmntent;
use libc::setmntent;
use libc::umount2;
use libc::getegid;
use libc::getuid;
use libc::getgid;
use libc::setutxent;
use libc::endutxent;






use libc::getenv;
use libc::geteuid;

use libc::getpid;









use libc::strcpy;





use libc::atoi;
use libc::fclose;
use libc::fprintf;
use libc::lstat;
use libc::printf;
use libc::puts;
use libc::rename;

use libc::sprintf;
use libc::strchr;
use libc::strcmp;
use libc::strrchr;
use libc::strstr;

use libc::open;

use libc::close;
use libc::free;
extern "C" {

  #[no_mangle]
  fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;








  #[no_mangle]
  fn sigaction(__sig: libc::c_int, __act: *const sigaction, __oact: *mut sigaction) -> libc::c_int;
  #[no_mangle]
  static mut stdin: *mut FILE;
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
  fn strtoul(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_ulong;



  #[no_mangle]
  fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;


  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;



  /* Copyright (C) 1991,92,95,96,97,98,99,2001 Free Software Foundation, Inc.
  This file is part of the GNU C Library.

  The GNU C Library is free software; you can redistribute it and/or
  modify it under the terms of the GNU Lesser General Public
  License as published by the Free Software Foundation; either
  version 2.1 of the License, or (at your option) any later version.

  The GNU C Library is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
  Lesser General Public License for more details.

  You should have received a copy of the GNU Lesser General Public
  License along with the GNU C Library; if not, write to the Free
  Software Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA
  02111-1307 USA.  */
  /*
   * POSIX Standard: 9.2.2 User Database Access	<pwd.h>
   */
  /* This file is #included after #include <pwd.h>
   * We will use libc-defined structures, but will #define function names
   * so that function calls are directed to bb_internal_XXX replacements
   */
  /* All function names below should be remapped by #defines above
   * in order to not collide with libc names. */
  /* Rewind the password-file stream.  */
  #[no_mangle]
  fn bb_internal_setpwent();
  /* Close the password-file stream.  */
  #[no_mangle]
  fn bb_internal_endpwent();
  /* Read an entry from the password-file stream, opening it if necessary.  */
  #[no_mangle]
  fn bb_internal_getpwent() -> *mut passwd;
  /* Search for an entry with a matching user ID.  */
  #[no_mangle]
  fn bb_internal_getpwuid(__uid: uid_t) -> *mut passwd;
  /* Search for an entry with a matching username.  */
  #[no_mangle]
  fn bb_internal_getpwnam(__name: *const libc::c_char) -> *mut passwd;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc_vector_helper(
    vector: *mut libc::c_void,
    sizeof_and_shift: libc::c_uint,
    idx: libc::c_int,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xstrndup(s: *const libc::c_char, n: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_process_escape_sequence(ptr: *mut *const libc::c_char) -> libc::c_char;
  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xrealloc_getcwd_or_warn(cwd: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn sigaction_set(sig: libc::c_int, act: *const sigaction) -> libc::c_int;
  #[no_mangle]
  fn xlseek(fd: libc::c_int, offset: off_t, whence: libc::c_int) -> off_t;
  #[no_mangle]
  fn strftime_HHMMSS(
    buf: *mut libc::c_char,
    len: libc::c_uint,
    tp: *mut time_t,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn overlapping_strcpy(dst: *mut libc::c_char, src: *const libc::c_char);
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn printable_string(str: *const libc::c_char) -> *const libc::c_char;
  #[no_mangle]
  fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn xmalloc_fgetline(file: *mut FILE) -> *mut libc::c_char;
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn xfdopen_for_write(fd: libc::c_int) -> *mut FILE;
  #[no_mangle]
  fn qsort_string_vector(sv: *mut *mut libc::c_char, count: libc::c_uint);
  #[no_mangle]
  fn safe_gethostname() -> *mut libc::c_char;
  #[no_mangle]
  fn concat_path_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn get_terminal_width(fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn tcsetattr_stdin_TCSANOW(tp: *const termios) -> libc::c_int;
  #[no_mangle]
  fn get_termios_and_make_raw(
    fd: libc::c_int,
    newterm: *mut termios,
    oldterm: *mut termios,
    flags: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn read_key(fd: libc::c_int, buffer: *mut libc::c_char, timeout: libc::c_int) -> int64_t;
  #[no_mangle]
  fn read_key_ungets(buffer: *mut libc::c_char, str: *const libc::c_char, len: libc::c_uint);
  #[no_mangle]
  static bb_msg_unknown: [libc::c_char; 0];
  #[no_mangle]
  static const_int_0: libc::c_int;
  /* Width on terminal */
  #[no_mangle]
  fn unicode_strwidth(string: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn bb_mbstowcs(dest: *mut wchar_t, src: *const libc::c_char, n: size_t) -> size_t;
  #[no_mangle]
  fn bb_wcstombs(dest: *mut libc::c_char, src: *const wchar_t, n: size_t) -> size_t;
  #[no_mangle]
  fn bb_wcrtomb(s: *mut libc::c_char, wc: wchar_t, ps: *mut bb_mbstate_t) -> size_t;
  #[no_mangle]
  fn bb_wcwidth(ucs: libc::c_uint) -> libc::c_int;
  /* See lineedit_ptr_hack.c */
  #[no_mangle]
  static lineedit_ptr_to_statics: *mut lineedit_statics;
}

pub type __int64_t = libc::c_long;

use libc::pid_t;

pub type int64_t = __int64_t;

/* NB: unaligned parameter should be a pointer, aligned one -
 * a lvalue. This makes it more likely to not swap them by mistake
 */
/* #elif ... - add your favorite arch today! */
/* Unaligned, fixed-endian accessors */
/* unxz needs an aligned fixed-endian accessor.
 * (however, the compiler does not realize it's aligned, the cast is still necessary)
 */
/* ---- Size-saving "small" ints (arch-dependent) ----------- */
/* add other arches which benefit from this... */
use crate::librb::size_t;
use crate::librb::smallint;
use libc::dirent;
use libc::off_t;
use libc::ssize_t;
use libc::uid_t;

use libc::sigval;
use libc::stat;
use libc::time_t;
use libc::DIR;

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub _pad: [libc::c_int; 28],
  pub _kill: C2RustUnnamed_8,
  pub _timer: C2RustUnnamed_7,
  pub _rt: C2RustUnnamed_6,
  pub _sigchld: C2RustUnnamed_5,
  pub _sigfault: C2RustUnnamed_2,
  pub _sigpoll: C2RustUnnamed_1,
  pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
  pub _call_addr: *mut libc::c_void,
  pub _syscall: libc::c_int,
  pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
  pub si_band: libc::c_long,
  pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
  pub si_addr: *mut libc::c_void,
  pub si_addr_lsb: libc::c_short,
  pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
  pub _addr_bnd: C2RustUnnamed_4,
  pub _pkey: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
  pub _lower: *mut libc::c_void,
  pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
  pub si_pid: pid_t,
  pub si_uid: uid_t,
  pub si_status: libc::c_int,
  pub si_utime: libc::clock_t,
  pub si_stime: libc::clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
  pub si_pid: pid_t,
  pub si_uid: uid_t,
  pub si_sigval: sigval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
  pub si_tid: libc::c_int,
  pub si_overrun: libc::c_int,
  pub si_sigval: sigval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
  pub si_pid: pid_t,
  pub si_uid: uid_t,
}

use crate::librb::signal::sigaction;

use libc::FILE;
pub type wchar_t = libc::c_int;

use libc::passwd;
use libc::termios;
pub type C2RustUnnamed_10 = libc::c_int;
pub const KEYCODE_BUFFER_SIZE: C2RustUnnamed_10 = 16;
pub const KEYCODE_CURSOR_POS: C2RustUnnamed_10 = -256;
pub const KEYCODE_ALT_D: C2RustUnnamed_10 = -45;
pub const KEYCODE_ALT_BACKSPACE: C2RustUnnamed_10 = -44;
pub const KEYCODE_ALT_LEFT: C2RustUnnamed_10 = -37;
pub const KEYCODE_ALT_RIGHT: C2RustUnnamed_10 = -36;
pub const KEYCODE_CTRL_LEFT: C2RustUnnamed_10 = -69;
pub const KEYCODE_CTRL_RIGHT: C2RustUnnamed_10 = -68;
pub const KEYCODE_D: C2RustUnnamed_10 = -13;
pub const KEYCODE_BACKSPACE: C2RustUnnamed_10 = -12;
pub const KEYCODE_PAGEDOWN: C2RustUnnamed_10 = -11;
pub const KEYCODE_PAGEUP: C2RustUnnamed_10 = -10;
pub const KEYCODE_DELETE: C2RustUnnamed_10 = -9;
pub const KEYCODE_INSERT: C2RustUnnamed_10 = -8;
pub const KEYCODE_END: C2RustUnnamed_10 = -7;
pub const KEYCODE_HOME: C2RustUnnamed_10 = -6;
pub const KEYCODE_LEFT: C2RustUnnamed_10 = -5;
pub const KEYCODE_RIGHT: C2RustUnnamed_10 = -4;
pub const KEYCODE_DOWN: C2RustUnnamed_10 = -3;
pub const KEYCODE_UP: C2RustUnnamed_10 = -2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line_input_t {
  pub flags: libc::c_int,
  pub timeout: libc::c_int,
  pub path_lookup: *const libc::c_char,
  pub cnt_history: libc::c_int,
  pub cur_history: libc::c_int,
  pub max_history: libc::c_int,
  pub cnt_history_in_file: libc::c_uint,
  pub hist_file: *const libc::c_char,
  pub history: [*mut libc::c_char; 256],
}
pub type C2RustUnnamed_11 = libc::c_uint;
pub const FOR_SHELL: C2RustUnnamed_11 = 7;
pub const WITH_PATH_LOOKUP: C2RustUnnamed_11 = 16;
pub const VI_MODE: C2RustUnnamed_11 = 0;
pub const USERNAME_COMPLETION: C2RustUnnamed_11 = 4;
pub const TAB_COMPLETION: C2RustUnnamed_11 = 2;
pub const DO_HISTORY: C2RustUnnamed_11 = 1;
/* We try to minimize both static and stack usage. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lineedit_statics {
  pub state: *mut line_input_t,
  pub cmdedit_termw: libc::c_uint,
  pub cmdedit_x: libc::c_uint,
  pub cmdedit_y: libc::c_uint,
  pub cmdedit_prmt_len: libc::c_uint,
  pub cursor: libc::c_uint,
  pub command_len: libc::c_int,
  pub maxsize: libc::c_int,
  pub command_ps: *mut wchar_t,
  pub cmdedit_prompt: *const libc::c_char,
  pub prompt_last_line: *const libc::c_char,
  pub user_buf: *mut libc::c_char,
  pub home_pwd_buf: *mut libc::c_char,
  pub matches: *mut *mut libc::c_char,
  pub num_matches: libc::c_uint,
  pub SIGWINCH_saved: libc::c_uint,
  pub SIGWINCH_count: libc::c_uint,
  pub ok_to_redraw: smallint,
  pub SIGWINCH_handler: sigaction,
}
pub const MAX_LINELEN: C2RustUnnamed_13 = 1024;
pub const UNICODE_ON: C2RustUnnamed_12 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bb_mbstate_t {
  pub bogus: libc::c_char,
}
pub const VI_CMDMODE_BIT: C2RustUnnamed_15 = 1073741824;
pub const FIND_DIR_ONLY: C2RustUnnamed_14 = 1;
pub const FIND_EXE_ONLY: C2RustUnnamed_14 = 0;
pub const FIND_FILE_ONLY: C2RustUnnamed_14 = 2;

/*
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
pub type C2RustUnnamed_12 = libc::c_uint;
pub const UNICODE_OFF: C2RustUnnamed_12 = 1;
pub const UNICODE_UNKNOWN: C2RustUnnamed_12 = 0;
//#define SEQ_CLEAR_TILL_END_OF_LINE  ESC"[K"
pub type C2RustUnnamed_13 = libc::c_uint;
/* FEATURE_USERNAME_COMPLETION */
pub type C2RustUnnamed_14 = libc::c_uint;
/* leave out the "vi-mode"-only case labels if vi editing isn't
 * configured. */
/* convert uppercase ascii to equivalent control char, for readability */
pub type C2RustUnnamed_15 = libc::c_uint;
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
unsafe extern "C" fn BB_isspace(mut c: wchar_t) -> bool {
  return (c as libc::c_uint) < 256i32 as libc::c_uint
    && ({
      let mut bb__isspace: libc::c_uchar = (c - 9i32) as libc::c_uchar;
      (bb__isspace as libc::c_int == ' ' as i32 - 9i32
        || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
    }) != 0;
}
unsafe extern "C" fn BB_ispunct(mut c: wchar_t) -> bool {
  return (c as libc::c_uint) < 256i32 as libc::c_uint
    && *strchrnul(
      b"!\"#$%&\'()*+,-./:;<=>?@[\\]^_`{|}~\x00" as *const u8 as *const libc::c_char,
      c,
    )
    .offset(0) as libc::c_int
      != 0;
}
static mut null_str: [libc::c_char; 1] = [0];
unsafe extern "C" fn deinit_S() {
  /* 0x80000000 bit flags KEYCODE_xxx */
  /* This one is allocated only if FANCY_PROMPT is on
   * (otherwise it points to verbatim prompt (NOT malloced)) */
  free((*lineedit_ptr_to_statics).cmdedit_prompt as *mut libc::c_char as *mut libc::c_void);
  free((*lineedit_ptr_to_statics).user_buf as *mut libc::c_void);
  if (*lineedit_ptr_to_statics).home_pwd_buf != null_str.as_ptr() as *mut libc::c_char {
    free((*lineedit_ptr_to_statics).home_pwd_buf as *mut libc::c_void);
  }
  free(lineedit_ptr_to_statics as *mut libc::c_void);
}
unsafe extern "C" fn load_string(mut src: *const libc::c_char) -> size_t {
  if UNICODE_ON as libc::c_int == UNICODE_ON as libc::c_int {
    let mut len: ssize_t = bb_mbstowcs(
      (*lineedit_ptr_to_statics).command_ps,
      src,
      ((*lineedit_ptr_to_statics).maxsize - 1i32) as size_t,
    ) as ssize_t;
    if len < 0 {
      len = 0i32 as ssize_t
    }
    *(*lineedit_ptr_to_statics).command_ps.offset(len as isize) = 0i32;
    return len as size_t;
  } else {
    let mut i: libc::c_uint = 0i32 as libc::c_uint;
    while *src.offset(i as isize) as libc::c_int != 0
      && i < ((*lineedit_ptr_to_statics).maxsize - 1i32) as libc::c_uint
    {
      *(*lineedit_ptr_to_statics).command_ps.offset(i as isize) =
        *src.offset(i as isize) as wchar_t;
      i = i.wrapping_add(1)
    }
    *(*lineedit_ptr_to_statics).command_ps.offset(i as isize) = 0i32;
    return i as size_t;
  };
}
unsafe extern "C" fn save_string(
  mut dst: *mut libc::c_char,
  mut maxsize: libc::c_uint,
) -> libc::c_uint {
  if UNICODE_ON as libc::c_int == UNICODE_ON as libc::c_int {
    let mut len: ssize_t = bb_wcstombs(
      dst,
      (*lineedit_ptr_to_statics).command_ps,
      maxsize.wrapping_sub(1i32 as libc::c_uint) as size_t,
    ) as ssize_t;
    if len < 0 {
      len = 0i32 as ssize_t
    }
    *dst.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    return len as libc::c_uint;
  } else {
    let mut i: libc::c_uint = 0i32 as libc::c_uint;
    loop {
      let ref mut fresh0 = *dst.offset(i as isize);
      *fresh0 = *(*lineedit_ptr_to_statics).command_ps.offset(i as isize) as libc::c_char;
      if !(*fresh0 as libc::c_int != 0i32) {
        break;
      }
      i = i.wrapping_add(1)
    }
    return i;
  };
}
/* I thought just fputwc(c, stdout) would work. But no... */
unsafe extern "C" fn BB_PUTCHAR(mut c: wchar_t) {
  if UNICODE_ON as libc::c_int == UNICODE_ON as libc::c_int {
    let mut buf: [libc::c_char; 7] = [0; 7];
    let mut mbst: bb_mbstate_t = {
      let mut init = bb_mbstate_t {
        bogus: 0i32 as libc::c_char,
      };
      init
    };
    let mut len: ssize_t = bb_wcrtomb(buf.as_mut_ptr(), c, &mut mbst) as ssize_t;
    if len > 0 {
      buf[len as usize] = '\u{0}' as i32 as libc::c_char;
      fputs_unlocked(buf.as_mut_ptr(), stdout);
    }
  } else {
    /* In this case, c is always one byte */
    putchar_unlocked(c);
  };
}
unsafe extern "C" fn adjust_width_and_validate_wc(mut wc: wchar_t) -> wchar_t {
  let mut w: libc::c_int = 1i32;
  if UNICODE_ON as libc::c_int == UNICODE_ON as libc::c_int {
    let mut current_block_2: u64;
    if wc > 767i32 {
      current_block_2 = 15232977139320167099;
    } else {
      w = bb_wcwidth(wc as libc::c_uint);
      if 0i32 != 0 && w < 0i32 || 0i32 == 0 && w <= 0i32 || 0i32 == 0 && w > 1i32 {
        current_block_2 = 15232977139320167099;
      } else {
        current_block_2 = 7815301370352969686;
      }
    }
    match current_block_2 {
      15232977139320167099 =>
      /* note: also true for unicode_is_raw_byte(wc) */
      {
        w = 1i32;
        wc = 63i32
      }
      _ => {}
    }
  }
  return wc;
}
/* !UNICODE */
/* Put 'command_ps[cursor]', cursor++.
 * Advance cursor on screen. If we reached right margin, scroll text up
 * and remove terminal margin effect by printing 'next_char' */
unsafe extern "C" fn put_cur_glyph_and_inc_cursor() {
  let mut c: wchar_t = *(*lineedit_ptr_to_statics)
    .command_ps
    .offset((*lineedit_ptr_to_statics).cursor as isize);
  let mut width: libc::c_uint = 0i32 as libc::c_uint;
  let mut ofs_to_right: libc::c_int = 0;
  if c == 0i32 {
    /* erase character after end of input string */
    c = ' ' as i32
  } else {
    /* advance cursor only if we aren't at the end yet */
    (*lineedit_ptr_to_statics).cursor = (*lineedit_ptr_to_statics).cursor.wrapping_add(1);
    if UNICODE_ON as libc::c_int == UNICODE_ON as libc::c_int {
      (*lineedit_ptr_to_statics).cmdedit_x = (*lineedit_ptr_to_statics).cmdedit_x.wrapping_add(1);
      c = adjust_width_and_validate_wc(c)
    } else {
      (*lineedit_ptr_to_statics).cmdedit_x = (*lineedit_ptr_to_statics).cmdedit_x.wrapping_add(1)
    }
  }
  ofs_to_right = (*lineedit_ptr_to_statics)
    .cmdedit_x
    .wrapping_sub((*lineedit_ptr_to_statics).cmdedit_termw) as libc::c_int;
  if 0i32 == 0 || ofs_to_right <= 0i32 {
    /* c fits on this line */
    BB_PUTCHAR(c);
  }
  if ofs_to_right >= 0i32 {
    /* we go to the next line */
    /* This works better if our idea of term width is wrong
     * and it is actually wider (often happens on serial lines).
     * Printing CR,LF *forces* cursor to next line.
     * OTOH if terminal width is correct AND terminal does NOT
     * have automargin (IOW: it is moving cursor to next line
     * by itself (which is wrong for VT-10x terminals)),
     * this will break things: there will be one extra empty line */
    puts(b"\r\x00" as *const u8 as *const libc::c_char); /* + implicit '\n' */
    (*lineedit_ptr_to_statics).cmdedit_y = (*lineedit_ptr_to_statics).cmdedit_y.wrapping_add(1); /* ofs_to_right > 0 */
    if 0i32 == 0 || ofs_to_right == 0i32 {
      width = 0i32 as libc::c_uint
    } else {
      /* wide char c didn't fit on prev line */
      BB_PUTCHAR(c);
    }
    (*lineedit_ptr_to_statics).cmdedit_x = width
  };
}
/* Move to end of line (by printing all chars till the end) */
unsafe extern "C" fn put_till_end_and_adv_cursor() {
  while (*lineedit_ptr_to_statics).cursor < (*lineedit_ptr_to_statics).command_len as libc::c_uint {
    put_cur_glyph_and_inc_cursor();
  }
}
/* Go to the next line */
unsafe extern "C" fn goto_new_line() {
  put_till_end_and_adv_cursor();
  /* "cursor == 0" is only if prompt is "" and user input is empty */
  if (*lineedit_ptr_to_statics).cursor == 0i32 as libc::c_uint
    || (*lineedit_ptr_to_statics).cmdedit_x != 0i32 as libc::c_uint
  {
    bb_putchar('\n' as i32);
  };
}
unsafe extern "C" fn beep() {
  bb_putchar('\u{7}' as i32);
}
/* Full or last/sole prompt line, reset edit cursor, calculate terminal cursor.
 * cmdedit_y is always calculated for the last/sole prompt line.
 */
unsafe extern "C" fn put_prompt_custom(mut is_full: bool) {
  fputs_unlocked(
    if is_full as libc::c_int != 0 {
      (*lineedit_ptr_to_statics).cmdedit_prompt
    } else {
      (*lineedit_ptr_to_statics).prompt_last_line
    },
    stdout,
  ); /* new quasireal y */
  (*lineedit_ptr_to_statics).cursor = 0i32 as libc::c_uint;
  (*lineedit_ptr_to_statics).cmdedit_y = (*lineedit_ptr_to_statics)
    .cmdedit_prmt_len
    .wrapping_div((*lineedit_ptr_to_statics).cmdedit_termw);
  (*lineedit_ptr_to_statics).cmdedit_x = (*lineedit_ptr_to_statics)
    .cmdedit_prmt_len
    .wrapping_rem((*lineedit_ptr_to_statics).cmdedit_termw);
}
/* Move back one character */
/* (optimized for slow terminals) */
unsafe extern "C" fn input_backward(mut num: libc::c_uint) {
  if num > (*lineedit_ptr_to_statics).cursor {
    num = (*lineedit_ptr_to_statics).cursor
  }
  if num == 0i32 as libc::c_uint {
    return;
  }
  (*lineedit_ptr_to_statics).cursor = (*lineedit_ptr_to_statics).cursor.wrapping_sub(num);
  if (0i32 != 0 || 0i32 != 0) && UNICODE_ON as libc::c_int == UNICODE_ON as libc::c_int {
    /* correct NUM to be equal to _screen_ width */
    let mut n: libc::c_int = num as libc::c_int;
    num = 0i32 as libc::c_uint;
    loop {
      n -= 1;
      if !(n >= 0i32) {
        break;
      }
      num = num.wrapping_add(1);
      adjust_width_and_validate_wc(
        *(*lineedit_ptr_to_statics).command_ps.offset(
          (*lineedit_ptr_to_statics)
            .cursor
            .wrapping_add(n as libc::c_uint) as isize,
        ),
      );
    }
    if num == 0i32 as libc::c_uint {
      return;
    }
  }
  if (*lineedit_ptr_to_statics).cmdedit_x >= num {
    (*lineedit_ptr_to_statics).cmdedit_x = (*lineedit_ptr_to_statics).cmdedit_x.wrapping_sub(num);
    if num <= 4i32 as libc::c_uint {
      loop
      /* This is longer by 5 bytes on x86.
       * Also gets miscompiled for ARM users
       * (busybox.net/bugs/view.php?id=2274).
       * printf(("\b\b\b\b" + 4) - num);
       * return;
       */
      {
        bb_putchar('\u{8}' as i32);
        num = num.wrapping_sub(1);
        if !(num != 0) {
          break;
        }
      }
      return;
    }
    printf(b"\x1b[%uD\x00" as *const u8 as *const libc::c_char, num);
    return;
  }
  /* Need to go one or more lines up */
  let mut lines_up: libc::c_int = 0;
  /* num = chars to go back from the beginning of current line: */
  num = num.wrapping_sub((*lineedit_ptr_to_statics).cmdedit_x);
  /* num=1...w: one line up, w+1...2w: two, etc: */
  lines_up = (1i32 as libc::c_uint).wrapping_add(
    num
      .wrapping_sub(1i32 as libc::c_uint)
      .wrapping_div((*lineedit_ptr_to_statics).cmdedit_termw),
  ) as libc::c_int;
  (*lineedit_ptr_to_statics).cmdedit_x = (*lineedit_ptr_to_statics)
    .cmdedit_termw
    .wrapping_mul((*lineedit_ptr_to_statics).cmdedit_y)
    .wrapping_sub(num)
    .wrapping_rem((*lineedit_ptr_to_statics).cmdedit_termw);
  (*lineedit_ptr_to_statics).cmdedit_y = (*lineedit_ptr_to_statics)
    .cmdedit_y
    .wrapping_sub(lines_up as libc::c_uint);
  /* go to 1st column; go up */
  printf(
    b"\r\x1b[%uA\x00" as *const u8 as *const libc::c_char,
    lines_up,
  );
  /* go to correct column.
   * xterm, konsole, Linux VT interpret 0 as 1 below! wow.
   * need to *make sure* we skip it if cmdedit_x == 0 */
  if (*lineedit_ptr_to_statics).cmdedit_x != 0 {
    printf(
      b"\x1b[%uC\x00" as *const u8 as *const libc::c_char,
      (*lineedit_ptr_to_statics).cmdedit_x,
    );
  };
}
/* See redraw and draw_full below */
unsafe extern "C" fn draw_custom(
  mut y: libc::c_int,
  mut back_cursor: libc::c_int,
  mut is_full: bool,
) {
  if y > 0i32 {
    /* up y lines */
    printf(b"\x1b[%uA\x00" as *const u8 as *const libc::c_char, y);
  }
  bb_putchar('\r' as i32);
  put_prompt_custom(is_full);
  put_till_end_and_adv_cursor();
  printf(b"\x1b[J\x00" as *const u8 as *const libc::c_char);
  input_backward(back_cursor as libc::c_uint);
}
/* Move y lines up, draw last/sole prompt line, editor line[s], and clear tail.
 * goal: redraw the prompt+input+cursor in-place, overwriting the previous */
/* Like above, but without moving up, and while using all the prompt lines.
 * goal: draw a full prompt+input+cursor unrelated to a previous position.
 * note: cmdedit_y always ends up relating to the last/sole prompt line */
/* Delete the char in front of the cursor, optionally saving it
 * for later putback */
unsafe extern "C" fn input_delete() {
  let mut j: libc::c_int = (*lineedit_ptr_to_statics).cursor as libc::c_int;
  if j == (*lineedit_ptr_to_statics).command_len {
    return;
  }
  memmove(
    (*lineedit_ptr_to_statics).command_ps.offset(j as isize) as *mut libc::c_void,
    (*lineedit_ptr_to_statics)
      .command_ps
      .offset(j as isize)
      .offset(1) as *const libc::c_void,
    (((*lineedit_ptr_to_statics).command_len - j) as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<wchar_t>() as libc::c_ulong),
  );
  (*lineedit_ptr_to_statics).command_len -= 1;
  put_till_end_and_adv_cursor();
  /* Last char is still visible, erase it (and more) */
  printf(b"\x1b[J\x00" as *const u8 as *const libc::c_char);
  input_backward(
    (*lineedit_ptr_to_statics)
      .cursor
      .wrapping_sub(j as libc::c_uint),
  );
  /* back to old pos cursor */
}
/* Delete the char in back of the cursor */
unsafe extern "C" fn input_backspace() {
  if (*lineedit_ptr_to_statics).cursor > 0i32 as libc::c_uint {
    input_backward(1i32 as libc::c_uint);
    input_delete();
  };
}
/* Move forward one character */
unsafe extern "C" fn input_forward() {
  if (*lineedit_ptr_to_statics).cursor < (*lineedit_ptr_to_statics).command_len as libc::c_uint {
    put_cur_glyph_and_inc_cursor();
  };
}
//FIXME:
//needs to be more clever: currently it thinks that "foo\ b<TAB>
//matches the file named "foo bar", which is untrue.
//Also, perhaps "foo b<TAB> needs to complete to "foo bar" <cursor>,
//not "foo bar <cursor>...
unsafe extern "C" fn free_tab_completion_data() {
  if !(*lineedit_ptr_to_statics).matches.is_null() {
    while (*lineedit_ptr_to_statics).num_matches != 0 {
      (*lineedit_ptr_to_statics).num_matches =
        (*lineedit_ptr_to_statics).num_matches.wrapping_sub(1);
      free(
        *(*lineedit_ptr_to_statics)
          .matches
          .offset((*lineedit_ptr_to_statics).num_matches as isize) as *mut libc::c_void,
      );
    }
    free((*lineedit_ptr_to_statics).matches as *mut libc::c_void);
    (*lineedit_ptr_to_statics).matches = 0 as *mut *mut libc::c_char
  };
}
unsafe extern "C" fn add_match(mut matched: *mut libc::c_char) {
  let mut p: *mut libc::c_uchar = matched as *mut libc::c_uchar;
  while *p != 0 {
    /* ESC attack fix: drop any string with control chars */
    if (*p as libc::c_int) < ' ' as i32
      || 1i32 == 0 && *p as libc::c_int >= 0x7fi32
      || 1i32 != 0 && *p as libc::c_int == 0x7fi32
    {
      free(matched as *mut libc::c_void);
      return;
    }
    p = p.offset(1)
  }
  (*lineedit_ptr_to_statics).matches = xrealloc_vector_helper(
    (*lineedit_ptr_to_statics).matches as *mut libc::c_void,
    ((::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) << 8i32)
      .wrapping_add(4i32 as libc::c_ulong) as libc::c_uint,
    (*lineedit_ptr_to_statics).num_matches as libc::c_int,
  ) as *mut *mut libc::c_char;
  let ref mut fresh1 = *(*lineedit_ptr_to_statics)
    .matches
    .offset((*lineedit_ptr_to_statics).num_matches as isize);
  *fresh1 = matched;
  (*lineedit_ptr_to_statics).num_matches = (*lineedit_ptr_to_statics).num_matches.wrapping_add(1);
}
/* Replace "~user/..." with "/homedir/...".
 * The parameter is malloced, free it or return it
 * unchanged if no user is matched.
 */
unsafe extern "C" fn username_path_completion(mut ud: *mut libc::c_char) -> *mut libc::c_char {
  let mut entry: *mut passwd = 0 as *mut passwd; /* skip ~ */
  let mut tilde_name: *mut libc::c_char = ud;
  let mut home: *mut libc::c_char = 0 as *mut libc::c_char;
  ud = ud.offset(1);
  if *ud as libc::c_int == '/' as i32 {
    /* "~/..." */
    home = (*lineedit_ptr_to_statics).home_pwd_buf
  } else {
    /* "~user/..." */
    ud = strchr(ud, '/' as i32); /* "~user" */
    *ud = '\u{0}' as i32 as libc::c_char; /* restore "~user/..." */
    entry = bb_internal_getpwnam(tilde_name.offset(1));
    *ud = '/' as i32 as libc::c_char;
    if !entry.is_null() {
      home = (*entry).pw_dir
    }
  }
  if !home.is_null() {
    ud = concat_path_file(home, ud);
    free(tilde_name as *mut libc::c_void);
    tilde_name = ud
  }
  return tilde_name;
}
/* ~use<tab> - find all users with this prefix.
 * Return the length of the prefix used for matching.
 */
#[inline(never)]
unsafe extern "C" fn complete_username(mut ud: *const libc::c_char) -> libc::c_uint {
  let mut pw: *mut passwd = 0 as *mut passwd; /* skip ~ */
  let mut userlen: libc::c_uint = 0;
  ud = ud.offset(1);
  userlen = strlen(ud) as libc::c_uint;
  bb_internal_setpwent();
  loop {
    pw = bb_internal_getpwent();
    if pw.is_null() {
      break;
    }
    /* Null usernames should result in all users as possible completions. */
    if !is_prefixed_with((*pw).pw_name, ud).is_null() {
      add_match(xasprintf(
        b"~%s/\x00" as *const u8 as *const libc::c_char,
        (*pw).pw_name,
      )); /* don't keep password file open */
    }
  }
  bb_internal_endpwent();
  return (1i32 as libc::c_uint).wrapping_add(userlen);
}
unsafe extern "C" fn path_parse(mut p: *mut *mut *mut libc::c_char) -> libc::c_int {
  let mut npth: libc::c_int = 0;
  let mut pth: *const libc::c_char = 0 as *const libc::c_char;
  let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut res: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  if (*(*lineedit_ptr_to_statics).state).flags & WITH_PATH_LOOKUP as libc::c_int != 0 {
    pth = (*(*lineedit_ptr_to_statics).state).path_lookup
  } else {
    pth = getenv(b"PATH\x00" as *const u8 as *const libc::c_char)
  }
  /* PATH="" or PATH=":"? */
  if pth.is_null()
    || *pth.offset(0) == 0
    || *pth.offset(0) as libc::c_int == ':' as i32 && *pth.offset(1) == 0
  {
    return 1i32;
  } /* path component count */
  tmp = pth as *mut libc::c_char; /* :<empty> */
  npth = 1i32; /* ':' -> '\0' */
  loop {
    tmp = strchr(tmp, ':' as i32); /* :<empty> */
    if tmp.is_null() {
      break;
    }
    tmp = tmp.offset(1);
    if *tmp as libc::c_int == '\u{0}' as i32 {
      break;
    }
    npth += 1
  }
  res = xmalloc(
    (npth as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
  ) as *mut *mut libc::c_char;
  *p = res;
  tmp = xstrdup(pth);
  let ref mut fresh2 = *res.offset(0);
  *fresh2 = tmp;
  npth = 1i32;
  loop {
    tmp = strchr(tmp, ':' as i32);
    if tmp.is_null() {
      break;
    }
    let fresh3 = tmp;
    tmp = tmp.offset(1);
    *fresh3 = '\u{0}' as i32 as libc::c_char;
    if *tmp as libc::c_int == '\u{0}' as i32 {
      break;
    }
    let fresh4 = npth;
    npth = npth + 1;
    let ref mut fresh5 = *res.offset(fresh4 as isize);
    *fresh5 = tmp
  }
  return npth;
}
/* Complete command, directory or file name.
 * Return the length of the prefix used for matching.
 */
#[inline(never)]
unsafe extern "C" fn complete_cmd_dir_file(
  mut command: *const libc::c_char,
  mut type_0: libc::c_int,
) -> libc::c_uint {
  let mut current_block: u64;
  let mut path1: [*mut libc::c_char; 1] = [0 as *mut libc::c_char; 1];
  let mut paths: *mut *mut libc::c_char = path1.as_mut_ptr();
  let mut npaths: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  let mut pf_len: libc::c_uint = 0;
  let mut pfind: *const libc::c_char = 0 as *const libc::c_char;
  let mut dirbuf: *mut libc::c_char = 0 as *mut libc::c_char;
  npaths = 1i32;
  path1[0] = b".\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
  pfind = strrchr(command, '/' as i32);
  if pfind.is_null() {
    if type_0 == FIND_EXE_ONLY as libc::c_int {
      npaths = path_parse(&mut paths)
    }
    pfind = command
  } else {
    /* point to 'l' in "..../last_component" */
    pfind = pfind.offset(1);
    /* dirbuf = ".../.../.../" */
    dirbuf = xstrndup(
      command,
      pfind.wrapping_offset_from(command) as libc::c_long as libc::c_int,
    );
    if *dirbuf.offset(0) as libc::c_int == '~' as i32 {
      /* ~/... or ~user/... */
      dirbuf = username_path_completion(dirbuf)
    } /* for every path */
    path1[0] = dirbuf
  } /* don't print an error */
  pf_len = strlen(pfind) as libc::c_uint;
  i = 0i32;
  while i < npaths {
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut next: *mut dirent = 0 as *mut dirent;
    let mut st: stat = std::mem::zeroed();
    let mut found: *mut libc::c_char = 0 as *mut libc::c_char;
    dir = opendir(*paths.offset(i as isize));
    if !dir.is_null() {
      loop {
        next = readdir(dir);
        if next.is_null() {
          break;
        }
        let mut len: libc::c_uint = 0;
        let mut name_found: *const libc::c_char = (*next).d_name.as_mut_ptr();
        /* .../<tab>: bash 3.2.0 shows dotfiles, but not . and .. */
        if *pfind.offset(0) == 0
          && (*name_found.offset(0) as libc::c_int == '.' as i32
            && (*name_found.offset(1) == 0
              || *name_found.offset(1) as libc::c_int == '.' as i32 && *name_found.offset(2) == 0))
        {
          continue;
        }
        /* match? */
        if is_prefixed_with(name_found, pfind).is_null() {
          continue; /* no */
        }
        found = concat_path_file(*paths.offset(i as isize), name_found);
        /* NB: stat() first so that we see is it a directory;
         * but if that fails, use lstat() so that
         * we still match dangling links */
        if !(stat(found, &mut st) != 0 && lstat(found, &mut st) != 0) {
          /* hmm, remove in progress? */
          /* Save only name */
          len = strlen(name_found) as libc::c_uint; /* +2: for slash and NUL */
          found = xrealloc(
            found as *mut libc::c_void,
            len.wrapping_add(2i32 as libc::c_uint) as size_t,
          ) as *mut libc::c_char;
          strcpy(found, name_found);
          if st.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint {
            /* name is a directory, add slash */
            *found.offset(len as isize) = '/' as i32 as libc::c_char;
            *found.offset(len.wrapping_add(1i32 as libc::c_uint) as isize) =
              '\u{0}' as i32 as libc::c_char;
            current_block = 14832935472441733737;
          } else if type_0 == FIND_DIR_ONLY as libc::c_int {
            current_block = 10580215428527682993;
          } else {
            current_block = 14832935472441733737;
          }
          match current_block {
            10580215428527682993 => {}
            _ => {
              /* skip files if looking for dirs only (example: cd) */
              /* add it to the list */
              add_match(found); /* allocated memory is only in first member */
              continue;
            }
          }
        }
        free(found as *mut libc::c_void);
      }
      closedir(dir);
    }
    i += 1
  }
  if paths != path1.as_mut_ptr() {
    free(*paths.offset(0) as *mut libc::c_void);
    free(paths as *mut libc::c_void);
  }
  free(dirbuf as *mut libc::c_void);
  return pf_len;
}
unsafe extern "C" fn remove_chunk(
  mut int_buf: *mut i16,
  mut beg: libc::c_int,
  mut end: libc::c_int,
) {
  /* beg must be <= end */
  if beg == end {
    return;
  }
  loop {
    let ref mut fresh6 = *int_buf.offset(beg as isize);
    *fresh6 = *int_buf.offset(end as isize);
    if !(*fresh6 as libc::c_int != 0i32) {
      break;
    }
    beg += 1;
    end += 1
  }
}
/* Caller ensures that match_buf points to a malloced buffer
 * big enough to hold strlen(match_buf)*2 + 2
 */
#[inline(never)]
unsafe extern "C" fn build_match_prefix(mut match_buf: *mut libc::c_char) -> libc::c_int {
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  let mut command_mode: libc::c_int = 0;
  let mut int_buf: *mut i16 = match_buf as *mut i16;
  /* Copy in reverse order, since they overlap */
  i = strlen(match_buf) as libc::c_int;
  loop {
    *int_buf.offset(i as isize) = *match_buf.offset(i as isize) as libc::c_uchar as i16;
    i -= 1;
    if !(i >= 0i32) {
      break;
    }
  }
  /* Mark every \c as "quoted c" */
  i = 0i32;
  while *int_buf.offset(i as isize) != 0 {
    if *int_buf.offset(i as isize) as libc::c_int == '\\' as i32 {
      remove_chunk(int_buf, i, i + 1i32);
      let ref mut fresh7 = *int_buf.offset(i as isize);
      *fresh7 = (*fresh7 as libc::c_int | 127i32 * 2i32 + 1i32 + 1i32) as i16
    }
    i += 1
  }
  /* Quote-mark "chars" and 'chars', drop delimiters */
  let mut in_quote: libc::c_int = 0i32;
  i = 0i32;
  while *int_buf.offset(i as isize) != 0 {
    let mut cur: libc::c_int = *int_buf.offset(i as isize) as libc::c_int;
    if cur == 0 {
      break;
    }
    if cur == '\'' as i32 || cur == '\"' as i32 {
      if in_quote == 0 || cur == in_quote {
        in_quote ^= cur;
        remove_chunk(int_buf, i, i + 1i32);
        continue;
      }
    }
    if in_quote != 0 {
      *int_buf.offset(i as isize) = (cur | 127i32 * 2i32 + 1i32 + 1i32) as i16
    }
    i += 1
  }
  /* Remove everything up to command delimiters:
   * ';' ';;' '&' '|' '&&' '||',
   * but careful with '>&' '<&' '>|'
   */
  i = 0i32;
  while *int_buf.offset(i as isize) != 0 {
    let mut cur_0: libc::c_int = *int_buf.offset(i as isize) as libc::c_int;
    if cur_0 == ';' as i32 || cur_0 == '&' as i32 || cur_0 == '|' as i32 {
      let mut prev: libc::c_int = if i != 0 {
        *int_buf.offset((i - 1i32) as isize) as libc::c_int
      } else {
        0i32
      };
      if !(cur_0 == '&' as i32 && (prev == '>' as i32 || prev == '<' as i32)) {
        if !(cur_0 == '|' as i32 && prev == '>' as i32) {
          remove_chunk(
            int_buf,
            0i32,
            i + 1i32
              + (cur_0 == *int_buf.offset((i + 1i32) as isize) as libc::c_int) as libc::c_int,
          );
          i = -1i32
        }
      }
      /* back to square 1 */
    }
    i += 1
  }
  let mut current_block_30: u64;
  /* Remove all `cmd` */
  i = 0i32;
  while *int_buf.offset(i as isize) != 0 {
    if *int_buf.offset(i as isize) as libc::c_int == '`' as i32 {
      j = i + 1i32;
      loop {
        if !(*int_buf.offset(j as isize) != 0) {
          current_block_30 = 313581471991351815;
          break;
        }
        if *int_buf.offset(j as isize) as libc::c_int == '`' as i32 {
          /* `cmd` should count as a word:
           * `cmd` c<tab> should search for files c*,
           * not commands c*. Therefore we don't drop
           * `cmd` entirely, we replace it with single `.
           */
          remove_chunk(int_buf, i, j);
          current_block_30 = 3934796541983872331;
          break;
        } else {
          j += 1
        }
      }
      match current_block_30 {
        3934796541983872331 => {}
        _ => {
          /* No closing ` - command mode, remove all up to ` */
          remove_chunk(int_buf, 0i32, i + 1i32);
          break;
        }
      }
    }
    i += 1
  }
  /* Remove "cmd (" and "cmd {"
   * Example: "if { c<tab>"
   * In this example, c should be matched as command pfx.
   */
  i = 0i32;
  while *int_buf.offset(i as isize) != 0 {
    if *int_buf.offset(i as isize) as libc::c_int == '(' as i32
      || *int_buf.offset(i as isize) as libc::c_int == '{' as i32
    {
      remove_chunk(int_buf, 0i32, i + 1i32);
      i = -1i32
      /* back to square 1 */
    }
    i += 1
  }
  /* Remove leading unquoted spaces */
  i = 0i32;
  while *int_buf.offset(i as isize) != 0 {
    if *int_buf.offset(i as isize) as libc::c_int != ' ' as i32 {
      break;
    }
    i += 1
  }
  remove_chunk(int_buf, 0i32, i);
  /* Determine completion mode */
  command_mode = FIND_EXE_ONLY as libc::c_int;
  i = 0i32;
  while *int_buf.offset(i as isize) != 0 {
    if *int_buf.offset(i as isize) as libc::c_int == ' ' as i32
      || *int_buf.offset(i as isize) as libc::c_int == '<' as i32
      || *int_buf.offset(i as isize) as libc::c_int == '>' as i32
    {
      if *int_buf.offset(i as isize) as libc::c_int == ' ' as i32
        && command_mode == FIND_EXE_ONLY as libc::c_int
        && *int_buf.offset(0) as libc::c_char as libc::c_int == 'c' as i32
        && *int_buf.offset(1) as libc::c_char as libc::c_int == 'd' as i32
        && i == 2i32
      {
        /* -> int_buf[2] == ' ' */
        command_mode = FIND_DIR_ONLY as libc::c_int
      } else {
        command_mode = FIND_FILE_ONLY as libc::c_int;
        break;
      }
    }
    i += 1
  }
  /* Remove everything except last word */
  i = 0i32;
  while *int_buf.offset(i as isize) != 0 {
    /* quasi-strlen(int_buf) */
    i += 1
  }
  i -= 1;
  while i >= 0i32 {
    let mut cur_1: libc::c_int = *int_buf.offset(i as isize) as libc::c_int;
    if cur_1 == ' ' as i32
      || cur_1 == '<' as i32
      || cur_1 == '>' as i32
      || cur_1 == '|' as i32
      || cur_1 == '&' as i32
    {
      remove_chunk(int_buf, 0i32, i + 1i32);
      break;
    } else {
      i -= 1
    }
  }
  /* Convert back to string of _chars_ */
  i = 0i32;
  loop {
    let ref mut fresh8 = *match_buf.offset(i as isize);
    *fresh8 = *int_buf.offset(i as isize) as libc::c_char;
    if !(*fresh8 as libc::c_int != '\u{0}' as i32) {
      break;
    }
    i += 1
  }
  return command_mode;
}
/*
 * Display by column (original idea from ls applet,
 * very optimized by me [Vladimir] :)
 */
unsafe extern "C" fn showfiles() {
  let mut ncols: libc::c_int = 0;
  let mut row: libc::c_int = 0;
  let mut column_width: libc::c_int = 0i32;
  let mut nfiles: libc::c_int = (*lineedit_ptr_to_statics).num_matches as libc::c_int;
  let mut nrows: libc::c_int = nfiles;
  let mut l: libc::c_int = 0;
  /* find the longest file name - use that as the column width */
  row = 0i32; /* min space for columns */
  while row < nrows {
    l = unicode_strwidth(*(*lineedit_ptr_to_statics).matches.offset(row as isize)) as libc::c_int;
    if column_width < l {
      column_width = l
    }
    row += 1
  }
  column_width += 2i32;
  ncols = (*lineedit_ptr_to_statics)
    .cmdedit_termw
    .wrapping_div(column_width as libc::c_uint) as libc::c_int;
  if ncols > 1i32 {
    nrows /= ncols;
    if nfiles % ncols != 0 {
      nrows += 1
    }
  /* round up fractionals */
  } else {
    ncols = 1i32
  }
  row = 0i32;
  while row < nrows {
    let mut n: libc::c_int = row;
    let mut nc: libc::c_int = 0;
    nc = 1i32;
    while nc < ncols && n + nrows < nfiles {
      printf(
        b"%s%-*s\x00" as *const u8 as *const libc::c_char,
        *(*lineedit_ptr_to_statics).matches.offset(n as isize),
        (column_width as libc::c_ulong).wrapping_sub(unicode_strwidth(
          *(*lineedit_ptr_to_statics).matches.offset(n as isize),
        )) as libc::c_int,
        b"\x00" as *const u8 as *const libc::c_char,
      );
      n += nrows;
      nc += 1
    }
    puts(printable_string(
      *(*lineedit_ptr_to_statics).matches.offset(n as isize),
    ));
    row += 1
  }
}
unsafe extern "C" fn is_special_char(mut c: libc::c_char) -> *const libc::c_char {
  return strchr(
    b" `\"#$%^&*()=+{}[]:;\'|\\<>\x00" as *const u8 as *const libc::c_char,
    c as libc::c_int,
  );
}
unsafe extern "C" fn quote_special_chars(mut found: *mut libc::c_char) -> *mut libc::c_char {
  let mut l: libc::c_int = 0i32;
  let mut s: *mut libc::c_char = xzalloc(
    strlen(found)
      .wrapping_add(1i32 as libc::c_ulong)
      .wrapping_mul(2i32 as libc::c_ulong),
  ) as *mut libc::c_char;
  while *found != 0 {
    if !is_special_char(*found).is_null() {
      let fresh9 = l;
      l = l + 1;
      *s.offset(fresh9 as isize) = '\\' as i32 as libc::c_char
    }
    let fresh10 = found;
    found = found.offset(1);
    let fresh11 = l;
    l = l + 1;
    *s.offset(fresh11 as isize) = *fresh10
  }
  /* s[l] = '\0'; - already is */
  return s;
}
/* Do TAB completion */
#[inline(never)]
unsafe extern "C" fn input_tab(mut lastWasTab: *mut smallint) {
  let mut current_block: u64;
  let mut chosen_match: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut match_buf: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut len_found: size_t = 0;
  /* Length of string used for matching */
  let mut match_pfx_len: libc::c_uint = 0;
  match_pfx_len = match_pfx_len;
  let mut find_type: libc::c_int = 0;
  /* cursor pos in command converted to multibyte form */
  let mut cursor_mb: libc::c_int = 0;
  if (*(*lineedit_ptr_to_statics).state).flags & TAB_COMPLETION as libc::c_int == 0 {
    return;
  }
  if *lastWasTab != 0 {
    /* The last char was a TAB too.
     * Print a list of all the available choices.
     */
    if (*lineedit_ptr_to_statics).num_matches > 0i32 as libc::c_uint {
      /* cursor will be changed by goto_new_line() */
      let mut sav_cursor: libc::c_int = (*lineedit_ptr_to_statics).cursor as libc::c_int;
      goto_new_line();
      showfiles();
      draw_custom(
        0i32,
        (*lineedit_ptr_to_statics).command_len - sav_cursor,
        1i32 != 0,
      );
    }
    return;
  }
  *lastWasTab = 1i32 as smallint;
  chosen_match = 0 as *mut libc::c_char;
  /* Make a local copy of the string up to the position of the cursor.
   * build_match_prefix will expand it into i16's, need to allocate
   * twice as much as the string_len+1.
   * (we then also (ab)use this extra space later - see (**))
   */
  match_buf = xmalloc(
    (MAX_LINELEN as libc::c_int as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<i16>() as libc::c_ulong),
  ) as *mut libc::c_char;
  let mut wc: wchar_t = *(*lineedit_ptr_to_statics)
    .command_ps
    .offset((*lineedit_ptr_to_statics).cursor as isize);
  *(*lineedit_ptr_to_statics)
    .command_ps
    .offset((*lineedit_ptr_to_statics).cursor as isize) = 0i32;
  save_string(match_buf, MAX_LINELEN as libc::c_int as libc::c_uint);
  *(*lineedit_ptr_to_statics)
    .command_ps
    .offset((*lineedit_ptr_to_statics).cursor as isize) = wc;
  cursor_mb = strlen(match_buf) as libc::c_int;
  find_type = build_match_prefix(match_buf);
  /* Free up any memory already allocated */
  free_tab_completion_data();
  /* If the word starts with ~ and there is no slash in the word,
   * then try completing this word as a username. */
  if (*(*lineedit_ptr_to_statics).state).flags & USERNAME_COMPLETION as libc::c_int != 0 {
    if *match_buf.offset(0) as libc::c_int == '~' as i32 && strchr(match_buf, '/' as i32).is_null()
    {
      match_pfx_len = complete_username(match_buf)
    }
  }
  /* If complete_username() did not match,
   * try to match a command in $PATH, or a directory, or a file */
  if (*lineedit_ptr_to_statics).matches.is_null() {
    match_pfx_len = complete_cmd_dir_file(match_buf, find_type)
  }
  /* Account for backslashes which will be inserted
   * by quote_special_chars() later */
  let mut e: *const libc::c_char = match_buf.offset(strlen(match_buf) as isize);
  let mut s: *const libc::c_char = e.offset(-(match_pfx_len as isize));
  while s < e {
    let fresh12 = s;
    s = s.offset(1);
    if !is_special_char(*fresh12).is_null() {
      match_pfx_len = match_pfx_len.wrapping_add(1)
    }
  }
  /* Remove duplicates */
  if !(*lineedit_ptr_to_statics).matches.is_null() {
    let mut i: libc::c_uint = 0;
    let mut n: libc::c_uint = 0i32 as libc::c_uint;
    qsort_string_vector(
      (*lineedit_ptr_to_statics).matches,
      (*lineedit_ptr_to_statics).num_matches,
    );
    i = 0i32 as libc::c_uint;
    while i
      < (*lineedit_ptr_to_statics)
        .num_matches
        .wrapping_sub(1i32 as libc::c_uint)
    {
      //if (matches[i] && matches[i+1]) { /* paranoia */
      if strcmp(
        *(*lineedit_ptr_to_statics).matches.offset(i as isize),
        *(*lineedit_ptr_to_statics)
          .matches
          .offset(i.wrapping_add(1i32 as libc::c_uint) as isize),
      ) == 0i32
      {
        free(*(*lineedit_ptr_to_statics).matches.offset(i as isize) as *mut libc::c_void);
      //matches[i] = NULL; /* paranoia */
      } else {
        let fresh13 = n;
        n = n.wrapping_add(1);
        let ref mut fresh14 = *(*lineedit_ptr_to_statics).matches.offset(fresh13 as isize);
        *fresh14 = *(*lineedit_ptr_to_statics).matches.offset(i as isize)
      }
      i = i.wrapping_add(1)
      //}
    }
    let fresh15 = n;
    n = n.wrapping_add(1);
    let ref mut fresh16 = *(*lineedit_ptr_to_statics).matches.offset(fresh15 as isize);
    *fresh16 = *(*lineedit_ptr_to_statics).matches.offset(i as isize);
    (*lineedit_ptr_to_statics).num_matches = n
  }
  /* Did we find exactly one match? */
  if (*lineedit_ptr_to_statics).num_matches != 1i32 as libc::c_uint {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char; /* exactly one match */
    beep();
    /* no */
    if (*lineedit_ptr_to_statics).matches.is_null() {
      current_block = 3431305713246613743; /* no matches at all */
    } else {
      /* Find common prefix */
      chosen_match = xstrdup(*(*lineedit_ptr_to_statics).matches.offset(0));
      cp = chosen_match;
      's_245: while *cp != 0 {
        let mut n_0: libc::c_uint = 0;
        n_0 = 1i32 as libc::c_uint;
        while n_0 < (*lineedit_ptr_to_statics).num_matches {
          if *(*(*lineedit_ptr_to_statics).matches.offset(n_0 as isize))
            .offset(cp.wrapping_offset_from(chosen_match) as libc::c_long as isize)
            as libc::c_int
            != *cp as libc::c_int
          {
            break 's_245;
          }
          n_0 = n_0.wrapping_add(1)
        }
        cp = cp.offset(1)
      }
      if cp == chosen_match {
        current_block = 3431305713246613743;
      /* no */
      } else {
        *cp = '\u{0}' as i32 as libc::c_char;
        cp = quote_special_chars(chosen_match);
        free(chosen_match as *mut libc::c_void);
        chosen_match = cp;
        len_found = strlen(chosen_match);
        current_block = 129780949503461575;
      }
    }
  } else {
    /* Next <tab> is not a double-tab */
    *lastWasTab = 0i32 as smallint;
    chosen_match = quote_special_chars(*(*lineedit_ptr_to_statics).matches.offset(0));
    len_found = strlen(chosen_match);
    if *chosen_match.offset(len_found.wrapping_sub(1i32 as libc::c_ulong) as isize) as libc::c_int
      != '/' as i32
    {
      *chosen_match.offset(len_found as isize) = ' ' as i32 as libc::c_char;
      len_found = len_found.wrapping_add(1);
      *chosen_match.offset(len_found as isize) = '\u{0}' as i32 as libc::c_char
    }
    current_block = 129780949503461575;
  }
  match current_block {
    129780949503461575 => {
      /* Use 2nd half of match_buf as scratch space - see (**) */
      let mut command: *mut libc::c_char = match_buf.offset(MAX_LINELEN as libc::c_int as isize);
      let mut len: libc::c_int =
        save_string(command, MAX_LINELEN as libc::c_int as libc::c_uint) as libc::c_int;
      /* Have space to place the match? */
      /* cursor_mb + (len_found - match_pfx_len) + (len - cursor_mb) */
      if (len_found
        .wrapping_sub(match_pfx_len as libc::c_ulong)
        .wrapping_add(len as libc::c_ulong) as libc::c_int)
        < MAX_LINELEN as libc::c_int
      {
        let mut pos: libc::c_int = 0;
        /* save tail */
        strcpy(match_buf, &mut *command.offset(cursor_mb as isize));
        /* where do we want to have cursor after all? */
        strcpy(
          &mut *command.offset(cursor_mb as isize),
          chosen_match.offset(match_pfx_len as isize),
        );
        len = load_string(command) as libc::c_int;
        /* add match and tail */
        sprintf(
          &mut *command.offset(cursor_mb as isize) as *mut libc::c_char,
          b"%s%s\x00" as *const u8 as *const libc::c_char,
          chosen_match.offset(match_pfx_len as isize),
          match_buf,
        );
        (*lineedit_ptr_to_statics).command_len = load_string(command) as libc::c_int;
        /* write out the matched command */
        /* paranoia: load_string can return 0 on conv error,
         * prevent passing pos = (0 - 12) to redraw */
        pos = (*lineedit_ptr_to_statics).command_len - len;
        draw_custom(
          (*lineedit_ptr_to_statics).cmdedit_y as libc::c_int,
          if pos >= 0i32 { pos } else { 0i32 },
          0i32 != 0,
        );
      }
    }
    _ => {}
  }
  /* have unique prefix? */
  free(chosen_match as *mut libc::c_void);
  free(match_buf as *mut libc::c_void);
}
/* FEATURE_TAB_COMPLETION */
#[no_mangle]
pub unsafe extern "C" fn new_line_input_t(mut flags: libc::c_int) -> *mut line_input_t {
  let mut n: *mut line_input_t =
    xzalloc(::std::mem::size_of::<line_input_t>() as libc::c_ulong) as *mut line_input_t;
  (*n).flags = flags;
  (*n).timeout = -1i32;
  (*n).max_history = 255i32 + 0i32;
  return n;
}
#[no_mangle]
pub unsafe extern "C" fn size_from_HISTFILESIZE(mut hp: *const libc::c_char) -> libc::c_uint {
  let mut size: libc::c_int = 255i32 + 0i32;
  if !hp.is_null() {
    size = atoi(hp);
    if size <= 0i32 {
      return 1i32 as libc::c_uint;
    }
    if size > 255i32 + 0i32 {
      return (255i32 + 0i32) as libc::c_uint;
    }
  }
  return size as libc::c_uint;
}
unsafe extern "C" fn save_command_ps_at_cur_history() {
  if *(*lineedit_ptr_to_statics).command_ps.offset(0) != 0i32 {
    let mut cur: libc::c_int = (*(*lineedit_ptr_to_statics).state).cur_history;
    free((*(*lineedit_ptr_to_statics).state).history[cur as usize] as *mut libc::c_void);
    let mut tbuf: [libc::c_char; 1024] = [0; 1024];
    save_string(
      tbuf.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_uint,
    );
    (*(*lineedit_ptr_to_statics).state).history[cur as usize] = xstrdup(tbuf.as_mut_ptr())
  };
}
/* state->flags is already checked to be nonzero */
unsafe extern "C" fn get_previous_history() -> libc::c_int {
  if (*(*lineedit_ptr_to_statics).state).flags & DO_HISTORY as libc::c_int != 0
    && (*(*lineedit_ptr_to_statics).state).cur_history != 0
  {
    save_command_ps_at_cur_history(); /* save the current history line */
    (*(*lineedit_ptr_to_statics).state).cur_history -= 1;
    return 1i32;
  }
  beep();
  return 0i32;
}
unsafe extern "C" fn get_next_history() -> libc::c_int {
  if (*(*lineedit_ptr_to_statics).state).flags & DO_HISTORY as libc::c_int != 0 {
    if (*(*lineedit_ptr_to_statics).state).cur_history
      < (*(*lineedit_ptr_to_statics).state).cnt_history
    {
      save_command_ps_at_cur_history();
      (*(*lineedit_ptr_to_statics).state).cur_history += 1;
      return (*(*lineedit_ptr_to_statics).state).cur_history;
    }
  }
  beep();
  return 0i32;
}
/* Lists command history. Used by shell 'history' builtins */
#[no_mangle]
pub unsafe extern "C" fn show_history(mut st: *const line_input_t) {
  let mut i: libc::c_int = 0;
  if st.is_null() {
    return;
  }
  i = 0i32;
  while i < (*st).cnt_history {
    printf(
      b"%4d %s\n\x00" as *const u8 as *const libc::c_char,
      i,
      (*st).history[i as usize],
    );
    i += 1
  }
}
#[no_mangle]
pub unsafe extern "C" fn free_line_input_t(mut n: *mut line_input_t) {
  let mut i: libc::c_int = (*n).cnt_history;
  while i > 0i32 {
    i -= 1;
    free((*n).history[i as usize] as *mut libc::c_void);
  }
  free(n as *mut libc::c_void);
}
/* We try to ensure that concurrent additions to the history
 * do not overwrite each other.
 * Otherwise shell users get unhappy.
 *
 * History file is trimmed lazily, when it grows several times longer
 * than configured MAX_HISTORY lines.
 */
/* state->flags is already checked to be nonzero */
unsafe extern "C" fn load_history(mut st_parm: *mut line_input_t) {
  let mut temp_h: [*mut libc::c_char; 255] = [0 as *mut libc::c_char; 255];
  let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut fp: *mut FILE = 0 as *mut FILE;
  let mut idx: libc::c_uint = 0;
  let mut i: libc::c_uint = 0;
  let mut line_len: libc::c_uint = 0;
  /* NB: do not trash old history if file can't be opened */
  fp = fopen_for_read((*st_parm).hist_file);
  if !fp.is_null() {
    /* clean up old history */
    idx = (*st_parm).cnt_history as libc::c_uint;
    while idx > 0i32 as libc::c_uint {
      idx = idx.wrapping_sub(1);
      free((*st_parm).history[idx as usize] as *mut libc::c_void);
      (*st_parm).history[idx as usize] = 0 as *mut libc::c_char
    }
    /* fill temp_h[], retaining only last MAX_HISTORY lines */
    memset(
      temp_h.as_mut_ptr() as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<[*mut libc::c_char; 255]>() as libc::c_ulong,
    );
    idx = 0i32 as libc::c_uint;
    (*st_parm).cnt_history_in_file = 0i32 as libc::c_uint;
    loop {
      line = xmalloc_fgetline(fp);
      if line.is_null() {
        break;
      }
      if *line.offset(0) as libc::c_int == '\u{0}' as i32 {
        free(line as *mut libc::c_void);
      } else {
        free(temp_h[idx as usize] as *mut libc::c_void);
        temp_h[idx as usize] = line;
        (*st_parm).cnt_history_in_file = (*st_parm).cnt_history_in_file.wrapping_add(1);
        idx = idx.wrapping_add(1);
        if idx == (*st_parm).max_history as libc::c_uint {
          idx = 0i32 as libc::c_uint
        }
      }
    }
    fclose(fp);
    /* find first non-NULL temp_h[], if any */
    if (*st_parm).cnt_history_in_file != 0 {
      while temp_h[idx as usize].is_null() {
        idx = idx.wrapping_add(1);
        if idx == (*st_parm).max_history as libc::c_uint {
          idx = 0i32 as libc::c_uint
        }
      }
    }
    /* copy temp_h[] to st_parm->history[] */
    i = 0i32 as libc::c_uint; /* paranoia */
    while i < (*st_parm).max_history as libc::c_uint {
      line = temp_h[idx as usize]; /* we (try to) do atomic write */
      if line.is_null() {
        break; /* "wtf?" */
      }
      idx = idx.wrapping_add(1);
      if idx == (*st_parm).max_history as libc::c_uint {
        idx = 0i32 as libc::c_uint
      }
      line_len = strlen(line) as libc::c_uint;
      if line_len >= MAX_LINELEN as libc::c_int as libc::c_uint {
        *line.offset((MAX_LINELEN as libc::c_int - 1i32) as isize) = '\u{0}' as i32 as libc::c_char
      }
      let fresh17 = i;
      i = i.wrapping_add(1);
      (*st_parm).history[fresh17 as usize] = line
    }
    (*st_parm).cnt_history = i as libc::c_int
  };
}
unsafe extern "C" fn save_history(mut str: *mut libc::c_char) {
  let mut fd: libc::c_int = 0;
  let mut len: libc::c_int = 0;
  let mut len2: libc::c_int = 0;
  if (*(*lineedit_ptr_to_statics).state).hist_file.is_null() {
    return;
  }
  fd = open(
    (*(*lineedit_ptr_to_statics).state).hist_file,
    0o1i32 | 0o100i32 | 0o2000i32,
    0o600i32,
  );
  if fd < 0i32 {
    return;
  }
  xlseek(fd, 0i32 as off_t, 2i32);
  len = strlen(str) as libc::c_int;
  *str.offset(len as isize) = '\n' as i32 as libc::c_char;
  len2 = full_write(fd, str as *const libc::c_void, (len + 1i32) as size_t) as libc::c_int;
  *str.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
  close(fd);
  if len2 != len + 1i32 {
    return;
  }
  /* did we write so much that history file needs trimming? */
  (*(*lineedit_ptr_to_statics).state).cnt_history_in_file = (*(*lineedit_ptr_to_statics).state)
    .cnt_history_in_file
    .wrapping_add(1);
  if (*(*lineedit_ptr_to_statics).state).cnt_history_in_file
    > ((*(*lineedit_ptr_to_statics).state).max_history * 4i32) as libc::c_uint
  {
    let mut new_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut st_temp: *mut line_input_t = 0 as *mut line_input_t;
    /* we may have concurrently written entries from others.
     * load them */
    st_temp = new_line_input_t((*(*lineedit_ptr_to_statics).state).flags);
    (*st_temp).hist_file = (*(*lineedit_ptr_to_statics).state).hist_file;
    (*st_temp).max_history = (*(*lineedit_ptr_to_statics).state).max_history;
    load_history(st_temp);
    /* write out temp file and replace hist_file atomically */
    new_name = xasprintf(
      b"%s.%u.new\x00" as *const u8 as *const libc::c_char,
      (*(*lineedit_ptr_to_statics).state).hist_file,
      getpid(),
    );
    fd = open(new_name, 0o1i32 | 0o100i32 | 0o1000i32, 0o600i32);
    if fd >= 0i32 {
      let mut fp: *mut FILE = 0 as *mut FILE;
      let mut i: libc::c_int = 0;
      fp = xfdopen_for_write(fd);
      i = 0i32;
      while i < (*st_temp).cnt_history {
        fprintf(
          fp,
          b"%s\n\x00" as *const u8 as *const libc::c_char,
          (*st_temp).history[i as usize],
        );
        i += 1
      }
      fclose(fp);
      if rename(new_name, (*(*lineedit_ptr_to_statics).state).hist_file) == 0i32 {
        (*(*lineedit_ptr_to_statics).state).cnt_history_in_file =
          (*st_temp).cnt_history as libc::c_uint
      }
    }
    free(new_name as *mut libc::c_void);
    free_line_input_t(st_temp);
  };
}
/* FEATURE_COMMAND_SAVEHISTORY */
unsafe extern "C" fn remember_in_history(mut str: *mut libc::c_char) {
  let mut i: libc::c_int = 0;
  if (*(*lineedit_ptr_to_statics).state).flags & DO_HISTORY as libc::c_int == 0 {
    return;
  }
  if *str.offset(0) as libc::c_int == '\u{0}' as i32 {
    return;
  }
  i = (*(*lineedit_ptr_to_statics).state).cnt_history;
  /* Don't save dupes */
  if i != 0
    && strcmp(
      (*(*lineedit_ptr_to_statics).state).history[(i - 1i32) as usize],
      str,
    ) == 0i32
  {
    return;
  } /* redundant, paranoia */
  free(
    (*(*lineedit_ptr_to_statics).state).history
      [(*(*lineedit_ptr_to_statics).state).max_history as usize] as *mut libc::c_void,
  ); /* redundant, paranoia */
  (*(*lineedit_ptr_to_statics).state).history
    [(*(*lineedit_ptr_to_statics).state).max_history as usize] = 0 as *mut libc::c_char;
  /* If history[] is full, remove the oldest command */
  /* we need to keep history[state->max_history] empty, hence >=, not > */
  if i >= (*(*lineedit_ptr_to_statics).state).max_history {
    free((*(*lineedit_ptr_to_statics).state).history[0] as *mut libc::c_void);
    i = 0i32;
    while i < (*(*lineedit_ptr_to_statics).state).max_history - 1i32 {
      (*(*lineedit_ptr_to_statics).state).history[i as usize] =
        (*(*lineedit_ptr_to_statics).state).history[(i + 1i32) as usize];
      i += 1
    }
    /* i == state->max_history-1 */
  }
  /* i <= state->max_history-1 */
  let fresh18 = i;
  i = i + 1;
  (*(*lineedit_ptr_to_statics).state).history[fresh18 as usize] = xstrdup(str);
  /* i <= state->max_history */
  (*(*lineedit_ptr_to_statics).state).cur_history = i;
  (*(*lineedit_ptr_to_statics).state).cnt_history = i;
  save_history(str);
}
/* MAX_HISTORY == 0 */
/* MAX_HISTORY */
/* Modelled after bash 4.0 behavior of Ctrl-<arrow> */
unsafe extern "C" fn ctrl_left() {
  let mut command: *mut wchar_t = (*lineedit_ptr_to_statics).command_ps;
  loop {
    let mut c: wchar_t = 0;
    input_backward(1i32 as libc::c_uint);
    if (*lineedit_ptr_to_statics).cursor == 0i32 as libc::c_uint {
      break;
    }
    c = *command.offset((*lineedit_ptr_to_statics).cursor as isize);
    if !(c != ' ' as i32 && !BB_ispunct(c)) {
      continue;
    }
    loop
    /* we reached a "word" delimited by spaces/punct.
     * go to its beginning */
    {
      c = *command.offset(
        (*lineedit_ptr_to_statics)
          .cursor
          .wrapping_sub(1i32 as libc::c_uint) as isize,
      );
      if c == ' ' as i32 || BB_ispunct(c) as libc::c_int != 0 {
        break;
      }
      input_backward(1i32 as libc::c_uint);
      if (*lineedit_ptr_to_statics).cursor == 0i32 as libc::c_uint {
        break;
      }
    }
    break;
  }
}
unsafe extern "C" fn ctrl_right() {
  let mut command: *mut wchar_t = (*lineedit_ptr_to_statics).command_ps;
  loop {
    let mut c: wchar_t = 0;
    c = *command.offset((*lineedit_ptr_to_statics).cursor as isize);
    if c == 0i32 {
      break;
    }
    if c != ' ' as i32 && !BB_ispunct(c) {
      loop
      /* we reached a "word" delimited by spaces/punct.
       * go to its end + 1 */
      {
        input_forward();
        c = *command.offset((*lineedit_ptr_to_statics).cursor as isize);
        if c == 0i32 || c == ' ' as i32 || BB_ispunct(c) as libc::c_int != 0 {
          break;
        }
      }
      break;
    } else {
      input_forward();
    }
  }
}
/*
 * read_line_input and its helpers
 */
/* Note about multi-line PS1 (e.g. "\n\w \u@\h\n> ") and prompt redrawing:
 *
 * If the prompt has any newlines, after we print it once we use only its last
 * line to redraw in-place, which makes it simpler to calculate how many lines
 * we should move the cursor up to align the redraw (cmdedit_y). The earlier
 * prompt lines just stay on screen and we redraw below them.
 *
 * Use cases for all prompt lines beyond the initial draw:
 * - After clear-screen (^L) or after displaying tab-completion choices, we
 *   print the full prompt, as it isn't redrawn in-place.
 * - During terminal resize we could try to redraw all lines, but we don't,
 *   because it requires delicate alignment, it's good enough with only the
 *   last line, and doing it wrong is arguably worse than not doing it at all.
 *
 * Terminology wise, if it doesn't mention "full", then it means the last/sole
 * prompt line. We use the prompt (last/sole line) while redrawing in-place,
 * and the full where we need a fresh one unrelated to an earlier position.
 *
 * If PS1 is not multiline, the last/sole line and the full are the same string.
 */
/* Called just once at read_line_input() init time */
unsafe extern "C" fn parse_and_put_prompt(mut prmt_ptr: *const libc::c_char) {
  let mut prmt_size: libc::c_int = 0i32;
  let mut prmt_mem_ptr: *mut libc::c_char = xzalloc(1i32 as size_t) as *mut libc::c_char;
  let mut cwd_buf: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut flg_not_length: libc::c_char = '[' as i32 as libc::c_char;
  let mut cbuf: [libc::c_char; 2] = [0; 2];
  /*cmdedit_prmt_len = 0; - already is */
  cbuf[1] = '\u{0}' as i32 as libc::c_char; /* never changes */
  let mut current_block_52: u64; /* while */
  while *prmt_ptr != 0 {
    let mut timebuf: [libc::c_char; 9] = [0; 9]; /* if */
    let mut free_me: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    pbuf = cbuf.as_mut_ptr();
    let fresh19 = prmt_ptr;
    prmt_ptr = prmt_ptr.offset(1);
    c = *fresh19;
    if c as libc::c_int == '\\' as i32 {
      let mut cp: *const libc::c_char = 0 as *const libc::c_char;
      let mut l: libc::c_int = 0;
      /* if */
      /*
       * Supported via bb_process_escape_sequence:
       * \a	ASCII bell character (07)
       * \e	ASCII escape character (033)
       * \n	newline
       * \r	carriage return
       * \\	backslash
       * \nnn	char with octal code nnn
       * Supported:
       * \$	if the effective UID is 0, a #, otherwise a $
       * \w	current working directory, with $HOME abbreviated with a tilde
       *	Note: we do not support $PROMPT_DIRTRIM=n feature
       * \W	basename of the current working directory, with $HOME abbreviated with a tilde
       * \h	hostname up to the first '.'
       * \H	hostname
       * \u	username
       * \[	begin a sequence of non-printing characters
       * \]	end a sequence of non-printing characters
       * \T	current time in 12-hour HH:MM:SS format
       * \@	current time in 12-hour am/pm format
       * \A	current time in 24-hour HH:MM format
       * \t	current time in 24-hour HH:MM:SS format
       *	(all of the above work as \A)
       * Not supported:
       * \!	history number of this command
       * \#	command number of this command
       * \j	number of jobs currently managed by the shell
       * \l	basename of the shell's terminal device name
       * \s	name of the shell, the basename of $0 (the portion following the final slash)
       * \V	release of bash, version + patch level (e.g., 2.00.0)
       * \d	date in "Weekday Month Date" format (e.g., "Tue May 26")
       * \D{format}
       *	format is passed to strftime(3).
       *	An empty format results in a locale-specific time representation.
       *	The braces are required.
       * Mishandled by bb_process_escape_sequence:
       * \v	version of bash (e.g., 2.00)
       */
      cp = prmt_ptr;
      c = *cp;
      if c as libc::c_int != 't' as i32 {
        /* don't treat \t as tab */
        c = bb_process_escape_sequence(&mut prmt_ptr)
      }
      if prmt_ptr == cp {
        if *cp as libc::c_int == '\u{0}' as i32 {
          break;
          /* switch */
        }
        let fresh20 = prmt_ptr;
        prmt_ptr = prmt_ptr.offset(1);
        c = *fresh20;
        match c as libc::c_int {
          117 => {
            current_block_52 = 9642695190694411401;
            match current_block_52 {
              18087889531735461995 => {
                free_me = safe_gethostname();
                pbuf = free_me;
                if c as libc::c_int == 'h' as i32 {
                  *strchrnul(pbuf, '.' as i32).offset(0) = '\u{0}' as i32 as libc::c_char
                }
                current_block_52 = 8869332144787829186;
              }
              11214990802921196617 =>
              /* current dir */
              /* basename of cur dir */
              {
                if cwd_buf.is_null() {
                  cwd_buf = xrealloc_getcwd_or_warn(0 as *mut libc::c_char);
                  if cwd_buf.is_null() {
                    cwd_buf = bb_msg_unknown.as_ptr() as *mut libc::c_char
                  } else if *(*lineedit_ptr_to_statics).home_pwd_buf.offset(0) != 0 {
                    let mut after_home_user: *mut libc::c_char = 0 as *mut libc::c_char;
                    /* /home/user[/something] -> ~[/something] */
                    after_home_user =
                      is_prefixed_with(cwd_buf, (*lineedit_ptr_to_statics).home_pwd_buf);
                    if !after_home_user.is_null()
                      && (*after_home_user as libc::c_int == '/' as i32
                        || *after_home_user as libc::c_int == '\u{0}' as i32)
                    {
                      *cwd_buf.offset(0) = '~' as i32 as libc::c_char;
                      overlapping_strcpy(cwd_buf.offset(1), after_home_user);
                    }
                  }
                }
                pbuf = cwd_buf;
                if c as libc::c_int == 'w' as i32 {
                  current_block_52 = 8869332144787829186;
                } else {
                  cp = strrchr(pbuf, '/' as i32);
                  if !cp.is_null() {
                    pbuf = (cp as *mut libc::c_char).offset(1)
                  }
                  current_block_52 = 8869332144787829186;
                }
              }
              12381812505308290051 =>
              // bb_process_escape_sequence does this now:
              //				case 'e': case 'E':     /* \e \E = \033 */
              //					c = '\033';
              //					break;
              {
                let mut buf2: [libc::c_char; 4] = [0; 4];
                l = 0i32;
                while l < 3i32 {
                  let mut h: libc::c_uint = 0;
                  let fresh21 = l;
                  l = l + 1;
                  buf2[fresh21 as usize] = *prmt_ptr;
                  buf2[l as usize] = '\u{0}' as i32 as libc::c_char;
                  h = strtoul(buf2.as_mut_ptr(), &mut pbuf, 16i32) as libc::c_uint;
                  if h > (127i32 * 2i32 + 1i32) as libc::c_uint
                    || (pbuf.wrapping_offset_from(buf2.as_mut_ptr()) as libc::c_long)
                      < l as libc::c_long
                  {
                    l -= 1;
                    buf2[l as usize] = '\u{0}' as i32 as libc::c_char;
                    break;
                  } else {
                    prmt_ptr = prmt_ptr.offset(1)
                  }
                }
                c = strtoul(buf2.as_mut_ptr(), 0 as *mut *mut libc::c_char, 16i32) as libc::c_char;
                if c as libc::c_int == 0i32 {
                  c = '?' as i32 as libc::c_char
                }
                pbuf = cbuf.as_mut_ptr();
                current_block_52 = 8869332144787829186;
              }
              10485851455608404399 => {
                if c as libc::c_int == flg_not_length as libc::c_int {
                  /* Toggle '['/']' hex 5b/5d */
                  flg_not_length = (flg_not_length as libc::c_int ^ 6i32) as libc::c_char;
                  continue;
                } else {
                  current_block_52 = 8869332144787829186;
                }
              }
              9642695190694411401 => {
                pbuf = if !(*lineedit_ptr_to_statics).user_buf.is_null() {
                  (*lineedit_ptr_to_statics).user_buf
                } else {
                  b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
                };
                current_block_52 = 8869332144787829186;
              }
              14477407255285059806 => {
                c = if geteuid() == 0i32 as libc::c_uint {
                  '#' as i32
                } else {
                  '$' as i32
                } as libc::c_char;
                current_block_52 = 8869332144787829186;
              }
              12410170708601199743 =>
              /* 12-hour HH:MM:SS format */
              {
                current_block_52 = 16491562349523041571;
              }
              _ => {}
            }
            match current_block_52 {
              8869332144787829186 => {}
              _ => {
                match current_block_52 {
                  16491562349523041571 =>
                    /* 12-hour am/pm format */
                    {}
                  _ => {}
                }
                /* 24-hour HH:MM format */
                /* 24-hour HH:MM:SS format */
                /* We show all of them as 24-hour HH:MM */
                *strftime_HHMMSS(
                  timebuf.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as libc::c_uint,
                  0 as *mut time_t,
                )
                .offset(-3i32 as isize) = '\u{0}' as i32 as libc::c_char;
                pbuf = timebuf.as_mut_ptr()
              }
            }
          }
          72 | 104 => {
            current_block_52 = 18087889531735461995;
            match current_block_52 {
              18087889531735461995 => {
                free_me = safe_gethostname();
                pbuf = free_me;
                if c as libc::c_int == 'h' as i32 {
                  *strchrnul(pbuf, '.' as i32).offset(0) = '\u{0}' as i32 as libc::c_char
                }
                current_block_52 = 8869332144787829186;
              }
              11214990802921196617 => {
                if cwd_buf.is_null() {
                  cwd_buf = xrealloc_getcwd_or_warn(0 as *mut libc::c_char);
                  if cwd_buf.is_null() {
                    cwd_buf = bb_msg_unknown.as_ptr() as *mut libc::c_char
                  } else if *(*lineedit_ptr_to_statics).home_pwd_buf.offset(0) != 0 {
                    let mut after_home_user: *mut libc::c_char = 0 as *mut libc::c_char;
                    after_home_user =
                      is_prefixed_with(cwd_buf, (*lineedit_ptr_to_statics).home_pwd_buf);
                    if !after_home_user.is_null()
                      && (*after_home_user as libc::c_int == '/' as i32
                        || *after_home_user as libc::c_int == '\u{0}' as i32)
                    {
                      *cwd_buf.offset(0) = '~' as i32 as libc::c_char;
                      overlapping_strcpy(cwd_buf.offset(1), after_home_user);
                    }
                  }
                }
                pbuf = cwd_buf;
                if c as libc::c_int == 'w' as i32 {
                  current_block_52 = 8869332144787829186;
                } else {
                  cp = strrchr(pbuf, '/' as i32);
                  if !cp.is_null() {
                    pbuf = (cp as *mut libc::c_char).offset(1)
                  }
                  current_block_52 = 8869332144787829186;
                }
              }
              12381812505308290051 => {
                let mut buf2: [libc::c_char; 4] = [0; 4];
                l = 0i32;
                while l < 3i32 {
                  let mut h: libc::c_uint = 0;
                  let fresh21 = l;
                  l = l + 1;
                  buf2[fresh21 as usize] = *prmt_ptr;
                  buf2[l as usize] = '\u{0}' as i32 as libc::c_char;
                  h = strtoul(buf2.as_mut_ptr(), &mut pbuf, 16i32) as libc::c_uint;
                  if h > (127i32 * 2i32 + 1i32) as libc::c_uint
                    || (pbuf.wrapping_offset_from(buf2.as_mut_ptr()) as libc::c_long)
                      < l as libc::c_long
                  {
                    l -= 1;
                    buf2[l as usize] = '\u{0}' as i32 as libc::c_char;
                    break;
                  } else {
                    prmt_ptr = prmt_ptr.offset(1)
                  }
                }
                c = strtoul(buf2.as_mut_ptr(), 0 as *mut *mut libc::c_char, 16i32) as libc::c_char;
                if c as libc::c_int == 0i32 {
                  c = '?' as i32 as libc::c_char
                }
                pbuf = cbuf.as_mut_ptr();
                current_block_52 = 8869332144787829186;
              }
              10485851455608404399 => {
                if c as libc::c_int == flg_not_length as libc::c_int {
                  flg_not_length = (flg_not_length as libc::c_int ^ 6i32) as libc::c_char;
                  continue;
                } else {
                  current_block_52 = 8869332144787829186;
                }
              }
              9642695190694411401 => {
                pbuf = if !(*lineedit_ptr_to_statics).user_buf.is_null() {
                  (*lineedit_ptr_to_statics).user_buf
                } else {
                  b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
                };
                current_block_52 = 8869332144787829186;
              }
              14477407255285059806 => {
                c = if geteuid() == 0i32 as libc::c_uint {
                  '#' as i32
                } else {
                  '$' as i32
                } as libc::c_char;
                current_block_52 = 8869332144787829186;
              }
              12410170708601199743 => {
                current_block_52 = 16491562349523041571;
              }
              _ => {}
            }
            match current_block_52 {
              8869332144787829186 => {}
              _ => {
                match current_block_52 {
                  16491562349523041571 => {}
                  _ => {}
                }
                *strftime_HHMMSS(
                  timebuf.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as libc::c_uint,
                  0 as *mut time_t,
                )
                .offset(-3i32 as isize) = '\u{0}' as i32 as libc::c_char;
                pbuf = timebuf.as_mut_ptr()
              }
            }
          }
          36 => {
            current_block_52 = 14477407255285059806;
            match current_block_52 {
              18087889531735461995 => {
                free_me = safe_gethostname();
                pbuf = free_me;
                if c as libc::c_int == 'h' as i32 {
                  *strchrnul(pbuf, '.' as i32).offset(0) = '\u{0}' as i32 as libc::c_char
                }
                current_block_52 = 8869332144787829186;
              }
              11214990802921196617 => {
                if cwd_buf.is_null() {
                  cwd_buf = xrealloc_getcwd_or_warn(0 as *mut libc::c_char);
                  if cwd_buf.is_null() {
                    cwd_buf = bb_msg_unknown.as_ptr() as *mut libc::c_char
                  } else if *(*lineedit_ptr_to_statics).home_pwd_buf.offset(0) != 0 {
                    let mut after_home_user: *mut libc::c_char = 0 as *mut libc::c_char;
                    after_home_user =
                      is_prefixed_with(cwd_buf, (*lineedit_ptr_to_statics).home_pwd_buf);
                    if !after_home_user.is_null()
                      && (*after_home_user as libc::c_int == '/' as i32
                        || *after_home_user as libc::c_int == '\u{0}' as i32)
                    {
                      *cwd_buf.offset(0) = '~' as i32 as libc::c_char;
                      overlapping_strcpy(cwd_buf.offset(1), after_home_user);
                    }
                  }
                }
                pbuf = cwd_buf;
                if c as libc::c_int == 'w' as i32 {
                  current_block_52 = 8869332144787829186;
                } else {
                  cp = strrchr(pbuf, '/' as i32);
                  if !cp.is_null() {
                    pbuf = (cp as *mut libc::c_char).offset(1)
                  }
                  current_block_52 = 8869332144787829186;
                }
              }
              12381812505308290051 => {
                let mut buf2: [libc::c_char; 4] = [0; 4];
                l = 0i32;
                while l < 3i32 {
                  let mut h: libc::c_uint = 0;
                  let fresh21 = l;
                  l = l + 1;
                  buf2[fresh21 as usize] = *prmt_ptr;
                  buf2[l as usize] = '\u{0}' as i32 as libc::c_char;
                  h = strtoul(buf2.as_mut_ptr(), &mut pbuf, 16i32) as libc::c_uint;
                  if h > (127i32 * 2i32 + 1i32) as libc::c_uint
                    || (pbuf.wrapping_offset_from(buf2.as_mut_ptr()) as libc::c_long)
                      < l as libc::c_long
                  {
                    l -= 1;
                    buf2[l as usize] = '\u{0}' as i32 as libc::c_char;
                    break;
                  } else {
                    prmt_ptr = prmt_ptr.offset(1)
                  }
                }
                c = strtoul(buf2.as_mut_ptr(), 0 as *mut *mut libc::c_char, 16i32) as libc::c_char;
                if c as libc::c_int == 0i32 {
                  c = '?' as i32 as libc::c_char
                }
                pbuf = cbuf.as_mut_ptr();
                current_block_52 = 8869332144787829186;
              }
              10485851455608404399 => {
                if c as libc::c_int == flg_not_length as libc::c_int {
                  flg_not_length = (flg_not_length as libc::c_int ^ 6i32) as libc::c_char;
                  continue;
                } else {
                  current_block_52 = 8869332144787829186;
                }
              }
              9642695190694411401 => {
                pbuf = if !(*lineedit_ptr_to_statics).user_buf.is_null() {
                  (*lineedit_ptr_to_statics).user_buf
                } else {
                  b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
                };
                current_block_52 = 8869332144787829186;
              }
              14477407255285059806 => {
                c = if geteuid() == 0i32 as libc::c_uint {
                  '#' as i32
                } else {
                  '$' as i32
                } as libc::c_char;
                current_block_52 = 8869332144787829186;
              }
              12410170708601199743 => {
                current_block_52 = 16491562349523041571;
              }
              _ => {}
            }
            match current_block_52 {
              8869332144787829186 => {}
              _ => {
                match current_block_52 {
                  16491562349523041571 => {}
                  _ => {}
                }
                *strftime_HHMMSS(
                  timebuf.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as libc::c_uint,
                  0 as *mut time_t,
                )
                .offset(-3i32 as isize) = '\u{0}' as i32 as libc::c_char;
                pbuf = timebuf.as_mut_ptr()
              }
            }
          }
          84 => {
            current_block_52 = 12410170708601199743;
            match current_block_52 {
              18087889531735461995 => {
                free_me = safe_gethostname();
                pbuf = free_me;
                if c as libc::c_int == 'h' as i32 {
                  *strchrnul(pbuf, '.' as i32).offset(0) = '\u{0}' as i32 as libc::c_char
                }
                current_block_52 = 8869332144787829186;
              }
              11214990802921196617 => {
                if cwd_buf.is_null() {
                  cwd_buf = xrealloc_getcwd_or_warn(0 as *mut libc::c_char);
                  if cwd_buf.is_null() {
                    cwd_buf = bb_msg_unknown.as_ptr() as *mut libc::c_char
                  } else if *(*lineedit_ptr_to_statics).home_pwd_buf.offset(0) != 0 {
                    let mut after_home_user: *mut libc::c_char = 0 as *mut libc::c_char;
                    after_home_user =
                      is_prefixed_with(cwd_buf, (*lineedit_ptr_to_statics).home_pwd_buf);
                    if !after_home_user.is_null()
                      && (*after_home_user as libc::c_int == '/' as i32
                        || *after_home_user as libc::c_int == '\u{0}' as i32)
                    {
                      *cwd_buf.offset(0) = '~' as i32 as libc::c_char;
                      overlapping_strcpy(cwd_buf.offset(1), after_home_user);
                    }
                  }
                }
                pbuf = cwd_buf;
                if c as libc::c_int == 'w' as i32 {
                  current_block_52 = 8869332144787829186;
                } else {
                  cp = strrchr(pbuf, '/' as i32);
                  if !cp.is_null() {
                    pbuf = (cp as *mut libc::c_char).offset(1)
                  }
                  current_block_52 = 8869332144787829186;
                }
              }
              12381812505308290051 => {
                let mut buf2: [libc::c_char; 4] = [0; 4];
                l = 0i32;
                while l < 3i32 {
                  let mut h: libc::c_uint = 0;
                  let fresh21 = l;
                  l = l + 1;
                  buf2[fresh21 as usize] = *prmt_ptr;
                  buf2[l as usize] = '\u{0}' as i32 as libc::c_char;
                  h = strtoul(buf2.as_mut_ptr(), &mut pbuf, 16i32) as libc::c_uint;
                  if h > (127i32 * 2i32 + 1i32) as libc::c_uint
                    || (pbuf.wrapping_offset_from(buf2.as_mut_ptr()) as libc::c_long)
                      < l as libc::c_long
                  {
                    l -= 1;
                    buf2[l as usize] = '\u{0}' as i32 as libc::c_char;
                    break;
                  } else {
                    prmt_ptr = prmt_ptr.offset(1)
                  }
                }
                c = strtoul(buf2.as_mut_ptr(), 0 as *mut *mut libc::c_char, 16i32) as libc::c_char;
                if c as libc::c_int == 0i32 {
                  c = '?' as i32 as libc::c_char
                }
                pbuf = cbuf.as_mut_ptr();
                current_block_52 = 8869332144787829186;
              }
              10485851455608404399 => {
                if c as libc::c_int == flg_not_length as libc::c_int {
                  flg_not_length = (flg_not_length as libc::c_int ^ 6i32) as libc::c_char;
                  continue;
                } else {
                  current_block_52 = 8869332144787829186;
                }
              }
              9642695190694411401 => {
                pbuf = if !(*lineedit_ptr_to_statics).user_buf.is_null() {
                  (*lineedit_ptr_to_statics).user_buf
                } else {
                  b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
                };
                current_block_52 = 8869332144787829186;
              }
              14477407255285059806 => {
                c = if geteuid() == 0i32 as libc::c_uint {
                  '#' as i32
                } else {
                  '$' as i32
                } as libc::c_char;
                current_block_52 = 8869332144787829186;
              }
              12410170708601199743 => {
                current_block_52 = 16491562349523041571;
              }
              _ => {}
            }
            match current_block_52 {
              8869332144787829186 => {}
              _ => {
                match current_block_52 {
                  16491562349523041571 => {}
                  _ => {}
                }
                *strftime_HHMMSS(
                  timebuf.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as libc::c_uint,
                  0 as *mut time_t,
                )
                .offset(-3i32 as isize) = '\u{0}' as i32 as libc::c_char;
                pbuf = timebuf.as_mut_ptr()
              }
            }
          }
          64 => {
            current_block_52 = 16491562349523041571;
            match current_block_52 {
              18087889531735461995 => {
                free_me = safe_gethostname();
                pbuf = free_me;
                if c as libc::c_int == 'h' as i32 {
                  *strchrnul(pbuf, '.' as i32).offset(0) = '\u{0}' as i32 as libc::c_char
                }
                current_block_52 = 8869332144787829186;
              }
              11214990802921196617 => {
                if cwd_buf.is_null() {
                  cwd_buf = xrealloc_getcwd_or_warn(0 as *mut libc::c_char);
                  if cwd_buf.is_null() {
                    cwd_buf = bb_msg_unknown.as_ptr() as *mut libc::c_char
                  } else if *(*lineedit_ptr_to_statics).home_pwd_buf.offset(0) != 0 {
                    let mut after_home_user: *mut libc::c_char = 0 as *mut libc::c_char;
                    after_home_user =
                      is_prefixed_with(cwd_buf, (*lineedit_ptr_to_statics).home_pwd_buf);
                    if !after_home_user.is_null()
                      && (*after_home_user as libc::c_int == '/' as i32
                        || *after_home_user as libc::c_int == '\u{0}' as i32)
                    {
                      *cwd_buf.offset(0) = '~' as i32 as libc::c_char;
                      overlapping_strcpy(cwd_buf.offset(1), after_home_user);
                    }
                  }
                }
                pbuf = cwd_buf;
                if c as libc::c_int == 'w' as i32 {
                  current_block_52 = 8869332144787829186;
                } else {
                  cp = strrchr(pbuf, '/' as i32);
                  if !cp.is_null() {
                    pbuf = (cp as *mut libc::c_char).offset(1)
                  }
                  current_block_52 = 8869332144787829186;
                }
              }
              12381812505308290051 => {
                let mut buf2: [libc::c_char; 4] = [0; 4];
                l = 0i32;
                while l < 3i32 {
                  let mut h: libc::c_uint = 0;
                  let fresh21 = l;
                  l = l + 1;
                  buf2[fresh21 as usize] = *prmt_ptr;
                  buf2[l as usize] = '\u{0}' as i32 as libc::c_char;
                  h = strtoul(buf2.as_mut_ptr(), &mut pbuf, 16i32) as libc::c_uint;
                  if h > (127i32 * 2i32 + 1i32) as libc::c_uint
                    || (pbuf.wrapping_offset_from(buf2.as_mut_ptr()) as libc::c_long)
                      < l as libc::c_long
                  {
                    l -= 1;
                    buf2[l as usize] = '\u{0}' as i32 as libc::c_char;
                    break;
                  } else {
                    prmt_ptr = prmt_ptr.offset(1)
                  }
                }
                c = strtoul(buf2.as_mut_ptr(), 0 as *mut *mut libc::c_char, 16i32) as libc::c_char;
                if c as libc::c_int == 0i32 {
                  c = '?' as i32 as libc::c_char
                }
                pbuf = cbuf.as_mut_ptr();
                current_block_52 = 8869332144787829186;
              }
              10485851455608404399 => {
                if c as libc::c_int == flg_not_length as libc::c_int {
                  flg_not_length = (flg_not_length as libc::c_int ^ 6i32) as libc::c_char;
                  continue;
                } else {
                  current_block_52 = 8869332144787829186;
                }
              }
              9642695190694411401 => {
                pbuf = if !(*lineedit_ptr_to_statics).user_buf.is_null() {
                  (*lineedit_ptr_to_statics).user_buf
                } else {
                  b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
                };
                current_block_52 = 8869332144787829186;
              }
              14477407255285059806 => {
                c = if geteuid() == 0i32 as libc::c_uint {
                  '#' as i32
                } else {
                  '$' as i32
                } as libc::c_char;
                current_block_52 = 8869332144787829186;
              }
              12410170708601199743 => {
                current_block_52 = 16491562349523041571;
              }
              _ => {}
            }
            match current_block_52 {
              8869332144787829186 => {}
              _ => {
                match current_block_52 {
                  16491562349523041571 => {}
                  _ => {}
                }
                *strftime_HHMMSS(
                  timebuf.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as libc::c_uint,
                  0 as *mut time_t,
                )
                .offset(-3i32 as isize) = '\u{0}' as i32 as libc::c_char;
                pbuf = timebuf.as_mut_ptr()
              }
            }
          }
          65 | 116 => {
            current_block_52 = 10426230885943900852;
            match current_block_52 {
              18087889531735461995 => {
                free_me = safe_gethostname();
                pbuf = free_me;
                if c as libc::c_int == 'h' as i32 {
                  *strchrnul(pbuf, '.' as i32).offset(0) = '\u{0}' as i32 as libc::c_char
                }
                current_block_52 = 8869332144787829186;
              }
              11214990802921196617 => {
                if cwd_buf.is_null() {
                  cwd_buf = xrealloc_getcwd_or_warn(0 as *mut libc::c_char);
                  if cwd_buf.is_null() {
                    cwd_buf = bb_msg_unknown.as_ptr() as *mut libc::c_char
                  } else if *(*lineedit_ptr_to_statics).home_pwd_buf.offset(0) != 0 {
                    let mut after_home_user: *mut libc::c_char = 0 as *mut libc::c_char;
                    after_home_user =
                      is_prefixed_with(cwd_buf, (*lineedit_ptr_to_statics).home_pwd_buf);
                    if !after_home_user.is_null()
                      && (*after_home_user as libc::c_int == '/' as i32
                        || *after_home_user as libc::c_int == '\u{0}' as i32)
                    {
                      *cwd_buf.offset(0) = '~' as i32 as libc::c_char;
                      overlapping_strcpy(cwd_buf.offset(1), after_home_user);
                    }
                  }
                }
                pbuf = cwd_buf;
                if c as libc::c_int == 'w' as i32 {
                  current_block_52 = 8869332144787829186;
                } else {
                  cp = strrchr(pbuf, '/' as i32);
                  if !cp.is_null() {
                    pbuf = (cp as *mut libc::c_char).offset(1)
                  }
                  current_block_52 = 8869332144787829186;
                }
              }
              12381812505308290051 => {
                let mut buf2: [libc::c_char; 4] = [0; 4];
                l = 0i32;
                while l < 3i32 {
                  let mut h: libc::c_uint = 0;
                  let fresh21 = l;
                  l = l + 1;
                  buf2[fresh21 as usize] = *prmt_ptr;
                  buf2[l as usize] = '\u{0}' as i32 as libc::c_char;
                  h = strtoul(buf2.as_mut_ptr(), &mut pbuf, 16i32) as libc::c_uint;
                  if h > (127i32 * 2i32 + 1i32) as libc::c_uint
                    || (pbuf.wrapping_offset_from(buf2.as_mut_ptr()) as libc::c_long)
                      < l as libc::c_long
                  {
                    l -= 1;
                    buf2[l as usize] = '\u{0}' as i32 as libc::c_char;
                    break;
                  } else {
                    prmt_ptr = prmt_ptr.offset(1)
                  }
                }
                c = strtoul(buf2.as_mut_ptr(), 0 as *mut *mut libc::c_char, 16i32) as libc::c_char;
                if c as libc::c_int == 0i32 {
                  c = '?' as i32 as libc::c_char
                }
                pbuf = cbuf.as_mut_ptr();
                current_block_52 = 8869332144787829186;
              }
              10485851455608404399 => {
                if c as libc::c_int == flg_not_length as libc::c_int {
                  flg_not_length = (flg_not_length as libc::c_int ^ 6i32) as libc::c_char;
                  continue;
                } else {
                  current_block_52 = 8869332144787829186;
                }
              }
              9642695190694411401 => {
                pbuf = if !(*lineedit_ptr_to_statics).user_buf.is_null() {
                  (*lineedit_ptr_to_statics).user_buf
                } else {
                  b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
                };
                current_block_52 = 8869332144787829186;
              }
              14477407255285059806 => {
                c = if geteuid() == 0i32 as libc::c_uint {
                  '#' as i32
                } else {
                  '$' as i32
                } as libc::c_char;
                current_block_52 = 8869332144787829186;
              }
              12410170708601199743 => {
                current_block_52 = 16491562349523041571;
              }
              _ => {}
            }
            match current_block_52 {
              8869332144787829186 => {}
              _ => {
                match current_block_52 {
                  16491562349523041571 => {}
                  _ => {}
                }
                *strftime_HHMMSS(
                  timebuf.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as libc::c_uint,
                  0 as *mut time_t,
                )
                .offset(-3i32 as isize) = '\u{0}' as i32 as libc::c_char;
                pbuf = timebuf.as_mut_ptr()
              }
            }
          }
          119 | 87 => {
            current_block_52 = 11214990802921196617;
            match current_block_52 {
              18087889531735461995 => {
                free_me = safe_gethostname();
                pbuf = free_me;
                if c as libc::c_int == 'h' as i32 {
                  *strchrnul(pbuf, '.' as i32).offset(0) = '\u{0}' as i32 as libc::c_char
                }
                current_block_52 = 8869332144787829186;
              }
              11214990802921196617 => {
                if cwd_buf.is_null() {
                  cwd_buf = xrealloc_getcwd_or_warn(0 as *mut libc::c_char);
                  if cwd_buf.is_null() {
                    cwd_buf = bb_msg_unknown.as_ptr() as *mut libc::c_char
                  } else if *(*lineedit_ptr_to_statics).home_pwd_buf.offset(0) != 0 {
                    let mut after_home_user: *mut libc::c_char = 0 as *mut libc::c_char;
                    after_home_user =
                      is_prefixed_with(cwd_buf, (*lineedit_ptr_to_statics).home_pwd_buf);
                    if !after_home_user.is_null()
                      && (*after_home_user as libc::c_int == '/' as i32
                        || *after_home_user as libc::c_int == '\u{0}' as i32)
                    {
                      *cwd_buf.offset(0) = '~' as i32 as libc::c_char;
                      overlapping_strcpy(cwd_buf.offset(1), after_home_user);
                    }
                  }
                }
                pbuf = cwd_buf;
                if c as libc::c_int == 'w' as i32 {
                  current_block_52 = 8869332144787829186;
                } else {
                  cp = strrchr(pbuf, '/' as i32);
                  if !cp.is_null() {
                    pbuf = (cp as *mut libc::c_char).offset(1)
                  }
                  current_block_52 = 8869332144787829186;
                }
              }
              12381812505308290051 => {
                let mut buf2: [libc::c_char; 4] = [0; 4];
                l = 0i32;
                while l < 3i32 {
                  let mut h: libc::c_uint = 0;
                  let fresh21 = l;
                  l = l + 1;
                  buf2[fresh21 as usize] = *prmt_ptr;
                  buf2[l as usize] = '\u{0}' as i32 as libc::c_char;
                  h = strtoul(buf2.as_mut_ptr(), &mut pbuf, 16i32) as libc::c_uint;
                  if h > (127i32 * 2i32 + 1i32) as libc::c_uint
                    || (pbuf.wrapping_offset_from(buf2.as_mut_ptr()) as libc::c_long)
                      < l as libc::c_long
                  {
                    l -= 1;
                    buf2[l as usize] = '\u{0}' as i32 as libc::c_char;
                    break;
                  } else {
                    prmt_ptr = prmt_ptr.offset(1)
                  }
                }
                c = strtoul(buf2.as_mut_ptr(), 0 as *mut *mut libc::c_char, 16i32) as libc::c_char;
                if c as libc::c_int == 0i32 {
                  c = '?' as i32 as libc::c_char
                }
                pbuf = cbuf.as_mut_ptr();
                current_block_52 = 8869332144787829186;
              }
              10485851455608404399 => {
                if c as libc::c_int == flg_not_length as libc::c_int {
                  flg_not_length = (flg_not_length as libc::c_int ^ 6i32) as libc::c_char;
                  continue;
                } else {
                  current_block_52 = 8869332144787829186;
                }
              }
              9642695190694411401 => {
                pbuf = if !(*lineedit_ptr_to_statics).user_buf.is_null() {
                  (*lineedit_ptr_to_statics).user_buf
                } else {
                  b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
                };
                current_block_52 = 8869332144787829186;
              }
              14477407255285059806 => {
                c = if geteuid() == 0i32 as libc::c_uint {
                  '#' as i32
                } else {
                  '$' as i32
                } as libc::c_char;
                current_block_52 = 8869332144787829186;
              }
              12410170708601199743 => {
                current_block_52 = 16491562349523041571;
              }
              _ => {}
            }
            match current_block_52 {
              8869332144787829186 => {}
              _ => {
                match current_block_52 {
                  16491562349523041571 => {}
                  _ => {}
                }
                *strftime_HHMMSS(
                  timebuf.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as libc::c_uint,
                  0 as *mut time_t,
                )
                .offset(-3i32 as isize) = '\u{0}' as i32 as libc::c_char;
                pbuf = timebuf.as_mut_ptr()
              }
            }
          }
          120 | 88 => {
            current_block_52 = 12381812505308290051;
            match current_block_52 {
              18087889531735461995 => {
                free_me = safe_gethostname();
                pbuf = free_me;
                if c as libc::c_int == 'h' as i32 {
                  *strchrnul(pbuf, '.' as i32).offset(0) = '\u{0}' as i32 as libc::c_char
                }
                current_block_52 = 8869332144787829186;
              }
              11214990802921196617 => {
                if cwd_buf.is_null() {
                  cwd_buf = xrealloc_getcwd_or_warn(0 as *mut libc::c_char);
                  if cwd_buf.is_null() {
                    cwd_buf = bb_msg_unknown.as_ptr() as *mut libc::c_char
                  } else if *(*lineedit_ptr_to_statics).home_pwd_buf.offset(0) != 0 {
                    let mut after_home_user: *mut libc::c_char = 0 as *mut libc::c_char;
                    after_home_user =
                      is_prefixed_with(cwd_buf, (*lineedit_ptr_to_statics).home_pwd_buf);
                    if !after_home_user.is_null()
                      && (*after_home_user as libc::c_int == '/' as i32
                        || *after_home_user as libc::c_int == '\u{0}' as i32)
                    {
                      *cwd_buf.offset(0) = '~' as i32 as libc::c_char;
                      overlapping_strcpy(cwd_buf.offset(1), after_home_user);
                    }
                  }
                }
                pbuf = cwd_buf;
                if c as libc::c_int == 'w' as i32 {
                  current_block_52 = 8869332144787829186;
                } else {
                  cp = strrchr(pbuf, '/' as i32);
                  if !cp.is_null() {
                    pbuf = (cp as *mut libc::c_char).offset(1)
                  }
                  current_block_52 = 8869332144787829186;
                }
              }
              12381812505308290051 => {
                let mut buf2: [libc::c_char; 4] = [0; 4];
                l = 0i32;
                while l < 3i32 {
                  let mut h: libc::c_uint = 0;
                  let fresh21 = l;
                  l = l + 1;
                  buf2[fresh21 as usize] = *prmt_ptr;
                  buf2[l as usize] = '\u{0}' as i32 as libc::c_char;
                  h = strtoul(buf2.as_mut_ptr(), &mut pbuf, 16i32) as libc::c_uint;
                  if h > (127i32 * 2i32 + 1i32) as libc::c_uint
                    || (pbuf.wrapping_offset_from(buf2.as_mut_ptr()) as libc::c_long)
                      < l as libc::c_long
                  {
                    l -= 1;
                    buf2[l as usize] = '\u{0}' as i32 as libc::c_char;
                    break;
                  } else {
                    prmt_ptr = prmt_ptr.offset(1)
                  }
                }
                c = strtoul(buf2.as_mut_ptr(), 0 as *mut *mut libc::c_char, 16i32) as libc::c_char;
                if c as libc::c_int == 0i32 {
                  c = '?' as i32 as libc::c_char
                }
                pbuf = cbuf.as_mut_ptr();
                current_block_52 = 8869332144787829186;
              }
              10485851455608404399 => {
                if c as libc::c_int == flg_not_length as libc::c_int {
                  flg_not_length = (flg_not_length as libc::c_int ^ 6i32) as libc::c_char;
                  continue;
                } else {
                  current_block_52 = 8869332144787829186;
                }
              }
              9642695190694411401 => {
                pbuf = if !(*lineedit_ptr_to_statics).user_buf.is_null() {
                  (*lineedit_ptr_to_statics).user_buf
                } else {
                  b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
                };
                current_block_52 = 8869332144787829186;
              }
              14477407255285059806 => {
                c = if geteuid() == 0i32 as libc::c_uint {
                  '#' as i32
                } else {
                  '$' as i32
                } as libc::c_char;
                current_block_52 = 8869332144787829186;
              }
              12410170708601199743 => {
                current_block_52 = 16491562349523041571;
              }
              _ => {}
            }
            match current_block_52 {
              8869332144787829186 => {}
              _ => {
                match current_block_52 {
                  16491562349523041571 => {}
                  _ => {}
                }
                *strftime_HHMMSS(
                  timebuf.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as libc::c_uint,
                  0 as *mut time_t,
                )
                .offset(-3i32 as isize) = '\u{0}' as i32 as libc::c_char;
                pbuf = timebuf.as_mut_ptr()
              }
            }
          }
          91 | 93 => {
            current_block_52 = 10485851455608404399;
            match current_block_52 {
              18087889531735461995 => {
                free_me = safe_gethostname();
                pbuf = free_me;
                if c as libc::c_int == 'h' as i32 {
                  *strchrnul(pbuf, '.' as i32).offset(0) = '\u{0}' as i32 as libc::c_char
                }
                current_block_52 = 8869332144787829186;
              }
              11214990802921196617 => {
                if cwd_buf.is_null() {
                  cwd_buf = xrealloc_getcwd_or_warn(0 as *mut libc::c_char);
                  if cwd_buf.is_null() {
                    cwd_buf = bb_msg_unknown.as_ptr() as *mut libc::c_char
                  } else if *(*lineedit_ptr_to_statics).home_pwd_buf.offset(0) != 0 {
                    let mut after_home_user: *mut libc::c_char = 0 as *mut libc::c_char;
                    after_home_user =
                      is_prefixed_with(cwd_buf, (*lineedit_ptr_to_statics).home_pwd_buf);
                    if !after_home_user.is_null()
                      && (*after_home_user as libc::c_int == '/' as i32
                        || *after_home_user as libc::c_int == '\u{0}' as i32)
                    {
                      *cwd_buf.offset(0) = '~' as i32 as libc::c_char;
                      overlapping_strcpy(cwd_buf.offset(1), after_home_user);
                    }
                  }
                }
                pbuf = cwd_buf;
                if c as libc::c_int == 'w' as i32 {
                  current_block_52 = 8869332144787829186;
                } else {
                  cp = strrchr(pbuf, '/' as i32);
                  if !cp.is_null() {
                    pbuf = (cp as *mut libc::c_char).offset(1)
                  }
                  current_block_52 = 8869332144787829186;
                }
              }
              12381812505308290051 => {
                let mut buf2: [libc::c_char; 4] = [0; 4];
                l = 0i32;
                while l < 3i32 {
                  let mut h: libc::c_uint = 0;
                  let fresh21 = l;
                  l = l + 1;
                  buf2[fresh21 as usize] = *prmt_ptr;
                  buf2[l as usize] = '\u{0}' as i32 as libc::c_char;
                  h = strtoul(buf2.as_mut_ptr(), &mut pbuf, 16i32) as libc::c_uint;
                  if h > (127i32 * 2i32 + 1i32) as libc::c_uint
                    || (pbuf.wrapping_offset_from(buf2.as_mut_ptr()) as libc::c_long)
                      < l as libc::c_long
                  {
                    l -= 1;
                    buf2[l as usize] = '\u{0}' as i32 as libc::c_char;
                    break;
                  } else {
                    prmt_ptr = prmt_ptr.offset(1)
                  }
                }
                c = strtoul(buf2.as_mut_ptr(), 0 as *mut *mut libc::c_char, 16i32) as libc::c_char;
                if c as libc::c_int == 0i32 {
                  c = '?' as i32 as libc::c_char
                }
                pbuf = cbuf.as_mut_ptr();
                current_block_52 = 8869332144787829186;
              }
              10485851455608404399 => {
                if c as libc::c_int == flg_not_length as libc::c_int {
                  flg_not_length = (flg_not_length as libc::c_int ^ 6i32) as libc::c_char;
                  continue;
                } else {
                  current_block_52 = 8869332144787829186;
                }
              }
              9642695190694411401 => {
                pbuf = if !(*lineedit_ptr_to_statics).user_buf.is_null() {
                  (*lineedit_ptr_to_statics).user_buf
                } else {
                  b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
                };
                current_block_52 = 8869332144787829186;
              }
              14477407255285059806 => {
                c = if geteuid() == 0i32 as libc::c_uint {
                  '#' as i32
                } else {
                  '$' as i32
                } as libc::c_char;
                current_block_52 = 8869332144787829186;
              }
              12410170708601199743 => {
                current_block_52 = 16491562349523041571;
              }
              _ => {}
            }
            match current_block_52 {
              8869332144787829186 => {}
              _ => {
                match current_block_52 {
                  16491562349523041571 => {}
                  _ => {}
                }
                *strftime_HHMMSS(
                  timebuf.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as libc::c_uint,
                  0 as *mut time_t,
                )
                .offset(-3i32 as isize) = '\u{0}' as i32 as libc::c_char;
                pbuf = timebuf.as_mut_ptr()
              }
            }
          }
          _ => {}
        }
      }
    }
    cbuf[0] = c;
    let mut n: libc::c_int = strlen(pbuf) as libc::c_int;
    prmt_size += n;
    if c as libc::c_int == '\n' as i32 {
      (*lineedit_ptr_to_statics).cmdedit_prmt_len = 0i32 as libc::c_uint
    } else if flg_not_length as libc::c_int != ']' as i32 {
      /*ENABLE_UNICODE_SUPPORT*/
      (*lineedit_ptr_to_statics).cmdedit_prmt_len = (*lineedit_ptr_to_statics)
        .cmdedit_prmt_len
        .wrapping_add(n as libc::c_uint)
    }
    prmt_mem_ptr = strcat(
      xrealloc(
        prmt_mem_ptr as *mut libc::c_void,
        (prmt_size + 1i32) as size_t,
      ) as *mut libc::c_char,
      pbuf,
    );
    free(free_me as *mut libc::c_void);
  }
  if cwd_buf != bb_msg_unknown.as_ptr() as *mut libc::c_char {
    free(cwd_buf as *mut libc::c_void);
  }
  /* see comment (above this function) about multiline prompt redrawing */
  (*lineedit_ptr_to_statics).prompt_last_line = prmt_mem_ptr;
  (*lineedit_ptr_to_statics).cmdedit_prompt = (*lineedit_ptr_to_statics).prompt_last_line;
  prmt_ptr = strrchr((*lineedit_ptr_to_statics).cmdedit_prompt, '\n' as i32);
  if !prmt_ptr.is_null() {
    (*lineedit_ptr_to_statics).prompt_last_line = prmt_ptr.offset(1)
  }
  put_prompt_custom(1i32 != 0);
}
unsafe extern "C" fn cmdedit_setwidth() {
  let mut new_y: libc::c_int = 0;
  (*lineedit_ptr_to_statics).cmdedit_termw = get_terminal_width(0i32) as libc::c_uint;
  /* new y for current cursor */
  new_y = (*lineedit_ptr_to_statics)
    .cursor
    .wrapping_add((*lineedit_ptr_to_statics).cmdedit_prmt_len)
    .wrapping_div((*lineedit_ptr_to_statics).cmdedit_termw) as libc::c_int;
  /* redraw */
  draw_custom(
    if new_y as libc::c_uint >= (*lineedit_ptr_to_statics).cmdedit_y {
      new_y as libc::c_uint
    } else {
      (*lineedit_ptr_to_statics).cmdedit_y
    } as libc::c_int,
    ((*lineedit_ptr_to_statics).command_len as libc::c_uint)
      .wrapping_sub((*lineedit_ptr_to_statics).cursor) as libc::c_int,
    0i32 != 0,
  );
}
unsafe extern "C" fn win_changed(mut _nsig: libc::c_int) {
  if (*lineedit_ptr_to_statics).ok_to_redraw != 0 {
    /* We are in read_key(), safe to redraw immediately */
    let mut sv_errno: libc::c_int = *bb_errno;
    cmdedit_setwidth();
    fflush_all();
    *bb_errno = sv_errno
  } else {
    /* Signal main loop that redraw is necessary */
    ::std::ptr::write_volatile(
      &mut (*lineedit_ptr_to_statics).SIGWINCH_count as *mut libc::c_uint,
      ::std::ptr::read_volatile::<libc::c_uint>(
        &(*lineedit_ptr_to_statics).SIGWINCH_count as *const libc::c_uint,
      )
      .wrapping_add(1),
    )
  };
}
unsafe extern "C" fn lineedit_read_key(
  mut read_key_buffer: *mut libc::c_char,
  mut timeout: libc::c_int,
) -> libc::c_int {
  let mut ic: int64_t = 0;
  let mut unicode_buf: [libc::c_char; 7] = [0; 7];
  let mut unicode_idx: libc::c_int = 0i32;
  fflush_all();
  let mut wc: wchar_t = 0;
  loop {
    /* Wait for input. TIMEOUT = -1 makes read_key wait even
     * on nonblocking stdin, TIMEOUT = 50 makes sure we won't
     * insist on full MB_CUR_MAX buffer to declare input like
     * "\xff\n",pause,"ls\n" invalid and thus won't lose "ls".
     *
     * Note: read_key sets errno to 0 on success.
     */
    ::std::ptr::write_volatile(
      &mut (*lineedit_ptr_to_statics).ok_to_redraw as *mut smallint,
      1i32 as smallint,
    );
    ic = read_key(0i32, read_key_buffer, timeout);
    ::std::ptr::write_volatile(
      &mut (*lineedit_ptr_to_statics).ok_to_redraw as *mut smallint,
      0i32 as smallint,
    );
    if *bb_errno != 0 {
      if !(*bb_errno == 11i32 && unicode_idx != 0i32) {
        break;
      }
    } else {
      if !(UNICODE_ON as libc::c_int == UNICODE_ON as libc::c_int) {
        break;
      }
      wc = 0;
      if (ic as i32) < 0i32 {
        break;
      }
      // TODO: imagine sequence like: 0xff,<left-arrow>: we are currently losing 0xff...
      let fresh22 = unicode_idx;
      unicode_idx = unicode_idx + 1;
      unicode_buf[fresh22 as usize] = ic as libc::c_char;
      unicode_buf[unicode_idx as usize] = '\u{0}' as i32 as libc::c_char;
      if bb_mbstowcs(&mut wc, unicode_buf.as_mut_ptr(), 1i32 as size_t) != 1i32 as libc::c_ulong {
        /* Not (yet?) a valid unicode char */
        if unicode_idx < 6i32 {
          timeout = 50i32;
          continue;
        }
      } else {
        /* Valid unicode char, return its code */
        ic = wc as int64_t;
        break;
      }
    }
    /* Invalid sequence. Save all "bad bytes" except first */
    read_key_ungets(
      read_key_buffer,
      unicode_buf.as_mut_ptr().offset(1),
      (unicode_idx - 1i32) as libc::c_uint,
    );
    ic = 63i32 as int64_t;
    break;
  }
  return ic as libc::c_int;
}
/* Mimic readline Ctrl-R reverse history search.
 * When invoked, it shows the following prompt:
 * (reverse-i-search)'': user_input [cursor pos unchanged by Ctrl-R]
 * and typing results in search being performed:
 * (reverse-i-search)'tmp': cd /tmp [cursor under t in /tmp]
 * Search is performed by looking at progressively older lines in history.
 * Ctrl-R again searches for the next match in history.
 * Backspace deletes last matched char.
 * Control keys exit search and return to normal editing (at current history line).
 */
unsafe extern "C" fn reverse_i_search(mut timeout: libc::c_int) -> i32 {
  let mut h: libc::c_int = 0; /* for user input */
  let mut match_buf_len: libc::c_uint = 0;
  let mut match_0: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut match_buf: [libc::c_char; 128] = [0; 128];
  let mut read_key_buffer: [libc::c_char; 16] = [0; 16];
  let mut matched_history_line: *const libc::c_char = 0 as *const libc::c_char;
  let mut saved_prompt: *const libc::c_char = 0 as *const libc::c_char;
  let mut saved_prmt_len: libc::c_uint = 0;
  let mut ic: i32 = 0;
  matched_history_line = 0 as *const libc::c_char;
  read_key_buffer[0] = 0i32 as libc::c_char;
  match_buf[0] = '\u{0}' as i32 as libc::c_char;
  /* Save and replace the prompt */
  saved_prompt = (*lineedit_ptr_to_statics).prompt_last_line; /* while (1) */
  saved_prmt_len = (*lineedit_ptr_to_statics).cmdedit_prmt_len;
  'c_12065: loop {
    (*lineedit_ptr_to_statics).prompt_last_line = xasprintf(
      b"(reverse-i-search)\'%s\': \x00" as *const u8 as *const libc::c_char,
      match_buf.as_mut_ptr(),
    );
    (*lineedit_ptr_to_statics).cmdedit_prmt_len =
      unicode_strwidth((*lineedit_ptr_to_statics).prompt_last_line) as libc::c_uint;
    draw_custom(
      (*lineedit_ptr_to_statics).cmdedit_y as libc::c_int,
      ((*lineedit_ptr_to_statics).command_len as libc::c_uint)
        .wrapping_sub((*lineedit_ptr_to_statics).cursor) as libc::c_int,
      0i32 != 0,
    );
    's_45: loop {
      h = 0;
      match_buf_len = strlen(match_buf.as_mut_ptr()) as libc::c_uint;
      //FIXME: correct timeout? (i.e. count it down?)
      ic = lineedit_read_key(read_key_buffer.as_mut_ptr(), timeout); /* switch (ic) */
      match ic {
        18 => {}
        8 | 127 => {
          /* Backspace */
          if UNICODE_ON as libc::c_int == UNICODE_ON as libc::c_int {
            while match_buf_len != 0i32 as libc::c_uint {
              match_buf_len = match_buf_len.wrapping_sub(1);
              let mut c: u8 = match_buf[match_buf_len as usize] as u8;
              if c as libc::c_int & 0xc0i32 != 0x80i32 {
                break;
              }
              /* yes */
            }
          } else if match_buf_len != 0i32 as libc::c_uint {
            match_buf_len = match_buf_len.wrapping_sub(1)
          }
          match_buf[match_buf_len as usize] = '\u{0}' as i32 as libc::c_char
        }
        _ => {
          if ic < ' ' as i32
            || 1i32 == 0 && ic >= 256i32
            || 1i32 != 0 && ic >= VI_CMDMODE_BIT as libc::c_int
          {
            break 'c_12065;
          }
          /* Append this char */
          if UNICODE_ON as libc::c_int == UNICODE_ON as libc::c_int {
            let mut mbstate: bb_mbstate_t = {
              let mut init = bb_mbstate_t {
                bogus: 0i32 as libc::c_char,
              };
              init
            };
            let mut buf: [libc::c_char; 7] = [0; 7];
            let mut len: libc::c_int =
              bb_wcrtomb(buf.as_mut_ptr(), ic, &mut mbstate) as libc::c_int;
            if len > 0i32 {
              buf[len as usize] = '\u{0}' as i32 as libc::c_char;
              if (match_buf_len.wrapping_add(len as libc::c_uint) as libc::c_ulong)
                < ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong
              {
                strcpy(
                  match_buf.as_mut_ptr().offset(match_buf_len as isize),
                  buf.as_mut_ptr(),
                );
              }
            }
          } else if (match_buf_len as libc::c_ulong)
            < (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
              .wrapping_sub(1i32 as libc::c_ulong)
          {
            match_buf[match_buf_len as usize] = ic as libc::c_char;
            match_buf[match_buf_len.wrapping_add(1i32 as libc::c_uint) as usize] =
              '\u{0}' as i32 as libc::c_char
          }
        }
      }
      /* searching for the next match */
      /* Search in history for match_buf */
      h = (*(*lineedit_ptr_to_statics).state).cur_history;
      if ic == 'R' as i32 & !0x40i32 {
        h -= 1
      }
      while h >= 0i32 {
        if !(*(*lineedit_ptr_to_statics).state).history[h as usize].is_null() {
          match_0 = strstr(
            (*(*lineedit_ptr_to_statics).state).history[h as usize],
            match_buf.as_mut_ptr(),
          );
          if !match_0.is_null() {
            break 's_45;
          }
        }
        h -= 1
      }
      /* Not found */
      match_buf[match_buf_len as usize] = '\u{0}' as i32 as libc::c_char;
      beep();
    }
    (*(*lineedit_ptr_to_statics).state).cur_history = h;
    matched_history_line = (*(*lineedit_ptr_to_statics).state).history[h as usize];
    (*lineedit_ptr_to_statics).command_len = load_string(matched_history_line) as libc::c_int;
    (*lineedit_ptr_to_statics).cursor =
      match_0.wrapping_offset_from(matched_history_line) as libc::c_long as libc::c_uint;
    //FIXME: cursor position for Unicode case
    free((*lineedit_ptr_to_statics).prompt_last_line as *mut libc::c_char as *mut libc::c_void);
  }
  if !matched_history_line.is_null() {
    (*lineedit_ptr_to_statics).command_len = load_string(matched_history_line) as libc::c_int
  }
  free((*lineedit_ptr_to_statics).prompt_last_line as *mut libc::c_char as *mut libc::c_void);
  (*lineedit_ptr_to_statics).prompt_last_line = saved_prompt;
  (*lineedit_ptr_to_statics).cmdedit_prmt_len = saved_prmt_len;
  draw_custom(
    (*lineedit_ptr_to_statics).cmdedit_y as libc::c_int,
    ((*lineedit_ptr_to_statics).command_len as libc::c_uint)
      .wrapping_sub((*lineedit_ptr_to_statics).cursor) as libc::c_int,
    0i32 != 0,
  );
  return ic;
}
unsafe extern "C" fn sigaction2(mut sig: libc::c_int, mut act: *mut sigaction) {
  // Grr... gcc 8.1.1:
  // "passing argument 3 to restrict-qualified parameter aliases with argument 2"
  // dance around that...
  let mut oact: *mut sigaction = 0 as *mut sigaction;
  oact = act;
  sigaction(sig, act, oact);
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
/* lzma has no signature, need a little helper. NB: exist only for ENABLE_FEATURE_SEAMLESS_LZMA=y */
// NB: will return short write on error, not -1,
// if some data was written before error occurred
/* Close fd, but check for failures (some types of write errors) */
/* Reads and prints to stdout till eof, then closes FILE. Exits on error: */
/* Reads a line from a text file, up to a newline or NUL byte, inclusive.
 * Returns malloc'ed char*. If end is NULL '\n' isn't considered
 * end of line. If end isn't NULL, length of the chunk is stored in it.
 * Returns NULL if EOF/error.
 */
/* Reads up to (and including) TERMINATING_STRING: */
/* Same, with limited max size, and returns the length (excluding NUL): */
/* Chops off TERMINATING_STRING from the end: */
/* Reads up to (and including) "\n" or NUL byte: */
/* Chops off '\n' from the end, unlike fgets: */
/* Same, but doesn't try to conserve space (may have some slack after the end) */
/* extern char *xmalloc_fgetline_fast(FILE *file) FAST_FUNC RETURNS_MALLOC; */
/* Prints warning to stderr and returns NULL on failure: */
/* "Opens" stdin if filename is special, else just opens file: */
/* not FAST_FUNC! */
/* Wrapper which restarts poll on EINTR or ENOMEM.
 * On other errors complains [perror("poll")] and returns.
 * Warning! May take (much) longer than timeout_ms to return!
 * If this is a problem, use bare poll and open-code EINTR/ENOMEM handling */
/* Convert each alpha char in str to lower-case */
/* Returns a pointer past the formatted number, does NOT null-terminate */
/* Intelligent formatters of bignums */
/* If block_size == 0, display size without fractional part,
 * else display (size * block_size) with one decimal digit.
 * If display_unit == 0, show value no bigger than 1024 with suffix (K,M,G...),
 * else divide by display_unit and do not use suffix. */
/* "1024.0G" */
//TODO: provide pointer to buf (avoid statics)?
/* Put a string of hex bytes ("1b2e66fe"...), return advanced pointer */
/* Reverse */
/* Generate a UUID */
/* Last element is marked by mult == 0 */
/* Specialized: */
/* Using xatoi() instead of naive atoi() is not always convenient -
 * in many places people want *non-negative* values, but store them
 * in signed int. Therefore we need this one:
 * dies if input is not in [0, INT_MAX] range. Also will reject '-0' etc.
 * It should really be named xatoi_nonnegative (since it allows 0),
 * but that would be too long.
 */
/* Useful for reading port numbers */
/* These parse entries in /etc/passwd and /etc/group.  This is desirable
 * for BusyBox since we want to avoid using the glibc NSS stuff, which
 * increases target size and is often not needed on embedded systems.  */
/* wrapper: allows string to contain numeric uid or gid */
/* always sets uid and gid; returns 0 on failure */
/* always sets uid and gid; exits on failure */
/* chown-like handling of "user[:[group]" */
/* versions which cache results (useful for ps, ls etc) */
/* internally usernames are saved in fixed-sized char[] buffers */
/*
 * Returns (-1) terminated malloced result of getgroups().
 * Reallocs group_array (useful for repeated calls).
 * ngroups is an initial size of array. It is rounded up to 32 for realloc.
 * ngroups is updated on return.
 * ngroups can be NULL: bb_getgroups(NULL, NULL) is valid usage.
 * Dies on errors (on Linux, only xrealloc can cause this, not internal getgroups call).
 */
/* BB_EXECxx always execs (it's not doing NOFORK/NOEXEC stuff),
 * but it may exec busybox and call applet instead of searching PATH.
 */
/* xvfork() can't be a _function_, return after vfork in child mangles stack
 * in the parent. It must be a macro. */
/* NOMMU friendy fork+exec: */
/* wait4pid: unlike waitpid, waits ONLY for one process.
 * Returns sig + 0x180 if child is killed by signal.
 * It's safe to pass negative 'pids' from failed [v]fork -
 * wait4pid will return -1 (and will not clobber [v]fork's errno).
 * IOW: rc = wait4pid(spawn(argv));
 *      if (rc < 0) bb_perror_msg("%s", argv[0]);
 *      if (rc > 0) bb_error_msg("exit code: %d", rc & 0xff);
 */
/* ***********************************************************************/
/* spawn_and_wait/run_nofork_applet/run_applet_no_and_exit need to work */
/* carefully together to reinit some global state while not disturbing  */
/* other. Be careful if you change them. Consult docs/nofork_noexec.txt */
/* ***********************************************************************/
/* Same as wait4pid(spawn(argv)), but with NOFORK/NOEXEC if configured: */
/* Does NOT check that applet is NOFORK, just blindly runs it */
/* Helpers for daemonization.
 *
 * bb_daemonize(flags) = daemonize, does not compile on NOMMU
 *
 * bb_daemonize_or_rexec(flags, argv) = daemonizes on MMU (and ignores argv),
 *      rexec's itself on NOMMU with argv passed as command line.
 * Thus bb_daemonize_or_rexec may cause your <applet>_main() to be re-executed
 * from the start. (It will detect it and not reexec again second time).
 * You have to audit carefully that you don't do something twice as a result
 * (opening files/sockets, parsing config files etc...)!
 *
 * Both of the above will redirect fd 0,1,2 to /dev/null and drop ctty
 * (will do setsid()).
 *
 * fork_or_rexec(argv) = bare-bones fork on MMU,
 *      "vfork + re-exec ourself" on NOMMU. No fd redirection, no setsid().
 *      On MMU ignores argv.
 *
 * Helper for network daemons in foreground mode:
 *
 * bb_sanitize_stdio() = make sure that fd 0,1,2 are opened by opening them
 * to /dev/null if they are not.
 */
/* internal use */
//DAEMON_DOUBLE_FORK     = 1 << 4, /* double fork to avoid controlling tty */
/* Clear dangerous stuff, set PATH. Return 1 if was run by different user. */
/* For top, ps. Some argv[i] are replaced by malloced "-opt" strings */
/* { "-", NULL } */
/* BSD-derived getopt() functions require that optind be set to 1 in
 * order to reset getopt() state.  This used to be generally accepted
 * way of resetting getopt().  However, glibc's getopt()
 * has additional getopt() state beyond optind (specifically, glibc
 * extensions such as '+' and '-' at the start of the string), and requires
 * that optind be set to zero to reset its state.  BSD-derived versions
 * of getopt() misbehaved if optind is set to 0 in order to reset getopt(),
 * and glibc's getopt() used to coredump if optind is set 1 in order
 * to reset getopt().
 * Then BSD introduced additional variable "optreset" which should be
 * set to 1 in order to reset getopt().  Sigh.  Standards, anyone?
 *
 * By ~2008, OpenBSD 3.4 was changed to survive glibc-like optind = 0
 * (to interpret it as if optreset was set).
 */
/*def __GLIBC__*/
/* BSD style */
/* Having next pointer as a first member allows easy creation
 * of "llist-compatible" structs, and using llist_FOO functions
 * on them.
 */
/* BTW, surprisingly, changing API to
 *   llist_t *llist_add_to(llist_t *old_head, void *data)
 * etc does not result in smaller code... */
/* start_stop_daemon and udhcpc are special - they want
 * to create pidfiles regardless of FEATURE_PIDFILE */
/* True only if we created pidfile which is *file*, not /dev/null etc */
/* We need to export XXX_main from libbusybox
 * only if we build "individual" binaries
 */
/* Embedded script support */
/* Applets which are useful from another applets */
/* If shell needs them, they exist even if not enabled as applets */
/* Similar, but used by chgrp, not shell */
/* Used by ftpd */
/* Don't need IF_xxx() guard for these */
/* Networking */
/* This structure defines protocol families and their handlers. */
/*int type,*/
/* may modify src */
/* This structure defines hardware protocols and their handlers. */
/*
 * If *devname is not NULL, use that name, otherwise try to find free one,
 * malloc and return it in *devname.
 * return value is the opened fd to the loop device, or < on error
 */
/* These constants match linux/loop.h (without BB_ prefix): */
/* Returns malloced str */
/* Like bb_ask_noecho, but asks on stdin with no timeout.  */
/* Returns -1 if input is invalid. current_mode is a base for e.g. "u+rw" */
/*
 * Config file parser
 */
// treat consecutive delimiters as one
// trim leading and trailing delimiters
// TODO: COLLAPSE and TRIM seem to always go in pair
// last token takes entire remainder of the line
// die if < min tokens found
// keep a copy of current line
// comments are recognized even if they aren't the first char
// delim[0] and delim[1] are two different allowed comment chars
// (so far, delim[0] will only work as comment char for full-line comment)
// (IOW: it works as if PARSE_EOL_COMMENTS is not set. sysctl applet is okay with this)
// comments are recognized even if there is whitespace before
// ("line start><space><tab><space>#comment" is also comment, not only "line start>#comment")
// NORMAL is:
// * remove leading and trailing delimiters and collapse
//   multiple delimiters into one
// * warn and continue if less than mintokens delimiters found
// * grab everything into last token
// * comments are recognized even if they aren't the first char
/* delims[0] is a comment char (use '\0' to disable), the rest are token delimiters */
/* Concatenate path and filename to new allocated buffer.
 * Add "/" only as needed (no duplicate "//" are produced).
 * If path is NULL, it is assumed to be "/".
 * filename should not be NULL. */
/* Returns NULL on . and .. */
/* Returns ptr to NUL */
/* Returns $SHELL, getpwuid(getuid())->pw_shell, or DEFAULT_SHELL.
 * Note that getpwuid result might need xstrdup'ing
 * if there is a possibility of intervening getpwxxx() calls.
 */
/* Structures inside "struct caps" are Linux-specific and libcap-specific: */
/* setup_environment:
 * if chdir pw->pw_dir: ok: else if to_tmp == 1: goto /tmp else: goto / or die
 * if clear_env = 1: cd(pw->pw_dir), clear environment, then set
 *   TERM=(old value)
 *   USER=pw->pw_name, LOGNAME=pw->pw_name
 *   PATH=bb_default_[root_]path
 *   HOME=pw->pw_dir
 *   SHELL=shell
 * else if change_env = 1:
 *   if not root (if pw->pw_uid != 0):
 *     USER=pw->pw_name, LOGNAME=pw->pw_name
 *   HOME=pw->pw_dir
 *   SHELL=shell
 * else does nothing
 *
 * NB: CHANGEENV and CLEARENV use setenv() - this leaks memory!
 * If setup_environment() is used is vforked child, this leaks memory _in parent too_!
 */
/* Returns a malloced string */
/*
 * rnd is additional random input. New one is returned.
 * Useful if you call crypt_make_salt many times in a row:
 * rnd = crypt_make_salt(buf1, 4, 0);
 * rnd = crypt_make_salt(buf2, 4, rnd);
 * rnd = crypt_make_salt(buf3, 4, rnd);
 * (otherwise we risk having same salt generated)
 */
/*, int rnd*/
/* "$N$" + sha_salt_16_bytes + NUL */
/* Returns number of lines changed, or -1 on error */
/* NB: typically you want to pass fd 0, not 1. Think 'applet | grep something' */
/* NB: "unsigned request" is crucial! "int request" will break some arches! */
/* At least glibc has horrendously large inline for this, so wrap it */
/* "Keycodes" that report an escape sequence.
 * We use something which fits into signed char,
 * yet doesn't represent any valid Unicode character.
 * Also, -1 is reserved for error indication and we don't use it. */
/* Used only if Alt/Ctrl/Shifted */
/* Used only if Alted */
/* ^^^^^ Be sure that last defined value is small enough.
 * Current read_key() code allows going up to -32 (0xfff..fffe0).
 * This gives three upper bits in LSB to play with:
 * KEYCODE_foo values are 0xfff..fffXX, lowest XX bits are: scavvvvv,
 * s=0 if SHIFT, c=0 if CTRL, a=0 if ALT,
 * vvvvv bits are the same for same key regardless of "shift bits".
 */
//KEYCODE_SHIFT_...   = KEYCODE_...   & ~0x80,
/* 0xfff..fff00 */
/* How long is the longest ESC sequence we know?
 * We want it big enough to be able to contain
 * cursor position sequence "ESC [ 9999 ; 9999 R"
 */
/* Note: fd may be in blocking or non-blocking mode, both make sense.
 * For one, less uses non-blocking mode.
 * Only the first read syscall inside read_key may block indefinitely
 * (unless fd is in non-blocking mode),
 * subsequent reads will time out after a few milliseconds.
 * Return of -1 means EOF or error (errno == 0 on EOF).
 * buffer[0] is used as a counter of buffered chars and must be 0
 * on first call.
 * timeout:
 * -2: do not poll(-1) for input - read() it, return on EAGAIN at once
 * -1: poll(-1) (i.e. block even on NONBLOCKed fd)
 * >=0: poll() for TIMEOUT milliseconds, return -1/EAGAIN on timeout
 */
/* It's NOT just ENABLEd or disabled. It's a number: */
/* must never be <= 0 */
/* meaning of this field depends on FEATURE_EDITING_SAVE_ON_EXIT:
 * if !FEATURE_EDITING_SAVE_ON_EXIT: "how many lines are
 * in on-disk history"
 * if FEATURE_EDITING_SAVE_ON_EXIT: "how many in-memory lines are
 * also in on-disk history (and thus need to be skipped on save)"
 */
/*
 * maxsize must be >= 2.
 * Returns:
 * -1 on read errors or EOF, or on bare Ctrl-D,
 * 0  on ctrl-C (the line entered is still returned in 'command'),
 * >0 length of input string, including terminating '\n'
 */
/* maxsize must be >= 2.
 * Returns:
 * -1 on read errors or EOF, or on bare Ctrl-D,
 * 0  on ctrl-C (the line entered is still returned in 'command'),
 * (in both cases the cursor remains on the input line, '\n' is not printed)
 * >0 length of input string, including terminating '\n'
 */
#[no_mangle]
pub unsafe extern "C" fn read_line_input(
  mut st: *mut line_input_t,
  mut prompt: *const libc::c_char,
  mut command: *mut libc::c_char,
  mut maxsize: libc::c_int,
) -> libc::c_int {
  let mut current_block: u64;
  let mut len: libc::c_int = 0;
  let mut n: libc::c_int = 0;
  let mut timeout: libc::c_int = 0;
  let mut lastWasTab: smallint = 0i32 as smallint;
  let mut break_out: smallint = 0i32 as smallint;
  let mut initial_settings: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
  };
  let mut new_settings: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
  };
  let mut read_key_buffer: [libc::c_char; 16] = [0; 16];
  let ref mut fresh23 =
    *(not_const_pp(&lineedit_ptr_to_statics as *const *mut lineedit_statics as *const libc::c_void)
      as *mut *mut lineedit_statics);
  *fresh23 =
    xzalloc(::std::mem::size_of::<lineedit_statics>() as libc::c_ulong) as *mut lineedit_statics;
  asm!("" : : : "memory" : "volatile");
  (*lineedit_ptr_to_statics).cmdedit_termw = 80i32 as libc::c_uint;
  (*lineedit_ptr_to_statics).home_pwd_buf = null_str.as_ptr() as *mut libc::c_char;
  n = get_termios_and_make_raw(
    0i32,
    &mut new_settings,
    &mut initial_settings,
    0i32 | 1i32 << 0i32,
  );
  if n != 0i32
    || initial_settings.c_lflag & (0o10i32 | 0o2i32) as libc::c_uint == 0o2i32 as libc::c_uint
  {
    /* Happens when e.g. stty -echo was run before.
     * But if ICANON is not set, we don't come here.
     * (example: interactive python ^Z-backgrounded,
     * tty is still in "raw mode").
     */
    parse_and_put_prompt(prompt); /* EOF or error */
    fflush_all();
    if fgets_unlocked(command, maxsize, stdin).is_null() {
      len = -1i32
    } else {
      len = strlen(command) as libc::c_int
    }
    deinit_S();
    return len;
  }
  // FIXME: audit & improve this
  if maxsize > MAX_LINELEN as libc::c_int {
    maxsize = MAX_LINELEN as libc::c_int
  }
  (*lineedit_ptr_to_statics).maxsize = maxsize;
  timeout = -1i32;
  /* Make state->flags == 0 if st is NULL.
   * With zeroed flags, no other fields are ever referenced.
   */
  (*lineedit_ptr_to_statics).state = &const_int_0 as *const libc::c_int as *mut line_input_t;
  if !st.is_null() {
    (*lineedit_ptr_to_statics).state = st;
    timeout = (*st).timeout
  }
  if (*(*lineedit_ptr_to_statics).state).flags & DO_HISTORY as libc::c_int != 0 {
    if !(*(*lineedit_ptr_to_statics).state).hist_file.is_null() {
      if (*(*lineedit_ptr_to_statics).state).cnt_history == 0i32 {
        load_history((*lineedit_ptr_to_statics).state);
      }
    }
    (*(*lineedit_ptr_to_statics).state).cur_history =
      (*(*lineedit_ptr_to_statics).state).cnt_history
  }
  /* prepare before init handlers */
  (*lineedit_ptr_to_statics).cmdedit_y = 0i32 as libc::c_uint; /* quasireal y, not true if line > xt*yt */
  (*lineedit_ptr_to_statics).command_len = 0i32;
  (*lineedit_ptr_to_statics).command_ps = xzalloc(
    (maxsize as libc::c_ulong).wrapping_mul(::std::mem::size_of::<wchar_t>() as libc::c_ulong),
  ) as *mut wchar_t;
  tcsetattr_stdin_TCSANOW(&mut new_settings);
  let mut entry: *mut passwd = 0 as *mut passwd;
  entry = bb_internal_getpwuid(geteuid());
  if !entry.is_null() {
    (*lineedit_ptr_to_statics).user_buf = xstrdup((*entry).pw_name);
    (*lineedit_ptr_to_statics).home_pwd_buf = xstrdup((*entry).pw_dir)
  }
  /* Get width (before printing prompt) */
  (*lineedit_ptr_to_statics).cmdedit_termw = get_terminal_width(0i32) as libc::c_uint;
  /* Print out the command prompt, optionally ask where cursor is */
  parse_and_put_prompt(prompt);
  /* Install window resize handler (NB: after *all* init is complete) */
  (*lineedit_ptr_to_statics)
    .SIGWINCH_handler
    .__sigaction_handler
    .sa_handler = Some(win_changed as unsafe extern "C" fn(_: libc::c_int) -> ()); /* while (1) */
  (*lineedit_ptr_to_statics).SIGWINCH_handler.sa_flags = 0x10000000i32;
  sigaction2(28i32, &mut (*lineedit_ptr_to_statics).SIGWINCH_handler);
  read_key_buffer[0] = 0i32 as libc::c_char;
  loop
  /*
   * The emacs and vi modes share much of the code in the big
   * command loop.  Commands entered when in vi's command mode
   * (aka "escape mode") get an extra bit added to distinguish
   * them - this keeps them from being self-inserted. This
   * clutters the big switch a bit, but keeps all the code
   * in one place.
   */
  {
    let mut ic: i32 = 0; /* switch (ic) */
    let mut ic_raw: i32 = 0;
    let mut count: libc::c_uint = 0;
    count = (*lineedit_ptr_to_statics).SIGWINCH_count;
    if (*lineedit_ptr_to_statics).SIGWINCH_saved != count {
      (*lineedit_ptr_to_statics).SIGWINCH_saved = count;
      cmdedit_setwidth();
    }
    ic_raw = lineedit_read_key(read_key_buffer.as_mut_ptr(), timeout);
    ic = ic_raw;
    loop {
      match ic {
        10 | 13 => {
          /* Enter */
          goto_new_line();
          break_out = 1i32 as smallint;
          current_block = 14184516523743666873;
          break;
        }
        1 => {
          /* Control-a -- Beginning of line */
          input_backward((*lineedit_ptr_to_statics).cursor); /* Move back one character */
          current_block = 14184516523743666873;
          break;
        }
        2 => {
          /* ^H */
          /* DEL */
          input_backward(1i32 as libc::c_uint);
          current_block = 14184516523743666873;
          break;
        }
        5 => {
          /* Control-e -- End of line */
          put_till_end_and_adv_cursor(); /* Move forward one character */
          current_block = 14184516523743666873;
          break;
        }
        6 => {
          input_forward();
          current_block = 14184516523743666873;
          break;
        }
        8 | 127 => {
          /* ^H */
          /* DEL */
          if 0i32 == 0 {
            input_backspace();
          } else {
            input_delete();
          }
          current_block = 14184516523743666873;
          break;
        }
        -9 => {
          if 0i32 == 0 {
            input_delete();
          } else {
            input_backspace();
          }
          current_block = 14184516523743666873;
          break;
        }
        9 => {
          input_tab(&mut lastWasTab);
          current_block = 14184516523743666873;
          break;
        }
        11 => {
          /* Control-k -- clear to end of line */
          *(*lineedit_ptr_to_statics)
            .command_ps
            .offset((*lineedit_ptr_to_statics).cursor as isize) = 0i32;
          (*lineedit_ptr_to_statics).command_len = (*lineedit_ptr_to_statics).cursor as libc::c_int;
          printf(b"\x1b[J\x00" as *const u8 as *const libc::c_char);
          current_block = 14184516523743666873;
          break;
        }
        12 => {
          /* Control-l -- clear screen */
          /* cursor to top,left; clear to the end of screen */
          printf(b"\x1b[H\x1b[J\x00" as *const u8 as *const libc::c_char);
          draw_custom(
            0i32,
            ((*lineedit_ptr_to_statics).command_len as libc::c_uint)
              .wrapping_sub((*lineedit_ptr_to_statics).cursor) as libc::c_int,
            1i32 != 0,
          );
          current_block = 14184516523743666873;
          break;
        }
        14 => {
          /* Control-n -- Get next command in history */
          if get_next_history() != 0 {
            current_block = 47308297132330914;
            break;
          } else {
            current_block = 14184516523743666873;
            break;
          }
        }
        16 => {
          /* Control-p -- Get previous command from history */
          if get_previous_history() != 0 {
            current_block = 47308297132330914;
            break;
          } else {
            current_block = 14184516523743666873;
            break;
          }
        }
        21 => {
          /* Control-U -- Clear line before cursor */
          if (*lineedit_ptr_to_statics).cursor != 0 {
            (*lineedit_ptr_to_statics).command_len = ((*lineedit_ptr_to_statics).command_len
              as libc::c_uint)
              .wrapping_sub((*lineedit_ptr_to_statics).cursor)
              as libc::c_int as libc::c_int;
            memmove(
              (*lineedit_ptr_to_statics).command_ps as *mut libc::c_void,
              (*lineedit_ptr_to_statics)
                .command_ps
                .offset((*lineedit_ptr_to_statics).cursor as isize)
                as *const libc::c_void,
              (((*lineedit_ptr_to_statics).command_len + 1i32) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<wchar_t>() as libc::c_ulong),
            );
            draw_custom(
              (*lineedit_ptr_to_statics).cmdedit_y as libc::c_int,
              (*lineedit_ptr_to_statics).command_len,
              0i32 != 0,
            );
          }
          current_block = 14184516523743666873;
          break;
        }
        23 => {
          /* Control-W -- Remove the last word */
          while (*lineedit_ptr_to_statics).cursor > 0i32 as libc::c_uint
            && BB_isspace(
              *(*lineedit_ptr_to_statics).command_ps.offset(
                (*lineedit_ptr_to_statics)
                  .cursor
                  .wrapping_sub(1i32 as libc::c_uint) as isize,
              ),
            ) as libc::c_int
              != 0
          {
            input_backspace();
          }
          while (*lineedit_ptr_to_statics).cursor > 0i32 as libc::c_uint
            && !BB_isspace(
              *(*lineedit_ptr_to_statics).command_ps.offset(
                (*lineedit_ptr_to_statics)
                  .cursor
                  .wrapping_sub(1i32 as libc::c_uint) as isize,
              ),
            )
          {
            input_backspace();
          }
          current_block = 14184516523743666873;
          break;
        }
        -45 => {
          /* Delete word forward */
          let mut nc: libc::c_int = 0;
          let mut sc: libc::c_int = (*lineedit_ptr_to_statics).cursor as libc::c_int;
          ctrl_right();
          nc = (*lineedit_ptr_to_statics)
            .cursor
            .wrapping_sub(sc as libc::c_uint) as libc::c_int;
          input_backward(nc as libc::c_uint);
          loop {
            nc -= 1;
            if !(nc >= 0i32) {
              break;
            }
            input_delete();
          }
          current_block = 14184516523743666873;
          break;
        }
        -44 => {
          /* Delete word backward */
          let mut sc_0: libc::c_int = (*lineedit_ptr_to_statics).cursor as libc::c_int;
          ctrl_left();
          loop {
            let fresh24 = sc_0;
            sc_0 = sc_0 - 1;
            if !(fresh24 as libc::c_uint > (*lineedit_ptr_to_statics).cursor) {
              break;
            }
            input_delete();
          }
          current_block = 14184516523743666873;
          break;
        }
        18 => {
          ic_raw = reverse_i_search(timeout);
          ic = ic_raw
        }
        -2 => {
          /* FEATURE_COMMAND_EDITING_VI */
          if get_previous_history() != 0 {
            current_block = 47308297132330914;
            break;
          } else {
            current_block = 2652804691515851435;
            break;
          }
        }
        -3 => {
          if get_next_history() == 0 {
            current_block = 14184516523743666873;
            break;
          } else {
            current_block = 47308297132330914;
            break;
          }
        }
        -4 => {
          input_forward();
          current_block = 14184516523743666873;
          break;
        }
        -5 => {
          input_backward(1i32 as libc::c_uint);
          current_block = 14184516523743666873;
          break;
        }
        -69 | -37 => {
          /* bash doesn't do it */
          ctrl_left();
          current_block = 14184516523743666873;
          break;
        }
        -68 | -36 => {
          /* bash doesn't do it */
          ctrl_right();
          current_block = 14184516523743666873;
          break;
        }
        -6 => {
          input_backward((*lineedit_ptr_to_statics).cursor);
          current_block = 14184516523743666873;
          break;
        }
        -7 => {
          put_till_end_and_adv_cursor();
          current_block = 14184516523743666873;
          break;
        }
        -1 => {
          current_block = 6027889462055013191;
          break;
        }
        _ => {
          if initial_settings.c_cc[0] as libc::c_int != 0i32
            && ic_raw == initial_settings.c_cc[0] as libc::c_int
          {
            current_block = 15417752026496523887;
            break;
          } else {
            current_block = 9430418855388998878;
            break;
          }
        }
      }
    }
    match current_block {
      47308297132330914 => {
        /* Rewrite the line with the selected history item */
        /* change command */
        (*lineedit_ptr_to_statics).command_len = load_string(
          if !(*(*lineedit_ptr_to_statics).state).history
            [(*(*lineedit_ptr_to_statics).state).cur_history as usize]
            .is_null()
          {
            (*(*lineedit_ptr_to_statics).state).history
              [(*(*lineedit_ptr_to_statics).state).cur_history as usize]
          } else {
            b"\x00" as *const u8 as *const libc::c_char
          },
        ) as libc::c_int;
        /* redraw and go to eol (bol, in vi) */
        draw_custom(
          (*lineedit_ptr_to_statics).cmdedit_y as libc::c_int,
          if (*(*lineedit_ptr_to_statics).state).flags & VI_MODE as libc::c_int != 0 {
            9999i32
          } else {
            0i32
          },
          0i32 != 0,
        );
        current_block = 14184516523743666873;
      }
      15417752026496523887 => {
        /* Ctrl-C (usually) - stop gathering input */
        (*lineedit_ptr_to_statics).command_len = 0i32; /* "do not append '\n'" */
        break_out = -1i32 as smallint;
        current_block = 14184516523743666873;
      }
      9430418855388998878 => {
        if initial_settings.c_cc[4] as libc::c_int != 0i32
          && ic_raw == initial_settings.c_cc[4] as libc::c_int
        {
          /* Ctrl-D (usually) - delete one character,
           * or exit if len=0 and no chars to delete */
          if (*lineedit_ptr_to_statics).command_len == 0i32 {
            *bb_errno = 0i32;
            current_block = 6027889462055013191;
          } else {
            input_delete();
            current_block = 14184516523743666873;
          }
        } else {
          //			/* Control-V -- force insert of next char */
          //			if (c == CTRL('V')) {
          //				if (safe_read(STDIN_FILENO, &c, 1) < 1)
          //					goto return_error_indicator;
          //				if (c == 0) {
          //					beep();
          //					break;
          //				}
          //			}
          if !(ic < ' ' as i32
            || 1i32 == 0 && ic >= 256i32
            || 1i32 != 0 && ic >= VI_CMDMODE_BIT as libc::c_int)
          {
            if !((*lineedit_ptr_to_statics).command_len >= maxsize - 2i32) {
              (*lineedit_ptr_to_statics).command_len += 1;
              if (*lineedit_ptr_to_statics).cursor
                == ((*lineedit_ptr_to_statics).command_len - 1i32) as libc::c_uint
              {
                /* We are at the end, append */
                *(*lineedit_ptr_to_statics)
                  .command_ps
                  .offset((*lineedit_ptr_to_statics).cursor as isize) = ic;
                *(*lineedit_ptr_to_statics).command_ps.offset(
                  (*lineedit_ptr_to_statics)
                    .cursor
                    .wrapping_add(1i32 as libc::c_uint) as isize,
                ) = 0i32;
                put_cur_glyph_and_inc_cursor();
              } else {
                /* In the middle, insert */
                let mut sc_1: libc::c_int = (*lineedit_ptr_to_statics).cursor as libc::c_int;
                memmove(
                  (*lineedit_ptr_to_statics)
                    .command_ps
                    .offset(sc_1 as isize)
                    .offset(1) as *mut libc::c_void,
                  (*lineedit_ptr_to_statics).command_ps.offset(sc_1 as isize)
                    as *const libc::c_void,
                  (((*lineedit_ptr_to_statics).command_len - sc_1) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<wchar_t>() as libc::c_ulong),
                );
                *(*lineedit_ptr_to_statics).command_ps.offset(sc_1 as isize) = ic;
                /* is right-to-left char, or neutral one (e.g. comma) was just added to rtl text? */
                if 0i32 == 0 {
                  sc_1 += 1
                } /* no */
                put_till_end_and_adv_cursor();
                /* to prev x pos + 1 */
                input_backward(
                  (*lineedit_ptr_to_statics)
                    .cursor
                    .wrapping_sub(sc_1 as libc::c_uint),
                );
              }
            }
          }
          current_block = 14184516523743666873;
        }
      }
      2652804691515851435 => {
        beep();
        current_block = 14184516523743666873;
      }
      _ => {}
    }
    match current_block {
      6027889462055013191 => {
        /* error (e.g. EIO when tty is destroyed) */
        (*lineedit_ptr_to_statics).command_len = -1i32;
        break_out = (*lineedit_ptr_to_statics).command_len as smallint
      }
      _ => {}
    }
    /* Not enough space for the char and EOL */
    if break_out != 0 {
      break;
    }
    if ic_raw != '\t' as i32 {
      lastWasTab = 0i32 as smallint
    }
  }
  /* End of bug-catching "command_must_not_be_used" trick */
  *command.offset(0) = '\u{0}' as i32 as libc::c_char;
  if (*lineedit_ptr_to_statics).command_len > 0i32 {
    (*lineedit_ptr_to_statics).command_len =
      save_string(command, (maxsize - 1i32) as libc::c_uint) as libc::c_int
  }
  free((*lineedit_ptr_to_statics).command_ps as *mut libc::c_void);
  if (*lineedit_ptr_to_statics).command_len > 0i32 {
    remember_in_history(command);
  }
  if break_out as libc::c_int > 0i32 {
    let fresh25 = (*lineedit_ptr_to_statics).command_len;
    (*lineedit_ptr_to_statics).command_len = (*lineedit_ptr_to_statics).command_len + 1;
    *command.offset(fresh25 as isize) = '\n' as i32 as libc::c_char;
    *command.offset((*lineedit_ptr_to_statics).command_len as isize) =
      '\u{0}' as i32 as libc::c_char
  }
  free_tab_completion_data();
  /* restore initial_settings */
  tcsetattr_stdin_TCSANOW(&mut initial_settings);
  /* restore SIGWINCH handler */
  sigaction_set(28i32, &mut (*lineedit_ptr_to_statics).SIGWINCH_handler);
  fflush_all();
  len = (*lineedit_ptr_to_statics).command_len;
  deinit_S();
  return len;
  /* can't return command_len, DEINIT_S() destroys it */
}
/* TEST */
/*
 * Testing
 */
/* !FEATURE_EDITING */
/* !FEATURE_EDITING */

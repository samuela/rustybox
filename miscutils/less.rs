use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::xfuncs_printf::xmalloc;
use crate::librb::signal::__sighandler_t;
use crate::librb::size_t;
use crate::librb::smallint;
use c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
use libc;
use libc::access;
use libc::close;
use libc::fclose;
use libc::fprintf;
use libc::free;
use libc::fstat;
use libc::getenv;
use libc::isatty;
use libc::open;
use libc::pollfd;
use libc::printf;
use libc::puts;
use libc::ssize_t;
use libc::stat;
use libc::strchr;
use libc::strcpy;
use libc::termios;
use libc::time;
use libc::time_t;
use libc::FILE;
extern "C" {
  #[no_mangle]
  fn sched_yield() -> libc::c_int;

  #[no_mangle]
  fn exit(_: libc::c_int) -> !;

  #[no_mangle]
  fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;

  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;

  #[no_mangle]
  static ptr_to_globals: *mut globals;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static bb_msg_standard_input: [libc::c_char; 0];
  #[no_mangle]
  fn read_key(fd: libc::c_int, buffer: *mut libc::c_char, timeout: libc::c_int) -> int64_t;
  #[no_mangle]
  fn get_termios_and_make_raw(
    fd: libc::c_int,
    newterm: *mut termios,
    oldterm: *mut termios,
    flags: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn get_terminal_width_height(
    fd: libc::c_int,
    width: *mut libc::c_uint,
    height: *mut libc::c_uint,
  ) -> libc::c_int;
  #[no_mangle]
  fn xmalloc_ttyname(fd: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
  #[no_mangle]
  fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn tcsetattr(
    __fd: libc::c_int,
    __optional_actions: libc::c_int,
    __termios_p: *const termios,
  ) -> libc::c_int;

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
  fn ndelay_on(fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn ndelay_off(fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn bb_signals(sigs: libc::c_int, f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>);
  #[no_mangle]
  fn kill_myself_with_sig(sig: libc::c_int) -> !;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn fopen_for_write(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_cat(argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];

  /*
   * Busybox xregcomp utility routine.  This isn't in libbb.h because the
   * C library we're linking against may not support regex.h.
   *
   * Based in part on code from sash, Copyright (c) 1999 by David I. Bell
   * Permission has been granted to redistribute this code under GPL.
   *
   * Licensed under GPLv2 or later, see file LICENSE in this source tree.
   */
  #[no_mangle]
  fn regcomp_or_errmsg(
    preg: *mut regex_t,
    regex: *const libc::c_char,
    cflags: libc::c_int,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn regexec(
    __preg: *const regex_t,
    __string: *const libc::c_char,
    __nmatch: size_t,
    __pmatch: *mut regmatch_t,
    __eflags: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn regfree(__preg: *mut regex_t);
}

pub type __int64_t = libc::c_long;
pub type int64_t = __int64_t;

pub type nfds_t = libc::c_ulong;

pub type C2RustUnnamed = libc::c_uint;
pub const BB_FATAL_SIGS: C2RustUnnamed = 117503054;
pub type C2RustUnnamed_0 = libc::c_int;
pub const KEYCODE_BUFFER_SIZE: C2RustUnnamed_0 = 16;
pub const KEYCODE_CURSOR_POS: C2RustUnnamed_0 = -256;
pub const KEYCODE_ALT_D: C2RustUnnamed_0 = -45;
pub const KEYCODE_ALT_BACKSPACE: C2RustUnnamed_0 = -44;
pub const KEYCODE_ALT_LEFT: C2RustUnnamed_0 = -37;
pub const KEYCODE_ALT_RIGHT: C2RustUnnamed_0 = -36;
pub const KEYCODE_CTRL_LEFT: C2RustUnnamed_0 = -69;
pub const KEYCODE_CTRL_RIGHT: C2RustUnnamed_0 = -68;
pub const KEYCODE_D: C2RustUnnamed_0 = -13;
pub const KEYCODE_BACKSPACE: C2RustUnnamed_0 = -12;
pub const KEYCODE_PAGEDOWN: C2RustUnnamed_0 = -11;
pub const KEYCODE_PAGEUP: C2RustUnnamed_0 = -10;
pub const KEYCODE_DELETE: C2RustUnnamed_0 = -9;
pub const KEYCODE_INSERT: C2RustUnnamed_0 = -8;
pub const KEYCODE_END: C2RustUnnamed_0 = -7;
pub const KEYCODE_HOME: C2RustUnnamed_0 = -6;
pub const KEYCODE_LEFT: C2RustUnnamed_0 = -5;
pub const KEYCODE_RIGHT: C2RustUnnamed_0 = -4;
pub const KEYCODE_DOWN: C2RustUnnamed_0 = -3;
pub const KEYCODE_UP: C2RustUnnamed_0 = -2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub cur_fline: libc::c_int,
  pub kbd_fd: libc::c_int,
  pub kbd_fd_orig_flags: libc::c_int,
  pub less_gets_pos: libc::c_int,
  pub last_line_pos: size_t,
  pub max_fline: libc::c_uint,
  pub max_lineno: libc::c_uint,
  pub max_displayed_line: libc::c_uint,
  pub width: libc::c_uint,
  pub winch_counter: libc::c_uint,
  pub eof_error: ssize_t,
  pub readpos: ssize_t,
  pub readeof: ssize_t,
  pub buffer: *mut *const libc::c_char,
  pub flines: *mut *const libc::c_char,
  pub empty_line_marker: *const libc::c_char,
  pub num_files: libc::c_uint,
  pub current_file: libc::c_uint,
  pub filename: *mut libc::c_char,
  pub files: *mut *mut libc::c_char,
  pub num_lines: libc::c_int,
  pub num_marks: libc::c_uint,
  pub mark_lines: [[libc::c_uint; 2]; 15],
  pub match_lines: *mut libc::c_uint,
  pub match_pos: libc::c_int,
  pub wanted_match: libc::c_int,
  pub num_matches: libc::c_int,
  pub pattern: regex_t,
  pub pattern_valid: smallint,
  pub in_escape: smallint,
  pub winsize_err: smallint,
  pub terminated: smallint,
  pub term_orig: termios,
  pub term_less: termios,
  pub kbd_input: [libc::c_char; 16],
}
pub type regex_t = re_pattern_buffer;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
  pub buffer: *mut libc::c_uchar,
  pub allocated: libc::c_ulong,
  pub used: libc::c_ulong,
  pub syntax: reg_syntax_t,
  pub fastmap: *mut libc::c_char,
  pub translate: *mut libc::c_uchar,
  pub re_nsub: size_t,
  #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
  #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
  #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
  #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
  #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
  #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
  #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
  pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
  #[bitfield(padding)]
  pub c2rust_padding: [u8; 7],
}
pub type reg_syntax_t = libc::c_ulong;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const COMMON_BUFSIZE: C2RustUnnamed_1 = 1024;
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
  pub rm_so: regoff_t,
  pub rm_eo: regoff_t,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const TILDES: C2RustUnnamed_2 = 1;
pub const MAXLINES: C2RustUnnamed_2 = 9999999;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const LESS_STATE_MATCH_BACKWARDS: C2RustUnnamed_3 = 32768;
pub const FLAG_R: C2RustUnnamed_3 = 256;
pub const FLAG_S: C2RustUnnamed_3 = 128;
pub const FLAG_F: C2RustUnnamed_3 = 64;
pub const FLAG_I: C2RustUnnamed_3 = 32;
pub const FLAG_TILDE: C2RustUnnamed_3 = 16;
pub const FLAG_N: C2RustUnnamed_3 = 8;
pub const FLAG_m: C2RustUnnamed_3 = 4;
pub const FLAG_M: C2RustUnnamed_3 = 2;
pub const FLAG_E: C2RustUnnamed_3 = 1;
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
/* This one is 100% not cached by compiler on read access */
/* flines[] are lines read from stdin, each in malloc'ed buffer.
 * Line numbers are stored as u32 prepended to each line.
 * Pointer is adjusted so that flines[i] points directly past
 * line number. Accessor: */
/* Reset terminal input to normal */
unsafe extern "C" fn set_tty_cooked() {
  fflush_all();
  tcsetattr(
    (*ptr_to_globals).kbd_fd,
    0i32,
    &mut (*ptr_to_globals).term_orig,
  );
}
/* Move the cursor to a position (x,y), where (0,0) is the
top-left corner of the console */
unsafe extern "C" fn move_cursor(mut line: libc::c_int, mut col: libc::c_int) {
  printf(
    b"\x1b[%u;%uH\x00" as *const u8 as *const libc::c_char,
    line,
    col,
  );
}
unsafe extern "C" fn clear_line() {
  printf(
    b"\x1b[%u;0H\x1b[K\x00" as *const u8 as *const libc::c_char,
    (*ptr_to_globals)
      .max_displayed_line
      .wrapping_add(2i32 as libc::c_uint),
  );
}
unsafe extern "C" fn print_hilite(mut str: *const libc::c_char) {
  printf(
    b"\x1b[7m%s\x1b[m\x00" as *const u8 as *const libc::c_char,
    str,
  );
}
unsafe extern "C" fn print_statusline(mut str: *const libc::c_char) {
  clear_line();
  printf(
    b"\x1b[7m%.*s\x1b[m\x00" as *const u8 as *const libc::c_char,
    (*ptr_to_globals).width.wrapping_sub(1i32 as libc::c_uint),
    str,
  );
}
/* Exit the program gracefully */
unsafe extern "C" fn less_exit(mut code: libc::c_int) {
  set_tty_cooked(); /* does not return */
  if (*ptr_to_globals).kbd_fd_orig_flags & 0o4000i32 == 0 {
    ndelay_off((*ptr_to_globals).kbd_fd);
  }
  clear_line();
  if code < 0i32 {
    kill_myself_with_sig(-code);
  }
  exit(code);
}
unsafe extern "C" fn re_wrap() {
  let mut sz: libc::c_int = 0;
  let mut w: libc::c_int = (*ptr_to_globals).width as libc::c_int;
  let mut new_line_pos: libc::c_int = 0;
  let mut src_idx: libc::c_int = 0;
  let mut dst_idx: libc::c_int = 0;
  let mut new_cur_fline: libc::c_int = 0i32;
  let mut lineno: u32 = 0;
  let vla = (w + 1i32) as usize;
  let mut linebuf: Vec<libc::c_char> = ::std::vec::from_elem(0, vla);
  let mut old_flines: *mut *const libc::c_char = (*ptr_to_globals).flines;
  let mut s: *const libc::c_char = 0 as *const libc::c_char;
  let mut new_flines: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut d: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  if option_mask32 & FLAG_N as libc::c_int as libc::c_uint != 0 {
    w -= 8i32
  }
  src_idx = 0i32;
  dst_idx = 0i32;
  s = *old_flines.offset(0);
  lineno = *(s.offset(-4) as *mut u32);
  d = linebuf.as_mut_ptr();
  new_line_pos = 0i32;
  loop {
    *d = *s;
    if *d as libc::c_int != '\u{0}' as i32 {
      new_line_pos += 1;
      if *d as libc::c_int == '\t' as i32 {
        /* tab */
        new_line_pos += 7i32;
        new_line_pos &= !7i32
      }
      s = s.offset(1);
      d = d.offset(1);
      if !(new_line_pos >= w) {
        continue;
      }
      sz = 0;
      /* new line is full, create next one */
      *d = '\u{0}' as i32 as libc::c_char
    } else {
      /* *d == NUL: old line ended, go to next old one */
      free(
        (*old_flines.offset(src_idx as isize) as *mut libc::c_char).offset(-4) as *mut libc::c_void,
      );
      /* btw, convert cur_fline... */
      if (*ptr_to_globals).cur_fline == src_idx {
        new_cur_fline = dst_idx
      }
      src_idx += 1;
      /* no more lines? finish last new line (and exit the loop) */
      if !(src_idx as libc::c_uint > (*ptr_to_globals).max_fline) {
        s = *old_flines.offset(src_idx as isize);
        if !(lineno != *(s.offset(-4) as *mut u32)) {
          continue;
        }
      }
    }
    /* this is not a continuation line!
     * create next _new_ line too */
    sz = (d.wrapping_offset_from(linebuf.as_mut_ptr()) as libc::c_long + 1) as libc::c_int;
    d = (xmalloc((sz + 4i32) as size_t) as *mut libc::c_char).offset(4);
    *(d.offset(-4) as *mut u32) = lineno;
    memcpy(
      d as *mut libc::c_void,
      linebuf.as_mut_ptr() as *const libc::c_void,
      sz as libc::c_ulong,
    );
    new_flines = xrealloc_vector_helper(
      new_flines as *mut libc::c_void,
      ((::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) << 8i32)
        .wrapping_add(8i32 as libc::c_ulong) as libc::c_uint,
      dst_idx,
    ) as *mut *mut libc::c_char;
    let ref mut fresh0 = *new_flines.offset(dst_idx as isize);
    *fresh0 = d;
    dst_idx += 1;
    if new_line_pos < w {
      /* if we came here thru "goto next_new" */
      if src_idx as libc::c_uint > (*ptr_to_globals).max_fline {
        break;
      }
      lineno = *(s.offset(-4) as *mut u32)
    }
    d = linebuf.as_mut_ptr();
    new_line_pos = 0i32
  }
  free(old_flines as *mut libc::c_void);
  (*ptr_to_globals).flines = new_flines as *mut *const libc::c_char;
  (*ptr_to_globals).max_fline = (dst_idx - 1i32) as libc::c_uint;
  (*ptr_to_globals).last_line_pos = new_line_pos as size_t;
  (*ptr_to_globals).cur_fline = new_cur_fline;
  /* max_lineno is screen-size independent */
  (*ptr_to_globals).pattern_valid = 0i32 as smallint;
}
unsafe extern "C" fn at_end() -> libc::c_int {
  return if option_mask32 & FLAG_S as libc::c_int as libc::c_uint != 0 {
    !((*ptr_to_globals).cur_fline as libc::c_uint <= (*ptr_to_globals).max_fline
      && (*ptr_to_globals).max_lineno
        > (*((*(*ptr_to_globals)
          .flines
          .offset((*ptr_to_globals).cur_fline as isize))
        .offset(-4) as *mut u32))
          .wrapping_add((*ptr_to_globals).max_displayed_line)) as libc::c_int
  } else {
    !((*ptr_to_globals).max_fline
      > ((*ptr_to_globals).cur_fline as libc::c_uint)
        .wrapping_add((*ptr_to_globals).max_displayed_line)) as libc::c_int
  };
}
/* Devilishly complex routine.
 *
 * Has to deal with EOF and EPIPE on input,
 * with line wrapping, with last line not ending in '\n'
 * (possibly not ending YET!), with backspace and tabs.
 * It reads input again if last time we got an EOF (thus supporting
 * growing files) or EPIPE (watching output of slow process like make).
 *
 * Variables used:
 * flines[] - array of lines already read. Linewrap may cause
 *      one source file line to occupy several flines[n].
 * flines[max_fline] - last line, possibly incomplete.
 * terminated - 1 if flines[max_fline] is 'terminated'
 *      (if there was '\n' [which isn't stored itself, we just remember
 *      that it was seen])
 * max_lineno - last line's number, this one doesn't increment
 *      on line wrap, only on "real" new lines.
 * readbuf[0..readeof-1] - small preliminary buffer.
 * readbuf[readpos] - next character to add to current line.
 * last_line_pos - screen line position of next char to be read
 *      (takes into account tabs and backspaces)
 * eof_error - < 0 error, == 0 EOF, > 0 not EOF/error
 *
 * "git log -p | less -m" on the kernel git tree is a good test for EAGAINs,
 * "/search on very long input" and "reaching max line count" corner cases.
 */
unsafe extern "C" fn read_lines() {
  let mut current_line: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut w: libc::c_int = (*ptr_to_globals).width as libc::c_int;
  let mut last_terminated: libc::c_char = (*ptr_to_globals).terminated as libc::c_char;
  let mut last_time: time_t = 0i32 as time_t;
  let mut retry_EAGAIN: libc::c_int = 2i32;
  let mut old_max_fline: libc::c_uint = (*ptr_to_globals).max_fline;
  /* (careful: max_fline can be -1) */
  if (*ptr_to_globals)
    .max_fline
    .wrapping_add(1i32 as libc::c_uint)
    > MAXLINES as libc::c_int as libc::c_uint
  {
    return;
  }
  if option_mask32 & FLAG_N as libc::c_int as libc::c_uint != 0 {
    w -= 8i32
  }
  current_line = (xmalloc((w + 5i32) as size_t) as *mut libc::c_char).offset(4);
  p = current_line;
  if last_terminated == 0 {
    let mut cp: *const libc::c_char = *(*ptr_to_globals)
      .flines
      .offset((*ptr_to_globals).max_fline as isize);
    p = stpcpy(p, cp);
    free((cp as *mut libc::c_char).offset(-4) as *mut libc::c_void);
  /* last_line_pos is still valid from previous read_lines() */
  } else {
    (*ptr_to_globals).max_fline = (*ptr_to_globals).max_fline.wrapping_add(1); /* end of "read lines until we reach cur_fline" loop */
    (*ptr_to_globals).last_line_pos = 0i32 as size_t
  }
  let mut current_block_58: u64;
  loop
  /* read lines until we reach cur_fline or wanted_match */
  {
    *p = '\u{0}' as i32 as libc::c_char; /* end of "read chars until we have a line" loop */
    (*ptr_to_globals).terminated = 0i32 as smallint;
    loop
    /* read chars until we have a line */
    {
      let mut c: libc::c_char = 0;
      /* if no unprocessed chars left, eat more */
      if (*ptr_to_globals).readpos >= (*ptr_to_globals).readeof {
        let mut flags: libc::c_int = ndelay_on(0i32); /* ndelay_off(0) */
        loop {
          let mut t: time_t = 0;
          *bb_errno = 0i32;
          (*ptr_to_globals).eof_error = safe_read(
            0i32,
            bb_common_bufsiz1.as_mut_ptr() as *mut libc::c_void,
            COMMON_BUFSIZE as libc::c_int as size_t,
          );
          if *bb_errno != 11i32 {
            break;
          }
          t = time(0 as *mut time_t);
          if t != last_time {
            last_time = t;
            retry_EAGAIN -= 1;
            if retry_EAGAIN < 0i32 {
              break;
            }
          }
          sched_yield();
        }
        fcntl(0i32, 4i32, flags);
        (*ptr_to_globals).readpos = 0i32 as ssize_t;
        (*ptr_to_globals).readeof = (*ptr_to_globals).eof_error;
        if (*ptr_to_globals).eof_error <= 0 {
          break;
        }
        retry_EAGAIN = 1i32
      }
      c = *bb_common_bufsiz1
        .as_mut_ptr()
        .offset((*ptr_to_globals).readpos as isize);
      /* backspace? [needed for manpages] */
      /* <tab><bs> is (a) insane and */
      /* (b) harder to do correctly, so we refuse to do it */
      if c as libc::c_int == '\u{8}' as i32
        && (*ptr_to_globals).last_line_pos != 0
        && *p.offset(-1i32 as isize) as libc::c_int != '\t' as i32
      {
        (*ptr_to_globals).readpos += 1; /* eat it */
        (*ptr_to_globals).last_line_pos = (*ptr_to_globals).last_line_pos.wrapping_sub(1);
        /* was buggy (p could end up <= current_line)... */
        p = p.offset(-1);
        *p = '\u{0}' as i32 as libc::c_char
      } else {
        if option_mask32 & FLAG_R as libc::c_int as libc::c_uint != 0 {
          if c as libc::c_int == '\u{1b}' as i32 {
            current_block_58 = 8727269132939964787;
          } else if (*ptr_to_globals).in_escape != 0 {
            if (c as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32
              || c as libc::c_int == '[' as i32
              || c as libc::c_int == ';' as i32
              || c as libc::c_int == 'm' as i32
            {
              current_block_58 = 8727269132939964787;
            } else {
              /* Hmm, unexpected end of "ESC [ N ; N m" sequence */
              (*ptr_to_globals).in_escape = 0i32 as smallint;
              current_block_58 = 5141539773904409130;
            }
          } else {
            current_block_58 = 5141539773904409130;
          }
          match current_block_58 {
            5141539773904409130 => {}
            _ => {
              (*ptr_to_globals).in_escape =
                (c as libc::c_int != 'm' as i32) as libc::c_int as smallint;
              (*ptr_to_globals).readpos += 1;
              continue;
            }
          }
        }
        let mut new_last_line_pos: size_t = (*ptr_to_globals)
          .last_line_pos
          .wrapping_add(1i32 as libc::c_ulong);
        if c as libc::c_int == '\t' as i32 {
          new_last_line_pos = (new_last_line_pos as libc::c_ulong)
            .wrapping_add(7i32 as libc::c_ulong) as size_t as size_t;
          new_last_line_pos &= !7i32 as libc::c_ulong
        }
        if new_last_line_pos as libc::c_int > w {
          break;
        }
        (*ptr_to_globals).last_line_pos = new_last_line_pos;
        /* ok, we will eat this char */
        (*ptr_to_globals).readpos += 1;
        if c as libc::c_int == '\n' as i32 {
          (*ptr_to_globals).terminated = 1i32 as smallint;
          (*ptr_to_globals).last_line_pos = 0i32 as size_t;
          break;
        } else {
          /* NUL is substituted by '\n'! */
          if c as libc::c_int == '\u{0}' as i32 {
            c = '\n' as i32 as libc::c_char
          } /* Pretend we saw EOF */
          let fresh1 = p;
          p = p.offset(1);
          *fresh1 = c;
          *p = '\u{0}' as i32 as libc::c_char
        }
      }
    }
    last_terminated = (*ptr_to_globals).terminated as libc::c_char;
    (*ptr_to_globals).flines = xrealloc_vector_helper(
      (*ptr_to_globals).flines as *mut libc::c_void,
      ((::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong) << 8i32)
        .wrapping_add(8i32 as libc::c_ulong) as libc::c_uint,
      (*ptr_to_globals).max_fline as libc::c_int,
    ) as *mut *const libc::c_char;
    let ref mut fresh2 = *(*ptr_to_globals)
      .flines
      .offset((*ptr_to_globals).max_fline as isize);
    *fresh2 = (xrealloc(
      current_line.offset(-4) as *mut libc::c_void,
      strlen(current_line)
        .wrapping_add(1i32 as libc::c_ulong)
        .wrapping_add(4i32 as libc::c_ulong),
    ) as *mut libc::c_char)
      .offset(4);
    *((*(*ptr_to_globals)
      .flines
      .offset((*ptr_to_globals).max_fline as isize))
    .offset(-4) as *mut u32) = (*ptr_to_globals).max_lineno;
    if (*ptr_to_globals).terminated != 0 {
      (*ptr_to_globals).max_lineno = (*ptr_to_globals).max_lineno.wrapping_add(1)
    }
    if (*ptr_to_globals).max_fline >= MAXLINES as libc::c_int as libc::c_uint {
      (*ptr_to_globals).eof_error = 0i32 as ssize_t;
      break;
    } else {
      if at_end() == 0 {
        if (*ptr_to_globals).wanted_match >= (*ptr_to_globals).num_matches {
          /* goto_match called us */
          fill_match_lines(old_max_fline);
          old_max_fline = (*ptr_to_globals).max_fline
        }
        if (*ptr_to_globals).wanted_match < (*ptr_to_globals).num_matches {
          break;
        }
      }
      if (*ptr_to_globals).eof_error <= 0 {
        break;
      }
      (*ptr_to_globals).max_fline = (*ptr_to_globals).max_fline.wrapping_add(1);
      current_line = (xmalloc((w + 5i32) as size_t) as *mut libc::c_char).offset(4);
      p = current_line;
      (*ptr_to_globals).last_line_pos = 0i32 as size_t
    }
  }
  if (*ptr_to_globals).eof_error < 0 {
    if *bb_errno == 11i32 {
      (*ptr_to_globals).eof_error = 1i32 as ssize_t
    } else {
      print_statusline(b"read error\x00" as *const u8 as *const libc::c_char);
    }
  } else if (*ptr_to_globals).eof_error == 0 {
    (*ptr_to_globals).num_lines = (*ptr_to_globals).max_lineno as libc::c_int
  }
  fill_match_lines(old_max_fline);
  /* prevent us from being stuck in search for a match */
  (*ptr_to_globals).wanted_match = -1i32;
}
unsafe extern "C" fn safe_lineno(mut fline: libc::c_int) -> libc::c_int {
  if fline as libc::c_uint >= (*ptr_to_globals).max_fline {
    fline = (*ptr_to_globals)
      .max_fline
      .wrapping_sub(1i32 as libc::c_uint) as libc::c_int
  }
  /* also catches empty file (max_fline == 0) */
  if fline < 0i32 {
    return 0i32;
  }
  return (*((*(*ptr_to_globals).flines.offset(fline as isize)).offset(-4) as *mut u32))
    .wrapping_add(1i32 as libc::c_uint) as libc::c_int;
}
/* count number of lines in file */
unsafe extern "C" fn update_num_lines() {
  let mut count: libc::c_int = 0;
  let mut fd: libc::c_int = 0;
  let mut stbuf: stat = std::mem::zeroed();
  let mut len: ssize_t = 0;
  let mut i: ssize_t = 0;
  let mut buf: [libc::c_char; 4096] = [0; 4096];
  /* only do this for regular files */
  if (*ptr_to_globals).num_lines == -1i32 || (*ptr_to_globals).num_lines == -2i32 {
    count = 0i32;
    fd = open(
      b"/proc/self/fd/0\x00" as *const u8 as *const libc::c_char,
      0i32,
    );
    if fd < 0i32 && (*ptr_to_globals).num_lines == -1i32 {
      /* "filename" is valid only if REOPEN_AND_COUNT */
      fd = open((*ptr_to_globals).filename, 0i32)
    }
    if fd < 0i32 {
      /* somebody stole my file! */
      (*ptr_to_globals).num_lines = -3i32;
      return;
    }
    if fstat(fd, &mut stbuf) != 0i32
      || !(stbuf.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint)
    {
      (*ptr_to_globals).num_lines = -3i32
    } else {
      's_69: loop {
        len = safe_read(
          fd,
          buf.as_mut_ptr() as *mut libc::c_void,
          ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        );
        if !(len > 0) {
          break;
        }
        i = 0i32 as ssize_t;
        while i < len {
          if buf[i as usize] as libc::c_int == '\n' as i32 && {
            count += 1;
            (count) == MAXLINES as libc::c_int
          } {
            break 's_69;
          }
          i += 1
        }
      }
      (*ptr_to_globals).num_lines = count
    }
    close(fd);
  };
}
/* Print a status line if -M was specified */
unsafe extern "C" fn m_status_print() {
  let mut first: libc::c_int = 0;
  let mut last: libc::c_int = 0;
  let mut percent: libc::c_uint = 0;
  if (*ptr_to_globals).less_gets_pos >= 0i32 {
    /* don't touch statusline while input is done! */
    return;
  }
  clear_line();
  printf(
    b"\x1b[7m%s\x00" as *const u8 as *const libc::c_char,
    (*ptr_to_globals).filename,
  );
  if (*ptr_to_globals).num_files > 1i32 as libc::c_uint {
    printf(
      b" (file %i of %i)\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).current_file,
      (*ptr_to_globals).num_files,
    );
  }
  first = safe_lineno((*ptr_to_globals).cur_fline);
  last = if option_mask32 & FLAG_S as libc::c_int as libc::c_uint != 0 {
    if (first as libc::c_uint).wrapping_add((*ptr_to_globals).max_displayed_line)
      < (*ptr_to_globals).max_lineno
    {
      (first as libc::c_uint).wrapping_add((*ptr_to_globals).max_displayed_line)
    } else {
      (*ptr_to_globals).max_lineno
    }
  } else {
    safe_lineno(
      ((*ptr_to_globals).cur_fline as libc::c_uint)
        .wrapping_add((*ptr_to_globals).max_displayed_line) as libc::c_int,
    ) as libc::c_uint
  } as libc::c_int;
  printf(
    b" lines %i-%i\x00" as *const u8 as *const libc::c_char,
    first,
    last,
  );
  update_num_lines();
  if (*ptr_to_globals).num_lines >= 0i32 {
    printf(
      b"/%i\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).num_lines,
    );
  }
  if at_end() != 0 {
    printf(b" (END)\x00" as *const u8 as *const libc::c_char);
    if (*ptr_to_globals).num_files > 1i32 as libc::c_uint
      && (*ptr_to_globals).current_file != (*ptr_to_globals).num_files
    {
      printf(
        b" - next: %s\x00" as *const u8 as *const libc::c_char,
        *(*ptr_to_globals)
          .files
          .offset((*ptr_to_globals).current_file as isize),
      );
    }
  } else if (*ptr_to_globals).num_lines > 0i32 {
    percent = ((100i32 * last + (*ptr_to_globals).num_lines / 2i32) / (*ptr_to_globals).num_lines)
      as libc::c_uint;
    printf(
      b" %i%%\x00" as *const u8 as *const libc::c_char,
      if percent <= 100i32 as libc::c_uint {
        percent
      } else {
        100i32 as libc::c_uint
      },
    );
  }
  printf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
}
/* Print the status line */
unsafe extern "C" fn status_print() {
  let mut p: *const libc::c_char = 0 as *const libc::c_char;
  if (*ptr_to_globals).less_gets_pos >= 0i32 {
    /* don't touch statusline while input is done! */
    return;
  }
  /* Change the status if flags have been set */
  if option_mask32 & (FLAG_M as libc::c_int | FLAG_m as libc::c_int) as libc::c_uint != 0 {
    m_status_print();
    return;
  }
  /* No flags set */
  clear_line();
  if (*ptr_to_globals).cur_fline != 0 && at_end() == 0 {
    bb_putchar(':' as i32);
    return;
  }
  p = b"(END)\x00" as *const u8 as *const libc::c_char;
  if (*ptr_to_globals).cur_fline == 0 {
    p = (*ptr_to_globals).filename
  }
  if (*ptr_to_globals).num_files > 1i32 as libc::c_uint {
    printf(
      b"\x1b[7m%s (file %i of %i)\x1b[m\x00" as *const u8 as *const libc::c_char,
      p,
      (*ptr_to_globals).current_file,
      (*ptr_to_globals).num_files,
    );
    return;
  }
  print_hilite(p);
}
static mut controls: [libc::c_char; 33] = [
  1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
  28, 29, 30, 31, 127, -101, 0,
];
/* DEL and infamous Meta-ESC :( */
static mut ctrlconv: [libc::c_char; 33] = [
  64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 64, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87,
  88, 89, 90, 91, 92, 93, 94, 95, 0,
];
unsafe extern "C" fn print_lineno(mut line: *const libc::c_char) {
  let mut fmt: *const libc::c_char = b"        \x00" as *const u8 as *const libc::c_char; /* for compiler */
  let mut n: libc::c_uint = 0;
  n = n;
  if line != (*ptr_to_globals).empty_line_marker {
    /* Width of 7 preserves tab spacing in the text */
    fmt = b"%7u \x00" as *const u8 as *const libc::c_char;
    n = (*(line.offset(-4) as *mut u32)).wrapping_add(1i32 as libc::c_uint);
    if n > 9999999i32 as libc::c_uint && MAXLINES as libc::c_int > 9999999i32 {
      n = n.wrapping_rem(10000000i32 as libc::c_uint);
      fmt = b"%07u \x00" as *const u8 as *const libc::c_char
    }
  }
  printf(fmt, n);
}
unsafe extern "C" fn print_found(mut line: *const libc::c_char) {
  let mut new: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut match_status: libc::c_int = 0;
  let mut eflags: libc::c_int = 0;
  let mut growline: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut match_structs: regmatch_t = regmatch_t { rm_so: 0, rm_eo: 0 };
  let vla = (*ptr_to_globals).width.wrapping_add(1i32 as libc::c_uint) as usize;
  let mut buf: Vec<libc::c_char> = ::std::vec::from_elem(0, vla);
  let mut str: *const libc::c_char = line;
  let mut p: *mut libc::c_char = buf.as_mut_ptr();
  let mut n: size_t = 0;
  while *str != 0 {
    n = strcspn(str, controls.as_ptr());
    if n != 0 {
      if *str.offset(n as isize) == 0 {
        break;
      }
      memcpy(p as *mut libc::c_void, str as *const libc::c_void, n);
      p = p.offset(n as isize);
      str = str.offset(n as isize)
    }
    n = strspn(str, controls.as_ptr());
    memset(p as *mut libc::c_void, '.' as i32, n);
    p = p.offset(n as isize);
    str = str.offset(n as isize)
  }
  strcpy(p, str);
  /* buf[] holds quarantined version of str */
  /* Each part of the line that matches has the HIGHLIGHT
   * and NORMAL escape sequences placed around it.
   * NB: we regex against line, but insert text
   * from quarantined copy (buf[]) */
  str = buf.as_mut_ptr();
  growline = std::ptr::null_mut::<libc::c_char>();
  eflags = 0i32;
  loop {
    /* Most of the time doesn't find the regex, optimize for that */
    match_status = regexec(
      &mut (*ptr_to_globals).pattern,
      line,
      1i32 as size_t,
      &mut match_structs,
      eflags,
    );
    /* if even "" matches, treat it as "not a match" */
    if match_structs.rm_so >= match_structs.rm_eo {
      match_status = 1i32
    }
    if !(match_status == 0i32) {
      break;
    }
    new = xasprintf(
      b"%s%.*s\x1b[7m%.*s\x1b[m\x00" as *const u8 as *const libc::c_char,
      if !growline.is_null() {
        growline
      } else {
        b"\x00" as *const u8 as *const libc::c_char
      },
      match_structs.rm_so,
      str,
      match_structs.rm_eo - match_structs.rm_so,
      str.offset(match_structs.rm_so as isize),
    );
    free(growline as *mut libc::c_void);
    growline = new;
    str = str.offset(match_structs.rm_eo as isize);
    line = line.offset(match_structs.rm_eo as isize);
    eflags = 1i32
  }
  printf(
    b"%s%s\n\x00" as *const u8 as *const libc::c_char,
    if !growline.is_null() {
      growline
    } else {
      b"\x00" as *const u8 as *const libc::c_char
    },
    str,
  );
  free(growline as *mut libc::c_void);
}
unsafe extern "C" fn print_ascii(mut str: *const libc::c_char) {
  let vla = (*ptr_to_globals).width.wrapping_add(1i32 as libc::c_uint) as usize;
  let mut buf: Vec<libc::c_char> = ::std::vec::from_elem(0, vla);
  let mut p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut n: size_t = 0;
  while *str != 0 {
    n = strcspn(str, controls.as_ptr());
    if n != 0 {
      if *str.offset(n as isize) == 0 {
        break;
      }
      printf(
        b"%.*s\x00" as *const u8 as *const libc::c_char,
        n as libc::c_int,
        str,
      );
      str = str.offset(n as isize)
    }
    n = strspn(str, controls.as_ptr());
    p = buf.as_mut_ptr();
    loop {
      if *str as libc::c_int == 0x7fi32 {
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 = '?' as i32 as libc::c_char
      } else if *str as libc::c_int == 0x9bi32 as libc::c_char as libc::c_int {
        /* VT100's CSI, aka Meta-ESC. Who's inventor? */
        /* I want to know who committed this sin */
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = '{' as i32 as libc::c_char
      } else {
        let fresh5 = p;
        p = p.offset(1);
        *fresh5 = ctrlconv[*str as libc::c_uchar as usize]
      }
      str = str.offset(1);
      n = n.wrapping_sub(1);
      if !(n != 0) {
        break;
      }
    }
    *p = '\u{0}' as i32 as libc::c_char;
    print_hilite(buf.as_mut_ptr());
  }
  puts(str);
}
/* Print the buffer */
unsafe extern "C" fn buffer_print() {
  let mut i: libc::c_uint = 0;
  move_cursor(0i32, 0i32);
  i = 0i32 as libc::c_uint;
  while i <= (*ptr_to_globals).max_displayed_line {
    printf(b"\x1b[K\x00" as *const u8 as *const libc::c_char);
    if option_mask32 & FLAG_N as libc::c_int as libc::c_uint != 0 {
      print_lineno(*(*ptr_to_globals).buffer.offset(i as isize));
    }
    if (*ptr_to_globals).pattern_valid != 0 {
      print_found(*(*ptr_to_globals).buffer.offset(i as isize));
    } else {
      print_ascii(*(*ptr_to_globals).buffer.offset(i as isize));
    }
    i = i.wrapping_add(1)
  }
  if option_mask32 & (FLAG_E as libc::c_int | FLAG_F as libc::c_int) as libc::c_uint != 0
    && (*ptr_to_globals).eof_error <= 0
  {
    i = if option_mask32 & FLAG_F as libc::c_int as libc::c_uint != 0 {
      0i32
    } else {
      (*ptr_to_globals).cur_fline
    } as libc::c_uint;
    if (*ptr_to_globals).max_fline.wrapping_sub(i) <= (*ptr_to_globals).max_displayed_line {
      less_exit(0i32);
    }
  }
  status_print();
}
unsafe extern "C" fn buffer_fill_and_print() {
  let mut i: libc::c_uint = 0;
  let mut fpos: libc::c_int = (*ptr_to_globals).cur_fline;
  if option_mask32 & FLAG_S as libc::c_int as libc::c_uint != 0 {
    /* Go back to the beginning of this line */
    while fpos != 0
      && *((*(*ptr_to_globals).flines.offset(fpos as isize)).offset(-4) as *mut u32)
        == *((*(*ptr_to_globals).flines.offset((fpos - 1i32) as isize)).offset(-4) as *mut u32)
    {
      fpos -= 1
    }
  }
  i = 0i32 as libc::c_uint;
  while i <= (*ptr_to_globals).max_displayed_line
    && fpos as libc::c_uint <= (*ptr_to_globals).max_fline
  {
    let mut lineno: libc::c_int =
      *((*(*ptr_to_globals).flines.offset(fpos as isize)).offset(-4) as *mut u32) as libc::c_int;
    let ref mut fresh6 = *(*ptr_to_globals).buffer.offset(i as isize);
    *fresh6 = *(*ptr_to_globals).flines.offset(fpos as isize);
    i = i.wrapping_add(1);
    loop {
      fpos += 1;
      if !(fpos as libc::c_uint <= (*ptr_to_globals).max_fline
        && option_mask32 & FLAG_S as libc::c_int as libc::c_uint != 0
        && lineno as libc::c_uint
          == *((*(*ptr_to_globals).flines.offset(fpos as isize)).offset(-4) as *mut u32))
      {
        break;
      }
    }
  }
  while i <= (*ptr_to_globals).max_displayed_line {
    let ref mut fresh7 = *(*ptr_to_globals).buffer.offset(i as isize);
    *fresh7 = (*ptr_to_globals).empty_line_marker;
    i = i.wrapping_add(1)
  }
  buffer_print();
}
/* move cur_fline to a given line number, reading lines if necessary */
unsafe extern "C" fn goto_lineno(mut target: libc::c_int) {
  if target <= 0i32 {
    (*ptr_to_globals).cur_fline = 0i32
  } else if target as libc::c_uint
    > *((*(*ptr_to_globals)
      .flines
      .offset((*ptr_to_globals).cur_fline as isize))
    .offset(-4) as *mut u32)
  {
    loop {
      while *((*(*ptr_to_globals)
        .flines
        .offset((*ptr_to_globals).cur_fline as isize))
      .offset(-4) as *mut u32)
        != target as libc::c_uint
        && ((*ptr_to_globals).cur_fline as libc::c_uint) < (*ptr_to_globals).max_fline
      {
        (*ptr_to_globals).cur_fline += 1
      }
      /* target not reached but more input is available */
      if !(*((*(*ptr_to_globals)
        .flines
        .offset((*ptr_to_globals).cur_fline as isize))
      .offset(-4) as *mut u32)
        != target as libc::c_uint
        && (*ptr_to_globals).eof_error > 0)
      {
        break;
      }
      read_lines();
    }
  } else {
    /* search backwards through already-read lines */
    while *((*(*ptr_to_globals)
      .flines
      .offset((*ptr_to_globals).cur_fline as isize))
    .offset(-4) as *mut u32)
      != target as libc::c_uint
      && (*ptr_to_globals).cur_fline > 0i32
    {
      (*ptr_to_globals).cur_fline -= 1
    }
  };
}
unsafe extern "C" fn cap_cur_fline() {
  if option_mask32 & FLAG_S as libc::c_int as libc::c_uint != 0 {
    if (*ptr_to_globals).cur_fline as libc::c_uint > (*ptr_to_globals).max_fline {
      (*ptr_to_globals).cur_fline = (*ptr_to_globals).max_fline as libc::c_int
    }
    if (*((*(*ptr_to_globals)
      .flines
      .offset((*ptr_to_globals).cur_fline as isize))
    .offset(-4) as *mut u32))
      .wrapping_add((*ptr_to_globals).max_displayed_line)
      > (*ptr_to_globals)
        .max_lineno
        .wrapping_add(TILDES as libc::c_int as libc::c_uint)
    {
      goto_lineno(
        (*ptr_to_globals)
          .max_lineno
          .wrapping_sub((*ptr_to_globals).max_displayed_line)
          .wrapping_add(TILDES as libc::c_int as libc::c_uint) as libc::c_int,
      );
      read_lines();
    }
  } else {
    if ((*ptr_to_globals).cur_fline as libc::c_uint)
      .wrapping_add((*ptr_to_globals).max_displayed_line)
      > (*ptr_to_globals)
        .max_fline
        .wrapping_add(TILDES as libc::c_int as libc::c_uint)
    {
      (*ptr_to_globals).cur_fline = (*ptr_to_globals)
        .max_fline
        .wrapping_sub((*ptr_to_globals).max_displayed_line)
        .wrapping_add(TILDES as libc::c_int as libc::c_uint)
        as libc::c_int
    }
    if (*ptr_to_globals).cur_fline < 0i32 {
      (*ptr_to_globals).cur_fline = 0i32
    }
  };
}
/* Move the buffer up and down in the file in order to scroll */
unsafe extern "C" fn buffer_down(mut nlines: libc::c_int) {
  if option_mask32 & FLAG_S as libc::c_int as libc::c_uint != 0 {
    goto_lineno(
      (*((*(*ptr_to_globals)
        .flines
        .offset((*ptr_to_globals).cur_fline as isize))
      .offset(-4) as *mut u32))
        .wrapping_add(nlines as libc::c_uint) as libc::c_int,
    );
  } else {
    (*ptr_to_globals).cur_fline += nlines
  }
  read_lines();
  cap_cur_fline();
  buffer_fill_and_print();
}
unsafe extern "C" fn buffer_up(mut nlines: libc::c_int) {
  if option_mask32 & FLAG_S as libc::c_int as libc::c_uint != 0 {
    goto_lineno(
      (*((*(*ptr_to_globals)
        .flines
        .offset((*ptr_to_globals).cur_fline as isize))
      .offset(-4) as *mut u32))
        .wrapping_sub(nlines as libc::c_uint) as libc::c_int,
    );
  } else {
    (*ptr_to_globals).cur_fline -= nlines;
    if (*ptr_to_globals).cur_fline < 0i32 {
      (*ptr_to_globals).cur_fline = 0i32
    }
  }
  read_lines();
  buffer_fill_and_print();
}
/* display a given line where the argument can be either an index into
 * the flines array or a line number */
unsafe extern "C" fn buffer_to_line(mut linenum: libc::c_int, mut is_lineno: libc::c_int) {
  if linenum <= 0i32 {
    (*ptr_to_globals).cur_fline = 0i32
  } else if is_lineno != 0 {
    goto_lineno(linenum);
  } else {
    (*ptr_to_globals).cur_fline = linenum
  }
  read_lines();
  cap_cur_fline();
  buffer_fill_and_print();
}
unsafe extern "C" fn buffer_line(mut linenum: libc::c_int) {
  buffer_to_line(linenum, 0i32);
}
unsafe extern "C" fn buffer_lineno(mut lineno: libc::c_int) {
  buffer_to_line(lineno, 1i32);
}
unsafe extern "C" fn open_file_and_read_lines() {
  if !(*ptr_to_globals).filename.is_null() {
    xmove_fd(xopen((*ptr_to_globals).filename, 0i32), 0i32);
    (*ptr_to_globals).num_lines = -1i32
  } else {
    /* "less" with no arguments in argv[] */
    /* For status line only */
    (*ptr_to_globals).filename = xstrdup(bb_msg_standard_input.as_ptr());
    (*ptr_to_globals).num_lines = -2i32
  }
  (*ptr_to_globals).readpos = 0i32 as ssize_t;
  (*ptr_to_globals).readeof = 0i32 as ssize_t;
  (*ptr_to_globals).last_line_pos = 0i32 as size_t;
  (*ptr_to_globals).terminated = 1i32 as smallint;
  read_lines();
}
/* Reinitialize everything for a new file - free the memory and start over */
unsafe extern "C" fn reinitialize() {
  let mut i: libc::c_uint = 0;
  if !(*ptr_to_globals).flines.is_null() {
    i = 0i32 as libc::c_uint;
    while i <= (*ptr_to_globals).max_fline {
      free(
        (*(*ptr_to_globals).flines.offset(i as isize) as *mut libc::c_char).offset(-4)
          as *mut libc::c_void,
      );
      i = i.wrapping_add(1)
    }
    free((*ptr_to_globals).flines as *mut libc::c_void);
    (*ptr_to_globals).flines = 0 as *mut *const libc::c_char
  }
  (*ptr_to_globals).max_fline = -1i32 as libc::c_uint;
  (*ptr_to_globals).cur_fline = 0i32;
  (*ptr_to_globals).max_lineno = 0i32 as libc::c_uint;
  open_file_and_read_lines();
  if (*ptr_to_globals).winsize_err != 0 {
    printf(b"\x1b[999;999H\x1b[6n\x00" as *const u8 as *const libc::c_char);
  }
  buffer_fill_and_print();
}
unsafe extern "C" fn getch_nowait() -> int64_t {
  let mut rd: libc::c_int = 0;
  let mut key64: int64_t = 0;
  let mut pfd: [pollfd; 2] = [pollfd {
    fd: 0,
    events: 0,
    revents: 0,
  }; 2];
  pfd[0].fd = 0i32;
  pfd[0].events = 0x1i32 as libc::c_short;
  pfd[1].fd = (*ptr_to_globals).kbd_fd;
  pfd[1].events = 0x1i32 as libc::c_short;
  loop {
    tcsetattr(
      (*ptr_to_globals).kbd_fd,
      0i32,
      &mut (*ptr_to_globals).term_less,
    );
    /* NB: select/poll returns whenever read will not block. Therefore:
     * if eof is reached, select/poll will return immediately
     * because read will immediately return 0 bytes.
     * Even if select/poll says that input is available, read CAN block
     * (switch fd into O_NONBLOCK'ed mode to avoid it)
     */
    rd = 1i32;
    /* Are we interested in stdin? */
    if at_end() != 0 {
      if (*ptr_to_globals).eof_error > 0 {
        /* did NOT reach eof yet */
        rd = 0i32
      }
      /* yes, we are interested in stdin */
    }
    /* Position cursor if line input is done */
    if (*ptr_to_globals).less_gets_pos >= 0i32 {
      move_cursor(
        (*ptr_to_globals)
          .max_displayed_line
          .wrapping_add(2i32 as libc::c_uint) as libc::c_int,
        (*ptr_to_globals).less_gets_pos + 1i32,
      );
    }
    fflush_all();
    if (*ptr_to_globals).kbd_input[0] as libc::c_int == 0i32 {
      loop
      /* if nothing is buffered */
      {
        let mut r: libc::c_int = 0;
        /* NB: SIGWINCH interrupts poll() */
        r = poll(
          pfd.as_mut_ptr().offset(rd as isize),
          (2i32 - rd) as nfds_t,
          -1i32,
        ); /* anything which has no defined function */
        if (*ptr_to_globals).winch_counter != 0 {
          return '\\' as i32 as int64_t;
        }
        if r != 0 {
          break;
        }
      }
    }
    /* We have kbd_fd in O_NONBLOCK mode, read inside read_key()
     * would not block even if there is no input available */
    key64 = read_key(
      (*ptr_to_globals).kbd_fd,
      (*ptr_to_globals).kbd_input.as_mut_ptr(),
      -2i32,
    );
    if !(key64 as libc::c_int == -1i32) {
      break;
    }
    if *bb_errno == 11i32 {
      /* No keyboard input available. Since poll() did return,
       * we should have input on stdin */
      read_lines();
      buffer_fill_and_print();
    } else {
      /* EOF/error (ssh session got killed etc) */
      less_exit(0i32);
      break;
    }
  }
  set_tty_cooked();
  return key64;
}
/* Grab a character from input without requiring the return key.
 * May return KEYCODE_xxx values.
 * Note that this function works best with raw input. */
unsafe extern "C" fn less_getch(mut pos: libc::c_int) -> int64_t {
  let mut key64: int64_t = 0;
  let mut key: libc::c_int = 0;
  loop {
    (*ptr_to_globals).less_gets_pos = pos;
    key64 = getch_nowait();
    key = key64 as libc::c_int;
    (*ptr_to_globals).less_gets_pos = -1i32;
    /* Discard Ctrl-something chars.
     * (checking only lower 32 bits is a size optimization:
     * upper 32 bits are used only by KEYCODE_CURSOR_POS)
     */
    if !(key >= 0i32 && key < ' ' as i32 && key != 0xdi32 && key != 8i32) {
      break;
    }
  }
  return key64;
}
unsafe extern "C" fn less_gets(mut sz: libc::c_int) -> *mut libc::c_char {
  let mut c: libc::c_int = 0;
  let mut i: libc::c_uint = 0i32 as libc::c_uint;
  let mut result: *mut libc::c_char = xzalloc(1i32 as size_t) as *mut libc::c_char;
  loop
  /* filters out KEYCODE_xxx too (<0) */
  {
    c = '\u{0}' as i32; /* len limit */
    (*ptr_to_globals).less_gets_pos = (sz as libc::c_uint).wrapping_add(i) as libc::c_int;
    c = getch_nowait() as libc::c_int;
    if c == 0xdi32 {
      *result.offset(i as isize) = '\u{0}' as i32 as libc::c_char;
      (*ptr_to_globals).less_gets_pos = -1i32;
      return result;
    }
    if c == 0x7fi32 {
      c = 8i32
    }
    if c == 8i32 && i != 0 {
      printf(b"\x08 \x08\x00" as *const u8 as *const libc::c_char);
      i = i.wrapping_sub(1)
    }
    if c < ' ' as i32 {
      continue;
    }
    if i
      >= (*ptr_to_globals)
        .width
        .wrapping_sub(sz as libc::c_uint)
        .wrapping_sub(1i32 as libc::c_uint)
    {
      continue;
    }
    bb_putchar(c);
    let fresh8 = i;
    i = i.wrapping_add(1);
    *result.offset(fresh8 as isize) = c as libc::c_char;
    result = xrealloc(
      result as *mut libc::c_void,
      i.wrapping_add(1i32 as libc::c_uint) as size_t,
    ) as *mut libc::c_char
  }
}
unsafe extern "C" fn examine_file() {
  let mut new_fname: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  print_statusline(b"Examine: \x00" as *const u8 as *const libc::c_char);
  new_fname = less_gets(
    (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int,
  );
  if *new_fname.offset(0) == 0 {
    status_print();
  } else if access(new_fname, 4i32) != 0i32 {
    print_statusline(b"Cannot read this file\x00" as *const u8 as *const libc::c_char);
  } else {
    free((*ptr_to_globals).filename as *mut libc::c_void);
    (*ptr_to_globals).filename = new_fname;
    /* files start by = argv. why we assume that argv is infinitely long??
    files[num_files] = filename;
    current_file = num_files + 1;
    num_files++; */
    let ref mut fresh9 = *(*ptr_to_globals).files.offset(0);
    *fresh9 = (*ptr_to_globals).filename;
    (*ptr_to_globals).current_file = 1i32 as libc::c_uint;
    (*ptr_to_globals).num_files = (*ptr_to_globals).current_file;
    reinitialize();
    return;
  }
  free(new_fname as *mut libc::c_void);
}
/* This function changes the file currently being paged. direction can be one of the following:
 * -1: go back one file
 *  0: go to the first file
 *  1: go forward one file */
unsafe extern "C" fn change_file(mut direction: libc::c_int) {
  if (*ptr_to_globals).current_file
    != (if direction > 0i32 {
      (*ptr_to_globals).num_files
    } else {
      1i32 as libc::c_uint
    })
  {
    (*ptr_to_globals).current_file = if direction != 0 {
      (*ptr_to_globals)
        .current_file
        .wrapping_add(direction as libc::c_uint)
    } else {
      1i32 as libc::c_uint
    };
    free((*ptr_to_globals).filename as *mut libc::c_void);
    (*ptr_to_globals).filename = xstrdup(
      *(*ptr_to_globals).files.offset(
        (*ptr_to_globals)
          .current_file
          .wrapping_sub(1i32 as libc::c_uint) as isize,
      ),
    );
    reinitialize();
  } else {
    print_statusline(if direction > 0i32 {
      b"No next file\x00" as *const u8 as *const libc::c_char
    } else {
      b"No previous file\x00" as *const u8 as *const libc::c_char
    });
  };
}
unsafe extern "C" fn remove_current_file() {
  let mut i: libc::c_uint = 0;
  if (*ptr_to_globals).num_files < 2i32 as libc::c_uint {
    return;
  }
  if (*ptr_to_globals).current_file != 1i32 as libc::c_uint {
    change_file(-1i32);
    i = 3i32 as libc::c_uint;
    while i <= (*ptr_to_globals).num_files {
      let ref mut fresh10 = *(*ptr_to_globals)
        .files
        .offset(i.wrapping_sub(2i32 as libc::c_uint) as isize);
      *fresh10 = *(*ptr_to_globals)
        .files
        .offset(i.wrapping_sub(1i32 as libc::c_uint) as isize);
      i = i.wrapping_add(1)
    }
    (*ptr_to_globals).num_files = (*ptr_to_globals).num_files.wrapping_sub(1)
  } else {
    change_file(1i32);
    i = 2i32 as libc::c_uint;
    while i <= (*ptr_to_globals).num_files {
      let ref mut fresh11 = *(*ptr_to_globals)
        .files
        .offset(i.wrapping_sub(2i32 as libc::c_uint) as isize);
      *fresh11 = *(*ptr_to_globals)
        .files
        .offset(i.wrapping_sub(1i32 as libc::c_uint) as isize);
      i = i.wrapping_add(1)
    }
    (*ptr_to_globals).num_files = (*ptr_to_globals).num_files.wrapping_sub(1);
    (*ptr_to_globals).current_file = (*ptr_to_globals).current_file.wrapping_sub(1)
  };
}
unsafe extern "C" fn colon_process() {
  let mut keypress: libc::c_int = 0;
  /* Clear the current line and print a prompt */
  print_statusline(b" :\x00" as *const u8 as *const libc::c_char);
  keypress = less_getch(2i32) as libc::c_int;
  match keypress {
    100 => {
      remove_current_file();
    }
    101 => {
      examine_file();
    }
    102 => {
      m_status_print();
    }
    110 => {
      change_file(1i32);
    }
    112 => {
      change_file(-1i32);
    }
    113 => {
      less_exit(0i32);
    }
    120 => {
      change_file(0i32);
    }
    _ => {}
  };
}
unsafe extern "C" fn normalize_match_pos(mut match_0: libc::c_int) {
  if match_0 >= (*ptr_to_globals).num_matches {
    match_0 = (*ptr_to_globals).num_matches - 1i32
  }
  if match_0 < 0i32 {
    match_0 = 0i32
  }
  (*ptr_to_globals).match_pos = match_0;
}
unsafe extern "C" fn goto_match(mut match_0: libc::c_int) {
  if (*ptr_to_globals).pattern_valid == 0 {
    return;
  }
  if match_0 < 0i32 {
    match_0 = 0i32
  }
  /* Try to find next match if eof isn't reached yet */
  if match_0 >= (*ptr_to_globals).num_matches && (*ptr_to_globals).eof_error > 0 {
    (*ptr_to_globals).wanted_match = match_0; /* "I want to read until I see N'th match" */
    read_lines();
  }
  if (*ptr_to_globals).num_matches != 0 {
    normalize_match_pos(match_0);
    buffer_line(
      *(*ptr_to_globals)
        .match_lines
        .offset((*ptr_to_globals).match_pos as isize) as libc::c_int,
    );
  } else {
    print_statusline(b"No matches found\x00" as *const u8 as *const libc::c_char);
  };
}
unsafe extern "C" fn fill_match_lines(mut pos: libc::c_uint) {
  if (*ptr_to_globals).pattern_valid == 0 {
    return;
  }
  /* Run the regex on each line of the current file */
  while pos <= (*ptr_to_globals).max_fline {
    /* If this line matches */
    if regexec(
      &mut (*ptr_to_globals).pattern,
      *(*ptr_to_globals).flines.offset(pos as isize),
      0i32 as size_t,
      0 as *mut regmatch_t,
      0i32,
    ) == 0i32
      && !((*ptr_to_globals).num_matches != 0
        && *(*ptr_to_globals)
          .match_lines
          .offset(((*ptr_to_globals).num_matches - 1i32) as isize)
          == pos)
    {
      (*ptr_to_globals).match_lines = xrealloc_vector_helper(
        (*ptr_to_globals).match_lines as *mut libc::c_void,
        ((::std::mem::size_of::<libc::c_uint>() as libc::c_ulong) << 8i32)
          .wrapping_add(4i32 as libc::c_ulong) as libc::c_uint,
        (*ptr_to_globals).num_matches,
      ) as *mut libc::c_uint;
      let fresh12 = (*ptr_to_globals).num_matches;
      (*ptr_to_globals).num_matches = (*ptr_to_globals).num_matches + 1;
      *(*ptr_to_globals).match_lines.offset(fresh12 as isize) = pos
    }
    pos = pos.wrapping_add(1)
  }
}
unsafe extern "C" fn regex_process() {
  let mut uncomp_regex: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut err: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  /* Reset variables */
  free((*ptr_to_globals).match_lines as *mut libc::c_void);
  (*ptr_to_globals).match_lines = 0 as *mut libc::c_uint;
  (*ptr_to_globals).match_pos = 0i32;
  (*ptr_to_globals).num_matches = 0i32;
  if (*ptr_to_globals).pattern_valid != 0 {
    regfree(&mut (*ptr_to_globals).pattern);
    (*ptr_to_globals).pattern_valid = 0i32 as smallint
  }
  /* Get the uncompiled regular expression from the user */
  clear_line();
  bb_putchar(
    if option_mask32 & LESS_STATE_MATCH_BACKWARDS as libc::c_int as libc::c_uint != 0 {
      '?' as i32
    } else {
      '/' as i32
    },
  );
  uncomp_regex = less_gets(1i32);
  if *uncomp_regex.offset(0) == 0 {
    free(uncomp_regex as *mut libc::c_void);
    buffer_print();
    return;
  }
  /* Compile the regex and check for errors */
  err = regcomp_or_errmsg(
    &mut (*ptr_to_globals).pattern,
    uncomp_regex,
    if option_mask32 & FLAG_I as libc::c_int as libc::c_uint != 0 {
      (1i32) << 1i32
    } else {
      0i32
    },
  );
  free(uncomp_regex as *mut libc::c_void);
  if !err.is_null() {
    print_statusline(err);
    free(err as *mut libc::c_void);
    return;
  }
  (*ptr_to_globals).pattern_valid = 1i32 as smallint;
  (*ptr_to_globals).match_pos = 0i32;
  fill_match_lines(0i32 as libc::c_uint);
  while (*ptr_to_globals).match_pos < (*ptr_to_globals).num_matches {
    if *(*ptr_to_globals)
      .match_lines
      .offset((*ptr_to_globals).match_pos as isize) as libc::c_int
      > (*ptr_to_globals).cur_fline
    {
      break;
    }
    (*ptr_to_globals).match_pos += 1
  }
  if option_mask32 & LESS_STATE_MATCH_BACKWARDS as libc::c_int as libc::c_uint != 0 {
    (*ptr_to_globals).match_pos -= 1
  }
  /* It's possible that no matches are found yet.
   * goto_match() will read input looking for match,
   * if needed */
  goto_match((*ptr_to_globals).match_pos); /* more than enough */
}
unsafe extern "C" fn number_process(mut first_digit: libc::c_int) {
  let mut i: libc::c_uint = 0;
  let mut num: libc::c_int = 0;
  let mut keypress: libc::c_int = 0;
  let mut num_input: [libc::c_char; 16] = [0; 16];
  num_input[0] = first_digit as libc::c_char;
  /* Clear the current line, print a prompt, and then print the digit */
  clear_line();
  printf(b":%c\x00" as *const u8 as *const libc::c_char, first_digit);
  /* Receive input until a letter is given */
  i = 1i32 as libc::c_uint;
  while (i as libc::c_ulong)
    < (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong)
  {
    keypress = less_getch(i.wrapping_add(1i32 as libc::c_uint) as libc::c_int) as libc::c_int;
    if keypress as libc::c_uint > 255i32 as libc::c_uint
      || !((keypress - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32)
    {
      break;
    }
    num_input[i as usize] = keypress as libc::c_char;
    bb_putchar(keypress);
    i = i.wrapping_add(1)
  }
  num_input[i as usize] = '\u{0}' as i32 as libc::c_char;
  num = bb_strtou(num_input.as_mut_ptr(), 0 as *mut *mut libc::c_char, 10i32) as libc::c_int;
  /* on format error, num == -1 */
  if num < 1i32 || num > MAXLINES as libc::c_int {
    buffer_print();
    return;
  }
  /* We now know the number and the letter entered, so we process them */
  match keypress {
    -3 | 122 | 100 | 101 | 32 | 13 => {
      buffer_down(num);
    }
    -2 | 98 | 119 | 121 | 117 => {
      buffer_up(num);
    }
    103 | 60 | 71 | 62 => {
      buffer_lineno(num - 1i32);
    }
    112 | 37 => {
      update_num_lines();
      num = (num as libc::c_uint)
        .wrapping_mul(if (*ptr_to_globals).num_lines > 0i32 {
          (*ptr_to_globals).num_lines as libc::c_uint
        } else {
          (*ptr_to_globals).max_lineno
        })
        .wrapping_div(100i32 as libc::c_uint) as libc::c_int;
      buffer_lineno(num);
    }
    110 => {
      goto_match((*ptr_to_globals).match_pos + num);
    }
    47 => {
      option_mask32 &= !(LESS_STATE_MATCH_BACKWARDS as libc::c_int) as libc::c_uint;
      regex_process();
    }
    63 => {
      option_mask32 |= LESS_STATE_MATCH_BACKWARDS as libc::c_int as libc::c_uint;
      regex_process();
    }
    _ => {}
  };
}
unsafe extern "C" fn flag_change() {
  let mut keypress: libc::c_int = 0;
  clear_line();
  bb_putchar('-' as i32);
  keypress = less_getch(1i32) as libc::c_int;
  match keypress {
    77 => option_mask32 ^= FLAG_M as libc::c_int as libc::c_uint,
    109 => option_mask32 ^= FLAG_m as libc::c_int as libc::c_uint,
    69 => option_mask32 ^= FLAG_E as libc::c_int as libc::c_uint,
    126 => option_mask32 ^= FLAG_TILDE as libc::c_int as libc::c_uint,
    83 => {
      option_mask32 ^= FLAG_S as libc::c_int as libc::c_uint;
      buffer_fill_and_print();
    }
    78 => {
      option_mask32 ^= FLAG_N as libc::c_int as libc::c_uint;
      re_wrap();
      buffer_fill_and_print();
    }
    _ => {}
  };
}
/* ENABLE_FEATURE_LESS_DASHCMD */
unsafe extern "C" fn save_input_to_file() {
  let mut msg: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
  let mut current_line: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut i: libc::c_uint = 0;
  let mut fp: *mut FILE = 0 as *mut FILE;
  print_statusline(b"Log file: \x00" as *const u8 as *const libc::c_char);
  current_line = less_gets(
    (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int,
  );
  if *current_line.offset(0) != 0 {
    fp = fopen_for_write(current_line);
    if fp.is_null() {
      msg = b"Error opening log file\x00" as *const u8 as *const libc::c_char
    } else {
      i = 0i32 as libc::c_uint;
      while i <= (*ptr_to_globals).max_fline {
        fprintf(
          fp,
          b"%s\n\x00" as *const u8 as *const libc::c_char,
          *(*ptr_to_globals).flines.offset(i as isize),
        );
        i = i.wrapping_add(1)
      }
      fclose(fp);
      msg = b"Done\x00" as *const u8 as *const libc::c_char
    }
  }
  print_statusline(msg);
  free(current_line as *mut libc::c_void);
}
unsafe extern "C" fn add_mark() {
  let mut letter: libc::c_int = 0;
  print_statusline(b"Mark: \x00" as *const u8 as *const libc::c_char);
  letter = less_getch(
    (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int,
  ) as libc::c_int;
  if ((letter | 0x20i32) - 'a' as i32) as libc::c_uchar as libc::c_int <= 'z' as i32 - 'a' as i32 {
    /* If we exceed 15 marks, start overwriting previous ones */
    if (*ptr_to_globals).num_marks == 14i32 as libc::c_uint {
      (*ptr_to_globals).num_marks = 0i32 as libc::c_uint
    }
    (*ptr_to_globals).mark_lines[(*ptr_to_globals).num_marks as usize][0] = letter as libc::c_uint;
    (*ptr_to_globals).mark_lines[(*ptr_to_globals).num_marks as usize][1] =
      (*ptr_to_globals).cur_fline as libc::c_uint;
    (*ptr_to_globals).num_marks = (*ptr_to_globals).num_marks.wrapping_add(1)
  } else {
    print_statusline(b"Invalid mark letter\x00" as *const u8 as *const libc::c_char);
  };
}
unsafe extern "C" fn goto_mark() {
  let mut letter: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  print_statusline(b"Go to mark: \x00" as *const u8 as *const libc::c_char);
  letter = less_getch(
    (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int,
  ) as libc::c_int;
  clear_line();
  if ((letter | 0x20i32) - 'a' as i32) as libc::c_uchar as libc::c_int <= 'z' as i32 - 'a' as i32 {
    i = 0i32;
    while i as libc::c_uint <= (*ptr_to_globals).num_marks {
      if letter as libc::c_uint == (*ptr_to_globals).mark_lines[i as usize][0] {
        buffer_line((*ptr_to_globals).mark_lines[i as usize][1] as libc::c_int);
        break;
      } else {
        i += 1
      }
    }
    if (*ptr_to_globals).num_marks == 14i32 as libc::c_uint
      && letter as libc::c_uint != (*ptr_to_globals).mark_lines[14][0]
    {
      print_statusline(b"Mark not set\x00" as *const u8 as *const libc::c_char);
    }
  } else {
    print_statusline(b"Invalid mark letter\x00" as *const u8 as *const libc::c_char);
  };
}
unsafe extern "C" fn opp_bracket(mut bracket: libc::c_char) -> libc::c_char {
  let mut current_block_3: u64;
  match bracket as libc::c_int {
    123 | 91 => {
      /* '}' == '{' + 2. Same for '[' */
      bracket += 1;
      current_block_3 = 12521119961513288483;
    }
    40 => {
      current_block_3 = 12521119961513288483;
    }
    125 | 93 => {
      bracket -= 1;
      current_block_3 = 549072370927991277;
    }
    41 => {
      current_block_3 = 549072370927991277;
    }
    _ => {
      current_block_3 = 7351195479953500246;
    }
  }
  match current_block_3 {
    12521119961513288483 => {
      /* ')' == '(' + 1 */
      bracket += 1
    }
    549072370927991277 => bracket -= 1,
    _ => {}
  }
  return bracket;
}
unsafe extern "C" fn match_right_bracket(mut bracket: libc::c_char) {
  let mut i: libc::c_uint = (*ptr_to_globals).cur_fline as libc::c_uint;
  if i >= (*ptr_to_globals).max_fline
    || strchr(
      *(*ptr_to_globals).flines.offset(i as isize),
      bracket as libc::c_int,
    )
    .is_null()
  {
    print_statusline(b"No bracket in top line\x00" as *const u8 as *const libc::c_char);
    return;
  }
  bracket = opp_bracket(bracket);
  while i < (*ptr_to_globals).max_fline {
    if !strchr(
      *(*ptr_to_globals).flines.offset(i as isize),
      bracket as libc::c_int,
    )
    .is_null()
    {
      /*
       * Line with matched right bracket becomes
       * last visible line
       */
      buffer_line(i.wrapping_sub((*ptr_to_globals).max_displayed_line) as libc::c_int);
      return;
    }
    i = i.wrapping_add(1)
  }
  print_statusline(b"No matching bracket found\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn match_left_bracket(mut bracket: libc::c_char) {
  let mut i: libc::c_int = ((*ptr_to_globals).cur_fline as libc::c_uint)
    .wrapping_add((*ptr_to_globals).max_displayed_line) as libc::c_int;
  if i as libc::c_uint >= (*ptr_to_globals).max_fline
    || strchr(
      *(*ptr_to_globals).flines.offset(i as isize),
      bracket as libc::c_int,
    )
    .is_null()
  {
    print_statusline(b"No bracket in bottom line\x00" as *const u8 as *const libc::c_char);
    return;
  }
  bracket = opp_bracket(bracket);
  while i >= 0i32 {
    if !strchr(
      *(*ptr_to_globals).flines.offset(i as isize),
      bracket as libc::c_int,
    )
    .is_null()
    {
      /*
       * Line with matched left bracket becomes
       * first visible line
       */
      buffer_line(i);
      return;
    }
    i -= 1
  }
  print_statusline(b"No matching bracket found\x00" as *const u8 as *const libc::c_char);
}
/* FEATURE_LESS_BRACKETS */
unsafe extern "C" fn keypress_process(mut keypress: libc::c_int) {
  match keypress {
    -3 | 101 | 106 | 13 => {
      buffer_down(1i32);
    }
    -2 | 121 | 107 => {
      buffer_up(1i32);
    }
    -11 | 32 | 122 | 102 => {
      buffer_down(
        (*ptr_to_globals)
          .max_displayed_line
          .wrapping_add(1i32 as libc::c_uint) as libc::c_int,
      );
    }
    -10 | 119 | 98 => {
      buffer_up(
        (*ptr_to_globals)
          .max_displayed_line
          .wrapping_add(1i32 as libc::c_uint) as libc::c_int,
      );
    }
    100 => {
      buffer_down(
        (*ptr_to_globals)
          .max_displayed_line
          .wrapping_add(1i32 as libc::c_uint)
          .wrapping_div(2i32 as libc::c_uint) as libc::c_int,
      );
    }
    117 => {
      buffer_up(
        (*ptr_to_globals)
          .max_displayed_line
          .wrapping_add(1i32 as libc::c_uint)
          .wrapping_div(2i32 as libc::c_uint) as libc::c_int,
      );
    }
    -6 | 103 | 112 | 60 | 37 => {
      buffer_line(0i32);
    }
    -7 | 71 | 62 => {
      (*ptr_to_globals).cur_fline = MAXLINES as libc::c_int;
      read_lines();
      buffer_line((*ptr_to_globals).cur_fline);
    }
    113 | 81 => {
      less_exit(0i32);
    }
    109 => {
      add_mark();
      buffer_print();
    }
    39 => {
      goto_mark();
      buffer_print();
    }
    114 | 82 => {
      /* TODO: (1) also bind ^R, ^L to this?
       * (2) re-measure window size?
       */
      buffer_print();
    }
    115 => {
      /*case 'R':
      full_repaint();
      break;*/
      save_input_to_file();
    }
    69 => {
      examine_file();
    }
    61 => {
      m_status_print();
    }
    47 => {
      option_mask32 &= !(LESS_STATE_MATCH_BACKWARDS as libc::c_int) as libc::c_uint;
      regex_process();
    }
    110 => {
      goto_match((*ptr_to_globals).match_pos + 1i32);
    }
    78 => {
      goto_match((*ptr_to_globals).match_pos - 1i32);
    }
    63 => {
      option_mask32 |= LESS_STATE_MATCH_BACKWARDS as libc::c_int as libc::c_uint;
      regex_process();
    }
    45 => {
      flag_change();
      buffer_print();
    }
    123 | 40 | 91 => {
      match_right_bracket(keypress as libc::c_char);
    }
    125 | 41 | 93 => {
      match_left_bracket(keypress as libc::c_char);
    }
    58 => {
      colon_process();
    }
    _ => {}
  }
  if (keypress - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
    number_process(keypress);
  };
}
unsafe extern "C" fn sig_catcher(mut sig: libc::c_int) {
  less_exit(-sig);
}
unsafe extern "C" fn sigwinch_handler(mut _sig: libc::c_int) {
  (*ptr_to_globals).winch_counter = (*ptr_to_globals).winch_counter.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn less_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut tty_name: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut tty_fd: libc::c_int = 0;
  let ref mut fresh13 =
    *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
      as *mut *mut globals);
  *fresh13 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  (*ptr_to_globals).less_gets_pos = -1i32;
  (*ptr_to_globals).empty_line_marker = b"~\x00" as *const u8 as *const libc::c_char;
  (*ptr_to_globals).current_file = 1i32 as libc::c_uint;
  (*ptr_to_globals).eof_error = 1i32 as ssize_t;
  (*ptr_to_globals).terminated = 1i32 as smallint;
  (*ptr_to_globals).wanted_match = -1i32;
  /* TODO: -x: do not interpret backspace, -xx: tab also
   * -xxx: newline also
   * -w N: assume width N (-xxx -w 32: hex viewer of sorts)
   * -s: condense many empty lines to one
   *     (used by some setups for manpage display)
   */
  getopt32(argv, b"EMmN~IFSRs\x00" as *const u8 as *const libc::c_char);
  argv = argv.offset(optind as isize);
  (*ptr_to_globals).num_files = (argc - optind) as libc::c_uint;
  (*ptr_to_globals).files = argv;
  /* Tools typically pass LESS="FRSXMK".
   * The options we don't understand are ignored. */
  let mut c: *mut libc::c_char = getenv(b"LESS\x00" as *const u8 as *const libc::c_char);
  if !c.is_null() {
    while *c != 0 {
      let fresh14 = c;
      c = c.offset(1);
      match *fresh14 as libc::c_int {
        70 => option_mask32 |= FLAG_F as libc::c_int as libc::c_uint,
        77 => option_mask32 |= FLAG_M as libc::c_int as libc::c_uint,
        82 => option_mask32 |= FLAG_R as libc::c_int as libc::c_uint,
        83 => option_mask32 |= FLAG_S as libc::c_int as libc::c_uint,
        _ => {}
      }
    }
  }
  /* Another popular pager, most, detects when stdout
   * is not a tty and turns into cat. This makes sense. */
  if isatty(1i32) == 0 {
    return bb_cat(argv);
  }
  if (*ptr_to_globals).num_files == 0 {
    if isatty(0i32) != 0 {
      /* Just "less"? No args and no redirection? */
      bb_show_usage();
    }
  } else {
    (*ptr_to_globals).filename = xstrdup(*(*ptr_to_globals).files.offset(0))
  }
  if option_mask32 & FLAG_TILDE as libc::c_int as libc::c_uint != 0 {
    (*ptr_to_globals).empty_line_marker = b"\x00" as *const u8 as *const libc::c_char
  }
  /* Some versions of less can survive w/o controlling tty,
   * try to do the same. This also allows to specify an alternative
   * tty via "less 1<>TTY".
   *
   * We prefer not to use STDOUT_FILENO directly,
   * since we want to set this fd to non-blocking mode,
   * and not interfere with other processes which share stdout with us.
   */
  tty_name = xmalloc_ttyname(1i32);
  let mut current_block_42: u64;
  if !tty_name.is_null() {
    tty_fd = open(tty_name, 0i32);
    free(tty_name as *mut libc::c_void);
    if tty_fd < 0i32 {
      current_block_42 = 904142995672188764;
    } else {
      current_block_42 = 313581471991351815;
    }
  } else {
    current_block_42 = 904142995672188764;
  }
  match current_block_42 {
    904142995672188764 =>
    /* Try controlling tty */
    {
      tty_fd = open(b"/dev/tty\x00" as *const u8 as *const libc::c_char, 0i32);
      if tty_fd < 0i32 {
        /* If all else fails, less 481 uses stdout. Mimic that */
        tty_fd = 1i32
      }
    }
    _ => {}
  } /* save in a global */
  (*ptr_to_globals).kbd_fd_orig_flags = ndelay_on(tty_fd);
  (*ptr_to_globals).kbd_fd = tty_fd;
  get_termios_and_make_raw(
    tty_fd,
    &mut (*ptr_to_globals).term_less,
    &mut (*ptr_to_globals).term_orig,
    1i32 << 1i32,
  );
  (*ptr_to_globals).winsize_err = get_terminal_width_height(
    tty_fd,
    &mut (*ptr_to_globals).width,
    &mut (*ptr_to_globals).max_displayed_line,
  ) as smallint;
  /* 20: two tabstops + 4 */
  if (*ptr_to_globals).width < 20i32 as libc::c_uint
    || (*ptr_to_globals).max_displayed_line < 3i32 as libc::c_uint
  {
    return bb_cat(argv);
  }
  (*ptr_to_globals).max_displayed_line = (*ptr_to_globals)
    .max_displayed_line
    .wrapping_sub(2i32 as libc::c_uint);
  /* We want to restore term_orig on exit */
  bb_signals(
    BB_FATAL_SIGS as libc::c_int,
    Some(sig_catcher as unsafe extern "C" fn(_: libc::c_int) -> ()),
  ); /* -1: do not position cursor */
  signal(
    28i32,
    Some(sigwinch_handler as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  (*ptr_to_globals).buffer = xmalloc(
    ((*ptr_to_globals)
      .max_displayed_line
      .wrapping_add(1i32 as libc::c_uint) as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
  ) as *mut *const libc::c_char;
  reinitialize();
  loop {
    let mut keypress: int64_t = 0;
    loop {
      if *(&mut (*ptr_to_globals).winch_counter as *mut libc::c_uint as *mut libc::c_uint) != 0 {
        current_block = 15393578494707868645;
      } else {
        keypress = less_getch(-1i32);
        if !(keypress as i32 == KEYCODE_CURSOR_POS as libc::c_int) {
          break;
        }
        let mut rc: u32 = (keypress >> 32i32) as u32;
        (*ptr_to_globals).width = rc & 0x7fffi32 as libc::c_uint;
        (*ptr_to_globals).max_displayed_line = rc >> 16i32 & 0x7fffi32 as libc::c_uint;
        current_block = 3466878300466805598;
      }
      loop {
        match current_block {
          3466878300466805598 => {
            /* 20: two tabstops + 4 */
            if (*ptr_to_globals).width < 20i32 as libc::c_uint {
              (*ptr_to_globals).width = 20i32 as libc::c_uint
            }
            if (*ptr_to_globals).max_displayed_line < 3i32 as libc::c_uint {
              (*ptr_to_globals).max_displayed_line = 3i32 as libc::c_uint
            }
            (*ptr_to_globals).max_displayed_line = (*ptr_to_globals)
              .max_displayed_line
              .wrapping_sub(2i32 as libc::c_uint);
            free((*ptr_to_globals).buffer as *mut libc::c_void);
            (*ptr_to_globals).buffer = xmalloc(
              ((*ptr_to_globals)
                .max_displayed_line
                .wrapping_add(1i32 as libc::c_uint) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
            ) as *mut *const libc::c_char;
            /* Avoid re-wrap and/or redraw if we already know
             * we need to do it again. These ops are expensive */
            if *(&mut (*ptr_to_globals).winch_counter as *mut libc::c_uint as *mut libc::c_uint)
              != 0
            {
              current_block = 15393578494707868645;
              continue;
            }
            re_wrap();
            if *(&mut (*ptr_to_globals).winch_counter as *mut libc::c_uint as *mut libc::c_uint)
              != 0
            {
              current_block = 15393578494707868645;
            } else {
              break;
            }
          }
          _ => {
            (*ptr_to_globals).winch_counter = (*ptr_to_globals).winch_counter.wrapping_sub(1);
            (*ptr_to_globals).winsize_err = get_terminal_width_height(
              (*ptr_to_globals).kbd_fd,
              &mut (*ptr_to_globals).width,
              &mut (*ptr_to_globals).max_displayed_line,
            ) as smallint;
            current_block = 3466878300466805598;
            /* This took some time. Loop back and check,
             * were there another SIGWINCH? */
          }
        }
      }
      buffer_fill_and_print();
    }
    keypress_process(keypress as libc::c_int);
  }
}
/*
Help text of less version 418 is below.
If you are implementing something, keeping
key and/or command line switch compatibility is a good idea:


                   SUMMARY OF LESS COMMANDS

      Commands marked with * may be preceded by a number, N.
      Notes in parentheses indicate the behavior if N is given.
  h  H                 Display this help.
  q  :q  Q  :Q  ZZ     Exit.
 ---------------------------------------------------------------------------
                           MOVING
  e  ^E  j  ^N  CR  *  Forward  one line   (or N lines).
  y  ^Y  k  ^K  ^P  *  Backward one line   (or N lines).
  f  ^F  ^V  SPACE  *  Forward  one window (or N lines).
  b  ^B  ESC-v      *  Backward one window (or N lines).
  z                 *  Forward  one window (and set window to N).
  w                 *  Backward one window (and set window to N).
  ESC-SPACE         *  Forward  one window, but don't stop at end-of-file.
  d  ^D             *  Forward  one half-window (and set half-window to N).
  u  ^U             *  Backward one half-window (and set half-window to N).
  ESC-)  RightArrow *  Left  one half screen width (or N positions).
  ESC-(  LeftArrow  *  Right one half screen width (or N positions).
  F                    Forward forever; like "tail -f".
  r  ^R  ^L            Repaint screen.
  R                    Repaint screen, discarding buffered input.
        ---------------------------------------------------
        Default "window" is the screen height.
        Default "half-window" is half of the screen height.
 ---------------------------------------------------------------------------
                          SEARCHING
  /pattern          *  Search forward for (N-th) matching line.
  ?pattern          *  Search backward for (N-th) matching line.
  n                 *  Repeat previous search (for N-th occurrence).
  N                 *  Repeat previous search in reverse direction.
  ESC-n             *  Repeat previous search, spanning files.
  ESC-N             *  Repeat previous search, reverse dir. & spanning files.
  ESC-u                Undo (toggle) search highlighting.
        ---------------------------------------------------
        Search patterns may be modified by one or more of:
        ^N or !  Search for NON-matching lines.
        ^E or *  Search multiple files (pass thru END OF FILE).
        ^F or @  Start search at FIRST file (for /) or last file (for ?).
        ^K       Highlight matches, but don't move (KEEP position).
        ^R       Don't use REGULAR EXPRESSIONS.
 ---------------------------------------------------------------------------
                           JUMPING
  g  <  ESC-<       *  Go to first line in file (or line N).
  G  >  ESC->       *  Go to last line in file (or line N).
  p  %              *  Go to beginning of file (or N percent into file).
  t                 *  Go to the (N-th) next tag.
  T                 *  Go to the (N-th) previous tag.
  {  (  [           *  Find close bracket } ) ].
  }  )  ]           *  Find open bracket { ( [.
  ESC-^F <c1> <c2>  *  Find close bracket <c2>.
  ESC-^B <c1> <c2>  *  Find open bracket <c1>
        ---------------------------------------------------
        Each "find close bracket" command goes forward to the close bracket
          matching the (N-th) open bracket in the top line.
        Each "find open bracket" command goes backward to the open bracket
          matching the (N-th) close bracket in the bottom line.
  m<letter>            Mark the current position with <letter>.
  '<letter>            Go to a previously marked position.
  ''                   Go to the previous position.
  ^X^X                 Same as '.
        ---------------------------------------------------
        A mark is any upper-case or lower-case letter.
        Certain marks are predefined:
             ^  means  beginning of the file
             $  means  end of the file
 ---------------------------------------------------------------------------
                        CHANGING FILES
  :e [file]            Examine a new file.
  ^X^V                 Same as :e.
  :n                *  Examine the (N-th) next file from the command line.
  :p                *  Examine the (N-th) previous file from the command line.
  :x                *  Examine the first (or N-th) file from the command line.
  :d                   Delete the current file from the command line list.
  =  ^G  :f            Print current file name.
 ---------------------------------------------------------------------------
                    MISCELLANEOUS COMMANDS
  -<flag>              Toggle a command line option [see OPTIONS below].
  --<name>             Toggle a command line option, by name.
  _<flag>              Display the setting of a command line option.
  __<name>             Display the setting of an option, by name.
  +cmd                 Execute the less cmd each time a new file is examined.
  !command             Execute the shell command with $SHELL.
  |Xcommand            Pipe file between current pos & mark X to shell command.
  v                    Edit the current file with $VISUAL or $EDITOR.
  V                    Print version number of "less".
 ---------------------------------------------------------------------------
                           OPTIONS
        Most options may be changed either on the command line,
        or from within less by using the - or -- command.
        Options may be given in one of two forms: either a single
        character preceded by a -, or a name preceded by --.
  -?  ........  --help
                  Display help (from command line).
  -a  ........  --search-skip-screen
                  Forward search skips current screen.
  -b [N]  ....  --buffers=[N]
                  Number of buffers.
  -B  ........  --auto-buffers
                  Don't automatically allocate buffers for pipes.
  -c  ........  --clear-screen
                  Repaint by clearing rather than scrolling.
  -d  ........  --dumb
                  Dumb terminal.
  -D [xn.n]  .  --color=xn.n
                  Set screen colors. (MS-DOS only)
  -e  -E  ....  --quit-at-eof  --QUIT-AT-EOF
                  Quit at end of file.
  -f  ........  --force
                  Force open non-regular files.
  -F  ........  --quit-if-one-screen
                  Quit if entire file fits on first screen.
  -g  ........  --hilite-search
                  Highlight only last match for searches.
  -G  ........  --HILITE-SEARCH
                  Don't highlight any matches for searches.
  -h [N]  ....  --max-back-scroll=[N]
                  Backward scroll limit.
  -i  ........  --ignore-case
                  Ignore case in searches that do not contain uppercase.
  -I  ........  --IGNORE-CASE
                  Ignore case in all searches.
  -j [N]  ....  --jump-target=[N]
                  Screen position of target lines.
  -J  ........  --status-column
                  Display a status column at left edge of screen.
  -k [file]  .  --lesskey-file=[file]
                  Use a lesskey file.
  -L  ........  --no-lessopen
                  Ignore the LESSOPEN environment variable.
  -m  -M  ....  --long-prompt  --LONG-PROMPT
                  Set prompt style.
  -n  -N  ....  --line-numbers  --LINE-NUMBERS
                  Don't use line numbers.
  -o [file]  .  --log-file=[file]
                  Copy to log file (standard input only).
  -O [file]  .  --LOG-FILE=[file]
                  Copy to log file (unconditionally overwrite).
  -p [pattern]  --pattern=[pattern]
                  Start at pattern (from command line).
  -P [prompt]   --prompt=[prompt]
                  Define new prompt.
  -q  -Q  ....  --quiet  --QUIET  --silent --SILENT
                  Quiet the terminal bell.
  -r  -R  ....  --raw-control-chars  --RAW-CONTROL-CHARS
                  Output "raw" control characters.
  -s  ........  --squeeze-blank-lines
                  Squeeze multiple blank lines.
  -S  ........  --chop-long-lines
                  Chop long lines.
  -t [tag]  ..  --tag=[tag]
                  Find a tag.
  -T [tagsfile] --tag-file=[tagsfile]
                  Use an alternate tags file.
  -u  -U  ....  --underline-special  --UNDERLINE-SPECIAL
                  Change handling of backspaces.
  -V  ........  --version
                  Display the version number of "less".
  -w  ........  --hilite-unread
                  Highlight first new line after forward-screen.
  -W  ........  --HILITE-UNREAD
                  Highlight first new line after any forward movement.
  -x [N[,...]]  --tabs=[N[,...]]
                  Set tab stops.
  -X  ........  --no-init
                  Don't use termcap init/deinit strings.
                --no-keypad
                  Don't use termcap keypad init/deinit strings.
  -y [N]  ....  --max-forw-scroll=[N]
                  Forward scroll limit.
  -z [N]  ....  --window=[N]
                  Set size of window.
  -" [c[c]]  .  --quotes=[c[c]]
                  Set shell quote characters.
  -~  ........  --tilde
                  Don't display tildes after end of file.
  -# [N]  ....  --shift=[N]
                  Horizontal scroll amount (0 = one half screen width)

 ---------------------------------------------------------------------------
                          LINE EDITING
        These keys can be used to edit text being entered
        on the "command line" at the bottom of the screen.
 RightArrow                       ESC-l     Move cursor right one character.
 LeftArrow                        ESC-h     Move cursor left one character.
 CNTL-RightArrow  ESC-RightArrow  ESC-w     Move cursor right one word.
 CNTL-LeftArrow   ESC-LeftArrow   ESC-b     Move cursor left one word.
 HOME                             ESC-0     Move cursor to start of line.
 END                              ESC-$     Move cursor to end of line.
 BACKSPACE                                  Delete char to left of cursor.
 DELETE                           ESC-x     Delete char under cursor.
 CNTL-BACKSPACE   ESC-BACKSPACE             Delete word to left of cursor.
 CNTL-DELETE      ESC-DELETE      ESC-X     Delete word under cursor.
 CNTL-U           ESC (MS-DOS only)         Delete entire line.
 UpArrow                          ESC-k     Retrieve previous command line.
 DownArrow                        ESC-j     Retrieve next command line.
 TAB                                        Complete filename & cycle.
 SHIFT-TAB                        ESC-TAB   Complete filename & reverse cycle.
 CNTL-L                                     Complete filename, list all.
*/

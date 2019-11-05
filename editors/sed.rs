use c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
use libc;
extern "C" {
  #[no_mangle]
  fn strtol(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_long;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> libc::c_int;
  #[no_mangle]
  fn unlink(__name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut stdin: *mut _IO_FILE;
  #[no_mangle]
  static mut stdout: *mut _IO_FILE;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn putc_unlocked(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn ferror_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
  #[no_mangle]
  fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
  #[no_mangle]
  fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn fchmod(__fd: libc::c_int, __mode: __mode_t) -> libc::c_int;
  #[no_mangle]
  fn skip_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xstrndup(s: *const libc::c_char, n: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn xrename(oldpath: *const libc::c_char, newpath: *const libc::c_char);
  #[no_mangle]
  fn xmkstemp(template: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn overlapping_strcpy(dst: *mut libc::c_char, src: *const libc::c_char);
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  /* Reads a line from a text file, up to a newline or NUL byte, inclusive.
   * Returns malloc'ed char*. If end is NULL '\n' isn't considered
   * end of line. If end isn't NULL, length of the chunk is stored in it.
   * Returns NULL if EOF/error.
   */
  #[no_mangle]
  fn bb_get_chunk_from_file(file: *mut FILE, end: *mut size_t) -> *mut libc::c_char;
  /* Chops off '\n' from the end, unlike fgets: */
  #[no_mangle]
  fn xmalloc_fgetline(file: *mut FILE) -> *mut libc::c_char;
  #[no_mangle]
  fn fclose_if_not_stdin(file: *mut FILE) -> libc::c_int;
  /* Prints warning to stderr and returns NULL on failure: */
  #[no_mangle]
  fn fopen_or_warn(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
  /* "Opens" stdin if filename is special, else just opens file: */
  #[no_mangle]
  fn xfopen_stdin(filename: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn xfopen_for_write(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn xfdopen_for_write(fd: libc::c_int) -> *mut FILE;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> uint32_t;
  #[no_mangle]
  fn llist_add_to_end(list_head: *mut *mut llist_t, data: *mut libc::c_void);
  #[no_mangle]
  fn llist_pop(elm: *mut *mut llist_t) -> *mut libc::c_void;
  #[no_mangle]
  static mut xfunc_error_retval: uint8_t;
  #[no_mangle]
  static mut die_func: Option<unsafe extern "C" fn() -> ()>;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  static bb_msg_requires_arg: [libc::c_char; 0];
  #[no_mangle]
  static bb_msg_standard_input: [libc::c_char; 0];
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
  #[no_mangle]
  fn regexec(
    __preg: *const regex_t,
    __string: *const libc::c_char,
    __nmatch: size_t,
    __pmatch: *mut regmatch_t,
    __eflags: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn xregcomp(preg: *mut regex_t, regex: *const libc::c_char, cflags: libc::c_int);
  #[no_mangle]
  fn sed_free_and_close_stuff();
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type smallint = libc::c_schar;
pub type size_t = libc::c_ulong;
use crate::librb::stat;
use crate::librb::timespec;
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
use crate::libbb::llist::llist_t;
//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub be_quiet: libc::c_int,
  pub regex_type: libc::c_int,
  pub nonstdout: *mut FILE,
  pub outname: *mut libc::c_char,
  pub hold_space: *mut libc::c_char,
  pub exitcode: smallint,
  pub current_input_file: libc::c_int,
  pub last_input_file: libc::c_int,
  pub input_file_list: *mut *mut libc::c_char,
  pub current_fp: *mut FILE,
  pub regmatch: [regmatch_t; 10],
  pub previous_regex_ptr: *mut regex_t,
  pub sed_cmd_head: *mut sed_cmd_t,
  pub sed_cmd_tail: *mut *mut sed_cmd_t,
  pub append_head: *mut llist_t,
  pub add_cmd_line: *mut libc::c_char,
  pub pipeline: pipeline,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pipeline {
  pub buf: *mut libc::c_char,
  pub idx: libc::c_int,
  pub len: libc::c_int,
}
pub type sed_cmd_t = sed_cmd_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct sed_cmd_s {
  pub next: *mut sed_cmd_s,
  pub beg_match: *mut regex_t,
  pub end_match: *mut regex_t,
  pub sub_match: *mut regex_t,
  pub beg_line: libc::c_int,
  pub beg_line_orig: libc::c_int,
  pub end_line: libc::c_int,
  pub end_line_orig: libc::c_int,
  pub sw_file: *mut FILE,
  pub string: *mut libc::c_char,
  pub which_match: libc::c_uint,
  #[bitfield(name = "invert", ty = "libc::c_uint", bits = "0..=0")]
  #[bitfield(name = "in_match", ty = "libc::c_uint", bits = "1..=1")]
  #[bitfield(name = "sub_p", ty = "libc::c_uint", bits = "2..=2")]
  pub invert_in_match_sub_p: [u8; 1],
  pub sw_last_char: libc::c_char,
  pub cmd: libc::c_char,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
  pub rm_so: regoff_t,
  pub rm_eo: regoff_t,
}
pub type regoff_t = libc::c_int;
pub type C2RustUnnamed = libc::c_int;
pub const REG_ERPAREN: C2RustUnnamed = 16;
pub const REG_ESIZE: C2RustUnnamed = 15;
pub const REG_EEND: C2RustUnnamed = 14;
pub const REG_BADRPT: C2RustUnnamed = 13;
pub const REG_ESPACE: C2RustUnnamed = 12;
pub const REG_ERANGE: C2RustUnnamed = 11;
pub const REG_BADBR: C2RustUnnamed = 10;
pub const REG_EBRACE: C2RustUnnamed = 9;
pub const REG_EPAREN: C2RustUnnamed = 8;
pub const REG_EBRACK: C2RustUnnamed = 7;
pub const REG_ESUBREG: C2RustUnnamed = 6;
pub const REG_EESCAPE: C2RustUnnamed = 5;
pub const REG_ECTYPE: C2RustUnnamed = 4;
pub const REG_ECOLLATE: C2RustUnnamed = 3;
pub const REG_BADPAT: C2RustUnnamed = 2;
pub const REG_NOMATCH: C2RustUnnamed = 1;
pub const REG_NOERROR: C2RustUnnamed = 0;
pub const REG_ENOSYS: C2RustUnnamed = -1;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const OPT_in_place: C2RustUnnamed_0 = 1;
pub const IDX_nul: C2RustUnnamed_1 = 27;
pub const IDX_y: C2RustUnnamed_1 = 10;
pub const IDX_T: C2RustUnnamed_1 = 9;
pub const IDX_w: C2RustUnnamed_1 = 5;
pub const IDX_c: C2RustUnnamed_1 = 3;
pub const IDX_s: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IDX_rbrace: C2RustUnnamed_1 = 26;
pub const IDX_lbrace: C2RustUnnamed_1 = 25;
pub const IDX_equal: C2RustUnnamed_1 = 24;
pub const IDX_x: C2RustUnnamed_1 = 23;
pub const IDX_q: C2RustUnnamed_1 = 22;
pub const IDX_P: C2RustUnnamed_1 = 21;
pub const IDX_p: C2RustUnnamed_1 = 20;
pub const IDX_N: C2RustUnnamed_1 = 19;
pub const IDX_n: C2RustUnnamed_1 = 18;
pub const IDX_l: C2RustUnnamed_1 = 17;
pub const IDX_H: C2RustUnnamed_1 = 16;
pub const IDX_h: C2RustUnnamed_1 = 15;
pub const IDX_G: C2RustUnnamed_1 = 14;
pub const IDX_g: C2RustUnnamed_1 = 13;
pub const IDX_D: C2RustUnnamed_1 = 12;
pub const IDX_d: C2RustUnnamed_1 = 11;
pub const IDX_t: C2RustUnnamed_1 = 8;
pub const IDX_b: C2RustUnnamed_1 = 7;
pub const IDX_colon: C2RustUnnamed_1 = 6;
pub const IDX_r: C2RustUnnamed_1 = 4;
pub const IDX_i: C2RustUnnamed_1 = 2;
pub const IDX_a: C2RustUnnamed_1 = 1;
/* Output line of text. */
/* Note:
 * The tricks with NO_EOL_CHAR and last_puts_char are there to emulate gnu sed.
 * Without them, we had this:
 * echo -n thingy >z1
 * echo -n again >z2
 * >znull
 * sed "s/i/z/" z1 z2 znull | hexdump -vC
 * output:
 * gnu sed 4.1.5:
 * 00000000  74 68 7a 6e 67 79 0a 61  67 61 7a 6e              |thzngy.agazn|
 * bbox:
 * 00000000  74 68 7a 6e 67 79 61 67  61 7a 6e                 |thzngyagazn|
 */
pub type C2RustUnnamed_2 = libc::c_uint;
pub const LAST_IS_NUL: C2RustUnnamed_2 = 2;
pub const NO_EOL_CHAR: C2RustUnnamed_2 = 1;
static mut semicolon_whitespace: [libc::c_char; 7] = [59, 32, 10, 13, 9, 11, 0];
/* If something bad happens during -i operation, delete temp file */
unsafe extern "C" fn cleanup_outname() {
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .outname
    .is_null()
  {
    unlink((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).outname);
  };
}
/* strcpy, replacing "\from" with 'to'. If to is NUL, replacing "\any" with 'any' */
unsafe extern "C" fn parse_escapes(
  mut dest: *mut libc::c_char,
  mut string: *const libc::c_char,
  mut len: libc::c_int,
  mut from: libc::c_char,
  mut to: libc::c_char,
) -> libc::c_uint {
  let mut d: *mut libc::c_char = dest; /* skip backslash in string[] */
  let mut i: libc::c_int = 0i32;
  if len == -1i32 {
    len = strlen(string) as libc::c_int
  }
  while i < len {
    if *string.offset(i as isize) as libc::c_int == '\\' as i32 {
      if to == 0 || *string.offset((i + 1i32) as isize) as libc::c_int == from as libc::c_int {
        *d = (if to as libc::c_int != 0 {
          to as libc::c_int
        } else {
          *string.offset((i + 1i32) as isize) as libc::c_int
        }) as libc::c_char;
        if *d as libc::c_int == '\u{0}' as i32 {
          return d.wrapping_offset_from(dest) as libc::c_long as libc::c_uint;
        }
        i += 2i32;
        d = d.offset(1);
        continue;
      } else {
        i += 1;
        let fresh0 = d;
        d = d.offset(1);
        *fresh0 = '\\' as i32 as libc::c_char
      }
      /* fall through: copy next char verbatim */
    }
    let fresh1 = i;
    i = i + 1;
    *d = *string.offset(fresh1 as isize);
    if *d as libc::c_int == '\u{0}' as i32 {
      return d.wrapping_offset_from(dest) as libc::c_long as libc::c_uint;
    }
    d = d.offset(1)
  }
  *d = '\u{0}' as i32 as libc::c_char;
  return d.wrapping_offset_from(dest) as libc::c_long as libc::c_uint;
}
unsafe extern "C" fn copy_parsing_escapes(
  mut string: *const libc::c_char,
  mut len: libc::c_int,
) -> *mut libc::c_char {
  let mut s: *const libc::c_char = 0 as *const libc::c_char;
  let mut dest: *mut libc::c_char = xmalloc((len + 1i32) as size_t) as *mut libc::c_char;
  /* sed recognizes \n */
  /* GNU sed also recognizes \t and \r */
  s = b"\nn\tt\rr\x00" as *const u8 as *const libc::c_char;
  while *s != 0 {
    len = parse_escapes(dest, string, len, *s.offset(1), *s.offset(0)) as libc::c_int;
    string = dest;
    s = s.offset(2)
  }
  return dest;
}
/*
 * index_of_next_unescaped_regexp_delim - walks left to right through a string
 * beginning at a specified index and returns the index of the next regular
 * expression delimiter (typically a forward slash ('/')) not preceded by
 * a backslash ('\').  A negative delimiter disables square bracket checking.
 */
unsafe extern "C" fn index_of_next_unescaped_regexp_delim(
  mut delimiter: libc::c_int,
  mut str: *const libc::c_char,
) -> libc::c_int {
  let mut bracket: libc::c_int = -1i32;
  let mut escaped: libc::c_int = 0i32;
  let mut idx: libc::c_int = 0i32;
  let mut ch: libc::c_char = 0;
  if delimiter < 0i32 {
    bracket -= 1;
    delimiter = -delimiter
  }
  loop {
    ch = *str.offset(idx as isize);
    if !(ch as libc::c_int != '\u{0}' as i32) {
      break;
    }
    if bracket >= 0i32 {
      if ch as libc::c_int == ']' as i32
        && !(bracket == idx - 1i32
          || bracket == idx - 2i32
            && *str.offset((idx - 1i32) as isize) as libc::c_int == '^' as i32)
      {
        bracket = -1i32
      }
    } else if escaped != 0 {
      escaped = 0i32
    } else if ch as libc::c_int == '\\' as i32 {
      escaped = 1i32
    } else if bracket == -1i32 && ch as libc::c_int == '[' as i32 {
      bracket = idx
    } else if ch as libc::c_int == delimiter {
      return idx;
    }
    idx += 1
  }
  /* if we make it to here, we've hit the end of the string */
  bb_error_msg_and_die(
    b"unmatched \'%c\'\x00" as *const u8 as *const libc::c_char,
    delimiter,
  );
}
/*
 *  Returns the index of the third delimiter
 */
unsafe extern "C" fn parse_regex_delim(
  mut cmdstr: *const libc::c_char,
  mut match_0: *mut *mut libc::c_char,
  mut replace: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut cmdstr_ptr: *const libc::c_char = cmdstr;
  let mut delimiter: libc::c_uchar = 0;
  let mut idx: libc::c_int = 0i32;
  /* verify that the 's' or 'y' is followed by something.  That something
   * (typically a 'slash') is now our regexp delimiter... */
  if *cmdstr as libc::c_int == '\u{0}' as i32 {
    bb_simple_error_msg_and_die(
      b"bad format in substitution expression\x00" as *const u8 as *const libc::c_char,
    );
  }
  let fresh2 = cmdstr_ptr;
  cmdstr_ptr = cmdstr_ptr.offset(1);
  delimiter = *fresh2 as libc::c_uchar;
  /* save the match string */
  idx = index_of_next_unescaped_regexp_delim(delimiter as libc::c_int, cmdstr_ptr);
  *match_0 = copy_parsing_escapes(cmdstr_ptr, idx);
  /* save the replacement string */
  cmdstr_ptr = cmdstr_ptr.offset((idx + 1i32) as isize);
  idx = index_of_next_unescaped_regexp_delim(-(delimiter as libc::c_int), cmdstr_ptr);
  *replace = copy_parsing_escapes(cmdstr_ptr, idx);
  return (cmdstr_ptr.wrapping_offset_from(cmdstr) as libc::c_long + idx as libc::c_long)
    as libc::c_int;
}
/*
 * returns the index in the string just past where the address ends.
 */
unsafe extern "C" fn get_address(
  mut my_str: *const libc::c_char,
  mut linenum: *mut libc::c_int,
  mut regex: *mut *mut regex_t,
) -> libc::c_int {
  let mut pos: *const libc::c_char = my_str;
  if (*my_str as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
    *linenum = strtol(
      my_str,
      &mut pos as *mut *const libc::c_char as *mut *mut libc::c_char,
      10i32,
    ) as libc::c_int
  /* endstr shouldn't ever equal NULL */
  } else if *my_str as libc::c_int == '$' as i32 {
    *linenum = -1i32;
    pos = pos.offset(1)
  } else if *my_str as libc::c_int == '/' as i32 || *my_str as libc::c_int == '\\' as i32 {
    let mut next: libc::c_int = 0;
    let mut delimiter: libc::c_char = 0;
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    delimiter = '/' as i32 as libc::c_char;
    if *my_str as libc::c_int == '\\' as i32 {
      pos = pos.offset(1);
      delimiter = *pos
    }
    pos = pos.offset(1);
    next = index_of_next_unescaped_regexp_delim(delimiter as libc::c_int, pos);
    if next != 0i32 {
      temp = copy_parsing_escapes(pos, next);
      *regex = xzalloc(::std::mem::size_of::<regex_t>() as libc::c_ulong) as *mut regex_t;
      let ref mut fresh3 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).previous_regex_ptr;
      *fresh3 = *regex;
      xregcomp(
        *regex,
        temp,
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).regex_type,
      );
      free(temp as *mut libc::c_void);
    } else {
      *regex = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).previous_regex_ptr;
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .previous_regex_ptr
        .is_null()
      {
        bb_simple_error_msg_and_die(b"no previous regexp\x00" as *const u8 as *const libc::c_char);
      }
    }
    /* Move position to next character after last delimiter */
    pos = pos.offset((next + 1i32) as isize)
  }
  return pos.wrapping_offset_from(my_str) as libc::c_long as libc::c_int;
}
/* Grab a filename.  Whitespace at start is skipped, then goes to EOL. */
unsafe extern "C" fn parse_file_cmd(
  mut filecmdstr: *const libc::c_char,
  mut retval: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut start: *const libc::c_char = 0 as *const libc::c_char;
  let mut eol: *const libc::c_char = 0 as *const libc::c_char;
  /* Skip whitespace, then grab filename to end of line */
  start = skip_whitespace(filecmdstr);
  eol = strchrnul(start, '\n' as i32);
  if eol == start {
    bb_simple_error_msg_and_die(b"empty filename\x00" as *const u8 as *const libc::c_char);
  }
  if *eol != 0 {
    /* If lines glued together, put backslash back. */
    *retval = xstrndup(
      start,
      (eol.wrapping_offset_from(start) as libc::c_long + 1i32 as libc::c_long) as libc::c_int,
    );
    *(*retval).offset(eol.wrapping_offset_from(start) as libc::c_long as isize) =
      '\\' as i32 as libc::c_char
  } else {
    /* eol is NUL */
    *retval = xstrdup(start)
  }
  return eol.wrapping_offset_from(filecmdstr) as libc::c_long as libc::c_int;
}
unsafe extern "C" fn parse_subst_cmd(
  mut sed_cmd: *mut sed_cmd_t,
  mut substr: *const libc::c_char,
) -> libc::c_int {
  let mut cflags: libc::c_int = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).regex_type;
  let mut match_0: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut idx: libc::c_int = 0;
  /*
   * A substitution command should look something like this:
   *    s/match/replace/ #giIpw
   *    ||     |        |||
   *    mandatory       optional
   */
  idx = parse_regex_delim(substr, &mut match_0, &mut (*sed_cmd).string);
  /* determine the number of back references in the match string */
  /* Note: we compute this here rather than in the do_subst_command()
   * function to save processor time, at the expense of a little more memory
   * (4 bits) per sed_cmd */
  /* process the flags */
  (*sed_cmd).which_match = 1i32 as libc::c_uint;
  loop {
    idx += 1;
    if !(*substr.offset(idx as isize) != 0) {
      break;
    }
    /* Parse match number */
    if (*substr.offset(idx as isize) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int
      <= 9i32
    {
      if *match_0.offset(0) as libc::c_int != '^' as i32 {
        /* Match 0 treated as all, multiple matches we take the last one. */
        let mut pos: *const libc::c_char = substr.offset(idx as isize);
        /* FIXME: error check? */
        (*sed_cmd).which_match = strtol(
          substr.offset(idx as isize),
          &mut pos as *mut *const libc::c_char as *mut *mut libc::c_char,
          10i32,
        ) as libc::c_uint;
        idx =
          (pos.wrapping_offset_from(substr) as libc::c_long - 1i32 as libc::c_long) as libc::c_int
      }
    } else {
      /* Skip spaces */
      if ({
        let mut bb__isspace: libc::c_uchar =
          (*substr.offset(idx as isize) as libc::c_int - 9i32) as libc::c_uchar;
        (bb__isspace as libc::c_int == ' ' as i32 - 9i32
          || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
      }) != 0
      {
        continue;
      }
      match *substr.offset(idx as isize) as libc::c_int {
        103 => {
          /* Replace all occurrences */
          if *match_0.offset(0) as libc::c_int != '^' as i32 {
            (*sed_cmd).which_match = 0i32 as libc::c_uint
          }
        }
        112 => {
          /* Print pattern space */
          (*sed_cmd).set_sub_p(1i32 as libc::c_uint)
        }
        119 => {
          /* Write to file */
          let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
          idx += parse_file_cmd(substr.offset(idx as isize).offset(1), &mut fname);
          (*sed_cmd).sw_file = xfopen_for_write(fname);
          (*sed_cmd).sw_last_char = '\n' as i32 as libc::c_char;
          free(fname as *mut libc::c_void);
        }
        105 | 73 => {
          /* Ignore case (gnu extension) */
          cflags |= 1i32 << 1i32
        }
        35 => {
          /* Comment */
          idx = (idx as libc::c_ulong).wrapping_add(strlen(substr.offset(idx as isize)))
            as libc::c_int as libc::c_int; // same
                                           // while (substr[++idx]) continue;
          break;
        }
        59 | 125 => {
          break;
        }
        _ => {
          bb_simple_error_msg_and_die(
            b"bad option in substitution expression\x00" as *const u8 as *const libc::c_char,
          );
        }
      }
    }
  }
  /* Fall through */
  /* End of command */
  /* compile the match string into a regex */
  if *match_0 as libc::c_int != '\u{0}' as i32 {
    /* If match is empty, we use last regex used at runtime */
    (*sed_cmd).sub_match =
      xzalloc(::std::mem::size_of::<regex_t>() as libc::c_ulong) as *mut regex_t;
    xregcomp((*sed_cmd).sub_match, match_0, cflags);
  }
  free(match_0 as *mut libc::c_void);
  return idx;
}
/*
 *  Process the commands arguments
 */
unsafe extern "C" fn parse_cmd_args(
  mut sed_cmd: *mut sed_cmd_t,
  mut cmdstr: *const libc::c_char,
) -> *const libc::c_char {
  static mut cmd_letters: [libc::c_char; 28] = [
    115, 97, 105, 99, 114, 119, 58, 98, 116, 84, 121, 100, 68, 103, 71, 104, 72, 108, 110, 78, 112,
    80, 113, 120, 61, 123, 125, 0,
  ];
  let mut idx: libc::c_uint = 0;
  idx = strchrnul(cmd_letters.as_ptr(), (*sed_cmd).cmd as libc::c_int)
    .wrapping_offset_from(cmd_letters.as_ptr()) as libc::c_long as libc::c_uint;
  /* handle (s)ubstitution command */
  if idx == IDX_s as libc::c_int as libc::c_uint {
    cmdstr = cmdstr.offset(parse_subst_cmd(sed_cmd, cmdstr) as isize)
  } else if idx <= IDX_c as libc::c_int as libc::c_uint {
    /* handle edit cmds: (a)ppend, (i)nsert, and (c)hange */
    /* a,i,c */
    let mut len: libc::c_uint = 0;
    if idx < IDX_c as libc::c_int as libc::c_uint {
      /* a,i */
      if (*sed_cmd).end_line != 0 || !(*sed_cmd).end_match.is_null() {
        bb_error_msg_and_die(
          b"command \'%c\' uses only one address\x00" as *const u8 as *const libc::c_char,
          (*sed_cmd).cmd as libc::c_int,
        );
      }
    }
    loop {
      if *cmdstr as libc::c_int == '\n' as i32 || *cmdstr as libc::c_int == '\\' as i32 {
        cmdstr = cmdstr.offset(1);
        break;
      } else {
        if ({
          let mut bb__isspace: libc::c_uchar = (*cmdstr as libc::c_int - 9i32) as libc::c_uchar;
          (bb__isspace as libc::c_int == ' ' as i32 - 9i32
            || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
        }) == 0
        {
          break;
        }
        cmdstr = cmdstr.offset(1)
      }
    }
    len = strlen(cmdstr) as libc::c_uint;
    (*sed_cmd).string = copy_parsing_escapes(cmdstr, len as libc::c_int);
    cmdstr = cmdstr.offset(len as isize);
    /* "\anychar" -> "anychar" */
    parse_escapes(
      (*sed_cmd).string,
      (*sed_cmd).string,
      -1i32,
      '\u{0}' as i32 as libc::c_char,
      '\u{0}' as i32 as libc::c_char,
    );
  } else if idx <= IDX_w as libc::c_int as libc::c_uint {
    /* handle file cmds: (r)ead */
    /* r,w */
    if idx < IDX_w as libc::c_int as libc::c_uint {
      /* r */
      if (*sed_cmd).end_line != 0 || !(*sed_cmd).end_match.is_null() {
        bb_error_msg_and_die(
          b"command \'%c\' uses only one address\x00" as *const u8 as *const libc::c_char,
          (*sed_cmd).cmd as libc::c_int,
        );
      }
    }
    cmdstr = cmdstr.offset(parse_file_cmd(cmdstr, &mut (*sed_cmd).string) as isize);
    if (*sed_cmd).cmd as libc::c_int == 'w' as i32 {
      (*sed_cmd).sw_file = xfopen_for_write((*sed_cmd).string);
      (*sed_cmd).sw_last_char = '\n' as i32 as libc::c_char
    }
  } else if idx <= IDX_T as libc::c_int as libc::c_uint {
    /* handle branch commands */
    /* :,b,t,T */
    let mut length: libc::c_int = 0;
    cmdstr = skip_whitespace(cmdstr);
    length = strcspn(cmdstr, semicolon_whitespace.as_ptr()) as libc::c_int;
    if length != 0 {
      (*sed_cmd).string = xstrndup(cmdstr, length);
      cmdstr = cmdstr.offset(length as isize)
    }
  } else if idx == IDX_y as libc::c_int as libc::c_uint {
    let mut match_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut replace: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = *cmdstr.offset(0) as libc::c_int;
    cmdstr = cmdstr.offset((parse_regex_delim(cmdstr, &mut match_0, &mut replace) + 1i32) as isize);
    /* translation command */
    /* \n already parsed, but \delimiter needs unescaping. */
    parse_escapes(
      match_0,
      match_0,
      -1i32,
      i as libc::c_char,
      i as libc::c_char,
    );
    parse_escapes(
      replace,
      replace,
      -1i32,
      i as libc::c_char,
      i as libc::c_char,
    );
    (*sed_cmd).string = xzalloc(
      strlen(match_0)
        .wrapping_add(1i32 as libc::c_ulong)
        .wrapping_mul(2i32 as libc::c_ulong),
    ) as *mut libc::c_char;
    i = 0i32;
    while *match_0.offset(i as isize) as libc::c_int != 0
      && *replace.offset(i as isize) as libc::c_int != 0
    {
      *(*sed_cmd).string.offset((i * 2i32) as isize) = *match_0.offset(i as isize);
      *(*sed_cmd).string.offset((i * 2i32 + 1i32) as isize) = *replace.offset(i as isize);
      i += 1
    }
    free(match_0 as *mut libc::c_void);
    free(replace as *mut libc::c_void);
  } else if idx >= IDX_nul as libc::c_int as libc::c_uint {
    /* if it wasn't a single-letter command that takes no arguments
     * then it must be an invalid command.
     */
    /* not d,D,g,G,h,H,l,n,N,p,P,q,x,=,{,} */
    bb_error_msg_and_die(
      b"unsupported command %c\x00" as *const u8 as *const libc::c_char,
      (*sed_cmd).cmd as libc::c_int,
    );
  }
  /* give back whatever's left over */
  return cmdstr;
}
/* Parse address+command sets, skipping comment lines. */
unsafe extern "C" fn add_cmd(mut cmdstr: *const libc::c_char) {
  let mut sed_cmd: *mut sed_cmd_t = 0 as *mut sed_cmd_t;
  let mut len: libc::c_uint = 0;
  let mut n: libc::c_uint = 0;
  /* Append this line to any unfinished line from last time. */
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .add_cmd_line
    .is_null()
  {
    let mut tp: *mut libc::c_char = xasprintf(
      b"%s\n%s\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).add_cmd_line,
      cmdstr,
    );
    free((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).add_cmd_line as *mut libc::c_void);
    let ref mut fresh4 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).add_cmd_line;
    *fresh4 = tp;
    cmdstr = *fresh4
  }
  /* If this line ends with unescaped backslash, request next line. */
  len = strlen(cmdstr) as libc::c_uint;
  n = len;
  while n != 0
    && *cmdstr.offset(n.wrapping_sub(1i32 as libc::c_uint) as isize) as libc::c_int == '\\' as i32
  {
    n = n.wrapping_sub(1)
  }
  if len.wrapping_sub(n) & 1i32 as libc::c_uint != 0 {
    /* if odd number of trailing backslashes */
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .add_cmd_line
      .is_null()
    {
      let ref mut fresh5 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).add_cmd_line;
      *fresh5 = xstrdup(cmdstr)
    }
    *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .add_cmd_line
      .offset(len.wrapping_sub(1i32 as libc::c_uint) as isize) = '\u{0}' as i32 as libc::c_char;
    return;
  }
  /* Loop parsing all commands in this line. */
  while *cmdstr != 0 {
    /* Skip leading whitespace and semicolons */
    cmdstr = cmdstr.offset(strspn(cmdstr, semicolon_whitespace.as_ptr()) as isize);
    /* If no more commands, exit. */
    if *cmdstr == 0 {
      break;
    }
    /* if this is a comment, jump past it and keep going */
    if *cmdstr as libc::c_int == '#' as i32 {
      /* "#n" is the same as using -n on the command line */
      if *cmdstr.offset(1) as libc::c_int == 'n' as i32 {
        let ref mut fresh6 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).be_quiet;
        *fresh6 += 1
      }
      cmdstr = strpbrk(cmdstr, b"\n\r\x00" as *const u8 as *const libc::c_char);
      if cmdstr.is_null() {
        break;
      }
    } else {
      /* parse the command
       * format is: [addr][,addr][!]cmd
       *            |----||-----||-|
       *            part1 part2  part3
       */
      sed_cmd = xzalloc(::std::mem::size_of::<sed_cmd_t>() as libc::c_ulong) as *mut sed_cmd_t;
      /* first part (if present) is an address: either a '$', a number or a /regex/ */
      cmdstr =
        cmdstr.offset(
          get_address(cmdstr, &mut (*sed_cmd).beg_line, &mut (*sed_cmd).beg_match) as isize,
        );
      (*sed_cmd).beg_line_orig = (*sed_cmd).beg_line;
      /* second part (if present) will begin with a comma */
      if *cmdstr as libc::c_int == ',' as i32 {
        let mut idx: libc::c_int = 0;
        cmdstr = cmdstr.offset(1);
        if *cmdstr as libc::c_int == '+' as i32
          && (*cmdstr.offset(1) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32
        {
          /* http://sed.sourceforge.net/sedfaq3.html#s3.3
           * Under GNU sed 3.02+, ssed, and sed15+, <address2>
           * may also be a notation of the form +num,
           * indicating the next num lines after <address1> is
           * matched.
           * GNU sed 4.2.1 accepts even "+" (meaning "+0").
           * We don't (we check for isdigit, see above), think
           * about the "+-3" case.
           */
          let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
          /* code is smaller compared to using &cmdstr here: */
          idx = strtol(cmdstr.offset(1), &mut end, 10i32) as libc::c_int;
          (*sed_cmd).end_line = -2i32 - idx;
          cmdstr = end
        } else {
          idx = get_address(cmdstr, &mut (*sed_cmd).end_line, &mut (*sed_cmd).end_match);
          cmdstr = cmdstr.offset(idx as isize);
          idx -= 1
          /* if 0, trigger error check below */
        }
        if idx < 0i32 {
          bb_simple_error_msg_and_die(
            b"no address after comma\x00" as *const u8 as *const libc::c_char,
          );
        }
        (*sed_cmd).end_line_orig = (*sed_cmd).end_line
      }
      /* skip whitespace before the command */
      cmdstr = skip_whitespace(cmdstr);
      /* Check for inversion flag */
      if *cmdstr as libc::c_int == '!' as i32 {
        (*sed_cmd).set_invert(1i32 as libc::c_uint);
        cmdstr = cmdstr.offset(1);
        /* skip whitespace before the command */
        cmdstr = skip_whitespace(cmdstr)
      }
      /* last part (mandatory) will be a command */
      if *cmdstr == 0 {
        bb_simple_error_msg_and_die(b"missing command\x00" as *const u8 as *const libc::c_char);
      }
      let fresh7 = cmdstr;
      cmdstr = cmdstr.offset(1);
      (*sed_cmd).cmd = *fresh7;
      cmdstr = parse_cmd_args(sed_cmd, cmdstr);
      /* cmdstr now points past args.
       * GNU sed requires a separator, if there are more commands,
       * else it complains "char N: extra characters after command".
       * Example: "sed 'p;d'". We also allow "sed 'pd'".
       */
      /* Add the command to the command array */
      let ref mut fresh8 = *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).sed_cmd_tail;
      *fresh8 = sed_cmd;
      let ref mut fresh9 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).sed_cmd_tail;
      *fresh9 = &mut (*sed_cmd).next
    }
  }
  /* If we glued multiple lines together, free the memory. */
  free((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).add_cmd_line as *mut libc::c_void);
  let ref mut fresh10 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).add_cmd_line;
  *fresh10 = 0 as *mut libc::c_char;
}
unsafe extern "C" fn pipe_putc(mut c: libc::c_char) {
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .pipeline
    .idx
    == (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .pipeline
      .len
  {
    let ref mut fresh11 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .pipeline
      .buf;
    *fresh11 = xrealloc(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .pipeline
        .buf as *mut libc::c_void,
      ((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .pipeline
        .len
        + 64i32) as size_t,
    ) as *mut libc::c_char;
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .pipeline
      .len += 64i32
  }
  let ref mut fresh12 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .pipeline
    .idx;
  let fresh13 = *fresh12;
  *fresh12 = *fresh12 + 1;
  *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .pipeline
    .buf
    .offset(fresh13 as isize) = c;
}
unsafe extern "C" fn do_subst_w_backrefs(
  mut line: *mut libc::c_char,
  mut replace: *mut libc::c_char,
) {
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  /* go through the replacement string */
  i = 0i32;
  while *replace.offset(i as isize) != 0 {
    /* if we find a backreference (\1, \2, etc.) print the backref'ed text */
    if *replace.offset(i as isize) as libc::c_int == '\\' as i32 {
      i += 1;
      let mut backref: libc::c_uint =
        (*replace.offset(i as isize) as libc::c_int - '0' as i32) as libc::c_uint;
      if backref <= 9i32 as libc::c_uint {
        /* print out the text held in G.regmatch[backref] */
        if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).regmatch[backref as usize].rm_so
          != -1i32
        {
          j = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).regmatch[backref as usize].rm_so;
          while j
            < (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).regmatch[backref as usize].rm_eo
          {
            let fresh14 = j;
            j = j + 1;
            pipe_putc(*line.offset(fresh14 as isize));
          }
        }
      } else {
        /* I _think_ it is impossible to get '\' to be
         * the last char in replace string. Thus we don't check
         * for replace[i] == NUL. (counterexample anyone?) */
        /* if we find a backslash escaped character, print the character */
        pipe_putc(*replace.offset(i as isize));
      }
    } else if *replace.offset(i as isize) as libc::c_int == '&' as i32 {
      j = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).regmatch[0].rm_so;
      while j < (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).regmatch[0].rm_eo {
        let fresh15 = j;
        j = j + 1;
        pipe_putc(*line.offset(fresh15 as isize));
      }
    } else {
      /* if we find an unescaped '&' print out the whole matched text. */
      /* Otherwise just output the character. */
      pipe_putc(*replace.offset(i as isize));
    }
    i += 1
  }
}
unsafe extern "C" fn do_subst_command(
  mut sed_cmd: *mut sed_cmd_t,
  mut line_p: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut line: *mut libc::c_char = *line_p;
  let mut match_count: libc::c_uint = 0i32 as libc::c_uint;
  let mut altered: bool = 0i32 != 0;
  let mut prev_match_empty: bool = 1i32 != 0;
  let mut tried_at_eol: bool = 0i32 != 0;
  let mut current_regex: *mut regex_t = 0 as *mut regex_t;
  current_regex = (*sed_cmd).sub_match;
  /* Handle empty regex. */
  if current_regex.is_null() {
    current_regex = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).previous_regex_ptr;
    if current_regex.is_null() {
      bb_simple_error_msg_and_die(b"no previous regexp\x00" as *const u8 as *const libc::c_char);
    }
  }
  let ref mut fresh16 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).previous_regex_ptr;
  *fresh16 = current_regex;
  /* Find the first match */
  if REG_NOMATCH as libc::c_int
    == regexec(
      current_regex,
      line,
      10i32 as size_t,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .regmatch
        .as_mut_ptr(),
      0i32,
    )
  {
    return 0i32;
  }
  /* Initialize temporary output buffer. */
  let ref mut fresh17 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .pipeline
    .buf;
  *fresh17 = xmalloc(64i32 as size_t) as *mut libc::c_char;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .pipeline
    .len = 64i32;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .pipeline
    .idx = 0i32;
  loop
  /* Now loop through, substituting for matches */
  {
    let mut start: libc::c_int =
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).regmatch[0].rm_so;
    let mut end: libc::c_int =
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).regmatch[0].rm_eo;
    let mut i: libc::c_int = 0;
    match_count = match_count.wrapping_add(1);
    /* If we aren't interested in this match, output old line to
     * end of match and continue */
    if (*sed_cmd).which_match != 0 && (*sed_cmd).which_match != match_count {
      i = 0i32;
      while i < end {
        let fresh18 = line;
        line = line.offset(1);
        pipe_putc(*fresh18);
        i += 1
      }
      /* Null match? Print one more char */
      if start == end && *line as libc::c_int != 0 {
        let fresh19 = line;
        line = line.offset(1);
        pipe_putc(*fresh19);
      }
    } else {
      /* Print everything before the match */
      i = 0i32;
      while i < start {
        pipe_putc(*line.offset(i as isize));
        i += 1
      }
      /* Then print the substitution string,
       * unless we just matched empty string after non-empty one.
       * Example: string "cccd", pattern "c*", repl "R":
       * result is "RdR", not "RRdR": first match "ccc",
       * second is "" before "d", third is "" after "d".
       * Second match is NOT replaced!
       */
      if prev_match_empty as libc::c_int != 0 || start != 0i32 || start != end {
        //dbg("%d %d %d", prev_match_empty, start, end);
        do_subst_w_backrefs(line, (*sed_cmd).string);
        /* Flag that something has changed */
        altered = 1i32 != 0
      }
      /* If matched string is empty (f.e. "c*" pattern),
       * copy verbatim one char after it before attempting more matches
       */
      prev_match_empty = start == end;
      if prev_match_empty {
        if *line.offset(end as isize) == 0 {
          tried_at_eol = 1i32 != 0
        } else {
          pipe_putc(*line.offset(end as isize));
          end += 1
        }
      }
      /* Advance past the match */
      line = line.offset(end as isize);
      /* if we're not doing this globally, get out now */
      if (*sed_cmd).which_match != 0i32 as libc::c_uint {
        break;
      }
    }
    /* Exit if we are at EOL and already tried matching at it */
    if *line as libc::c_int == '\u{0}' as i32 {
      if tried_at_eol {
        break;
      }
      tried_at_eol = 1i32 != 0
    }
    if !(regexec(
      current_regex,
      line,
      10i32 as size_t,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .regmatch
        .as_mut_ptr(),
      1i32,
    ) != REG_NOMATCH as libc::c_int)
    {
      break;
    }
    //maybe (end ? REG_NOTBOL : 0) instead of unconditional REG_NOTBOL?
  }
  loop
  /* Copy rest of string into output pipeline */
  {
    let fresh20 = line;
    line = line.offset(1);
    let mut c: libc::c_char = *fresh20;
    pipe_putc(c);
    if c as libc::c_int == '\u{0}' as i32 {
      break;
    }
  }
  free(*line_p as *mut libc::c_void);
  *line_p = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .pipeline
    .buf;
  return altered as libc::c_int;
}
/* Set command pointer to point to this label.  (Does not handle null label.) */
unsafe extern "C" fn branch_to(mut label: *mut libc::c_char) -> *mut sed_cmd_t {
  let mut sed_cmd: *mut sed_cmd_t = 0 as *mut sed_cmd_t;
  sed_cmd = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).sed_cmd_head;
  while !sed_cmd.is_null() {
    if (*sed_cmd).cmd as libc::c_int == ':' as i32
      && !(*sed_cmd).string.is_null()
      && strcmp((*sed_cmd).string, label) == 0i32
    {
      return sed_cmd;
    }
    sed_cmd = (*sed_cmd).next
  }
  bb_error_msg_and_die(
    b"can\'t find label for jump to \'%s\'\x00" as *const u8 as *const libc::c_char,
    label,
  );
}
unsafe extern "C" fn append(mut s: *mut libc::c_char) {
  llist_add_to_end(
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).append_head,
    s as *mut libc::c_void,
  );
}
unsafe extern "C" fn puts_maybe_newline(
  mut s: *mut libc::c_char,
  mut file: *mut FILE,
  mut last_puts_char: *mut libc::c_char,
  mut last_gets_char: libc::c_char,
) {
  let mut lpc: libc::c_char = *last_puts_char;
  /* Need to insert a '\n' between two files because first file's
   * last line wasn't terminated? */
  if lpc as libc::c_int != '\n' as i32 && lpc as libc::c_int != '\u{0}' as i32 {
    putc_unlocked('\n' as i32, file);
    lpc = '\n' as i32 as libc::c_char
  }
  fputs_unlocked(s, file);
  /* 'x' - just something which is not '\n', '\0' or NO_EOL_CHAR */
  if *s.offset(0) != 0 {
    lpc = 'x' as i32 as libc::c_char
  }
  /* had trailing '\0' and it was last char of file? */
  if last_gets_char as libc::c_int == LAST_IS_NUL as libc::c_int {
    putc_unlocked('\u{0}' as i32, file);
    lpc = 'x' as i32 as libc::c_char
  /* */
  } else if last_gets_char as libc::c_int != NO_EOL_CHAR as libc::c_int {
    putc_unlocked(last_gets_char as libc::c_int, file);
    lpc = last_gets_char
  }
  if ferror_unlocked(file) != 0 {
    /* had trailing '\n' or '\0'? */
    xfunc_error_retval = 4i32 as uint8_t; /* It's what gnu sed exits with... */
    bb_simple_error_msg_and_die(b"write error\x00" as *const u8 as *const libc::c_char);
  }
  *last_puts_char = lpc;
}
unsafe extern "C" fn flush_append(mut last_puts_char: *mut libc::c_char) {
  let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
  loop
  /* Output appended lines. */
  {
    data = llist_pop(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).append_head)
      as *mut libc::c_char;
    if data.is_null() {
      break;
    }
    /* Append command does not respect "nonterminated-ness"
     * of last line. Try this:
     * $ echo -n "woot" | sed -e '/woot/a woo' -
     * woot
     * woo
     * (both lines are terminated with \n)
     * Therefore we do not propagate "last_gets_char" here,
     * pass '\n' instead:
     */
    puts_maybe_newline(
      data,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
      last_puts_char,
      '\n' as i32 as libc::c_char,
    );
    free(data as *mut libc::c_void);
  }
}
/* Get next line of input from G.input_file_list, flushing append buffer and
 * noting if we ran out of files without a newline on the last line we read.
 */
unsafe extern "C" fn get_next_line(
  mut gets_char: *mut libc::c_char,
  mut last_puts_char: *mut libc::c_char,
) -> *mut libc::c_char {
  let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut len: size_t = 0;
  let mut gc: libc::c_char = 0;
  flush_append(last_puts_char);
  /* will be returned if last line in the file
   * doesn't end with either '\n' or '\0' */
  gc = NO_EOL_CHAR as libc::c_int as libc::c_char;
  let mut current_block_17: u64;
  while (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).current_input_file
    <= (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).last_input_file
  {
    let mut fp: *mut FILE = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).current_fp;
    if fp.is_null() {
      let mut path: *const libc::c_char = *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .input_file_list
        .offset((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).current_input_file as isize);
      fp = stdin;
      if path != bb_msg_standard_input.as_ptr() {
        fp = fopen_or_warn(path, b"r\x00" as *const u8 as *const libc::c_char);
        if fp.is_null() {
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).exitcode = 1i32 as smallint;
          current_block_17 = 17179679302217393232;
        } else {
          current_block_17 = 1841672684692190573;
        }
      } else {
        current_block_17 = 1841672684692190573;
      }
      match current_block_17 {
        17179679302217393232 => {}
        _ => {
          let ref mut fresh21 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).current_fp;
          *fresh21 = fp;
          current_block_17 = 3512920355445576850;
        }
      }
    } else {
      current_block_17 = 3512920355445576850;
    }
    match current_block_17 {
      3512920355445576850 => {
        /* Read line up to a newline or NUL byte, inclusive,
         * return malloc'ed char[]. length of the chunk read
         * is stored in len. NULL if EOF/error */
        temp = bb_get_chunk_from_file(fp, &mut len);
        if !temp.is_null() {
          /* len > 0 here, it's ok to do temp[len-1] */
          let mut c: libc::c_char = *temp.offset(len.wrapping_sub(1i32 as libc::c_ulong) as isize);
          if c as libc::c_int == '\n' as i32 || c as libc::c_int == '\u{0}' as i32 {
            *temp.offset(len.wrapping_sub(1i32 as libc::c_ulong) as isize) =
              '\u{0}' as i32 as libc::c_char;
            gc = c;
            if c as libc::c_int == '\u{0}' as i32 {
              let mut ch: libc::c_int = getc_unlocked(fp);
              if ch != -1i32 {
                ungetc(ch, fp);
              } else {
                gc = LAST_IS_NUL as libc::c_int as libc::c_char
              }
            }
          }
          break;
        /* NB: I had the idea of peeking next file(s) and returning
         * NO_EOL_CHAR only if it is the *last* non-empty
         * input file. But there is a case where this won't work:
         * file1: "a woo\nb woo"
         * file2: "c no\nd no"
         * sed -ne 's/woo/bang/p' input1 input2 => "a bang\nb bang"
         * (note: *no* newline after "b bang"!) */
        } else {
          /* Close this file and advance to next one */
          fclose_if_not_stdin(fp);
          let ref mut fresh22 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).current_fp;
          *fresh22 = 0 as *mut FILE
        }
      }
      _ => {}
    }
    let ref mut fresh23 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).current_input_file;
    *fresh23 += 1
  }
  *gets_char = gc;
  return temp;
}
unsafe extern "C" fn beg_match(
  mut sed_cmd: *mut sed_cmd_t,
  mut pattern_space: *const libc::c_char,
) -> libc::c_int {
  let mut retval: libc::c_int = (!(*sed_cmd).beg_match.is_null()
    && regexec(
      (*sed_cmd).beg_match,
      pattern_space,
      0i32 as size_t,
      0 as *mut regmatch_t,
      0i32,
    ) == 0) as libc::c_int;
  if retval != 0 {
    let ref mut fresh24 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).previous_regex_ptr;
    *fresh24 = (*sed_cmd).beg_match
  }
  return retval;
}
/* Process all the lines in all the files */
unsafe extern "C" fn process_files() {
  let mut current_block: u64;
  let mut pattern_space: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut next_line: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut linenum: libc::c_int = 0i32;
  let mut last_puts_char: libc::c_char = '\n' as i32 as libc::c_char;
  let mut last_gets_char: libc::c_char = 0;
  let mut next_gets_char: libc::c_char = 0;
  let mut sed_cmd: *mut sed_cmd_t = 0 as *mut sed_cmd_t;
  let mut substituted: libc::c_int = 0;
  /* Prime the pump */
  next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
  loop
  /* Go through every line in each file */
  {
    substituted = 0i32;
    /* Advance to next line.  Stop if out of lines. */
    pattern_space = next_line;
    if pattern_space.is_null() {
      return;
    }
    last_gets_char = next_gets_char;
    /* Read one line in advance so we can act on the last line,
     * the '$' address */
    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
    linenum += 1;
    's_54: loop
    /* For every line, go through all the commands */
    {
      sed_cmd = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).sed_cmd_head; /* for each cmd */
      loop {
        if sed_cmd.is_null() {
          current_block = 8563197331115798083;
          break 's_54;
        }
        let mut old_matched: libc::c_int = 0;
        let mut matched: libc::c_int = 0;
        old_matched = (*sed_cmd).in_match() as libc::c_int;
        if old_matched == 0 {
          (*sed_cmd).end_line = (*sed_cmd).end_line_orig
        }
        /* switch */
        /* Determine if this command matches this line: */
        /* Are we continuing a previous multi-line match? */
        (*sed_cmd).set_in_match(
          ((*sed_cmd).in_match() as libc::c_int != 0
            || (*sed_cmd).beg_line == 0
              && (*sed_cmd).end_line == 0
              && (*sed_cmd).beg_match.is_null()
              && (*sed_cmd).end_match.is_null()
            || (*sed_cmd).beg_line > 0i32
              && (if (*sed_cmd).end_line != 0 || !(*sed_cmd).end_match.is_null() {
                ((*sed_cmd).beg_line <= linenum) as libc::c_int
              } else {
                ((*sed_cmd).beg_line == linenum) as libc::c_int
              }) != 0
            || beg_match(sed_cmd, pattern_space) != 0
            || (*sed_cmd).beg_line == -1i32 && next_line.is_null()) as libc::c_int
            as libc::c_uint,
        );
        /* Snapshot the value */
        matched = (*sed_cmd).in_match() as libc::c_int;
        /* Is this line the end of the current match? */
        if matched != 0 {
          if (*sed_cmd).end_line <= -2i32 {
            /* address2 is +N, i.e. N lines from beg_line */
            (*sed_cmd).end_line = linenum + (-(*sed_cmd).end_line - 2i32)
          }
          /* once matched, "n,xxx" range is dead, disabling it */
          if (*sed_cmd).beg_line > 0i32 {
            (*sed_cmd).beg_line = -2i32
          }
          (*sed_cmd).set_in_match(
            !((if (*sed_cmd).end_line != 0 {
              (if (*sed_cmd).end_line == -1i32 {
                next_line.is_null() as libc::c_int
              } else {
                ((*sed_cmd).end_line <= linenum) as libc::c_int
              })
            } else {
              (*sed_cmd).end_match.is_null() as libc::c_int
            }) != 0
              || !(*sed_cmd).end_match.is_null()
                && old_matched != 0
                && regexec(
                  (*sed_cmd).end_match,
                  pattern_space,
                  0i32 as size_t,
                  0 as *mut regmatch_t,
                  0i32,
                ) == 0i32) as libc::c_int as libc::c_uint,
          )
        }
        /* Skip blocks of commands we didn't match */
        if (*sed_cmd).cmd as libc::c_int == '{' as i32 {
          if if (*sed_cmd).invert() as libc::c_int != 0 {
            matched
          } else {
            (matched == 0) as libc::c_int
          } != 0
          {
            let mut nest_cnt: libc::c_uint = 0i32 as libc::c_uint;
            loop {
              if (*sed_cmd).cmd as libc::c_int == '{' as i32 {
                nest_cnt = nest_cnt.wrapping_add(1)
              }
              if (*sed_cmd).cmd as libc::c_int == '}' as i32 {
                nest_cnt = nest_cnt.wrapping_sub(1);
                if nest_cnt == 0i32 as libc::c_uint {
                  break;
                }
              }
              sed_cmd = (*sed_cmd).next;
              if sed_cmd.is_null() {
                bb_simple_error_msg_and_die(
                  b"unterminated {\x00" as *const u8 as *const libc::c_char,
                );
              }
            }
          }
        } else if !(if (*sed_cmd).invert() as libc::c_int != 0 {
          matched
        } else {
          (matched == 0) as libc::c_int
        } != 0)
        {
          /* Okay, so did this line match? */
          /* no */
          /* Update last used regex in case a blank substitute BRE is found */
          if !(*sed_cmd).beg_match.is_null() {
            let ref mut fresh25 =
              (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).previous_regex_ptr;
            *fresh25 = (*sed_cmd).beg_match
          }
          /* actual sedding */
          match (*sed_cmd).cmd as libc::c_int {
            61 => {
              current_block = 332836909292088582;
              match current_block {
                11227437541145425351 =>
                /* Append newline and pattern space to hold space */
                {
                  let mut hold_space_size_0: libc::c_int = 2i32;
                  let mut pattern_space_size_0: libc::c_int = 0i32;
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size_0 = (hold_space_size_0 as libc::c_ulong).wrapping_add(strlen(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    )) as libc::c_int as libc::c_int
                  }
                  if !pattern_space.is_null() {
                    pattern_space_size_0 = strlen(pattern_space) as libc::c_int
                  }
                  let ref mut fresh27 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh27 = xrealloc(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                    (hold_space_size_0 + pattern_space_size_0) as size_t,
                  ) as *mut libc::c_char;
                  if hold_space_size_0 == 2i32 {
                    *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space =
                      0i32 as libc::c_char
                  }
                  strcat(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    b"\n\x00" as *const u8 as *const libc::c_char,
                  );
                  if !pattern_space.is_null() {
                    strcat(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                      pattern_space,
                    );
                  }
                  current_block = 17965632435239708295;
                }
                17838047945882858323 => {
                  /* Replace hold space with pattern space */
                  free(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                  );
                  let ref mut fresh26 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh26 = xstrdup(pattern_space);
                  current_block = 17965632435239708295;
                }
                10468276026569382870 =>
                /* Append newline and hold space to pattern space */
                {
                  let mut pattern_space_size: libc::c_int = 2i32;
                  let mut hold_space_size: libc::c_int = 0i32;
                  if !pattern_space.is_null() {
                    pattern_space_size = (pattern_space_size as libc::c_ulong)
                      .wrapping_add(strlen(pattern_space))
                      as libc::c_int as libc::c_int
                  }
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size =
                      strlen((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space)
                        as libc::c_int
                  }
                  pattern_space = xrealloc(
                    pattern_space as *mut libc::c_void,
                    (pattern_space_size + hold_space_size) as size_t,
                  ) as *mut libc::c_char;
                  if pattern_space_size == 2i32 {
                    *pattern_space.offset(0) = 0i32 as libc::c_char
                  }
                  strcat(pattern_space, b"\n\x00" as *const u8 as *const libc::c_char);
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    strcat(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    );
                  }
                  last_gets_char = '\n' as i32 as libc::c_char;
                  current_block = 17965632435239708295;
                }
                14127364983570718321 => {
                  /* Replace pattern space with hold space */
                  free(pattern_space as *mut libc::c_void);
                  pattern_space = xstrdup(
                    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                      .hold_space
                      .is_null()
                    {
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                    } else {
                      b"\x00" as *const u8 as *const libc::c_char
                    },
                  );
                  current_block = 17965632435239708295;
                }
                1739363794695357236 =>
                /* Transliterate characters */
                {
                  let mut i: libc::c_int = 0;
                  let mut j: libc::c_int = 0;
                  i = 0i32;
                  while *pattern_space.offset(i as isize) != 0 {
                    j = 0i32;
                    while *(*sed_cmd).string.offset(j as isize) != 0 {
                      if *pattern_space.offset(i as isize) as libc::c_int
                        == *(*sed_cmd).string.offset(j as isize) as libc::c_int
                      {
                        *pattern_space.offset(i as isize) =
                          *(*sed_cmd).string.offset((j + 1i32) as isize);
                        break;
                      } else {
                        j += 2i32
                      }
                    }
                    i += 1
                  }
                  current_block = 17965632435239708295;
                }
                17611301398573303352 =>
                /* Test/branch if substitution occurred */
                {
                  if substituted == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted = 0i32;
                    current_block = 13861430101487131366;
                  }
                }
                7297078374430259003 =>
                /* Append the next line to the current line */
                {
                  let mut len: libc::c_int = 0;
                  /* If no next line, jump to end of script and exit. */
                  /* http://www.gnu.org/software/sed/manual/sed.html:
                   * "Most versions of sed exit without printing anything
                   * when the N command is issued on the last line of
                   * a file. GNU sed prints pattern space before exiting
                   * unless of course the -n command switch has been
                   * specified. This choice is by design."
                   */
                  if next_line.is_null() {
                    //goto discard_line;
                    current_block = 8563197331115798083;
                    break 's_54;
                  /* GNU behavior */
                  } else {
                    /* Append next_line, read new next_line. */
                    len = strlen(pattern_space) as libc::c_int;
                    pattern_space = xrealloc(
                      pattern_space as *mut libc::c_void,
                      (len as libc::c_ulong)
                        .wrapping_add(strlen(next_line))
                        .wrapping_add(2i32 as libc::c_ulong),
                    ) as *mut libc::c_char;
                    *pattern_space.offset(len as isize) = '\n' as i32 as libc::c_char;
                    strcpy(pattern_space.offset(len as isize).offset(1), next_line);
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                1534452661327725232 =>
                /* Read next line from input */
                {
                  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).be_quiet == 0 {
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      last_gets_char,
                    );
                  }
                  if next_line.is_null() {
                    /* If no next line, jump to end of script and exit. */
                    current_block = 4142149688065477410;
                    break 's_54;
                  } else {
                    free(pattern_space as *mut libc::c_void);
                    pattern_space = next_line;
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    substituted = 0i32;
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                726525485109251713 =>
                /* Read file, append contents to output */
                {
                  let mut rfile: *mut FILE = 0 as *mut FILE;
                  rfile = fopen_for_read((*sed_cmd).string);
                  if !rfile.is_null() {
                    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
                    loop {
                      line = xmalloc_fgetline(rfile);
                      if line.is_null() {
                        break;
                      }
                      append(line);
                    }
                    fclose(rfile);
                  }
                  current_block = 17965632435239708295;
                }
                13714184482889841412 =>
                /* Substitute with regex */
                {
                  if do_subst_command(sed_cmd, &mut pattern_space) == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted |= 1i32;
                    /* handle p option */
                    if (*sed_cmd).sub_p() != 0 {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        last_gets_char,
                      );
                    }
                    /* handle w option */
                    if !(*sed_cmd).sw_file.is_null() {
                      puts_maybe_newline(
                        pattern_space,
                        (*sed_cmd).sw_file,
                        &mut (*sed_cmd).sw_last_char,
                        last_gets_char,
                      );
                    }
                    current_block = 17965632435239708295;
                  }
                }
                2290177392965769716 => {
                  let mut tmp: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp.is_null() {
                    *tmp = '\u{0}' as i32 as libc::c_char;
                    /* Fall Through */
                    /* TODO: explain why '\n' below */
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                    *tmp = '\n' as i32 as libc::c_char;
                    current_block = 17965632435239708295;
                  } else {
                    current_block = 7330787439563864000;
                  }
                }
                3546145585875536353 =>
                /* Delete up through first newline */
                {
                  let mut tmp_0: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp_0.is_null() {
                    overlapping_strcpy(pattern_space, tmp_0.offset(1));
                    break;
                  } else {
                    /* discard this line. */
                    current_block = 4142149688065477410;
                    break 's_54;
                  }
                }
                10499578865704240760 =>
                /* Quit.  End of script, end of input. */
                /* Exit the outer while loop */
                {
                  free(next_line as *mut libc::c_void);
                  next_line = 0 as *mut libc::c_char;
                  current_block = 8563197331115798083;
                  break 's_54;
                }
                13665239467142187023 =>
                /* Exchange hold and pattern space */
                {
                  let mut tmp_1: *mut libc::c_char = pattern_space;
                  pattern_space = if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void
                  } else {
                    xzalloc(1i32 as size_t)
                  } as *mut libc::c_char;
                  last_gets_char = '\n' as i32 as libc::c_char;
                  let ref mut fresh28 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh28 = tmp_1;
                  current_block = 17965632435239708295;
                }
                332836909292088582 =>
                /* Print line number */
                {
                  fprintf(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    b"%d\n\x00" as *const u8 as *const libc::c_char,
                    linenum,
                  );
                  current_block = 17965632435239708295;
                }
                17152203569385922329 =>
                /* Append line to linked list to be printed later */
                {
                  append(xstrdup((*sed_cmd).string));
                  current_block = 17965632435239708295;
                }
                14487425527653873875 =>
                /* Insert text before this line */
                {
                  puts_maybe_newline(
                    (*sed_cmd).string,
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    &mut last_puts_char,
                    '\n' as i32 as libc::c_char,
                  );
                  current_block = 17965632435239708295;
                }
                2782169280456925021 =>
                /* Cut and paste text (replace) */
                /* Only triggers on last line of a matching range. */
                {
                  if (*sed_cmd).in_match() == 0 {
                    puts_maybe_newline(
                      (*sed_cmd).string,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                  }
                  current_block = 4142149688065477410;
                  break 's_54;
                }
                5622786547828025790 =>
                /* Write pattern space to file. */
                {
                  puts_maybe_newline(
                    pattern_space,
                    (*sed_cmd).sw_file,
                    &mut (*sed_cmd).sw_last_char,
                    last_gets_char,
                  );
                  current_block = 17965632435239708295;
                }
                _ => {}
              }
              match current_block {
                17965632435239708295 => {}
                _ => {
                  match current_block {
                    7330787439563864000 =>
                    /* Write the current pattern space to output */
                    /* NB: we print this _before_ the last line
                     * (of current file) is printed. Even if
                     * that line is nonterminated, we print
                     * '\n' here (gnu sed does the same) */
                    {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        '\n' as i32 as libc::c_char,
                      );
                      current_block = 17965632435239708295;
                    }
                    13861430101487131366 =>
                    /* Fall through */
                    /* Test/branch if substitution didn't occur */
                    {
                      if substituted != 0 {
                        current_block = 17965632435239708295;
                      } else {
                        current_block = 2887315643959147419;
                      }
                    }
                    _ => {}
                  }
                  match current_block {
                    17965632435239708295 => {}
                    _ =>
                    /* Fall through */
                    /* Branch to label */
                    {
                      if (*sed_cmd).string.is_null() {
                        current_block = 8563197331115798083;
                        break 's_54;
                      }
                      sed_cmd = branch_to((*sed_cmd).string)
                    }
                  }
                }
              }
            }
            80 => {
              current_block = 2290177392965769716;
              match current_block {
                11227437541145425351 => {
                  let mut hold_space_size_0: libc::c_int = 2i32;
                  let mut pattern_space_size_0: libc::c_int = 0i32;
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size_0 = (hold_space_size_0 as libc::c_ulong).wrapping_add(strlen(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    )) as libc::c_int as libc::c_int
                  }
                  if !pattern_space.is_null() {
                    pattern_space_size_0 = strlen(pattern_space) as libc::c_int
                  }
                  let ref mut fresh27 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh27 = xrealloc(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                    (hold_space_size_0 + pattern_space_size_0) as size_t,
                  ) as *mut libc::c_char;
                  if hold_space_size_0 == 2i32 {
                    *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space =
                      0i32 as libc::c_char
                  }
                  strcat(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    b"\n\x00" as *const u8 as *const libc::c_char,
                  );
                  if !pattern_space.is_null() {
                    strcat(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                      pattern_space,
                    );
                  }
                  current_block = 17965632435239708295;
                }
                17838047945882858323 => {
                  free(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                  );
                  let ref mut fresh26 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh26 = xstrdup(pattern_space);
                  current_block = 17965632435239708295;
                }
                10468276026569382870 => {
                  let mut pattern_space_size: libc::c_int = 2i32;
                  let mut hold_space_size: libc::c_int = 0i32;
                  if !pattern_space.is_null() {
                    pattern_space_size = (pattern_space_size as libc::c_ulong)
                      .wrapping_add(strlen(pattern_space))
                      as libc::c_int as libc::c_int
                  }
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size =
                      strlen((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space)
                        as libc::c_int
                  }
                  pattern_space = xrealloc(
                    pattern_space as *mut libc::c_void,
                    (pattern_space_size + hold_space_size) as size_t,
                  ) as *mut libc::c_char;
                  if pattern_space_size == 2i32 {
                    *pattern_space.offset(0) = 0i32 as libc::c_char
                  }
                  strcat(pattern_space, b"\n\x00" as *const u8 as *const libc::c_char);
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    strcat(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    );
                  }
                  last_gets_char = '\n' as i32 as libc::c_char;
                  current_block = 17965632435239708295;
                }
                14127364983570718321 => {
                  free(pattern_space as *mut libc::c_void);
                  pattern_space = xstrdup(
                    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                      .hold_space
                      .is_null()
                    {
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                    } else {
                      b"\x00" as *const u8 as *const libc::c_char
                    },
                  );
                  current_block = 17965632435239708295;
                }
                1739363794695357236 => {
                  let mut i: libc::c_int = 0;
                  let mut j: libc::c_int = 0;
                  i = 0i32;
                  while *pattern_space.offset(i as isize) != 0 {
                    j = 0i32;
                    while *(*sed_cmd).string.offset(j as isize) != 0 {
                      if *pattern_space.offset(i as isize) as libc::c_int
                        == *(*sed_cmd).string.offset(j as isize) as libc::c_int
                      {
                        *pattern_space.offset(i as isize) =
                          *(*sed_cmd).string.offset((j + 1i32) as isize);
                        break;
                      } else {
                        j += 2i32
                      }
                    }
                    i += 1
                  }
                  current_block = 17965632435239708295;
                }
                17611301398573303352 => {
                  if substituted == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted = 0i32;
                    current_block = 13861430101487131366;
                  }
                }
                7297078374430259003 => {
                  let mut len: libc::c_int = 0;
                  if next_line.is_null() {
                    current_block = 8563197331115798083;
                    break 's_54;
                  } else {
                    len = strlen(pattern_space) as libc::c_int;
                    pattern_space = xrealloc(
                      pattern_space as *mut libc::c_void,
                      (len as libc::c_ulong)
                        .wrapping_add(strlen(next_line))
                        .wrapping_add(2i32 as libc::c_ulong),
                    ) as *mut libc::c_char;
                    *pattern_space.offset(len as isize) = '\n' as i32 as libc::c_char;
                    strcpy(pattern_space.offset(len as isize).offset(1), next_line);
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                1534452661327725232 => {
                  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).be_quiet == 0 {
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      last_gets_char,
                    );
                  }
                  if next_line.is_null() {
                    current_block = 4142149688065477410;
                    break 's_54;
                  } else {
                    free(pattern_space as *mut libc::c_void);
                    pattern_space = next_line;
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    substituted = 0i32;
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                726525485109251713 => {
                  let mut rfile: *mut FILE = 0 as *mut FILE;
                  rfile = fopen_for_read((*sed_cmd).string);
                  if !rfile.is_null() {
                    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
                    loop {
                      line = xmalloc_fgetline(rfile);
                      if line.is_null() {
                        break;
                      }
                      append(line);
                    }
                    fclose(rfile);
                  }
                  current_block = 17965632435239708295;
                }
                13714184482889841412 => {
                  if do_subst_command(sed_cmd, &mut pattern_space) == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted |= 1i32;
                    if (*sed_cmd).sub_p() != 0 {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        last_gets_char,
                      );
                    }
                    if !(*sed_cmd).sw_file.is_null() {
                      puts_maybe_newline(
                        pattern_space,
                        (*sed_cmd).sw_file,
                        &mut (*sed_cmd).sw_last_char,
                        last_gets_char,
                      );
                    }
                    current_block = 17965632435239708295;
                  }
                }
                2290177392965769716 => {
                  let mut tmp: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp.is_null() {
                    *tmp = '\u{0}' as i32 as libc::c_char;
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                    *tmp = '\n' as i32 as libc::c_char;
                    current_block = 17965632435239708295;
                  } else {
                    current_block = 7330787439563864000;
                  }
                }
                3546145585875536353 => {
                  let mut tmp_0: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp_0.is_null() {
                    overlapping_strcpy(pattern_space, tmp_0.offset(1));
                    break;
                  } else {
                    current_block = 4142149688065477410;
                    break 's_54;
                  }
                }
                10499578865704240760 => {
                  free(next_line as *mut libc::c_void);
                  next_line = 0 as *mut libc::c_char;
                  current_block = 8563197331115798083;
                  break 's_54;
                }
                13665239467142187023 => {
                  let mut tmp_1: *mut libc::c_char = pattern_space;
                  pattern_space = if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void
                  } else {
                    xzalloc(1i32 as size_t)
                  } as *mut libc::c_char;
                  last_gets_char = '\n' as i32 as libc::c_char;
                  let ref mut fresh28 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh28 = tmp_1;
                  current_block = 17965632435239708295;
                }
                332836909292088582 => {
                  fprintf(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    b"%d\n\x00" as *const u8 as *const libc::c_char,
                    linenum,
                  );
                  current_block = 17965632435239708295;
                }
                17152203569385922329 => {
                  append(xstrdup((*sed_cmd).string));
                  current_block = 17965632435239708295;
                }
                14487425527653873875 => {
                  puts_maybe_newline(
                    (*sed_cmd).string,
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    &mut last_puts_char,
                    '\n' as i32 as libc::c_char,
                  );
                  current_block = 17965632435239708295;
                }
                2782169280456925021 => {
                  if (*sed_cmd).in_match() == 0 {
                    puts_maybe_newline(
                      (*sed_cmd).string,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                  }
                  current_block = 4142149688065477410;
                  break 's_54;
                }
                5622786547828025790 => {
                  puts_maybe_newline(
                    pattern_space,
                    (*sed_cmd).sw_file,
                    &mut (*sed_cmd).sw_last_char,
                    last_gets_char,
                  );
                  current_block = 17965632435239708295;
                }
                _ => {}
              }
              match current_block {
                17965632435239708295 => {}
                _ => {
                  match current_block {
                    7330787439563864000 => {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        '\n' as i32 as libc::c_char,
                      );
                      current_block = 17965632435239708295;
                    }
                    13861430101487131366 => {
                      if substituted != 0 {
                        current_block = 17965632435239708295;
                      } else {
                        current_block = 2887315643959147419;
                      }
                    }
                    _ => {}
                  }
                  match current_block {
                    17965632435239708295 => {}
                    _ => {
                      if (*sed_cmd).string.is_null() {
                        current_block = 8563197331115798083;
                        break 's_54;
                      }
                      sed_cmd = branch_to((*sed_cmd).string)
                    }
                  }
                }
              }
            }
            112 => {
              current_block = 7330787439563864000;
              match current_block {
                11227437541145425351 => {
                  let mut hold_space_size_0: libc::c_int = 2i32;
                  let mut pattern_space_size_0: libc::c_int = 0i32;
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size_0 = (hold_space_size_0 as libc::c_ulong).wrapping_add(strlen(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    )) as libc::c_int as libc::c_int
                  }
                  if !pattern_space.is_null() {
                    pattern_space_size_0 = strlen(pattern_space) as libc::c_int
                  }
                  let ref mut fresh27 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh27 = xrealloc(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                    (hold_space_size_0 + pattern_space_size_0) as size_t,
                  ) as *mut libc::c_char;
                  if hold_space_size_0 == 2i32 {
                    *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space =
                      0i32 as libc::c_char
                  }
                  strcat(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    b"\n\x00" as *const u8 as *const libc::c_char,
                  );
                  if !pattern_space.is_null() {
                    strcat(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                      pattern_space,
                    );
                  }
                  current_block = 17965632435239708295;
                }
                17838047945882858323 => {
                  free(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                  );
                  let ref mut fresh26 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh26 = xstrdup(pattern_space);
                  current_block = 17965632435239708295;
                }
                10468276026569382870 => {
                  let mut pattern_space_size: libc::c_int = 2i32;
                  let mut hold_space_size: libc::c_int = 0i32;
                  if !pattern_space.is_null() {
                    pattern_space_size = (pattern_space_size as libc::c_ulong)
                      .wrapping_add(strlen(pattern_space))
                      as libc::c_int as libc::c_int
                  }
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size =
                      strlen((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space)
                        as libc::c_int
                  }
                  pattern_space = xrealloc(
                    pattern_space as *mut libc::c_void,
                    (pattern_space_size + hold_space_size) as size_t,
                  ) as *mut libc::c_char;
                  if pattern_space_size == 2i32 {
                    *pattern_space.offset(0) = 0i32 as libc::c_char
                  }
                  strcat(pattern_space, b"\n\x00" as *const u8 as *const libc::c_char);
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    strcat(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    );
                  }
                  last_gets_char = '\n' as i32 as libc::c_char;
                  current_block = 17965632435239708295;
                }
                14127364983570718321 => {
                  free(pattern_space as *mut libc::c_void);
                  pattern_space = xstrdup(
                    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                      .hold_space
                      .is_null()
                    {
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                    } else {
                      b"\x00" as *const u8 as *const libc::c_char
                    },
                  );
                  current_block = 17965632435239708295;
                }
                1739363794695357236 => {
                  let mut i: libc::c_int = 0;
                  let mut j: libc::c_int = 0;
                  i = 0i32;
                  while *pattern_space.offset(i as isize) != 0 {
                    j = 0i32;
                    while *(*sed_cmd).string.offset(j as isize) != 0 {
                      if *pattern_space.offset(i as isize) as libc::c_int
                        == *(*sed_cmd).string.offset(j as isize) as libc::c_int
                      {
                        *pattern_space.offset(i as isize) =
                          *(*sed_cmd).string.offset((j + 1i32) as isize);
                        break;
                      } else {
                        j += 2i32
                      }
                    }
                    i += 1
                  }
                  current_block = 17965632435239708295;
                }
                17611301398573303352 => {
                  if substituted == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted = 0i32;
                    current_block = 13861430101487131366;
                  }
                }
                7297078374430259003 => {
                  let mut len: libc::c_int = 0;
                  if next_line.is_null() {
                    current_block = 8563197331115798083;
                    break 's_54;
                  } else {
                    len = strlen(pattern_space) as libc::c_int;
                    pattern_space = xrealloc(
                      pattern_space as *mut libc::c_void,
                      (len as libc::c_ulong)
                        .wrapping_add(strlen(next_line))
                        .wrapping_add(2i32 as libc::c_ulong),
                    ) as *mut libc::c_char;
                    *pattern_space.offset(len as isize) = '\n' as i32 as libc::c_char;
                    strcpy(pattern_space.offset(len as isize).offset(1), next_line);
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                1534452661327725232 => {
                  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).be_quiet == 0 {
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      last_gets_char,
                    );
                  }
                  if next_line.is_null() {
                    current_block = 4142149688065477410;
                    break 's_54;
                  } else {
                    free(pattern_space as *mut libc::c_void);
                    pattern_space = next_line;
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    substituted = 0i32;
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                726525485109251713 => {
                  let mut rfile: *mut FILE = 0 as *mut FILE;
                  rfile = fopen_for_read((*sed_cmd).string);
                  if !rfile.is_null() {
                    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
                    loop {
                      line = xmalloc_fgetline(rfile);
                      if line.is_null() {
                        break;
                      }
                      append(line);
                    }
                    fclose(rfile);
                  }
                  current_block = 17965632435239708295;
                }
                13714184482889841412 => {
                  if do_subst_command(sed_cmd, &mut pattern_space) == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted |= 1i32;
                    if (*sed_cmd).sub_p() != 0 {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        last_gets_char,
                      );
                    }
                    if !(*sed_cmd).sw_file.is_null() {
                      puts_maybe_newline(
                        pattern_space,
                        (*sed_cmd).sw_file,
                        &mut (*sed_cmd).sw_last_char,
                        last_gets_char,
                      );
                    }
                    current_block = 17965632435239708295;
                  }
                }
                2290177392965769716 => {
                  let mut tmp: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp.is_null() {
                    *tmp = '\u{0}' as i32 as libc::c_char;
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                    *tmp = '\n' as i32 as libc::c_char;
                    current_block = 17965632435239708295;
                  } else {
                    current_block = 7330787439563864000;
                  }
                }
                3546145585875536353 => {
                  let mut tmp_0: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp_0.is_null() {
                    overlapping_strcpy(pattern_space, tmp_0.offset(1));
                    break;
                  } else {
                    current_block = 4142149688065477410;
                    break 's_54;
                  }
                }
                10499578865704240760 => {
                  free(next_line as *mut libc::c_void);
                  next_line = 0 as *mut libc::c_char;
                  current_block = 8563197331115798083;
                  break 's_54;
                }
                13665239467142187023 => {
                  let mut tmp_1: *mut libc::c_char = pattern_space;
                  pattern_space = if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void
                  } else {
                    xzalloc(1i32 as size_t)
                  } as *mut libc::c_char;
                  last_gets_char = '\n' as i32 as libc::c_char;
                  let ref mut fresh28 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh28 = tmp_1;
                  current_block = 17965632435239708295;
                }
                332836909292088582 => {
                  fprintf(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    b"%d\n\x00" as *const u8 as *const libc::c_char,
                    linenum,
                  );
                  current_block = 17965632435239708295;
                }
                17152203569385922329 => {
                  append(xstrdup((*sed_cmd).string));
                  current_block = 17965632435239708295;
                }
                14487425527653873875 => {
                  puts_maybe_newline(
                    (*sed_cmd).string,
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    &mut last_puts_char,
                    '\n' as i32 as libc::c_char,
                  );
                  current_block = 17965632435239708295;
                }
                2782169280456925021 => {
                  if (*sed_cmd).in_match() == 0 {
                    puts_maybe_newline(
                      (*sed_cmd).string,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                  }
                  current_block = 4142149688065477410;
                  break 's_54;
                }
                5622786547828025790 => {
                  puts_maybe_newline(
                    pattern_space,
                    (*sed_cmd).sw_file,
                    &mut (*sed_cmd).sw_last_char,
                    last_gets_char,
                  );
                  current_block = 17965632435239708295;
                }
                _ => {}
              }
              match current_block {
                17965632435239708295 => {}
                _ => {
                  match current_block {
                    7330787439563864000 => {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        '\n' as i32 as libc::c_char,
                      );
                      current_block = 17965632435239708295;
                    }
                    13861430101487131366 => {
                      if substituted != 0 {
                        current_block = 17965632435239708295;
                      } else {
                        current_block = 2887315643959147419;
                      }
                    }
                    _ => {}
                  }
                  match current_block {
                    17965632435239708295 => {}
                    _ => {
                      if (*sed_cmd).string.is_null() {
                        current_block = 8563197331115798083;
                        break 's_54;
                      }
                      sed_cmd = branch_to((*sed_cmd).string)
                    }
                  }
                }
              }
            }
            68 => {
              current_block = 3546145585875536353;
              match current_block {
                11227437541145425351 => {
                  let mut hold_space_size_0: libc::c_int = 2i32;
                  let mut pattern_space_size_0: libc::c_int = 0i32;
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size_0 = (hold_space_size_0 as libc::c_ulong).wrapping_add(strlen(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    )) as libc::c_int as libc::c_int
                  }
                  if !pattern_space.is_null() {
                    pattern_space_size_0 = strlen(pattern_space) as libc::c_int
                  }
                  let ref mut fresh27 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh27 = xrealloc(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                    (hold_space_size_0 + pattern_space_size_0) as size_t,
                  ) as *mut libc::c_char;
                  if hold_space_size_0 == 2i32 {
                    *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space =
                      0i32 as libc::c_char
                  }
                  strcat(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    b"\n\x00" as *const u8 as *const libc::c_char,
                  );
                  if !pattern_space.is_null() {
                    strcat(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                      pattern_space,
                    );
                  }
                  current_block = 17965632435239708295;
                }
                17838047945882858323 => {
                  free(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                  );
                  let ref mut fresh26 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh26 = xstrdup(pattern_space);
                  current_block = 17965632435239708295;
                }
                10468276026569382870 => {
                  let mut pattern_space_size: libc::c_int = 2i32;
                  let mut hold_space_size: libc::c_int = 0i32;
                  if !pattern_space.is_null() {
                    pattern_space_size = (pattern_space_size as libc::c_ulong)
                      .wrapping_add(strlen(pattern_space))
                      as libc::c_int as libc::c_int
                  }
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size =
                      strlen((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space)
                        as libc::c_int
                  }
                  pattern_space = xrealloc(
                    pattern_space as *mut libc::c_void,
                    (pattern_space_size + hold_space_size) as size_t,
                  ) as *mut libc::c_char;
                  if pattern_space_size == 2i32 {
                    *pattern_space.offset(0) = 0i32 as libc::c_char
                  }
                  strcat(pattern_space, b"\n\x00" as *const u8 as *const libc::c_char);
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    strcat(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    );
                  }
                  last_gets_char = '\n' as i32 as libc::c_char;
                  current_block = 17965632435239708295;
                }
                14127364983570718321 => {
                  free(pattern_space as *mut libc::c_void);
                  pattern_space = xstrdup(
                    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                      .hold_space
                      .is_null()
                    {
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                    } else {
                      b"\x00" as *const u8 as *const libc::c_char
                    },
                  );
                  current_block = 17965632435239708295;
                }
                1739363794695357236 => {
                  let mut i: libc::c_int = 0;
                  let mut j: libc::c_int = 0;
                  i = 0i32;
                  while *pattern_space.offset(i as isize) != 0 {
                    j = 0i32;
                    while *(*sed_cmd).string.offset(j as isize) != 0 {
                      if *pattern_space.offset(i as isize) as libc::c_int
                        == *(*sed_cmd).string.offset(j as isize) as libc::c_int
                      {
                        *pattern_space.offset(i as isize) =
                          *(*sed_cmd).string.offset((j + 1i32) as isize);
                        break;
                      } else {
                        j += 2i32
                      }
                    }
                    i += 1
                  }
                  current_block = 17965632435239708295;
                }
                17611301398573303352 => {
                  if substituted == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted = 0i32;
                    current_block = 13861430101487131366;
                  }
                }
                7297078374430259003 => {
                  let mut len: libc::c_int = 0;
                  if next_line.is_null() {
                    current_block = 8563197331115798083;
                    break 's_54;
                  } else {
                    len = strlen(pattern_space) as libc::c_int;
                    pattern_space = xrealloc(
                      pattern_space as *mut libc::c_void,
                      (len as libc::c_ulong)
                        .wrapping_add(strlen(next_line))
                        .wrapping_add(2i32 as libc::c_ulong),
                    ) as *mut libc::c_char;
                    *pattern_space.offset(len as isize) = '\n' as i32 as libc::c_char;
                    strcpy(pattern_space.offset(len as isize).offset(1), next_line);
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                1534452661327725232 => {
                  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).be_quiet == 0 {
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      last_gets_char,
                    );
                  }
                  if next_line.is_null() {
                    current_block = 4142149688065477410;
                    break 's_54;
                  } else {
                    free(pattern_space as *mut libc::c_void);
                    pattern_space = next_line;
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    substituted = 0i32;
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                726525485109251713 => {
                  let mut rfile: *mut FILE = 0 as *mut FILE;
                  rfile = fopen_for_read((*sed_cmd).string);
                  if !rfile.is_null() {
                    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
                    loop {
                      line = xmalloc_fgetline(rfile);
                      if line.is_null() {
                        break;
                      }
                      append(line);
                    }
                    fclose(rfile);
                  }
                  current_block = 17965632435239708295;
                }
                13714184482889841412 => {
                  if do_subst_command(sed_cmd, &mut pattern_space) == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted |= 1i32;
                    if (*sed_cmd).sub_p() != 0 {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        last_gets_char,
                      );
                    }
                    if !(*sed_cmd).sw_file.is_null() {
                      puts_maybe_newline(
                        pattern_space,
                        (*sed_cmd).sw_file,
                        &mut (*sed_cmd).sw_last_char,
                        last_gets_char,
                      );
                    }
                    current_block = 17965632435239708295;
                  }
                }
                2290177392965769716 => {
                  let mut tmp: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp.is_null() {
                    *tmp = '\u{0}' as i32 as libc::c_char;
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                    *tmp = '\n' as i32 as libc::c_char;
                    current_block = 17965632435239708295;
                  } else {
                    current_block = 7330787439563864000;
                  }
                }
                3546145585875536353 => {
                  let mut tmp_0: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp_0.is_null() {
                    overlapping_strcpy(pattern_space, tmp_0.offset(1));
                    break;
                  } else {
                    current_block = 4142149688065477410;
                    break 's_54;
                  }
                }
                10499578865704240760 => {
                  free(next_line as *mut libc::c_void);
                  next_line = 0 as *mut libc::c_char;
                  current_block = 8563197331115798083;
                  break 's_54;
                }
                13665239467142187023 => {
                  let mut tmp_1: *mut libc::c_char = pattern_space;
                  pattern_space = if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void
                  } else {
                    xzalloc(1i32 as size_t)
                  } as *mut libc::c_char;
                  last_gets_char = '\n' as i32 as libc::c_char;
                  let ref mut fresh28 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh28 = tmp_1;
                  current_block = 17965632435239708295;
                }
                332836909292088582 => {
                  fprintf(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    b"%d\n\x00" as *const u8 as *const libc::c_char,
                    linenum,
                  );
                  current_block = 17965632435239708295;
                }
                17152203569385922329 => {
                  append(xstrdup((*sed_cmd).string));
                  current_block = 17965632435239708295;
                }
                14487425527653873875 => {
                  puts_maybe_newline(
                    (*sed_cmd).string,
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    &mut last_puts_char,
                    '\n' as i32 as libc::c_char,
                  );
                  current_block = 17965632435239708295;
                }
                2782169280456925021 => {
                  if (*sed_cmd).in_match() == 0 {
                    puts_maybe_newline(
                      (*sed_cmd).string,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                  }
                  current_block = 4142149688065477410;
                  break 's_54;
                }
                5622786547828025790 => {
                  puts_maybe_newline(
                    pattern_space,
                    (*sed_cmd).sw_file,
                    &mut (*sed_cmd).sw_last_char,
                    last_gets_char,
                  );
                  current_block = 17965632435239708295;
                }
                _ => {}
              }
              match current_block {
                17965632435239708295 => {}
                _ => {
                  match current_block {
                    7330787439563864000 => {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        '\n' as i32 as libc::c_char,
                      );
                      current_block = 17965632435239708295;
                    }
                    13861430101487131366 => {
                      if substituted != 0 {
                        current_block = 17965632435239708295;
                      } else {
                        current_block = 2887315643959147419;
                      }
                    }
                    _ => {}
                  }
                  match current_block {
                    17965632435239708295 => {}
                    _ => {
                      if (*sed_cmd).string.is_null() {
                        current_block = 8563197331115798083;
                        break 's_54;
                      }
                      sed_cmd = branch_to((*sed_cmd).string)
                    }
                  }
                }
              }
            }
            100 => {
              current_block = 4142149688065477410;
              break 's_54;
            }
            115 => {
              current_block = 13714184482889841412;
              match current_block {
                11227437541145425351 => {
                  let mut hold_space_size_0: libc::c_int = 2i32;
                  let mut pattern_space_size_0: libc::c_int = 0i32;
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size_0 = (hold_space_size_0 as libc::c_ulong).wrapping_add(strlen(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    )) as libc::c_int as libc::c_int
                  }
                  if !pattern_space.is_null() {
                    pattern_space_size_0 = strlen(pattern_space) as libc::c_int
                  }
                  let ref mut fresh27 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh27 = xrealloc(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                    (hold_space_size_0 + pattern_space_size_0) as size_t,
                  ) as *mut libc::c_char;
                  if hold_space_size_0 == 2i32 {
                    *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space =
                      0i32 as libc::c_char
                  }
                  strcat(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    b"\n\x00" as *const u8 as *const libc::c_char,
                  );
                  if !pattern_space.is_null() {
                    strcat(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                      pattern_space,
                    );
                  }
                  current_block = 17965632435239708295;
                }
                17838047945882858323 => {
                  free(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                  );
                  let ref mut fresh26 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh26 = xstrdup(pattern_space);
                  current_block = 17965632435239708295;
                }
                10468276026569382870 => {
                  let mut pattern_space_size: libc::c_int = 2i32;
                  let mut hold_space_size: libc::c_int = 0i32;
                  if !pattern_space.is_null() {
                    pattern_space_size = (pattern_space_size as libc::c_ulong)
                      .wrapping_add(strlen(pattern_space))
                      as libc::c_int as libc::c_int
                  }
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size =
                      strlen((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space)
                        as libc::c_int
                  }
                  pattern_space = xrealloc(
                    pattern_space as *mut libc::c_void,
                    (pattern_space_size + hold_space_size) as size_t,
                  ) as *mut libc::c_char;
                  if pattern_space_size == 2i32 {
                    *pattern_space.offset(0) = 0i32 as libc::c_char
                  }
                  strcat(pattern_space, b"\n\x00" as *const u8 as *const libc::c_char);
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    strcat(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    );
                  }
                  last_gets_char = '\n' as i32 as libc::c_char;
                  current_block = 17965632435239708295;
                }
                14127364983570718321 => {
                  free(pattern_space as *mut libc::c_void);
                  pattern_space = xstrdup(
                    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                      .hold_space
                      .is_null()
                    {
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                    } else {
                      b"\x00" as *const u8 as *const libc::c_char
                    },
                  );
                  current_block = 17965632435239708295;
                }
                1739363794695357236 => {
                  let mut i: libc::c_int = 0;
                  let mut j: libc::c_int = 0;
                  i = 0i32;
                  while *pattern_space.offset(i as isize) != 0 {
                    j = 0i32;
                    while *(*sed_cmd).string.offset(j as isize) != 0 {
                      if *pattern_space.offset(i as isize) as libc::c_int
                        == *(*sed_cmd).string.offset(j as isize) as libc::c_int
                      {
                        *pattern_space.offset(i as isize) =
                          *(*sed_cmd).string.offset((j + 1i32) as isize);
                        break;
                      } else {
                        j += 2i32
                      }
                    }
                    i += 1
                  }
                  current_block = 17965632435239708295;
                }
                17611301398573303352 => {
                  if substituted == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted = 0i32;
                    current_block = 13861430101487131366;
                  }
                }
                7297078374430259003 => {
                  let mut len: libc::c_int = 0;
                  if next_line.is_null() {
                    current_block = 8563197331115798083;
                    break 's_54;
                  } else {
                    len = strlen(pattern_space) as libc::c_int;
                    pattern_space = xrealloc(
                      pattern_space as *mut libc::c_void,
                      (len as libc::c_ulong)
                        .wrapping_add(strlen(next_line))
                        .wrapping_add(2i32 as libc::c_ulong),
                    ) as *mut libc::c_char;
                    *pattern_space.offset(len as isize) = '\n' as i32 as libc::c_char;
                    strcpy(pattern_space.offset(len as isize).offset(1), next_line);
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                1534452661327725232 => {
                  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).be_quiet == 0 {
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      last_gets_char,
                    );
                  }
                  if next_line.is_null() {
                    current_block = 4142149688065477410;
                    break 's_54;
                  } else {
                    free(pattern_space as *mut libc::c_void);
                    pattern_space = next_line;
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    substituted = 0i32;
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                726525485109251713 => {
                  let mut rfile: *mut FILE = 0 as *mut FILE;
                  rfile = fopen_for_read((*sed_cmd).string);
                  if !rfile.is_null() {
                    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
                    loop {
                      line = xmalloc_fgetline(rfile);
                      if line.is_null() {
                        break;
                      }
                      append(line);
                    }
                    fclose(rfile);
                  }
                  current_block = 17965632435239708295;
                }
                13714184482889841412 => {
                  if do_subst_command(sed_cmd, &mut pattern_space) == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted |= 1i32;
                    if (*sed_cmd).sub_p() != 0 {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        last_gets_char,
                      );
                    }
                    if !(*sed_cmd).sw_file.is_null() {
                      puts_maybe_newline(
                        pattern_space,
                        (*sed_cmd).sw_file,
                        &mut (*sed_cmd).sw_last_char,
                        last_gets_char,
                      );
                    }
                    current_block = 17965632435239708295;
                  }
                }
                2290177392965769716 => {
                  let mut tmp: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp.is_null() {
                    *tmp = '\u{0}' as i32 as libc::c_char;
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                    *tmp = '\n' as i32 as libc::c_char;
                    current_block = 17965632435239708295;
                  } else {
                    current_block = 7330787439563864000;
                  }
                }
                3546145585875536353 => {
                  let mut tmp_0: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp_0.is_null() {
                    overlapping_strcpy(pattern_space, tmp_0.offset(1));
                    break;
                  } else {
                    current_block = 4142149688065477410;
                    break 's_54;
                  }
                }
                10499578865704240760 => {
                  free(next_line as *mut libc::c_void);
                  next_line = 0 as *mut libc::c_char;
                  current_block = 8563197331115798083;
                  break 's_54;
                }
                13665239467142187023 => {
                  let mut tmp_1: *mut libc::c_char = pattern_space;
                  pattern_space = if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void
                  } else {
                    xzalloc(1i32 as size_t)
                  } as *mut libc::c_char;
                  last_gets_char = '\n' as i32 as libc::c_char;
                  let ref mut fresh28 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh28 = tmp_1;
                  current_block = 17965632435239708295;
                }
                332836909292088582 => {
                  fprintf(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    b"%d\n\x00" as *const u8 as *const libc::c_char,
                    linenum,
                  );
                  current_block = 17965632435239708295;
                }
                17152203569385922329 => {
                  append(xstrdup((*sed_cmd).string));
                  current_block = 17965632435239708295;
                }
                14487425527653873875 => {
                  puts_maybe_newline(
                    (*sed_cmd).string,
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    &mut last_puts_char,
                    '\n' as i32 as libc::c_char,
                  );
                  current_block = 17965632435239708295;
                }
                2782169280456925021 => {
                  if (*sed_cmd).in_match() == 0 {
                    puts_maybe_newline(
                      (*sed_cmd).string,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                  }
                  current_block = 4142149688065477410;
                  break 's_54;
                }
                5622786547828025790 => {
                  puts_maybe_newline(
                    pattern_space,
                    (*sed_cmd).sw_file,
                    &mut (*sed_cmd).sw_last_char,
                    last_gets_char,
                  );
                  current_block = 17965632435239708295;
                }
                _ => {}
              }
              match current_block {
                17965632435239708295 => {}
                _ => {
                  match current_block {
                    7330787439563864000 => {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        '\n' as i32 as libc::c_char,
                      );
                      current_block = 17965632435239708295;
                    }
                    13861430101487131366 => {
                      if substituted != 0 {
                        current_block = 17965632435239708295;
                      } else {
                        current_block = 2887315643959147419;
                      }
                    }
                    _ => {}
                  }
                  match current_block {
                    17965632435239708295 => {}
                    _ => {
                      if (*sed_cmd).string.is_null() {
                        current_block = 8563197331115798083;
                        break 's_54;
                      }
                      sed_cmd = branch_to((*sed_cmd).string)
                    }
                  }
                }
              }
            }
            97 => {
              current_block = 17152203569385922329;
              match current_block {
                11227437541145425351 => {
                  let mut hold_space_size_0: libc::c_int = 2i32;
                  let mut pattern_space_size_0: libc::c_int = 0i32;
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size_0 = (hold_space_size_0 as libc::c_ulong).wrapping_add(strlen(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    )) as libc::c_int as libc::c_int
                  }
                  if !pattern_space.is_null() {
                    pattern_space_size_0 = strlen(pattern_space) as libc::c_int
                  }
                  let ref mut fresh27 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh27 = xrealloc(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                    (hold_space_size_0 + pattern_space_size_0) as size_t,
                  ) as *mut libc::c_char;
                  if hold_space_size_0 == 2i32 {
                    *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space =
                      0i32 as libc::c_char
                  }
                  strcat(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    b"\n\x00" as *const u8 as *const libc::c_char,
                  );
                  if !pattern_space.is_null() {
                    strcat(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                      pattern_space,
                    );
                  }
                  current_block = 17965632435239708295;
                }
                17838047945882858323 => {
                  free(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                  );
                  let ref mut fresh26 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh26 = xstrdup(pattern_space);
                  current_block = 17965632435239708295;
                }
                10468276026569382870 => {
                  let mut pattern_space_size: libc::c_int = 2i32;
                  let mut hold_space_size: libc::c_int = 0i32;
                  if !pattern_space.is_null() {
                    pattern_space_size = (pattern_space_size as libc::c_ulong)
                      .wrapping_add(strlen(pattern_space))
                      as libc::c_int as libc::c_int
                  }
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size =
                      strlen((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space)
                        as libc::c_int
                  }
                  pattern_space = xrealloc(
                    pattern_space as *mut libc::c_void,
                    (pattern_space_size + hold_space_size) as size_t,
                  ) as *mut libc::c_char;
                  if pattern_space_size == 2i32 {
                    *pattern_space.offset(0) = 0i32 as libc::c_char
                  }
                  strcat(pattern_space, b"\n\x00" as *const u8 as *const libc::c_char);
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    strcat(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    );
                  }
                  last_gets_char = '\n' as i32 as libc::c_char;
                  current_block = 17965632435239708295;
                }
                14127364983570718321 => {
                  free(pattern_space as *mut libc::c_void);
                  pattern_space = xstrdup(
                    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                      .hold_space
                      .is_null()
                    {
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                    } else {
                      b"\x00" as *const u8 as *const libc::c_char
                    },
                  );
                  current_block = 17965632435239708295;
                }
                1739363794695357236 => {
                  let mut i: libc::c_int = 0;
                  let mut j: libc::c_int = 0;
                  i = 0i32;
                  while *pattern_space.offset(i as isize) != 0 {
                    j = 0i32;
                    while *(*sed_cmd).string.offset(j as isize) != 0 {
                      if *pattern_space.offset(i as isize) as libc::c_int
                        == *(*sed_cmd).string.offset(j as isize) as libc::c_int
                      {
                        *pattern_space.offset(i as isize) =
                          *(*sed_cmd).string.offset((j + 1i32) as isize);
                        break;
                      } else {
                        j += 2i32
                      }
                    }
                    i += 1
                  }
                  current_block = 17965632435239708295;
                }
                17611301398573303352 => {
                  if substituted == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted = 0i32;
                    current_block = 13861430101487131366;
                  }
                }
                7297078374430259003 => {
                  let mut len: libc::c_int = 0;
                  if next_line.is_null() {
                    current_block = 8563197331115798083;
                    break 's_54;
                  } else {
                    len = strlen(pattern_space) as libc::c_int;
                    pattern_space = xrealloc(
                      pattern_space as *mut libc::c_void,
                      (len as libc::c_ulong)
                        .wrapping_add(strlen(next_line))
                        .wrapping_add(2i32 as libc::c_ulong),
                    ) as *mut libc::c_char;
                    *pattern_space.offset(len as isize) = '\n' as i32 as libc::c_char;
                    strcpy(pattern_space.offset(len as isize).offset(1), next_line);
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                1534452661327725232 => {
                  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).be_quiet == 0 {
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      last_gets_char,
                    );
                  }
                  if next_line.is_null() {
                    current_block = 4142149688065477410;
                    break 's_54;
                  } else {
                    free(pattern_space as *mut libc::c_void);
                    pattern_space = next_line;
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    substituted = 0i32;
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                726525485109251713 => {
                  let mut rfile: *mut FILE = 0 as *mut FILE;
                  rfile = fopen_for_read((*sed_cmd).string);
                  if !rfile.is_null() {
                    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
                    loop {
                      line = xmalloc_fgetline(rfile);
                      if line.is_null() {
                        break;
                      }
                      append(line);
                    }
                    fclose(rfile);
                  }
                  current_block = 17965632435239708295;
                }
                13714184482889841412 => {
                  if do_subst_command(sed_cmd, &mut pattern_space) == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted |= 1i32;
                    if (*sed_cmd).sub_p() != 0 {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        last_gets_char,
                      );
                    }
                    if !(*sed_cmd).sw_file.is_null() {
                      puts_maybe_newline(
                        pattern_space,
                        (*sed_cmd).sw_file,
                        &mut (*sed_cmd).sw_last_char,
                        last_gets_char,
                      );
                    }
                    current_block = 17965632435239708295;
                  }
                }
                2290177392965769716 => {
                  let mut tmp: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp.is_null() {
                    *tmp = '\u{0}' as i32 as libc::c_char;
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                    *tmp = '\n' as i32 as libc::c_char;
                    current_block = 17965632435239708295;
                  } else {
                    current_block = 7330787439563864000;
                  }
                }
                3546145585875536353 => {
                  let mut tmp_0: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp_0.is_null() {
                    overlapping_strcpy(pattern_space, tmp_0.offset(1));
                    break;
                  } else {
                    current_block = 4142149688065477410;
                    break 's_54;
                  }
                }
                10499578865704240760 => {
                  free(next_line as *mut libc::c_void);
                  next_line = 0 as *mut libc::c_char;
                  current_block = 8563197331115798083;
                  break 's_54;
                }
                13665239467142187023 => {
                  let mut tmp_1: *mut libc::c_char = pattern_space;
                  pattern_space = if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void
                  } else {
                    xzalloc(1i32 as size_t)
                  } as *mut libc::c_char;
                  last_gets_char = '\n' as i32 as libc::c_char;
                  let ref mut fresh28 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh28 = tmp_1;
                  current_block = 17965632435239708295;
                }
                332836909292088582 => {
                  fprintf(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    b"%d\n\x00" as *const u8 as *const libc::c_char,
                    linenum,
                  );
                  current_block = 17965632435239708295;
                }
                17152203569385922329 => {
                  append(xstrdup((*sed_cmd).string));
                  current_block = 17965632435239708295;
                }
                14487425527653873875 => {
                  puts_maybe_newline(
                    (*sed_cmd).string,
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    &mut last_puts_char,
                    '\n' as i32 as libc::c_char,
                  );
                  current_block = 17965632435239708295;
                }
                2782169280456925021 => {
                  if (*sed_cmd).in_match() == 0 {
                    puts_maybe_newline(
                      (*sed_cmd).string,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                  }
                  current_block = 4142149688065477410;
                  break 's_54;
                }
                5622786547828025790 => {
                  puts_maybe_newline(
                    pattern_space,
                    (*sed_cmd).sw_file,
                    &mut (*sed_cmd).sw_last_char,
                    last_gets_char,
                  );
                  current_block = 17965632435239708295;
                }
                _ => {}
              }
              match current_block {
                17965632435239708295 => {}
                _ => {
                  match current_block {
                    7330787439563864000 => {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        '\n' as i32 as libc::c_char,
                      );
                      current_block = 17965632435239708295;
                    }
                    13861430101487131366 => {
                      if substituted != 0 {
                        current_block = 17965632435239708295;
                      } else {
                        current_block = 2887315643959147419;
                      }
                    }
                    _ => {}
                  }
                  match current_block {
                    17965632435239708295 => {}
                    _ => {
                      if (*sed_cmd).string.is_null() {
                        current_block = 8563197331115798083;
                        break 's_54;
                      }
                      sed_cmd = branch_to((*sed_cmd).string)
                    }
                  }
                }
              }
            }
            105 => {
              current_block = 14487425527653873875;
              match current_block {
                11227437541145425351 => {
                  let mut hold_space_size_0: libc::c_int = 2i32;
                  let mut pattern_space_size_0: libc::c_int = 0i32;
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size_0 = (hold_space_size_0 as libc::c_ulong).wrapping_add(strlen(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    )) as libc::c_int as libc::c_int
                  }
                  if !pattern_space.is_null() {
                    pattern_space_size_0 = strlen(pattern_space) as libc::c_int
                  }
                  let ref mut fresh27 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh27 = xrealloc(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                    (hold_space_size_0 + pattern_space_size_0) as size_t,
                  ) as *mut libc::c_char;
                  if hold_space_size_0 == 2i32 {
                    *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space =
                      0i32 as libc::c_char
                  }
                  strcat(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    b"\n\x00" as *const u8 as *const libc::c_char,
                  );
                  if !pattern_space.is_null() {
                    strcat(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                      pattern_space,
                    );
                  }
                  current_block = 17965632435239708295;
                }
                17838047945882858323 => {
                  free(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                  );
                  let ref mut fresh26 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh26 = xstrdup(pattern_space);
                  current_block = 17965632435239708295;
                }
                10468276026569382870 => {
                  let mut pattern_space_size: libc::c_int = 2i32;
                  let mut hold_space_size: libc::c_int = 0i32;
                  if !pattern_space.is_null() {
                    pattern_space_size = (pattern_space_size as libc::c_ulong)
                      .wrapping_add(strlen(pattern_space))
                      as libc::c_int as libc::c_int
                  }
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size =
                      strlen((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space)
                        as libc::c_int
                  }
                  pattern_space = xrealloc(
                    pattern_space as *mut libc::c_void,
                    (pattern_space_size + hold_space_size) as size_t,
                  ) as *mut libc::c_char;
                  if pattern_space_size == 2i32 {
                    *pattern_space.offset(0) = 0i32 as libc::c_char
                  }
                  strcat(pattern_space, b"\n\x00" as *const u8 as *const libc::c_char);
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    strcat(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    );
                  }
                  last_gets_char = '\n' as i32 as libc::c_char;
                  current_block = 17965632435239708295;
                }
                14127364983570718321 => {
                  free(pattern_space as *mut libc::c_void);
                  pattern_space = xstrdup(
                    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                      .hold_space
                      .is_null()
                    {
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                    } else {
                      b"\x00" as *const u8 as *const libc::c_char
                    },
                  );
                  current_block = 17965632435239708295;
                }
                1739363794695357236 => {
                  let mut i: libc::c_int = 0;
                  let mut j: libc::c_int = 0;
                  i = 0i32;
                  while *pattern_space.offset(i as isize) != 0 {
                    j = 0i32;
                    while *(*sed_cmd).string.offset(j as isize) != 0 {
                      if *pattern_space.offset(i as isize) as libc::c_int
                        == *(*sed_cmd).string.offset(j as isize) as libc::c_int
                      {
                        *pattern_space.offset(i as isize) =
                          *(*sed_cmd).string.offset((j + 1i32) as isize);
                        break;
                      } else {
                        j += 2i32
                      }
                    }
                    i += 1
                  }
                  current_block = 17965632435239708295;
                }
                17611301398573303352 => {
                  if substituted == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted = 0i32;
                    current_block = 13861430101487131366;
                  }
                }
                7297078374430259003 => {
                  let mut len: libc::c_int = 0;
                  if next_line.is_null() {
                    current_block = 8563197331115798083;
                    break 's_54;
                  } else {
                    len = strlen(pattern_space) as libc::c_int;
                    pattern_space = xrealloc(
                      pattern_space as *mut libc::c_void,
                      (len as libc::c_ulong)
                        .wrapping_add(strlen(next_line))
                        .wrapping_add(2i32 as libc::c_ulong),
                    ) as *mut libc::c_char;
                    *pattern_space.offset(len as isize) = '\n' as i32 as libc::c_char;
                    strcpy(pattern_space.offset(len as isize).offset(1), next_line);
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                1534452661327725232 => {
                  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).be_quiet == 0 {
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      last_gets_char,
                    );
                  }
                  if next_line.is_null() {
                    current_block = 4142149688065477410;
                    break 's_54;
                  } else {
                    free(pattern_space as *mut libc::c_void);
                    pattern_space = next_line;
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    substituted = 0i32;
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                726525485109251713 => {
                  let mut rfile: *mut FILE = 0 as *mut FILE;
                  rfile = fopen_for_read((*sed_cmd).string);
                  if !rfile.is_null() {
                    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
                    loop {
                      line = xmalloc_fgetline(rfile);
                      if line.is_null() {
                        break;
                      }
                      append(line);
                    }
                    fclose(rfile);
                  }
                  current_block = 17965632435239708295;
                }
                13714184482889841412 => {
                  if do_subst_command(sed_cmd, &mut pattern_space) == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted |= 1i32;
                    if (*sed_cmd).sub_p() != 0 {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        last_gets_char,
                      );
                    }
                    if !(*sed_cmd).sw_file.is_null() {
                      puts_maybe_newline(
                        pattern_space,
                        (*sed_cmd).sw_file,
                        &mut (*sed_cmd).sw_last_char,
                        last_gets_char,
                      );
                    }
                    current_block = 17965632435239708295;
                  }
                }
                2290177392965769716 => {
                  let mut tmp: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp.is_null() {
                    *tmp = '\u{0}' as i32 as libc::c_char;
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                    *tmp = '\n' as i32 as libc::c_char;
                    current_block = 17965632435239708295;
                  } else {
                    current_block = 7330787439563864000;
                  }
                }
                3546145585875536353 => {
                  let mut tmp_0: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp_0.is_null() {
                    overlapping_strcpy(pattern_space, tmp_0.offset(1));
                    break;
                  } else {
                    current_block = 4142149688065477410;
                    break 's_54;
                  }
                }
                10499578865704240760 => {
                  free(next_line as *mut libc::c_void);
                  next_line = 0 as *mut libc::c_char;
                  current_block = 8563197331115798083;
                  break 's_54;
                }
                13665239467142187023 => {
                  let mut tmp_1: *mut libc::c_char = pattern_space;
                  pattern_space = if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void
                  } else {
                    xzalloc(1i32 as size_t)
                  } as *mut libc::c_char;
                  last_gets_char = '\n' as i32 as libc::c_char;
                  let ref mut fresh28 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh28 = tmp_1;
                  current_block = 17965632435239708295;
                }
                332836909292088582 => {
                  fprintf(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    b"%d\n\x00" as *const u8 as *const libc::c_char,
                    linenum,
                  );
                  current_block = 17965632435239708295;
                }
                17152203569385922329 => {
                  append(xstrdup((*sed_cmd).string));
                  current_block = 17965632435239708295;
                }
                14487425527653873875 => {
                  puts_maybe_newline(
                    (*sed_cmd).string,
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    &mut last_puts_char,
                    '\n' as i32 as libc::c_char,
                  );
                  current_block = 17965632435239708295;
                }
                2782169280456925021 => {
                  if (*sed_cmd).in_match() == 0 {
                    puts_maybe_newline(
                      (*sed_cmd).string,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                  }
                  current_block = 4142149688065477410;
                  break 's_54;
                }
                5622786547828025790 => {
                  puts_maybe_newline(
                    pattern_space,
                    (*sed_cmd).sw_file,
                    &mut (*sed_cmd).sw_last_char,
                    last_gets_char,
                  );
                  current_block = 17965632435239708295;
                }
                _ => {}
              }
              match current_block {
                17965632435239708295 => {}
                _ => {
                  match current_block {
                    7330787439563864000 => {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        '\n' as i32 as libc::c_char,
                      );
                      current_block = 17965632435239708295;
                    }
                    13861430101487131366 => {
                      if substituted != 0 {
                        current_block = 17965632435239708295;
                      } else {
                        current_block = 2887315643959147419;
                      }
                    }
                    _ => {}
                  }
                  match current_block {
                    17965632435239708295 => {}
                    _ => {
                      if (*sed_cmd).string.is_null() {
                        current_block = 8563197331115798083;
                        break 's_54;
                      }
                      sed_cmd = branch_to((*sed_cmd).string)
                    }
                  }
                }
              }
            }
            99 => {
              current_block = 2782169280456925021;
              match current_block {
                11227437541145425351 => {
                  let mut hold_space_size_0: libc::c_int = 2i32;
                  let mut pattern_space_size_0: libc::c_int = 0i32;
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size_0 = (hold_space_size_0 as libc::c_ulong).wrapping_add(strlen(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    )) as libc::c_int as libc::c_int
                  }
                  if !pattern_space.is_null() {
                    pattern_space_size_0 = strlen(pattern_space) as libc::c_int
                  }
                  let ref mut fresh27 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh27 = xrealloc(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                    (hold_space_size_0 + pattern_space_size_0) as size_t,
                  ) as *mut libc::c_char;
                  if hold_space_size_0 == 2i32 {
                    *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space =
                      0i32 as libc::c_char
                  }
                  strcat(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    b"\n\x00" as *const u8 as *const libc::c_char,
                  );
                  if !pattern_space.is_null() {
                    strcat(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                      pattern_space,
                    );
                  }
                  current_block = 17965632435239708295;
                }
                17838047945882858323 => {
                  free(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                  );
                  let ref mut fresh26 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh26 = xstrdup(pattern_space);
                  current_block = 17965632435239708295;
                }
                10468276026569382870 => {
                  let mut pattern_space_size: libc::c_int = 2i32;
                  let mut hold_space_size: libc::c_int = 0i32;
                  if !pattern_space.is_null() {
                    pattern_space_size = (pattern_space_size as libc::c_ulong)
                      .wrapping_add(strlen(pattern_space))
                      as libc::c_int as libc::c_int
                  }
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size =
                      strlen((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space)
                        as libc::c_int
                  }
                  pattern_space = xrealloc(
                    pattern_space as *mut libc::c_void,
                    (pattern_space_size + hold_space_size) as size_t,
                  ) as *mut libc::c_char;
                  if pattern_space_size == 2i32 {
                    *pattern_space.offset(0) = 0i32 as libc::c_char
                  }
                  strcat(pattern_space, b"\n\x00" as *const u8 as *const libc::c_char);
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    strcat(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    );
                  }
                  last_gets_char = '\n' as i32 as libc::c_char;
                  current_block = 17965632435239708295;
                }
                14127364983570718321 => {
                  free(pattern_space as *mut libc::c_void);
                  pattern_space = xstrdup(
                    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                      .hold_space
                      .is_null()
                    {
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                    } else {
                      b"\x00" as *const u8 as *const libc::c_char
                    },
                  );
                  current_block = 17965632435239708295;
                }
                1739363794695357236 => {
                  let mut i: libc::c_int = 0;
                  let mut j: libc::c_int = 0;
                  i = 0i32;
                  while *pattern_space.offset(i as isize) != 0 {
                    j = 0i32;
                    while *(*sed_cmd).string.offset(j as isize) != 0 {
                      if *pattern_space.offset(i as isize) as libc::c_int
                        == *(*sed_cmd).string.offset(j as isize) as libc::c_int
                      {
                        *pattern_space.offset(i as isize) =
                          *(*sed_cmd).string.offset((j + 1i32) as isize);
                        break;
                      } else {
                        j += 2i32
                      }
                    }
                    i += 1
                  }
                  current_block = 17965632435239708295;
                }
                17611301398573303352 => {
                  if substituted == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted = 0i32;
                    current_block = 13861430101487131366;
                  }
                }
                7297078374430259003 => {
                  let mut len: libc::c_int = 0;
                  if next_line.is_null() {
                    current_block = 8563197331115798083;
                    break 's_54;
                  } else {
                    len = strlen(pattern_space) as libc::c_int;
                    pattern_space = xrealloc(
                      pattern_space as *mut libc::c_void,
                      (len as libc::c_ulong)
                        .wrapping_add(strlen(next_line))
                        .wrapping_add(2i32 as libc::c_ulong),
                    ) as *mut libc::c_char;
                    *pattern_space.offset(len as isize) = '\n' as i32 as libc::c_char;
                    strcpy(pattern_space.offset(len as isize).offset(1), next_line);
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                1534452661327725232 => {
                  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).be_quiet == 0 {
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      last_gets_char,
                    );
                  }
                  if next_line.is_null() {
                    current_block = 4142149688065477410;
                    break 's_54;
                  } else {
                    free(pattern_space as *mut libc::c_void);
                    pattern_space = next_line;
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    substituted = 0i32;
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                726525485109251713 => {
                  let mut rfile: *mut FILE = 0 as *mut FILE;
                  rfile = fopen_for_read((*sed_cmd).string);
                  if !rfile.is_null() {
                    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
                    loop {
                      line = xmalloc_fgetline(rfile);
                      if line.is_null() {
                        break;
                      }
                      append(line);
                    }
                    fclose(rfile);
                  }
                  current_block = 17965632435239708295;
                }
                13714184482889841412 => {
                  if do_subst_command(sed_cmd, &mut pattern_space) == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted |= 1i32;
                    if (*sed_cmd).sub_p() != 0 {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        last_gets_char,
                      );
                    }
                    if !(*sed_cmd).sw_file.is_null() {
                      puts_maybe_newline(
                        pattern_space,
                        (*sed_cmd).sw_file,
                        &mut (*sed_cmd).sw_last_char,
                        last_gets_char,
                      );
                    }
                    current_block = 17965632435239708295;
                  }
                }
                2290177392965769716 => {
                  let mut tmp: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp.is_null() {
                    *tmp = '\u{0}' as i32 as libc::c_char;
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                    *tmp = '\n' as i32 as libc::c_char;
                    current_block = 17965632435239708295;
                  } else {
                    current_block = 7330787439563864000;
                  }
                }
                3546145585875536353 => {
                  let mut tmp_0: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp_0.is_null() {
                    overlapping_strcpy(pattern_space, tmp_0.offset(1));
                    break;
                  } else {
                    current_block = 4142149688065477410;
                    break 's_54;
                  }
                }
                10499578865704240760 => {
                  free(next_line as *mut libc::c_void);
                  next_line = 0 as *mut libc::c_char;
                  current_block = 8563197331115798083;
                  break 's_54;
                }
                13665239467142187023 => {
                  let mut tmp_1: *mut libc::c_char = pattern_space;
                  pattern_space = if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void
                  } else {
                    xzalloc(1i32 as size_t)
                  } as *mut libc::c_char;
                  last_gets_char = '\n' as i32 as libc::c_char;
                  let ref mut fresh28 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh28 = tmp_1;
                  current_block = 17965632435239708295;
                }
                332836909292088582 => {
                  fprintf(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    b"%d\n\x00" as *const u8 as *const libc::c_char,
                    linenum,
                  );
                  current_block = 17965632435239708295;
                }
                17152203569385922329 => {
                  append(xstrdup((*sed_cmd).string));
                  current_block = 17965632435239708295;
                }
                14487425527653873875 => {
                  puts_maybe_newline(
                    (*sed_cmd).string,
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    &mut last_puts_char,
                    '\n' as i32 as libc::c_char,
                  );
                  current_block = 17965632435239708295;
                }
                2782169280456925021 => {
                  if (*sed_cmd).in_match() == 0 {
                    puts_maybe_newline(
                      (*sed_cmd).string,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                  }
                  current_block = 4142149688065477410;
                  break 's_54;
                }
                5622786547828025790 => {
                  puts_maybe_newline(
                    pattern_space,
                    (*sed_cmd).sw_file,
                    &mut (*sed_cmd).sw_last_char,
                    last_gets_char,
                  );
                  current_block = 17965632435239708295;
                }
                _ => {}
              }
              match current_block {
                17965632435239708295 => {}
                _ => {
                  match current_block {
                    7330787439563864000 => {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        '\n' as i32 as libc::c_char,
                      );
                      current_block = 17965632435239708295;
                    }
                    13861430101487131366 => {
                      if substituted != 0 {
                        current_block = 17965632435239708295;
                      } else {
                        current_block = 2887315643959147419;
                      }
                    }
                    _ => {}
                  }
                  match current_block {
                    17965632435239708295 => {}
                    _ => {
                      if (*sed_cmd).string.is_null() {
                        current_block = 8563197331115798083;
                        break 's_54;
                      }
                      sed_cmd = branch_to((*sed_cmd).string)
                    }
                  }
                }
              }
            }
            114 => {
              current_block = 726525485109251713;
              match current_block {
                11227437541145425351 => {
                  let mut hold_space_size_0: libc::c_int = 2i32;
                  let mut pattern_space_size_0: libc::c_int = 0i32;
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size_0 = (hold_space_size_0 as libc::c_ulong).wrapping_add(strlen(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    )) as libc::c_int as libc::c_int
                  }
                  if !pattern_space.is_null() {
                    pattern_space_size_0 = strlen(pattern_space) as libc::c_int
                  }
                  let ref mut fresh27 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh27 = xrealloc(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                    (hold_space_size_0 + pattern_space_size_0) as size_t,
                  ) as *mut libc::c_char;
                  if hold_space_size_0 == 2i32 {
                    *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space =
                      0i32 as libc::c_char
                  }
                  strcat(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    b"\n\x00" as *const u8 as *const libc::c_char,
                  );
                  if !pattern_space.is_null() {
                    strcat(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                      pattern_space,
                    );
                  }
                  current_block = 17965632435239708295;
                }
                17838047945882858323 => {
                  free(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                  );
                  let ref mut fresh26 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh26 = xstrdup(pattern_space);
                  current_block = 17965632435239708295;
                }
                10468276026569382870 => {
                  let mut pattern_space_size: libc::c_int = 2i32;
                  let mut hold_space_size: libc::c_int = 0i32;
                  if !pattern_space.is_null() {
                    pattern_space_size = (pattern_space_size as libc::c_ulong)
                      .wrapping_add(strlen(pattern_space))
                      as libc::c_int as libc::c_int
                  }
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size =
                      strlen((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space)
                        as libc::c_int
                  }
                  pattern_space = xrealloc(
                    pattern_space as *mut libc::c_void,
                    (pattern_space_size + hold_space_size) as size_t,
                  ) as *mut libc::c_char;
                  if pattern_space_size == 2i32 {
                    *pattern_space.offset(0) = 0i32 as libc::c_char
                  }
                  strcat(pattern_space, b"\n\x00" as *const u8 as *const libc::c_char);
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    strcat(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    );
                  }
                  last_gets_char = '\n' as i32 as libc::c_char;
                  current_block = 17965632435239708295;
                }
                14127364983570718321 => {
                  free(pattern_space as *mut libc::c_void);
                  pattern_space = xstrdup(
                    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                      .hold_space
                      .is_null()
                    {
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                    } else {
                      b"\x00" as *const u8 as *const libc::c_char
                    },
                  );
                  current_block = 17965632435239708295;
                }
                1739363794695357236 => {
                  let mut i: libc::c_int = 0;
                  let mut j: libc::c_int = 0;
                  i = 0i32;
                  while *pattern_space.offset(i as isize) != 0 {
                    j = 0i32;
                    while *(*sed_cmd).string.offset(j as isize) != 0 {
                      if *pattern_space.offset(i as isize) as libc::c_int
                        == *(*sed_cmd).string.offset(j as isize) as libc::c_int
                      {
                        *pattern_space.offset(i as isize) =
                          *(*sed_cmd).string.offset((j + 1i32) as isize);
                        break;
                      } else {
                        j += 2i32
                      }
                    }
                    i += 1
                  }
                  current_block = 17965632435239708295;
                }
                17611301398573303352 => {
                  if substituted == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted = 0i32;
                    current_block = 13861430101487131366;
                  }
                }
                7297078374430259003 => {
                  let mut len: libc::c_int = 0;
                  if next_line.is_null() {
                    current_block = 8563197331115798083;
                    break 's_54;
                  } else {
                    len = strlen(pattern_space) as libc::c_int;
                    pattern_space = xrealloc(
                      pattern_space as *mut libc::c_void,
                      (len as libc::c_ulong)
                        .wrapping_add(strlen(next_line))
                        .wrapping_add(2i32 as libc::c_ulong),
                    ) as *mut libc::c_char;
                    *pattern_space.offset(len as isize) = '\n' as i32 as libc::c_char;
                    strcpy(pattern_space.offset(len as isize).offset(1), next_line);
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                1534452661327725232 => {
                  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).be_quiet == 0 {
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      last_gets_char,
                    );
                  }
                  if next_line.is_null() {
                    current_block = 4142149688065477410;
                    break 's_54;
                  } else {
                    free(pattern_space as *mut libc::c_void);
                    pattern_space = next_line;
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    substituted = 0i32;
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                726525485109251713 => {
                  let mut rfile: *mut FILE = 0 as *mut FILE;
                  rfile = fopen_for_read((*sed_cmd).string);
                  if !rfile.is_null() {
                    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
                    loop {
                      line = xmalloc_fgetline(rfile);
                      if line.is_null() {
                        break;
                      }
                      append(line);
                    }
                    fclose(rfile);
                  }
                  current_block = 17965632435239708295;
                }
                13714184482889841412 => {
                  if do_subst_command(sed_cmd, &mut pattern_space) == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted |= 1i32;
                    if (*sed_cmd).sub_p() != 0 {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        last_gets_char,
                      );
                    }
                    if !(*sed_cmd).sw_file.is_null() {
                      puts_maybe_newline(
                        pattern_space,
                        (*sed_cmd).sw_file,
                        &mut (*sed_cmd).sw_last_char,
                        last_gets_char,
                      );
                    }
                    current_block = 17965632435239708295;
                  }
                }
                2290177392965769716 => {
                  let mut tmp: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp.is_null() {
                    *tmp = '\u{0}' as i32 as libc::c_char;
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                    *tmp = '\n' as i32 as libc::c_char;
                    current_block = 17965632435239708295;
                  } else {
                    current_block = 7330787439563864000;
                  }
                }
                3546145585875536353 => {
                  let mut tmp_0: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp_0.is_null() {
                    overlapping_strcpy(pattern_space, tmp_0.offset(1));
                    break;
                  } else {
                    current_block = 4142149688065477410;
                    break 's_54;
                  }
                }
                10499578865704240760 => {
                  free(next_line as *mut libc::c_void);
                  next_line = 0 as *mut libc::c_char;
                  current_block = 8563197331115798083;
                  break 's_54;
                }
                13665239467142187023 => {
                  let mut tmp_1: *mut libc::c_char = pattern_space;
                  pattern_space = if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void
                  } else {
                    xzalloc(1i32 as size_t)
                  } as *mut libc::c_char;
                  last_gets_char = '\n' as i32 as libc::c_char;
                  let ref mut fresh28 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh28 = tmp_1;
                  current_block = 17965632435239708295;
                }
                332836909292088582 => {
                  fprintf(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    b"%d\n\x00" as *const u8 as *const libc::c_char,
                    linenum,
                  );
                  current_block = 17965632435239708295;
                }
                17152203569385922329 => {
                  append(xstrdup((*sed_cmd).string));
                  current_block = 17965632435239708295;
                }
                14487425527653873875 => {
                  puts_maybe_newline(
                    (*sed_cmd).string,
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    &mut last_puts_char,
                    '\n' as i32 as libc::c_char,
                  );
                  current_block = 17965632435239708295;
                }
                2782169280456925021 => {
                  if (*sed_cmd).in_match() == 0 {
                    puts_maybe_newline(
                      (*sed_cmd).string,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                  }
                  current_block = 4142149688065477410;
                  break 's_54;
                }
                5622786547828025790 => {
                  puts_maybe_newline(
                    pattern_space,
                    (*sed_cmd).sw_file,
                    &mut (*sed_cmd).sw_last_char,
                    last_gets_char,
                  );
                  current_block = 17965632435239708295;
                }
                _ => {}
              }
              match current_block {
                17965632435239708295 => {}
                _ => {
                  match current_block {
                    7330787439563864000 => {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        '\n' as i32 as libc::c_char,
                      );
                      current_block = 17965632435239708295;
                    }
                    13861430101487131366 => {
                      if substituted != 0 {
                        current_block = 17965632435239708295;
                      } else {
                        current_block = 2887315643959147419;
                      }
                    }
                    _ => {}
                  }
                  match current_block {
                    17965632435239708295 => {}
                    _ => {
                      if (*sed_cmd).string.is_null() {
                        current_block = 8563197331115798083;
                        break 's_54;
                      }
                      sed_cmd = branch_to((*sed_cmd).string)
                    }
                  }
                }
              }
            }
            119 => {
              current_block = 5622786547828025790;
              match current_block {
                11227437541145425351 => {
                  let mut hold_space_size_0: libc::c_int = 2i32;
                  let mut pattern_space_size_0: libc::c_int = 0i32;
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size_0 = (hold_space_size_0 as libc::c_ulong).wrapping_add(strlen(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    )) as libc::c_int as libc::c_int
                  }
                  if !pattern_space.is_null() {
                    pattern_space_size_0 = strlen(pattern_space) as libc::c_int
                  }
                  let ref mut fresh27 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh27 = xrealloc(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                    (hold_space_size_0 + pattern_space_size_0) as size_t,
                  ) as *mut libc::c_char;
                  if hold_space_size_0 == 2i32 {
                    *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space =
                      0i32 as libc::c_char
                  }
                  strcat(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    b"\n\x00" as *const u8 as *const libc::c_char,
                  );
                  if !pattern_space.is_null() {
                    strcat(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                      pattern_space,
                    );
                  }
                  current_block = 17965632435239708295;
                }
                17838047945882858323 => {
                  free(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                  );
                  let ref mut fresh26 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh26 = xstrdup(pattern_space);
                  current_block = 17965632435239708295;
                }
                10468276026569382870 => {
                  let mut pattern_space_size: libc::c_int = 2i32;
                  let mut hold_space_size: libc::c_int = 0i32;
                  if !pattern_space.is_null() {
                    pattern_space_size = (pattern_space_size as libc::c_ulong)
                      .wrapping_add(strlen(pattern_space))
                      as libc::c_int as libc::c_int
                  }
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size =
                      strlen((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space)
                        as libc::c_int
                  }
                  pattern_space = xrealloc(
                    pattern_space as *mut libc::c_void,
                    (pattern_space_size + hold_space_size) as size_t,
                  ) as *mut libc::c_char;
                  if pattern_space_size == 2i32 {
                    *pattern_space.offset(0) = 0i32 as libc::c_char
                  }
                  strcat(pattern_space, b"\n\x00" as *const u8 as *const libc::c_char);
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    strcat(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    );
                  }
                  last_gets_char = '\n' as i32 as libc::c_char;
                  current_block = 17965632435239708295;
                }
                14127364983570718321 => {
                  free(pattern_space as *mut libc::c_void);
                  pattern_space = xstrdup(
                    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                      .hold_space
                      .is_null()
                    {
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                    } else {
                      b"\x00" as *const u8 as *const libc::c_char
                    },
                  );
                  current_block = 17965632435239708295;
                }
                1739363794695357236 => {
                  let mut i: libc::c_int = 0;
                  let mut j: libc::c_int = 0;
                  i = 0i32;
                  while *pattern_space.offset(i as isize) != 0 {
                    j = 0i32;
                    while *(*sed_cmd).string.offset(j as isize) != 0 {
                      if *pattern_space.offset(i as isize) as libc::c_int
                        == *(*sed_cmd).string.offset(j as isize) as libc::c_int
                      {
                        *pattern_space.offset(i as isize) =
                          *(*sed_cmd).string.offset((j + 1i32) as isize);
                        break;
                      } else {
                        j += 2i32
                      }
                    }
                    i += 1
                  }
                  current_block = 17965632435239708295;
                }
                17611301398573303352 => {
                  if substituted == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted = 0i32;
                    current_block = 13861430101487131366;
                  }
                }
                7297078374430259003 => {
                  let mut len: libc::c_int = 0;
                  if next_line.is_null() {
                    current_block = 8563197331115798083;
                    break 's_54;
                  } else {
                    len = strlen(pattern_space) as libc::c_int;
                    pattern_space = xrealloc(
                      pattern_space as *mut libc::c_void,
                      (len as libc::c_ulong)
                        .wrapping_add(strlen(next_line))
                        .wrapping_add(2i32 as libc::c_ulong),
                    ) as *mut libc::c_char;
                    *pattern_space.offset(len as isize) = '\n' as i32 as libc::c_char;
                    strcpy(pattern_space.offset(len as isize).offset(1), next_line);
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                1534452661327725232 => {
                  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).be_quiet == 0 {
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      last_gets_char,
                    );
                  }
                  if next_line.is_null() {
                    current_block = 4142149688065477410;
                    break 's_54;
                  } else {
                    free(pattern_space as *mut libc::c_void);
                    pattern_space = next_line;
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    substituted = 0i32;
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                726525485109251713 => {
                  let mut rfile: *mut FILE = 0 as *mut FILE;
                  rfile = fopen_for_read((*sed_cmd).string);
                  if !rfile.is_null() {
                    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
                    loop {
                      line = xmalloc_fgetline(rfile);
                      if line.is_null() {
                        break;
                      }
                      append(line);
                    }
                    fclose(rfile);
                  }
                  current_block = 17965632435239708295;
                }
                13714184482889841412 => {
                  if do_subst_command(sed_cmd, &mut pattern_space) == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted |= 1i32;
                    if (*sed_cmd).sub_p() != 0 {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        last_gets_char,
                      );
                    }
                    if !(*sed_cmd).sw_file.is_null() {
                      puts_maybe_newline(
                        pattern_space,
                        (*sed_cmd).sw_file,
                        &mut (*sed_cmd).sw_last_char,
                        last_gets_char,
                      );
                    }
                    current_block = 17965632435239708295;
                  }
                }
                2290177392965769716 => {
                  let mut tmp: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp.is_null() {
                    *tmp = '\u{0}' as i32 as libc::c_char;
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                    *tmp = '\n' as i32 as libc::c_char;
                    current_block = 17965632435239708295;
                  } else {
                    current_block = 7330787439563864000;
                  }
                }
                3546145585875536353 => {
                  let mut tmp_0: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp_0.is_null() {
                    overlapping_strcpy(pattern_space, tmp_0.offset(1));
                    break;
                  } else {
                    current_block = 4142149688065477410;
                    break 's_54;
                  }
                }
                10499578865704240760 => {
                  free(next_line as *mut libc::c_void);
                  next_line = 0 as *mut libc::c_char;
                  current_block = 8563197331115798083;
                  break 's_54;
                }
                13665239467142187023 => {
                  let mut tmp_1: *mut libc::c_char = pattern_space;
                  pattern_space = if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void
                  } else {
                    xzalloc(1i32 as size_t)
                  } as *mut libc::c_char;
                  last_gets_char = '\n' as i32 as libc::c_char;
                  let ref mut fresh28 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh28 = tmp_1;
                  current_block = 17965632435239708295;
                }
                332836909292088582 => {
                  fprintf(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    b"%d\n\x00" as *const u8 as *const libc::c_char,
                    linenum,
                  );
                  current_block = 17965632435239708295;
                }
                17152203569385922329 => {
                  append(xstrdup((*sed_cmd).string));
                  current_block = 17965632435239708295;
                }
                14487425527653873875 => {
                  puts_maybe_newline(
                    (*sed_cmd).string,
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    &mut last_puts_char,
                    '\n' as i32 as libc::c_char,
                  );
                  current_block = 17965632435239708295;
                }
                2782169280456925021 => {
                  if (*sed_cmd).in_match() == 0 {
                    puts_maybe_newline(
                      (*sed_cmd).string,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                  }
                  current_block = 4142149688065477410;
                  break 's_54;
                }
                5622786547828025790 => {
                  puts_maybe_newline(
                    pattern_space,
                    (*sed_cmd).sw_file,
                    &mut (*sed_cmd).sw_last_char,
                    last_gets_char,
                  );
                  current_block = 17965632435239708295;
                }
                _ => {}
              }
              match current_block {
                17965632435239708295 => {}
                _ => {
                  match current_block {
                    7330787439563864000 => {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        '\n' as i32 as libc::c_char,
                      );
                      current_block = 17965632435239708295;
                    }
                    13861430101487131366 => {
                      if substituted != 0 {
                        current_block = 17965632435239708295;
                      } else {
                        current_block = 2887315643959147419;
                      }
                    }
                    _ => {}
                  }
                  match current_block {
                    17965632435239708295 => {}
                    _ => {
                      if (*sed_cmd).string.is_null() {
                        current_block = 8563197331115798083;
                        break 's_54;
                      }
                      sed_cmd = branch_to((*sed_cmd).string)
                    }
                  }
                }
              }
            }
            110 => {
              current_block = 1534452661327725232;
              match current_block {
                11227437541145425351 => {
                  let mut hold_space_size_0: libc::c_int = 2i32;
                  let mut pattern_space_size_0: libc::c_int = 0i32;
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size_0 = (hold_space_size_0 as libc::c_ulong).wrapping_add(strlen(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    )) as libc::c_int as libc::c_int
                  }
                  if !pattern_space.is_null() {
                    pattern_space_size_0 = strlen(pattern_space) as libc::c_int
                  }
                  let ref mut fresh27 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh27 = xrealloc(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                    (hold_space_size_0 + pattern_space_size_0) as size_t,
                  ) as *mut libc::c_char;
                  if hold_space_size_0 == 2i32 {
                    *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space =
                      0i32 as libc::c_char
                  }
                  strcat(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    b"\n\x00" as *const u8 as *const libc::c_char,
                  );
                  if !pattern_space.is_null() {
                    strcat(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                      pattern_space,
                    );
                  }
                  current_block = 17965632435239708295;
                }
                17838047945882858323 => {
                  free(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                  );
                  let ref mut fresh26 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh26 = xstrdup(pattern_space);
                  current_block = 17965632435239708295;
                }
                10468276026569382870 => {
                  let mut pattern_space_size: libc::c_int = 2i32;
                  let mut hold_space_size: libc::c_int = 0i32;
                  if !pattern_space.is_null() {
                    pattern_space_size = (pattern_space_size as libc::c_ulong)
                      .wrapping_add(strlen(pattern_space))
                      as libc::c_int as libc::c_int
                  }
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size =
                      strlen((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space)
                        as libc::c_int
                  }
                  pattern_space = xrealloc(
                    pattern_space as *mut libc::c_void,
                    (pattern_space_size + hold_space_size) as size_t,
                  ) as *mut libc::c_char;
                  if pattern_space_size == 2i32 {
                    *pattern_space.offset(0) = 0i32 as libc::c_char
                  }
                  strcat(pattern_space, b"\n\x00" as *const u8 as *const libc::c_char);
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    strcat(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    );
                  }
                  last_gets_char = '\n' as i32 as libc::c_char;
                  current_block = 17965632435239708295;
                }
                14127364983570718321 => {
                  free(pattern_space as *mut libc::c_void);
                  pattern_space = xstrdup(
                    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                      .hold_space
                      .is_null()
                    {
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                    } else {
                      b"\x00" as *const u8 as *const libc::c_char
                    },
                  );
                  current_block = 17965632435239708295;
                }
                1739363794695357236 => {
                  let mut i: libc::c_int = 0;
                  let mut j: libc::c_int = 0;
                  i = 0i32;
                  while *pattern_space.offset(i as isize) != 0 {
                    j = 0i32;
                    while *(*sed_cmd).string.offset(j as isize) != 0 {
                      if *pattern_space.offset(i as isize) as libc::c_int
                        == *(*sed_cmd).string.offset(j as isize) as libc::c_int
                      {
                        *pattern_space.offset(i as isize) =
                          *(*sed_cmd).string.offset((j + 1i32) as isize);
                        break;
                      } else {
                        j += 2i32
                      }
                    }
                    i += 1
                  }
                  current_block = 17965632435239708295;
                }
                17611301398573303352 => {
                  if substituted == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted = 0i32;
                    current_block = 13861430101487131366;
                  }
                }
                7297078374430259003 => {
                  let mut len: libc::c_int = 0;
                  if next_line.is_null() {
                    current_block = 8563197331115798083;
                    break 's_54;
                  } else {
                    len = strlen(pattern_space) as libc::c_int;
                    pattern_space = xrealloc(
                      pattern_space as *mut libc::c_void,
                      (len as libc::c_ulong)
                        .wrapping_add(strlen(next_line))
                        .wrapping_add(2i32 as libc::c_ulong),
                    ) as *mut libc::c_char;
                    *pattern_space.offset(len as isize) = '\n' as i32 as libc::c_char;
                    strcpy(pattern_space.offset(len as isize).offset(1), next_line);
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                1534452661327725232 => {
                  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).be_quiet == 0 {
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      last_gets_char,
                    );
                  }
                  if next_line.is_null() {
                    current_block = 4142149688065477410;
                    break 's_54;
                  } else {
                    free(pattern_space as *mut libc::c_void);
                    pattern_space = next_line;
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    substituted = 0i32;
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                726525485109251713 => {
                  let mut rfile: *mut FILE = 0 as *mut FILE;
                  rfile = fopen_for_read((*sed_cmd).string);
                  if !rfile.is_null() {
                    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
                    loop {
                      line = xmalloc_fgetline(rfile);
                      if line.is_null() {
                        break;
                      }
                      append(line);
                    }
                    fclose(rfile);
                  }
                  current_block = 17965632435239708295;
                }
                13714184482889841412 => {
                  if do_subst_command(sed_cmd, &mut pattern_space) == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted |= 1i32;
                    if (*sed_cmd).sub_p() != 0 {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        last_gets_char,
                      );
                    }
                    if !(*sed_cmd).sw_file.is_null() {
                      puts_maybe_newline(
                        pattern_space,
                        (*sed_cmd).sw_file,
                        &mut (*sed_cmd).sw_last_char,
                        last_gets_char,
                      );
                    }
                    current_block = 17965632435239708295;
                  }
                }
                2290177392965769716 => {
                  let mut tmp: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp.is_null() {
                    *tmp = '\u{0}' as i32 as libc::c_char;
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                    *tmp = '\n' as i32 as libc::c_char;
                    current_block = 17965632435239708295;
                  } else {
                    current_block = 7330787439563864000;
                  }
                }
                3546145585875536353 => {
                  let mut tmp_0: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp_0.is_null() {
                    overlapping_strcpy(pattern_space, tmp_0.offset(1));
                    break;
                  } else {
                    current_block = 4142149688065477410;
                    break 's_54;
                  }
                }
                10499578865704240760 => {
                  free(next_line as *mut libc::c_void);
                  next_line = 0 as *mut libc::c_char;
                  current_block = 8563197331115798083;
                  break 's_54;
                }
                13665239467142187023 => {
                  let mut tmp_1: *mut libc::c_char = pattern_space;
                  pattern_space = if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void
                  } else {
                    xzalloc(1i32 as size_t)
                  } as *mut libc::c_char;
                  last_gets_char = '\n' as i32 as libc::c_char;
                  let ref mut fresh28 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh28 = tmp_1;
                  current_block = 17965632435239708295;
                }
                332836909292088582 => {
                  fprintf(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    b"%d\n\x00" as *const u8 as *const libc::c_char,
                    linenum,
                  );
                  current_block = 17965632435239708295;
                }
                17152203569385922329 => {
                  append(xstrdup((*sed_cmd).string));
                  current_block = 17965632435239708295;
                }
                14487425527653873875 => {
                  puts_maybe_newline(
                    (*sed_cmd).string,
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    &mut last_puts_char,
                    '\n' as i32 as libc::c_char,
                  );
                  current_block = 17965632435239708295;
                }
                2782169280456925021 => {
                  if (*sed_cmd).in_match() == 0 {
                    puts_maybe_newline(
                      (*sed_cmd).string,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                  }
                  current_block = 4142149688065477410;
                  break 's_54;
                }
                5622786547828025790 => {
                  puts_maybe_newline(
                    pattern_space,
                    (*sed_cmd).sw_file,
                    &mut (*sed_cmd).sw_last_char,
                    last_gets_char,
                  );
                  current_block = 17965632435239708295;
                }
                _ => {}
              }
              match current_block {
                17965632435239708295 => {}
                _ => {
                  match current_block {
                    7330787439563864000 => {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        '\n' as i32 as libc::c_char,
                      );
                      current_block = 17965632435239708295;
                    }
                    13861430101487131366 => {
                      if substituted != 0 {
                        current_block = 17965632435239708295;
                      } else {
                        current_block = 2887315643959147419;
                      }
                    }
                    _ => {}
                  }
                  match current_block {
                    17965632435239708295 => {}
                    _ => {
                      if (*sed_cmd).string.is_null() {
                        current_block = 8563197331115798083;
                        break 's_54;
                      }
                      sed_cmd = branch_to((*sed_cmd).string)
                    }
                  }
                }
              }
            }
            113 => {
              current_block = 10499578865704240760;
              match current_block {
                11227437541145425351 => {
                  let mut hold_space_size_0: libc::c_int = 2i32;
                  let mut pattern_space_size_0: libc::c_int = 0i32;
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size_0 = (hold_space_size_0 as libc::c_ulong).wrapping_add(strlen(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    )) as libc::c_int as libc::c_int
                  }
                  if !pattern_space.is_null() {
                    pattern_space_size_0 = strlen(pattern_space) as libc::c_int
                  }
                  let ref mut fresh27 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh27 = xrealloc(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                    (hold_space_size_0 + pattern_space_size_0) as size_t,
                  ) as *mut libc::c_char;
                  if hold_space_size_0 == 2i32 {
                    *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space =
                      0i32 as libc::c_char
                  }
                  strcat(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    b"\n\x00" as *const u8 as *const libc::c_char,
                  );
                  if !pattern_space.is_null() {
                    strcat(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                      pattern_space,
                    );
                  }
                  current_block = 17965632435239708295;
                }
                17838047945882858323 => {
                  free(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                  );
                  let ref mut fresh26 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh26 = xstrdup(pattern_space);
                  current_block = 17965632435239708295;
                }
                10468276026569382870 => {
                  let mut pattern_space_size: libc::c_int = 2i32;
                  let mut hold_space_size: libc::c_int = 0i32;
                  if !pattern_space.is_null() {
                    pattern_space_size = (pattern_space_size as libc::c_ulong)
                      .wrapping_add(strlen(pattern_space))
                      as libc::c_int as libc::c_int
                  }
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size =
                      strlen((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space)
                        as libc::c_int
                  }
                  pattern_space = xrealloc(
                    pattern_space as *mut libc::c_void,
                    (pattern_space_size + hold_space_size) as size_t,
                  ) as *mut libc::c_char;
                  if pattern_space_size == 2i32 {
                    *pattern_space.offset(0) = 0i32 as libc::c_char
                  }
                  strcat(pattern_space, b"\n\x00" as *const u8 as *const libc::c_char);
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    strcat(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    );
                  }
                  last_gets_char = '\n' as i32 as libc::c_char;
                  current_block = 17965632435239708295;
                }
                14127364983570718321 => {
                  free(pattern_space as *mut libc::c_void);
                  pattern_space = xstrdup(
                    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                      .hold_space
                      .is_null()
                    {
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                    } else {
                      b"\x00" as *const u8 as *const libc::c_char
                    },
                  );
                  current_block = 17965632435239708295;
                }
                1739363794695357236 => {
                  let mut i: libc::c_int = 0;
                  let mut j: libc::c_int = 0;
                  i = 0i32;
                  while *pattern_space.offset(i as isize) != 0 {
                    j = 0i32;
                    while *(*sed_cmd).string.offset(j as isize) != 0 {
                      if *pattern_space.offset(i as isize) as libc::c_int
                        == *(*sed_cmd).string.offset(j as isize) as libc::c_int
                      {
                        *pattern_space.offset(i as isize) =
                          *(*sed_cmd).string.offset((j + 1i32) as isize);
                        break;
                      } else {
                        j += 2i32
                      }
                    }
                    i += 1
                  }
                  current_block = 17965632435239708295;
                }
                17611301398573303352 => {
                  if substituted == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted = 0i32;
                    current_block = 13861430101487131366;
                  }
                }
                7297078374430259003 => {
                  let mut len: libc::c_int = 0;
                  if next_line.is_null() {
                    current_block = 8563197331115798083;
                    break 's_54;
                  } else {
                    len = strlen(pattern_space) as libc::c_int;
                    pattern_space = xrealloc(
                      pattern_space as *mut libc::c_void,
                      (len as libc::c_ulong)
                        .wrapping_add(strlen(next_line))
                        .wrapping_add(2i32 as libc::c_ulong),
                    ) as *mut libc::c_char;
                    *pattern_space.offset(len as isize) = '\n' as i32 as libc::c_char;
                    strcpy(pattern_space.offset(len as isize).offset(1), next_line);
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                1534452661327725232 => {
                  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).be_quiet == 0 {
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      last_gets_char,
                    );
                  }
                  if next_line.is_null() {
                    current_block = 4142149688065477410;
                    break 's_54;
                  } else {
                    free(pattern_space as *mut libc::c_void);
                    pattern_space = next_line;
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    substituted = 0i32;
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                726525485109251713 => {
                  let mut rfile: *mut FILE = 0 as *mut FILE;
                  rfile = fopen_for_read((*sed_cmd).string);
                  if !rfile.is_null() {
                    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
                    loop {
                      line = xmalloc_fgetline(rfile);
                      if line.is_null() {
                        break;
                      }
                      append(line);
                    }
                    fclose(rfile);
                  }
                  current_block = 17965632435239708295;
                }
                13714184482889841412 => {
                  if do_subst_command(sed_cmd, &mut pattern_space) == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted |= 1i32;
                    if (*sed_cmd).sub_p() != 0 {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        last_gets_char,
                      );
                    }
                    if !(*sed_cmd).sw_file.is_null() {
                      puts_maybe_newline(
                        pattern_space,
                        (*sed_cmd).sw_file,
                        &mut (*sed_cmd).sw_last_char,
                        last_gets_char,
                      );
                    }
                    current_block = 17965632435239708295;
                  }
                }
                2290177392965769716 => {
                  let mut tmp: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp.is_null() {
                    *tmp = '\u{0}' as i32 as libc::c_char;
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                    *tmp = '\n' as i32 as libc::c_char;
                    current_block = 17965632435239708295;
                  } else {
                    current_block = 7330787439563864000;
                  }
                }
                3546145585875536353 => {
                  let mut tmp_0: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp_0.is_null() {
                    overlapping_strcpy(pattern_space, tmp_0.offset(1));
                    break;
                  } else {
                    current_block = 4142149688065477410;
                    break 's_54;
                  }
                }
                10499578865704240760 => {
                  free(next_line as *mut libc::c_void);
                  next_line = 0 as *mut libc::c_char;
                  current_block = 8563197331115798083;
                  break 's_54;
                }
                13665239467142187023 => {
                  let mut tmp_1: *mut libc::c_char = pattern_space;
                  pattern_space = if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void
                  } else {
                    xzalloc(1i32 as size_t)
                  } as *mut libc::c_char;
                  last_gets_char = '\n' as i32 as libc::c_char;
                  let ref mut fresh28 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh28 = tmp_1;
                  current_block = 17965632435239708295;
                }
                332836909292088582 => {
                  fprintf(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    b"%d\n\x00" as *const u8 as *const libc::c_char,
                    linenum,
                  );
                  current_block = 17965632435239708295;
                }
                17152203569385922329 => {
                  append(xstrdup((*sed_cmd).string));
                  current_block = 17965632435239708295;
                }
                14487425527653873875 => {
                  puts_maybe_newline(
                    (*sed_cmd).string,
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    &mut last_puts_char,
                    '\n' as i32 as libc::c_char,
                  );
                  current_block = 17965632435239708295;
                }
                2782169280456925021 => {
                  if (*sed_cmd).in_match() == 0 {
                    puts_maybe_newline(
                      (*sed_cmd).string,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                  }
                  current_block = 4142149688065477410;
                  break 's_54;
                }
                5622786547828025790 => {
                  puts_maybe_newline(
                    pattern_space,
                    (*sed_cmd).sw_file,
                    &mut (*sed_cmd).sw_last_char,
                    last_gets_char,
                  );
                  current_block = 17965632435239708295;
                }
                _ => {}
              }
              match current_block {
                17965632435239708295 => {}
                _ => {
                  match current_block {
                    7330787439563864000 => {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        '\n' as i32 as libc::c_char,
                      );
                      current_block = 17965632435239708295;
                    }
                    13861430101487131366 => {
                      if substituted != 0 {
                        current_block = 17965632435239708295;
                      } else {
                        current_block = 2887315643959147419;
                      }
                    }
                    _ => {}
                  }
                  match current_block {
                    17965632435239708295 => {}
                    _ => {
                      if (*sed_cmd).string.is_null() {
                        current_block = 8563197331115798083;
                        break 's_54;
                      }
                      sed_cmd = branch_to((*sed_cmd).string)
                    }
                  }
                }
              }
            }
            78 => {
              current_block = 7297078374430259003;
              match current_block {
                11227437541145425351 => {
                  let mut hold_space_size_0: libc::c_int = 2i32;
                  let mut pattern_space_size_0: libc::c_int = 0i32;
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size_0 = (hold_space_size_0 as libc::c_ulong).wrapping_add(strlen(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    )) as libc::c_int as libc::c_int
                  }
                  if !pattern_space.is_null() {
                    pattern_space_size_0 = strlen(pattern_space) as libc::c_int
                  }
                  let ref mut fresh27 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh27 = xrealloc(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                    (hold_space_size_0 + pattern_space_size_0) as size_t,
                  ) as *mut libc::c_char;
                  if hold_space_size_0 == 2i32 {
                    *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space =
                      0i32 as libc::c_char
                  }
                  strcat(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    b"\n\x00" as *const u8 as *const libc::c_char,
                  );
                  if !pattern_space.is_null() {
                    strcat(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                      pattern_space,
                    );
                  }
                  current_block = 17965632435239708295;
                }
                17838047945882858323 => {
                  free(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                  );
                  let ref mut fresh26 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh26 = xstrdup(pattern_space);
                  current_block = 17965632435239708295;
                }
                10468276026569382870 => {
                  let mut pattern_space_size: libc::c_int = 2i32;
                  let mut hold_space_size: libc::c_int = 0i32;
                  if !pattern_space.is_null() {
                    pattern_space_size = (pattern_space_size as libc::c_ulong)
                      .wrapping_add(strlen(pattern_space))
                      as libc::c_int as libc::c_int
                  }
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size =
                      strlen((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space)
                        as libc::c_int
                  }
                  pattern_space = xrealloc(
                    pattern_space as *mut libc::c_void,
                    (pattern_space_size + hold_space_size) as size_t,
                  ) as *mut libc::c_char;
                  if pattern_space_size == 2i32 {
                    *pattern_space.offset(0) = 0i32 as libc::c_char
                  }
                  strcat(pattern_space, b"\n\x00" as *const u8 as *const libc::c_char);
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    strcat(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    );
                  }
                  last_gets_char = '\n' as i32 as libc::c_char;
                  current_block = 17965632435239708295;
                }
                14127364983570718321 => {
                  free(pattern_space as *mut libc::c_void);
                  pattern_space = xstrdup(
                    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                      .hold_space
                      .is_null()
                    {
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                    } else {
                      b"\x00" as *const u8 as *const libc::c_char
                    },
                  );
                  current_block = 17965632435239708295;
                }
                1739363794695357236 => {
                  let mut i: libc::c_int = 0;
                  let mut j: libc::c_int = 0;
                  i = 0i32;
                  while *pattern_space.offset(i as isize) != 0 {
                    j = 0i32;
                    while *(*sed_cmd).string.offset(j as isize) != 0 {
                      if *pattern_space.offset(i as isize) as libc::c_int
                        == *(*sed_cmd).string.offset(j as isize) as libc::c_int
                      {
                        *pattern_space.offset(i as isize) =
                          *(*sed_cmd).string.offset((j + 1i32) as isize);
                        break;
                      } else {
                        j += 2i32
                      }
                    }
                    i += 1
                  }
                  current_block = 17965632435239708295;
                }
                17611301398573303352 => {
                  if substituted == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted = 0i32;
                    current_block = 13861430101487131366;
                  }
                }
                7297078374430259003 => {
                  let mut len: libc::c_int = 0;
                  if next_line.is_null() {
                    current_block = 8563197331115798083;
                    break 's_54;
                  } else {
                    len = strlen(pattern_space) as libc::c_int;
                    pattern_space = xrealloc(
                      pattern_space as *mut libc::c_void,
                      (len as libc::c_ulong)
                        .wrapping_add(strlen(next_line))
                        .wrapping_add(2i32 as libc::c_ulong),
                    ) as *mut libc::c_char;
                    *pattern_space.offset(len as isize) = '\n' as i32 as libc::c_char;
                    strcpy(pattern_space.offset(len as isize).offset(1), next_line);
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                1534452661327725232 => {
                  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).be_quiet == 0 {
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      last_gets_char,
                    );
                  }
                  if next_line.is_null() {
                    current_block = 4142149688065477410;
                    break 's_54;
                  } else {
                    free(pattern_space as *mut libc::c_void);
                    pattern_space = next_line;
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    substituted = 0i32;
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                726525485109251713 => {
                  let mut rfile: *mut FILE = 0 as *mut FILE;
                  rfile = fopen_for_read((*sed_cmd).string);
                  if !rfile.is_null() {
                    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
                    loop {
                      line = xmalloc_fgetline(rfile);
                      if line.is_null() {
                        break;
                      }
                      append(line);
                    }
                    fclose(rfile);
                  }
                  current_block = 17965632435239708295;
                }
                13714184482889841412 => {
                  if do_subst_command(sed_cmd, &mut pattern_space) == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted |= 1i32;
                    if (*sed_cmd).sub_p() != 0 {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        last_gets_char,
                      );
                    }
                    if !(*sed_cmd).sw_file.is_null() {
                      puts_maybe_newline(
                        pattern_space,
                        (*sed_cmd).sw_file,
                        &mut (*sed_cmd).sw_last_char,
                        last_gets_char,
                      );
                    }
                    current_block = 17965632435239708295;
                  }
                }
                2290177392965769716 => {
                  let mut tmp: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp.is_null() {
                    *tmp = '\u{0}' as i32 as libc::c_char;
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                    *tmp = '\n' as i32 as libc::c_char;
                    current_block = 17965632435239708295;
                  } else {
                    current_block = 7330787439563864000;
                  }
                }
                3546145585875536353 => {
                  let mut tmp_0: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp_0.is_null() {
                    overlapping_strcpy(pattern_space, tmp_0.offset(1));
                    break;
                  } else {
                    current_block = 4142149688065477410;
                    break 's_54;
                  }
                }
                10499578865704240760 => {
                  free(next_line as *mut libc::c_void);
                  next_line = 0 as *mut libc::c_char;
                  current_block = 8563197331115798083;
                  break 's_54;
                }
                13665239467142187023 => {
                  let mut tmp_1: *mut libc::c_char = pattern_space;
                  pattern_space = if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void
                  } else {
                    xzalloc(1i32 as size_t)
                  } as *mut libc::c_char;
                  last_gets_char = '\n' as i32 as libc::c_char;
                  let ref mut fresh28 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh28 = tmp_1;
                  current_block = 17965632435239708295;
                }
                332836909292088582 => {
                  fprintf(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    b"%d\n\x00" as *const u8 as *const libc::c_char,
                    linenum,
                  );
                  current_block = 17965632435239708295;
                }
                17152203569385922329 => {
                  append(xstrdup((*sed_cmd).string));
                  current_block = 17965632435239708295;
                }
                14487425527653873875 => {
                  puts_maybe_newline(
                    (*sed_cmd).string,
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    &mut last_puts_char,
                    '\n' as i32 as libc::c_char,
                  );
                  current_block = 17965632435239708295;
                }
                2782169280456925021 => {
                  if (*sed_cmd).in_match() == 0 {
                    puts_maybe_newline(
                      (*sed_cmd).string,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                  }
                  current_block = 4142149688065477410;
                  break 's_54;
                }
                5622786547828025790 => {
                  puts_maybe_newline(
                    pattern_space,
                    (*sed_cmd).sw_file,
                    &mut (*sed_cmd).sw_last_char,
                    last_gets_char,
                  );
                  current_block = 17965632435239708295;
                }
                _ => {}
              }
              match current_block {
                17965632435239708295 => {}
                _ => {
                  match current_block {
                    7330787439563864000 => {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        '\n' as i32 as libc::c_char,
                      );
                      current_block = 17965632435239708295;
                    }
                    13861430101487131366 => {
                      if substituted != 0 {
                        current_block = 17965632435239708295;
                      } else {
                        current_block = 2887315643959147419;
                      }
                    }
                    _ => {}
                  }
                  match current_block {
                    17965632435239708295 => {}
                    _ => {
                      if (*sed_cmd).string.is_null() {
                        current_block = 8563197331115798083;
                        break 's_54;
                      }
                      sed_cmd = branch_to((*sed_cmd).string)
                    }
                  }
                }
              }
            }
            116 => {
              current_block = 17611301398573303352;
              match current_block {
                11227437541145425351 => {
                  let mut hold_space_size_0: libc::c_int = 2i32;
                  let mut pattern_space_size_0: libc::c_int = 0i32;
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size_0 = (hold_space_size_0 as libc::c_ulong).wrapping_add(strlen(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    )) as libc::c_int as libc::c_int
                  }
                  if !pattern_space.is_null() {
                    pattern_space_size_0 = strlen(pattern_space) as libc::c_int
                  }
                  let ref mut fresh27 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh27 = xrealloc(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                    (hold_space_size_0 + pattern_space_size_0) as size_t,
                  ) as *mut libc::c_char;
                  if hold_space_size_0 == 2i32 {
                    *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space =
                      0i32 as libc::c_char
                  }
                  strcat(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    b"\n\x00" as *const u8 as *const libc::c_char,
                  );
                  if !pattern_space.is_null() {
                    strcat(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                      pattern_space,
                    );
                  }
                  current_block = 17965632435239708295;
                }
                17838047945882858323 => {
                  free(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                  );
                  let ref mut fresh26 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh26 = xstrdup(pattern_space);
                  current_block = 17965632435239708295;
                }
                10468276026569382870 => {
                  let mut pattern_space_size: libc::c_int = 2i32;
                  let mut hold_space_size: libc::c_int = 0i32;
                  if !pattern_space.is_null() {
                    pattern_space_size = (pattern_space_size as libc::c_ulong)
                      .wrapping_add(strlen(pattern_space))
                      as libc::c_int as libc::c_int
                  }
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size =
                      strlen((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space)
                        as libc::c_int
                  }
                  pattern_space = xrealloc(
                    pattern_space as *mut libc::c_void,
                    (pattern_space_size + hold_space_size) as size_t,
                  ) as *mut libc::c_char;
                  if pattern_space_size == 2i32 {
                    *pattern_space.offset(0) = 0i32 as libc::c_char
                  }
                  strcat(pattern_space, b"\n\x00" as *const u8 as *const libc::c_char);
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    strcat(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    );
                  }
                  last_gets_char = '\n' as i32 as libc::c_char;
                  current_block = 17965632435239708295;
                }
                14127364983570718321 => {
                  free(pattern_space as *mut libc::c_void);
                  pattern_space = xstrdup(
                    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                      .hold_space
                      .is_null()
                    {
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                    } else {
                      b"\x00" as *const u8 as *const libc::c_char
                    },
                  );
                  current_block = 17965632435239708295;
                }
                1739363794695357236 => {
                  let mut i: libc::c_int = 0;
                  let mut j: libc::c_int = 0;
                  i = 0i32;
                  while *pattern_space.offset(i as isize) != 0 {
                    j = 0i32;
                    while *(*sed_cmd).string.offset(j as isize) != 0 {
                      if *pattern_space.offset(i as isize) as libc::c_int
                        == *(*sed_cmd).string.offset(j as isize) as libc::c_int
                      {
                        *pattern_space.offset(i as isize) =
                          *(*sed_cmd).string.offset((j + 1i32) as isize);
                        break;
                      } else {
                        j += 2i32
                      }
                    }
                    i += 1
                  }
                  current_block = 17965632435239708295;
                }
                17611301398573303352 => {
                  if substituted == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted = 0i32;
                    current_block = 13861430101487131366;
                  }
                }
                7297078374430259003 => {
                  let mut len: libc::c_int = 0;
                  if next_line.is_null() {
                    current_block = 8563197331115798083;
                    break 's_54;
                  } else {
                    len = strlen(pattern_space) as libc::c_int;
                    pattern_space = xrealloc(
                      pattern_space as *mut libc::c_void,
                      (len as libc::c_ulong)
                        .wrapping_add(strlen(next_line))
                        .wrapping_add(2i32 as libc::c_ulong),
                    ) as *mut libc::c_char;
                    *pattern_space.offset(len as isize) = '\n' as i32 as libc::c_char;
                    strcpy(pattern_space.offset(len as isize).offset(1), next_line);
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                1534452661327725232 => {
                  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).be_quiet == 0 {
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      last_gets_char,
                    );
                  }
                  if next_line.is_null() {
                    current_block = 4142149688065477410;
                    break 's_54;
                  } else {
                    free(pattern_space as *mut libc::c_void);
                    pattern_space = next_line;
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    substituted = 0i32;
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                726525485109251713 => {
                  let mut rfile: *mut FILE = 0 as *mut FILE;
                  rfile = fopen_for_read((*sed_cmd).string);
                  if !rfile.is_null() {
                    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
                    loop {
                      line = xmalloc_fgetline(rfile);
                      if line.is_null() {
                        break;
                      }
                      append(line);
                    }
                    fclose(rfile);
                  }
                  current_block = 17965632435239708295;
                }
                13714184482889841412 => {
                  if do_subst_command(sed_cmd, &mut pattern_space) == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted |= 1i32;
                    if (*sed_cmd).sub_p() != 0 {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        last_gets_char,
                      );
                    }
                    if !(*sed_cmd).sw_file.is_null() {
                      puts_maybe_newline(
                        pattern_space,
                        (*sed_cmd).sw_file,
                        &mut (*sed_cmd).sw_last_char,
                        last_gets_char,
                      );
                    }
                    current_block = 17965632435239708295;
                  }
                }
                2290177392965769716 => {
                  let mut tmp: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp.is_null() {
                    *tmp = '\u{0}' as i32 as libc::c_char;
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                    *tmp = '\n' as i32 as libc::c_char;
                    current_block = 17965632435239708295;
                  } else {
                    current_block = 7330787439563864000;
                  }
                }
                3546145585875536353 => {
                  let mut tmp_0: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp_0.is_null() {
                    overlapping_strcpy(pattern_space, tmp_0.offset(1));
                    break;
                  } else {
                    current_block = 4142149688065477410;
                    break 's_54;
                  }
                }
                10499578865704240760 => {
                  free(next_line as *mut libc::c_void);
                  next_line = 0 as *mut libc::c_char;
                  current_block = 8563197331115798083;
                  break 's_54;
                }
                13665239467142187023 => {
                  let mut tmp_1: *mut libc::c_char = pattern_space;
                  pattern_space = if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void
                  } else {
                    xzalloc(1i32 as size_t)
                  } as *mut libc::c_char;
                  last_gets_char = '\n' as i32 as libc::c_char;
                  let ref mut fresh28 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh28 = tmp_1;
                  current_block = 17965632435239708295;
                }
                332836909292088582 => {
                  fprintf(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    b"%d\n\x00" as *const u8 as *const libc::c_char,
                    linenum,
                  );
                  current_block = 17965632435239708295;
                }
                17152203569385922329 => {
                  append(xstrdup((*sed_cmd).string));
                  current_block = 17965632435239708295;
                }
                14487425527653873875 => {
                  puts_maybe_newline(
                    (*sed_cmd).string,
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    &mut last_puts_char,
                    '\n' as i32 as libc::c_char,
                  );
                  current_block = 17965632435239708295;
                }
                2782169280456925021 => {
                  if (*sed_cmd).in_match() == 0 {
                    puts_maybe_newline(
                      (*sed_cmd).string,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                  }
                  current_block = 4142149688065477410;
                  break 's_54;
                }
                5622786547828025790 => {
                  puts_maybe_newline(
                    pattern_space,
                    (*sed_cmd).sw_file,
                    &mut (*sed_cmd).sw_last_char,
                    last_gets_char,
                  );
                  current_block = 17965632435239708295;
                }
                _ => {}
              }
              match current_block {
                17965632435239708295 => {}
                _ => {
                  match current_block {
                    7330787439563864000 => {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        '\n' as i32 as libc::c_char,
                      );
                      current_block = 17965632435239708295;
                    }
                    13861430101487131366 => {
                      if substituted != 0 {
                        current_block = 17965632435239708295;
                      } else {
                        current_block = 2887315643959147419;
                      }
                    }
                    _ => {}
                  }
                  match current_block {
                    17965632435239708295 => {}
                    _ => {
                      if (*sed_cmd).string.is_null() {
                        current_block = 8563197331115798083;
                        break 's_54;
                      }
                      sed_cmd = branch_to((*sed_cmd).string)
                    }
                  }
                }
              }
            }
            84 => {
              current_block = 13861430101487131366;
              match current_block {
                11227437541145425351 => {
                  let mut hold_space_size_0: libc::c_int = 2i32;
                  let mut pattern_space_size_0: libc::c_int = 0i32;
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size_0 = (hold_space_size_0 as libc::c_ulong).wrapping_add(strlen(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    )) as libc::c_int as libc::c_int
                  }
                  if !pattern_space.is_null() {
                    pattern_space_size_0 = strlen(pattern_space) as libc::c_int
                  }
                  let ref mut fresh27 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh27 = xrealloc(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                    (hold_space_size_0 + pattern_space_size_0) as size_t,
                  ) as *mut libc::c_char;
                  if hold_space_size_0 == 2i32 {
                    *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space =
                      0i32 as libc::c_char
                  }
                  strcat(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    b"\n\x00" as *const u8 as *const libc::c_char,
                  );
                  if !pattern_space.is_null() {
                    strcat(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                      pattern_space,
                    );
                  }
                  current_block = 17965632435239708295;
                }
                17838047945882858323 => {
                  free(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                  );
                  let ref mut fresh26 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh26 = xstrdup(pattern_space);
                  current_block = 17965632435239708295;
                }
                10468276026569382870 => {
                  let mut pattern_space_size: libc::c_int = 2i32;
                  let mut hold_space_size: libc::c_int = 0i32;
                  if !pattern_space.is_null() {
                    pattern_space_size = (pattern_space_size as libc::c_ulong)
                      .wrapping_add(strlen(pattern_space))
                      as libc::c_int as libc::c_int
                  }
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size =
                      strlen((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space)
                        as libc::c_int
                  }
                  pattern_space = xrealloc(
                    pattern_space as *mut libc::c_void,
                    (pattern_space_size + hold_space_size) as size_t,
                  ) as *mut libc::c_char;
                  if pattern_space_size == 2i32 {
                    *pattern_space.offset(0) = 0i32 as libc::c_char
                  }
                  strcat(pattern_space, b"\n\x00" as *const u8 as *const libc::c_char);
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    strcat(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    );
                  }
                  last_gets_char = '\n' as i32 as libc::c_char;
                  current_block = 17965632435239708295;
                }
                14127364983570718321 => {
                  free(pattern_space as *mut libc::c_void);
                  pattern_space = xstrdup(
                    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                      .hold_space
                      .is_null()
                    {
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                    } else {
                      b"\x00" as *const u8 as *const libc::c_char
                    },
                  );
                  current_block = 17965632435239708295;
                }
                1739363794695357236 => {
                  let mut i: libc::c_int = 0;
                  let mut j: libc::c_int = 0;
                  i = 0i32;
                  while *pattern_space.offset(i as isize) != 0 {
                    j = 0i32;
                    while *(*sed_cmd).string.offset(j as isize) != 0 {
                      if *pattern_space.offset(i as isize) as libc::c_int
                        == *(*sed_cmd).string.offset(j as isize) as libc::c_int
                      {
                        *pattern_space.offset(i as isize) =
                          *(*sed_cmd).string.offset((j + 1i32) as isize);
                        break;
                      } else {
                        j += 2i32
                      }
                    }
                    i += 1
                  }
                  current_block = 17965632435239708295;
                }
                17611301398573303352 => {
                  if substituted == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted = 0i32;
                    current_block = 13861430101487131366;
                  }
                }
                7297078374430259003 => {
                  let mut len: libc::c_int = 0;
                  if next_line.is_null() {
                    current_block = 8563197331115798083;
                    break 's_54;
                  } else {
                    len = strlen(pattern_space) as libc::c_int;
                    pattern_space = xrealloc(
                      pattern_space as *mut libc::c_void,
                      (len as libc::c_ulong)
                        .wrapping_add(strlen(next_line))
                        .wrapping_add(2i32 as libc::c_ulong),
                    ) as *mut libc::c_char;
                    *pattern_space.offset(len as isize) = '\n' as i32 as libc::c_char;
                    strcpy(pattern_space.offset(len as isize).offset(1), next_line);
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                1534452661327725232 => {
                  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).be_quiet == 0 {
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      last_gets_char,
                    );
                  }
                  if next_line.is_null() {
                    current_block = 4142149688065477410;
                    break 's_54;
                  } else {
                    free(pattern_space as *mut libc::c_void);
                    pattern_space = next_line;
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    substituted = 0i32;
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                726525485109251713 => {
                  let mut rfile: *mut FILE = 0 as *mut FILE;
                  rfile = fopen_for_read((*sed_cmd).string);
                  if !rfile.is_null() {
                    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
                    loop {
                      line = xmalloc_fgetline(rfile);
                      if line.is_null() {
                        break;
                      }
                      append(line);
                    }
                    fclose(rfile);
                  }
                  current_block = 17965632435239708295;
                }
                13714184482889841412 => {
                  if do_subst_command(sed_cmd, &mut pattern_space) == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted |= 1i32;
                    if (*sed_cmd).sub_p() != 0 {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        last_gets_char,
                      );
                    }
                    if !(*sed_cmd).sw_file.is_null() {
                      puts_maybe_newline(
                        pattern_space,
                        (*sed_cmd).sw_file,
                        &mut (*sed_cmd).sw_last_char,
                        last_gets_char,
                      );
                    }
                    current_block = 17965632435239708295;
                  }
                }
                2290177392965769716 => {
                  let mut tmp: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp.is_null() {
                    *tmp = '\u{0}' as i32 as libc::c_char;
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                    *tmp = '\n' as i32 as libc::c_char;
                    current_block = 17965632435239708295;
                  } else {
                    current_block = 7330787439563864000;
                  }
                }
                3546145585875536353 => {
                  let mut tmp_0: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp_0.is_null() {
                    overlapping_strcpy(pattern_space, tmp_0.offset(1));
                    break;
                  } else {
                    current_block = 4142149688065477410;
                    break 's_54;
                  }
                }
                10499578865704240760 => {
                  free(next_line as *mut libc::c_void);
                  next_line = 0 as *mut libc::c_char;
                  current_block = 8563197331115798083;
                  break 's_54;
                }
                13665239467142187023 => {
                  let mut tmp_1: *mut libc::c_char = pattern_space;
                  pattern_space = if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void
                  } else {
                    xzalloc(1i32 as size_t)
                  } as *mut libc::c_char;
                  last_gets_char = '\n' as i32 as libc::c_char;
                  let ref mut fresh28 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh28 = tmp_1;
                  current_block = 17965632435239708295;
                }
                332836909292088582 => {
                  fprintf(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    b"%d\n\x00" as *const u8 as *const libc::c_char,
                    linenum,
                  );
                  current_block = 17965632435239708295;
                }
                17152203569385922329 => {
                  append(xstrdup((*sed_cmd).string));
                  current_block = 17965632435239708295;
                }
                14487425527653873875 => {
                  puts_maybe_newline(
                    (*sed_cmd).string,
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    &mut last_puts_char,
                    '\n' as i32 as libc::c_char,
                  );
                  current_block = 17965632435239708295;
                }
                2782169280456925021 => {
                  if (*sed_cmd).in_match() == 0 {
                    puts_maybe_newline(
                      (*sed_cmd).string,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                  }
                  current_block = 4142149688065477410;
                  break 's_54;
                }
                5622786547828025790 => {
                  puts_maybe_newline(
                    pattern_space,
                    (*sed_cmd).sw_file,
                    &mut (*sed_cmd).sw_last_char,
                    last_gets_char,
                  );
                  current_block = 17965632435239708295;
                }
                _ => {}
              }
              match current_block {
                17965632435239708295 => {}
                _ => {
                  match current_block {
                    7330787439563864000 => {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        '\n' as i32 as libc::c_char,
                      );
                      current_block = 17965632435239708295;
                    }
                    13861430101487131366 => {
                      if substituted != 0 {
                        current_block = 17965632435239708295;
                      } else {
                        current_block = 2887315643959147419;
                      }
                    }
                    _ => {}
                  }
                  match current_block {
                    17965632435239708295 => {}
                    _ => {
                      if (*sed_cmd).string.is_null() {
                        current_block = 8563197331115798083;
                        break 's_54;
                      }
                      sed_cmd = branch_to((*sed_cmd).string)
                    }
                  }
                }
              }
            }
            98 => {
              current_block = 2887315643959147419;
              match current_block {
                11227437541145425351 => {
                  let mut hold_space_size_0: libc::c_int = 2i32;
                  let mut pattern_space_size_0: libc::c_int = 0i32;
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size_0 = (hold_space_size_0 as libc::c_ulong).wrapping_add(strlen(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    )) as libc::c_int as libc::c_int
                  }
                  if !pattern_space.is_null() {
                    pattern_space_size_0 = strlen(pattern_space) as libc::c_int
                  }
                  let ref mut fresh27 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh27 = xrealloc(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                    (hold_space_size_0 + pattern_space_size_0) as size_t,
                  ) as *mut libc::c_char;
                  if hold_space_size_0 == 2i32 {
                    *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space =
                      0i32 as libc::c_char
                  }
                  strcat(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    b"\n\x00" as *const u8 as *const libc::c_char,
                  );
                  if !pattern_space.is_null() {
                    strcat(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                      pattern_space,
                    );
                  }
                  current_block = 17965632435239708295;
                }
                17838047945882858323 => {
                  free(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                  );
                  let ref mut fresh26 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh26 = xstrdup(pattern_space);
                  current_block = 17965632435239708295;
                }
                10468276026569382870 => {
                  let mut pattern_space_size: libc::c_int = 2i32;
                  let mut hold_space_size: libc::c_int = 0i32;
                  if !pattern_space.is_null() {
                    pattern_space_size = (pattern_space_size as libc::c_ulong)
                      .wrapping_add(strlen(pattern_space))
                      as libc::c_int as libc::c_int
                  }
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size =
                      strlen((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space)
                        as libc::c_int
                  }
                  pattern_space = xrealloc(
                    pattern_space as *mut libc::c_void,
                    (pattern_space_size + hold_space_size) as size_t,
                  ) as *mut libc::c_char;
                  if pattern_space_size == 2i32 {
                    *pattern_space.offset(0) = 0i32 as libc::c_char
                  }
                  strcat(pattern_space, b"\n\x00" as *const u8 as *const libc::c_char);
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    strcat(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    );
                  }
                  last_gets_char = '\n' as i32 as libc::c_char;
                  current_block = 17965632435239708295;
                }
                14127364983570718321 => {
                  free(pattern_space as *mut libc::c_void);
                  pattern_space = xstrdup(
                    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                      .hold_space
                      .is_null()
                    {
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                    } else {
                      b"\x00" as *const u8 as *const libc::c_char
                    },
                  );
                  current_block = 17965632435239708295;
                }
                1739363794695357236 => {
                  let mut i: libc::c_int = 0;
                  let mut j: libc::c_int = 0;
                  i = 0i32;
                  while *pattern_space.offset(i as isize) != 0 {
                    j = 0i32;
                    while *(*sed_cmd).string.offset(j as isize) != 0 {
                      if *pattern_space.offset(i as isize) as libc::c_int
                        == *(*sed_cmd).string.offset(j as isize) as libc::c_int
                      {
                        *pattern_space.offset(i as isize) =
                          *(*sed_cmd).string.offset((j + 1i32) as isize);
                        break;
                      } else {
                        j += 2i32
                      }
                    }
                    i += 1
                  }
                  current_block = 17965632435239708295;
                }
                17611301398573303352 => {
                  if substituted == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted = 0i32;
                    current_block = 13861430101487131366;
                  }
                }
                7297078374430259003 => {
                  let mut len: libc::c_int = 0;
                  if next_line.is_null() {
                    current_block = 8563197331115798083;
                    break 's_54;
                  } else {
                    len = strlen(pattern_space) as libc::c_int;
                    pattern_space = xrealloc(
                      pattern_space as *mut libc::c_void,
                      (len as libc::c_ulong)
                        .wrapping_add(strlen(next_line))
                        .wrapping_add(2i32 as libc::c_ulong),
                    ) as *mut libc::c_char;
                    *pattern_space.offset(len as isize) = '\n' as i32 as libc::c_char;
                    strcpy(pattern_space.offset(len as isize).offset(1), next_line);
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                1534452661327725232 => {
                  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).be_quiet == 0 {
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      last_gets_char,
                    );
                  }
                  if next_line.is_null() {
                    current_block = 4142149688065477410;
                    break 's_54;
                  } else {
                    free(pattern_space as *mut libc::c_void);
                    pattern_space = next_line;
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    substituted = 0i32;
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                726525485109251713 => {
                  let mut rfile: *mut FILE = 0 as *mut FILE;
                  rfile = fopen_for_read((*sed_cmd).string);
                  if !rfile.is_null() {
                    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
                    loop {
                      line = xmalloc_fgetline(rfile);
                      if line.is_null() {
                        break;
                      }
                      append(line);
                    }
                    fclose(rfile);
                  }
                  current_block = 17965632435239708295;
                }
                13714184482889841412 => {
                  if do_subst_command(sed_cmd, &mut pattern_space) == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted |= 1i32;
                    if (*sed_cmd).sub_p() != 0 {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        last_gets_char,
                      );
                    }
                    if !(*sed_cmd).sw_file.is_null() {
                      puts_maybe_newline(
                        pattern_space,
                        (*sed_cmd).sw_file,
                        &mut (*sed_cmd).sw_last_char,
                        last_gets_char,
                      );
                    }
                    current_block = 17965632435239708295;
                  }
                }
                2290177392965769716 => {
                  let mut tmp: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp.is_null() {
                    *tmp = '\u{0}' as i32 as libc::c_char;
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                    *tmp = '\n' as i32 as libc::c_char;
                    current_block = 17965632435239708295;
                  } else {
                    current_block = 7330787439563864000;
                  }
                }
                3546145585875536353 => {
                  let mut tmp_0: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp_0.is_null() {
                    overlapping_strcpy(pattern_space, tmp_0.offset(1));
                    break;
                  } else {
                    current_block = 4142149688065477410;
                    break 's_54;
                  }
                }
                10499578865704240760 => {
                  free(next_line as *mut libc::c_void);
                  next_line = 0 as *mut libc::c_char;
                  current_block = 8563197331115798083;
                  break 's_54;
                }
                13665239467142187023 => {
                  let mut tmp_1: *mut libc::c_char = pattern_space;
                  pattern_space = if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void
                  } else {
                    xzalloc(1i32 as size_t)
                  } as *mut libc::c_char;
                  last_gets_char = '\n' as i32 as libc::c_char;
                  let ref mut fresh28 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh28 = tmp_1;
                  current_block = 17965632435239708295;
                }
                332836909292088582 => {
                  fprintf(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    b"%d\n\x00" as *const u8 as *const libc::c_char,
                    linenum,
                  );
                  current_block = 17965632435239708295;
                }
                17152203569385922329 => {
                  append(xstrdup((*sed_cmd).string));
                  current_block = 17965632435239708295;
                }
                14487425527653873875 => {
                  puts_maybe_newline(
                    (*sed_cmd).string,
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    &mut last_puts_char,
                    '\n' as i32 as libc::c_char,
                  );
                  current_block = 17965632435239708295;
                }
                2782169280456925021 => {
                  if (*sed_cmd).in_match() == 0 {
                    puts_maybe_newline(
                      (*sed_cmd).string,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                  }
                  current_block = 4142149688065477410;
                  break 's_54;
                }
                5622786547828025790 => {
                  puts_maybe_newline(
                    pattern_space,
                    (*sed_cmd).sw_file,
                    &mut (*sed_cmd).sw_last_char,
                    last_gets_char,
                  );
                  current_block = 17965632435239708295;
                }
                _ => {}
              }
              match current_block {
                17965632435239708295 => {}
                _ => {
                  match current_block {
                    7330787439563864000 => {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        '\n' as i32 as libc::c_char,
                      );
                      current_block = 17965632435239708295;
                    }
                    13861430101487131366 => {
                      if substituted != 0 {
                        current_block = 17965632435239708295;
                      } else {
                        current_block = 2887315643959147419;
                      }
                    }
                    _ => {}
                  }
                  match current_block {
                    17965632435239708295 => {}
                    _ => {
                      if (*sed_cmd).string.is_null() {
                        current_block = 8563197331115798083;
                        break 's_54;
                      }
                      sed_cmd = branch_to((*sed_cmd).string)
                    }
                  }
                }
              }
            }
            121 => {
              current_block = 1739363794695357236;
              match current_block {
                11227437541145425351 => {
                  let mut hold_space_size_0: libc::c_int = 2i32;
                  let mut pattern_space_size_0: libc::c_int = 0i32;
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size_0 = (hold_space_size_0 as libc::c_ulong).wrapping_add(strlen(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    )) as libc::c_int as libc::c_int
                  }
                  if !pattern_space.is_null() {
                    pattern_space_size_0 = strlen(pattern_space) as libc::c_int
                  }
                  let ref mut fresh27 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh27 = xrealloc(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                    (hold_space_size_0 + pattern_space_size_0) as size_t,
                  ) as *mut libc::c_char;
                  if hold_space_size_0 == 2i32 {
                    *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space =
                      0i32 as libc::c_char
                  }
                  strcat(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    b"\n\x00" as *const u8 as *const libc::c_char,
                  );
                  if !pattern_space.is_null() {
                    strcat(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                      pattern_space,
                    );
                  }
                  current_block = 17965632435239708295;
                }
                17838047945882858323 => {
                  free(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                  );
                  let ref mut fresh26 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh26 = xstrdup(pattern_space);
                  current_block = 17965632435239708295;
                }
                10468276026569382870 => {
                  let mut pattern_space_size: libc::c_int = 2i32;
                  let mut hold_space_size: libc::c_int = 0i32;
                  if !pattern_space.is_null() {
                    pattern_space_size = (pattern_space_size as libc::c_ulong)
                      .wrapping_add(strlen(pattern_space))
                      as libc::c_int as libc::c_int
                  }
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size =
                      strlen((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space)
                        as libc::c_int
                  }
                  pattern_space = xrealloc(
                    pattern_space as *mut libc::c_void,
                    (pattern_space_size + hold_space_size) as size_t,
                  ) as *mut libc::c_char;
                  if pattern_space_size == 2i32 {
                    *pattern_space.offset(0) = 0i32 as libc::c_char
                  }
                  strcat(pattern_space, b"\n\x00" as *const u8 as *const libc::c_char);
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    strcat(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    );
                  }
                  last_gets_char = '\n' as i32 as libc::c_char;
                  current_block = 17965632435239708295;
                }
                14127364983570718321 => {
                  free(pattern_space as *mut libc::c_void);
                  pattern_space = xstrdup(
                    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                      .hold_space
                      .is_null()
                    {
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                    } else {
                      b"\x00" as *const u8 as *const libc::c_char
                    },
                  );
                  current_block = 17965632435239708295;
                }
                1739363794695357236 => {
                  let mut i: libc::c_int = 0;
                  let mut j: libc::c_int = 0;
                  i = 0i32;
                  while *pattern_space.offset(i as isize) != 0 {
                    j = 0i32;
                    while *(*sed_cmd).string.offset(j as isize) != 0 {
                      if *pattern_space.offset(i as isize) as libc::c_int
                        == *(*sed_cmd).string.offset(j as isize) as libc::c_int
                      {
                        *pattern_space.offset(i as isize) =
                          *(*sed_cmd).string.offset((j + 1i32) as isize);
                        break;
                      } else {
                        j += 2i32
                      }
                    }
                    i += 1
                  }
                  current_block = 17965632435239708295;
                }
                17611301398573303352 => {
                  if substituted == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted = 0i32;
                    current_block = 13861430101487131366;
                  }
                }
                7297078374430259003 => {
                  let mut len: libc::c_int = 0;
                  if next_line.is_null() {
                    current_block = 8563197331115798083;
                    break 's_54;
                  } else {
                    len = strlen(pattern_space) as libc::c_int;
                    pattern_space = xrealloc(
                      pattern_space as *mut libc::c_void,
                      (len as libc::c_ulong)
                        .wrapping_add(strlen(next_line))
                        .wrapping_add(2i32 as libc::c_ulong),
                    ) as *mut libc::c_char;
                    *pattern_space.offset(len as isize) = '\n' as i32 as libc::c_char;
                    strcpy(pattern_space.offset(len as isize).offset(1), next_line);
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                1534452661327725232 => {
                  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).be_quiet == 0 {
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      last_gets_char,
                    );
                  }
                  if next_line.is_null() {
                    current_block = 4142149688065477410;
                    break 's_54;
                  } else {
                    free(pattern_space as *mut libc::c_void);
                    pattern_space = next_line;
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    substituted = 0i32;
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                726525485109251713 => {
                  let mut rfile: *mut FILE = 0 as *mut FILE;
                  rfile = fopen_for_read((*sed_cmd).string);
                  if !rfile.is_null() {
                    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
                    loop {
                      line = xmalloc_fgetline(rfile);
                      if line.is_null() {
                        break;
                      }
                      append(line);
                    }
                    fclose(rfile);
                  }
                  current_block = 17965632435239708295;
                }
                13714184482889841412 => {
                  if do_subst_command(sed_cmd, &mut pattern_space) == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted |= 1i32;
                    if (*sed_cmd).sub_p() != 0 {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        last_gets_char,
                      );
                    }
                    if !(*sed_cmd).sw_file.is_null() {
                      puts_maybe_newline(
                        pattern_space,
                        (*sed_cmd).sw_file,
                        &mut (*sed_cmd).sw_last_char,
                        last_gets_char,
                      );
                    }
                    current_block = 17965632435239708295;
                  }
                }
                2290177392965769716 => {
                  let mut tmp: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp.is_null() {
                    *tmp = '\u{0}' as i32 as libc::c_char;
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                    *tmp = '\n' as i32 as libc::c_char;
                    current_block = 17965632435239708295;
                  } else {
                    current_block = 7330787439563864000;
                  }
                }
                3546145585875536353 => {
                  let mut tmp_0: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp_0.is_null() {
                    overlapping_strcpy(pattern_space, tmp_0.offset(1));
                    break;
                  } else {
                    current_block = 4142149688065477410;
                    break 's_54;
                  }
                }
                10499578865704240760 => {
                  free(next_line as *mut libc::c_void);
                  next_line = 0 as *mut libc::c_char;
                  current_block = 8563197331115798083;
                  break 's_54;
                }
                13665239467142187023 => {
                  let mut tmp_1: *mut libc::c_char = pattern_space;
                  pattern_space = if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void
                  } else {
                    xzalloc(1i32 as size_t)
                  } as *mut libc::c_char;
                  last_gets_char = '\n' as i32 as libc::c_char;
                  let ref mut fresh28 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh28 = tmp_1;
                  current_block = 17965632435239708295;
                }
                332836909292088582 => {
                  fprintf(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    b"%d\n\x00" as *const u8 as *const libc::c_char,
                    linenum,
                  );
                  current_block = 17965632435239708295;
                }
                17152203569385922329 => {
                  append(xstrdup((*sed_cmd).string));
                  current_block = 17965632435239708295;
                }
                14487425527653873875 => {
                  puts_maybe_newline(
                    (*sed_cmd).string,
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    &mut last_puts_char,
                    '\n' as i32 as libc::c_char,
                  );
                  current_block = 17965632435239708295;
                }
                2782169280456925021 => {
                  if (*sed_cmd).in_match() == 0 {
                    puts_maybe_newline(
                      (*sed_cmd).string,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                  }
                  current_block = 4142149688065477410;
                  break 's_54;
                }
                5622786547828025790 => {
                  puts_maybe_newline(
                    pattern_space,
                    (*sed_cmd).sw_file,
                    &mut (*sed_cmd).sw_last_char,
                    last_gets_char,
                  );
                  current_block = 17965632435239708295;
                }
                _ => {}
              }
              match current_block {
                17965632435239708295 => {}
                _ => {
                  match current_block {
                    7330787439563864000 => {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        '\n' as i32 as libc::c_char,
                      );
                      current_block = 17965632435239708295;
                    }
                    13861430101487131366 => {
                      if substituted != 0 {
                        current_block = 17965632435239708295;
                      } else {
                        current_block = 2887315643959147419;
                      }
                    }
                    _ => {}
                  }
                  match current_block {
                    17965632435239708295 => {}
                    _ => {
                      if (*sed_cmd).string.is_null() {
                        current_block = 8563197331115798083;
                        break 's_54;
                      }
                      sed_cmd = branch_to((*sed_cmd).string)
                    }
                  }
                }
              }
            }
            103 => {
              current_block = 14127364983570718321;
              match current_block {
                11227437541145425351 => {
                  let mut hold_space_size_0: libc::c_int = 2i32;
                  let mut pattern_space_size_0: libc::c_int = 0i32;
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size_0 = (hold_space_size_0 as libc::c_ulong).wrapping_add(strlen(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    )) as libc::c_int as libc::c_int
                  }
                  if !pattern_space.is_null() {
                    pattern_space_size_0 = strlen(pattern_space) as libc::c_int
                  }
                  let ref mut fresh27 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh27 = xrealloc(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                    (hold_space_size_0 + pattern_space_size_0) as size_t,
                  ) as *mut libc::c_char;
                  if hold_space_size_0 == 2i32 {
                    *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space =
                      0i32 as libc::c_char
                  }
                  strcat(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    b"\n\x00" as *const u8 as *const libc::c_char,
                  );
                  if !pattern_space.is_null() {
                    strcat(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                      pattern_space,
                    );
                  }
                  current_block = 17965632435239708295;
                }
                17838047945882858323 => {
                  free(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                  );
                  let ref mut fresh26 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh26 = xstrdup(pattern_space);
                  current_block = 17965632435239708295;
                }
                10468276026569382870 => {
                  let mut pattern_space_size: libc::c_int = 2i32;
                  let mut hold_space_size: libc::c_int = 0i32;
                  if !pattern_space.is_null() {
                    pattern_space_size = (pattern_space_size as libc::c_ulong)
                      .wrapping_add(strlen(pattern_space))
                      as libc::c_int as libc::c_int
                  }
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size =
                      strlen((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space)
                        as libc::c_int
                  }
                  pattern_space = xrealloc(
                    pattern_space as *mut libc::c_void,
                    (pattern_space_size + hold_space_size) as size_t,
                  ) as *mut libc::c_char;
                  if pattern_space_size == 2i32 {
                    *pattern_space.offset(0) = 0i32 as libc::c_char
                  }
                  strcat(pattern_space, b"\n\x00" as *const u8 as *const libc::c_char);
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    strcat(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    );
                  }
                  last_gets_char = '\n' as i32 as libc::c_char;
                  current_block = 17965632435239708295;
                }
                14127364983570718321 => {
                  free(pattern_space as *mut libc::c_void);
                  pattern_space = xstrdup(
                    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                      .hold_space
                      .is_null()
                    {
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                    } else {
                      b"\x00" as *const u8 as *const libc::c_char
                    },
                  );
                  current_block = 17965632435239708295;
                }
                1739363794695357236 => {
                  let mut i: libc::c_int = 0;
                  let mut j: libc::c_int = 0;
                  i = 0i32;
                  while *pattern_space.offset(i as isize) != 0 {
                    j = 0i32;
                    while *(*sed_cmd).string.offset(j as isize) != 0 {
                      if *pattern_space.offset(i as isize) as libc::c_int
                        == *(*sed_cmd).string.offset(j as isize) as libc::c_int
                      {
                        *pattern_space.offset(i as isize) =
                          *(*sed_cmd).string.offset((j + 1i32) as isize);
                        break;
                      } else {
                        j += 2i32
                      }
                    }
                    i += 1
                  }
                  current_block = 17965632435239708295;
                }
                17611301398573303352 => {
                  if substituted == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted = 0i32;
                    current_block = 13861430101487131366;
                  }
                }
                7297078374430259003 => {
                  let mut len: libc::c_int = 0;
                  if next_line.is_null() {
                    current_block = 8563197331115798083;
                    break 's_54;
                  } else {
                    len = strlen(pattern_space) as libc::c_int;
                    pattern_space = xrealloc(
                      pattern_space as *mut libc::c_void,
                      (len as libc::c_ulong)
                        .wrapping_add(strlen(next_line))
                        .wrapping_add(2i32 as libc::c_ulong),
                    ) as *mut libc::c_char;
                    *pattern_space.offset(len as isize) = '\n' as i32 as libc::c_char;
                    strcpy(pattern_space.offset(len as isize).offset(1), next_line);
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                1534452661327725232 => {
                  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).be_quiet == 0 {
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      last_gets_char,
                    );
                  }
                  if next_line.is_null() {
                    current_block = 4142149688065477410;
                    break 's_54;
                  } else {
                    free(pattern_space as *mut libc::c_void);
                    pattern_space = next_line;
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    substituted = 0i32;
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                726525485109251713 => {
                  let mut rfile: *mut FILE = 0 as *mut FILE;
                  rfile = fopen_for_read((*sed_cmd).string);
                  if !rfile.is_null() {
                    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
                    loop {
                      line = xmalloc_fgetline(rfile);
                      if line.is_null() {
                        break;
                      }
                      append(line);
                    }
                    fclose(rfile);
                  }
                  current_block = 17965632435239708295;
                }
                13714184482889841412 => {
                  if do_subst_command(sed_cmd, &mut pattern_space) == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted |= 1i32;
                    if (*sed_cmd).sub_p() != 0 {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        last_gets_char,
                      );
                    }
                    if !(*sed_cmd).sw_file.is_null() {
                      puts_maybe_newline(
                        pattern_space,
                        (*sed_cmd).sw_file,
                        &mut (*sed_cmd).sw_last_char,
                        last_gets_char,
                      );
                    }
                    current_block = 17965632435239708295;
                  }
                }
                2290177392965769716 => {
                  let mut tmp: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp.is_null() {
                    *tmp = '\u{0}' as i32 as libc::c_char;
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                    *tmp = '\n' as i32 as libc::c_char;
                    current_block = 17965632435239708295;
                  } else {
                    current_block = 7330787439563864000;
                  }
                }
                3546145585875536353 => {
                  let mut tmp_0: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp_0.is_null() {
                    overlapping_strcpy(pattern_space, tmp_0.offset(1));
                    break;
                  } else {
                    current_block = 4142149688065477410;
                    break 's_54;
                  }
                }
                10499578865704240760 => {
                  free(next_line as *mut libc::c_void);
                  next_line = 0 as *mut libc::c_char;
                  current_block = 8563197331115798083;
                  break 's_54;
                }
                13665239467142187023 => {
                  let mut tmp_1: *mut libc::c_char = pattern_space;
                  pattern_space = if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void
                  } else {
                    xzalloc(1i32 as size_t)
                  } as *mut libc::c_char;
                  last_gets_char = '\n' as i32 as libc::c_char;
                  let ref mut fresh28 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh28 = tmp_1;
                  current_block = 17965632435239708295;
                }
                332836909292088582 => {
                  fprintf(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    b"%d\n\x00" as *const u8 as *const libc::c_char,
                    linenum,
                  );
                  current_block = 17965632435239708295;
                }
                17152203569385922329 => {
                  append(xstrdup((*sed_cmd).string));
                  current_block = 17965632435239708295;
                }
                14487425527653873875 => {
                  puts_maybe_newline(
                    (*sed_cmd).string,
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    &mut last_puts_char,
                    '\n' as i32 as libc::c_char,
                  );
                  current_block = 17965632435239708295;
                }
                2782169280456925021 => {
                  if (*sed_cmd).in_match() == 0 {
                    puts_maybe_newline(
                      (*sed_cmd).string,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                  }
                  current_block = 4142149688065477410;
                  break 's_54;
                }
                5622786547828025790 => {
                  puts_maybe_newline(
                    pattern_space,
                    (*sed_cmd).sw_file,
                    &mut (*sed_cmd).sw_last_char,
                    last_gets_char,
                  );
                  current_block = 17965632435239708295;
                }
                _ => {}
              }
              match current_block {
                17965632435239708295 => {}
                _ => {
                  match current_block {
                    7330787439563864000 => {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        '\n' as i32 as libc::c_char,
                      );
                      current_block = 17965632435239708295;
                    }
                    13861430101487131366 => {
                      if substituted != 0 {
                        current_block = 17965632435239708295;
                      } else {
                        current_block = 2887315643959147419;
                      }
                    }
                    _ => {}
                  }
                  match current_block {
                    17965632435239708295 => {}
                    _ => {
                      if (*sed_cmd).string.is_null() {
                        current_block = 8563197331115798083;
                        break 's_54;
                      }
                      sed_cmd = branch_to((*sed_cmd).string)
                    }
                  }
                }
              }
            }
            71 => {
              current_block = 10468276026569382870;
              match current_block {
                11227437541145425351 => {
                  let mut hold_space_size_0: libc::c_int = 2i32;
                  let mut pattern_space_size_0: libc::c_int = 0i32;
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size_0 = (hold_space_size_0 as libc::c_ulong).wrapping_add(strlen(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    )) as libc::c_int as libc::c_int
                  }
                  if !pattern_space.is_null() {
                    pattern_space_size_0 = strlen(pattern_space) as libc::c_int
                  }
                  let ref mut fresh27 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh27 = xrealloc(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                    (hold_space_size_0 + pattern_space_size_0) as size_t,
                  ) as *mut libc::c_char;
                  if hold_space_size_0 == 2i32 {
                    *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space =
                      0i32 as libc::c_char
                  }
                  strcat(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    b"\n\x00" as *const u8 as *const libc::c_char,
                  );
                  if !pattern_space.is_null() {
                    strcat(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                      pattern_space,
                    );
                  }
                  current_block = 17965632435239708295;
                }
                17838047945882858323 => {
                  free(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                  );
                  let ref mut fresh26 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh26 = xstrdup(pattern_space);
                  current_block = 17965632435239708295;
                }
                10468276026569382870 => {
                  let mut pattern_space_size: libc::c_int = 2i32;
                  let mut hold_space_size: libc::c_int = 0i32;
                  if !pattern_space.is_null() {
                    pattern_space_size = (pattern_space_size as libc::c_ulong)
                      .wrapping_add(strlen(pattern_space))
                      as libc::c_int as libc::c_int
                  }
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size =
                      strlen((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space)
                        as libc::c_int
                  }
                  pattern_space = xrealloc(
                    pattern_space as *mut libc::c_void,
                    (pattern_space_size + hold_space_size) as size_t,
                  ) as *mut libc::c_char;
                  if pattern_space_size == 2i32 {
                    *pattern_space.offset(0) = 0i32 as libc::c_char
                  }
                  strcat(pattern_space, b"\n\x00" as *const u8 as *const libc::c_char);
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    strcat(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    );
                  }
                  last_gets_char = '\n' as i32 as libc::c_char;
                  current_block = 17965632435239708295;
                }
                14127364983570718321 => {
                  free(pattern_space as *mut libc::c_void);
                  pattern_space = xstrdup(
                    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                      .hold_space
                      .is_null()
                    {
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                    } else {
                      b"\x00" as *const u8 as *const libc::c_char
                    },
                  );
                  current_block = 17965632435239708295;
                }
                1739363794695357236 => {
                  let mut i: libc::c_int = 0;
                  let mut j: libc::c_int = 0;
                  i = 0i32;
                  while *pattern_space.offset(i as isize) != 0 {
                    j = 0i32;
                    while *(*sed_cmd).string.offset(j as isize) != 0 {
                      if *pattern_space.offset(i as isize) as libc::c_int
                        == *(*sed_cmd).string.offset(j as isize) as libc::c_int
                      {
                        *pattern_space.offset(i as isize) =
                          *(*sed_cmd).string.offset((j + 1i32) as isize);
                        break;
                      } else {
                        j += 2i32
                      }
                    }
                    i += 1
                  }
                  current_block = 17965632435239708295;
                }
                17611301398573303352 => {
                  if substituted == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted = 0i32;
                    current_block = 13861430101487131366;
                  }
                }
                7297078374430259003 => {
                  let mut len: libc::c_int = 0;
                  if next_line.is_null() {
                    current_block = 8563197331115798083;
                    break 's_54;
                  } else {
                    len = strlen(pattern_space) as libc::c_int;
                    pattern_space = xrealloc(
                      pattern_space as *mut libc::c_void,
                      (len as libc::c_ulong)
                        .wrapping_add(strlen(next_line))
                        .wrapping_add(2i32 as libc::c_ulong),
                    ) as *mut libc::c_char;
                    *pattern_space.offset(len as isize) = '\n' as i32 as libc::c_char;
                    strcpy(pattern_space.offset(len as isize).offset(1), next_line);
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                1534452661327725232 => {
                  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).be_quiet == 0 {
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      last_gets_char,
                    );
                  }
                  if next_line.is_null() {
                    current_block = 4142149688065477410;
                    break 's_54;
                  } else {
                    free(pattern_space as *mut libc::c_void);
                    pattern_space = next_line;
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    substituted = 0i32;
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                726525485109251713 => {
                  let mut rfile: *mut FILE = 0 as *mut FILE;
                  rfile = fopen_for_read((*sed_cmd).string);
                  if !rfile.is_null() {
                    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
                    loop {
                      line = xmalloc_fgetline(rfile);
                      if line.is_null() {
                        break;
                      }
                      append(line);
                    }
                    fclose(rfile);
                  }
                  current_block = 17965632435239708295;
                }
                13714184482889841412 => {
                  if do_subst_command(sed_cmd, &mut pattern_space) == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted |= 1i32;
                    if (*sed_cmd).sub_p() != 0 {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        last_gets_char,
                      );
                    }
                    if !(*sed_cmd).sw_file.is_null() {
                      puts_maybe_newline(
                        pattern_space,
                        (*sed_cmd).sw_file,
                        &mut (*sed_cmd).sw_last_char,
                        last_gets_char,
                      );
                    }
                    current_block = 17965632435239708295;
                  }
                }
                2290177392965769716 => {
                  let mut tmp: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp.is_null() {
                    *tmp = '\u{0}' as i32 as libc::c_char;
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                    *tmp = '\n' as i32 as libc::c_char;
                    current_block = 17965632435239708295;
                  } else {
                    current_block = 7330787439563864000;
                  }
                }
                3546145585875536353 => {
                  let mut tmp_0: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp_0.is_null() {
                    overlapping_strcpy(pattern_space, tmp_0.offset(1));
                    break;
                  } else {
                    current_block = 4142149688065477410;
                    break 's_54;
                  }
                }
                10499578865704240760 => {
                  free(next_line as *mut libc::c_void);
                  next_line = 0 as *mut libc::c_char;
                  current_block = 8563197331115798083;
                  break 's_54;
                }
                13665239467142187023 => {
                  let mut tmp_1: *mut libc::c_char = pattern_space;
                  pattern_space = if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void
                  } else {
                    xzalloc(1i32 as size_t)
                  } as *mut libc::c_char;
                  last_gets_char = '\n' as i32 as libc::c_char;
                  let ref mut fresh28 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh28 = tmp_1;
                  current_block = 17965632435239708295;
                }
                332836909292088582 => {
                  fprintf(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    b"%d\n\x00" as *const u8 as *const libc::c_char,
                    linenum,
                  );
                  current_block = 17965632435239708295;
                }
                17152203569385922329 => {
                  append(xstrdup((*sed_cmd).string));
                  current_block = 17965632435239708295;
                }
                14487425527653873875 => {
                  puts_maybe_newline(
                    (*sed_cmd).string,
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    &mut last_puts_char,
                    '\n' as i32 as libc::c_char,
                  );
                  current_block = 17965632435239708295;
                }
                2782169280456925021 => {
                  if (*sed_cmd).in_match() == 0 {
                    puts_maybe_newline(
                      (*sed_cmd).string,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                  }
                  current_block = 4142149688065477410;
                  break 's_54;
                }
                5622786547828025790 => {
                  puts_maybe_newline(
                    pattern_space,
                    (*sed_cmd).sw_file,
                    &mut (*sed_cmd).sw_last_char,
                    last_gets_char,
                  );
                  current_block = 17965632435239708295;
                }
                _ => {}
              }
              match current_block {
                17965632435239708295 => {}
                _ => {
                  match current_block {
                    7330787439563864000 => {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        '\n' as i32 as libc::c_char,
                      );
                      current_block = 17965632435239708295;
                    }
                    13861430101487131366 => {
                      if substituted != 0 {
                        current_block = 17965632435239708295;
                      } else {
                        current_block = 2887315643959147419;
                      }
                    }
                    _ => {}
                  }
                  match current_block {
                    17965632435239708295 => {}
                    _ => {
                      if (*sed_cmd).string.is_null() {
                        current_block = 8563197331115798083;
                        break 's_54;
                      }
                      sed_cmd = branch_to((*sed_cmd).string)
                    }
                  }
                }
              }
            }
            104 => {
              current_block = 17838047945882858323;
              match current_block {
                11227437541145425351 => {
                  let mut hold_space_size_0: libc::c_int = 2i32;
                  let mut pattern_space_size_0: libc::c_int = 0i32;
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size_0 = (hold_space_size_0 as libc::c_ulong).wrapping_add(strlen(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    )) as libc::c_int as libc::c_int
                  }
                  if !pattern_space.is_null() {
                    pattern_space_size_0 = strlen(pattern_space) as libc::c_int
                  }
                  let ref mut fresh27 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh27 = xrealloc(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                    (hold_space_size_0 + pattern_space_size_0) as size_t,
                  ) as *mut libc::c_char;
                  if hold_space_size_0 == 2i32 {
                    *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space =
                      0i32 as libc::c_char
                  }
                  strcat(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    b"\n\x00" as *const u8 as *const libc::c_char,
                  );
                  if !pattern_space.is_null() {
                    strcat(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                      pattern_space,
                    );
                  }
                  current_block = 17965632435239708295;
                }
                17838047945882858323 => {
                  free(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                  );
                  let ref mut fresh26 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh26 = xstrdup(pattern_space);
                  current_block = 17965632435239708295;
                }
                10468276026569382870 => {
                  let mut pattern_space_size: libc::c_int = 2i32;
                  let mut hold_space_size: libc::c_int = 0i32;
                  if !pattern_space.is_null() {
                    pattern_space_size = (pattern_space_size as libc::c_ulong)
                      .wrapping_add(strlen(pattern_space))
                      as libc::c_int as libc::c_int
                  }
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size =
                      strlen((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space)
                        as libc::c_int
                  }
                  pattern_space = xrealloc(
                    pattern_space as *mut libc::c_void,
                    (pattern_space_size + hold_space_size) as size_t,
                  ) as *mut libc::c_char;
                  if pattern_space_size == 2i32 {
                    *pattern_space.offset(0) = 0i32 as libc::c_char
                  }
                  strcat(pattern_space, b"\n\x00" as *const u8 as *const libc::c_char);
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    strcat(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    );
                  }
                  last_gets_char = '\n' as i32 as libc::c_char;
                  current_block = 17965632435239708295;
                }
                14127364983570718321 => {
                  free(pattern_space as *mut libc::c_void);
                  pattern_space = xstrdup(
                    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                      .hold_space
                      .is_null()
                    {
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                    } else {
                      b"\x00" as *const u8 as *const libc::c_char
                    },
                  );
                  current_block = 17965632435239708295;
                }
                1739363794695357236 => {
                  let mut i: libc::c_int = 0;
                  let mut j: libc::c_int = 0;
                  i = 0i32;
                  while *pattern_space.offset(i as isize) != 0 {
                    j = 0i32;
                    while *(*sed_cmd).string.offset(j as isize) != 0 {
                      if *pattern_space.offset(i as isize) as libc::c_int
                        == *(*sed_cmd).string.offset(j as isize) as libc::c_int
                      {
                        *pattern_space.offset(i as isize) =
                          *(*sed_cmd).string.offset((j + 1i32) as isize);
                        break;
                      } else {
                        j += 2i32
                      }
                    }
                    i += 1
                  }
                  current_block = 17965632435239708295;
                }
                17611301398573303352 => {
                  if substituted == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted = 0i32;
                    current_block = 13861430101487131366;
                  }
                }
                7297078374430259003 => {
                  let mut len: libc::c_int = 0;
                  if next_line.is_null() {
                    current_block = 8563197331115798083;
                    break 's_54;
                  } else {
                    len = strlen(pattern_space) as libc::c_int;
                    pattern_space = xrealloc(
                      pattern_space as *mut libc::c_void,
                      (len as libc::c_ulong)
                        .wrapping_add(strlen(next_line))
                        .wrapping_add(2i32 as libc::c_ulong),
                    ) as *mut libc::c_char;
                    *pattern_space.offset(len as isize) = '\n' as i32 as libc::c_char;
                    strcpy(pattern_space.offset(len as isize).offset(1), next_line);
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                1534452661327725232 => {
                  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).be_quiet == 0 {
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      last_gets_char,
                    );
                  }
                  if next_line.is_null() {
                    current_block = 4142149688065477410;
                    break 's_54;
                  } else {
                    free(pattern_space as *mut libc::c_void);
                    pattern_space = next_line;
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    substituted = 0i32;
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                726525485109251713 => {
                  let mut rfile: *mut FILE = 0 as *mut FILE;
                  rfile = fopen_for_read((*sed_cmd).string);
                  if !rfile.is_null() {
                    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
                    loop {
                      line = xmalloc_fgetline(rfile);
                      if line.is_null() {
                        break;
                      }
                      append(line);
                    }
                    fclose(rfile);
                  }
                  current_block = 17965632435239708295;
                }
                13714184482889841412 => {
                  if do_subst_command(sed_cmd, &mut pattern_space) == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted |= 1i32;
                    if (*sed_cmd).sub_p() != 0 {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        last_gets_char,
                      );
                    }
                    if !(*sed_cmd).sw_file.is_null() {
                      puts_maybe_newline(
                        pattern_space,
                        (*sed_cmd).sw_file,
                        &mut (*sed_cmd).sw_last_char,
                        last_gets_char,
                      );
                    }
                    current_block = 17965632435239708295;
                  }
                }
                2290177392965769716 => {
                  let mut tmp: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp.is_null() {
                    *tmp = '\u{0}' as i32 as libc::c_char;
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                    *tmp = '\n' as i32 as libc::c_char;
                    current_block = 17965632435239708295;
                  } else {
                    current_block = 7330787439563864000;
                  }
                }
                3546145585875536353 => {
                  let mut tmp_0: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp_0.is_null() {
                    overlapping_strcpy(pattern_space, tmp_0.offset(1));
                    break;
                  } else {
                    current_block = 4142149688065477410;
                    break 's_54;
                  }
                }
                10499578865704240760 => {
                  free(next_line as *mut libc::c_void);
                  next_line = 0 as *mut libc::c_char;
                  current_block = 8563197331115798083;
                  break 's_54;
                }
                13665239467142187023 => {
                  let mut tmp_1: *mut libc::c_char = pattern_space;
                  pattern_space = if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void
                  } else {
                    xzalloc(1i32 as size_t)
                  } as *mut libc::c_char;
                  last_gets_char = '\n' as i32 as libc::c_char;
                  let ref mut fresh28 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh28 = tmp_1;
                  current_block = 17965632435239708295;
                }
                332836909292088582 => {
                  fprintf(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    b"%d\n\x00" as *const u8 as *const libc::c_char,
                    linenum,
                  );
                  current_block = 17965632435239708295;
                }
                17152203569385922329 => {
                  append(xstrdup((*sed_cmd).string));
                  current_block = 17965632435239708295;
                }
                14487425527653873875 => {
                  puts_maybe_newline(
                    (*sed_cmd).string,
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    &mut last_puts_char,
                    '\n' as i32 as libc::c_char,
                  );
                  current_block = 17965632435239708295;
                }
                2782169280456925021 => {
                  if (*sed_cmd).in_match() == 0 {
                    puts_maybe_newline(
                      (*sed_cmd).string,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                  }
                  current_block = 4142149688065477410;
                  break 's_54;
                }
                5622786547828025790 => {
                  puts_maybe_newline(
                    pattern_space,
                    (*sed_cmd).sw_file,
                    &mut (*sed_cmd).sw_last_char,
                    last_gets_char,
                  );
                  current_block = 17965632435239708295;
                }
                _ => {}
              }
              match current_block {
                17965632435239708295 => {}
                _ => {
                  match current_block {
                    7330787439563864000 => {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        '\n' as i32 as libc::c_char,
                      );
                      current_block = 17965632435239708295;
                    }
                    13861430101487131366 => {
                      if substituted != 0 {
                        current_block = 17965632435239708295;
                      } else {
                        current_block = 2887315643959147419;
                      }
                    }
                    _ => {}
                  }
                  match current_block {
                    17965632435239708295 => {}
                    _ => {
                      if (*sed_cmd).string.is_null() {
                        current_block = 8563197331115798083;
                        break 's_54;
                      }
                      sed_cmd = branch_to((*sed_cmd).string)
                    }
                  }
                }
              }
            }
            72 => {
              current_block = 11227437541145425351;
              match current_block {
                11227437541145425351 => {
                  let mut hold_space_size_0: libc::c_int = 2i32;
                  let mut pattern_space_size_0: libc::c_int = 0i32;
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size_0 = (hold_space_size_0 as libc::c_ulong).wrapping_add(strlen(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    )) as libc::c_int as libc::c_int
                  }
                  if !pattern_space.is_null() {
                    pattern_space_size_0 = strlen(pattern_space) as libc::c_int
                  }
                  let ref mut fresh27 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh27 = xrealloc(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                    (hold_space_size_0 + pattern_space_size_0) as size_t,
                  ) as *mut libc::c_char;
                  if hold_space_size_0 == 2i32 {
                    *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space =
                      0i32 as libc::c_char
                  }
                  strcat(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    b"\n\x00" as *const u8 as *const libc::c_char,
                  );
                  if !pattern_space.is_null() {
                    strcat(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                      pattern_space,
                    );
                  }
                  current_block = 17965632435239708295;
                }
                17838047945882858323 => {
                  free(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                  );
                  let ref mut fresh26 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh26 = xstrdup(pattern_space);
                  current_block = 17965632435239708295;
                }
                10468276026569382870 => {
                  let mut pattern_space_size: libc::c_int = 2i32;
                  let mut hold_space_size: libc::c_int = 0i32;
                  if !pattern_space.is_null() {
                    pattern_space_size = (pattern_space_size as libc::c_ulong)
                      .wrapping_add(strlen(pattern_space))
                      as libc::c_int as libc::c_int
                  }
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size =
                      strlen((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space)
                        as libc::c_int
                  }
                  pattern_space = xrealloc(
                    pattern_space as *mut libc::c_void,
                    (pattern_space_size + hold_space_size) as size_t,
                  ) as *mut libc::c_char;
                  if pattern_space_size == 2i32 {
                    *pattern_space.offset(0) = 0i32 as libc::c_char
                  }
                  strcat(pattern_space, b"\n\x00" as *const u8 as *const libc::c_char);
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    strcat(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    );
                  }
                  last_gets_char = '\n' as i32 as libc::c_char;
                  current_block = 17965632435239708295;
                }
                14127364983570718321 => {
                  free(pattern_space as *mut libc::c_void);
                  pattern_space = xstrdup(
                    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                      .hold_space
                      .is_null()
                    {
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                    } else {
                      b"\x00" as *const u8 as *const libc::c_char
                    },
                  );
                  current_block = 17965632435239708295;
                }
                1739363794695357236 => {
                  let mut i: libc::c_int = 0;
                  let mut j: libc::c_int = 0;
                  i = 0i32;
                  while *pattern_space.offset(i as isize) != 0 {
                    j = 0i32;
                    while *(*sed_cmd).string.offset(j as isize) != 0 {
                      if *pattern_space.offset(i as isize) as libc::c_int
                        == *(*sed_cmd).string.offset(j as isize) as libc::c_int
                      {
                        *pattern_space.offset(i as isize) =
                          *(*sed_cmd).string.offset((j + 1i32) as isize);
                        break;
                      } else {
                        j += 2i32
                      }
                    }
                    i += 1
                  }
                  current_block = 17965632435239708295;
                }
                17611301398573303352 => {
                  if substituted == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted = 0i32;
                    current_block = 13861430101487131366;
                  }
                }
                7297078374430259003 => {
                  let mut len: libc::c_int = 0;
                  if next_line.is_null() {
                    current_block = 8563197331115798083;
                    break 's_54;
                  } else {
                    len = strlen(pattern_space) as libc::c_int;
                    pattern_space = xrealloc(
                      pattern_space as *mut libc::c_void,
                      (len as libc::c_ulong)
                        .wrapping_add(strlen(next_line))
                        .wrapping_add(2i32 as libc::c_ulong),
                    ) as *mut libc::c_char;
                    *pattern_space.offset(len as isize) = '\n' as i32 as libc::c_char;
                    strcpy(pattern_space.offset(len as isize).offset(1), next_line);
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                1534452661327725232 => {
                  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).be_quiet == 0 {
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      last_gets_char,
                    );
                  }
                  if next_line.is_null() {
                    current_block = 4142149688065477410;
                    break 's_54;
                  } else {
                    free(pattern_space as *mut libc::c_void);
                    pattern_space = next_line;
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    substituted = 0i32;
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                726525485109251713 => {
                  let mut rfile: *mut FILE = 0 as *mut FILE;
                  rfile = fopen_for_read((*sed_cmd).string);
                  if !rfile.is_null() {
                    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
                    loop {
                      line = xmalloc_fgetline(rfile);
                      if line.is_null() {
                        break;
                      }
                      append(line);
                    }
                    fclose(rfile);
                  }
                  current_block = 17965632435239708295;
                }
                13714184482889841412 => {
                  if do_subst_command(sed_cmd, &mut pattern_space) == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted |= 1i32;
                    if (*sed_cmd).sub_p() != 0 {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        last_gets_char,
                      );
                    }
                    if !(*sed_cmd).sw_file.is_null() {
                      puts_maybe_newline(
                        pattern_space,
                        (*sed_cmd).sw_file,
                        &mut (*sed_cmd).sw_last_char,
                        last_gets_char,
                      );
                    }
                    current_block = 17965632435239708295;
                  }
                }
                2290177392965769716 => {
                  let mut tmp: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp.is_null() {
                    *tmp = '\u{0}' as i32 as libc::c_char;
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                    *tmp = '\n' as i32 as libc::c_char;
                    current_block = 17965632435239708295;
                  } else {
                    current_block = 7330787439563864000;
                  }
                }
                3546145585875536353 => {
                  let mut tmp_0: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp_0.is_null() {
                    overlapping_strcpy(pattern_space, tmp_0.offset(1));
                    break;
                  } else {
                    current_block = 4142149688065477410;
                    break 's_54;
                  }
                }
                10499578865704240760 => {
                  free(next_line as *mut libc::c_void);
                  next_line = 0 as *mut libc::c_char;
                  current_block = 8563197331115798083;
                  break 's_54;
                }
                13665239467142187023 => {
                  let mut tmp_1: *mut libc::c_char = pattern_space;
                  pattern_space = if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void
                  } else {
                    xzalloc(1i32 as size_t)
                  } as *mut libc::c_char;
                  last_gets_char = '\n' as i32 as libc::c_char;
                  let ref mut fresh28 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh28 = tmp_1;
                  current_block = 17965632435239708295;
                }
                332836909292088582 => {
                  fprintf(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    b"%d\n\x00" as *const u8 as *const libc::c_char,
                    linenum,
                  );
                  current_block = 17965632435239708295;
                }
                17152203569385922329 => {
                  append(xstrdup((*sed_cmd).string));
                  current_block = 17965632435239708295;
                }
                14487425527653873875 => {
                  puts_maybe_newline(
                    (*sed_cmd).string,
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    &mut last_puts_char,
                    '\n' as i32 as libc::c_char,
                  );
                  current_block = 17965632435239708295;
                }
                2782169280456925021 => {
                  if (*sed_cmd).in_match() == 0 {
                    puts_maybe_newline(
                      (*sed_cmd).string,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                  }
                  current_block = 4142149688065477410;
                  break 's_54;
                }
                5622786547828025790 => {
                  puts_maybe_newline(
                    pattern_space,
                    (*sed_cmd).sw_file,
                    &mut (*sed_cmd).sw_last_char,
                    last_gets_char,
                  );
                  current_block = 17965632435239708295;
                }
                _ => {}
              }
              match current_block {
                17965632435239708295 => {}
                _ => {
                  match current_block {
                    7330787439563864000 => {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        '\n' as i32 as libc::c_char,
                      );
                      current_block = 17965632435239708295;
                    }
                    13861430101487131366 => {
                      if substituted != 0 {
                        current_block = 17965632435239708295;
                      } else {
                        current_block = 2887315643959147419;
                      }
                    }
                    _ => {}
                  }
                  match current_block {
                    17965632435239708295 => {}
                    _ => {
                      if (*sed_cmd).string.is_null() {
                        current_block = 8563197331115798083;
                        break 's_54;
                      }
                      sed_cmd = branch_to((*sed_cmd).string)
                    }
                  }
                }
              }
            }
            120 => {
              current_block = 13665239467142187023;
              match current_block {
                11227437541145425351 => {
                  let mut hold_space_size_0: libc::c_int = 2i32;
                  let mut pattern_space_size_0: libc::c_int = 0i32;
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size_0 = (hold_space_size_0 as libc::c_ulong).wrapping_add(strlen(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    )) as libc::c_int as libc::c_int
                  }
                  if !pattern_space.is_null() {
                    pattern_space_size_0 = strlen(pattern_space) as libc::c_int
                  }
                  let ref mut fresh27 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh27 = xrealloc(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                    (hold_space_size_0 + pattern_space_size_0) as size_t,
                  ) as *mut libc::c_char;
                  if hold_space_size_0 == 2i32 {
                    *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space =
                      0i32 as libc::c_char
                  }
                  strcat(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    b"\n\x00" as *const u8 as *const libc::c_char,
                  );
                  if !pattern_space.is_null() {
                    strcat(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                      pattern_space,
                    );
                  }
                  current_block = 17965632435239708295;
                }
                17838047945882858323 => {
                  free(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void,
                  );
                  let ref mut fresh26 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh26 = xstrdup(pattern_space);
                  current_block = 17965632435239708295;
                }
                10468276026569382870 => {
                  let mut pattern_space_size: libc::c_int = 2i32;
                  let mut hold_space_size: libc::c_int = 0i32;
                  if !pattern_space.is_null() {
                    pattern_space_size = (pattern_space_size as libc::c_ulong)
                      .wrapping_add(strlen(pattern_space))
                      as libc::c_int as libc::c_int
                  }
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    hold_space_size =
                      strlen((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space)
                        as libc::c_int
                  }
                  pattern_space = xrealloc(
                    pattern_space as *mut libc::c_void,
                    (pattern_space_size + hold_space_size) as size_t,
                  ) as *mut libc::c_char;
                  if pattern_space_size == 2i32 {
                    *pattern_space.offset(0) = 0i32 as libc::c_char
                  }
                  strcat(pattern_space, b"\n\x00" as *const u8 as *const libc::c_char);
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    strcat(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space,
                    );
                  }
                  last_gets_char = '\n' as i32 as libc::c_char;
                  current_block = 17965632435239708295;
                }
                14127364983570718321 => {
                  free(pattern_space as *mut libc::c_void);
                  pattern_space = xstrdup(
                    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                      .hold_space
                      .is_null()
                    {
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                    } else {
                      b"\x00" as *const u8 as *const libc::c_char
                    },
                  );
                  current_block = 17965632435239708295;
                }
                1739363794695357236 => {
                  let mut i: libc::c_int = 0;
                  let mut j: libc::c_int = 0;
                  i = 0i32;
                  while *pattern_space.offset(i as isize) != 0 {
                    j = 0i32;
                    while *(*sed_cmd).string.offset(j as isize) != 0 {
                      if *pattern_space.offset(i as isize) as libc::c_int
                        == *(*sed_cmd).string.offset(j as isize) as libc::c_int
                      {
                        *pattern_space.offset(i as isize) =
                          *(*sed_cmd).string.offset((j + 1i32) as isize);
                        break;
                      } else {
                        j += 2i32
                      }
                    }
                    i += 1
                  }
                  current_block = 17965632435239708295;
                }
                17611301398573303352 => {
                  if substituted == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted = 0i32;
                    current_block = 13861430101487131366;
                  }
                }
                7297078374430259003 => {
                  let mut len: libc::c_int = 0;
                  if next_line.is_null() {
                    current_block = 8563197331115798083;
                    break 's_54;
                  } else {
                    len = strlen(pattern_space) as libc::c_int;
                    pattern_space = xrealloc(
                      pattern_space as *mut libc::c_void,
                      (len as libc::c_ulong)
                        .wrapping_add(strlen(next_line))
                        .wrapping_add(2i32 as libc::c_ulong),
                    ) as *mut libc::c_char;
                    *pattern_space.offset(len as isize) = '\n' as i32 as libc::c_char;
                    strcpy(pattern_space.offset(len as isize).offset(1), next_line);
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                1534452661327725232 => {
                  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).be_quiet == 0 {
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      last_gets_char,
                    );
                  }
                  if next_line.is_null() {
                    current_block = 4142149688065477410;
                    break 's_54;
                  } else {
                    free(pattern_space as *mut libc::c_void);
                    pattern_space = next_line;
                    last_gets_char = next_gets_char;
                    next_line = get_next_line(&mut next_gets_char, &mut last_puts_char);
                    substituted = 0i32;
                    linenum += 1
                  }
                  current_block = 17965632435239708295;
                }
                726525485109251713 => {
                  let mut rfile: *mut FILE = 0 as *mut FILE;
                  rfile = fopen_for_read((*sed_cmd).string);
                  if !rfile.is_null() {
                    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
                    loop {
                      line = xmalloc_fgetline(rfile);
                      if line.is_null() {
                        break;
                      }
                      append(line);
                    }
                    fclose(rfile);
                  }
                  current_block = 17965632435239708295;
                }
                13714184482889841412 => {
                  if do_subst_command(sed_cmd, &mut pattern_space) == 0 {
                    current_block = 17965632435239708295;
                  } else {
                    substituted |= 1i32;
                    if (*sed_cmd).sub_p() != 0 {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        last_gets_char,
                      );
                    }
                    if !(*sed_cmd).sw_file.is_null() {
                      puts_maybe_newline(
                        pattern_space,
                        (*sed_cmd).sw_file,
                        &mut (*sed_cmd).sw_last_char,
                        last_gets_char,
                      );
                    }
                    current_block = 17965632435239708295;
                  }
                }
                2290177392965769716 => {
                  let mut tmp: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp.is_null() {
                    *tmp = '\u{0}' as i32 as libc::c_char;
                    puts_maybe_newline(
                      pattern_space,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                    *tmp = '\n' as i32 as libc::c_char;
                    current_block = 17965632435239708295;
                  } else {
                    current_block = 7330787439563864000;
                  }
                }
                3546145585875536353 => {
                  let mut tmp_0: *mut libc::c_char = strchr(pattern_space, '\n' as i32);
                  if !tmp_0.is_null() {
                    overlapping_strcpy(pattern_space, tmp_0.offset(1));
                    break;
                  } else {
                    current_block = 4142149688065477410;
                    break 's_54;
                  }
                }
                10499578865704240760 => {
                  free(next_line as *mut libc::c_void);
                  next_line = 0 as *mut libc::c_char;
                  current_block = 8563197331115798083;
                  break 's_54;
                }
                13665239467142187023 => {
                  let mut tmp_1: *mut libc::c_char = pattern_space;
                  pattern_space = if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .hold_space
                    .is_null()
                  {
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space
                      as *mut libc::c_void
                  } else {
                    xzalloc(1i32 as size_t)
                  } as *mut libc::c_char;
                  last_gets_char = '\n' as i32 as libc::c_char;
                  let ref mut fresh28 =
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hold_space;
                  *fresh28 = tmp_1;
                  current_block = 17965632435239708295;
                }
                332836909292088582 => {
                  fprintf(
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    b"%d\n\x00" as *const u8 as *const libc::c_char,
                    linenum,
                  );
                  current_block = 17965632435239708295;
                }
                17152203569385922329 => {
                  append(xstrdup((*sed_cmd).string));
                  current_block = 17965632435239708295;
                }
                14487425527653873875 => {
                  puts_maybe_newline(
                    (*sed_cmd).string,
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                    &mut last_puts_char,
                    '\n' as i32 as libc::c_char,
                  );
                  current_block = 17965632435239708295;
                }
                2782169280456925021 => {
                  if (*sed_cmd).in_match() == 0 {
                    puts_maybe_newline(
                      (*sed_cmd).string,
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                      &mut last_puts_char,
                      '\n' as i32 as libc::c_char,
                    );
                  }
                  current_block = 4142149688065477410;
                  break 's_54;
                }
                5622786547828025790 => {
                  puts_maybe_newline(
                    pattern_space,
                    (*sed_cmd).sw_file,
                    &mut (*sed_cmd).sw_last_char,
                    last_gets_char,
                  );
                  current_block = 17965632435239708295;
                }
                _ => {}
              }
              match current_block {
                17965632435239708295 => {}
                _ => {
                  match current_block {
                    7330787439563864000 => {
                      puts_maybe_newline(
                        pattern_space,
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
                        &mut last_puts_char,
                        '\n' as i32 as libc::c_char,
                      );
                      current_block = 17965632435239708295;
                    }
                    13861430101487131366 => {
                      if substituted != 0 {
                        current_block = 17965632435239708295;
                      } else {
                        current_block = 2887315643959147419;
                      }
                    }
                    _ => {}
                  }
                  match current_block {
                    17965632435239708295 => {}
                    _ => {
                      if (*sed_cmd).string.is_null() {
                        current_block = 8563197331115798083;
                        break 's_54;
                      }
                      sed_cmd = branch_to((*sed_cmd).string)
                    }
                  }
                }
              }
            }
            _ => {}
          }
        }
        sed_cmd = (*sed_cmd).next
      }
    }
    match current_block {
      8563197331115798083 =>
      /*
      * Exit point from sedding...
      */
               /* we will print the line unless we were told to be quiet ('-n')
        or if the line was suppressed (ala 'd'elete) */
      {
        if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).be_quiet == 0 {
          puts_maybe_newline(
            pattern_space,
            (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout,
            &mut last_puts_char,
            last_gets_char,
          );
        }
      }
      _ => {}
    }
    /* Delete and such jump here. */
    flush_append(&mut last_puts_char);
    free(pattern_space as *mut libc::c_void);
  }
}
/* It is possible to have a command line argument with embedded
 * newlines.  This counts as multiple command lines.
 * However, newline can be escaped: 's/e/z\<newline>z/'
 * add_cmd() handles this.
 */
unsafe extern "C" fn add_cmd_block(mut cmdstr: *mut libc::c_char) {
  let mut sv: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut eol: *mut libc::c_char = 0 as *mut libc::c_char;
  sv = xstrdup(cmdstr);
  cmdstr = sv;
  loop {
    eol = strchr(cmdstr, '\n' as i32);
    if !eol.is_null() {
      *eol = '\u{0}' as i32 as libc::c_char
    }
    add_cmd(cmdstr);
    cmdstr = eol.offset(1);
    if eol.is_null() {
      break;
    }
  }
  free(sv as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn sed_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opt: libc::c_uint = 0;
  let mut opt_e: *mut llist_t = 0 as *mut llist_t;
  let mut opt_f: *mut llist_t = 0 as *mut llist_t;
  let mut opt_i: *mut libc::c_char = 0 as *mut libc::c_char;
  static mut sed_longopts: [libc::c_char; 67] = [
    105, 110, 45, 112, 108, 97, 99, 101, 0, 2, 105, 114, 101, 103, 101, 120, 112, 45, 101, 120,
    116, 101, 110, 100, 101, 100, 0, 0, 114, 113, 117, 105, 101, 116, 0, 0, 110, 115, 105, 108,
    101, 110, 116, 0, 0, 110, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 0, 1, 101, 102,
    105, 108, 101, 0, 1, 102, 0,
  ];
  let ref mut fresh29 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).sed_cmd_tail;
  *fresh29 = &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).sed_cmd_head;
  /* destroy command strings on exit */
  /* Lie to autoconf when it starts asking stupid questions. */
  if !(*argv.offset(1)).is_null()
    && strcmp(
      *argv.offset(1),
      b"--version\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
  {
    puts(b"This is not GNU sed version 4.0\x00" as *const u8 as *const libc::c_char);
    return 0i32;
  }
  /* do normal option parsing */
  opt_f = 0 as *mut llist_t;
  opt_e = opt_f;
  opt_i = 0 as *mut libc::c_char;
  /* -i must be first, to match OPT_in_place definition */
  /* -E is a synonym of -r:
   * GNU sed 4.2.1 mentions it in neither --help
   * nor manpage, but does recognize it.
   */
  opt = getopt32long(
    argv,
    b"^i::rEne:*f:*\x00nn\x00" as *const u8 as *const libc::c_char,
    sed_longopts.as_ptr(),
    &mut opt_i as *mut *mut libc::c_char,
    &mut opt_e as *mut *mut llist_t,
    &mut opt_f as *mut *mut llist_t,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).be_quiet as *mut libc::c_int,
  ); /* counter for -n */
  //argc -= optind;
  argv = argv.offset(optind as isize);
  if opt & OPT_in_place as libc::c_int as libc::c_uint != 0 {
    // -i
    die_func = Some(cleanup_outname as unsafe extern "C" fn() -> ())
  } // -r or -E
  if opt & (2i32 | 4i32) as libc::c_uint != 0 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).regex_type |= 1i32
  }
  //if (opt & 8)
  //	G.be_quiet++; // -n (implemented with a counter instead)
  while !opt_e.is_null() {
    // -e
    add_cmd_block(llist_pop(&mut opt_e) as *mut libc::c_char);
  }
  while !opt_f.is_null() {
    // -f
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmdfile: *mut FILE = 0 as *mut FILE;
    cmdfile = xfopen_stdin(llist_pop(&mut opt_f) as *const libc::c_char);
    loop {
      line = xmalloc_fgetline(cmdfile);
      if line.is_null() {
        break;
      }
      add_cmd(line);
      free(line as *mut libc::c_void);
    }
    fclose_if_not_stdin(cmdfile);
  }
  /* if we didn't get a pattern from -e or -f, use argv[0] */
  if opt & 0x30i32 as libc::c_uint == 0 {
    if (*argv).is_null() {
      bb_show_usage();
    }
    let fresh30 = argv;
    argv = argv.offset(1);
    add_cmd_block(*fresh30);
  }
  /* Flush any unfinished commands. */
  add_cmd(b"\x00" as *const u8 as *const libc::c_char);
  /* By default, we write to stdout */
  let ref mut fresh31 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout;
  *fresh31 = stdout;
  /* argv[0..(argc-1)] should be names of file to process. If no
   * files were specified or '-' was specified, take input from stdin.
   * Otherwise, we process all the files specified. */
  let ref mut fresh32 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).input_file_list;
  *fresh32 = argv;
  if (*argv.offset(0)).is_null() {
    if opt & OPT_in_place as libc::c_int as libc::c_uint != 0 {
      bb_error_msg_and_die(
        bb_msg_requires_arg.as_ptr(),
        b"-i\x00" as *const u8 as *const libc::c_char,
      );
    }
    let ref mut fresh33 = *argv.offset(0);
    *fresh33 = bb_msg_standard_input.as_ptr() as *mut libc::c_char
  /* G.last_input_file = 0; - already is */
  } else {
    let mut statbuf: stat = stat {
      st_dev: 0,
      st_ino: 0,
      st_nlink: 0,
      st_mode: 0,
      st_uid: 0,
      st_gid: 0,
      __pad0: 0,
      st_rdev: 0,
      st_size: 0,
      st_blksize: 0,
      st_blocks: 0,
      st_atim: timespec {
        tv_sec: 0,
        tv_nsec: 0,
      },
      st_mtim: timespec {
        tv_sec: 0,
        tv_nsec: 0,
      },
      st_ctim: timespec {
        tv_sec: 0,
        tv_nsec: 0,
      },
      __glibc_reserved: [0; 3],
    };
    let mut nonstdoutfd: libc::c_int = 0;
    let mut sed_cmd: *mut sed_cmd_t = 0 as *mut sed_cmd_t;
    loop {
      if opt & OPT_in_place as libc::c_int as libc::c_uint == 0 {
        if *(*argv).offset(0) as libc::c_int == '-' as i32 && *(*argv).offset(1) == 0 {
          *argv = bb_msg_standard_input.as_ptr() as *mut libc::c_char;
          process_files();
        }
      } else if stat(*argv, &mut statbuf) != 0i32 {
        bb_simple_perror_msg(*argv);
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).exitcode = 1i32 as smallint;
        let ref mut fresh35 =
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).current_input_file;
        *fresh35 += 1
      } else {
        let ref mut fresh36 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).outname;
        *fresh36 = xasprintf(b"%sXXXXXX\x00" as *const u8 as *const libc::c_char, *argv);
        nonstdoutfd = xmkstemp((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).outname);
        let ref mut fresh37 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout;
        *fresh37 = xfdopen_for_write(nonstdoutfd);
        /* -i: process each FILE separately: */
        /* Set permissions/owner of output file */
        /* chmod'ing AFTER chown would preserve suid/sgid bits,
         * but GNU sed 4.2.1 does not preserve them either */
        fchmod(nonstdoutfd, statbuf.st_mode);
        fchown(nonstdoutfd, statbuf.st_uid, statbuf.st_gid);
        process_files();
        fclose((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout);
        let ref mut fresh38 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nonstdout;
        *fresh38 = stdout;
        if !opt_i.is_null() {
          let mut backupname: *mut libc::c_char = xasprintf(
            b"%s%s\x00" as *const u8 as *const libc::c_char,
            *argv,
            opt_i,
          );
          xrename(*argv, backupname);
          free(backupname as *mut libc::c_void);
        }
        /* else unlink(*argv); - rename below does this */
        xrename(
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).outname,
          *argv,
        ); //TODO: rollback backup on error?
        free((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).outname as *mut libc::c_void);
        let ref mut fresh39 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).outname;
        *fresh39 = 0 as *mut libc::c_char;
        /* Fix disabled range matches and mangled ",+N" ranges */
        sed_cmd = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).sed_cmd_head;
        while !sed_cmd.is_null() {
          (*sed_cmd).beg_line = (*sed_cmd).beg_line_orig;
          (*sed_cmd).end_line = (*sed_cmd).end_line_orig;
          sed_cmd = (*sed_cmd).next
        }
      }
      argv = argv.offset(1);
      if (*argv).is_null() {
        break;
      }
      statbuf = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
          tv_sec: 0,
          tv_nsec: 0,
        },
        st_mtim: timespec {
          tv_sec: 0,
          tv_nsec: 0,
        },
        st_ctim: timespec {
          tv_sec: 0,
          tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
      };
      nonstdoutfd = 0;
      sed_cmd = 0 as *mut sed_cmd_t;
      let ref mut fresh34 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).last_input_file;
      *fresh34 += 1
      /* Here, to handle "sed 'cmds' nonexistent_file" case we did:
       * if (G.current_input_file[G.current_input_file] == NULL)
       *	return G.exitcode;
       * but it's not needed since process_files() works correctly
       * in this case too. */
    }
  }
  process_files();
  return (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).exitcode as libc::c_int;
}

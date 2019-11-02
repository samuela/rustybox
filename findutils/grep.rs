use c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn exit(_: libc::c_int) -> !;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut stdin: *mut _IO_FILE;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strcasestr(
    __haystack: *const libc::c_char,
    __needle: *const libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn recursive_action(
    fileName: *const libc::c_char,
    flags: libc::c_uint,
    fileAction: Option<
      unsafe extern "C" fn(
        _: *const libc::c_char,
        _: *mut stat,
        _: *mut libc::c_void,
        _: libc::c_int,
      ) -> libc::c_int,
    >,
    dirAction: Option<
      unsafe extern "C" fn(
        _: *const libc::c_char,
        _: *mut stat,
        _: *mut libc::c_void,
        _: libc::c_int,
      ) -> libc::c_int,
    >,
    userData: *mut libc::c_void,
    depth: libc::c_uint,
  ) -> libc::c_int;
  #[no_mangle]
  fn xmalloc_fgetline(file: *mut FILE) -> *mut libc::c_char;
  #[no_mangle]
  fn fclose_if_not_stdin(file: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn xfopen_stdin(filename: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  static mut option_mask32: uint32_t;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> uint32_t;
  #[no_mangle]
  fn llist_add_to(old_head: *mut *mut llist_t, data: *mut libc::c_void);
  #[no_mangle]
  static mut xfunc_error_retval: uint8_t;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
  #[no_mangle]
  fn regfree(__preg: *mut regex_t);
  #[no_mangle]
  fn xregcomp(preg: *mut regex_t, regex: *const libc::c_char, cflags: libc::c_int);
  #[no_mangle]
  fn regexec(
    __preg: *const regex_t,
    __string: *const libc::c_char,
    __nmatch: size_t,
    __pmatch: *mut regmatch_t,
    __eflags: libc::c_int,
  ) -> libc::c_int;
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
/* vi: set sw=4 ts=4: */
/*
 * Copyright 2006, Bernhard Reutner-Fischer
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Convenience macros to test the version of gcc. */
/* __restrict is known in EGCS 1.2 and above. */
/* "The malloc attribute is used to tell the compiler that a function
 * may be treated as if any non-NULL pointer it returns cannot alias
 * any other pointer valid when the function returns. This will often
 * improve optimization. Standard functions with this property include
 * malloc and calloc. realloc-like functions have this property as long
 * as the old pointer is never referred to (including comparing it
 * to the new pointer) after the function returns a non-NULL value."
 */
/* __NO_INLINE__: some gcc's do not honor inlining! :( */
/* I've seen a toolchain where I needed __noinline__ instead of noinline */
/* used by unit test machinery to run registration functions before calling main() */
/* -fwhole-program makes all symbols local. The attribute externally_visible
 * forces a symbol global.  */
//__attribute__ ((__externally_visible__))
/* At 4.4 gcc become much more anal about this, need to use "aliased" types */
/* We use __extension__ in some places to suppress -pedantic warnings
 * about GCC extensions.  This feature didn't work properly before
 * gcc 2.8.  */
/* FAST_FUNC is a qualifier which (possibly) makes function call faster
 * and/or smaller by using modified ABI. It is usually only needed
 * on non-static, busybox internal functions. Recent versions of gcc
 * optimize statics automatically. FAST_FUNC on static is required
 * only if you need to match a function pointer's type.
 * FAST_FUNC may not work well with -flto so allow user to disable this.
 * (-DFAST_FUNC= )
 */
/* Make all declarations hidden (-fvisibility flag only affects definitions) */
/* (don't include system headers after this until corresponding pop!) */
/* gcc-2.95 had no va_copy but only __va_copy. */
/* ---- Endian Detection ------------------------------------ */
/* SWAP_LEnn means "convert CPU<->little_endian by swapping bytes" */
/* ---- Unaligned access ------------------------------------ */
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
pub type smalluint = libc::c_uchar;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
  pub tv_sec: __time_t,
  pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
  pub st_dev: __dev_t,
  pub st_ino: __ino_t,
  pub st_nlink: __nlink_t,
  pub st_mode: __mode_t,
  pub st_uid: __uid_t,
  pub st_gid: __gid_t,
  pub __pad0: libc::c_int,
  pub st_rdev: __dev_t,
  pub st_size: __off_t,
  pub st_blksize: __blksize_t,
  pub st_blocks: __blkcnt_t,
  pub st_atim: timespec,
  pub st_mtim: timespec,
  pub st_ctim: timespec,
  pub __glibc_reserved: [__syscall_slong_t; 3],
}
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
pub type C2RustUnnamed = libc::c_uint;
pub const ACTION_DANGLING_OK: C2RustUnnamed = 64;
pub const ACTION_QUIET: C2RustUnnamed = 32;
pub const ACTION_DEPTHFIRST: C2RustUnnamed = 8;
pub const ACTION_FOLLOWLINKS_L0: C2RustUnnamed = 4;
pub const ACTION_FOLLOWLINKS: C2RustUnnamed = 2;
pub const ACTION_RECURSE: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct llist_t {
  pub link: *mut llist_t,
  pub data: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub max_matches: libc::c_int,
  pub reflags: libc::c_int,
  pub invert_search: smalluint,
  pub print_filename: smalluint,
  pub open_errors: smalluint,
  pub did_print_line: smalluint,
  pub lines_before: libc::c_int,
  pub lines_after: libc::c_int,
  pub before_buf: *mut *mut libc::c_char,
  pub last_line_printed: libc::c_int,
  pub pattern_head: *mut llist_t,
  pub cur_file: *const libc::c_char,
}
pub type reg_syntax_t = libc::c_ulong;
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
pub type regex_t = re_pattern_buffer;
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
  pub rm_so: regoff_t,
  pub rm_eo: regoff_t,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const OPT_z: C2RustUnnamed_0 = 0;
pub const OPT_E: C2RustUnnamed_0 = 2097152;
pub const OPT_C: C2RustUnnamed_0 = 1048576;
pub const OPT_B: C2RustUnnamed_0 = 524288;
pub const OPT_A: C2RustUnnamed_0 = 262144;
pub const OPT_x: C2RustUnnamed_0 = 131072;
pub const OPT_w: C2RustUnnamed_0 = 65536;
pub const OPT_m: C2RustUnnamed_0 = 32768;
pub const OPT_r: C2RustUnnamed_0 = 16384;
pub const OPT_o: C2RustUnnamed_0 = 8192;
pub const OPT_L: C2RustUnnamed_0 = 4096;
pub const OPT_f: C2RustUnnamed_0 = 2048;
pub const OPT_e: C2RustUnnamed_0 = 1024;
pub const OPT_h: C2RustUnnamed_0 = 512;
pub const OPT_H: C2RustUnnamed_0 = 256;
pub const OPT_i: C2RustUnnamed_0 = 128;
pub const OPT_F: C2RustUnnamed_0 = 64;
pub const OPT_c: C2RustUnnamed_0 = 32;
pub const OPT_s: C2RustUnnamed_0 = 16;
pub const OPT_v: C2RustUnnamed_0 = 8;
pub const OPT_q: C2RustUnnamed_0 = 4;
pub const OPT_n: C2RustUnnamed_0 = 2;
pub const OPT_l: C2RustUnnamed_0 = 1;
pub const OPTBIT_E: C2RustUnnamed_0 = 21;
pub const OPTBIT_C: C2RustUnnamed_0 = 20;
pub const OPTBIT_B: C2RustUnnamed_0 = 19;
pub const OPTBIT_A: C2RustUnnamed_0 = 18;
pub const OPTBIT_x: C2RustUnnamed_0 = 17;
pub const OPTBIT_w: C2RustUnnamed_0 = 16;
pub const OPTBIT_m: C2RustUnnamed_0 = 15;
pub const OPTBIT_r: C2RustUnnamed_0 = 14;
pub const OPTBIT_o: C2RustUnnamed_0 = 13;
pub const OPTBIT_L: C2RustUnnamed_0 = 12;
pub const OPTBIT_f: C2RustUnnamed_0 = 11;
pub const OPTBIT_e: C2RustUnnamed_0 = 10;
pub const OPTBIT_h: C2RustUnnamed_0 = 9;
pub const OPTBIT_H: C2RustUnnamed_0 = 8;
pub const OPTBIT_i: C2RustUnnamed_0 = 7;
pub const OPTBIT_F: C2RustUnnamed_0 = 6;
pub const OPTBIT_c: C2RustUnnamed_0 = 5;
pub const OPTBIT_s: C2RustUnnamed_0 = 4;
pub const OPTBIT_v: C2RustUnnamed_0 = 3;
pub const OPTBIT_q: C2RustUnnamed_0 = 2;
pub const OPTBIT_n: C2RustUnnamed_0 = 1;
pub const OPTBIT_l: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct grep_list_data_t {
  pub pattern: *mut libc::c_char,
  pub compiled_regex: regex_t,
  pub matched_range: regmatch_t,
  pub flg_mem_allocated_compiled: libc::c_int,
}
#[inline(always)]
unsafe extern "C" fn bb_ascii_isalnum(mut a: libc::c_uchar) -> libc::c_int {
  let mut b: libc::c_uchar = (a as libc::c_int - '0' as i32) as libc::c_uchar;
  if b as libc::c_int <= 9i32 {
    return (b as libc::c_int <= 9i32) as libc::c_int;
  }
  b = ((a as libc::c_int | 0x20i32) - 'a' as i32) as libc::c_uchar;
  return (b as libc::c_int <= 'z' as i32 - 'a' as i32) as libc::c_int;
}
unsafe extern "C" fn print_line(
  mut line: *const libc::c_char,
  mut linenum: libc::c_int,
  mut decoration: libc::c_char,
) {
  /* Happens when we go to next file, immediately hit match
   * and try to print prev context... from prev file! Don't do it */
  if linenum < 1i32 {
    return;
  }
  /* possibly print the little '--' separator */
  if ((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lines_before != 0
    || (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lines_after != 0)
    && (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).did_print_line as libc::c_int != 0
    && (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).last_line_printed != linenum - 1i32
  {
    puts(b"--\x00" as *const u8 as *const libc::c_char);
  }
  /* guard against printing "--" before first line of first file */
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).did_print_line = 1i32 as smalluint;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).last_line_printed = linenum;
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).print_filename != 0 {
    printf(
      b"%s%c\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cur_file,
      decoration as libc::c_int,
    );
  }
  if option_mask32 & OPT_n as libc::c_int as libc::c_uint != 0 {
    printf(
      b"%i%c\x00" as *const u8 as *const libc::c_char,
      linenum,
      decoration as libc::c_int,
    );
  }
  /* Emulate weird GNU grep behavior with -ov */
  if option_mask32 & (OPT_v as libc::c_int | OPT_o as libc::c_int) as libc::c_uint
    != (OPT_v as libc::c_int | OPT_o as libc::c_int) as libc::c_uint
  {
    puts(line); /* track where we are in the circular 'before' buffer */
  }; /* used for iteration through the circular buffer */
}
unsafe extern "C" fn grep_file(mut file: *mut FILE) -> libc::c_int {
  let mut current_block: u64; /* while (read line) */
  let mut found: smalluint = 0; /* for gcc */
  let mut linenum: libc::c_int = 0i32; /* while (pattern_ptr) */
  let mut nmatches: libc::c_int = 0i32;
  let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut print_n_lines_after: libc::c_int = 0i32;
  let mut curpos: libc::c_int = 0i32;
  let mut idx: libc::c_int = 0i32;
  loop {
    line = xmalloc_fgetline(file);
    if line.is_null() {
      break;
    }
    let mut pattern_ptr: *mut llist_t =
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pattern_head;
    let mut gl: *mut grep_list_data_t = 0 as *mut grep_list_data_t;
    gl = gl;
    linenum += 1;
    found = 0i32 as smalluint;
    while !pattern_ptr.is_null() {
      gl = (*pattern_ptr).data as *mut grep_list_data_t;
      if option_mask32 & OPT_F as libc::c_int as libc::c_uint != 0 {
        let mut match_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut str: *mut libc::c_char = line;
        loop {
          match_0 = if option_mask32 & OPT_i as libc::c_int as libc::c_uint != 0 {
            strcasestr(str, (*gl).pattern)
          } else {
            strstr(str, (*gl).pattern)
          };
          if match_0.is_null() {
            current_block = 9859671972921157070;
            break;
          }
          if option_mask32 & OPT_x as libc::c_int as libc::c_uint != 0 {
            if match_0 != str {
              current_block = 9859671972921157070;
              break;
            } else {
              current_block = 1054647088692577877;
              break;
            }
          } else {
            if !(option_mask32 & OPT_w as libc::c_int as libc::c_uint != 0) {
              current_block = 1768479419963508908;
              break;
            }
            let mut c: libc::c_char = if match_0 != line {
              *match_0.offset(-1i32 as isize) as libc::c_int
            } else {
              ' ' as i32
            } as libc::c_char;
            if bb_ascii_isalnum(c as libc::c_uchar) == 0 && c as libc::c_int != '_' as i32 {
              c = *match_0.offset(strlen((*gl).pattern) as isize);
              if c == 0
                || bb_ascii_isalnum(c as libc::c_uchar) == 0 && c as libc::c_int != '_' as i32
              {
                current_block = 1768479419963508908;
                break;
              }
            }
            str = match_0.offset(1)
          }
        }
        match current_block {
          9859671972921157070 => {}
          _ => {
            match current_block {
              1054647088692577877 => {
                if *str.offset(strlen((*gl).pattern) as isize) as libc::c_int != '\u{0}' as i32 {
                  current_block = 9859671972921157070;
                } else {
                  current_block = 1768479419963508908;
                }
              }
              _ => {}
            }
            match current_block {
              9859671972921157070 => {}
              _ => found = 1i32 as smalluint,
            }
          }
        }
      } else {
        let mut match_flg: libc::c_int = 0;
        let mut match_at: *mut libc::c_char = 0 as *mut libc::c_char;
        if (*gl).flg_mem_allocated_compiled & 2i32 == 0 {
          (*gl).flg_mem_allocated_compiled |= 2i32;
          xregcomp(
            &mut (*gl).compiled_regex,
            (*gl).pattern,
            (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).reflags,
          );
        }
        (*gl).matched_range.rm_so = 0i32;
        (*gl).matched_range.rm_eo = 0i32;
        match_flg = 0i32;
        match_at = line;
        //bb_error_msg("'%s' start_pos:%d line_len:%d", match_at, start_pos, line_len);
        while regexec(
          &mut (*gl).compiled_regex,
          match_at,
          1i32 as size_t,
          &mut (*gl).matched_range,
          match_flg,
        ) == 0i32
        {
          if option_mask32 & OPT_x as libc::c_int as libc::c_uint != 0 {
            found = (found as libc::c_int
              | ((*gl).matched_range.rm_so == 0i32
                && *match_at.offset((*gl).matched_range.rm_eo as isize) as libc::c_int
                  == '\u{0}' as i32) as libc::c_int) as smalluint;
            break;
          } else if option_mask32 & OPT_w as libc::c_int as libc::c_uint == 0 {
            found = 1i32 as smalluint;
            break;
          } else {
            let mut c_0: libc::c_char = ' ' as i32 as libc::c_char;
            if match_at > line || (*gl).matched_range.rm_so != 0i32 {
              c_0 = *match_at.offset(((*gl).matched_range.rm_so - 1i32) as isize)
            }
            if bb_ascii_isalnum(c_0 as libc::c_uchar) == 0 && c_0 as libc::c_int != '_' as i32 {
              c_0 = *match_at.offset((*gl).matched_range.rm_eo as isize)
            }
            if bb_ascii_isalnum(c_0 as libc::c_uchar) == 0 && c_0 as libc::c_int != '_' as i32 {
              found = 1i32 as smalluint;
              break;
            } else {
              /*
               * Why check gl->matched_range.rm_eo?
               * Zero-length match makes -w skip the line:
               * "echo foo | grep ^" prints "foo",
               * "echo foo | grep -w ^" prints nothing.
               * Without such check, we can loop forever.
               */
              if !((*gl).matched_range.rm_eo != 0i32) {
                break;
              }
              match_at = match_at.offset((*gl).matched_range.rm_eo as isize);
              match_flg |= 1i32
            }
          }
        }
      }
      /* If it's a non-inverted search, we can stop
       * at first match and report it.
       * If it's an inverted search, we can move on
       * to the next line of input, ignoring the
       * rest of the patterns.
       */
      if found != 0 {
        break;
      // this accomplishes both
      } else {
        pattern_ptr = (*pattern_ptr).link
      }
    }
    //if (invert_search)
    //	goto do_not_found;
    //goto do_found;
    if found as libc::c_int
      ^ (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).invert_search as libc::c_int
      != 0
    {
      //do_found:
      /* keep track of matches */
      nmatches += 1;
      /* quiet/print (non)matching file names only? */
      if option_mask32
        & (OPT_q as libc::c_int | OPT_l as libc::c_int | OPT_L as libc::c_int) as libc::c_uint
        != 0
      {
        free(line as *mut libc::c_void);
        if option_mask32 & OPT_q as libc::c_int as libc::c_uint != 0 {
          /* we don't need line anymore */
          /* one match */
          /* manpage says about -q:
           * "exit immediately with zero status
           * if any match is found,
           * even if errors were detected" */
          exit(0i32);
        }
        if option_mask32 & OPT_l as libc::c_int as libc::c_uint != 0 {
          puts((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cur_file);
          /* if we're just printing filenames, we stop after the first match */
          /* fall through to "return 1" */
        }
        return 1i32;
      }
      /* OPT_L aka PRINT_FILES_WITHOUT_MATCHES: return early */
      /* Were we printing context and saw next (unwanted) match? */
      if option_mask32 & OPT_m as libc::c_int as libc::c_uint != 0
        && nmatches > (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_matches
      {
        break;
      }
      /* print the matched line */
      if option_mask32 & OPT_c as libc::c_int as libc::c_uint == 0i32 as libc::c_uint {
        let mut prevpos: libc::c_int = if curpos == 0i32 {
          ((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lines_before) - 1i32
        } else {
          (curpos) - 1i32
        };
        /* if we were told to print 'before' lines and there is at least
         * one line in the circular buffer, print them */
        if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lines_before != 0
          && !(*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .before_buf
            .offset(prevpos as isize))
          .is_null()
        {
          let mut first_buf_entry_line_num: libc::c_int =
            linenum - (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lines_before;
          /* advance to the first entry in the circular buffer, and
           * figure out the line number is of the first line in the
           * buffer */
          idx = curpos;
          while (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .before_buf
            .offset(idx as isize))
          .is_null()
          {
            idx = (idx + 1i32) % (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lines_before;
            first_buf_entry_line_num += 1
          }
          /* now print each line in the buffer, clearing them as we go */
          while !(*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .before_buf
            .offset(idx as isize))
          .is_null()
          {
            print_line(
              *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                .before_buf
                .offset(idx as isize),
              first_buf_entry_line_num,
              '-' as i32 as libc::c_char,
            );
            free(
              *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                .before_buf
                .offset(idx as isize) as *mut libc::c_void,
            );
            let ref mut fresh0 = *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
              .before_buf
              .offset(idx as isize);
            *fresh0 = 0 as *mut libc::c_char;
            idx = (idx + 1i32) % (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lines_before;
            first_buf_entry_line_num += 1
          }
        }
        /* make a note that we need to print 'after' lines */
        print_n_lines_after = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lines_after;
        if option_mask32 & OPT_o as libc::c_int as libc::c_uint != 0 {
          if option_mask32 & OPT_F as libc::c_int as libc::c_uint != 0 {
            /* -Fo just prints the pattern
             * (unless -v: -Fov doesn't print anything at all) */
            if found != 0 {
              print_line((*gl).pattern, linenum, ':' as i32 as libc::c_char);
            }
          } else {
            loop {
              let mut start: libc::c_uint = (*gl).matched_range.rm_so as libc::c_uint;
              let mut end: libc::c_uint = (*gl).matched_range.rm_eo as libc::c_uint;
              let mut len: libc::c_uint = end.wrapping_sub(start);
              let mut old: libc::c_char = *line.offset(end as isize);
              *line.offset(end as isize) = '\u{0}' as i32 as libc::c_char;
              /* Empty match is not printed: try "echo test | grep -o ''" */
              if len != 0i32 as libc::c_uint {
                print_line(
                  line.offset(start as isize),
                  linenum,
                  ':' as i32 as libc::c_char,
                );
              }
              if old as libc::c_int == '\u{0}' as i32 {
                break;
              }
              *line.offset(end as isize) = old;
              if len == 0i32 as libc::c_uint {
                end = end.wrapping_add(1)
              }
              if regexec(
                &mut (*gl).compiled_regex,
                line.offset(end as isize),
                1i32 as size_t,
                &mut (*gl).matched_range,
                1i32,
              ) != 0i32
              {
                break;
              }
              (*gl).matched_range.rm_so = ((*gl).matched_range.rm_so as libc::c_uint)
                .wrapping_add(end) as regoff_t
                as regoff_t;
              (*gl).matched_range.rm_eo = ((*gl).matched_range.rm_eo as libc::c_uint)
                .wrapping_add(end) as regoff_t as regoff_t
            }
          }
        } else {
          print_line(line, linenum, ':' as i32 as libc::c_char);
        }
      }
    } else if print_n_lines_after != 0 {
      print_line(line, linenum, '-' as i32 as libc::c_char);
      print_n_lines_after -= 1
    } else if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lines_before != 0 {
      /* no match */
      //do_not_found:
      /* if we need to print some context lines after the last match, do so */
      /* Add the line to the circular 'before' buffer */
      free(
        *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .before_buf
          .offset(curpos as isize) as *mut libc::c_void,
      );
      let ref mut fresh1 = *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .before_buf
        .offset(curpos as isize);
      *fresh1 = line;
      curpos = (curpos + 1i32) % (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lines_before;
      /* avoid free(line) - we took the line */
      line = 0 as *mut libc::c_char
    }
    /* ENABLE_FEATURE_GREP_CONTEXT */
    free(line as *mut libc::c_void);
    /* Did we print all context after last requested match? */
    if option_mask32 & OPT_m as libc::c_int as libc::c_uint != 0
      && print_n_lines_after == 0
      && nmatches == (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_matches
    {
      break;
    }
  }
  /* special-case file post-processing for options where we don't print line
   * matches, just filenames and possibly match counts */
  /* grep -c: print [filename:]count, even if count is zero */
  if option_mask32 & OPT_c as libc::c_int as libc::c_uint != 0 {
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).print_filename != 0 {
      printf(
        b"%s:\x00" as *const u8 as *const libc::c_char,
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cur_file,
      );
    }
    printf(b"%d\n\x00" as *const u8 as *const libc::c_char, nmatches);
  }
  /* grep -L: print just the filename */
  if option_mask32 & OPT_L as libc::c_int as libc::c_uint != 0 {
    /* nmatches is zero, no need to check it:
     * we return 1 early if we detected a match
     * and PRINT_FILES_WITHOUT_MATCHES is set */
    puts((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cur_file);
  }
  return nmatches;
}
unsafe extern "C" fn add_grep_list_data(mut pattern: *mut libc::c_char) -> *mut libc::c_char {
  let mut gl: *mut grep_list_data_t =
    xzalloc(::std::mem::size_of::<grep_list_data_t>() as libc::c_ulong) as *mut grep_list_data_t;
  (*gl).pattern = pattern;
  /*gl->flg_mem_allocated_compiled = 0;*/
  return gl as *mut libc::c_char;
}
unsafe extern "C" fn load_regexes_from_file(mut fopt: *mut llist_t) {
  while !fopt.is_null() {
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut cur: *mut llist_t = fopt;
    let mut ffile: *mut libc::c_char = (*cur).data;
    fopt = (*cur).link;
    free(cur as *mut libc::c_void);
    fp = xfopen_stdin(ffile);
    loop {
      line = xmalloc_fgetline(fp);
      if line.is_null() {
        break;
      }
      llist_add_to(
        &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pattern_head,
        add_grep_list_data(line) as *mut libc::c_void,
      );
    }
    fclose_if_not_stdin(fp);
  }
}
unsafe extern "C" fn file_action_grep(
  mut filename: *const libc::c_char,
  mut statbuf: *mut stat,
  mut matched: *mut libc::c_void,
  mut depth: libc::c_int,
) -> libc::c_int {
  let mut file: *mut FILE = 0 as *mut FILE;
  /* If we are given a link to a directory, we should bail out now, rather
   * than trying to open the "file" and hoping getline gives us nothing,
   * since that is not portable across operating systems (FreeBSD for
   * example will return the raw directory contents). */
  if (*statbuf).st_mode & 0o170000i32 as libc::c_uint == 0o120000i32 as libc::c_uint {
    let mut sb: stat = stat {
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
    if stat(filename, &mut sb) != 0i32 {
      if option_mask32 & OPT_s as libc::c_int as libc::c_uint == 0 {
        bb_simple_perror_msg(filename);
      }
      return 0i32;
    }
    if sb.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint {
      return 1i32;
    }
  }
  file = fopen_for_read(filename);
  if file.is_null() {
    if option_mask32 & OPT_s as libc::c_int as libc::c_uint == 0 {
      bb_simple_perror_msg(filename);
    }
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).open_errors = 1i32 as smalluint;
    return 0i32;
  }
  let ref mut fresh2 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cur_file;
  *fresh2 = filename;
  *(matched as *mut libc::c_int) += grep_file(file);
  fclose(file);
  return 1i32;
}
unsafe extern "C" fn grep_dir(mut dir: *const libc::c_char) -> libc::c_int {
  let mut matched: libc::c_int = 0i32;
  recursive_action(
    dir,
    (ACTION_RECURSE as libc::c_int
      | ACTION_FOLLOWLINKS_L0 as libc::c_int
      | ACTION_DEPTHFIRST as libc::c_int) as libc::c_uint,
    Some(
      file_action_grep
        as unsafe extern "C" fn(
          _: *const libc::c_char,
          _: *mut stat,
          _: *mut libc::c_void,
          _: libc::c_int,
        ) -> libc::c_int,
    ),
    None,
    &mut matched as *mut libc::c_int as *mut libc::c_void,
    0i32 as libc::c_uint,
  );
  return matched;
}
#[no_mangle]
pub unsafe extern "C" fn grep_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut file: *mut FILE = 0 as *mut FILE;
  let mut matched: libc::c_int = 0;
  let mut fopt: *mut llist_t = 0 as *mut llist_t;
  let mut Copt: libc::c_int = 0;
  let mut opts: libc::c_int = 0;
  /* For grep, exitcode of 1 is "not found". Other errors are 2: */
  xfunc_error_retval = 2i32 as uint8_t;
  /* do normal option parsing */
  /* -H unsets -h; -C unsets -A,-B */
  opts = getopt32long(
    argv,
    b"^lnqvscFiHhe:*f:*Lorm:+wxA:+B:+C:+EaI\x00H-h:C-AB\x00" as *const u8 as *const libc::c_char,
    b"color\x00\x02\xff\x00" as *const u8 as *const libc::c_char,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pattern_head as *mut *mut llist_t,
    &mut fopt as *mut *mut llist_t,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_matches as *mut libc::c_int,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lines_after as *mut libc::c_int,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lines_before as *mut libc::c_int,
    &mut Copt as *mut libc::c_int,
    0 as *mut libc::c_void,
  ) as libc::c_int;
  if opts & OPT_C as libc::c_int != 0 {
    /* -C unsets prev -A and -B, but following -A or -B
     * may override it */
    if opts & OPT_A as libc::c_int == 0 {
      /* not overridden */
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lines_after = Copt
    }
    if opts & OPT_B as libc::c_int == 0 {
      /* not overridden */
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lines_before = Copt
    }
  }
  /* sanity checks */
  if opts
    & (OPT_c as libc::c_int | OPT_q as libc::c_int | OPT_l as libc::c_int | OPT_L as libc::c_int)
    != 0
  {
    option_mask32 &= !(OPT_n as libc::c_int) as libc::c_uint;
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lines_before = 0i32;
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lines_after = 0i32
  } else if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lines_before > 0i32 {
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lines_before as libc::c_ulong
      > (2147483647i32 as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_longlong>() as libc::c_ulong)
    {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lines_before = (2147483647i32
        as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_longlong>() as libc::c_ulong)
        as libc::c_int
    }
    /* overflow in (lines_before * sizeof(x)) is prevented (above) */
    let ref mut fresh3 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).before_buf; /* 0 | 1 */
    *fresh3 = xzalloc(
      ((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lines_before as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char
  }
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).invert_search =
    (option_mask32 & OPT_v as libc::c_int as libc::c_uint != 0i32 as libc::c_uint) as libc::c_int
      as smalluint;
  /* convert char **argv to grep_list_data_t */
  let mut cur: *mut llist_t = 0 as *mut llist_t;
  cur = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pattern_head;
  while !cur.is_null() {
    (*cur).data = add_grep_list_data((*cur).data);
    cur = (*cur).link
  }
  if option_mask32 & OPT_f as libc::c_int as libc::c_uint != 0 {
    load_regexes_from_file(fopt);
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .pattern_head
      .is_null()
    {
      /* -f EMPTY_FILE? */
      /* GNU grep treats it as "nothing matches" */
      llist_add_to(
        &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pattern_head,
        add_grep_list_data(b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char)
          as *mut libc::c_void,
      );
      let ref mut fresh4 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).invert_search;
      *fresh4 = (*fresh4 as libc::c_int ^ 1i32) as smalluint
    }
  }
  if 1i32 != 0 && *applet_name.offset(0) as libc::c_int == 'f' as i32 {
    option_mask32 |= OPT_F as libc::c_int as libc::c_uint
  }
  if option_mask32
    & (OPT_o as libc::c_int | OPT_w as libc::c_int | OPT_x as libc::c_int) as libc::c_uint
    == 0
  {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).reflags = 1i32 << 1i32 << 1i32 << 1i32
  }
  if 1i32 != 0 && *applet_name.offset(0) as libc::c_int == 'e' as i32
    || option_mask32 & OPT_E as libc::c_int as libc::c_uint != 0
  {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).reflags |= 1i32
  }
  if option_mask32 & OPT_i as libc::c_int as libc::c_uint != 0 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).reflags |= 1i32 << 1i32
  }
  argv = argv.offset(optind as isize);
  /* if we didn't get a pattern from -e and no command file was specified,
   * first parameter should be the pattern. no pattern, no worky */
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .pattern_head
    .is_null()
  {
    let mut pattern: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*argv).is_null() {
      bb_show_usage();
    }
    let fresh5 = argv;
    argv = argv.offset(1);
    pattern = add_grep_list_data(*fresh5);
    llist_add_to(
      &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pattern_head,
      pattern as *mut libc::c_void,
    );
  }
  /* argv[0..(argc-1)] should be names of file to grep through. If
   * there is more than one file to grep, we will print the filenames. */
  if !(*argv.offset(0)).is_null() && !(*argv.offset(1)).is_null() {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).print_filename = 1i32 as smalluint
  }
  /* -H / -h of course override */
  if option_mask32 & OPT_H as libc::c_int as libc::c_uint != 0 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).print_filename = 1i32 as smalluint
  }
  if option_mask32 & OPT_h as libc::c_int as libc::c_uint != 0 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).print_filename = 0i32 as smalluint
  }
  /* If no files were specified, or '-' was specified, take input from
   * stdin. Otherwise, we grep through all the files specified. */
  matched = 0i32;
  let mut current_block_71: u64;
  loop {
    let ref mut fresh6 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cur_file;
    *fresh6 = *argv;
    file = stdin;
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .cur_file
      .is_null()
      || *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .cur_file
        .offset(0) as libc::c_int
        == '-' as i32
        && *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .cur_file
          .offset(1)
          == 0
    {
      let ref mut fresh7 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cur_file;
      *fresh7 = b"(standard input)\x00" as *const u8 as *const libc::c_char;
      current_block_71 = 9437375157805982253;
    } else {
      if option_mask32 & OPT_r as libc::c_int as libc::c_uint != 0 {
        let mut st: stat = stat {
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
        if stat(
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cur_file,
          &mut st,
        ) == 0i32
          && st.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint
        {
          if option_mask32 & OPT_h as libc::c_int as libc::c_uint == 0 {
            (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).print_filename = 1i32 as smalluint
          }
          matched += grep_dir((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cur_file);
          current_block_71 = 8545136480011357681;
        } else {
          current_block_71 = 5028470053297453708;
        }
      } else {
        current_block_71 = 5028470053297453708;
      }
      match current_block_71 {
        8545136480011357681 => {}
        _ => {
          /* else: fopen(dir) will succeed, but reading won't */
          file = fopen_for_read((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cur_file);
          if file.is_null() {
            if option_mask32 & OPT_s as libc::c_int as libc::c_uint == 0 {
              bb_simple_perror_msg((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cur_file);
            }
            (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).open_errors = 1i32 as smalluint;
            current_block_71 = 8545136480011357681;
          } else {
            current_block_71 = 9437375157805982253;
          }
        }
      }
    }
    match current_block_71 {
      9437375157805982253 => {
        matched += grep_file(file);
        fclose_if_not_stdin(file);
      }
      _ => {}
    }
    if !(!(*argv).is_null() && {
      argv = argv.offset(1);
      !(*argv).is_null()
    }) {
      break;
    }
  }
  /* destroy all the elements in the pattern list */
  /* 0 = success, 1 = failed, 2 = error */
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).open_errors != 0 {
    return 2i32;
  }
  return (matched == 0) as libc::c_int;
  /* invert return value: 0 = success, 1 = failed */
}

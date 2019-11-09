use crate::libbb::llist::llist_t;
use crate::librb::__compar_fn_t;
use crate::librb::size_t;
use libc;
use libc::free;
use libc::FILE;

extern "C" {
  #[no_mangle]
  fn atof(__nptr: *const libc::c_char) -> libc::c_double;
  #[no_mangle]
  fn strtod(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char) -> libc::c_double;
  #[no_mangle]
  fn strtoul(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_ulong;

  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut stderr: *mut FILE;
  #[no_mangle]
  fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn strverscmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strptime(
    __s: *const libc::c_char,
    __fmt: *const libc::c_char,
    __tp: *mut tm,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
  fn xstrndup(s: *const libc::c_char, n: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn bb_get_chunk_from_file(file: *mut FILE, end: *mut size_t) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc_fgetline(file: *mut FILE) -> *mut libc::c_char;
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
  #[no_mangle]
  fn fclose_if_not_stdin(file: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn xfopen_stdin(filename: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn llist_pop(elm: *mut *mut llist_t) -> *mut libc::c_void;
  #[no_mangle]
  static mut xfunc_error_retval: u8;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
}

use libc::tm;

/*
 * SuS3 compliant sort implementation for busybox
 *
 * Copyright (C) 2004 by Rob Landley <rob@landley.net>
 *
 * MAINTAINER: Rob Landley <rob@landley.net>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 *
 * See SuS3 sort standard at:
 * http://www.opengroup.org/onlinepubs/007904975/utilities/sort.html
 */
//config:config SORT
//config:	bool "sort (7.7 kb)"
//config:	default y
//config:	help
//config:	sort is used to sort lines of text in specified files.
//config:
//config:config FEATURE_SORT_BIG
//config:	bool "Full SuSv3 compliant sort (support -ktcbdfiogM)"
//config:	default y
//config:	depends on SORT
//config:	help
//config:	Without this, sort only supports -rusz, and an integer version
//config:	of -n. Selecting this adds sort keys, floating point support, and
//config:	more. This adds a little over 3k to a nonstatic build on x86.
//config:
//config:	The SuSv3 sort standard is available at:
//config:	http://www.opengroup.org/onlinepubs/007904975/utilities/sort.html
//config:
//config:config FEATURE_SORT_OPTIMIZE_MEMORY
//config:	bool "Use less memory (but might be slower)"
//config:	default n   # defaults to N since we are size-paranoid tribe
//config:	depends on SORT
//config:	help
//config:	Attempt to use less memory (by storing only one copy
//config:	of duplicated lines, and such). Useful if you work on huge files.
//applet:IF_SORT(APPLET_NOEXEC(sort, sort, BB_DIR_USR_BIN, BB_SUID_DROP, sort))
//kbuild:lib-$(CONFIG_SORT) += sort.o
//usage:#define sort_trivial_usage
//usage:       "[-nru"
//usage:	IF_FEATURE_SORT_BIG("gMcszbdfiokt] [-o FILE] [-k start[.offset][opts][,end[.offset][opts]] [-t CHAR")
//usage:       "] [FILE]..."
//usage:#define sort_full_usage "\n\n"
//usage:       "Sort lines of text\n"
//usage:	IF_FEATURE_SORT_BIG(
//usage:     "\n	-o FILE	Output to FILE"
//usage:     "\n	-c	Check whether input is sorted"
//usage:     "\n	-b	Ignore leading blanks"
//usage:     "\n	-f	Ignore case"
//usage:     "\n	-i	Ignore unprintable characters"
//usage:     "\n	-d	Dictionary order (blank or alphanumeric only)"
//usage:	)
//-h, --human-numeric-sort: compare human readable numbers (e.g., 2K 1G)
//usage:     "\n	-n	Sort numbers"
//usage:	IF_FEATURE_SORT_BIG(
//usage:     "\n	-g	General numerical sort"
//usage:     "\n	-M	Sort month"
//usage:     "\n	-V	Sort version"
//usage:     "\n	-t CHAR	Field separator"
//usage:     "\n	-k N[,M] Sort by Nth field"
//usage:	)
//usage:     "\n	-r	Reverse sort order"
//usage:     "\n	-s	Stable (don't sort ties alphabetically)"
//usage:     "\n	-u	Suppress duplicate lines"
//usage:     "\n	-z	Lines are terminated by NUL, not newline"
// /////:     "\n	-m	Ignored for GNU compatibility"
// /////:     "\n	-S BUFSZ Ignored for GNU compatibility"
// /////:     "\n	-T TMPDIR Ignored for GNU compatibility"
//usage:
//usage:#define sort_example_usage
//usage:       "$ echo -e \"e\\nf\\nb\\nd\\nc\\na\" | sort\n"
//usage:       "a\n"
//usage:       "b\n"
//usage:       "c\n"
//usage:       "d\n"
//usage:       "e\n"
//usage:       "f\n"
//usage:	IF_FEATURE_SORT_BIG(
//usage:		"$ echo -e \"c 3\\nb 2\\nd 2\" | $SORT -k 2,2n -k 1,1r\n"
//usage:		"d 2\n"
//usage:		"b 2\n"
//usage:		"c 3\n"
//usage:	)
//usage:       ""
/* These are sort types */
pub type C2RustUnnamed = libc::c_uint;
/* Ignore trailing blanks  */
pub const FLAG_no_tie_break: C2RustUnnamed = 1073741824;
pub const FLAG_bb: C2RustUnnamed = 2147483648;
pub const FLAG_t: C2RustUnnamed = 262144;
pub const FLAG_k: C2RustUnnamed = 131072;
/* ignored: -T, --temporary-directory=DIR */
pub const FLAG_o: C2RustUnnamed = 65536;
/* ignored: -S, --buffer-size=SIZE */
pub const FLAG_T: C2RustUnnamed = 32768;
/* ignored: merge already sorted files; do not sort */
pub const FLAG_S: C2RustUnnamed = 16384;
/* Ignore !isprint() */
pub const FLAG_m: C2RustUnnamed = 8192;
/* Force uppercase */
pub const FLAG_i: C2RustUnnamed = 4096;
/* Ignore !(isalnum()|isspace()) */
pub const FLAG_f: C2RustUnnamed = 2048;
/* Reverse */
pub const FLAG_d: C2RustUnnamed = 1024;
/* Ignore leading blanks */
pub const FLAG_r: C2RustUnnamed = 512;
/* Input and output is NUL terminated, not \n */
/* These can be applied to search keys, the previous four can't */
pub const FLAG_b: C2RustUnnamed = 256;
/* Stable sort, no ascii fallback at end */
pub const FLAG_z: C2RustUnnamed = 128;
/* Check: no output, exit(!ordered) */
pub const FLAG_s: C2RustUnnamed = 64;
/* Unique */
pub const FLAG_c: C2RustUnnamed = 32;
/* Sort version */
/* ucsz apply to root level only, not keys.  b at root level implies bb */
pub const FLAG_u: C2RustUnnamed = 16;
/* Sort date */
pub const FLAG_V: C2RustUnnamed = 8;
/* Sort using strtod() */
pub const FLAG_M: C2RustUnnamed = 4;
/* Numeric sort */
pub const FLAG_g: C2RustUnnamed = 2;
pub const FLAG_n: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sort_key {
  pub next_key: *mut sort_key,
  pub range: [libc::c_uint; 4],
  pub flags: libc::c_uint,
}
pub const FLAG_allowed_for_k: C2RustUnnamed_0 = 7943;
pub type C2RustUnnamed_0 = libc::c_uint;
#[inline(always)]
unsafe extern "C" fn bb_ascii_toupper(mut a: libc::c_uchar) -> libc::c_uchar {
  let mut b: libc::c_uchar = (a as libc::c_int - 'a' as i32) as libc::c_uchar;
  if b as libc::c_int <= 'z' as i32 - 'a' as i32 {
    a = (a as libc::c_int - ('a' as i32 - 'A' as i32)) as libc::c_uchar
  }
  return a;
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
static mut sort_opt_str: [libc::c_char; 37] = [
  94, 110, 103, 77, 86, 117, 99, 115, 122, 98, 114, 100, 102, 105, 109, 83, 58, 84, 58, 111, 58,
  107, 58, 42, 116, 58, 0, 111, 45, 45, 111, 58, 116, 45, 45, 116, 0,
];
static mut key_separator: libc::c_char = 0;
static mut key_list: *mut sort_key = 0 as *const sort_key as *mut sort_key;
/* Numeric sort */
/* Sort using strtod() */
/* Sort date */
/* Ignore leading blanks */
/* Reverse */
/* Ignore !(isalnum()|isspace()) */
/* Force uppercase */
/* Ignore !isprint() */
/* This is a NOEXEC applet. Be very careful! */
unsafe extern "C" fn get_key(
  mut str: *mut libc::c_char,
  mut key: *mut sort_key,
  mut flags: libc::c_int,
) -> *mut libc::c_char {
  let mut start: libc::c_int = 0; /* for compiler */
  start = start;
  let mut end: libc::c_int = 0;
  let mut len: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  let mut i: libc::c_uint = 0;
  /* Special case whole string, so we don't have to make a copy */
  if (*key).range[0] == 1i32 as libc::c_uint
    && (*key).range[1] == 0
    && (*key).range[2] == 0
    && (*key).range[3] == 0
    && flags as libc::c_uint
      & ((FLAG_b as libc::c_int
        | FLAG_d as libc::c_int
        | FLAG_f as libc::c_int
        | FLAG_i as libc::c_int) as libc::c_uint
        | FLAG_bb as libc::c_uint)
      == 0
  {
    return str;
  }
  /* Find start of key on first pass, end on second pass */
  len = strlen(str) as libc::c_int;
  j = 0i32;
  while j < 2i32 {
    if (*key).range[(2i32 * j) as usize] == 0 {
      end = len
    } else {
      /* Loop through fields */
      let mut ch: libc::c_uchar = 0i32 as libc::c_uchar;
      end = 0i32;
      i = 1i32 as libc::c_uint;
      while i < (*key).range[(2i32 * j) as usize].wrapping_add(j as libc::c_uint) {
        if key_separator != 0 {
          loop
          /* Skip body of key and separator */
          {
            ch = *str.offset(end as isize) as libc::c_uchar;
            if !(ch as libc::c_int != '\u{0}' as i32) {
              break;
            }
            end += 1;
            if ch as libc::c_int == key_separator as libc::c_int {
              break;
            }
          }
        } else {
          /* Skip leading blanks */
          while ({
            let mut bb__isspace: libc::c_uchar =
              (*str.offset(end as isize) as libc::c_int - 9i32) as libc::c_uchar;
            (bb__isspace as libc::c_int == ' ' as i32 - 9i32
              || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
          }) != 0
          {
            end += 1
          }
          /* Skip body of key */
          while *str.offset(end as isize) as libc::c_int != '\u{0}' as i32 {
            if ({
              let mut bb__isspace: libc::c_uchar =
                (*str.offset(end as isize) as libc::c_int - 9i32) as libc::c_uchar;
              (bb__isspace as libc::c_int == ' ' as i32 - 9i32
                || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
            }) != 0
            {
              break;
            }
            end += 1
          }
        }
        i = i.wrapping_add(1)
      }
      /* Remove last delim: "abc:def:" => "abc:def" */
      if j != 0 && ch as libc::c_int != 0 {
        //if (str[end-1] != key_separator)
        //  bb_error_msg(_and_die("BUG! "
        //  "str[start:%d,end:%d]:'%.*s'",
        //  start, end, (int)(end-start), &str[start]);
        end -= 1
      }
    }
    if j == 0 {
      start = end
    }
    j += 1
  }
  /* Strip leading whitespace if necessary */
  if flags & FLAG_b as libc::c_int != 0 {
    /* not using skip_whitespace() for speed */
    while ({
      let mut bb__isspace: libc::c_uchar =
        (*str.offset(start as isize) as libc::c_int - 9i32) as libc::c_uchar;
      (bb__isspace as libc::c_int == ' ' as i32 - 9i32
        || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
    }) != 0
    {
      start += 1
    }
  }
  /* Strip trailing whitespace if necessary */
  if flags as libc::c_uint & FLAG_bb as libc::c_uint != 0 {
    while end > start
      && ({
        let mut bb__isspace: libc::c_uchar =
          (*str.offset((end - 1i32) as isize) as libc::c_int - 9i32) as libc::c_uchar;
        (bb__isspace as libc::c_int == ' ' as i32 - 9i32
          || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
      }) != 0
    {
      end -= 1
    }
  }
  /* -kSTART,N.ENDCHAR: honor ENDCHAR (1-based) */
  if (*key).range[3] != 0 {
    end = (*key).range[3] as libc::c_int;
    if end > len {
      end = len
    }
  }
  /* -kN.STARTCHAR[,...]: honor STARTCHAR (1-based) */
  if (*key).range[1] != 0 {
    start = (start as libc::c_uint).wrapping_add((*key).range[1].wrapping_sub(1i32 as libc::c_uint))
      as libc::c_int as libc::c_int;
    if start > len {
      start = len
    }
  }
  /* Make the copy */
  if end < start {
    end = start
  }
  str = xstrndup(str.offset(start as isize), end - start);
  /* Handle -d */
  if flags & FLAG_d as libc::c_int != 0 {
    end = 0i32;
    start = end;
    while *str.offset(end as isize) != 0 {
      if ({
        let mut bb__isspace: libc::c_uchar =
          (*str.offset(end as isize) as libc::c_int - 9i32) as libc::c_uchar;
        (bb__isspace as libc::c_int == ' ' as i32 - 9i32
          || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
      }) != 0
        || bb_ascii_isalnum(*str.offset(end as isize) as libc::c_uchar) != 0
      {
        let fresh0 = start;
        start = start + 1;
        *str.offset(fresh0 as isize) = *str.offset(end as isize)
      }
      end += 1
    }
    *str.offset(start as isize) = '\u{0}' as i32 as libc::c_char
  }
  /* Handle -i */
  if flags & FLAG_i as libc::c_int != 0 {
    end = 0i32;
    start = end;
    while *str.offset(end as isize) != 0 {
      if (*str.offset(end as isize) as libc::c_int - 0x20i32) as libc::c_uint
        <= (0x7ei32 - 0x20i32) as libc::c_uint
      {
        let fresh1 = start;
        start = start + 1;
        *str.offset(fresh1 as isize) = *str.offset(end as isize)
      }
      end += 1
    }
    *str.offset(start as isize) = '\u{0}' as i32 as libc::c_char
  }
  /* Handle -f */
  if flags & FLAG_f as libc::c_int != 0 {
    i = 0i32 as libc::c_uint;
    while *str.offset(i as isize) != 0 {
      *str.offset(i as isize) =
        bb_ascii_toupper(*str.offset(i as isize) as libc::c_uchar) as libc::c_char;
      i = i.wrapping_add(1)
    }
  }
  return str;
}
unsafe extern "C" fn add_key() -> *mut sort_key {
  let mut pkey: *mut *mut sort_key = &mut key_list;
  while !(*pkey).is_null() {
    pkey = &mut (**pkey).next_key
  }
  *pkey = xzalloc(::std::mem::size_of::<sort_key>() as libc::c_ulong) as *mut sort_key;
  return *pkey;
}
/* Iterate through keys list and perform comparisons */
unsafe extern "C" fn compare_keys(
  mut xarg: *const libc::c_void,
  mut yarg: *const libc::c_void,
) -> libc::c_int {
  let mut flags: libc::c_int = option_mask32 as libc::c_int; /* for */
  let mut retval: libc::c_int = 0i32;
  let mut x: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut y: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut key: *mut sort_key = 0 as *mut sort_key;
  key = key_list;
  while retval == 0 && !key.is_null() {
    flags = if (*key).flags != 0 {
      (*key).flags
    } else {
      option_mask32
    } as libc::c_int;
    /* if (retval) break; - done by for () anyway */
    x = get_key(*(xarg as *mut *mut libc::c_char), key, flags);
    y = get_key(*(yarg as *mut *mut libc::c_char), key, flags);
    match flags
      & (FLAG_n as libc::c_int
        | FLAG_g as libc::c_int
        | FLAG_M as libc::c_int
        | FLAG_V as libc::c_int)
    {
      8 => retval = strverscmp(x, y),
      0 => {
        /* Chop out and modify key chunks, handling -dfib */
        /* Perform actual comparison */
        /* switch */
        /* Ascii sort */
        retval = strcmp(x, y)
      }
      2 => {
        let mut xx: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut yy: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut dx: libc::c_double = strtod(x, &mut xx);
        let mut dy: libc::c_double = strtod(y, &mut yy);
        /* not numbers < NaN < -infinity < numbers < +infinity) */
        if x == xx {
          retval = if y == yy { 0i32 } else { -1i32 }
        } else if y == yy {
          retval = 1i32
        } else if dx != dx {
          retval = if dy != dy { 0i32 } else { -1i32 }
        } else if dy != dy {
          retval = 1i32
        } else if 1.0f64 / dx == 0.0f64 {
          if dx < 0i32 as libc::c_double {
            retval = if 1.0f64 / dy == 0.0f64 && dy < 0i32 as libc::c_double {
              0i32
            } else {
              -1i32
            }
          } else {
            retval = if 1.0f64 / dy == 0.0f64 && dy > 0i32 as libc::c_double {
              0i32
            } else {
              1i32
            }
          }
        } else if 1.0f64 / dy == 0.0f64 {
          retval = if dy < 0i32 as libc::c_double {
            1i32
          } else {
            -1i32
          }
        } else {
          retval = if dx > dy {
            1i32
          } else if dx < dy {
            -1i32
          } else {
            0i32
          }
        }
      }
      4 => {
        let mut thyme: tm = tm {
          tm_sec: 0,
          tm_min: 0,
          tm_hour: 0,
          tm_mday: 0,
          tm_mon: 0,
          tm_year: 0,
          tm_wday: 0,
          tm_yday: 0,
          tm_isdst: 0,
          tm_gmtoff: 0,
          tm_zone: 0 as *const libc::c_char,
        };
        let mut dx_0: libc::c_int = 0;
        let mut xx_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut yy_0: *mut libc::c_char = 0 as *mut libc::c_char;
        xx_0 = strptime(x, b"%b\x00" as *const u8 as *const libc::c_char, &mut thyme);
        dx_0 = thyme.tm_mon;
        yy_0 = strptime(y, b"%b\x00" as *const u8 as *const libc::c_char, &mut thyme);
        if xx_0.is_null() {
          retval = if yy_0.is_null() { 0i32 } else { -1i32 }
        } else if yy_0.is_null() {
          retval = 1i32
        } else {
          retval = dx_0 - thyme.tm_mon
        }
      }
      1 => {
        /* Check for isnan */
        /* Check for infinity.  Could underflow, but it avoids libm. */
        /* Full floating point version of -n */
        let mut dx_1: libc::c_double = atof(x);
        let mut dy_0: libc::c_double = atof(y);
        retval = if dx_1 > dy_0 {
          1i32
        } else if dx_1 < dy_0 {
          -1i32
        } else {
          0i32
        }
      }
      _ => {
        bb_simple_error_msg_and_die(b"unknown sort type\x00" as *const u8 as *const libc::c_char);
      }
    }
    if x != *(xarg as *mut *mut libc::c_char) {
      free(x as *mut libc::c_void);
    }
    if y != *(yarg as *mut *mut libc::c_char) {
      free(y as *mut libc::c_void);
    }
    key = (*key).next_key
  }
  if retval == 0i32 {
    /* Free key copies. */
    /* So far lines are "the same" */
    if option_mask32 & FLAG_s as libc::c_int as libc::c_uint != 0 {
      /* "Stable sort": later line is "greater than",
       * IOW: do not allow qsort() to swap equal lines.
       */
      let mut p32: *mut u32 = 0 as *mut u32;
      let mut x32: u32 = 0;
      let mut y32: u32 = 0;
      let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
      let mut len: libc::c_uint = 0;
      line = *(xarg as *mut *mut libc::c_char);
      len =
        (strlen(line).wrapping_add(4i32 as libc::c_ulong) & !3u32 as libc::c_ulong) as libc::c_uint;
      p32 = line.offset(len as isize) as *mut libc::c_void as *mut u32;
      x32 = *p32;
      line = *(yarg as *mut *mut libc::c_char);
      len =
        (strlen(line).wrapping_add(4i32 as libc::c_ulong) & !3u32 as libc::c_ulong) as libc::c_uint;
      p32 = line.offset(len as isize) as *mut libc::c_void as *mut u32;
      y32 = *p32;
      /* If x > y, 1, else -1 */
      retval = (x32 > y32) as libc::c_int * 2i32 - 1i32
    } else if option_mask32 & FLAG_no_tie_break as libc::c_int as libc::c_uint == 0 {
      /* fallback sort */
      flags = option_mask32 as libc::c_int;
      retval = strcmp(
        *(xarg as *mut *mut libc::c_char),
        *(yarg as *mut *mut libc::c_char),
      )
    }
  }
  if flags & FLAG_r as libc::c_int != 0 {
    return -retval;
  }
  return retval;
}
unsafe extern "C" fn str2u(mut str: *mut *mut libc::c_char) -> libc::c_uint {
  let mut lu: libc::c_ulong = 0;
  if !((*(*str).offset(0) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32) {
    bb_simple_error_msg_and_die(b"bad field specification\x00" as *const u8 as *const libc::c_char);
  }
  lu = strtoul(*str, str, 10i32);
  if ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
    > ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
    && lu > 2147483647i32 as libc::c_ulong
    || lu == 0
  {
    bb_simple_error_msg_and_die(b"bad field specification\x00" as *const u8 as *const libc::c_char);
  }
  return lu as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn sort_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut lines: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut str_ignored: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut str_o: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut str_t: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut lst_k: *mut llist_t = 0 as *mut llist_t;
  let mut i: libc::c_int = 0;
  let mut linecount: libc::c_int = 0;
  let mut opts: libc::c_uint = 0;
  xfunc_error_retval = 2i32 as u8;
  /* Parse command line options */
  opts = getopt32(
    argv,
    sort_opt_str.as_ptr(),
    &mut str_ignored as *mut *mut libc::c_char,
    &mut str_ignored as *mut *mut libc::c_char,
    &mut str_o as *mut *mut libc::c_char,
    &mut lst_k as *mut *mut llist_t,
    &mut str_t as *mut *mut libc::c_char,
  );
  /* global b strips leading and trailing spaces */
  if opts & FLAG_b as libc::c_int as libc::c_uint != 0 {
    option_mask32 |= FLAG_bb as libc::c_uint
  }
  if opts & FLAG_t as libc::c_int as libc::c_uint != 0 {
    if *str_t.offset(0) == 0 || *str_t.offset(1) as libc::c_int != 0 {
      bb_simple_error_msg_and_die(b"bad -t parameter\x00" as *const u8 as *const libc::c_char);
    }
    key_separator = *str_t.offset(0)
  }
  /* note: below this point we use option_mask32, not opts,
   * since that reduces register pressure and makes code smaller */
  /* Parse sort key */
  while !lst_k.is_null() {
    let mut key: *mut sort_key = add_key(); /* i==0 before comma, 1 after (-k3,6) */
    let mut str_k: *mut libc::c_char = llist_pop(&mut lst_k) as *mut libc::c_char;
    i = 0i32;
    while *str_k != 0 {
      /* Start of range */
      /* Cannot use bb_strtou - suffix can be a letter */
      (*key).range[(2i32 * i) as usize] = str2u(&mut str_k); /* no else needed: fall through to syntax error
                                                             because comma isn't in OPT_STR */
      if *str_k as libc::c_int == '.' as i32 {
        str_k = str_k.offset(1);
        (*key).range[(2i32 * i + 1i32) as usize] = str2u(&mut str_k)
      }
      while *str_k != 0 {
        let mut flag: libc::c_int = 0;
        let mut idx: *const libc::c_char = 0 as *const libc::c_char;
        if *str_k as libc::c_int == ',' as i32 && {
          let fresh2 = i;
          i = i + 1;
          (fresh2) == 0
        } {
          str_k = str_k.offset(1);
          break;
        } else {
          idx = strchr(sort_opt_str.as_ptr().offset(1), *str_k as libc::c_int);
          if idx.is_null() {
            bb_simple_error_msg_and_die(
              b"unknown key option\x00" as *const u8 as *const libc::c_char,
            );
          }
          flag = 1i32 << idx.wrapping_offset_from(sort_opt_str.as_ptr().offset(1)) as libc::c_long;
          if flag & !(FLAG_allowed_for_k as libc::c_int) != 0 {
            bb_simple_error_msg_and_die(
              b"unknown sort type\x00" as *const u8 as *const libc::c_char,
            );
          }
          /* b after ',' means strip _trailing_ space */
          if i != 0 && flag == FLAG_b as libc::c_int {
            flag = FLAG_bb as libc::c_uint as libc::c_int
          }
          (*key).flags |= flag as libc::c_uint;
          str_k = str_k.offset(1)
        }
      }
    }
  }
  /* Open input files and read data */
  argv = argv.offset(optind as isize);
  if (*argv).is_null() {
    argv = argv.offset(-1);
    *argv = b"-\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }
  linecount = 0i32;
  lines = 0 as *mut *mut libc::c_char;
  loop {
    /* coreutils 6.9 compat: abort on first open error,
     * do not continue to next file: */
    let mut fp: *mut FILE = xfopen_stdin(*argv);
    loop {
      let mut line: *mut libc::c_char =
        if option_mask32 & FLAG_z as libc::c_int as libc::c_uint != 0 {
          bb_get_chunk_from_file(fp, 0 as *mut size_t)
        } else {
          xmalloc_fgetline(fp)
        };
      if line.is_null() {
        break;
      }
      //TODO: lighter version which only drops total dups if can_drop_dups == true
      lines = xrealloc_vector_helper(
        lines as *mut libc::c_void,
        ((::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) << 8i32)
          .wrapping_add(6i32 as libc::c_ulong) as libc::c_uint,
        linecount,
      ) as *mut *mut libc::c_char;
      let fresh3 = linecount;
      linecount = linecount + 1;
      let ref mut fresh4 = *lines.offset(fresh3 as isize);
      *fresh4 = line
    }
    fclose_if_not_stdin(fp);
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  /* If no key, perform alphabetic sort */
  if key_list.is_null() {
    (*add_key()).range[0] = 1i32 as libc::c_uint
  }
  /* Handle -c */
  if option_mask32 & FLAG_c as libc::c_int as libc::c_uint != 0 {
    let mut j: libc::c_int = if option_mask32 & FLAG_u as libc::c_int as libc::c_uint != 0 {
      -1i32
    } else {
      0i32
    };
    i = 1i32;
    while i < linecount {
      if compare_keys(
        &mut *lines.offset((i - 1i32) as isize) as *mut *mut libc::c_char as *const libc::c_void,
        &mut *lines.offset(i as isize) as *mut *mut libc::c_char as *const libc::c_void,
      ) > j
      {
        fprintf(
          stderr,
          b"Check line %u\n\x00" as *const u8 as *const libc::c_char,
          i,
        );
        return 1i32;
      }
      i += 1
    }
    return 0i32;
  }
  /* For stable sort, store original line position beyond terminating NUL */
  if option_mask32 & FLAG_s as libc::c_int as libc::c_uint != 0 {
    i = 0i32;
    while i < linecount {
      let mut p32: *mut u32 = 0 as *mut u32;
      let mut line_0: *mut libc::c_char = 0 as *mut libc::c_char;
      let mut len: libc::c_uint = 0;
      line_0 = *lines.offset(i as isize);
      len = (strlen(line_0).wrapping_add(4i32 as libc::c_ulong) & !3u32 as libc::c_ulong)
        as libc::c_uint;
      line_0 = xrealloc(
        line_0 as *mut libc::c_void,
        len.wrapping_add(4i32 as libc::c_uint) as size_t,
      ) as *mut libc::c_char;
      let ref mut fresh5 = *lines.offset(i as isize);
      *fresh5 = line_0;
      p32 = line_0.offset(len as isize) as *mut libc::c_void as *mut u32;
      *p32 = i as u32;
      i += 1
    }
    /*option_mask32 |= FLAG_no_tie_break;*/
    /* ^^^redundant: if FLAG_s, compare_keys() does no tie break */
  }
  /* Perform the actual sort */
  qsort(
    lines as *mut libc::c_void,
    linecount as size_t,
    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    Some(
      compare_keys
        as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int,
    ),
  );
  /* Handle -u */
  if option_mask32 & FLAG_u as libc::c_int as libc::c_uint != 0 {
    let mut j_0: libc::c_int = 0i32;
    /* coreutils 6.3 drop lines for which only key is the same
     * -- disabling last-resort compare, or else compare_keys()
     * will be the same only for completely identical lines.
     */
    option_mask32 |= FLAG_no_tie_break as libc::c_int as libc::c_uint;
    i = 1i32;
    while i < linecount {
      if compare_keys(
        &mut *lines.offset(j_0 as isize) as *mut *mut libc::c_char as *const libc::c_void,
        &mut *lines.offset(i as isize) as *mut *mut libc::c_char as *const libc::c_void,
      ) == 0i32
      {
        free(*lines.offset(i as isize) as *mut libc::c_void);
      } else {
        j_0 += 1;
        let ref mut fresh6 = *lines.offset(j_0 as isize);
        *fresh6 = *lines.offset(i as isize)
      }
      i += 1
    }
    if linecount != 0 {
      linecount = j_0 + 1i32
    }
  }
  /* Print it */
  /* Open output file _after_ we read all input ones */
  if option_mask32 & FLAG_o as libc::c_int as libc::c_uint != 0 {
    xmove_fd(xopen(str_o, 0o1i32 | 0o100i32 | 0o1000i32), 1i32);
  }
  let mut ch: libc::c_int = if option_mask32 & FLAG_z as libc::c_int as libc::c_uint != 0 {
    '\u{0}' as i32
  } else {
    '\n' as i32
  };
  i = 0i32;
  while i < linecount {
    printf(
      b"%s%c\x00" as *const u8 as *const libc::c_char,
      *lines.offset(i as isize),
      ch,
    );
    i += 1
  }
  fflush_stdout_and_exit(0i32);
}

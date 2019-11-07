use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrndup(s: *const libc::c_char, n: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_process_escape_sequence(ptr: *mut *const libc::c_char) -> libc::c_char;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn xwrite(fd: libc::c_int, buf: *const libc::c_void, count: size_t);
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
}

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
use crate::librb::size_t;
use libc::ssize_t;

/*
 * Mini tr implementation for busybox
 *
 * Copyright (c) 1987,1997, Prentice Hall   All rights reserved.
 *
 * The name of Prentice Hall may not be used to endorse or promote
 * products derived from this software without specific prior
 * written permission.
 *
 * Copyright (c) Michiel Huisjes
 *
 * This version of tr is adapted from Minix tr and was modified
 * by Erik Andersen <andersen@codepoet.org> to be used in busybox.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* http://www.opengroup.org/onlinepubs/009695399/utilities/tr.html
 * TODO: graph, print
 */
//config:config TR
//config:	bool "tr (5.1 kb)"
//config:	default y
//config:	help
//config:	tr is used to squeeze, and/or delete characters from standard
//config:	input, writing to standard output.
//config:
//config:config FEATURE_TR_CLASSES
//config:	bool "Enable character classes (such as [:upper:])"
//config:	default y
//config:	depends on TR
//config:	help
//config:	Enable character classes, enabling commands such as:
//config:	tr [:upper:] [:lower:] to convert input into lowercase.
//config:
//config:config FEATURE_TR_EQUIV
//config:	bool "Enable equivalence classes"
//config:	default y
//config:	depends on TR
//config:	help
//config:	Enable equivalence classes, which essentially add the enclosed
//config:	character to the current set. For instance, tr [=a=] xyz would
//config:	replace all instances of 'a' with 'xyz'. This option is mainly
//config:	useful for cases when no other way of expressing a character
//config:	is possible.
//applet:IF_TR(APPLET(tr, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_TR) += tr.o
//usage:#define tr_trivial_usage
//usage:       "[-cds] STRING1 [STRING2]"
//usage:#define tr_full_usage "\n\n"
//usage:       "Translate, squeeze, or delete characters from stdin, writing to stdout\n"
//usage:     "\n	-c	Take complement of STRING1"
//usage:     "\n	-d	Delete input characters coded STRING1"
//usage:     "\n	-s	Squeeze multiple output characters of STRING2 into one character"
//usage:
//usage:#define tr_example_usage
//usage:       "$ echo \"gdkkn vnqkc\" | tr [a-y] [b-z]\n"
//usage:       "hello world\n"
pub type C2RustUnnamed = libc::c_uint;
/* string buffer needs to be at least as big as the whole "alphabet".
 * BUFSIZ == ASCII is ok, but we will realloc in expand
 * even for smallest patterns, let's avoid that by using *2:
 */
pub const TR_BUFSIZ: C2RustUnnamed = 8192;
pub const ASCII: C2RustUnnamed = 256;
pub const CLASS_xdigit: C2RustUnnamed_0 = 10;
pub const CLASS_cntrl: C2RustUnnamed_0 = 9;
pub const CLASS_punct: C2RustUnnamed_0 = 8;
pub const CLASS_space: C2RustUnnamed_0 = 6;
pub const CLASS_blank: C2RustUnnamed_0 = 7;
pub const CLASS_lower: C2RustUnnamed_0 = 4;
pub const CLASS_alnum: C2RustUnnamed_0 = 2;
/* we increment the retval */
pub const CLASS_alpha: C2RustUnnamed_0 = 1;
pub const CLASS_upper: C2RustUnnamed_0 = 5;
pub const CLASS_digit: C2RustUnnamed_0 = 3;
pub const CLASS_invalid: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
#[inline(always)]
unsafe extern "C" fn bb_ascii_isalnum(mut a: libc::c_uchar) -> libc::c_int {
  let mut b: libc::c_uchar = (a as libc::c_int - '0' as i32) as libc::c_uchar;
  if b as libc::c_int <= 9i32 {
    return (b as libc::c_int <= 9i32) as libc::c_int;
  }
  b = ((a as libc::c_int | 0x20i32) - 'a' as i32) as libc::c_uchar;
  return (b as libc::c_int <= 'z' as i32 - 'a' as i32) as libc::c_int;
}
unsafe extern "C" fn map(
  mut pvector: *mut libc::c_char,
  mut string1: *mut libc::c_char,
  mut string1_len: libc::c_uint,
  mut string2: *mut libc::c_char,
  mut string2_len: libc::c_uint,
) {
  let mut last: libc::c_char = '0' as i32 as libc::c_char;
  let mut i: libc::c_uint = 0;
  let mut j: libc::c_uint = 0;
  j = 0i32 as libc::c_uint;
  i = 0i32 as libc::c_uint;
  while i < string1_len {
    if string2_len <= j {
      *pvector.offset(*string1.offset(i as isize) as libc::c_uchar as isize) = last
    } else {
      let fresh0 = j;
      j = j.wrapping_add(1);
      last = *string2.offset(fresh0 as isize);
      *pvector.offset(*string1.offset(i as isize) as libc::c_uchar as isize) = last
    }
    i = i.wrapping_add(1)
  }
}
//CLASS_graph = 11,
//CLASS_print = 12,
/* supported constructs:
 *   Ranges,  e.g.,  0-9   ==>  0123456789
 *   Escapes, e.g.,  \a    ==>  Control-G
 *   Character classes, e.g. [:upper:] ==> A...Z
 *   Equiv classess, e.g. [=A=] ==> A   (hmmmmmmm?)
 * not supported:
 *   [x*N] - repeat char x N times
 *   [x*] - repeat char x until it fills STRING2:
 * # echo qwe123 | /usr/bin/tr 123456789 '[d]'
 * qwe[d]
 * # echo qwe123 | /usr/bin/tr 123456789 '[d*]'
 * qweddd
 */
unsafe extern "C" fn expand(
  mut arg: *mut libc::c_char,
  mut buffer_p: *mut *mut libc::c_char,
) -> libc::c_uint {
  let mut buffer: *mut libc::c_char = *buffer_p; /* can't be unsigned char: must be able to hold 256 */
  let mut pos: libc::c_uint = 0i32 as libc::c_uint;
  let mut size: libc::c_uint = TR_BUFSIZ as libc::c_int as libc::c_uint;
  let mut i: libc::c_uint = 0;
  let mut ac: libc::c_uchar = 0;
  while *arg != 0 {
    if pos.wrapping_add(ASCII as libc::c_int as libc::c_uint) > size {
      size = size.wrapping_add(ASCII as libc::c_int as libc::c_uint);
      buffer = xrealloc(buffer as *mut libc::c_void, size as size_t) as *mut libc::c_char;
      *buffer_p = buffer
    }
    if *arg as libc::c_int == '\\' as i32 {
      let mut z: *const libc::c_char = 0 as *const libc::c_char;
      arg = arg.offset(1);
      z = arg;
      ac = bb_process_escape_sequence(&mut z) as libc::c_uchar;
      arg = z as *mut libc::c_char;
      arg = arg.offset(-1);
      *arg = ac as libc::c_char
      /*
       * fall through, there may be a range.
       * If not, current char will be treated anyway.
       */
    }
    if *arg.offset(1) as libc::c_int == '-' as i32 {
      /* "0-9..." */
      ac = *arg.offset(2) as libc::c_uchar;
      if ac as libc::c_int == '\u{0}' as i32 {
        /* "0-": copy verbatim */
        let fresh1 = arg; /* copy '0' */
        arg = arg.offset(1);
        let fresh2 = pos;
        pos = pos.wrapping_add(1);
        *buffer.offset(fresh2 as isize) = *fresh1
      /* next iter will copy '-' and stop */
      } else {
        i = *arg as libc::c_uchar as libc::c_uint; /* skip 0-9 or 0-\ */
        arg = arg.offset(3);
        if ac as libc::c_int == '\\' as i32 {
          let mut z_0: *const libc::c_char = 0 as *const libc::c_char;
          z_0 = arg;
          ac = bb_process_escape_sequence(&mut z_0) as libc::c_uchar;
          arg = z_0 as *mut libc::c_char
        }
        while i <= ac as libc::c_uint {
          /* ok: i is unsigned _int_ */
          let fresh3 = i;
          i = i.wrapping_add(1);
          let fresh4 = pos;
          pos = pos.wrapping_add(1);
          *buffer.offset(fresh4 as isize) = fresh3 as libc::c_char
        }
      }
    } else {
      if (1i32 != 0 || 1i32 != 0) && *arg as libc::c_int == '[' as i32 {
        arg = arg.offset(1);
        let fresh5 = arg;
        arg = arg.offset(1);
        i = *fresh5 as libc::c_uchar as libc::c_uint;
        /* points to "[" in "[xyz..." */
        /* "[xyz...". i=x, arg points to y */
        if 1i32 != 0 && i == ':' as i32 as libc::c_uint {
          /* [:class:] */
          static mut classes: [libc::c_char; 82] = [
            97, 108, 112, 104, 97, 58, 93, 0, 97, 108, 110, 117, 109, 58, 93, 0, 100, 105, 103,
            105, 116, 58, 93, 0, 108, 111, 119, 101, 114, 58, 93, 0, 117, 112, 112, 101, 114, 58,
            93, 0, 115, 112, 97, 99, 101, 58, 93, 0, 98, 108, 97, 110, 107, 58, 93, 0, 112, 117,
            110, 99, 116, 58, 93, 0, 99, 110, 116, 114, 108, 58, 93, 0, 120, 100, 105, 103, 105,
            116, 58, 93, 0, 0,
          ];
          let mut j: smalluint = 0;
          let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
          /* xdigit needs 8, not 7 */
          i = (7i32 + (*arg.offset(0) as libc::c_int == 'x' as i32) as libc::c_int) as libc::c_uint;
          tmp = xstrndup(arg, i as libc::c_int);
          j = (index_in_strings(classes.as_ptr(), tmp) + 1i32) as smalluint;
          free(tmp as *mut libc::c_void);
          if !(j as libc::c_int == CLASS_invalid as libc::c_int) {
            arg = arg.offset(i as isize);
            if j as libc::c_int == CLASS_alnum as libc::c_int
              || j as libc::c_int == CLASS_digit as libc::c_int
              || j as libc::c_int == CLASS_xdigit as libc::c_int
            {
              i = '0' as i32 as libc::c_uint;
              while i <= '9' as i32 as libc::c_uint {
                let fresh6 = pos;
                pos = pos.wrapping_add(1);
                *buffer.offset(fresh6 as isize) = i as libc::c_char;
                i = i.wrapping_add(1)
              }
            }
            if j as libc::c_int == CLASS_alpha as libc::c_int
              || j as libc::c_int == CLASS_alnum as libc::c_int
              || j as libc::c_int == CLASS_upper as libc::c_int
            {
              i = 'A' as i32 as libc::c_uint;
              while i <= 'Z' as i32 as libc::c_uint {
                let fresh7 = pos;
                pos = pos.wrapping_add(1);
                *buffer.offset(fresh7 as isize) = i as libc::c_char;
                i = i.wrapping_add(1)
              }
            }
            if j as libc::c_int == CLASS_alpha as libc::c_int
              || j as libc::c_int == CLASS_alnum as libc::c_int
              || j as libc::c_int == CLASS_lower as libc::c_int
            {
              i = 'a' as i32 as libc::c_uint;
              while i <= 'z' as i32 as libc::c_uint {
                let fresh8 = pos;
                pos = pos.wrapping_add(1);
                *buffer.offset(fresh8 as isize) = i as libc::c_char;
                i = i.wrapping_add(1)
              }
            }
            if j as libc::c_int == CLASS_space as libc::c_int
              || j as libc::c_int == CLASS_blank as libc::c_int
            {
              let fresh9 = pos;
              pos = pos.wrapping_add(1);
              *buffer.offset(fresh9 as isize) = '\t' as i32 as libc::c_char;
              if j as libc::c_int == CLASS_space as libc::c_int {
                let fresh10 = pos;
                pos = pos.wrapping_add(1);
                *buffer.offset(fresh10 as isize) = '\n' as i32 as libc::c_char;
                let fresh11 = pos;
                pos = pos.wrapping_add(1);
                *buffer.offset(fresh11 as isize) = '\u{b}' as i32 as libc::c_char;
                let fresh12 = pos;
                pos = pos.wrapping_add(1);
                *buffer.offset(fresh12 as isize) = '\u{c}' as i32 as libc::c_char;
                let fresh13 = pos;
                pos = pos.wrapping_add(1);
                *buffer.offset(fresh13 as isize) = '\r' as i32 as libc::c_char
              }
              let fresh14 = pos;
              pos = pos.wrapping_add(1);
              *buffer.offset(fresh14 as isize) = ' ' as i32 as libc::c_char
            }
            if j as libc::c_int == CLASS_punct as libc::c_int
              || j as libc::c_int == CLASS_cntrl as libc::c_int
            {
              i = '\u{0}' as i32 as libc::c_uint;
              while i < ASCII as libc::c_int as libc::c_uint {
                if j as libc::c_int == CLASS_punct as libc::c_int
                  && i.wrapping_sub(0x20i32 as libc::c_uint) <= (0x7ei32 - 0x20i32) as libc::c_uint
                  && bb_ascii_isalnum(i as libc::c_uchar) == 0
                  && ({
                    let mut bb__isspace: libc::c_uchar =
                      i.wrapping_sub(9i32 as libc::c_uint) as libc::c_uchar;
                    (bb__isspace as libc::c_int == ' ' as i32 - 9i32
                      || bb__isspace as libc::c_int <= 13i32 - 9i32)
                      as libc::c_int
                  }) == 0
                  || j as libc::c_int == CLASS_cntrl as libc::c_int
                    && ({
                      let mut bb__iscntrl: libc::c_uchar = i as libc::c_uchar;
                      ((bb__iscntrl as libc::c_int) < ' ' as i32
                        || bb__iscntrl as libc::c_int == 0x7fi32)
                        as libc::c_int
                    }) != 0
                {
                  let fresh15 = pos;
                  pos = pos.wrapping_add(1);
                  *buffer.offset(fresh15 as isize) = i as libc::c_char
                }
                i = i.wrapping_add(1)
              }
            }
            if j as libc::c_int == CLASS_xdigit as libc::c_int {
              i = 'A' as i32 as libc::c_uint;
              while i <= 'F' as i32 as libc::c_uint {
                *buffer.offset(pos.wrapping_add(6i32 as libc::c_uint) as isize) =
                  (i | 0x20i32 as libc::c_uint) as libc::c_char;
                let fresh16 = pos;
                pos = pos.wrapping_add(1);
                *buffer.offset(fresh16 as isize) = i as libc::c_char;
                i = i.wrapping_add(1)
              }
              pos = pos.wrapping_add(6i32 as libc::c_uint)
            }
            continue;
          }
        } else if 1i32 != 0 && i == '=' as i32 as libc::c_uint {
          /* "[xyz...", i=x, arg points to y */
          /* [=CHAR=] */
          let fresh17 = pos; /* copy CHAR */
          pos = pos.wrapping_add(1); /* skip CHAR=] */
          *buffer.offset(fresh17 as isize) = *arg;
          if *arg.offset(0) == 0
            || *arg.offset(1) as libc::c_int != '=' as i32
            || *arg.offset(2) as libc::c_int != ']' as i32
          {
            bb_show_usage();
          }
          arg = arg.offset(3);
          continue;
        }
        arg = arg.offset(-2)
      }
      let fresh18 = arg;
      arg = arg.offset(1);
      let fresh19 = pos;
      pos = pos.wrapping_add(1);
      *buffer.offset(fresh19 as isize) = *fresh18
    }
  }
  return pos;
}
/* The rest of "[xyz..." cases is treated as normal
 * string, "[" has no special meaning here:
 * tr "[a-z]" "[A-Z]" can be written as tr "a-z" "A-Z",
 * also try tr "[a-z]" "_A-Z+" and you'll see that
 * [] is not special here.
 */
/* NB: buffer is guaranteed to be at least TR_BUFSIZE
 * (which is >= ASCII) big.
 */
unsafe extern "C" fn complement(
  mut buffer: *mut libc::c_char,
  mut buffer_len: libc::c_int,
) -> libc::c_int {
  let mut len: libc::c_int = 0; /* not equal to any char */
  let mut conv: [libc::c_char; 256] = [0; 256];
  let mut ch: libc::c_uchar = 0;
  len = 0i32;
  ch = '\u{0}' as i32 as libc::c_uchar;
  loop {
    if memchr(
      buffer as *const libc::c_void,
      ch as libc::c_int,
      buffer_len as libc::c_ulong,
    )
    .is_null()
    {
      let fresh20 = len;
      len = len + 1;
      conv[fresh20 as usize] = ch as libc::c_char
    }
    ch = ch.wrapping_add(1);
    if ch as libc::c_int == '\u{0}' as i32 {
      break;
    }
  }
  memcpy(
    buffer as *mut libc::c_void,
    conv.as_mut_ptr() as *const libc::c_void,
    len as libc::c_ulong,
  );
  return len;
}
#[no_mangle]
pub unsafe extern "C" fn tr_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut i: libc::c_int = 0;
  let mut opts: smalluint = 0;
  let mut read_chars: ssize_t = 0;
  let mut in_index: size_t = 0;
  let mut out_index: size_t = 0;
  let mut last: libc::c_uint = (127i32 * 2i32 + 1i32 + 1i32) as libc::c_uint;
  let mut coded: libc::c_uchar = 0;
  let mut c: libc::c_uchar = 0;
  let mut str1: *mut libc::c_char =
    xmalloc(TR_BUFSIZ as libc::c_int as size_t) as *mut libc::c_char;
  let mut str2: *mut libc::c_char =
    xmalloc(TR_BUFSIZ as libc::c_int as size_t) as *mut libc::c_char;
  let mut str2_length: libc::c_int = 0;
  let mut str1_length: libc::c_int = 0;
  let mut vector: *mut libc::c_char =
    xzalloc((ASCII as libc::c_int * 3i32) as size_t) as *mut libc::c_char;
  let mut invec: *mut libc::c_char = vector.offset(ASCII as libc::c_int as isize);
  let mut outvec: *mut libc::c_char = vector.offset((ASCII as libc::c_int * 2i32) as isize);
  i = 0i32;
  while i < ASCII as libc::c_int {
    *vector.offset(i as isize) = i as libc::c_char;
    i += 1
    /*invec[i] = outvec[i] = FALSE; - done by xzalloc */
  }
  /* -C/-c difference is that -C complements "characters",
   * and -c complements "values" (binary bytes I guess).
   * In POSIX locale, these are the same.
   */
  /* '+': stop at first non-option */
  opts = getopt32(
    argv,
    b"^+Ccds\x00-1\x00" as *const u8 as *const libc::c_char,
  ) as smalluint;
  argv = argv.offset(optind as isize);
  let fresh21 = argv;
  argv = argv.offset(1);
  str1_length = expand(*fresh21, &mut str1) as libc::c_int;
  str2_length = 0i32;
  if opts as libc::c_int & 3i32 << 0i32 != 0 {
    str1_length = complement(str1, str1_length)
  }
  if !(*argv).is_null() {
    if *(*argv.offset(0)).offset(0) as libc::c_int == '\u{0}' as i32 {
      bb_simple_error_msg_and_die(
        b"STRING2 cannot be empty\x00" as *const u8 as *const libc::c_char,
      );
    }
    str2_length = expand(*argv, &mut str2) as libc::c_int;
    map(
      vector,
      str1,
      str1_length as libc::c_uint,
      str2,
      str2_length as libc::c_uint,
    );
  }
  i = 0i32;
  while i < str1_length {
    *invec.offset(*str1.offset(i as isize) as libc::c_uchar as isize) = 1i32 as libc::c_char;
    i += 1
  }
  i = 0i32;
  while i < str2_length {
    *outvec.offset(*str2.offset(i as isize) as libc::c_uchar as isize) = 1i32 as libc::c_char;
    i += 1
  }
  'c_9891: loop {
    out_index = 0i32 as size_t;
    loop {
      read_chars = safe_read(
        0i32,
        str1 as *mut libc::c_void,
        TR_BUFSIZ as libc::c_int as size_t,
      );
      if read_chars <=0{
        if read_chars < 0 {
          bb_simple_perror_msg_and_die(b"read error\x00" as *const u8 as *const libc::c_char);
        }
        break 'c_9891;
      } else {
        in_index = 0i32 as size_t;
        loop {
          let fresh22 = in_index;
          in_index = in_index.wrapping_add(1);
          c = *str1.offset(fresh22 as isize) as libc::c_uchar;
          if !(opts as libc::c_int & 1i32 << 2i32 != 0
            && *invec.offset(c as isize) as libc::c_int != 0)
          {
            coded = *vector.offset(c as isize) as libc::c_uchar;
            if !(opts as libc::c_int & 1i32 << 3i32 != 0
              && last == coded as libc::c_uint
              && (*invec.offset(c as isize) as libc::c_int != 0
                || *outvec.offset(coded as isize) as libc::c_int != 0))
            {
              last = coded as libc::c_uint;
              let fresh23 = out_index;
              out_index = out_index.wrapping_add(1);
              *str2.offset(fresh23 as isize) = last as libc::c_char
            }
          }
          /* If we're out of input, flush output and read more input. */
          if in_index as ssize_t == read_chars {
            break;
          }
        }
        if !(out_index != 0) {
          continue;
        }
        xwrite(1i32, str2 as *const libc::c_void, out_index);
        break;
      }
    }
  }
  return 0i32;
}

use libc;
extern "C" {

  #[no_mangle]
  static bb_msg_standard_input: [libc::c_char; 0];
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xopen_stdin(pathname: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn xwrite(fd: libc::c_int, buf: *const libc::c_void, count: size_t);
  #[no_mangle]
  fn xatoull_range(
    str: *const libc::c_char,
    l: libc::c_ulonglong,
    u: libc::c_ulonglong,
  ) -> libc::c_ulonglong;
  #[no_mangle]
  fn xatoull_sfx(str: *const libc::c_char, sfx: *const suffix_mult) -> libc::c_ulonglong;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

use crate::librb::size_t;
use libc::off_t;
use libc::ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct suffix_mult {
  pub suffix: [libc::c_char; 4],
  pub mult: libc::c_uint,
}
pub type C2RustUnnamed = libc::c_uint;
pub const READ_BUFFER_SIZE: C2RustUnnamed = 1023;
#[inline(always)]
unsafe extern "C" fn xatoul_range(
  mut str: *const libc::c_char,
  mut l: libc::c_ulong,
  mut u: libc::c_ulong,
) -> libc::c_ulong {
  return xatoull_range(str, l as libc::c_ulonglong, u as libc::c_ulonglong) as libc::c_ulong;
}

/*
 * split - split a file into pieces
 * Copyright (c) 2007 Bernhard Reutner-Fischer
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config SPLIT
//config:	bool "split (5 kb)"
//config:	default y
//config:	help
//config:	Split a file into pieces.
//config:
//config:config FEATURE_SPLIT_FANCY
//config:	bool "Fancy extensions"
//config:	default y
//config:	depends on SPLIT
//config:	help
//config:	Add support for features not required by SUSv3.
//config:	Supports additional suffixes 'b' for 512 bytes,
//config:	'g' for 1GiB for the -b option.
//applet:IF_SPLIT(APPLET(split, BB_DIR_USR_BIN, SUID_DROP))
//kbuild:lib-$(CONFIG_SPLIT) += split.o
/* BB_AUDIT: SUSv3 compliant
 * SUSv3 requirements:
 * http://www.opengroup.org/onlinepubs/009695399/utilities/split.html
 */
//usage:#define split_trivial_usage
//usage:       "[OPTIONS] [INPUT [PREFIX]]"
//usage:#define split_full_usage "\n\n"
//usage:       "	-b N[k|m]	Split by N (kilo|mega)bytes"
//usage:     "\n	-l N		Split by N lines"
//usage:     "\n	-a N		Use N letters as suffix"
//usage:
//usage:#define split_example_usage
//usage:       "$ split TODO foo\n"
//usage:       "$ cat TODO | split -a 2 -l 2 TODO_\n"
static mut split_suffixes: [suffix_mult; 5] = [
  {
    let mut init = suffix_mult {
      suffix: [98, 0, 0, 0],
      mult: 512i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [107, 0, 0, 0],
      mult: 1024i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [109, 0, 0, 0],
      mult: (1024i32 * 1024i32) as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [103, 0, 0, 0],
      mult: (1024i32 * 1024i32 * 1024i32) as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [0, 0, 0, 0],
      mult: 0i32 as libc::c_uint,
    };
    init
  },
];
/* Increment the suffix part of the filename.
 * Returns NULL if we are out of filenames.
 */
unsafe extern "C" fn next_file(
  mut old: *mut libc::c_char,
  mut suffix_len: libc::c_uint,
) -> *mut libc::c_char {
  let mut end: size_t = strlen(old);
  let mut i: libc::c_uint = 1i32 as libc::c_uint;
  let mut curr: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  loop {
    curr = old.offset(end as isize).offset(-(i as isize));
    if (*curr as libc::c_int) < 'z' as i32 {
      *curr = (*curr as libc::c_int + 1i32) as libc::c_char;
      break;
    } else {
      i = i.wrapping_add(1);
      if i > suffix_len {
        return std::ptr::null_mut::<libc::c_char>();
      }
      *curr = 'a' as i32 as libc::c_char
    }
  }
  return old;
}
#[no_mangle]
pub unsafe extern "C" fn split_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut suffix_len: libc::c_uint = 2i32 as libc::c_uint;
  let mut pfx: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut count_p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut sfx: *const libc::c_char = 0 as *const libc::c_char;
  let mut cnt: off_t = 1000i32 as off_t;
  let mut remaining: off_t = 0i32 as off_t;
  let mut opt: libc::c_uint = 0;
  let mut bytes_read: ssize_t = 0;
  let mut to_write: ssize_t = 0;
  let mut src: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  opt = getopt32(
    argv,
    b"^l:b:a:+\x00?2\x00" as *const u8 as *const libc::c_char,
    &mut count_p as *mut *mut libc::c_char,
    &mut count_p as *mut *mut libc::c_char,
    &mut suffix_len as *mut libc::c_uint,
  );
  if opt & (1i32 << 0i32) as libc::c_uint != 0 {
    cnt = xatoul_range(
      count_p,
      0i32 as libc::c_ulong,
      9223372036854775807i64 as libc::c_ulong,
    ) as off_t
  }
  if opt & (1i32 << 1i32) as libc::c_uint != 0 {
    // FIXME: also needs XATOOFF
    cnt = xatoull_sfx(count_p, split_suffixes.as_ptr()) as off_t
  }
  sfx = b"x\x00" as *const u8 as *const libc::c_char;
  argv = argv.offset(optind as isize);
  if !(*argv.offset(0)).is_null() {
    let mut fd: libc::c_int = 0;
    if !(*argv.offset(1)).is_null() {
      sfx = *argv.offset(1)
    }
    fd = xopen_stdin(*argv.offset(0));
    xmove_fd(fd, 0i32);
  } else {
    let ref mut fresh0 = *argv.offset(0);
    *fresh0 = bb_msg_standard_input.as_ptr() as *mut libc::c_char
  }
  if (255i32 as libc::c_ulong) < strlen(sfx).wrapping_add(suffix_len as libc::c_ulong) {
    bb_simple_error_msg_and_die(b"suffix too long\x00" as *const u8 as *const libc::c_char);
  }
  let mut char_p: *mut libc::c_char =
    xzalloc(suffix_len.wrapping_add(1i32 as libc::c_uint) as size_t) as *mut libc::c_char;
  memset(
    char_p as *mut libc::c_void,
    'a' as i32,
    suffix_len as libc::c_ulong,
  );
  pfx = xasprintf(b"%s%s\x00" as *const u8 as *const libc::c_char, sfx, char_p);
  loop {
    bytes_read = safe_read(
      0i32,
      bb_common_bufsiz1.as_mut_ptr() as *mut libc::c_void,
      READ_BUFFER_SIZE as libc::c_int as size_t,
    );
    if bytes_read == 0 {
      break;
    }
    if bytes_read < 0 {
      bb_simple_perror_msg_and_die(*argv.offset(0));
    }
    src = bb_common_bufsiz1.as_mut_ptr();
    loop {
      if remaining == 0 {
        if pfx.is_null() {
          bb_simple_error_msg_and_die(
            b"suffixes exhausted\x00" as *const u8 as *const libc::c_char,
          );
        }
        xmove_fd(xopen(pfx, 0o1i32 | 0o100i32 | 0o1000i32), 1i32);
        pfx = next_file(pfx, suffix_len);
        remaining = cnt
      }
      if opt & (1i32 << 1i32) as libc::c_uint != 0 {
        /* split by bytes */
        to_write = if bytes_read < remaining as isize {
          bytes_read
        } else {
          remaining as isize
        };
        remaining -= to_write as i64
      } else {
        /* split by lines */
        /* can be sped up by using _memrchr_
         * and writing many lines at once... */
        let mut end: *mut libc::c_char = memchr(
          src as *const libc::c_void,
          '\n' as i32,
          bytes_read as libc::c_ulong,
        ) as *mut libc::c_char;
        if !end.is_null() {
          remaining -= 1;
          to_write = end.wrapping_offset_from(src) + 1
        } else {
          to_write = bytes_read
        }
      }
      xwrite(1i32, src as *const libc::c_void, to_write as size_t);
      bytes_read -= to_write;
      src = src.offset(to_write as isize);
      if !(bytes_read != 0) {
        break;
      }
    }
  }
  return 0i32;
}

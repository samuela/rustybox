use crate::libbb::xfuncs_printf::xmalloc;
use crate::librb::size_t;
use libc;
use libc::free;
use libc::getopt;
use libc::printf;
use libc::putchar_unlocked;
use libc::FILE;
extern "C" {

  #[no_mangle]
  static mut optarg: *mut libc::c_char;
  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  static mut stdin: *mut FILE;
  #[no_mangle]
  static mut stdout: *mut FILE;

  #[no_mangle]
  fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  static bkm_suffixes: [suffix_mult; 0];

  #[no_mangle]
  static bb_msg_standard_input: [libc::c_char; 0];
}

use crate::librb::suffix_mult;
#[inline(always)]
unsafe fn xatoul_sfx(
  mut str: *const libc::c_char,
  mut sfx: *const suffix_mult,
) -> libc::c_ulong {
  return crate::libbb::xatonum::xatoull_sfx(str, sfx) as libc::c_ulong;
}

/*
 * head implementation for busybox
 *
 * Copyright (C) 2003  Manuel Novoa III  <mjn3@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config HEAD
//config:	bool "head (3.8 kb)"
//config:	default y
//config:	help
//config:	head is used to print the first specified number of lines
//config:	from files.
//config:
//config:config FEATURE_FANCY_HEAD
//config:	bool "Enable -c, -q, and -v"
//config:	default y
//config:	depends on HEAD
//applet:IF_HEAD(APPLET_NOEXEC(head, head, BB_DIR_USR_BIN, SUID_DROP, head))
//kbuild:lib-$(CONFIG_HEAD) += head.o
/* BB_AUDIT SUSv3 compliant */
/* BB_AUDIT GNU compatible -c, -q, and -v options in 'fancy' configuration. */
/* http://www.opengroup.org/onlinepubs/007904975/utilities/head.html */
//usage:#define head_trivial_usage
//usage:       "[OPTIONS] [FILE]..."
//usage:#define head_full_usage "\n\n"
//usage:       "Print first 10 lines of each FILE (or stdin) to stdout.\n"
//usage:       "With more than one FILE, precede each with a filename header.\n"
//usage:     "\n	-n N[kbm]	Print first N lines"
//usage:	IF_FEATURE_FANCY_HEAD(
//usage:     "\n	-n -N[kbm]	Print all except N last lines"
//usage:     "\n	-c [-]N[kbm]	Print first N bytes"
//usage:     "\n	-q		Never print headers"
//usage:     "\n	-v		Always print headers"
//usage:	)
//usage:     "\n"
//usage:     "\nN may be suffixed by k (x1024), b (x512), or m (x1024^2)."
//usage:
//usage:#define head_example_usage
//usage:       "$ head -n 2 /etc/passwd\n"
//usage:       "root:x:0:0:root:/root:/bin/bash\n"
//usage:       "daemon:x:1:1:daemon:/usr/sbin:/bin/sh\n"
/* This is a NOEXEC applet. Be very careful! */
unsafe fn print_first_N(
  mut fp: *mut FILE,
  mut count: libc::c_ulong,
  mut count_bytes: bool,
) {
  while count != 0 {
    let mut c: libc::c_int = getc_unlocked(fp);
    if c == -1i32 {
      break;
    }
    if count_bytes as libc::c_int != 0 || c == '\n' as i32 {
      count = count.wrapping_sub(1)
    }
    putchar_unlocked(c);
  }
}
unsafe fn print_except_N_last_bytes(mut fp: *mut FILE, mut count: libc::c_uint) {
  let mut current_block: u64;
  count = count.wrapping_add(1);
  let mut circle: *mut libc::c_uchar = xmalloc(count as size_t) as *mut libc::c_uchar;
  let mut head: libc::c_uint = 0 as libc::c_uint;
  loop {
    let mut c: libc::c_int = 0;
    c = getc_unlocked(fp);
    if c == -1i32 {
      current_block = 7076737922188077789;
      break;
    }
    let fresh0 = head;
    head = head.wrapping_add(1);
    *circle.offset(fresh0 as isize) = c as libc::c_uchar;
    if head == count {
      current_block = 3276175668257526147;
      break;
    }
  }
  loop {
    match current_block {
      7076737922188077789 => {
        free(circle as *mut libc::c_void);
        break;
      }
      _ => {
        let mut c_0: libc::c_int = 0;
        if head == count {
          head = 0 as libc::c_uint
        }
        putchar_unlocked(*circle.offset(head as isize) as libc::c_int);
        c_0 = getc_unlocked(fp);
        if c_0 == -1i32 {
          current_block = 7076737922188077789;
          continue;
        }
        *circle.offset(head as isize) = c_0 as libc::c_uchar;
        head = head.wrapping_add(1);
        current_block = 3276175668257526147;
      }
    }
  }
}
unsafe fn print_except_N_last_lines(mut fp: *mut FILE, mut count: libc::c_uint) {
  let mut current_block: u64;
  count = count.wrapping_add(1);
  let mut circle: *mut *mut libc::c_char = crate::libbb::xfuncs_printf::xzalloc(
    (count as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
  ) as *mut *mut libc::c_char;
  let mut head: libc::c_uint = 0 as libc::c_uint;
  loop {
    let mut c: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    c = crate::libbb::get_line_from_file::xmalloc_fgets(fp);
    if c.is_null() {
      current_block = 349542114677786389;
      break;
    }
    let fresh1 = head;
    head = head.wrapping_add(1);
    let ref mut fresh2 = *circle.offset(fresh1 as isize);
    *fresh2 = c;
    if head == count {
      current_block = 3276175668257526147;
      break;
    }
  }
  loop {
    match current_block {
      349542114677786389 => {
        head = 0 as libc::c_uint;
        break;
      }
      _ => {
        let mut c_0: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
        if head == count {
          head = 0 as libc::c_uint
        }
        fputs_unlocked(*circle.offset(head as isize), stdout);
        c_0 = crate::libbb::get_line_from_file::xmalloc_fgets(fp);
        if c_0.is_null() {
          current_block = 349542114677786389;
          continue;
        }
        free(*circle.offset(head as isize) as *mut libc::c_void);
        let fresh3 = head;
        head = head.wrapping_add(1);
        let ref mut fresh4 = *circle.offset(fresh3 as isize);
        *fresh4 = c_0;
        current_block = 3276175668257526147;
      }
    }
  }
  loop {
    let fresh5 = head;
    head = head.wrapping_add(1);
    free(*circle.offset(fresh5 as isize) as *mut libc::c_void);
    if head == count {
      break;
    }
  }
  free(circle as *mut libc::c_void);
}
unsafe fn eat_num(
  mut negative_N: *mut bool,
  mut p: *const libc::c_char,
) -> libc::c_ulong {
  if *p as libc::c_int == '-' as i32 {
    *negative_N = 1i32 != 0;
    p = p.offset(1)
  }
  return xatoul_sfx(p, bkm_suffixes.as_ptr());
}
static mut head_opts: [libc::c_char; 7] = [110, 58, 99, 58, 113, 118, 0];
pub unsafe fn head_main(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut current_block: u64;
  let mut count: libc::c_ulong = 10i32 as libc::c_ulong;
  let mut header_threshhold: libc::c_int = 1i32;
  let mut count_bytes: bool = 0 != 0;
  let mut negative_N: bool = 0 != 0;
  let mut fp: *mut FILE = std::ptr::null_mut();
  let mut fmt: *const libc::c_char = std::ptr::null();
  let mut p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut opt: libc::c_int = 0;
  let mut retval: libc::c_int = 0;
  /* Allow legacy syntax of an initial numeric option without -n. */
  if !(*argv.offset(1)).is_null()
    && *(*argv.offset(1)).offset(0) as libc::c_int == '-' as i32
    && (*(*argv.offset(1)).offset(1) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int
      <= 9i32
  {
    argc -= 1;
    argv = argv.offset(1);
    p = (*argv.offset(0)).offset(1);
    current_block = 1181006765757951898;
  } else {
    current_block = 3640593987805443782;
  }
  loop {
    match current_block {
      1181006765757951898 => {
        count = eat_num(&mut negative_N, p);
        current_block = 3640593987805443782;
      }
      _ =>
      /* No size benefit in converting this to getopt32 */
      {
        opt = getopt(argc, argv, head_opts.as_ptr());
        if !(opt > 0) {
          break;
        }
        match opt {
          113 => {
            header_threshhold = 2147483647i32;
            current_block = 3640593987805443782;
            continue;
          }
          118 => {
            header_threshhold = -1i32;
            current_block = 3640593987805443782;
            continue;
          }
          99 => count_bytes = 1i32 != 0,
          110 => {}
          _ => {
            crate::libbb::appletlib::bb_show_usage();
          }
        }
        /* fall through */
        p = optarg;
        current_block = 1181006765757951898;
      }
    }
  }
  argc -= optind;
  argv = argv.offset(optind as isize);
  if (*argv).is_null() {
    argv = argv.offset(-1);
    *argv = b"-\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }
  fmt = (b"\n==> %s <==\n\x00" as *const u8 as *const libc::c_char).offset(1);
  if argc <= header_threshhold {
    header_threshhold = 0
  }
  if negative_N {
    if count
      >= (2147483647i32 as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    {
      crate::libbb::verror_msg::bb_error_msg(
        b"count is too big: %lu\x00" as *const u8 as *const libc::c_char,
        count,
      );
    }
  }
  loop {
    fp = crate::libbb::wfopen_input::fopen_or_warn_stdin(*argv);
    if !fp.is_null() {
      if fp == stdin {
        *argv = bb_msg_standard_input.as_ptr() as *mut libc::c_char
      }
      if header_threshhold != 0 {
        printf(fmt, *argv);
      }
      if negative_N {
        if count_bytes {
          print_except_N_last_bytes(fp, count as libc::c_uint);
        } else {
          print_except_N_last_lines(fp, count as libc::c_uint);
        }
      } else {
        print_first_N(fp, count, count_bytes);
      }
      crate::libbb::xfuncs_printf::die_if_ferror_stdout();
      if crate::libbb::fclose_nonstdin::fclose_if_not_stdin(fp) != 0 {
        crate::libbb::perror_msg::bb_simple_perror_msg(*argv);
        retval = 1i32
      }
    } else {
      retval = 1i32
    }
    fmt = b"\n==> %s <==\n\x00" as *const u8 as *const libc::c_char;
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  crate::libbb::fflush_stdout_and_exit::fflush_stdout_and_exit(retval);
}

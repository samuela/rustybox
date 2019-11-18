use crate::libbb::ptr_to_globals::bb_errno;
use libc;
use libc::open;
use libc::stat;
use libc::suseconds_t;
use libc::time;
use libc::time_t;
use libc::timeval;
use libc::tm;
extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn utimes(__file: *const libc::c_char, __tvp: *const timeval) -> libc::c_int;
  #[no_mangle]
  fn lutimes(__file: *const libc::c_char, __tvp: *const timeval) -> libc::c_int;

  #[no_mangle]
  fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;

  #[no_mangle]
  fn xstat(pathname: *const libc::c_char, buf: *mut stat);
  #[no_mangle]
  fn parse_datestr(date_str: *const libc::c_char, ptm: *mut tm);
  #[no_mangle]
  fn validate_tm_time(date_str: *const libc::c_char, ptm: *mut tm) -> time_t;
  #[no_mangle]
  fn xclose(fd: libc::c_int);
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> u32;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
}

pub const OPT_c: C2RustUnnamed = 1;
pub const OPT_h: C2RustUnnamed = 16;
pub type C2RustUnnamed = libc::c_uint;
pub const OPT_t: C2RustUnnamed = 8;
pub const OPT_d: C2RustUnnamed = 4;
pub const OPT_r: C2RustUnnamed = 2;

/*
 * Mini touch implementation for busybox
 *
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Mar 16, 2003      Manuel Novoa III   (mjn3@codepoet.org)
 *
 * Previous version called open() and then utime().  While this will be
 * be necessary to implement -r and -t, it currently only makes things bigger.
 * Also, exiting on a failure was a bug.  All args should be processed.
 */
//config:config TOUCH
//config:	bool "touch (5.9 kb)"
//config:	default y
//config:	help
//config:	touch is used to create or change the access and/or
//config:	modification timestamp of specified files.
//config:
//config:config FEATURE_TOUCH_NODEREF
//config:	bool "Add support for -h"
//config:	default y
//config:	depends on TOUCH
//config:	help
//config:	Enable touch to have the -h option.
//config:	This requires libc support for lutimes() function.
//config:
//config:config FEATURE_TOUCH_SUSV3
//config:	bool "Add support for SUSV3 features (-d -t -r)"
//config:	default y
//config:	depends on TOUCH
//config:	help
//config:	Enable touch to use a reference file or a given date/time argument.
//applet:IF_TOUCH(APPLET_NOFORK(touch, touch, BB_DIR_BIN, BB_SUID_DROP, touch))
//kbuild:lib-$(CONFIG_TOUCH) += touch.o
/* BB_AUDIT SUSv3 _NOT_ compliant -- options -a, -m not supported. */
/* http://www.opengroup.org/onlinepubs/007904975/utilities/touch.html */
//usage:#define touch_trivial_usage
//usage:       "[-c]" IF_FEATURE_TOUCH_SUSV3(" [-d DATE] [-t DATE] [-r FILE]") " FILE..."
//usage:#define touch_full_usage "\n\n"
//usage:       "Update the last-modified date on the given FILE[s]\n"
//usage:     "\n	-c	Don't create files"
//usage:	IF_FEATURE_TOUCH_NODEREF(
//usage:     "\n	-h	Don't follow links"
//usage:	)
//usage:	IF_FEATURE_TOUCH_SUSV3(
//usage:     "\n	-d DT	Date/time to use"
//usage:     "\n	-t DT	Date/time to use"
//usage:     "\n	-r FILE	Use FILE's date/time"
//usage:	)
//usage:
//usage:#define touch_example_usage
//usage:       "$ ls -l /tmp/foo\n"
//usage:       "/bin/ls: /tmp/foo: No such file or directory\n"
//usage:       "$ touch /tmp/foo\n"
//usage:       "$ ls -l /tmp/foo\n"
//usage:       "-rw-rw-r--    1 andersen andersen        0 Apr 15 01:11 /tmp/foo\n"
/* coreutils implements:
 * -a   change only the access time
 * -c, --no-create
 *      do not create any files
 * -d, --date=STRING
 *      parse STRING and use it instead of current time
 * -f   (ignored, BSD compat)
 * -m   change only the modification time
 * -h, --no-dereference
 * -r, --reference=FILE
 *      use this file's times instead of current time
 * -t STAMP
 *      use [[CC]YY]MMDDhhmm[.ss] instead of current time
 * --time=WORD
 *      change the specified time: WORD is access, atime, or use
 */
#[no_mangle]
pub unsafe extern "C" fn touch_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut fd: libc::c_int = 0;
  let mut status: libc::c_int = 0i32;
  let mut opts: libc::c_int = 0;
  static mut touch_longopts: [libc::c_char; 49] = [
    110, 111, 45, 99, 114, 101, 97, 116, 101, 0, 0, 99, 114, 101, 102, 101, 114, 101, 110, 99, 101,
    0, 1, 114, 100, 97, 116, 101, 0, 1, 100, 110, 111, 45, 100, 101, 114, 101, 102, 101, 114, 101,
    110, 99, 101, 0, 0, 104, 0,
  ];
  let mut reference_file: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut date_str: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut timebuf: [timeval; 2] = [timeval {
    tv_sec: 0,
    tv_usec: 0,
  }; 2];
  timebuf[0].tv_usec = 0i32 as suseconds_t;
  timebuf[1].tv_usec = timebuf[0].tv_usec;
  /* -d and -t both set time. In coreutils,
   * accepted data format differs a bit between -d and -t.
   * We accept the same formats for both */
  opts = getopt32long(
    argv,
    b"cr:d:t:hfma\x00" as *const u8 as *const libc::c_char,
    touch_longopts.as_ptr(),
    &mut reference_file as *mut *mut libc::c_char,
    &mut date_str as *mut *mut libc::c_char,
    &mut date_str as *mut *mut libc::c_char,
  ) as libc::c_int;
  argv = argv.offset(optind as isize);
  if (*argv).is_null() {
    bb_show_usage();
  }
  if !reference_file.is_null() {
    let mut stbuf: stat = std::mem::zeroed();
    xstat(reference_file, &mut stbuf);
    timebuf[0].tv_sec = stbuf.st_mtime;
    timebuf[1].tv_sec = timebuf[0].tv_sec
    /* Can use .st_mtime.tv_nsec
     * (or is it .st_mtimeensec?? see date.c)
     * to set microseconds too.
     */
  }
  if !date_str.is_null() {
    let mut tm_time: tm = tm {
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
    let mut t: time_t = 0;
    //memset(&tm_time, 0, sizeof(tm_time));
    /* Better than memset: makes "HH:MM" dates meaningful */
    time(&mut t);
    localtime_r(&mut t, &mut tm_time);
    parse_datestr(date_str, &mut tm_time);
    /* Correct any day of week and day of year etc. fields */
    tm_time.tm_isdst = -1i32; /* Be sure to recheck dst */
    t = validate_tm_time(date_str, &mut tm_time);
    timebuf[0].tv_sec = t;
    timebuf[1].tv_sec = timebuf[0].tv_sec
  }
  let mut current_block_24: u64;
  loop {
    let mut result: libc::c_int = 0;
    result = if opts & OPT_h as libc::c_int != 0 {
      Some(
        lutimes as unsafe extern "C" fn(_: *const libc::c_char, _: *const timeval) -> libc::c_int,
      )
    } else {
      Some(utimes as unsafe extern "C" fn(_: *const libc::c_char, _: *const timeval) -> libc::c_int)
    }
    .expect("non-null function pointer")(
      *argv,
      if !reference_file.is_null() || !date_str.is_null() {
        timebuf.as_mut_ptr()
      } else {
        0 as *mut timeval
      } as *const timeval,
    );
    if result != 0i32 {
      if *bb_errno == 2i32 {
        /* no such file? */
        if opts & OPT_c as libc::c_int != 0 {
          current_block_24 = 11042950489265723346;
        } else {
          /* Try to create the file */
          fd = open(*argv, 0o2i32 | 0o100i32, 0o666i32);
          if fd >= 0i32 {
            xclose(fd);
            if !reference_file.is_null() || !date_str.is_null() {
              utimes(*argv, timebuf.as_mut_ptr() as *const timeval);
            }
            current_block_24 = 11042950489265723346;
          } else {
            current_block_24 = 8704759739624374314;
          }
        }
      } else {
        current_block_24 = 8704759739624374314;
      }
      match current_block_24 {
        11042950489265723346 => {}
        _ => {
          status = 1i32;
          bb_simple_perror_msg(*argv);
        }
      }
    }
    /* Creation is disabled, so ignore */
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  return status;
}

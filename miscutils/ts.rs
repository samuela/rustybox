use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn putenv(__string: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut stdin: *mut FILE;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> libc::c_int;
  #[no_mangle]
  fn strftime(
    __s: *mut libc::c_char,
    __maxsize: size_t,
    __format: *const libc::c_char,
    __tp: *const tm,
  ) -> size_t;
  #[no_mangle]
  fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
  #[no_mangle]
  fn tzset();
  #[no_mangle]
  fn is_suffixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc_fgets(file: *mut FILE) -> *mut libc::c_char;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type int32_t = __int32_t;
use crate::librb::uint32_t;
pub type size_t = libc::c_ulong;
use crate::librb::time_t;
use crate::librb::timeval;



use crate::librb::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
  pub tz_minuteswest: libc::c_int,
  pub tz_dsttime: libc::c_int,
}
pub type __timezone_ptr_t = *mut timezone;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
  pub tm_sec: libc::c_int,
  pub tm_min: libc::c_int,
  pub tm_hour: libc::c_int,
  pub tm_mday: libc::c_int,
  pub tm_mon: libc::c_int,
  pub tm_year: libc::c_int,
  pub tm_wday: libc::c_int,
  pub tm_yday: libc::c_int,
  pub tm_isdst: libc::c_int,
  pub tm_gmtoff: libc::c_long,
  pub tm_zone: *const libc::c_char,
}
pub type C2RustUnnamed = libc::c_uint;
pub const COMMON_BUFSIZE: C2RustUnnamed = 1024;
/* vi: set sw=4 ts=4: */
/*
 * Copyright (C) 2019 Denys Vlasenko <vda.linux@googlemail.com>
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config TS
//config:	bool "ts (450 bytes)"
//config:	default y
//applet:IF_TS(APPLET(ts, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_TS) += ts.o
//usage:#define ts_trivial_usage
//usage:       "[-is] [STRFTIME]"
//usage:#define ts_full_usage ""
#[no_mangle]
pub unsafe extern "C" fn ts_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut base: timeval = timeval {
    tv_sec: 0,
    tv_usec: 0,
  };
  let mut opt: libc::c_uint = 0;
  let mut frac: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut fmt_dt2str: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
  opt = getopt32(argv, b"^is\x00?1\x00" as *const u8 as *const libc::c_char);
  if opt != 0 {
    putenv(b"TZ=UTC0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
    tzset();
  }
  /*argc -= optind;*/
  argv = argv.offset(optind as isize);
  fmt_dt2str = if !(*argv.offset(0)).is_null() {
    *argv.offset(0)
  } else {
    (if opt != 0 {
      (b"%b %d %H:%M:%S\x00" as *const u8 as *const libc::c_char).offset(6)
    } else {
      b"%b %d %H:%M:%S\x00" as *const u8 as *const libc::c_char
    }) as *mut libc::c_char
  };
  frac = is_suffixed_with(fmt_dt2str, b"%.S\x00" as *const u8 as *const libc::c_char);
  if frac.is_null() {
    frac = is_suffixed_with(fmt_dt2str, b"%.s\x00" as *const u8 as *const libc::c_char)
  }
  if !frac.is_null() {
    frac = frac.offset(1);
    *frac.offset(0) = *frac.offset(1);
    *frac.offset(1) = '\u{0}' as i32 as libc::c_char
  }
  gettimeofday(&mut base, 0 as *mut timezone);
  loop {
    line = xmalloc_fgets(stdin);
    if line.is_null() {
      break;
    }
    let mut ts: timeval = timeval {
      tv_sec: 0,
      tv_usec: 0,
    };
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
    gettimeofday(&mut ts, 0 as *mut timezone);
    if opt != 0 {
      /* -i and/or -s */
      let mut ts1: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
      };
      ts1 = ts1;
      if opt & 1i32 as libc::c_uint != 0 {
        /* -i */
        ts1 = ts
      }
      //printf("%d %d\n", ts.tv_sec, base.tv_sec);
      ts.tv_sec -= base.tv_sec;
      //printf("%d %d\n", ts.tv_sec, base.tv_sec);
      ts.tv_usec -= base.tv_usec;
      if (ts.tv_usec as int32_t) < 0i32 {
        ts.tv_sec -= 1;
        ts.tv_usec += (1000i32 * 1000i32) as libc::c_long
      }
      if opt & 1i32 as libc::c_uint != 0 {
        /* -i */
        base = ts1
      }
    }
    localtime_r(&mut ts.tv_sec, &mut tm_time);
    strftime(
      bb_common_bufsiz1.as_mut_ptr(),
      COMMON_BUFSIZE as libc::c_int as size_t,
      fmt_dt2str,
      &mut tm_time,
    );
    if frac.is_null() {
      printf(
        b"%s %s\x00" as *const u8 as *const libc::c_char,
        bb_common_bufsiz1.as_mut_ptr(),
        line,
      );
    } else {
      printf(
        b"%s.%06u %s\x00" as *const u8 as *const libc::c_char,
        bb_common_bufsiz1.as_mut_ptr(),
        ts.tv_usec as libc::c_uint,
        line,
      );
    }
    free(line as *mut libc::c_void);
  }
  return 0i32;
}

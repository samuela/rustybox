use libc;
extern "C" {
  #[no_mangle]
  fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn putenv(__string: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn usleep(__useconds: __useconds_t) -> libc::c_int;
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut FILE,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn mktime(__tp: *mut tm) -> time_t;
  #[no_mangle]
  fn tzset();
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn bb_xioctl(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
use crate::librb::size_t;
use crate::librb::time_t;



use crate::librb::FILE;
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
/*
 * Everything below this point has been copied from linux/rtc.h
 * to eliminate the kernel header dependency
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_rtc_time {
  pub tm_sec: libc::c_int,
  pub tm_min: libc::c_int,
  pub tm_hour: libc::c_int,
  pub tm_mday: libc::c_int,
  pub tm_mon: libc::c_int,
  pub tm_year: libc::c_int,
  pub tm_wday: libc::c_int,
  pub tm_yday: libc::c_int,
  pub tm_isdst: libc::c_int,
}
/*
 * Common defines/structures/etc... for applets that need to work with the RTC.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn rtc_adjtime_is_utc() -> libc::c_int {
  let mut utc: libc::c_int = 0i32;
  let mut f: *mut FILE = fopen_for_read(b"/etc/adjtime\x00" as *const u8 as *const libc::c_char);
  if !f.is_null() {
    let mut buffer: [libc::c_char; 128] = [0; 128];
    while !fgets_unlocked(
      buffer.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
      f,
    )
    .is_null()
    {
      if is_prefixed_with(
        buffer.as_mut_ptr(),
        b"UTC\x00" as *const u8 as *const libc::c_char,
      )
      .is_null()
      {
        continue;
      }
      utc = 1i32;
      break;
    }
    fclose(f);
  }
  return utc;
}
/* rtc opens are exclusive.
 * Try to run two "hwclock -w" at the same time to see it.
 * Users wouldn't expect that to fail merely because /dev/rtc
 * was momentarily busy, let's try a bit harder on errno == EBUSY.
 */
unsafe extern "C" fn open_loop_on_busy(
  mut name: *const libc::c_char,
  mut flags: libc::c_int,
) -> libc::c_int {
  let mut rtc: libc::c_int = 0;
  /*
   * Tested with two parallel "hwclock -w" loops.
   * With try = 10, no failures with 2x1000000 loop iterations.
   */
  let mut try_0: libc::c_int = 1000i32 / 20i32;
  loop {
    *bb_errno = 0i32;
    rtc = open(name, flags);
    if *bb_errno == 16i32 {
      usleep((20i32 * 1000i32) as __useconds_t);
      try_0 -= 1;
      if try_0 != 0i32 {
        continue;
      }
      /* EBUSY. Last try, exit on error instead of returning -1 */
      return xopen(name, flags);
    } else {
      return rtc;
    }
  }
}
/* Never fails */
#[no_mangle]
pub unsafe extern "C" fn rtc_xopen(
  mut default_rtc: *mut *const libc::c_char,
  mut flags: libc::c_int,
) -> libc::c_int {
  let mut current_block: u64; /*else: we have rtc name, don't try other names */
  let mut rtc: libc::c_int = 0;
  let mut name: *const libc::c_char =
    b"/dev/rtc\x00/dev/rtc0\x00/dev/misc/rtc\x00\x00" as *const u8 as *const libc::c_char;
  if (*default_rtc).is_null() {
    current_block = 9167590035537892259;
  } else {
    name = b"\x00" as *const u8 as *const libc::c_char;
    current_block = 715039052867723359;
  }
  loop {
    match current_block {
      9167590035537892259 => {
        *default_rtc = name;
        name = name.offset(strlen(name).wrapping_add(1i32 as libc::c_ulong) as isize);
        current_block = 715039052867723359;
      }
      _ => {
        rtc = open_loop_on_busy(*default_rtc, flags);
        if rtc >= 0i32 {
          return rtc;
        }
        if *name.offset(0) == 0 {
          return xopen(*default_rtc, flags);
        }
        current_block = 9167590035537892259;
      }
    }
  }
}
#[no_mangle]
pub unsafe extern "C" fn rtc_read_tm(mut ptm: *mut tm, mut fd: libc::c_int) {
  memset(
    ptm as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<tm>() as libc::c_ulong,
  );
  bb_xioctl(
    fd,
    ((2u32 << 0i32 + 8i32 + 8i32 + 14i32
      | (('p' as i32) << 0i32 + 8i32) as libc::c_uint
      | (0x9i32 << 0i32) as libc::c_uint) as libc::c_ulong
      | (::std::mem::size_of::<linux_rtc_time>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
      as libc::c_uint,
    ptm as *mut libc::c_void,
    b"RTC_RD_TIME\x00" as *const u8 as *const libc::c_char,
  );
  (*ptm).tm_isdst = -1i32;
  /* "not known" */
}
#[no_mangle]
pub unsafe extern "C" fn rtc_tm2time(mut ptm: *mut tm, mut utc: libc::c_int) -> time_t {
  let mut oldtz: *mut libc::c_char = 0 as *mut libc::c_char; /* for compiler */
  oldtz = oldtz;
  let mut t: time_t = 0;
  if utc != 0 {
    oldtz = getenv(b"TZ\x00" as *const u8 as *const libc::c_char);
    putenv(b"TZ=UTC0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
    tzset();
  }
  t = mktime(ptm);
  if utc != 0 {
    unsetenv(b"TZ\x00" as *const u8 as *const libc::c_char);
    if !oldtz.is_null() {
      putenv(oldtz.offset(-3));
    }
    tzset();
  }
  return t;
}

use libc;
use libc::printf;
use libc::ssize_t;
use libc::strcmp;
use libc::sync;
use libc::time;
use libc::time_t;
use libc::tm;
use libc::useconds_t;
extern "C" {

  #[no_mangle]
  fn snprintf(
    _: *mut libc::c_char,
    _: libc::c_ulong,
    _: *const libc::c_char,
    _: ...
  ) -> libc::c_int;
  #[no_mangle]
  fn usleep(__useconds: useconds_t) -> libc::c_int;

  #[no_mangle]
  fn localtime(__timer: *const time_t) -> *mut tm;
  #[no_mangle]
  fn ctime(__timer: *const time_t) -> *mut libc::c_char;

/*
 * Common defines/structures/etc... for applets that need to work with the RTC.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */

}

/*
 * Everything below this point has been copied from linux/rtc.h
 * to eliminate the kernel header dependency
 */

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct linux_rtc_wkalrm {
  pub enabled: libc::c_uchar,
  pub pending: libc::c_uchar,
  pub time: linux_rtc_time,
  /* time the alarm is set to */
}
#[inline(always)]
unsafe fn xatol(mut str: *const libc::c_char) -> libc::c_long {
  return crate::libbb::xatonum::xatoll(str) as libc::c_long;
}
#[inline(never)]
unsafe fn may_wakeup(mut rtcname: *const libc::c_char) -> bool {
  let mut ret: ssize_t = 0;
  let mut buf: [libc::c_char; 128] = [0; 128];
  /* strip "/dev/" from the rtcname here */
  rtcname = crate::libbb::skip_whitespace::skip_dev_pfx(rtcname);
  snprintf(
    buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
    b"/sys/class/rtc/%s/device/power/wakeup\x00" as *const u8 as *const libc::c_char,
    rtcname,
  );
  ret = crate::libbb::read::open_read_close(
    buf.as_mut_ptr(),
    buf.as_mut_ptr() as *mut libc::c_void,
    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
  );
  if ret < 0 {
    return 0 != 0;
  }
  /* wakeup events could be disabled or not supported */
  return !crate::libbb::compare_string_array::is_prefixed_with(
    buf.as_mut_ptr(),
    b"enabled\n\x00" as *const u8 as *const libc::c_char,
  )
  .is_null();
}
#[inline(never)]
unsafe fn setup_alarm(
  mut fd: libc::c_int,
  mut wakeup: *mut time_t,
  mut rtc_time: time_t,
) {
  let mut ptm: *mut tm = std::ptr::null_mut();
  let mut wake: linux_rtc_wkalrm = linux_rtc_wkalrm {
    enabled: 0,
    pending: 0,
    time: linux_rtc_time {
      tm_sec: 0,
      tm_min: 0,
      tm_hour: 0,
      tm_mday: 0,
      tm_mon: 0,
      tm_year: 0,
      tm_wday: 0,
      tm_yday: 0,
      tm_isdst: 0,
    },
  };
  /* The wakeup time is in POSIX time (more or less UTC).
   * Ideally RTCs use that same time; but PCs can't do that
   * if they need to boot MS-Windows.  Messy...
   *
   * When running in utc mode this process's timezone is UTC,
   * so we'll pass a UTC date to the RTC.
   *
   * Else mode is local so the time given to the RTC
   * will instead use the local time zone.
   */
  ptm = localtime(wakeup);
  wake.time.tm_sec = (*ptm).tm_sec;
  wake.time.tm_min = (*ptm).tm_min;
  wake.time.tm_hour = (*ptm).tm_hour;
  wake.time.tm_mday = (*ptm).tm_mday;
  wake.time.tm_mon = (*ptm).tm_mon;
  wake.time.tm_year = (*ptm).tm_year;
  /* wday, yday, and isdst fields are unused by Linux */
  wake.time.tm_wday = -1i32;
  wake.time.tm_yday = -1i32;
  wake.time.tm_isdst = -1i32;
  /* many rtc alarms only support up to 24 hours from 'now',
   * so use the "more than 24 hours" request only if we must
   */
  if rtc_time + (24i32 * 60i32 * 60i32) as libc::c_long > *wakeup {
    crate::libbb::xfuncs_printf::bb_xioctl(
      fd,
      ((1u32 << 0 + 8i32 + 8i32 + 14i32
        | (('p' as i32) << 0 + 8i32) as libc::c_uint
        | (0x7i32 << 0) as libc::c_uint) as libc::c_ulong
        | (::std::mem::size_of::<linux_rtc_time>() as libc::c_ulong) << 0 + 8i32 + 8i32)
        as libc::c_uint,
      &mut wake.time as *mut linux_rtc_time as *mut libc::c_void,
      b"RTC_ALM_SET\x00" as *const u8 as *const libc::c_char,
    );
    crate::libbb::xfuncs_printf::bb_xioctl(
      fd,
      0u32 << 0 + 8i32 + 8i32 + 14i32
        | (('p' as i32) << 0 + 8i32) as libc::c_uint
        | (0x1i32 << 0) as libc::c_uint
        | (0i32 << 0 + 8i32 + 8i32) as libc::c_uint,
      0 as *mut libc::c_void,
      b"RTC_AIE_ON\x00" as *const u8 as *const libc::c_char,
    );
  } else {
    /* avoid an extra AIE_ON call */
    wake.enabled = 1i32 as libc::c_uchar; /* for compiler */
    crate::libbb::xfuncs_printf::bb_xioctl(
      fd,
      ((1u32 << 0 + 8i32 + 8i32 + 14i32
        | (('p' as i32) << 0 + 8i32) as libc::c_uint
        | (0xfi32 << 0) as libc::c_uint) as libc::c_ulong
        | (::std::mem::size_of::<linux_rtc_wkalrm>() as libc::c_ulong) << 0 + 8i32 + 8i32)
        as libc::c_uint,
      &mut wake as *mut linux_rtc_wkalrm as *mut libc::c_void,
      b"RTC_WKALM_SET\x00" as *const u8 as *const libc::c_char,
    );
  };
}
pub unsafe fn rtcwake_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opt: libc::c_uint = 0;
  let mut rtcname: *const libc::c_char = std::ptr::null();
  let mut suspend: *const libc::c_char = b"standby\x00" as *const u8 as *const libc::c_char;
  let mut opt_seconds: *const libc::c_char = std::ptr::null();
  let mut opt_time: *const libc::c_char = std::ptr::null();
  let mut rtc_time: time_t = 0;
  let mut sys_time: time_t = 0;
  let mut alarm_time: time_t = 0;
  alarm_time = alarm_time;
  let mut seconds: libc::c_uint = 0;
  seconds = seconds;
  let mut utc: libc::c_int = -1i32;
  let mut fd: libc::c_int = 0;
  static mut rtcwake_longopts: [libc::c_char; 55] = [
    97, 117, 116, 111, 0, 0, 97, 108, 111, 99, 97, 108, 0, 0, 108, 117, 116, 99, 0, 0, 117, 100,
    101, 118, 105, 99, 101, 0, 1, 100, 109, 111, 100, 101, 0, 1, 109, 115, 101, 99, 111, 110, 100,
    115, 0, 1, 115, 116, 105, 109, 101, 0, 1, 116, 0,
  ];
  opt = crate::libbb::getopt32::getopt32long(
    argv,
    b"^alud:m:s:t:\x00s:t:s--t:t--s\x00" as *const u8 as *const libc::c_char,
    rtcwake_longopts.as_ptr(),
    &mut rtcname as *mut *const libc::c_char,
    &mut suspend as *mut *const libc::c_char,
    &mut opt_seconds as *mut *const libc::c_char,
    &mut opt_time as *mut *const libc::c_char,
  );
  /* this is the default
  if (opt & RTCWAKE_OPT_AUTO)
    utc = -1;
  */
  if opt & (0x4i32 | 0x2i32) as libc::c_uint != 0 {
    utc = (opt & 0x4i32 as libc::c_uint) as libc::c_int
  }
  if opt & 0x20i32 as libc::c_uint != 0 {
    /* alarm time, seconds-to-sleep (relative) */
    seconds = crate::libbb::xatonum::xatou(opt_seconds)
  } else if ::std::mem::size_of::<time_t>() as libc::c_ulong
    <= ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
  {
    alarm_time = xatol(opt_time)
  } else {
    alarm_time = crate::libbb::xatonum::xatoll(opt_time) as time_t
  }
  if utc == -1i32 {
    utc = crate::libbb::rtc::rtc_adjtime_is_utc()
  }
  /* RTCWAKE_OPT_TIME */
  /* alarm time, time_t (absolute, seconds since 1/1 1970 UTC) */
  /* the rtcname is relative to /dev */
  crate::libbb::xfuncs_printf::xchdir(b"/dev\x00" as *const u8 as *const libc::c_char);
  /* this RTC must exist and (if we'll sleep) be wakeup-enabled */
  fd = crate::libbb::rtc::rtc_xopen(&mut rtcname, 0);
  if strcmp(suspend, b"on\x00" as *const u8 as *const libc::c_char) != 0 {
    if !may_wakeup(rtcname) {
      crate::libbb::verror_msg::bb_error_msg_and_die(
        b"%s not enabled for wakeup events\x00" as *const u8 as *const libc::c_char,
        rtcname,
      );
    }
  }
  /* relative or absolute alarm time, normalized to time_t */
  sys_time = time(0 as *mut time_t);
  let mut tm_time: tm = std::mem::zeroed();
  crate::libbb::rtc::rtc_read_tm(&mut tm_time, fd);
  rtc_time = crate::libbb::rtc::rtc_tm2time(&mut tm_time, utc);
  if opt & 0x40i32 as libc::c_uint != 0 {
    /* Correct for RTC<->system clock difference */
    alarm_time += rtc_time - sys_time;
    if alarm_time < rtc_time {
      /*
       * Compat message text.
       * I'd say "RTC time is already ahead of ..." instead.
       */
      crate::libbb::verror_msg::bb_error_msg_and_die(
        b"time doesn\'t go backward to %s\x00" as *const u8 as *const libc::c_char,
        ctime(&mut alarm_time),
      );
    }
  } else {
    alarm_time = rtc_time + seconds as libc::c_long + 1
  }
  setup_alarm(fd, &mut alarm_time, rtc_time);
  sync();
  /*debug*/
  printf(
    b"wakeup from \"%s\" at %s\x00" as *const u8 as *const libc::c_char,
    suspend,
    ctime(&mut alarm_time),
  );
  crate::libbb::xfuncs_printf::fflush_all();
  usleep((10i32 * 1000i32) as useconds_t);
  if strcmp(suspend, b"on\x00" as *const u8 as *const libc::c_char) != 0 {
    crate::libbb::write::xopen_xwrite_close(
      b"/sys/power/state\x00" as *const u8 as *const libc::c_char,
      suspend,
    );
  } else {
    /* "fake" suspend ... we'll do the delay ourselves */
    let mut data: libc::c_ulong = 0;
    loop {
      let mut ret: ssize_t = crate::libbb::read::safe_read(
        fd,
        &mut data as *mut libc::c_ulong as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong,
      );
      if ret < 0 {
        crate::libbb::perror_msg::bb_simple_perror_msg(
          b"rtc read\x00" as *const u8 as *const libc::c_char,
        );
        break;
      } else if !(data & 0x20i32 as libc::c_ulong == 0) {
        break;
      }
    }
  }
  crate::libbb::xfuncs_printf::bb_xioctl(
    fd,
    0u32 << 0 + 8i32 + 8i32 + 14i32
      | (('p' as i32) << 0 + 8i32) as libc::c_uint
      | (0x2i32 << 0) as libc::c_uint
      | (0i32 << 0 + 8i32 + 8i32) as libc::c_uint,
    0 as *mut libc::c_void,
    b"RTC_AIE_OFF\x00" as *const u8 as *const libc::c_char,
  );
  return 0;
}

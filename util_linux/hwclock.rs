use libc;
use libc::printf;
use libc::suseconds_t;
use libc::time_t;
use libc::timeval;
use libc::tm;
extern "C" {

  #[no_mangle]
  fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> libc::c_int;

  #[no_mangle]
  fn settimeofday(__tv: *const timeval, __tz: *const timezone) -> libc::c_int;

  #[no_mangle]
  fn localtime(__timer: *const time_t) -> *mut tm;

  #[no_mangle]
  fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;

  #[no_mangle]
  fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;

  #[no_mangle]
  fn ctime(__timer: *const time_t) -> *mut libc::c_char;

  #[no_mangle]
  fn tzset();

  #[no_mangle]
  static mut timezone: libc::c_long;

/*
 * Common defines/structures/etc... for applets that need to work with the RTC.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timezone {
  pub tz_minuteswest: libc::c_int,
  pub tz_dsttime: libc::c_int,
}
pub type __timezone_ptr_t = *mut timezone;

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

/*
 * Mini hwclock implementation for busybox
 *
 * Copyright (C) 2002 Robert Griebl <griebl@gmx.de>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config HWCLOCK
//config:	bool "hwclock (5.8 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	The hwclock utility is used to read and set the hardware clock
//config:	on a system. This is primarily used to set the current time on
//config:	shutdown in the hardware clock, so the hardware will keep the
//config:	correct time when Linux is _not_ running.
//config:
//config:config FEATURE_HWCLOCK_ADJTIME_FHS
//config:	bool "Use FHS /var/lib/hwclock/adjtime"
//config:	default n  # util-linux-ng in Fedora 13 still uses /etc/adjtime
//config:	depends on HWCLOCK
//config:	help
//config:	Starting with FHS 2.3, the adjtime state file is supposed to exist
//config:	at /var/lib/hwclock/adjtime instead of /etc/adjtime. If you wish
//config:	to use the FHS behavior, answer Y here, otherwise answer N for the
//config:	classic /etc/adjtime path.
//config:
//config:	pathname.com/fhs/pub/fhs-2.3.html#VARLIBHWCLOCKSTATEDIRECTORYFORHWCLO
//applet:IF_HWCLOCK(APPLET(hwclock, BB_DIR_SBIN, SUID_DROP))
//kbuild:lib-$(CONFIG_HWCLOCK) += hwclock.o
/* After libbb.h, since it needs sys/types.h on some systems */
/* diff code is disabled: it's not sys/hw clock diff, it's some useless
 * "time between hwclock was started and we saw CMOS tick" quantity.
 * It's useless since hwclock is started at a random moment,
 * thus the quantity is also random, useless. Showing 0.000000 does not
 * deprive us from any useful info.
 *
 * SHOW_HWCLOCK_DIFF code in this file shows the difference between system
 * and hw clock. It is useful, but not compatible with standard hwclock.
 * Thus disabled.
 */
unsafe extern "C" fn read_rtc(
  mut pp_rtcname: *mut *const libc::c_char,
  mut utc: libc::c_int,
) -> time_t {
  let mut tm_time: tm = std::mem::zeroed();
  let mut fd: libc::c_int = 0;
  fd = crate::libbb::rtc::rtc_xopen(pp_rtcname, 0i32);
  crate::libbb::rtc::rtc_read_tm(&mut tm_time, fd);
  return crate::libbb::rtc::rtc_tm2time(&mut tm_time, utc);
}
unsafe extern "C" fn show_clock(mut pp_rtcname: *mut *const libc::c_char, mut utc: libc::c_int) {
  let mut t: time_t = read_rtc(pp_rtcname, utc);
  let mut cp: *mut libc::c_char = ctime(&mut t);
  crate::libbb::chomp::chomp(cp);
  printf(
    b"%s  0.000000 seconds\n\x00" as *const u8 as *const libc::c_char,
    cp,
  );
}
unsafe extern "C" fn to_sys_clock(mut pp_rtcname: *mut *const libc::c_char, mut utc: libc::c_int) {
  let mut tv: timeval = timeval {
    tv_sec: 0,
    tv_usec: 0,
  };
  let mut tz: timezone = timezone {
    tz_minuteswest: 0,
    tz_dsttime: 0,
  };
  tz.tz_minuteswest = (timezone / 60i32 as libc::c_long) as libc::c_int;
  /* ^^^ used to also subtract 60*daylight, but it's wrong:
   * daylight!=0 means "this timezone has some DST
   * during the year", not "DST is in effect now".
   */
  tz.tz_dsttime = 0i32;
  tv.tv_sec = read_rtc(pp_rtcname, utc);
  tv.tv_usec = 0i32 as suseconds_t;
  if settimeofday(&mut tv, &mut tz) != 0 {
    crate::libbb::perror_msg::bb_simple_perror_msg_and_die(
      b"settimeofday\x00" as *const u8 as *const libc::c_char,
    );
  };
}
unsafe extern "C" fn from_sys_clock(
  mut pp_rtcname: *mut *const libc::c_char,
  mut utc: libc::c_int,
) {
  let mut tv: timeval = timeval {
    tv_sec: 0,
    tv_usec: 0,
  };
  let mut tm_time: tm = std::mem::zeroed();
  let mut rtc: libc::c_int = 0;
  rtc = crate::libbb::rtc::rtc_xopen(pp_rtcname, 0o1i32);
  gettimeofday(&mut tv, 0 as *mut timezone);
  /* Prepare tm_time */
  if ::std::mem::size_of::<time_t>() as libc::c_ulong
    == ::std::mem::size_of::<time_t>() as libc::c_ulong
  {
    if utc != 0 {
      gmtime_r(&mut tv.tv_sec as *mut time_t as *mut time_t, &mut tm_time);
    } else {
      localtime_r(&mut tv.tv_sec as *mut time_t as *mut time_t, &mut tm_time);
    }
  } else {
    let mut t: time_t = tv.tv_sec;
    if utc != 0 {
      gmtime_r(&mut t, &mut tm_time);
    } else {
      localtime_r(&mut t, &mut tm_time);
    }
  }
  tm_time.tm_isdst = 0i32;
  crate::libbb::xfuncs_printf::bb_xioctl(
    rtc,
    ((1u32 << 0i32 + 8i32 + 8i32 + 14i32
      | (('p' as i32) << 0i32 + 8i32) as libc::c_uint
      | (0xai32 << 0i32) as libc::c_uint) as libc::c_ulong
      | (::std::mem::size_of::<linux_rtc_time>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
      as libc::c_uint,
    &mut tm_time as *mut tm as *mut libc::c_void,
    b"RTC_SET_TIME\x00" as *const u8 as *const libc::c_char,
  );
}
/*
 * At system boot, kernel may set system time from RTC,
 * but it knows nothing about timezones. If RTC is in local time,
 * then system time is wrong - it is offset by timezone.
 * This option corrects system time if RTC is in local time,
 * and (always) sets in-kernel timezone.
 *
 * This is an alternate option to --hctosys that does not read the
 * hardware clock.
 */
unsafe extern "C" fn set_system_clock_timezone(mut utc: libc::c_int) {
  let mut tv: timeval = timeval {
    tv_sec: 0,
    tv_usec: 0,
  };
  let mut broken: *mut tm = 0 as *mut tm;
  let mut tz: timezone = timezone {
    tz_minuteswest: 0,
    tz_dsttime: 0,
  };
  gettimeofday(&mut tv, 0 as *mut timezone);
  broken = localtime(&mut tv.tv_sec);
  tz.tz_minuteswest = (timezone / 60i32 as libc::c_long) as libc::c_int;
  if (*broken).tm_isdst > 0i32 {
    tz.tz_minuteswest -= 60i32
  }
  tz.tz_dsttime = 0i32;
  gettimeofday(&mut tv, 0 as *mut timezone);
  if utc == 0 {
    tv.tv_sec += (tz.tz_minuteswest * 60i32) as libc::c_long
  }
  if settimeofday(&mut tv, &mut tz) != 0 {
    crate::libbb::perror_msg::bb_simple_perror_msg_and_die(
      b"settimeofday\x00" as *const u8 as *const libc::c_char,
    );
  };
}
#[no_mangle]
pub unsafe extern "C" fn hwclock_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut rtcname: *const libc::c_char = 0 as *const libc::c_char;
  let mut opt: libc::c_uint = 0;
  let mut utc: libc::c_int = 0;
  static mut hwclock_longopts: [libc::c_char; 60] = [
    108, 111, 99, 97, 108, 116, 105, 109, 101, 0, 0, 108, 117, 116, 99, 0, 0, 117, 115, 104, 111,
    119, 0, 0, 114, 104, 99, 116, 111, 115, 121, 115, 0, 0, 115, 115, 121, 115, 116, 111, 104, 99,
    0, 0, 119, 115, 121, 115, 116, 122, 0, 0, 116, 114, 116, 99, 0, 1, 102, 0,
  ];
  /* Initialize "timezone" (libc global variable) */
  tzset();
  opt = crate::libbb::getopt32::getopt32long(
    argv,
    b"^lurswtf:\x00r--wst:w--rst:s--wrt:t--rsw:l--u:u--l\x00" as *const u8 as *const libc::c_char,
    hwclock_longopts.as_ptr(),
    &mut rtcname as *mut *const libc::c_char,
  );
  /* If -u or -l wasn't given check if we are using utc */
  if opt & (0x2i32 | 0x1i32) as libc::c_uint != 0 {
    utc = (opt & 0x2i32 as libc::c_uint) as libc::c_int
  } else {
    utc = crate::libbb::rtc::rtc_adjtime_is_utc()
  }
  if opt & 0x8i32 as libc::c_uint != 0 {
    to_sys_clock(&mut rtcname, utc);
  } else if opt & 0x10i32 as libc::c_uint != 0 {
    from_sys_clock(&mut rtcname, utc);
  } else if opt & 0x20i32 as libc::c_uint != 0 {
    set_system_clock_timezone(utc);
  } else {
    /* default HWCLOCK_OPT_SHOW */
    show_clock(&mut rtcname, utc);
  }
  return 0i32;
}

use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn time(__timer: *mut time_t) -> time_t;
  #[no_mangle]
  fn strftime(
    __s: *mut libc::c_char,
    __maxsize: size_t,
    __format: *const libc::c_char,
    __tp: *const tm,
  ) -> size_t;
  #[no_mangle]
  fn localtime(__timer: *const time_t) -> *mut tm;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
  #[no_mangle]
  fn xatou_range(str: *const libc::c_char, l: libc::c_uint, u: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  static mut option_mask32: uint32_t;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_show_usage() -> !;
  //UNUSED: char* FAST_FUNC unicode_conv_to_printable_maxwidth(uni_stat_t *stats, const char *src, unsigned maxwidth);
  #[no_mangle]
  fn unicode_conv_to_printable_fixedwidth(
    src: *const libc::c_char,
    width: libc::c_uint,
  ) -> *mut libc::c_char;
}

use crate::librb::size_t;
use crate::librb::time_t;
use crate::librb::uint32_t;
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
/* used in day array */
static mut days_in_month: [libc::c_uchar; 13] = [
  0i32 as libc::c_uchar,
  31i32 as libc::c_uchar,
  28i32 as libc::c_uchar,
  31i32 as libc::c_uchar,
  30i32 as libc::c_uchar,
  31i32 as libc::c_uchar,
  30i32 as libc::c_uchar,
  31i32 as libc::c_uchar,
  31i32 as libc::c_uchar,
  30i32 as libc::c_uchar,
  31i32 as libc::c_uchar,
  30i32 as libc::c_uchar,
  31i32 as libc::c_uchar,
];
static mut sep1752: [libc::c_uchar; 19] = [
  1i32 as libc::c_uchar,
  2i32 as libc::c_uchar,
  14i32 as libc::c_uchar,
  15i32 as libc::c_uchar,
  16i32 as libc::c_uchar,
  17i32 as libc::c_uchar,
  18i32 as libc::c_uchar,
  19i32 as libc::c_uchar,
  20i32 as libc::c_uchar,
  21i32 as libc::c_uchar,
  22i32 as libc::c_uchar,
  23i32 as libc::c_uchar,
  24i32 as libc::c_uchar,
  25i32 as libc::c_uchar,
  26i32 as libc::c_uchar,
  27i32 as libc::c_uchar,
  28i32 as libc::c_uchar,
  29i32 as libc::c_uchar,
  30i32 as libc::c_uchar,
];
/* Set to 0 or 1 in main */
/* leap year -- account for Gregorian reformation in 1752 */
unsafe extern "C" fn leap_year(mut yr: libc::c_uint) -> libc::c_int {
  if yr <= 1752i32 as libc::c_uint {
    return (yr.wrapping_rem(4i32 as libc::c_uint) == 0) as libc::c_int;
  }
  return (yr.wrapping_rem(4i32 as libc::c_uint) == 0
    && yr.wrapping_rem(100i32 as libc::c_uint) != 0
    || yr.wrapping_rem(400i32 as libc::c_uint) == 0) as libc::c_int;
}
/* spaces between day headings */
#[no_mangle]
pub unsafe extern "C" fn cal_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut zero_tm: tm = tm {
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
  let mut now: time_t = 0;
  let mut month: libc::c_uint = 0;
  let mut year: libc::c_uint = 0;
  let mut flags: libc::c_uint = 0;
  let mut i: libc::c_uint = 0;
  let mut month_names: [*mut libc::c_char; 12] = [0 as *mut libc::c_char; 12];
  /* normal heading: */
  /* "Su Mo Tu We Th Fr Sa" */
  /* -j heading: */
  /* " Su  Mo  Tu  We  Th  Fr  Sa" */
  let mut day_headings: [libc::c_char; 168] = [0; 168];
  let mut hp: *mut libc::c_char = day_headings.as_mut_ptr();
  let mut buf: [libc::c_char; 40] = [0; 40];
  flags = getopt32(argv, b"jy\x00" as *const u8 as *const libc::c_char);
  /* This sets julian = flags & 1: */
  option_mask32 &= 1i32 as libc::c_uint;
  month = 0i32 as libc::c_uint;
  argv = argv.offset(optind as isize);
  if (*argv.offset(0)).is_null() {
    let mut ptm: *mut tm = 0 as *mut tm;
    time(&mut now);
    ptm = localtime(&mut now);
    year = ((*ptm).tm_year + 1900i32) as libc::c_uint;
    if flags & 2i32 as libc::c_uint == 0 {
      /* no -y */
      month = ((*ptm).tm_mon + 1i32) as libc::c_uint
    }
  } else {
    if !(*argv.offset(1)).is_null() {
      if !(*argv.offset(2)).is_null() {
        bb_show_usage();
      }
      if flags & 2i32 as libc::c_uint == 0 {
        /* no -y */
        month = xatou_range(*argv, 1i32 as libc::c_uint, 12i32 as libc::c_uint)
      }
      argv = argv.offset(1)
    }
    year = xatou_range(*argv, 1i32 as libc::c_uint, 9999i32 as libc::c_uint)
  }
  blank_string(
    day_headings.as_mut_ptr(),
    (::std::mem::size_of::<[libc::c_char; 168]>() as libc::c_ulong)
      .wrapping_sub(7i32 as libc::c_ulong)
      .wrapping_add((7i32 as libc::c_uint).wrapping_mul(option_mask32) as libc::c_ulong),
  );
  i = 0i32 as libc::c_uint;
  loop {
    zero_tm.tm_mon = i as libc::c_int;
    /* full month name according to locale */
    strftime(
      buf.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
      b"%B\x00" as *const u8 as *const libc::c_char,
      &mut zero_tm,
    );
    month_names[i as usize] = xstrdup(buf.as_mut_ptr());
    if i < 7i32 as libc::c_uint {
      zero_tm.tm_wday = i as libc::c_int;
      /* abbreviated weekday name according to locale */
      strftime(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
        b"%a\x00" as *const u8 as *const libc::c_char,
        &mut zero_tm,
      ); /* two \n's */
      if option_mask32 != 0 {
        let fresh0 = hp;
        hp = hp.offset(1);
        *fresh0 = ' ' as i32 as libc::c_char
      }
      let mut two_wchars: *mut libc::c_char =
        unicode_conv_to_printable_fixedwidth(buf.as_mut_ptr(), 2i32 as libc::c_uint);
      strcpy(hp, two_wchars);
      free(two_wchars as *mut libc::c_void);
      hp = hp.offset(strlen(hp) as isize);
      let fresh1 = hp;
      hp = hp.offset(1);
      *fresh1 = ' ' as i32 as libc::c_char
    }
    i = i.wrapping_add(1);
    if !(i < 12i32 as libc::c_uint) {
      break;
    }
  }
  *hp.offset(-1i32 as isize) = '\u{0}' as i32 as libc::c_char;
  if month != 0 {
    let mut row: libc::c_uint = 0;
    let mut len: libc::c_uint = 0;
    let mut days: [libc::c_uint; 42] = [0; 42];
    let mut dp: *mut libc::c_uint = days.as_mut_ptr();
    let mut lineout: [libc::c_char; 30] = [0; 30];
    day_array(month, year, dp);
    len = sprintf(
      lineout.as_mut_ptr(),
      b"%s %u\x00" as *const u8 as *const libc::c_char,
      month_names[month.wrapping_sub(1i32 as libc::c_uint) as usize],
      year,
    ) as libc::c_uint;
    printf(
      b"%*s%s\n%s\n\x00" as *const u8 as *const libc::c_char,
      (7i32 as libc::c_uint)
        .wrapping_mul(option_mask32)
        .wrapping_add(20i32 as libc::c_uint)
        .wrapping_sub(len)
        .wrapping_div(2i32 as libc::c_uint),
      b"\x00" as *const u8 as *const libc::c_char,
      lineout.as_mut_ptr(),
      day_headings.as_mut_ptr(),
    );
    row = 0i32 as libc::c_uint;
    while row < 6i32 as libc::c_uint {
      *build_row(lineout.as_mut_ptr(), dp).offset(0) = '\u{0}' as i32 as libc::c_char;
      dp = dp.offset(7);
      trim_trailing_spaces_and_print(lineout.as_mut_ptr());
      row = row.wrapping_add(1)
    }
  } else {
    let mut row_0: libc::c_uint = 0;
    let mut which_cal: libc::c_uint = 0;
    let mut week_len: libc::c_uint = 0;
    let mut days_0: [[libc::c_uint; 42]; 12] = [[0; 42]; 12];
    let mut dp_0: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut lineout_0: [libc::c_char; 80] = [0; 80];
    sprintf(
      lineout_0.as_mut_ptr(),
      b"%u\x00" as *const u8 as *const libc::c_char,
      year,
    );
    center(
      lineout_0.as_mut_ptr(),
      ((20i32 * 3i32 + 2i32 * 2i32) as libc::c_uint).wrapping_add(option_mask32.wrapping_mul(
        ((20i32 + 7i32) * 2i32 + 2i32 - (20i32 * 3i32 + 2i32 * 2i32)) as libc::c_uint,
      )),
      0i32 as libc::c_uint,
    );
    puts(b"\n\x00" as *const u8 as *const libc::c_char);
    i = 0i32 as libc::c_uint;
    while i < 12i32 as libc::c_uint {
      day_array(
        i.wrapping_add(1i32 as libc::c_uint),
        year,
        days_0[i as usize].as_mut_ptr(),
      );
      i = i.wrapping_add(1)
    }
    blank_string(
      lineout_0.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong,
    );
    week_len = (20i32 as libc::c_uint)
      .wrapping_add(option_mask32.wrapping_mul((20i32 + 7i32 - 20i32) as libc::c_uint));
    month = 0i32 as libc::c_uint;
    while month < 12i32 as libc::c_uint {
      center(month_names[month as usize], week_len, 2i32 as libc::c_uint);
      if option_mask32 == 0 {
        center(
          month_names[month.wrapping_add(1i32 as libc::c_uint) as usize],
          week_len,
          2i32 as libc::c_uint,
        );
      }
      center(
        month_names[month
          .wrapping_add(2i32 as libc::c_uint)
          .wrapping_sub(option_mask32) as usize],
        week_len,
        0i32 as libc::c_uint,
      );
      printf(
        b"\n%s%*s%s\x00" as *const u8 as *const libc::c_char,
        day_headings.as_mut_ptr(),
        2i32,
        b"\x00" as *const u8 as *const libc::c_char,
        day_headings.as_mut_ptr(),
      );
      if option_mask32 == 0 {
        printf(
          b"%*s%s\x00" as *const u8 as *const libc::c_char,
          2i32,
          b"\x00" as *const u8 as *const libc::c_char,
          day_headings.as_mut_ptr(),
        );
      }
      bb_putchar('\n' as i32);
      row_0 = 0i32 as libc::c_uint;
      while row_0 < (6i32 * 7i32) as libc::c_uint {
        which_cal = 0i32 as libc::c_uint;
        while which_cal < (3i32 as libc::c_uint).wrapping_sub(option_mask32) {
          dp_0 = days_0[month.wrapping_add(which_cal) as usize]
            .as_mut_ptr()
            .offset(row_0 as isize);
          build_row(
            lineout_0
              .as_mut_ptr()
              .offset(which_cal.wrapping_mul(week_len.wrapping_add(2i32 as libc::c_uint)) as isize),
            dp_0,
          );
          which_cal = which_cal.wrapping_add(1)
        }
        /* blank_string took care of nul termination. */
        trim_trailing_spaces_and_print(lineout_0.as_mut_ptr());
        row_0 = row_0.wrapping_add(7i32 as libc::c_uint)
      }
      month = month.wrapping_add((3i32 as libc::c_uint).wrapping_sub(option_mask32))
    }
  }
  fflush_stdout_and_exit(0i32);
}
/*
 * day_array --
 *	Fill in an array of 42 integers with a calendar.  Assume for a moment
 *	that you took the (maximum) 6 rows in a calendar and stretched them
 *	out end to end.  You would have 42 numbers or spaces.  This routine
 *	builds that array for any month from Jan. 1 through Dec. 9999.
 */
unsafe extern "C" fn day_array(
  mut month: libc::c_uint,
  mut year: libc::c_uint,
  mut days: *mut libc::c_uint,
) {
  let mut temp: libc::c_ulong = 0;
  let mut i: libc::c_uint = 0;
  let mut day: libc::c_uint = 0;
  let mut dw: libc::c_uint = 0;
  let mut dm: libc::c_uint = 0;
  memset(
    days as *mut libc::c_void,
    -1i32,
    (42i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
  );
  if month == 9i32 as libc::c_uint && year == 1752i32 as libc::c_uint {
    /* Assumes the Gregorian reformation eliminates
     * 3 Sep. 1752 through 13 Sep. 1752.
     */
    let mut j_offset: libc::c_uint = option_mask32.wrapping_mul(244i32 as libc::c_uint);
    let mut oday: size_t = 0i32 as size_t;
    loop {
      *days.offset(oday.wrapping_add(2i32 as libc::c_ulong) as isize) =
        (sep1752[oday as usize] as libc::c_uint).wrapping_add(j_offset);
      oday = oday.wrapping_add(1);
      if !(oday < ::std::mem::size_of::<[libc::c_uchar; 19]>() as libc::c_ulong) {
        break;
      }
    }
    return;
  }
  /* day_in_year
   * return the 1 based day number within the year
   */
  day = 1i32 as libc::c_uint;
  if month > 2i32 as libc::c_uint && leap_year(year) != 0 {
    day = day.wrapping_add(1)
  }
  i = month;
  while i != 0 {
    i = i.wrapping_sub(1);
    day = day.wrapping_add(days_in_month[i as usize] as libc::c_uint)
  }
  /* day_in_week
   * return the 0 based day number for any date from 1 Jan. 1 to
   * 31 Dec. 9999.  Assumes the Gregorian reformation eliminates
   * 3 Sep. 1752 through 13 Sep. 1752.  Returns Thursday for all
   * missing days.
   */
  temp = (year.wrapping_sub(1i32 as libc::c_uint) as libc::c_long * 365i32 as libc::c_long
    + year
      .wrapping_sub(1i32 as libc::c_uint)
      .wrapping_div(4i32 as libc::c_uint)
      .wrapping_sub(
        if year.wrapping_sub(1i32 as libc::c_uint) > 1700i32 as libc::c_uint {
          year
            .wrapping_sub(1i32 as libc::c_uint)
            .wrapping_div(100i32 as libc::c_uint)
            .wrapping_sub(17i32 as libc::c_uint)
        } else {
          0i32 as libc::c_uint
        },
      )
      .wrapping_add(
        if year.wrapping_sub(1i32 as libc::c_uint) > 1600i32 as libc::c_uint {
          year
            .wrapping_sub(1i32 as libc::c_uint)
            .wrapping_sub(1600i32 as libc::c_uint)
            .wrapping_div(400i32 as libc::c_uint)
        } else {
          0i32 as libc::c_uint
        },
      ) as libc::c_long
    + day as libc::c_long) as libc::c_ulong;
  if temp < 639787i32 as libc::c_ulong {
    dw = temp
      .wrapping_sub(1i32 as libc::c_ulong)
      .wrapping_add(6i32 as libc::c_ulong)
      .wrapping_rem(7i32 as libc::c_ulong) as libc::c_uint
  } else {
    dw = temp
      .wrapping_sub(1i32 as libc::c_ulong)
      .wrapping_add(6i32 as libc::c_ulong)
      .wrapping_sub(11i32 as libc::c_ulong)
      .wrapping_rem(7i32 as libc::c_ulong) as libc::c_uint
  }
  if option_mask32 == 0 {
    day = 1i32 as libc::c_uint
  }
  dm = days_in_month[month as usize] as libc::c_uint;
  if month == 2i32 as libc::c_uint && leap_year(year) != 0 {
    dm = dm.wrapping_add(1)
  }
  loop {
    let fresh2 = day;
    day = day.wrapping_add(1);
    let fresh3 = dw;
    dw = dw.wrapping_add(1);
    *days.offset(fresh3 as isize) = fresh2;
    dm = dm.wrapping_sub(1);
    if !(dm != 0) {
      break;
    }
  }
}
unsafe extern "C" fn trim_trailing_spaces_and_print(mut s: *mut libc::c_char) {
  let mut p: *mut libc::c_char = s;
  while *p != 0 {
    p = p.offset(1)
  }
  while p != s {
    p = p.offset(-1);
    if !(({
      let mut bb__isspace: libc::c_uchar = (*p as libc::c_int - 9i32) as libc::c_uchar;
      (bb__isspace as libc::c_int == ' ' as i32 - 9i32
        || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
    }) == 0)
    {
      continue;
    }
    *p.offset(1) = '\u{0}' as i32 as libc::c_char;
    break;
  }
  puts(s);
}
/* number of centuries since 1700, not inclusive */
/* number of centuries since 1700 whose modulo of 400 is 0 */
/* number of leap years between year 1 and this year, not inclusive */
unsafe extern "C" fn center(
  mut str: *mut libc::c_char,
  mut len: libc::c_uint,
  mut separate: libc::c_uint,
) {
  let mut n: libc::c_uint = strlen(str) as libc::c_uint;
  len = len.wrapping_sub(n);
  printf(
    b"%*s%*s\x00" as *const u8 as *const libc::c_char,
    len.wrapping_div(2i32 as libc::c_uint).wrapping_add(n),
    str,
    len
      .wrapping_div(2i32 as libc::c_uint)
      .wrapping_add(len.wrapping_rem(2i32 as libc::c_uint))
      .wrapping_add(separate),
    b"\x00" as *const u8 as *const libc::c_char,
  );
}
unsafe extern "C" fn blank_string(mut buf: *mut libc::c_char, mut buflen: size_t) {
  memset(buf as *mut libc::c_void, ' ' as i32, buflen);
  *buf.offset(buflen.wrapping_sub(1i32 as libc::c_ulong) as isize) = '\u{0}' as i32 as libc::c_char;
}
unsafe extern "C" fn build_row(
  mut p: *mut libc::c_char,
  mut dp: *mut libc::c_uint,
) -> *mut libc::c_char {
  let mut col: libc::c_uint = 0;
  let mut val: libc::c_uint = 0;
  let mut day: libc::c_uint = 0;
  memset(
    p as *mut libc::c_void,
    ' ' as i32,
    option_mask32
      .wrapping_add(3i32 as libc::c_uint)
      .wrapping_mul(7i32 as libc::c_uint) as libc::c_ulong,
  );
  col = 0i32 as libc::c_uint;
  loop {
    let fresh4 = dp;
    dp = dp.offset(1);
    day = *fresh4;
    if day != -1i32 as libc::c_uint {
      if option_mask32 != 0 {
        p = p.offset(1);
        if day >= 100i32 as libc::c_uint {
          *p = '0' as i32 as libc::c_char;
          *p.offset(-1i32 as isize) = day
            .wrapping_div(100i32 as libc::c_uint)
            .wrapping_add('0' as i32 as libc::c_uint)
            as libc::c_char;
          day = day.wrapping_rem(100i32 as libc::c_uint)
        }
      }
      val = day.wrapping_div(10i32 as libc::c_uint);
      if val > 0i32 as libc::c_uint {
        *p = val.wrapping_add('0' as i32 as libc::c_uint) as libc::c_char
      }
      p = p.offset(1);
      *p = day
        .wrapping_rem(10i32 as libc::c_uint)
        .wrapping_add('0' as i32 as libc::c_uint) as libc::c_char;
      p = p.offset(2)
    } else {
      p = p.offset((3i32 as libc::c_uint).wrapping_add(option_mask32) as isize)
    }
    col = col.wrapping_add(1);
    if !(col < 7i32 as libc::c_uint) {
      break;
    }
  }
  return p;
}
/*
 * Copyright (c) 1989, 1993, 1994
 *	The Regents of the University of California.  All rights reserved.
 *
 * This code is derived from software contributed to Berkeley by
 * Kim Letkeman.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ''AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

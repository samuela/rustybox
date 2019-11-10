use crate::librb::size_t;
use libc;
use libc::putenv;

























use libc::strcpy;

use libc::time;

use libc::puts;

use libc::stat;
use libc::time_t;
use libc::timespec;
use libc::tm;

extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;

  #[no_mangle]
  fn strftime(
    __s: *mut libc::c_char,
    __maxsize: size_t,
    __format: *const libc::c_char,
    __tp: *const tm,
  ) -> size_t;
  #[no_mangle]
  fn strptime(
    __s: *const libc::c_char,
    __fmt: *const libc::c_char,
    __tp: *mut tm,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
  #[no_mangle]
  fn stime(__when: *const time_t) -> libc::c_int;
  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xstat(pathname: *const libc::c_char, buf: *mut stat);
  #[no_mangle]
  fn parse_datestr(date_str: *const libc::c_char, ptm: *mut tm);
  #[no_mangle]
  fn validate_tm_time(date_str: *const libc::c_char, ptm: *mut tm) -> time_t;
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
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn index_in_substrings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static bb_msg_invalid_date: [libc::c_char; 0];
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

pub type C2RustUnnamed = libc::c_uint;
pub const COMMON_BUFSIZE: C2RustUnnamed = 1024;

/*
 * Mini date implementation for busybox
 *
 * by Matthew Grant <grantma@anathoth.gen.nz>
 *
 * iso-format handling added by Robert Griebl <griebl@gmx.de>
 * bugfixes and cleanup by Bernhard Reutner-Fischer
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* This 'date' command supports only 2 time setting formats,
all the GNU strftime stuff (its in libc, lets use it),
setting time using UTC and displaying it, as well as
an RFC 2822 compliant date output for shell scripting
mail commands */
/* Input parsing code is always bulky - used heavy duty libc stuff as
much as possible, missed out a lot of bounds checking */
//config:config DATE
//config:	bool "date (7 kb)"
//config:	default y
//config:	help
//config:	date is used to set the system date or display the
//config:	current time in the given format.
//config:
//config:config FEATURE_DATE_ISOFMT
//config:	bool "Enable ISO date format output (-I)"
//config:	default y
//config:	depends on DATE
//config:	help
//config:	Enable option (-I) to output an ISO-8601 compliant
//config:	date/time string.
//config:
//config:config FEATURE_DATE_NANO
//config:	bool "Support %[num]N nanosecond format specifier"
//config:	default n # stat's nanosecond field is a bit non-portable
//config:	depends on DATE
//config:	select PLATFORM_LINUX
//config:	help
//config:	Support %[num]N format specifier. Adds ~250 bytes of code.
//config:
//config:config FEATURE_DATE_COMPAT
//config:	bool "Support weird 'date MMDDhhmm[[YY]YY][.ss]' format"
//config:	default y
//config:	depends on DATE
//config:	help
//config:	System time can be set by 'date -s DATE' and simply 'date DATE',
//config:	but formats of DATE string are different. 'date DATE' accepts
//config:	a rather weird MMDDhhmm[[YY]YY][.ss] format with completely
//config:	unnatural placement of year between minutes and seconds.
//config:	date -s (and other commands like touch -d) use more sensible
//config:	formats (for one, ISO format YYYY-MM-DD hh:mm:ss.ssssss).
//config:
//config:	With this option off, 'date DATE' and 'date -s DATE' support
//config:	the same format. With it on, 'date DATE' additionally supports
//config:	MMDDhhmm[[YY]YY][.ss] format.
//applet:IF_DATE(APPLET_NOEXEC(date, date, BB_DIR_BIN, BB_SUID_DROP, date))
/* bb_common_bufsiz1 usage here is safe wrt NOEXEC: not expecting it to be zeroed. */
//kbuild:lib-$(CONFIG_DATE) += date.o
/* GNU coreutils 6.9 man page:
 * date [OPTION]... [+FORMAT]
 * date [-u|--utc|--universal] [MMDDhhmm[[CC]YY][.ss]]
 * -d, --date=STRING
 *      display time described by STRING, not 'now'
 * -f, --file=DATEFILE
 *      like --date once for each line of DATEFILE
 * -r, --reference=FILE
 *      display the last modification time of FILE
 * -R, --rfc-2822
 *      output date and time in RFC 2822 format.
 *      Example: Mon, 07 Aug 2006 12:34:56 -0600
 * --rfc-3339=TIMESPEC
 *      output date and time in RFC 3339 format.
 *      TIMESPEC='date', 'seconds', or 'ns'
 *      Date and time components are separated by a single space:
 *      2006-08-07 12:34:56-06:00
 * -s, --set=STRING
 *      set time described by STRING
 * -u, --utc, --universal
 *      print or set Coordinated Universal Time
 *
 * Busybox:
 * long options are not supported
 * -f is not supported
 * -I seems to roughly match --rfc-3339, but -I has _optional_ param
 *    (thus "-I seconds" doesn't work, only "-Iseconds"),
 *    and does not support -Ins
 * -D FMT is a bbox extension for _input_ conversion of -d DATE
 */
//usage:#define date_trivial_usage
//usage:       "[OPTIONS] [+FMT] [TIME]"
//usage:#define date_full_usage "\n\n"
//usage:       "Display time (using +FMT), or set time\n"
//usage:	IF_NOT_LONG_OPTS(
//usage:     "\n	[-s] TIME	Set time to TIME"
//usage:     "\n	-u		Work in UTC (don't convert to local time)"
//usage:     "\n	-R		Output RFC-2822 compliant date string"
//usage:	) IF_LONG_OPTS(
//usage:     "\n	[-s,--set] TIME	Set time to TIME"
//usage:     "\n	-u,--utc	Work in UTC (don't convert to local time)"
//usage:     "\n	-R,--rfc-2822	Output RFC-2822 compliant date string"
//usage:	)
//usage:	IF_FEATURE_DATE_ISOFMT(
//usage:     "\n	-I[SPEC]	Output ISO-8601 compliant date string"
//usage:     "\n			SPEC='date' (default) for date only,"
//usage:     "\n			'hours', 'minutes', or 'seconds' for date and"
//usage:     "\n			time to the indicated precision"
//usage:	)
//usage:	IF_NOT_LONG_OPTS(
//usage:     "\n	-r FILE		Display last modification time of FILE"
//usage:     "\n	-d TIME		Display TIME, not 'now'"
//usage:	) IF_LONG_OPTS(
//usage:     "\n	-r,--reference FILE	Display last modification time of FILE"
//usage:     "\n	-d,--date TIME	Display TIME, not 'now'"
//usage:	)
//usage:	IF_FEATURE_DATE_ISOFMT(
//usage:     "\n	-D FMT		Use FMT (strptime format) for -d TIME conversion"
//usage:	)
//usage:     "\n"
//usage:     "\nRecognized TIME formats:"
//usage:     "\n	hh:mm[:ss]"
//usage:     "\n	[YYYY.]MM.DD-hh:mm[:ss]"
//usage:     "\n	YYYY-MM-DD hh:mm[:ss]"
//usage:     "\n	[[[[[YY]YY]MM]DD]hh]mm[.ss]"
//usage:	IF_FEATURE_DATE_COMPAT(
//usage:     "\n	'date TIME' form accepts MMDDhhmm[[YY]YY][.ss] instead"
//usage:	)
//usage:
//usage:#define date_example_usage
//usage:       "$ date\n"
//usage:       "Wed Apr 12 18:52:41 MDT 2000\n"

pub type C2RustUnnamed_0 = libc::c_uint;
/* D */
pub const OPT_HINT: C2RustUnnamed_0 = 64;
/* I */
pub const OPT_TIMESPEC: C2RustUnnamed_0 = 32;
/* r */
pub const OPT_REFERENCE: C2RustUnnamed_0 = 16;
/* d */
pub const OPT_DATE: C2RustUnnamed_0 = 8;
/* u */
pub const OPT_UTC: C2RustUnnamed_0 = 4;
/* s */
pub const OPT_SET: C2RustUnnamed_0 = 2;
/* R */
pub const OPT_RFC2822: C2RustUnnamed_0 = 1;

// Originally:
// static const char date_longopts[] ALIGN1 =
// 		"rfc-822\0"   No_argument       "R"
// 		"rfc-2822\0"  No_argument       "R"
// 		"set\0"       Required_argument "s"
// 		"utc\0"       No_argument       "u"
// 	/*	"universal\0" No_argument       "u" */
// 		"date\0"      Required_argument "d"
// 		"reference\0" Required_argument "r"
// 		;
static mut date_longopts: [libc::c_char; 53] = [
  114, 102, 99, 45, 56, 50, 50, 0, 0, 82, 114, 102, 99, 45, 50, 56, 50, 50, 0, 0, 82, 115, 101,
  116, 0, 1, 115, 117, 116, 99, 0, 0, 117, 100, 97, 116, 101, 0, 1, 100, 114, 101, 102, 101, 114,
  101, 110, 99, 101, 0, 1, 114, 0,
];

/* We are a NOEXEC applet.
 * Obstacles to NOFORK:
 * - we change env
 * - xasprintf result not freed
 * - after xasprintf we use other xfuncs
 */

unsafe extern "C" fn maybe_set_utc(mut opt: libc::c_int) {
  if opt & OPT_UTC as libc::c_int != 0 {
    putenv(b"TZ=UTC0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
  };
}

#[no_mangle]
pub unsafe extern "C" fn date_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut ts: timespec = std::mem::zeroed(); /* skip over the '+' */
  let mut tm_time: tm = std::mem::zeroed(); /* can be NULL */
  let mut buf_fmt_dt2str: [libc::c_char; 64] = [0; 64];
  let mut opt: libc::c_uint = 0;
  let mut ifmt: libc::c_int = -1i32;
  let mut date_str: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut fmt_dt2str: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut fmt_str2dt: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut isofmt_arg: *mut libc::c_char = 0 as *mut libc::c_char;
  opt = getopt32long(
    argv,
    b"^Rs:ud:r:I::D:\x00d--s:s--d:R--I:I--R\x00" as *const u8 as *const libc::c_char,
    date_longopts.as_ptr(),
    &mut date_str as *mut *mut libc::c_char,
    &mut date_str as *mut *mut libc::c_char,
    &mut filename as *mut *mut libc::c_char,
    &mut isofmt_arg as *mut *mut libc::c_char,
    &mut fmt_str2dt as *mut *mut libc::c_char,
  );
  argv = argv.offset(optind as isize);
  maybe_set_utc(opt as libc::c_int);
  if 1i32 != 0 && opt & OPT_TIMESPEC as libc::c_int as libc::c_uint != 0 {
    ifmt = 0i32; /* default is date */
    if !isofmt_arg.is_null() {
      // "date\0""hours\0""minutes\0""seconds\0"
      static mut isoformats: [libc::c_char; 28] = [
        100, 97, 116, 101, 0, 104, 111, 117, 114, 115, 0, 109, 105, 110, 117, 116, 101, 115, 0,
        115, 101, 99, 111, 110, 100, 115, 0, 0,
      ]; /* ns? */
      ifmt = index_in_substrings(isoformats.as_ptr(), isofmt_arg);
      if ifmt < 0 {
        bb_show_usage();
      }
    }
  }
  fmt_dt2str = 0 as *mut libc::c_char;
  if !(*argv.offset(0)).is_null() && *(*argv.offset(0)).offset(0) as libc::c_int == '+' as i32 {
    fmt_dt2str = &mut *(*argv.offset(0)).offset(1) as *mut libc::c_char;
    argv = argv.offset(1)
  }
  if opt & (OPT_SET as libc::c_int | OPT_DATE as libc::c_int) as libc::c_uint == 0 {
    opt |= OPT_SET as libc::c_int as libc::c_uint;
    date_str = *argv.offset(0);
    if !date_str.is_null() {
      let mut len: libc::c_int = strspn(
        date_str,
        b"0123456789\x00" as *const u8 as *const libc::c_char,
      ) as libc::c_int;
      if *date_str.offset(len as isize) as libc::c_int == '\u{0}' as i32
        || *date_str.offset(len as isize) as libc::c_int == '.' as i32
          && (*date_str.offset((len + 1i32) as isize) as libc::c_int - '0' as i32) as libc::c_uchar
            as libc::c_int
            <= 9i32
          && (*date_str.offset((len + 2i32) as isize) as libc::c_int - '0' as i32) as libc::c_uchar
            as libc::c_int
            <= 9i32
          && *date_str.offset((len + 3i32) as isize) as libc::c_int == '\u{0}' as i32
      {
        /* Dreaded MMDDhhmm[[CC]YY][.ss] format!
         * It does not match -d or -s format.
         * Some users actually do use it.
         */
        len -= 8i32;
        if len < 0i32 || len > 4i32 || len & 1i32 != 0 {
          bb_error_msg_and_die(bb_msg_invalid_date.as_ptr(), date_str);
        }
        if len != 0i32 {
          /* move YY or CCYY to front */
          let mut buf: [libc::c_char; 4] = [0; 4];
          memcpy(
            buf.as_mut_ptr() as *mut libc::c_void,
            date_str.offset(8) as *const libc::c_void,
            len as libc::c_ulong,
          );
          memmove(
            date_str.offset(len as isize) as *mut libc::c_void,
            date_str as *const libc::c_void,
            8i32 as libc::c_ulong,
          );
          memcpy(
            date_str as *mut libc::c_void,
            buf.as_mut_ptr() as *const libc::c_void,
            len as libc::c_ulong,
          );
        }
      }
      argv = argv.offset(1)
    }
  }
  if !(*argv).is_null() {
    bb_show_usage();
  }
  /* Now we have parsed all the information except the date format
   * which depends on whether the clock is being set or read */
  if opt & OPT_REFERENCE as libc::c_int as libc::c_uint != 0 {
    let mut statbuf: stat = std::mem::zeroed();
    xstat(filename, &mut statbuf);
    ts.tv_sec = statbuf.st_mtime
  } else {
    time(&mut ts.tv_sec);
  }
  localtime_r(&mut ts.tv_sec, &mut tm_time);
  /* If date string is given, update tm_time, and maybe set date */
  if !date_str.is_null() {
    /* Zero out fields - take her back to midnight! */
    tm_time.tm_sec = 0i32;
    tm_time.tm_min = 0i32;
    tm_time.tm_hour = 0i32;
    /* Process any date input to UNIX time since 1 Jan 1970 */
    if 1i32 != 0 && opt & OPT_HINT as libc::c_int as libc::c_uint != 0 {
      if strptime(date_str, fmt_str2dt, &mut tm_time).is_null() {
        bb_error_msg_and_die(bb_msg_invalid_date.as_ptr(), date_str);
      }
    } else {
      parse_datestr(date_str, &mut tm_time);
    }
    /* Correct any day of week and day of year etc. fields */
    /* Be sure to recheck dst (but not if date is time_t format) */
    if *date_str.offset(0) as libc::c_int != '@' as i32 {
      tm_time.tm_isdst = -1i32
    }
    ts.tv_sec = validate_tm_time(date_str, &mut tm_time);
    /* if setting time, set it */
    if opt & OPT_SET as libc::c_int as libc::c_uint != 0 && stime(&mut ts.tv_sec) < 0i32 {
      bb_simple_perror_msg(b"can\'t set date\x00" as *const u8 as *const libc::c_char);
    }
  }
  /* Display output */
  /* Deal with format string */
  if fmt_dt2str.is_null() {
    let mut i: libc::c_int = 0;
    fmt_dt2str = buf_fmt_dt2str.as_mut_ptr();
    let mut current_block_70: u64;
    if 1i32 != 0 && ifmt >= 0i32 {
      /* -I[SPEC]: 0:date 1:hours 2:minutes 3:seconds */
      strcpy(
        fmt_dt2str,
        b"%Y-%m-%dT%H:%M:%S\x00" as *const u8 as *const libc::c_char,
      ); /* default case */
      i = 8i32 + 3i32 * ifmt;
      if ifmt != 0i32 {
        current_block_70 = 15120539257450533204;
      } else {
        current_block_70 = 4216521074440650966;
      }
    } else if opt & OPT_RFC2822 as libc::c_int as libc::c_uint != 0 {
      strcpy(
        fmt_dt2str,
        b"%a, %d %b %Y %H:%M:%S \x00" as *const u8 as *const libc::c_char,
      );
      i = (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int;
      current_block_70 = 15120539257450533204;
    } else {
      fmt_dt2str =
        b"%a %b %e %H:%M:%S %Z %Y\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
      current_block_70 = 11052029508375673978;
    }
    match current_block_70 {
      15120539257450533204 =>
      /* -R. undo busybox.c setlocale */
      /* TODO: if (ifmt==4) i += sprintf(&fmt_dt2str[i], ",%09u", nanoseconds); */
      {
        let fresh0 = i;
        i = i + 1;
        *fmt_dt2str.offset(fresh0 as isize) = '%' as i32 as libc::c_char;
        let fresh1 = i;
        i = i + 1;
        *fmt_dt2str.offset(fresh1 as isize) = if opt & OPT_UTC as libc::c_int as libc::c_uint != 0 {
          'Z' as i32
        } else {
          'z' as i32
        } as libc::c_char;
        current_block_70 = 4216521074440650966;
      }
      _ => {}
    }
    match current_block_70 {
      4216521074440650966 => *fmt_dt2str.offset(i as isize) = '\u{0}' as i32 as libc::c_char,
      _ => {}
    }
  }
  if *fmt_dt2str as libc::c_int == '\u{0}' as i32 {
    /* With no format string, just print a blank line */
    *bb_common_bufsiz1.as_mut_ptr().offset(0) = '\u{0}' as i32 as libc::c_char
  } else {
    /* Handle special conversions */
    if !is_prefixed_with(fmt_dt2str, b"%f\x00" as *const u8 as *const libc::c_char).is_null() {
      fmt_dt2str = b"%Y.%m.%d-%H:%M:%S\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    /* Generate output string */
    strftime(
      bb_common_bufsiz1.as_mut_ptr(),
      COMMON_BUFSIZE as libc::c_int as size_t,
      fmt_dt2str,
      &mut tm_time,
    );
  }
  puts(bb_common_bufsiz1.as_mut_ptr());
  return 0i32;
}

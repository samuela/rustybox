use crate::librb::size_t;
use libc;
use libc::time_t;
use libc::timespec;

extern "C" {
  #[no_mangle]
  fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn time(__timer: *mut time_t) -> time_t;
  #[no_mangle]
  fn mktime(__tp: *mut tm) -> time_t;
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
  fn localtime(__timer: *const time_t) -> *mut tm;
  #[no_mangle]
  fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  static bb_msg_invalid_date: [libc::c_char; 0];
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_strtoll(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_longlong;
}

pub type __clockid_t = libc::c_int;
pub type clockid_t = __clockid_t;
use libc::tm;

/*
 * ascii-to-numbers implementations for busybox
 *
 * Copyright (C) 2003  Manuel Novoa III  <mjn3@codepoet.org>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* Provides extern declarations of functions */
/* Unsigned long long functions always exist */
/* Provides inline definitions of functions */
/* (useful for mapping them to the type of the same width) */
/* If long == long long, then just map them one-to-one */
/* Same for int -> [long] long */
/* Specialized */
/* Non-aborting kind of convertors: bb_strto[u][l]l */
/* On exit: errno = 0 only if there was non-empty, '\0' terminated value
 * errno = EINVAL if value was not '\0' terminated, but otherwise ok
 *    Return value is still valid, caller should just check whether end[0]
 *    is a valid terminating char for particular case. OTOH, if caller
 *    requires '\0' terminated input, [s]he can just check errno == 0.
 * errno = ERANGE if value had alphanumeric terminating char ("1234abcg").
 * errno = ERANGE if value is out of range, missing, etc.
 * errno = ERANGE if value had minus sign for strtouXX (even "-0" is not ok )
 *    return value is all-ones in this case.
 */
#[inline(always)]
unsafe extern "C" fn bb_strtol(
  mut arg: *const libc::c_char,
  mut endp: *mut *mut libc::c_char,
  mut base: libc::c_int,
) -> libc::c_long {
  return bb_strtoll(arg, endp, base) as libc::c_long;
}

/*
 * Utility routines.
 *
 * Copyright (C) 2007 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn parse_datestr(mut date_str: *const libc::c_char, mut ptm: *mut tm) {
  let mut end: libc::c_char = '\u{0}' as i32 as libc::c_char;
  let mut last_colon: *const libc::c_char = strrchr(date_str, ':' as i32);
  if !last_colon.is_null() {
    /* Parse input and assign appropriately to ptm */
    let mut endp: *const libc::c_char = 0 as *const libc::c_char;
    /* HH:MM */
    if !(sscanf(
      date_str,
      b"%u:%u%c\x00" as *const u8 as *const libc::c_char,
      &mut (*ptm).tm_hour as *mut libc::c_int,
      &mut (*ptm).tm_min as *mut libc::c_int,
      &mut end as *mut libc::c_char,
    ) >= 2i32)
    {
      /* mm.dd-HH:MM */
      if sscanf(
        date_str,
        b"%u.%u-%u:%u%c\x00" as *const u8 as *const libc::c_char,
        &mut (*ptm).tm_mon as *mut libc::c_int,
        &mut (*ptm).tm_mday as *mut libc::c_int,
        &mut (*ptm).tm_hour as *mut libc::c_int,
        &mut (*ptm).tm_min as *mut libc::c_int,
        &mut end as *mut libc::c_char,
      ) >= 4i32
      {
        /* Adjust month from 1-12 to 0-11 */
        (*ptm).tm_mon -= 1i32
      } else if sscanf(
        date_str,
        b"%u.%u.%u-%u:%u%c\x00" as *const u8 as *const libc::c_char,
        &mut (*ptm).tm_year as *mut libc::c_int,
        &mut (*ptm).tm_mon as *mut libc::c_int,
        &mut (*ptm).tm_mday as *mut libc::c_int,
        &mut (*ptm).tm_hour as *mut libc::c_int,
        &mut (*ptm).tm_min as *mut libc::c_int,
        &mut end as *mut libc::c_char,
      ) >= 5i32
        || sscanf(
          date_str,
          b"%u-%u-%u %u:%u%c\x00" as *const u8 as *const libc::c_char,
          &mut (*ptm).tm_year as *mut libc::c_int,
          &mut (*ptm).tm_mon as *mut libc::c_int,
          &mut (*ptm).tm_mday as *mut libc::c_int,
          &mut (*ptm).tm_hour as *mut libc::c_int,
          &mut (*ptm).tm_min as *mut libc::c_int,
          &mut end as *mut libc::c_char,
        ) >= 5i32
      {
        /* yyyy.mm.dd-HH:MM */
        (*ptm).tm_year -= 1900i32;
        (*ptm).tm_mon -= 1i32 /* Adjust years */
      /* Adjust month from 1-12 to 0-11 */
      } else {
        /* strptime is BIG: ~1k in uclibc, ~10k in glibc */
        /* month_name d HH:MM:SS YYYY. Supported by GNU date */
        endp = strptime(
          date_str,
          b"%b %d %T %Y\x00" as *const u8 as *const libc::c_char,
          ptm,
        );
        if !endp.is_null() && *endp as libc::c_int == '\u{0}' as i32 {
          return;
        /* don't fall through to end == ":" check */
        } else {
          bb_error_msg_and_die(bb_msg_invalid_date.as_ptr(), date_str);
        }
      }
    }
    if end as libc::c_int == ':' as i32 {
      /* xxx:SS */
      if sscanf(
        last_colon.offset(1),
        b"%u%c\x00" as *const u8 as *const libc::c_char,
        &mut (*ptm).tm_sec as *mut libc::c_int,
        &mut end as *mut libc::c_char,
      ) == 1i32
      {
        end = '\u{0}' as i32 as libc::c_char
      }
      /* else end != NUL and we error out */
    }
  } else if !strchr(date_str, '-' as i32).is_null()
    && (sscanf(
      date_str,
      b"%u-%u-%u %u%c\x00" as *const u8 as *const libc::c_char,
      &mut (*ptm).tm_year as *mut libc::c_int,
      &mut (*ptm).tm_mon as *mut libc::c_int,
      &mut (*ptm).tm_mday as *mut libc::c_int,
      &mut (*ptm).tm_hour as *mut libc::c_int,
      &mut end as *mut libc::c_char,
    ) >= 4i32
      || sscanf(
        date_str,
        b"%u-%u-%u%c\x00" as *const u8 as *const libc::c_char,
        &mut (*ptm).tm_year as *mut libc::c_int,
        &mut (*ptm).tm_mon as *mut libc::c_int,
        &mut (*ptm).tm_mday as *mut libc::c_int,
        &mut end as *mut libc::c_char,
      ) >= 3i32)
  {
    (*ptm).tm_year -= 1900i32;
    (*ptm).tm_mon -= 1i32 /* Adjust years */
  /* Adjust month from 1-12 to 0-11 */
  } else if *date_str.offset(0) as libc::c_int == '@' as i32 {
    let mut t: time_t = bb_strtol(date_str.offset(1), 0 as *mut *mut libc::c_char, 10i32);
    if *bb_errno == 0 {
      let mut lt: *mut tm = localtime(&mut t);
      if !lt.is_null() {
        *ptm = *lt;
        return;
      }
    }
    end = '1' as i32 as libc::c_char
  } else {
    's_304: {
      let mut current_block_52: u64;
      /* Googled the following on an old date manpage:
       *
       * The canonical representation for setting the date/time is:
       * cc   Century (either 19 or 20)
       * yy   Year in abbreviated form (e.g. 89, 06)
       * mm   Numeric month, a number from 1 to 12
       * dd   Day, a number from 1 to 31
       * HH   Hour, a number from 0 to 23
       * MM   Minutes, a number from 0 to 59
       * .SS  Seconds, a number from 0 to 61 (with leap seconds)
       * Everything but the minutes is optional
       *
       * "touch -t DATETIME" format: [[[[[YY]YY]MM]DD]hh]mm[.ss]
       * Some, but not all, Unix "date DATETIME" commands
       * move [[YY]YY] past minutes mm field (!).
       * Coreutils date does it, and SUS mandates it.
       * (date -s DATETIME does not support this format. lovely!)
       * In bbox, this format is special-cased in date applet
       * (IOW: this function assumes "touch -t" format).
       */
      let mut cur_year: libc::c_uint = (*ptm).tm_year as libc::c_uint;
      let mut len: libc::c_int = strchrnul(date_str, '.' as i32).wrapping_offset_from(date_str)
        as libc::c_long as libc::c_int;
      /* MM[.SS] */
      if len == 2i32
        && sscanf(
          date_str,
          (b"%2u%2u%2u%2u%2u%c\x00" as *const u8 as *const libc::c_char).offset(12),
          &mut (*ptm).tm_min as *mut libc::c_int,
          &mut end as *mut libc::c_char,
        ) >= 1i32
      {
        current_block_52 = 1724319918354933278;
      } else if len == 4i32
        && sscanf(
          date_str,
          (b"%2u%2u%2u%2u%2u%c\x00" as *const u8 as *const libc::c_char).offset(9),
          &mut (*ptm).tm_hour as *mut libc::c_int,
          &mut (*ptm).tm_min as *mut libc::c_int,
          &mut end as *mut libc::c_char,
        ) >= 2i32
      {
        current_block_52 = 1724319918354933278;
      } else if len == 6i32
        && sscanf(
          date_str,
          (b"%2u%2u%2u%2u%2u%c\x00" as *const u8 as *const libc::c_char).offset(6),
          &mut (*ptm).tm_mday as *mut libc::c_int,
          &mut (*ptm).tm_hour as *mut libc::c_int,
          &mut (*ptm).tm_min as *mut libc::c_int,
          &mut end as *mut libc::c_char,
        ) >= 3i32
      {
        current_block_52 = 1724319918354933278;
      } else if len == 8i32
        && sscanf(
          date_str,
          (b"%2u%2u%2u%2u%2u%c\x00" as *const u8 as *const libc::c_char).offset(3),
          &mut (*ptm).tm_mon as *mut libc::c_int,
          &mut (*ptm).tm_mday as *mut libc::c_int,
          &mut (*ptm).tm_hour as *mut libc::c_int,
          &mut (*ptm).tm_min as *mut libc::c_int,
          &mut end as *mut libc::c_char,
        ) >= 4i32
      {
        /* HHMM[.SS] */
        /* ddHHMM[.SS] */
        /* mmddHHMM[.SS] */
        /* Adjust month from 1-12 to 0-11 */
        (*ptm).tm_mon -= 1i32;
        current_block_52 = 1724319918354933278;
      } else if len == 10i32
        && sscanf(
          date_str,
          b"%2u%2u%2u%2u%2u%c\x00" as *const u8 as *const libc::c_char,
          &mut (*ptm).tm_year as *mut libc::c_int,
          &mut (*ptm).tm_mon as *mut libc::c_int,
          &mut (*ptm).tm_mday as *mut libc::c_int,
          &mut (*ptm).tm_hour as *mut libc::c_int,
          &mut (*ptm).tm_min as *mut libc::c_int,
          &mut end as *mut libc::c_char,
        ) >= 5i32
      {
        /* yymmddHHMM[.SS] */
        /* Adjust month from 1-12 to 0-11 */
        (*ptm).tm_mon -= 1i32;
        if cur_year as libc::c_int >= 50i32 {
          /* >= 1950 */
          /* Adjust year: */
          /* 1. Put it in the current century */
          (*ptm).tm_year = ((*ptm).tm_year as libc::c_uint).wrapping_add(
            cur_year
              .wrapping_div(100i32 as libc::c_uint)
              .wrapping_mul(100i32 as libc::c_uint),
          ) as libc::c_int as libc::c_int;
          /* 2. If too far in the past, +100 years */
          if ((*ptm).tm_year as libc::c_uint) < cur_year.wrapping_sub(50i32 as libc::c_uint) {
            (*ptm).tm_year += 100i32
          }
          /* 3. If too far in the future, -100 years */
          if (*ptm).tm_year as libc::c_uint > cur_year.wrapping_add(50i32 as libc::c_uint) {
            (*ptm).tm_year -= 100i32
          }
        }
        current_block_52 = 1724319918354933278;
      } else if len == 12i32
        && sscanf(
          date_str,
          b"%4u%2u%2u%2u%2u%c\x00" as *const u8 as *const libc::c_char,
          &mut (*ptm).tm_year as *mut libc::c_int,
          &mut (*ptm).tm_mon as *mut libc::c_int,
          &mut (*ptm).tm_mday as *mut libc::c_int,
          &mut (*ptm).tm_hour as *mut libc::c_int,
          &mut (*ptm).tm_min as *mut libc::c_int,
          &mut end as *mut libc::c_char,
        ) >= 5i32
      {
        /* ccyymmddHHMM[.SS] */
        (*ptm).tm_year -= 1900i32;
        (*ptm).tm_mon -= 1i32;
        current_block_52 = 1724319918354933278; /* Adjust years */
      /* Adjust month from 1-12 to 0-11 */
      } else {
        current_block_52 = 14372610947245646695; /* assume zero if [.SS] is not given */
      }
      match current_block_52 {
        1724319918354933278 => {
          (*ptm).tm_sec = 0i32;
          if end as libc::c_int == '.' as i32 {
            /* xxx.SS */
            if sscanf(
              strchr(date_str, '.' as i32).offset(1),
              b"%u%c\x00" as *const u8 as *const libc::c_char,
              &mut (*ptm).tm_sec as *mut libc::c_int,
              &mut end as *mut libc::c_char,
            ) == 1i32
            {
              end = '\u{0}' as i32 as libc::c_char
            }
            /* else end != NUL and we error out */
          }
          /* Users were confused by "date -s 20180923"
           * working (not in the way they were expecting).
           * It was interpreted as MMDDhhmm, and not bothered by
           * "month #20" in the least. Prevent such cases:
           */
          if !((*ptm).tm_sec > 60i32
            || (*ptm).tm_min > 59i32
            || (*ptm).tm_hour > 23i32
            || (*ptm).tm_mday > 31i32
            || (*ptm).tm_mon > 11i32)
          {
            break 's_304;
          }
        }
        _ => {}
      }
      /* month# is 0..11, not 1..12 */
      bb_error_msg_and_die(bb_msg_invalid_date.as_ptr(), date_str);
    }
  }
  if end as libc::c_int != '\u{0}' as i32 {
    bb_error_msg_and_die(bb_msg_invalid_date.as_ptr(), date_str);
  };
}
#[no_mangle]
pub unsafe extern "C" fn validate_tm_time(
  mut date_str: *const libc::c_char,
  mut ptm: *mut tm,
) -> time_t {
  let mut t: time_t = mktime(ptm);
  if t == -1i64 {
    bb_error_msg_and_die(bb_msg_invalid_date.as_ptr(), date_str);
  }
  return t;
}
unsafe extern "C" fn strftime_fmt(
  mut buf: *mut libc::c_char,
  mut len: libc::c_uint,
  mut tp: *mut time_t,
  mut fmt: *const libc::c_char,
) -> *mut libc::c_char {
  let mut t: time_t = 0;
  if tp.is_null() {
    tp = &mut t;
    time(tp);
  }
  /* Returns pointer to NUL */
  return buf.offset(strftime(buf, len as size_t, fmt, localtime(tp)) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn strftime_HHMMSS(
  mut buf: *mut libc::c_char,
  mut len: libc::c_uint,
  mut tp: *mut time_t,
) -> *mut libc::c_char {
  return strftime_fmt(
    buf,
    len,
    tp,
    b"%H:%M:%S\x00" as *const u8 as *const libc::c_char,
  );
}
/* dmalloc will redefine these to it's own implementation. It is safe
 * to have the prototypes here unconditionally.  */
/* After v = xrealloc_vector(v, SHIFT, idx) it's ok to use
 * at least v[idx] and v[idx+1], for all idx values.
 * SHIFT specifies how many new elements are added (1:2, 2:4, ..., 8:256...)
 * when all elements are used up. New elements are zeroed out.
 * xrealloc_vector(v, SHIFT, idx) *MUST* be called with consecutive IDXs -
 * skipping an index is a bad bug - it may miss a realloc!
 */
//TODO: supply a pointer to char[11] buffer (avoid statics)?
/* cp.c, mv.c, install.c depend on these values. CAREFUL when changing them! */
/* -p */
/* !-d */
/* -R */
/* -f */
/* -i */
/* -l */
/* -s */
/* -L */
/* -H */
/* -a = -pdR (mapped in cp.c) */
/* -r = -dR  (mapped in cp.c) */
/* -P = -d   (mapped in cp.c) */
/* -v */
/* -u */
/* -T */
/* --remove-destination */
/* bit 17 skipped for "cp --parents" */
/* cp --reflink=auto */
/* cp --reflink[=always] */
/*
 * Hole. cp may have some bits set here,
 * they should not affect remove_file()/copy_file()
 */
/* NB: without FILEUTILS_RECUR in flags, it will basically "cat"
 * the source, not copy (unless "source" is a directory).
 * This makes "cp /dev/null file" and "install /dev/null file" (!!!)
 * work coreutils-compatibly. */
/*ACTION_REVERSE      = (1 << 4), - unused */
/* more than enough for "/dev/ttyXXX" */
/* bb_copyfd_XX print read/write errors and return -1 if they occur */
/* "short" copy can be detected by return value < size */
/* this helper yells "short read!" if param is not -1 */
/* xxxx_strip version can modify its parameter:
 * "/"        -> "/"
 * "abc"      -> "abc"
 * "abc/def"  -> "def"
 * "abc/def/" -> "def" !!
 */
/* "abc/def/" -> "" and it never modifies 'path' */
/* Simpler version: does not special case "/" string */
/* NB: can violate const-ness (similarly to strchr) */
/* !RETURNS_MALLOC: it's a realloc-like function */
/* bb_signals(BB_FATAL_SIGS, handler) catches all signals which
 * otherwise would kill us, except for those resulting from bugs:
 * SIGSEGV, SIGILL, SIGFPE.
 * Other fatal signals not included (TODO?):
 * SIGBUS   Bus error (bad memory access)
 * SIGPOLL  Pollable event. Synonym of SIGIO
 * SIGPROF  Profiling timer expired
 * SIGSYS   Bad argument to routine
 * SIGTRAP  Trace/breakpoint trap
 *
 * The only known arch with some of these sigs not fitting
 * into 32 bits is parisc (SIGXCPU=33, SIGXFSZ=34, SIGSTKFLT=36).
 * Dance around with long long to guard against that...
 */
// Write to pipe with no readers
// Quit from keyboard
// Abort signal from abort(3)
// Timer signal from alarm(2)
// Virtual alarm clock
// CPU time limit exceeded
// File size limit exceeded
// Yes kids, these are also fatal!
/* Unlike signal() and bb_signals, sets handler with sigaction()
 * and in a way that while signal handler is run, no other signals
 * will be blocked; syscalls will not be restarted: */
/* syscalls like read() will be interrupted with EINTR: */
/* syscalls like read() won't be interrupted (though select/poll will be): */
/* Will do sigaction(signum, act, NULL): */
/* SIG_BLOCK/SIG_UNBLOCK all signals: */
/* Return old set in the same set: */
/* Standard handler which just records signo */
/* not FAST_FUNC! */
/* In this form code with pipes is much more readable */
/* Useful for having small structure members/global variables */
/* | AF_DECnet */
/* | AF_IPX */
#[no_mangle]
pub unsafe extern "C" fn strftime_YYYYMMDDHHMMSS(
  mut buf: *mut libc::c_char,
  mut len: libc::c_uint,
  mut tp: *mut time_t,
) -> *mut libc::c_char {
  return strftime_fmt(
    buf,
    len,
    tp,
    b"%Y-%m-%d %H:%M:%S\x00" as *const u8 as *const libc::c_char,
  );
}
/* Old glibc (< 2.3.4) does not provide this constant. We use syscall
 * directly so this definition is safe. */
unsafe extern "C" fn get_mono(mut ts: *mut timespec) {
  if clock_gettime(1i32, ts) != 0 {
    bb_simple_error_msg_and_die(
      b"clock_gettime(MONOTONIC) failed\x00" as *const u8 as *const libc::c_char,
    );
  };
}
#[no_mangle]
pub unsafe extern "C" fn monotonic_ns() -> libc::c_ulonglong {
  let mut ts: timespec = timespec {
    tv_sec: 0,
    tv_nsec: 0,
  };
  get_mono(&mut ts);
  return (ts.tv_sec as libc::c_ulonglong)
    .wrapping_mul(1000000000u64)
    .wrapping_add(ts.tv_nsec as libc::c_ulonglong);
}
#[no_mangle]
pub unsafe extern "C" fn monotonic_us() -> libc::c_ulonglong {
  let mut ts: timespec = timespec {
    tv_sec: 0,
    tv_nsec: 0,
  };
  get_mono(&mut ts);
  return (ts.tv_sec as libc::c_ulonglong)
    .wrapping_mul(1000000u64)
    .wrapping_add((ts.tv_nsec / 1000i32 as libc::c_long) as libc::c_ulonglong);
}
#[no_mangle]
pub unsafe extern "C" fn monotonic_ms() -> libc::c_ulonglong {
  let mut ts: timespec = timespec {
    tv_sec: 0,
    tv_nsec: 0,
  };
  get_mono(&mut ts);
  return (ts.tv_sec as libc::c_ulonglong)
    .wrapping_mul(1000u64)
    .wrapping_add((ts.tv_nsec / 1000000i32 as libc::c_long) as libc::c_ulonglong);
}

/*
 * Busybox main internal header file
 *
 * Based in part on code from sash, Copyright (c) 1999 by David I. Bell
 * Permission has been granted to redistribute this code under GPL.
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* TODO: and glibc? */
/* There are two incompatible basename's, let's not use them! */
/* See the dirname/basename man page for details */
/* dirname,basename */
/* Don't do this here:
 * #include <sys/sysinfo.h>
 * Some linux/ includes pull in conflicting definition
 * of struct sysinfo (only in some toolchanins), which breaks build.
 * Include sys/sysinfo.h only in those files which need it.
 */
/* Just in case libc doesn't define some of these... */
//This breaks on bionic:
//# if !defined(__socklen_t_defined) && !defined(_SOCKLEN_T_DECLARED)
// /* We #define socklen_t *after* includes, otherwise we get
// * typedef redefinition errors from system headers
// * (in case "is it defined already" detection above failed)
// */
//#  define socklen_t bb_socklen_t
//   typedef unsigned socklen_t;
//# endif
//if this is still needed, add a fix along the lines of
//  ifdef SPECIFIC_BROKEN_LIBC_CHECK / typedef socklen_t / endif
//in platform.h instead!
/*
 * Use '%m' to append error string on platforms that support it,
 * '%s' and strerror() on those that don't.
 */
/*nothing*/
/* Some libc's forget to declare these, do it ourself */
/* klogctl is in libc's klog.h, but we cheat and not #include that */
/* Busybox does not use threads, we can speed up stdio. */
/* Above functions are required by POSIX.1-2008, below ones are extensions */
/* musl <= 1.1.15 does not support fflush_unlocked(NULL) */
//# undef  fflush
//# define fflush(stream) fflush_unlocked(stream)
/* Make all declarations hidden (-fvisibility flag only affects definitions) */
/* (don't include system headers after this until corresponding pop!) */
/* Tested to work correctly with all int types (IIRC :]) */
/* Large file support */
/* Note that CONFIG_LFS=y forces bbox to be built with all common ops
 * (stat, lseek etc) mapped to "largefile" variants by libc.
 * Practically it means that open() automatically has O_LARGEFILE added
 * and all filesize/file_offset parameters and struct members are "large"
 * (in today's world - signed 64bit). For full support of large files,
 * we need a few helper #defines (below) and careful use of off_t
 * instead of int/ssize_t. No lseek64(), O_LARGEFILE etc necessary */
/* CONFIG_LFS is on */
/* "long" is long enough on this system */
/* usage: sz = BB_STRTOOFF(s, NULL, 10); if (errno || sz < 0) die(); */
/* usage: printf("size: %"OFF_FMT"d (%"OFF_FMT"x)\n", sz, sz); */
/* scary. better ideas? (but do *test* them first!) */
/* Users report bionic to use 32-bit off_t even if LARGEFILE support is requested.
 * We misdetected that. Don't let it build:
 */
/* Some useful definitions */
/* Macros for min/max.  */
/* buffer allocation schemes */
/* glibc uses __errno_location() to get a ptr to errno */
/* We can just memorize it once - no multithreading in busybox :) */
#[no_mangle]
pub unsafe extern "C" fn monotonic_sec() -> libc::c_uint {
  let mut ts: timespec = timespec {
    tv_sec: 0,
    tv_nsec: 0,
  };
  get_mono(&mut ts);
  return ts.tv_sec as libc::c_uint;
}

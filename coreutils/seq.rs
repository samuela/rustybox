use libc;


extern "C" {
  #[no_mangle]
  fn strtod(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char) -> libc::c_double;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_show_usage() -> !;
}

use crate::librb::size_t;

pub const OPT_w: C2RustUnnamed = 1;
pub type C2RustUnnamed = libc::c_uint;
pub const OPT_s: C2RustUnnamed = 2;

/*
 * seq implementation for busybox
 *
 * Copyright (C) 2004, Glenn McGrath
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config SEQ
//config:	bool "seq (3.8 kb)"
//config:	default y
//config:	help
//config:	print a sequence of numbers
//applet:IF_SEQ(APPLET_NOEXEC(seq, seq, BB_DIR_USR_BIN, BB_SUID_DROP, seq))
/* was NOFORK, but then "seq 1 999999999" can't be ^C'ed if run by hush */
//kbuild:lib-$(CONFIG_SEQ) += seq.o
//usage:#define seq_trivial_usage
//usage:       "[-w] [-s SEP] [FIRST [INC]] LAST"
//usage:#define seq_full_usage "\n\n"
//usage:       "Print numbers from FIRST to LAST, in steps of INC.\n"
//usage:       "FIRST, INC default to 1.\n"
//usage:     "\n	-w	Pad to last with leading zeros"
//usage:     "\n	-s SEP	String separator"
/* This is a NOEXEC applet. Be very careful! */
#[no_mangle]
pub unsafe extern "C" fn seq_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut first: libc::c_double = 0.;
  let mut last: libc::c_double = 0.;
  let mut increment: libc::c_double = 0.;
  let mut v: libc::c_double = 0.;
  let mut n: libc::c_uint = 0;
  let mut width: libc::c_uint = 0;
  let mut frac_part: libc::c_uint = 0;
  let mut sep: *const libc::c_char = 0 as *const libc::c_char;
  let mut opt_s: *const libc::c_char = b"\n\x00" as *const u8 as *const libc::c_char;
  let mut opt: libc::c_uint = 0;
  opt = getopt32(
    argv,
    b"+ws:\x00" as *const u8 as *const libc::c_char,
    &mut opt_s as *mut *const libc::c_char,
  );
  argc -= optind;
  argv = argv.offset(optind as isize);
  increment = 1i32 as libc::c_double;
  first = increment;
  *bb_errno = 0i32;
  's_87: {
    let mut pp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut current_block_10: u64;
    match argc {
      3 => {
        increment = strtod(*argv.offset(1), &mut pp);
        *bb_errno |= *pp as libc::c_int;
        current_block_10 = 7542340127038738699;
      }
      2 => {
        current_block_10 = 7542340127038738699;
      }
      1 => {
        current_block_10 = 9763345305552516820;
      }
      _ => {
        current_block_10 = 9712944753701882441;
      }
    }
    match current_block_10 {
      7542340127038738699 => {
        first = strtod(*argv.offset(0), &mut pp);
        *bb_errno |= *pp as libc::c_int;
        current_block_10 = 9763345305552516820;
      }
      _ => {}
    }
    match current_block_10 {
      9763345305552516820 => {
        last = strtod(*argv.offset((argc - 1i32) as isize), &mut pp);
        if *bb_errno == 0 && *pp as libc::c_int == '\u{0}' as i32 {
          break 's_87;
        }
      }
      _ => {}
    }
    bb_show_usage();
  }
  /* Last checked to be compatible with: coreutils-6.10 */
  width = 0i32 as libc::c_uint;
  frac_part = 0i32 as libc::c_uint;
  loop {
    let mut dot: *mut libc::c_char = strchrnul(*argv, '.' as i32);
    let mut w: libc::c_int = dot.wrapping_offset_from(*argv) as libc::c_long as libc::c_int;
    let mut f: libc::c_int = strlen(dot) as libc::c_int;
    if width < w as libc::c_uint {
      width = w as libc::c_uint
    }
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
    /* Why do the above _before_ frac check below?
     * Try "seq 1 2.0" and "seq 1.0 2.0":
     * coreutils never pay attention to the number
     * of fractional digits in last arg. */
    if frac_part < f as libc::c_uint {
      frac_part = f as libc::c_uint
    }
  } /* I/O error, bail out (yes, this really happens) */
  if frac_part != 0 {
    frac_part = frac_part.wrapping_sub(1);
    if frac_part != 0 {
      width = width.wrapping_add(frac_part.wrapping_add(1i32 as libc::c_uint))
    }
  }
  if opt & OPT_w as libc::c_int as libc::c_uint == 0 {
    width = 0i32 as libc::c_uint
  }
  sep = b"\x00" as *const u8 as *const libc::c_char;
  v = first;
  n = 0i32 as libc::c_uint;
  while if increment >= 0i32 as libc::c_double {
    (v <= last) as libc::c_int
  } else {
    (v >= last) as libc::c_int
  } != 0
  {
    if printf(
      b"%s%0*.*f\x00" as *const u8 as *const libc::c_char,
      sep,
      width,
      frac_part,
      v,
    ) < 0i32
    {
      break;
    }
    sep = opt_s;
    /* v += increment; - would accumulate floating point errors */
    n = n.wrapping_add(1);
    v = first + n as libc::c_double * increment
  }
  if n != 0 {
    /* if while loop executed at least once */
    bb_putchar('\n' as i32);
  }
  return fflush_all();
}

use crate::libbb::ptr_to_globals::bb_errno;
use libc;
use libc::printf;
extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn ferror_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
  #[no_mangle]
  fn fclose_if_not_stdin(file: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fopen_or_warn_stdin(filename: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  static bb_msg_standard_input: [libc::c_char; 0];
}

use crate::librb::smallint;
use libc::FILE;
/*
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
pub type C2RustUnnamed = libc::c_uint;
pub const UNICODE_ON: C2RustUnnamed = 2;
pub const UNICODE_OFF: C2RustUnnamed = 1;
pub const UNICODE_UNKNOWN: C2RustUnnamed = 0;

/*
 * wc implementation for busybox
 *
 * Copyright (C) 2003  Manuel Novoa III  <mjn3@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Mar 16, 2003      Manuel Novoa III   (mjn3@codepoet.org)
 *
 * Rewritten to fix a number of problems and do some size optimizations.
 * Problems in the previous busybox implementation (besides bloat) included:
 *  1) broken 'wc -c' optimization (read note below)
 *  2) broken handling of '-' args
 *  3) no checking of ferror on EOF returns
 *  4) isprint() wasn't considered when word counting.
 *
 * NOTES:
 *
 * The previous busybox wc attempted an optimization using stat for the
 * case of counting chars only.  I omitted that because it was broken.
 * It didn't take into account the possibility of input coming from a
 * pipe, or input from a file with file pointer not at the beginning.
 *
 * To implement such a speed optimization correctly, not only do you
 * need the size, but also the file position.  Note also that the
 * file position may be past the end of file.  Consider the example
 * (adapted from example in gnu wc.c)
 *
 *      echo hello > /tmp/testfile &&
 *      (dd ibs=1k skip=1 count=0 &> /dev/null; wc -c) < /tmp/testfile
 *
 * for which 'wc -c' should output '0'.
 */
//config:config WC
//config:	bool "wc (4.5 kb)"
//config:	default y
//config:	help
//config:	wc is used to print the number of bytes, words, and lines,
//config:	in specified files.
//config:
//config:config FEATURE_WC_LARGE
//config:	bool "Support very large counts"
//config:	default y
//config:	depends on WC
//config:	help
//config:	Use "unsigned long long" for counter variables.
//applet:IF_WC(APPLET(wc, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_WC) += wc.o
/* BB_AUDIT SUSv3 compliant. */
/* http://www.opengroup.org/onlinepubs/007904975/utilities/wc.html */
/* We support -m even when UNICODE_SUPPORT is off,
 * we just don't advertise it in help text,
 * since it is the same as -c in this case.
 */
//usage:#define wc_trivial_usage
//usage:       "[-c"IF_UNICODE_SUPPORT("m")"lwL] [FILE]..."
//usage:
//usage:#define wc_full_usage "\n\n"
//usage:       "Count lines, words, and bytes for each FILE (or stdin)\n"
//usage:     "\n	-c	Count bytes"
//usage:	IF_UNICODE_SUPPORT(
//usage:     "\n	-m	Count characters"
//usage:	)
//usage:     "\n	-l	Count newlines"
//usage:     "\n	-w	Count words"
//usage:     "\n	-L	Print longest line length"
//usage:
//usage:#define wc_example_usage
//usage:       "$ wc /etc/passwd\n"
//usage:       "     31      46    1365 /etc/passwd\n"
/* Order is important if we want to be compatible with
 * column order in "wc -cmlwL" output:
 */
pub type C2RustUnnamed_0 = libc::c_uint;
/* -L */
pub const NUM_WCS: C2RustUnnamed_0 = 5;
/* -c */
pub const WC_LENGTH: C2RustUnnamed_0 = 4;
/* -m */
pub const WC_BYTES: C2RustUnnamed_0 = 3;
/* -w */
pub const WC_UNICHARS: C2RustUnnamed_0 = 2;
/* -l */
pub const WC_WORDS: C2RustUnnamed_0 = 1;
pub const WC_LINES: C2RustUnnamed_0 = 0;
#[no_mangle]
pub unsafe extern "C" fn wc_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut fp: *mut FILE = 0 as *mut FILE;
  let mut s: *const libc::c_char = 0 as *const libc::c_char;
  let mut u: libc::c_uint = 0;
  let mut linepos: libc::c_uint = 0;
  let mut in_word: smallint = 0;
  let mut arg: *const libc::c_char = 0 as *const libc::c_char;
  let mut start_fmt: *const libc::c_char =
    (b" %9llu\x00" as *const u8 as *const libc::c_char).offset(1);
  let mut fname_fmt: *const libc::c_char = b" %s\n\x00" as *const u8 as *const libc::c_char;
  let mut pcounts: *mut libc::c_ulonglong = 0 as *mut libc::c_ulonglong;
  let mut counts: [libc::c_ulonglong; 5] = [0; 5];
  let mut totals: [libc::c_ulonglong; 5] = [0; 5];
  let mut num_files: libc::c_int = 0;
  let mut status: smallint = 0i32 as smallint;
  let mut print_type: libc::c_uint = 0;
  print_type = getopt32(argv, b"lwmcL\x00" as *const u8 as *const libc::c_char);
  if print_type == 0i32 as libc::c_uint {
    print_type = (1i32 << WC_LINES as libc::c_int
      | 1i32 << WC_WORDS as libc::c_int
      | 1i32 << WC_BYTES as libc::c_int) as libc::c_uint
  }
  argv = argv.offset(optind as isize);
  if (*argv.offset(0)).is_null() {
    argv = argv.offset(-1);
    *argv = bb_msg_standard_input.as_ptr() as *mut libc::c_char;
    fname_fmt = b"\n\x00" as *const u8 as *const libc::c_char
  }
  if (*argv.offset(1)).is_null() {
    /* zero or one filename? */
    if print_type.wrapping_sub(1i32 as libc::c_uint) & print_type == 0 {
      /* exactly one option? */
      start_fmt = b"%llu\x00" as *const u8 as *const libc::c_char
    }
  }
  memset(
    totals.as_mut_ptr() as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<[libc::c_ulonglong; 5]>() as libc::c_ulong,
  );
  pcounts = counts.as_mut_ptr();
  num_files = 0i32;
  loop {
    let fresh0 = argv;
    argv = argv.offset(1);
    arg = *fresh0;
    if !arg.is_null() {
      fp = 0 as *mut FILE;
      s = 0 as *const libc::c_char;
      u = 0;
      linepos = 0;
      in_word = 0;
      num_files += 1;
      fp = fopen_or_warn_stdin(arg);
      if fp.is_null() {
        status = 1i32 as smallint;
        continue;
      } else {
        memset(
          counts.as_mut_ptr() as *mut libc::c_void,
          0i32,
          ::std::mem::size_of::<[libc::c_ulonglong; 5]>() as libc::c_ulong,
        );
        linepos = 0i32 as libc::c_uint;
        in_word = 0i32 as smallint;
        let mut current_block_46: u64;
        loop {
          let mut c: libc::c_int = 0;
          /* Our -w doesn't match GNU wc exactly... oh well */
          c = getc_unlocked(fp);
          if c == -1i32 {
            if ferror_unlocked(fp) != 0 {
              bb_simple_perror_msg(arg);
              status = 1i32 as smallint
            }
            current_block_46 = 4254989650113872746;
          /* Treat an EOF as '\r'. */
          } else {
            /* Cater for -c and -m */
            counts[WC_BYTES as libc::c_int as usize] =
              counts[WC_BYTES as libc::c_int as usize].wrapping_add(1);
            if UNICODE_ON as libc::c_int != UNICODE_ON as libc::c_int || c & 0xc0i32 != 0x80i32 {
              /* it isn't a 2nd+ byte of a Unicode char */
              counts[WC_UNICHARS as libc::c_int as usize] =
                counts[WC_UNICHARS as libc::c_int as usize].wrapping_add(1)
            }
            if (c - 0x20i32) as libc::c_uint <= (0x7ei32 - 0x20i32) as libc::c_uint {
              /* FIXME: not unicode-aware */
              linepos = linepos.wrapping_add(1);
              if !(c == ' ' as i32) {
                in_word = 1i32 as smallint;
                continue;
              } else {
                current_block_46 = 1623252117315916725;
              }
            } else {
              if !((c - 9i32) as libc::c_uint <= 4i32 as libc::c_uint) {
                continue;
              }
              /* \t  9
               * \n 10
               * \v 11
               * \f 12
               * \r 13
               */
              if c == '\t' as i32 {
                linepos = (linepos | 7i32 as libc::c_uint).wrapping_add(1i32 as libc::c_uint); /* '\n', '\r', '\f', or '\v' */
                current_block_46 = 1623252117315916725;
              } else {
                current_block_46 = 4254989650113872746;
              }
            }
          }
          match current_block_46 {
            4254989650113872746 => {
              if linepos as libc::c_ulonglong > counts[WC_LENGTH as libc::c_int as usize] {
                counts[WC_LENGTH as libc::c_int as usize] = linepos as libc::c_ulonglong
              }
              if c == '\n' as i32 {
                counts[WC_LINES as libc::c_int as usize] =
                  counts[WC_LINES as libc::c_int as usize].wrapping_add(1)
              }
              if c != '\u{b}' as i32 {
                linepos = 0i32 as libc::c_uint
              }
            }
            _ => {}
          }
          counts[WC_WORDS as libc::c_int as usize] =
            counts[WC_WORDS as libc::c_int as usize].wrapping_add(in_word as libc::c_ulonglong);
          in_word = 0i32 as smallint;
          if c == -1i32 {
            break;
          }
        }
        fclose_if_not_stdin(fp);
        if totals[WC_LENGTH as libc::c_int as usize] < counts[WC_LENGTH as libc::c_int as usize] {
          totals[WC_LENGTH as libc::c_int as usize] = counts[WC_LENGTH as libc::c_int as usize]
        }
        totals[WC_LENGTH as libc::c_int as usize] = totals[WC_LENGTH as libc::c_int as usize]
          .wrapping_sub(counts[WC_LENGTH as libc::c_int as usize])
      }
    } else {
      /* If more than one file was processed, we want the totals.  To save some
       * space, we set the pcounts ptr to the totals array.  This has the side
       * effect of trashing the totals array after outputting it, but that's
       * irrelavent since we no longer need it. */
      if !(num_files > 1i32) {
        break; /* Make sure we don't get here again. */
      }
      num_files = 0i32;
      arg = b"total\x00" as *const u8 as *const libc::c_char;
      pcounts = totals.as_mut_ptr();
      argv = argv.offset(-1)
    }
    /* coreutils wc tries hard to print pretty columns
     * (saves results for all files, finds max col len etc...)
     * we won't try that hard, it will bloat us too much */
    s = start_fmt;
    u = 0i32 as libc::c_uint;
    loop {
      if print_type & (1i32 << u) as libc::c_uint != 0 {
        printf(s, *pcounts.offset(u as isize));
        s = b" %9llu\x00" as *const u8 as *const libc::c_char
        /* Ok... restore the leading space. */
      }
      totals[u as usize] = totals[u as usize].wrapping_add(*pcounts.offset(u as isize));
      u = u.wrapping_add(1);
      if !(u < NUM_WCS as libc::c_int as libc::c_uint) {
        break;
      }
    }
    printf(fname_fmt, arg);
  }
  fflush_stdout_and_exit(status as libc::c_int);
}

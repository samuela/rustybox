use libc;
extern "C" {
  #[no_mangle]
  fn strtod(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char) -> libc::c_double;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn skip_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_process_escape_sequence(ptr: *mut *const libc::c_char) -> libc::c_char;
  #[no_mangle]
  fn overlapping_strcpy(dst: *mut libc::c_char, src: *const libc::c_char);
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn utoa(n: libc::c_uint) -> *mut libc::c_char;
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
  #[no_mangle]
  fn bb_strtoull(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_ulonglong;
  #[no_mangle]
  fn bb_strtoll(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_longlong;
  #[no_mangle]
  fn bb_strtoi(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
}
use crate::librb::ptrdiff_t;
use crate::librb::size_t;

/*
 * printf - format and print data
 *
 * Copyright 1999 Dave Cinege
 * Portions copyright (C) 1990-1996 Free Software Foundation, Inc.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Usage: printf format [argument...]
 *
 * A front end to the printf function that lets it be used from the shell.
 *
 * Backslash escapes:
 *
 * \" = double quote
 * \\ = backslash
 * \a = alert (bell)
 * \b = backspace
 * \c = produce no further output
 * \f = form feed
 * \n = new line
 * \r = carriage return
 * \t = horizontal tab
 * \v = vertical tab
 * \0ooo = octal number (ooo is 0 to 3 digits)
 * \xhhh = hexadecimal number (hhh is 1 to 3 digits)
 *
 * Additional directive:
 *
 * %b = print an argument string, interpreting backslash escapes
 *
 * The 'format' argument is re-used as many times as necessary
 * to convert all of the given arguments.
 *
 * David MacKenzie <djm@gnu.ai.mit.edu>
 */
/* 19990508 Busy Boxed! Dave Cinege */
//config:config PRINTF
//config:	bool "printf (3.8 kb)"
//config:	default y
//config:	help
//config:	printf is used to format and print specified strings.
//config:	It's similar to 'echo' except it has more options.
//applet:IF_PRINTF(APPLET_NOFORK(printf, printf, BB_DIR_USR_BIN, BB_SUID_DROP, printf))
//kbuild:lib-$(CONFIG_PRINTF) += printf.o
//kbuild:lib-$(CONFIG_ASH_PRINTF)  += printf.o
//kbuild:lib-$(CONFIG_HUSH_PRINTF) += printf.o
//usage:#define printf_trivial_usage
//usage:       "FORMAT [ARG]..."
//usage:#define printf_full_usage "\n\n"
//usage:       "Format and print ARG(s) according to FORMAT (a-la C printf)"
//usage:
//usage:#define printf_example_usage
//usage:       "$ printf \"Val=%d\\n\" 5\n"
//usage:       "Val=5\n"
/* A note on bad input: neither bash 3.2 nor coreutils 6.10 stop on it.
 * They report it:
 *  bash: printf: XXX: invalid number
 *  printf: XXX: expected a numeric value
 *  bash: printf: 123XXX: invalid number
 *  printf: 123XXX: value not completely converted
 * but then they use 0 (or partially converted numeric prefix) as a value
 * and continue. They exit with 1 in this case.
 * Both accept insane field width/precision (e.g. %9999999999.9999999999d).
 * Both print error message and assume 0 if %*.*f width/precision is "bad"
 *  (but negative numbers are not "bad").
 * Both accept negative numbers for %u specifier.
 *
 * We try to be compatible.
 */
pub type converter =
  Option<unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_void) -> ()>;
unsafe extern "C" fn multiconvert(
  mut arg: *const libc::c_char,
  mut result: *mut libc::c_void,
  mut convert: converter,
) -> libc::c_int {
  if *arg as libc::c_int == '\"' as i32 || *arg as libc::c_int == '\'' as i32 {
    arg = utoa(*arg.offset(1) as libc::c_uchar as libc::c_uint)
  }
  *bb_errno = 0i32;
  convert.expect("non-null function pointer")(arg, result);
  if *bb_errno != 0 {
    bb_error_msg(
      b"invalid number \'%s\'\x00" as *const u8 as *const libc::c_char,
      arg,
    );
    return 1i32;
  }
  return 0i32;
}
unsafe extern "C" fn conv_strtoull(mut arg: *const libc::c_char, mut result: *mut libc::c_void) {
  /* Allow leading '+' - bb_strtoull() by itself does not allow it,
   * and probably shouldn't (other callers might require purely numeric
   * inputs to be allowed.
   */
  if *arg.offset(0) as libc::c_int == '+' as i32 {
    arg = arg.offset(1)
  }
  *(result as *mut libc::c_ulonglong) = bb_strtoull(arg, 0 as *mut *mut libc::c_char, 0i32);
  /* both coreutils 6.10 and bash 3.2:
   * $ printf '%x\n' -2
   * fffffffffffffffe
   * Mimic that:
   */
  if *bb_errno != 0 {
    *(result as *mut libc::c_ulonglong) =
      bb_strtoll(arg, 0 as *mut *mut libc::c_char, 0i32) as libc::c_ulonglong
  };
}
unsafe extern "C" fn conv_strtoll(mut arg: *const libc::c_char, mut result: *mut libc::c_void) {
  if *arg.offset(0) as libc::c_int == '+' as i32 {
    arg = arg.offset(1)
  }
  *(result as *mut libc::c_longlong) = bb_strtoll(arg, 0 as *mut *mut libc::c_char, 0i32);
}
unsafe extern "C" fn conv_strtod(mut arg: *const libc::c_char, mut result: *mut libc::c_void) {
  let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
  /* Well, this one allows leading whitespace... so what? */
  /* What I like much less is that "-" accepted too! :( */
  *(result as *mut libc::c_double) = strtod(arg, &mut end);
  if *end.offset(0) != 0 {
    *bb_errno = 34i32;
    *(result as *mut libc::c_double) = 0i32 as libc::c_double
  };
}
/* Callers should check errno to detect errors */
unsafe extern "C" fn my_xstrtoull(mut arg: *const libc::c_char) -> libc::c_ulonglong {
  let mut result: libc::c_ulonglong = 0;
  if multiconvert(
    arg,
    &mut result as *mut libc::c_ulonglong as *mut libc::c_void,
    Some(conv_strtoull as unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_void) -> ()),
  ) != 0
  {
    result = 0i32 as libc::c_ulonglong
  }
  return result;
}
unsafe extern "C" fn my_xstrtoll(mut arg: *const libc::c_char) -> libc::c_longlong {
  let mut result: libc::c_longlong = 0;
  if multiconvert(
    arg,
    &mut result as *mut libc::c_longlong as *mut libc::c_void,
    Some(conv_strtoll as unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_void) -> ()),
  ) != 0
  {
    result = 0i32 as libc::c_longlong
  }
  return result;
}
unsafe extern "C" fn my_xstrtod(mut arg: *const libc::c_char) -> libc::c_double {
  let mut result: libc::c_double = 0.;
  multiconvert(
    arg,
    &mut result as *mut libc::c_double as *mut libc::c_void,
    Some(conv_strtod as unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_void) -> ()),
  );
  return result;
}
/* Handles %b; return 1 if output is to be short-circuited by \c */
unsafe extern "C" fn print_esc_string(mut str: *const libc::c_char) -> libc::c_int {
  let mut c: libc::c_char = 0;
  loop {
    c = *str;
    if !(c as libc::c_int != '\u{0}' as i32) {
      break;
    }
    str = str.offset(1);
    if c as libc::c_int == '\\' as i32 {
      /* %b also accepts 4-digit octals of the form \0### */
      if *str as libc::c_int == '0' as i32 {
        if ((*str.offset(1) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int) < 8i32 {
          /* 2nd char is 0..7: skip leading '0' */
          str = str.offset(1)
        }
      } else if *str as libc::c_int == 'c' as i32 {
        return 1i32;
      }
      /* optimization: don't force arg to be on-stack,
       * use another variable for that. */
      let mut z: *const libc::c_char = str;
      c = bb_process_escape_sequence(&mut z);
      str = z
    }
    putchar_unlocked(c as libc::c_int);
  }
  return 0i32;
}
unsafe extern "C" fn print_direc(
  mut format: *mut libc::c_char,
  mut fmt_length: libc::c_uint,
  mut field_width: libc::c_int,
  mut precision: libc::c_int,
  mut argument: *const libc::c_char,
) {
  let mut current_block: u64;
  let mut llv: libc::c_longlong = 0;
  let mut dv: libc::c_double = 0.;
  let mut saved: libc::c_char = 0;
  let mut have_prec: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut have_width: *mut libc::c_char = 0 as *mut libc::c_char;
  saved = *format.offset(fmt_length as isize);
  *format.offset(fmt_length as isize) = '\u{0}' as i32 as libc::c_char;
  have_prec = strstr(format, b".*\x00" as *const u8 as *const libc::c_char);
  have_width = strchr(format, '*' as i32);
  if have_width.offset(-1) == have_prec {
    have_width = 0 as *mut libc::c_char
  }
  /* multiconvert sets errno = 0, but %s needs it cleared */
  *bb_errno = 0i32; /* switch */
  match *format.offset(fmt_length.wrapping_sub(1i32 as libc::c_uint) as isize) as libc::c_int {
    99 => {
      printf(format, *argument as libc::c_int);
      current_block = 12497913735442871383;
    }
    100 | 105 => {
      llv = my_xstrtoll(skip_whitespace(argument));
      current_block = 11311982278797531854;
    }
    111 | 117 | 120 | 88 => {
      llv = my_xstrtoull(skip_whitespace(argument)) as libc::c_longlong;
      current_block = 11311982278797531854;
    }
    115 => {
      /* Are char* and long long the same? */
      if ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_longlong>() as libc::c_ulong
      {
        llv = argument as ptrdiff_t as libc::c_longlong;
        current_block = 11311982278797531854;
      } else {
        /* Hope compiler will optimize it out by moving call
         * instruction after the ifs... */
        if have_width.is_null() {
          if have_prec.is_null() {
            printf(format, argument, argument, argument);
          } else {
            printf(format, precision, argument, argument);
          }
        } else if have_prec.is_null() {
          printf(format, field_width, argument, argument);
        } else {
          printf(format, field_width, precision, argument);
        }
        current_block = 12497913735442871383;
      }
    }
    102 | 101 | 69 | 103 | 71 => {
      dv = my_xstrtod(argument);
      if have_width.is_null() {
        if have_prec.is_null() {
          printf(format, dv);
        } else {
          printf(format, precision, dv);
        }
      } else if have_prec.is_null() {
        printf(format, field_width, dv);
      } else {
        printf(format, field_width, precision, dv);
      }
      current_block = 12497913735442871383;
    }
    _ => {
      current_block = 12497913735442871383;
    }
  }
  match current_block {
    11311982278797531854 =>
    /* cheat: unsigned long and long have same width, so... */
    {
      if have_width.is_null() {
        if have_prec.is_null() {
          printf(format, llv);
        } else {
          printf(format, precision, llv);
        }
      } else if have_prec.is_null() {
        printf(format, field_width, llv);
      } else {
        printf(format, field_width, precision, llv);
      }
    }
    _ => {}
  }
  *format.offset(fmt_length as isize) = saved;
}
/* Handle params for "%*.*f". Negative numbers are ok (compat). */
unsafe extern "C" fn get_width_prec(mut str: *const libc::c_char) -> libc::c_int {
  let mut v: libc::c_int = bb_strtoi(str, 0 as *mut *mut libc::c_char, 10i32);
  if *bb_errno != 0 {
    bb_error_msg(
      b"invalid number \'%s\'\x00" as *const u8 as *const libc::c_char,
      str,
    );
    v = 0i32
  }
  return v;
}
/* Print the text in FORMAT, using ARGV for arguments to any '%' directives.
Return advanced ARGV.  */
unsafe extern "C" fn print_formatted(
  mut f: *mut libc::c_char,
  mut argv: *mut *mut libc::c_char,
  mut conv_err: *mut libc::c_int,
) -> *mut *mut libc::c_char {
  let mut direc_start: *mut libc::c_char = 0 as *mut libc::c_char; /* Start of % directive.  */
  let mut direc_length: libc::c_uint = 0; /* Length of % directive.  */
  let mut field_width: libc::c_int = 0; /* Arg to first '*' */
  let mut precision: libc::c_int = 0; /* Arg to second '*' */
  let mut saved_argv: *mut *mut libc::c_char = argv; /* causes main() to exit */
  while *f != 0 {
    match *f as libc::c_int {
      37 => {
        let fresh0 = f;
        f = f.offset(1);
        direc_start = fresh0;
        direc_length = 1i32 as libc::c_uint;
        precision = 0i32;
        field_width = precision;
        if *f as libc::c_int == '%' as i32 {
          bb_putchar('%' as i32);
        } else if *f as libc::c_int == 'b' as i32 {
          if !(*argv).is_null() {
            if print_esc_string(*argv) != 0 {
              return saved_argv;
            }
            argv = argv.offset(1)
          }
        } else {
          if *f as libc::c_int != 0
            && !strchr(
              b"-+ #\x00" as *const u8 as *const libc::c_char,
              *f as libc::c_int,
            )
            .is_null()
          {
            f = f.offset(1);
            direc_length = direc_length.wrapping_add(1)
          }
          if *f as libc::c_int == '*' as i32 {
            f = f.offset(1);
            direc_length = direc_length.wrapping_add(1);
            if !(*argv).is_null() {
              let fresh1 = argv;
              argv = argv.offset(1);
              field_width = get_width_prec(*fresh1)
            }
          } else {
            while (*f as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
              f = f.offset(1);
              direc_length = direc_length.wrapping_add(1)
            }
          }
          if *f as libc::c_int == '.' as i32 {
            f = f.offset(1);
            direc_length = direc_length.wrapping_add(1);
            if *f as libc::c_int == '*' as i32 {
              f = f.offset(1);
              direc_length = direc_length.wrapping_add(1);
              if !(*argv).is_null() {
                let fresh2 = argv;
                argv = argv.offset(1);
                precision = get_width_prec(*fresh2)
              }
            } else {
              while (*f as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
                f = f.offset(1);
                direc_length = direc_length.wrapping_add(1)
              }
            }
          }
          /* Remove "lLhz" size modifiers, repeatedly.
           * bash does not like "%lld", but coreutils
           * happily takes even "%Llllhhzhhzd"!
           * We are permissive like coreutils */
          while *f as libc::c_int | 0x20i32 == 'l' as i32
            || *f as libc::c_int == 'h' as i32
            || *f as libc::c_int == 'z' as i32
          {
            overlapping_strcpy(f, f.offset(1));
          }
          /* Add "ll" if integer modifier, then print */
          static mut format_chars: [libc::c_char; 14] = [
            100, 105, 111, 117, 120, 88, 102, 101, 69, 103, 71, 99, 115, 0,
          ];
          let mut p: *mut libc::c_char = strchr(format_chars.as_ptr(), *f as libc::c_int);
          /* needed - try "printf %" without it */
          if p.is_null() || *f as libc::c_int == '\u{0}' as i32 {
            bb_error_msg(
              b"%s: invalid format\x00" as *const u8 as *const libc::c_char,
              direc_start,
            );
            /* causes main() to exit with error */
            return saved_argv.offset(-1);
          }
          direc_length = direc_length.wrapping_add(1);
          if p.wrapping_offset_from(format_chars.as_ptr()) as libc::c_long <= 5i32 as libc::c_long {
            /* it is one of "diouxX" */
            p = xmalloc(direc_length.wrapping_add(3i32 as libc::c_uint) as size_t)
              as *mut libc::c_char;
            memcpy(
              p as *mut libc::c_void,
              direc_start as *const libc::c_void,
              direc_length as libc::c_ulong,
            );
            *p.offset(direc_length.wrapping_add(1i32 as libc::c_uint) as isize) =
              *p.offset(direc_length.wrapping_sub(1i32 as libc::c_uint) as isize);
            *p.offset(direc_length.wrapping_sub(1i32 as libc::c_uint) as isize) =
              'l' as i32 as libc::c_char;
            *p.offset(direc_length as isize) = 'l' as i32 as libc::c_char;
            //bb_error_msg("<%s>", p);
            direc_length = direc_length.wrapping_add(2i32 as libc::c_uint);
            direc_start = p
          } else {
            p = 0 as *mut libc::c_char
          }
          if !(*argv).is_null() {
            let fresh3 = argv;
            argv = argv.offset(1);
            print_direc(direc_start, direc_length, field_width, precision, *fresh3);
          } else {
            print_direc(
              direc_start,
              direc_length,
              field_width,
              precision,
              b"\x00" as *const u8 as *const libc::c_char,
            );
          }
          *conv_err |= *bb_errno;
          free(p as *mut libc::c_void);
        }
      }
      92 => {
        f = f.offset(1);
        if *f as libc::c_int == 'c' as i32 {
          return saved_argv;
          /* causes main() to exit */
        }
        bb_putchar(bb_process_escape_sequence(
          &mut f as *mut *mut libc::c_char as *mut *const libc::c_char,
        ) as libc::c_int);
        f = f.offset(-1)
      }
      _ => {
        putchar_unlocked(*f as libc::c_int);
      }
    }
    f = f.offset(1)
  }
  return argv;
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
/* SO_REUSEADDR allows a server to rebind to an address that is already
 * "in use" by old connections to e.g. previous server instance which is
 * killed or crashed. Without it bind will fail until all such connections
 * time out. Linux does not allow multiple live binds on same ip:port
 * regardless of SO_REUSEADDR (unlike some other flavors of Unix).
 * Turn it on before you call bind(). */
/* On Linux this never fails. */
/* NB: returns port in host byte order */
/* Create stream socket, and allocate suitable lsa.
 * (lsa of correct size and lsa->sa.sa_family (AF_INET/AF_INET6))
 * af == AF_UNSPEC will result in trying to create IPv6 socket,
 * and if kernel doesn't support it, fall back to IPv4.
 * This is useful if you plan to bind to resulting local lsa.
 */
/* Create server socket bound to bindaddr:port. bindaddr can be NULL,
 * numeric IP ("N.N.N.N") or numeric IPv6 address,
 * and can have ":PORT" suffix (for IPv6 use "[X:X:...:X]:PORT").
 * Only if there is no suffix, port argument is used */
/* NB: these set SO_REUSEADDR before bind */
/* Create client TCP socket connected to peer:port. Peer cannot be NULL.
 * Peer can be numeric IP ("N.N.N.N"), numeric IPv6 address or hostname,
 * and can have ":PORT" suffix (for IPv6 use "[X:X:...:X]:PORT").
 * If there is no suffix, port argument is used */
/* Connect to peer identified by lsa */
/* Get local address of bound or accepted socket */
/* Get remote address of connected or accepted socket */
/* Return malloc'ed len_and_sockaddr with socket address of host:port
 * Currently will return IPv4 or IPv6 sockaddrs only
 * (depending on host), but in theory nothing prevents e.g.
 * UNIX socket address being returned, IPX sockaddr etc...
 * On error does bb_error_msg and returns NULL */
/* Version which dies on error */
/* Same, useful if you want to force family (e.g. IPv6) */
/* Assign sin[6]_port member if the socket is an AF_INET[6] one,
 * otherwise no-op. Useful for ftp.
 * NB: does NOT do htons() internally, just direct assignment. */
/* Retrieve sin[6]_port or return -1 for non-INET[6] lsa's */
/* Reverse DNS. Returns NULL on failure. */
/* This one doesn't append :PORTNUM */
/* This one also doesn't fall back to dotted IP (returns NULL) */
/* inet_[ap]ton on steroids */
// "old" (ipv4 only) API
// users: traceroute.c hostname.c - use _list_ of all IPs
// Also mount.c and inetd.c are using gethostbyname(),
// + inet_common.c has additional IPv4-only stuff
/* opaque */
// RFC 5246
// sequence number
//   Each connection state contains a sequence number, which is
//   maintained separately for read and write states.  The sequence
//   number MUST be set to zero whenever a connection state is made the
//   active state.  Sequence numbers are of type uint64 and may not
//   exceed 2^64-1.
/*uint64_t read_seq64_be;*/
/*u8 *server_write_MAC_key;*/
//used by AES_GCM
/* 0 if argv[0] is NULL: */
/* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
 * But potentially slow, don't use in one-billion-times loops */
/* Note: does not use stdio, writes to fd 2 directly */
// gcc-4.1.1 still isn't good enough at optimizing it
// (+200 bytes compared to macro)
//static ALWAYS_INLINE
//int LONE_DASH(const char *s) { return s[0] == '-' && !s[1]; }
//static ALWAYS_INLINE
//int NOT_LONE_DASH(const char *s) { return s[0] != '-' || s[1]; }
/* Returns a string with unprintable chars replaced by '?' or
 * SUBST_WCHAR. This function is unicode-aware. */
/* Prints unprintable char ch as ^C or M-c to file
 * (M-c is used only if ch is ORed with PRINTABLE_META),
 * else it is printed as-is (except for ch = 0x9b) */
/* Return a string that is the printable representation of character ch.
 * Buffer must hold at least four characters. */
// NB: will return short read on error, not -1,
// if some data was read before error occurred
// Reads one line a-la fgets (but doesn't save terminating '\n').
// Reads byte-by-byte. Useful when it is important to not read ahead.
// Bytes are appended to pfx (which must be malloced, or NULL).
/* Reads block up to *maxsz_p (default: INT_MAX - 4095) */
/* Returns NULL if file can't be opened (default max size: INT_MAX - 4095) */
/* Never returns NULL */
/* Else use variable one (a bit more expensive) */
/* Autodetects gzip/bzip2 formats. fd may be in the middle of the file! */
/* Autodetects .gz etc */
/* lzma has no signature, need a little helper. NB: exist only for ENABLE_FEATURE_SEAMLESS_LZMA=y */
// NB: will return short write on error, not -1,
// if some data was written before error occurred
/* Close fd, but check for failures (some types of write errors) */
/* Reads and prints to stdout till eof, then closes FILE. Exits on error: */
/* Reads a line from a text file, up to a newline or NUL byte, inclusive.
 * Returns malloc'ed char*. If end is NULL '\n' isn't considered
 * end of line. If end isn't NULL, length of the chunk is stored in it.
 * Returns NULL if EOF/error.
 */
/* Reads up to (and including) TERMINATING_STRING: */
/* Same, with limited max size, and returns the length (excluding NUL): */
/* Chops off TERMINATING_STRING from the end: */
/* Reads up to (and including) "\n" or NUL byte: */
/* Chops off '\n' from the end, unlike fgets: */
/* Same, but doesn't try to conserve space (may have some slack after the end) */
/* extern char *xmalloc_fgetline_fast(FILE *file) FAST_FUNC RETURNS_MALLOC; */
/* Prints warning to stderr and returns NULL on failure: */
/* "Opens" stdin if filename is special, else just opens file: */
/* not FAST_FUNC! */
/* Wrapper which restarts poll on EINTR or ENOMEM.
 * On other errors complains [perror("poll")] and returns.
 * Warning! May take (much) longer than timeout_ms to return!
 * If this is a problem, use bare poll and open-code EINTR/ENOMEM handling */
/* Convert each alpha char in str to lower-case */
/* Returns a pointer past the formatted number, does NOT null-terminate */
/* Intelligent formatters of bignums */
/* If block_size == 0, display size without fractional part,
 * else display (size * block_size) with one decimal digit.
 * If display_unit == 0, show value no bigger than 1024 with suffix (K,M,G...),
 * else divide by display_unit and do not use suffix. */
/* "1024.0G" */
//TODO: provide pointer to buf (avoid statics)?
/* Put a string of hex bytes ("1b2e66fe"...), return advanced pointer */
/* Reverse */
/* Generate a UUID */
/* Last element is marked by mult == 0 */
/* Specialized: */
/* Using xatoi() instead of naive atoi() is not always convenient -
 * in many places people want *non-negative* values, but store them
 * in signed int. Therefore we need this one:
 * dies if input is not in [0, INT_MAX] range. Also will reject '-0' etc.
 * It should really be named xatoi_nonnegative (since it allows 0),
 * but that would be too long.
 */
/* Useful for reading port numbers */
/* These parse entries in /etc/passwd and /etc/group.  This is desirable
 * for BusyBox since we want to avoid using the glibc NSS stuff, which
 * increases target size and is often not needed on embedded systems.  */
/* wrapper: allows string to contain numeric uid or gid */
/* always sets uid and gid; returns 0 on failure */
/* always sets uid and gid; exits on failure */
/* chown-like handling of "user[:[group]" */
/* versions which cache results (useful for ps, ls etc) */
/* internally usernames are saved in fixed-sized char[] buffers */
/*
 * Returns (-1) terminated malloced result of getgroups().
 * Reallocs group_array (useful for repeated calls).
 * ngroups is an initial size of array. It is rounded up to 32 for realloc.
 * ngroups is updated on return.
 * ngroups can be NULL: bb_getgroups(NULL, NULL) is valid usage.
 * Dies on errors (on Linux, only xrealloc can cause this, not internal getgroups call).
 */
/* BB_EXECxx always execs (it's not doing NOFORK/NOEXEC stuff),
 * but it may exec busybox and call applet instead of searching PATH.
 */
/* xvfork() can't be a _function_, return after vfork in child mangles stack
 * in the parent. It must be a macro. */
/* NOMMU friendy fork+exec: */
/* wait4pid: unlike waitpid, waits ONLY for one process.
 * Returns sig + 0x180 if child is killed by signal.
 * It's safe to pass negative 'pids' from failed [v]fork -
 * wait4pid will return -1 (and will not clobber [v]fork's errno).
 * IOW: rc = wait4pid(spawn(argv));
 *      if (rc < 0) bb_perror_msg("%s", argv[0]);
 *      if (rc > 0) bb_error_msg("exit code: %d", rc & 0xff);
 */
/* ***********************************************************************/
/* spawn_and_wait/run_nofork_applet/run_applet_no_and_exit need to work */
/* carefully together to reinit some global state while not disturbing  */
/* other. Be careful if you change them. Consult docs/nofork_noexec.txt */
/* ***********************************************************************/
/* Same as wait4pid(spawn(argv)), but with NOFORK/NOEXEC if configured: */
/* Does NOT check that applet is NOFORK, just blindly runs it */
/* Helpers for daemonization.
 *
 * bb_daemonize(flags) = daemonize, does not compile on NOMMU
 *
 * bb_daemonize_or_rexec(flags, argv) = daemonizes on MMU (and ignores argv),
 *      rexec's itself on NOMMU with argv passed as command line.
 * Thus bb_daemonize_or_rexec may cause your <applet>_main() to be re-executed
 * from the start. (It will detect it and not reexec again second time).
 * You have to audit carefully that you don't do something twice as a result
 * (opening files/sockets, parsing config files etc...)!
 *
 * Both of the above will redirect fd 0,1,2 to /dev/null and drop ctty
 * (will do setsid()).
 *
 * fork_or_rexec(argv) = bare-bones fork on MMU,
 *      "vfork + re-exec ourself" on NOMMU. No fd redirection, no setsid().
 *      On MMU ignores argv.
 *
 * Helper for network daemons in foreground mode:
 *
 * bb_sanitize_stdio() = make sure that fd 0,1,2 are opened by opening them
 * to /dev/null if they are not.
 */
/* internal use */
//DAEMON_DOUBLE_FORK     = 1 << 4, /* double fork to avoid controlling tty */
/* Clear dangerous stuff, set PATH. Return 1 if was run by different user. */
/* For top, ps. Some argv[i] are replaced by malloced "-opt" strings */
/* { "-", NULL } */
/* BSD-derived getopt() functions require that optind be set to 1 in
 * order to reset getopt() state.  This used to be generally accepted
 * way of resetting getopt().  However, glibc's getopt()
 * has additional getopt() state beyond optind (specifically, glibc
 * extensions such as '+' and '-' at the start of the string), and requires
 * that optind be set to zero to reset its state.  BSD-derived versions
 * of getopt() misbehaved if optind is set to 0 in order to reset getopt(),
 * and glibc's getopt() used to coredump if optind is set 1 in order
 * to reset getopt().
 * Then BSD introduced additional variable "optreset" which should be
 * set to 1 in order to reset getopt().  Sigh.  Standards, anyone?
 *
 * By ~2008, OpenBSD 3.4 was changed to survive glibc-like optind = 0
 * (to interpret it as if optreset was set).
 */
/*def __GLIBC__*/
/* BSD style */
/* Having next pointer as a first member allows easy creation
 * of "llist-compatible" structs, and using llist_FOO functions
 * on them.
 */
/* BTW, surprisingly, changing API to
 *   llist_t *llist_add_to(llist_t *old_head, void *data)
 * etc does not result in smaller code... */
/* start_stop_daemon and udhcpc are special - they want
 * to create pidfiles regardless of FEATURE_PIDFILE */
/* True only if we created pidfile which is *file*, not /dev/null etc */
/* We need to export XXX_main from libbusybox
 * only if we build "individual" binaries
 */
/* Embedded script support */
/* Applets which are useful from another applets */
/* If shell needs them, they exist even if not enabled as applets */
#[no_mangle]
pub unsafe extern "C" fn printf_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut conv_err: libc::c_int = 0;
  let mut format: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut argv2: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  /* We must check that stdout is not closed.
   * The reason for this is highly non-obvious.
   * printf_main is used from shell.
   * Shell must correctly handle 'printf "%s" foo'
   * if stdout is closed. With stdio, output gets shoveled into
   * stdout buffer, and even fflush cannot clear it out. It seems that
   * even if libc receives EBADF on write attempts, it feels determined
   * to output data no matter what. So it will try later,
   * and possibly will clobber future output. Not good. */
  // TODO: check fcntl() & O_ACCMODE == O_WRONLY or O_RDWR?
  if fcntl(1i32, 3i32) == -1i32 {
    return 1i32;
  } /* match coreutils 6.10 (sans error msg to stderr) */
  //if (dup2(1, 1) != 1) - old way
  //	return 1;
  /* bash builtin errors out on "printf '-%s-\n' foo",
   * coreutils-6.9 works. Both work with "printf -- '-%s-\n' foo".
   * We will mimic coreutils. */
  if !(*argv.offset(1)).is_null()
    && *(*argv.offset(1)).offset(0) as libc::c_int == '-' as i32
    && *(*argv.offset(1)).offset(1) as libc::c_int == '-' as i32
    && *(*argv.offset(1)).offset(2) == 0
  {
    argv = argv.offset(1)
  }
  if (*argv.offset(1)).is_null() {
    if 1i32 != 0 && *applet_name.offset(0) as libc::c_int != 'p' as i32 {
      bb_simple_error_msg(
        b"usage: printf FORMAT [ARGUMENT...]\x00" as *const u8 as *const libc::c_char,
      );
      return 2i32;
      /* bash compat */
    }
    bb_show_usage();
  }
  format = *argv.offset(1);
  argv2 = argv.offset(2);
  conv_err = 0i32;
  loop {
    argv = argv2;
    argv2 = print_formatted(format, argv, &mut conv_err);
    if !(argv2 > argv && !(*argv2).is_null()) {
      break;
    }
  }
  /* coreutils compat (bash doesn't do this):
  if (*argv)
    fprintf(stderr, "excess args ignored");
  */
  return (argv2 < argv || conv_err != 0) as libc::c_int;
  /* print_formatted saw invalid number */
}

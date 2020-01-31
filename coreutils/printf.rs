use crate::libbb::appletlib::applet_name;
use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::skip_whitespace::skip_whitespace;
use crate::libbb::xfuncs_printf::xmalloc;
use crate::librb::size_t;
use libc;
use libc::free;
use libc::printf;
use libc::ptrdiff_t;
use libc::putchar_unlocked;
use libc::strchr;
use libc::strstr;
extern "C" {
  #[no_mangle]
  fn strtod(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char) -> libc::c_double;

  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

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

}

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
//applet:IF_PRINTF(APPLET_NOFORK(printf, printf, BB_DIR_USR_BIN, SUID_DROP, printf))
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
pub type converter = Option<unsafe fn(_: *const libc::c_char, _: *mut libc::c_void) -> ()>;

unsafe fn multiconvert(
  mut arg: *const libc::c_char,
  mut result: *mut libc::c_void,
  mut convert: converter,
) -> libc::c_int {
  if *arg as libc::c_int == '\"' as i32 || *arg as libc::c_int == '\'' as i32 {
    arg = crate::libbb::xfuncs::utoa(*arg.offset(1) as libc::c_uchar as libc::c_uint)
  }
  *bb_errno = 0;
  convert.expect("non-null function pointer")(arg, result);
  if *bb_errno != 0 {
    crate::libbb::verror_msg::bb_error_msg(
      b"invalid number \'%s\'\x00" as *const u8 as *const libc::c_char,
      arg,
    );
    return 1;
  }
  return 0;
}

unsafe fn conv_strtoull(mut arg: *const libc::c_char, mut result: *mut libc::c_void) {
  /* Allow leading '+' - bb_strtoull() by itself does not allow it,
   * and probably shouldn't (other callers might require purely numeric
   * inputs to be allowed.
   */
  if *arg.offset(0) as libc::c_int == '+' as i32 {
    arg = arg.offset(1)
  }
  *(result as *mut libc::c_ulonglong) =
    crate::libbb::bb_strtonum::bb_strtoull(arg, 0 as *mut *mut libc::c_char, 0);
  /* both coreutils 6.10 and bash 3.2:
   * $ printf '%x\n' -2
   * fffffffffffffffe
   * Mimic that:
   */
  if *bb_errno != 0 {
    *(result as *mut libc::c_ulonglong) =
      crate::libbb::bb_strtonum::bb_strtoll(arg, 0 as *mut *mut libc::c_char, 0)
        as libc::c_ulonglong
  };
}

unsafe fn conv_strtoll(mut arg: *const libc::c_char, mut result: *mut libc::c_void) {
  if *arg.offset(0) as libc::c_int == '+' as i32 {
    arg = arg.offset(1)
  }
  *(result as *mut libc::c_longlong) =
    crate::libbb::bb_strtonum::bb_strtoll(arg, 0 as *mut *mut libc::c_char, 0);
}

unsafe fn conv_strtod(mut arg: *const libc::c_char, mut result: *mut libc::c_void) {
  let mut end: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  /* Well, this one allows leading whitespace... so what? */
  /* What I like much less is that "-" accepted too! :( */
  *(result as *mut libc::c_double) = strtod(arg, &mut end);
  if *end.offset(0) != 0 {
    *bb_errno = 34i32;
    *(result as *mut libc::c_double) = 0 as libc::c_double
  };
}

/* Callers should check errno to detect errors */
unsafe fn my_xstrtoull(mut arg: *const libc::c_char) -> libc::c_ulonglong {
  let mut result: libc::c_ulonglong = 0;
  if multiconvert(
    arg,
    &mut result as *mut libc::c_ulonglong as *mut libc::c_void,
    Some(conv_strtoull as unsafe fn(_: *const libc::c_char, _: *mut libc::c_void) -> ()),
  ) != 0
  {
    result = 0 as libc::c_ulonglong
  }
  return result;
}

unsafe fn my_xstrtoll(mut arg: *const libc::c_char) -> libc::c_longlong {
  let mut result: libc::c_longlong = 0;
  if multiconvert(
    arg,
    &mut result as *mut libc::c_longlong as *mut libc::c_void,
    Some(conv_strtoll as unsafe fn(_: *const libc::c_char, _: *mut libc::c_void) -> ()),
  ) != 0
  {
    result = 0 as libc::c_longlong
  }
  return result;
}

unsafe fn my_xstrtod(mut arg: *const libc::c_char) -> libc::c_double {
  let mut result: libc::c_double = 0.;
  multiconvert(
    arg,
    &mut result as *mut libc::c_double as *mut libc::c_void,
    Some(conv_strtod as unsafe fn(_: *const libc::c_char, _: *mut libc::c_void) -> ()),
  );
  return result;
}

/* Handles %b; return 1 if output is to be short-circuited by \c */
unsafe fn print_esc_string(mut str: *const libc::c_char) -> libc::c_int {
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
      c = crate::libbb::process_escape_sequence::bb_process_escape_sequence(&mut z);
      str = z
    }
    putchar_unlocked(c as libc::c_int);
  }
  return 0;
}

unsafe fn print_direc(
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
  let mut have_prec: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut have_width: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  saved = *format.offset(fmt_length as isize);
  *format.offset(fmt_length as isize) = '\u{0}' as i32 as libc::c_char;
  have_prec = strstr(format, b".*\x00" as *const u8 as *const libc::c_char);
  have_width = strchr(format, '*' as i32);
  if have_width.offset(-1) == have_prec {
    have_width = std::ptr::null_mut::<libc::c_char>()
  }
  /* multiconvert sets errno = 0, but %s needs it cleared */
  *bb_errno = 0; /* switch */
  match *format.offset(fmt_length.wrapping_sub(1) as isize) as libc::c_int {
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
unsafe fn get_width_prec(mut str: *const libc::c_char) -> libc::c_int {
  let mut v: libc::c_int =
    crate::libbb::bb_strtonum::bb_strtoi(str, 0 as *mut *mut libc::c_char, 10i32);
  if *bb_errno != 0 {
    crate::libbb::verror_msg::bb_error_msg(
      b"invalid number \'%s\'\x00" as *const u8 as *const libc::c_char,
      str,
    );
    v = 0
  }
  return v;
}

/* Print the text in FORMAT, using ARGV for arguments to any '%' directives.
Return advanced ARGV.  */
unsafe fn print_formatted(
  mut f: *mut libc::c_char,
  mut argv: *mut *mut libc::c_char,
  mut conv_err: *mut libc::c_int,
) -> *mut *mut libc::c_char {
  let mut direc_start: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>(); /* Start of % directive.  */
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
        direc_length = 1;
        precision = 0;
        field_width = precision;
        if *f as libc::c_int == '%' as i32 {
          crate::libbb::xfuncs_printf::bb_putchar('%' as i32);
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
            crate::libbb::safe_strncpy::overlapping_strcpy(f, f.offset(1));
          }
          /* Add "ll" if integer modifier, then print */
          static mut format_chars: [libc::c_char; 14] = [
            100, 105, 111, 117, 120, 88, 102, 101, 69, 103, 71, 99, 115, 0,
          ];
          let mut p: *mut libc::c_char = strchr(format_chars.as_ptr(), *f as libc::c_int);
          /* needed - try "printf %" without it */
          if p.is_null() || *f as libc::c_int == '\u{0}' as i32 {
            crate::libbb::verror_msg::bb_error_msg(
              b"%s: invalid format\x00" as *const u8 as *const libc::c_char,
              direc_start,
            );
            /* causes main() to exit with error */
            return saved_argv.offset(-1);
          }
          direc_length = direc_length.wrapping_add(1);
          if p.wrapping_offset_from(format_chars.as_ptr()) as libc::c_long <= 5i32 as libc::c_long {
            /* it is one of "diouxX" */
            p = xmalloc(direc_length.wrapping_add(3) as size_t) as *mut libc::c_char;
            memcpy(
              p as *mut libc::c_void,
              direc_start as *const libc::c_void,
              direc_length as libc::c_ulong,
            );
            *p.offset(direc_length.wrapping_add(1) as isize) =
              *p.offset(direc_length.wrapping_sub(1) as isize);
            *p.offset(direc_length.wrapping_sub(1) as isize) = 'l' as i32 as libc::c_char;
            *p.offset(direc_length as isize) = 'l' as i32 as libc::c_char;
            //bb_error_msg("<%s>", p);
            direc_length = direc_length.wrapping_add(2);
            direc_start = p
          } else {
            p = std::ptr::null_mut::<libc::c_char>()
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
        crate::libbb::xfuncs_printf::bb_putchar(
          crate::libbb::process_escape_sequence::bb_process_escape_sequence(
            &mut f as *mut *mut libc::c_char as *mut *const libc::c_char,
          ) as libc::c_int,
        );
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

pub unsafe fn printf_main(mut _argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut conv_err: libc::c_int = 0;
  let mut format: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut argv2: *mut *mut libc::c_char = std::ptr::null_mut();
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
  if libc::fcntl(1, 3) == -1 {
    return 1;
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
    if 1 != 0 && *applet_name.offset(0) as libc::c_int != 'p' as i32 {
      crate::libbb::verror_msg::bb_simple_error_msg(
        b"usage: printf FORMAT [ARGUMENT...]\x00" as *const u8 as *const libc::c_char,
      );
      return 2;
      /* bash compat */
    }
    crate::libbb::appletlib::bb_show_usage();
  }
  format = *argv.offset(1);
  argv2 = argv.offset(2);
  conv_err = 0;
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

use crate::libbb::xfuncs_printf::xmalloc;
use libc;
use libc::getenv;
use libc::printf;
use libc::puts;
use libc::strchr;
use libc::strcmp;
use libc::strtok;
extern "C" {
  #[no_mangle]
  static mut optarg: *mut libc::c_char;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut opterr: libc::c_int;
  #[no_mangle]
  fn getopt_long(
    ___argc: libc::c_int,
    ___argv: *const *mut libc::c_char,
    __shortopts: *const libc::c_char,
    __longopts: *const option,
    __longind: *mut libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn getopt_long_only(
    ___argc: libc::c_int,
    ___argv: *const *mut libc::c_char,
    __shortopts: *const libc::c_char,
    __longopts: *const option,
    __longind: *mut libc::c_int,
  ) -> libc::c_int;

  #[no_mangle]
  fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  fn xrealloc_vector_helper(
    vector: *mut libc::c_void,
    sizeof_and_shift: libc::c_uint,
    idx: libc::c_int,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> u32;
  #[no_mangle]
  fn llist_pop(elm: *mut *mut llist_t) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
  pub name: *const libc::c_char,
  pub has_arg: libc::c_int,
  pub flag: *mut libc::c_int,
  pub val: libc::c_int,
}

use crate::libbb::llist::llist_t;
use crate::librb::size_t;
/*
 * getopt.c - Enhanced implementation of BSD getopt(1)
 * Copyright (c) 1997, 1998, 1999, 2000  Frodo Looijaard <frodol@dds.nl>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/*
 * Version 1.0-b4: Tue Sep 23 1997. First public release.
 * Version 1.0: Wed Nov 19 1997.
 *   Bumped up the version number to 1.0
 *   Fixed minor typo (CSH instead of TCSH)
 * Version 1.0.1: Tue Jun 3 1998
 *   Fixed sizeof instead of strlen bug
 *   Bumped up the version number to 1.0.1
 * Version 1.0.2: Thu Jun 11 1998 (not present)
 *   Fixed gcc-2.8.1 warnings
 *   Fixed --version/-V option (not present)
 * Version 1.0.5: Tue Jun 22 1999
 *   Make -u option work (not present)
 * Version 1.0.6: Tue Jun 27 2000
 *   No important changes
 * Version 1.1.0: Tue Jun 30 2000
 *   Added NLS support (partly written by Arkadiusz Mickiewicz
 *     <misiek@misiek.eu.org>)
 * Ported to Busybox - Alfred M. Szmidt <ams@trillian.itslinux.org>
 *  Removed --version/-V and --help/-h
 *  Removed parse_error(), using bb_error_msg() from Busybox instead
 *  Replaced our_malloc with xmalloc and our_realloc with xrealloc
 */
//config:config GETOPT
//config:	bool "getopt (5.8 kb)"
//config:	default y
//config:	help
//config:	The getopt utility is used to break up (parse) options in command
//config:	lines to make it easy to write complex shell scripts that also check
//config:	for legal (and illegal) options. If you want to write horribly
//config:	complex shell scripts, or use some horribly complex shell script
//config:	written by others, this utility may be for you. Most people will
//config:	wisely leave this disabled.
//config:
//config:config FEATURE_GETOPT_LONG
//config:	bool "Support -l LONGOPTs"
//config:	default y
//config:	depends on GETOPT && LONG_OPTS
//config:	help
//config:	Enable support for long options (option -l).
//applet:IF_GETOPT(APPLET_NOEXEC(getopt, getopt, BB_DIR_BIN, BB_SUID_DROP, getopt))
//kbuild:lib-$(CONFIG_GETOPT) += getopt.o
//usage:#define getopt_trivial_usage
//usage:       "[OPTIONS] [--] OPTSTRING PARAMS"
//usage:#define getopt_full_usage "\n\n"
//usage:	IF_FEATURE_GETOPT_LONG(
//usage:       "	-a		Allow long options starting with single -\n"
//usage:       "	-l LOPT[,...]	Long options to recognize\n"
//usage:	)
//usage:       "	-n PROGNAME	The name under which errors are reported"
//usage:     "\n	-o OPTSTRING	Short options to recognize"
//usage:     "\n	-q		No error messages on unrecognized options"
//usage:     "\n	-Q		No normal output"
//usage:     "\n	-s SHELL	Set shell quoting conventions"
//usage:     "\n	-T		Version test (exits with 4)"
//usage:     "\n	-u		Don't quote output"
//usage:	IF_FEATURE_GETOPT_LONG( /* example uses -l, needs FEATURE_GETOPT_LONG */
//usage:     "\n"
//usage:     "\nExample:"
//usage:     "\n"
//usage:     "\nO=`getopt -l bb: -- ab:c:: \"$@\"` || exit 1"
//usage:     "\neval set -- \"$O\""
//usage:     "\nwhile true; do"
//usage:     "\n	case \"$1\" in"
//usage:     "\n	-a)	echo A; shift;;"
//usage:     "\n	-b|--bb) echo \"B:'$2'\"; shift 2;;"
//usage:     "\n	-c)	case \"$2\" in"
//usage:     "\n		\"\")	echo C; shift 2;;"
//usage:     "\n		*)	echo \"C:'$2'\"; shift 2;;"
//usage:     "\n		esac;;"
//usage:     "\n	--)	shift; break;;"
//usage:     "\n	*)	echo Error; exit 1;;"
//usage:     "\n	esac"
//usage:     "\ndone"
//usage:	)
//usage:
//usage:#define getopt_example_usage
//usage:       "$ cat getopt.test\n"
//usage:       "#!/bin/sh\n"
//usage:       "GETOPT=`getopt -o ab:c:: --long a-long,b-long:,c-long:: \\\n"
//usage:       "       -n 'example.busybox' -- \"$@\"`\n"
//usage:       "if [ $? != 0 ]; then exit 1; fi\n"
//usage:       "eval set -- \"$GETOPT\"\n"
//usage:       "while true; do\n"
//usage:       " case $1 in\n"
//usage:       "   -a|--a-long) echo \"Option a\"; shift;;\n"
//usage:       "   -b|--b-long) echo \"Option b, argument '$2'\"; shift 2;;\n"
//usage:       "   -c|--c-long)\n"
//usage:       "     case \"$2\" in\n"
//usage:       "       \"\") echo \"Option c, no argument\"; shift 2;;\n"
//usage:       "       *)  echo \"Option c, argument '$2'\"; shift 2;;\n"
//usage:       "     esac;;\n"
//usage:       "   --) shift; break;;\n"
//usage:       "   *) echo \"Internal error!\"; exit 1;;\n"
//usage:       " esac\n"
//usage:       "done\n"
/* NON_OPT is the code that is returned when a non-option is found in '+'
mode */
pub type C2RustUnnamed = libc::c_uint;
/* LONG_OPT is the code that is returned when a long option is found. */
pub const LONG_OPT: C2RustUnnamed = 2;
pub const NON_OPT: C2RustUnnamed = 1;

/* For finding activated option flags. Must match getopt32 call! */
pub type C2RustUnnamed_0 = libc::c_uint;
/* hijack this bit for other purposes */
// -l
pub const SHELL_IS_TCSH: C2RustUnnamed_0 = 32768;
// -a
// pub const OPT_l: C2RustUnnamed_0 = 256;
// -u
pub const OPT_a: C2RustUnnamed_0 = 128;
// -T
pub const OPT_u: C2RustUnnamed_0 = 64;
// -s
pub const OPT_T: C2RustUnnamed_0 = 32;
// -Q
pub const OPT_s: C2RustUnnamed_0 = 16;
// -q
pub const OPT_Q: C2RustUnnamed_0 = 8;
// -n
pub const OPT_q: C2RustUnnamed_0 = 4;
// -o
// pub const OPT_n: C2RustUnnamed_0 = 2;
// pub const OPT_o: C2RustUnnamed_0 = 1;

/* 0 is getopt_long, 1 is getopt_long_only */
/*
 * This function 'normalizes' a single argument: it puts single quotes around
 * it and escapes other special characters. If quote is false, it just
 * returns its argument.
 * Bash only needs special treatment for single quotes; tcsh also recognizes
 * exclamation marks within single quotes, and nukes whitespace.
 * This function returns a pointer to a buffer that is overwritten by
 * each call.
 */
unsafe extern "C" fn normalize(mut arg: *const libc::c_char) -> *const libc::c_char {
  let mut bufptr: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut BUFFER: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  if option_mask32 & OPT_u as libc::c_int as libc::c_uint != 0 {
    /* Just copy arg */
    BUFFER = xstrdup(arg);
    return BUFFER;
  }
  /* Each character in arg may take up to four characters in the result:
  For a quote we need a closing quote, a backslash, a quote and an
  opening quote! We need also the global opening and closing quote,
  and one extra character for '\0'. */
  BUFFER = xmalloc(
    strlen(arg)
      .wrapping_mul(4i32 as libc::c_ulong)
      .wrapping_add(3i32 as libc::c_ulong),
  ) as *mut libc::c_char;
  bufptr = BUFFER;
  let fresh0 = bufptr;
  bufptr = bufptr.offset(1);
  *fresh0 = '\'' as i32 as libc::c_char;
  while *arg != 0 {
    if *arg as libc::c_int == '\'' as i32 {
      /* Quote: replace it with: '\'' */
      let fresh1 = bufptr;
      bufptr = bufptr.offset(1);
      *fresh1 = '\'' as i32 as libc::c_char;
      let fresh2 = bufptr;
      bufptr = bufptr.offset(1);
      *fresh2 = '\\' as i32 as libc::c_char;
      let fresh3 = bufptr;
      bufptr = bufptr.offset(1);
      *fresh3 = '\'' as i32 as libc::c_char;
      let fresh4 = bufptr;
      bufptr = bufptr.offset(1);
      *fresh4 = '\'' as i32 as libc::c_char
    } else if option_mask32 & SHELL_IS_TCSH as libc::c_int as libc::c_uint != 0
      && *arg as libc::c_int == '!' as i32
    {
      /* Exclamation mark: replace it with: \! */
      let fresh5 = bufptr;
      bufptr = bufptr.offset(1);
      *fresh5 = '\'' as i32 as libc::c_char;
      let fresh6 = bufptr;
      bufptr = bufptr.offset(1);
      *fresh6 = '\\' as i32 as libc::c_char;
      let fresh7 = bufptr;
      bufptr = bufptr.offset(1);
      *fresh7 = '!' as i32 as libc::c_char;
      let fresh8 = bufptr;
      bufptr = bufptr.offset(1);
      *fresh8 = '\'' as i32 as libc::c_char
    } else if option_mask32 & SHELL_IS_TCSH as libc::c_int as libc::c_uint != 0
      && *arg as libc::c_int == '\n' as i32
    {
      /* Newline: replace it with: \n */
      let fresh9 = bufptr;
      bufptr = bufptr.offset(1);
      *fresh9 = '\\' as i32 as libc::c_char;
      let fresh10 = bufptr;
      bufptr = bufptr.offset(1);
      *fresh10 = 'n' as i32 as libc::c_char
    } else if option_mask32 & SHELL_IS_TCSH as libc::c_int as libc::c_uint != 0
      && ({
        let mut bb__isspace: libc::c_uchar = (*arg as libc::c_int - 9i32) as libc::c_uchar;
        (bb__isspace as libc::c_int == ' ' as i32 - 9i32
          || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
      }) != 0
    {
      /* Non-newline whitespace: replace it with \<ws> */
      let fresh11 = bufptr;
      bufptr = bufptr.offset(1);
      *fresh11 = '\'' as i32 as libc::c_char;
      let fresh12 = bufptr;
      bufptr = bufptr.offset(1);
      *fresh12 = '\\' as i32 as libc::c_char;
      let fresh13 = bufptr;
      bufptr = bufptr.offset(1);
      *fresh13 = *arg;
      let fresh14 = bufptr;
      bufptr = bufptr.offset(1);
      *fresh14 = '\'' as i32 as libc::c_char
    } else {
      /* Just copy */
      let fresh15 = bufptr;
      bufptr = bufptr.offset(1);
      *fresh15 = *arg
    }
    arg = arg.offset(1)
  }
  let fresh16 = bufptr;
  bufptr = bufptr.offset(1);
  *fresh16 = '\'' as i32 as libc::c_char;
  let fresh17 = bufptr;
  bufptr = bufptr.offset(1);
  *fresh17 = '\u{0}' as i32 as libc::c_char;
  return BUFFER;
}
/*
 * Generate the output. argv[0] is the program name (used for reporting errors).
 * argv[1..] contains the options to be parsed. argc must be the number of
 * elements in argv (ie. 1 if there are no options, only the program name),
 * optstr must contain the short options, and longopts the long options.
 * Other settings are found in global variables.
 */
unsafe extern "C" fn generate_output(
  mut argv: *mut *mut libc::c_char,
  mut argc: libc::c_int,
  mut optstr: *const libc::c_char,
  mut longopts: *const option,
) -> libc::c_int {
  let mut exit_code: libc::c_int = 0i32; /* We assume everything will be OK */
  if option_mask32 & OPT_q as libc::c_int as libc::c_uint != 0 {
    /* No error reporting from getopt(3) */
    opterr = 0i32
  }
  /* We used it already in main() in getopt32(),
   * we *must* reset getopt(3): */
  optind = 0i32;
  loop {
    let mut longindex: libc::c_int = 0;
    let mut opt: libc::c_int = if option_mask32 & OPT_a as libc::c_int as libc::c_uint != 0 {
      getopt_long_only(argc, argv, optstr, longopts, &mut longindex)
    } else {
      getopt_long(argc, argv, optstr, longopts, &mut longindex)
    };
    if opt == -1i32 {
      break;
    }
    if opt == '?' as i32 || opt == ':' as i32 {
      exit_code = 1i32
    } else if option_mask32 & OPT_Q as libc::c_int as libc::c_uint == 0 {
      if opt == LONG_OPT as libc::c_int {
        printf(
          b" --%s\x00" as *const u8 as *const libc::c_char,
          (*longopts.offset(longindex as isize)).name,
        );
        if (*longopts.offset(longindex as isize)).has_arg != 0 {
          printf(
            b" %s\x00" as *const u8 as *const libc::c_char,
            normalize(if !optarg.is_null() {
              optarg
            } else {
              b"\x00" as *const u8 as *const libc::c_char
            }),
          );
        }
      } else if opt == NON_OPT as libc::c_int {
        printf(
          b" %s\x00" as *const u8 as *const libc::c_char,
          normalize(optarg),
        );
      } else {
        let mut charptr: *const libc::c_char = 0 as *const libc::c_char;
        printf(b" -%c\x00" as *const u8 as *const libc::c_char, opt);
        charptr = strchr(optstr, opt);
        if !charptr.is_null() && {
          charptr = charptr.offset(1);
          (*charptr as libc::c_int) == ':' as i32
        } {
          printf(
            b" %s\x00" as *const u8 as *const libc::c_char,
            normalize(if !optarg.is_null() {
              optarg
            } else {
              b"\x00" as *const u8 as *const libc::c_char
            }),
          );
        }
      }
    }
  }
  if option_mask32 & OPT_Q as libc::c_int as libc::c_uint == 0 {
    let mut idx: libc::c_uint = 0;
    printf(b" --\x00" as *const u8 as *const libc::c_char);
    idx = optind as libc::c_uint;
    while !(*argv.offset(idx as isize)).is_null() {
      let fresh18 = idx;
      idx = idx.wrapping_add(1);
      printf(
        b" %s\x00" as *const u8 as *const libc::c_char,
        normalize(*argv.offset(fresh18 as isize)),
      );
    }
    bb_putchar('\n' as i32);
  }
  return exit_code;
}
/*
 * Register several long options. options is a string of long options,
 * separated by commas or whitespace.
 * This nukes options!
 */
unsafe extern "C" fn add_long_options(
  mut long_options: *mut option,
  mut options: *mut libc::c_char,
) -> *mut option {
  let mut long_nr: libc::c_int = 0i32;
  let mut arg_opt: libc::c_int = 0;
  let mut tlen: libc::c_int = 0;
  let mut tokptr: *mut libc::c_char =
    strtok(options, b", \t\n\x00" as *const u8 as *const libc::c_char);
  if !long_options.is_null() {
    while !(*long_options.offset(long_nr as isize)).name.is_null() {
      long_nr += 1
    }
  }
  while !tokptr.is_null() {
    arg_opt = 0i32;
    tlen = strlen(tokptr) as libc::c_int;
    if tlen != 0 {
      tlen -= 1;
      if *tokptr.offset(tlen as isize) as libc::c_int == ':' as i32 {
        arg_opt = 1i32;
        if tlen != 0 && *tokptr.offset((tlen - 1i32) as isize) as libc::c_int == ':' as i32 {
          tlen -= 1;
          arg_opt = 2i32
        }
        *tokptr.offset(tlen as isize) = '\u{0}' as i32 as libc::c_char;
        if tlen == 0i32 {
          bb_simple_error_msg_and_die(
            b"empty long option specified\x00" as *const u8 as *const libc::c_char,
          );
        }
      }
      long_options = xrealloc_vector_helper(
        long_options as *mut libc::c_void,
        ((::std::mem::size_of::<option>() as libc::c_ulong) << 8i32)
          .wrapping_add(4i32 as libc::c_ulong) as libc::c_uint,
        long_nr,
      ) as *mut option;
      (*long_options.offset(long_nr as isize)).has_arg = arg_opt;
      /*memset(&long_options[long_nr], 0, sizeof(long_options[0])); - xrealloc_vector did it */
      (*long_options.offset(long_nr as isize)).val = LONG_OPT as libc::c_int;
      let ref mut fresh19 = (*long_options.offset(long_nr as isize)).name;
      *fresh19 = xstrdup(tokptr);
      long_nr += 1
    }
    tokptr = strtok(
      std::ptr::null_mut::<libc::c_char>(),
      b", \t\n\x00" as *const u8 as *const libc::c_char,
    )
  }
  return long_options;
}
unsafe extern "C" fn set_shell(mut new_shell: *const libc::c_char) {
  if strcmp(new_shell, b"bash\x00" as *const u8 as *const libc::c_char) == 0i32
    || strcmp(new_shell, b"sh\x00" as *const u8 as *const libc::c_char) == 0i32
  {
    return;
  }
  if strcmp(new_shell, b"tcsh\x00" as *const u8 as *const libc::c_char) == 0i32
    || strcmp(new_shell, b"csh\x00" as *const u8 as *const libc::c_char) == 0i32
  {
    option_mask32 |= SHELL_IS_TCSH as libc::c_int as libc::c_uint
  } else {
    bb_error_msg(
      b"unknown shell \'%s\', assuming bash\x00" as *const u8 as *const libc::c_char,
      new_shell,
    );
  };
}
/*long_options[long_nr].flag = NULL; - xrealloc_vector did it */
/* Exit codes:
 *   0) No errors, successful operation.
 *   1) getopt(3) returned an error.
 *   2) A problem with parameter parsing for getopt(1).
 *   3) Internal error, out of memory
 *   4) Returned for -T
 */
static mut getopt_longopts: [libc::c_char; 95] = [
  111, 112, 116, 105, 111, 110, 115, 0, 1, 111, 108, 111, 110, 103, 111, 112, 116, 105, 111, 110,
  115, 0, 1, 108, 113, 117, 105, 101, 116, 0, 0, 113, 113, 117, 105, 101, 116, 45, 111, 117, 116,
  112, 117, 116, 0, 0, 81, 115, 104, 101, 108, 108, 0, 1, 115, 116, 101, 115, 116, 0, 0, 84, 117,
  110, 113, 117, 111, 116, 101, 100, 0, 0, 117, 97, 108, 116, 101, 114, 110, 97, 116, 105, 118,
  101, 0, 0, 97, 110, 97, 109, 101, 0, 1, 110, 0,
];
#[no_mangle]
pub unsafe extern "C" fn getopt_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut n: libc::c_int = 0; /* used as yes/no flag */
  let mut optstr: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut name: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut opt: libc::c_uint = 0;
  let mut compatible: *const libc::c_char = 0 as *const libc::c_char;
  let mut s_arg: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut long_options: *mut option = 0 as *mut option;
  let mut l_arg: *mut llist_t = 0 as *mut llist_t;
  compatible = getenv(b"GETOPT_COMPATIBLE\x00" as *const u8 as *const libc::c_char);
  if (*argv.offset(1)).is_null() {
    if !compatible.is_null() {
      /* For some reason, the original getopt gave no error
       * when there were no arguments. */
      puts(b" --\x00" as *const u8 as *const libc::c_char); /* quoting off */
      return 0i32;
    }
    bb_simple_error_msg_and_die(
      b"missing optstring argument\x00" as *const u8 as *const libc::c_char,
    );
  }
  if *(*argv.offset(1)).offset(0) as libc::c_int != '-' as i32 || !compatible.is_null() {
    let mut s: *mut libc::c_char = *argv.offset(1);
    option_mask32 |= OPT_u as libc::c_int as libc::c_uint;
    s = xstrdup(s.offset(strspn(s, b"-+\x00" as *const u8 as *const libc::c_char) as isize));
    let ref mut fresh20 = *argv.offset(1);
    *fresh20 = *argv.offset(0);
    return generate_output(argv.offset(1), argc - 1i32, s, long_options);
  }
  opt = getopt32long(
    argv,
    b"+o:n:qQs:Tual:*\x00" as *const u8 as *const libc::c_char,
    getopt_longopts.as_ptr(),
    &mut optstr as *mut *mut libc::c_char,
    &mut name as *mut *mut libc::c_char,
    &mut s_arg as *mut *mut libc::c_char,
    &mut l_arg as *mut *mut llist_t,
  );
  /* Effectuate the read options for the applet itself */
  while !l_arg.is_null() {
    long_options = add_long_options(long_options, llist_pop(&mut l_arg) as *mut libc::c_char)
  }
  if opt & OPT_s as libc::c_int as libc::c_uint != 0 {
    set_shell(s_arg);
  }
  if opt & OPT_T as libc::c_int as libc::c_uint != 0 {
    return 4i32;
  }
  /* All options controlling the applet have now been parsed */
  n = optind - 1i32;
  if optstr.is_null() {
    n += 1;
    optstr = *argv.offset(n as isize);
    if optstr.is_null() {
      bb_simple_error_msg_and_die(
        b"missing optstring argument\x00" as *const u8 as *const libc::c_char,
      );
    }
  }
  let ref mut fresh21 = *argv.offset(n as isize);
  *fresh21 = if !name.is_null() {
    name
  } else {
    *argv.offset(0)
  };
  return generate_output(argv.offset(n as isize), argc - n, optstr, long_options);
}

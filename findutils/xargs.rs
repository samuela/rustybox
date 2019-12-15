use crate::libbb::ptr_to_globals::bb_errno;
use crate::librb::size_t;
use libc;
use libc::fclose;
use libc::fprintf;
use libc::free;
use libc::pid_t;
use libc::strcmp;
use libc::FILE;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut stderr: *mut FILE;

  #[no_mangle]
  fn getchar_unlocked() -> libc::c_int;
  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn overlapping_strcpy(dst: *mut libc::c_char, src: *const libc::c_char);
  #[no_mangle]
  fn count_strstr(str: *const libc::c_char, sub: *const libc::c_char) -> libc::c_uint;
  #[no_mangle]
  fn xmalloc_substitute_string(
    src: *const libc::c_char,
    count: libc::c_int,
    sub: *const libc::c_char,
    repl: *const libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_putchar_stderr(ch: libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn bb_arg_max() -> libc::c_uint;
  #[no_mangle]
  fn xfopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn xatou_range(str: *const libc::c_char, l: libc::c_uint, u: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn spawn(argv: *mut *mut libc::c_char) -> pid_t;
  #[no_mangle]
  fn safe_waitpid(pid: pid_t, wstat: *mut libc::c_int, options: libc::c_int) -> pid_t;
  #[no_mangle]
  fn wait_any_nohang(wstat: *mut libc::c_int) -> pid_t;
  #[no_mangle]
  fn spawn_and_wait(argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> u32;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_ask_y_confirmation_FILE(fp: *mut FILE) -> libc::c_int;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

/*
 * Copyright 2006, Bernhard Reutner-Fischer
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Convenience macros to test the version of gcc. */
/* __restrict is known in EGCS 1.2 and above. */
/* "The malloc attribute is used to tell the compiler that a function
 * may be treated as if any non-NULL pointer it returns cannot alias
 * any other pointer valid when the function returns. This will often
 * improve optimization. Standard functions with this property include
 * malloc and calloc. realloc-like functions have this property as long
 * as the old pointer is never referred to (including comparing it
 * to the new pointer) after the function returns a non-NULL value."
 */
/* __NO_INLINE__: some gcc's do not honor inlining! :( */
/* I've seen a toolchain where I needed __noinline__ instead of noinline */
/* used by unit test machinery to run registration functions before calling main() */
/* -fwhole-program makes all symbols local. The attribute externally_visible
 * forces a symbol global.  */
//__attribute__ ((__externally_visible__))
/* At 4.4 gcc become much more anal about this, need to use "aliased" types */
/* We use __extension__ in some places to suppress -pedantic warnings
 * about GCC extensions.  This feature didn't work properly before
 * gcc 2.8.  */
/* FAST_FUNC is a qualifier which (possibly) makes function call faster
 * and/or smaller by using modified ABI. It is usually only needed
 * on non-static, busybox internal functions. Recent versions of gcc
 * optimize statics automatically. FAST_FUNC on static is required
 * only if you need to match a function pointer's type.
 * FAST_FUNC may not work well with -flto so allow user to disable this.
 * (-DFAST_FUNC= )
 */
/* Make all declarations hidden (-fvisibility flag only affects definitions) */
/* (don't include system headers after this until corresponding pop!) */
/* gcc-2.95 had no va_copy but only __va_copy. */
/* ---- Endian Detection ------------------------------------ */
/* SWAP_LEnn means "convert CPU<->little_endian by swapping bytes" */
/* ---- Unaligned access ------------------------------------ */
/* NB: unaligned parameter should be a pointer, aligned one -
 * a lvalue. This makes it more likely to not swap them by mistake
 */
/* #elif ... - add your favorite arch today! */
/* Unaligned, fixed-endian accessors */
/* unxz needs an aligned fixed-endian accessor.
 * (however, the compiler does not realize it's aligned, the cast is still necessary)
 */
/* ---- Size-saving "small" ints (arch-dependent) ----------- */
/* add other arches which benefit from this... */
pub type smalluint = libc::c_uchar;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub args: *mut *mut libc::c_char,
  pub argv: *mut *mut libc::c_char,
  pub repl_str: *const libc::c_char,
  pub eol_ch: libc::c_char,
  pub eof_str: *const libc::c_char,
  pub idx: libc::c_int,
  pub running_procs: libc::c_int,
  pub max_procs: libc::c_int,
  pub xargs_exitcode: smalluint,
}
//usage:#define xargs_trivial_usage
//usage:       "[OPTIONS] [PROG ARGS]"
//usage:#define xargs_full_usage "\n\n"
//usage:       "Run PROG on every item given by stdin\n"
//usage:	IF_FEATURE_XARGS_SUPPORT_ZERO_TERM(
//usage:     "\n	-0	Input is separated by NULs"
//usage:	)
//usage:	IF_FEATURE_XARGS_SUPPORT_ARGS_FILE(
//usage:     "\n	-a FILE	Read from FILE instead of stdin"
//usage:	)
//usage:     "\n	-r	Don't run command if input is empty"
//usage:     "\n	-t	Print the command on stderr before execution"
//usage:	IF_FEATURE_XARGS_SUPPORT_CONFIRMATION(
//usage:     "\n	-p	Ask user whether to run each command"
//usage:	)
//usage:     "\n	-E STR,-e[STR]	STR stops input processing"
//usage:	IF_FEATURE_XARGS_SUPPORT_REPL_STR(
//usage:     "\n	-I STR	Replace STR within PROG ARGS with input line"
//usage:	)
//usage:     "\n	-n N	Pass no more than N args to PROG"
//usage:     "\n	-s N	Pass command line of no more than N bytes"
//usage:	IF_FEATURE_XARGS_SUPPORT_PARALLEL(
//usage:     "\n	-P N	Run up to N PROGs in parallel"
//usage:	)
//usage:	IF_FEATURE_XARGS_SUPPORT_TERMOPT(
//usage:     "\n	-x	Exit if size is exceeded"
//usage:	)
//usage:#define xargs_example_usage
//usage:       "$ ls | xargs gzip\n"
//usage:       "$ find . -name '*.c' -print | xargs rm\n"
/* Correct regardless of combination of CONFIG_xxx */
pub const OPT_REPLSTR1: libc::c_uint = 1024;
pub const OPT_REPLSTR: libc::c_uint = 512;
pub const OPT_ZEROTERM: libc::c_uint = 256;
pub const OPT_TERMINATE: libc::c_uint = 128;
/* SUS: -E<param> */
pub const OPT_INTERACTIVE: libc::c_uint = 64;
/* GNU: -e[<param>] */
pub const OPT_EOF_STRING1: libc::c_uint = 32;
pub const OPT_EOF_STRING: libc::c_uint = 16;
pub const OPT_UPTO_SIZE: libc::c_uint = 8;
pub const OPT_UPTO_NUMBER: libc::c_uint = 4;
pub const OPT_NO_EMPTY: libc::c_uint = 2;
pub const OPT_VERBOSE: libc::c_uint = 1;
pub const OPTBIT_REPLSTR1: libc::c_uint = 10;
pub const OPTBIT_REPLSTR: libc::c_uint = 9;
pub const OPTBIT_ZEROTERM: libc::c_uint = 8;
pub const OPTBIT_TERMINATE: libc::c_uint = 7;
pub const OPTBIT_INTERACTIVE: libc::c_uint = 6;
pub const OPTBIT_EOF_STRING1: libc::c_uint = 5;
pub const OPTBIT_EOF_STRING: libc::c_uint = 4;
pub const OPTBIT_UPTO_SIZE: libc::c_uint = 3;
pub const OPTBIT_UPTO_NUMBER: libc::c_uint = 2;
pub const OPTBIT_NO_EMPTY: libc::c_uint = 1;
pub const OPTBIT_VERBOSE: libc::c_uint = 0;
/* need to clear by hand because we are NOEXEC applet */
/*
 * Returns 0 if xargs should continue (but may set G.xargs_exitcode to 123).
 * Else sets G.xargs_exitcode to error code and returns nonzero.
 *
 * If G.max_procs == 0, performs final waitpid() loop for all children.
 */
unsafe extern "C" fn xargs_exec() -> libc::c_int {
  let mut status: libc::c_int = 0;
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_procs == 1i32 {
    status = spawn_and_wait((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args)
  } else {
    let mut pid: pid_t = 0;
    let mut wstat: libc::c_int = 0;
    loop {
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).running_procs
        >= (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_procs
      {
        pid = safe_waitpid(-1i32, &mut wstat, 0i32)
      } else {
        pid = wait_any_nohang(&mut wstat)
      }
      if pid > 0i32 {
        /* We may have children we don't know about:
         * sh -c 'sleep 1 & exec xargs ...'
         * Do not make G.running_procs go negative.
         */
        if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).running_procs != 0i32 {
          let ref mut fresh0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).running_procs;
          *fresh0 -= 1
        }
        status = if ((wstat & 0x7fi32) + 1i32) as libc::c_schar as libc::c_int >> 1i32 > 0i32 {
          (0x180i32) + (wstat & 0x7fi32)
        } else {
          (wstat & 0xff00i32) >> 8i32
        };
        if status > 0i32 && status < 255i32 {
          /* maybe we have more children? */
          /* else: "bad" status, will bail out */
          /* See below why 123 does not abort */
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).xargs_exitcode = 123i32 as smalluint;
          status = 0i32
        }
        if !(status == 0i32) {
          break;
        }
      } else {
        if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_procs != 0i32 {
          /* Not in final waitpid() loop,
           * and G.running_procs < G.max_procs: start more procs
           */
          status = spawn((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args);
          /* else: status == -1 (failed to fork or exec) */
          if status > 0i32 {
            let ref mut fresh1 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).running_procs;
            *fresh1 += 1;
            status = 0i32
          }
        } else {
          /* here "status" actually holds pid, or -1 */
          /* final waitpid() loop: must be ECHILD "no more children" */
          status = 0i32
        }
        break;
      }
    }
  }
  /* Manpage:
   * """xargs exits with the following status:
   * 0 if it succeeds
   * 123 if any invocation of the command exited with status 1-125
   * 124 if the command exited with status 255
   *     ("""If any invocation of the command exits with a status of 255,
   *     xargs will stop immediately without reading any further input.
   *     An error message is issued on stderr when this happens.""")
   * 125 if the command is killed by a signal
   * 126 if the command cannot be run
   * 127 if the command is not found
   * 1 if some other error occurred."""
   */
  if status < 0i32 {
    bb_simple_perror_msg(
      *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .args
        .offset(0),
    );
    status = if *bb_errno == 2i32 { 127i32 } else { 126i32 }
  } else if status >= 0x180i32 {
    bb_error_msg(
      b"\'%s\' terminated by signal %u\x00" as *const u8 as *const libc::c_char,
      *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .args
        .offset(0),
      status - 0x180i32,
    );
    status = 125i32
  } else if status != 0i32 {
    if status == 255i32 {
      bb_error_msg(
        b"%s: exited with status 255; aborting\x00" as *const u8 as *const libc::c_char,
        *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .args
          .offset(0),
      );
      status = 124i32
    } else {
      /* "123 if any invocation of the command exited with status 1-125"
       * This implies that nonzero exit code is remembered,
       * but does not cause xargs to stop: we return 0.
       */
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).xargs_exitcode = 123i32 as smalluint;
      status = 0i32
    }
  }
  if status != 0i32 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).xargs_exitcode = status as smalluint
  }
  return status;
}
/* In POSIX/C locale isspace is only these chars: "\t\n\v\f\r" and space.
 * "\t\n\v\f\r" happen to have ASCII codes 9,10,11,12,13.
 */
unsafe extern "C" fn store_param(mut s: *mut libc::c_char) {
  /* Grow by 256 elements at once */
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).idx & 0xffi32 == 0 {
    /* G.idx == N*256? */
    /* Enlarge, make G.args[(N+1)*256 - 1] last valid idx */
    let ref mut fresh2 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args;
    *fresh2 = xrealloc(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args as *mut libc::c_void,
      (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong).wrapping_mul(
        ((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).idx + 0x100i32) as libc::c_ulong,
      ),
    ) as *mut *mut libc::c_char
  }
  let ref mut fresh3 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).idx;
  let fresh4 = *fresh3;
  *fresh3 = *fresh3 + 1;
  let ref mut fresh5 = *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .args
    .offset(fresh4 as isize);
  *fresh5 = s;
}
/* process[0]_stdin:
 * Read characters into buf[n_max_chars+1], and when parameter delimiter
 * is seen, store the address of a new parameter to args[].
 * If reading discovers that last chars do not form the complete
 * parameter, the pointer to the first such "tail character" is returned.
 * (buf has extra byte at the end to accommodate terminating NUL
 * of "tail characters" string).
 * Otherwise, the returned pointer points to NUL byte.
 * On entry, buf[] may contain some "seed chars" which are to become
 * the beginning of the first parameter.
 */
unsafe extern "C" fn process_stdin(
  mut n_max_chars: libc::c_int,
  mut n_max_arg: libc::c_int,
  mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
  let mut current_block: u64; /* quote char */
  let mut q: libc::c_char = '\u{0}' as i32 as libc::c_char; /* start of the word */
  let mut state: libc::c_char = 0i32 as libc::c_char; /* end of the word */
  let mut s: *mut libc::c_char = buf; /* past buffer's end */
  let mut p: *mut libc::c_char = s.offset(strlen(buf) as isize);
  buf = buf.offset(n_max_chars as isize);
  loop
  /* "goto ret" is used instead of "break" to make control flow
   * more obvious: */
  {
    let mut c: libc::c_int = getchar_unlocked(); /* if (state == NORM) */
    if c == -1i32 {
      if !(p != s) {
        break;
      }
      current_block = 9201467417380152874;
    } else if state as libc::c_int == 2i32 {
      state = 0i32 as libc::c_char;
      current_block = 6025286053310698807;
    } else if state as libc::c_int == 1i32 {
      if c != q as libc::c_int {
        current_block = 6025286053310698807;
      } else {
        q = '\u{0}' as i32 as libc::c_char;
        state = 0i32 as libc::c_char;
        current_block = 4488286894823169796;
      }
    } else if ({
      let mut xargs__isspace: libc::c_uchar = (c - 9i32) as libc::c_uchar;
      (xargs__isspace as libc::c_int == ' ' as i32 - 9i32
        || xargs__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
    }) != 0
    {
      if p != s {
        current_block = 9201467417380152874;
      } else {
        current_block = 4488286894823169796;
      }
    } else if c == '\\' as i32 {
      state = 2i32 as libc::c_char;
      current_block = 4488286894823169796;
    } else if c == '\'' as i32 || c == '\"' as i32 {
      q = c as libc::c_char;
      state = 1i32 as libc::c_char;
      current_block = 4488286894823169796;
    } else {
      current_block = 6025286053310698807;
    }
    match current_block {
      9201467417380152874 => {
        state = 4i32 as libc::c_char;
        c = '\u{0}' as i32;
        current_block = 6025286053310698807;
      }
      _ => {}
    }
    match current_block {
      6025286053310698807 => {
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = c as libc::c_char
      }
      _ => {}
    }
    if state as libc::c_int == 4i32 {
      /* word's delimiter or EOF detected */
      if q != 0 {
        bb_error_msg_and_die(
          b"unmatched %s quote\x00" as *const u8 as *const libc::c_char,
          if q as libc::c_int == '\'' as i32 {
            b"single\x00" as *const u8 as *const libc::c_char
          } else {
            b"double\x00" as *const u8 as *const libc::c_char
          },
        );
      }
      /* A full word is loaded */
      if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .eof_str
        .is_null()
      {
        if strcmp(
          s,
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).eof_str,
        ) == 0i32
        {
          while getchar_unlocked() != -1i32 {}
          p = s;
          break;
        }
      }
      store_param(s);
      s = p;
      n_max_arg -= 1;
      if n_max_arg == 0i32 {
        break;
      }
      state = 0i32 as libc::c_char
    }
    if p == buf {
      break;
    }
  }
  *p = '\u{0}' as i32 as libc::c_char;
  /* store_param(NULL) - caller will do it */
  return s;
}
/* FEATURE_XARGS_SUPPORT_QUOTES */
unsafe extern "C" fn process0_stdin(
  mut n_max_chars: libc::c_int,
  mut n_max_arg: libc::c_int,
  mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
  let mut s: *mut libc::c_char = buf; /* start of the word */
  let mut p: *mut libc::c_char = s.offset(strlen(buf) as isize); /* end of the word */
  buf = buf.offset(n_max_chars as isize); /* past buffer's end */
  loop {
    let mut c: libc::c_int = getchar_unlocked();
    if c == -1i32 {
      if p == s {
        break;
      }
      c = '\u{0}' as i32
    }
    let fresh7 = p;
    p = p.offset(1);
    *fresh7 = c as libc::c_char;
    if c == '\u{0}' as i32 {
      /* NUL or EOF detected */
      /* A full word is loaded */
      store_param(s);
      s = p;
      n_max_arg -= 1;
      if n_max_arg == 0i32 {
        break;
      }
    }
    if p == buf {
      break;
    }
  }
  *p = '\u{0}' as i32 as libc::c_char;
  /* store_param(NULL) - caller will do it */
  return s;
}
/* FEATURE_XARGS_SUPPORT_ZERO_TERM */
/*
 * Used if -I<repl> was specified.
 * In this mode, words aren't appended to PROG ARGS.
 * Instead, entire input line is read, then <repl> string
 * in every PROG and ARG is replaced with the line:
 *  echo -e "ho ho\nhi" | xargs -I_ cmd __ _
 * results in "cmd 'ho hoho ho' 'ho ho'"; "cmd 'hihi' 'hi'".
 * -n MAX_ARGS seems to be ignored.
 * Tested with GNU findutils 4.5.10.
 */
//FIXME: n_max_chars is not handled the same way as in GNU findutils.
//FIXME: quoting is not implemented.
unsafe extern "C" fn process_stdin_with_replace(
  mut n_max_chars: libc::c_int,
  mut _n_max_arg: libc::c_int,
  mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
  let mut i: libc::c_int = 0;
  let mut end: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  /* Free strings from last invocation, if any */
  i = 0i32; /* empty line */
  while !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .args
    .is_null()
    && !(*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .args
      .offset(i as isize))
    .is_null()
  {
    if *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .args
      .offset(i as isize)
      != *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .argv
        .offset(i as isize)
    {
      free(
        *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .args
          .offset(i as isize) as *mut libc::c_void,
      );
    }
    i += 1
  }
  end = buf.offset(n_max_chars as isize);
  p = buf;
  loop {
    let mut c: libc::c_int = getchar_unlocked();
    if c == -1i32 || c == (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).eol_ch as libc::c_int
    {
      if p == buf {
        break;
      }
      c = '\u{0}' as i32
    }
    let fresh8 = p;
    p = p.offset(1);
    *fresh8 = c as libc::c_char;
    if c == '\u{0}' as i32 {
      /* EOL or EOF detected */
      i = 0i32;
      while !(*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .argv
        .offset(i as isize))
      .is_null()
      {
        let mut arg: *mut libc::c_char = *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .argv
          .offset(i as isize);
        let mut count: libc::c_int = count_strstr(
          arg,
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).repl_str,
        ) as libc::c_int;
        if count != 0i32 {
          arg = xmalloc_substitute_string(
            arg,
            count,
            (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).repl_str,
            buf,
          )
        }
        store_param(arg);
        i += 1
      }
      p = buf;
      break;
    } else if p == end {
      break;
    }
  }
  *p = '\u{0}' as i32 as libc::c_char;
  /* store_param(NULL) - caller will do it */
  return buf;
}
/* Prompt the user for a response, and
 * if user responds affirmatively, return true;
 * otherwise, return false. Uses "/dev/tty", not stdin.
 */
unsafe extern "C" fn xargs_ask_confirmation() -> libc::c_int {
  let mut tty_stream: *mut FILE = std::ptr::null_mut(); /* let's not go crazy high */
  let mut r: libc::c_int = 0;
  tty_stream = xfopen_for_read(b"/dev/tty\x00" as *const u8 as *const libc::c_char);
  fputs_unlocked(b" ?...\x00" as *const u8 as *const libc::c_char, stderr);
  r = bb_ask_y_confirmation_FILE(tty_stream);
  fclose(tty_stream);
  return r;
}
#[no_mangle]
pub unsafe extern "C" fn xargs_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut initial_idx: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  let mut max_args: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut max_chars: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut buf: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut opt: libc::c_uint = 0;
  let mut n_max_chars: libc::c_int = 0;
  let mut n_max_arg: libc::c_int = 0;
  let mut read_args: Option<
    unsafe extern "C" fn(_: libc::c_int, _: libc::c_int, _: *mut libc::c_char) -> *mut libc::c_char,
  > = Some(
    process_stdin
      as unsafe extern "C" fn(
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_char,
      ) -> *mut libc::c_char,
  );
  let mut opt_a: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let ref mut fresh9 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).eof_str;
  *fresh9 = 0 as *const libc::c_char;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).idx = 0i32;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).running_procs = 0i32;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_procs = 1i32;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).xargs_exitcode = 0i32 as smalluint;
  let ref mut fresh10 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).repl_str;
  *fresh10 = b"{}\x00" as *const u8 as *const libc::c_char;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).eol_ch = '\n' as i32 as libc::c_char;
  opt = getopt32long(
    argv,
    b"+trn:s:e::E:px0I:i::P:+a:\x00" as *const u8 as *const libc::c_char,
    b"no-run-if-empty\x00\x00r\x00" as *const u8 as *const libc::c_char,
    &mut max_args as *mut *mut libc::c_char,
    &mut max_chars as *mut *mut libc::c_char,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).eof_str as *mut *const libc::c_char,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).eof_str as *mut *const libc::c_char,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).repl_str as *mut *const libc::c_char,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).repl_str as *mut *const libc::c_char,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_procs as *mut libc::c_int,
    &mut opt_a as *mut *mut libc::c_char,
  );
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_procs <= 0i32 {
    /* -P0 means "run lots of them" */
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_procs = 100i32
  }
  if !opt_a.is_null() {
    xmove_fd(xopen(opt_a, 0i32), 0i32);
  }
  /* -E ""? You may wonder why not just omit -E?
   * This is used for portability:
   * old xargs was using "_" as default for -E / -e */
  if opt & OPT_EOF_STRING1 as libc::c_int as libc::c_uint != 0
    && *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .eof_str
      .offset(0) as libc::c_int
      == '\u{0}' as i32
  {
    let ref mut fresh11 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).eof_str;
    *fresh11 = 0 as *const libc::c_char
  }
  if opt & OPT_ZEROTERM as libc::c_int as libc::c_uint != 0 {
    read_args = Some(
      process0_stdin
        as unsafe extern "C" fn(
          _: libc::c_int,
          _: libc::c_int,
          _: *mut libc::c_char,
        ) -> *mut libc::c_char,
    );
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).eol_ch = '\u{0}' as i32 as libc::c_char
  }
  argv = argv.offset(optind as isize);
  //argc -= optind;
  if (*argv.offset(0)).is_null() {
    /* default behavior is to echo all the filenames */
    argv = argv.offset(-1);
    *argv = b"echo\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    //argc++;
  }
  /*
   * The Open Group Base Specifications Issue 6:
   * "The xargs utility shall limit the command line length such that
   * when the command line is invoked, the combined argument
   * and environment lists (see the exec family of functions
   * in the System Interfaces volume of IEEE Std 1003.1-2001)
   * shall not exceed {ARG_MAX}-2048 bytes".
   */
  n_max_chars = bb_arg_max() as libc::c_int;
  if n_max_chars > 32i32 * 1024i32 {
    n_max_chars = 32i32 * 1024i32
  }
  /*
   * POSIX suggests substracting 2048 bytes from sysconf(_SC_ARG_MAX)
   * so that the process may safely modify its environment.
   */
  n_max_chars -= 2048i32;
  if opt & OPT_UPTO_SIZE as libc::c_int as libc::c_uint != 0 {
    n_max_chars = xatou_range(
      max_chars,
      1i32 as libc::c_uint,
      2147483647i32 as libc::c_uint,
    ) as libc::c_int
  }
  /* Account for prepended fixed arguments */
  let mut n_chars: size_t = 0i32 as size_t;
  i = 0i32;
  while !(*argv.offset(i as isize)).is_null() {
    n_chars = (n_chars as libc::c_ulong)
      .wrapping_add(strlen(*argv.offset(i as isize)).wrapping_add(1i32 as libc::c_ulong))
      as size_t as size_t;
    i += 1
  }
  n_max_chars = (n_max_chars as libc::c_ulong).wrapping_sub(n_chars) as libc::c_int as libc::c_int;
  /* Sanity check */
  if n_max_chars <= 0i32 {
    bb_simple_error_msg_and_die(
      b"can\'t fit single argument within argument list size limit\x00" as *const u8
        as *const libc::c_char,
    );
  }
  buf = xzalloc((n_max_chars + 1i32) as size_t) as *mut libc::c_char;
  n_max_arg = n_max_chars;
  if opt & OPT_UPTO_NUMBER as libc::c_int as libc::c_uint != 0 {
    n_max_arg = xatou_range(
      max_args,
      1i32 as libc::c_uint,
      2147483647i32 as libc::c_uint,
    ) as libc::c_int
    /* Not necessary, we use growable args[]: */
    /* if (n_max_arg > n_max_chars) n_max_arg = n_max_chars */
  }
  if opt & (OPT_REPLSTR as libc::c_int | OPT_REPLSTR1 as libc::c_int) as libc::c_uint != 0 {
    /*
     * -I<str>:
     * Unmodified args are kept in G.argv[i],
     * G.args[i] receives malloced G.argv[i] with <str> replaced
     * with input line. Setting this up:
     */
    let ref mut fresh12 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args;
    *fresh12 = std::ptr::null_mut();
    let ref mut fresh13 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).argv;
    *fresh13 = argv;
    read_args = Some(
      process_stdin_with_replace
        as unsafe extern "C" fn(
          _: libc::c_int,
          _: libc::c_int,
          _: *mut libc::c_char,
        ) -> *mut libc::c_char,
    );
    /* Make -I imply -r. GNU findutils seems to do the same: */
    /* (otherwise "echo -n | xargs -I% echo %" would SEGV) */
    opt |= OPT_NO_EMPTY as libc::c_int as libc::c_uint
  } else {
    /* Store the command to be executed, part 1.
     * We can statically allocate (argc + n_max_arg + 1) elements
     * and do not bother with resizing args[], but on 64-bit machines
     * this results in args[] vector which is ~8 times bigger
     * than n_max_chars! That is, with n_max_chars == 20k,
     * args[] will take 160k (!), which will most likely be
     * almost entirely unused.
     */
    i = 0i32; /* while */
    while !(*argv.offset(i as isize)).is_null() {
      store_param(*argv.offset(i as isize));
      i += 1
    }
  }
  initial_idx = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).idx;
  loop {
    let mut rem: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).idx = initial_idx;
    rem = read_args.expect("non-null function pointer")(n_max_chars, n_max_arg, buf);
    store_param(0 as *mut libc::c_char);
    if (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .args
      .offset(initial_idx as isize))
    .is_null()
    {
      /* not even one ARG was added? */
      if *rem as libc::c_int != '\u{0}' as i32 {
        bb_simple_error_msg_and_die(
          b"argument line too long\x00" as *const u8 as *const libc::c_char,
        );
      }
      if opt & OPT_NO_EMPTY as libc::c_int as libc::c_uint != 0 {
        break;
      }
    }
    opt |= OPT_NO_EMPTY as libc::c_int as libc::c_uint;
    if opt & (OPT_INTERACTIVE as libc::c_int | OPT_VERBOSE as libc::c_int) as libc::c_uint != 0 {
      let mut fmt: *const libc::c_char = (b" %s\x00" as *const u8 as *const libc::c_char).offset(1);
      let mut args: *mut *mut libc::c_char =
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args;
      i = 0i32;
      while !(*args.offset(i as isize)).is_null() {
        fprintf(stderr, fmt, *args.offset(i as isize));
        fmt = b" %s\x00" as *const u8 as *const libc::c_char;
        i += 1
      }
      if opt & OPT_INTERACTIVE as libc::c_int as libc::c_uint == 0 {
        bb_putchar_stderr('\n' as i32 as libc::c_char);
      }
    }
    if opt & OPT_INTERACTIVE as libc::c_int as libc::c_uint == 0 || xargs_ask_confirmation() != 0 {
      if xargs_exec() != 0i32 {
        break;
        /* G.xargs_exitcode is set by xargs_exec() */
      }
    } /* final waitpid() loop */
    overlapping_strcpy(buf, rem);
  }
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_procs = 0i32;
  xargs_exec();
  return (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).xargs_exitcode as libc::c_int;
}
/* TEST */

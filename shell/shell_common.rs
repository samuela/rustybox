use crate::libbb::ptr_to_globals::bb_errno;
use crate::librb::size_t;
use crate::librb::smallint;
use libc;
use libc::cc_t;
use libc::free;
use libc::getopt;
use libc::isatty;
use libc::pollfd;
use libc::printf;
use libc::puts;
use libc::ssize_t;
use libc::strchr;
use libc::strcmp;
use libc::termios;
use libc::FILE;
extern "C" {

  #[no_mangle]
  static mut optarg: *mut libc::c_char;
  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  static mut stderr: *mut FILE;

  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;

  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn getrlimit(__resource: __rlimit_resource_t, __rlimits: *mut rlimit) -> libc::c_int;
  #[no_mangle]
  fn setrlimit(__resource: __rlimit_resource_t, __rlimits: *const rlimit) -> libc::c_int;
  #[no_mangle]
  fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
  #[no_mangle]
  fn tcsetattr(
    __fd: libc::c_int,
    __optional_actions: libc::c_int,
    __termios_p: *const termios,
  ) -> libc::c_int;
/* Some useful definitions */
/* Macros for min/max.  */
/* buffer allocation schemes */
/* glibc uses __errno_location() to get a ptr to errno */
/* We can just memorize it once - no multithreading in busybox :) */

/* 0 if argv[0] is NULL: */

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

pub type __rlim64_t = libc::c_ulong;

pub type uintptr_t = libc::c_ulong;
pub type nfds_t = libc::c_ulong;
pub type __rlimit_resource = libc::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
pub type rlim_t = __rlim64_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rlimit {
  pub rlim_cur: rlim_t,
  pub rlim_max: rlim_t,
}
pub type __rlimit_resource_t = __rlimit_resource;

/* "OPTIND=1" */
/* Builtins */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct builtin_read_params {
  pub read_flags: libc::c_int,
  pub setvar: Option<unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> ()>,
  pub argv: *mut *mut libc::c_char,
  pub ifs: *const libc::c_char,
  pub opt_n: *const libc::c_char,
  pub opt_p: *const libc::c_char,
  pub opt_t: *const libc::c_char,
  pub opt_u: *const libc::c_char,
  pub opt_d: *const libc::c_char,
}
pub type C2RustUnnamed = libc::c_uint;
pub const BUILTIN_READ_RAW: C2RustUnnamed = 2;
pub const BUILTIN_READ_SILENT: C2RustUnnamed = 1;
/* ulimit builtin */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct limits {
  pub cmd: u8,
  pub factor_shift: u8,
  /* shift by to get rlim_{cur,max} values */
}
pub const OPT_soft: C2RustUnnamed_0 = 2;
pub const OPT_hard: C2RustUnnamed_0 = 1;
pub const OPT_all: C2RustUnnamed_0 = 4;
pub type C2RustUnnamed_0 = libc::c_uint;
#[inline(always)]
unsafe extern "C" fn bb_strtoul(
  mut arg: *const libc::c_char,
  mut endp: *mut *mut libc::c_char,
  mut base: libc::c_int,
) -> libc::c_ulong {
  return crate::libbb::bb_strtonum::bb_strtoull(arg, endp, base) as libc::c_ulong;
}

/*
 * Adapted from ash applet code
 *
 * This code is derived from software contributed to Berkeley by
 * Kenneth Almquist.
 *
 * Copyright (c) 1989, 1991, 1993, 1994
 *      The Regents of the University of California.  All rights reserved.
 *
 * Copyright (c) 1997-2005 Herbert Xu <herbert@gondor.apana.org.au>
 * was re-ported from NetBSD and debianized.
 *
 * Copyright (c) 2010 Denys Vlasenko
 * Split from ash.c
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub static mut defifsvar: [libc::c_char; 8] = [73, 70, 83, 61, 32, 9, 10, 0];
#[no_mangle]
pub static mut defoptindvar: [libc::c_char; 9] = [79, 80, 84, 73, 78, 68, 61, 49, 0];
//TODO? do not provide bashisms if not asked for:
//#if !ENABLE_HUSH_BASH_COMPAT && !ENABLE_ASH_BASH_COMPAT
//#define shell_builtin_read(setvar,argv,ifs,read_flags,n,p,t,u,d)
//	shell_builtin_read(setvar,argv,ifs,read_flags)
//#endif
/* read builtin */
/* Needs to be interruptible: shell must handle traps and shell-special signals
 * while inside read. To implement this, be sure to not loop on EINTR
 * and return errno == EINTR reliably.
 */
//TODO: use more efficient setvar() which takes a pointer to malloced "VAR=VAL"
//string. hush naturally has it, and ash has setvareq().
//Here we can simply store "VAR=" at buffer start and store read data directly
//after "=", then pass buffer to setvar() to consume.
#[no_mangle]
pub unsafe extern "C" fn shell_builtin_read(
  mut params: *mut builtin_read_params,
) -> *const libc::c_char {
  let mut current_block: u64;
  let mut pfd: [pollfd; 1] = [pollfd {
    fd: 0,
    events: 0,
    revents: 0,
  }; 1];
  /* -u FD */
  let mut err: libc::c_uint = 0; /* -t TIMEOUT */
  let mut end_ms: libc::c_uint = 0; /* -n NUM */
  let mut nchars: libc::c_int = 0; /* need to be able to hold -1 */
  let mut pp: *mut *mut libc::c_char = std::ptr::null_mut();
  let mut buffer: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut delim: libc::c_char = 0;
  let mut tty: termios = std::mem::zeroed();
  let mut old_tty: termios = std::mem::zeroed();
  let mut retval: *const libc::c_char = 0 as *const libc::c_char;
  let mut bufpos: libc::c_int = 0;
  let mut startword: libc::c_int = 0;
  let mut backslash: smallint = 0;
  let mut argv: *mut *mut libc::c_char = std::ptr::null_mut();
  let mut ifs: *const libc::c_char = 0 as *const libc::c_char;
  let mut read_flags: libc::c_int = 0;
  err = 0i32 as libc::c_uint;
  *bb_errno = err as libc::c_int;
  argv = (*params).argv;
  pp = argv;
  while !(*pp).is_null() {
    if *crate::libbb::endofname::endofname(*pp).offset(0) as libc::c_int != '\u{0}' as i32 {
      /* Mimic bash message */
      crate::libbb::verror_msg::bb_error_msg(
        b"read: \'%s\': not a valid identifier\x00" as *const u8 as *const libc::c_char,
        *pp,
      ); /* if != 0, -n is in effect */
      return 1i32 as uintptr_t as *const libc::c_char;
    }
    pp = pp.offset(1)
  }
  nchars = 0i32;
  if !(*params).opt_n.is_null() {
    nchars =
      crate::libbb::bb_strtonum::bb_strtou((*params).opt_n, 0 as *mut *mut libc::c_char, 10i32)
        as libc::c_int;
    if nchars < 0i32 || *bb_errno != 0 {
      return b"invalid count\x00" as *const u8 as *const libc::c_char;
    }
    /* note: "-n 0": off (bash 3.2 does this too) */
  }
  end_ms = 0i32 as libc::c_uint;
  if !(*params).opt_t.is_null() && 1i32 == 0 {
    end_ms =
      crate::libbb::bb_strtonum::bb_strtou((*params).opt_t, 0 as *mut *mut libc::c_char, 10i32);
    if *bb_errno != 0 {
      return b"invalid timeout\x00" as *const u8 as *const libc::c_char;
    }
    if end_ms
      > (2147483647i32 as libc::c_uint)
        .wrapping_mul(2u32)
        .wrapping_add(1u32)
        .wrapping_div(2048i32 as libc::c_uint)
    {
      /* be safely away from overflow */
      end_ms = (2147483647i32 as libc::c_uint)
        .wrapping_mul(2u32)
        .wrapping_add(1u32)
        .wrapping_div(2048i32 as libc::c_uint)
    }
    end_ms = end_ms.wrapping_mul(1000i32 as libc::c_uint)
  }
  if !(*params).opt_t.is_null() && 1i32 != 0 {
    /* bash 4.3 (maybe earlier) supports -t N.NNNNNN */
    let mut p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    /* Eat up to three fractional digits */
    let mut frac_digits: libc::c_int = 3i32 + 1i32;
    end_ms = crate::libbb::bb_strtonum::bb_strtou((*params).opt_t, &mut p, 10i32);
    if end_ms
      > (2147483647i32 as libc::c_uint)
        .wrapping_mul(2u32)
        .wrapping_add(1u32)
        .wrapping_div(2048i32 as libc::c_uint)
    {
      /* be safely away from overflow */
      end_ms = (2147483647i32 as libc::c_uint)
        .wrapping_mul(2u32)
        .wrapping_add(1u32)
        .wrapping_div(2048i32 as libc::c_uint)
    }
    if *bb_errno != 0 {
      /* EINVAL = number is ok, but not NUL terminated */
      if *bb_errno != 22i32 || *p as libc::c_int != '.' as i32 {
        return b"invalid timeout\x00" as *const u8 as *const libc::c_char;
      }
      loop
      /* Do not check the rest: bash allows "0.123456xyz" */
      {
        p = p.offset(1);
        if !(*p as libc::c_int != 0 && {
          frac_digits -= 1;
          (frac_digits) != 0
        }) {
          break;
        }
        end_ms = end_ms.wrapping_mul(10i32 as libc::c_uint);
        end_ms = end_ms.wrapping_add((*p as libc::c_int - '0' as i32) as libc::c_uint);
        if (*p as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int > 9i32 {
          return b"invalid timeout\x00" as *const u8 as *const libc::c_char;
        }
      }
    }
    loop {
      frac_digits -= 1;
      if !(frac_digits > 0i32) {
        break;
      }
      end_ms = end_ms.wrapping_mul(10i32 as libc::c_uint)
    }
  }
  pfd[0].fd = 0i32;
  if !(*params).opt_u.is_null() {
    pfd[0].fd =
      crate::libbb::bb_strtonum::bb_strtou((*params).opt_u, 0 as *mut *mut libc::c_char, 10i32)
        as libc::c_int;
    if pfd[0].fd < 0i32 || *bb_errno != 0 {
      return b"invalid file descriptor\x00" as *const u8 as *const libc::c_char;
    }
  }
  if !(*params).opt_t.is_null() && end_ms == 0i32 as libc::c_uint {
    /* "If timeout is 0, read returns immediately, without trying
     * to read any data. The exit status is 0 if input is available
     * on the specified file descriptor, non-zero otherwise."
     * bash seems to ignore -p PROMPT for this use case.
     */
    let mut r: libc::c_int = 0;
    pfd[0].events = 0x1i32 as libc::c_short;
    r = poll(pfd.as_mut_ptr(), 1i32 as nfds_t, 0i32);
    /* Return 0 only if poll returns 1 ("one fd ready"), else return 1: */
    return (r <= 0i32) as libc::c_int as uintptr_t as *const libc::c_char;
  }
  if !(*params).opt_p.is_null() && isatty(pfd[0].fd) != 0 {
    fputs_unlocked((*params).opt_p, stderr);
    crate::libbb::xfuncs_printf::fflush_all();
  }
  ifs = (*params).ifs;
  if ifs.is_null() {
    ifs = defifsvar.as_ptr().offset(4)
  }
  read_flags = (*params).read_flags;
  if nchars != 0 || read_flags & BUILTIN_READ_SILENT as libc::c_int != 0 {
    tcgetattr(pfd[0].fd, &mut tty);
    old_tty = tty;
    if nchars != 0 {
      tty.c_lflag &= !0o2i32 as libc::c_uint;
      // Setting it to more than 1 breaks poll():
      // it blocks even if there's data. !??
      //tty.c_cc[VMIN] = nchars < 256 ? nchars : 255;
      /* reads will block only if < 1 char is available */
      tty.c_cc[6] = 1i32 as cc_t;
      /* no timeout (reads block forever) */
      tty.c_cc[5] = 0i32 as cc_t
    }
    if read_flags & BUILTIN_READ_SILENT as libc::c_int != 0 {
      tty.c_lflag &= !(0o10i32 | 0o40i32 | 0o100i32) as libc::c_uint
    }
    /* This forces execution of "restoring" tcgetattr later */
    read_flags |= BUILTIN_READ_SILENT as libc::c_int;
    /* if tcgetattr failed, tcsetattr will fail too.
     * Ignoring, it's harmless. */
    tcsetattr(pfd[0].fd, 0i32, &mut tty);
  }
  retval = 0 as *const libc::c_char;
  startword = 1i32;
  backslash = 0i32 as smallint;
  if !(*params).opt_t.is_null() {
    end_ms = end_ms.wrapping_add(crate::libbb::time::monotonic_ms() as libc::c_uint)
  }
  buffer = std::ptr::null_mut::<libc::c_char>();
  bufpos = 0i32;
  delim = if !(*params).opt_d.is_null() {
    *(*params).opt_d.offset(0) as libc::c_int
  } else {
    '\n' as i32
  } as libc::c_char;
  loop {
    let mut c: libc::c_char = 0;
    let mut timeout: libc::c_int = 0;
    if bufpos & 0xffi32 == 0i32 {
      buffer = crate::libbb::xfuncs_printf::xrealloc(
        buffer as *mut libc::c_void,
        (bufpos + 0x101i32) as size_t,
      ) as *mut libc::c_char
    }
    timeout = -1i32;
    if !(*params).opt_t.is_null() {
      timeout =
        end_ms.wrapping_sub(crate::libbb::time::monotonic_ms() as libc::c_uint) as libc::c_int;
      /* ^^^^^^^^^^^^^ all values are unsigned,
       * wrapping math is used here, good even if
       * 32-bit unix time wrapped (year 2038+).
       */
      if timeout <= 0i32 {
        /* already late? */
        retval = 1i32 as uintptr_t as *const libc::c_char;
        current_block = 2968968806559280500;
        break;
      }
    }
    /* We must poll even if timeout is -1:
     * we want to be interrupted if signal arrives,
     * regardless of SA_RESTART-ness of that signal!
     */
    *bb_errno = 0i32;
    pfd[0].events = 0x1i32 as libc::c_short;
    if poll(pfd.as_mut_ptr(), 1i32 as nfds_t, timeout) <= 0i32 {
      /* timed out, or EINTR */
      err = *bb_errno as libc::c_uint;
      retval = 1i32 as uintptr_t as *const libc::c_char;
      current_block = 2968968806559280500;
      break;
    } else if read(
      pfd[0].fd,
      &mut *buffer.offset(bufpos as isize) as *mut libc::c_char as *mut libc::c_void,
      1i32 as size_t,
    ) != 1
    {
      err = *bb_errno as libc::c_uint;
      retval = 1i32 as uintptr_t as *const libc::c_char;
      current_block = 13256895345714485905;
      break;
    } else {
      c = *buffer.offset(bufpos as isize);
      if !(c as libc::c_int == '\u{0}' as i32) {
        if read_flags & BUILTIN_READ_RAW as libc::c_int == 0 {
          if backslash != 0 {
            backslash = 0i32 as smallint;
            if c as libc::c_int != '\n' as i32 {
              current_block = 17751066025094275769;
            } else {
              current_block = 12027283704867122503;
            }
          } else if c as libc::c_int == '\\' as i32 {
            backslash = 1i32 as smallint;
            current_block = 12027283704867122503;
          } else {
            current_block = 14913924298693586572;
          }
        } else {
          current_block = 14913924298693586572;
        }
        match current_block {
          12027283704867122503 => {}
          _ => {
            match current_block {
              14913924298693586572 => {
                if c as libc::c_int == delim as libc::c_int {
                  current_block = 13256895345714485905;
                  break;
                }
                /* $IFS splitting. NOT done if we run "read"
                 * without variable names (bash compat).
                 * Thus, "read" and "read REPLY" are not the same.
                 */
                if (*params).opt_d.is_null() && !(*argv.offset(0)).is_null() {
                  /* http://www.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#tag_18_06_05 */
                  let mut is_ifs: *const libc::c_char = strchr(ifs, c as libc::c_int);
                  if startword != 0 && !is_ifs.is_null() {
                    if ({
                      let mut bb__isspace: libc::c_uchar =
                        (c as libc::c_int - 9i32) as libc::c_uchar;
                      (bb__isspace as libc::c_int == ' ' as i32 - 9i32
                        || bb__isspace as libc::c_int <= 13i32 - 9i32)
                        as libc::c_int
                    }) != 0
                    {
                      current_block = 12027283704867122503;
                    } else {
                      /* it is a non-space ifs char */
                      startword -= 1;
                      if startword == 1i32 {
                        current_block = 12027283704867122503;
                      } else {
                        current_block = 14648249180243006330;
                      }
                    }
                  /* yes, it is not next word yet */
                  } else {
                    current_block = 14648249180243006330;
                  }
                  match current_block {
                    12027283704867122503 => {}
                    _ => {
                      startword = 0i32;
                      if !(*argv.offset(1)).is_null() && !is_ifs.is_null() {
                        *buffer.offset(bufpos as isize) = '\u{0}' as i32 as libc::c_char;
                        bufpos = 0i32;
                        (*params).setvar.expect("non-null function pointer")(*argv, buffer);
                        argv = argv.offset(1);
                        /* can we skip one non-space ifs char? (2: yes) */
                        startword = if ({
                          let mut bb__isspace: libc::c_uchar =
                            (c as libc::c_int - 9i32) as libc::c_uchar;
                          (bb__isspace as libc::c_int == ' ' as i32 - 9i32
                            || bb__isspace as libc::c_int <= 13i32 - 9i32)
                            as libc::c_int
                        }) != 0
                        {
                          2i32
                        } else {
                          1i32
                        };
                        current_block = 12027283704867122503;
                      } else {
                        current_block = 17751066025094275769;
                      }
                    }
                  }
                } else {
                  current_block = 17751066025094275769;
                }
              }
              _ => {}
            }
            match current_block {
              12027283704867122503 => {}
              _ => bufpos += 1,
            }
          }
        }
      }
      /* first one? */
      nchars -= 1;
      if !(nchars != 0) {
        current_block = 13256895345714485905;
        break;
      }
    }
  }
  match current_block {
    13256895345714485905 =>
    /* '\n' or -d CHAR */
    {
      if !(*argv.offset(0)).is_null() {
        loop
        /* Remove trailing space $IFS chars */
        {
          bufpos -= 1;
          if !(bufpos >= 0i32
            && ({
              let mut bb__isspace: libc::c_uchar =
                (*buffer.offset(bufpos as isize) as libc::c_int - 9i32) as libc::c_uchar;
              (bb__isspace as libc::c_int == ' ' as i32 - 9i32
                || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
            }) != 0
            && !strchr(ifs, *buffer.offset(bufpos as isize) as libc::c_int).is_null())
          {
            break;
          }
        }
        *buffer.offset((bufpos + 1i32) as isize) = '\u{0}' as i32 as libc::c_char;
        /* Last variable takes the entire remainder with delimiters
         * (sans trailing whitespace $IFS),
         * but ***only "if there are fewer vars than fields"(c)***!
         * The "X:Y:" case below: there are two fields,
         * and therefore last delimiter (:) is eaten:
         * IFS=": "
         * echo "X:Y:Z:"  | (read x y; echo "|$x|$y|") # |X|Y:Z:|
         * echo "X:Y:Z"   | (read x y; echo "|$x|$y|") # |X|Y:Z|
         * echo "X:Y:"    | (read x y; echo "|$x|$y|") # |X|Y|, not |X|Y:|
         * echo "X:Y  : " | (read x y; echo "|$x|$y|") # |X|Y|
         */
        if bufpos >= 0i32 && !strchr(ifs, *buffer.offset(bufpos as isize) as libc::c_int).is_null()
        {
          loop
          /* There _is_ a non-whitespace IFS char */
          /* Skip whitespace IFS char before it */
          {
            bufpos -= 1;
            if !(bufpos >= 0i32
              && ({
                let mut bb__isspace: libc::c_uchar =
                  (*buffer.offset(bufpos as isize) as libc::c_int - 9i32) as libc::c_uchar;
                (bb__isspace as libc::c_int == ' ' as i32 - 9i32
                  || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
              }) != 0
              && !strchr(ifs, *buffer.offset(bufpos as isize) as libc::c_int).is_null())
            {
              break;
            }
          }
          /* Are there $IFS chars? */
          bufpos += 1;
          if strcspn(buffer, ifs) >= bufpos as libc::c_ulong {
            /* No: last var takes one field, not more */
            /* So, drop trailing IFS delims */
            *buffer.offset(bufpos as isize) = '\u{0}' as i32 as libc::c_char
          }
        }
        /* Use the remainder as a value for the next variable */
        (*params).setvar.expect("non-null function pointer")(*argv, buffer);
        loop
        /* Set the rest to "" */
        {
          argv = argv.offset(1);
          if (*argv).is_null() {
            break;
          }
          (*params).setvar.expect("non-null function pointer")(
            *argv,
            b"\x00" as *const u8 as *const libc::c_char,
          );
        }
      } else {
        /* Note: no $IFS removal */
        *buffer.offset(bufpos as isize) = '\u{0}' as i32 as libc::c_char;
        (*params).setvar.expect("non-null function pointer")(
          b"REPLY\x00" as *const u8 as *const libc::c_char,
          buffer,
        );
      }
    }
    _ => {}
  }
  free(buffer as *mut libc::c_void);
  if read_flags & BUILTIN_READ_SILENT as libc::c_int != 0 {
    tcsetattr(pfd[0].fd, 0i32, &mut old_tty);
  }
  *bb_errno = err as libc::c_int;
  return retval;
}
static mut limits_tbl: [limits; 15] = [
  {
    let mut init = limits {
      cmd: RLIMIT_CORE as libc::c_int as u8,
      factor_shift: 9i32 as u8,
    };
    init
  },
  {
    let mut init = limits {
      cmd: RLIMIT_DATA as libc::c_int as u8,
      factor_shift: 10i32 as u8,
    };
    init
  },
  {
    let mut init = limits {
      cmd: __RLIMIT_NICE as libc::c_int as u8,
      factor_shift: 0i32 as u8,
    };
    init
  },
  {
    let mut init = limits {
      cmd: RLIMIT_FSIZE as libc::c_int as u8,
      factor_shift: 9i32 as u8,
    };
    init
  },
  {
    let mut init = limits {
      cmd: __RLIMIT_SIGPENDING as libc::c_int as u8,
      factor_shift: 0i32 as u8,
    };
    init
  },
  {
    let mut init = limits {
      cmd: __RLIMIT_MEMLOCK as libc::c_int as u8,
      factor_shift: 10i32 as u8,
    };
    init
  },
  {
    let mut init = limits {
      cmd: __RLIMIT_RSS as libc::c_int as u8,
      factor_shift: 10i32 as u8,
    };
    init
  },
  {
    let mut init = limits {
      cmd: RLIMIT_NOFILE as libc::c_int as u8,
      factor_shift: 0i32 as u8,
    };
    init
  },
  {
    let mut init = limits {
      cmd: __RLIMIT_MSGQUEUE as libc::c_int as u8,
      factor_shift: 0i32 as u8,
    };
    init
  },
  {
    let mut init = limits {
      cmd: __RLIMIT_RTPRIO as libc::c_int as u8,
      factor_shift: 0i32 as u8,
    };
    init
  },
  {
    let mut init = limits {
      cmd: RLIMIT_STACK as libc::c_int as u8,
      factor_shift: 10i32 as u8,
    };
    init
  },
  {
    let mut init = limits {
      cmd: RLIMIT_CPU as libc::c_int as u8,
      factor_shift: 0i32 as u8,
    };
    init
  },
  {
    let mut init = limits {
      cmd: __RLIMIT_NPROC as libc::c_int as u8,
      factor_shift: 0i32 as u8,
    };
    init
  },
  {
    let mut init = limits {
      cmd: RLIMIT_AS as libc::c_int as u8,
      factor_shift: 10i32 as u8,
    };
    init
  },
  {
    let mut init = limits {
      cmd: __RLIMIT_LOCKS as libc::c_int as u8,
      factor_shift: 0i32 as u8,
    };
    init
  },
];
// bash also shows:
//pipe size            (512 bytes, -p) 8
static mut limits_help: [libc::c_char; 286] = [
  99, 111, 114, 101, 32, 102, 105, 108, 101, 32, 115, 105, 122, 101, 32, 40, 98, 108, 111, 99, 107,
  115, 41, 0, 100, 97, 116, 97, 32, 115, 101, 103, 32, 115, 105, 122, 101, 32, 40, 107, 98, 41, 0,
  115, 99, 104, 101, 100, 117, 108, 105, 110, 103, 32, 112, 114, 105, 111, 114, 105, 116, 121, 0,
  102, 105, 108, 101, 32, 115, 105, 122, 101, 32, 40, 98, 108, 111, 99, 107, 115, 41, 0, 112, 101,
  110, 100, 105, 110, 103, 32, 115, 105, 103, 110, 97, 108, 115, 0, 109, 97, 120, 32, 108, 111, 99,
  107, 101, 100, 32, 109, 101, 109, 111, 114, 121, 32, 40, 107, 98, 41, 0, 109, 97, 120, 32, 109,
  101, 109, 111, 114, 121, 32, 115, 105, 122, 101, 32, 40, 107, 98, 41, 0, 111, 112, 101, 110, 32,
  102, 105, 108, 101, 115, 0, 80, 79, 83, 73, 88, 32, 109, 101, 115, 115, 97, 103, 101, 32, 113,
  117, 101, 117, 101, 115, 32, 40, 98, 121, 116, 101, 115, 41, 0, 114, 101, 97, 108, 45, 116, 105,
  109, 101, 32, 112, 114, 105, 111, 114, 105, 116, 121, 0, 115, 116, 97, 99, 107, 32, 115, 105,
  122, 101, 32, 40, 107, 98, 41, 0, 99, 112, 117, 32, 116, 105, 109, 101, 32, 40, 115, 101, 99,
  111, 110, 100, 115, 41, 0, 109, 97, 120, 32, 117, 115, 101, 114, 32, 112, 114, 111, 99, 101, 115,
  115, 101, 115, 0, 118, 105, 114, 116, 117, 97, 108, 32, 109, 101, 109, 111, 114, 121, 32, 40,
  107, 98, 41, 0, 102, 105, 108, 101, 32, 108, 111, 99, 107, 115, 0,
];
// -x
static mut limit_chars: [libc::c_char; 16] = [
  99, 100, 101, 102, 105, 108, 109, 110, 113, 114, 115, 116, 117, 118, 120, 0,
];
/* "-": treat args as parameters of option with ASCII code 1 */
static mut ulimit_opt_string: [libc::c_char; 50] = [
  45, 72, 83, 97, 99, 58, 58, 100, 58, 58, 101, 58, 58, 102, 58, 58, 105, 58, 58, 108, 58, 58, 109,
  58, 58, 110, 58, 58, 113, 58, 58, 114, 58, 58, 115, 58, 58, 116, 58, 58, 117, 58, 58, 118, 58,
  58, 120, 58, 58, 0,
];
unsafe extern "C" fn printlim(
  mut opts: libc::c_uint,
  mut limit: *const rlimit,
  mut l: *const limits,
) {
  let mut val: rlim_t = 0;
  val = (*limit).rlim_max;
  if opts & OPT_soft as libc::c_int as libc::c_uint != 0 {
    val = (*limit).rlim_cur
  }
  if val as libc::c_ulonglong == 0xffffffffffffffffu64 {
    puts(b"unlimited\x00" as *const u8 as *const libc::c_char);
  } else {
    val >>= (*l).factor_shift as libc::c_int;
    printf(
      b"%llu\n\x00" as *const u8 as *const libc::c_char,
      val as libc::c_longlong,
    );
  };
}
#[no_mangle]
pub unsafe extern "C" fn shell_builtin_ulimit(mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut limit: rlimit = rlimit {
    rlim_cur: 0,
    rlim_max: 0,
  };
  let mut opt_cnt: libc::c_uint = 0;
  let mut opts: libc::c_uint = 0;
  let mut argc: libc::c_uint = 0;
  let mut i: libc::c_uint = 0;
  /* We can't use getopt32: need to handle commands like
   * ulimit 123 -c2 -l 456
   */
  /* In case getopt() was already called:
   * reset libc getopt() internal state.
   */
  optind = 0i32;
  // bash 4.4.23:
  //
  // -H and/or -S change meaning even of options *before* them: ulimit -f 2000 -H
  // sets hard limit, ulimit -a -H prints hard limits.
  //
  // -a is equivalent for requesting all limits to be shown.
  //
  // If -a is specified, attempts to set limits are ignored:
  //  ulimit -m 1000; ulimit -m 2000 -a
  // shows 1000, not 2000. HOWEVER, *implicit* -f form "ulimit 2000 -a"
  // DOES set -f limit [we don't implement this quirk], "ulimit -a 2000" does not.
  // Options are still parsed: ulimit -az complains about unknown -z opt.
  //
  // -a is not cumulative: "ulimit -a -a" = "ulimit -a -f -m" = "ulimit -a"
  //
  // -HSa can be combined in one argument and with one other option (example: -Sm),
  // but other options can't: limit value is an optional argument,
  // thus "-mf" means "-m f", f is the parameter of -m.
  //
  // Limit can be set and then printed: ulimit -m 2000 -m
  // If set more than once, they are set and printed in order:
  // try ulimit -m -m 1000 -m -m 2000 -m -m 3000 -m
  //
  // Limits are shown in the order of options given:
  // ulimit -m -f is not the same as ulimit -f -m.
  //
  // If both -S and -H are given, show soft limit.
  //
  // Short printout (limit value only) is printed only if just one option
  // is given: ulimit -m. ulimit -f -m prints verbose lines.
  // ulimit -f -f prints same verbose line twice.
  // ulimit -m 10000 -f prints verbose line for -f.
  argc = crate::libbb::appletlib::string_array_len(argv);
  /* First pass over options: detect -H/-S/-a status,
   * and "bare ulimit" and "only one option" cases
   * by counting other opts.
   */
  opt_cnt = 0i32 as libc::c_uint; /* while (there are options) */
  opts = 0i32 as libc::c_uint;
  loop {
    let mut opt_char: libc::c_int = getopt(argc as libc::c_int, argv, ulimit_opt_string.as_ptr());
    if opt_char == -1i32 {
      break;
    }
    if opt_char == 'H' as i32 {
      opts |= OPT_hard as libc::c_int as libc::c_uint
    } else if opt_char == 'S' as i32 {
      opts |= OPT_soft as libc::c_int as libc::c_uint
    } else if opt_char == 'a' as i32 {
      opts |= OPT_all as libc::c_int as libc::c_uint
    } else {
      if opt_char == '?' as i32 {
        /* bad option. getopt already complained. */
        return 1i32;
      }
      opt_cnt = opt_cnt.wrapping_add(1)
    }
  }
  if opts & (OPT_hard as libc::c_int | OPT_soft as libc::c_int) as libc::c_uint == 0 {
    opts |= (OPT_hard as libc::c_int | OPT_soft as libc::c_int) as libc::c_uint
  }
  if opts & OPT_all as libc::c_int as libc::c_uint != 0 {
    let mut help: *const libc::c_char = limits_help.as_ptr();
    i = 0i32 as libc::c_uint;
    while i
      < (::std::mem::size_of::<[limits; 15]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<limits>() as libc::c_ulong) as libc::c_uint
    {
      getrlimit(
        limits_tbl[i as usize].cmd as __rlimit_resource_t,
        &mut limit,
      );
      printf(
        b"%-32s(-%c) \x00" as *const u8 as *const libc::c_char,
        help,
        limit_chars[i as usize] as libc::c_int,
      );
      printlim(opts, &mut limit, &*limits_tbl.as_ptr().offset(i as isize));
      help = help.offset(strlen(help).wrapping_add(1i32 as libc::c_ulong) as isize);
      i = i.wrapping_add(1)
    }
    return 0i32;
  }
  /* Second pass: set or print limits, in order */
  optind = 0i32; /* while (there are options) */
  loop {
    let mut val_str: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut opt_char_0: libc::c_int = getopt(argc as libc::c_int, argv, ulimit_opt_string.as_ptr());
    if opt_char_0 == -1i32 {
      break;
    }
    if opt_char_0 == 'H' as i32 {
      continue;
    }
    if opt_char_0 == 'S' as i32 {
      continue;
    }
    //if (opt_char == 'a') - impossible
    if opt_char_0 == 1i32 {
      /* if "ulimit NNN", -f is assumed */
      opt_char_0 = 'f' as i32
    }
    i = strchrnul(limit_chars.as_ptr(), opt_char_0).wrapping_offset_from(limit_chars.as_ptr())
      as libc::c_long as libc::c_uint;
    //if (i >= ARRAY_SIZE(limits_tbl)) - bad option, impossible
    val_str = optarg; /* ++ skips NN in "-c NN" case */
    if val_str.is_null()
      && !(*argv.offset(optind as isize)).is_null()
      && *(*argv.offset(optind as isize)).offset(0) as libc::c_int != '-' as i32
    {
      let fresh0 = optind;
      optind = optind + 1;
      val_str = *argv.offset(fresh0 as isize)
    }
    getrlimit(
      limits_tbl[i as usize].cmd as __rlimit_resource_t,
      &mut limit,
    );
    if val_str.is_null() {
      if opt_cnt > 1i32 as libc::c_uint {
        printf(
          b"%-32s(-%c) \x00" as *const u8 as *const libc::c_char,
          crate::libbb::compare_string_array::nth_string(limits_help.as_ptr(), i as libc::c_int),
          limit_chars[i as usize] as libc::c_int,
        );
      }
      printlim(opts, &mut limit, &*limits_tbl.as_ptr().offset(i as isize));
    } else {
            let mut val: rlim_t = 0xffffffffffffffffu64 as rlim_t;
            if strcmp(val_str,
                      b"unlimited\x00" as *const u8 as *const libc::c_char) !=
                   0i32 {
                if ::std::mem::size_of::<rlim_t>() as libc::c_ulong ==
                       ::std::mem::size_of::<libc::c_int>() as libc::c_ulong {
                    val =
                        crate::libbb::bb_strtonum::bb_strtou(val_str, 0 as *mut *mut libc::c_char, 10i32)
                            as rlim_t
                } else if ::std::mem::size_of::<rlim_t>() as libc::c_ulong ==
                              ::std::mem::size_of::<libc::c_long>() as
                                  libc::c_ulong {
                    val =
                        bb_strtoul(val_str, 0 as *mut *mut libc::c_char,
                                   10i32)
                } else {
                    val =
                        crate::libbb::bb_strtonum::bb_strtoull(val_str, 0 as *mut *mut libc::c_char,
                                    10i32) as rlim_t
                }
                if *bb_errno != 0 {
                    crate::libbb::verror_msg::bb_error_msg(b"invalid number \'%s\'\x00" as *const u8 as
                                     *const libc::c_char, val_str);
                    return 1i32
                }
                val <<= limits_tbl[i as usize].factor_shift as libc::c_int
            }
            //bb_error_msg("opt %c val_str:'%s' val:%lld", opt_char, val_str, (long long)val);
			/* from man bash: "If neither -H nor -S
			 * is specified, both the soft and hard
			 * limits are set. */
            if opts & OPT_hard as libc::c_int as libc::c_uint != 0 {
                limit.rlim_max = val
            }
            if opts & OPT_soft as libc::c_int as libc::c_uint != 0 {
                limit.rlim_cur = val
            }
            //bb_error_msg("setrlimit(%d, %lld, %lld)", limits_tbl[i].cmd, (long long)limit.rlim_cur, (long long)limit.rlim_max);
            if setrlimit(limits_tbl[i as usize].cmd as __rlimit_resource_t,
                         &mut limit) < 0i32 {
                crate::libbb::perror_msg::bb_simple_perror_msg(b"error setting limit\x00" as *const u8
                                         as *const libc::c_char);
                return 1i32
            }
        }
  }
  if opt_cnt == 0i32 as libc::c_uint {
    /* "bare ulimit": treat it as if it was -f */
    getrlimit(limits_tbl[3].cmd as __rlimit_resource_t, &mut limit);
    printlim(opts, &mut limit, &*limits_tbl.as_ptr().offset(3));
  }
  return 0i32;
}

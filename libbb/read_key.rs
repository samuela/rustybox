use libc;
extern "C" {
  #[no_mangle]
  fn strtoul(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_ulong;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn safe_poll(ufds: *mut pollfd, nfds: nfds_t, timeout_ms: libc::c_int) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
  pub fd: libc::c_int,
  pub events: libc::c_short,
  pub revents: libc::c_short,
}
pub type C2RustUnnamed = libc::c_int;
pub const KEYCODE_BUFFER_SIZE: C2RustUnnamed = 16;
pub const KEYCODE_CURSOR_POS: C2RustUnnamed = -256;
pub const KEYCODE_ALT_D: C2RustUnnamed = -45;
pub const KEYCODE_ALT_BACKSPACE: C2RustUnnamed = -44;
pub const KEYCODE_ALT_LEFT: C2RustUnnamed = -37;
pub const KEYCODE_ALT_RIGHT: C2RustUnnamed = -36;
pub const KEYCODE_CTRL_LEFT: C2RustUnnamed = -69;
pub const KEYCODE_CTRL_RIGHT: C2RustUnnamed = -68;
pub const KEYCODE_D: C2RustUnnamed = -13;
pub const KEYCODE_BACKSPACE: C2RustUnnamed = -12;
pub const KEYCODE_PAGEDOWN: C2RustUnnamed = -11;
pub const KEYCODE_PAGEUP: C2RustUnnamed = -10;
pub const KEYCODE_DELETE: C2RustUnnamed = -9;
pub const KEYCODE_INSERT: C2RustUnnamed = -8;
pub const KEYCODE_END: C2RustUnnamed = -7;
pub const KEYCODE_HOME: C2RustUnnamed = -6;
pub const KEYCODE_LEFT: C2RustUnnamed = -5;
pub const KEYCODE_RIGHT: C2RustUnnamed = -4;
pub const KEYCODE_DOWN: C2RustUnnamed = -3;
pub const KEYCODE_UP: C2RustUnnamed = -2;
/* vi: set sw=4 ts=4: */
/*
 * Utility routines.
 *
 * Copyright (C) 2008 Rob Landley <rob@landley.net>
 * Copyright (C) 2008 Denys Vlasenko <vda.linux@googlemail.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn read_key(
  mut fd: libc::c_int,
  mut buffer: *mut libc::c_char,
  mut timeout: libc::c_int,
) -> int64_t {
  let mut current_block: u64;
  let mut pfd: pollfd = pollfd {
    fd: 0,
    events: 0,
    revents: 0,
  };
  let mut seq: *const libc::c_char = 0 as *const libc::c_char;
  let mut n: libc::c_int = 0;
  /* Known escape sequences for cursor and function keys.
   * See "Xterm Control Sequences"
   * http://invisible-island.net/xterm/ctlseqs/ctlseqs.html
   * Array should be sorted from shortest to longest.
   */
  static mut esccmds: [libc::c_char; 103] = [
    ('\u{7f}' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_ALT_BACKSPACE as libc::c_int as libc::c_char,
    ('\u{8}' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_ALT_BACKSPACE as libc::c_int as libc::c_char,
    ('d' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_ALT_D as libc::c_int as libc::c_char,
    ('f' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_ALT_RIGHT as libc::c_int as libc::c_char,
    ('b' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_ALT_LEFT as libc::c_int as libc::c_char,
    'O' as i32 as libc::c_char,
    ('A' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_UP as libc::c_int as libc::c_char,
    'O' as i32 as libc::c_char,
    ('B' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_DOWN as libc::c_int as libc::c_char,
    'O' as i32 as libc::c_char,
    ('C' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_RIGHT as libc::c_int as libc::c_char,
    'O' as i32 as libc::c_char,
    ('D' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_LEFT as libc::c_int as libc::c_char,
    'O' as i32 as libc::c_char,
    ('H' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_HOME as libc::c_int as libc::c_char,
    'O' as i32 as libc::c_char,
    ('F' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_END as libc::c_int as libc::c_char,
    '[' as i32 as libc::c_char,
    ('A' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_UP as libc::c_int as libc::c_char,
    '[' as i32 as libc::c_char,
    ('B' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_DOWN as libc::c_int as libc::c_char,
    '[' as i32 as libc::c_char,
    ('C' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_RIGHT as libc::c_int as libc::c_char,
    '[' as i32 as libc::c_char,
    ('D' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_LEFT as libc::c_int as libc::c_char,
    '[' as i32 as libc::c_char,
    ('H' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_HOME as libc::c_int as libc::c_char,
    '[' as i32 as libc::c_char,
    ('F' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_END as libc::c_int as libc::c_char,
    '[' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ('~' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_HOME as libc::c_int as libc::c_char,
    '[' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    ('~' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_INSERT as libc::c_int as libc::c_char,
    '[' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    ('~' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_DELETE as libc::c_int as libc::c_char,
    '[' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    ('~' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_END as libc::c_int as libc::c_char,
    '[' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    ('~' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_PAGEUP as libc::c_int as libc::c_char,
    '[' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    ('~' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_PAGEDOWN as libc::c_int as libc::c_char,
    '[' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    ('~' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_HOME as libc::c_int as libc::c_char,
    '[' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    ('~' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_END as libc::c_int as libc::c_char,
    '[' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    ('C' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_CTRL_RIGHT as libc::c_int as libc::c_char,
    '[' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    ('D' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_CTRL_LEFT as libc::c_int as libc::c_char,
    '[' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    ('C' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_ALT_RIGHT as libc::c_int as libc::c_char,
    '[' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ';' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    ('D' as i32 | 0x80i32) as libc::c_char,
    KEYCODE_ALT_LEFT as libc::c_int as libc::c_char,
    0i32 as libc::c_char,
  ]; /* saved chars counter is in buffer[-1] now */
  pfd.fd = fd;
  pfd.events = 0x1i32 as libc::c_short;
  buffer = buffer.offset(1);
  loop {
    *bb_errno = 0i32;
    n = *buffer.offset(-1i32 as isize) as libc::c_uchar as libc::c_int;
    if n == 0i32 {
      /* If no data, wait for input.
       * If requested, wait TIMEOUT ms. TIMEOUT = -1 is useful
       * if fd can be in non-blocking mode.
       */
      if timeout >= -1i32 {
        if safe_poll(&mut pfd, 1i32 as nfds_t, timeout) == 0i32 {
          /* Timed out */
          *bb_errno = 11i32;
          return -1i32 as int64_t;
        }
      }
      /* It is tempting to read more than one byte here,
       * but it breaks pasting. Example: at shell prompt,
       * user presses "c","a","t" and then pastes "\nline\n".
       * When we were reading 3 bytes here, we were eating
       * "li" too, and cat was getting wrong input.
       */
      n = safe_read(fd, buffer as *mut libc::c_void, 1i32 as size_t) as libc::c_int;
      if n <= 0i32 {
        return -1i32 as int64_t;
      }
    }
    let mut c: libc::c_uchar = *buffer.offset(0) as libc::c_uchar;
    n -= 1;
    if n != 0 {
      memmove(
        buffer as *mut libc::c_void,
        buffer.offset(1) as *const libc::c_void,
        n as libc::c_ulong,
      );
    }
    /* Only ESC starts ESC sequences */
    if c as libc::c_int != 27i32 {
      *buffer.offset(-1i32 as isize) = n as libc::c_char;
      return c as int64_t;
    }
    /* Loop through known ESC sequences */
    seq = esccmds.as_ptr();
    's_125: loop {
      if !(*seq as libc::c_int != '\u{0}' as i32) {
        current_block = 6476622998065200121;
        break;
      }
      /* n - position in sequence we did not read yet */
      let mut i: libc::c_int = 0i32; /* position in sequence to compare */
      /* Loop through chars in this sequence */
      loop
      /* So far escape sequence matched up to [i-1] */
      {
        if n <= i {
          /* Need more chars, read another one if it wouldn't block.
           * Note that escape sequences come in as a unit,
           * so if we block for long it's not really an escape sequence.
           * Timeout is needed to reconnect escape sequences
           * split up by transmission over a serial console. */
          if safe_poll(&mut pfd, 1i32 as nfds_t, 50i32) == 0i32 {
            current_block = 16551332604341906318;
            break 's_125;
          }
          *bb_errno = 0i32;
          if safe_read(
            fd,
            buffer.offset(n as isize) as *mut libc::c_void,
            1i32 as size_t,
          ) <= 0i32 as libc::c_long
          {
            /* If EAGAIN, then fd is O_NONBLOCK and poll lied:
             * in fact, there is no data. */
            if *bb_errno != 11i32 {
              /* otherwise: it's EOF/error */
              *buffer.offset(-1i32 as isize) = 0i32 as libc::c_char;
              return -1i32 as int64_t;
            }
            current_block = 16551332604341906318;
            break 's_125;
          } else {
            n += 1
          }
        }
        if *buffer.offset(i as isize) as libc::c_int
          != *seq.offset(i as isize) as libc::c_int & 0x7fi32
        {
          /* This seq doesn't match, go to next */
          seq = seq.offset(i as isize);
          /* Forward to last char */
          while *seq as libc::c_int & 0x80i32 == 0 {
            seq = seq.offset(1)
          }
          /* Skip it and the keycode which follows */
          seq = seq.offset(2);
          break;
        } else {
          if *seq.offset(i as isize) as libc::c_int & 0x80i32 != 0 {
            /* Entire seq matched */
            n = 0i32;
            /* n -= i; memmove(...);
             * would be more correct,
             * but we never read ahead that much,
             * and n == i here. */
            *buffer.offset(-1i32 as isize) = 0i32 as libc::c_char;
            return *seq.offset((i + 1i32) as isize) as libc::c_schar as int64_t;
          }
          i += 1
        }
      }
    }
    match current_block {
      6476622998065200121 => {
        /* We did not find matching sequence.
         * We possibly read and stored more input in buffer[] by now.
         * n = bytes read. Try to read more until we time out.
         */
        while n < KEYCODE_BUFFER_SIZE as libc::c_int - 1i32 {
          /* 1 for count byte at buffer[-1] */
          if safe_poll(&mut pfd, 1i32 as nfds_t, 50i32) == 0i32 {
            break;
          }
          *bb_errno = 0i32;
          if safe_read(
            fd,
            buffer.offset(n as isize) as *mut libc::c_void,
            1i32 as size_t,
          ) <= 0i32 as libc::c_long
          {
            /* If EAGAIN, then fd is O_NONBLOCK and poll lied:
             * in fact, there is no data. */
            if *bb_errno != 11i32 {
              /* otherwise: it's EOF/error */
              *buffer.offset(-1i32 as isize) = 0i32 as libc::c_char;
              return -1i32 as int64_t;
            }
            break;
          } else {
            n += 1;
            /* Try to decipher "ESC [ NNN ; NNN R" sequence */
            if !((0i32 != 0 || 1i32 != 0 || 1i32 != 0)
              && n >= 5i32
              && *buffer.offset(0) as libc::c_int == '[' as i32
              && *buffer.offset((n - 1i32) as isize) as libc::c_int == 'R' as i32
              && (*buffer.offset(1) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int
                <= 9i32)
            {
              continue;
            }
            let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut row: libc::c_ulong = 0;
            let mut col: libc::c_ulong = 0;
            row = strtoul(buffer.offset(1), &mut end, 10i32);
            if *end as libc::c_int != ';' as i32
              || !((*end.offset(1) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int
                <= 9i32)
            {
              continue;
            }
            col = strtoul(end.offset(1), &mut end, 10i32);
            if *end as libc::c_int != 'R' as i32 {
              continue;
            }
            if row < 1i32 as libc::c_ulong
              || col < 1i32 as libc::c_ulong
              || row | col > 0x7fffi32 as libc::c_ulong
            {
              continue;
            }
            *buffer.offset(-1i32 as isize) = 0i32 as libc::c_char;
            /* Pack into "1 <row15bits> <col16bits>" 32-bit sequence */
            row |= ((-1i32 as libc::c_uint) << 15i32) as libc::c_ulong;
            col |= row << 16i32;
            /* Return it in high-order word */
            return (col as int64_t) << 32i32
              | KEYCODE_CURSOR_POS as libc::c_int as uint32_t as libc::c_long;
          }
        }
      }
      _ => {}
    }
    /* No more data!
     * Array is sorted from shortest to longest,
     * we can't match anything later in array -
     * anything later is longer than this seq.
     * Break out of both loops. */
    if n <= 1i32 {
      /* Alt-x is usually returned as ESC x.
       * Report ESC, x is remembered for the next call.
       */
      *buffer.offset(-1i32 as isize) = n as libc::c_char;
      return 27i32 as int64_t;
    }
    /* We were doing "buffer[-1] = n; return c;" here, but this results
     * in unknown key sequences being interpreted as ESC + garbage.
     * This was not useful. Pretend there was no key pressed,
     * go and wait for a new keypress:
     */
    *buffer.offset(-1i32 as isize) = 0i32 as libc::c_char
  }
}
/* vi: set sw=4 ts=4: */
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
/*uint8_t *server_write_MAC_key;*/
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
/* Similar, but used by chgrp, not shell */
/* Used by ftpd */
/* Don't need IF_xxx() guard for these */
/* Networking */
/* This structure defines protocol families and their handlers. */
/*int type,*/
/* may modify src */
/* This structure defines hardware protocols and their handlers. */
/*
 * If *devname is not NULL, use that name, otherwise try to find free one,
 * malloc and return it in *devname.
 * return value is the opened fd to the loop device, or < on error
 */
/* These constants match linux/loop.h (without BB_ prefix): */
/* Returns malloced str */
/* Like bb_ask_noecho, but asks on stdin with no timeout.  */
/* Returns -1 if input is invalid. current_mode is a base for e.g. "u+rw" */
/*
 * Config file parser
 */
// treat consecutive delimiters as one
// trim leading and trailing delimiters
// TODO: COLLAPSE and TRIM seem to always go in pair
// last token takes entire remainder of the line
// die if < min tokens found
// keep a copy of current line
// comments are recognized even if they aren't the first char
// delim[0] and delim[1] are two different allowed comment chars
// (so far, delim[0] will only work as comment char for full-line comment)
// (IOW: it works as if PARSE_EOL_COMMENTS is not set. sysctl applet is okay with this)
// comments are recognized even if there is whitespace before
// ("line start><space><tab><space>#comment" is also comment, not only "line start>#comment")
// NORMAL is:
// * remove leading and trailing delimiters and collapse
//   multiple delimiters into one
// * warn and continue if less than mintokens delimiters found
// * grab everything into last token
// * comments are recognized even if they aren't the first char
/* delims[0] is a comment char (use '\0' to disable), the rest are token delimiters */
/* Concatenate path and filename to new allocated buffer.
 * Add "/" only as needed (no duplicate "//" are produced).
 * If path is NULL, it is assumed to be "/".
 * filename should not be NULL. */
/* Returns NULL on . and .. */
/* Returns ptr to NUL */
/* Returns $SHELL, getpwuid(getuid())->pw_shell, or DEFAULT_SHELL.
 * Note that getpwuid result might need xstrdup'ing
 * if there is a possibility of intervening getpwxxx() calls.
 */
/* Structures inside "struct caps" are Linux-specific and libcap-specific: */
/* setup_environment:
 * if chdir pw->pw_dir: ok: else if to_tmp == 1: goto /tmp else: goto / or die
 * if clear_env = 1: cd(pw->pw_dir), clear environment, then set
 *   TERM=(old value)
 *   USER=pw->pw_name, LOGNAME=pw->pw_name
 *   PATH=bb_default_[root_]path
 *   HOME=pw->pw_dir
 *   SHELL=shell
 * else if change_env = 1:
 *   if not root (if pw->pw_uid != 0):
 *     USER=pw->pw_name, LOGNAME=pw->pw_name
 *   HOME=pw->pw_dir
 *   SHELL=shell
 * else does nothing
 *
 * NB: CHANGEENV and CLEARENV use setenv() - this leaks memory!
 * If setup_environment() is used is vforked child, this leaks memory _in parent too_!
 */
/* Returns a malloced string */
/*
 * rnd is additional random input. New one is returned.
 * Useful if you call crypt_make_salt many times in a row:
 * rnd = crypt_make_salt(buf1, 4, 0);
 * rnd = crypt_make_salt(buf2, 4, rnd);
 * rnd = crypt_make_salt(buf3, 4, rnd);
 * (otherwise we risk having same salt generated)
 */
/*, int rnd*/
/* "$N$" + sha_salt_16_bytes + NUL */
/* Returns number of lines changed, or -1 on error */
/* NB: typically you want to pass fd 0, not 1. Think 'applet | grep something' */
/* NB: "unsigned request" is crucial! "int request" will break some arches! */
/* At least glibc has horrendously large inline for this, so wrap it */
/* "Keycodes" that report an escape sequence.
 * We use something which fits into signed char,
 * yet doesn't represent any valid Unicode character.
 * Also, -1 is reserved for error indication and we don't use it. */
/* Used only if Alt/Ctrl/Shifted */
/* Used only if Alted */
/* ^^^^^ Be sure that last defined value is small enough.
 * Current read_key() code allows going up to -32 (0xfff..fffe0).
 * This gives three upper bits in LSB to play with:
 * KEYCODE_foo values are 0xfff..fffXX, lowest XX bits are: scavvvvv,
 * s=0 if SHIFT, c=0 if CTRL, a=0 if ALT,
 * vvvvv bits are the same for same key regardless of "shift bits".
 */
//KEYCODE_SHIFT_...   = KEYCODE_...   & ~0x80,
/* 0xfff..fff00 */
/* How long is the longest ESC sequence we know?
 * We want it big enough to be able to contain
 * cursor position sequence "ESC [ 9999 ; 9999 R"
 */
/* Note: fd may be in blocking or non-blocking mode, both make sense.
 * For one, less uses non-blocking mode.
 * Only the first read syscall inside read_key may block indefinitely
 * (unless fd is in non-blocking mode),
 * subsequent reads will time out after a few milliseconds.
 * Return of -1 means EOF or error (errno == 0 on EOF).
 * buffer[0] is used as a counter of buffered chars and must be 0
 * on first call.
 * timeout:
 * -2: do not poll(-1) for input - read() it, return on EAGAIN at once
 * -1: poll(-1) (i.e. block even on NONBLOCKed fd)
 * >=0: poll() for TIMEOUT milliseconds, return -1/EAGAIN on timeout
 */
#[no_mangle]
pub unsafe extern "C" fn read_key_ungets(
  mut buffer: *mut libc::c_char,
  mut str: *const libc::c_char,
  mut len: libc::c_uint,
) {
  let mut cur_len: libc::c_uint = *buffer.offset(0) as libc::c_uchar as libc::c_uint;
  if len > ((KEYCODE_BUFFER_SIZE as libc::c_int - 1i32) as libc::c_uint).wrapping_sub(cur_len) {
    len = ((KEYCODE_BUFFER_SIZE as libc::c_int - 1i32) as libc::c_uint).wrapping_sub(cur_len)
  }
  memcpy(
    buffer.offset(1).offset(cur_len as isize) as *mut libc::c_void,
    str as *const libc::c_void,
    len as libc::c_ulong,
  );
  let ref mut fresh0 = *buffer.offset(0);
  *fresh0 = (*fresh0 as libc::c_uint).wrapping_add(len) as libc::c_char as libc::c_char;
}

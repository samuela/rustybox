use crate::librb::size_t;
use libc;








use libc::ioctl;






















use libc::getenv;



use libc::isatty;














use libc::atoi;













use libc::open;

use libc::close;

use libc::cc_t;
use libc::pid_t;
use libc::ssize_t;
use libc::termios;
use libc::winsize;

extern "C" {



  #[no_mangle]
  fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;

  #[no_mangle]
  fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  fn waitpid(__pid: pid_t, __stat_loc: *mut libc::c_int, __options: libc::c_int) -> pid_t;
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
  #[no_mangle]
  static bb_errno: *mut libc::c_int;

  #[no_mangle]
  fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;

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
  /* It's NOT just ENABLEd or disabled. It's a number: */
  /* must never be <= 0 */
  /* meaning of this field depends on FEATURE_EDITING_SAVE_ON_EXIT:
   * if !FEATURE_EDITING_SAVE_ON_EXIT: "how many lines are
   * in on-disk history"
   * if FEATURE_EDITING_SAVE_ON_EXIT: "how many in-memory lines are
   * also in on-disk history (and thus need to be skipped on save)"
   */
  /*
   * maxsize must be >= 2.
   * Returns:
   * -1 on read errors or EOF, or on bare Ctrl-D,
   * 0  on ctrl-C (the line entered is still returned in 'command'),
   * >0 length of input string, including terminating '\n'
   */
  /* synchronize with sizeof(task_struct.comm) in /usr/include/linux/sched.h */
  // For mixed 32/64 userspace, 32-bit pmap still needs
  // 64-bit field here to correctly show 64-bit processes:
  // (strictly speaking, other fields need to be wider too,
  // but they are in kbytes, not bytes, and they hold sizes,
  // not start addresses, sizes tend to be less than 4 terabytes)
  /* Fields are set to 0/NULL if failed to determine (or not requested) */
  /* Everything below must contain no ptrs to malloc'ed data:
   * it is memset(0) for each process in procps_scan() */
  /* we round it to kbytes */
  /* basename of executable in exec(2), read from /proc/N/stat
   * (if executable is symlink or script, it is NOT replaced
   * by link target or interpreter name) */
  /* user/group? - use passwd/group parsing functions */
  /* flag bits for procps_scan(xx, flags) calls */
  /* PSSCAN_CMD      = 1 << 6, - use read_cmdline instead */
  /* NB: used by find_pid_by_name(). Any applet using it
   * needs to be mentioned here. */
  //procps_status_t* alloc_procps_scan(void) FAST_FUNC;
  /* Format cmdline (up to col chars) into char buf[size] */
  /* Puts [comm] if cmdline is empty (-> process is a kernel thread) */
  /* Use strict=1 if you process input from untrusted source:
   * it will return NULL on invalid %xx (bad hex chars)
   * and str + 1 if decoded char is / or NUL.
   * In non-strict mode, it always succeeds (returns str),
   * and also it additionally decoded '+' to space.
   */
  /* Sign-extends to a value which never matches fgetc result: */
  /* always correctly aligned for uint64_t */
  /* must be directly before hash[] */
  /* 4 elements for md5, 5 for sha1, 8 for sha256 */
  /* must be directly before hash[] */
  /* always correctly aligned for uint64_t */
  /* TLS benefits from knowing that sha1 and sha256 share these. Give them "agnostic" names too */
  /*unsigned last_eta;*/
  /* Some older linkers don't perform string merging, we used to have common strings
   * as global arrays to do it by hand. But:
   * (1) newer linkers do it themselves,
   * (2) however, they DONT merge string constants with global arrays,
   * even if the value is the same (!). Thus global arrays actually
   * increased size a bit: for example, "/etc/passwd" string from libc
   * wasn't merged with bb_path_passwd_file[] array!
   * Therefore now we use #defines.
   */
  /* "BusyBox vN.N.N (timestamp or extra_version)" */
  /* NB: (bb_hexdigits_upcase[i] | 0x20) -> lowercase hex digit */
  #[no_mangle]
  static bb_hexdigits_upcase: [libc::c_char; 0];

  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
}

pub const IFNAMSIZ: C2RustUnnamed = 16;
pub type C2RustUnnamed = libc::c_uint;

/*
 * Utility routines.
 *
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 * Copyright (C) 2006 Rob Landley
 * Copyright (C) 2006 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* We need to have separate xfuncs.c and xfuncs_printf.c because
 * with current linkers, even with section garbage collection,
 * if *.o module references any of XXXprintf functions, you pull in
 * entire printf machinery. Even if you do not use the function
 * which uses XXXprintf.
 *
 * xfuncs.c contains functions (not necessarily xfuncs)
 * which do not pull in printf, directly or indirectly.
 * xfunc_printf.c contains those which do.
 *
 * TODO: move xmalloc() and xatonum() here.
 */
/* Turn on nonblocking I/O on a fd */
#[no_mangle]
pub unsafe extern "C" fn ndelay_on(mut fd: libc::c_int) -> libc::c_int {
  let mut flags: libc::c_int = fcntl(fd, 3i32);
  if flags & 0o4000i32 != 0 {
    return flags;
  }
  fcntl(fd, 4i32, flags | 0o4000i32);
  return flags;
}
#[no_mangle]
pub unsafe extern "C" fn ndelay_off(mut fd: libc::c_int) -> libc::c_int {
  let mut flags: libc::c_int = fcntl(fd, 3i32);
  if flags & 0o4000i32 == 0 {
    return flags;
  }
  fcntl(fd, 4i32, flags & !0o4000i32);
  return flags;
}
#[no_mangle]
pub unsafe extern "C" fn close_on_exec_on(mut fd: libc::c_int) {
  fcntl(fd, 2i32, 1i32);
}
#[no_mangle]
pub unsafe extern "C" fn strncpy_IFNAMSIZ(
  mut dst: *mut libc::c_char,
  mut src: *const libc::c_char,
) -> *mut libc::c_char {
  return strncpy(dst, src, IFNAMSIZ as libc::c_int as libc::c_ulong);
}
/* Convert unsigned integer to ascii, writing into supplied buffer.
 * A truncated result contains the first few digits of the result ala strncpy.
 * Returns a pointer past last generated digit, does _not_ store NUL.
 */
#[no_mangle]
pub unsafe extern "C" fn utoa_to_buf(
  mut n: libc::c_uint,
  mut buf: *mut libc::c_char,
  mut buflen: libc::c_uint,
) -> *mut libc::c_char {
  let mut i: libc::c_uint = 0;
  let mut out: libc::c_uint = 0;
  let mut res: libc::c_uint = 0;
  if buflen != 0 {
    out = 0i32 as libc::c_uint;
    if ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong == 4i32 as libc::c_ulong {
      // 2^32-1 = 4294967295
      i = 1000000000i32 as libc::c_uint
    }
    /* prevents warning about "const too large" */
    while i != 0 {
      res = n.wrapping_div(i);
      n = n.wrapping_rem(i);
      if res != 0 || out != 0 || i == 1i32 as libc::c_uint {
        buflen = buflen.wrapping_sub(1);
        if buflen == 0i32 as libc::c_uint {
          break;
        }
        out = out.wrapping_add(1);
        let fresh0 = buf;
        buf = buf.offset(1);
        *fresh0 = ('0' as i32 as libc::c_uint).wrapping_add(res) as libc::c_char
      }
      i = i.wrapping_div(10i32 as libc::c_uint)
    }
  }
  return buf;
}
/* Convert signed integer to ascii, like utoa_to_buf() */
#[no_mangle]
pub unsafe extern "C" fn itoa_to_buf(
  mut n: libc::c_int,
  mut buf: *mut libc::c_char,
  mut buflen: libc::c_uint,
) -> *mut libc::c_char {
  if buflen == 0 {
    return buf;
  }
  if n < 0i32 {
    n = -n;
    let fresh1 = buf;
    buf = buf.offset(1);
    *fresh1 = '-' as i32 as libc::c_char;
    buflen = buflen.wrapping_sub(1)
  }
  return utoa_to_buf(n as libc::c_uint, buf, buflen);
}
// The following two functions use a static buffer, so calling either one a
// second time will overwrite previous results.
//
// The largest 32 bit integer is -2 billion plus NUL, or 1+10+1=12 bytes.
// It so happens that sizeof(int) * 3 is enough for 32+ bit ints.
// (sizeof(int) * 3 + 2 is correct for any width, even 8-bit)
static mut local_buf: [libc::c_char; 12] = [0; 12];

/* Convert unsigned integer to ascii using a static buffer (returned). */
#[no_mangle]
pub unsafe extern "C" fn utoa(mut n: libc::c_uint) -> *mut libc::c_char {
  *utoa_to_buf(
    n,
    local_buf.as_mut_ptr(),
    (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong) as libc::c_uint,
  ) = '\u{0}' as i32 as libc::c_char;
  return local_buf.as_mut_ptr();
}

/* Convert signed integer to ascii using a static buffer (returned). */
#[no_mangle]
pub unsafe extern "C" fn itoa(mut n: libc::c_int) -> *mut libc::c_char {
  *itoa_to_buf(
    n,
    local_buf.as_mut_ptr(),
    (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong) as libc::c_uint,
  ) = '\u{0}' as i32 as libc::c_char;
  return local_buf.as_mut_ptr();
}

/* Emit a string of hex representation of bytes */
#[no_mangle]
pub unsafe extern "C" fn bin2hex(
  mut p: *mut libc::c_char,
  mut cp: *const libc::c_char,
  mut count: libc::c_int,
) -> *mut libc::c_char {
  while count != 0 {
    let fresh2 = cp;
    cp = cp.offset(1);
    let mut c: libc::c_uchar = *fresh2 as libc::c_uchar;
    /* put lowercase hex digits */
    let fresh3 = p;
    p = p.offset(1);
    *fresh3 = (0x20i32
      | *bb_hexdigits_upcase
        .as_ptr()
        .offset((c as libc::c_int >> 4i32) as isize) as libc::c_int) as libc::c_char;
    let fresh4 = p;
    p = p.offset(1);
    *fresh4 = (0x20i32
      | *bb_hexdigits_upcase
        .as_ptr()
        .offset((c as libc::c_int & 0xfi32) as isize) as libc::c_int) as libc::c_char;
    count -= 1
  }
  return p;
}

/* Convert "[x]x[:][x]x[:][x]x[:][x]x" hex string to binary, no more than COUNT bytes */
#[no_mangle]
pub unsafe extern "C" fn hex2bin(
  mut dst: *mut libc::c_char,
  mut str: *const libc::c_char,
  mut count: libc::c_int,
) -> *mut libc::c_char {
  *bb_errno = 22i32;
  while *str as libc::c_int != 0 && count != 0 {
    let mut val: u8 = 0;
    let fresh5 = str;
    str = str.offset(1);
    let mut c: u8 = *fresh5 as u8;
    if (c as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
      val = (c as libc::c_int - '0' as i32) as u8
    } else if c as libc::c_int | 0x20i32 >= 'a' as i32 && c as libc::c_int | 0x20i32 <= 'f' as i32 {
      val = ((c as libc::c_int | 0x20i32) - ('a' as i32 - 10i32)) as u8
    } else {
      return 0 as *mut libc::c_char;
    }
    val = ((val as libc::c_int) << 4i32) as u8;
    c = *str as u8;
    if (c as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
      val = (val as libc::c_int | c as libc::c_int - '0' as i32) as u8
    } else if c as libc::c_int | 0x20i32 >= 'a' as i32 && c as libc::c_int | 0x20i32 <= 'f' as i32 {
      val = (val as libc::c_int | (c as libc::c_int | 0x20i32) - ('a' as i32 - 10i32)) as u8
    } else if c as libc::c_int == ':' as i32 || c as libc::c_int == '\u{0}' as i32 {
      val = (val as libc::c_int >> 4i32) as u8
    } else {
      return 0 as *mut libc::c_char;
    }
    let fresh6 = dst;
    dst = dst.offset(1);
    *fresh6 = val as libc::c_char;
    if c as libc::c_int != '\u{0}' as i32 {
      str = str.offset(1)
    }
    if *str as libc::c_int == ':' as i32 {
      str = str.offset(1)
    }
    count -= 1
  }
  *bb_errno = if *str as libc::c_int != 0 {
    34i32
  } else {
    0i32
  };
  return dst;
}

/* Return how long the file at fd is, if there's any way to determine it. */
#[no_mangle]
pub unsafe extern "C" fn bb_putchar_stderr(mut ch: libc::c_char) -> libc::c_int {
  return write(
    2i32,
    &mut ch as *mut libc::c_char as *const libc::c_void,
    1i32 as size_t,
  ) as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn full_write1_str(mut string: *const libc::c_char) -> ssize_t {
  return full_write(1i32, string as *const libc::c_void, strlen(string));
}

#[no_mangle]
pub unsafe extern "C" fn full_write2_str(mut string: *const libc::c_char) -> ssize_t {
  return full_write(2i32, string as *const libc::c_void, strlen(string));
}

unsafe extern "C" fn wh_helper(
  mut value: libc::c_int,
  mut def_val: libc::c_int,
  mut env_name: *const libc::c_char,
  mut err: *mut libc::c_int,
) -> libc::c_int {
  /* Envvars override even if "value" from ioctl is valid (>0).
   * Rationale: it's impossible to guess what user wants.
   * For example: "man CMD | ...": should "man" format output
   * to stdout's width? stdin's width? /dev/tty's width? 80 chars?
   * We _cant_ know it. If "..." saves text for e.g. email,
   * then it's probably 80 chars.
   * If "..." is, say, "grep -v DISCARD | $PAGER", then user
   * would prefer his tty's width to be used!
   *
   * Since we don't know, at least allow user to do this:
   * "COLUMNS=80 man CMD | ..."
   */
  let mut s: *mut libc::c_char = getenv(env_name);
  if !s.is_null() {
    value = atoi(s);
    /* If LINES/COLUMNS are set, pretend that there is
     * no error getting w/h, this prevents some ugly
     * cursor tricks by our callers */
    *err = 0i32
  }
  if value <= 1i32 || value >= 30000i32 {
    value = def_val
  }
  return value;
}
/* It is perfectly ok to pass in a NULL for either width or for
 * height, in which case that value will not be set.  */
#[no_mangle]
pub unsafe extern "C" fn get_terminal_width_height(
  mut fd: libc::c_int,
  mut width: *mut libc::c_uint,
  mut height: *mut libc::c_uint,
) -> libc::c_int {
  let mut win: winsize = winsize {
    ws_row: 0,
    ws_col: 0,
    ws_xpixel: 0,
    ws_ypixel: 0,
  };
  let mut err: libc::c_int = 0;
  let mut close_me: libc::c_int = -1i32;
  if fd == -1i32 {
    if isatty(1i32) != 0 {
      fd = 1i32
    } else if isatty(2i32) != 0 {
      fd = 2i32
    } else if isatty(0i32) != 0 {
      fd = 0i32
    } else {
      fd = open(b"/dev/tty\x00" as *const u8 as *const libc::c_char, 0i32);
      close_me = fd
    }
  }
  win.ws_row = 0i32 as libc::c_ushort;
  win.ws_col = 0i32 as libc::c_ushort;
  /* I've seen ioctl returning 0, but row/col is (still?) 0.
   * We treat that as an error too.  */
  err = (ioctl(fd, 0x5413i32 as libc::c_ulong, &mut win as *mut winsize) != 0i32
    || win.ws_row as libc::c_int == 0i32) as libc::c_int;
  if !height.is_null() {
    *height = wh_helper(
      win.ws_row as libc::c_int,
      24i32,
      b"LINES\x00" as *const u8 as *const libc::c_char,
      &mut err,
    ) as libc::c_uint
  }
  if !width.is_null() {
    *width = wh_helper(
      win.ws_col as libc::c_int,
      80i32,
      b"COLUMNS\x00" as *const u8 as *const libc::c_char,
      &mut err,
    ) as libc::c_uint
  }
  if close_me >= 0i32 {
    close(close_me);
  }
  return err;
}
#[no_mangle]
pub unsafe extern "C" fn get_terminal_width(mut fd: libc::c_int) -> libc::c_int {
  let mut width: libc::c_uint = 0;
  get_terminal_width_height(fd, &mut width, 0 as *mut libc::c_uint);
  return width as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tcsetattr_stdin_TCSANOW(mut tp: *const termios) -> libc::c_int {
  return tcsetattr(0i32, 0i32, tp);
}
#[no_mangle]
pub unsafe extern "C" fn get_termios_and_make_raw(
  mut fd: libc::c_int,
  mut newterm: *mut termios,
  mut oldterm: *mut termios,
  mut flags: libc::c_int,
) -> libc::c_int {
  //TODO: slattach, shell read might be adapted to use this too: grep for "tcsetattr", "[VTIME] = 0"
  let mut r: libc::c_int = 0; /* paranoia */
  memset(
    oldterm as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<termios>() as libc::c_ulong,
  );
  r = tcgetattr(fd, oldterm);
  *newterm = *oldterm;
  /* Turn off buffered input (ICANON)
   * Turn off echoing (ECHO)
   * and separate echoing of newline (ECHONL, normally off anyway)
   */
  (*newterm).c_lflag &= !(0o2i32 | 0o10i32 | 0o100i32) as libc::c_uint;
  if flags & 1i32 << 0i32 != 0 {
    /* dont recognize INT/QUIT/SUSP chars */
    (*newterm).c_lflag &= !0o1i32 as libc::c_uint
  }
  /* reads will block only if < 1 char is available */
  (*newterm).c_cc[6] = 1i32 as cc_t;
  /* no timeout (reads block forever) */
  (*newterm).c_cc[5] = 0i32 as cc_t;
  /* IXON, IXOFF, and IXANY:
   * IXOFF=1: sw flow control is enabled on input queue:
   * tty transmits a STOP char when input queue is close to full
   * and transmits a START char when input queue is nearly empty.
   * IXON=1: sw flow control is enabled on output queue:
   * tty will stop sending if STOP char is received,
   * and resume sending if START is received, or if any char
   * is received and IXANY=1.
   */
  if flags & 1i32 << 1i32 != 0 {
    /* IXON=0: XON/XOFF chars are treated as normal chars (why we do this?) */
    /* dont convert CR to NL on input */
    (*newterm).c_iflag &= !(0o2000i32 | 0o400i32) as libc::c_uint
  }
  if flags & 1i32 << 2i32 != 0 {
    /* dont convert NL to CR+NL on output */
    (*newterm).c_oflag &= !0o4i32 as libc::c_uint
    /* Maybe clear more c_oflag bits? Usually, only OPOST and ONLCR are set.
     * OPOST  Enable output processing (reqd for OLCUC and *NL* bits to work)
     * OLCUC  Map lowercase characters to uppercase on output.
     * OCRNL  Map CR to NL on output.
     * ONOCR  Don't output CR at column 0.
     * ONLRET Don't output CR.
     */
  }
  if flags & 1i32 << 3i32 != 0 {
    /* IXOFF=0: disable sending XON/XOFF if input buf is full
     * IXON=0: input XON/XOFF chars are not special
     * BRKINT=0: dont send SIGINT on break
     * IMAXBEL=0: dont echo BEL on input line too long
     * INLCR,ICRNL,IUCLC: dont convert anything on input
     */
    (*newterm).c_iflag &=
      !(0o10000i32 | 0o2000i32 | 0o4000i32 | 0o2i32 | 0o100i32 | 0o400i32 | 0o1000i32 | 0o20000i32)
        as libc::c_uint
  }
  return r;
}
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
#[no_mangle]
pub unsafe extern "C" fn set_termios_to_raw(
  mut fd: libc::c_int,
  mut oldterm: *mut termios,
  mut flags: libc::c_int,
) -> libc::c_int {
  let mut newterm: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
  };
  get_termios_and_make_raw(fd, &mut newterm, oldterm, flags);
  return tcsetattr(fd, 0i32, &mut newterm);
}
#[no_mangle]
pub unsafe extern "C" fn safe_waitpid(
  mut pid: pid_t,
  mut wstat: *mut libc::c_int,
  mut options: libc::c_int,
) -> pid_t {
  let mut r: pid_t = 0;
  loop {
    r = waitpid(pid, wstat, options);
    if !(r == -1i32 && *bb_errno == 4i32) {
      break;
    }
  }
  return r;
}
#[no_mangle]
pub unsafe extern "C" fn wait_any_nohang(mut wstat: *mut libc::c_int) -> pid_t {
  return safe_waitpid(-1i32, wstat, 1i32);
}
// Wait for the specified child PID to exit, returning child's error return.
#[no_mangle]
pub unsafe extern "C" fn wait4pid(mut pid: pid_t) -> libc::c_int {
  let mut status: libc::c_int = 0;
  if pid <= 0i32 {
    /*errno = ECHILD; -- wrong. */
    /* we expect errno to be already set from failed [v]fork/exec */
    return -1i32;
  }
  if safe_waitpid(pid, &mut status, 0i32) == -1i32 {
    return -1i32;
  }
  if status & 0x7fi32 == 0i32 {
    return (status & 0xff00i32) >> 8i32;
  }
  if ((status & 0x7fi32) + 1i32) as libc::c_schar as libc::c_int >> 1i32 > 0i32 {
    return (status & 0x7fi32) + 0x180i32;
  }
  return 0i32;
}
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
// Useful when we do know that pid is valid, and we just want to wait
// for it to exit. Not existing pid is fatal. waitpid() status is not returned.
#[no_mangle]
pub unsafe extern "C" fn wait_for_exitstatus(mut pid: pid_t) -> libc::c_int {
  let mut exit_status: libc::c_int = 0;
  let mut n: libc::c_int = 0;
  n = safe_waitpid(pid, &mut exit_status, 0i32);
  if n < 0i32 {
    bb_simple_perror_msg_and_die(b"waitpid\x00" as *const u8 as *const libc::c_char);
  }
  return exit_status;
}

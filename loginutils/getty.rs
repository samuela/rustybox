use crate::libbb::appletlib::applet_name;
use crate::libbb::ptr_to_globals::bb_errno;
use crate::librb::signal::__sighandler_t;
use crate::librb::size_t;
use crate::librb::smallint;
use libc;
use libc::alarm;
use libc::cc_t;
use libc::close;
use libc::fchmod;
use libc::getpid;
use libc::gid_t;
use libc::ioctl;
use libc::mode_t;
use libc::open;
use libc::openlog;
use libc::pid_t;
use libc::setsid;
use libc::sleep;
use libc::speed_t;
use libc::ssize_t;
use libc::tcflag_t;
use libc::termios;
use libc::uid_t;
use libc::useconds_t;
extern "C" {
  #[no_mangle]
  fn exit(_: libc::c_int) -> !;

  #[no_mangle]
  fn getsid(__pid: pid_t) -> pid_t;
  #[no_mangle]
  fn tcsetpgrp(__fd: libc::c_int, __pgrp_id: pid_t) -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;

  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
  #[no_mangle]
  static ptr_to_globals: *mut globals;
  #[no_mangle]
  fn _exit(_: libc::c_int) -> !;
  #[no_mangle]
  fn execlp(__file: *const libc::c_char, __arg: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn usleep(__useconds: useconds_t) -> libc::c_int;
  #[no_mangle]
  fn fchown(__fd: libc::c_int, __owner: uid_t, __group: gid_t) -> libc::c_int;

  #[no_mangle]
  fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;

  #[no_mangle]
  fn strsep(__stringp: *mut *mut libc::c_char, __delim: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn cfsetspeed(__termios_p: *mut termios, __speed: speed_t) -> libc::c_int;
  #[no_mangle]
  fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
  #[no_mangle]
  fn tcdrain(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn tcflush(__fd: libc::c_int, __queue_selector: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn tcgetsid(__fd: libc::c_int) -> pid_t;

  #[no_mangle]
  fn skip_dev_pfx(tty_name: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strcpy_and_process_escape_sequences(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn ndelay_off(fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xdup2(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn signal_no_SA_RESTART_empty_mask(
    sig: libc::c_int,
    handler: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
  );
  #[no_mangle]
  fn record_signo(signo: libc::c_int);
  #[no_mangle]
  fn xsetenv(key: *const libc::c_char, value: *const libc::c_char);
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn full_write1_str(str: *const libc::c_char) -> ssize_t;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  #[no_mangle]
  fn update_utmp(
    pid: pid_t,
    new_type: libc::c_int,
    tty_name: *const libc::c_char,
    username: *const libc::c_char,
    hostname: *const libc::c_char,
  );
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  static mut msg_eol: *const libc::c_char;
  #[no_mangle]
  static mut logmode: smallint;
  #[no_mangle]
  static mut die_func: Option<unsafe extern "C" fn() -> ()>;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn tty_value_to_baud(value: libc::c_uint) -> speed_t;
  #[no_mangle]
  fn print_login_issue(issue_file: *const libc::c_char, tty: *const libc::c_char);
  #[no_mangle]
  fn print_login_prompt();
  #[no_mangle]
  fn xmalloc_ttyname(fd: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn tcsetattr_stdin_TCSANOW(tp: *const termios) -> libc::c_int;

}

pub type sighandler_t = __sighandler_t;

pub type C2RustUnnamed = libc::c_uint;
pub const LOGMODE_BOTH: C2RustUnnamed = 3;
pub const LOGMODE_SYSLOG: C2RustUnnamed = 2;
pub const LOGMODE_STDIO: C2RustUnnamed = 1;
pub const LOGMODE_NONE: C2RustUnnamed = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub timeout: libc::c_uint,
  pub login: *const libc::c_char,
  pub fakehost: *const libc::c_char,
  pub tty_name: *const libc::c_char,
  pub initstring: *mut libc::c_char,
  pub issue: *const libc::c_char,
  pub numspeed: libc::c_int,
  pub speeds: [libc::c_int; 10],
  pub eol: libc::c_uchar,
  pub tty_attrs: termios,
  pub line_buf: [libc::c_char; 128],
}
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
/* -n */
/* convert speed string to speed code; return <= 0 on failure */
unsafe extern "C" fn bcode(mut s: *const libc::c_char) -> libc::c_int {
  let mut value: libc::c_int = bb_strtou(s, 0 as *mut *mut libc::c_char, 10i32) as libc::c_int; /* yes, int is intended! */
  if value < 0 {
    /* bad terminating char, overflow, etc */
    return value;
  }
  return tty_value_to_baud(value as libc::c_uint) as libc::c_int;
}
/* parse alternate baud rates */
unsafe extern "C" fn parse_speeds(mut arg: *mut libc::c_char) {
  let mut cp: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  /* NB: at least one iteration is always done */
  loop {
    cp = strsep(&mut arg, b",\x00" as *const u8 as *const libc::c_char);
    if cp.is_null() {
      break;
    }
    (*ptr_to_globals).speeds[(*ptr_to_globals).numspeed as usize] = bcode(cp);
    if (*ptr_to_globals).speeds[(*ptr_to_globals).numspeed as usize] < 0 {
      bb_error_msg_and_die(b"bad speed: %s\x00" as *const u8 as *const libc::c_char, cp);
    }
    /* note: arg "0" turns into speed B0 */
    (*ptr_to_globals).numspeed += 1;
    if (*ptr_to_globals).numspeed > 10i32 {
      bb_simple_error_msg_and_die(
        b"too many alternate speeds\x00" as *const u8 as *const libc::c_char,
      );
    }
  }
}
/* parse command-line arguments */
unsafe extern "C" fn parse_args(mut argv: *mut *mut libc::c_char) {
  let mut ts: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut flags: libc::c_int = 0;
  flags = getopt32(
    argv,
    b"^I:LH:f:hil:mt:+wn\x00-2\x00" as *const u8 as *const libc::c_char,
    &mut (*ptr_to_globals).initstring as *mut *mut libc::c_char,
    &mut (*ptr_to_globals).fakehost as *mut *const libc::c_char,
    &mut (*ptr_to_globals).issue as *mut *const libc::c_char,
    &mut (*ptr_to_globals).login as *mut *const libc::c_char,
    &mut (*ptr_to_globals).timeout as *mut libc::c_uint,
  ) as libc::c_int;
  if flags & 1i32 << 0 != 0 {
    (*ptr_to_globals).initstring = xstrdup((*ptr_to_globals).initstring);
    /* decode \ddd octal codes into chars */
    strcpy_and_process_escape_sequences((*ptr_to_globals).initstring, (*ptr_to_globals).initstring);
  }
  argv = argv.offset(optind as isize);
  /* We loosen up a bit and accept both "baudrate tty" and "tty baudrate" */
  (*ptr_to_globals).tty_name = *argv.offset(0); /* baud rate(s) */
  ts = *argv.offset(1);
  if (*(*argv.offset(0)).offset(0) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int
    <= 9i32
  {
    /* A number first, assume it's a speed (BSD style) */
    (*ptr_to_globals).tty_name = ts;
    ts = *argv.offset(0) /* tty name is in argv[1] */
    /* baud rate(s) */
  }
  parse_speeds(ts);
  if !(*argv.offset(2)).is_null() {
    xsetenv(
      b"TERM\x00" as *const u8 as *const libc::c_char,
      *argv.offset(2),
    );
  };
}
/* set up tty as standard input, output, error */
unsafe extern "C" fn open_tty() {
  /* Set up new standard input, unless we are given an already opened port */
  if *(*ptr_to_globals).tty_name.offset(0) as libc::c_int != '-' as i32
    || *(*ptr_to_globals).tty_name.offset(1) as libc::c_int != 0
  {
    if *(*ptr_to_globals).tty_name.offset(0) as libc::c_int != '/' as i32 {
      (*ptr_to_globals).tty_name = xasprintf(
        b"/dev/%s\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).tty_name,
      )
    } /* will leak it */
    /* crw--w---- */
    close(0i32);
    xopen((*ptr_to_globals).tty_name, 0o2i32 | 0o4000i32);
    fchown(0i32, 0 as uid_t, 0 as gid_t);
    fchmod(0i32, 0o620i32 as mode_t);
  } else {
    let mut n: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    /* Open the tty as standard input */
    /* uses fd 0 */
    /* Set proper protections and ownership */
    /* 0:0 */
    /*
     * Standard input should already be connected to an open port.
     * Make sure it is open for read/write.
     */
    if fcntl(0i32, 3i32) & (0o2i32 | 0 | 0o1i32) != 0o2i32 {
      bb_simple_error_msg_and_die(
        b"stdin is not open for read/write\x00" as *const u8 as *const libc::c_char,
      );
    }
    /* Try to get real tty name instead of "-" */
    n = xmalloc_ttyname(0i32);
    if !n.is_null() {
      (*ptr_to_globals).tty_name = n
    }
  }
  applet_name = xasprintf(
    b"getty: %s\x00" as *const u8 as *const libc::c_char,
    skip_dev_pfx((*ptr_to_globals).tty_name),
  );
}
unsafe extern "C" fn set_tty_attrs() {
  if tcsetattr_stdin_TCSANOW(&mut (*ptr_to_globals).tty_attrs) < 0 {
    bb_simple_perror_msg_and_die(b"tcsetattr\x00" as *const u8 as *const libc::c_char);
  };
}
/* We manipulate tty_attrs this way:
 * - first, we read existing tty_attrs
 * - init_tty_attrs modifies some parts and sets it
 * - auto_baud and/or BREAK processing can set different speed and set tty attrs
 * - finalize_tty_attrs again modifies some parts and sets tty attrs before
 *   execing login
 */
unsafe extern "C" fn init_tty_attrs(mut speed: libc::c_int) {
  /* Try to drain output buffer, with 5 sec timeout.
   * Added on request from users of ~600 baud serial interface
   * with biggish buffer on a 90MHz CPU.
   * They were losing hundreds of bytes of buffered output
   * on tcflush.
   */
  signal_no_SA_RESTART_empty_mask(
    14i32,
    Some(record_signo as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  alarm(5i32 as libc::c_uint);
  tcdrain(0i32);
  alarm(0i32 as libc::c_uint);
  /* Flush input and output queues, important for modems! */
  tcflush(0i32, 2i32);
  /* Set speed if it wasn't specified as "0" on command line */
  if speed != 0 {
    cfsetspeed(&mut (*ptr_to_globals).tty_attrs, speed as speed_t);
  }
  /* Initial settings: 8-bit characters, raw mode, blocking i/o.
   * Special characters are set after we have read the login name; all
   * reads will be done in raw mode anyway.
   */
  /* Clear all bits except: */
  (*ptr_to_globals).tty_attrs.c_cflag &= (0i32
    | 0o100i32
    | 0o400i32
    | 0o1000i32
    | 0o10000000000i32
    | 0o10017i32
    | 0o10000i32
    | 0o2003600000i32) as libc::c_uint;
  /* Set: 8 bits; hang up (drop DTR) on last close; enable receive */
  (*ptr_to_globals).tty_attrs.c_cflag |= (0o60i32 | 0o2000i32 | 0o200i32) as libc::c_uint;
  if option_mask32 & (1i32 << 1i32) as libc::c_uint != 0 {
    /* ignore Carrier Detect pin:
     * opens don't block when CD is low,
     * losing CD doesn't hang up processes whose ctty is this tty
     */
    (*ptr_to_globals).tty_attrs.c_cflag |= 0o4000i32 as libc::c_uint
  } /* flow control using RTS/CTS pins */
  if option_mask32 & (1i32 << 4i32) as libc::c_uint != 0 {
    (*ptr_to_globals).tty_attrs.c_cflag |= 0o20000000000u32
  }
  (*ptr_to_globals).tty_attrs.c_iflag = 0 as tcflag_t;
  (*ptr_to_globals).tty_attrs.c_lflag = 0 as tcflag_t;
  /* non-raw output; add CR to each NL */
  (*ptr_to_globals).tty_attrs.c_oflag = (0o1i32 | 0o4i32) as tcflag_t;
  /* reads will block only if < 1 char is available */
  (*ptr_to_globals).tty_attrs.c_cc[6] = 1i32 as cc_t;
  /* no timeout (reads block forever) */
  (*ptr_to_globals).tty_attrs.c_cc[5] = 0 as cc_t;
  (*ptr_to_globals).tty_attrs.c_line = 0 as cc_t;
  set_tty_attrs();
}
unsafe extern "C" fn finalize_tty_attrs() {
  /* software flow control on output (stop sending if XOFF is recvd);
   * and on input (send XOFF when buffer is full)
   */
  (*ptr_to_globals).tty_attrs.c_iflag |= (0o2000i32 | 0o10000i32) as libc::c_uint;
  if (*ptr_to_globals).eol as libc::c_int == '\r' as i32 {
    (*ptr_to_globals).tty_attrs.c_iflag |= 0o400i32 as libc::c_uint
    /* map CR on input to NL */
  }
  /* Other bits in c_iflag:
   * IXANY   Any recvd char enables output (any char is also a XON)
   * INPCK   Enable parity check
   * IGNPAR  Ignore parity errors (drop bad bytes)
   * PARMRK  Mark parity errors with 0xff, 0x00 prefix
   *         (else bad byte is received as 0x00)
   * ISTRIP  Strip parity bit
   * IGNBRK  Ignore break condition
   * BRKINT  Send SIGINT on break - maybe set this?
   * INLCR   Map NL to CR
   * IGNCR   Ignore CR
   * ICRNL   Map CR to NL
   * IUCLC   Map uppercase to lowercase
   * IMAXBEL Echo BEL on input line too long
   * IUTF8   Appears to affect tty's idea of char widths,
   *         observed to improve backspacing through Unicode chars
   */
  /* ICANON  line buffered input (NL or EOL or EOF chars end a line);
   * ISIG    recognize INT/QUIT/SUSP chars;
   * ECHO    echo input chars;
   * ECHOE   echo BS-SP-BS on erase character;
   * ECHOK   echo kill char specially, not as ^c (ECHOKE controls how exactly);
   * ECHOKE  erase all input via BS-SP-BS on kill char (else go to next line)
   * ECHOCTL Echo ctrl chars as ^c (else echo verbatim:
   *         e.g. up arrow emits "ESC-something" and thus moves cursor up!)
   */
  (*ptr_to_globals).tty_attrs.c_lflag |=
    (0o2i32 | 0o1i32 | 0o10i32 | 0o20i32 | 0o40i32 | 0o4000i32 | 0o1000i32) as libc::c_uint;
  /* Other bits in c_lflag:
   * XCASE   Map uppercase to \lowercase [tried, doesn't work]
   * ECHONL  Echo NL even if ECHO is not set
   * ECHOPRT On erase, echo erased chars
   *         [qwe<BS><BS><BS> input looks like "qwe\ewq/" on screen]
   * NOFLSH  Don't flush input buffer after interrupt or quit chars
   * IEXTEN  Enable extended functions (??)
   *         [glibc says it enables c_cc[LNEXT] "enter literal char"
   *         and c_cc[VDISCARD] "toggle discard buffered output" chars]
   * FLUSHO  Output being flushed (c_cc[VDISCARD] is in effect)
   * PENDIN  Retype pending input at next read or input char
   *         (c_cc[VREPRINT] is being processed)
   * TOSTOP  Send SIGTTOU for background output
   *         (why "stty sane" unsets this bit?)
   */
  (*ptr_to_globals).tty_attrs.c_cc[0] = ('C' as i32 ^ 0o100i32) as cc_t;
  (*ptr_to_globals).tty_attrs.c_cc[1] = ('\\' as i32 ^ 0o100i32) as cc_t;
  (*ptr_to_globals).tty_attrs.c_cc[4] = ('D' as i32 ^ 0o100i32) as cc_t;
  (*ptr_to_globals).tty_attrs.c_cc[11] = '\n' as i32 as cc_t;
  (*ptr_to_globals).tty_attrs.c_cc[7] = 0 as cc_t;
  (*ptr_to_globals).tty_attrs.c_cc[3] = ('U' as i32 ^ 0o100i32) as cc_t;
  /* Other control chars:
   * VEOL2
   * VERASE, VWERASE - (word) erase. we may set VERASE in get_logname
   * VREPRINT - reprint current input buffer
   * VLNEXT, VDISCARD, VSTATUS
   * VSUSP, VDSUSP - send (delayed) SIGTSTP
   * VSTART, VSTOP - chars used for IXON/IXOFF
   */
  set_tty_attrs();
  /* Now the newline character should be properly written */
  full_write(
    1i32,
    b"\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
    1i32 as size_t,
  );
}
/* extract baud rate from modem status message */
unsafe extern "C" fn auto_baud() {
  let mut nread: libc::c_int = 0;
  /*
   * This works only if the modem produces its status code AFTER raising
   * the DCD line, and if the computer is fast enough to set the proper
   * baud rate before the message has gone by. We expect a message of the
   * following format:
   *
   * <junk><number><junk>
   *
   * The number is interpreted as the baud rate of the incoming call. If the
   * modem does not tell us the baud rate within one second, we will keep
   * using the current baud rate. It is advisable to enable BREAK
   * processing (comma-separated list of baud rates) if the processing of
   * modem status messages is enabled.
   */
  (*ptr_to_globals).tty_attrs.c_cc[6] = 0 as cc_t; /* don't block reads (min read is 0 chars) */
  set_tty_attrs();
  /*
   * Wait for a while, then read everything the modem has said so far and
   * try to extract the speed of the dial-in call.
   */
  sleep(1i32 as libc::c_uint);
  nread = safe_read(
    0,
    (*ptr_to_globals).line_buf.as_mut_ptr() as *mut libc::c_void,
    (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong),
  ) as libc::c_int;
  if nread > 0 {
    let mut speed: libc::c_int = 0;
    let mut bp: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    (*ptr_to_globals).line_buf[nread as usize] = '\u{0}' as i32 as libc::c_char;
    bp = (*ptr_to_globals).line_buf.as_mut_ptr();
    while bp
      < (*ptr_to_globals)
        .line_buf
        .as_mut_ptr()
        .offset(nread as isize)
    {
      if (*bp as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
        speed = bcode(bp);
        if speed > 0 {
          cfsetspeed(&mut (*ptr_to_globals).tty_attrs, speed as speed_t);
        }
        break;
      } else {
        bp = bp.offset(1)
      }
    }
  }
  /* Restore terminal settings */
  (*ptr_to_globals).tty_attrs.c_cc[6] = 1i32 as cc_t; /* restore to value set by init_tty_attrs */
  set_tty_attrs();
}
/* get user name, establish parity, speed, erase, kill, eol;
 * return NULL on BREAK, logname on success
 */
unsafe extern "C" fn get_logname() -> *mut libc::c_char {
  let mut bp: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut c: libc::c_char = 0;
  /* Flush pending input (esp. after parsing or switching the baud rate) */
  usleep((100i32 * 1000i32) as useconds_t); /* 0.1 sec */
  tcflush(0i32, 0);
  loop
  /* Prompt for and read a login name */
  {
    if option_mask32 & (1i32 << 5i32) as libc::c_uint == 0 {
      print_login_issue((*ptr_to_globals).issue, (*ptr_to_globals).tty_name); /* while logname is empty */
    }
    print_login_prompt();
    /* Write issue file and prompt */
    /* Read name, watch for break, erase, kill, end-of-line */
    bp = (*ptr_to_globals).line_buf.as_mut_ptr(); /* end of get char loop */
    loop
    /* Do not report trivial EINTR/EIO errors */
    {
      *bb_errno = 4i32; /* make read of 0 bytes be silent too */
      if read(
        0,
        &mut c as *mut libc::c_char as *mut libc::c_void,
        1i32 as size_t,
      ) < 1
      {
        finalize_tty_attrs();
        if *bb_errno == 4i32 || *bb_errno == 5i32 {
          exit(0i32);
        }
        bb_simple_perror_msg_and_die(b"read error\x00" as *const u8 as *const libc::c_char);
      }
      match c as libc::c_int {
        13 | 10 => {
          *bp = '\u{0}' as i32 as libc::c_char;
          (*ptr_to_globals).eol = c as libc::c_uchar;
          break;
        }
        8 | 127 => {
          (*ptr_to_globals).tty_attrs.c_cc[2] = c as cc_t;
          if bp > (*ptr_to_globals).line_buf.as_mut_ptr() {
            full_write(
              1i32,
              b"\x08 \x08\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
              3i32 as size_t,
            );
            bp = bp.offset(-1)
          }
          continue;
        }
        21 => {
          while bp > (*ptr_to_globals).line_buf.as_mut_ptr() {
            full_write(
              1i32,
              b"\x08 \x08\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
              3i32 as size_t,
            );
            bp = bp.offset(-1)
          }
          continue;
        }
        3 | 4 => {
          finalize_tty_attrs();
          exit(0i32);
        }
        0 => {
          /* BREAK. If we have speeds to try,
           * return NULL (will switch speeds and return here) */
          if (*ptr_to_globals).numspeed > 1i32 {
            return std::ptr::null_mut::<libc::c_char>();
          }
        }
        _ => {}
      }
      /* fall through and ignore it */
      if !((c as libc::c_uchar as libc::c_int) < ' ' as i32) {
        if (bp.wrapping_offset_from((*ptr_to_globals).line_buf.as_mut_ptr()) as libc::c_long
          as libc::c_int as libc::c_ulong)
          < (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
        {
          /* echo and store the character */
          full_write(
            1i32,
            &mut c as *mut libc::c_char as *const libc::c_void,
            1i32 as size_t,
          ); /* default login program */
          let fresh0 = bp; /* default issue file */
          bp = bp.offset(1);
          *fresh0 = c
        }
      }
    }
    if !((*ptr_to_globals).line_buf[0] as libc::c_int == '\u{0}' as i32) {
      break;
    }
  }
  return (*ptr_to_globals).line_buf.as_mut_ptr();
}
unsafe extern "C" fn alarm_handler(mut _sig: libc::c_int) {
  finalize_tty_attrs();
  _exit(0i32);
}
unsafe extern "C" fn sleep10() {
  sleep(10i32 as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn getty_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut n: libc::c_int = 0;
  let mut pid: pid_t = 0;
  let mut tsid: pid_t = 0;
  let mut logname: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let ref mut fresh1 = *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
    as *mut *mut globals);
  *fresh1 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  (*ptr_to_globals).login = b"/bin/login\x00" as *const u8 as *const libc::c_char;
  (*ptr_to_globals).issue = b"/etc/issue\x00" as *const u8 as *const libc::c_char;
  (*ptr_to_globals).eol = '\r' as i32 as libc::c_uchar;
  /* Parse command-line arguments */
  parse_args(argv);
  /* Create new session and pgrp, lose controlling tty */
  pid = setsid(); /* this also gives us our pid :) */
  if pid < 0 {
    let mut fd: libc::c_int = 0;
    /* :(
     * docs/ctty.htm says:
     * "This is allowed only when the current process
     *  is not a process group leader".
     * Thus, setsid() will fail if we _already_ are
     * a session leader - which is quite possible for getty!
     */
    pid = getpid();
    if getsid(0i32) != pid {
      //for debugging:
      //bb_perror_msg_and_die("setsid failed:"
      //	" pid %d ppid %d"
      //	" sid %d pgid %d",
      //	pid, getppid(),
      //	getsid(0), getpgid(0));
      bb_simple_perror_msg_and_die(b"setsid\x00" as *const u8 as *const libc::c_char);
      /*
       * When we can end up here?
       * Example: setsid() fails when run alone in interactive shell:
       *  # getty 115200 /dev/tty2
       * because shell's child (getty) is put in a new process group.
       * But doesn't fail if shell is not interactive
       * (and therefore doesn't create process groups for pipes),
       * or if getty is not the first process in the process group:
       *  # true | getty 115200 /dev/tty2
       */
    }
    /* Looks like we are already a session leader.
     * In this case (setsid failed) we may still have ctty,
     * and it may be different from tty we need to control!
     * If we still have ctty, on Linux ioctl(TIOCSCTTY)
     * (which we are going to use a bit later) always fails -
     * even if we try to take ctty which is already ours!
     * Try to drop old ctty now to prevent that.
     * Use O_NONBLOCK: old ctty may be a serial line.
     */
    fd = open(
      b"/dev/tty\x00" as *const u8 as *const libc::c_char,
      0o2i32 | 0o4000i32,
    );
    if fd >= 0 {
      /* TIOCNOTTY sends SIGHUP to the foreground
       * process group - which may include us!
       * Make sure to not die on it:
       */
      let mut old: sighandler_t = signal(
        1i32,
        ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
      );
      ioctl(fd, 0x5422i32 as libc::c_ulong);
      close(fd);
      signal(1i32, old);
    }
  }
  /* Close stdio, and stray descriptors, just in case */
  n = xopen(b"/dev/null\x00" as *const u8 as *const libc::c_char, 0o2i32);
  /* dup2(n, 0); - no, we need to handle "getty - 9600" too */
  xdup2(n, 1i32);
  xdup2(n, 2i32);
  while n > 2i32 {
    let fresh2 = n;
    n = n - 1;
    close(fresh2);
  }
  /* Logging. We want special flavor of error_msg_and_die */
  die_func = Some(sleep10 as unsafe extern "C" fn() -> ());
  msg_eol = b"\r\n\x00" as *const u8 as *const libc::c_char;
  /* most likely will internally use fd #3 in CLOEXEC mode: */
  openlog(applet_name, 0x1i32, 4i32 << 3i32);
  logmode = LOGMODE_BOTH as libc::c_int as smallint;
  /* Open the tty as standard input, if it is not "-" */
  open_tty();
  ndelay_off(0i32);
  xdup2(0i32, 1i32);
  xdup2(0i32, 2i32);
  /* Steal ctty if we don't have it yet */
  tsid = tcgetsid(0i32);
  if tsid < 0 || pid != tsid {
    if ioctl(0i32, 0x540ei32 as libc::c_ulong, 1) < 0 {
      bb_simple_perror_msg_and_die(b"TIOCSCTTY\x00" as *const u8 as *const libc::c_char);
    }
  }
  /* Make ourself a foreground process group within our session */
  if tcsetpgrp(0i32, pid) < 0 {
    bb_simple_perror_msg_and_die(b"tcsetpgrp\x00" as *const u8 as *const libc::c_char);
  }
  /*
   * The following ioctl will fail if stdin is not a tty, but also when
   * there is noise on the modem control lines. In the latter case, the
   * common course of action is (1) fix your cables (2) give the modem more
   * time to properly reset after hanging up. SunOS users can achieve (2)
   * by patching the SunOS kernel variable "zsadtrlow" to a larger value;
   * 5 seconds seems to be a good value.
   */
  if tcgetattr(0i32, &mut (*ptr_to_globals).tty_attrs) < 0 {
    bb_simple_perror_msg_and_die(b"tcgetattr\x00" as *const u8 as *const libc::c_char);
  }
  /* Update the utmp file. This tty is ours now! */
  update_utmp(
    pid,
    6i32,
    (*ptr_to_globals).tty_name,
    b"LOGIN\x00" as *const u8 as *const libc::c_char,
    (*ptr_to_globals).fakehost,
  );
  /* Initialize tty attrs (raw mode, eight-bit, blocking i/o) */
  init_tty_attrs((*ptr_to_globals).speeds[0]);
  /* Write the modem init string and DON'T flush the buffers */
  if option_mask32 & (1i32 << 0) as libc::c_uint != 0 {
    full_write1_str((*ptr_to_globals).initstring);
  }
  /* Optionally detect the baud rate from the modem status message */
  if option_mask32 & (1i32 << 7i32) as libc::c_uint != 0 {
    auto_baud();
  }
  /* Set the optional timer */
  signal(
    14i32,
    Some(alarm_handler as unsafe extern "C" fn(_: libc::c_int) -> ()),
  ); /* if 0, alarm is not set */
  alarm((*ptr_to_globals).timeout);
  /* Optionally wait for CR or LF before writing /etc/issue */
  if option_mask32 & (1i32 << 9i32) as libc::c_uint != 0 {
    let mut ch: libc::c_char = 0;
    while safe_read(
      0,
      &mut ch as *mut libc::c_char as *mut libc::c_void,
      1i32 as size_t,
    ) == 1
    {
      if ch as libc::c_int == '\n' as i32 || ch as libc::c_int == '\r' as i32 {
        break;
      }
    }
  }
  logname = std::ptr::null_mut::<libc::c_char>();
  if option_mask32 & (1i32 << 10i32) as libc::c_uint == 0 {
    /* NB: init_tty_attrs already set line speed
     * to G.speeds[0] */
    let mut baud_index: libc::c_int = 0;
    loop
    /* Read the login name */
    {
      logname = get_logname();
      if !logname.is_null() {
        break;
      }
      /* We are here only if G.numspeed > 1 */
      baud_index = (baud_index + 1i32) % (*ptr_to_globals).numspeed;
      cfsetspeed(
        &mut (*ptr_to_globals).tty_attrs,
        (*ptr_to_globals).speeds[baud_index as usize] as speed_t,
      );
      set_tty_attrs();
    }
  }
  /* Disable timer */
  alarm(0i32 as libc::c_uint);
  finalize_tty_attrs();
  /* Let the login program take care of password validation */
  /* We use PATH because we trust that root doesn't set "bad" PATH,
   * and getty is not suid-root applet */
  /* With -n, logname == NULL, and login will ask for username instead */
  execlp(
    (*ptr_to_globals).login,
    (*ptr_to_globals).login,
    b"--\x00" as *const u8 as *const libc::c_char,
    logname,
    std::ptr::null_mut::<libc::c_char>(),
  );
  bb_error_msg_and_die(
    b"can\'t execute \'%s\'\x00" as *const u8 as *const libc::c_char,
    (*ptr_to_globals).login,
  );
}

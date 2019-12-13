use crate::librb::signal::sigaction;
use crate::librb::size_t;
use libc;
use libc::alarm;
use libc::free;
use libc::ssize_t;
use libc::termios;
use libc::FILE;
extern "C" {

  #[no_mangle]
  fn sigaction(__sig: libc::c_int, __act: *const sigaction, __oact: *mut sigaction) -> libc::c_int;

  #[no_mangle]
  static mut stdout: *mut FILE;

  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;

  #[no_mangle]
  fn tcsetattr(
    __fd: libc::c_int,
    __optional_actions: libc::c_int,
    __termios_p: *const termios,
  ) -> libc::c_int;

  #[no_mangle]
  fn tcflush(__fd: libc::c_int, __queue_selector: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn sigaction_set(sig: libc::c_int, act: *const sigaction) -> libc::c_int;

  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn fflush_all() -> libc::c_int;

  #[no_mangle]
  fn nuke_str(str: *mut libc::c_char);
}

/* do nothing signal handler */
unsafe extern "C" fn askpass_timeout(mut _ignore: libc::c_int) {}

#[no_mangle]
pub unsafe extern "C" fn bb_ask_noecho(
  mut fd: libc::c_int,
  mut timeout: libc::c_int,
  mut prompt: *const libc::c_char,
) -> *mut libc::c_char {
  let mut ret: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut i: libc::c_int = 0;
  let mut sa: sigaction = std::mem::zeroed();
  let mut oldsa: sigaction = std::mem::zeroed();
  let mut tio: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
  };
  let mut oldtio: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
  };
  tcflush(fd, 0i32);
  /* Was buggy: was printing prompt *before* flushing input,
   * which was upsetting "expect" based scripts of some users.
   */
  fputs_unlocked(prompt, stdout);
  fflush_all();
  tcgetattr(fd, &mut oldtio);
  tio = oldtio;
  /* Switch off echo. ECHOxyz meaning:
   * ECHO    echo input chars
   * ECHOE   echo BS-SP-BS on erase character
   * ECHOK   echo kill char specially, not as ^c (ECHOKE controls how exactly)
   * ECHOKE  erase all input via BS-SP-BS on kill char (else go to next line)
   * ECHOCTL Echo ctrl chars as ^c (else echo verbatim:
   *         e.g. up arrow emits "ESC-something" and thus moves cursor up!)
   * ECHONL  Echo NL even if ECHO is not set
   * ECHOPRT On erase, echo erased chars
   *         [qwe<BS><BS><BS> input looks like "qwe\ewq/" on screen]
   */
  tio.c_lflag &= !(0o10i32 | 0o20i32 | 0o40i32 | 0o100i32) as libc::c_uint;
  tcsetattr(fd, 0i32, &mut tio);
  memset(
    &mut sa as *mut sigaction as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<sigaction>() as libc::c_ulong,
  );
  /* sa.sa_flags = 0; - no SA_RESTART! */
  /* SIGINT and SIGALRM will interrupt reads below */
  sa.__sigaction_handler.sa_handler =
    Some(askpass_timeout as unsafe extern "C" fn(_: libc::c_int) -> ());
  sigaction(2i32, &mut sa, &mut oldsa);
  if timeout != 0 {
    sigaction_set(14i32, &mut sa);
    alarm(timeout as libc::c_uint);
  }
  ret = std::ptr::null_mut::<libc::c_char>();
  i = 0i32;
  loop {
    let mut r: libc::c_int = 0;
    /* User input is uber-slow, no need to optimize reallocs.
     * Grow it on every char.
     */
    ret = xrealloc(ret as *mut libc::c_void, (i + 2i32) as size_t) as *mut libc::c_char;
    r = read(
      fd,
      &mut *ret.offset(i as isize) as *mut libc::c_char as *mut libc::c_void,
      1i32 as size_t,
    ) as libc::c_int;
    if i == 0i32 && r == 0i32 || r < 0i32 {
      /* read is interrupted by timeout or ^C */
      *ret.offset(i as isize) = '\u{0}' as i32 as libc::c_char; /* paranoia */
      nuke_str(ret); /* paranoia */
      free(ret as *mut libc::c_void);
      ret = std::ptr::null_mut::<libc::c_char>();
      break;
    } else {
      if !(r == 0i32
        || *ret.offset(i as isize) as libc::c_int == '\r' as i32
        || *ret.offset(i as isize) as libc::c_int == '\n' as i32
        || {
          i += 1;
          (i) == 0xfffi32
        })
      {
        continue;
      }
      /* line limit */
      *ret.offset(i as isize) = '\u{0}' as i32 as libc::c_char;
      break;
    }
  }
  if timeout != 0 {
    alarm(0i32 as libc::c_uint);
  }
  sigaction_set(2i32, &mut oldsa);
  tcsetattr(fd, 0i32, &mut oldtio);
  bb_putchar('\n' as i32);
  fflush_all();
  return ret;
}

/* Like bb_ask_noecho, but asks on stdin with no timeout.  */
#[no_mangle]
pub unsafe extern "C" fn bb_ask_noecho_stdin(mut prompt: *const libc::c_char) -> *mut libc::c_char {
  return bb_ask_noecho(0i32, 0i32, prompt);
}

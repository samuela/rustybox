use crate::librb::signal::__sighandler_t;
use crate::librb::size_t;
use crate::librb::smallint;
use libc;



use libc::closelog;
use libc::dup2;

use libc::getenv;


use libc::getpid;

use libc::kill;
use libc::openlog;



use libc::sleep;


use libc::strcpy;


use libc::syslog;






use libc::open;




use libc::sprintf;

use libc::strcmp;




use crate::librb::signal::sigaction;
use libc::cc_t;
use libc::close;
use libc::free;
use libc::pid_t;
use libc::sigset_t;
use libc::sigval;
use libc::ssize_t;
use libc::tcflag_t;
use libc::termios;
use libc::uid_t;
use libc::FILE;

extern "C" {
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;

  #[no_mangle]
  fn execvp(__file: *const libc::c_char, __argv: *const *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn _exit(_: libc::c_int) -> !;

  #[no_mangle]
  fn setsid() -> pid_t;
  #[no_mangle]
  fn fork() -> pid_t;
  #[no_mangle]
  fn vfork() -> libc::c_int;
  #[no_mangle]
  fn sync();

  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;

  #[no_mangle]
  fn sigfillset(__set: *mut sigset_t) -> libc::c_int;
  #[no_mangle]
  fn sigdelset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn vsnprintf(
    _: *mut libc::c_char,
    _: libc::c_ulong,
    _: *const libc::c_char,
    _: ::std::ffi::VaList,
  ) -> libc::c_int;

  #[no_mangle]
  fn putenv(__string: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn strsep(__stringp: *mut *mut libc::c_char, __delim: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
  #[no_mangle]
  fn setrlimit(__resource: __rlimit_resource_t, __rlimits: *const rlimit) -> libc::c_int;
  #[no_mangle]
  fn wait(__stat_loc: *mut libc::c_int) -> pid_t;
  #[no_mangle]
  fn waitpid(__pid: pid_t, __stat_loc: *mut libc::c_int, __options: libc::c_int) -> pid_t;
  #[no_mangle]
  fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn skip_dev_pfx(tty_name: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn device_open(device: *const libc::c_char, mode: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn bb_signals(sigs: libc::c_int, f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>);
  #[no_mangle]
  fn bb_signals_recursive_norestart(
    sigs: libc::c_int,
    f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
  );
  #[no_mangle]
  fn sigaction_set(sig: libc::c_int, act: *const sigaction) -> libc::c_int;
  #[no_mangle]
  fn sigprocmask_allsigs(how: libc::c_int) -> libc::c_int;
  #[no_mangle]
  static mut bb_got_signal: smallint;
  #[no_mangle]
  fn record_signo(signo: libc::c_int);
  #[no_mangle]
  fn xchdir(path: *const libc::c_char);
  #[no_mangle]
  fn xsetenv(key: *const libc::c_char, value: *const libc::c_char);
  #[no_mangle]
  fn safe_strncpy(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    size: size_t,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn update_utmp_DEAD_PROCESS(pid: pid_t);
  #[no_mangle]
  fn wait_any_nohang(wstat: *mut libc::c_int) -> pid_t;
  #[no_mangle]
  fn bb_sanitize_stdio();
  #[no_mangle]
  static mut die_func: Option<unsafe extern "C" fn() -> ()>;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn config_open2(
    filename: *const libc::c_char,
    fopen_func: Option<unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE>,
  ) -> *mut parser_t;
  /* delims[0] is a comment char (use '\0' to disable), the rest are token delimiters */
  #[no_mangle]
  fn config_read(
    parser: *mut parser_t,
    tokens: *mut *mut libc::c_char,
    flags: libc::c_uint,
    delims: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  fn config_close(parser: *mut parser_t);
  /* Concatenate path and filename to new allocated buffer.
   * Add "/" only as needed (no duplicate "//" are produced).
   * If path is NULL, it is assumed to be "/".
   * filename should not be NULL. */
  #[no_mangle]
  fn concat_path_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn nuke_str(str: *mut libc::c_char);
  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn tcsetattr_stdin_TCSANOW(tp: *const termios) -> libc::c_int;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  /* allow default system PATH to be extended via CFLAGS */
  #[no_mangle]
  static bb_PATH_root_path: [libc::c_char; 0];
  /* At least gcc 3.4.6 on mipsel system needs optimization barrier */
  /* You can change LIBBB_DEFAULT_LOGIN_SHELL, but don't use it,
   * use bb_default_login_shell and following defines.
   * If you change LIBBB_DEFAULT_LOGIN_SHELL,
   * don't forget to change increment constant. */
  #[no_mangle]
  static bb_default_login_shell: [libc::c_char; 0];
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];

  #[no_mangle]
  fn reboot(__howto: libc::c_int) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
  pub gp_offset: libc::c_uint,
  pub fp_offset: libc::c_uint,
  pub overflow_arg_area: *mut libc::c_void,
  pub reg_save_area: *mut libc::c_void,
}

pub type __rlim64_t = libc::c_ulong;

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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub _pad: [libc::c_int; 28],
  pub _kill: C2RustUnnamed_8,
  pub _timer: C2RustUnnamed_7,
  pub _rt: C2RustUnnamed_6,
  pub _sigchld: C2RustUnnamed_5,
  pub _sigfault: C2RustUnnamed_2,
  pub _sigpoll: C2RustUnnamed_1,
  pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
  pub _call_addr: *mut libc::c_void,
  pub _syscall: libc::c_int,
  pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
  pub si_band: libc::c_long,
  pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
  pub si_addr: *mut libc::c_void,
  pub si_addr_lsb: libc::c_short,
  pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
  pub _addr_bnd: C2RustUnnamed_4,
  pub _pkey: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
  pub _lower: *mut libc::c_void,
  pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
  pub si_pid: pid_t,
  pub si_uid: uid_t,
  pub si_status: libc::c_int,
  pub si_utime: libc::clock_t,
  pub si_stime: libc::clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
  pub si_pid: pid_t,
  pub si_uid: uid_t,
  pub si_sigval: sigval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
  pub si_tid: libc::c_int,
  pub si_overrun: libc::c_int,
  pub si_sigval: sigval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
  pub si_pid: pid_t,
  pub si_uid: uid_t,
}

pub type va_list = __builtin_va_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
  pub rlim_cur: rlim_t,
  pub rlim_max: rlim_t,
}
pub type __rlimit_resource_t = __rlimit_resource;

/*
 * Config file parser
 */
pub type C2RustUnnamed_10 = libc::c_uint;
// comments are recognized even if there is whitespace before
// ("line start><space><tab><space>#comment" is also comment, not only "line start>#comment")
// NORMAL is:
// * remove leading and trailing delimiters and collapse
//   multiple delimiters into one
// * warn and continue if less than mintokens delimiters found
// * grab everything into last token
// * comments are recognized even if they aren't the first char
pub const PARSE_NORMAL: C2RustUnnamed_10 = 4653056;
// delim[0] and delim[1] are two different allowed comment chars
// (so far, delim[0] will only work as comment char for full-line comment)
// (IOW: it works as if PARSE_EOL_COMMENTS is not set. sysctl applet is okay with this)
pub const PARSE_WS_COMMENTS: C2RustUnnamed_10 = 16777216;
// comments are recognized even if they aren't the first char
pub const PARSE_ALT_COMMENTS: C2RustUnnamed_10 = 8388608;
pub const PARSE_EOL_COMMENTS: C2RustUnnamed_10 = 4194304;
// die if < min tokens found
// keep a copy of current line
pub const PARSE_KEEP_COPY: C2RustUnnamed_10 = 2097152;
// last token takes entire remainder of the line
pub const PARSE_MIN_DIE: C2RustUnnamed_10 = 1048576;
// trim leading and trailing delimiters
// TODO: COLLAPSE and TRIM seem to always go in pair
pub const PARSE_GREEDY: C2RustUnnamed_10 = 262144;
// treat consecutive delimiters as one
pub const PARSE_TRIM: C2RustUnnamed_10 = 131072;
pub const PARSE_COLLAPSE: C2RustUnnamed_10 = 65536;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser_t {
  pub fp: *mut FILE,
  pub data: *mut libc::c_char,
  pub line: *mut libc::c_char,
  pub nline: *mut libc::c_char,
  pub line_alloc: size_t,
  pub nline_alloc: size_t,
  pub lineno: libc::c_int,
}
//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub init_action_list: *mut init_action,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct init_action {
  pub next: *mut init_action,
  pub pid: pid_t,
  pub action_type: u8,
  pub terminal: [libc::c_char; 32],
  pub command: [libc::c_char; 1],
}
pub type C2RustUnnamed_11 = libc::c_uint;
pub const L_CONSOLE: C2RustUnnamed_11 = 2;
pub const L_LOG: C2RustUnnamed_11 = 1;
/* Print a message to the specified device.
 * "where" may be bitwise-or'd from L_LOG | L_CONSOLE
 * NB: careful, we can be called after vfork!
 */
unsafe extern "C" fn message(
  mut where_0: libc::c_int,
  mut fmt: *const libc::c_char,
  mut args: ...
) {
  let mut arguments: ::std::ffi::VaListImpl;
  let mut l: libc::c_uint = 0;
  let mut msg: [libc::c_char; 128] = [0; 128];
  msg[0] = '\r' as i32 as libc::c_char;
  arguments = args.clone();
  l = (1i32
    + vsnprintf(
      msg.as_mut_ptr().offset(1),
      (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
        .wrapping_sub(2i32 as libc::c_ulong),
      fmt,
      arguments.as_va_list(),
    )) as libc::c_uint;
  if l as libc::c_ulong
    > (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
      .wrapping_sub(2i32 as libc::c_ulong)
  {
    l = (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
      .wrapping_sub(2i32 as libc::c_ulong) as libc::c_uint
  }
  msg[l as usize] = '\u{0}' as i32 as libc::c_char;
  if where_0 & L_LOG as libc::c_int != 0 {
    /* Log the message to syslogd */
    openlog(applet_name, 0i32, 3i32 << 3i32);
    /* don't print "\r" */
    syslog(
      6i32,
      b"%s\x00" as *const u8 as *const libc::c_char,
      msg.as_mut_ptr().offset(1),
    );
    closelog();
  }
  let fresh0 = l;
  l = l.wrapping_add(1);
  msg[fresh0 as usize] = '\n' as i32 as libc::c_char;
  msg[l as usize] = '\u{0}' as i32 as libc::c_char;
  if where_0 & L_CONSOLE as libc::c_int != 0 {
    /* Send console messages to console so people will see them. */
    full_write(2i32, msg.as_mut_ptr() as *const libc::c_void, l as size_t);
  };
}
unsafe extern "C" fn console_init() {
  let mut vtno: libc::c_int = 0;
  let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
  s = getenv(b"CONSOLE\x00" as *const u8 as *const libc::c_char);
  if s.is_null() {
    s = getenv(b"console\x00" as *const u8 as *const libc::c_char)
  }
  if !s.is_null() {
    let mut fd: libc::c_int = open(s, 0o2i32 | 0o4000i32 | 0o400i32);
    if fd >= 0i32 {
      dup2(fd, 0i32);
      dup2(fd, 1i32);
      xmove_fd(fd, 2i32);
    }
  } else {
    /* Make sure fd 0,1,2 are not closed
     * (so that they won't be used by future opens) */
    bb_sanitize_stdio();
    // Users report problems
    //		/* Make sure init can't be blocked by writing to stderr */
    //		fcntl(STDERR_FILENO, F_SETFL, fcntl(STDERR_FILENO, F_GETFL) | O_NONBLOCK);
  }
  s = getenv(b"TERM\x00" as *const u8 as *const libc::c_char);
  if ioctl(
    0i32,
    0x5600i32 as libc::c_ulong,
    &mut vtno as *mut libc::c_int,
  ) != 0i32
  {
    /* Not a linux terminal, probably serial console.
     * Force the TERM setting to vt102
     * if TERM is set to linux (the default) */
    if s.is_null() || strcmp(s, b"linux\x00" as *const u8 as *const libc::c_char) == 0i32 {
      putenv(b"TERM=vt102\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
    }
  } else if s.is_null() {
    putenv(b"TERM=linux\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
  };
}
/* Set terminal settings to reasonable defaults.
 * NB: careful, we can be called after vfork! */
unsafe extern "C" fn set_sane_term() {
  let mut tty: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
  };
  tcgetattr(0i32, &mut tty);
  /* set control chars */
  tty.c_cc[0] = 3i32 as cc_t; /* C-c */
  tty.c_cc[1] = 28i32 as cc_t; /* C-\ */
  tty.c_cc[2] = 127i32 as cc_t; /* C-? */
  tty.c_cc[3] = 21i32 as cc_t; /* C-u */
  tty.c_cc[4] = 4i32 as cc_t; /* C-d */
  tty.c_cc[8] = 17i32 as cc_t; /* C-q */
  tty.c_cc[9] = 19i32 as cc_t; /* C-s */
  tty.c_cc[10] = 26i32 as cc_t; /* C-z */
  /* use line discipline 0 */
  tty.c_line = 0i32 as cc_t;
  /* Make it be sane */
  /* added CRTSCTS to fix Debian bug 528560 */
  tty.c_cflag &= (0o10017i32 | 0o10000i32 | 0o60i32 | 0o100i32 | 0o400i32 | 0o1000i32)
    as libc::c_uint
    | 0o20000000000u32;
  tty.c_cflag |= (0o200i32 | 0o2000i32 | 0o4000i32) as libc::c_uint;
  /* input modes */
  tty.c_iflag = (0o400i32 | 0o2000i32 | 0o10000i32) as tcflag_t;
  /* output modes */
  tty.c_oflag = (0o1i32 | 0o4i32) as tcflag_t;
  /* local modes */
  tty.c_lflag =
    (0o1i32 | 0o2i32 | 0o10i32 | 0o20i32 | 0o40i32 | 0o1000i32 | 0o4000i32 | 0o100000i32)
      as tcflag_t;
  tcsetattr_stdin_TCSANOW(&mut tty);
}
/* Open the new terminal device.
 * NB: careful, we can be called after vfork! */
unsafe extern "C" fn open_stdio_to_tty(mut tty_name: *const libc::c_char) -> libc::c_int {
  /* empty tty_name means "use init's tty", else... */
  if *tty_name.offset(0) != 0 {
    let mut fd: libc::c_int = 0;
    close(0i32);
    /* fd can be only < 0 or 0: */
    fd = device_open(tty_name, 0o2i32);
    if fd != 0 {
      message(
        L_LOG as libc::c_int | L_CONSOLE as libc::c_int,
        b"can\'t open %s: %m\x00" as *const u8 as *const libc::c_char,
        tty_name,
      );
      return 0i32;
      /* failure */
    }
    dup2(0i32, 1i32);
    dup2(0i32, 2i32);
  }
  set_sane_term();
  return 1i32;
  /* success */
}
unsafe extern "C" fn reset_sighandlers_and_unblock_sigs() {
  bb_signals(
    0i32
      + (1i32 << 10i32)
      + (1i32 << 12i32)
      + (1i32 << 15i32)
      + (1i32 << 3i32)
      + (1i32 << 2i32)
      + (1i32 << 1i32)
      + (1i32 << 20i32)
      + (1i32 << 19i32),
    None,
  );
  sigprocmask_allsigs(1i32);
}
/* Wrapper around exec:
 * Takes string.
 * If chars like '>' detected, execs '[-]/bin/sh -c "exec ......."'.
 * Otherwise splits words on whitespace, deals with leading dash,
 * and uses plain exec().
 * NB: careful, we can be called after vfork!
 */
unsafe extern "C" fn init_exec(mut command: *const libc::c_char) {
  /* +8 allows to write VLA sizes below more efficiently: */
  let mut command_size: libc::c_uint =
    strlen(command).wrapping_add(8i32 as libc::c_ulong) as libc::c_uint;
  /* strlen(command) + strlen("exec ")+1: */
  let vla = command_size as usize;
  let mut buf: Vec<libc::c_char> = ::std::vec::from_elem(0, vla);
  /* strlen(command) / 2 + 4: */
  let vla_0 = command_size.wrapping_div(2i32 as libc::c_uint) as usize;
  let mut cmd: Vec<*mut libc::c_char> = ::std::vec::from_elem(0 as *mut libc::c_char, vla_0);
  let mut dash: libc::c_int = 0;
  dash = (*command.offset(0) as libc::c_int == '-' as i32) as libc::c_int;
  command = command.offset(dash as isize);
  /* See if any special /bin/sh requiring characters are present */
  if !strpbrk(
    command,
    b"~`!$^&*()=|\\{}[];\"\'<>?\x00" as *const u8 as *const libc::c_char,
  )
  .is_null()
  {
    sprintf(
      buf.as_mut_ptr(),
      b"exec %s\x00" as *const u8 as *const libc::c_char,
      command,
    ); /* excluding "-" */
    /* NB: LIBBB_DEFAULT_LOGIN_SHELL define has leading dash */
    let ref mut fresh1 = *cmd.as_mut_ptr().offset(0);
    *fresh1 = (b"-/bin/sh\x00" as *const u8 as *const libc::c_char)
      .offset((dash == 0) as libc::c_int as isize) as *mut libc::c_char;
    let ref mut fresh2 = *cmd.as_mut_ptr().offset(1);
    *fresh2 = b"-c\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    let ref mut fresh3 = *cmd.as_mut_ptr().offset(2);
    *fresh3 = buf.as_mut_ptr();
    let ref mut fresh4 = *cmd.as_mut_ptr().offset(3);
    *fresh4 = 0 as *mut libc::c_char;
    command = (b"-/bin/sh\x00" as *const u8 as *const libc::c_char).offset(1)
  } else {
    /* Convert command (char*) into cmd (char**, one word per string) */
    let mut word: *mut libc::c_char = 0 as *mut libc::c_char; /* command including "-" */
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0i32;
    next = strcpy(buf.as_mut_ptr(), command.offset(-(dash as isize)));
    command = next.offset(dash as isize);
    loop {
      word = strsep(&mut next, b" \t\x00" as *const u8 as *const libc::c_char);
      if word.is_null() {
        break;
      }
      if *word as libc::c_int != '\u{0}' as i32 {
        /* not two spaces/tabs together? */
        let ref mut fresh5 = *cmd.as_mut_ptr().offset(i as isize);
        *fresh5 = word;
        i += 1
      }
    }
    let ref mut fresh6 = *cmd.as_mut_ptr().offset(i as isize);
    *fresh6 = 0 as *mut libc::c_char
  }
  /* If we saw leading "-", it is interactive shell.
   * Try harder to give it a controlling tty.
   */
  if 1i32 != 0 && dash != 0 {
    /* _Attempt_ to make stdin a controlling tty. */
    ioctl(0i32, 0x540ei32 as libc::c_ulong, 0i32);
  }
  /* Here command never contains the dash, cmd[0] might */
  execvp(command, cmd.as_mut_ptr() as *const *mut libc::c_char);
  message(
    L_LOG as libc::c_int | L_CONSOLE as libc::c_int,
    b"can\'t run \'%s\': %m\x00" as *const u8 as *const libc::c_char,
    command,
  );
  /* returns if execvp fails */
}
/* Used only by run_actions */
unsafe extern "C" fn run(mut a: *const init_action) -> pid_t {
  let mut pid: pid_t = 0;
  /* Careful: don't be affected by a signal in vforked child */
  sigprocmask_allsigs(0i32);
  if 1i32 != 0 && (*a).action_type as libc::c_int & 0x10i32 != 0 {
    pid = fork()
  } else {
    pid = vfork()
  }
  if pid < 0i32 {
    message(
      L_LOG as libc::c_int | L_CONSOLE as libc::c_int,
      b"can\'t fork\x00" as *const u8 as *const libc::c_char,
    );
  }
  if pid != 0 {
    sigprocmask_allsigs(1i32);
    return pid;
    /* Parent or error */
  }
  /* Child */
  /* Reset signal handlers that were set by the parent process */
  reset_sighandlers_and_unblock_sigs();
  /* Create a new session and make ourself the process group leader */
  setsid();
  /* Open the new terminal device */
  if open_stdio_to_tty((*a).terminal.as_ptr()) == 0 {
    _exit(1i32);
  }
  /* NB: on NOMMU we can't wait for input in child, so
   * "askfirst" will work the same as "respawn". */
  if 1i32 != 0 && (*a).action_type as libc::c_int & 0x10i32 != 0 {
    static mut press_enter: [libc::c_char; 47] = [
      10, 80, 108, 101, 97, 115, 101, 32, 112, 114, 101, 115, 115, 32, 69, 110, 116, 101, 114, 32,
      116, 111, 32, 97, 99, 116, 105, 118, 97, 116, 101, 32, 116, 104, 105, 115, 32, 99, 111, 110,
      115, 111, 108, 101, 46, 32, 0,
    ];
    let mut c: libc::c_char = 0;
    /*
     * Save memory by not exec-ing anything large (like a shell)
     * before the user wants it. This is critical if swap is not
     * enabled and the system has low memory. Generally this will
     * be run on the second virtual console, and the first will
     * be allowed to start a shell or whatever an init script
     * specifies.
     */
    full_write(
      1i32,
      press_enter.as_ptr() as *const libc::c_void,
      (::std::mem::size_of::<[libc::c_char; 47]>() as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong),
    );
    while safe_read(
      0i32,
      &mut c as *mut libc::c_char as *mut libc::c_void,
      1i32 as size_t,
    ) == 1
      && c as libc::c_int != '\n' as i32
    {}
  }
  /*
   * When a file named /.init_enable_core exists, setrlimit is called
   * before processes are spawned to set core file size as unlimited.
   * This is for debugging only.  Don't use this is production, unless
   * you want core dumps lying about....
   */
  /* Log the process name and args */
  message(
    L_LOG as libc::c_int,
    b"starting pid %u, tty \'%s\': \'%s\'\x00" as *const u8 as *const libc::c_char,
    getpid(),
    (*a).terminal.as_ptr(),
    (*a).command.as_ptr(),
  );
  /* Now run it.  The new program will take over this PID,
   * so nothing further in init.c should be run. */
  init_exec((*a).command.as_ptr());
  /* We're still here?  Some error happened. */
  _exit(-1i32);
}
unsafe extern "C" fn mark_terminated(mut pid: pid_t) -> *mut init_action {
  let mut a: *mut init_action = 0 as *mut init_action;
  if pid > 0i32 {
    update_utmp_DEAD_PROCESS(pid);
    a = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).init_action_list;
    while !a.is_null() {
      if (*a).pid == pid {
        (*a).pid = 0i32;
        return a;
      }
      a = (*a).next
    }
  }
  return 0 as *mut init_action;
}
unsafe extern "C" fn waitfor(mut pid: pid_t) {
  /* waitfor(run(x)): protect against failed fork inside run() */
  if pid <= 0i32 {
    return;
  }
  loop
  /* Wait for any child (prevent zombies from exiting orphaned processes)
   * but exit the loop only when specified one has exited. */
  {
    let mut wpid: pid_t = wait(0 as *mut libc::c_int);
    mark_terminated(wpid);
    /* Unsafe. SIGTSTP handler might have wait'ed it already */
    /*if (wpid == pid) break;*/
    /* More reliable: */
    if kill(pid, 0i32) != 0 {
      break;
    }
  }
}
/* Run all commands of a particular type */
unsafe extern "C" fn run_actions(mut action_type: libc::c_int) {
  let mut a: *mut init_action = 0 as *mut init_action;
  a = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).init_action_list;
  while !a.is_null() {
    if !((*a).action_type as libc::c_int & action_type == 0) {
      if (*a).action_type as libc::c_int & (0x1i32 | 0x2i32 | 0x4i32 | 0x20i32 | 0x40i32) != 0 {
        let mut pid: pid_t = run(a);
        if (*a).action_type as libc::c_int & (0x1i32 | 0x2i32 | 0x20i32 | 0x40i32) != 0 {
          waitfor(pid);
        }
      }
      if (*a).action_type as libc::c_int & (0x8i32 | 0x10i32) != 0 {
        /* Only run stuff with pid == 0. If pid != 0,
         * it is already running
         */
        if (*a).pid == 0i32 {
          (*a).pid = run(a)
        }
      }
    }
    a = (*a).next
  }
}
unsafe extern "C" fn new_init_action(
  mut action_type: u8,
  mut command: *const libc::c_char,
  mut cons: *const libc::c_char,
) {
  let mut current_block: u64;
  let mut a: *mut init_action = 0 as *mut init_action;
  let mut nextp: *mut *mut init_action = 0 as *mut *mut init_action;
  /* Scenario:
   * old inittab:
   * ::shutdown:umount -a -r
   * ::shutdown:swapoff -a
   * new inittab:
   * ::shutdown:swapoff -a
   * ::shutdown:umount -a -r
   * On reload, we must ensure entries end up in correct order.
   * To achieve that, if we find a matching entry, we move it
   * to the end.
   */
  nextp = &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).init_action_list;
  loop {
    a = *nextp;
    if a.is_null() {
      current_block = 3276175668257526147;
      break;
    }
    /* Don't enter action if it's already in the list.
     * This prevents losing running RESPAWNs.
     */
    if strcmp((*a).command.as_mut_ptr(), command) == 0i32
      && strcmp((*a).terminal.as_mut_ptr(), cons) == 0i32
    {
      /* Remove from list */
      *nextp = (*a).next;
      /* Find the end of the list */
      while !(*nextp).is_null() {
        nextp = &mut (**nextp).next
      }
      (*a).next = 0 as *mut init_action;
      current_block = 2443472067725968818;
      break;
    } else {
      nextp = &mut (*a).next
    }
  }
  match current_block {
    3276175668257526147 => {
      a = xzalloc(
        (::std::mem::size_of::<init_action>() as libc::c_ulong).wrapping_add(strlen(command)),
      ) as *mut init_action
    }
    _ => {}
  }
  /* Append to the end of the list */
  *nextp = a;
  (*a).action_type = action_type;
  strcpy((*a).command.as_mut_ptr(), command);
  safe_strncpy(
    (*a).terminal.as_mut_ptr(),
    cons,
    ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
  );
}
/* NOTE that if CONFIG_FEATURE_USE_INITTAB is NOT defined,
 * then parse_inittab() simply adds in some default
 * actions (i.e., runs INIT_SCRIPT and then starts a pair
 * of "askfirst" shells).  If CONFIG_FEATURE_USE_INITTAB
 * _is_ defined, but /etc/inittab is missing, this
 * results in the same set of default behaviors.
 */
unsafe extern "C" fn parse_inittab() {
  let mut token: [*mut libc::c_char; 4] = [0 as *mut libc::c_char; 4];
  let mut parser: *mut parser_t = config_open2(
    b"/etc/inittab\x00" as *const u8 as *const libc::c_char,
    Some(fopen_for_read as unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE),
  );
  if parser.is_null() {
    /* No inittab file - set up some default behavior */
    /* Sysinit */
    new_init_action(
      0x1i32 as u8,
      b"/etc/init.d/rcS\x00" as *const u8 as *const libc::c_char,
      b"\x00" as *const u8 as *const libc::c_char,
    );
    /* Askfirst shell on tty1-4 */
    new_init_action(
      0x10i32 as u8,
      bb_default_login_shell.as_ptr(),
      b"\x00" as *const u8 as *const libc::c_char,
    );
    //TODO: VC_1 instead of ""? "" is console -> ctty problems -> angry users
    new_init_action(
      0x10i32 as u8,
      bb_default_login_shell.as_ptr(),
      b"/dev/tty2\x00" as *const u8 as *const libc::c_char,
    );
    new_init_action(
      0x10i32 as u8,
      bb_default_login_shell.as_ptr(),
      b"/dev/tty3\x00" as *const u8 as *const libc::c_char,
    );
    new_init_action(
      0x10i32 as u8,
      bb_default_login_shell.as_ptr(),
      b"/dev/tty4\x00" as *const u8 as *const libc::c_char,
    );
    /* Reboot on Ctrl-Alt-Del */
    new_init_action(
      0x20i32 as u8,
      b"reboot\x00" as *const u8 as *const libc::c_char,
      b"\x00" as *const u8 as *const libc::c_char,
    );
    /* Umount all filesystems on halt/reboot */
    new_init_action(
      0x40i32 as u8,
      b"umount -a -r\x00" as *const u8 as *const libc::c_char,
      b"\x00" as *const u8 as *const libc::c_char,
    );
    /* Swapoff on halt/reboot */
    new_init_action(
      0x40i32 as u8,
      b"swapoff -a\x00" as *const u8 as *const libc::c_char,
      b"\x00" as *const u8 as *const libc::c_char,
    );
    /* Restart init when a QUIT is received */
    new_init_action(
      0x80i32 as u8,
      b"init\x00" as *const u8 as *const libc::c_char,
      b"\x00" as *const u8 as *const libc::c_char,
    );
    return;
  }
  /* optional_tty:ignored_runlevel:action:command
   * Delims are not to be collapsed and need exactly 4 tokens
   */
  while config_read(
    parser,
    token.as_mut_ptr(),
    (PARSE_NORMAL as libc::c_int & !(PARSE_TRIM as libc::c_int | PARSE_COLLAPSE as libc::c_int)
      | (0i32 & 0xffi32) << 8i32
      | 4i32 & 0xffi32) as libc::c_uint,
    b"#:\x00" as *const u8 as *const libc::c_char,
  ) != 0
  {
    /* order must correspond to SYSINIT..RESTART constants */
    static mut actions: [libc::c_char; 64] = [
      115, 121, 115, 105, 110, 105, 116, 0, 119, 97, 105, 116, 0, 111, 110, 99, 101, 0, 114, 101,
      115, 112, 97, 119, 110, 0, 97, 115, 107, 102, 105, 114, 115, 116, 0, 99, 116, 114, 108, 97,
      108, 116, 100, 101, 108, 0, 115, 104, 117, 116, 100, 111, 119, 110, 0, 114, 101, 115, 116,
      97, 114, 116, 0, 0,
    ];
    let mut action: libc::c_int = 0;
    let mut tty: *mut libc::c_char = token[0];
    if !token[3].is_null() {
      action = index_in_strings(actions.as_ptr(), token[2]);
      if !(action < 0i32 || *token[3].offset(0) == 0) {
        /* turn .*TTY -> /dev/TTY */
        if *tty.offset(0) != 0 {
          tty = concat_path_file(
            b"/dev/\x00" as *const u8 as *const libc::c_char,
            skip_dev_pfx(tty),
          )
        }
        new_init_action((1i32 << action) as u8, token[3], tty);
        if *tty.offset(0) != 0 {
          free(tty as *mut libc::c_void);
        }
        continue;
      }
    }
    /* token[3]: command */
    message(
      L_LOG as libc::c_int | L_CONSOLE as libc::c_int,
      b"Bad inittab entry at line %d\x00" as *const u8 as *const libc::c_char,
      (*parser).lineno,
    );
  }
  config_close(parser);
}
unsafe extern "C" fn pause_and_low_level_reboot(mut magic: libc::c_uint) -> ! {
  let mut pid: pid_t = 0;
  /* Allow time for last message to reach serial console, etc */
  sleep(1i32 as libc::c_uint);
  /* We have to fork here, since the kernel calls do_exit(EXIT_SUCCESS)
   * in linux/kernel/sys.c, which can cause the machine to panic when
   * the init process exits... */
  pid = vfork();
  if pid == 0i32 {
    /* child */
    reboot(magic as libc::c_int);
    _exit(0i32);
  }
  /* Used to have "while (1) sleep(1)" here.
   * However, in containers reboot() call is ignored, and with that loop
   * we would eternally sleep here - not what we want.
   */
  waitpid(pid, 0 as *mut libc::c_int, 0i32); /* paranoia */
  sleep(1i32 as libc::c_uint);
  _exit(0i32);
}
unsafe extern "C" fn run_shutdown_and_kill_processes() {
  /* Run everything to be run at "shutdown".  This is done _prior_
   * to killing everything, in case people wish to use scripts to
   * shut things down gracefully... */
  run_actions(0x40i32);
  message(
    L_CONSOLE as libc::c_int | L_LOG as libc::c_int,
    b"The system is going down NOW!\x00" as *const u8 as *const libc::c_char,
  );
  /* Send signals to every process _except_ pid 1 */
  kill(-1i32, 15i32);
  message(
    L_CONSOLE as libc::c_int,
    b"Sent SIG%s to all processes\x00" as *const u8 as *const libc::c_char,
    b"TERM\x00" as *const u8 as *const libc::c_char,
  );
  sync();
  sleep(1i32 as libc::c_uint);
  kill(-1i32, 9i32);
  message(
    L_CONSOLE as libc::c_int,
    b"Sent SIG%s to all processes\x00" as *const u8 as *const libc::c_char,
    b"KILL\x00" as *const u8 as *const libc::c_char,
  );
  sync();
  /*sleep(1); - callers take care about making a pause */
}
/* Signal handling by init:
 *
 * For process with PID==1, on entry kernel sets all signals to SIG_DFL
 * and unmasks all signals. However, for process with PID==1,
 * default action (SIG_DFL) on any signal is to ignore it,
 * even for special signals SIGKILL and SIGCONT.
 * Also, any signal can be caught or blocked.
 * (but SIGSTOP is still handled specially, at least in 2.6.20)
 *
 * We install two kinds of handlers, "immediate" and "delayed".
 *
 * Immediate handlers execute at any time, even while, say, sysinit
 * is running.
 *
 * Delayed handlers just set a flag variable. The variable is checked
 * in the main loop and acted upon.
 *
 * halt/poweroff/reboot and restart have immediate handlers.
 * They only traverse linked list of struct action's, never modify it,
 * this should be safe to do even in signal handler. Also they
 * never return.
 *
 * SIGSTOP and SIGTSTP have immediate handlers. They just wait
 * for SIGCONT to happen.
 *
 * SIGHUP has a delayed handler, because modifying linked list
 * of struct action's from a signal handler while it is manipulated
 * by the program may be disastrous.
 *
 * Ctrl-Alt-Del has a delayed handler. Not a must, but allowing
 * it to happen even somewhere inside "sysinit" would be a bit awkward.
 *
 * There is a tiny probability that SIGHUP and Ctrl-Alt-Del will collide
 * and only one will be remembered and acted upon.
 */
/* The SIGPWR/SIGUSR[12]/SIGTERM handler */
unsafe extern "C" fn halt_reboot_pwoff(mut sig: libc::c_int) -> ! {
  let mut m: *const libc::c_char = 0 as *const libc::c_char;
  let mut rb: libc::c_uint = 0;
  /* We may call run() and it unmasks signals,
   * including the one masked inside this signal handler.
   * Testcase which would start multiple reboot scripts:
   *  while true; do reboot; done
   * Preventing it:
   */
  reset_sighandlers_and_unblock_sigs();
  run_shutdown_and_kill_processes();
  m = b"halt\x00" as *const u8 as *const libc::c_char;
  rb = 0xcdef0123u32;
  if sig == 15i32 {
    m = b"reboot\x00" as *const u8 as *const libc::c_char;
    rb = 0x1234567i32 as libc::c_uint
  } else if sig == 12i32 {
    m = b"poweroff\x00" as *const u8 as *const libc::c_char;
    rb = 0x4321fedci32 as libc::c_uint
  }
  message(
    L_CONSOLE as libc::c_int,
    b"Requesting system %s\x00" as *const u8 as *const libc::c_char,
    m,
  );
  pause_and_low_level_reboot(rb);
  /* not reached */
}
/* Handler for QUIT - exec "restart" action,
 * else (no such action defined) do nothing */
unsafe extern "C" fn exec_restart_action() {
  let mut a: *mut init_action = 0 as *mut init_action;
  a = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).init_action_list;
  while !a.is_null() {
    if (*a).action_type as libc::c_int & 0x80i32 == 0 {
      a = (*a).next
    } else {
      /* Starting from here, we won't return.
       * Thus don't need to worry about preserving errno
       * and such.
       */
      reset_sighandlers_and_unblock_sigs();
      run_shutdown_and_kill_processes();
      /* Allow Ctrl-Alt-Del to reboot the system.
       * This is how kernel sets it up for init, we follow suit.
       */
      reboot(0x89abcdefu32 as libc::c_int); /* misnomer */
      if open_stdio_to_tty((*a).terminal.as_mut_ptr()) != 0 {
        /* Theoretically should be safe.
         * But in practice, kernel bugs may leave
         * unkillable processes, and wait() may block forever.
         * Oh well. Hoping "new" init won't be too surprised
         * by having children it didn't create.
         */
        //while (wait(NULL) > 0)
        //	continue;
        init_exec((*a).command.as_mut_ptr());
      }
      /* Open or exec failed */
      pause_and_low_level_reboot(0xcdef0123u32);
    }
    /* not reached */
  }
}
/* The SIGSTOP/SIGTSTP handler
 * NB: inside it, all signals except SIGCONT are masked
 * via appropriate setup in sigaction().
 */
unsafe extern "C" fn stop_handler(mut _sig: libc::c_int) {
  let mut saved_bb_got_signal: smallint = 0;
  let mut saved_errno: libc::c_int = 0;
  saved_bb_got_signal = bb_got_signal;
  saved_errno = *bb_errno;
  signal(
    18i32,
    Some(record_signo as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  loop {
    let mut wpid: pid_t = 0;
    if bb_got_signal as libc::c_int == 18i32 {
      break;
    }
    /* NB: this can accidentally wait() for a process
     * which we waitfor() elsewhere! waitfor() must have
     * code which is resilient against this.
     */
    wpid = wait_any_nohang(0 as *mut libc::c_int);
    mark_terminated(wpid);
    sleep(1i32 as libc::c_uint);
  }
  signal(18i32, None);
  *bb_errno = saved_errno;
  bb_got_signal = saved_bb_got_signal;
}
unsafe extern "C" fn reload_inittab() {
  let mut a: *mut init_action = 0 as *mut init_action;
  let mut nextp: *mut *mut init_action = 0 as *mut *mut init_action;
  message(
    L_LOG as libc::c_int,
    b"reloading /etc/inittab\x00" as *const u8 as *const libc::c_char,
  );
  /* Disable old entries */
  a = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).init_action_list;
  while !a.is_null() {
    (*a).action_type = 0i32 as u8;
    a = (*a).next
  }
  /* Append new entries, or modify existing entries
   * (incl. setting a->action_type) if cmd and device name
   * match new ones. End result: only entries with
   * a->action_type == 0 are stale.
   */
  parse_inittab();
  /* Remove stale entries and SYSINIT entries.
   * We never rerun SYSINIT entries anyway,
   * removing them too saves a few bytes
   */
  nextp = &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).init_action_list;
  loop {
    a = *nextp;
    if a.is_null() {
      break;
    }
    /*
     * Why pid == 0 check?
     * Process can be removed from inittab and added *later*.
     * If we delete its entry but process still runs,
     * duplicate is spawned when the entry is re-added.
     */
    if (*a).action_type as libc::c_int & !0x1i32 == 0i32 && (*a).pid == 0i32 {
      *nextp = (*a).next;
      free(a as *mut libc::c_void);
    } else {
      nextp = &mut (*a).next
    }
  }
  /* Not needed: */
  /* run_actions(RESPAWN | ASKFIRST); */
  /* - we return to main loop, which does this automagically */
}
unsafe extern "C" fn check_delayed_sigs() -> libc::c_int {
  let mut sigs_seen: libc::c_int = 0i32;
  loop {
    let mut sig: smallint = bb_got_signal;
    if sig == 0 {
      return sigs_seen;
    }
    bb_got_signal = 0i32 as smallint;
    sigs_seen = 1i32;
    if sig as libc::c_int == 1i32 {
      reload_inittab();
    }
    if sig as libc::c_int == 2i32 {
      run_actions(0x20i32);
    }
    if sig as libc::c_int == 3i32 {
      exec_restart_action();
      /* returns only if no restart action defined */
    }
    if 1i32 << sig as libc::c_int
      & 0i32 + (1i32 << 30i32) + (1i32 << 10i32) + (1i32 << 12i32) + (1i32 << 15i32)
      != 0
    {
      halt_reboot_pwoff(sig as libc::c_int);
    }
  }
}
unsafe extern "C" fn sleep_much() {
  sleep((30i32 * 24i32 * 60i32 * 60i32) as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn init_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  if !(*argv.offset(1)).is_null()
    && strcmp(
      *argv.offset(1),
      b"-q\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
  {
    return kill(1i32, 1i32);
  }
  if 0i32 == 0 {
    /* Some users send poweroff signals to init VERY early.
     * To handle this, mask signals early,
     * and unmask them only after signal handlers are installed.
     */
    sigprocmask_allsigs(0i32);
    /* misnomer */
    if getpid() != 1i32 && (1i32 == 0 || *applet_name.offset(0) as libc::c_int != 'l' as i32) {
      /* Expect to be invoked as init with PID=1 or be invoked as linuxrc */
      /* not linuxrc? */
      bb_simple_error_msg_and_die(b"must be run as PID 1\x00" as *const u8 as *const libc::c_char);
    }
    reboot(0i32);
  }
  /* Turn off rebooting via CTL-ALT-DEL - we get a
   * SIGINT on CAD so we can shut things down gracefully... */
  /* If, say, xmalloc would ever die, we don't want to oops kernel
   * by exiting.
   * NB: we set die_func *after* PID 1 check and bb_show_usage.
   * Otherwise, for example, "init u" ("please rexec yourself"
   * command for sysvinit) will show help text (which isn't too bad),
   * *and sleep forever* (which is bad!)
   */
  die_func = Some(sleep_much as unsafe extern "C" fn() -> ());
  /* Figure out where the default console should be */
  console_init();
  set_sane_term();
  xchdir(b"/\x00" as *const u8 as *const libc::c_char);
  setsid();
  /* Make sure environs is set to something sane */
  putenv(b"HOME=/\x00" as *const u8 as *const libc::c_char as *mut libc::c_char); /* needed? why? */
  putenv(bb_PATH_root_path.as_ptr() as *mut libc::c_char);
  putenv(b"SHELL=/bin/sh\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
  putenv(b"USER=root\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
  if !(*argv.offset(1)).is_null() {
    xsetenv(
      b"RUNLEVEL\x00" as *const u8 as *const libc::c_char,
      *argv.offset(1),
    );
  }
  /* Check if we are supposed to be in single user mode */
  if !(*argv.offset(1)).is_null()
    && (strcmp(
      *argv.offset(1),
      b"single\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
      || strcmp(
        *argv.offset(1),
        b"-s\x00" as *const u8 as *const libc::c_char,
      ) == 0i32
      || *(*argv.offset(1)).offset(0) as libc::c_int == '1' as i32
        && *(*argv.offset(1)).offset(1) == 0)
  {
    /* ??? shouldn't we set RUNLEVEL="b" here? */
    /* Start a shell on console */
    new_init_action(
      0x8i32 as u8,
      bb_default_login_shell.as_ptr(),
      b"\x00" as *const u8 as *const libc::c_char,
    );
  } else {
    /* Not in single user mode - see what inittab says */
    /* NOTE that if CONFIG_FEATURE_USE_INITTAB is NOT defined,
     * then parse_inittab() simply adds in some default
     * actions (i.e., INIT_SCRIPT and a pair
     * of "askfirst" shells) */
    parse_inittab();
  }
  /* Make the command line just say "init"  - that's all, nothing else */
  strncpy(
    *argv.offset(0),
    b"init\x00" as *const u8 as *const libc::c_char,
    strlen(*argv.offset(0)),
  );
  loop
  /* Wipe argv[1]-argv[N] so they don't clutter the ps listing */
  {
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
    nuke_str(*argv);
  }
  /* Set up signal handlers */
  if 0i32 == 0 {
    let mut sa: sigaction = std::mem::zeroed();
    /* Stop handler must allow only SIGCONT inside itself */
    memset(
      &mut sa as *mut sigaction as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<sigaction>() as libc::c_ulong,
    );
    sigfillset(&mut sa.sa_mask);
    sigdelset(&mut sa.sa_mask, 18i32);
    sa.__sigaction_handler.sa_handler =
      Some(stop_handler as unsafe extern "C" fn(_: libc::c_int) -> ());
    /* NB: sa_flags doesn't have SA_RESTART.
     * It must be able to interrupt wait().
     */
    sigaction_set(20i32, &mut sa); /* pause */
    /* Does not work as intended, at least in 2.6.20.
     * SIGSTOP is simply ignored by init:
     */
    sigaction_set(19i32, &mut sa); /* pause */
    /* These signals must interrupt wait(),
     * setting handler without SA_RESTART flag.
     */
    bb_signals_recursive_norestart(
      0i32
        + (1i32 << 2i32)
        + (1i32 << 3i32)
        + (1i32 << 30i32)
        + (1i32 << 10i32)
        + (1i32 << 15i32)
        + (1i32 << 12i32)
        + (1i32 << 1i32),
      Some(record_signo as unsafe extern "C" fn(_: libc::c_int) -> ()),
    );
    sigprocmask_allsigs(1i32);
  }
  /* Now run everything that needs to be run */
  /* First run the sysinit command */
  run_actions(0x1i32);
  check_delayed_sigs();
  /* Next run anything that wants to block */
  run_actions(0x2i32);
  check_delayed_sigs();
  /* Next run anything to be run only once */
  run_actions(0x4i32);
  loop
  /* Now run the looping stuff for the rest of forever.
   */
  {
    let mut maybe_WNOHANG: libc::c_int = 0;
    maybe_WNOHANG = check_delayed_sigs();
    /* (Re)run the respawn/askfirst stuff */
    run_actions(0x8i32 | 0x10i32);
    maybe_WNOHANG |= check_delayed_sigs();
    /* Don't consume all CPU time - sleep a bit */
    sleep(1i32 as libc::c_uint);
    maybe_WNOHANG |= check_delayed_sigs();
    /* Wait for any child process(es) to exit.
     *
     * If check_delayed_sigs above reported that a signal
     * was caught, wait will be nonblocking. This ensures
     * that if SIGHUP has reloaded inittab, respawn and askfirst
     * actions will not be delayed until next child death.
     */
    if maybe_WNOHANG != 0 {
      maybe_WNOHANG = 1i32
    }
    loop {
      let mut wpid: pid_t = 0;
      let mut a: *mut init_action = 0 as *mut init_action;
      /* If signals happen _in_ the wait, they interrupt it,
       * bb_signals_recursive_norestart set them up that way
       */
      wpid = waitpid(-1i32, 0 as *mut libc::c_int, maybe_WNOHANG);
      if wpid <= 0i32 {
        break;
      }
      a = mark_terminated(wpid);
      if !a.is_null() {
        message(
          L_LOG as libc::c_int,
          b"process \'%s\' (pid %d) exited. Scheduling for restart.\x00" as *const u8
            as *const libc::c_char,
          (*a).command.as_mut_ptr(),
          wpid,
        );
      }
      /* See if anyone else is waiting to be reaped */
      maybe_WNOHANG = 1i32
    }
  }
  /* while (1) */
}
//usage:#define init_trivial_usage
//usage:       ""
//usage:#define init_full_usage "\n\n"
//usage:       "Init is the first process started during boot. It never exits."
//usage:	IF_FEATURE_USE_INITTAB(
//usage:   "\n""It (re)spawns children according to /etc/inittab."
//usage:	)
//usage:	IF_NOT_FEATURE_USE_INITTAB(
//usage:   "\n""This version of init doesn't use /etc/inittab,"
//usage:   "\n""has fixed set of processed to run."
//usage:	)
//usage:
//usage:#define init_notes_usage
//usage:	"This version of init is designed to be run only by the kernel.\n"
//usage:	"\n"
//usage:	"BusyBox init doesn't support multiple runlevels. The runlevels field of\n"
//usage:	"the /etc/inittab file is completely ignored by BusyBox init. If you want\n"
//usage:	"runlevels, use sysvinit.\n"
//usage:	"\n"
//usage:	"BusyBox init works just fine without an inittab. If no inittab is found,\n"
//usage:	"it has the following default behavior:\n"
//usage:	"\n"
//usage:	"	::sysinit:/etc/init.d/rcS\n"
//usage:	"	::askfirst:/bin/sh\n"
//usage:	"	::ctrlaltdel:/sbin/reboot\n"
//usage:	"	::shutdown:/sbin/swapoff -a\n"
//usage:	"	::shutdown:/bin/umount -a -r\n"
//usage:	"	::restart:/sbin/init\n"
//usage:	"	tty2::askfirst:/bin/sh\n"
//usage:	"	tty3::askfirst:/bin/sh\n"
//usage:	"	tty4::askfirst:/bin/sh\n"
//usage:	"\n"
//usage:	"If you choose to use an /etc/inittab file, the inittab entry format is as follows:\n"
//usage:	"\n"
//usage:	"	<id>:<runlevels>:<action>:<process>\n"
//usage:	"\n"
//usage:	"	<id>:\n"
//usage:	"\n"
//usage:	"		WARNING: This field has a non-traditional meaning for BusyBox init!\n"
//usage:	"		The id field is used by BusyBox init to specify the controlling tty for\n"
//usage:	"		the specified process to run on. The contents of this field are\n"
//usage:	"		appended to \"/dev/\" and used as-is. There is no need for this field to\n"
//usage:	"		be unique, although if it isn't you may have strange results. If this\n"
//usage:	"		field is left blank, then the init's stdin/out will be used.\n"
//usage:	"\n"
//usage:	"	<runlevels>:\n"
//usage:	"\n"
//usage:	"		The runlevels field is completely ignored.\n"
//usage:	"\n"
//usage:	"	<action>:\n"
//usage:	"\n"
//usage:	"		Valid actions include: sysinit, respawn, askfirst, wait,\n"
//usage:	"		once, restart, ctrlaltdel, and shutdown.\n"
//usage:	"\n"
//usage:	"		The available actions can be classified into two groups: actions\n"
//usage:	"		that are run only once, and actions that are re-run when the specified\n"
//usage:	"		process exits.\n"
//usage:	"\n"
//usage:	"		Run only-once actions:\n"
//usage:	"\n"
//usage:	"			'sysinit' is the first item run on boot. init waits until all\n"
//usage:	"			sysinit actions are completed before continuing. Following the\n"
//usage:	"			completion of all sysinit actions, all 'wait' actions are run.\n"
//usage:	"			'wait' actions, like 'sysinit' actions, cause init to wait until\n"
//usage:	"			the specified task completes. 'once' actions are asynchronous,\n"
//usage:	"			therefore, init does not wait for them to complete. 'restart' is\n"
//usage:	"			the action taken to restart the init process. By default this should\n"
//usage:	"			simply run /sbin/init, but can be a script which runs pivot_root or it\n"
//usage:	"			can do all sorts of other interesting things. The 'ctrlaltdel' init\n"
//usage:	"			actions are run when the system detects that someone on the system\n"
//usage:	"			console has pressed the CTRL-ALT-DEL key combination. Typically one\n"
//usage:	"			wants to run 'reboot' at this point to cause the system to reboot.\n"
//usage:	"			Finally the 'shutdown' action specifies the actions to taken when\n"
//usage:	"			init is told to reboot. Unmounting filesystems and disabling swap\n"
//usage:	"			is a very good here.\n"
//usage:	"\n"
//usage:	"		Run repeatedly actions:\n"
//usage:	"\n"
//usage:	"			'respawn' actions are run after the 'once' actions. When a process\n"
//usage:	"			started with a 'respawn' action exits, init automatically restarts\n"
//usage:	"			it. Unlike sysvinit, BusyBox init does not stop processes from\n"
//usage:	"			respawning out of control. The 'askfirst' actions acts just like\n"
//usage:	"			respawn, except that before running the specified process it\n"
//usage:	"			displays the line \"Please press Enter to activate this console.\"\n"
//usage:	"			and then waits for the user to press enter before starting the\n"
//usage:	"			specified process.\n"
//usage:	"\n"
//usage:	"		Unrecognized actions (like initdefault) will cause init to emit an\n"
//usage:	"		error message, and then go along with its business. All actions are\n"
//usage:	"		run in the order they appear in /etc/inittab.\n"
//usage:	"\n"
//usage:	"	<process>:\n"
//usage:	"\n"
//usage:	"		Specifies the process to be executed and its command line.\n"
//usage:	"\n"
//usage:	"Example /etc/inittab file:\n"
//usage:	"\n"
//usage:	"	# This is run first except when booting in single-user mode\n"
//usage:	"	#\n"
//usage:	"	::sysinit:/etc/init.d/rcS\n"
//usage:	"	\n"
//usage:	"	# /bin/sh invocations on selected ttys\n"
//usage:	"	#\n"
//usage:	"	# Start an \"askfirst\" shell on the console (whatever that may be)\n"
//usage:	"	::askfirst:-/bin/sh\n"
//usage:	"	# Start an \"askfirst\" shell on /dev/tty2-4\n"
//usage:	"	tty2::askfirst:-/bin/sh\n"
//usage:	"	tty3::askfirst:-/bin/sh\n"
//usage:	"	tty4::askfirst:-/bin/sh\n"
//usage:	"	\n"
//usage:	"	# /sbin/getty invocations for selected ttys\n"
//usage:	"	#\n"
//usage:	"	tty4::respawn:/sbin/getty 38400 tty4\n"
//usage:	"	tty5::respawn:/sbin/getty 38400 tty5\n"
//usage:	"	\n"
//usage:	"	\n"
//usage:	"	# Example of how to put a getty on a serial line (for a terminal)\n"
//usage:	"	#\n"
//usage:	"	#::respawn:/sbin/getty -L ttyS0 9600 vt100\n"
//usage:	"	#::respawn:/sbin/getty -L ttyS1 9600 vt100\n"
//usage:	"	#\n"
//usage:	"	# Example how to put a getty on a modem line\n"
//usage:	"	#::respawn:/sbin/getty 57600 ttyS2\n"
//usage:	"	\n"
//usage:	"	# Stuff to do when restarting the init process\n"
//usage:	"	::restart:/sbin/init\n"
//usage:	"	\n"
//usage:	"	# Stuff to do before rebooting\n"
//usage:	"	::ctrlaltdel:/sbin/reboot\n"
//usage:	"	::shutdown:/bin/umount -a -r\n"
//usage:	"	::shutdown:/sbin/swapoff -a\n"
//usage:#define linuxrc_trivial_usage NOUSAGE_STR
//usage:#define linuxrc_full_usage ""

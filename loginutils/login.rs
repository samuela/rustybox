use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn exit(_: libc::c_int) -> !;
  #[no_mangle]
  fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn _exit(_: libc::c_int) -> !;
  #[no_mangle]
  fn getpid() -> __pid_t;
  #[no_mangle]
  fn isatty(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
  #[no_mangle]
  static mut stdin: *mut _IO_FILE;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn getchar_unlocked() -> libc::c_int;
  #[no_mangle]
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut FILE,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
  #[no_mangle]
  fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn fchmod(__fd: libc::c_int, __mode: __mode_t) -> libc::c_int;
  #[no_mangle]
  fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
  #[no_mangle]
  fn tcdrain(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn tcflush(__fd: libc::c_int, __queue_selector: libc::c_int) -> libc::c_int;
  /* Search for an entry with a matching username.  */
  #[no_mangle]
  fn bb_internal_getpwnam(__name: *const libc::c_char) -> *mut passwd;
  #[no_mangle]
  fn skip_dev_pfx(tty_name: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_copyfd_eof(fd1: libc::c_int, fd2: libc::c_int) -> off_t;
  #[no_mangle]
  fn ndelay_on(fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn ndelay_off(fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xsetenv(key: *const libc::c_char, value: *const libc::c_char);
  #[no_mangle]
  fn safe_strncpy(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    size: size_t,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn utoa(n: libc::c_uint) -> *mut libc::c_char;
  #[no_mangle]
  fn update_utmp(
    pid: pid_t,
    new_type: libc::c_int,
    tty_name: *const libc::c_char,
    username: *const libc::c_char,
    hostname: *const libc::c_char,
  );
  #[no_mangle]
  fn spawn_and_wait(argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn bb_daemonize_or_rexec(flags: libc::c_int);
  #[no_mangle]
  fn sanitize_env_if_suid() -> libc::c_int;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_do_delay(seconds: libc::c_int);
  #[no_mangle]
  fn change_identity(pw: *const passwd);
  #[no_mangle]
  fn run_shell(
    shell: *const libc::c_char,
    loginshell: libc::c_int,
    args: *mut *const libc::c_char,
  ) -> !;
  #[no_mangle]
  fn setup_environment(shell: *const libc::c_char, flags: libc::c_int, pw: *const passwd);
  #[no_mangle]
  fn is_tty_secure(short_tty: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn ask_and_check_password(pw: *const passwd) -> libc::c_int;
  #[no_mangle]
  fn print_login_prompt();
  #[no_mangle]
  fn xmalloc_ttyname(fd: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn tcsetattr_stdin_TCSANOW(tp: *const termios) -> libc::c_int;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
  #[no_mangle]
  fn openlog(__ident: *const libc::c_char, __option: libc::c_int, __facility: libc::c_int);
  #[no_mangle]
  fn syslog(__pri: libc::c_int, __fmt: *const libc::c_char, _: ...);
}
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type off_t = __off64_t;
pub type pid_t = __pid_t;
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
use crate::librb::_IO_FILE;
pub type _IO_lock_t = ();
use crate::librb::_IO_marker;
use crate::librb::FILE;
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
  pub c_iflag: tcflag_t,
  pub c_oflag: tcflag_t,
  pub c_cflag: tcflag_t,
  pub c_lflag: tcflag_t,
  pub c_line: cc_t,
  pub c_cc: [cc_t; 32],
  pub c_ispeed: speed_t,
  pub c_ospeed: speed_t,
}
use crate::librb::passwd;
pub type C2RustUnnamed = libc::c_uint;
pub const DAEMON_ONLY_SANITIZE: C2RustUnnamed = 8;
pub const DAEMON_CLOSE_EXTRA_FDS: C2RustUnnamed = 4;
pub const DAEMON_DEVNULL_STDIO: C2RustUnnamed = 2;
pub const DAEMON_CHDIR_ROOT: C2RustUnnamed = 1;
//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub tty_attrs: termios,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const TTYNAME_SIZE: C2RustUnnamed_0 = 32;
pub const USERNAME_SIZE: C2RustUnnamed_0 = 64;
pub const EMPTY_USERNAME_COUNT: C2RustUnnamed_0 = 10;
pub const TIMEOUT: C2RustUnnamed_0 = 60;
pub const LOGIN_OPT_p: C2RustUnnamed_1 = 4;
pub const LOGIN_OPT_f: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const LOGIN_OPT_h: C2RustUnnamed_1 = 2;
unsafe extern "C" fn die_if_nologin() {
  let mut fp: *mut FILE = 0 as *mut FILE;
  let mut c: libc::c_int = 0;
  let mut empty: libc::c_int = 1i32;
  fp = fopen_for_read(b"/etc/nologin\x00" as *const u8 as *const libc::c_char);
  if fp.is_null() {
    /* assuming it does not exist */
    return;
  }
  loop {
    c = getc_unlocked(fp);
    if !(c != -1i32) {
      break;
    }
    if c == '\n' as i32 {
      bb_putchar('\r' as i32);
    }
    bb_putchar(c);
    empty = 0i32
  }
  if empty != 0 {
    puts(b"\r\nSystem closed for routine maintenance\r\x00" as *const u8 as *const libc::c_char);
  }
  fclose(fp);
  fflush_all();
  /* Users say that they do need this prior to exit: */
  tcdrain(1i32); /* NOMMU-friendly */
  exit(1i32);
}
unsafe extern "C" fn run_login_script(mut pw: *mut passwd, mut full_tty: *mut libc::c_char) {
  let mut t_argv: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
  t_argv[0] = getenv(b"LOGIN_PRE_SUID_SCRIPT\x00" as *const u8 as *const libc::c_char);
  if !t_argv[0].is_null() {
    t_argv[1] = 0 as *mut libc::c_char;
    xsetenv(
      b"LOGIN_TTY\x00" as *const u8 as *const libc::c_char,
      full_tty,
    );
    xsetenv(
      b"LOGIN_USER\x00" as *const u8 as *const libc::c_char,
      (*pw).pw_name,
    );
    xsetenv(
      b"LOGIN_UID\x00" as *const u8 as *const libc::c_char,
      utoa((*pw).pw_uid),
    );
    xsetenv(
      b"LOGIN_GID\x00" as *const u8 as *const libc::c_char,
      utoa((*pw).pw_gid),
    );
    xsetenv(
      b"LOGIN_SHELL\x00" as *const u8 as *const libc::c_char,
      (*pw).pw_shell,
    );
    spawn_and_wait(t_argv.as_mut_ptr());
    unsetenv(b"LOGIN_TTY\x00" as *const u8 as *const libc::c_char);
    unsetenv(b"LOGIN_USER\x00" as *const u8 as *const libc::c_char);
    unsetenv(b"LOGIN_UID\x00" as *const u8 as *const libc::c_char);
    unsetenv(b"LOGIN_GID\x00" as *const u8 as *const libc::c_char);
    unsetenv(b"LOGIN_SHELL\x00" as *const u8 as *const libc::c_char);
  };
}
unsafe extern "C" fn get_username_or_die(mut buf: *mut libc::c_char, mut size_buf: libc::c_int) {
  let mut c: libc::c_int = 0;
  let mut cntdown: libc::c_int = 0;
  cntdown = EMPTY_USERNAME_COUNT as libc::c_int;
  'c_8997: loop {
    print_login_prompt();
    loop
    /* skip whitespace */
    {
      c = getchar_unlocked(); /* maybe isblank? */
      if c == -1i32 {
        exit(1i32);
      }
      if c == '\n' as i32 {
        cntdown -= 1;
        if cntdown == 0 {
          exit(1i32);
        }
        break;
      } else if !(({
        let mut bb__isspace: libc::c_uchar = (c - 9i32) as libc::c_uchar;
        (bb__isspace as libc::c_int == ' ' as i32 - 9i32
          || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
      }) != 0)
      {
        break 'c_8997;
      }
    }
  }
  let fresh0 = buf;
  buf = buf.offset(1);
  *fresh0 = c as libc::c_char;
  if fgets_unlocked(buf, size_buf - 2i32, stdin).is_null() {
    exit(1i32);
  }
  if strchr(buf, '\n' as i32).is_null() {
    exit(1i32);
  }
  while *buf as libc::c_uchar as libc::c_int > ' ' as i32 {
    buf = buf.offset(1)
  }
  *buf = '\u{0}' as i32 as libc::c_char;
}
unsafe extern "C" fn motd() {
  let mut fd: libc::c_int = 0;
  fd = open(b"/etc/motd\x00" as *const u8 as *const libc::c_char, 0i32);
  if fd >= 0i32 {
    fflush_all();
    bb_copyfd_eof(fd, 1i32);
    close(fd);
  };
}
unsafe extern "C" fn alarm_handler(mut _sig: libc::c_int) {
  /* This is the escape hatch! Poor serial line users and the like
   * arrive here when their connection is broken.
   * We don't want to block here */
  ndelay_on(1i32);
  /* Test for correct attr restoring:
   * run "getty 0 -" from a shell, enter bogus username, stop at
   * password prompt, let it time out. Without the tcsetattr below,
   * when you are back at shell prompt, echo will be still off.
   */
  tcsetattr_stdin_TCSANOW(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tty_attrs);
  printf(
    b"\r\nLogin timed out after %u seconds\r\n\x00" as *const u8 as *const libc::c_char,
    TIMEOUT as libc::c_int,
  );
  fflush_all();
  /* unix API is brain damaged regarding O_NONBLOCK,
   * we should undo it, or else we can affect other processes */
  ndelay_off(1i32); /* for compiler */
  _exit(0i32);
}
#[no_mangle]
pub unsafe extern "C" fn login_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut fromhost: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut username: [libc::c_char; 64] = [0; 64];
  let mut run_by_root: libc::c_int = 0;
  let mut opt: libc::c_uint = 0;
  let mut count: libc::c_int = 0i32;
  let mut pw: *mut passwd = 0 as *mut passwd;
  let mut opt_host: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opt_user: *mut libc::c_char = 0 as *mut libc::c_char;
  opt_user = opt_user;
  let mut full_tty: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut short_tty: *mut libc::c_char = 0 as *mut libc::c_char;
  /* More of suid paranoia if called by non-root: */
  /* Clear dangerous stuff, set PATH */
  run_by_root = (sanitize_env_if_suid() == 0) as libc::c_int;
  /* Mandatory paranoia for suid applet:
   * ensure that fd# 0,1,2 are opened (at least to /dev/null)
   * and any extra open fd's are closed.
   */
  bb_daemonize_or_rexec(
    DAEMON_CLOSE_EXTRA_FDS as libc::c_int | DAEMON_ONLY_SANITIZE as libc::c_int,
  );
  username[0] = '\u{0}' as i32 as libc::c_char;
  opt = getopt32(
    argv,
    b"f:h:p\x00" as *const u8 as *const libc::c_char,
    &mut opt_user as *mut *mut libc::c_char,
    &mut opt_host as *mut *mut libc::c_char,
  );
  if opt & LOGIN_OPT_f as libc::c_int as libc::c_uint != 0 {
    if run_by_root == 0 {
      bb_simple_error_msg_and_die(b"-f is for root only\x00" as *const u8 as *const libc::c_char);
    }
    safe_strncpy(
      username.as_mut_ptr(),
      opt_user,
      ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
    );
  }
  argv = argv.offset(optind as isize);
  if !(*argv.offset(0)).is_null() {
    /* user from command line (getty) */
    safe_strncpy(
      username.as_mut_ptr(),
      *argv.offset(0),
      ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
    );
  }
  /* Save tty attributes - and by doing it, check that it's indeed a tty */
  if tcgetattr(
    0i32,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tty_attrs,
  ) < 0i32
    || isatty(1i32) == 0
  {
    /*|| !isatty(STDERR_FILENO) - no, guess some people might want to redirect this */
    return 1i32;
    /* Must be a terminal */
  }
  /* We install timeout handler only _after_ we saved G.tty_attrs */
  signal(
    14i32,
    Some(alarm_handler as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  alarm(TIMEOUT as libc::c_int as libc::c_uint);
  /* Find out and memorize our tty name */
  full_tty = xmalloc_ttyname(0i32);
  if full_tty.is_null() {
    full_tty = xstrdup(b"UNKNOWN\x00" as *const u8 as *const libc::c_char)
  }
  short_tty = skip_dev_pfx(full_tty);
  if !opt_host.is_null() {
    fromhost = xasprintf(
      b" on \'%s\' from \'%s\'\x00" as *const u8 as *const libc::c_char,
      short_tty,
      opt_host,
    )
  } else {
    fromhost = xasprintf(
      b" on \'%s\'\x00" as *const u8 as *const libc::c_char,
      short_tty,
    )
  }
  /* Was breaking "login <username>" from shell command line: */
  /*bb_setpgrp();*/
  openlog(applet_name, 0x1i32 | 0x2i32, 4i32 << 3i32); /* while (1) */
  loop
  /* flush away any type-ahead (as getty does) */
  {
    tcflush(0i32, 0i32);
    if username[0] == 0 {
      get_username_or_die(
        username.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
      );
    }
    /* not PAM */
    pw = bb_internal_getpwnam(username.as_mut_ptr()); /* -f USER: success without asking passwd */
    if pw.is_null() {
      strcpy(
        username.as_mut_ptr(),
        b"UNKNOWN\x00" as *const u8 as *const libc::c_char,
      );
      current_block = 6331257520130940356;
    } else if *(*pw).pw_passwd.offset(0) as libc::c_int == '!' as i32
      || *(*pw).pw_passwd.offset(0) as libc::c_int == '*' as i32
    {
      current_block = 8456411428248478739;
    } else {
      if opt & LOGIN_OPT_f as libc::c_int as libc::c_uint != 0 {
        break;
      }
      if (*pw).pw_uid == 0i32 as libc::c_uint && is_tty_secure(short_tty) == 0 {
        current_block = 8456411428248478739;
      } else {
        /* Don't check the password if password entry is empty (!) */
        if *(*pw).pw_passwd.offset(0) == 0 {
          break;
        }
        current_block = 6331257520130940356;
      }
    }
    match current_block {
      6331257520130940356 =>
      /* Password reading and authorization takes place here.
       * Note that reads (in no-echo mode) trash tty attributes.
       * If we get interrupted by SIGALRM, we need to restore attrs.
       */
      {
        if ask_and_check_password(pw) > 0i32 {
          break;
        }
      }
      _ => {}
    }
    /* ENABLE_PAM */
    opt &= !(LOGIN_OPT_f as libc::c_int) as libc::c_uint;
    bb_do_delay(3i32);
    /* TODO: doesn't sound like correct English phrase to me */
    puts(b"Login incorrect\x00" as *const u8 as *const libc::c_char);
    count += 1;
    if count == 3i32 {
      syslog(
        4i32,
        b"invalid password for \'%s\'%s\x00" as *const u8 as *const libc::c_char,
        username.as_mut_ptr(),
        fromhost,
      );
      return 1i32;
    }
    username[0] = '\u{0}' as i32 as libc::c_char
  }
  alarm(0i32 as libc::c_uint);
  /* We can ignore /etc/nologin if we are logging in as root,
   * it doesn't matter whether we are run by root or not */
  if (*pw).pw_uid != 0i32 as libc::c_uint {
    die_if_nologin();
  }
  /* Try these, but don't complain if they fail.
   * _f_chown is safe wrt race t=ttyname(0);...;chown(t); */
  fchown(0i32, (*pw).pw_uid, (*pw).pw_gid);
  fchmod(0i32, 0o600i32 as __mode_t);
  update_utmp(
    getpid(),
    7i32,
    short_tty,
    username.as_mut_ptr(),
    if run_by_root != 0 {
      opt_host
    } else {
      0 as *mut libc::c_char
    },
  );
  /* We trust environment only if we run by root */
  if 1i32 != 0 && run_by_root != 0 {
    run_login_script(pw, full_tty);
  }
  change_identity(pw);
  setup_environment(
    (*pw).pw_shell,
    (opt & LOGIN_OPT_p as libc::c_int as libc::c_uint == 0) as libc::c_int * (1i32 << 1i32)
      + (1i32 << 0i32),
    pw,
  );
  if access(b".hushlogin\x00" as *const u8 as *const libc::c_char, 0i32) != 0i32 {
    motd();
  }
  if (*pw).pw_uid == 0i32 as libc::c_uint {
    syslog(
      6i32,
      b"root login%s\x00" as *const u8 as *const libc::c_char,
      fromhost,
    );
  }
  /* well, a simple setexeccon() here would do the job as well,
   * but let's play the game for now */
  // util-linux login also does:
  // /* start new session */
  // setsid();
  // /* TIOCSCTTY: steal tty from other process group */
  // if (ioctl(0, TIOCSCTTY, 1)) error_msg...
  // BBox login used to do this (see above):
  // bb_setpgrp();
  // If this stuff is really needed, add it and explain why!
  /* Set signals to defaults */
  /* Non-ignored signals revert to SIG_DFL on exec anyway */
  /*signal(SIGALRM, SIG_DFL);*/
  /* Is this correct? This way user can ctrl-c out of /etc/profile,
   * potentially creating security breach (tested with bash 3.0).
   * But without this, bash 3.0 will not enable ctrl-c either.
   * Maybe bash is buggy?
   * Need to find out what standards say about /bin/login -
   * should we leave SIGINT etc enabled or disabled? */
  signal(2i32, None);
  /* Exec login shell with no additional parameters */
  run_shell((*pw).pw_shell, 1i32, 0 as *mut *const libc::c_char);
  /* return EXIT_FAILURE; - not reached */
}

use crate::libbb::ptr_to_globals::bb_errno;
use crate::librb::signal::__sighandler_t;
use crate::librb::size_t;
use crate::librb::smallint;

use libc;
use libc::close;
use libc::fprintf;
use libc::ioctl;
use libc::pid_t;
use libc::pollfd;
use libc::printf;
use libc::setsid;
use libc::ssize_t;
use libc::termios;
use libc::time;
use libc::time_t;
use libc::timeval;
use libc::winsize;
use libc::FILE;
extern "C" {
  #[no_mangle]
  fn execl(__path: *const libc::c_char, __arg: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn vfork() -> libc::c_int;

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;

  #[no_mangle]
  static mut stderr: *mut FILE;

  #[no_mangle]
  fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> libc::c_int;

  #[no_mangle]
  fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;

  #[no_mangle]
  fn tcsetattr(
    __fd: libc::c_int,
    __optional_actions: libc::c_int,
    __termios_p: *const termios,
  ) -> libc::c_int;

  #[no_mangle]
  fn cfmakeraw(__termios_p: *mut termios);

  /* Some useful definitions */
  /* Macros for min/max.  */
  /* buffer allocation schemes */
  /* glibc uses __errno_location() to get a ptr to errno */
  /* We can just memorize it once - no multithreading in busybox :) */

  /* more than enough for "/dev/ttyXXX" */
  #[no_mangle]
  fn xgetpty(line: *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ndelay_on(fd: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn xdup2(_: libc::c_int, _: libc::c_int);

  /* Standard handler which just records signo */
  #[no_mangle]
  static mut bb_got_signal: smallint;

  #[no_mangle]
  fn record_signo(signo: libc::c_int);

  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf_0: *mut libc::c_void, count: size_t) -> ssize_t;

  // NB: will return short write on error, not -1,
  // if some data was written before error occurred
  #[no_mangle]
  fn full_write(fd: libc::c_int, buf_0: *const libc::c_void, count: size_t) -> ssize_t;

  #[no_mangle]
  fn xfopen_for_write(path: *const libc::c_char) -> *mut FILE;

  #[no_mangle]
  fn bb_sanitize_stdio();

  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> u32;

  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;

  /* Returns $SHELL, getpwuid(getuid())->pw_shell, or DEFAULT_SHELL.
   * Note that getpwuid result might need xstrdup'ing
   * if there is a possibility of intervening getpwxxx() calls.
   */
  #[no_mangle]
  fn get_shell_name() -> *const libc::c_char;

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

pub type nfds_t = libc::c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
  pub tz_minuteswest: libc::c_int,
  pub tz_dsttime: libc::c_int,
}

pub type __timezone_ptr_t = *mut timezone;

pub type C2RustUnnamed = libc::c_uint;
pub const COMMON_BUFSIZE: C2RustUnnamed = 1024;
pub const OPT_q: C2RustUnnamed_0 = 8;
pub const OPT_t: C2RustUnnamed_0 = 16;
pub const OPT_c: C2RustUnnamed_0 = 2;
pub const OPT_a: C2RustUnnamed_0 = 1;

pub type C2RustUnnamed_0 = libc::c_uint;
// pub const OPT_f: C2RustUnnamed_0 = 4;

/*
 * script implementation for busybox
 *
 * pascal.bellard@ads-lu.com
 *
 * Based on code from util-linux v 2.12r
 * Copyright (c) 1980
 * The Regents of the University of California.  All rights reserved.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config SCRIPT
//config:	bool "script (8.6 kb)"
//config:	default y
//config:	help
//config:	The script makes typescript of terminal session.
//applet:IF_SCRIPT(APPLET(script, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_SCRIPT) += script.o
//usage:#define script_trivial_usage
//usage:       "[-afq] [-t[FILE]] [-c PROG] [OUTFILE]"
//usage:#define script_full_usage "\n\n"
//usage:       "Default OUTFILE is 'typescript'"
//usage:     "\n"
//usage:     "\n	-a	Append output"
//usage:     "\n	-c PROG	Run PROG, not shell"
/* Accepted but has no effect (we never buffer output) */
/*//usage:     "\n	-f	Flush output after each write"*/
//usage:     "\n	-q	Quiet"
//usage:     "\n	-t[FILE] Send timing to stderr or FILE"
//util-linux-2.28:
//-e: return exit code of the child
//FYI (reported as bbox bug #2749):
// > script -q -c 'echo -e -n "1\n2\n3\n"' /dev/null </dev/null >123.txt
// > The output file on full-blown ubuntu system contains 6 bytes.
// > Output on Busybox system (arm-linux) contains extra '\r' byte in each line.
//however, in my test, "script" from util-linux-2.28 seems to also add '\r' bytes.
#[no_mangle]
pub unsafe extern "C" fn script_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opt: libc::c_int = 0; /* NB: 0: ok */
  let mut mode: libc::c_int = 0;
  let mut child_pid: libc::c_int = 0;
  let mut attr_ok: libc::c_int = 0;
  let mut winsz_ok: libc::c_int = 0;
  let mut pty: libc::c_int = 0;
  let mut pty_line: [libc::c_char; 16] = [0; 16];
  let mut tt: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
  };
  let mut rtt: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
  };
  let mut win: winsize = winsize {
    ws_row: 0,
    ws_col: 0,
    ws_xpixel: 0,
    ws_ypixel: 0,
  };
  let mut timing_fp: *mut FILE = 0 as *mut FILE;
  let mut str_t: *const libc::c_char = 0 as *const libc::c_char;
  let mut fname: *const libc::c_char = b"typescript\x00" as *const u8 as *const libc::c_char;
  let mut shell: *const libc::c_char = 0 as *const libc::c_char;
  let mut shell_opt: [libc::c_char; 3] =
    *::std::mem::transmute::<&[u8; 3], &mut [libc::c_char; 3]>(b"-i\x00");
  let mut shell_arg: *mut libc::c_char = 0 as *mut libc::c_char;
  static mut script_longopts: [libc::c_char; 45] = [
    97, 112, 112, 101, 110, 100, 0, 0, 97, 99, 111, 109, 109, 97, 110, 100, 0, 1, 99, 102, 108,
    117, 115, 104, 0, 0, 102, 113, 117, 105, 101, 116, 0, 0, 113, 116, 105, 109, 105, 110, 103, 0,
    2, 116, 0,
  ];
  opt = getopt32long(
    argv,
    b"^ac:fqt::\x00?1\x00" as *const u8 as *const libc::c_char,
    script_longopts.as_ptr(),
    &mut shell_arg as *mut *mut libc::c_char,
    &mut str_t as *mut *const libc::c_char,
  ) as libc::c_int;
  //argc -= optind;
  argv = argv.offset(optind as isize);
  if !(*argv.offset(0)).is_null() {
    fname = *argv.offset(0)
  }
  mode = 0o100i32 | 0o1000i32 | 0o1i32;
  if opt & OPT_a as libc::c_int != 0 {
    mode = 0o100i32 | 0o2000i32 | 0o1i32
  }
  if opt & OPT_c as libc::c_int != 0 {
    shell_opt[1] = 'c' as i32 as libc::c_char
  }
  if opt & OPT_q as libc::c_int == 0 {
    printf(
      b"Script started, file is %s\n\x00" as *const u8 as *const libc::c_char,
      fname,
    );
  }
  timing_fp = stderr;
  if !str_t.is_null() {
    timing_fp = xfopen_for_write(str_t)
  }
  shell = get_shell_name();
  /* Some people run "script ... 0>&-".
   * Our code assumes that STDIN_FILENO != pty.
   * Ensure STDIN_FILENO is not closed:
   */
  bb_sanitize_stdio();
  pty = xgetpty(pty_line.as_mut_ptr());
  /* get current stdin's tty params */
  attr_ok = tcgetattr(0i32, &mut tt);
  winsz_ok = ioctl(
    0i32,
    0x5413i32 as libc::c_ulong,
    &mut win as *mut winsize as *mut libc::c_char,
  );
  rtt = tt;
  cfmakeraw(&mut rtt);
  rtt.c_lflag &= !0o10i32 as libc::c_uint;
  tcsetattr(0i32, 2i32, &mut rtt);
  /* "script" from util-linux exits when child exits,
   * we wouldn't wait for EOF from slave pty
   * (output may be produced by grandchildren of child) */
  signal(
    17i32,
    Some(record_signo as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  /* TODO: SIGWINCH? pass window size changes down to slave? */
  child_pid = {
    let mut bb__xvfork_pid: pid_t = vfork();
    if bb__xvfork_pid < 0i32 {
      bb_simple_perror_msg_and_die(b"vfork\x00" as *const u8 as *const libc::c_char);
    }
    bb__xvfork_pid
  };
  if child_pid != 0 {
    let mut current_block_67: u64;
    /* parent */
    let mut pfd: [pollfd; 2] = [pollfd {
      fd: 0,
      events: 0,
      revents: 0,
    }; 2]; /* this descriptor is not shared, can do this */
    let mut outfd: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut loop_0: libc::c_int = 0;
    let mut oldtime: libc::c_double = time(0 as *mut time_t) as libc::c_double;
    let mut fd_count: smallint = 2i32 as smallint;
    outfd = xopen(fname, mode);
    pfd[0].fd = pty;
    pfd[0].events = 0x1i32 as libc::c_short;
    pfd[1].fd = 0i32;
    pfd[1].events = 0x1i32 as libc::c_short;
    ndelay_on(pty);
    loop
    /* ndelay_on(STDIN_FILENO); - NO, stdin can be shared! Pity :( */
    /* copy stdin to pty master input,
     * copy pty master output to stdout and file */
    /* TODO: don't use full_write's, use proper write buffering */
    {
      if !(fd_count as libc::c_int != 0 && bb_got_signal == 0) {
        current_block_67 = 7178192492338286402;
        break;
      }
      /* not safe_poll! we want SIGCHLD to EINTR poll */
      if poll(pfd.as_mut_ptr(), fd_count as nfds_t, -1i32) < 0i32 && *bb_errno != 4i32 {
        current_block_67 = 7178192492338286402;
        break;
      }
      if pfd[0].revents != 0 {
        *bb_errno = 0i32;
        count = safe_read(
          pty,
          bb_common_bufsiz1.as_mut_ptr() as *mut libc::c_void,
          COMMON_BUFSIZE as libc::c_int as size_t,
        ) as libc::c_int;
        if count <= 0i32 && *bb_errno != 11i32 {
          current_block_67 = 916363241452857900;
          break;
        }
        if count > 0i32 {
          if opt & OPT_t as libc::c_int != 0 {
            let mut tv: timeval = timeval {
              tv_sec: 0,
              tv_usec: 0,
            };
            let mut newtime: libc::c_double = 0.;
            gettimeofday(&mut tv, 0 as *mut timezone);
            newtime = tv.tv_sec as libc::c_double
              + tv.tv_usec as libc::c_double / 1000000i32 as libc::c_double;
            fprintf(
              timing_fp,
              b"%f %u\n\x00" as *const u8 as *const libc::c_char,
              newtime - oldtime,
              count,
            );
            oldtime = newtime
          }
          full_write(
            1i32,
            bb_common_bufsiz1.as_mut_ptr() as *const libc::c_void,
            count as size_t,
          );
          full_write(
            outfd,
            bb_common_bufsiz1.as_mut_ptr() as *const libc::c_void,
            count as size_t,
          );
          // If we'd be using (buffered) FILE i/o, we'd need this:
          //if (opt & OPT_f) {
          //	fflush(outfd);
          //}
        }
      }
      if pfd[1].revents != 0 {
        count = safe_read(
          0i32,
          bb_common_bufsiz1.as_mut_ptr() as *mut libc::c_void,
          COMMON_BUFSIZE as libc::c_int as size_t,
        ) as libc::c_int;
        if count <= 0i32 {
          /* err/eof from stdin: don't read stdin anymore */
          pfd[1].revents = 0i32 as libc::c_short;
          fd_count -= 1
        } else {
          full_write(
            pty,
            bb_common_bufsiz1.as_mut_ptr() as *const libc::c_void,
            count as size_t,
          );
        }
      }
    }
    match current_block_67 {
      7178192492338286402 =>
      /* If child exits too quickly, we may get EIO:
       * for example, try "script -c true" */
      /* If loop was exited because SIGCHLD handler set bb_got_signal,
       * there still can be some buffered output. But dont loop forever:
       * we won't pump orphaned grandchildren's output indefinitely.
       * Testcase: running this in script:
       *      exec dd if=/dev/zero bs=1M count=1
       * must have "1+0 records in, 1+0 records out" captured too.
       * (util-linux's script doesn't do this. buggy :) */
      {
        loop_0 = 999i32;
        loop
        /* pty is in O_NONBLOCK mode, we exit as soon as buffer is empty */
        {
          loop_0 -= 1;
          if !(loop_0 != 0 && {
            count = safe_read(
              pty,
              bb_common_bufsiz1.as_mut_ptr() as *mut libc::c_void,
              COMMON_BUFSIZE as libc::c_int as size_t,
            ) as libc::c_int;
            (count) > 0i32
          }) {
            break;
          }
          full_write(
            1i32,
            bb_common_bufsiz1.as_mut_ptr() as *const libc::c_void,
            count as size_t,
          );
          full_write(
            outfd,
            bb_common_bufsiz1.as_mut_ptr() as *const libc::c_void,
            count as size_t,
          );
        }
      }
      _ => {}
    }
    /* err/eof from pty: exit */
    if attr_ok == 0i32 {
      tcsetattr(0i32, 2i32, &mut tt);
    }
    if opt & OPT_q as libc::c_int == 0 {
      printf(
        b"Script done, file is %s\n\x00" as *const u8 as *const libc::c_char,
        fname,
      );
    }
    return 0i32;
  }
  /* child: make pty slave to be input, output, error; run shell */
  close(pty); /* close pty master */
  /* open pty slave to fd 0,1,2 */
  close(0i32); /* uses fd 0 */
  xopen(pty_line.as_mut_ptr(), 0o2i32);
  xdup2(0i32, 1i32);
  xdup2(0i32, 2i32);
  /* copy our original stdin tty's parameters to pty */
  if attr_ok == 0i32 {
    tcsetattr(0i32, 2i32, &mut tt);
  }
  if winsz_ok == 0i32 {
    ioctl(
      0i32,
      0x5414i32 as libc::c_ulong,
      &mut win as *mut winsize as *mut libc::c_char,
    );
  }
  /* set pty as a controlling tty */
  setsid();
  ioctl(0i32, 0x540ei32 as libc::c_ulong, 0i32);
  /* Non-ignored signals revert to SIG_DFL on exec anyway */
  /*signal(SIGCHLD, SIG_DFL);*/
  execl(
    shell,
    shell,
    shell_opt.as_mut_ptr(),
    shell_arg,
    0 as *mut libc::c_void as *mut libc::c_char,
  );
  bb_simple_perror_msg_and_die(shell);
}

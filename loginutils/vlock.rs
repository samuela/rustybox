use libc;
use libc::unlink;


extern "C" {
  #[no_mangle]
  fn getuid() -> uid_t;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
  #[no_mangle]
  fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
  #[no_mangle]
  fn xdup2(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn bb_signals(sigs: libc::c_int, f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>);
  #[no_mangle]
  fn signal_SA_RESTART_empty_mask(
    sig: libc::c_int,
    handler: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
  );
  #[no_mangle]
  fn sig_unblock(sig: libc::c_int);
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
  #[no_mangle]
  fn xgetpwuid(uid: uid_t) -> *mut passwd;
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_do_delay(seconds: libc::c_int);
  #[no_mangle]
  fn ask_and_check_password(pw: *const passwd) -> libc::c_int;
  #[no_mangle]
  fn tcsetattr_stdin_TCSANOW(tp: *const termios) -> libc::c_int;
  #[no_mangle]
  fn bb_xioctl(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
}

use crate::librb::signal::__sighandler_t;
use libc::uid_t;

use libc::termios;
use libc::passwd;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vt_mode {
  pub mode: libc::c_char,
  pub waitv: libc::c_char,
  pub relsig: libc::c_short,
  pub acqsig: libc::c_short,
  pub frsig: libc::c_short,
}

/*
 * vlock implementation for busybox
 *
 * Copyright (C) 2000 by spoon <spoon@ix.netcom.com>
 * Written by spoon <spon@ix.netcom.com>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Shoutz to Michael K. Johnson <johnsonm@redhat.com>, author of the
 * original vlock.  I snagged a bunch of his code to write this
 * minimalistic vlock.
 */
/* Fixed by Erik Andersen to do passwords the tinylogin way...
 * It now works with md5, sha1, etc passwords.
 */
//config:config VLOCK
//config:	bool "vlock (17 kb)"
//config:	default y
//config:	help
//config:	Build the "vlock" applet which allows you to lock (virtual) terminals.
//config:
//config:	Note that busybox binary must be setuid root for this applet to
//config:	work properly.
//applet:/* Needs to be run by root or be suid root - needs to change uid and gid: */
//applet:IF_VLOCK(APPLET(vlock, BB_DIR_USR_BIN, BB_SUID_REQUIRE))
//kbuild:lib-$(CONFIG_VLOCK) += vlock.o
//usage:#define vlock_trivial_usage
//usage:       "[-a]"
//usage:#define vlock_full_usage "\n\n"
//usage:       "Lock a virtual terminal. A password is required to unlock.\n"
//usage:     "\n	-a	Lock all VTs"
unsafe extern "C" fn release_vt(mut _signo: libc::c_int) {
  /* If -a, param is 0, which means:
   * "no, kernel, we don't allow console switch away from us!" */
  ioctl(
    0i32,
    0x5605i32 as libc::c_ulong,
    (option_mask32 == 0) as libc::c_int as libc::c_ulong,
  );
}
unsafe extern "C" fn acquire_vt(mut _signo: libc::c_int) {
  /* ACK to kernel that switch to console is successful */
  ioctl(0i32, 0x5605i32 as libc::c_ulong, 0x2i32);
}
#[no_mangle]
pub unsafe extern "C" fn vlock_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut vtm: vt_mode = vt_mode {
    mode: 0,
    waitv: 0,
    relsig: 0,
    acqsig: 0,
    frsig: 0,
  };
  let mut ovtm: vt_mode = vt_mode {
    mode: 0,
    waitv: 0,
    relsig: 0,
    acqsig: 0,
    frsig: 0,
  };
  let mut term: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
  };
  let mut oterm: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
  };
  let mut pw: *mut passwd = 0 as *mut passwd;
  pw = xgetpwuid(getuid());
  getopt32(argv, b"^a\x00=0\x00" as *const u8 as *const libc::c_char);
  /* Ignore some signals so that we don't get killed by them */
  bb_signals(
    0i32
      + (1i32 << 20i32)
      + (1i32 << 21i32)
      + (1i32 << 22i32)
      + (1i32 << 1i32)
      + (1i32 << 17i32)
      + (1i32 << 3i32)
      + (1i32 << 2i32),
    ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
  );
  /* We will use SIGUSRx for console switch control: */
  /* 1: set handlers */
  signal_SA_RESTART_empty_mask(
    10i32,
    Some(release_vt as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  signal_SA_RESTART_empty_mask(
    12i32,
    Some(acquire_vt as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  /* 2: unmask them */
  sig_unblock(10i32);
  sig_unblock(12i32);
  /* Revert stdin/out to our controlling tty
   * (or die if we have none) */
  xmove_fd(
    xopen(b"/dev/tty\x00" as *const u8 as *const libc::c_char, 0o2i32),
    0i32,
  );
  xdup2(0i32, 1i32);
  bb_xioctl(
    0i32,
    0x5601i32 as libc::c_uint,
    &mut vtm as *mut vt_mode as *mut libc::c_void,
    b"VT_GETMODE\x00" as *const u8 as *const libc::c_char,
  );
  ovtm = vtm;
  /* "console switches are controlled by us, not kernel!" */
  vtm.mode = 0x1i32 as libc::c_char;
  vtm.relsig = 10i32 as libc::c_short;
  vtm.acqsig = 12i32 as libc::c_short;
  ioctl(0i32, 0x5602i32 as libc::c_ulong, &mut vtm as *mut vt_mode);
  //TODO: use set_termios_to_raw()
  tcgetattr(0i32, &mut oterm); /* ignore serial break (why? VTs don't have breaks, right?) */
  term = oterm; /* redundant? "dont translate break to SIGINT" */
  term.c_iflag |= 0o1i32 as libc::c_uint; /* ignore ^C ^Z, echo off */
  term.c_iflag &= !0o2i32 as libc::c_uint;
  term.c_lflag &= !(0o1i32 | 0o10i32 | 0o1000i32) as libc::c_uint;
  tcsetattr_stdin_TCSANOW(&mut term);
  loop {
    printf(
      b"Virtual console%s locked by %s.\n\x00" as *const u8 as *const libc::c_char,
      (b"s\x00" as *const u8 as *const libc::c_char)
        .offset((option_mask32 == 0) as libc::c_int as isize),
      (*pw).pw_name,
    );
    if ask_and_check_password(pw) > 0i32 {
      break;
    }
    bb_do_delay(3i32);
    puts(b"Incorrect password\x00" as *const u8 as *const libc::c_char);
  }
  ioctl(0i32, 0x5602i32 as libc::c_ulong, &mut ovtm as *mut vt_mode);
  tcsetattr_stdin_TCSANOW(&mut oterm);
  fflush_stdout_and_exit(0i32);
}

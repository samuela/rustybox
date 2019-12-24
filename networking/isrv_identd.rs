use crate::libbb::appletlib::applet_name;
use crate::libbb::ptr_to_globals::bb_errno;
use crate::librb::signal::__sighandler_t;
use crate::librb::smallint;
use crate::networking::isrv::isrv_state_t;
use libc;
use libc::alarm;
use libc::free;
use libc::openlog;
use libc::strcpy;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
  #[no_mangle]
  fn dprintf(__fd: libc::c_int, __fmt: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
  #[no_mangle]
  fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  static mut logmode: smallint;

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];

/* callbacks */

/* Driver:
 *
 * Select on listen_fd for <linger_timeout> (or forever if 0).
 *
 * If we time out and we have no peers, exit.
 * If we have peers, call do_timeout(peer_param),
 * if it returns !0, peer is removed.
 *
 * If listen_fd is active, accept new connection ("peer"),
 * call new_peer() on it, and if it returns 0,
 * add it to fds to select on.
 * Now, select will wait for <timeout>, not <linger_timeout>
 * (as long as we have more than zero peers).
 *
 * If a peer's fd is active, we call do_rd() on it if read
 * bit was set, and then do_wr() if write bit was also set.
 * If either returns !0, peer is removed.
 * Reaching this place also resets timeout counter for this peer.
 *
 * Note that peer must indicate that he wants to be selected
 * for read and/or write using isrv_want_rd()/isrv_want_wr()
 * [can be called in new_peer() or in do_rd()/do_wr()].
 * If it never wants to be selected for write, do_wr()
 * will never be called (can be NULL).
 */

}

pub type C2RustUnnamed = libc::c_uint;
pub const LOGMODE_BOTH: C2RustUnnamed = 3;
pub const LOGMODE_SYSLOG: C2RustUnnamed = 2;
pub const LOGMODE_STDIO: C2RustUnnamed = 1;
pub const LOGMODE_NONE: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const COMMON_BUFSIZE: C2RustUnnamed_0 = 1024;

/*
 * Fake identd server.
 *
 * Copyright (C) 2007 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config FAKEIDENTD
//config:	bool "fakeidentd (8.7 kb)"
//config:	default y
//config:	select FEATURE_SYSLOG
//config:	help
//config:	fakeidentd listens on the ident port and returns a predefined
//config:	fake value on any query.
//applet:IF_FAKEIDENTD(APPLET(fakeidentd, BB_DIR_USR_SBIN, SUID_DROP))
//kbuild:lib-$(CONFIG_FAKEIDENTD) += isrv_identd.o isrv.o
//usage:#define fakeidentd_trivial_usage
//usage:       "[-fiw] [-b ADDR] [STRING]"
//usage:#define fakeidentd_full_usage "\n\n"
//usage:       "Provide fake ident (auth) service\n"
//usage:     "\n	-f	Run in foreground"
//usage:     "\n	-i	Inetd mode"
//usage:     "\n	-w	Inetd 'wait' mode"
//usage:     "\n	-b ADDR	Bind to specified address"
//usage:     "\n	STRING	Ident answer string (default: nobody)"
pub type C2RustUnnamed_1 = libc::c_uint;
pub const TIMEOUT: C2RustUnnamed_1 = 20;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct identd_buf_t {
  pub pos: libc::c_int,
  pub buf: [libc::c_char; 60],
}
pub const OPT_inetdwait: C2RustUnnamed_2 = 4;
pub const OPT_inetd: C2RustUnnamed_2 = 2;
pub const OPT_fiw: C2RustUnnamed_2 = 7;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const OPT_bindaddr: C2RustUnnamed_2 = 8;
pub const OPT_foreground: C2RustUnnamed_2 = 1;
unsafe extern "C" fn new_peer(mut state: *mut isrv_state_t, mut fd: libc::c_int) -> libc::c_int {
  let mut peer: libc::c_int = 0; /* failure */
  let mut buf: *mut identd_buf_t = crate::libbb::xfuncs_printf::xzalloc(::std::mem::size_of::<
    identd_buf_t,
  >() as libc::c_ulong) as *mut identd_buf_t; /* failure, unregister peer */
  peer = crate::networking::isrv::isrv_register_peer(state, buf as *mut libc::c_void);
  if peer < 0 {
    return 0;
  }
  if crate::networking::isrv::isrv_register_fd(state, peer, fd) < 0 {
    return peer;
  }
  crate::libbb::xfuncs::ndelay_on(fd);
  crate::networking::isrv::isrv_want_rd(state, fd);
  return 0;
}
unsafe extern "C" fn do_rd(mut fd: libc::c_int, mut paramp: *mut *mut libc::c_void) -> libc::c_int {
  let mut buf: *mut identd_buf_t = *paramp as *mut identd_buf_t;
  let mut cur: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut sz: libc::c_int = 0;
  cur = (*buf).buf.as_mut_ptr().offset((*buf).pos as isize);
  sz = crate::libbb::read::safe_read(
    fd,
    cur as *mut libc::c_void,
    (::std::mem::size_of::<[libc::c_char; 60]>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong)
      .wrapping_sub((*buf).pos as libc::c_ulong),
  ) as libc::c_int;
  if sz < 0 {
    if !(*bb_errno != 11i32) {
      return 0;
    }
  /* "session is ok" */
  } else {
    (*buf).pos += sz; /* "session is ok" */
    (*buf).buf[(*buf).pos as usize] = '\u{0}' as i32 as libc::c_char;
    p = strpbrk(cur, b"\r\n\x00" as *const u8 as *const libc::c_char);
    if !p.is_null() {
      *p = '\u{0}' as i32 as libc::c_char
    }
    if p.is_null() && sz != 0 {
      return 0;
    }
    /* Terminate session. If we are in server mode, then
     * fd is still in nonblocking mode - we never block here */
    if fd == 0 {
      fd += 1
    } /* inetd mode? then write to fd 1 */
    dprintf(
      fd,
      b"%s : USERID : UNIX : %s\r\n\x00" as *const u8 as *const libc::c_char,
      (*buf).buf.as_mut_ptr(),
      bb_common_bufsiz1.as_mut_ptr(),
    );
  }
  /*
   * Why bother if we are going to close fd now anyway?
   * if (server)
   *	ndelay_off(fd);
   */
  free(buf as *mut libc::c_void);
  return 1i32;
  /* "terminate" */
}
unsafe extern "C" fn do_timeout(mut _paramp: *mut *mut libc::c_void) -> libc::c_int {
  return 1i32;
  /* terminate session */
}
unsafe extern "C" fn inetd_mode() {
  let mut buf: *mut identd_buf_t = crate::libbb::xfuncs_printf::xzalloc(::std::mem::size_of::<
    identd_buf_t,
  >() as libc::c_ulong) as *mut identd_buf_t;
  loop
  /* buf->pos = 0; - xzalloc did it */
  {
    alarm(TIMEOUT as libc::c_int as libc::c_uint);
    if !(do_rd(
      0,
      &mut buf as *mut *mut identd_buf_t as *mut libc::c_void as *mut *mut libc::c_void,
    ) == 0)
    {
      break;
    }
  }
}
#[no_mangle]
pub unsafe extern "C" fn fakeidentd_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut bind_address: *const libc::c_char = std::ptr::null();
  let mut opt: libc::c_uint = 0;
  let mut fd: libc::c_int = 0;
  opt = crate::libbb::getopt32::getopt32(
    argv,
    b"fiwb:\x00" as *const u8 as *const libc::c_char,
    &mut bind_address as *mut *const libc::c_char,
  );
  strcpy(
    bb_common_bufsiz1.as_mut_ptr(),
    b"nobody\x00" as *const u8 as *const libc::c_char,
  );
  if !(*argv.offset(optind as isize)).is_null() {
    strncpy(
      bb_common_bufsiz1.as_mut_ptr(),
      *argv.offset(optind as isize),
      (COMMON_BUFSIZE as libc::c_int - 1i32) as libc::c_ulong,
    );
  }
  /* Daemonize if no -f and no -i and no -w */
  if opt & OPT_fiw as libc::c_int as libc::c_uint == 0 {
    crate::libbb::vfork_daemon_rexec::bb_daemonize_or_rexec(0i32);
  }
  /* Where to log in inetd modes? "Classic" inetd
   * probably has its stderr /dev/null'ed (we need log to syslog?),
   * but daemontools-like utilities usually expect that children
   * log to stderr. I like daemontools more. Go their way.
   * (Or maybe we need yet another option "log to syslog") */
  if opt & OPT_fiw as libc::c_int as libc::c_uint == 0 {
    /* || (opt & OPT_syslog) */
    openlog(applet_name, 0x1i32, 3i32 << 3i32);
    logmode = LOGMODE_SYSLOG as libc::c_int as smallint
  }
  if opt & OPT_inetd as libc::c_int as libc::c_uint != 0 {
    inetd_mode();
    return 0;
  }
  /* Ignore closed connections when writing */
  signal(
    13i32,
    ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
  );
  fd = 0;
  if opt & OPT_inetdwait as libc::c_int as libc::c_uint == 0 {
    fd = crate::libbb::xconnect::create_and_bind_stream_or_die(bind_address, 113i32);
    crate::libbb::xfuncs_printf::xlisten(fd, 5i32);
  }
  crate::networking::isrv::isrv_run(
    fd,
    Some(new_peer as unsafe extern "C" fn(_: *mut isrv_state_t, _: libc::c_int) -> libc::c_int),
    Some(do_rd as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_void) -> libc::c_int),
    None,
    Some(do_timeout as unsafe extern "C" fn(_: *mut *mut libc::c_void) -> libc::c_int),
    TIMEOUT as libc::c_int,
    if opt & OPT_inetdwait as libc::c_int as libc::c_uint != 0 {
      TIMEOUT as libc::c_int
    } else {
      0
    },
  );
  return 0;
}

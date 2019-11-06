use libc;
extern "C" {
  #[no_mangle]
  fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn dup(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  /* Search for an entry with a matching user ID.  */
  #[no_mangle]
  fn bb_internal_getpwuid(__uid: uid_t) -> *mut passwd;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  static mut logmode: smallint;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_simple_info_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_do_delay(seconds: libc::c_int);
  #[no_mangle]
  fn run_shell(
    shell: *const libc::c_char,
    loginshell: libc::c_int,
    args: *mut *const libc::c_char,
  ) -> !;
  #[no_mangle]
  fn ask_and_check_password_extended(
    pw: *const passwd,
    timeout: libc::c_int,
    prompt: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  fn openlog(__ident: *const libc::c_char, __option: libc::c_int, __facility: libc::c_int);
}

use crate::librb::passwd;
use crate::librb::smallint;
use libc::uid_t;

pub type C2RustUnnamed = libc::c_uint;
pub const LOGMODE_BOTH: C2RustUnnamed = 3;
pub const LOGMODE_SYSLOG: C2RustUnnamed = 2;
pub const LOGMODE_STDIO: C2RustUnnamed = 1;
pub const LOGMODE_NONE: C2RustUnnamed = 0;

/*
 * Mini sulogin implementation for busybox
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config SULOGIN
//config:	bool "sulogin (17 kb)"
//config:	default y
//config:	select FEATURE_SYSLOG
//config:	help
//config:	sulogin is invoked when the system goes into single user
//config:	mode (this is done through an entry in inittab).
//applet:IF_SULOGIN(APPLET_NOEXEC(sulogin, sulogin, BB_DIR_SBIN, BB_SUID_DROP, sulogin))
//kbuild:lib-$(CONFIG_SULOGIN) += sulogin.o
//usage:#define sulogin_trivial_usage
//usage:       "[-t N] [TTY]"
//usage:#define sulogin_full_usage "\n\n"
//usage:       "Single user login\n"
//usage:     "\n	-t N	Timeout"
#[no_mangle]
pub unsafe extern "C" fn sulogin_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut timeout: libc::c_int = 0i32;
  let mut pwd: *mut passwd = 0 as *mut passwd;
  let mut shell: *const libc::c_char = 0 as *const libc::c_char;
  /* Note: sulogin is not a suid app. It is meant to be run by init
   * for single user / emergency mode. init starts it as root.
   * Normal users (potentially malicious ones) can only run it under
   * their UID, therefore no paranoia here is warranted:
   * $LD_LIBRARY_PATH in env, TTY = /dev/sda
   * are no more dangerous here than in e.g. cp applet.
   */
  logmode = LOGMODE_BOTH as libc::c_int as smallint;
  openlog(applet_name, 0i32, 4i32 << 3i32);
  getopt32(
    argv,
    b"t:+\x00" as *const u8 as *const libc::c_char,
    &mut timeout as *mut libc::c_int,
  );
  argv = argv.offset(optind as isize);
  if !(*argv.offset(0)).is_null() {
    close(0i32);
    close(1i32);
    dup(xopen(*argv.offset(0), 0o2i32));
    close(2i32);
    dup(0i32);
  }
  pwd = bb_internal_getpwuid(0i32 as uid_t);
  if pwd.is_null() {
    bb_simple_error_msg_and_die(
      b"no password entry for root\x00" as *const u8 as *const libc::c_char,
    );
  }
  loop {
    let mut r: libc::c_int = 0;
    r = ask_and_check_password_extended(
      pwd,
      timeout,
      b"Give root password for system maintenance\n(or type Control-D for normal startup):\x00"
        as *const u8 as *const libc::c_char,
    );
    if r < 0i32 {
      /* ^D, ^C, timeout, or read error */
      bb_simple_info_msg(b"normal startup\x00" as *const u8 as *const libc::c_char);
      return 0i32;
    }
    if r > 0i32 {
      break;
    }
    bb_do_delay(3i32);
    bb_simple_info_msg(b"Login incorrect\x00" as *const u8 as *const libc::c_char);
  }
  bb_simple_info_msg(
    b"starting shell for system maintenance\x00" as *const u8 as *const libc::c_char,
  );
  shell = getenv(b"SUSHELL\x00" as *const u8 as *const libc::c_char);
  if shell.is_null() {
    shell = getenv(b"sushell\x00" as *const u8 as *const libc::c_char)
  }
  if shell.is_null() {
    shell = (*pwd).pw_shell
  }
  /* Exec login shell with no additional parameters. Never returns. */
  run_shell(shell, 1i32, 0 as *mut *const libc::c_char);
}

use crate::libbb::appletlib::applet_name;
use libc;
use libc::closelog;
use libc::getenv;
use libc::getuid;
use libc::openlog;
use libc::strcmp;
use libc::syslog;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;

  /* Search for an entry with a matching user ID.  */

  #[no_mangle]
  fn endusershell();
  #[no_mangle]
  fn getusershell() -> *mut libc::c_char;
  #[no_mangle]
  fn getlogin_r(__name: *mut libc::c_char, __name_len: size_t) -> libc::c_int;

}

use crate::librb::size_t;
use libc::passwd;
use libc::uid_t;
/*
 * Mini su implementation for busybox
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config SU
//config:	bool "su (19 kb)"
//config:	default y
//config:	select FEATURE_SYSLOG
//config:	help
//config:	su is used to become another user during a login session.
//config:	Invoked without a username, su defaults to becoming the super user.
//config:	Note that busybox binary must be setuid root for this applet to
//config:	work properly.
//config:
//config:config FEATURE_SU_SYSLOG
//config:	bool "Log to syslog all attempts to use su"
//config:	default y
//config:	depends on SU
//config:
//config:config FEATURE_SU_CHECKS_SHELLS
//config:	bool "If user's shell is not in /etc/shells, disallow -s PROG"
//config:	default y
//config:	depends on SU
//config:
//config:config FEATURE_SU_BLANK_PW_NEEDS_SECURE_TTY
//config:	bool "Allow blank passwords only on TTYs in /etc/securetty"
//config:	default n
//config:	depends on SU
//applet:/* Needs to be run by root or be suid root - needs to change uid and gid: */
//applet:IF_SU(APPLET(su, BB_DIR_BIN, SUID_REQUIRE))
//kbuild:lib-$(CONFIG_SU) += su.o
//usage:#define su_trivial_usage
//usage:       "[-lmp] [-] [-s SH] [USER [SCRIPT ARGS / -c 'CMD' ARG0 ARGS]]"
//usage:#define su_full_usage "\n\n"
//usage:       "Run shell under USER (by default, root)\n"
//usage:     "\n	-,-l	Clear environment, go to home dir, run shell as login shell"
//usage:     "\n	-p,-m	Do not set new $HOME, $SHELL, $USER, $LOGNAME"
//usage:     "\n	-c CMD	Command to pass to 'sh -c'"
//usage:     "\n	-s SH	Shell to use instead of user's default"
/* Return 1 if SHELL is a restricted shell (one not returned by
 * getusershell), else 0, meaning it is a standard shell.  */
unsafe fn restricted_shell(mut shell: *const libc::c_char) -> libc::c_int {
  let mut line: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut result: libc::c_int = 1i32;
  loop
  /*setusershell(); - getusershell does it itself*/
  {
    line = getusershell();
    if line.is_null() {
      break;
    }
    if !(strcmp(line, shell) == 0) {
      continue;
    }
    result = 0;
    break;
  }
  return result;
}
pub unsafe fn su_main(mut _argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut flags: libc::c_uint = 0;
  let mut opt_shell: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut opt_command: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut opt_username: *const libc::c_char = b"root\x00" as *const u8 as *const libc::c_char;
  let mut pw: *mut passwd = std::ptr::null_mut();
  let mut cur_uid: uid_t = getuid();
  let mut tty: *const libc::c_char = std::ptr::null();
  let mut user_buf: [libc::c_char; 64] = [0; 64];
  let mut old_user: *const libc::c_char = std::ptr::null();
  let mut r: libc::c_int = 0;
  /* Note: we don't use "'+': stop at first non-option" idiom here.
   * For su, "SCRIPT ARGS" or "-c CMD ARGS" do not stop option parsing:
   * ARGS starting with dash will be treated as su options,
   * not passed to shell. (Tested on util-linux 2.28).
   */
  flags = crate::libbb::getopt32::getopt32(
    argv,
    b"mplc:s:\x00" as *const u8 as *const libc::c_char,
    &mut opt_command as *mut *mut libc::c_char,
    &mut opt_shell as *mut *mut libc::c_char,
  );
  argv = argv.offset(optind as isize);
  if !(*argv.offset(0)).is_null()
    && (*(*argv.offset(0)).offset(0) as libc::c_int == '-' as i32
      && *(*argv.offset(0)).offset(1) == 0)
  {
    flags |= 4i32 as libc::c_uint;
    argv = argv.offset(1)
  }
  /* get user if specified */
  if !(*argv.offset(0)).is_null() {
    opt_username = *argv.offset(0);
    argv = argv.offset(1)
  }
  tty = crate::libbb::xfuncs_printf::xmalloc_ttyname(0i32);
  if tty.is_null() {
    tty = b"none\x00" as *const u8 as *const libc::c_char
  }
  tty = crate::libbb::skip_whitespace::skip_dev_pfx(tty);
  /* The utmp entry (via getlogin) is probably the best way to
   * identify the user, especially if someone su's from a su-shell.
   * But getlogin can fail -- usually due to lack of utmp entry.
   * in this case resort to getpwuid.  */
  old_user = user_buf.as_mut_ptr();
  if getlogin_r(
    user_buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
  ) != 0
  {
    pw = crate::libpwdgrp::pwd_grp::bb_internal_getpwuid(cur_uid);
    old_user = if !pw.is_null() {
      crate::libbb::xfuncs_printf::xstrdup((*pw).pw_name)
    } else {
      b"\x00" as *const u8 as *const libc::c_char
    }
  }
  openlog(applet_name, 0, 4i32 << 3i32);
  pw = crate::libbb::bb_pwd::xgetpwnam(opt_username);
  r = 1i32;
  if cur_uid != 0 as libc::c_uint {
    r = crate::libbb::correct_password::ask_and_check_password(pw)
  }
  's_191: {
    if r > 0 {
      if !(0i32 != 0 && r == 2i32 && crate::libbb::securetty::is_tty_secure(tty) == 0) {
        syslog(
          5i32,
          b"%c %s %s:%s\x00" as *const u8 as *const libc::c_char,
          '+' as i32,
          tty,
          old_user,
          opt_username,
        );
        break 's_191;
      }
    }
    syslog(
      5i32,
      b"%c %s %s:%s\x00" as *const u8 as *const libc::c_char,
      '-' as i32,
      tty,
      old_user,
      opt_username,
    );
    crate::libbb::bb_do_delay::bb_do_delay(3i32);
    crate::libbb::verror_msg::bb_simple_error_msg_and_die(
      b"incorrect password\x00" as *const u8 as *const libc::c_char,
    );
  }
  if false && 1i32 != 0 {
    closelog();
  }
  if opt_shell.is_null() && flags & 3i32 as libc::c_uint != 0 {
    /* -s SHELL is not given, but "preserve env" opt is */
    opt_shell = getenv(b"SHELL\x00" as *const u8 as *const libc::c_char)
  }
  if !opt_shell.is_null()
    && cur_uid != 0 as libc::c_uint
    && !(*pw).pw_shell.is_null()
    && restricted_shell((*pw).pw_shell) != 0
  {
    /* The user being su'd to has a nonstandard shell, and so is
     * probably a uucp account or has restricted access.  Don't
     * compromise the account by allowing access with a standard
     * shell.  */
    crate::libbb::verror_msg::bb_simple_error_msg(
      b"using restricted shell\x00" as *const u8 as *const libc::c_char,
    );
    opt_shell = std::ptr::null_mut::<libc::c_char>()
    /* ignore -s PROG */
  }
  /* else: user can run whatever he wants via "su -s PROG USER".
   * This is safe since PROG is run under user's uid/gid. */
  if opt_shell.is_null() {
    opt_shell = (*pw).pw_shell
  }
  crate::libbb::change_identity::change_identity(pw);
  crate::libbb::setup_environment::setup_environment(
    opt_shell,
    (flags & 4i32 as libc::c_uint)
      .wrapping_div(4i32 as libc::c_uint)
      .wrapping_mul((1i32 << 1i32) as libc::c_uint)
      .wrapping_add(
        ((flags & 3i32 as libc::c_uint == 0) as libc::c_int * (1i32 << 0)) as libc::c_uint,
      )
      .wrapping_add(
        ((flags & 4i32 as libc::c_uint == 0) as libc::c_int * (1i32 << 4i32)) as libc::c_uint,
      ) as libc::c_int,
    pw,
  );
  if !opt_command.is_null() {
    argv = argv.offset(-1);
    *argv = opt_command;
    argv = argv.offset(-1);
    *argv = b"-c\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }
  /* A nasty ioctl exists which can stuff data into input queue:
   * #include <sys/ioctl.h>
   * int main() {
   *	const char *msg = "echo $UID\n";
   *	while (*msg) ioctl(0, TIOCSTI, *msg++);
   *	return 0;
   * }
   * With "su USER -c EXPLOIT" run by root, exploit can make root shell
   * read as input and execute arbitrary command.
   * It's debatable whether we need to protect against this
   * (root may hesitate to run unknown scripts interactively).
   *
   * Some versions of su run -c CMD in a different session:
   * ioctl(TIOCSTI) works only on the controlling tty.
   */
  /* Never returns */
  crate::libbb::run_shell::run_shell(
    opt_shell,
    (flags & 4i32 as libc::c_uint) as libc::c_int,
    argv as *mut *const libc::c_char,
  );
  /* return EXIT_FAILURE; - not reached */
}

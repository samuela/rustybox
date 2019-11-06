use libc;
extern "C" {
  #[no_mangle]
  fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn getuid() -> __uid_t;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  /* Search for an entry with a matching user ID.  */
  #[no_mangle]
  fn bb_internal_getpwuid(__uid: uid_t) -> *mut passwd;
  #[no_mangle]
  fn skip_dev_pfx(tty_name: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xgetpwnam(name: *const libc::c_char) -> *mut passwd;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
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
  fn xmalloc_ttyname(fd: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn endusershell();
  #[no_mangle]
  fn getusershell() -> *mut libc::c_char;
  #[no_mangle]
  fn getlogin_r(__name: *mut libc::c_char, __name_len: size_t) -> libc::c_int;
  #[no_mangle]
  fn closelog();
  #[no_mangle]
  fn openlog(__ident: *const libc::c_char, __option: libc::c_int, __facility: libc::c_int);
  #[no_mangle]
  fn syslog(__pri: libc::c_int, __fmt: *const libc::c_char, _: ...);
}

use crate::librb::__uid_t;
use crate::librb::passwd;
use crate::librb::size_t;
use libc::uid_t;
use libc::uint32_t;

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
//applet:IF_SU(APPLET(su, BB_DIR_BIN, BB_SUID_REQUIRE))
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
unsafe extern "C" fn restricted_shell(mut shell: *const libc::c_char) -> libc::c_int {
  let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut result: libc::c_int = 1i32;
  loop
  /*setusershell(); - getusershell does it itself*/
  {
    line = getusershell();
    if line.is_null() {
      break;
    }
    if !(strcmp(line, shell) == 0i32) {
      continue;
    }
    result = 0i32;
    break;
  }
  return result;
}
#[no_mangle]
pub unsafe extern "C" fn su_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut flags: libc::c_uint = 0;
  let mut opt_shell: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opt_command: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opt_username: *const libc::c_char = b"root\x00" as *const u8 as *const libc::c_char;
  let mut pw: *mut passwd = 0 as *mut passwd;
  let mut cur_uid: uid_t = getuid();
  let mut tty: *const libc::c_char = 0 as *const libc::c_char;
  let mut user_buf: [libc::c_char; 64] = [0; 64];
  let mut old_user: *const libc::c_char = 0 as *const libc::c_char;
  let mut r: libc::c_int = 0;
  /* Note: we don't use "'+': stop at first non-option" idiom here.
   * For su, "SCRIPT ARGS" or "-c CMD ARGS" do not stop option parsing:
   * ARGS starting with dash will be treated as su options,
   * not passed to shell. (Tested on util-linux 2.28).
   */
  flags = getopt32(
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
  tty = xmalloc_ttyname(0i32);
  if tty.is_null() {
    tty = b"none\x00" as *const u8 as *const libc::c_char
  }
  tty = skip_dev_pfx(tty);
  /* The utmp entry (via getlogin) is probably the best way to
   * identify the user, especially if someone su's from a su-shell.
   * But getlogin can fail -- usually due to lack of utmp entry.
   * in this case resort to getpwuid.  */
  old_user = user_buf.as_mut_ptr();
  if getlogin_r(
    user_buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
  ) != 0i32
  {
    pw = bb_internal_getpwuid(cur_uid);
    old_user = if !pw.is_null() {
      xstrdup((*pw).pw_name)
    } else {
      b"\x00" as *const u8 as *const libc::c_char
    }
  }
  openlog(applet_name, 0i32, 4i32 << 3i32);
  pw = xgetpwnam(opt_username);
  r = 1i32;
  if cur_uid != 0i32 as libc::c_uint {
    r = ask_and_check_password(pw)
  }
  's_191: {
    if r > 0i32 {
      if !(0i32 != 0 && r == 2i32 && is_tty_secure(tty) == 0) {
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
    bb_do_delay(3i32);
    bb_simple_error_msg_and_die(b"incorrect password\x00" as *const u8 as *const libc::c_char);
  }
  if 0i32 != 0 && 1i32 != 0 {
    closelog();
  }
  if opt_shell.is_null() && flags & 3i32 as libc::c_uint != 0 {
    /* -s SHELL is not given, but "preserve env" opt is */
    opt_shell = getenv(b"SHELL\x00" as *const u8 as *const libc::c_char)
  }
  if !opt_shell.is_null()
    && cur_uid != 0i32 as libc::c_uint
    && !(*pw).pw_shell.is_null()
    && restricted_shell((*pw).pw_shell) != 0
  {
    /* The user being su'd to has a nonstandard shell, and so is
     * probably a uucp account or has restricted access.  Don't
     * compromise the account by allowing access with a standard
     * shell.  */
    bb_simple_error_msg(b"using restricted shell\x00" as *const u8 as *const libc::c_char);
    opt_shell = 0 as *mut libc::c_char
    /* ignore -s PROG */
  }
  /* else: user can run whatever he wants via "su -s PROG USER".
   * This is safe since PROG is run under user's uid/gid. */
  if opt_shell.is_null() {
    opt_shell = (*pw).pw_shell
  }
  change_identity(pw);
  setup_environment(
    opt_shell,
    (flags & 4i32 as libc::c_uint)
      .wrapping_div(4i32 as libc::c_uint)
      .wrapping_mul((1i32 << 1i32) as libc::c_uint)
      .wrapping_add(
        ((flags & 3i32 as libc::c_uint == 0) as libc::c_int * (1i32 << 0i32)) as libc::c_uint,
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
  run_shell(
    opt_shell,
    (flags & 4i32 as libc::c_uint) as libc::c_int,
    argv as *mut *const libc::c_char,
  );
  /* return EXIT_FAILURE; - not reached */
}

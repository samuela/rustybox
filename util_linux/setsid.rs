use libc;
extern "C" {
  #[no_mangle]
  fn setsid() -> __pid_t;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
  #[no_mangle]
  fn BB_EXECVP_or_die(argv: *mut *mut libc::c_char) -> !;
  #[no_mangle]
  fn xfork() -> pid_t;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
}
pub type __uint32_t = libc::c_uint;
pub type __pid_t = libc::c_int;
use crate::librb::uint32_t;
use crate::librb::pid_t;

/*
 * setsid.c -- execute a command in a new session
 * Rick Sladkey <jrs@world.std.com>
 * In the public domain.
 *
 * 1999-02-22 Arkadiusz Mickiewicz <misiek@pld.ORG.PL>
 * - added Native Language Support
 *
 * 2001-01-18 John Fremlin <vii@penguinpowered.com>
 * - fork in case we are process group leader
 *
 * 2004-11-12 Paul Fox
 * - busyboxed
 */
//config:config SETSID
//config:	bool "setsid (3.6 kb)"
//config:	default y
//config:	help
//config:	setsid runs a program in a new session
//applet:IF_SETSID(APPLET(setsid, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_SETSID) += setsid.o
//usage:#define setsid_trivial_usage
//usage:       "[-c] PROG ARGS"
//usage:#define setsid_full_usage "\n\n"
//usage:       "Run PROG in a new session. PROG will have no controlling terminal\n"
//usage:       "and will not be affected by keyboard signals (^C etc).\n"
//usage:     "\n	-c	Set controlling terminal to stdin"
#[no_mangle]
pub unsafe extern "C" fn setsid_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opt: libc::c_uint = 0;
  /* +: stop on first non-opt */
  opt = getopt32(argv, b"^+c\x00-1\x00" as *const u8 as *const libc::c_char);
  argv = argv.offset(optind as isize);
  /* setsid() is allowed only when we are not a process group leader.
   * Otherwise our PID serves as PGID of some existing process group
   * and cannot be used as PGID of a new process group.
   *
   * Example: setsid() below fails when run alone in interactive shell:
   *  $ setsid PROG
   * because shell's child (setsid) is put in a new process group.
   * But doesn't fail if shell is not interactive
   * (and therefore doesn't create process groups for pipes),
   * or if setsid is not the first process in the process group:
   *  $ true | setsid PROG
   * or if setsid is executed in backquotes (`setsid PROG`)...
   */
  if setsid() < 0i32 {
    let mut pid: pid_t = xfork();
    if pid != 0i32 {
      /* parent */
      /* TODO:
       * we can waitpid(pid, &status, 0) and then even
       * emulate exitcode, making the behavior consistent
       * in both forked and non forked cases.
       * However, the code is larger and upstream
       * does not do such trick.
       */
      return 0i32;
    }
    /* child */
    /* now there should be no error: */
    setsid();
  }
  if opt != 0 {
    /* -c: set (with stealing) controlling tty */
    ioctl(0i32, 0x540ei32 as libc::c_ulong, 1i32);
  }
  BB_EXECVP_or_die(argv);
}

use libc;
use libc::unlink;


use libc::pid_t;

extern "C" {
  #[no_mangle]
  fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn getppid() -> pid_t;
  #[no_mangle]
  fn vfork() -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn kill(__pid: pid_t, __sig: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn wait(__stat_loc: *mut libc::c_int) -> pid_t;
  #[no_mangle]
  fn parse_duration_str(str: *mut libc::c_char) -> duration_t;
  #[no_mangle]
  fn BB_EXECVP_or_die(argv: *mut *mut libc::c_char) -> !;
  #[no_mangle]
  fn bb_daemonize_or_rexec(flags: libc::c_int);
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn get_signum(name: *const libc::c_char) -> libc::c_int;
}

pub type duration_t = libc::c_double;
/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
 * COPYING NOTES
 *
 * timeout.c -- a timeout handler for shell commands
 *
 * Copyright (C) 2005-6, Roberto A. Foglietta <me@roberto.foglietta.name>
 *
 *   This program is free software; you can redistribute it and/or modify
 *   it under the terms of the GNU General Public License as published by
 *   the Free Software Foundation; version 2 of the License.
 *
 *   This program is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU General Public License for more details.
 *
 *   You should have received a copy of the GNU General Public License
 *   along with this program; if not, write to the Free Software
 *   Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307, USA.
 */
/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
 * REVISION NOTES:
 * released 17-11-2005 by Roberto A. Foglietta
 * talarm   04-12-2005 by Roberto A. Foglietta
 * modified 05-12-2005 by Roberto A. Foglietta
 * sizerdct 06-12-2005 by Roberto A. Foglietta
 * splitszf 12-05-2006 by Roberto A. Foglietta
 * rewrite  14-11-2008 vda
 */
//config:config TIMEOUT
//config:	bool "timeout (6 kb)"
//config:	default y
//config:	help
//config:	Runs a program and watches it. If it does not terminate in
//config:	specified number of seconds, it is sent a signal.
//applet:IF_TIMEOUT(APPLET(timeout, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_TIMEOUT) += timeout.o
//usage:#define timeout_trivial_usage
//usage:       "[-s SIG] SECS PROG ARGS"
//usage:#define timeout_full_usage "\n\n"
//usage:       "Runs PROG. Sends SIG to it if it is not gone in SECS seconds.\n"
//usage:       "Default SIG: TERM."
#[no_mangle]
pub unsafe extern "C" fn timeout_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut signo: libc::c_int = 0;
  let mut status: libc::c_int = 0;
  let mut parent: libc::c_int = 0i32;
  let mut timeout: libc::c_int = 0;
  let mut pid: pid_t = 0;
  let mut opt_s: *const libc::c_char = b"TERM\x00" as *const u8 as *const libc::c_char;
  /* -p option is not documented, it is needed to support NOMMU. */
  /* -t SECONDS; -p PARENT_PID */
  /* '+': stop at first non-option */
  getopt32(
    argv,
    b"+s:\x00" as *const u8 as *const libc::c_char,
    &mut opt_s as *mut *const libc::c_char,
    &mut parent as *mut libc::c_int,
  );
  /*argv += optind; - no, wait for bb_daemonize_or_rexec! */
  signo = get_signum(opt_s);
  if signo < 0i32 {
    bb_error_msg_and_die(
      b"unknown signal \'%s\'\x00" as *const u8 as *const libc::c_char,
      opt_s,
    );
  }
  if (*argv.offset(optind as isize)).is_null() {
    bb_show_usage();
  }
  let fresh0 = optind;
  optind = optind + 1;
  timeout = parse_duration_str(*argv.offset(fresh0 as isize)) as libc::c_int;
  if (*argv.offset(optind as isize)).is_null() {
    /* no PROG? */
    bb_show_usage();
  }
  /* We want to create a grandchild which will watch
   * and kill the grandparent. Other methods:
   * making parent watch child disrupts parent<->child link
   * (example: "tcpsvd 0.0.0.0 1234 timeout service_prog" -
   * it's better if service_prog is a child of tcpsvd!),
   * making child watch parent results in programs having
   * unexpected children.	*/
  if !(parent != 0) {
    pid = {
      let mut bb__xvfork_pid: pid_t = vfork();
      if bb__xvfork_pid < 0i32 {
        bb_simple_perror_msg_and_die(b"vfork\x00" as *const u8 as *const libc::c_char);
      }
      bb__xvfork_pid
    };
    if pid == 0i32 {
      /* Child: spawn grandchild and exit */
      parent = getppid();
      /* NB: exits with nonzero on error: */
      bb_daemonize_or_rexec(0i32);
    } else {
      /* Parent */
      wait(&mut status); /* wait for child to die */
      /* Did intermediate [v]fork or exec fail? */
      if !(status & 0x7fi32 == 0i32) || (status & 0xff00i32) >> 8i32 != 0i32 {
        return 1i32;
      }
      /* Ok, exec a program as requested */
      argv = argv.offset(optind as isize);
      BB_EXECVP_or_die(argv);
    }
  }
  /* Here we are grandchild. Sleep, then kill grandparent */
  loop
  /* Just sleep(HUGE_NUM); kill(parent) may kill wrong process! */
  {
    sleep(1i32 as libc::c_uint);
    timeout -= 1;
    if timeout <= 0i32 {
      break;
    }
    if kill(parent, 0i32) != 0 {
      /* process is gone */
      return 0i32;
    }
  }
  kill(parent, signo);
  return 0i32;
}

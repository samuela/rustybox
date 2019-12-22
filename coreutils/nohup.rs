use crate::librb::signal::__sighandler_t;
use libc;
use libc::close;
use libc::dup2;
use libc::getenv;
use libc::isatty;
use libc::open;
extern "C" {

  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;

  #[no_mangle]
  static mut xfunc_error_retval: u8;

}

/*
 * nohup - invoke a utility immune to hangups.
 *
 * Busybox version based on nohup specification at
 * http://www.opengroup.org/onlinepubs/007904975/utilities/nohup.html
 *
 * Copyright 2006 Rob Landley <rob@landley.net>
 * Copyright 2006 Bernhard Reutner-Fischer
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config NOHUP
//config:	bool "nohup (2 kb)"
//config:	default y
//config:	help
//config:	run a command immune to hangups, with output to a non-tty.
//applet:IF_NOHUP(APPLET_NOEXEC(nohup, nohup, BB_DIR_USR_BIN, SUID_DROP, nohup))
//kbuild:lib-$(CONFIG_NOHUP) += nohup.o
//usage:#define nohup_trivial_usage
//usage:       "PROG ARGS"
//usage:#define nohup_full_usage "\n\n"
//usage:       "Run PROG immune to hangups, with output to a non-tty"
//usage:
//usage:#define nohup_example_usage
//usage:       "$ nohup make &"
/* Compat info: nohup (GNU coreutils 6.8) does this:
# nohup true
nohup: ignoring input and appending output to 'nohup.out'
# nohup true 1>/dev/null
nohup: ignoring input and redirecting stderr to stdout
# nohup true 2>zz
# cat zz
nohup: ignoring input and appending output to 'nohup.out'
# nohup true 2>zz 1>/dev/null
# cat zz
nohup: ignoring input
# nohup true </dev/null 1>/dev/null
nohup: redirecting stderr to stdout
# nohup true </dev/null 2>zz 1>/dev/null
# cat zz
  (nothing)
#
*/
#[no_mangle]
pub unsafe extern "C" fn nohup_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut nohupout: *const libc::c_char = std::ptr::null();
  let mut home: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  xfunc_error_retval = 127i32 as u8;
  if (*argv.offset(1)).is_null() {
    crate::libbb::appletlib::bb_show_usage();
  }
  /* If stdin is a tty, detach from it. */
  if isatty(0i32) != 0 {
    /* bb_error_msg("ignoring input"); */
    close(0i32);
    crate::libbb::xfuncs_printf::xopen(b"/dev/null\x00" as *const u8 as *const libc::c_char, 0i32);
    /* will be fd 0 (STDIN_FILENO) */
  }
  nohupout = b"nohup.out\x00" as *const u8 as *const libc::c_char;
  /* Redirect stdout to nohup.out, either in "." or in "$HOME". */
  if isatty(1i32) != 0 {
    close(1i32);
    if open(nohupout, 0o100i32 | 0o1i32 | 0o2000i32, 0o400i32 | 0o200i32) < 0i32 {
      home = getenv(b"HOME\x00" as *const u8 as *const libc::c_char);
      if !home.is_null() {
        nohupout = crate::libbb::concat_path_file::concat_path_file(home, nohupout);
        crate::libbb::xfuncs_printf::xopen3(
          nohupout,
          0o100i32 | 0o1i32 | 0o2000i32,
          0o400i32 | 0o200i32,
        );
      } else {
        crate::libbb::xfuncs_printf::xopen(
          b"/dev/null\x00" as *const u8 as *const libc::c_char,
          0i32,
        );
        /* will be fd 1 */
      }
    }
    crate::libbb::verror_msg::bb_error_msg(
      b"appending output to %s\x00" as *const u8 as *const libc::c_char,
      nohupout,
    );
  }
  /* If we have a tty on stderr, redirect to stdout. */
  if isatty(2i32) != 0 {
    /* if (stdout_wasnt_a_tty)
    bb_error_msg("redirecting stderr to stdout"); */
    dup2(1i32, 2i32);
  }
  signal(
    1i32,
    ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
  );
  argv = argv.offset(1);
  crate::libbb::executable::BB_EXECVP_or_die(argv);
}

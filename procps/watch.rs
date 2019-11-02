use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn system(__command: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strftime_YYYYMMDDHHMMSS(
    buf: *mut libc::c_char,
    len: libc::c_uint,
    tp: *mut time_t,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn sleep_for_duration(duration: duration_t);
  #[no_mangle]
  fn parse_duration_str(str: *mut libc::c_char) -> duration_t;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn get_terminal_width(fd: libc::c_int) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type time_t = __time_t;
pub type duration_t = libc::c_double;
/* vi: set sw=4 ts=4: */
/*
 * Mini watch implementation for busybox
 *
 * Copyright (C) 2001 by Michael Habermann <mhabermann@gmx.de>
 * Copyrigjt (C) Mar 16, 2003 Manuel Novoa III   (mjn3@codepoet.org)
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config WATCH
//config:	bool "watch (4.4 kb)"
//config:	default y
//config:	help
//config:	watch is used to execute a program periodically, showing
//config:	output to the screen.
//applet:IF_WATCH(APPLET(watch, BB_DIR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_WATCH) += watch.o
//usage:#define watch_trivial_usage
//usage:       "[-n SEC] [-t] PROG ARGS"
//usage:#define watch_full_usage "\n\n"
//usage:       "Run PROG periodically\n"
//usage:     "\n	-n SEC	Loop period (default 2)"
//usage:     "\n	-t	Don't print header"
//usage:
//usage:#define watch_example_usage
//usage:       "$ watch date\n"
//usage:       "Mon Dec 17 10:31:40 GMT 2000\n"
//usage:       "Mon Dec 17 10:31:42 GMT 2000\n"
//usage:       "Mon Dec 17 10:31:44 GMT 2000"
/* BB_AUDIT SUSv3 N/A */
/* BB_AUDIT GNU defects -- only option -n is supported. */
// procps 2.0.18:
// watch [-d] [-n seconds]
//   [--differences[=cumulative]] [--interval=seconds] command
//
// procps-3.2.3:
// watch [-dt] [-n seconds]
//   [--differences[=cumulative]] [--interval=seconds] [--no-title] command
//
// (procps 3.x and procps 2.x are forks, not newer/older versions of the same)
#[no_mangle]
pub unsafe extern "C" fn watch_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut period: duration_t = 0.;
  let mut period_str: *mut libc::c_char =
    b"2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
  let mut opt: libc::c_uint = 0;
  let mut width: libc::c_uint = 0;
  let mut new_width: libc::c_uint = 0;
  let mut header: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
  // maybe ENABLE_DESKTOP?
  // "+": stop at first non-option (procps 3.x only); -n NUM
  // at least one param
  opt = getopt32(
    argv,
    b"^+dtn:\x00-1\x00" as *const u8 as *const libc::c_char,
    &mut period_str as *mut *mut libc::c_char,
  );
  argv = argv.offset(optind as isize);
  // watch from both procps 2.x and 3.x does concatenation. Example:
  // watch ls -l "a /tmp" "2>&1" - ls won't see "a /tmp" as one param
  cmd = *argv; // leaks cmd
  loop {
    argv = argv.offset(1); // make sure first time new_width != width
    if (*argv).is_null() {
      break;
    }
    cmd = xasprintf(b"%s %s\x00" as *const u8 as *const libc::c_char, cmd, *argv)
  }
  period = parse_duration_str(period_str);
  width = -1i32 as libc::c_uint;
  header = 0 as *mut libc::c_char;
  loop {
    /* home; clear to the end of screen */
    printf(b"\x1b[H\x1b[J\x00" as *const u8 as *const libc::c_char);
    if opt & 0x2i32 as libc::c_uint == 0 {
      // no -t
      let time_len: libc::c_uint =
        ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong as libc::c_uint;
      // STDERR_FILENO is procps3 compat:
      // "watch ls 2>/dev/null" does not detect tty size
      new_width = get_terminal_width(2i32) as libc::c_uint;
      if new_width != width {
        width = new_width;
        free(header as *mut libc::c_void);
        header = xasprintf(
          b"Every %.1fs: %-*s\x00" as *const u8 as *const libc::c_char,
          period,
          width as libc::c_int,
          cmd,
        )
      }
      if time_len < width {
        strftime_YYYYMMDDHHMMSS(
          header.offset(width as isize).offset(-(time_len as isize)),
          time_len,
          0 as *mut time_t,
        );
      }
      // compat: empty line between header and cmd output
      printf(b"%s\n\n\x00" as *const u8 as *const libc::c_char, header);
    }
    fflush_all();
    // TODO: 'real' watch pipes cmd's output to itself
    // and does not allow it to overflow the screen
    // (taking into account linewrap!)
    system(cmd);
    sleep_for_duration(period);
  }
  // gcc thinks we can reach this :)
}

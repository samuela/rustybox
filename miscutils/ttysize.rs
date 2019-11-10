use libc;
use libc::chdir;
use libc::chmod;
use libc::chown;
use libc::closelog;
use libc::dup2;
use libc::fstat;
use libc::getenv;
use libc::geteuid;
use libc::getopt;
use libc::getpid;
use libc::isatty;
use libc::kill;
use libc::openlog;
use libc::sigaddset;
use libc::sigemptyset;
use libc::sigprocmask;
use libc::sleep;
use libc::sscanf;
use libc::strcasecmp;
use libc::strcpy;
use libc::symlink;
use libc::syscall;
use libc::syslog;
use libc::time;
use libc::access;
use libc::atoi;
use libc::fclose;
use libc::fprintf;
use libc::lstat;
use libc::printf;
use libc::puts;
use libc::rename;
use libc::rmdir;
use libc::sprintf;
use libc::strchr;
use libc::strcmp;
use libc::strrchr;
use libc::strstr;
use libc::system;




use libc::winsize;

extern "C" {

  #[no_mangle]
  fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
}

/*
 * Replacement for "stty size", which is awkward for shell script use.
 * - Allows to request width, height, or both, in any order.
 * - Does not complain on error, but returns width 80, height 24.
 * - Size: less than 200 bytes
 *
 * Copyright (C) 2007 by Denys Vlasenko <vda.linux@googlemail.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config TTYSIZE
//config:	bool "ttysize (432 bytes)"
//config:	default y
//config:	help
//config:	A replacement for "stty size". Unlike stty, can report only width,
//config:	only height, or both, in any order. It also does not complain on
//config:	error, but returns default 80x24.
//config:	Usage in shell scripts: width=`ttysize w`.
//applet:IF_TTYSIZE(APPLET_NOFORK(ttysize, ttysize, BB_DIR_USR_BIN, BB_SUID_DROP, ttysize))
//kbuild:lib-$(CONFIG_TTYSIZE) += ttysize.o
//usage:#define ttysize_trivial_usage
//usage:       "[w] [h]"
//usage:#define ttysize_full_usage "\n\n"
//usage:       "Print dimensions of stdin tty, or 80x24"
#[no_mangle]
pub unsafe extern "C" fn ttysize_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut w: libc::c_uint = 0; /* "%u" */
  let mut h: libc::c_uint = 0;
  let mut wsz: winsize = winsize {
    ws_row: 0,
    ws_col: 0,
    ws_xpixel: 0,
    ws_ypixel: 0,
  };
  w = 80i32 as libc::c_uint;
  h = 24i32 as libc::c_uint;
  if ioctl(0i32, 0x5413i32 as libc::c_ulong, &mut wsz as *mut winsize) == 0i32
    || ioctl(1i32, 0x5413i32 as libc::c_ulong, &mut wsz as *mut winsize) == 0i32
    || ioctl(2i32, 0x5413i32 as libc::c_ulong, &mut wsz as *mut winsize) == 0i32
  {
    w = wsz.ws_col as libc::c_uint;
    h = wsz.ws_row as libc::c_uint
  }
  if (*argv.offset(1)).is_null() {
    printf(b"%u %u\x00" as *const u8 as *const libc::c_char, w, h);
  } else {
    let mut fmt: *const libc::c_char = 0 as *const libc::c_char;
    let mut arg: *const libc::c_char = 0 as *const libc::c_char;
    fmt = (b"%u %u\x00" as *const u8 as *const libc::c_char).offset(3);
    loop {
      argv = argv.offset(1);
      arg = *argv;
      if arg.is_null() {
        break;
      }
      let mut c: libc::c_char = *arg.offset(0);
      if c as libc::c_int == 'w' as i32 {
        printf(fmt, w);
      }
      if c as libc::c_int == 'h' as i32 {
        printf(fmt, h);
      }
      fmt = (b"%u %u\x00" as *const u8 as *const libc::c_char).offset(2)
      /* " %u" */
    }
  }
  bb_putchar('\n' as i32);
  return 0i32;
}

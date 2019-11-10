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




extern "C" {

  /*
   * Copyright (C) 2008 by Denys Vlasenko <vda.linux@googlemail.com>
   *
   * Licensed under GPLv2, see file LICENSE in this source tree.
   */
  pub type globals;
}
/* We cheat here. It is declared as const ptr in libbb.h,
 * but here we make it live in R/W memory */
#[no_mangle]
pub static mut ptr_to_globals: *mut globals = 0 as *const globals as *mut globals;
#[no_mangle]
pub static mut bb_errno: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;

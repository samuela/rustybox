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
   * bb_perror_nomsg implementation for busybox
   *
   * Copyright (C) 2003  Manuel Novoa III  <mjn3@codepoet.org>
   *
   * Licensed under GPLv2 or later, see file LICENSE in this source tree.
   */
  /* gcc warns about a null format string, therefore we provide
   * modified definition without "attribute (format)"
   * instead of including libbb.h */
  //#include "libbb.h"
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
}
/* suppress gcc "no previous prototype" warning */
#[no_mangle]
pub unsafe extern "C" fn bb_perror_nomsg() {
  bb_simple_perror_msg(0 as *const libc::c_char);
}

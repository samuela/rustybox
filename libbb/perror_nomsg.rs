use libc;
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

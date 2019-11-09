use libc;
use libc::open;



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

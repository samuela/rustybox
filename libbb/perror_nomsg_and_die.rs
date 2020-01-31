use libc;
extern "C" {

  /*
   * bb_perror_nomsg_and_die implementation for busybox
   *
   * Copyright (C) 2003  Manuel Novoa III  <mjn3@codepoet.org>
   *
   * Licensed under GPLv2 or later, see file LICENSE in this source tree.
   */
  /* gcc warns about a null format string, therefore we provide
   * modified definition without "attribute (format)"
   * instead of including libbb.h */
  //#include "libbb.h"

}
/* suppress gcc "no previous prototype" warning */
pub unsafe fn bb_perror_nomsg_and_die() {
  crate::libbb::perror_msg::bb_simple_perror_msg_and_die(0 as *const libc::c_char);
}

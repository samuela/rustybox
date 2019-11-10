use libc;
use libc::putenv;
use libc::umask;
use libc::mknod;
use libc::fchmod;
use libc::fscanf;
use libc::alarm;
use libc::sync;
use libc::setsid;
use libc::ioctl;
use libc::statfs;
use libc::mount;
use libc::prctl;
use libc::opendir;
use libc::closedir;
use libc::readdir;
use libc::strtok;
use libc::putchar_unlocked;
use libc::endmntent;
use libc::setmntent;
use libc::umount2;
use libc::getegid;
use libc::getuid;
use libc::getgid;
use libc::setutxent;
use libc::endutxent;











































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
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char);
}
/* suppress gcc "no previous prototype" warning */
#[no_mangle]
pub unsafe extern "C" fn bb_perror_nomsg_and_die() {
  bb_simple_perror_msg_and_die(0 as *const libc::c_char);
}

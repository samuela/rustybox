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

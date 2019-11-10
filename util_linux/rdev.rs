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
use libc::printf;

extern "C" {

  #[no_mangle]
  fn find_block_device(path: *const libc::c_char) -> *mut libc::c_char;
}

/*
 * rdev - print device node associated with a filesystem
 *
 * Copyright (c) 2008 Nuovation System Designs, LLC
 *   Grant Erickson <gerickson@nuovations.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config RDEV
//config:	bool "rdev (1.8 kb)"
//config:	default y
//config:	help
//config:	Print the device node associated with the filesystem mounted at '/'.
//applet:IF_RDEV(APPLET_NOEXEC(rdev, rdev, BB_DIR_USR_SBIN, BB_SUID_DROP, rdev))
//kbuild:lib-$(CONFIG_RDEV) += rdev.o
//usage:#define rlibc::dev_trivial_usage
//usage:       ""
//usage:#define rdev_full_usage "\n\n"
//usage:       "Print the device node associated with the filesystem mounted at '/'"
//usage:
//usage:#define rdev_example_usage
//usage:       "$ rdev\n"
//usage:       "/dev/mtdblock9 /\n"
#[no_mangle]
pub unsafe extern "C" fn rdev_main(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut root_device: *const libc::c_char =
    find_block_device(b"/\x00" as *const u8 as *const libc::c_char);
  if !root_device.is_null() {
    printf(
      b"%s /\n\x00" as *const u8 as *const libc::c_char,
      root_device,
    );
    return 0i32;
  }
  return 1i32;
}

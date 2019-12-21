use libc;
use libc::printf;

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
//applet:IF_RDEV(APPLET_NOEXEC(rdev, rdev, BB_DIR_USR_SBIN, SUID_DROP, rdev))
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
    crate::libbb::find_root_device::find_block_device(b"/\x00" as *const u8 as *const libc::c_char);
  if !root_device.is_null() {
    printf(
      b"%s /\n\x00" as *const u8 as *const libc::c_char,
      root_device,
    );
    return 0i32;
  }
  return 1i32;
}

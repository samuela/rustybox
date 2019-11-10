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
  #[no_mangle]
  fn display_uuid_cache(scan_devices: libc::c_int);
  #[no_mangle]
  fn add_to_uuid_cache(device: *const libc::c_char) -> libc::c_int;
}

/*
 * Print UUIDs on all filesystems
 *
 * Copyright (C) 2008 Denys Vlasenko.
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config BLKID
//config:	bool "blkid (12 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	select VOLUMEID
//config:	help
//config:	Lists labels and UUIDs of all filesystems.
//config:
//config:config FEATURE_BLKID_TYPE
//config:	bool "Print filesystem type"
//config:	default y
//config:	depends on BLKID
//config:	help
//config:	Show TYPE="filesystem type"
//applet:IF_BLKID(APPLET_NOEXEC(blkid, blkid, BB_DIR_SBIN, BB_SUID_DROP, blkid))
//kbuild:lib-$(CONFIG_BLKID) += blkid.o
//usage:#define blkid_trivial_usage
//usage:       "[BLOCKDEV]..."
//usage:#define blkid_full_usage "\n\n"
//usage:       "Print UUIDs of all filesystems"
#[no_mangle]
pub unsafe extern "C" fn blkid_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut scan_devices: libc::c_int = 1i32;
  loop {
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
    /* Note: bogus device names don't cause any error messages */
    add_to_uuid_cache(*argv);
    scan_devices = 0i32
  }
  display_uuid_cache(scan_devices);
  return 0i32;
}

use libc;

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
//applet:IF_BLKID(APPLET_NOEXEC(blkid, blkid, BB_DIR_SBIN, SUID_DROP, blkid))
//kbuild:lib-$(CONFIG_BLKID) += blkid.o
//usage:#define blkid_trivial_usage
//usage:       "[BLOCKDEV]..."
//usage:#define blkid_full_usage "\n\n"
//usage:       "Print UUIDs of all filesystems"
pub unsafe fn blkid_main(mut _argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut scan_devices: libc::c_int = 1i32;
  loop {
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
    /* Note: bogus device names don't cause any error messages */
    crate::util_linux::volume_id::get_devname::add_to_uuid_cache(*argv);
    scan_devices = 0
  }
  crate::util_linux::volume_id::get_devname::display_uuid_cache(scan_devices);
  return 0;
}

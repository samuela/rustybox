use libc;

/*
 * raidautorun implementation for busybox
 *
 * Copyright (C) 2006 Bernhard Reutner-Fischer
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config RAIDAUTORUN
//config:	bool "raidautorun (1.3 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	raidautorun tells the kernel md driver to
//config:	search and start RAID arrays.
//applet:IF_RAIDAUTORUN(APPLET_NOEXEC(raidautorun, raidautorun, BB_DIR_SBIN, SUID_DROP, raidautorun))
//kbuild:lib-$(CONFIG_RAIDAUTORUN) += raidautorun.o
//usage:#define raidautorun_trivial_usage
//usage:       "DEVICE"
//usage:#define raidautorun_full_usage "\n\n"
//usage:       "Tell the kernel to automatically search and start RAID arrays"
//usage:
//usage:#define raidautorun_example_usage
//usage:       "$ raidautorun /dev/md0"
pub unsafe fn raidautorun_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  crate::libbb::xfuncs_printf::bb_xioctl(
    crate::libbb::xfuncs_printf::xopen(crate::libbb::single_argv::single_argv(argv), 0),
    0u32 << 0 + 8 + 8 + 14
      | (9 << 0 + 8) as libc::c_uint
      | (0x14 << 0) as libc::c_uint
      | (0 << 0 + 8 + 8) as libc::c_uint,
    0 as *mut libc::c_void,
    b"RAID_AUTORUN\x00" as *const u8 as *const libc::c_char,
  );
  return 0;
}

use libc;
extern "C" {
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn single_argv(argv: *mut *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_xioctl(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
}

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
//applet:IF_RAIDAUTORUN(APPLET_NOEXEC(raidautorun, raidautorun, BB_DIR_SBIN, BB_SUID_DROP, raidautorun))
//kbuild:lib-$(CONFIG_RAIDAUTORUN) += raidautorun.o
//usage:#define raidautorun_trivial_usage
//usage:       "DEVICE"
//usage:#define raidautorun_full_usage "\n\n"
//usage:       "Tell the kernel to automatically search and start RAID arrays"
//usage:
//usage:#define raidautorun_example_usage
//usage:       "$ raidautorun /dev/md0"
#[no_mangle]
pub unsafe extern "C" fn raidautorun_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  bb_xioctl(
    xopen(single_argv(argv), 0i32),
    0u32 << 0i32 + 8i32 + 8i32 + 14i32
      | (9i32 << 0i32 + 8i32) as libc::c_uint
      | (0x14i32 << 0i32) as libc::c_uint
      | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint,
    0 as *mut libc::c_void,
    b"RAID_AUTORUN\x00" as *const u8 as *const libc::c_char,
  );
  return 0i32;
}

use libc;
extern "C" {
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn ioctl_or_perror_and_die(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    fmt: *const libc::c_char,
    _: ...
  ) -> libc::c_int;
}

use crate::librb::uint32_t;

/*
 * Copyright (C) 2017 Denys Vlasenko <vda.linux@googlemail.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config PARTPROBE
//config:	bool "partprobe (3.5 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	Ask kernel to rescan partition table.
//applet:IF_PARTPROBE(APPLET_NOEXEC(partprobe, partprobe, BB_DIR_USR_SBIN, BB_SUID_DROP, partprobe))
//kbuild:lib-$(CONFIG_PARTPROBE) += partprobe.o
//usage:#define partprobe_trivial_usage
//usage:	"DEVICE..."
//usage:#define partprobe_full_usage "\n\n"
//usage:	"Ask kernel to rescan partition table"
//
// partprobe (GNU parted) 3.2:
// -d, --dry-run	Don't update the kernel
// -s, --summary	Show a summary of devices and their partitions
#[no_mangle]
pub unsafe extern "C" fn partprobe_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  getopt32(argv, b"\x00" as *const u8 as *const libc::c_char);
  argv = argv.offset(optind as isize);
  /* "partprobe" with no arguments just does nothing */
  while !(*argv).is_null() {
    let mut fd: libc::c_int = xopen(*argv, 0i32);
    /*
     * Newer versions of parted scan partition tables themselves and
     * use BLKPG ioctl (BLKPG_DEL_PARTITION / BLKPG_ADD_PARTITION)
     * since this way kernel does not need to know
     * partition table formats.
     * We use good old BLKRRPART:
     */
    ioctl_or_perror_and_die(
      fd,
      0u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (0x12i32 << 0i32 + 8i32) as libc::c_uint
        | (95i32 << 0i32) as libc::c_uint
        | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint,
      0 as *mut libc::c_void,
      b"%s\x00" as *const u8 as *const libc::c_char,
      *argv,
    );
    close(fd);
    argv = argv.offset(1)
  }
  return 0i32;
}

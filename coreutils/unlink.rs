use libc;
use libc::open;



extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn xunlink(pathname: *const libc::c_char);
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
}



/*
 * unlink for busybox
 *
 * Copyright (C) 2014 Isaac Dunham <ibid.ag@gmail.com>
 *
 * Licensed under GPLv2, see LICENSE in this source tree
 */
//config:config UNLINK
//config:	bool "unlink (3.2 kb)"
//config:	default y
//config:	help
//config:	unlink deletes a file by calling unlink()
//applet:IF_UNLINK(APPLET_NOFORK(unlink, unlink, BB_DIR_USR_BIN, BB_SUID_DROP, unlink))
//kbuild:lib-$(CONFIG_UNLINK) += unlink.o
//usage:#define unlink_trivial_usage
//usage:	"FILE"
//usage:#define unlink_full_usage "\n\n"
//usage:	"Delete FILE by calling unlink()"
#[no_mangle]
pub unsafe extern "C" fn unlink_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  getopt32(argv, b"^\x00=1\x00" as *const u8 as *const libc::c_char);
  argv = argv.offset(optind as isize);
  xunlink(*argv.offset(0));
  return 0i32;
}

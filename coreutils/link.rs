use libc;
extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn link(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;
}
use crate::librb::__uint32_t;
use crate::librb::uint32_t;
/*
 * link implementation for busybox
 *
 * Copyright (C) 2017 Denys Vlasenko <vda.linux@googlemail.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config LINK
//config:	bool "link (3.2 kb)"
//config:	default y
//config:	help
//config:	link creates hard links between files.
//applet:IF_LINK(APPLET_NOFORK(link, link, BB_DIR_BIN, BB_SUID_DROP, link))
//kbuild:lib-$(CONFIG_LINK) += link.o
//usage:#define link_trivial_usage
//usage:       "FILE LINK"
//usage:#define link_full_usage "\n\n"
//usage:       "Create hard LINK to FILE"
/* This is a NOFORK applet. Be very careful! */
#[no_mangle]
pub unsafe extern "C" fn link_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  getopt32(argv, b"^\x00=2\x00" as *const u8 as *const libc::c_char);
  argv = argv.offset(optind as isize);
  if link(*argv.offset(0), *argv.offset(1)) != 0i32 {
    /* shared message */
    bb_perror_msg_and_die(
      b"can\'t create %slink \'%s\' to \'%s\'\x00" as *const u8 as *const libc::c_char,
      b"hard\x00" as *const u8 as *const libc::c_char,
      *argv.offset(1),
      *argv.offset(0),
    );
  }
  return 0i32;
}

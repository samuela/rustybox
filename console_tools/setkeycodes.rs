use libc;

/*
 * setkeycodes
 *
 * Copyright (C) 1994-1998 Andries E. Brouwer <aeb@cwi.nl>
 *
 * Adjusted for BusyBox by Erik Andersen <andersen@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config SETKEYCODES
//config:	bool "setkeycodes (2.1 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	This program loads entries into the kernel's scancode-to-keycode
//config:	map, allowing unusual keyboards to generate usable keycodes.
//applet:IF_SETKEYCODES(APPLET_NOEXEC(setkeycodes, setkeycodes, BB_DIR_USR_BIN, SUID_DROP, setkeycodes))
//kbuild:lib-$(CONFIG_SETKEYCODES) += setkeycodes.o
//usage:#define setkeycodes_trivial_usage
//usage:       "{ SCANCODE KEYCODE }..."
//usage:#define setkeycodes_full_usage "\n\n"
//usage:       "Modify kernel's scancode-to-keycode map,\n"
//usage:       "allowing unusual keyboards to generate usable keycodes.\n\n"
//usage:       "SCANCODE is either xx or e0xx (hexadecimal), KEYCODE is decimal."
//usage:
//usage:#define setkeycodes_example_usage
//usage:       "$ setkeycodes e030 127\n"
/* From <linux/kd.h> */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kbkeycode {
  pub scancode: libc::c_uint,
  pub keycode: libc::c_uint,
}
pub type C2RustUnnamed = libc::c_uint;
pub const KDSETKEYCODE: C2RustUnnamed = 19277;
#[inline(always)]
unsafe fn xstrtoul_range(
  mut str: *const libc::c_char,
  mut b: libc::c_int,
  mut l: libc::c_ulong,
  mut u: libc::c_ulong,
) -> libc::c_ulong {
  return crate::libbb::xatonum::xstrtoull_range(
    str,
    b,
    l as libc::c_ulonglong,
    u as libc::c_ulonglong,
  ) as libc::c_ulong;
}
pub unsafe fn setkeycodes_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut fd: libc::c_int = 0;
  if argc & 1i32 == 0 || argc < 2i32 {
    crate::libbb::appletlib::bb_show_usage();
  }
  fd = crate::libbb::get_console::get_console_fd_or_die();
  while !(*argv.offset(1)).is_null() {
    let mut a: kbkeycode = kbkeycode {
      scancode: 0,
      keycode: 0,
    };
    let mut sc: libc::c_int = 0;
    sc = xstrtoul_range(
      *argv.offset(1),
      16i32,
      0 as libc::c_ulong,
      0xe07fi32 as libc::c_ulong,
    ) as libc::c_int;
    if sc >= 0xe000i32 {
      sc -= 0xe000i32;
      sc += 0x80i32
    }
    a.scancode = sc as libc::c_uint;
    a.keycode = crate::libbb::xatonum::xatou_range(
      *argv.offset(2),
      0 as libc::c_uint,
      255i32 as libc::c_uint,
    );
    crate::libbb::xfuncs_printf::ioctl_or_perror_and_die(
      fd,
      KDSETKEYCODE as libc::c_int as libc::c_uint,
      &mut a as *mut kbkeycode as *mut libc::c_void,
      b"can\'t set SCANCODE %x to KEYCODE %d\x00" as *const u8 as *const libc::c_char,
      sc,
      a.keycode,
    );
    argv = argv.offset(2)
  }
  return 0;
}
/* write kernel keycode table entry */

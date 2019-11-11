
use crate::libbb::appletlib::applet_name;
use libc;
extern "C" {
  #[no_mangle]
  fn bb_xioctl(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
}

/*
 * setconsole.c - redirect system console output
 *
 * Copyright (C) 2004,2005  Enrik Berkhan <Enrik.Berkhan@inka.de>
 * Copyright (C) 2008 Bernhard Reutner-Fischer
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config SETCONSOLE
//config:	bool "setconsole (3.6 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	Redirect writes to /dev/console to another device,
//config:	like the current tty while logged in via telnet.
//config:	This does not redirect kernel log, only writes
//config:	from user space.
//config:
//config:config FEATURE_SETCONSOLE_LONG_OPTIONS
//config:	bool "Enable long options"
//config:	default y
//config:	depends on SETCONSOLE && LONG_OPTS
//applet:IF_SETCONSOLE(APPLET_NOEXEC(setconsole, setconsole, BB_DIR_SBIN, BB_SUID_DROP, setconsole))
//kbuild:lib-$(CONFIG_SETCONSOLE) += setconsole.o
//usage:#define setconsole_trivial_usage
//usage:       "[-r] [DEVICE]"
//usage:#define setconsole_full_usage "\n\n"
//usage:       "Make writes to /dev/console appear on DEVICE (default: /dev/tty)."
//usage:   "\n""Does not redirect kernel log output or reads from /dev/console."
//usage:   "\n"
//usage:   "\n""	-r	Reset: writes to /dev/console go to kernel log tty(s)"
/* It was a bbox-specific invention, but SUSE does have a similar utility.
 * SUSE has no -r option, though.
 */
#[no_mangle]
pub unsafe extern "C" fn setconsole_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut device: *const libc::c_char = b"/dev/tty\x00" as *const u8 as *const libc::c_char;
  let mut reset: libc::c_int = 0;
  /* at most one non-option argument */
  reset = getopt32(argv, b"^r\x00?1\x00" as *const u8 as *const libc::c_char) as libc::c_int;
  argv = argv.offset((1i32 + reset) as isize);
  if !(*argv).is_null() {
    device = *argv
  } else if reset != 0 {
    device = b"/dev/console\x00" as *const u8 as *const libc::c_char
  }
  //TODO: fails if TIOCCONS redir is already active to some tty.
  //I think SUSE version first does TIOCCONS on /dev/console fd (iow: resets)
  //then TIOCCONS to new tty?
  bb_xioctl(
    xopen(device, 0o1i32),
    0x541di32 as libc::c_uint,
    0 as *mut libc::c_void,
    b"TIOCCONS\x00" as *const u8 as *const libc::c_char,
  );
  return 0i32;
}

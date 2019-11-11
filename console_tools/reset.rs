
use crate::libbb::appletlib::applet_name;
use libc;
use libc::isatty;
use libc::printf;
extern "C" {

  /*
   * Mini reset implementation for busybox
   *
   * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
   * Written by Erik Andersen and Kent Robotti <robotti@metconnect.com>
   *
   * Licensed under GPLv2 or later, see file LICENSE in this source tree.
   */
  //config:config RESET
  //config:	bool "reset (345 bytes)"
  //config:	default y
  //config:	help
  //config:	This program is used to reset the terminal screen, if it
  //config:	gets messed up.
  //applet:IF_RESET(APPLET_NOEXEC(reset, reset, BB_DIR_USR_BIN, BB_SUID_DROP, reset))
  //kbuild:lib-$(CONFIG_RESET) += reset.o
  //usage:#define reset_trivial_usage
  //usage:       ""
  //usage:#define reset_full_usage "\n\n"
  //usage:       "Reset the screen"
  /* "Standard" version of this tool is in ncurses package */
  #[no_mangle]
  fn stty_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn reset_main(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  static mut args: [*const libc::c_char; 3] = [
    b"stty\x00" as *const u8 as *const libc::c_char,
    b"sane\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
  ];
  /* no options, no getopt */
  if isatty(1i32) != 0 {
    /* See 'man 4 console_codes' for details:
     * "ESC c"        -- Reset
     * "ESC ( B"      -- Select G0 Character Set (B = US)
     * "ESC [ m"      -- Reset all display attributes
     * "ESC [ J"      -- Erase to the end of screen
     * "ESC [ ? 25 h" -- Make cursor visible
     */
    printf(b"\x1bc\x1b(B\x1b[m\x1b[J\x1b[?25h\x00" as *const u8 as *const libc::c_char);
    /* http://bugs.busybox.net/view.php?id=1414:
     * people want it to reset echo etc: */
    return stty_main(2i32, args.as_ptr() as *mut *mut libc::c_char);
  }
  return 0i32;
}

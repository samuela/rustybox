use libc;
extern "C" {
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn get_console_fd_or_die() -> libc::c_int;
  #[no_mangle]
  fn xopen_nonblocking(pathname: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_xioctl(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type ptrdiff_t = libc::c_long;
pub const UNICODE: C2RustUnnamed = 8;
pub type C2RustUnnamed = libc::c_uint;
pub const MEDIUMRAW: C2RustUnnamed = 4;
pub const ASCII: C2RustUnnamed = 2;
pub const SCANCODE: C2RustUnnamed = 1;
/* vi: set sw=4 ts=4: */
/*
 * Mini kbd_mode implementation for busybox
 *
 * Copyright (C) 2007 Loic Grenie <loic.grenie@gmail.com>
 *   written using Andries Brouwer <aeb@cwi.nl>'s kbd_mode from
 *   console-utils v0.2.3, licensed under GNU GPLv2
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config KBD_MODE
//config:	bool "kbd_mode (4.1 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	This program reports and sets keyboard mode.
//applet:IF_KBD_MODE(APPLET_NOEXEC(kbd_mode, kbd_mode, BB_DIR_BIN, BB_SUID_DROP, kbd_mode))
//kbuild:lib-$(CONFIG_KBD_MODE) += kbd_mode.o
//usage:#define kbd_mode_trivial_usage
//usage:       "[-a|k|s|u] [-C TTY]"
//usage:#define kbd_mode_full_usage "\n\n"
//usage:       "Report or set VT console keyboard mode\n"
//usage:     "\n	-a	Default (ASCII)"
//usage:     "\n	-k	Medium-raw (keycode)"
//usage:     "\n	-s	Raw (scancode)"
//usage:     "\n	-u	Unicode (utf-8)"
//usage:     "\n	-C TTY	Affect TTY"
#[no_mangle]
pub unsafe extern "C" fn kbd_mode_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut fd: libc::c_int = 0; /* clear -C bit, see (*) */
  let mut opt: libc::c_uint = 0;
  let mut tty_name: *const libc::c_char = 0 as *const libc::c_char;
  opt = getopt32(
    argv,
    b"sakuC:\x00" as *const u8 as *const libc::c_char,
    &mut tty_name as *mut *const libc::c_char,
  );
  if opt & 0x10i32 as libc::c_uint != 0 {
    opt &= 0xfi32 as libc::c_uint;
    fd = xopen_nonblocking(tty_name)
  } else {
    /* kbd-2.0.3 tries in sequence:
     * fd#0, /dev/tty, /dev/tty0.
     * get_console_fd_or_die: /dev/console, /dev/tty0, /dev/tty.
     * kbd-2.0.3 checks KDGKBTYPE, get_console_fd_or_die checks too.
     */
    fd = get_console_fd_or_die()
  }
  if opt == 0 {
    /* print current setting */
    let mut mode: *const libc::c_char = b"unknown\x00" as *const u8 as *const libc::c_char;
    let mut m: libc::c_int = 0;
    bb_xioctl(
      fd,
      0x4b44i32 as libc::c_uint,
      &mut m as *mut libc::c_int as *mut libc::c_void,
      b"KDGKBMODE\x00" as *const u8 as *const libc::c_char,
    );
    if m == 0i32 {
      mode = b"raw (scancode)\x00" as *const u8 as *const libc::c_char
    } else if m == 0x1i32 {
      mode = b"default (ASCII)\x00" as *const u8 as *const libc::c_char
    } else if m == 0x2i32 {
      mode = b"mediumraw (keycode)\x00" as *const u8 as *const libc::c_char
    } else if m == 0x3i32 {
      mode = b"Unicode (UTF-8)\x00" as *const u8 as *const libc::c_char
    } else if m == 4i32 {
      /*K_OFF*/
      /* kbd-2.0.3 does not show this mode, says "unknown" */
      mode = b"off\x00" as *const u8 as *const libc::c_char
    }
    printf(
      b"The keyboard is in %s mode\n\x00" as *const u8 as *const libc::c_char,
      mode,
    );
  } else {
    /* here we depend on specific bits assigned to options (*)
     * KDSKBMODE constants have these values:
     * #define K_RAW           0x00
     * #define K_XLATE         0x01
     * #define K_MEDIUMRAW     0x02
     * #define K_UNICODE       0x03
     * #define K_OFF           0x04
     * (looks like "-ak" together would cause the same effect as -u)
     */
    opt = if opt & UNICODE as libc::c_int as libc::c_uint != 0 {
      3i32 as libc::c_uint
    } else {
      (opt) >> 1i32
    };
    /* double cast prevents warnings about widening conversion */
    bb_xioctl(
      fd,
      0x4b45i32 as libc::c_uint,
      opt as ptrdiff_t as *mut libc::c_void,
      b"KDSKBMODE\x00" as *const u8 as *const libc::c_char,
    );
  }
  return 0i32;
}

use libc;
extern "C" {
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xlseek(fd: libc::c_int, offset: off_t, whence: libc::c_int) -> off_t;
  #[no_mangle]
  fn xread(fd: libc::c_int, buf: *mut libc::c_void, count: size_t);
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
}
pub type __off64_t = libc::c_long;
use crate::librb::size_t;
use crate::librb::off_t;
/*
 * Reads and displays CD-ROM volume name
 *
 * Several people have asked how to read CD volume names so I wrote this
 * small program to do it.
 *
 * usage: volname [<device-file>]
 *
 * Copyright (C) 2000-2001 Jeff Tranter (tranter@pobox.com)
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA
 */
/*
 * mods from distributed source (eject-2.0.13) are by
 * Matthew Stoltenberg <d3matt@gmail.com>
 */
//config:config VOLNAME
//config:	bool "volname (1.6 kb)"
//config:	default y
//config:	help
//config:	Prints a CD-ROM volume name.
//applet:IF_VOLNAME(APPLET(volname, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_VOLNAME) += volname.o
//usage:#define volname_trivial_usage
//usage:       "[DEVICE]"
//usage:#define volname_full_usage "\n\n"
//usage:       "Show CD volume name of the DEVICE (default /dev/cdrom)"
#[no_mangle]
pub unsafe extern "C" fn volname_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut fd: libc::c_int = 0;
  let mut buffer: [libc::c_char; 32] = [0; 32];
  let mut device: *const libc::c_char = 0 as *const libc::c_char;
  device = b"/dev/cdrom\x00" as *const u8 as *const libc::c_char;
  if !(*argv.offset(1)).is_null() {
    device = *argv.offset(1);
    if !(*argv.offset(2)).is_null() {
      bb_show_usage();
    }
  }
  fd = xopen(device, 0i32);
  xlseek(fd, 32808i32 as off_t, 0i32);
  xread(
    fd,
    buffer.as_mut_ptr() as *mut libc::c_void,
    32i32 as size_t,
  );
  printf(
    b"%32.32s\n\x00" as *const u8 as *const libc::c_char,
    buffer.as_mut_ptr(),
  );
  return 0i32;
}

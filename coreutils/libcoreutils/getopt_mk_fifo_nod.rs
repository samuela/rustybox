use libc;
use libc::mode_t;
use libc::umask;
/*
 * coreutils utility routine
 *
 * Copyright (C) 2003  Manuel Novoa III  <mjn3@codepoet.org>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA
 */
pub unsafe fn getopt_mk_fifo_nod(mut argv: *mut *mut libc::c_char) -> mode_t {
  let mut mode: mode_t = 0o666i32 as mode_t;
  let mut smode: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut opt: libc::c_int = 0;
  opt = crate::libbb::getopt32::getopt32(
    argv,
    b"m:\x00" as *const u8 as *const libc::c_char,
    &mut smode as *mut *mut libc::c_char,
  ) as libc::c_int;
  if opt & 1i32 != 0 {
    mode = crate::libbb::parse_mode::bb_parse_mode(smode, mode) as mode_t;
    if mode != -1i32 as mode_t {
      /* if mode is valid */
      /* make future mknod/mkfifo set mode bits exactly */
      umask(0i32 as mode_t);
    }
  }
  return mode;
}

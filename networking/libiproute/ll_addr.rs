use libc;
use libc::access;
use libc::atoi;
use libc::fclose;
use libc::fprintf;
use libc::lstat;
use libc::printf;
use libc::puts;
use libc::rename;
use libc::rmdir;
use libc::sprintf;
use libc::strchr;
use libc::strcmp;
use libc::strrchr;
use libc::strstr;
use libc::system;




extern "C" {
  #[no_mangle]
  fn snprintf(
    _: *mut libc::c_char,
    _: libc::c_ulong,
    _: *const libc::c_char,
    _: ...
  ) -> libc::c_int;
  #[no_mangle]
  fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn inet_ntop(
    __af: libc::c_int,
    __cp: *const libc::c_void,
    __buf: *mut libc::c_char,
    __len: socklen_t,
  ) -> *const libc::c_char;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn get_addr_1(dst: *mut inet_prefix, arg: *mut libc::c_char, family: libc::c_int) -> libc::c_int;
}

pub type __socklen_t = libc::c_uint;

pub type socklen_t = __socklen_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct inet_prefix {
  pub family: u8,
  pub bytelen: u8,
  pub bitlen: i16,
  pub data: [u32; 4],
}

/*
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version
 * 2 of the License, or (at your option) any later version.
 *
 * Authors: Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
 */
#[no_mangle]
pub unsafe extern "C" fn ll_addr_n2a(
  mut addr: *mut libc::c_uchar,
  mut alen: libc::c_int,
  mut type_0: libc::c_int,
  mut buf: *mut libc::c_char,
  mut blen: libc::c_int,
) -> *const libc::c_char {
  let mut i: libc::c_int = 0;
  let mut l: libc::c_int = 0;
  if alen == 4i32 && (type_0 == 768i32 || type_0 == 776i32 || type_0 == 778i32) {
    return inet_ntop(2i32, addr as *const libc::c_void, buf, blen as socklen_t);
  }
  l = 0i32;
  i = 0i32;
  while i < alen {
    if i == 0i32 {
      snprintf(
        buf.offset(l as isize),
        blen as libc::c_ulong,
        (b":%02x\x00" as *const u8 as *const libc::c_char).offset(1),
        *addr.offset(i as isize) as libc::c_int,
      );
      blen -= 2i32;
      l += 2i32
    } else {
      snprintf(
        buf.offset(l as isize),
        blen as libc::c_ulong,
        b":%02x\x00" as *const u8 as *const libc::c_char,
        *addr.offset(i as isize) as libc::c_int,
      );
      blen -= 3i32;
      l += 3i32
    }
    i += 1
  }
  return buf;
}
#[no_mangle]
pub unsafe extern "C" fn ll_addr_a2n(
  mut lladdr: *mut libc::c_uchar,
  mut len: libc::c_int,
  mut arg: *mut libc::c_char,
) -> libc::c_int {
  let mut i: libc::c_int = 0;
  if !strchr(arg, '.' as i32).is_null() {
    let mut pfx: inet_prefix = inet_prefix {
      family: 0,
      bytelen: 0,
      bitlen: 0,
      data: [0; 4],
    };
    if get_addr_1(&mut pfx, arg, 2i32) != 0 {
      bb_error_msg(
        b"\"%s\" is invalid lladdr\x00" as *const u8 as *const libc::c_char,
        arg,
      );
      return -1i32;
    }
    if len < 4i32 {
      return -1i32;
    }
    memcpy(
      lladdr as *mut libc::c_void,
      pfx.data.as_mut_ptr() as *const libc::c_void,
      4i32 as libc::c_ulong,
    );
    return 4i32;
  }
  i = 0i32;
  while i < len {
    let mut temp: libc::c_int = 0;
    let mut cp: *mut libc::c_char = strchr(arg, ':' as i32);
    if !cp.is_null() {
      *cp = 0i32 as libc::c_char;
      cp = cp.offset(1)
    }
    if sscanf(
      arg,
      b"%x\x00" as *const u8 as *const libc::c_char,
      &mut temp as *mut libc::c_int,
    ) != 1i32
      || (temp < 0i32 || temp > 255i32)
    {
      bb_error_msg(
        b"\"%s\" is invalid lladdr\x00" as *const u8 as *const libc::c_char,
        arg,
      );
      return -1i32;
    }
    *lladdr.offset(i as isize) = temp as libc::c_uchar;
    if cp.is_null() {
      break;
    }
    arg = cp;
    i += 1
  }
  return i + 1i32;
}

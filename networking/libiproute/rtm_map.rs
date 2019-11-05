use libc;
extern "C" {
  #[no_mangle]
  fn strtoul(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_ulong;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn itoa(n: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn index_in_substrings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn rtnl_rtrealm_a2n(id: *mut uint32_t, arg: *mut libc::c_char) -> libc::c_int;
}

use crate::librb::uint32_t;
pub type smalluint = libc::c_uchar;
pub type C2RustUnnamed = libc::c_uint;
pub const __RTN_MAX: C2RustUnnamed = 12;
pub const RTN_XRESOLVE: C2RustUnnamed = 11;
pub const RTN_NAT: C2RustUnnamed = 10;
pub const RTN_THROW: C2RustUnnamed = 9;
pub const RTN_PROHIBIT: C2RustUnnamed = 8;
pub const RTN_UNREACHABLE: C2RustUnnamed = 7;
pub const RTN_BLACKHOLE: C2RustUnnamed = 6;
pub const RTN_MULTICAST: C2RustUnnamed = 5;
pub const RTN_ANYCAST: C2RustUnnamed = 4;
pub const RTN_BROADCAST: C2RustUnnamed = 3;
pub const RTN_LOCAL: C2RustUnnamed = 2;
pub const RTN_UNICAST: C2RustUnnamed = 1;
pub const RTN_UNSPEC: C2RustUnnamed = 0;
pub const ARG_throw: C2RustUnnamed_0 = 12;
pub const ARG_unicast: C2RustUnnamed_0 = 11;
pub const ARG_xresolve: C2RustUnnamed_0 = 10;
pub const ARG_blackhole: C2RustUnnamed_0 = 9;
pub const ARG_unreachable: C2RustUnnamed_0 = 8;
pub const ARG_prohibit: C2RustUnnamed_0 = 7;
pub const ARG_multicast: C2RustUnnamed_0 = 6;
pub const ARG_anycast: C2RustUnnamed_0 = 5;
pub const ARG_brd: C2RustUnnamed_0 = 4;
pub const ARG_broadcast: C2RustUnnamed_0 = 3;
pub const ARG_nat: C2RustUnnamed_0 = 2;
pub const ARG_local: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_0 = libc::c_uint;

/*
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version
 * 2 of the License, or (at your option) any later version.
 *
 * Authors: Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
 */
#[no_mangle]
pub unsafe extern "C" fn rtnl_rtntype_n2a(mut id: libc::c_int) -> *const libc::c_char {
  match id {
    0 => return b"none\x00" as *const u8 as *const libc::c_char,
    1 => return b"unicast\x00" as *const u8 as *const libc::c_char,
    2 => return b"local\x00" as *const u8 as *const libc::c_char,
    3 => return b"broadcast\x00" as *const u8 as *const libc::c_char,
    4 => return b"anycast\x00" as *const u8 as *const libc::c_char,
    5 => return b"multicast\x00" as *const u8 as *const libc::c_char,
    6 => return b"blackhole\x00" as *const u8 as *const libc::c_char,
    7 => return b"unreachable\x00" as *const u8 as *const libc::c_char,
    8 => return b"prohibit\x00" as *const u8 as *const libc::c_char,
    9 => return b"throw\x00" as *const u8 as *const libc::c_char,
    10 => return b"nat\x00" as *const u8 as *const libc::c_char,
    11 => return b"xresolve\x00" as *const u8 as *const libc::c_char,
    _ => return itoa(id),
  };
}
#[no_mangle]
pub unsafe extern "C" fn rtnl_rtntype_a2n(
  mut id: *mut libc::c_int,
  mut arg: *mut libc::c_char,
) -> libc::c_int {
  static mut keywords: [libc::c_char; 97] = [
    108, 111, 99, 97, 108, 0, 110, 97, 116, 0, 98, 114, 111, 97, 100, 99, 97, 115, 116, 0, 98, 114,
    100, 0, 97, 110, 121, 99, 97, 115, 116, 0, 109, 117, 108, 116, 105, 99, 97, 115, 116, 0, 112,
    114, 111, 104, 105, 98, 105, 116, 0, 117, 110, 114, 101, 97, 99, 104, 97, 98, 108, 101, 0, 98,
    108, 97, 99, 107, 104, 111, 108, 101, 0, 120, 114, 101, 115, 111, 108, 118, 101, 0, 117, 110,
    105, 99, 97, 115, 116, 0, 116, 104, 114, 111, 119, 0, 0,
  ];
  let key: smalluint = (index_in_substrings(keywords.as_ptr(), arg) + 1i32) as smalluint;
  let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut res: libc::c_ulong = 0;
  if key as libc::c_int == ARG_local as libc::c_int {
    res = RTN_LOCAL as libc::c_int as libc::c_ulong
  } else if key as libc::c_int == ARG_nat as libc::c_int {
    res = RTN_NAT as libc::c_int as libc::c_ulong
  } else if key as libc::c_int == ARG_broadcast as libc::c_int
    || key as libc::c_int == ARG_brd as libc::c_int
  {
    res = RTN_BROADCAST as libc::c_int as libc::c_ulong
  } else if key as libc::c_int == ARG_anycast as libc::c_int {
    res = RTN_ANYCAST as libc::c_int as libc::c_ulong
  } else if key as libc::c_int == ARG_multicast as libc::c_int {
    res = RTN_MULTICAST as libc::c_int as libc::c_ulong
  } else if key as libc::c_int == ARG_prohibit as libc::c_int {
    res = RTN_PROHIBIT as libc::c_int as libc::c_ulong
  } else if key as libc::c_int == ARG_unreachable as libc::c_int {
    res = RTN_UNREACHABLE as libc::c_int as libc::c_ulong
  } else if key as libc::c_int == ARG_blackhole as libc::c_int {
    res = RTN_BLACKHOLE as libc::c_int as libc::c_ulong
  } else if key as libc::c_int == ARG_xresolve as libc::c_int {
    res = RTN_XRESOLVE as libc::c_int as libc::c_ulong
  } else if key as libc::c_int == ARG_unicast as libc::c_int {
    res = RTN_UNICAST as libc::c_int as libc::c_ulong
  } else if key as libc::c_int == ARG_throw as libc::c_int {
    res = RTN_THROW as libc::c_int as libc::c_ulong
  } else {
    res = strtoul(arg, &mut end, 0i32);
    if end == arg || *end as libc::c_int != 0 || res > 255i32 as libc::c_ulong {
      return -1i32;
    }
  }
  *id = res as libc::c_int;
  return 0i32;
}

#[no_mangle]
pub unsafe extern "C" fn get_rt_realms(
  mut realms: *mut uint32_t,
  mut arg: *mut libc::c_char,
) -> libc::c_int {
  let mut realm: uint32_t = 0i32 as uint32_t;
  let mut p: *mut libc::c_char = strchr(arg, '/' as i32);
  *realms = 0i32 as uint32_t;
  if !p.is_null() {
    *p = 0i32 as libc::c_char;
    if rtnl_rtrealm_a2n(realms, arg) != 0 {
      *p = '/' as i32 as libc::c_char;
      return -1i32;
    }
    *realms <<= 16i32;
    *p = '/' as i32 as libc::c_char;
    arg = p.offset(1)
  }
  if *arg as libc::c_int != 0 && rtnl_rtrealm_a2n(&mut realm, arg) != 0 {
    return -1i32;
  }
  *realms |= realm;
  return 0i32;
}

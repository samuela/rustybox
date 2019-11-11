use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::appletlib::applet_name;
use libc;
use libc::strcasecmp;
extern "C" {
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn snprintf(
    _: *mut libc::c_char,
    _: libc::c_ulong,
    _: *const libc::c_char,
    _: ...
  ) -> libc::c_int;

  #[no_mangle]
  fn nth_string(strings: *const libc::c_char, n: libc::c_int) -> *const libc::c_char;

  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
}

use crate::librb::size_t;
/*
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version
 * 2 of the License, or (at your option) any later version.
 *
 * Authors: Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
 */
/* Please conditionalize exotic protocols on CONFIG_something */
static mut llproto_ids: [u16; 43] = [
  0x60i32 as u16,
  0x200i32 as u16,
  0x201i32 as u16,
  0x800i32 as u16,
  0x805i32 as u16,
  0x806i32 as u16,
  0x8ffi32 as u16,
  0xa00i32 as u16,
  0xa01i32 as u16,
  0x6000i32 as u16,
  0x6001i32 as u16,
  0x6002i32 as u16,
  0x6003i32 as u16,
  0x6004i32 as u16,
  0x6005i32 as u16,
  0x6006i32 as u16,
  0x6007i32 as u16,
  0x8035i32 as u16,
  0x809bi32 as u16,
  0x80f3i32 as u16,
  0x8137i32 as u16,
  0x86ddi32 as u16,
  0x8863i32 as u16,
  0x8864i32 as u16,
  0x884ci32 as u16,
  0x8884i32 as u16,
  0x1i32 as u16,
  0x2i32 as u16,
  0x3i32 as u16,
  0x4i32 as u16,
  0x5i32 as u16,
  0x6i32 as u16,
  0x7i32 as u16,
  0x8i32 as u16,
  0x9i32 as u16,
  0x10i32 as u16,
  0x11i32 as u16,
  0x15i32 as u16,
  0x16i32 as u16,
  0x17i32 as u16,
  0x18i32 as u16,
  0x8100i32 as u16,
  0x800i32 as u16,
];
/* Keep declarations above and below in sync! */
static mut llproto_names: [libc::c_char; 264] = [
  108, 111, 111, 112, 0, 112, 117, 112, 0, 112, 117, 112, 97, 116, 0, 105, 112, 0, 120, 50, 53, 0,
  97, 114, 112, 0, 98, 112, 113, 0, 105, 101, 101, 101, 112, 117, 112, 0, 105, 101, 101, 101, 112,
  117, 112, 97, 116, 0, 100, 101, 99, 0, 100, 110, 97, 95, 100, 108, 0, 100, 110, 97, 95, 114, 99,
  0, 100, 110, 97, 95, 114, 116, 0, 108, 97, 116, 0, 100, 105, 97, 103, 0, 99, 117, 115, 116, 0,
  115, 99, 97, 0, 114, 97, 114, 112, 0, 97, 116, 97, 108, 107, 0, 97, 97, 114, 112, 0, 105, 112,
  120, 0, 105, 112, 118, 54, 0, 112, 112, 112, 95, 100, 105, 115, 99, 0, 112, 112, 112, 95, 115,
  101, 115, 0, 97, 116, 109, 109, 112, 111, 97, 0, 97, 116, 109, 102, 97, 116, 101, 0, 56, 48, 50,
  95, 51, 0, 97, 120, 50, 53, 0, 97, 108, 108, 0, 56, 48, 50, 95, 50, 0, 115, 110, 97, 112, 0, 100,
  100, 99, 109, 112, 0, 119, 97, 110, 95, 112, 112, 112, 0, 112, 112, 112, 95, 109, 112, 0, 108,
  111, 99, 97, 108, 116, 97, 108, 107, 0, 112, 112, 112, 116, 97, 108, 107, 0, 116, 114, 95, 56,
  48, 50, 95, 50, 0, 109, 111, 98, 105, 116, 101, 120, 0, 99, 111, 110, 116, 114, 111, 108, 0, 105,
  114, 100, 97, 0, 101, 99, 111, 110, 101, 116, 0, 56, 48, 50, 46, 49, 81, 0, 105, 112, 118, 52, 0,
  0,
];
#[no_mangle]
pub unsafe extern "C" fn ll_proto_n2a(
  mut id: libc::c_ushort,
  mut buf: *mut libc::c_char,
  mut len: libc::c_int,
) -> *const libc::c_char {
  let mut i: libc::c_uint = 0;
  id = {
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = id;
    if 0 != 0 {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh0 = &mut __v;
      let fresh1;
      let fresh2 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh1) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2)) :
                      "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
    }
    __v
  };
  i = 0i32 as libc::c_uint;
  while i
    < (::std::mem::size_of::<[u16; 43]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<u16>() as libc::c_ulong) as libc::c_uint
  {
    if llproto_ids[i as usize] as libc::c_int == id as libc::c_int {
      return nth_string(llproto_names.as_ptr(), i as libc::c_int);
    }
    i = i.wrapping_add(1)
  }
  snprintf(
    buf,
    len as libc::c_ulong,
    b"[%u]\x00" as *const u8 as *const libc::c_char,
    id as libc::c_int,
  );
  return buf;
}
#[no_mangle]
pub unsafe extern "C" fn ll_proto_a2n(
  mut id: *mut libc::c_ushort,
  mut buf: *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut i: libc::c_uint = 0;
  let mut name: *const libc::c_char = llproto_names.as_ptr();
  i = 0i32 as libc::c_uint;
  loop {
    if !(i
      < (::std::mem::size_of::<[u16; 43]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<u16>() as libc::c_ulong) as libc::c_uint)
    {
      current_block = 11006700562992250127;
      break;
    }
    if strcasecmp(name, buf) == 0i32 {
      i = llproto_ids[i as usize] as libc::c_uint;
      current_block = 5363633542473747200;
      break;
    } else {
      name = name.offset(strlen(name).wrapping_add(1i32 as libc::c_ulong) as isize);
      i = i.wrapping_add(1)
    }
  }
  match current_block {
    11006700562992250127 => {
      *bb_errno = 0i32;
      i = bb_strtou(buf, 0 as *mut *mut libc::c_char, 0i32);
      if *bb_errno != 0 || i > 0xffffi32 as libc::c_uint {
        return -1i32;
      }
    }
    _ => {}
  }
  *id = {
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = i as libc::c_ushort;
    if 0 != 0 {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh3 = &mut __v;
      let fresh4;
      let fresh5 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh4) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5)) :
                      "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
    }
    __v
  };
  return 0i32;
}


use libc;
pub type uintptr_t = libc::c_ulong;

/* implementation of the LZO1[XY]-1 compression algorithm

  This file is part of the LZO real-time data compression library.

  Copyright (C) 1996..2008 Markus Franz Xaver Johannes Oberhumer
  All Rights Reserved.

  Markus F.X.J. Oberhumer <markus@oberhumer.com>
  http://www.oberhumer.com/opensource/lzo/

  The LZO library is free software; you can redistribute it and/or
  modify it under the terms of the GNU General Public License as
  published by the Free Software Foundation; either version 2 of
  the License, or (at your option) any later version.

  The LZO library is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with the LZO library; see the file COPYING.
  If not, write to the Free Software Foundation, Inc.,
  51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.
*/
/* **********************************************************************
// compress a block of data.
************************************************************************/
#[inline(never)]
unsafe extern "C" fn do_compress(
  mut in_0: *const u8,
  mut in_len: libc::c_uint,
  mut out: *mut u8,
  mut out_len: *mut libc::c_uint,
  mut wrkmem: *mut libc::c_void,
) -> libc::c_uint {
  let mut current_block: u64;
  let mut ip: *const u8 = 0 as *const u8;
  let mut op: *mut u8 = 0 as *mut u8;
  let in_end: *const u8 = in_0.offset(in_len as isize);
  let ip_end: *const u8 = in_0.offset(in_len as isize).offset(-8).offset(-5);
  let mut ii: *const u8 = 0 as *const u8;
  let dict: *mut *const libc::c_void = wrkmem as *mut *const libc::c_void;
  op = out;
  ip = in_0;
  ii = ip;
  ip = ip.offset(4);
  loop {
    let mut m_pos: *const u8 = 0 as *const u8;
    let mut m_off: libc::c_uint = 0;
    let mut m_len: libc::c_uint = 0;
    let mut dindex: libc::c_uint = 0;
    dindex = (0x21i32 as libc::c_uint).wrapping_mul(
      (((*ip.offset(1).offset(2) as libc::c_uint) << 6i32
        ^ *ip.offset(1).offset(1) as libc::c_uint)
        << 5i32
        ^ *ip.offset(1).offset(0) as libc::c_uint)
        << 5i32
        ^ *ip.offset(0) as libc::c_uint,
    ) >> 5i32
      & (1u32 << 14i32).wrapping_sub(1i32 as libc::c_uint);
    m_pos = *dict.offset(dindex as isize) as *const u8;
    m_pos = ip.offset(-(ip.wrapping_offset_from(m_pos) as libc::c_long as libc::c_uint as isize));
    if !((m_pos as uintptr_t) < in_0 as uintptr_t
      || {
        m_off = ip.wrapping_offset_from(m_pos) as libc::c_long as libc::c_uint;
        (m_off) <= 0i32 as libc::c_uint
      }
      || m_off > 0xbfffi32 as libc::c_uint)
    {
      if m_off <= 0x800i32 as libc::c_uint
        || *m_pos.offset(3) as libc::c_int == *ip.offset(3) as libc::c_int
      {
        current_block = 1789672072190159347;
      } else {
        dindex = dindex
          & ((1u32 << 14i32).wrapping_sub(1i32 as libc::c_uint) & 0x7ffi32 as libc::c_uint)
          ^ (((1u32 << 14i32).wrapping_sub(1i32 as libc::c_uint) >> 1i32)
            .wrapping_add(1i32 as libc::c_uint)
            | 0x1fi32 as libc::c_uint);
        m_pos = *dict.offset(dindex as isize) as *const u8;
        m_pos =
          ip.offset(-(ip.wrapping_offset_from(m_pos) as libc::c_long as libc::c_uint as isize));
        if (m_pos as uintptr_t) < in_0 as uintptr_t
          || {
            m_off = ip.wrapping_offset_from(m_pos) as libc::c_long as libc::c_uint;
            (m_off) <= 0i32 as libc::c_uint
          }
          || m_off > 0xbfffi32 as libc::c_uint
        {
          current_block = 3833634915736423129;
        } else if m_off <= 0x800i32 as libc::c_uint
          || *m_pos.offset(3) as libc::c_int == *ip.offset(3) as libc::c_int
        {
          current_block = 1789672072190159347;
        } else {
          current_block = 3833634915736423129;
        }
      }
      match current_block {
        3833634915736423129 => {}
        _ => {
          if !(*m_pos.offset(0) as libc::c_int != *ip.offset(0) as libc::c_int
            || *m_pos.offset(1) as libc::c_int != *ip.offset(1) as libc::c_int)
          {
            if *m_pos.offset(2) as libc::c_int == *ip.offset(2) as libc::c_int {
              /* a match */
              let ref mut fresh1 = *dict.offset(dindex as isize);
              *fresh1 = ip as *const libc::c_void;
              /* store current literal run */
              if ip.wrapping_offset_from(ii) as libc::c_long as libc::c_uint > 0i32 as libc::c_uint
              {
                let mut t: libc::c_uint =
                  ip.wrapping_offset_from(ii) as libc::c_long as libc::c_uint;
                if t <= 3i32 as libc::c_uint {
                  let ref mut fresh2 = *op.offset(-2i32 as isize);
                  *fresh2 = (*fresh2 as libc::c_int | t as u8 as libc::c_int) as u8
                } else if t <= 18i32 as libc::c_uint {
                  let fresh3 = op;
                  op = op.offset(1);
                  *fresh3 = t.wrapping_sub(3i32 as libc::c_uint) as u8
                } else {
                  let mut tt: libc::c_uint = t.wrapping_sub(18i32 as libc::c_uint);
                  let fresh4 = op;
                  op = op.offset(1);
                  *fresh4 = 0i32 as u8;
                  while tt > 255i32 as libc::c_uint {
                    tt = tt.wrapping_sub(255i32 as libc::c_uint);
                    let fresh5 = op;
                    op = op.offset(1);
                    *fresh5 = 0i32 as u8
                  }
                  let fresh6 = op;
                  op = op.offset(1);
                  *fresh6 = tt as u8
                }
                loop {
                  let fresh7 = ii;
                  ii = ii.offset(1);
                  let fresh8 = op;
                  op = op.offset(1);
                  *fresh8 = *fresh7;
                  t = t.wrapping_sub(1);
                  if !(t > 0i32 as libc::c_uint) {
                    break;
                  }
                }
              }
              /* code the match */
              ip = ip.offset(3);
              let fresh9 = ip;
              ip = ip.offset(1);
              if *m_pos.offset(3) as libc::c_int != *fresh9 as libc::c_int
                || {
                  let fresh10 = ip;
                  ip = ip.offset(1);
                  (*m_pos.offset(4) as libc::c_int) != *fresh10 as libc::c_int
                }
                || {
                  let fresh11 = ip;
                  ip = ip.offset(1);
                  (*m_pos.offset(5) as libc::c_int) != *fresh11 as libc::c_int
                }
                || {
                  let fresh12 = ip;
                  ip = ip.offset(1);
                  (*m_pos.offset(6) as libc::c_int) != *fresh12 as libc::c_int
                }
                || {
                  let fresh13 = ip;
                  ip = ip.offset(1);
                  (*m_pos.offset(7) as libc::c_int) != *fresh13 as libc::c_int
                }
                || {
                  let fresh14 = ip;
                  ip = ip.offset(1);
                  (*m_pos.offset(8) as libc::c_int) != *fresh14 as libc::c_int
                }
              {
                ip = ip.offset(-1);
                m_len = ip.wrapping_offset_from(ii) as libc::c_long as libc::c_uint;
                if m_off <= 0x800i32 as libc::c_uint {
                  m_off = m_off.wrapping_sub(1i32 as libc::c_uint);
                  let fresh15 = op;
                  op = op.offset(1);
                  *fresh15 = (m_len.wrapping_sub(1i32 as libc::c_uint) << 5i32
                    | (m_off & 7i32 as libc::c_uint) << 2i32) as u8;
                  let fresh16 = op;
                  op = op.offset(1);
                  *fresh16 = (m_off >> 3i32) as u8;
                  current_block = 5023038348526654800;
                } else {
                  if m_off <= 0x4000i32 as libc::c_uint {
                    m_off = m_off.wrapping_sub(1i32 as libc::c_uint);
                    let fresh17 = op;
                    op = op.offset(1);
                    *fresh17 =
                      (32i32 as libc::c_uint | m_len.wrapping_sub(2i32 as libc::c_uint)) as u8
                  } else {
                    m_off = m_off.wrapping_sub(0x4000i32 as libc::c_uint);
                    let fresh18 = op;
                    op = op.offset(1);
                    *fresh18 = (16i32 as libc::c_uint
                      | (m_off & 0x4000i32 as libc::c_uint) >> 11i32
                      | m_len.wrapping_sub(2i32 as libc::c_uint))
                      as u8
                  }
                  current_block = 9439802700345910410;
                }
              } else {
                let mut end: *const u8 = in_end;
                let mut m: *const u8 = m_pos.offset(8).offset(1);
                while ip < end && *m as libc::c_int == *ip as libc::c_int {
                  m = m.offset(1);
                  ip = ip.offset(1)
                }
                m_len = ip.wrapping_offset_from(ii) as libc::c_long as libc::c_uint;
                let mut current_block_67: u64;
                if m_off <= 0x4000i32 as libc::c_uint {
                  m_off = m_off.wrapping_sub(1i32 as libc::c_uint);
                  if m_len <= 33i32 as libc::c_uint {
                    let fresh19 = op;
                    op = op.offset(1);
                    *fresh19 =
                      (32i32 as libc::c_uint | m_len.wrapping_sub(2i32 as libc::c_uint)) as u8;
                    current_block_67 = 16037123508100270995;
                  } else {
                    m_len = m_len.wrapping_sub(33i32 as libc::c_uint);
                    let fresh20 = op;
                    op = op.offset(1);
                    *fresh20 = (32i32 | 0i32) as u8;
                    current_block_67 = 9936362640277870553;
                  }
                } else {
                  m_off = m_off.wrapping_sub(0x4000i32 as libc::c_uint);
                  if m_len <= 9i32 as libc::c_uint {
                    let fresh21 = op;
                    op = op.offset(1);
                    *fresh21 = (16i32 as libc::c_uint
                      | (m_off & 0x4000i32 as libc::c_uint) >> 11i32
                      | m_len.wrapping_sub(2i32 as libc::c_uint))
                      as u8;
                    current_block_67 = 16037123508100270995;
                  } else {
                    m_len = m_len.wrapping_sub(9i32 as libc::c_uint);
                    let fresh22 = op;
                    op = op.offset(1);
                    *fresh22 =
                      (16i32 as libc::c_uint | (m_off & 0x4000i32 as libc::c_uint) >> 11i32) as u8;
                    current_block_67 = 9936362640277870553;
                  }
                }
                match current_block_67 {
                  9936362640277870553 => {
                    while m_len > 255i32 as libc::c_uint {
                      m_len = m_len.wrapping_sub(255i32 as libc::c_uint);
                      let fresh23 = op;
                      op = op.offset(1);
                      *fresh23 = 0i32 as u8
                    }
                    let fresh24 = op;
                    op = op.offset(1);
                    *fresh24 = m_len as u8
                  }
                  _ => {}
                }
                current_block = 9439802700345910410;
              }
              match current_block {
                9439802700345910410 => {
                  let fresh25 = op;
                  op = op.offset(1);
                  *fresh25 = ((m_off & 63i32 as libc::c_uint) << 2i32) as u8;
                  let fresh26 = op;
                  op = op.offset(1);
                  *fresh26 = (m_off >> 6i32) as u8
                }
                _ => {}
              }
              ii = ip;
              if ip >= ip_end {
                break;
              } else {
                continue;
              }
            }
          }
        }
      }
    }
    /* a literal */
    let ref mut fresh0 = *dict.offset(dindex as isize);
    *fresh0 = ip as *const libc::c_void;
    ip = ip.offset(1);
    if ip >= ip_end {
      break;
    }
  }
  *out_len = op.wrapping_offset_from(out) as libc::c_long as libc::c_uint;
  return in_end.wrapping_offset_from(ii) as libc::c_long as libc::c_uint;
}
/*
  This file is part of the LZO real-time data compression library.

  Copyright (C) 1996..2008 Markus Franz Xaver Johannes Oberhumer
  All Rights Reserved.

  Markus F.X.J. Oberhumer <markus@oberhumer.com>
  http://www.oberhumer.com/opensource/lzo/

  The LZO library is free software; you can redistribute it and/or
  modify it under the terms of the GNU General Public License as
  published by the Free Software Foundation; either version 2 of
  the License, or (at your option) any later version.

  The LZO library is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with the LZO library; see the file COPYING.
  If not, write to the Free Software Foundation, Inc.,
  51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.
*/
/*
static void die_at(int line)
{
  bb_error_msg_and_die("internal error at %d", line);
}
#define assert(v) if (!(v)) die_at(__LINE__)
*/
/* **********************************************************************
// public entry point
************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn lzo1x_1_compress(
  mut in_0: *const u8,
  mut in_len: libc::c_uint,
  mut out: *mut u8,
  mut out_len: *mut libc::c_uint,
  mut wrkmem: *mut libc::c_void,
) -> libc::c_int {
  let mut op: *mut u8 = out;
  let mut t: libc::c_uint = 0;
  if in_len <= (8i32 + 5i32) as libc::c_uint {
    t = in_len
  } else {
    t = do_compress(in_0, in_len, op, out_len, wrkmem);
    op = op.offset(*out_len as isize)
  }
  if t > 0i32 as libc::c_uint {
    let mut ii: *const u8 = in_0.offset(in_len as isize).offset(-(t as isize));
    if op == out && t <= 238i32 as libc::c_uint {
      let fresh27 = op;
      op = op.offset(1);
      *fresh27 = (17i32 as libc::c_uint).wrapping_add(t) as u8
    } else if t <= 3i32 as libc::c_uint {
      let ref mut fresh28 = *op.offset(-2i32 as isize);
      *fresh28 = (*fresh28 as libc::c_int | t as u8 as libc::c_int) as u8
    } else if t <= 18i32 as libc::c_uint {
      let fresh29 = op;
      op = op.offset(1);
      *fresh29 = t.wrapping_sub(3i32 as libc::c_uint) as u8
    } else {
      let mut tt: libc::c_uint = t.wrapping_sub(18i32 as libc::c_uint);
      let fresh30 = op;
      op = op.offset(1);
      *fresh30 = 0i32 as u8;
      while tt > 255i32 as libc::c_uint {
        tt = tt.wrapping_sub(255i32 as libc::c_uint);
        let fresh31 = op;
        op = op.offset(1);
        *fresh31 = 0i32 as u8
      }
      let fresh32 = op;
      op = op.offset(1);
      *fresh32 = tt as u8
    }
    loop {
      let fresh33 = ii;
      ii = ii.offset(1);
      let fresh34 = op;
      op = op.offset(1);
      *fresh34 = *fresh33;
      t = t.wrapping_sub(1);
      if !(t > 0i32 as libc::c_uint) {
        break;
      }
    }
  }
  let fresh35 = op;
  op = op.offset(1);
  *fresh35 = (16i32 | 1i32) as u8;
  let fresh36 = op;
  op = op.offset(1);
  *fresh36 = 0i32 as u8;
  let fresh37 = op;
  op = op.offset(1);
  *fresh37 = 0i32 as u8;
  *out_len = op.wrapping_offset_from(out) as libc::c_long as libc::c_uint;
  return 0i32;
  /*LZO_E_OK*/
}

use crate::libbb::appletlib::applet_name;
use crate::librb::size_t;
use crate::librb::smallint;
use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
use libc::free;
use libc::strcmp;
use libc::strrchr;
extern "C" {
  #[no_mangle]
  fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  static mut option_mask32: u32;

  #[no_mangle]
  static mut global_crc32_table: *mut u32;

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];

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

/* decompression */
//int lzo1x_decompress(const u8* src, unsigned src_len,
//		u8* dst, unsigned* dst_len /*, void* wrkmem */);
/* safe decompression with overrun testing */

}

/* NB: unaligned parameter should be a pointer, aligned one -
 * a lvalue. This makes it more likely to not swap them by mistake
 */
/* #elif ... - add your favorite arch today! */
/* Unaligned, fixed-endian accessors */
/* unxz needs an aligned fixed-endian accessor.
 * (however, the compiler does not realize it's aligned, the cast is still necessary)
 */
/* ---- Size-saving "small" ints (arch-dependent) ----------- */
/* add other arches which benefit from this... */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct globals {
  pub chksum: chksum_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chksum_t {
  pub f_adler32: u32,
  pub f_crc32: u32,
}
use crate::archival::libarchive::bb_archive::transformer_state_t;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const M_LZO1X_999: C2RustUnnamed_0 = 3;
pub const M_LZO1X_1_15: C2RustUnnamed_0 = 2;
pub const M_LZO1X_1: C2RustUnnamed_0 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct header_t {
  pub version_be16: u16,
  pub lib_version_be16: u16,
  pub version_needed_to_extract_be16: u16,
  pub method: u8,
  pub level: u8,
  pub flags32: u32,
  pub mode_be32: u32,
  pub mtime_be32: u32,
  pub gmtdiff_be32: u32,
  pub len_and_name: [libc::c_char; 257],
}
/* Note: must be kept in sync with archival/bbunzip.c */
pub type C2RustUnnamed_1 = libc::c_uint;
pub const OPT_789: C2RustUnnamed_1 = 57344;
pub const OPT_k: C2RustUnnamed_1 = 262144;
pub const OPT_F: C2RustUnnamed_1 = 131072;
pub const OPT_C: C2RustUnnamed_1 = 65536;
pub const OPT_9: C2RustUnnamed_1 = 32768;
pub const OPT_8: C2RustUnnamed_1 = 16384;
pub const OPT_7: C2RustUnnamed_1 = 8192;
pub const OPT_6: C2RustUnnamed_1 = 4096;
pub const OPT_5: C2RustUnnamed_1 = 2048;
pub const OPT_4: C2RustUnnamed_1 = 1024;
pub const OPT_3: C2RustUnnamed_1 = 512;
pub const OPT_2: C2RustUnnamed_1 = 256;
pub const OPT_1: C2RustUnnamed_1 = 128;
pub const OPT_TEST: C2RustUnnamed_1 = 64;
pub const OPT_DECOMPRESS: C2RustUnnamed_1 = 32;
pub const OPT_QUIET: C2RustUnnamed_1 = 16;
pub const OPT_VERBOSE: C2RustUnnamed_1 = 8;
pub const OPT_KEEP: C2RustUnnamed_1 = 4;
pub const OPT_FORCE: C2RustUnnamed_1 = 2;
pub const OPT_STDOUT: C2RustUnnamed_1 = 1;
pub const LZO_BASE: C2RustUnnamed_2 = 65521;
/* largest prime smaller than 65536 */
/* NMAX is the largest n such that
 * 255n(n+1)/2 + (n+1)(BASE-1) <= 2^32-1 */
pub const LZO_NMAX: C2RustUnnamed_2 = 5552;
pub type C2RustUnnamed_2 = libc::c_uint;
unsafe extern "C" fn copy2(mut ip: *mut u8, mut m_pos: *const u8, mut off: libc::c_uint) {
  *ip.offset(0) = *m_pos.offset(0);
  if off == 1i32 as libc::c_uint {
    *ip.offset(1) = *m_pos.offset(0)
  } else {
    *ip.offset(1) = *m_pos.offset(1)
  };
}
unsafe extern "C" fn copy3(mut ip: *mut u8, mut m_pos: *const u8, mut off: libc::c_uint) {
  *ip.offset(0) = *m_pos.offset(0);
  if off == 1i32 as libc::c_uint {
    let ref mut fresh0 = *ip.offset(1);
    *fresh0 = *m_pos.offset(0);
    *ip.offset(2) = *fresh0
  } else if off == 2i32 as libc::c_uint {
    *ip.offset(1) = *m_pos.offset(1);
    *ip.offset(2) = *m_pos.offset(0)
  } else {
    *ip.offset(1) = *m_pos.offset(1);
    *ip.offset(2) = *m_pos.offset(2)
  };
}
#[inline(never)]
unsafe extern "C" fn lzo1x_optimize(
  mut in_0: *mut u8,
  mut in_len: libc::c_uint,
  mut out: *mut u8,
  mut out_len: *mut libc::c_uint,
) -> libc::c_int {
  let mut current_block: u64;
  let mut op: *mut u8 = std::ptr::null_mut();
  let mut ip: *mut u8 = std::ptr::null_mut();
  let mut t: libc::c_uint = 0;
  let mut m_pos: *mut u8 = std::ptr::null_mut();
  let ip_end: *mut u8 = in_0.offset(in_len as isize);
  let op_end: *mut u8 = out.offset(*out_len as isize);
  let mut litp: *mut u8 = std::ptr::null_mut();
  let mut lit: libc::c_uint = 0 as libc::c_uint;
  let mut next_lit: libc::c_uint = (2147483647i32 as libc::c_uint)
    .wrapping_mul(2u32)
    .wrapping_add(1u32);
  let mut nl: libc::c_uint = 0;
  let mut o_m1_a: libc::c_ulong = 0 as libc::c_ulong;
  let mut o_m1_b: libc::c_ulong = 0 as libc::c_ulong;
  let mut o_m2: libc::c_ulong = 0 as libc::c_ulong;
  let mut o_m3_a: libc::c_ulong = 0 as libc::c_ulong;
  let mut o_m3_b: libc::c_ulong = 0 as libc::c_ulong;
  *out_len = 0 as libc::c_uint;
  op = out;
  ip = in_0;
  if *ip as libc::c_int > 17i32 {
    let fresh1 = ip;
    ip = ip.offset(1);
    t = (*fresh1 as libc::c_int - 17i32) as libc::c_uint;
    if t < 4i32 as libc::c_uint {
      current_block = 1121007611148936282;
    } else {
      current_block = 9940819403756327342;
    }
  } else {
    current_block = 11050875288958768710;
  }
  loop {
    match current_block {
      11050875288958768710 => {
        if !(ip < ip_end && op <= op_end) {
          current_block = 7293850626974290116;
          break;
        }
        let fresh2 = ip;
        ip = ip.offset(1);
        t = *fresh2 as libc::c_uint;
        if t >= 16i32 as libc::c_uint {
          current_block = 16888526124901371750;
        } else {
          litp = ip.offset(-1);
          if t == 0 as libc::c_uint {
            t = 15i32 as libc::c_uint;
            while *ip as libc::c_int == 0 {
              t = t.wrapping_add(255i32 as libc::c_uint);
              ip = ip.offset(1)
            }
            let fresh3 = ip;
            ip = ip.offset(1);
            t = t.wrapping_add(*fresh3 as libc::c_uint)
          }
          lit = t.wrapping_add(3i32 as libc::c_uint);
          current_block = 11274834565634332203;
        }
      }
      9940819403756327342 => {
        loop {
          let fresh10 = ip;
          ip = ip.offset(1);
          let fresh11 = op;
          op = op.offset(1);
          *fresh11 = *fresh10;
          t = t.wrapping_sub(1);
          if !(t > 0 as libc::c_uint) {
            break;
          }
        }
        let fresh12 = ip;
        ip = ip.offset(1);
        t = *fresh12 as libc::c_uint;
        if t >= 16i32 as libc::c_uint {
          current_block = 16888526124901371750;
        } else {
          m_pos = op.offset(-1).offset(-(0x800i32 as isize));
          m_pos = m_pos.offset(-((t >> 2i32) as isize));
          let fresh13 = ip;
          ip = ip.offset(1);
          m_pos = m_pos.offset(-(((*fresh13 as libc::c_int) << 2i32) as isize));
          let fresh14 = m_pos;
          m_pos = m_pos.offset(1);
          let fresh15 = op;
          op = op.offset(1);
          *fresh15 = *fresh14;
          let fresh16 = m_pos;
          m_pos = m_pos.offset(1);
          let fresh17 = op;
          op = op.offset(1);
          *fresh17 = *fresh16;
          let fresh18 = m_pos;
          m_pos = m_pos.offset(1);
          let fresh19 = op;
          op = op.offset(1);
          *fresh19 = *fresh18;
          lit = 0 as libc::c_uint;
          current_block = 3582612801380737862;
        }
      }
      _ => {
        loop {
          let fresh57 = ip;
          ip = ip.offset(1);
          let fresh58 = op;
          op = op.offset(1);
          *fresh58 = *fresh57;
          t = t.wrapping_sub(1);
          if !(t > 0 as libc::c_uint) {
            break;
          }
        }
        let fresh59 = ip;
        ip = ip.offset(1);
        t = *fresh59 as libc::c_uint;
        if !(ip < ip_end && op <= op_end) {
          current_block = 11050875288958768710;
          continue;
        }
        if t < 16i32 as libc::c_uint {
          m_pos = op.offset(-1);
          m_pos = m_pos.offset(-((t >> 2i32) as isize));
          let fresh20 = ip;
          ip = ip.offset(1);
          m_pos = m_pos.offset(-(((*fresh20 as libc::c_int) << 2i32) as isize));
          if litp.is_null() {
            current_block = 1563163433588720361;
          } else {
            nl = (*ip.offset(-2i32 as isize) as libc::c_int & 3i32) as libc::c_uint;
            if nl == 0 as libc::c_uint
              && lit == 1i32 as libc::c_uint
              && *ip.offset(0) as libc::c_int >= 16i32
            {
              next_lit = nl;
              lit = lit.wrapping_add(2i32 as libc::c_uint);
              *litp = ((*litp as libc::c_int & !3i32) as libc::c_uint | lit) as libc::c_uchar;
              copy2(
                ip.offset(-2),
                m_pos,
                op.wrapping_offset_from(m_pos) as libc::c_long as libc::c_uint,
              );
              o_m1_a = o_m1_a.wrapping_add(1);
              current_block = 1563163433588720361;
            } else if nl == 0 as libc::c_uint
              && (*ip.offset(0) as libc::c_int) < 16i32
              && *ip.offset(0) as libc::c_int != 0
              && lit
                .wrapping_add(2i32 as libc::c_uint)
                .wrapping_add(*ip.offset(0) as libc::c_uint)
                < 16i32 as libc::c_uint
            {
              let fresh21 = ip;
              ip = ip.offset(1);
              t = *fresh21 as libc::c_uint;
              *litp = (*litp as libc::c_int & !3i32) as u8;
              copy2(
                ip.offset(-3).offset(1),
                m_pos,
                op.wrapping_offset_from(m_pos) as libc::c_long as libc::c_uint,
              );
              litp = litp.offset(2);
              if lit > 0 as libc::c_uint {
                memmove(
                  litp.offset(1) as *mut libc::c_void,
                  litp as *const libc::c_void,
                  lit as libc::c_ulong,
                );
              }
              lit = lit.wrapping_add(
                (2i32 as libc::c_uint)
                  .wrapping_add(t)
                  .wrapping_add(3i32 as libc::c_uint),
              );
              *litp = lit.wrapping_sub(3i32 as libc::c_uint) as libc::c_uchar;
              o_m1_b = o_m1_b.wrapping_add(1);
              let fresh22 = m_pos;
              m_pos = m_pos.offset(1);
              let fresh23 = op;
              op = op.offset(1);
              *fresh23 = *fresh22;
              let fresh24 = m_pos;
              m_pos = m_pos.offset(1);
              let fresh25 = op;
              op = op.offset(1);
              *fresh25 = *fresh24;
              current_block = 11274834565634332203;
            } else {
              current_block = 1563163433588720361;
            }
          }
          match current_block {
            11274834565634332203 => {}
            _ => {
              let fresh26 = m_pos;
              m_pos = m_pos.offset(1);
              let fresh27 = op;
              op = op.offset(1);
              *fresh27 = *fresh26;
              let fresh28 = m_pos;
              m_pos = m_pos.offset(1);
              let fresh29 = op;
              op = op.offset(1);
              *fresh29 = *fresh28;
              current_block = 3582612801380737862;
            }
          }
        } else {
          current_block = 16888526124901371750;
        }
      }
    }
    match current_block {
      16888526124901371750 => {
        if t >= 64i32 as libc::c_uint {
          m_pos = op.offset(-1);
          m_pos = m_pos.offset(-((t >> 2i32 & 7i32 as libc::c_uint) as isize));
          let fresh30 = ip;
          ip = ip.offset(1);
          m_pos = m_pos.offset(-(((*fresh30 as libc::c_int) << 3i32) as isize));
          t = (t >> 5i32).wrapping_sub(1i32 as libc::c_uint);
          if litp.is_null() {
            current_block = 65461189516158820;
          } else {
            nl = (*ip.offset(-2i32 as isize) as libc::c_int & 3i32) as libc::c_uint;
            if t == 1i32 as libc::c_uint
              && lit > 3i32 as libc::c_uint
              && nl == 0 as libc::c_uint
              && (*ip.offset(0) as libc::c_int) < 16i32
              && *ip.offset(0) as libc::c_int != 0
              && lit
                .wrapping_add(3i32 as libc::c_uint)
                .wrapping_add(*ip.offset(0) as libc::c_uint)
                < 16i32 as libc::c_uint
            {
              let fresh31 = ip;
              ip = ip.offset(1);
              t = *fresh31 as libc::c_uint;
              copy3(
                ip.offset(-1).offset(-2),
                m_pos,
                op.wrapping_offset_from(m_pos) as libc::c_long as libc::c_uint,
              );
              lit = lit.wrapping_add(
                (3i32 as libc::c_uint)
                  .wrapping_add(t)
                  .wrapping_add(3i32 as libc::c_uint),
              );
              *litp = lit.wrapping_sub(3i32 as libc::c_uint) as libc::c_uchar;
              o_m2 = o_m2.wrapping_add(1);
              let fresh32 = m_pos;
              m_pos = m_pos.offset(1);
              let fresh33 = op;
              op = op.offset(1);
              *fresh33 = *fresh32;
              let fresh34 = m_pos;
              m_pos = m_pos.offset(1);
              let fresh35 = op;
              op = op.offset(1);
              *fresh35 = *fresh34;
              let fresh36 = m_pos;
              m_pos = m_pos.offset(1);
              let fresh37 = op;
              op = op.offset(1);
              *fresh37 = *fresh36;
              current_block = 11274834565634332203;
            } else {
              current_block = 65461189516158820;
            }
          }
        } else {
          if t >= 32i32 as libc::c_uint {
            t &= 31i32 as libc::c_uint;
            if t == 0 as libc::c_uint {
              t = 31i32 as libc::c_uint;
              while *ip as libc::c_int == 0 {
                t = t.wrapping_add(255i32 as libc::c_uint);
                ip = ip.offset(1)
              }
              let fresh38 = ip;
              ip = ip.offset(1);
              t = t.wrapping_add(*fresh38 as libc::c_uint)
            }
            m_pos = op.offset(-1);
            let fresh39 = ip;
            ip = ip.offset(1);
            m_pos = m_pos.offset(-((*fresh39 as libc::c_int >> 2i32) as isize));
            let fresh40 = ip;
            ip = ip.offset(1);
            m_pos = m_pos.offset(-(((*fresh40 as libc::c_int) << 6i32) as isize))
          } else {
            m_pos = op;
            m_pos = m_pos.offset(-(((t & 8i32 as libc::c_uint) << 11i32) as isize));
            t &= 7i32 as libc::c_uint;
            if t == 0 as libc::c_uint {
              t = 7i32 as libc::c_uint;
              while *ip as libc::c_int == 0 {
                t = t.wrapping_add(255i32 as libc::c_uint);
                ip = ip.offset(1)
              }
              let fresh41 = ip;
              ip = ip.offset(1);
              t = t.wrapping_add(*fresh41 as libc::c_uint)
            }
            let fresh42 = ip;
            ip = ip.offset(1);
            m_pos = m_pos.offset(-((*fresh42 as libc::c_int >> 2i32) as isize));
            let fresh43 = ip;
            ip = ip.offset(1);
            m_pos = m_pos.offset(-(((*fresh43 as libc::c_int) << 6i32) as isize));
            if m_pos == op {
              current_block = 8050654829899665499;
              break;
            }
            m_pos = m_pos.offset(-(0x4000i32 as isize))
          }
          if litp.is_null() {
            current_block = 65461189516158820;
          } else {
            nl = (*ip.offset(-2i32 as isize) as libc::c_int & 3i32) as libc::c_uint;
            if t == 1i32 as libc::c_uint
              && lit == 0 as libc::c_uint
              && nl == 0 as libc::c_uint
              && *ip.offset(0) as libc::c_int >= 16i32
            {
              next_lit = nl;
              lit = lit.wrapping_add(3i32 as libc::c_uint);
              *litp = ((*litp as libc::c_int & !3i32) as libc::c_uint | lit) as libc::c_uchar;
              copy3(
                ip.offset(-3),
                m_pos,
                op.wrapping_offset_from(m_pos) as libc::c_long as libc::c_uint,
              );
              o_m3_a = o_m3_a.wrapping_add(1);
              current_block = 65461189516158820;
            } else if t == 1i32 as libc::c_uint
              && lit <= 3i32 as libc::c_uint
              && nl == 0 as libc::c_uint
              && (*ip.offset(0) as libc::c_int) < 16i32
              && *ip.offset(0) as libc::c_int != 0
              && lit
                .wrapping_add(3i32 as libc::c_uint)
                .wrapping_add(*ip.offset(0) as libc::c_uint)
                < 16i32 as libc::c_uint
            {
              let fresh44 = ip;
              ip = ip.offset(1);
              t = *fresh44 as libc::c_uint;
              *litp = (*litp as libc::c_int & !3i32) as u8;
              copy3(
                ip.offset(-4).offset(1),
                m_pos,
                op.wrapping_offset_from(m_pos) as libc::c_long as libc::c_uint,
              );
              litp = litp.offset(2);
              if lit > 0 as libc::c_uint {
                memmove(
                  litp.offset(1) as *mut libc::c_void,
                  litp as *const libc::c_void,
                  lit as libc::c_ulong,
                );
              }
              lit = lit.wrapping_add(
                (3i32 as libc::c_uint)
                  .wrapping_add(t)
                  .wrapping_add(3i32 as libc::c_uint),
              );
              *litp = lit.wrapping_sub(3i32 as libc::c_uint) as libc::c_uchar;
              o_m3_b = o_m3_b.wrapping_add(1);
              let fresh45 = m_pos;
              m_pos = m_pos.offset(1);
              let fresh46 = op;
              op = op.offset(1);
              *fresh46 = *fresh45;
              let fresh47 = m_pos;
              m_pos = m_pos.offset(1);
              let fresh48 = op;
              op = op.offset(1);
              *fresh48 = *fresh47;
              let fresh49 = m_pos;
              m_pos = m_pos.offset(1);
              let fresh50 = op;
              op = op.offset(1);
              *fresh50 = *fresh49;
              current_block = 11274834565634332203;
            } else {
              current_block = 65461189516158820;
            }
          }
        }
        match current_block {
          11274834565634332203 => {}
          _ => {
            let fresh51 = m_pos;
            m_pos = m_pos.offset(1);
            let fresh52 = op;
            op = op.offset(1);
            *fresh52 = *fresh51;
            let fresh53 = m_pos;
            m_pos = m_pos.offset(1);
            let fresh54 = op;
            op = op.offset(1);
            *fresh54 = *fresh53;
            loop {
              let fresh55 = m_pos;
              m_pos = m_pos.offset(1);
              let fresh56 = op;
              op = op.offset(1);
              *fresh56 = *fresh55;
              t = t.wrapping_sub(1);
              if !(t > 0 as libc::c_uint) {
                break;
              }
            }
            current_block = 3582612801380737862;
          }
        }
      }
      _ => {}
    }
    match current_block {
      3582612801380737862 => {
        if next_lit
          == (2147483647i32 as libc::c_uint)
            .wrapping_mul(2u32)
            .wrapping_add(1u32)
        {
          t = (*ip.offset(-2i32 as isize) as libc::c_int & 3i32) as libc::c_uint;
          lit = t;
          litp = ip.offset(-2)
        } else {
          t = next_lit
        }
        next_lit = (2147483647i32 as libc::c_uint)
          .wrapping_mul(2u32)
          .wrapping_add(1u32);
        if t == 0 as libc::c_uint {
          current_block = 11050875288958768710;
        } else {
          current_block = 1121007611148936282;
        }
      }
      _ => {
        let fresh4 = ip;
        ip = ip.offset(1);
        let fresh5 = op;
        op = op.offset(1);
        *fresh5 = *fresh4;
        let fresh6 = ip;
        ip = ip.offset(1);
        let fresh7 = op;
        op = op.offset(1);
        *fresh7 = *fresh6;
        let fresh8 = ip;
        ip = ip.offset(1);
        let fresh9 = op;
        op = op.offset(1);
        *fresh9 = *fresh8;
        current_block = 9940819403756327342;
      }
    }
  }
  match current_block {
    8050654829899665499 => {
      *out_len = op.wrapping_offset_from(out) as libc::c_long as libc::c_uint;
      return if ip == ip_end {
        0
      } else if ip < ip_end {
        -8i32
      } else {
        -4i32
      };
    }
    _ => {
      *out_len = op.wrapping_offset_from(out) as libc::c_long as libc::c_uint;
      return -7i32;
    }
  };
}
/* *********************************************************************/
// adler32 checksum
// adapted from free code by Mark Adler <madler@alumni.caltech.edu>
// see http://www.zlib.org/
/* *********************************************************************/
unsafe extern "C" fn lzo_adler32(mut adler: u32, mut buf: *const u8, mut len: libc::c_uint) -> u32 {
  let mut s1: u32 = adler & 0xffffi32 as libc::c_uint;
  let mut s2: u32 = adler >> 16i32 & 0xffffi32 as libc::c_uint;
  let mut k: libc::c_uint = 0;
  if buf.is_null() {
    return 1i32 as u32;
  }
  while len > 0 as libc::c_uint {
    k = if len < LZO_NMAX as libc::c_int as libc::c_uint {
      len
    } else {
      LZO_NMAX as libc::c_int as libc::c_uint
    };
    len = len.wrapping_sub(k);
    if k != 0 as libc::c_uint {
      loop {
        let fresh60 = buf;
        buf = buf.offset(1);
        s1 = (s1 as libc::c_uint).wrapping_add(*fresh60 as libc::c_uint) as u32 as u32;
        s2 = (s2 as libc::c_uint).wrapping_add(s1) as u32 as u32;
        k = k.wrapping_sub(1);
        if !(k > 0 as libc::c_uint) {
          break;
        }
      }
    }
    s1 = (s1 as libc::c_uint).wrapping_rem(LZO_BASE as libc::c_int as libc::c_uint) as u32 as u32;
    s2 = (s2 as libc::c_uint).wrapping_rem(LZO_BASE as libc::c_int as libc::c_uint) as u32 as u32
  }
  return s2 << 16i32 | s1;
}
unsafe extern "C" fn lzo_crc32(mut c: u32, mut buf: *const u8, mut len: libc::c_uint) -> u32 {
  //if (buf == NULL) - impossible
  //	return 0;
  return !crate::libbb::crc32::crc32_block_endian0(
    !c,
    buf as *const libc::c_void,
    len,
    global_crc32_table,
  );
}
/* *********************************************************************/
unsafe extern "C" fn init_chksum() {
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .chksum
    .f_adler32 = 1i32 as u32;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .chksum
    .f_crc32 = 0 as u32;
}
unsafe extern "C" fn add_bytes_to_chksum(mut buf: *const libc::c_void, mut cnt: libc::c_int) {
  /* We need to handle the two checksums at once, because at the
   * beginning of the header, we don't know yet which one we'll
   * eventually need */
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .chksum
    .f_adler32 = lzo_adler32(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .chksum
      .f_adler32,
    buf as *const u8,
    cnt as libc::c_uint,
  );
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .chksum
    .f_crc32 = lzo_crc32(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .chksum
      .f_crc32,
    buf as *const u8,
    cnt as libc::c_uint,
  );
}
unsafe extern "C" fn chksum_getresult(mut h_flags32: u32) -> u32 {
  return if h_flags32 as libc::c_long & 0x1000i64 != 0 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .chksum
      .f_crc32
  } else {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .chksum
      .f_adler32
  };
}
/* *********************************************************************/
unsafe extern "C" fn read32() -> u32 {
  let mut v: u32 = 0;
  crate::libbb::read_printf::xread(0, &mut v as *mut u32 as *mut libc::c_void, 4i32 as size_t);
  return {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = v;
    if false {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh61 = &mut __v;
      let fresh62;
      let fresh63 = __x;
      asm!("bswap $0" : "=r" (fresh62) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh61, fresh63)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh61, fresh63, fresh62);
    }
    __v
  };
}
unsafe extern "C" fn f_read(mut buf: *mut libc::c_void, mut cnt: libc::c_int) {
  crate::libbb::read_printf::xread(0i32, buf, cnt as size_t);
  add_bytes_to_chksum(buf, cnt);
}
//static int f_read8(void)
//{
//	u8 v;
//	f_read(&v, 1);
//	return v;
//}
//static unsigned f_read16(void)
//{
//	u16 v;
//	f_read(&v, 2);
//	return ntohs(v);
//}
unsafe extern "C" fn f_read32() -> u32 {
  let mut v: u32 = 0;
  f_read(&mut v as *mut u32 as *mut libc::c_void, 4i32);
  return {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = v;
    if false {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh64 = &mut __v;
      let fresh65;
      let fresh66 = __x;
      asm!("bswap $0" : "=r" (fresh65) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh64, fresh66)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh64, fresh66, fresh65);
    }
    __v
  };
}
unsafe extern "C" fn write32(mut v: u32) {
  v = {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = v;
    if false {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh67 = &mut __v;
      let fresh68;
      let fresh69 = __x;
      asm!("bswap $0" : "=r" (fresh68) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh67, fresh69)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh67, fresh69, fresh68);
    }
    __v
  };
  crate::libbb::xfuncs_printf::xwrite(
    1i32,
    &mut v as *mut u32 as *const libc::c_void,
    4i32 as size_t,
  );
}
unsafe extern "C" fn f_write(mut buf: *const libc::c_void, mut cnt: libc::c_int) {
  crate::libbb::xfuncs_printf::xwrite(1i32, buf, cnt as size_t);
  add_bytes_to_chksum(buf, cnt);
}
/* DO NOT CHANGE */
/* LZO may expand uncompressible data by a small amount */
/* *********************************************************************/
// compress a file
/* *********************************************************************/
#[inline(never)]
unsafe extern "C" fn lzo_compress(mut h: *const header_t) -> libc::c_int {
  let mut block_size: libc::c_uint = (256i32 as libc::c_long * 1024i64) as libc::c_uint; /* LZO_E_OK */
  let mut r: libc::c_int = 0;
  let b1: *mut u8 = crate::libbb::xfuncs_printf::xzalloc(block_size as size_t) as *mut u8;
  let b2: *mut u8 = crate::libbb::xfuncs_printf::xzalloc(
    block_size
      .wrapping_add(block_size.wrapping_div(16i32 as libc::c_uint))
      .wrapping_add(64i32 as libc::c_uint)
      .wrapping_add(3i32 as libc::c_uint) as size_t,
  ) as *mut u8;
  let mut d_adler32: u32 = 1i32 as u32;
  let mut d_crc32: u32 = 0 as u32;
  let mut wrk_mem: *mut u8 = std::ptr::null_mut();
  /* Only these methods are possible, see lzo_set_method():
   * -1:    M_LZO1X_1_15
   * -2..6: M_LZO1X_1
   * -7..9: M_LZO1X_999 if ENABLE_LZOP_COMPR_HIGH
   */
  if (*h).method as libc::c_int == M_LZO1X_1 as libc::c_int {
    wrk_mem = crate::libbb::xfuncs_printf::xzalloc(
      (16384i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut u8>() as libc::c_ulong),
    ) as *mut u8
  } else {
    /* check only if it's not the only possibility */
    wrk_mem = crate::libbb::xfuncs_printf::xzalloc(
      (32768i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut u8>() as libc::c_ulong),
    ) as *mut u8
  }
  loop {
    let mut src_len: libc::c_uint = 0;
    let mut dst_len: libc::c_uint = 0;
    let mut l: libc::c_int = 0;
    let mut wordbuf: [u32; 6] = [0; 6];
    let mut wordptr: *mut u32 = wordbuf.as_mut_ptr();
    // /* if full_read() was nevertheless "short", it was EOF */
    // if (src_len < block_size)
    // 	break;
    /* read a block */
    l = crate::libbb::read::full_read(0i32, b1 as *mut libc::c_void, block_size as size_t)
      as libc::c_int;
    src_len = if l > 0 { l } else { 0 } as libc::c_uint;
    if src_len == 0 as libc::c_uint {
      write32(0i32 as u32);
      break;
    } else {
      let fresh73 = wordptr;
      wordptr = wordptr.offset(1);
      *fresh73 = {
        let mut __v: libc::c_uint = 0;
        let mut __x: libc::c_uint = src_len;
        if false {
          __v = (__x & 0xff000000u32) >> 24i32
            | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
            | (__x & 0xff00i32 as libc::c_uint) << 8i32
            | (__x & 0xffi32 as libc::c_uint) << 24i32
        } else {
          let fresh70 = &mut __v;
          let fresh71;
          let fresh72 = __x;
          asm!("bswap $0" : "=r" (fresh71) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh70, fresh72)) :);
          c2rust_asm_casts::AsmCast::cast_out(fresh70, fresh72, fresh71);
        }
        __v
      };
      /* write uncompressed block size */
      /* exit if last block */
      /* compute checksum of uncompressed block */
      if (*h).flags32 as libc::c_long & 0x1i64 != 0 {
        d_adler32 = lzo_adler32(1i32 as u32, b1, src_len)
      }
      if (*h).flags32 as libc::c_long & 0x100i64 != 0 {
        d_crc32 = lzo_crc32(0i32 as u32, b1, src_len)
      }
      /* compress */
      if (*h).method as libc::c_int == M_LZO1X_1 as libc::c_int {
        r = crate::archival::libarchive::lzo1x_1::lzo1x_1_compress(
          b1,
          src_len,
          b2,
          &mut dst_len,
          wrk_mem as *mut libc::c_void,
        )
      } else {
        r = crate::archival::libarchive::lzo1x_1o::lzo1x_1_15_compress(
          b1,
          src_len,
          b2,
          &mut dst_len,
          wrk_mem as *mut libc::c_void,
        )
      }
      if r != 0 {
        /* not LZO_E_OK */
        crate::libbb::verror_msg::bb_error_msg_and_die(
          b"%s: %s\x00" as *const u8 as *const libc::c_char,
          b"internal error\x00" as *const u8 as *const libc::c_char,
          b"compression\x00" as *const u8 as *const libc::c_char,
        );
      }
      /* write compressed block size */
      if dst_len < src_len {
        /* optimize */
        if (*h).method as libc::c_int == M_LZO1X_999 as libc::c_int {
          let mut new_len: libc::c_uint = src_len;
          r = lzo1x_optimize(b2, dst_len, b1, &mut new_len);
          if r != 0 || new_len != src_len {
            crate::libbb::verror_msg::bb_error_msg_and_die(
              b"%s: %s\x00" as *const u8 as *const libc::c_char,
              b"internal error\x00" as *const u8 as *const libc::c_char,
              b"optimization\x00" as *const u8 as *const libc::c_char,
            );
          }
        }
        let fresh77 = wordptr;
        wordptr = wordptr.offset(1);
        *fresh77 = {
          let mut __v: libc::c_uint = 0;
          let mut __x: libc::c_uint = dst_len;
          if false {
            __v = (__x & 0xff000000u32) >> 24i32
              | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
              | (__x & 0xff00i32 as libc::c_uint) << 8i32
              | (__x & 0xffi32 as libc::c_uint) << 24i32
          } else {
            let fresh74 = &mut __v;
            let fresh75;
            let fresh76 = __x;
            asm!("bswap $0" : "=r" (fresh75) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh74, fresh76)) :);
            c2rust_asm_casts::AsmCast::cast_out(fresh74, fresh76, fresh75);
          }
          __v
        }
      } else {
        /* data actually expanded => store data uncompressed */
        let fresh81 = wordptr;
        wordptr = wordptr.offset(1);
        *fresh81 = {
          let mut __v: libc::c_uint = 0;
          let mut __x: libc::c_uint = src_len;
          if false {
            __v = (__x & 0xff000000u32) >> 24i32
              | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
              | (__x & 0xff00i32 as libc::c_uint) << 8i32
              | (__x & 0xffi32 as libc::c_uint) << 24i32
          } else {
            let fresh78 = &mut __v;
            let fresh79;
            let fresh80 = __x;
            asm!("bswap $0" : "=r" (fresh79) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh78, fresh80)) :);
            c2rust_asm_casts::AsmCast::cast_out(fresh78, fresh80, fresh79);
          }
          __v
        }
      }
      /* write checksum of uncompressed block */
      if (*h).flags32 as libc::c_long & 0x1i64 != 0 {
        let fresh85 = wordptr;
        wordptr = wordptr.offset(1);
        *fresh85 = {
          let mut __v: libc::c_uint = 0;
          let mut __x: libc::c_uint = d_adler32;
          if false {
            __v = (__x & 0xff000000u32) >> 24i32
              | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
              | (__x & 0xff00i32 as libc::c_uint) << 8i32
              | (__x & 0xffi32 as libc::c_uint) << 24i32
          } else {
            let fresh82 = &mut __v;
            let fresh83;
            let fresh84 = __x;
            asm!("bswap $0" : "=r" (fresh83) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh82, fresh84)) :);
            c2rust_asm_casts::AsmCast::cast_out(fresh82, fresh84, fresh83);
          }
          __v
        }
      }
      if (*h).flags32 as libc::c_long & 0x100i64 != 0 {
        let fresh89 = wordptr;
        wordptr = wordptr.offset(1);
        *fresh89 = {
          let mut __v: libc::c_uint = 0;
          let mut __x: libc::c_uint = d_crc32;
          if false {
            __v = (__x & 0xff000000u32) >> 24i32
              | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
              | (__x & 0xff00i32 as libc::c_uint) << 8i32
              | (__x & 0xffi32 as libc::c_uint) << 24i32
          } else {
            let fresh86 = &mut __v;
            let fresh87;
            let fresh88 = __x;
            asm!("bswap $0" : "=r" (fresh87) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh86, fresh88)) :);
            c2rust_asm_casts::AsmCast::cast_out(fresh86, fresh88, fresh87);
          }
          __v
        }
      }
      if dst_len < src_len {
        /* write checksum of compressed block */
        if (*h).flags32 as libc::c_long & 0x2i64 != 0 {
          let fresh93 = wordptr;
          wordptr = wordptr.offset(1);
          *fresh93 = {
            let mut __v: libc::c_uint = 0;
            let mut __x: libc::c_uint = lzo_adler32(1i32 as u32, b2, dst_len);
            if false {
              __v = (__x & 0xff000000u32) >> 24i32
                | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
                | (__x & 0xff00i32 as libc::c_uint) << 8i32
                | (__x & 0xffi32 as libc::c_uint) << 24i32
            } else {
              let fresh90 = &mut __v;
              let fresh91;
              let fresh92 = __x;
              asm!("bswap $0" : "=r" (fresh91) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh90, fresh92)) :);
              c2rust_asm_casts::AsmCast::cast_out(fresh90, fresh92, fresh91);
            }
            __v
          }
        }
        if (*h).flags32 as libc::c_long & 0x200i64 != 0 {
          let fresh97 = wordptr;
          wordptr = wordptr.offset(1);
          *fresh97 = {
            let mut __v: libc::c_uint = 0;
            let mut __x: libc::c_uint = lzo_crc32(0i32 as u32, b2, dst_len);
            if false {
              __v = (__x & 0xff000000u32) >> 24i32
                | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
                | (__x & 0xff00i32 as libc::c_uint) << 8i32
                | (__x & 0xffi32 as libc::c_uint) << 24i32
            } else {
              let fresh94 = &mut __v;
              let fresh95;
              let fresh96 = __x;
              asm!("bswap $0" : "=r" (fresh95) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh94, fresh96)) :);
              c2rust_asm_casts::AsmCast::cast_out(fresh94, fresh96, fresh95);
            }
            __v
          }
        }
      }
      crate::libbb::xfuncs_printf::xwrite(
        1i32,
        wordbuf.as_mut_ptr() as *const libc::c_void,
        (wordptr as *mut libc::c_char)
          .wrapping_offset_from(wordbuf.as_mut_ptr() as *mut libc::c_char) as libc::c_long
          as size_t,
      );
      if dst_len < src_len {
        /* write compressed block data */
        crate::libbb::xfuncs_printf::xwrite(1i32, b2 as *const libc::c_void, dst_len as size_t);
      } else {
        /* write uncompressed block data */
        crate::libbb::xfuncs_printf::xwrite(1i32, b1 as *const libc::c_void, src_len as size_t);
      }
    }
  }
  free(wrk_mem as *mut libc::c_void);
  free(b1 as *mut libc::c_void);
  free(b2 as *mut libc::c_void);
  return 1i32;
}
unsafe extern "C" fn lzo_check(
  mut init: u32,
  mut buf: *mut u8,
  mut len: libc::c_uint,
  mut fn_0: Option<unsafe extern "C" fn(_: u32, _: *const u8, _: libc::c_uint) -> u32>,
  mut ref_0: u32,
) {
  /* This function, by having the same order of parameters
   * as fn, and by being marked FAST_FUNC (same as fn),
   * saves a dozen bytes of code.
   */
  let mut c: u32 = fn_0.expect("non-null function pointer")(init, buf, len);
  if c != ref_0 {
    crate::libbb::verror_msg::bb_simple_error_msg_and_die(
      b"checksum error\x00" as *const u8 as *const libc::c_char,
    );
  };
}
/* *********************************************************************/
// decompress a file
/* *********************************************************************/
// used to have "const header_t *h" parameter, but since it uses
// only flags32 field, changed to receive only that.
#[inline(never)]
unsafe extern "C" fn lzo_decompress(mut h_flags32: u32) -> libc::c_int {
  let mut block_size: libc::c_uint = (256i32 as libc::c_long * 1024i64) as libc::c_uint;
  let mut r: libc::c_int = 0;
  let mut src_len: u32 = 0;
  let mut dst_len: u32 = 0;
  let mut c_adler32: u32 = 1i32 as u32;
  let mut d_adler32: u32 = 1i32 as u32;
  let mut c_crc32: u32 = 0 as u32;
  let mut d_crc32: u32 = 0 as u32;
  let mut b1: *mut u8 = std::ptr::null_mut();
  let mut mcs_block_size: u32 = block_size
    .wrapping_add(block_size.wrapping_div(16i32 as libc::c_uint))
    .wrapping_add(64i32 as libc::c_uint)
    .wrapping_add(3i32 as libc::c_uint);
  let mut b2: *mut u8 = std::ptr::null_mut();
  loop {
    let mut dst: *mut u8 = std::ptr::null_mut();
    /* read uncompressed block size */
    dst_len = read32();
    /* exit if last block */
    if dst_len == 0 as libc::c_uint {
      break;
    }
    /* error if split file */
    if dst_len as libc::c_long == 0xffffffffi64 {
      /* should not happen - not yet implemented */
      crate::libbb::verror_msg::bb_simple_error_msg_and_die(
        b"this file is a split lzop file\x00" as *const u8 as *const libc::c_char,
      );
    }
    if dst_len as libc::c_long > 64i32 as libc::c_long * 1024i64 * 1024i64 {
      crate::libbb::verror_msg::bb_simple_error_msg_and_die(
        b"corrupted data\x00" as *const u8 as *const libc::c_char,
      );
    }
    /* read compressed block size */
    src_len = read32();
    if src_len <= 0 as libc::c_uint || src_len > dst_len {
      crate::libbb::verror_msg::bb_simple_error_msg_and_die(
        b"corrupted data\x00" as *const u8 as *const libc::c_char,
      );
    }
    if dst_len > block_size {
      if !b2.is_null() {
        free(b2 as *mut libc::c_void);
        b2 = std::ptr::null_mut()
      }
      block_size = dst_len;
      mcs_block_size = block_size
        .wrapping_add(block_size.wrapping_div(16i32 as libc::c_uint))
        .wrapping_add(64i32 as libc::c_uint)
        .wrapping_add(3i32 as libc::c_uint)
    }
    /* read checksum of uncompressed block */
    if h_flags32 as libc::c_long & 0x1i64 != 0 {
      d_adler32 = read32()
    }
    if h_flags32 as libc::c_long & 0x100i64 != 0 {
      d_crc32 = read32()
    }
    /* read checksum of compressed block */
    if src_len < dst_len {
      if h_flags32 as libc::c_long & 0x2i64 != 0 {
        c_adler32 = read32()
      }
      if h_flags32 as libc::c_long & 0x200i64 != 0 {
        c_crc32 = read32()
      }
    }
    if b2.is_null() {
      b2 = crate::libbb::xfuncs_printf::xzalloc(mcs_block_size as size_t) as *mut u8
    }
    /* read the block into the end of our buffer */
    b1 = b2
      .offset(mcs_block_size as isize)
      .offset(-(src_len as isize));
    crate::libbb::read_printf::xread(0i32, b1 as *mut libc::c_void, src_len as size_t);
    if src_len < dst_len {
      let mut d: libc::c_uint = dst_len;
      if option_mask32 & OPT_F as libc::c_int as libc::c_uint == 0 {
        /* verify checksum of compressed block */
        if h_flags32 as libc::c_long & 0x2i64 != 0 {
          lzo_check(
            1i32 as u32,
            b1,
            src_len,
            Some(lzo_adler32 as unsafe extern "C" fn(_: u32, _: *const u8, _: libc::c_uint) -> u32),
            c_adler32,
          );
        }
        if h_flags32 as libc::c_long & 0x200i64 != 0 {
          lzo_check(
            0 as u32,
            b1,
            src_len,
            Some(lzo_crc32 as unsafe extern "C" fn(_: u32, _: *const u8, _: libc::c_uint) -> u32),
            c_crc32,
          );
        }
      }
      /* decompress */
      //			if (option_mask32 & OPT_F)
      //				r = lzo1x_decompress(b1, src_len, b2, &d /*, NULL*/);
      //			else
      r = crate::archival::libarchive::lzo1x_d::lzo1x_decompress_safe(b1, src_len, b2, &mut d);
      if r != 0 || dst_len != d {
        crate::libbb::verror_msg::bb_simple_error_msg_and_die(
          b"corrupted data\x00" as *const u8 as *const libc::c_char,
        );
      }
      dst = b2
    } else {
      /* "stored" block => no decompression */
      dst = b1
    }
    if option_mask32 & OPT_F as libc::c_int as libc::c_uint == 0 {
      /* verify checksum of uncompressed block */
      if h_flags32 as libc::c_long & 0x1i64 != 0 {
        lzo_check(
          1i32 as u32,
          dst,
          dst_len,
          Some(lzo_adler32 as unsafe extern "C" fn(_: u32, _: *const u8, _: libc::c_uint) -> u32),
          d_adler32,
        );
      }
      if h_flags32 as libc::c_long & 0x100i64 != 0 {
        lzo_check(
          0 as u32,
          dst,
          dst_len,
          Some(lzo_crc32 as unsafe extern "C" fn(_: u32, _: *const u8, _: libc::c_uint) -> u32),
          d_crc32,
        );
      }
    }
    /* write uncompressed block data */
    crate::libbb::xfuncs_printf::xwrite(1i32, dst as *const libc::c_void, dst_len as size_t);
  }
  free(b2 as *mut libc::c_void);
  return 1i32;
}
/* *********************************************************************/
// lzop file signature (shamelessly borrowed from PNG)
/* *********************************************************************/
/*
 * The first nine bytes of a lzop file always contain the following values:
 *
 *                                 0   1   2   3   4   5   6   7   8
 *                               --- --- --- --- --- --- --- --- ---
 * (hex)                          89  4c  5a  4f  00  0d  0a  1a  0a
 * (decimal)                     137  76  90  79   0  13  10  26  10
 * (C notation - ASCII)         \211   L   Z   O  \0  \r  \n \032 \n
 */
/* (vda) comparison with lzop v1.02rc1 ("lzop -1 <FILE" cmd):
 * Only slight differences in header:
 * -00000000  89 4c 5a 4f 00 0d 0a 1a 0a 10 20 20 20 09 40 02
 * +00000000  89 4c 5a 4f 00 0d 0a 1a 0a 10 10 20 30 09 40 02
 *                                       ^^^^^ ^^^^^
 *                                     version lib_version
 * -00000010  01 03 00 00 0d 00 00 81 a4 49 f7 a6 3f 00 00 00
 * +00000010  01 03 00 00 01 00 00 00 00 00 00 00 00 00 00 00
 *               ^^^^^^^^^^^ ^^^^^^^^^^^ ^^^^^^^^^^^
 *               flags       mode        mtime
 * -00000020  00 00 2d 67 04 17 00 04 00 00 00 03 ed ec 9d 6d
 * +00000020  00 00 10 5f 00 c1 00 04 00 00 00 03 ed ec 9d 6d
 *                  ^^^^^^^^^^^
 *                  chksum
 * The rest is identical.
*/
static mut lzop_magic: [libc::c_uchar; 9] = [
  0x89i32 as libc::c_uchar,
  0x4ci32 as libc::c_uchar,
  0x5ai32 as libc::c_uchar,
  0x4fi32 as libc::c_uchar,
  0 as libc::c_uchar,
  0xdi32 as libc::c_uchar,
  0xai32 as libc::c_uchar,
  0x1ai32 as libc::c_uchar,
  0xai32 as libc::c_uchar,
];
/* This coding is derived from Alexander Lehmann's pngcheck code. */
unsafe extern "C" fn check_magic() {
  let mut magic: [libc::c_uchar; 9] = [0; 9];
  crate::libbb::read_printf::xread(
    0,
    magic.as_mut_ptr() as *mut libc::c_void,
    ::std::mem::size_of::<[libc::c_uchar; 9]>() as libc::c_ulong,
  );
  if memcmp(
    magic.as_mut_ptr() as *const libc::c_void,
    lzop_magic.as_ptr() as *const libc::c_void,
    ::std::mem::size_of::<[libc::c_uchar; 9]>() as libc::c_ulong,
  ) != 0
  {
    crate::libbb::verror_msg::bb_simple_error_msg_and_die(
      b"bad magic number\x00" as *const u8 as *const libc::c_char,
    );
  };
}
/* *********************************************************************/
// lzop file header
/* *********************************************************************/
unsafe extern "C" fn write_header(mut h: *mut header_t) {
  let mut end: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  crate::libbb::xfuncs_printf::xwrite(
    1i32,
    lzop_magic.as_ptr() as *const libc::c_void,
    ::std::mem::size_of::<[libc::c_uchar; 9]>() as libc::c_ulong,
  );
  init_chksum();
  /* Our caller leaves name zero-filled, so len == 0 */
  end = (*h).len_and_name.as_mut_ptr().offset(1).offset(0); /* 0 is strlen(h->len_and_name+1) */
  /* Store length byte */
  /*h->len_and_name[0] = end - (h->len_and_name+1); - zero already */
  f_write(
    &mut (*h).version_be16 as *mut u16 as *const libc::c_void,
    end.wrapping_offset_from(&mut (*h).version_be16 as *mut u16 as *mut libc::c_char)
      as libc::c_long as libc::c_int,
  ); /* native endianness for lzo_compress() */
  (*h).flags32 = {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = (*h).flags32;
    if false {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh98 = &mut __v;
      let fresh99;
      let fresh100 = __x;
      asm!("bswap $0" : "=r" (fresh99) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh98, fresh100)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh98, fresh100, fresh99);
    }
    __v
  };
  write32(chksum_getresult((*h).flags32));
}
unsafe extern "C" fn read_header(mut h: *mut header_t) -> libc::c_int {
  let mut l: libc::c_int = 0;
  let mut checksum: u32 = 0;
  /* As it stands now, only h->flags32 is used by our caller.
   * Therefore we don't store many fields in h->FIELD.
   */
  let mut h_version: libc::c_uint = 0;
  let mut h_version_needed_to_extract: libc::c_uint = 0;
  init_chksum();
  /* We don't support versions < 0.94, since 0.94
   * came only 2 months after 0.90:
   * 0.90 (10 Aug 1997): First public release of lzop
   * 0.94 (15 Oct 1997): Header format change
   */
  /* Read up to and including name length byte */
  f_read(
    &mut (*h).version_be16 as *mut u16 as *mut libc::c_void,
    (&mut *(*h).len_and_name.as_mut_ptr().offset(1) as *mut libc::c_char)
      .wrapping_offset_from(&mut (*h).version_be16 as *mut u16 as *mut libc::c_char)
      as libc::c_long as libc::c_int,
  );
  h_version = ({
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = (*h).version_be16;
    if false {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh101 = &mut __v;
      let fresh102;
      let fresh103 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh102) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh101, fresh103)) : "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh101, fresh103, fresh102);
    }
    __v
  }) as libc::c_uint;
  if h_version < 0x940i32 as libc::c_uint {
    return 3i32;
  }
  h_version_needed_to_extract = ({
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = (*h).version_needed_to_extract_be16;
    if false {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh104 = &mut __v;
      let fresh105;
      let fresh106 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh105) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh104, fresh106)) : "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh104, fresh106, fresh105);
    }
    __v
  }) as libc::c_uint;
  if h_version_needed_to_extract > 0x1010i32 as libc::c_uint {
    return 16i32;
  }
  if h_version_needed_to_extract < 0x940i32 as libc::c_uint {
    return 3i32;
  }
  if (*h).method as libc::c_int <= 0 {
    return 14i32;
  }
  /* former lzo_get_method(h): */
  if (*h).method as libc::c_int == M_LZO1X_1 as libc::c_int {
    if (*h).level as libc::c_int == 0 {
      (*h).level = 3i32 as u8
    }
  } else if (*h).method as libc::c_int == M_LZO1X_1_15 as libc::c_int {
    if (*h).level as libc::c_int == 0 {
      (*h).level = 1i32 as u8
    }
  } else if (*h).method as libc::c_int == M_LZO1X_999 as libc::c_int {
    if (*h).level as libc::c_int == 0 {
      (*h).level = 9i32 as u8
    }
  } else {
    return -1i32;
  } /* not a LZO method */
  /* check compression level */
  if ((*h).level as libc::c_int) < 1i32 || (*h).level as libc::c_int > 9i32 {
    return 15i32;
  } /* filter not supported */
  (*h).flags32 = {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = (*h).flags32;
    if false {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh107 = &mut __v;
      let fresh108;
      let fresh109 = __x;
      asm!("bswap $0" : "=r" (fresh108) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh107, fresh109)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh107, fresh109, fresh108);
    }
    __v
  };
  if (*h).flags32 as libc::c_long & 0x800i64 != 0 {
    return 16i32;
  }
  /* check reserved flags */
  if (*h).flags32 as libc::c_long & ((0x3fffi64 | 0xff000000i64 | 0xf00000i64) ^ 0xffffffffi64) != 0
  {
    return -13i32;
  }
  l = (*h).len_and_name[0] as libc::c_int;
  if l > 0 {
    /* UNUSED */
    f_read(
      (*h).len_and_name.as_mut_ptr().offset(1) as *mut libc::c_void,
      l,
    );
  }
  /* UNUSED h->len_and_name[1+l] = 0; */
  checksum = chksum_getresult((*h).flags32);
  if read32() != checksum {
    return 2i32;
  }
  /* skip extra field [not used yet] */
  if (*h).flags32 as libc::c_long & 0x40i64 != 0 {
    let mut extra_field_len: u32 = 0;
    let mut extra_field_checksum: u32 = 0;
    let mut k: u32 = 0;
    let mut dummy: libc::c_char = 0;
    /* note: the checksum also covers the length */
    init_chksum();
    extra_field_len = f_read32();
    k = 0 as u32;
    while k < extra_field_len {
      f_read(&mut dummy as *mut libc::c_char as *mut libc::c_void, 1i32);
      k = k.wrapping_add(1)
    }
    checksum = chksum_getresult((*h).flags32);
    extra_field_checksum = read32();
    if extra_field_checksum != checksum {
      return 3i32;
    }
  }
  return 0;
}
/* *********************************************************************/
// compress
/* *********************************************************************/
unsafe extern "C" fn lzo_set_method(mut h: *mut header_t) {
  let mut level: smallint = 0;
  /* levels 2..6 or none (defaults to level 3) */
  (*h).method = M_LZO1X_1 as libc::c_int as u8; /* levels 2-6 are actually the same */
  level = 5i32 as smallint;
  if option_mask32 & OPT_1 as libc::c_int as libc::c_uint != 0 {
    (*h).method = M_LZO1X_1_15 as libc::c_int as u8;
    level = 1i32 as smallint
  }
  if option_mask32 & OPT_789 as libc::c_int as libc::c_uint != 0 {
    crate::libbb::verror_msg::bb_simple_error_msg_and_die(
      b"high compression not compiled in\x00" as *const u8 as *const libc::c_char,
    );
  }
  (*h).level = level as u8;
}
unsafe extern "C" fn do_lzo_compress() -> libc::c_int {
  let mut header: header_t = header_t {
    version_be16: 0,
    lib_version_be16: 0,
    version_needed_to_extract_be16: 0,
    method: 0,
    level: 0,
    flags32: 0,
    mode_be32: 0,
    mtime_be32: 0,
    gmtdiff_be32: 0,
    len_and_name: [0; 257],
  };
  memset(
    &mut header as *mut header_t as *mut libc::c_void,
    0,
    ::std::mem::size_of::<header_t>() as libc::c_ulong,
  );
  lzo_set_method(&mut header);
  header.version_be16 = {
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = (0x1010i32 & 0xffffi32) as libc::c_ushort;
    if false {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh110 = &mut __v;
      let fresh111;
      let fresh112 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh111) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh110, fresh112)) : "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh110, fresh112, fresh111);
    }
    __v
  };
  header.version_needed_to_extract_be16 = {
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = 0x940i32 as libc::c_ushort;
    if false {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh113 = &mut __v;
      let fresh114;
      let fresh115 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh114) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh113, fresh115)) : "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh113, fresh115, fresh114);
    }
    __v
  };
  header.lib_version_be16 = {
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = (0x2030i32 & 0xffffi32) as libc::c_ushort;
    if false {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh116 = &mut __v;
      let fresh117;
      let fresh118 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh117) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh116, fresh118)) : "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh116, fresh118, fresh117);
    }
    __v
  };
  header.flags32 = {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = (0x3000000i64 & 0xff000000i64 | 0i64 & 0xf00000i64) as libc::c_uint;
    if false {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh119 = &mut __v;
      let fresh120;
      let fresh121 = __x;
      asm!("bswap $0" : "=r" (fresh120) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh119, fresh121)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh119, fresh121, fresh120);
    }
    __v
  };
  if option_mask32 & OPT_F as libc::c_int as libc::c_uint == 0
    || header.method as libc::c_int == M_LZO1X_999 as libc::c_int
  {
    header.flags32 |= {
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = 0x1i64 as libc::c_uint;
      if false {
        __v = (__x & 0xff000000u32) >> 24i32
          | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
          | (__x & 0xff00i32 as libc::c_uint) << 8i32
          | (__x & 0xffi32 as libc::c_uint) << 24i32
      } else {
        let fresh122 = &mut __v;
        let fresh123;
        let fresh124 = __x;
        asm!("bswap $0" : "=r" (fresh123) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh122, fresh124)) :);
        c2rust_asm_casts::AsmCast::cast_out(fresh122, fresh124, fresh123);
      }
      __v
    };
    if option_mask32 & OPT_C as libc::c_int as libc::c_uint != 0 {
      header.flags32 |= {
        let mut __v: libc::c_uint = 0;
        let mut __x: libc::c_uint = 0x2i64 as libc::c_uint;
        if false {
          __v = (__x & 0xff000000u32) >> 24i32
            | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
            | (__x & 0xff00i32 as libc::c_uint) << 8i32
            | (__x & 0xffi32 as libc::c_uint) << 24i32
        } else {
          let fresh125 = &mut __v;
          let fresh126;
          let fresh127 = __x;
          asm!("bswap $0" : "=r" (fresh126) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh125, fresh127)) :);
          c2rust_asm_casts::AsmCast::cast_out(fresh125, fresh127, fresh126);
        }
        __v
      }
    }
  }
  /* write_header() also converts h->flags32 to native endianness */
  write_header(&mut header);
  return lzo_compress(&mut header);
}
/* *********************************************************************/
// decompress
/* *********************************************************************/
unsafe extern "C" fn do_lzo_decompress() -> libc::c_int {
  let mut r: libc::c_int = 0;
  let mut header: header_t = header_t {
    version_be16: 0,
    lib_version_be16: 0,
    version_needed_to_extract_be16: 0,
    method: 0,
    level: 0,
    flags32: 0,
    mode_be32: 0,
    mtime_be32: 0,
    gmtdiff_be32: 0,
    len_and_name: [0; 257],
  };
  check_magic();
  r = read_header(&mut header);
  if r != 0 {
    crate::libbb::verror_msg::bb_error_msg_and_die(
      b"header_error %d\x00" as *const u8 as *const libc::c_char,
      r,
    );
  }
  return lzo_decompress(header.flags32);
}
unsafe extern "C" fn make_new_name_lzop(
  mut filename: *mut libc::c_char,
  mut _expected_ext: *const libc::c_char,
) -> *mut libc::c_char {
  if option_mask32 & OPT_DECOMPRESS as libc::c_int as libc::c_uint != 0 {
    let mut extension: *mut libc::c_char = strrchr(filename, '.' as i32);
    if extension.is_null()
      || strcmp(
        extension.offset(1),
        b"lzo\x00" as *const u8 as *const libc::c_char,
      ) != 0
    {
      return crate::libbb::xfuncs_printf::xasprintf(
        b"%s.out\x00" as *const u8 as *const libc::c_char,
        filename,
      );
    }
    *extension = '\u{0}' as i32 as libc::c_char;
    return filename;
  }
  return crate::libbb::xfuncs_printf::xasprintf(
    b"%s.lzo\x00" as *const u8 as *const libc::c_char,
    filename,
  );
}
unsafe extern "C" fn pack_lzop(mut _xstate: *mut transformer_state_t) -> libc::c_longlong {
  if option_mask32 & OPT_DECOMPRESS as libc::c_int as libc::c_uint != 0 {
    return do_lzo_decompress() as libc::c_longlong;
  }
  return do_lzo_compress() as libc::c_longlong;
}
pub unsafe fn lzop_main(mut _argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
  crate::libbb::getopt32::getopt32(
    argv,
    b"cfUvqdt123456789CFk\x00" as *const u8 as *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  /* -U is "anti -k", invert bit for bbunpack(): */
  option_mask32 ^= OPT_KEEP as libc::c_int as libc::c_uint;
  /* -k disables -U (if any): */
  /* opt_complementary "k-U"? - nope, only handles -Uk, not -kU */
  if option_mask32 & OPT_k as libc::c_int as libc::c_uint != 0 {
    option_mask32 |= OPT_KEEP as libc::c_int as libc::c_uint
  }
  /* lzopcat? */
  if false && *applet_name.offset(4) as libc::c_int == 'c' as i32 {
    option_mask32 |= (OPT_STDOUT as libc::c_int | OPT_DECOMPRESS as libc::c_int) as libc::c_uint
  }
  /* unlzop? */
  if false && *applet_name.offset(4) as libc::c_int == 'o' as i32 {
    option_mask32 |= OPT_DECOMPRESS as libc::c_int as libc::c_uint
  }
  crate::libbb::crc32::global_crc32_new_table_le();
  return crate::archival::bbunzip::bbunpack(
    argv,
    Some(pack_lzop as unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong),
    Some(
      make_new_name_lzop
        as unsafe extern "C" fn(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char,
    ),
    0 as *const libc::c_char,
  );
}

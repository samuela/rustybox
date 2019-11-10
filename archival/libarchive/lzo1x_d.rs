use libc;
use libc::chdir;
use libc::chmod;
use libc::chown;
use libc::closelog;
use libc::dup2;
use libc::fstat;
use libc::getenv;
use libc::geteuid;
use libc::getopt;
use libc::getpid;
use libc::isatty;
use libc::kill;
use libc::openlog;
use libc::sigaddset;
use libc::sigemptyset;
use libc::sigprocmask;
use libc::sleep;
use libc::sscanf;
use libc::strcasecmp;
use libc::strcpy;
use libc::symlink;
use libc::syscall;
use libc::syslog;
use libc::time;
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







/* implementation of the LZO1X decompression algorithm

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
// decompress a block of data.
************************************************************************/
/* safe decompression with overrun testing */
#[no_mangle]
pub unsafe extern "C" fn lzo1x_decompress_safe(
  mut in_0: *const u8,
  mut in_len: libc::c_uint,
  mut out: *mut u8,
  mut out_len: *mut libc::c_uint,
) -> libc::c_int
/*, void* wrkmem */ {
  let mut current_block: u64; /* possibly not needed */
  let mut op: *mut u8 = 0 as *mut u8;
  let mut ip: *const u8 = 0 as *const u8;
  let mut t: libc::c_uint = 0;
  let mut m_pos: *const u8 = 0 as *const u8;
  let ip_end: *const u8 = in_0.offset(in_len as isize);
  let op_end: *mut u8 = out.offset(*out_len as isize);
  //	LZO_UNUSED(wrkmem);
  /* COPY_DICT */
  *out_len = 0i32 as libc::c_uint;
  op = out;
  ip = in_0;
  if *ip as libc::c_int > 17i32 {
    let fresh0 = ip;
    ip = ip.offset(1);
    t = (*fresh0 as libc::c_int - 17i32) as libc::c_uint;
    if t < 4i32 as libc::c_uint {
      current_block = 2652804691515851435;
    } else if (op_end.wrapping_offset_from(op) as libc::c_long as libc::c_uint) < t {
      current_block = 13165906339582574725;
    } else if (ip_end.wrapping_offset_from(ip) as libc::c_long as libc::c_uint)
      < t.wrapping_add(1i32 as libc::c_uint)
    {
      current_block = 2691384274927532760;
    } else {
      loop {
        let fresh1 = ip;
        ip = ip.offset(1);
        let fresh2 = op;
        op = op.offset(1);
        *fresh2 = *fresh1;
        t = t.wrapping_sub(1);
        if !(t > 0i32 as libc::c_uint) {
          break;
        }
      }
      current_block = 16536959610807766281;
    }
  } else {
    current_block = 7976072742316086414;
  }
  's_77: loop {
    match current_block {
      13165906339582574725 =>
      //#endif
      //#if defined(HAVE_NEED_OP)
      {
        *out_len = op.wrapping_offset_from(out) as libc::c_long as libc::c_uint;
        return -5i32;
      }
      2652804691515851435 =>
      /* copy literals */
      {
        if (op_end.wrapping_offset_from(op) as libc::c_long as libc::c_uint) < t {
          current_block = 13165906339582574725;
          continue;
        }
        if (ip_end.wrapping_offset_from(ip) as libc::c_long as libc::c_uint)
          < t.wrapping_add(1i32 as libc::c_uint)
        {
          current_block = 2691384274927532760;
          continue;
        }
        let fresh33 = ip;
        ip = ip.offset(1);
        let fresh34 = op;
        op = op.offset(1);
        *fresh34 = *fresh33;
        if t > 1i32 as libc::c_uint {
          let fresh35 = ip;
          ip = ip.offset(1);
          let fresh36 = op;
          op = op.offset(1);
          *fresh36 = *fresh35;
          if t > 2i32 as libc::c_uint {
            let fresh37 = ip;
            ip = ip.offset(1);
            let fresh38 = op;
            op = op.offset(1);
            *fresh38 = *fresh37
          }
        }
        let fresh39 = ip;
        ip = ip.offset(1);
        t = *fresh39 as libc::c_uint;
        if !(ip < ip_end && 1i32 != 0) {
          current_block = 7976072742316086414;
          continue;
        }
        current_block = 14594744208347999533;
      }
      16536959610807766281 => {
        let fresh13 = ip;
        ip = ip.offset(1);
        t = *fresh13 as libc::c_uint;
        if t >= 16i32 as libc::c_uint {
          current_block = 14594744208347999533;
        } else {
          /* !COPY_DICT */
          m_pos = op.offset(-((1i32 + 0x800i32) as isize));
          m_pos = m_pos.offset(-((t >> 2i32) as isize));
          let fresh14 = ip;
          ip = ip.offset(1);
          m_pos = m_pos.offset(-(((*fresh14 as libc::c_int) << 2i32) as isize));
          if m_pos < out || m_pos >= op {
            break;
          }
          if (op_end.wrapping_offset_from(op) as libc::c_long as libc::c_uint)
            < 3i32 as libc::c_uint
          {
            current_block = 13165906339582574725;
            continue;
          }
          let fresh15 = m_pos;
          m_pos = m_pos.offset(1);
          let fresh16 = op;
          op = op.offset(1);
          *fresh16 = *fresh15;
          let fresh17 = m_pos;
          m_pos = m_pos.offset(1);
          let fresh18 = op;
          op = op.offset(1);
          *fresh18 = *fresh17;
          let fresh19 = op;
          op = op.offset(1);
          *fresh19 = *m_pos;
          /* COPY_DICT */
          current_block = 6194671515245402134;
        }
      }
      2691384274927532760 =>
      //#if defined(HAVE_NEED_IP)
      {
        *out_len = op.wrapping_offset_from(out) as libc::c_long as libc::c_uint;
        return -4i32;
      }
      _ => {
        if ip < ip_end && 1i32 != 0 {
          let fresh3 = ip;
          ip = ip.offset(1);
          t = *fresh3 as libc::c_uint;
          if !(t >= 16i32 as libc::c_uint) {
            /* a literal run */
            if t == 0i32 as libc::c_uint {
              if (ip_end.wrapping_offset_from(ip) as libc::c_long as libc::c_uint)
                < 1i32 as libc::c_uint
              {
                current_block = 2691384274927532760;
                continue;
              }
              while *ip as libc::c_int == 0i32 {
                t = t.wrapping_add(255i32 as libc::c_uint);
                ip = ip.offset(1);
                if (ip_end.wrapping_offset_from(ip) as libc::c_long as libc::c_uint)
                  < 1i32 as libc::c_uint
                {
                  current_block = 2691384274927532760;
                  continue 's_77;
                }
              }
              if t > (0i32 as libc::c_uint).wrapping_sub(511i32 as libc::c_uint) {
                current_block = 2691384274927532760;
                continue;
              }
              let fresh4 = ip;
              ip = ip.offset(1);
              t = t.wrapping_add((15i32 + *fresh4 as libc::c_int) as libc::c_uint)
            }
            /* copy literals */
            if (op_end.wrapping_offset_from(op) as libc::c_long as libc::c_uint)
              < t.wrapping_add(3i32 as libc::c_uint)
            {
              current_block = 13165906339582574725;
              continue;
            }
            if (ip_end.wrapping_offset_from(ip) as libc::c_long as libc::c_uint)
              < t.wrapping_add(4i32 as libc::c_uint)
            {
              current_block = 2691384274927532760;
              continue;
            }
            let fresh5 = ip;
            ip = ip.offset(1);
            let fresh6 = op;
            op = op.offset(1);
            *fresh6 = *fresh5;
            let fresh7 = ip;
            ip = ip.offset(1);
            let fresh8 = op;
            op = op.offset(1);
            *fresh8 = *fresh7;
            let fresh9 = ip;
            ip = ip.offset(1);
            let fresh10 = op;
            op = op.offset(1);
            *fresh10 = *fresh9;
            loop {
              let fresh11 = ip;
              ip = ip.offset(1);
              let fresh12 = op;
              op = op.offset(1);
              *fresh12 = *fresh11;
              t = t.wrapping_sub(1);
              if !(t > 0i32 as libc::c_uint) {
                break;
              }
            }
            current_block = 16536959610807766281;
            continue;
          }
        } else {
          //#if defined(HAVE_TEST_IP) || defined(HAVE_TEST_OP)
          /* no EOF code was found */
          *out_len = op.wrapping_offset_from(out) as libc::c_long as libc::c_uint;
          return -7i32;
        }
        current_block = 14594744208347999533;
      }
    }
    match current_block {
      14594744208347999533 =>
      /* handle matches */
      {
        if t >= 64i32 as libc::c_uint {
          /* a M2 match */
          /* !COPY_DICT */
          m_pos = op.offset(-1);
          m_pos = m_pos.offset(-((t >> 2i32 & 7i32 as libc::c_uint) as isize));
          let fresh20 = ip;
          ip = ip.offset(1);
          m_pos = m_pos.offset(-(((*fresh20 as libc::c_int) << 3i32) as isize));
          t = (t >> 5i32).wrapping_sub(1i32 as libc::c_uint);
          if m_pos < out || m_pos >= op {
            break;
          }
          if (op_end.wrapping_offset_from(op) as libc::c_long as libc::c_uint)
            < t
              .wrapping_add(3i32 as libc::c_uint)
              .wrapping_sub(1i32 as libc::c_uint)
          {
            current_block = 13165906339582574725;
            continue;
          }
          current_block = 7715594215898856897;
        /* COPY_DICT */
        } else {
          if t >= 32i32 as libc::c_uint {
            /* a M3 match */
            t &= 31i32 as libc::c_uint;
            if t == 0i32 as libc::c_uint {
              if (ip_end.wrapping_offset_from(ip) as libc::c_long as libc::c_uint)
                < 1i32 as libc::c_uint
              {
                current_block = 2691384274927532760;
                continue;
              }
              while *ip as libc::c_int == 0i32 {
                t = t.wrapping_add(255i32 as libc::c_uint);
                ip = ip.offset(1);
                if (ip_end.wrapping_offset_from(ip) as libc::c_long as libc::c_uint)
                  < 1i32 as libc::c_uint
                {
                  current_block = 2691384274927532760;
                  continue 's_77;
                }
              }
              if t > (0i32 as libc::c_uint).wrapping_sub(511i32 as libc::c_uint) {
                current_block = 2691384274927532760;
                continue;
              }
              let fresh21 = ip;
              ip = ip.offset(1);
              t = t.wrapping_add((31i32 + *fresh21 as libc::c_int) as libc::c_uint)
            }
            /* !COPY_DICT */
            m_pos = op.offset(-1);
            m_pos = m_pos.offset(
              -(((*ip.offset(0) as libc::c_int >> 2i32) + ((*ip.offset(1) as libc::c_int) << 6i32))
                as isize),
            );
            /* COPY_DICT */
            ip = ip.offset(2);
            current_block = 16313536926714486912;
          } else if t >= 16i32 as libc::c_uint {
            /* a M4 match */
            /* !COPY_DICT */
            m_pos = op;
            m_pos = m_pos.offset(-(((t & 8i32 as libc::c_uint) << 11i32) as isize));
            /* COPY_DICT */
            /* COPY_DICT */
            t &= 7i32 as libc::c_uint;
            if t == 0i32 as libc::c_uint {
              if (ip_end.wrapping_offset_from(ip) as libc::c_long as libc::c_uint)
                < 1i32 as libc::c_uint
              {
                current_block = 2691384274927532760;
                continue;
              }
              while *ip as libc::c_int == 0i32 {
                t = t.wrapping_add(255i32 as libc::c_uint);
                ip = ip.offset(1);
                if (ip_end.wrapping_offset_from(ip) as libc::c_long as libc::c_uint)
                  < 1i32 as libc::c_uint
                {
                  current_block = 2691384274927532760;
                  continue 's_77;
                }
              }
              if t > (0i32 as libc::c_uint).wrapping_sub(511i32 as libc::c_uint) {
                current_block = 2691384274927532760;
                continue;
              }
              let fresh22 = ip;
              ip = ip.offset(1);
              t = t.wrapping_add((7i32 + *fresh22 as libc::c_int) as libc::c_uint)
            }
            /* !COPY_DICT */
            m_pos = m_pos.offset(
              -(((*ip.offset(0) as libc::c_int >> 2i32) + ((*ip.offset(1) as libc::c_int) << 6i32))
                as isize),
            );
            ip = ip.offset(2);
            if m_pos == op {
              //#endif
              *out_len = op.wrapping_offset_from(out) as libc::c_long as libc::c_uint;
              return if ip == ip_end {
                0i32
              } else if ip < ip_end {
                -8i32
              } else {
                -4i32
              };
            } else {
              m_pos = m_pos.offset(-(0x4000i32 as isize))
            }
            current_block = 16313536926714486912;
          } else {
            /* a M1 match */
            /* !COPY_DICT */
            m_pos = op.offset(-1);
            m_pos = m_pos.offset(-((t >> 2i32) as isize));
            let fresh23 = ip;
            ip = ip.offset(1);
            m_pos = m_pos.offset(-(((*fresh23 as libc::c_int) << 2i32) as isize));
            if m_pos < out || m_pos >= op {
              break;
            }
            if (op_end.wrapping_offset_from(op) as libc::c_long as libc::c_uint)
              < 2i32 as libc::c_uint
            {
              current_block = 13165906339582574725;
              continue;
            }
            let fresh24 = m_pos;
            m_pos = m_pos.offset(1);
            let fresh25 = op;
            op = op.offset(1);
            *fresh25 = *fresh24;
            let fresh26 = op;
            op = op.offset(1);
            *fresh26 = *m_pos;
            /* COPY_DICT */
            current_block = 6194671515245402134;
          }
          match current_block {
            6194671515245402134 => {}
            _ =>
            /* copy match */
            /* !COPY_DICT */
            {
              if m_pos < out || m_pos >= op {
                break;
              }
              if (op_end.wrapping_offset_from(op) as libc::c_long as libc::c_uint)
                < t
                  .wrapping_add(3i32 as libc::c_uint)
                  .wrapping_sub(1i32 as libc::c_uint)
              {
                current_block = 13165906339582574725;
                continue;
              }
              current_block = 7715594215898856897;
            }
          }
        }
        match current_block {
          6194671515245402134 => {}
          _ => {
            let fresh27 = m_pos;
            m_pos = m_pos.offset(1);
            let fresh28 = op;
            op = op.offset(1);
            *fresh28 = *fresh27;
            let fresh29 = m_pos;
            m_pos = m_pos.offset(1);
            let fresh30 = op;
            op = op.offset(1);
            *fresh30 = *fresh29;
            loop {
              let fresh31 = m_pos;
              m_pos = m_pos.offset(1);
              let fresh32 = op;
              op = op.offset(1);
              *fresh32 = *fresh31;
              t = t.wrapping_sub(1);
              if !(t > 0i32 as libc::c_uint) {
                break;
              }
            }
          }
        }
      }
      _ => {}
    }
    /* COPY_DICT */
    t = (*ip.offset(-2i32 as isize) as libc::c_int & 3i32) as libc::c_uint;
    if t == 0i32 as libc::c_uint {
      current_block = 7976072742316086414;
    } else {
      current_block = 2652804691515851435;
    }
  }
  //#endif
  //#if defined(LZO_TEST_OVERRUN_LOOKBEHIND)
  *out_len = op.wrapping_offset_from(out) as libc::c_long as libc::c_uint;
  return -6i32;
  //#endif
}

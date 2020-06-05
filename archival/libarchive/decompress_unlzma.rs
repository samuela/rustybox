use crate::archival::libarchive::bb_archive::transformer_state_t;
use crate::libbb::xfuncs_printf::xmalloc;
use crate::librb::size_t;
use libc;
use libc::free;
use libc::ssize_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rc_t {
  pub fd: libc::c_int,
  pub ptr: *mut u8,
  pub buffer_end: *mut u8,
  pub code: u32,
  pub range: u32,
  pub bound: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct lzma_header_t {
  pub pos: u8,
  pub dict_size: u32,
  pub dst_size: u64,
}
pub const LZMA_MATCH_MIN_LEN: C2RustUnnamed_0 = 2;
pub const LZMA_ALIGN: C2RustUnnamed_0 = 802;
pub const LZMA_NUM_ALIGN_BITS: C2RustUnnamed_0 = 4;
pub const LZMA_SPEC_POS: C2RustUnnamed_0 = 688;
pub const LZMA_END_POS_MODEL_INDEX: C2RustUnnamed_0 = 14;
pub const LZMA_START_POS_MODEL_INDEX: C2RustUnnamed_0 = 4;
pub const LZMA_NUM_POS_SLOT_BITS: C2RustUnnamed_0 = 6;
pub const LZMA_NUM_LEN_TO_POS_STATES: C2RustUnnamed_0 = 4;
pub const LZMA_POS_SLOT: C2RustUnnamed_0 = 432;
pub const LZMA_NUM_LIT_STATES: C2RustUnnamed_0 = 7;
pub const LZMA_LEN_NUM_LOW_BITS: C2RustUnnamed_0 = 3;
pub const LZMA_LEN_NUM_HIGH_BITS: C2RustUnnamed_0 = 8;
pub const LZMA_LEN_NUM_MID_BITS: C2RustUnnamed_0 = 3;
pub const LZMA_LEN_CHOICE_2: C2RustUnnamed_0 = 1;
pub const LZMA_LEN_HIGH: C2RustUnnamed_0 = 258;
pub const LZMA_LEN_MID: C2RustUnnamed_0 = 130;
pub const LZMA_LEN_CHOICE: C2RustUnnamed_0 = 0;
pub const LZMA_LEN_LOW: C2RustUnnamed_0 = 2;
pub const LZMA_REP_LEN_CODER: C2RustUnnamed_0 = 1332;
pub const LZMA_IS_REP_G1: C2RustUnnamed_0 = 216;
pub const LZMA_IS_REP_G2: C2RustUnnamed_0 = 228;
pub const LZMA_IS_REP_G0: C2RustUnnamed_0 = 204;
pub const LZMA_NUM_POS_BITS_MAX: C2RustUnnamed_0 = 4;
pub const LZMA_IS_REP_0_LONG: C2RustUnnamed_0 = 240;
pub const LZMA_IS_REP: C2RustUnnamed_0 = 192;
pub const LZMA_LEN_CODER: C2RustUnnamed_0 = 818;
pub const LZMA_IS_MATCH: C2RustUnnamed_0 = 0;
pub const LZMA_LIT_SIZE: C2RustUnnamed_0 = 768;
pub const LZMA_LITERAL: C2RustUnnamed_0 = 1846;
pub const LZMA_BASE_SIZE: C2RustUnnamed_0 = 1846;
/* #defines will force compiler to compute/optimize each one with each usage.
 * Have heart and use enum instead. */
pub type C2RustUnnamed_0 = libc::c_uint;
pub const LZMA_NUM_FULL_DISTANCES: C2RustUnnamed_0 = 128;
pub const LZMA_NUM_STATES: C2RustUnnamed_0 = 12;
pub const LZMA_NUM_LEN_PROBS: C2RustUnnamed_0 = 514;
/* Called once in rc_do_normalize() */
unsafe fn rc_read(mut rc: *mut rc_t) {
  let mut buffer_size: libc::c_int = crate::libbb::read::safe_read(
    (*rc).fd,
    rc.offset(1) as *mut u8 as *mut libc::c_void,
    0x10000i32 as size_t,
  ) as libc::c_int;
  //TODO: return -1 instead
  //This will make unlzma delete broken unpacked file on unpack errors
  if buffer_size <= 0 {
    crate::libbb::verror_msg::bb_simple_error_msg_and_die(
      b"unexpected EOF\x00" as *const u8 as *const libc::c_char,
    );
  }
  (*rc).buffer_end = (rc.offset(1) as *mut u8).offset(buffer_size as isize);
  (*rc).ptr = rc.offset(1) as *mut u8;
}
/* Called twice, but one callsite is in speed_inline'd rc_is_bit_1() */
unsafe fn rc_do_normalize(mut rc: *mut rc_t) {
  if (*rc).ptr >= (*rc).buffer_end {
    rc_read(rc);
  }
  (*rc).range <<= 8i32;
  let fresh0 = (*rc).ptr;
  (*rc).ptr = (*rc).ptr.offset(1);
  (*rc).code = (*rc).code << 8i32 | *fresh0 as libc::c_uint;
}
#[inline(always)]
unsafe fn rc_normalize(mut rc: *mut rc_t) {
  if (*rc).range < (1i32 << 24i32) as libc::c_uint {
    rc_do_normalize(rc);
  };
}
/* Called once */
#[inline(always)]
unsafe fn rc_init(mut fd: libc::c_int) -> *mut rc_t
/*, int buffer_size) */ {
  let mut i: libc::c_int = 0;
  let mut rc: *mut rc_t = std::ptr::null_mut();
  rc = crate::libbb::xfuncs_printf::xzalloc(
    (::std::mem::size_of::<rc_t>() as libc::c_ulong).wrapping_add(0x10000i32 as libc::c_ulong),
  ) as *mut rc_t;
  (*rc).fd = fd;
  /* rc->ptr = rc->buffer_end; */
  i = 0;
  while i < 5i32 {
    rc_do_normalize(rc);
    i += 1
  }
  (*rc).range = 0xffffffffu32;
  return rc;
}
/* Called once  */
#[inline(always)]
unsafe fn rc_free(mut rc: *mut rc_t) {
  free(rc as *mut libc::c_void);
}
/* rc_is_bit_1 is called 9 times */
unsafe fn rc_is_bit_1(mut rc: *mut rc_t, mut p: *mut u16) -> libc::c_int {
  rc_normalize(rc);
  (*rc).bound = (*p as libc::c_uint).wrapping_mul((*rc).range >> 11i32);
  if (*rc).code < (*rc).bound {
    (*rc).range = (*rc).bound;
    *p = (*p as libc::c_int + ((1i32 << 11i32) - *p as libc::c_int >> 5i32)) as u16;
    return 0;
  }
  (*rc).range = ((*rc).range as libc::c_uint).wrapping_sub((*rc).bound) as u32 as u32;
  (*rc).code = ((*rc).code as libc::c_uint).wrapping_sub((*rc).bound) as u32 as u32;
  *p = (*p as libc::c_int - (*p as libc::c_int >> 5i32)) as u16;
  return 1i32;
}
/* Called 4 times in unlzma loop */
#[inline(always)]
unsafe fn rc_get_bit(
  mut rc: *mut rc_t,
  mut p: *mut u16,
  mut symbol: *mut libc::c_int,
) -> libc::c_int {
  let mut ret: libc::c_int = rc_is_bit_1(rc, p);
  *symbol = *symbol * 2i32 + ret;
  return ret;
}
/* Called once */
#[inline(always)]
unsafe fn rc_direct_bit(mut rc: *mut rc_t) -> libc::c_int {
  rc_normalize(rc);
  (*rc).range >>= 1i32;
  if (*rc).code >= (*rc).range {
    (*rc).code = ((*rc).code as libc::c_uint).wrapping_sub((*rc).range) as u32 as u32;
    return 1i32;
  }
  return 0;
}
/* Called twice */
unsafe fn rc_bit_tree_decode(
  mut rc: *mut rc_t,
  mut p: *mut u16,
  mut num_levels: libc::c_int,
  mut symbol: *mut libc::c_int,
) {
  let mut i: libc::c_int = num_levels;
  *symbol = 1i32;
  loop {
    let fresh1 = i;
    i = i - 1;
    if !(fresh1 != 0) {
      break;
    }
    rc_get_bit(rc, p.offset(*symbol as isize), symbol);
  }
  *symbol -= 1i32 << num_levels;
}
pub unsafe fn unpack_lzma_stream(mut xstate: *mut transformer_state_t) -> libc::c_longlong {
  let mut num_bits: libc::c_int = 0;
  let mut offset: libc::c_int = 0;
  let mut prob2: *mut u16 = std::ptr::null_mut();
  let mut pos_0: u32 = 0;
  let mut current_block: u64;
  let mut total_written: libc::c_longlong = 0 as libc::c_longlong;
  let mut header: lzma_header_t = lzma_header_t {
    pos: 0,
    dict_size: 0,
    dst_size: 0,
  };
  let mut lc: libc::c_int = 0;
  let mut pb: libc::c_int = 0;
  let mut lp: libc::c_int = 0;
  let mut pos_state_mask: u32 = 0;
  let mut literal_pos_mask: u32 = 0;
  let mut p: *mut u16 = std::ptr::null_mut();
  let mut rc: *mut rc_t = std::ptr::null_mut();
  let mut i: libc::c_int = 0;
  let mut buffer: *mut u8 = std::ptr::null_mut();
  let mut buffer_size: u32 = 0;
  let mut previous_byte: u8 = 0 as u8;
  let mut buffer_pos: size_t = 0 as size_t;
  let mut global_pos: size_t = 0 as size_t;
  let mut len: libc::c_int = 0;
  let mut state: libc::c_int = 0;
  let mut rep0: u32 = 1i32 as u32;
  let mut rep1: u32 = 1i32 as u32;
  let mut rep2: u32 = 1i32 as u32;
  let mut rep3: u32 = 1i32 as u32;
  if crate::libbb::read::full_read(
    (*xstate).src_fd,
    &mut header as *mut lzma_header_t as *mut libc::c_void,
    ::std::mem::size_of::<lzma_header_t>() as libc::c_ulong,
  ) as libc::c_ulong
    != ::std::mem::size_of::<lzma_header_t>() as libc::c_ulong
    || header.pos as libc::c_int >= 9i32 * 5i32 * 5i32
  {
    crate::libbb::verror_msg::bb_simple_error_msg(
      b"bad lzma header\x00" as *const u8 as *const libc::c_char,
    );
    return -1i32 as libc::c_longlong;
  }
  i = header.pos as libc::c_int / 9i32;
  lc = header.pos as libc::c_int % 9i32;
  pb = i / 5i32;
  lp = i % 5i32;
  pos_state_mask = ((1i32 << pb) - 1i32) as u32;
  literal_pos_mask = ((1i32 << lp) - 1i32) as u32;
  /* Example values from linux-3.3.4.tar.lzma:
   * dict_size: 64M, dst_size: 2^64-1
   */
  header.dict_size = header.dict_size; /*, RC_BUFFER_SIZE); */
  header.dst_size = header.dst_size; /* 0x100 or 0 */
  if header.dict_size == 0 as libc::c_uint {
    header.dict_size = header.dict_size.wrapping_add(1)
  }
  buffer_size = if header.dst_size < header.dict_size as u64 {
    header.dst_size
  } else {
    header.dict_size as u64
  } as u32;
  buffer = xmalloc(buffer_size as size_t) as *mut u8;
  let mut num_probs: libc::c_int = 0;
  num_probs = LZMA_BASE_SIZE as libc::c_int + ((LZMA_LIT_SIZE as libc::c_int) << lc + lp);
  p = xmalloc(
    (num_probs as libc::c_ulong).wrapping_mul(::std::mem::size_of::<u16>() as libc::c_ulong),
  ) as *mut u16;
  num_probs += LZMA_LITERAL as libc::c_int - LZMA_BASE_SIZE as libc::c_int;
  i = 0;
  while i < num_probs {
    *p.offset(i as isize) = (1i32 << 11i32 >> 1i32) as u16;
    i += 1
  }
  rc = rc_init((*xstate).src_fd);
  's_151: loop {
    if !((global_pos.wrapping_add(buffer_pos) as u64) < header.dst_size) {
      current_block = 2884634553824165030;
      break;
    }
    let mut pos_state: libc::c_int =
      (buffer_pos.wrapping_add(global_pos) & pos_state_mask as libc::c_ulong) as libc::c_int;
    let mut prob: *mut u16 = p
      .offset(LZMA_IS_MATCH as libc::c_int as isize)
      .offset((state << LZMA_NUM_POS_BITS_MAX as libc::c_int) as isize)
      .offset(pos_state as isize);
    if rc_is_bit_1(rc, prob) == 0 {
      static mut next_state: [libc::c_char; 12] = [
        0 as libc::c_char,
        0 as libc::c_char,
        0 as libc::c_char,
        0 as libc::c_char,
        1i32 as libc::c_char,
        2i32 as libc::c_char,
        3i32 as libc::c_char,
        4i32 as libc::c_char,
        5i32 as libc::c_char,
        6i32 as libc::c_char,
        4i32 as libc::c_char,
        5i32 as libc::c_char,
      ];
      let mut mi: libc::c_int = 1i32;
      prob = p.offset(LZMA_LITERAL as libc::c_int as isize).offset(
        (LZMA_LIT_SIZE as libc::c_int as libc::c_ulong).wrapping_mul(
          ((buffer_pos.wrapping_add(global_pos) & literal_pos_mask as libc::c_ulong) << lc)
            .wrapping_add((previous_byte as libc::c_int >> 8i32 - lc) as libc::c_ulong),
        ) as isize,
      );
      if state >= LZMA_NUM_LIT_STATES as libc::c_int {
        let mut match_byte: libc::c_int = 0;
        let mut pos: u32 = 0;
        pos = buffer_pos.wrapping_sub(rep0 as libc::c_ulong) as u32;
        if (pos as i32) < 0 {
          pos = (pos as libc::c_uint).wrapping_add(header.dict_size) as u32 as u32
        }
        match_byte = *buffer.offset(pos as isize) as libc::c_int;
        loop {
          let mut bit: libc::c_int = 0;
          match_byte <<= 1i32;
          bit = match_byte & 0x100i32;
          bit ^= rc_get_bit(
            rc,
            prob
              .offset(0x100i32 as isize)
              .offset(bit as isize)
              .offset(mi as isize),
            &mut mi,
          ) << 8i32;
          if bit != 0 {
            break;
          }
          if !(mi < 0x100i32) {
            break;
          }
        }
      }
      while mi < 0x100i32 {
        rc_get_bit(rc, prob.offset(mi as isize), &mut mi);
      }
      state = next_state[state as usize] as libc::c_int;
      previous_byte = mi as u8;
      len = 1i32;
      current_block = 8572389853916933330;
    } else {
      num_bits = 0;
      offset = 0;
      prob2 = std::ptr::null_mut();
      prob2 = p
        .offset(LZMA_IS_REP as libc::c_int as isize)
        .offset(state as isize);
      if rc_is_bit_1(rc, prob2) == 0 {
        rep3 = rep2;
        rep2 = rep1;
        rep1 = rep0;
        state = if state < LZMA_NUM_LIT_STATES as libc::c_int {
          0
        } else {
          3i32
        };
        prob2 = p.offset(LZMA_LEN_CODER as libc::c_int as isize);
        current_block = 9879896046554623444;
      } else {
        prob2 = prob2.offset((LZMA_IS_REP_G0 as libc::c_int - LZMA_IS_REP as libc::c_int) as isize);
        if rc_is_bit_1(rc, prob2) == 0 {
          prob2 = p
            .offset(LZMA_IS_REP_0_LONG as libc::c_int as isize)
            .offset((state << LZMA_NUM_POS_BITS_MAX as libc::c_int) as isize)
            .offset(pos_state as isize);
          if rc_is_bit_1(rc, prob2) == 0 {
            state = if state < LZMA_NUM_LIT_STATES as libc::c_int {
              9i32
            } else {
              11i32
            };
            len = 1i32;
            current_block = 11702799181856929651;
          } else {
            current_block = 576355610076403033;
          }
        } else {
          let mut distance: u32 = 0;
          prob2 =
            prob2.offset((LZMA_IS_REP_G1 as libc::c_int - LZMA_IS_REP_G0 as libc::c_int) as isize);
          distance = rep1;
          if rc_is_bit_1(rc, prob2) != 0 {
            prob2 = prob2
              .offset((LZMA_IS_REP_G2 as libc::c_int - LZMA_IS_REP_G1 as libc::c_int) as isize);
            distance = rep2;
            if rc_is_bit_1(rc, prob2) != 0 {
              distance = rep3;
              rep3 = rep2
            }
            rep2 = rep1
          }
          rep1 = rep0;
          rep0 = distance;
          current_block = 576355610076403033;
        }
        match current_block {
          11702799181856929651 => {}
          _ => {
            state = if state < LZMA_NUM_LIT_STATES as libc::c_int {
              8i32
            } else {
              11i32
            };
            prob2 = p.offset(LZMA_REP_LEN_CODER as libc::c_int as isize);
            current_block = 9879896046554623444;
          }
        }
      }
      match current_block {
        11702799181856929651 => {}
        _ => {
          prob2 = prob2.offset(LZMA_LEN_CHOICE as libc::c_int as isize);
          num_bits = LZMA_LEN_NUM_LOW_BITS as libc::c_int;
          if rc_is_bit_1(rc, prob2) == 0 {
            prob2 = prob2.offset(
              (LZMA_LEN_LOW as libc::c_int - LZMA_LEN_CHOICE as libc::c_int
                + (pos_state << LZMA_LEN_NUM_LOW_BITS as libc::c_int)) as isize,
            );
            offset = 0
          } else {
            prob2 = prob2
              .offset((LZMA_LEN_CHOICE_2 as libc::c_int - LZMA_LEN_CHOICE as libc::c_int) as isize);
            if rc_is_bit_1(rc, prob2) == 0 {
              prob2 = prob2.offset(
                (LZMA_LEN_MID as libc::c_int - LZMA_LEN_CHOICE_2 as libc::c_int
                  + (pos_state << LZMA_LEN_NUM_MID_BITS as libc::c_int)) as isize,
              );
              offset = 1i32 << LZMA_LEN_NUM_LOW_BITS as libc::c_int;
              num_bits +=
                LZMA_LEN_NUM_MID_BITS as libc::c_int - LZMA_LEN_NUM_LOW_BITS as libc::c_int
            } else {
              prob2 = prob2
                .offset((LZMA_LEN_HIGH as libc::c_int - LZMA_LEN_CHOICE_2 as libc::c_int) as isize);
              offset = (1i32 << LZMA_LEN_NUM_LOW_BITS as libc::c_int)
                + (1i32 << LZMA_LEN_NUM_MID_BITS as libc::c_int);
              num_bits +=
                LZMA_LEN_NUM_HIGH_BITS as libc::c_int - LZMA_LEN_NUM_LOW_BITS as libc::c_int
            }
          }
          rc_bit_tree_decode(rc, prob2, num_bits, &mut len);
          len += offset;
          if state < 4i32 {
            let mut pos_slot: libc::c_int = 0;
            let mut prob3: *mut u16 = std::ptr::null_mut();
            state += LZMA_NUM_LIT_STATES as libc::c_int;
            prob3 = p.offset(LZMA_POS_SLOT as libc::c_int as isize).offset(
              ((if len < LZMA_NUM_LEN_TO_POS_STATES as libc::c_int {
                len
              } else {
                (LZMA_NUM_LEN_TO_POS_STATES as libc::c_int) - 1i32
              }) << LZMA_NUM_POS_SLOT_BITS as libc::c_int) as isize,
            );
            rc_bit_tree_decode(
              rc,
              prob3,
              LZMA_NUM_POS_SLOT_BITS as libc::c_int,
              &mut pos_slot,
            );
            rep0 = pos_slot as u32;
            if pos_slot >= LZMA_START_POS_MODEL_INDEX as libc::c_int {
              let mut i2: libc::c_int = 0;
              let mut mi2: libc::c_int = 0;
              let mut num_bits2: libc::c_int = (pos_slot >> 1i32) - 1i32;
              rep0 = (2i32 | pos_slot & 1i32) as u32;
              if pos_slot < LZMA_END_POS_MODEL_INDEX as libc::c_int {
                rep0 <<= num_bits2;
                prob3 = p
                  .offset(LZMA_SPEC_POS as libc::c_int as isize)
                  .offset(rep0 as isize)
                  .offset(-(pos_slot as isize))
                  .offset(-1)
              } else {
                while num_bits2 != LZMA_NUM_ALIGN_BITS as libc::c_int {
                  rep0 = rep0 << 1i32 | rc_direct_bit(rc) as libc::c_uint;
                  num_bits2 -= 1
                }
                rep0 <<= LZMA_NUM_ALIGN_BITS as libc::c_int;
                /* FIXME: ...........^^^^^
                 * shouldn't it be "global_pos + buffer_pos < header.dst_size"?
                 * It probably should, but it is a "do we accidentally
                 * unpack more bytes than expected?" check - which
                 * never happens for well-formed compression data...
                 */
                // Note: (i32)rep0 may be < 0 here
                // (I have linux-3.3.4.tar.lzma which has it).
                // I moved the check after "++rep0 == 0" check below.
                prob3 = p.offset(LZMA_ALIGN as libc::c_int as isize)
              }
              i2 = 1i32;
              mi2 = 1i32;
              loop {
                let fresh2 = num_bits2;
                num_bits2 = num_bits2 - 1;
                if !(fresh2 != 0) {
                  break;
                }
                if rc_get_bit(rc, prob3.offset(mi2 as isize), &mut mi2) != 0 {
                  rep0 |= i2 as libc::c_uint
                }
                i2 <<= 1i32
              }
            }
            rep0 = rep0.wrapping_add(1);
            if rep0 as i32 <= 0 {
              if rep0 == 0 as libc::c_uint {
                current_block = 2884634553824165030;
                break;
              } else {
                current_block = 7192488959635554372;
                break;
              }
            }
          }
          len += LZMA_MATCH_MIN_LEN as libc::c_int;
          current_block = 11702799181856929651;
        }
      }
    }
    loop {
      match current_block {
        8572389853916933330 => {
          let fresh3 = buffer_pos;
          buffer_pos = buffer_pos.wrapping_add(1);
          *buffer.offset(fresh3 as isize) = previous_byte;
          if buffer_pos == header.dict_size as libc::c_ulong {
            buffer_pos = 0 as size_t;
            global_pos = (global_pos as libc::c_ulong)
              .wrapping_add(header.dict_size as libc::c_ulong) as size_t
              as size_t;
            if crate::archival::libarchive::open_transformer::transformer_write(
              xstate,
              buffer as *const libc::c_void,
              header.dict_size as size_t,
            ) != header.dict_size as ssize_t
            {
              current_block = 7192488959635554372;
              break 's_151;
            }
            total_written += header.dict_size as libc::c_longlong
          }
          len -= 1;
          if len != 0 && (buffer_pos as u64) < header.dst_size {
            current_block = 11702799181856929651;
          } else {
            break;
          }
        }
        _ =>
        /*
         * LZMA SDK has this optimized:
         * it precalculates size and copies many bytes
         * in a loop with simpler checks, a-la:
         *	do
         *		*(dest) = *(dest + ofs);
         *	while (++dest != lim);
         * and
         *	do {
         *		buffer[buffer_pos++] = buffer[pos];
         *		if (++pos == header.dict_size)
         *			pos = 0;
         *	} while (--cur_len != 0);
         * Our code is slower (more checks per byte copy):
         */
        {
          pos_0 = buffer_pos.wrapping_sub(rep0 as libc::c_ulong) as u32;
          if (pos_0 as i32) < 0 {
            pos_0 = (pos_0 as libc::c_uint).wrapping_add(header.dict_size) as u32 as u32;
            /* bug 10436 has an example file where this triggers: */
            //if ((i32)pos < 0)
            //	goto bad;
            /* more stringent test (see unzip_bad_lzma_1.zip): */
            if pos_0 >= buffer_size {
              current_block = 7192488959635554372;
              break 's_151;
            }
          }
          previous_byte = *buffer.offset(pos_0 as isize);
          current_block = 8572389853916933330;
        }
      }
    }
  }
  match current_block {
    2884634553824165030 => {
      /* success */
      total_written = (total_written as libc::c_ulonglong)
        .wrapping_add(buffer_pos as libc::c_ulonglong) as libc::c_longlong
        as libc::c_longlong;
      if crate::archival::libarchive::open_transformer::transformer_write(
        xstate,
        buffer as *const libc::c_void,
        buffer_pos,
      ) != buffer_pos as ssize_t
      {
        current_block = 7192488959635554372;
      } else {
        current_block = 14358540534591340610;
      }
    }
    _ => {}
  }
  match current_block {
    7192488959635554372 => {
      /* One of our users, bbunpack(), expects _us_ to emit
       * the error message (since it's the best place to give
       * potentially more detailed information).
       * Do not fail silently.
       */
      crate::libbb::verror_msg::bb_simple_error_msg(
        b"corrupted data\x00" as *const u8 as *const libc::c_char,
      );
      total_written = -1i32 as libc::c_longlong
      /* failure */
    }
    _ => {}
  }
  rc_free(rc);
  free(p as *mut libc::c_void);
  free(buffer as *mut libc::c_void);
  return total_written;
}

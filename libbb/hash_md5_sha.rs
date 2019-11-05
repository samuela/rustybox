use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
extern "C" {
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
}

use crate::librb::__uint16_t;

use crate::librb::__uint64_t;
use crate::librb::uint8_t;
use crate::librb::uint16_t;
use crate::librb::uint32_t;
use crate::librb::uint64_t;
pub type bb__aliased_uint64_t = uint64_t;
use crate::librb::size_t;
use crate::librb::md5_ctx_t;
use crate::librb::sha1_ctx_t;
use crate::librb::sha256_ctx_t;
use crate::librb::sha512_ctx_t;
use crate::librb::sha3_ctx_t;
/* Constants for SHA512 from FIPS 180-2:4.2.3.
 * SHA256 constants from FIPS 180-2:4.2.2
 * are the most significant half of first 64 elements
 * of the same array.
 */
pub type sha_K_int = uint64_t;
pub const NROUNDS: C2RustUnnamed = 24;
pub type C2RustUnnamed = libc::c_uint;
/* gcc 4.2.1 optimizes rotr64 better with inline than with macro
 * (for rotX32, there is no difference). Why? My guess is that
 * macro requires clever common subexpression elimination heuristics
 * in gcc, while inline basically forces it to happen.
 */
//#define rotl32(x,n) (((x) << (n)) | ((x) >> (32 - (n))))
#[inline(always)]
unsafe extern "C" fn rotl32(mut x: uint32_t, mut n: libc::c_uint) -> uint32_t {
  return x << n | x >> (32i32 as libc::c_uint).wrapping_sub(n);
}
//#define rotr32(x,n) (((x) >> (n)) | ((x) << (32 - (n))))
#[inline(always)]
unsafe extern "C" fn rotr32(mut x: uint32_t, mut n: libc::c_uint) -> uint32_t {
  return x >> n | x << (32i32 as libc::c_uint).wrapping_sub(n);
}
/* rotr64 in needed for sha512 only: */
//#define rotr64(x,n) (((x) >> (n)) | ((x) << (64 - (n))))
#[inline(always)]
unsafe extern "C" fn rotr64(mut x: uint64_t, mut n: libc::c_uint) -> uint64_t {
  return x >> n | x << (64i32 as libc::c_uint).wrapping_sub(n);
}
/* rotl64 only used for sha3 currently */
#[inline(always)]
unsafe extern "C" fn rotl64(mut x: uint64_t, mut n: libc::c_uint) -> uint64_t {
  return x << n | x >> (64i32 as libc::c_uint).wrapping_sub(n);
}
/* Feed data through a temporary buffer.
 * The internal buffer remembers previous data until it has 64
 * bytes worth to pass on.
 */
unsafe extern "C" fn common64_hash(
  mut ctx: *mut md5_ctx_t,
  mut buffer: *const libc::c_void,
  mut len: size_t,
) {
  let mut bufpos: libc::c_uint = ((*ctx).total64 & 63i32 as libc::c_ulong) as libc::c_uint;
  (*ctx).total64 = ((*ctx).total64 as libc::c_ulong).wrapping_add(len) as uint64_t as uint64_t;
  loop {
    let mut remaining: libc::c_uint = (64i32 as libc::c_uint).wrapping_sub(bufpos);
    if remaining as libc::c_ulong > len {
      remaining = len as libc::c_uint
    }
    /*bufpos = 0; - already is */
    /* Copy data into aligned buffer */
    memcpy(
      (*ctx).wbuffer.as_mut_ptr().offset(bufpos as isize) as *mut libc::c_void,
      buffer,
      remaining as libc::c_ulong,
    );
    len = (len as libc::c_ulong).wrapping_sub(remaining as libc::c_ulong) as size_t as size_t;
    buffer = (buffer as *const libc::c_char).offset(remaining as isize) as *const libc::c_void;
    bufpos = bufpos.wrapping_add(remaining);
    /* Clever way to do "if (bufpos != N) break; ... ; bufpos = 0;" */
    bufpos = bufpos.wrapping_sub(64i32 as libc::c_uint);
    if bufpos != 0i32 as libc::c_uint {
      break;
    }
    (*ctx).process_block.expect("non-null function pointer")(ctx);
  }
}
/* Buffer is filled up, process it */
/* Process the remaining bytes in the buffer */
unsafe extern "C" fn common64_end(mut ctx: *mut md5_ctx_t, mut swap_needed: libc::c_int) {
  let mut bufpos: libc::c_uint = ((*ctx).total64 & 63i32 as libc::c_ulong) as libc::c_uint;
  /* Pad the buffer to the next 64-byte boundary with 0x80,0,0,0... */
  let fresh0 = bufpos;
  bufpos = bufpos.wrapping_add(1);
  (*ctx).wbuffer[fresh0 as usize] = 0x80i32 as uint8_t;
  loop
  /* This loop iterates either once or twice, no more, no less */
  {
    let mut remaining: libc::c_uint = (64i32 as libc::c_uint).wrapping_sub(bufpos);
    memset(
      (*ctx).wbuffer.as_mut_ptr().offset(bufpos as isize) as *mut libc::c_void,
      0i32,
      remaining as libc::c_ulong,
    );
    /* Do we have enough space for the length count? */
    if remaining >= 8i32 as libc::c_uint {
      /* Store the 64-bit counter of bits in the buffer */
      let mut t: uint64_t = (*ctx).total64 << 3i32;
      if swap_needed != 0 {
        t = {
          let mut __v: __uint64_t = 0;
          let mut __x: __uint64_t = t;
          if 0 != 0 {
            __v = ((__x as libc::c_ulonglong & 0xff00000000000000u64) >> 56i32
              | (__x as libc::c_ulonglong & 0xff000000000000u64) >> 40i32
              | (__x as libc::c_ulonglong & 0xff0000000000u64) >> 24i32
              | (__x as libc::c_ulonglong & 0xff00000000u64) >> 8i32
              | (__x as libc::c_ulonglong & 0xff000000u64) << 8i32
              | (__x as libc::c_ulonglong & 0xff0000u64) << 24i32
              | (__x as libc::c_ulonglong & 0xff00u64) << 40i32
              | (__x as libc::c_ulonglong & 0xffu64) << 56i32) as __uint64_t
          } else {
            let fresh1 = &mut __v;
            let fresh2;
            let fresh3 = __x;
            asm!("bswap ${0:q}" : "=r" (fresh2) : "0"
                                  (c2rust_asm_casts::AsmCast::cast_in(fresh1, fresh3))
                                  :);
            c2rust_asm_casts::AsmCast::cast_out(fresh1, fresh3, fresh2);
          }
          __v
        }
      }
      /* wbuffer is suitably aligned for this */
      *(&mut *(*ctx).wbuffer.as_mut_ptr().offset((64i32 - 8i32) as isize) as *mut uint8_t
        as *mut bb__aliased_uint64_t) = t
    }
    (*ctx).process_block.expect("non-null function pointer")(ctx);
    if remaining >= 8i32 as libc::c_uint {
      break;
    }
    bufpos = 0i32 as libc::c_uint
  }
}
/*
 * Compute MD5 checksum of strings according to the
 * definition of MD5 in RFC 1321 from April 1992.
 *
 * Written by Ulrich Drepper <drepper@gnu.ai.mit.edu>, 1995.
 *
 * Copyright (C) 1995-1999 Free Software Foundation, Inc.
 * Copyright (C) 2001 Manuel Novoa III
 * Copyright (C) 2003 Glenn L. McGrath
 * Copyright (C) 2003 Erik Andersen
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* 0: fastest, 3: smallest */
/* These are the four functions used in the four steps of the MD5 algorithm
 * and defined in the RFC 1321.  The first function is a little bit optimized
 * (as found in Colin Plumbs public domain implementation).
 * #define FF(b, c, d) ((b & c) | (~b & d))
 */
/* Hash a single block, 64 bytes long and 4-byte aligned */
unsafe extern "C" fn md5_process_block64(mut ctx: *mut md5_ctx_t) {
  /* Before we start, one word to the strange constants.
    They are defined in RFC 1321 as
    T[i] = (int)(2^32 * fabs(sin(i))), i=1..64
  */
  static mut C_array: [uint32_t; 64] = [
    0xd76aa478u32,
    0xe8c7b756u32,
    0x242070dbi32 as uint32_t,
    0xc1bdceeeu32,
    0xf57c0fafu32,
    0x4787c62ai32 as uint32_t,
    0xa8304613u32,
    0xfd469501u32,
    0x698098d8i32 as uint32_t,
    0x8b44f7afu32,
    0xffff5bb1u32,
    0x895cd7beu32,
    0x6b901122i32 as uint32_t,
    0xfd987193u32,
    0xa679438eu32,
    0x49b40821i32 as uint32_t,
    0xf61e2562u32,
    0xc040b340u32,
    0x265e5a51i32 as uint32_t,
    0xe9b6c7aau32,
    0xd62f105du32,
    0x2441453i32 as uint32_t,
    0xd8a1e681u32,
    0xe7d3fbc8u32,
    0x21e1cde6i32 as uint32_t,
    0xc33707d6u32,
    0xf4d50d87u32,
    0x455a14edi32 as uint32_t,
    0xa9e3e905u32,
    0xfcefa3f8u32,
    0x676f02d9i32 as uint32_t,
    0x8d2a4c8au32,
    0xfffa3942u32,
    0x8771f681u32,
    0x6d9d6122i32 as uint32_t,
    0xfde5380cu32,
    0xa4beea44u32,
    0x4bdecfa9i32 as uint32_t,
    0xf6bb4b60u32,
    0xbebfbc70u32,
    0x289b7ec6i32 as uint32_t,
    0xeaa127fau32,
    0xd4ef3085u32,
    0x4881d05i32 as uint32_t,
    0xd9d4d039u32,
    0xe6db99e5u32,
    0x1fa27cf8i32 as uint32_t,
    0xc4ac5665u32,
    0xf4292244u32,
    0x432aff97i32 as uint32_t,
    0xab9423a7u32,
    0xfc93a039u32,
    0x655b59c3i32 as uint32_t,
    0x8f0ccc92u32,
    0xffeff47du32,
    0x85845dd1u32,
    0x6fa87e4fi32 as uint32_t,
    0xfe2ce6e0u32,
    0xa3014314u32,
    0x4e0811a1i32 as uint32_t,
    0xf7537e82u32,
    0xbd3af235u32,
    0x2ad7d2bbi32 as uint32_t,
    0xeb86d391u32,
  ];
  static mut P_array: [libc::c_char; 48] = [
    1i32 as libc::c_char,
    6i32 as libc::c_char,
    11i32 as libc::c_char,
    0i32 as libc::c_char,
    5i32 as libc::c_char,
    10i32 as libc::c_char,
    15i32 as libc::c_char,
    4i32 as libc::c_char,
    9i32 as libc::c_char,
    14i32 as libc::c_char,
    3i32 as libc::c_char,
    8i32 as libc::c_char,
    13i32 as libc::c_char,
    2i32 as libc::c_char,
    7i32 as libc::c_char,
    12i32 as libc::c_char,
    5i32 as libc::c_char,
    8i32 as libc::c_char,
    11i32 as libc::c_char,
    14i32 as libc::c_char,
    1i32 as libc::c_char,
    4i32 as libc::c_char,
    7i32 as libc::c_char,
    10i32 as libc::c_char,
    13i32 as libc::c_char,
    0i32 as libc::c_char,
    3i32 as libc::c_char,
    6i32 as libc::c_char,
    9i32 as libc::c_char,
    12i32 as libc::c_char,
    15i32 as libc::c_char,
    2i32 as libc::c_char,
    0i32 as libc::c_char,
    7i32 as libc::c_char,
    14i32 as libc::c_char,
    5i32 as libc::c_char,
    12i32 as libc::c_char,
    3i32 as libc::c_char,
    10i32 as libc::c_char,
    1i32 as libc::c_char,
    8i32 as libc::c_char,
    15i32 as libc::c_char,
    6i32 as libc::c_char,
    13i32 as libc::c_char,
    4i32 as libc::c_char,
    11i32 as libc::c_char,
    2i32 as libc::c_char,
    9i32 as libc::c_char,
  ];
  let mut words: *mut uint32_t = (*ctx).wbuffer.as_mut_ptr() as *mut libc::c_void as *mut uint32_t;
  let mut A: uint32_t = (*ctx).hash[0];
  let mut B: uint32_t = (*ctx).hash[1];
  let mut C: uint32_t = (*ctx).hash[2];
  let mut D: uint32_t = (*ctx).hash[3];
  /* 2 or 3 */
  /* MD5_SMALL == 0 or 1 */
  let mut pc: *const uint32_t = 0 as *const uint32_t;
  let mut pp: *const libc::c_char = 0 as *const libc::c_char;
  let mut i: libc::c_int = 0;
  /* First round: using the given function, the context and a constant
  the next context is computed.  Because the algorithm's processing
  unit is a 32-bit word and it is determined to work on words in
  little endian byte order we perhaps have to change the byte order
  before the computation.  To reduce the work for the next steps
  we save swapped words in WORDS array.  */
  /* Round 1 */
  pc = C_array.as_ptr();
  i = 0i32;
  while i < 4i32 {
    let fresh4 = pc;
    pc = pc.offset(1);
    A = (A as libc::c_uint)
      .wrapping_add((D ^ B & (C ^ D)).wrapping_add(*words).wrapping_add(*fresh4))
      as uint32_t as uint32_t;
    words = words.offset(1);
    A = rotl32(A, 7i32 as libc::c_uint);
    A = (A as libc::c_uint).wrapping_add(B) as uint32_t as uint32_t;
    let fresh5 = pc;
    pc = pc.offset(1);
    D = (D as libc::c_uint)
      .wrapping_add((C ^ A & (B ^ C)).wrapping_add(*words).wrapping_add(*fresh5))
      as uint32_t as uint32_t;
    words = words.offset(1);
    D = rotl32(D, 12i32 as libc::c_uint);
    D = (D as libc::c_uint).wrapping_add(A) as uint32_t as uint32_t;
    let fresh6 = pc;
    pc = pc.offset(1);
    C = (C as libc::c_uint)
      .wrapping_add((B ^ D & (A ^ B)).wrapping_add(*words).wrapping_add(*fresh6))
      as uint32_t as uint32_t;
    words = words.offset(1);
    C = rotl32(C, 17i32 as libc::c_uint);
    C = (C as libc::c_uint).wrapping_add(D) as uint32_t as uint32_t;
    let fresh7 = pc;
    pc = pc.offset(1);
    B = (B as libc::c_uint)
      .wrapping_add((A ^ C & (D ^ A)).wrapping_add(*words).wrapping_add(*fresh7))
      as uint32_t as uint32_t;
    words = words.offset(1);
    B = rotl32(B, 22i32 as libc::c_uint);
    B = (B as libc::c_uint).wrapping_add(C) as uint32_t as uint32_t;
    i += 1
  }
  words = words.offset(-16);
  /* For the second to fourth round we have the possibly swapped words
  in WORDS.  Redefine the macro to take an additional first
  argument specifying the function to use.  */
  /* Round 2 */
  pp = P_array.as_ptr();
  i = 0i32;
  while i < 4i32 {
    let fresh8 = pp;
    pp = pp.offset(1);
    let fresh9 = pc;
    pc = pc.offset(1);
    A = (A as libc::c_uint).wrapping_add(
      (C ^ D & (B ^ C))
        .wrapping_add(*words.offset(*fresh8 as libc::c_int as isize))
        .wrapping_add(*fresh9),
    ) as uint32_t as uint32_t;
    A = rotl32(A, 5i32 as libc::c_uint);
    A = (A as libc::c_uint).wrapping_add(B) as uint32_t as uint32_t;
    let fresh10 = pp;
    pp = pp.offset(1);
    let fresh11 = pc;
    pc = pc.offset(1);
    D = (D as libc::c_uint).wrapping_add(
      (B ^ C & (A ^ B))
        .wrapping_add(*words.offset(*fresh10 as libc::c_int as isize))
        .wrapping_add(*fresh11),
    ) as uint32_t as uint32_t;
    D = rotl32(D, 9i32 as libc::c_uint);
    D = (D as libc::c_uint).wrapping_add(A) as uint32_t as uint32_t;
    let fresh12 = pp;
    pp = pp.offset(1);
    let fresh13 = pc;
    pc = pc.offset(1);
    C = (C as libc::c_uint).wrapping_add(
      (A ^ B & (D ^ A))
        .wrapping_add(*words.offset(*fresh12 as libc::c_int as isize))
        .wrapping_add(*fresh13),
    ) as uint32_t as uint32_t;
    C = rotl32(C, 14i32 as libc::c_uint);
    C = (C as libc::c_uint).wrapping_add(D) as uint32_t as uint32_t;
    let fresh14 = pp;
    pp = pp.offset(1);
    let fresh15 = pc;
    pc = pc.offset(1);
    B = (B as libc::c_uint).wrapping_add(
      (D ^ A & (C ^ D))
        .wrapping_add(*words.offset(*fresh14 as libc::c_int as isize))
        .wrapping_add(*fresh15),
    ) as uint32_t as uint32_t;
    B = rotl32(B, 20i32 as libc::c_uint);
    B = (B as libc::c_uint).wrapping_add(C) as uint32_t as uint32_t;
    i += 1
  }
  /* Round 3 */
  i = 0i32;
  while i < 4i32 {
    let fresh16 = pp;
    pp = pp.offset(1);
    let fresh17 = pc;
    pc = pc.offset(1);
    A = (A as libc::c_uint).wrapping_add(
      (B ^ C ^ D)
        .wrapping_add(*words.offset(*fresh16 as libc::c_int as isize))
        .wrapping_add(*fresh17),
    ) as uint32_t as uint32_t;
    A = rotl32(A, 4i32 as libc::c_uint);
    A = (A as libc::c_uint).wrapping_add(B) as uint32_t as uint32_t;
    let fresh18 = pp;
    pp = pp.offset(1);
    let fresh19 = pc;
    pc = pc.offset(1);
    D = (D as libc::c_uint).wrapping_add(
      (A ^ B ^ C)
        .wrapping_add(*words.offset(*fresh18 as libc::c_int as isize))
        .wrapping_add(*fresh19),
    ) as uint32_t as uint32_t;
    D = rotl32(D, 11i32 as libc::c_uint);
    D = (D as libc::c_uint).wrapping_add(A) as uint32_t as uint32_t;
    let fresh20 = pp;
    pp = pp.offset(1);
    let fresh21 = pc;
    pc = pc.offset(1);
    C = (C as libc::c_uint).wrapping_add(
      (D ^ A ^ B)
        .wrapping_add(*words.offset(*fresh20 as libc::c_int as isize))
        .wrapping_add(*fresh21),
    ) as uint32_t as uint32_t;
    C = rotl32(C, 16i32 as libc::c_uint);
    C = (C as libc::c_uint).wrapping_add(D) as uint32_t as uint32_t;
    let fresh22 = pp;
    pp = pp.offset(1);
    let fresh23 = pc;
    pc = pc.offset(1);
    B = (B as libc::c_uint).wrapping_add(
      (C ^ D ^ A)
        .wrapping_add(*words.offset(*fresh22 as libc::c_int as isize))
        .wrapping_add(*fresh23),
    ) as uint32_t as uint32_t;
    B = rotl32(B, 23i32 as libc::c_uint);
    B = (B as libc::c_uint).wrapping_add(C) as uint32_t as uint32_t;
    i += 1
  }
  /* Round 4 */
  i = 0i32;
  while i < 4i32 {
    let fresh24 = pp;
    pp = pp.offset(1);
    let fresh25 = pc;
    pc = pc.offset(1);
    A = (A as libc::c_uint).wrapping_add(
      (C ^ (B | !D))
        .wrapping_add(*words.offset(*fresh24 as libc::c_int as isize))
        .wrapping_add(*fresh25),
    ) as uint32_t as uint32_t;
    A = rotl32(A, 6i32 as libc::c_uint);
    A = (A as libc::c_uint).wrapping_add(B) as uint32_t as uint32_t;
    let fresh26 = pp;
    pp = pp.offset(1);
    let fresh27 = pc;
    pc = pc.offset(1);
    D = (D as libc::c_uint).wrapping_add(
      (B ^ (A | !C))
        .wrapping_add(*words.offset(*fresh26 as libc::c_int as isize))
        .wrapping_add(*fresh27),
    ) as uint32_t as uint32_t;
    D = rotl32(D, 10i32 as libc::c_uint);
    D = (D as libc::c_uint).wrapping_add(A) as uint32_t as uint32_t;
    let fresh28 = pp;
    pp = pp.offset(1);
    let fresh29 = pc;
    pc = pc.offset(1);
    C = (C as libc::c_uint).wrapping_add(
      (A ^ (D | !B))
        .wrapping_add(*words.offset(*fresh28 as libc::c_int as isize))
        .wrapping_add(*fresh29),
    ) as uint32_t as uint32_t;
    C = rotl32(C, 15i32 as libc::c_uint);
    C = (C as libc::c_uint).wrapping_add(D) as uint32_t as uint32_t;
    let fresh30 = pp;
    pp = pp.offset(1);
    let fresh31 = pc;
    pc = pc.offset(1);
    B = (B as libc::c_uint).wrapping_add(
      (D ^ (C | !A))
        .wrapping_add(*words.offset(*fresh30 as libc::c_int as isize))
        .wrapping_add(*fresh31),
    ) as uint32_t as uint32_t;
    B = rotl32(B, 21i32 as libc::c_uint);
    B = (B as libc::c_uint).wrapping_add(C) as uint32_t as uint32_t;
    i += 1
  }
  /* Add checksum to the starting values */
  (*ctx).hash[0] = ((*ctx).hash[0] as libc::c_uint).wrapping_add(A) as uint32_t as uint32_t;
  (*ctx).hash[1] = ((*ctx).hash[1] as libc::c_uint).wrapping_add(B) as uint32_t as uint32_t;
  (*ctx).hash[2] = ((*ctx).hash[2] as libc::c_uint).wrapping_add(C) as uint32_t as uint32_t;
  (*ctx).hash[3] = ((*ctx).hash[3] as libc::c_uint).wrapping_add(D) as uint32_t as uint32_t;
}
/* Initialize structure containing state of computation.
 * (RFC 1321, 3.3: Step 3)
 */
#[no_mangle]
pub unsafe extern "C" fn md5_begin(mut ctx: *mut md5_ctx_t) {
  (*ctx).hash[0] = 0x67452301i32 as uint32_t;
  (*ctx).hash[1] = 0xefcdab89u32;
  (*ctx).hash[2] = 0x98badcfeu32;
  (*ctx).hash[3] = 0x10325476i32 as uint32_t;
  (*ctx).total64 = 0i32 as uint64_t;
  (*ctx).process_block = Some(md5_process_block64 as unsafe extern "C" fn(_: *mut md5_ctx_t) -> ());
}
/* Used also for sha1 and sha256 */
#[no_mangle]
pub unsafe extern "C" fn md5_hash(
  mut ctx: *mut md5_ctx_t,
  mut buffer: *const libc::c_void,
  mut len: size_t,
) {
  common64_hash(ctx, buffer, len);
}
/* Process the remaining bytes in the buffer and put result from CTX
 * in first 16 bytes following RESBUF.  The result is always in little
 * endian byte order, so that a byte-wise output yields to the wanted
 * ASCII representation of the message digest.
 */
#[no_mangle]
pub unsafe extern "C" fn md5_end(
  mut ctx: *mut md5_ctx_t,
  mut resbuf: *mut libc::c_void,
) -> libc::c_uint {
  /* MD5 stores total in LE, need to swap on BE arches: */
  common64_end(ctx, 0i32);
  /* The MD5 result is in little endian byte order */
  memcpy(
    resbuf,
    (*ctx).hash.as_mut_ptr() as *const libc::c_void,
    (::std::mem::size_of::<uint32_t>() as libc::c_ulong).wrapping_mul(4i32 as libc::c_ulong),
  );
  return (::std::mem::size_of::<uint32_t>() as libc::c_ulong).wrapping_mul(4i32 as libc::c_ulong)
    as libc::c_uint;
}
/*
 * SHA1 part is:
 * Copyright 2007 Rob Landley <rob@landley.net>
 *
 * Based on the public domain SHA-1 in C by Steve Reid <steve@edmweb.com>
 * from http://www.mirrors.wiretapped.net/security/cryptography/hashes/sha1/
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 *
 * ---------------------------------------------------------------------------
 *
 * SHA256 and SHA512 parts are:
 * Released into the Public Domain by Ulrich Drepper <drepper@redhat.com>.
 * Shrank by Denys Vlasenko.
 *
 * ---------------------------------------------------------------------------
 *
 * The best way to test random blocksizes is to go to coreutils/md5_sha1_sum.c
 * and replace "4096" with something like "2000 + time(NULL) % 2097",
 * then rebuild and compare "shaNNNsum bigfile" results.
 */
unsafe extern "C" fn sha1_process_block64(mut ctx: *mut sha1_ctx_t) {
  static mut rconsts: [uint32_t; 4] = [
    0x5a827999i32 as uint32_t,
    0x6ed9eba1i32 as uint32_t,
    0x8f1bbcdcu32,
    0xca62c1d6u32,
  ];
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  let mut cnt: libc::c_int = 0;
  let mut W: [uint32_t; 32] = [0; 32];
  let mut a: uint32_t = 0;
  let mut b: uint32_t = 0;
  let mut c: uint32_t = 0;
  let mut d: uint32_t = 0;
  let mut e: uint32_t = 0;
  /* On-stack work buffer frees up one register in the main loop
   * which otherwise will be needed to hold ctx pointer */
  i = 0i32;
  while i < 16i32 {
    W[(i + 16i32) as usize] = {
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint =
        *((*ctx).wbuffer.as_mut_ptr() as *mut uint32_t).offset(i as isize);
      if 0 != 0 {
        __v = (__x & 0xff000000u32) >> 24i32
          | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
          | (__x & 0xff00i32 as libc::c_uint) << 8i32
          | (__x & 0xffi32 as libc::c_uint) << 24i32
      } else {
        let fresh32 = &mut __v;
        let fresh33;
        let fresh34 = __x;
        asm!("bswap $0" : "=r" (fresh33) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh32, fresh34))
                          :);
        c2rust_asm_casts::AsmCast::cast_out(fresh32, fresh34, fresh33);
      }
      __v
    };
    W[i as usize] = W[(i + 16i32) as usize];
    i += 1
  }
  a = (*ctx).hash[0];
  b = (*ctx).hash[1];
  c = (*ctx).hash[2];
  d = (*ctx).hash[3];
  e = (*ctx).hash[4];
  /* 4 rounds of 20 operations each */
  cnt = 0i32;
  i = 0i32;
  while i < 4i32 {
    j = 19i32;
    loop {
      let mut work: uint32_t = 0;
      work = c ^ d;
      let mut current_block_18: u64;
      if i == 0i32 {
        work = work & b ^ d;
        if j <= 3i32 {
          current_block_18 = 12098554138476111035;
        } else {
          /* Used to do SWAP_BE32 here, but this
           * requires ctx (see comment above) */
          work = (work as libc::c_uint).wrapping_add(W[cnt as usize]) as uint32_t as uint32_t;
          current_block_18 = 16203760046146113240;
        }
      } else {
        if i == 2i32 {
          work = (b | c) & d | b & c
        } else {
          /* i = 1 or 3 */
          work ^= b
        }
        current_block_18 = 12098554138476111035;
      }
      match current_block_18 {
        12098554138476111035 => {
          W[(cnt + 16i32) as usize] = rotl32(
            W[(cnt + 13i32) as usize]
              ^ W[(cnt + 8i32) as usize]
              ^ W[(cnt + 2i32) as usize]
              ^ W[cnt as usize],
            1i32 as libc::c_uint,
          );
          W[cnt as usize] = W[(cnt + 16i32) as usize];
          work = (work as libc::c_uint).wrapping_add(W[cnt as usize]) as uint32_t as uint32_t
        }
        _ => {}
      }
      work = (work as libc::c_uint).wrapping_add(
        e.wrapping_add(rotl32(a, 5i32 as libc::c_uint))
          .wrapping_add(rconsts[i as usize]),
      ) as uint32_t as uint32_t;
      /* Rotate by one for next time */
      e = d;
      d = c;
      c = rotl32(b, 30i32 as libc::c_uint);
      b = a;
      a = work;
      cnt = cnt + 1i32 & 15i32;
      j -= 1;
      if !(j >= 0i32) {
        break;
      }
    }
    i += 1
  }
  (*ctx).hash[0] = ((*ctx).hash[0] as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
  (*ctx).hash[1] = ((*ctx).hash[1] as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
  (*ctx).hash[2] = ((*ctx).hash[2] as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
  (*ctx).hash[3] = ((*ctx).hash[3] as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
  (*ctx).hash[4] = ((*ctx).hash[4] as libc::c_uint).wrapping_add(e) as uint32_t as uint32_t;
}
static mut sha_K: [sha_K_int; 80] = [
  0x428a2f98d728ae22u64 as sha_K_int,
  0x7137449123ef65cdu64 as sha_K_int,
  0xb5c0fbcfec4d3b2fu64 as sha_K_int,
  0xe9b5dba58189dbbcu64 as sha_K_int,
  0x3956c25bf348b538u64 as sha_K_int,
  0x59f111f1b605d019u64 as sha_K_int,
  0x923f82a4af194f9bu64 as sha_K_int,
  0xab1c5ed5da6d8118u64 as sha_K_int,
  0xd807aa98a3030242u64 as sha_K_int,
  0x12835b0145706fbeu64 as sha_K_int,
  0x243185be4ee4b28cu64 as sha_K_int,
  0x550c7dc3d5ffb4e2u64 as sha_K_int,
  0x72be5d74f27b896fu64 as sha_K_int,
  0x80deb1fe3b1696b1u64 as sha_K_int,
  0x9bdc06a725c71235u64 as sha_K_int,
  0xc19bf174cf692694u64 as sha_K_int,
  0xe49b69c19ef14ad2u64 as sha_K_int,
  0xefbe4786384f25e3u64 as sha_K_int,
  0xfc19dc68b8cd5b5u64 as sha_K_int,
  0x240ca1cc77ac9c65u64 as sha_K_int,
  0x2de92c6f592b0275u64 as sha_K_int,
  0x4a7484aa6ea6e483u64 as sha_K_int,
  0x5cb0a9dcbd41fbd4u64 as sha_K_int,
  0x76f988da831153b5u64 as sha_K_int,
  0x983e5152ee66dfabu64 as sha_K_int,
  0xa831c66d2db43210u64 as sha_K_int,
  0xb00327c898fb213fu64 as sha_K_int,
  0xbf597fc7beef0ee4u64 as sha_K_int,
  0xc6e00bf33da88fc2u64 as sha_K_int,
  0xd5a79147930aa725u64 as sha_K_int,
  0x6ca6351e003826fu64 as sha_K_int,
  0x142929670a0e6e70u64 as sha_K_int,
  0x27b70a8546d22ffcu64 as sha_K_int,
  0x2e1b21385c26c926u64 as sha_K_int,
  0x4d2c6dfc5ac42aedu64 as sha_K_int,
  0x53380d139d95b3dfu64 as sha_K_int,
  0x650a73548baf63deu64 as sha_K_int,
  0x766a0abb3c77b2a8u64 as sha_K_int,
  0x81c2c92e47edaee6u64 as sha_K_int,
  0x92722c851482353bu64 as sha_K_int,
  0xa2bfe8a14cf10364u64 as sha_K_int,
  0xa81a664bbc423001u64 as sha_K_int,
  0xc24b8b70d0f89791u64 as sha_K_int,
  0xc76c51a30654be30u64 as sha_K_int,
  0xd192e819d6ef5218u64 as sha_K_int,
  0xd69906245565a910u64 as sha_K_int,
  0xf40e35855771202au64 as sha_K_int,
  0x106aa07032bbd1b8u64 as sha_K_int,
  0x19a4c116b8d2d0c8u64 as sha_K_int,
  0x1e376c085141ab53u64 as sha_K_int,
  0x2748774cdf8eeb99u64 as sha_K_int,
  0x34b0bcb5e19b48a8u64 as sha_K_int,
  0x391c0cb3c5c95a63u64 as sha_K_int,
  0x4ed8aa4ae3418acbu64 as sha_K_int,
  0x5b9cca4f7763e373u64 as sha_K_int,
  0x682e6ff3d6b2b8a3u64 as sha_K_int,
  0x748f82ee5defb2fcu64 as sha_K_int,
  0x78a5636f43172f60u64 as sha_K_int,
  0x84c87814a1f0ab72u64 as sha_K_int,
  0x8cc702081a6439ecu64 as sha_K_int,
  0x90befffa23631e28u64 as sha_K_int,
  0xa4506cebde82bde9u64 as sha_K_int,
  0xbef9a3f7b2c67915u64 as sha_K_int,
  0xc67178f2e372532bu64 as sha_K_int,
  0xca273eceea26619cu64 as sha_K_int,
  0xd186b8c721c0c207u64 as sha_K_int,
  0xeada7dd6cde0eb1eu64 as sha_K_int,
  0xf57d4f7fee6ed178u64 as sha_K_int,
  0x6f067aa72176fbau64 as sha_K_int,
  0xa637dc5a2c898a6u64 as sha_K_int,
  0x113f9804bef90daeu64 as sha_K_int,
  0x1b710b35131c471bu64 as sha_K_int,
  0x28db77f523047d84u64 as sha_K_int,
  0x32caab7b40c72493u64 as sha_K_int,
  0x3c9ebe0a15c9bebcu64 as sha_K_int,
  0x431d67c49c100d4cu64 as sha_K_int,
  0x4cc5d4becb3e42b6u64 as sha_K_int,
  0x597f299cfc657e2au64 as sha_K_int,
  0x5fcb6fab3ad6faecu64 as sha_K_int,
  0x6c44198c4a475817u64 as sha_K_int,
];
unsafe extern "C" fn sha256_process_block64(mut ctx: *mut sha256_ctx_t) {
  let mut t: libc::c_uint = 0;
  let mut W: [uint32_t; 64] = [0; 64];
  let mut a: uint32_t = 0;
  let mut b: uint32_t = 0;
  let mut c: uint32_t = 0;
  let mut d: uint32_t = 0;
  let mut e: uint32_t = 0;
  let mut f: uint32_t = 0;
  let mut g: uint32_t = 0;
  let mut h: uint32_t = 0;
  let mut words: *const uint32_t = (*ctx).wbuffer.as_mut_ptr() as *mut uint32_t;
  /* Operators defined in FIPS 180-2:4.1.2.  */
  /* Compute the message schedule according to FIPS 180-2:6.2.2 step 2.  */
  t = 0i32 as libc::c_uint;
  while t < 16i32 as libc::c_uint {
    W[t as usize] = {
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = *words.offset(t as isize);
      if 0 != 0 {
        __v = (__x & 0xff000000u32) >> 24i32
          | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
          | (__x & 0xff00i32 as libc::c_uint) << 8i32
          | (__x & 0xffi32 as libc::c_uint) << 24i32
      } else {
        let fresh35 = &mut __v;
        let fresh36;
        let fresh37 = __x;
        asm!("bswap $0" : "=r" (fresh36) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh35, fresh37))
                          :);
        c2rust_asm_casts::AsmCast::cast_out(fresh35, fresh37, fresh36);
      }
      __v
    };
    t = t.wrapping_add(1)
  }
  while t < 64i32 as libc::c_uint {
    W[t as usize] = (rotr32(
      W[t.wrapping_sub(2i32 as libc::c_uint) as usize],
      17i32 as libc::c_uint,
    ) ^ rotr32(
      W[t.wrapping_sub(2i32 as libc::c_uint) as usize],
      19i32 as libc::c_uint,
    ) ^ W[t.wrapping_sub(2i32 as libc::c_uint) as usize] >> 10i32)
      .wrapping_add(W[t.wrapping_sub(7i32 as libc::c_uint) as usize])
      .wrapping_add(
        rotr32(
          W[t.wrapping_sub(15i32 as libc::c_uint) as usize],
          7i32 as libc::c_uint,
        ) ^ rotr32(
          W[t.wrapping_sub(15i32 as libc::c_uint) as usize],
          18i32 as libc::c_uint,
        ) ^ W[t.wrapping_sub(15i32 as libc::c_uint) as usize] >> 3i32,
      )
      .wrapping_add(W[t.wrapping_sub(16i32 as libc::c_uint) as usize]);
    t = t.wrapping_add(1)
  }
  a = (*ctx).hash[0];
  b = (*ctx).hash[1];
  c = (*ctx).hash[2];
  d = (*ctx).hash[3];
  e = (*ctx).hash[4];
  f = (*ctx).hash[5];
  g = (*ctx).hash[6];
  h = (*ctx).hash[7];
  /* The actual computation according to FIPS 180-2:6.2.2 step 3.  */
  t = 0i32 as libc::c_uint;
  while t < 64i32 as libc::c_uint {
    /* Need to fetch upper half of sha_K[t]
     * (I hope compiler is clever enough to just fetch
     * upper half)
     */
    let mut K_t: uint32_t = if 1i32 != 0 || 1i32 != 0 {
      (sha_K[t as usize]) >> 32i32
    } else {
      sha_K[t as usize]
    } as uint32_t;
    let mut T1: uint32_t = h
      .wrapping_add(
        rotr32(e, 6i32 as libc::c_uint)
          ^ rotr32(e, 11i32 as libc::c_uint)
          ^ rotr32(e, 25i32 as libc::c_uint),
      )
      .wrapping_add(e & f ^ !e & g)
      .wrapping_add(K_t)
      .wrapping_add(W[t as usize]);
    let mut T2: uint32_t = (rotr32(a, 2i32 as libc::c_uint)
      ^ rotr32(a, 13i32 as libc::c_uint)
      ^ rotr32(a, 22i32 as libc::c_uint))
    .wrapping_add(a & b ^ a & c ^ b & c);
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(T1);
    d = c;
    c = b;
    b = a;
    a = T1.wrapping_add(T2);
    t = t.wrapping_add(1)
  }
  /* Add the starting values of the context according to FIPS 180-2:6.2.2
  step 4.  */
  (*ctx).hash[0] = ((*ctx).hash[0] as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
  (*ctx).hash[1] = ((*ctx).hash[1] as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
  (*ctx).hash[2] = ((*ctx).hash[2] as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
  (*ctx).hash[3] = ((*ctx).hash[3] as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
  (*ctx).hash[4] = ((*ctx).hash[4] as libc::c_uint).wrapping_add(e) as uint32_t as uint32_t;
  (*ctx).hash[5] = ((*ctx).hash[5] as libc::c_uint).wrapping_add(f) as uint32_t as uint32_t;
  (*ctx).hash[6] = ((*ctx).hash[6] as libc::c_uint).wrapping_add(g) as uint32_t as uint32_t;
  (*ctx).hash[7] = ((*ctx).hash[7] as libc::c_uint).wrapping_add(h) as uint32_t as uint32_t;
}
unsafe extern "C" fn sha512_process_block128(mut ctx: *mut sha512_ctx_t) {
  let mut t: libc::c_uint = 0;
  let mut W: [uint64_t; 80] = [0; 80];
  /* On i386, having assignments here (not later as sha256 does)
   * produces 99 bytes smaller code with gcc 4.3.1
   */
  let mut a: uint64_t = (*ctx).hash[0];
  let mut b: uint64_t = (*ctx).hash[1];
  let mut c: uint64_t = (*ctx).hash[2];
  let mut d: uint64_t = (*ctx).hash[3];
  let mut e: uint64_t = (*ctx).hash[4];
  let mut f: uint64_t = (*ctx).hash[5];
  let mut g: uint64_t = (*ctx).hash[6];
  let mut h: uint64_t = (*ctx).hash[7];
  let mut words: *const uint64_t = (*ctx).wbuffer.as_mut_ptr() as *mut uint64_t;
  /* Operators defined in FIPS 180-2:4.1.2.  */
  /* Compute the message schedule according to FIPS 180-2:6.3.2 step 2.  */
  t = 0i32 as libc::c_uint;
  while t < 16i32 as libc::c_uint {
    W[t as usize] = {
      let mut __v: __uint64_t = 0;
      let mut __x: __uint64_t = *words.offset(t as isize);
      if 0 != 0 {
        __v = ((__x as libc::c_ulonglong & 0xff00000000000000u64) >> 56i32
          | (__x as libc::c_ulonglong & 0xff000000000000u64) >> 40i32
          | (__x as libc::c_ulonglong & 0xff0000000000u64) >> 24i32
          | (__x as libc::c_ulonglong & 0xff00000000u64) >> 8i32
          | (__x as libc::c_ulonglong & 0xff000000u64) << 8i32
          | (__x as libc::c_ulonglong & 0xff0000u64) << 24i32
          | (__x as libc::c_ulonglong & 0xff00u64) << 40i32
          | (__x as libc::c_ulonglong & 0xffu64) << 56i32) as __uint64_t
      } else {
        let fresh38 = &mut __v;
        let fresh39;
        let fresh40 = __x;
        asm!("bswap ${0:q}" : "=r" (fresh39) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh38, fresh40))
                          :);
        c2rust_asm_casts::AsmCast::cast_out(fresh38, fresh40, fresh39);
      }
      __v
    };
    t = t.wrapping_add(1)
  }
  while t < 80i32 as libc::c_uint {
    W[t as usize] = (rotr64(
      W[t.wrapping_sub(2i32 as libc::c_uint) as usize],
      19i32 as libc::c_uint,
    ) ^ rotr64(
      W[t.wrapping_sub(2i32 as libc::c_uint) as usize],
      61i32 as libc::c_uint,
    ) ^ W[t.wrapping_sub(2i32 as libc::c_uint) as usize] >> 6i32)
      .wrapping_add(W[t.wrapping_sub(7i32 as libc::c_uint) as usize])
      .wrapping_add(
        rotr64(
          W[t.wrapping_sub(15i32 as libc::c_uint) as usize],
          1i32 as libc::c_uint,
        ) ^ rotr64(
          W[t.wrapping_sub(15i32 as libc::c_uint) as usize],
          8i32 as libc::c_uint,
        ) ^ W[t.wrapping_sub(15i32 as libc::c_uint) as usize] >> 7i32,
      )
      .wrapping_add(W[t.wrapping_sub(16i32 as libc::c_uint) as usize]);
    t = t.wrapping_add(1)
  }
  /* The actual computation according to FIPS 180-2:6.3.2 step 3.  */
  t = 0i32 as libc::c_uint;
  while t < 80i32 as libc::c_uint {
    let mut T1: uint64_t = h
      .wrapping_add(
        rotr64(e, 14i32 as libc::c_uint)
          ^ rotr64(e, 18i32 as libc::c_uint)
          ^ rotr64(e, 41i32 as libc::c_uint),
      )
      .wrapping_add(e & f ^ !e & g)
      .wrapping_add(sha_K[t as usize])
      .wrapping_add(W[t as usize]);
    let mut T2: uint64_t = (rotr64(a, 28i32 as libc::c_uint)
      ^ rotr64(a, 34i32 as libc::c_uint)
      ^ rotr64(a, 39i32 as libc::c_uint))
    .wrapping_add(a & b ^ a & c ^ b & c);
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(T1);
    d = c;
    c = b;
    b = a;
    a = T1.wrapping_add(T2);
    t = t.wrapping_add(1)
  }
  /* Add the starting values of the context according to FIPS 180-2:6.3.2
  step 4.  */
  (*ctx).hash[0] = ((*ctx).hash[0] as libc::c_ulong).wrapping_add(a) as uint64_t as uint64_t;
  (*ctx).hash[1] = ((*ctx).hash[1] as libc::c_ulong).wrapping_add(b) as uint64_t as uint64_t;
  (*ctx).hash[2] = ((*ctx).hash[2] as libc::c_ulong).wrapping_add(c) as uint64_t as uint64_t;
  (*ctx).hash[3] = ((*ctx).hash[3] as libc::c_ulong).wrapping_add(d) as uint64_t as uint64_t;
  (*ctx).hash[4] = ((*ctx).hash[4] as libc::c_ulong).wrapping_add(e) as uint64_t as uint64_t;
  (*ctx).hash[5] = ((*ctx).hash[5] as libc::c_ulong).wrapping_add(f) as uint64_t as uint64_t;
  (*ctx).hash[6] = ((*ctx).hash[6] as libc::c_ulong).wrapping_add(g) as uint64_t as uint64_t;
  (*ctx).hash[7] = ((*ctx).hash[7] as libc::c_ulong).wrapping_add(h) as uint64_t as uint64_t;
}
/* NEED_SHA512 */
#[no_mangle]
pub unsafe extern "C" fn sha1_begin(mut ctx: *mut sha1_ctx_t) {
  (*ctx).hash[0] = 0x67452301i32 as uint32_t;
  (*ctx).hash[1] = 0xefcdab89u32;
  (*ctx).hash[2] = 0x98badcfeu32;
  (*ctx).hash[3] = 0x10325476i32 as uint32_t;
  (*ctx).hash[4] = 0xc3d2e1f0u32;
  (*ctx).total64 = 0i32 as uint64_t;
  (*ctx).process_block =
    Some(sha1_process_block64 as unsafe extern "C" fn(_: *mut sha1_ctx_t) -> ());
}
static mut init256: [uint32_t; 10] = [
  0i32 as uint32_t,
  0i32 as uint32_t,
  0x6a09e667i32 as uint32_t,
  0xbb67ae85u32,
  0x3c6ef372i32 as uint32_t,
  0xa54ff53au32,
  0x510e527fi32 as uint32_t,
  0x9b05688cu32,
  0x1f83d9abi32 as uint32_t,
  0x5be0cd19i32 as uint32_t,
];
static mut init512_lo: [uint32_t; 10] = [
  0i32 as uint32_t,
  0i32 as uint32_t,
  0xf3bcc908u32,
  0x84caa73bu32,
  0xfe94f82bu32,
  0x5f1d36f1i32 as uint32_t,
  0xade682d1u32,
  0x2b3e6c1fi32 as uint32_t,
  0xfb41bd6bu32,
  0x137e2179i32 as uint32_t,
];
/* NEED_SHA512 */
// Note: SHA-384 is identical to SHA-512, except that initial hash values are
// 0xcbbb9d5dc1059ed8, 0x629a292a367cd507, 0x9159015a3070dd17, 0x152fecd8f70e5939,
// 0x67332667ffc00b31, 0x8eb44a8768581511, 0xdb0c2e0d64f98fa7, 0x47b5481dbefa4fa4,
// and the output is constructed by omitting last two 64-bit words of it.
/* Initialize structure containing state of computation.
(FIPS 180-2:5.3.2)  */
#[no_mangle]
pub unsafe extern "C" fn sha256_begin(mut ctx: *mut sha256_ctx_t) {
  memcpy(
    &mut (*ctx).total64 as *mut uint64_t as *mut libc::c_void,
    init256.as_ptr() as *const libc::c_void,
    ::std::mem::size_of::<[uint32_t; 10]>() as libc::c_ulong,
  );
  /*ctx->total64 = 0; - done by prepending two 32-bit zeros to init256 */
  (*ctx).process_block =
    Some(sha256_process_block64 as unsafe extern "C" fn(_: *mut sha256_ctx_t) -> ());
}
/* Initialize structure containing state of computation.
(FIPS 180-2:5.3.3)  */
#[no_mangle]
pub unsafe extern "C" fn sha512_begin(mut ctx: *mut sha512_ctx_t) {
  let mut i: libc::c_int = 0;
  /* Two extra iterations zero out ctx->total64[2] */
  let mut tp: *mut uint64_t = (*ctx).total64.as_mut_ptr();
  i = 0i32;
  while i < 2i32 + 8i32 {
    *tp.offset(i as isize) = ((init256[i as usize] as uint64_t) << 32i32)
      .wrapping_add(init512_lo[i as usize] as libc::c_ulong);
    i += 1
  }
  /*ctx->total64[0] = ctx->total64[1] = 0; - already done */
}
#[no_mangle]
pub unsafe extern "C" fn sha512_hash(
  mut ctx: *mut sha512_ctx_t,
  mut buffer: *const libc::c_void,
  mut len: size_t,
) {
  let mut bufpos: libc::c_uint = ((*ctx).total64[0] & 127i32 as libc::c_ulong) as libc::c_uint;
  let mut remaining: libc::c_uint = 0;
  /* First increment the byte count.  FIPS 180-2 specifies the possible
  length of the file up to 2^128 _bits_.
  We compute the number of _bytes_ and convert to bits later.  */
  (*ctx).total64[0] =
    ((*ctx).total64[0] as libc::c_ulong).wrapping_add(len) as uint64_t as uint64_t;
  if (*ctx).total64[0] < len {
    (*ctx).total64[1] = (*ctx).total64[1].wrapping_add(1)
  }
  loop {
    remaining = (128i32 as libc::c_uint).wrapping_sub(bufpos);
    if remaining as libc::c_ulong > len {
      remaining = len as libc::c_uint
    }
    /*bufpos = 0; - already is */
    /* Copy data into aligned buffer */
    memcpy(
      (*ctx).wbuffer.as_mut_ptr().offset(bufpos as isize) as *mut libc::c_void,
      buffer,
      remaining as libc::c_ulong,
    );
    len = (len as libc::c_ulong).wrapping_sub(remaining as libc::c_ulong) as size_t as size_t;
    buffer = (buffer as *const libc::c_char).offset(remaining as isize) as *const libc::c_void;
    bufpos = bufpos.wrapping_add(remaining);
    /* Clever way to do "if (bufpos != N) break; ... ; bufpos = 0;" */
    bufpos = bufpos.wrapping_sub(128i32 as libc::c_uint);
    if bufpos != 0i32 as libc::c_uint {
      break;
    }
    sha512_process_block128(ctx);
  }
}
/* Buffer is filled up, process it */
/* NEED_SHA512 */
/* Used also for sha256 */
#[no_mangle]
pub unsafe extern "C" fn sha1_end(
  mut ctx: *mut sha1_ctx_t,
  mut resbuf: *mut libc::c_void,
) -> libc::c_uint {
  let mut hash_size: libc::c_uint = 0;
  /* SHA stores total in BE, need to swap on LE arches: */
  common64_end(ctx, 1i32);
  hash_size = if (*ctx).process_block
    == Some(sha1_process_block64 as unsafe extern "C" fn(_: *mut sha1_ctx_t) -> ())
  {
    5i32
  } else {
    8i32
  } as libc::c_uint;
  /* This way we do not impose alignment constraints on resbuf: */
  let mut i: libc::c_uint = 0;
  i = 0i32 as libc::c_uint;
  while i < hash_size {
    (*ctx).hash[i as usize] = {
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = (*ctx).hash[i as usize];
      if 0 != 0 {
        __v = (__x & 0xff000000u32) >> 24i32
          | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
          | (__x & 0xff00i32 as libc::c_uint) << 8i32
          | (__x & 0xffi32 as libc::c_uint) << 24i32
      } else {
        let fresh41 = &mut __v;
        let fresh42;
        let fresh43 = __x;
        asm!("bswap $0" : "=r" (fresh42) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh41, fresh43))
                          :);
        c2rust_asm_casts::AsmCast::cast_out(fresh41, fresh43, fresh42);
      }
      __v
    };
    i = i.wrapping_add(1)
  }
  hash_size = (hash_size as libc::c_ulong)
    .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong) as libc::c_uint
    as libc::c_uint;
  memcpy(
    resbuf,
    (*ctx).hash.as_mut_ptr() as *const libc::c_void,
    hash_size as libc::c_ulong,
  );
  return hash_size;
}
#[no_mangle]
pub unsafe extern "C" fn sha512_end(
  mut ctx: *mut sha512_ctx_t,
  mut resbuf: *mut libc::c_void,
) -> libc::c_uint {
  let mut bufpos: libc::c_uint = ((*ctx).total64[0] & 127i32 as libc::c_ulong) as libc::c_uint;
  /* Pad the buffer to the next 128-byte boundary with 0x80,0,0,0... */
  let fresh44 = bufpos;
  bufpos = bufpos.wrapping_add(1);
  (*ctx).wbuffer[fresh44 as usize] = 0x80i32 as uint8_t;
  loop {
    let mut remaining: libc::c_uint = (128i32 as libc::c_uint).wrapping_sub(bufpos);
    memset(
      (*ctx).wbuffer.as_mut_ptr().offset(bufpos as isize) as *mut libc::c_void,
      0i32,
      remaining as libc::c_ulong,
    );
    if remaining >= 16i32 as libc::c_uint {
      /* Store the 128-bit counter of bits in the buffer in BE format */
      let mut t: uint64_t = 0;
      t = (*ctx).total64[0] << 3i32;
      t = {
        let mut __v: __uint64_t = 0;
        let mut __x: __uint64_t = t;
        if 0 != 0 {
          __v = ((__x as libc::c_ulonglong & 0xff00000000000000u64) >> 56i32
            | (__x as libc::c_ulonglong & 0xff000000000000u64) >> 40i32
            | (__x as libc::c_ulonglong & 0xff0000000000u64) >> 24i32
            | (__x as libc::c_ulonglong & 0xff00000000u64) >> 8i32
            | (__x as libc::c_ulonglong & 0xff000000u64) << 8i32
            | (__x as libc::c_ulonglong & 0xff0000u64) << 24i32
            | (__x as libc::c_ulonglong & 0xff00u64) << 40i32
            | (__x as libc::c_ulonglong & 0xffu64) << 56i32) as __uint64_t
        } else {
          let fresh45 = &mut __v;
          let fresh46;
          let fresh47 = __x;
          asm!("bswap ${0:q}" : "=r" (fresh46) : "0"
                              (c2rust_asm_casts::AsmCast::cast_in(fresh45, fresh47))
                              :);
          c2rust_asm_casts::AsmCast::cast_out(fresh45, fresh47, fresh46);
        }
        __v
      };
      *(&mut *(*ctx).wbuffer.as_mut_ptr().offset((128i32 - 8i32) as isize) as *mut uint8_t
        as *mut bb__aliased_uint64_t) = t;
      t = (*ctx).total64[1] << 3i32 | (*ctx).total64[0] >> 61i32;
      t = {
        let mut __v: __uint64_t = 0;
        let mut __x: __uint64_t = t;
        if 0 != 0 {
          __v = ((__x as libc::c_ulonglong & 0xff00000000000000u64) >> 56i32
            | (__x as libc::c_ulonglong & 0xff000000000000u64) >> 40i32
            | (__x as libc::c_ulonglong & 0xff0000000000u64) >> 24i32
            | (__x as libc::c_ulonglong & 0xff00000000u64) >> 8i32
            | (__x as libc::c_ulonglong & 0xff000000u64) << 8i32
            | (__x as libc::c_ulonglong & 0xff0000u64) << 24i32
            | (__x as libc::c_ulonglong & 0xff00u64) << 40i32
            | (__x as libc::c_ulonglong & 0xffu64) << 56i32) as __uint64_t
        } else {
          let fresh48 = &mut __v;
          let fresh49;
          let fresh50 = __x;
          asm!("bswap ${0:q}" : "=r" (fresh49) : "0"
                              (c2rust_asm_casts::AsmCast::cast_in(fresh48, fresh50))
                              :);
          c2rust_asm_casts::AsmCast::cast_out(fresh48, fresh50, fresh49);
        }
        __v
      };
      *(&mut *(*ctx)
        .wbuffer
        .as_mut_ptr()
        .offset((128i32 - 16i32) as isize) as *mut uint8_t as *mut bb__aliased_uint64_t) = t
    }
    sha512_process_block128(ctx);
    if remaining >= 16i32 as libc::c_uint {
      break;
    }
    bufpos = 0i32 as libc::c_uint
  }
  let mut i: libc::c_uint = 0;
  i = 0i32 as libc::c_uint;
  while i
    < (::std::mem::size_of::<[uint64_t; 8]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<uint64_t>() as libc::c_ulong) as libc::c_uint
  {
    (*ctx).hash[i as usize] = {
      let mut __v: __uint64_t = 0;
      let mut __x: __uint64_t = (*ctx).hash[i as usize];
      if 0 != 0 {
        __v = ((__x as libc::c_ulonglong & 0xff00000000000000u64) >> 56i32
          | (__x as libc::c_ulonglong & 0xff000000000000u64) >> 40i32
          | (__x as libc::c_ulonglong & 0xff0000000000u64) >> 24i32
          | (__x as libc::c_ulonglong & 0xff00000000u64) >> 8i32
          | (__x as libc::c_ulonglong & 0xff000000u64) << 8i32
          | (__x as libc::c_ulonglong & 0xff0000u64) << 24i32
          | (__x as libc::c_ulonglong & 0xff00u64) << 40i32
          | (__x as libc::c_ulonglong & 0xffu64) << 56i32) as __uint64_t
      } else {
        let fresh51 = &mut __v;
        let fresh52;
        let fresh53 = __x;
        asm!("bswap ${0:q}" : "=r" (fresh52) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh51, fresh53))
                          :);
        c2rust_asm_casts::AsmCast::cast_out(fresh51, fresh53, fresh52);
      }
      __v
    };
    i = i.wrapping_add(1)
  }
  memcpy(
    resbuf,
    (*ctx).hash.as_mut_ptr() as *const libc::c_void,
    ::std::mem::size_of::<[uint64_t; 8]>() as libc::c_ulong,
  );
  return ::std::mem::size_of::<[uint64_t; 8]>() as libc::c_ulong as libc::c_uint;
}
/*
 * SHA3 can be optimized for 32-bit CPUs with bit-slicing:
 * every 64-bit word of state[] can be split into two 32-bit words
 * by even/odd bits. In this form, all rotations of sha3 round
 * are 32-bit - and there are lots of them.
 * However, it requires either splitting/combining state words
 * before/after sha3 round (code does this now)
 * or shuffling bits before xor'ing them into state and in sha3_end.
 * Without shuffling, bit-slicing results in -130 bytes of code
 * and marginal speedup (but of course it gives wrong result).
 * With shuffling it works, but +260 code bytes, and slower.
 * Disabled for now:
 */
/* LONG_MAX == 0x7fffffff */
/*
 * In the crypto literature this function is usually called Keccak-f().
 */
unsafe extern "C" fn sha3_process_block72(mut state: *mut uint64_t) {
  /* Native 64-bit algorithm */
  static mut IOTA_CONST: [uint16_t; 24] = [
    0x1i32 as uint16_t,
    0x8082i32 as uint16_t,
    0x808ai32 as uint16_t,
    0x8000i32 as uint16_t,
    0x808bi32 as uint16_t,
    0x1i32 as uint16_t,
    0x8081i32 as uint16_t,
    0x8009i32 as uint16_t,
    0x8ai32 as uint16_t,
    0x88i32 as uint16_t,
    0x8009i32 as uint16_t,
    0xai32 as uint16_t,
    0x808bi32 as uint16_t,
    0x8bi32 as uint16_t,
    0x8089i32 as uint16_t,
    0x8003i32 as uint16_t,
    0x8002i32 as uint16_t,
    0x80i32 as uint16_t,
    0x800ai32 as uint16_t,
    0xai32 as uint16_t,
    0x8081i32 as uint16_t,
    0x8080i32 as uint16_t,
    0x1i32 as uint16_t,
    0x8008i32 as uint16_t,
  ];
  /* bit for CONST[0] is in msb: 0011 0011 0000 0111 1101 1101 */
  let IOTA_CONST_bit63: uint32_t = 0x3307dd00i32 as uint32_t;
  /* bit for CONST[0] is in msb: 0001 0110 0011 1000 0001 1011 */
  let IOTA_CONST_bit31: uint32_t = 0x16381b00i32 as uint32_t;
  static mut ROT_CONST: [uint8_t; 24] = [
    1i32 as uint8_t,
    3i32 as uint8_t,
    6i32 as uint8_t,
    10i32 as uint8_t,
    15i32 as uint8_t,
    21i32 as uint8_t,
    28i32 as uint8_t,
    36i32 as uint8_t,
    45i32 as uint8_t,
    55i32 as uint8_t,
    2i32 as uint8_t,
    14i32 as uint8_t,
    27i32 as uint8_t,
    41i32 as uint8_t,
    56i32 as uint8_t,
    8i32 as uint8_t,
    25i32 as uint8_t,
    43i32 as uint8_t,
    62i32 as uint8_t,
    18i32 as uint8_t,
    39i32 as uint8_t,
    61i32 as uint8_t,
    20i32 as uint8_t,
    44i32 as uint8_t,
  ];
  static mut PI_LANE: [uint8_t; 24] = [
    10i32 as uint8_t,
    7i32 as uint8_t,
    11i32 as uint8_t,
    17i32 as uint8_t,
    18i32 as uint8_t,
    3i32 as uint8_t,
    5i32 as uint8_t,
    16i32 as uint8_t,
    8i32 as uint8_t,
    21i32 as uint8_t,
    24i32 as uint8_t,
    4i32 as uint8_t,
    15i32 as uint8_t,
    23i32 as uint8_t,
    19i32 as uint8_t,
    13i32 as uint8_t,
    12i32 as uint8_t,
    2i32 as uint8_t,
    20i32 as uint8_t,
    14i32 as uint8_t,
    22i32 as uint8_t,
    9i32 as uint8_t,
    6i32 as uint8_t,
    1i32 as uint8_t,
  ];
  /*static const uint8_t MOD5[10] = { 0, 1, 2, 3, 4, 0, 1, 2, 3, 4, };*/
  let mut x: libc::c_uint = 0;
  let mut round: libc::c_uint = 0;
  round = 0i32 as libc::c_uint;
  while round < NROUNDS as libc::c_int as libc::c_uint {
    /* Theta */
    let mut BC: [uint64_t; 10] = [0; 10];
    x = 0i32 as libc::c_uint;
    while x < 5i32 as libc::c_uint {
      BC[x as usize] = *state.offset(x as isize)
        ^ *state.offset(x.wrapping_add(5i32 as libc::c_uint) as isize)
        ^ *state.offset(x.wrapping_add(10i32 as libc::c_uint) as isize)
        ^ *state.offset(x.wrapping_add(15i32 as libc::c_uint) as isize)
        ^ *state.offset(x.wrapping_add(20i32 as libc::c_uint) as isize);
      BC[x.wrapping_add(5i32 as libc::c_uint) as usize] = BC[x as usize];
      x = x.wrapping_add(1)
    }
    /* Using 2x5 vector above eliminates the need to use
     * BC[MOD5[x+N]] trick below to fetch BC[(x+N) % 5],
     * and the code is a bit _smaller_.
     */
    x = 0i32 as libc::c_uint;
    while x < 5i32 as libc::c_uint {
      let mut temp: uint64_t = BC[x.wrapping_add(4i32 as libc::c_uint) as usize]
        ^ rotl64(
          BC[x.wrapping_add(1i32 as libc::c_uint) as usize],
          1i32 as libc::c_uint,
        );
      let ref mut fresh54 = *state.offset(x as isize);
      *fresh54 ^= temp;
      let ref mut fresh55 = *state.offset(x.wrapping_add(5i32 as libc::c_uint) as isize);
      *fresh55 ^= temp;
      let ref mut fresh56 = *state.offset(x.wrapping_add(10i32 as libc::c_uint) as isize);
      *fresh56 ^= temp;
      let ref mut fresh57 = *state.offset(x.wrapping_add(15i32 as libc::c_uint) as isize);
      *fresh57 ^= temp;
      let ref mut fresh58 = *state.offset(x.wrapping_add(20i32 as libc::c_uint) as isize);
      *fresh58 ^= temp;
      x = x.wrapping_add(1)
    }
    /* Rho Pi */
    let mut t1: uint64_t = *state.offset(1);
    x = 0i32 as libc::c_uint;
    while x < 24i32 as libc::c_uint {
      let mut t0: uint64_t = *state.offset(PI_LANE[x as usize] as isize);
      *state.offset(PI_LANE[x as usize] as isize) =
        rotl64(t1, ROT_CONST[x as usize] as libc::c_uint);
      t1 = t0;
      x = x.wrapping_add(1)
    }
    /* Chi */
    x = 0i32 as libc::c_uint;
    while x <= 20i32 as libc::c_uint {
      let mut BC0: uint64_t = 0;
      let mut BC1: uint64_t = 0;
      let mut BC2: uint64_t = 0;
      let mut BC3: uint64_t = 0;
      let mut BC4: uint64_t = 0;
      BC0 = *state.offset(x.wrapping_add(0i32 as libc::c_uint) as isize);
      BC1 = *state.offset(x.wrapping_add(1i32 as libc::c_uint) as isize);
      BC2 = *state.offset(x.wrapping_add(2i32 as libc::c_uint) as isize);
      *state.offset(x.wrapping_add(0i32 as libc::c_uint) as isize) = BC0 ^ !BC1 & BC2;
      BC3 = *state.offset(x.wrapping_add(3i32 as libc::c_uint) as isize);
      *state.offset(x.wrapping_add(1i32 as libc::c_uint) as isize) = BC1 ^ !BC2 & BC3;
      BC4 = *state.offset(x.wrapping_add(4i32 as libc::c_uint) as isize);
      *state.offset(x.wrapping_add(2i32 as libc::c_uint) as isize) = BC2 ^ !BC3 & BC4;
      *state.offset(x.wrapping_add(3i32 as libc::c_uint) as isize) = BC3 ^ !BC4 & BC0;
      *state.offset(x.wrapping_add(4i32 as libc::c_uint) as isize) = BC4 ^ !BC0 & BC1;
      x = x.wrapping_add(5i32 as libc::c_uint)
    }
    /* long is 32-bit */
    /* Iota */
    let ref mut fresh59 = *state.offset(0);
    *fresh59 ^= (IOTA_CONST[round as usize] as libc::c_uint
      | IOTA_CONST_bit31 << round & 0x80000000u32) as libc::c_ulong
      | ((IOTA_CONST_bit63 << round & 0x80000000u32) as uint64_t) << 32i32;
    round = round.wrapping_add(1)
  }
}
#[no_mangle]
pub unsafe extern "C" fn sha3_begin(mut ctx: *mut sha3_ctx_t) {
  memset(
    ctx as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<sha3_ctx_t>() as libc::c_ulong,
  );
  /* SHA3-512, user can override */
  (*ctx).input_block_bytes = ((1600i32 - 512i32 * 2i32) / 8i32) as libc::c_uint;
  /* 72 bytes */
}
#[no_mangle]
pub unsafe extern "C" fn sha3_hash(
  mut ctx: *mut sha3_ctx_t,
  mut buffer: *const libc::c_void,
  mut len: size_t,
) {
  let mut data: *const uint8_t = buffer as *const uint8_t;
  let mut bufpos: libc::c_uint = (*ctx).bytes_queued;
  loop {
    let mut remaining: libc::c_uint = (*ctx).input_block_bytes.wrapping_sub(bufpos);
    if remaining as libc::c_ulong > len {
      remaining = len as libc::c_uint
    }
    len = (len as libc::c_ulong).wrapping_sub(remaining as libc::c_ulong) as size_t as size_t;
    /*bufpos = 0; - already is */
    /* XOR data into buffer */
    while remaining != 0i32 as libc::c_uint {
      let mut buf: *mut uint8_t = (*ctx).state.as_mut_ptr() as *mut uint8_t;
      let fresh60 = data;
      data = data.offset(1);
      let ref mut fresh61 = *buf.offset(bufpos as isize);
      *fresh61 = (*fresh61 as libc::c_int ^ *fresh60 as libc::c_int) as uint8_t;
      bufpos = bufpos.wrapping_add(1);
      remaining = remaining.wrapping_sub(1)
    }
    /* Clever way to do "if (bufpos != N) break; ... ; bufpos = 0;" */
    bufpos = bufpos.wrapping_sub((*ctx).input_block_bytes);
    if bufpos != 0i32 as libc::c_uint {
      break;
    }
    sha3_process_block72((*ctx).state.as_mut_ptr());
  }
  (*ctx).bytes_queued = bufpos.wrapping_add((*ctx).input_block_bytes);
}
/* Buffer is filled up, process it */

/*
 * Busybox main internal header file
 *
 * Based in part on code from sash, Copyright (c) 1999 by David I. Bell
 * Permission has been granted to redistribute this code under GPL.
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* TODO: and glibc? */
/* There are two incompatible basename's, let's not use them! */
/* See the dirname/basename man page for details */
/* dirname,basename */
/* Don't do this here:
 * #include <sys/sysinfo.h>
 * Some linux/ includes pull in conflicting definition
 * of struct sysinfo (only in some toolchanins), which breaks build.
 * Include sys/sysinfo.h only in those files which need it.
 */
/* Just in case libc doesn't define some of these... */
//This breaks on bionic:
//# if !defined(__socklen_t_defined) && !defined(_SOCKLEN_T_DECLARED)
// /* We #define socklen_t *after* includes, otherwise we get
// * typedef redefinition errors from system headers
// * (in case "is it defined already" detection above failed)
// */
//#  define socklen_t bb_socklen_t
//   typedef unsigned socklen_t;
//# endif
//if this is still needed, add a fix along the lines of
//  ifdef SPECIFIC_BROKEN_LIBC_CHECK / typedef socklen_t / endif
//in platform.h instead!
/*
 * Use '%m' to append error string on platforms that support it,
 * '%s' and strerror() on those that don't.
 */
/*nothing*/
/* Some libc's forget to declare these, do it ourself */
/* klogctl is in libc's klog.h, but we cheat and not #include that */
/* Busybox does not use threads, we can speed up stdio. */
/* Above functions are required by POSIX.1-2008, below ones are extensions */
/* musl <= 1.1.15 does not support fflush_unlocked(NULL) */
//# undef  fflush
//# define fflush(stream) fflush_unlocked(stream)
/* Make all declarations hidden (-fvisibility flag only affects definitions) */
/* (don't include system headers after this until corresponding pop!) */
/* Tested to work correctly with all int types (IIRC :]) */
/* Large file support */
/* Note that CONFIG_LFS=y forces bbox to be built with all common ops
 * (stat, lseek etc) mapped to "largefile" variants by libc.
 * Practically it means that open() automatically has O_LARGEFILE added
 * and all filesize/file_offset parameters and struct members are "large"
 * (in today's world - signed 64bit). For full support of large files,
 * we need a few helper #defines (below) and careful use of off_t
 * instead of int/ssize_t. No lseek64(), O_LARGEFILE etc necessary */
/* CONFIG_LFS is on */
/* "long" is long enough on this system */
/* usage: sz = BB_STRTOOFF(s, NULL, 10); if (errno || sz < 0) die(); */
/* usage: printf("size: %"OFF_FMT"d (%"OFF_FMT"x)\n", sz, sz); */
/* scary. better ideas? (but do *test* them first!) */
/* Users report bionic to use 32-bit off_t even if LARGEFILE support is requested.
 * We misdetected that. Don't let it build:
 */
/* Some useful definitions */
/* Macros for min/max.  */
/* buffer allocation schemes */
/* glibc uses __errno_location() to get a ptr to errno */
/* We can just memorize it once - no multithreading in busybox :) */
/* dmalloc will redefine these to it's own implementation. It is safe
 * to have the prototypes here unconditionally.  */
/* After v = xrealloc_vector(v, SHIFT, idx) it's ok to use
 * at least v[idx] and v[idx+1], for all idx values.
 * SHIFT specifies how many new elements are added (1:2, 2:4, ..., 8:256...)
 * when all elements are used up. New elements are zeroed out.
 * xrealloc_vector(v, SHIFT, idx) *MUST* be called with consecutive IDXs -
 * skipping an index is a bad bug - it may miss a realloc!
 */
//TODO: supply a pointer to char[11] buffer (avoid statics)?
/* cp.c, mv.c, install.c depend on these values. CAREFUL when changing them! */
/* -p */
/* !-d */
/* -R */
/* -f */
/* -i */
/* -l */
/* -s */
/* -L */
/* -H */
/* -a = -pdR (mapped in cp.c) */
/* -r = -dR  (mapped in cp.c) */
/* -P = -d   (mapped in cp.c) */
/* -v */
/* -u */
/* -T */
/* --remove-destination */
/* bit 17 skipped for "cp --parents" */
/* cp --reflink=auto */
/* cp --reflink[=always] */
/*
 * Hole. cp may have some bits set here,
 * they should not affect remove_file()/copy_file()
 */
/* NB: without FILEUTILS_RECUR in flags, it will basically "cat"
 * the source, not copy (unless "source" is a directory).
 * This makes "cp /dev/null file" and "install /dev/null file" (!!!)
 * work coreutils-compatibly. */
/*ACTION_REVERSE      = (1 << 4), - unused */
/* more than enough for "/dev/ttyXXX" */
/* bb_copyfd_XX print read/write errors and return -1 if they occur */
/* "short" copy can be detected by return value < size */
/* this helper yells "short read!" if param is not -1 */
/* xxxx_strip version can modify its parameter:
 * "/"        -> "/"
 * "abc"      -> "abc"
 * "abc/def"  -> "def"
 * "abc/def/" -> "def" !!
 */
/* "abc/def/" -> "" and it never modifies 'path' */
/* Simpler version: does not special case "/" string */
/* NB: can violate const-ness (similarly to strchr) */
/* !RETURNS_MALLOC: it's a realloc-like function */
/* bb_signals(BB_FATAL_SIGS, handler) catches all signals which
 * otherwise would kill us, except for those resulting from bugs:
 * SIGSEGV, SIGILL, SIGFPE.
 * Other fatal signals not included (TODO?):
 * SIGBUS   Bus error (bad memory access)
 * SIGPOLL  Pollable event. Synonym of SIGIO
 * SIGPROF  Profiling timer expired
 * SIGSYS   Bad argument to routine
 * SIGTRAP  Trace/breakpoint trap
 *
 * The only known arch with some of these sigs not fitting
 * into 32 bits is parisc (SIGXCPU=33, SIGXFSZ=34, SIGSTKFLT=36).
 * Dance around with long long to guard against that...
 */
// Write to pipe with no readers
// Quit from keyboard
// Abort signal from abort(3)
// Timer signal from alarm(2)
// Virtual alarm clock
// CPU time limit exceeded
// File size limit exceeded
// Yes kids, these are also fatal!
/* Unlike signal() and bb_signals, sets handler with sigaction()
 * and in a way that while signal handler is run, no other signals
 * will be blocked; syscalls will not be restarted: */
/* syscalls like read() will be interrupted with EINTR: */
/* syscalls like read() won't be interrupted (though select/poll will be): */
/* Will do sigaction(signum, act, NULL): */
/* SIG_BLOCK/SIG_UNBLOCK all signals: */
/* Return old set in the same set: */
/* Standard handler which just records signo */
/* not FAST_FUNC! */
/* In this form code with pipes is much more readable */
/* Useful for having small structure members/global variables */
/* | AF_DECnet */
/* | AF_IPX */
/* SO_REUSEADDR allows a server to rebind to an address that is already
 * "in use" by old connections to e.g. previous server instance which is
 * killed or crashed. Without it bind will fail until all such connections
 * time out. Linux does not allow multiple live binds on same ip:port
 * regardless of SO_REUSEADDR (unlike some other flavors of Unix).
 * Turn it on before you call bind(). */
/* On Linux this never fails. */
/* NB: returns port in host byte order */
/* Create stream socket, and allocate suitable lsa.
 * (lsa of correct size and lsa->sa.sa_family (AF_INET/AF_INET6))
 * af == AF_UNSPEC will result in trying to create IPv6 socket,
 * and if kernel doesn't support it, fall back to IPv4.
 * This is useful if you plan to bind to resulting local lsa.
 */
/* Create server socket bound to bindaddr:port. bindaddr can be NULL,
 * numeric IP ("N.N.N.N") or numeric IPv6 address,
 * and can have ":PORT" suffix (for IPv6 use "[X:X:...:X]:PORT").
 * Only if there is no suffix, port argument is used */
/* NB: these set SO_REUSEADDR before bind */
/* Create client TCP socket connected to peer:port. Peer cannot be NULL.
 * Peer can be numeric IP ("N.N.N.N"), numeric IPv6 address or hostname,
 * and can have ":PORT" suffix (for IPv6 use "[X:X:...:X]:PORT").
 * If there is no suffix, port argument is used */
/* Connect to peer identified by lsa */
/* Get local address of bound or accepted socket */
/* Get remote address of connected or accepted socket */
/* Return malloc'ed len_and_sockaddr with socket address of host:port
 * Currently will return IPv4 or IPv6 sockaddrs only
 * (depending on host), but in theory nothing prevents e.g.
 * UNIX socket address being returned, IPX sockaddr etc...
 * On error does bb_error_msg and returns NULL */
/* Version which dies on error */
/* Same, useful if you want to force family (e.g. IPv6) */
/* Assign sin[6]_port member if the socket is an AF_INET[6] one,
 * otherwise no-op. Useful for ftp.
 * NB: does NOT do htons() internally, just direct assignment. */
/* Retrieve sin[6]_port or return -1 for non-INET[6] lsa's */
/* Reverse DNS. Returns NULL on failure. */
/* This one doesn't append :PORTNUM */
/* This one also doesn't fall back to dotted IP (returns NULL) */
/* inet_[ap]ton on steroids */
// "old" (ipv4 only) API
// users: traceroute.c hostname.c - use _list_ of all IPs
// Also mount.c and inetd.c are using gethostbyname(),
// + inet_common.c has additional IPv4-only stuff
/* opaque */
// RFC 5246
// sequence number
//   Each connection state contains a sequence number, which is
//   maintained separately for read and write states.  The sequence
//   number MUST be set to zero whenever a connection state is made the
//   active state.  Sequence numbers are of type uint64 and may not
//   exceed 2^64-1.
/*uint64_t read_seq64_be;*/
/*uint8_t *server_write_MAC_key;*/
//used by AES_GCM
/* 0 if argv[0] is NULL: */
/* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
 * But potentially slow, don't use in one-billion-times loops */
/* Note: does not use stdio, writes to fd 2 directly */
// gcc-4.1.1 still isn't good enough at optimizing it
// (+200 bytes compared to macro)
//static ALWAYS_INLINE
//int LONE_DASH(const char *s) { return s[0] == '-' && !s[1]; }
//static ALWAYS_INLINE
//int NOT_LONE_DASH(const char *s) { return s[0] != '-' || s[1]; }
/* Returns a string with unprintable chars replaced by '?' or
 * SUBST_WCHAR. This function is unicode-aware. */
/* Prints unprintable char ch as ^C or M-c to file
 * (M-c is used only if ch is ORed with PRINTABLE_META),
 * else it is printed as-is (except for ch = 0x9b) */
/* Return a string that is the printable representation of character ch.
 * Buffer must hold at least four characters. */
// NB: will return short read on error, not -1,
// if some data was read before error occurred
// Reads one line a-la fgets (but doesn't save terminating '\n').
// Reads byte-by-byte. Useful when it is important to not read ahead.
// Bytes are appended to pfx (which must be malloced, or NULL).
/* Reads block up to *maxsz_p (default: INT_MAX - 4095) */
/* Returns NULL if file can't be opened (default max size: INT_MAX - 4095) */
/* Never returns NULL */
/* Else use variable one (a bit more expensive) */
/* Autodetects gzip/bzip2 formats. fd may be in the middle of the file! */
/* Autodetects .gz etc */
/* lzma has no signature, need a little helper. NB: exist only for ENABLE_FEATURE_SEAMLESS_LZMA=y */
// NB: will return short write on error, not -1,
// if some data was written before error occurred
/* Close fd, but check for failures (some types of write errors) */
/* Reads and prints to stdout till eof, then closes FILE. Exits on error: */
/* Reads a line from a text file, up to a newline or NUL byte, inclusive.
 * Returns malloc'ed char*. If end is NULL '\n' isn't considered
 * end of line. If end isn't NULL, length of the chunk is stored in it.
 * Returns NULL if EOF/error.
 */
/* Reads up to (and including) TERMINATING_STRING: */
/* Same, with limited max size, and returns the length (excluding NUL): */
/* Chops off TERMINATING_STRING from the end: */
/* Reads up to (and including) "\n" or NUL byte: */
/* Chops off '\n' from the end, unlike fgets: */
/* Same, but doesn't try to conserve space (may have some slack after the end) */
/* extern char *xmalloc_fgetline_fast(FILE *file) FAST_FUNC RETURNS_MALLOC; */
/* Prints warning to stderr and returns NULL on failure: */
/* "Opens" stdin if filename is special, else just opens file: */
/* not FAST_FUNC! */
/* Wrapper which restarts poll on EINTR or ENOMEM.
 * On other errors complains [perror("poll")] and returns.
 * Warning! May take (much) longer than timeout_ms to return!
 * If this is a problem, use bare poll and open-code EINTR/ENOMEM handling */
/* Convert each alpha char in str to lower-case */
/* Returns a pointer past the formatted number, does NOT null-terminate */
/* Intelligent formatters of bignums */
/* If block_size == 0, display size without fractional part,
 * else display (size * block_size) with one decimal digit.
 * If display_unit == 0, show value no bigger than 1024 with suffix (K,M,G...),
 * else divide by display_unit and do not use suffix. */
/* "1024.0G" */
//TODO: provide pointer to buf (avoid statics)?
/* Put a string of hex bytes ("1b2e66fe"...), return advanced pointer */
/* Reverse */
/* Generate a UUID */
/* Last element is marked by mult == 0 */
/* Specialized: */
/* Using xatoi() instead of naive atoi() is not always convenient -
 * in many places people want *non-negative* values, but store them
 * in signed int. Therefore we need this one:
 * dies if input is not in [0, INT_MAX] range. Also will reject '-0' etc.
 * It should really be named xatoi_nonnegative (since it allows 0),
 * but that would be too long.
 */
/* Useful for reading port numbers */
/* These parse entries in /etc/passwd and /etc/group.  This is desirable
 * for BusyBox since we want to avoid using the glibc NSS stuff, which
 * increases target size and is often not needed on embedded systems.  */
/* wrapper: allows string to contain numeric uid or gid */
/* always sets uid and gid; returns 0 on failure */
/* always sets uid and gid; exits on failure */
/* chown-like handling of "user[:[group]" */
/* versions which cache results (useful for ps, ls etc) */
/* internally usernames are saved in fixed-sized char[] buffers */
/*
 * Returns (-1) terminated malloced result of getgroups().
 * Reallocs group_array (useful for repeated calls).
 * ngroups is an initial size of array. It is rounded up to 32 for realloc.
 * ngroups is updated on return.
 * ngroups can be NULL: bb_getgroups(NULL, NULL) is valid usage.
 * Dies on errors (on Linux, only xrealloc can cause this, not internal getgroups call).
 */
/* BB_EXECxx always execs (it's not doing NOFORK/NOEXEC stuff),
 * but it may exec busybox and call applet instead of searching PATH.
 */
/* xvfork() can't be a _function_, return after vfork in child mangles stack
 * in the parent. It must be a macro. */
/* NOMMU friendy fork+exec: */
/* wait4pid: unlike waitpid, waits ONLY for one process.
 * Returns sig + 0x180 if child is killed by signal.
 * It's safe to pass negative 'pids' from failed [v]fork -
 * wait4pid will return -1 (and will not clobber [v]fork's errno).
 * IOW: rc = wait4pid(spawn(argv));
 *      if (rc < 0) bb_perror_msg("%s", argv[0]);
 *      if (rc > 0) bb_error_msg("exit code: %d", rc & 0xff);
 */
/* ***********************************************************************/
/* spawn_and_wait/run_nofork_applet/run_applet_no_and_exit need to work */
/* carefully together to reinit some global state while not disturbing  */
/* other. Be careful if you change them. Consult docs/nofork_noexec.txt */
/* ***********************************************************************/
/* Same as wait4pid(spawn(argv)), but with NOFORK/NOEXEC if configured: */
/* Does NOT check that applet is NOFORK, just blindly runs it */
/* Helpers for daemonization.
 *
 * bb_daemonize(flags) = daemonize, does not compile on NOMMU
 *
 * bb_daemonize_or_rexec(flags, argv) = daemonizes on MMU (and ignores argv),
 *      rexec's itself on NOMMU with argv passed as command line.
 * Thus bb_daemonize_or_rexec may cause your <applet>_main() to be re-executed
 * from the start. (It will detect it and not reexec again second time).
 * You have to audit carefully that you don't do something twice as a result
 * (opening files/sockets, parsing config files etc...)!
 *
 * Both of the above will redirect fd 0,1,2 to /dev/null and drop ctty
 * (will do setsid()).
 *
 * fork_or_rexec(argv) = bare-bones fork on MMU,
 *      "vfork + re-exec ourself" on NOMMU. No fd redirection, no setsid().
 *      On MMU ignores argv.
 *
 * Helper for network daemons in foreground mode:
 *
 * bb_sanitize_stdio() = make sure that fd 0,1,2 are opened by opening them
 * to /dev/null if they are not.
 */
/* internal use */
//DAEMON_DOUBLE_FORK     = 1 << 4, /* double fork to avoid controlling tty */
/* Clear dangerous stuff, set PATH. Return 1 if was run by different user. */
/* For top, ps. Some argv[i] are replaced by malloced "-opt" strings */
/* { "-", NULL } */
/* BSD-derived getopt() functions require that optind be set to 1 in
 * order to reset getopt() state.  This used to be generally accepted
 * way of resetting getopt().  However, glibc's getopt()
 * has additional getopt() state beyond optind (specifically, glibc
 * extensions such as '+' and '-' at the start of the string), and requires
 * that optind be set to zero to reset its state.  BSD-derived versions
 * of getopt() misbehaved if optind is set to 0 in order to reset getopt(),
 * and glibc's getopt() used to coredump if optind is set 1 in order
 * to reset getopt().
 * Then BSD introduced additional variable "optreset" which should be
 * set to 1 in order to reset getopt().  Sigh.  Standards, anyone?
 *
 * By ~2008, OpenBSD 3.4 was changed to survive glibc-like optind = 0
 * (to interpret it as if optreset was set).
 */
/*def __GLIBC__*/
/* BSD style */
/* Having next pointer as a first member allows easy creation
 * of "llist-compatible" structs, and using llist_FOO functions
 * on them.
 */
/* BTW, surprisingly, changing API to
 *   llist_t *llist_add_to(llist_t *old_head, void *data)
 * etc does not result in smaller code... */
/* start_stop_daemon and udhcpc are special - they want
 * to create pidfiles regardless of FEATURE_PIDFILE */
/* True only if we created pidfile which is *file*, not /dev/null etc */
/* We need to export XXX_main from libbusybox
 * only if we build "individual" binaries
 */
/* Embedded script support */
/* Applets which are useful from another applets */
/* If shell needs them, they exist even if not enabled as applets */
/* Similar, but used by chgrp, not shell */
/* Used by ftpd */
/* Don't need IF_xxx() guard for these */
/* Networking */
/* This structure defines protocol families and their handlers. */
/*int type,*/
/* may modify src */
/* This structure defines hardware protocols and their handlers. */
/*
 * If *devname is not NULL, use that name, otherwise try to find free one,
 * malloc and return it in *devname.
 * return value is the opened fd to the loop device, or < on error
 */
/* These constants match linux/loop.h (without BB_ prefix): */
/* Returns malloced str */
/* Like bb_ask_noecho, but asks on stdin with no timeout.  */
/* Returns -1 if input is invalid. current_mode is a base for e.g. "u+rw" */
/*
 * Config file parser
 */
// treat consecutive delimiters as one
// trim leading and trailing delimiters
// TODO: COLLAPSE and TRIM seem to always go in pair
// last token takes entire remainder of the line
// die if < min tokens found
// keep a copy of current line
// comments are recognized even if they aren't the first char
// delim[0] and delim[1] are two different allowed comment chars
// (so far, delim[0] will only work as comment char for full-line comment)
// (IOW: it works as if PARSE_EOL_COMMENTS is not set. sysctl applet is okay with this)
// comments are recognized even if there is whitespace before
// ("line start><space><tab><space>#comment" is also comment, not only "line start>#comment")
// NORMAL is:
// * remove leading and trailing delimiters and collapse
//   multiple delimiters into one
// * warn and continue if less than mintokens delimiters found
// * grab everything into last token
// * comments are recognized even if they aren't the first char
/* delims[0] is a comment char (use '\0' to disable), the rest are token delimiters */
/* Concatenate path and filename to new allocated buffer.
 * Add "/" only as needed (no duplicate "//" are produced).
 * If path is NULL, it is assumed to be "/".
 * filename should not be NULL. */
/* Returns NULL on . and .. */
/* Returns ptr to NUL */
/* Returns $SHELL, getpwuid(getuid())->pw_shell, or DEFAULT_SHELL.
 * Note that getpwuid result might need xstrdup'ing
 * if there is a possibility of intervening getpwxxx() calls.
 */
/* Structures inside "struct caps" are Linux-specific and libcap-specific: */
/* setup_environment:
 * if chdir pw->pw_dir: ok: else if to_tmp == 1: goto /tmp else: goto / or die
 * if clear_env = 1: cd(pw->pw_dir), clear environment, then set
 *   TERM=(old value)
 *   USER=pw->pw_name, LOGNAME=pw->pw_name
 *   PATH=bb_default_[root_]path
 *   HOME=pw->pw_dir
 *   SHELL=shell
 * else if change_env = 1:
 *   if not root (if pw->pw_uid != 0):
 *     USER=pw->pw_name, LOGNAME=pw->pw_name
 *   HOME=pw->pw_dir
 *   SHELL=shell
 * else does nothing
 *
 * NB: CHANGEENV and CLEARENV use setenv() - this leaks memory!
 * If setup_environment() is used is vforked child, this leaks memory _in parent too_!
 */
/* Returns a malloced string */
/*
 * rnd is additional random input. New one is returned.
 * Useful if you call crypt_make_salt many times in a row:
 * rnd = crypt_make_salt(buf1, 4, 0);
 * rnd = crypt_make_salt(buf2, 4, rnd);
 * rnd = crypt_make_salt(buf3, 4, rnd);
 * (otherwise we risk having same salt generated)
 */
/*, int rnd*/
/* "$N$" + sha_salt_16_bytes + NUL */
/* Returns number of lines changed, or -1 on error */
/* NB: typically you want to pass fd 0, not 1. Think 'applet | grep something' */
/* NB: "unsigned request" is crucial! "int request" will break some arches! */
/* At least glibc has horrendously large inline for this, so wrap it */
/* "Keycodes" that report an escape sequence.
 * We use something which fits into signed char,
 * yet doesn't represent any valid Unicode character.
 * Also, -1 is reserved for error indication and we don't use it. */
/* Used only if Alt/Ctrl/Shifted */
/* Used only if Alted */
/* ^^^^^ Be sure that last defined value is small enough.
 * Current read_key() code allows going up to -32 (0xfff..fffe0).
 * This gives three upper bits in LSB to play with:
 * KEYCODE_foo values are 0xfff..fffXX, lowest XX bits are: scavvvvv,
 * s=0 if SHIFT, c=0 if CTRL, a=0 if ALT,
 * vvvvv bits are the same for same key regardless of "shift bits".
 */
//KEYCODE_SHIFT_...   = KEYCODE_...   & ~0x80,
/* 0xfff..fff00 */
/* How long is the longest ESC sequence we know?
 * We want it big enough to be able to contain
 * cursor position sequence "ESC [ 9999 ; 9999 R"
 */
/* Note: fd may be in blocking or non-blocking mode, both make sense.
 * For one, less uses non-blocking mode.
 * Only the first read syscall inside read_key may block indefinitely
 * (unless fd is in non-blocking mode),
 * subsequent reads will time out after a few milliseconds.
 * Return of -1 means EOF or error (errno == 0 on EOF).
 * buffer[0] is used as a counter of buffered chars and must be 0
 * on first call.
 * timeout:
 * -2: do not poll(-1) for input - read() it, return on EAGAIN at once
 * -1: poll(-1) (i.e. block even on NONBLOCKed fd)
 * >=0: poll() for TIMEOUT milliseconds, return -1/EAGAIN on timeout
 */
/* It's NOT just ENABLEd or disabled. It's a number: */
/* must never be <= 0 */
/* meaning of this field depends on FEATURE_EDITING_SAVE_ON_EXIT:
 * if !FEATURE_EDITING_SAVE_ON_EXIT: "how many lines are
 * in on-disk history"
 * if FEATURE_EDITING_SAVE_ON_EXIT: "how many in-memory lines are
 * also in on-disk history (and thus need to be skipped on save)"
 */
/*
 * maxsize must be >= 2.
 * Returns:
 * -1 on read errors or EOF, or on bare Ctrl-D,
 * 0  on ctrl-C (the line entered is still returned in 'command'),
 * >0 length of input string, including terminating '\n'
 */
/* synchronize with sizeof(task_struct.comm) in /usr/include/linux/sched.h */
// For mixed 32/64 userspace, 32-bit pmap still needs
// 64-bit field here to correctly show 64-bit processes:
// (strictly speaking, other fields need to be wider too,
// but they are in kbytes, not bytes, and they hold sizes,
// not start addresses, sizes tend to be less than 4 terabytes)
/* Fields are set to 0/NULL if failed to determine (or not requested) */
/* Everything below must contain no ptrs to malloc'ed data:
 * it is memset(0) for each process in procps_scan() */
/* we round it to kbytes */
/* basename of executable in exec(2), read from /proc/N/stat
 * (if executable is symlink or script, it is NOT replaced
 * by link target or interpreter name) */
/* user/group? - use passwd/group parsing functions */
/* flag bits for procps_scan(xx, flags) calls */
/* PSSCAN_CMD      = 1 << 6, - use read_cmdline instead */
/* NB: used by find_pid_by_name(). Any applet using it
 * needs to be mentioned here. */
//procps_status_t* alloc_procps_scan(void) FAST_FUNC;
/* Format cmdline (up to col chars) into char buf[size] */
/* Puts [comm] if cmdline is empty (-> process is a kernel thread) */
/* Use strict=1 if you process input from untrusted source:
 * it will return NULL on invalid %xx (bad hex chars)
 * and str + 1 if decoded char is / or NUL.
 * In non-strict mode, it always succeeds (returns str),
 * and also it additionally decoded '+' to space.
 */
/* Sign-extends to a value which never matches fgetc result: */
/* always correctly aligned for uint64_t */
/* must be directly before hash[] */
/* 4 elements for md5, 5 for sha1, 8 for sha256 */
/* must be directly before hash[] */
/* always correctly aligned for uint64_t */
#[no_mangle]
pub unsafe extern "C" fn sha3_end(
  mut ctx: *mut sha3_ctx_t,
  mut resbuf: *mut libc::c_void,
) -> libc::c_uint {
  /* Padding */
  let mut buf: *mut uint8_t = (*ctx).state.as_mut_ptr() as *mut uint8_t;
  /*
   * Keccak block padding is: add 1 bit after last bit of input,
   * then add zero bits until the end of block, and add the last 1 bit
   * (the last bit in the block) - the "10*1" pattern.
   * SHA3 standard appends additional two bits, 01,  before that padding:
   *
   * SHA3-224(M) = KECCAK[448](M||01, 224)
   * SHA3-256(M) = KECCAK[512](M||01, 256)
   * SHA3-384(M) = KECCAK[768](M||01, 384)
   * SHA3-512(M) = KECCAK[1024](M||01, 512)
   * (M is the input, || is bit concatenation)
   *
   * The 6 below contains 01 "SHA3" bits and the first 1 "Keccak" bit:
   */
  let ref mut fresh62 = *buf.offset((*ctx).bytes_queued as isize); /* bit pattern 00000110 */
  *fresh62 = (*fresh62 as libc::c_int ^ 6i32) as uint8_t;
  let ref mut fresh63 =
    *buf.offset((*ctx).input_block_bytes.wrapping_sub(1i32 as libc::c_uint) as isize);
  *fresh63 = (*fresh63 as libc::c_int ^ 0x80i32) as uint8_t;
  sha3_process_block72((*ctx).state.as_mut_ptr());
  /* Output */
  memcpy(
    resbuf,
    (*ctx).state.as_mut_ptr() as *const libc::c_void,
    64i32 as libc::c_ulong,
  );
  return 64i32 as libc::c_uint;
}

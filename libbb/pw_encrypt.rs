use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;

use libc;
use libc::free;
use libc::getpid;
use libc::sprintf;
use libc::strcasecmp;
use libc::strcpy;
extern "C" {

  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;

  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  fn monotonic_us() -> libc::c_ulonglong;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xstrndup(s: *const libc::c_char, n: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  #[no_mangle]
  fn sha512_end(ctx: *mut sha512_ctx_t, resbuf: *mut libc::c_void) -> libc::c_uint;
  #[no_mangle]
  fn sha512_hash(ctx: *mut sha512_ctx_t, buffer: *const libc::c_void, len: size_t);
  #[no_mangle]
  fn sha512_begin(ctx: *mut sha512_ctx_t);
  #[no_mangle]
  fn sha1_end(ctx: *mut sha1_ctx_t, resbuf: *mut libc::c_void) -> libc::c_uint;
  #[no_mangle]
  fn md5_hash(ctx: *mut md5_ctx_t, buffer: *const libc::c_void, len: size_t);
  #[no_mangle]
  fn sha256_begin(ctx: *mut sha256_ctx_t);
  #[no_mangle]
  fn md5_end(ctx: *mut md5_ctx_t, resbuf: *mut libc::c_void) -> libc::c_uint;
  #[no_mangle]
  fn md5_begin(ctx: *mut md5_ctx_t);
}

use crate::librb::size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct des_ctx {
  pub const_ctx: *const const_des_ctx,
  pub saltbits: u32,
  pub un_pbox: [u8; 32],
  pub inv_comp_perm: [u8; 56],
  pub inv_key_perm: [u8; 64],
  pub en_keysl: [u32; 16],
  pub en_keysr: [u32; 16],
  pub fp_maskl: [[u32; 256]; 8],
  pub fp_maskr: [[u32; 256]; 8],
  pub key_perm_maskl: [[u32; 128]; 8],
  pub key_perm_maskr: [[u32; 128]; 8],
  pub comp_maskl: [[u32; 128]; 8],
  pub comp_maskr: [[u32; 128]; 8],
  pub psbox: [[u32; 256]; 4],
  /* 5 times */
}
/* Static stuff that stays resident and doesn't change after
 * being initialized, and therefore doesn't need to be made
 * reentrant. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct const_des_ctx {
  pub final_perm: [u8; 64],
  pub m_sbox: [[u8; 4096]; 4],
  /* 5 times */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
  pub alt_result: [libc::c_uchar; 64],
  pub temp_result: [libc::c_uchar; 64],
  pub ctx: C2RustUnnamed_1,
  pub alt_ctx: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
  pub x: sha256_ctx_t,
  pub y: sha512_ctx_t,
}
use crate::librb::md5_ctx_t;
use crate::librb::sha256_ctx_t;
use crate::librb::sha512_ctx_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
  pub x: sha256_ctx_t,
  pub y: sha512_ctx_t,
}
use crate::librb::sha1_ctx_t;
/*
 * Utility routines.
 *
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* static const u8 ascii64[] ALIGN1 =
 * "./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
 */
unsafe extern "C" fn i64c(mut i: libc::c_int) -> libc::c_int {
  i &= 0x3fi32;
  if i == 0i32 {
    return '.' as i32;
  }
  if i == 1i32 {
    return '/' as i32;
  }
  if i < 12i32 {
    return '0' as i32 - 2i32 + i;
  }
  if i < 38i32 {
    return 'A' as i32 - 12i32 + i;
  }
  return 'a' as i32 - 38i32 + i;
}
#[no_mangle]
pub unsafe extern "C" fn crypt_make_salt(
  mut p: *mut libc::c_char,
  mut cnt: libc::c_int,
) -> libc::c_int
/*, int x */ {
  /* was: x += ... */
  let mut x: libc::c_uint =
    (getpid() as libc::c_ulonglong).wrapping_add(monotonic_us()) as libc::c_uint;
  loop {
    /* x = (x*1664525 + 1013904223) % 2^32 generator is lame
     * (low-order bit is not "random", etc...),
     * but for our purposes it is good enough */
    x = x
      .wrapping_mul(1664525i32 as libc::c_uint)
      .wrapping_add(1013904223i32 as libc::c_uint);
    /* BTW, Park and Miller's "minimal standard generator" is
     * x = x*16807 % ((2^31)-1)
     * It has no problem with visibly alternating lowest bit
     * but is also weak in cryptographic sense + needs div,
     * which needs more code (and slower) on many CPUs */
    let fresh0 = p;
    p = p.offset(1);
    *fresh0 = i64c((x >> 16i32) as libc::c_int) as libc::c_char;
    let fresh1 = p;
    p = p.offset(1);
    *fresh1 = i64c((x >> 22i32) as libc::c_int) as libc::c_char;
    cnt -= 1;
    if !(cnt != 0) {
      break;
    }
  }
  *p = '\u{0}' as i32 as libc::c_char;
  return x as libc::c_int;
}
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
#[no_mangle]
pub unsafe extern "C" fn crypt_make_pw_salt(
  mut salt: *mut libc::c_char,
  mut algo: *const libc::c_char,
) -> *mut libc::c_char {
  let mut len: libc::c_int = 2i32 / 2i32;
  let mut salt_ptr: *mut libc::c_char = salt;
  /* Standard chpasswd uses uppercase algos ("MD5", not "md5").
   * Need to be case-insensitive in the code below.
   */
  if *algo.offset(0) as libc::c_int | 0x20i32 != 'd' as i32 {
    /* not des */
    len = 8i32 / 2i32; /* so far assuming md5 */
    let fresh2 = salt_ptr;
    salt_ptr = salt_ptr.offset(1);
    *fresh2 = '$' as i32 as libc::c_char;
    let fresh3 = salt_ptr;
    salt_ptr = salt_ptr.offset(1);
    *fresh3 = '1' as i32 as libc::c_char;
    let fresh4 = salt_ptr;
    salt_ptr = salt_ptr.offset(1);
    *fresh4 = '$' as i32 as libc::c_char;
    if *algo.offset(0) as libc::c_int | 0x20i32 == 's' as i32 {
      /* sha */
      *salt.offset(1) = ('5' as i32
        + (strcasecmp(algo, b"sha512\x00" as *const u8 as *const libc::c_char) == 0i32)
          as libc::c_int) as libc::c_char;
      len = 16i32 / 2i32
    }
  }
  crypt_make_salt(salt_ptr, len);
  return salt_ptr;
}
unsafe extern "C" fn to64(
  mut s: *mut libc::c_char,
  mut v: libc::c_uint,
  mut n: libc::c_int,
) -> *mut libc::c_char {
  loop {
    n -= 1;
    if !(n >= 0i32) {
      break;
    }
    /* *s++ = ascii64[v & 0x3f]; */
    let fresh5 = s;
    s = s.offset(1);
    *fresh5 = i64c(v as libc::c_int) as libc::c_char;
    v >>= 6i32
  }
  return s;
}
/*
 * FreeSec: libcrypt for NetBSD
 *
 * Copyright (c) 1994 David Burren
 * All rights reserved.
 *
 * Adapted for FreeBSD-2.0 by Geoffrey M. Rehmet
 *	this file should now *only* export crypt(), in order to make
 *	binaries of libcrypt exportable from the USA
 *
 * Adapted for FreeBSD-4.0 by Mark R V Murray
 *	this file should now *only* export crypt_des(), in order to make
 *	a module that can be optionally included in libcrypt.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the author nor the names of other contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ''AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * This is an original implementation of the DES and the crypt(3) interfaces
 * by David Burren <davidb@werj.com.au>.
 *
 * An excellent reference on the underlying algorithm (and related
 * algorithms) is:
 *
 *	B. Schneier, Applied Cryptography: protocols, algorithms,
 *	and source code in C, John Wiley & Sons, 1994.
 *
 * Note that in that book's description of DES the lookups for the initial,
 * pbox, and final permutations are inverted (this has been brought to the
 * attention of the author).  A list of errata for this book has been
 * posted to the sci.crypt newsgroup by the author and is available for FTP.
 *
 * ARCHITECTURE ASSUMPTIONS:
 *	It is assumed that the 8-byte arrays passed by reference can be
 *	addressed as arrays of u32's (ie. the CPU is not picky about
 *	alignment).
 */
/* Parts busybox doesn't need or had optimized */
/* A pile of data */
static mut IP: [u8; 64] = [
  58i32 as u8,
  50i32 as u8,
  42i32 as u8,
  34i32 as u8,
  26i32 as u8,
  18i32 as u8,
  10i32 as u8,
  2i32 as u8,
  60i32 as u8,
  52i32 as u8,
  44i32 as u8,
  36i32 as u8,
  28i32 as u8,
  20i32 as u8,
  12i32 as u8,
  4i32 as u8,
  62i32 as u8,
  54i32 as u8,
  46i32 as u8,
  38i32 as u8,
  30i32 as u8,
  22i32 as u8,
  14i32 as u8,
  6i32 as u8,
  64i32 as u8,
  56i32 as u8,
  48i32 as u8,
  40i32 as u8,
  32i32 as u8,
  24i32 as u8,
  16i32 as u8,
  8i32 as u8,
  57i32 as u8,
  49i32 as u8,
  41i32 as u8,
  33i32 as u8,
  25i32 as u8,
  17i32 as u8,
  9i32 as u8,
  1i32 as u8,
  59i32 as u8,
  51i32 as u8,
  43i32 as u8,
  35i32 as u8,
  27i32 as u8,
  19i32 as u8,
  11i32 as u8,
  3i32 as u8,
  61i32 as u8,
  53i32 as u8,
  45i32 as u8,
  37i32 as u8,
  29i32 as u8,
  21i32 as u8,
  13i32 as u8,
  5i32 as u8,
  63i32 as u8,
  55i32 as u8,
  47i32 as u8,
  39i32 as u8,
  31i32 as u8,
  23i32 as u8,
  15i32 as u8,
  7i32 as u8,
];
static mut key_perm: [u8; 56] = [
  57i32 as u8,
  49i32 as u8,
  41i32 as u8,
  33i32 as u8,
  25i32 as u8,
  17i32 as u8,
  9i32 as u8,
  1i32 as u8,
  58i32 as u8,
  50i32 as u8,
  42i32 as u8,
  34i32 as u8,
  26i32 as u8,
  18i32 as u8,
  10i32 as u8,
  2i32 as u8,
  59i32 as u8,
  51i32 as u8,
  43i32 as u8,
  35i32 as u8,
  27i32 as u8,
  19i32 as u8,
  11i32 as u8,
  3i32 as u8,
  60i32 as u8,
  52i32 as u8,
  44i32 as u8,
  36i32 as u8,
  63i32 as u8,
  55i32 as u8,
  47i32 as u8,
  39i32 as u8,
  31i32 as u8,
  23i32 as u8,
  15i32 as u8,
  7i32 as u8,
  62i32 as u8,
  54i32 as u8,
  46i32 as u8,
  38i32 as u8,
  30i32 as u8,
  22i32 as u8,
  14i32 as u8,
  6i32 as u8,
  61i32 as u8,
  53i32 as u8,
  45i32 as u8,
  37i32 as u8,
  29i32 as u8,
  21i32 as u8,
  13i32 as u8,
  5i32 as u8,
  28i32 as u8,
  20i32 as u8,
  12i32 as u8,
  4i32 as u8,
];
static mut key_shifts: [u8; 16] = [
  1i32 as u8, 1i32 as u8, 2i32 as u8, 2i32 as u8, 2i32 as u8, 2i32 as u8, 2i32 as u8, 2i32 as u8,
  1i32 as u8, 2i32 as u8, 2i32 as u8, 2i32 as u8, 2i32 as u8, 2i32 as u8, 2i32 as u8, 1i32 as u8,
];
static mut comp_perm: [u8; 48] = [
  14i32 as u8,
  17i32 as u8,
  11i32 as u8,
  24i32 as u8,
  1i32 as u8,
  5i32 as u8,
  3i32 as u8,
  28i32 as u8,
  15i32 as u8,
  6i32 as u8,
  21i32 as u8,
  10i32 as u8,
  23i32 as u8,
  19i32 as u8,
  12i32 as u8,
  4i32 as u8,
  26i32 as u8,
  8i32 as u8,
  16i32 as u8,
  7i32 as u8,
  27i32 as u8,
  20i32 as u8,
  13i32 as u8,
  2i32 as u8,
  41i32 as u8,
  52i32 as u8,
  31i32 as u8,
  37i32 as u8,
  47i32 as u8,
  55i32 as u8,
  30i32 as u8,
  40i32 as u8,
  51i32 as u8,
  45i32 as u8,
  33i32 as u8,
  48i32 as u8,
  44i32 as u8,
  49i32 as u8,
  39i32 as u8,
  56i32 as u8,
  34i32 as u8,
  53i32 as u8,
  46i32 as u8,
  42i32 as u8,
  50i32 as u8,
  36i32 as u8,
  29i32 as u8,
  32i32 as u8,
];
/*
 * No E box is used, as it's replaced by some ANDs, shifts, and ORs.
 */
/* precomputed, with half-bytes packed into one byte */
static mut u_sbox: [[u8; 32]; 8] = [
  [
    0xei32 as u8,
    0xf4i32 as u8,
    0x7di32 as u8,
    0x41i32 as u8,
    0xe2i32 as u8,
    0x2fi32 as u8,
    0xdbi32 as u8,
    0x18i32 as u8,
    0xa3i32 as u8,
    0x6ai32 as u8,
    0xc6i32 as u8,
    0xbci32 as u8,
    0x95i32 as u8,
    0x59i32 as u8,
    0x30i32 as u8,
    0x87i32 as u8,
    0xf4i32 as u8,
    0xc1i32 as u8,
    0x8ei32 as u8,
    0x28i32 as u8,
    0x4di32 as u8,
    0x96i32 as u8,
    0x12i32 as u8,
    0x7bi32 as u8,
    0x5fi32 as u8,
    0xbci32 as u8,
    0x39i32 as u8,
    0xe7i32 as u8,
    0xa3i32 as u8,
    0xai32 as u8,
    0x65i32 as u8,
    0xd0i32 as u8,
  ],
  [
    0x3fi32 as u8,
    0xd1i32 as u8,
    0x48i32 as u8,
    0x7ei32 as u8,
    0xf6i32 as u8,
    0x2bi32 as u8,
    0x83i32 as u8,
    0xe4i32 as u8,
    0xc9i32 as u8,
    0x7i32 as u8,
    0x12i32 as u8,
    0xadi32 as u8,
    0x6ci32 as u8,
    0x90i32 as u8,
    0xb5i32 as u8,
    0x5ai32 as u8,
    0xd0i32 as u8,
    0x8ei32 as u8,
    0xa7i32 as u8,
    0x1bi32 as u8,
    0x3ai32 as u8,
    0xf4i32 as u8,
    0x4di32 as u8,
    0x21i32 as u8,
    0xb5i32 as u8,
    0x68i32 as u8,
    0x7ci32 as u8,
    0xc6i32 as u8,
    0x9i32 as u8,
    0x53i32 as u8,
    0xe2i32 as u8,
    0x9fi32 as u8,
  ],
  [
    0xdai32 as u8,
    0x70i32 as u8,
    0x9i32 as u8,
    0x9ei32 as u8,
    0x36i32 as u8,
    0x43i32 as u8,
    0x6fi32 as u8,
    0xa5i32 as u8,
    0x21i32 as u8,
    0x8di32 as u8,
    0x5ci32 as u8,
    0xe7i32 as u8,
    0xcbi32 as u8,
    0xb4i32 as u8,
    0xf2i32 as u8,
    0x18i32 as u8,
    0x1di32 as u8,
    0xa6i32 as u8,
    0xd4i32 as u8,
    0x9i32 as u8,
    0x68i32 as u8,
    0x9fi32 as u8,
    0x83i32 as u8,
    0x70i32 as u8,
    0x4bi32 as u8,
    0xf1i32 as u8,
    0xe2i32 as u8,
    0x3ci32 as u8,
    0xb5i32 as u8,
    0x5ai32 as u8,
    0x2ei32 as u8,
    0xc7i32 as u8,
  ],
  [
    0xd7i32 as u8,
    0x8di32 as u8,
    0xbei32 as u8,
    0x53i32 as u8,
    0x60i32 as u8,
    0xf6i32 as u8,
    0x9i32 as u8,
    0x3ai32 as u8,
    0x41i32 as u8,
    0x72i32 as u8,
    0x28i32 as u8,
    0xc5i32 as u8,
    0x1bi32 as u8,
    0xaci32 as u8,
    0xe4i32 as u8,
    0x9fi32 as u8,
    0x3ai32 as u8,
    0xf6i32 as u8,
    0x9i32 as u8,
    0x60i32 as u8,
    0xaci32 as u8,
    0x1bi32 as u8,
    0xd7i32 as u8,
    0x8di32 as u8,
    0x9fi32 as u8,
    0x41i32 as u8,
    0x53i32 as u8,
    0xbei32 as u8,
    0xc5i32 as u8,
    0x72i32 as u8,
    0x28i32 as u8,
    0xe4i32 as u8,
  ],
  [
    0xe2i32 as u8,
    0xbci32 as u8,
    0x24i32 as u8,
    0xc1i32 as u8,
    0x47i32 as u8,
    0x7ai32 as u8,
    0xdbi32 as u8,
    0x16i32 as u8,
    0x58i32 as u8,
    0x5i32 as u8,
    0xf3i32 as u8,
    0xafi32 as u8,
    0x3di32 as u8,
    0x90i32 as u8,
    0x8ei32 as u8,
    0x69i32 as u8,
    0xb4i32 as u8,
    0x82i32 as u8,
    0xc1i32 as u8,
    0x7bi32 as u8,
    0x1ai32 as u8,
    0xedi32 as u8,
    0x27i32 as u8,
    0xd8i32 as u8,
    0x6fi32 as u8,
    0xf9i32 as u8,
    0xci32 as u8,
    0x95i32 as u8,
    0xa6i32 as u8,
    0x43i32 as u8,
    0x50i32 as u8,
    0x3ei32 as u8,
  ],
  [
    0xaci32 as u8,
    0xf1i32 as u8,
    0x4ai32 as u8,
    0x2fi32 as u8,
    0x79i32 as u8,
    0xc2i32 as u8,
    0x96i32 as u8,
    0x58i32 as u8,
    0x60i32 as u8,
    0x1di32 as u8,
    0xd3i32 as u8,
    0xe4i32 as u8,
    0xei32 as u8,
    0xb7i32 as u8,
    0x35i32 as u8,
    0x8bi32 as u8,
    0x49i32 as u8,
    0x3ei32 as u8,
    0x2fi32 as u8,
    0xc5i32 as u8,
    0x92i32 as u8,
    0x58i32 as u8,
    0xfci32 as u8,
    0xa3i32 as u8,
    0xb7i32 as u8,
    0xe0i32 as u8,
    0x14i32 as u8,
    0x7ai32 as u8,
    0x61i32 as u8,
    0xdi32 as u8,
    0x8bi32 as u8,
    0xd6i32 as u8,
  ],
  [
    0xd4i32 as u8,
    0xbi32 as u8,
    0xb2i32 as u8,
    0x7ei32 as u8,
    0x4fi32 as u8,
    0x90i32 as u8,
    0x18i32 as u8,
    0xadi32 as u8,
    0xe3i32 as u8,
    0x3ci32 as u8,
    0x59i32 as u8,
    0xc7i32 as u8,
    0x25i32 as u8,
    0xfai32 as u8,
    0x86i32 as u8,
    0x61i32 as u8,
    0x61i32 as u8,
    0xb4i32 as u8,
    0xdbi32 as u8,
    0x8di32 as u8,
    0x1ci32 as u8,
    0x43i32 as u8,
    0xa7i32 as u8,
    0x7ei32 as u8,
    0x9ai32 as u8,
    0x5fi32 as u8,
    0x6i32 as u8,
    0xf8i32 as u8,
    0xe0i32 as u8,
    0x25i32 as u8,
    0x39i32 as u8,
    0xc2i32 as u8,
  ],
  [
    0x1di32 as u8,
    0xf2i32 as u8,
    0xd8i32 as u8,
    0x84i32 as u8,
    0xa6i32 as u8,
    0x3fi32 as u8,
    0x7bi32 as u8,
    0x41i32 as u8,
    0xcai32 as u8,
    0x59i32 as u8,
    0x63i32 as u8,
    0xbei32 as u8,
    0x5i32 as u8,
    0xe0i32 as u8,
    0x9ci32 as u8,
    0x27i32 as u8,
    0x27i32 as u8,
    0x1bi32 as u8,
    0xe4i32 as u8,
    0x71i32 as u8,
    0x49i32 as u8,
    0xaci32 as u8,
    0x8ei32 as u8,
    0xd2i32 as u8,
    0xf0i32 as u8,
    0xc6i32 as u8,
    0x9ai32 as u8,
    0xdi32 as u8,
    0x3fi32 as u8,
    0x53i32 as u8,
    0x65i32 as u8,
    0xb8i32 as u8,
  ],
];
static mut pbox: [u8; 32] = [
  16i32 as u8,
  7i32 as u8,
  20i32 as u8,
  21i32 as u8,
  29i32 as u8,
  12i32 as u8,
  28i32 as u8,
  17i32 as u8,
  1i32 as u8,
  15i32 as u8,
  23i32 as u8,
  26i32 as u8,
  5i32 as u8,
  18i32 as u8,
  31i32 as u8,
  10i32 as u8,
  2i32 as u8,
  8i32 as u8,
  24i32 as u8,
  14i32 as u8,
  32i32 as u8,
  27i32 as u8,
  3i32 as u8,
  9i32 as u8,
  19i32 as u8,
  13i32 as u8,
  30i32 as u8,
  6i32 as u8,
  22i32 as u8,
  11i32 as u8,
  4i32 as u8,
  25i32 as u8,
];
static mut bits32: [u32; 32] = [
  0x80000000u32,
  0x40000000i32 as u32,
  0x20000000i32 as u32,
  0x10000000i32 as u32,
  0x8000000i32 as u32,
  0x4000000i32 as u32,
  0x2000000i32 as u32,
  0x1000000i32 as u32,
  0x800000i32 as u32,
  0x400000i32 as u32,
  0x200000i32 as u32,
  0x100000i32 as u32,
  0x80000i32 as u32,
  0x40000i32 as u32,
  0x20000i32 as u32,
  0x10000i32 as u32,
  0x8000i32 as u32,
  0x4000i32 as u32,
  0x2000i32 as u32,
  0x1000i32 as u32,
  0x800i32 as u32,
  0x400i32 as u32,
  0x200i32 as u32,
  0x100i32 as u32,
  0x80i32 as u32,
  0x40i32 as u32,
  0x20i32 as u32,
  0x10i32 as u32,
  0x8i32 as u32,
  0x4i32 as u32,
  0x2i32 as u32,
  0x1i32 as u32,
];
static mut bits8: [u8; 8] = [
  0x80i32 as u8,
  0x40i32 as u8,
  0x20i32 as u8,
  0x10i32 as u8,
  0x8i32 as u8,
  0x4i32 as u8,
  0x2i32 as u8,
  0x1i32 as u8,
];
unsafe extern "C" fn ascii_to_bin(mut ch: libc::c_char) -> libc::c_int {
  if ch as libc::c_int > 'z' as i32 {
    return 0i32;
  }
  if ch as libc::c_int >= 'a' as i32 {
    return ch as libc::c_int - 'a' as i32 + 38i32;
  }
  if ch as libc::c_int > 'Z' as i32 {
    return 0i32;
  }
  if ch as libc::c_int >= 'A' as i32 {
    return ch as libc::c_int - 'A' as i32 + 12i32;
  }
  if ch as libc::c_int > '9' as i32 {
    return 0i32;
  }
  if ch as libc::c_int >= '.' as i32 {
    return ch as libc::c_int - '.' as i32;
  }
  return 0i32;
}
unsafe extern "C" fn const_des_init() -> *mut const_des_ctx {
  let mut i: libc::c_uint = 0;
  let mut j: libc::c_uint = 0;
  let mut b: libc::c_uint = 0;
  let mut cctx: *mut const_des_ctx = 0 as *mut const_des_ctx;
  cctx = xmalloc(::std::mem::size_of::<const_des_ctx>() as libc::c_ulong) as *mut const_des_ctx;
  /*
   * Convert the inverted S-boxes into 4 arrays of 8 bits.
   * Each will handle 12 bits of the S-box input.
   */
  b = 0i32 as libc::c_uint;
  while b < 4i32 as libc::c_uint {
    i = 0i32 as libc::c_uint;
    while i < 64i32 as libc::c_uint {
      j = 0i32 as libc::c_uint;
      while j < 64i32 as libc::c_uint {
        let mut lo: u8 = 0;
        let mut hi: u8 = 0;
        hi = u_sbox[(b << 1i32) as usize][i.wrapping_div(2i32 as libc::c_uint) as usize];
        if i & 1i32 as libc::c_uint == 0 {
          hi = ((hi as libc::c_int) << 4i32) as u8
        }
        lo = u_sbox[(b << 1i32).wrapping_add(1i32 as libc::c_uint) as usize]
          [j.wrapping_div(2i32 as libc::c_uint) as usize];
        if j & 1i32 as libc::c_uint != 0 {
          lo = (lo as libc::c_int >> 4i32) as u8
        }
        (*cctx).m_sbox[b as usize][(i << 6i32 | j) as usize] =
          (hi as libc::c_int & 0xf0i32 | lo as libc::c_int & 0xfi32) as u8;
        j = j.wrapping_add(1)
      }
      i = i.wrapping_add(1)
    }
    b = b.wrapping_add(1)
  }
  /*
   * Set up the initial & final permutations into a useful form.
   */
  i = 0i32 as libc::c_uint;
  while i < 64i32 as libc::c_uint {
    (*cctx).final_perm[i as usize] = (IP[i as usize] as libc::c_int - 1i32) as u8;
    i = i.wrapping_add(1)
  }
  return cctx;
}
unsafe extern "C" fn des_init(
  mut ctx: *mut des_ctx,
  mut cctx: *const const_des_ctx,
) -> *mut des_ctx {
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  let mut b: libc::c_int = 0;
  let mut k: libc::c_int = 0;
  let mut inbit: libc::c_int = 0;
  let mut obit: libc::c_int = 0;
  let mut p: u32 = 0;
  let mut bits28: *const u32 = 0 as *const u32;
  let mut bits24: *const u32 = 0 as *const u32;
  if ctx.is_null() {
    ctx = xmalloc(::std::mem::size_of::<des_ctx>() as libc::c_ulong) as *mut des_ctx
  }
  (*ctx).const_ctx = cctx;
  (*ctx).saltbits = 0i32 as u32;
  bits28 = bits32.as_ptr().offset(4);
  bits24 = bits28.offset(4);
  /* Initialise the inverted key permutation. */
  i = 0i32;
  while i < 64i32 {
    (*ctx).inv_key_perm[i as usize] = 255i32 as u8;
    i += 1
  }
  /*
   * Invert the key permutation and initialise the inverted key
   * compression permutation.
   */
  i = 0i32;
  while i < 56i32 {
    (*ctx).inv_key_perm[(key_perm[i as usize] as libc::c_int - 1i32) as usize] = i as u8;
    (*ctx).inv_comp_perm[i as usize] = 255i32 as u8;
    i += 1
  }
  /* Invert the key compression permutation. */
  i = 0i32;
  while i < 48i32 {
    (*ctx).inv_comp_perm[(comp_perm[i as usize] as libc::c_int - 1i32) as usize] = i as u8;
    i += 1
  }
  /*
   * Set up the OR-mask arrays for the initial and final permutations,
   * and for the key initial and compression permutations.
   */
  k = 0i32;
  while k < 8i32 {
    let mut il: u32 = 0;
    let mut ir: u32 = 0;
    let mut fl: u32 = 0;
    let mut fr: u32 = 0;
    i = 0i32;
    while i < 256i32 {
      fl = 0i32 as u32;
      fr = 0i32 as u32;
      j = 0i32;
      while j < 8i32 {
        inbit = 8i32 * k + j;
        if i & bits8[j as usize] as libc::c_int != 0 {
          obit = (*cctx).final_perm[inbit as usize] as libc::c_int;
          if obit < 32i32 {
            fl |= bits32[obit as usize]
          } else {
            fr |= bits32[(obit - 32i32) as usize]
          }
        }
        j += 1
      }
      (*ctx).fp_maskl[k as usize][i as usize] = fl;
      (*ctx).fp_maskr[k as usize][i as usize] = fr;
      i += 1
    }
    i = 0i32;
    while i < 128i32 {
      il = 0i32 as u32;
      ir = 0i32 as u32;
      j = 0i32;
      while j < 7i32 {
        inbit = 8i32 * k + j;
        if i & bits8[(j + 1i32) as usize] as libc::c_int != 0 {
          obit = (*ctx).inv_key_perm[inbit as usize] as libc::c_int;
          if !(obit == 255i32) {
            if obit < 28i32 {
              il |= *bits28.offset(obit as isize)
            } else {
              ir |= *bits28.offset((obit - 28i32) as isize)
            }
          }
        }
        j += 1
      }
      (*ctx).key_perm_maskl[k as usize][i as usize] = il;
      (*ctx).key_perm_maskr[k as usize][i as usize] = ir;
      il = 0i32 as u32;
      ir = 0i32 as u32;
      j = 0i32;
      while j < 7i32 {
        inbit = 7i32 * k + j;
        if i & bits8[(j + 1i32) as usize] as libc::c_int != 0 {
          obit = (*ctx).inv_comp_perm[inbit as usize] as libc::c_int;
          if !(obit == 255i32) {
            if obit < 24i32 {
              il |= *bits24.offset(obit as isize)
            } else {
              ir |= *bits24.offset((obit - 24i32) as isize)
            }
          }
        }
        j += 1
      }
      (*ctx).comp_maskl[k as usize][i as usize] = il;
      (*ctx).comp_maskr[k as usize][i as usize] = ir;
      i += 1
    }
    k += 1
  }
  /*
   * Invert the P-box permutation, and convert into OR-masks for
   * handling the output of the S-box arrays setup above.
   */
  i = 0i32;
  while i < 32i32 {
    (*ctx).un_pbox[(pbox[i as usize] as libc::c_int - 1i32) as usize] = i as u8;
    i += 1
  }
  b = 0i32;
  while b < 4i32 {
    i = 0i32;
    while i < 256i32 {
      p = 0i32 as u32;
      j = 0i32;
      while j < 8i32 {
        if i & bits8[j as usize] as libc::c_int != 0 {
          p |= bits32[(*ctx).un_pbox[(8i32 * b + j) as usize] as usize]
        }
        j += 1
      }
      (*ctx).psbox[b as usize][i as usize] = p;
      i += 1
    }
    b += 1
  }
  return ctx;
}
unsafe extern "C" fn setup_salt(mut ctx: *mut des_ctx, mut salt: u32) {
  let mut obit: u32 = 0;
  let mut saltbit: u32 = 0;
  let mut i: libc::c_int = 0;
  (*ctx).saltbits = 0i32 as u32;
  saltbit = 1i32 as u32;
  obit = 0x800000i32 as u32;
  i = 0i32;
  while i < 24i32 {
    if salt & saltbit != 0 {
      (*ctx).saltbits |= obit
    }
    saltbit <<= 1i32;
    obit >>= 1i32;
    i += 1
  }
}
unsafe extern "C" fn des_setkey(mut ctx: *mut des_ctx, mut key: *const libc::c_char) {
  let mut k0: u32 = 0;
  let mut k1: u32 = 0;
  let mut rawkey0: u32 = 0;
  let mut rawkey1: u32 = 0;
  let mut shifts: libc::c_int = 0;
  let mut round: libc::c_int = 0;
  rawkey0 = {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = *(key as *const u32);
    if false {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh6 = &mut __v;
      let fresh7;
      let fresh8 = __x;
      asm!("bswap $0" : "=r" (fresh7) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh6, fresh8)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
    }
    __v
  };
  rawkey1 = {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = *(key.offset(4) as *const u32);
    if false {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh9 = &mut __v;
      let fresh10;
      let fresh11 = __x;
      asm!("bswap $0" : "=r" (fresh10) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh9, fresh11))
                      :);
      c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
    }
    __v
  };
  /*
   * Do key permutation and split into two 28-bit subkeys.
   */
  k0 = (*ctx).key_perm_maskl[0][(rawkey0 >> 25i32) as usize]
    | (*ctx).key_perm_maskl[1][(rawkey0 >> 17i32 & 0x7fi32 as libc::c_uint) as usize]
    | (*ctx).key_perm_maskl[2][(rawkey0 >> 9i32 & 0x7fi32 as libc::c_uint) as usize]
    | (*ctx).key_perm_maskl[3][(rawkey0 >> 1i32 & 0x7fi32 as libc::c_uint) as usize]
    | (*ctx).key_perm_maskl[4][(rawkey1 >> 25i32) as usize]
    | (*ctx).key_perm_maskl[5][(rawkey1 >> 17i32 & 0x7fi32 as libc::c_uint) as usize]
    | (*ctx).key_perm_maskl[6][(rawkey1 >> 9i32 & 0x7fi32 as libc::c_uint) as usize]
    | (*ctx).key_perm_maskl[7][(rawkey1 >> 1i32 & 0x7fi32 as libc::c_uint) as usize];
  k1 = (*ctx).key_perm_maskr[0][(rawkey0 >> 25i32) as usize]
    | (*ctx).key_perm_maskr[1][(rawkey0 >> 17i32 & 0x7fi32 as libc::c_uint) as usize]
    | (*ctx).key_perm_maskr[2][(rawkey0 >> 9i32 & 0x7fi32 as libc::c_uint) as usize]
    | (*ctx).key_perm_maskr[3][(rawkey0 >> 1i32 & 0x7fi32 as libc::c_uint) as usize]
    | (*ctx).key_perm_maskr[4][(rawkey1 >> 25i32) as usize]
    | (*ctx).key_perm_maskr[5][(rawkey1 >> 17i32 & 0x7fi32 as libc::c_uint) as usize]
    | (*ctx).key_perm_maskr[6][(rawkey1 >> 9i32 & 0x7fi32 as libc::c_uint) as usize]
    | (*ctx).key_perm_maskr[7][(rawkey1 >> 1i32 & 0x7fi32 as libc::c_uint) as usize];
  /*
   * Rotate subkeys and do compression permutation.
   */
  shifts = 0i32;
  round = 0i32;
  while round < 16i32 {
    let mut t0: u32 = 0;
    let mut t1: u32 = 0;
    shifts += key_shifts[round as usize] as libc::c_int;
    t0 = k0 << shifts | k0 >> 28i32 - shifts;
    t1 = k1 << shifts | k1 >> 28i32 - shifts;
    (*ctx).en_keysl[round as usize] = (*ctx).comp_maskl[0]
      [(t0 >> 21i32 & 0x7fi32 as libc::c_uint) as usize]
      | (*ctx).comp_maskl[1][(t0 >> 14i32 & 0x7fi32 as libc::c_uint) as usize]
      | (*ctx).comp_maskl[2][(t0 >> 7i32 & 0x7fi32 as libc::c_uint) as usize]
      | (*ctx).comp_maskl[3][(t0 & 0x7fi32 as libc::c_uint) as usize]
      | (*ctx).comp_maskl[4][(t1 >> 21i32 & 0x7fi32 as libc::c_uint) as usize]
      | (*ctx).comp_maskl[5][(t1 >> 14i32 & 0x7fi32 as libc::c_uint) as usize]
      | (*ctx).comp_maskl[6][(t1 >> 7i32 & 0x7fi32 as libc::c_uint) as usize]
      | (*ctx).comp_maskl[7][(t1 & 0x7fi32 as libc::c_uint) as usize];
    (*ctx).en_keysr[round as usize] = (*ctx).comp_maskr[0]
      [(t0 >> 21i32 & 0x7fi32 as libc::c_uint) as usize]
      | (*ctx).comp_maskr[1][(t0 >> 14i32 & 0x7fi32 as libc::c_uint) as usize]
      | (*ctx).comp_maskr[2][(t0 >> 7i32 & 0x7fi32 as libc::c_uint) as usize]
      | (*ctx).comp_maskr[3][(t0 & 0x7fi32 as libc::c_uint) as usize]
      | (*ctx).comp_maskr[4][(t1 >> 21i32 & 0x7fi32 as libc::c_uint) as usize]
      | (*ctx).comp_maskr[5][(t1 >> 14i32 & 0x7fi32 as libc::c_uint) as usize]
      | (*ctx).comp_maskr[6][(t1 >> 7i32 & 0x7fi32 as libc::c_uint) as usize]
      | (*ctx).comp_maskr[7][(t1 & 0x7fi32 as libc::c_uint) as usize];
    round += 1
  }
}
unsafe extern "C" fn do_des(
  mut ctx: *mut des_ctx,
  mut l_out: *mut u32,
  mut r_out: *mut u32,
  mut count: libc::c_int,
) {
  let mut cctx: *const const_des_ctx = (*ctx).const_ctx;
  /*
   * l_in, r_in, l_out, and r_out are in pseudo-"big-endian" format.
   */
  let mut l: u32 = 0; /* silence gcc */
  let mut r: u32 = 0;
  let mut kl: *mut u32 = 0 as *mut u32;
  let mut kr: *mut u32 = 0 as *mut u32;
  let mut f: u32 = 0;
  f = f;
  let mut r48l: u32 = 0;
  let mut r48r: u32 = 0;
  let mut round: libc::c_int = 0;
  /* Do initial permutation (IP). */
  /* -65 bytes (using the fact that l_in == r_in == 0) */
  /* using the fact that ip_maskX[] is constant (written to by des_init) */
  r = 0i32 as u32;
  l = r;
  loop {
    /* Do each round. */
    kl = (*ctx).en_keysl.as_mut_ptr();
    kr = (*ctx).en_keysr.as_mut_ptr();
    round = 16i32;
    loop {
      /* Expand R to 48 bits (simulate the E-box). */
      r48l = (r & 0x1i32 as libc::c_uint) << 23i32
        | (r & 0xf8000000u32) >> 9i32
        | (r & 0x1f800000i32 as libc::c_uint) >> 11i32
        | (r & 0x1f80000i32 as libc::c_uint) >> 13i32
        | (r & 0x1f8000i32 as libc::c_uint) >> 15i32;
      r48r = (r & 0x1f800i32 as libc::c_uint) << 7i32
        | (r & 0x1f80i32 as libc::c_uint) << 5i32
        | (r & 0x1f8i32 as libc::c_uint) << 3i32
        | (r & 0x1fi32 as libc::c_uint) << 1i32
        | (r & 0x80000000u32) >> 31i32;
      /*
       * Do salting for crypt() and friends, and
       * XOR with the permuted key.
       */
      f = (r48l ^ r48r) & (*ctx).saltbits;
      let fresh12 = kl;
      kl = kl.offset(1);
      r48l ^= f ^ *fresh12;
      let fresh13 = kr;
      kr = kr.offset(1);
      r48r ^= f ^ *fresh13;
      /*
       * Do sbox lookups (which shrink it back to 32 bits)
       * and do the pbox permutation at the same time.
       */
      f = (*ctx).psbox[0][(*cctx).m_sbox[0][(r48l >> 12i32) as usize] as usize]
        | (*ctx).psbox[1][(*cctx).m_sbox[1][(r48l & 0xfffi32 as libc::c_uint) as usize] as usize]
        | (*ctx).psbox[2][(*cctx).m_sbox[2][(r48r >> 12i32) as usize] as usize]
        | (*ctx).psbox[3][(*cctx).m_sbox[3][(r48r & 0xfffi32 as libc::c_uint) as usize] as usize];
      /* Now that we've permuted things, complete f(). */
      f ^= l;
      l = r;
      r = f;
      round -= 1;
      if !(round != 0) {
        break;
      }
    }
    r = l;
    l = f;
    count -= 1;
    if !(count != 0) {
      break;
    }
  }
  /* Do final permutation (inverse of IP). */
  *l_out = (*ctx).fp_maskl[0][(l >> 24i32) as usize]
    | (*ctx).fp_maskl[1][(l >> 16i32 & 0xffi32 as libc::c_uint) as usize]
    | (*ctx).fp_maskl[2][(l >> 8i32 & 0xffi32 as libc::c_uint) as usize]
    | (*ctx).fp_maskl[3][(l & 0xffi32 as libc::c_uint) as usize]
    | (*ctx).fp_maskl[4][(r >> 24i32) as usize]
    | (*ctx).fp_maskl[5][(r >> 16i32 & 0xffi32 as libc::c_uint) as usize]
    | (*ctx).fp_maskl[6][(r >> 8i32 & 0xffi32 as libc::c_uint) as usize]
    | (*ctx).fp_maskl[7][(r & 0xffi32 as libc::c_uint) as usize]; /* bits 23..18 */
  *r_out = (*ctx).fp_maskr[0][(l >> 24i32) as usize]
    | (*ctx).fp_maskr[1][(l >> 16i32 & 0xffi32 as libc::c_uint) as usize]
    | (*ctx).fp_maskr[2][(l >> 8i32 & 0xffi32 as libc::c_uint) as usize]
    | (*ctx).fp_maskr[3][(l & 0xffi32 as libc::c_uint) as usize]
    | (*ctx).fp_maskr[4][(r >> 24i32) as usize]
    | (*ctx).fp_maskr[5][(r >> 16i32 & 0xffi32 as libc::c_uint) as usize]
    | (*ctx).fp_maskr[6][(r >> 8i32 & 0xffi32 as libc::c_uint) as usize]
    | (*ctx).fp_maskr[7][(r & 0xffi32 as libc::c_uint) as usize]; /* bits 17..12 */
}
unsafe extern "C" fn to64_msb_first(mut s: *mut libc::c_char, mut v: libc::c_uint) {
  let fresh14 = s; /* bits 11..6 */
  s = s.offset(1);
  *fresh14 = i64c((v >> 18i32) as libc::c_int) as libc::c_char;
  let fresh15 = s;
  s = s.offset(1);
  *fresh15 = i64c((v >> 12i32) as libc::c_int) as libc::c_char;
  let fresh16 = s;
  s = s.offset(1);
  *fresh16 = i64c((v >> 6i32) as libc::c_int) as libc::c_char;
  *s = i64c(v as libc::c_int) as libc::c_char;
  /* bits 5..0 */
}
#[inline(never)]
unsafe extern "C" fn des_crypt(
  mut ctx: *mut des_ctx,
  mut output: *mut libc::c_char,
  mut key: *const libc::c_uchar,
  mut setting: *const libc::c_uchar,
) -> *mut libc::c_char {
  let mut salt: u32 = 0;
  let mut r0: u32 = 0;
  let mut r1: u32 = 0;
  let mut keybuf: [u32; 2] = [0; 2];
  let mut q: *mut u8 = 0 as *mut u8;
  /*
   * Copy the key, shifting each character up by one bit
   * and padding with zeros.
   */
  q = keybuf.as_mut_ptr() as *mut u8;
  while q.wrapping_offset_from(keybuf.as_mut_ptr() as *mut u8) as libc::c_long
    != 8i32 as libc::c_long
  {
    *q = ((*key as libc::c_int) << 1i32) as u8;
    if *q != 0 {
      key = key.offset(1)
    }
    q = q.offset(1)
  }
  des_setkey(ctx, keybuf.as_mut_ptr() as *mut libc::c_char);
  /*
   * setting - 2 bytes of salt
   * key - up to 8 characters
   */
  salt = (ascii_to_bin(*setting.offset(1) as libc::c_char) << 6i32
    | ascii_to_bin(*setting.offset(0) as libc::c_char)) as u32;
  *output.offset(0) = *setting.offset(0) as libc::c_char;
  /*
   * If the encrypted password that the salt was extracted from
   * is only 1 character long, the salt will be corrupted.  We
   * need to ensure that the output string doesn't have an extra
   * NUL in it!
   */
  *output.offset(1) = if *setting.offset(1) as libc::c_int != 0 {
    *setting.offset(1) as libc::c_int
  } else {
    *output.offset(0) as libc::c_int
  } as libc::c_char;
  setup_salt(ctx, salt);
  /* Do it. */
  do_des(ctx, &mut r0, &mut r1, 25i32);
  /* Now encode the result. */
  /* Each call takes low-order 24 bits and stores 4 chars */
  /* bits 31..8 of r0 */
  to64_msb_first(output.offset(2), r0 >> 8i32);
  /* bits 7..0 of r0 and 31..16 of r1 */
  to64_msb_first(output.offset(6), r0 << 16i32 | r1 >> 16i32);
  /* bits 15..0 of r1 and two zero bits (plus extra zero byte) */
  to64_msb_first(output.offset(10), r1 << 8i32);
  /* extra zero byte is encoded as '.', fixing it */
  *output.offset(13) = '\u{0}' as i32 as libc::c_char; /* final[16] exists only to aid in looping */
  return output;
}
#[inline(never)]
unsafe extern "C" fn md5_crypt(
  mut result: *mut libc::c_char,
  mut pw: *const libc::c_uchar,
  mut salt: *const libc::c_uchar,
) -> *mut libc::c_char {
  let mut p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut final_0: [libc::c_uchar; 17] = [0; 17];
  let mut sl: libc::c_int = 0;
  let mut pl: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  let mut pw_len: libc::c_int = 0;
  let mut ctx: md5_ctx_t = md5_ctx_t {
    wbuffer: [0; 64],
    process_block: None,
    total64: 0,
    hash: [0; 8],
  };
  let mut ctx1: md5_ctx_t = md5_ctx_t {
    wbuffer: [0; 64],
    process_block: None,
    total64: 0,
    hash: [0; 8],
  };
  /* NB: in busybox, "$1$" in salt is always present */
  /* Refine the Salt first */
  /* Get the length of the salt including "$1$" */
  sl = 3i32;
  while sl < 3i32 + 8i32
    && *salt.offset(sl as isize) as libc::c_int != 0
    && *salt.offset(sl as isize) as libc::c_int != '$' as i32
  {
    sl += 1
  }
  /* Hash. the password first, since that is what is most unknown */
  md5_begin(&mut ctx);
  pw_len = strlen(pw as *mut libc::c_char) as libc::c_int;
  md5_hash(&mut ctx, pw as *const libc::c_void, pw_len as size_t);
  /* Then the salt including "$1$" */
  md5_hash(&mut ctx, salt as *const libc::c_void, sl as size_t);
  /* Copy salt to result; skip "$1$" */
  memcpy(
    result as *mut libc::c_void,
    salt as *const libc::c_void,
    sl as libc::c_ulong,
  );
  *result.offset(sl as isize) = '$' as i32 as libc::c_char;
  salt = salt.offset(3);
  sl -= 3i32;
  /* Then just as many characters of the MD5(pw, salt, pw) */
  md5_begin(&mut ctx1);
  md5_hash(&mut ctx1, pw as *const libc::c_void, pw_len as size_t);
  md5_hash(&mut ctx1, salt as *const libc::c_void, sl as size_t);
  md5_hash(&mut ctx1, pw as *const libc::c_void, pw_len as size_t);
  md5_end(&mut ctx1, final_0.as_mut_ptr() as *mut libc::c_void);
  pl = pw_len;
  while pl > 0i32 {
    md5_hash(
      &mut ctx,
      final_0.as_mut_ptr() as *const libc::c_void,
      if pl > 16i32 { 16i32 } else { pl } as size_t,
    );
    pl -= 16i32
  }
  /* Then something really weird... */
  memset(
    final_0.as_mut_ptr() as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<[libc::c_uchar; 17]>() as libc::c_ulong,
  );
  i = pw_len;
  while i != 0 {
    md5_hash(
      &mut ctx,
      if i & 1i32 != 0 {
        final_0.as_mut_ptr()
      } else {
        pw
      } as *const libc::c_void,
      1i32 as size_t,
    );
    i >>= 1i32
  }
  md5_end(&mut ctx, final_0.as_mut_ptr() as *mut libc::c_void);
  /* And now, just to make sure things don't run too fast.
   * On a 60 Mhz Pentium this takes 34 msec, so you would
   * need 30 seconds to build a 1000 entry dictionary...
   */
  i = 0i32; /* 12 bytes max (sl is up to 8 bytes) */
  while i < 1000i32 {
    md5_begin(&mut ctx1);
    if i & 1i32 != 0 {
      md5_hash(&mut ctx1, pw as *const libc::c_void, pw_len as size_t);
    } else {
      md5_hash(
        &mut ctx1,
        final_0.as_mut_ptr() as *const libc::c_void,
        16i32 as size_t,
      );
    }
    if i % 3i32 != 0 {
      md5_hash(&mut ctx1, salt as *const libc::c_void, sl as size_t);
    }
    if i % 7i32 != 0 {
      md5_hash(&mut ctx1, pw as *const libc::c_void, pw_len as size_t);
    }
    if i & 1i32 != 0 {
      md5_hash(
        &mut ctx1,
        final_0.as_mut_ptr() as *const libc::c_void,
        16i32 as size_t,
      );
    } else {
      md5_hash(&mut ctx1, pw as *const libc::c_void, pw_len as size_t);
    }
    md5_end(&mut ctx1, final_0.as_mut_ptr() as *mut libc::c_void);
    i += 1
  }
  p = result.offset(sl as isize).offset(4);
  /* Add 5*4+2 = 22 bytes of hash, + NUL byte. */
  final_0[16] = final_0[5];
  i = 0i32;
  while i < 5i32 {
    let mut l: libc::c_uint = ((final_0[i as usize] as libc::c_int) << 16i32
      | (final_0[(i + 6i32) as usize] as libc::c_int) << 8i32
      | final_0[(i + 12i32) as usize] as libc::c_int) as libc::c_uint;
    p = to64(p, l, 4i32);
    i += 1
  }
  p = to64(p, final_0[11] as libc::c_uint, 2i32);
  *p = '\u{0}' as i32 as libc::c_char;
  /* Don't leave anything around in vm they could use. */
  memset(
    final_0.as_mut_ptr() as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<[libc::c_uchar; 17]>() as libc::c_ulong,
  );
  return result;
}
/* SHA256 and SHA512-based Unix crypt implementation.
 * Released into the Public Domain by Ulrich Drepper <drepper@redhat.com>.
 */
/* Prefix for optional rounds specification.  */
static mut str_rounds: [libc::c_char; 11] = [114, 111, 117, 110, 100, 115, 61, 37, 117, 36, 0];
#[inline(never)]
unsafe extern "C" fn sha_crypt(
  mut key_data: *mut libc::c_char,
  mut salt_data: *mut libc::c_char,
) -> *mut libc::c_char {
  let mut sha_begin: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()> = None;
  let mut sha_hash: Option<
    unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_void, _: size_t) -> (),
  > = None;
  let mut sha_end: Option<
    unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void) -> libc::c_uint,
  > = None;
  let mut _32or64: libc::c_int = 0;
  let mut result: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut resptr: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  /* btw, sha256 needs [32] and u32 only */
  let mut L: C2RustUnnamed = C2RustUnnamed {
    alt_result: [0; 64],
    temp_result: [0; 64],
    ctx: C2RustUnnamed_1 {
      x: md5_ctx_t {
        wbuffer: [0; 64],
        process_block: None,
        total64: 0,
        hash: [0; 8],
      },
    },
    alt_ctx: C2RustUnnamed_0 {
      x: md5_ctx_t {
        wbuffer: [0; 64],
        process_block: None,
        total64: 0,
        hash: [0; 8],
      },
    },
  };
  let mut salt_len: libc::c_uint = 0;
  let mut key_len: libc::c_uint = 0;
  let mut cnt: libc::c_uint = 0;
  let mut rounds: libc::c_uint = 0;
  let mut cp: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  /* Analyze salt, construct already known part of result */
  cnt = strlen(salt_data)
    .wrapping_add(1i32 as libc::c_ulong)
    .wrapping_add(43i32 as libc::c_ulong)
    .wrapping_add(1i32 as libc::c_ulong) as libc::c_uint;
  _32or64 = 32i32;
  if *salt_data.offset(1) as libc::c_int == '6' as i32 {
    /* sha512 */
    _32or64 *= 2i32; /*64*/
    cnt = cnt.wrapping_add(43i32 as libc::c_uint)
  } /* will provide NUL terminator */
  resptr = xzalloc(cnt as size_t) as *mut libc::c_char;
  result = resptr;
  let fresh17 = resptr;
  resptr = resptr.offset(1);
  *fresh17 = '$' as i32 as libc::c_char;
  let fresh18 = resptr;
  resptr = resptr.offset(1);
  *fresh18 = *salt_data.offset(1);
  let fresh19 = resptr;
  resptr = resptr.offset(1);
  *fresh19 = '$' as i32 as libc::c_char;
  rounds = 5000i32 as libc::c_uint;
  salt_data = salt_data.offset(3);
  if strncmp(salt_data, str_rounds.as_ptr(), 7i32 as libc::c_ulong) == 0i32 {
    /* 7 == strlen("rounds=") */
    let mut endp: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    cnt = bb_strtou(salt_data.offset(7), &mut endp, 10i32);
    if *endp as libc::c_int == '$' as i32 {
      salt_data = endp.offset(1);
      rounds = cnt;
      if rounds < 1000i32 as libc::c_uint {
        rounds = 1000i32 as libc::c_uint
      }
      if rounds > 999999999i32 as libc::c_uint {
        rounds = 999999999i32 as libc::c_uint
      }
      /* add "rounds=NNNNN$" to result */
      resptr = resptr.offset(sprintf(resptr, str_rounds.as_ptr(), rounds) as isize)
    }
  }
  salt_len = strchrnul(salt_data, '$' as i32).wrapping_offset_from(salt_data) as libc::c_long
    as libc::c_uint;
  if salt_len > 16i32 as libc::c_uint {
    salt_len = 16i32 as libc::c_uint
  }
  /* xstrdup assures suitable alignment; also we will use it
  as a scratch space later. */
  salt_data = xstrndup(salt_data, salt_len as libc::c_int);
  /* add "salt$" to result */
  strcpy(resptr, salt_data);
  resptr = resptr.offset(salt_len as isize);
  let fresh20 = resptr;
  resptr = resptr.offset(1);
  *fresh20 = '$' as i32 as libc::c_char;
  /* key data doesn't need much processing */
  key_len = strlen(key_data) as libc::c_uint;
  key_data = xstrdup(key_data);
  /* Which flavor of SHAnnn ops to use? */
  sha_begin = ::std::mem::transmute::<
    *mut libc::c_void,
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
  >(::std::mem::transmute::<
    Option<unsafe extern "C" fn(_: *mut sha256_ctx_t) -> ()>,
    *mut libc::c_void,
  >(Some(
    sha256_begin as unsafe extern "C" fn(_: *mut sha256_ctx_t) -> (),
  )));
  sha_hash = ::std::mem::transmute::<
    *mut libc::c_void,
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_void, _: size_t) -> ()>,
  >(::std::mem::transmute::<
    Option<unsafe extern "C" fn(_: *mut md5_ctx_t, _: *const libc::c_void, _: size_t) -> ()>,
    *mut libc::c_void,
  >(Some(
    md5_hash as unsafe extern "C" fn(_: *mut md5_ctx_t, _: *const libc::c_void, _: size_t) -> (),
  )));
  sha_end = ::std::mem::transmute::<
    *mut libc::c_void,
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void) -> libc::c_uint>,
  >(::std::mem::transmute::<
    Option<unsafe extern "C" fn(_: *mut sha1_ctx_t, _: *mut libc::c_void) -> libc::c_uint>,
    *mut libc::c_void,
  >(Some(
    sha1_end as unsafe extern "C" fn(_: *mut sha1_ctx_t, _: *mut libc::c_void) -> libc::c_uint,
  )));
  if _32or64 != 32i32 {
    sha_begin = ::std::mem::transmute::<
      *mut libc::c_void,
      Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    >(::std::mem::transmute::<
      Option<unsafe extern "C" fn(_: *mut sha512_ctx_t) -> ()>,
      *mut libc::c_void,
    >(Some(
      sha512_begin as unsafe extern "C" fn(_: *mut sha512_ctx_t) -> (),
    )));
    sha_hash = ::std::mem::transmute::<
      *mut libc::c_void,
      Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_void, _: size_t) -> ()>,
    >(::std::mem::transmute::<
      Option<unsafe extern "C" fn(_: *mut sha512_ctx_t, _: *const libc::c_void, _: size_t) -> ()>,
      *mut libc::c_void,
    >(Some(
      sha512_hash
        as unsafe extern "C" fn(_: *mut sha512_ctx_t, _: *const libc::c_void, _: size_t) -> (),
    )));
    sha_end = ::std::mem::transmute::<
      *mut libc::c_void,
      Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void) -> libc::c_uint>,
    >(::std::mem::transmute::<
      Option<unsafe extern "C" fn(_: *mut sha512_ctx_t, _: *mut libc::c_void) -> libc::c_uint>,
      *mut libc::c_void,
    >(Some(
      sha512_end
        as unsafe extern "C" fn(_: *mut sha512_ctx_t, _: *mut libc::c_void) -> libc::c_uint,
    )))
  }
  /* Add KEY, SALT.  */
  sha_begin.expect("non-null function pointer")(
    &mut L.ctx as *mut C2RustUnnamed_1 as *mut libc::c_void,
  );
  sha_hash.expect("non-null function pointer")(
    &mut L.ctx as *mut C2RustUnnamed_1 as *mut libc::c_void,
    key_data as *const libc::c_void,
    key_len as size_t,
  );
  sha_hash.expect("non-null function pointer")(
    &mut L.ctx as *mut C2RustUnnamed_1 as *mut libc::c_void,
    salt_data as *const libc::c_void,
    salt_len as size_t,
  );
  /* Compute alternate SHA sum with input KEY, SALT, and KEY.
  The final result will be added to the first context.  */
  sha_begin.expect("non-null function pointer")(
    &mut L.alt_ctx as *mut C2RustUnnamed_0 as *mut libc::c_void,
  );
  sha_hash.expect("non-null function pointer")(
    &mut L.alt_ctx as *mut C2RustUnnamed_0 as *mut libc::c_void,
    key_data as *const libc::c_void,
    key_len as size_t,
  );
  sha_hash.expect("non-null function pointer")(
    &mut L.alt_ctx as *mut C2RustUnnamed_0 as *mut libc::c_void,
    salt_data as *const libc::c_void,
    salt_len as size_t,
  );
  sha_hash.expect("non-null function pointer")(
    &mut L.alt_ctx as *mut C2RustUnnamed_0 as *mut libc::c_void,
    key_data as *const libc::c_void,
    key_len as size_t,
  );
  sha_end.expect("non-null function pointer")(
    &mut L.alt_ctx as *mut C2RustUnnamed_0 as *mut libc::c_void,
    L.alt_result.as_mut_ptr() as *mut libc::c_void,
  );
  /* Add result of this to the other context.  */
  /* Add for any character in the key one byte of the alternate sum.  */
  cnt = key_len;
  while cnt > _32or64 as libc::c_uint {
    sha_hash.expect("non-null function pointer")(
      &mut L.ctx as *mut C2RustUnnamed_1 as *mut libc::c_void,
      L.alt_result.as_mut_ptr() as *const libc::c_void,
      _32or64 as size_t,
    );
    cnt = cnt.wrapping_sub(_32or64 as libc::c_uint)
  }
  sha_hash.expect("non-null function pointer")(
    &mut L.ctx as *mut C2RustUnnamed_1 as *mut libc::c_void,
    L.alt_result.as_mut_ptr() as *const libc::c_void,
    cnt as size_t,
  );
  /* Take the binary representation of the length of the key and for every
  1 add the alternate sum, for every 0 the key.  */
  cnt = key_len;
  while cnt != 0i32 as libc::c_uint {
    if cnt & 1i32 as libc::c_uint != 0i32 as libc::c_uint {
      sha_hash.expect("non-null function pointer")(
        &mut L.ctx as *mut C2RustUnnamed_1 as *mut libc::c_void,
        L.alt_result.as_mut_ptr() as *const libc::c_void,
        _32or64 as size_t,
      );
    } else {
      sha_hash.expect("non-null function pointer")(
        &mut L.ctx as *mut C2RustUnnamed_1 as *mut libc::c_void,
        key_data as *const libc::c_void,
        key_len as size_t,
      );
    }
    cnt >>= 1i32
  }
  /* Create intermediate result.  */
  sha_end.expect("non-null function pointer")(
    &mut L.ctx as *mut C2RustUnnamed_1 as *mut libc::c_void,
    L.alt_result.as_mut_ptr() as *mut libc::c_void,
  );
  /* Start computation of P byte sequence.  */
  /* For every character in the password add the entire password.  */
  sha_begin.expect("non-null function pointer")(
    &mut L.alt_ctx as *mut C2RustUnnamed_0 as *mut libc::c_void,
  );
  cnt = 0i32 as libc::c_uint;
  while cnt < key_len {
    sha_hash.expect("non-null function pointer")(
      &mut L.alt_ctx as *mut C2RustUnnamed_0 as *mut libc::c_void,
      key_data as *const libc::c_void,
      key_len as size_t,
    );
    cnt = cnt.wrapping_add(1)
  }
  sha_end.expect("non-null function pointer")(
    &mut L.alt_ctx as *mut C2RustUnnamed_0 as *mut libc::c_void,
    L.temp_result.as_mut_ptr() as *mut libc::c_void,
  );
  /* NB: past this point, raw key_data is not used anymore */
  /* Create byte sequence P.  */
  /* reuse the buffer as it is of the key_len size */
  cp = key_data; /* was: ... = alloca(key_len); */
  cnt = key_len;
  while cnt >= _32or64 as libc::c_uint {
    cp = memcpy(
      cp as *mut libc::c_void,
      L.temp_result.as_mut_ptr() as *const libc::c_void,
      _32or64 as libc::c_ulong,
    ) as *mut libc::c_char;
    cp = cp.offset(_32or64 as isize);
    cnt = cnt.wrapping_sub(_32or64 as libc::c_uint)
  }
  memcpy(
    cp as *mut libc::c_void,
    L.temp_result.as_mut_ptr() as *const libc::c_void,
    cnt as libc::c_ulong,
  );
  /* Start computation of S byte sequence.  */
  /* For every character in the password add the entire password.  */
  sha_begin.expect("non-null function pointer")(
    &mut L.alt_ctx as *mut C2RustUnnamed_0 as *mut libc::c_void,
  );
  cnt = 0i32 as libc::c_uint;
  while cnt < (16i32 + L.alt_result[0] as libc::c_int) as libc::c_uint {
    sha_hash.expect("non-null function pointer")(
      &mut L.alt_ctx as *mut C2RustUnnamed_0 as *mut libc::c_void,
      salt_data as *const libc::c_void,
      salt_len as size_t,
    );
    cnt = cnt.wrapping_add(1)
  }
  sha_end.expect("non-null function pointer")(
    &mut L.alt_ctx as *mut C2RustUnnamed_0 as *mut libc::c_void,
    L.temp_result.as_mut_ptr() as *mut libc::c_void,
  );
  /* NB: past this point, raw salt_data is not used anymore */
  /* Create byte sequence S.  */
  /* reuse the buffer as it is of the salt_len size */
  cp = salt_data; /* was: ... = alloca(salt_len); */
  cnt = salt_len;
  while cnt >= _32or64 as libc::c_uint {
    cp = memcpy(
      cp as *mut libc::c_void,
      L.temp_result.as_mut_ptr() as *const libc::c_void,
      _32or64 as libc::c_ulong,
    ) as *mut libc::c_char;
    cp = cp.offset(_32or64 as isize);
    cnt = cnt.wrapping_sub(_32or64 as libc::c_uint)
  }
  memcpy(
    cp as *mut libc::c_void,
    L.temp_result.as_mut_ptr() as *const libc::c_void,
    cnt as libc::c_ulong,
  );
  /* Repeatedly run the collected hash value through SHA to burn
  CPU cycles.  */
  cnt = 0i32 as libc::c_uint;
  while cnt < rounds {
    sha_begin.expect("non-null function pointer")(
      &mut L.ctx as *mut C2RustUnnamed_1 as *mut libc::c_void,
    );
    /* Add key or last result.  */
    if cnt & 1i32 as libc::c_uint != 0i32 as libc::c_uint {
      sha_hash.expect("non-null function pointer")(
        &mut L.ctx as *mut C2RustUnnamed_1 as *mut libc::c_void,
        key_data as *const libc::c_void,
        key_len as size_t,
      );
    } else {
      sha_hash.expect("non-null function pointer")(
        &mut L.ctx as *mut C2RustUnnamed_1 as *mut libc::c_void,
        L.alt_result.as_mut_ptr() as *const libc::c_void,
        _32or64 as size_t,
      );
    }
    /* Add salt for numbers not divisible by 3.  */
    if cnt.wrapping_rem(3i32 as libc::c_uint) != 0i32 as libc::c_uint {
      sha_hash.expect("non-null function pointer")(
        &mut L.ctx as *mut C2RustUnnamed_1 as *mut libc::c_void,
        salt_data as *const libc::c_void,
        salt_len as size_t,
      );
    }
    /* Add key for numbers not divisible by 7.  */
    if cnt.wrapping_rem(7i32 as libc::c_uint) != 0i32 as libc::c_uint {
      sha_hash.expect("non-null function pointer")(
        &mut L.ctx as *mut C2RustUnnamed_1 as *mut libc::c_void,
        key_data as *const libc::c_void,
        key_len as size_t,
      );
    }
    /* Add key or last result.  */
    if cnt & 1i32 as libc::c_uint != 0i32 as libc::c_uint {
      sha_hash.expect("non-null function pointer")(
        &mut L.ctx as *mut C2RustUnnamed_1 as *mut libc::c_void,
        L.alt_result.as_mut_ptr() as *const libc::c_void,
        _32or64 as size_t,
      );
    } else {
      sha_hash.expect("non-null function pointer")(
        &mut L.ctx as *mut C2RustUnnamed_1 as *mut libc::c_void,
        key_data as *const libc::c_void,
        key_len as size_t,
      );
    }
    sha_end.expect("non-null function pointer")(
      &mut L.ctx as *mut C2RustUnnamed_1 as *mut libc::c_void,
      L.alt_result.as_mut_ptr() as *mut libc::c_void,
    );
    cnt = cnt.wrapping_add(1)
  }
  /* Append encrypted password to result buffer */
  //TODO: replace with something like
  //	bb_uuencode(cp, src, length, bb_uuenc_tbl_XXXbase64);
  if _32or64 == 32i32 {
    /* sha256 */
    let mut i: libc::c_uint = 0i32 as libc::c_uint;
    loop {
      let mut j: libc::c_uint = i.wrapping_add(10i32 as libc::c_uint);
      let mut k: libc::c_uint = i.wrapping_add(20i32 as libc::c_uint);
      if j >= 30i32 as libc::c_uint {
        j = j.wrapping_sub(30i32 as libc::c_uint)
      }
      if k >= 30i32 as libc::c_uint {
        k = k.wrapping_sub(30i32 as libc::c_uint)
      }
      let mut w: libc::c_uint = ((L.alt_result[i as usize] as libc::c_int) << 16i32
        | (L.alt_result[j as usize] as libc::c_int) << 8i32
        | L.alt_result[k as usize] as libc::c_int) as libc::c_uint;
      resptr = to64(resptr, w, 4i32);
      if k == 29i32 as libc::c_uint {
        break;
      }
      i = k.wrapping_add(1i32 as libc::c_uint)
    }
    let mut w_0: libc::c_uint = (0i32 << 16i32
      | (L.alt_result[31] as libc::c_int) << 8i32
      | L.alt_result[30] as libc::c_int) as libc::c_uint;
    resptr = to64(resptr, w_0, 3i32)
  } else {
    let mut i_0: libc::c_uint = 0i32 as libc::c_uint;
    loop {
      let mut j_0: libc::c_uint = i_0.wrapping_add(21i32 as libc::c_uint);
      let mut k_0: libc::c_uint = i_0.wrapping_add(42i32 as libc::c_uint);
      if j_0 >= 63i32 as libc::c_uint {
        j_0 = j_0.wrapping_sub(63i32 as libc::c_uint)
      }
      if k_0 >= 63i32 as libc::c_uint {
        k_0 = k_0.wrapping_sub(63i32 as libc::c_uint)
      }
      let mut w_1: libc::c_uint = ((L.alt_result[i_0 as usize] as libc::c_int) << 16i32
        | (L.alt_result[j_0 as usize] as libc::c_int) << 8i32
        | L.alt_result[k_0 as usize] as libc::c_int)
        as libc::c_uint;
      resptr = to64(resptr, w_1, 4i32);
      if j_0 == 20i32 as libc::c_uint {
        break;
      }
      i_0 = j_0.wrapping_add(1i32 as libc::c_uint)
    }
    let mut w_2: libc::c_uint =
      (0i32 << 16i32 | 0i32 << 8i32 | L.alt_result[63] as libc::c_int) as libc::c_uint;
    resptr = to64(resptr, w_2, 2i32)
    /* was:
    b64_from_24bit(alt_result[0], alt_result[21], alt_result[42], 4);
    b64_from_24bit(alt_result[22], alt_result[43], alt_result[1], 4);
    b64_from_24bit(alt_result[44], alt_result[2], alt_result[23], 4);
    b64_from_24bit(alt_result[3], alt_result[24], alt_result[45], 4);
    b64_from_24bit(alt_result[25], alt_result[46], alt_result[4], 4);
    b64_from_24bit(alt_result[47], alt_result[5], alt_result[26], 4);
    b64_from_24bit(alt_result[6], alt_result[27], alt_result[48], 4);
    b64_from_24bit(alt_result[28], alt_result[49], alt_result[7], 4);
    b64_from_24bit(alt_result[50], alt_result[8], alt_result[29], 4);
    b64_from_24bit(alt_result[9], alt_result[30], alt_result[51], 4);
    b64_from_24bit(alt_result[31], alt_result[52], alt_result[10], 4);
    b64_from_24bit(alt_result[53], alt_result[11], alt_result[32], 4);
    b64_from_24bit(alt_result[12], alt_result[33], alt_result[54], 4);
    b64_from_24bit(alt_result[34], alt_result[55], alt_result[13], 4);
    b64_from_24bit(alt_result[56], alt_result[14], alt_result[35], 4);
    b64_from_24bit(alt_result[15], alt_result[36], alt_result[57], 4);
    b64_from_24bit(alt_result[37], alt_result[58], alt_result[16], 4);
    b64_from_24bit(alt_result[59], alt_result[17], alt_result[38], 4);
    b64_from_24bit(alt_result[18], alt_result[39], alt_result[60], 4);
    b64_from_24bit(alt_result[40], alt_result[61], alt_result[19], 4);
    b64_from_24bit(alt_result[62], alt_result[20], alt_result[41], 4);
    b64_from_24bit(0, 0, alt_result[63], 2);
    */
  }
  /* *resptr = '\0'; - xzalloc did it */
  /* Clear the buffer for the intermediate result so that people
  attaching to processes or reading core dumps cannot get any
  information.  */
  memset(
    &mut L as *mut C2RustUnnamed as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<C2RustUnnamed>() as libc::c_ulong,
  ); /* [alt]_ctx and XXX_result buffers */
  memset(
    key_data as *mut libc::c_void,
    0i32,
    key_len as libc::c_ulong,
  ); /* also p_bytes */
  memset(
    salt_data as *mut libc::c_void,
    0i32,
    salt_len as libc::c_ulong,
  ); /* also s_bytes */
  free(key_data as *mut libc::c_void);
  free(salt_data as *mut libc::c_void);
  return result;
}
/*
 * DES and MD5 crypt implementations are taken from uclibc.
 * They were modified to not use static buffers.
 */
/* Other advanced crypt ids (TODO?): */
/* $2$ or $2a$: Blowfish */
static mut des_cctx: *mut const_des_ctx = 0 as *const const_des_ctx as *mut const_des_ctx;
static mut des_ctx: *mut des_ctx = 0 as *const des_ctx as *mut des_ctx;
/* my_crypt returns malloc'ed data */
unsafe extern "C" fn my_crypt(
  mut key: *const libc::c_char,
  mut salt: *const libc::c_char,
) -> *mut libc::c_char {
  /* MD5 or SHA? */
  if *salt.offset(0) as libc::c_int == '$' as i32
    && *salt.offset(1) as libc::c_int != 0
    && *salt.offset(2) as libc::c_int == '$' as i32
  {
    if *salt.offset(1) as libc::c_int == '1' as i32 {
      return md5_crypt(
        xzalloc(36i32 as size_t) as *mut libc::c_char,
        key as *mut libc::c_uchar,
        salt as *mut libc::c_uchar,
      );
    }
    if *salt.offset(1) as libc::c_int == '5' as i32 || *salt.offset(1) as libc::c_int == '6' as i32
    {
      return sha_crypt(key as *mut libc::c_char, salt as *mut libc::c_char);
    }
  }
  if des_cctx.is_null() {
    des_cctx = const_des_init()
  }
  des_ctx = des_init(des_ctx, des_cctx);
  return des_crypt(
    des_ctx,
    xzalloc(21i32 as size_t) as *mut libc::c_char,
    key as *mut libc::c_uchar,
    salt as *mut libc::c_uchar,
  );
}
/* So far nobody wants to have it public */
unsafe extern "C" fn my_crypt_cleanup() {
  free(des_cctx as *mut libc::c_void);
  free(des_ctx as *mut libc::c_void);
  des_cctx = 0 as *mut const_des_ctx;
  des_ctx = 0 as *mut des_ctx;
}

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
/*u8 *server_write_MAC_key;*/
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
#[no_mangle]
pub unsafe extern "C" fn pw_encrypt(
  mut clear: *const libc::c_char,
  mut salt: *const libc::c_char,
  mut cleanup: libc::c_int,
) -> *mut libc::c_char {
  let mut encrypted: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  encrypted = my_crypt(clear, salt);
  if cleanup != 0 {
    my_crypt_cleanup();
  }
  return encrypted;
}
/* if !ENABLE_USE_BB_CRYPT */

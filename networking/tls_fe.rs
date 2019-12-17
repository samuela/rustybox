use libc;
extern "C" {
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
}

/*
 * Copyright (C) 2018 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
pub type byte = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
  pub f25519_one: [byte; 32],
  pub xm: [byte; 32],
  pub zm: [byte; 32],
  pub xm1: [byte; 32],
  pub zm1: [byte; 32],
  // = {0};
}
pub type word32 = u32;
/* The code below is taken from wolfssl-3.15.3/wolfcrypt/src/fe_low_mem.c
 * Header comment is kept intact:
 */
/* fe_low_mem.c
 *
 * Copyright (C) 2006-2017 wolfSSL Inc.
 *
 * This file is part of wolfSSL.
 *
 * wolfSSL is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * wolfSSL is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1335, USA
 */
/* Based from Daniel Beer's public domain work. */
//UNUSED
unsafe extern "C" fn lm_copy(mut x: *mut byte, mut a: *const byte) {
  memcpy(
    x as *mut libc::c_void,
    a as *const libc::c_void,
    32i32 as libc::c_ulong,
  );
}
//UNUSED
unsafe extern "C" fn fe_select(
  mut dst: *mut byte,
  mut zero: *const byte,
  mut one: *const byte,
  mut condition: byte,
) {
  let mask: byte = -(condition as libc::c_int) as byte;
  let mut i: libc::c_int = 0;
  i = 0;
  while i < 32i32 {
    *dst.offset(i as isize) = (*zero.offset(i as isize) as libc::c_int
      ^ mask as libc::c_int
        & (*one.offset(i as isize) as libc::c_int ^ *zero.offset(i as isize) as libc::c_int))
      as byte;
    i += 1
  }
}
//UNUSED
//UNUSED
//UNUSED
//UNUSED
//UNUSED
//UNUSED
//UNUSED
unsafe extern "C" fn fe_normalize(mut x: *mut byte) {
  let mut minusp: [byte; 32] = [0; 32];
  let mut c: libc::c_uint = 0;
  let mut i: libc::c_int = 0;
  /* Reduce using 2^255 = 19 mod p */
  c = ((*x.offset(31) as libc::c_int >> 7i32) * 19i32) as libc::c_uint;
  let ref mut fresh0 = *x.offset(31);
  *fresh0 = (*fresh0 as libc::c_int & 127i32) as byte;
  i = 0;
  while i < 32i32 {
    c = c.wrapping_add(*x.offset(i as isize) as libc::c_uint);
    *x.offset(i as isize) = c as byte;
    c >>= 8i32;
    i += 1
  }
  /* The number is now less than 2^255 + 18, and therefore less than
   * 2p. Try subtracting p, and conditionally load the subtracted
   * value if underflow did not occur.
   */
  c = 19i32 as libc::c_uint;
  i = 0;
  while i < 32i32 - 1i32 {
    c = c.wrapping_add(*x.offset(i as isize) as libc::c_uint);
    minusp[i as usize] = c as byte;
    c >>= 8i32;
    i += 1
  }
  c = c.wrapping_add((*x.offset(i as isize) as libc::c_uint).wrapping_sub(128i32 as libc::c_uint));
  minusp[31] = c as byte;
  /* Load x-p if no underflow */
  fe_select(
    x,
    minusp.as_mut_ptr(),
    x,
    (c >> 15i32 & 1i32 as libc::c_uint) as byte,
  );
}
unsafe extern "C" fn lm_add(mut r: *mut byte, mut a: *const byte, mut b: *const byte) {
  let mut c: libc::c_uint = 0 as libc::c_uint;
  let mut i: libc::c_int = 0;
  /* Add */
  i = 0;
  while i < 32i32 {
    c >>= 8i32;
    c = c.wrapping_add(
      (*a.offset(i as isize) as libc::c_uint).wrapping_add(*b.offset(i as isize) as libc::c_uint),
    );
    *r.offset(i as isize) = c as byte;
    i += 1
  }
  /* Reduce with 2^255 = 19 mod p */
  let ref mut fresh1 = *r.offset(31);
  *fresh1 = (*fresh1 as libc::c_int & 127i32) as byte;
  c = (c >> 7i32).wrapping_mul(19i32 as libc::c_uint);
  i = 0;
  while i < 32i32 {
    c = c.wrapping_add(*r.offset(i as isize) as libc::c_uint);
    *r.offset(i as isize) = c as byte;
    c >>= 8i32;
    i += 1
  }
}
unsafe extern "C" fn lm_sub(mut r: *mut byte, mut a: *const byte, mut b: *const byte) {
  let mut c: word32 = 0 as word32;
  let mut i: libc::c_int = 0;
  /* Calculate a + 2p - b, to avoid underflow */
  c = 218i32 as word32;
  i = 0;
  while i + 1i32 < 32i32 {
    c = (c as libc::c_uint).wrapping_add(
      (65280i32 as libc::c_uint)
        .wrapping_add(*a.offset(i as isize) as word32)
        .wrapping_sub(*b.offset(i as isize) as word32),
    ) as word32 as word32;
    *r.offset(i as isize) = c as byte;
    c >>= 8i32;
    i += 1
  }
  c = (c as libc::c_uint)
    .wrapping_add((*a.offset(31) as word32).wrapping_sub(*b.offset(31) as word32)) as word32
    as word32;
  *r.offset(31) = (c & 127i32 as libc::c_uint) as byte;
  c = (c >> 7i32).wrapping_mul(19i32 as libc::c_uint);
  i = 0;
  while i < 32i32 {
    c = (c as libc::c_uint).wrapping_add(*r.offset(i as isize) as libc::c_uint) as word32 as word32;
    *r.offset(i as isize) = c as byte;
    c >>= 8i32;
    i += 1
  }
}
//UNUSED
unsafe extern "C" fn fe_mul__distinct(mut r: *mut byte, mut a: *const byte, mut b: *const byte) {
  let mut c: word32 = 0 as word32;
  let mut i: libc::c_int = 0;
  i = 0;
  while i < 32i32 {
    let mut j: libc::c_int = 0;
    c >>= 8i32;
    j = 0;
    while j <= i {
      c = (c as libc::c_uint).wrapping_add(
        (*a.offset(j as isize) as word32).wrapping_mul(*b.offset((i - j) as isize) as word32),
      ) as word32 as word32;
      j += 1
    }
    while j < 32i32 {
      c = (c as libc::c_uint).wrapping_add(
        (*a.offset(j as isize) as word32)
          .wrapping_mul(*b.offset((i + 32i32 - j) as isize) as word32)
          .wrapping_mul(38i32 as libc::c_uint),
      ) as word32 as word32;
      j += 1
    }
    *r.offset(i as isize) = c as byte;
    i += 1
  }
  let ref mut fresh2 = *r.offset(31);
  *fresh2 = (*fresh2 as libc::c_int & 127i32) as byte;
  c = (c >> 7i32).wrapping_mul(19i32 as libc::c_uint);
  i = 0;
  while i < 32i32 {
    c = (c as libc::c_uint).wrapping_add(*r.offset(i as isize) as libc::c_uint) as word32 as word32;
    *r.offset(i as isize) = c as byte;
    c >>= 8i32;
    i += 1
  }
}
//UNUSED
unsafe extern "C" fn fe_mul_c(mut r: *mut byte, mut a: *const byte, mut b: word32) {
  let mut c: word32 = 0 as word32;
  let mut i: libc::c_int = 0;
  i = 0;
  while i < 32i32 {
    c >>= 8i32;
    c = (c as libc::c_uint).wrapping_add(b.wrapping_mul(*a.offset(i as isize) as word32)) as word32
      as word32;
    *r.offset(i as isize) = c as byte;
    i += 1
  }
  let ref mut fresh3 = *r.offset(31);
  *fresh3 = (*fresh3 as libc::c_int & 127i32) as byte;
  c >>= 7i32;
  c = (c as libc::c_uint).wrapping_mul(19i32 as libc::c_uint) as word32 as word32;
  i = 0;
  while i < 32i32 {
    c = (c as libc::c_uint).wrapping_add(*r.offset(i as isize) as libc::c_uint) as word32 as word32;
    *r.offset(i as isize) = c as byte;
    c >>= 8i32;
    i += 1
  }
}
unsafe extern "C" fn fe_inv__distinct(mut r: *mut byte, mut x: *const byte) {
  let mut s: [byte; 32] = [0; 32];
  let mut i: libc::c_int = 0;
  /* This is a prime field, so by Fermat's little theorem:
   *
   *     x^(p-1) = 1 mod p
   *
   * Therefore, raise to (p-2) = 2^255-21 to get a multiplicative
   * inverse.
   *
   * This is a 255-bit binary number with the digits:
   *
   *     11111111... 01011
   *
   * We compute the result by the usual binary chain, but
   * alternate between keeping the accumulator in r and s, so as
   * to avoid copying temporaries.
   */
  /* 1 1 */
  fe_mul__distinct(s.as_mut_ptr(), x, x);
  fe_mul__distinct(r, s.as_mut_ptr(), x);
  /* 1 x 248 */
  i = 0;
  while i < 248i32 {
    fe_mul__distinct(s.as_mut_ptr(), r, r);
    fe_mul__distinct(r, s.as_mut_ptr(), x);
    i += 1
  }
  /* 0 */
  fe_mul__distinct(s.as_mut_ptr(), r, r);
  /* 1 */
  fe_mul__distinct(r, s.as_mut_ptr(), s.as_mut_ptr());
  fe_mul__distinct(s.as_mut_ptr(), r, x);
  /* 0 */
  fe_mul__distinct(r, s.as_mut_ptr(), s.as_mut_ptr());
  /* 1 */
  fe_mul__distinct(s.as_mut_ptr(), r, r);
  fe_mul__distinct(r, s.as_mut_ptr(), x);
  /* 1 */
  fe_mul__distinct(s.as_mut_ptr(), r, r);
  fe_mul__distinct(r, s.as_mut_ptr(), x);
}
//UNUSED
//UNUSED
//UNUSED
/* Differential addition */
unsafe extern "C" fn xc_diffadd(
  mut x5: *mut byte,
  mut z5: *mut byte,
  mut x1: *const byte,
  mut z1: *const byte,
  mut x2: *const byte,
  mut z2: *const byte,
  mut x3: *const byte,
  mut z3: *const byte,
) {
  /* Explicit formulas database: dbl-1987-m3
   *
   * source 1987 Montgomery "Speeding the Pollard and elliptic curve
   *   methods of factorization", page 261, fifth display, plus
   *   common-subexpression elimination
   * compute A = X2+Z2
   * compute B = X2-Z2
   * compute C = X3+Z3
   * compute D = X3-Z3
   * compute DA = D A
   * compute CB = C B
   * compute X5 = Z1(DA+CB)^2
   * compute Z5 = X1(DA-CB)^2
   */
  let mut da: [byte; 32] = [0; 32]; /* D */
  let mut cb: [byte; 32] = [0; 32]; /* C */
  let mut a: [byte; 32] = [0; 32];
  let mut b: [byte; 32] = [0; 32];
  lm_add(a.as_mut_ptr(), x2, z2);
  lm_sub(b.as_mut_ptr(), x3, z3);
  fe_mul__distinct(da.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
  lm_sub(b.as_mut_ptr(), x2, z2);
  lm_add(a.as_mut_ptr(), x3, z3);
  fe_mul__distinct(cb.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
  lm_add(a.as_mut_ptr(), da.as_mut_ptr(), cb.as_mut_ptr());
  fe_mul__distinct(b.as_mut_ptr(), a.as_mut_ptr(), a.as_mut_ptr());
  fe_mul__distinct(x5, z1, b.as_mut_ptr());
  lm_sub(a.as_mut_ptr(), da.as_mut_ptr(), cb.as_mut_ptr());
  fe_mul__distinct(b.as_mut_ptr(), a.as_mut_ptr(), a.as_mut_ptr());
  fe_mul__distinct(z5, x1, b.as_mut_ptr());
}
/* Double an X-coordinate */
unsafe extern "C" fn xc_double(
  mut x3: *mut byte,
  mut z3: *mut byte,
  mut x1: *const byte,
  mut z1: *const byte,
) {
  /* Explicit formulas database: dbl-1987-m
   *
   * source 1987 Montgomery "Speeding the Pollard and elliptic
   *   curve methods of factorization", page 261, fourth display
   * compute X3 = (X1^2-Z1^2)^2
   * compute Z3 = 4 X1 Z1 (X1^2 + a X1 Z1 + Z1^2)
   */
  let mut x1sq: [byte; 32] = [0; 32];
  let mut z1sq: [byte; 32] = [0; 32];
  let mut x1z1: [byte; 32] = [0; 32];
  let mut a: [byte; 32] = [0; 32];
  fe_mul__distinct(x1sq.as_mut_ptr(), x1, x1);
  fe_mul__distinct(z1sq.as_mut_ptr(), z1, z1);
  fe_mul__distinct(x1z1.as_mut_ptr(), x1, z1);
  lm_sub(a.as_mut_ptr(), x1sq.as_mut_ptr(), z1sq.as_mut_ptr());
  fe_mul__distinct(x3, a.as_mut_ptr(), a.as_mut_ptr());
  fe_mul_c(a.as_mut_ptr(), x1z1.as_mut_ptr(), 486662i32 as word32);
  lm_add(a.as_mut_ptr(), x1sq.as_mut_ptr(), a.as_mut_ptr());
  lm_add(a.as_mut_ptr(), z1sq.as_mut_ptr(), a.as_mut_ptr());
  fe_mul__distinct(x1sq.as_mut_ptr(), x1z1.as_mut_ptr(), a.as_mut_ptr());
  fe_mul_c(z3, x1sq.as_mut_ptr(), 4i32 as word32);
}
/*
 * Copyright (C) 2018 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn curve25519(mut result: *mut byte, mut e: *const byte, mut q: *const byte) {
  let mut i: libc::c_int = 0;
  let mut z: C2RustUnnamed = C2RustUnnamed {
    f25519_one: [0; 32],
    xm: [0; 32],
    zm: [0; 32],
    xm1: [0; 32],
    zm1: [0; 32],
  };
  memset(
    &mut z as *mut C2RustUnnamed as *mut libc::c_void,
    0,
    ::std::mem::size_of::<C2RustUnnamed>() as libc::c_ulong,
  );
  z.f25519_one[0] = 1i32 as byte;
  z.zm[0] = 1i32 as byte;
  z.xm1[0] = 1i32 as byte;
  /* Note: bit 254 is assumed to be 1 */
  lm_copy(z.xm.as_mut_ptr(), q);
  i = 253i32;
  while i >= 0 {
    let bit: libc::c_int = *e.offset((i >> 3i32) as isize) as libc::c_int >> (i & 7i32) & 1i32;
    let mut xms: [byte; 32] = [0; 32];
    let mut zms: [byte; 32] = [0; 32];
    /* From P_m and P_(m-1), compute P_(2m) and P_(2m-1) */
    xc_diffadd(
      z.xm1.as_mut_ptr(),
      z.zm1.as_mut_ptr(),
      q,
      z.f25519_one.as_mut_ptr(),
      z.xm.as_mut_ptr(),
      z.zm.as_mut_ptr(),
      z.xm1.as_mut_ptr(),
      z.zm1.as_mut_ptr(),
    );
    xc_double(
      z.xm.as_mut_ptr(),
      z.zm.as_mut_ptr(),
      z.xm.as_mut_ptr(),
      z.zm.as_mut_ptr(),
    );
    /* Compute P_(2m+1) */
    xc_diffadd(
      xms.as_mut_ptr(),
      zms.as_mut_ptr(),
      z.xm1.as_mut_ptr(),
      z.zm1.as_mut_ptr(),
      z.xm.as_mut_ptr(),
      z.zm.as_mut_ptr(),
      q,
      z.f25519_one.as_mut_ptr(),
    );
    /* Select:
     *   bit = 1 --> (P_(2m+1), P_(2m))
     *   bit = 0 --> (P_(2m), P_(2m-1))
     */
    fe_select(
      z.xm1.as_mut_ptr(),
      z.xm1.as_mut_ptr(),
      z.xm.as_mut_ptr(),
      bit as byte,
    );
    fe_select(
      z.zm1.as_mut_ptr(),
      z.zm1.as_mut_ptr(),
      z.zm.as_mut_ptr(),
      bit as byte,
    );
    fe_select(
      z.xm.as_mut_ptr(),
      z.xm.as_mut_ptr(),
      xms.as_mut_ptr(),
      bit as byte,
    );
    fe_select(
      z.zm.as_mut_ptr(),
      z.zm.as_mut_ptr(),
      zms.as_mut_ptr(),
      bit as byte,
    );
    i -= 1
  }
  /* Freeze out of projective coordinates */
  fe_inv__distinct(z.zm1.as_mut_ptr(), z.zm.as_mut_ptr());
  fe_mul__distinct(result, z.zm1.as_mut_ptr(), z.xm.as_mut_ptr());
  fe_normalize(result);
}

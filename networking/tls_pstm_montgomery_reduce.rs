use libc;
use libc::free;
extern "C" {

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn pstm_clamp(a: *mut pstm_int);
  #[no_mangle]
  fn pstm_cmp_mag(a: *mut pstm_int, b: *mut pstm_int) -> int32;
  #[no_mangle]
  fn s_pstm_sub(a: *mut pstm_int, b: *mut pstm_int, c: *mut pstm_int) -> int32;
}

use crate::librb::size_t;
/*
 * Copyright (C) 2017 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* Interface glue between bbox code and minimally tweaked matrixssl
 * code. All C files (matrixssl and bbox (ones which need TLS))
 * include this file, and guaranteed to see a consistent API,
 * defines, types, etc.
 */
/* Config tweaks */
/* pstm: multiprecision numbers */
//#if defined(__GNUC__) && defined(__x86_64__)
//  /* PSTM_X86_64 works correctly, but +782 bytes. */
//  /* Looks like most of the growth is because of PSTM_64BIT. */
//# define PSTM_64BIT
//# define PSTM_X86_64
//#endif
//#if SOME_COND #define PSTM_MIPS, #define PSTM_32BIT
//#if SOME_COND #define PSTM_ARM,  #define PSTM_32BIT
/* Failure due to bad function param */
/* Failure as a result of system call error */
/* Failure to allocate requested memory */
/* Failure on sanity/limit tests */
pub type uint64 = u64;
pub type uint32 = u32;
pub type int32 = i32;
pub type pstm_digit = uint32;
pub type pstm_word = uint64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pstm_int {
  pub used: libc::c_int,
  pub alloc: libc::c_int,
  pub sign: libc::c_int,
  pub dp: *mut pstm_digit,
}
/*
 * Copyright (C) 2017 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* The file is taken almost verbatim from matrixssl-3-7-2b-open/crypto/math/.
 * Changes are flagged with //bbox
 */
/* *
 *	@file    pstm.h
 *	@version 33ef80f (HEAD, tag: MATRIXSSL-3-7-2-OPEN, tag: MATRIXSSL-3-7-2-COMM, origin/master, origin/HEAD, master)
 *
 *	multiple-precision integer library.
 */
/*
 *	Copyright (c) 2013-2015 INSIDE Secure Corporation
 *	Copyright (c) PeerSec Networks, 2002-2011
 *	All Rights Reserved
 *
 *	The latest version of this code is available at http://www.matrixssl.org
 *
 *	This software is open source; you can redistribute it and/or modify
 *	it under the terms of the GNU General Public License as published by
 *	the Free Software Foundation; either version 2 of the License, or
 *	(at your option) any later version.
 *
 *	This General Public License does NOT permit incorporating this software
 *	into proprietary programs.  If you are unable to comply with the GPL, a
 *	commercial license for this software may be purchased from INSIDE at
 *	http://www.insidesecure.com/eng/Company/Locations
 *
 *	This program is distributed in WITHOUT ANY WARRANTY; without even the
 *	implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 *	See the GNU General Public License for more details.
 *
 *	You should have received a copy of the GNU General Public License
 *	along with this program; if not, write to the Free Software
 *	Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *	http://www.gnu.org/copyleft/gpl.html
 */
/* *****************************************************************************/
/* Define this here to avoid including circular limits.h on some platforms */
/* *****************************************************************************/
/*
  If native 64 bit integers are not supported, we do not support 32x32->64
  in hardware, so we must set the 16 bit flag to produce 16x16->32 products.
*/
/* ! HAVE_NATIVE_INT64 */
/* *****************************************************************************/
/*
 Some default configurations.

 pstm_word should be the largest value the processor can hold as the product
   of a multiplication. Most platforms support a 32x32->64 MAC instruction,
   so 64bits is the default pstm_word size.
 pstm_digit should be half the size of pstm_word
*/
/*	This is the default case, 32-bit digits, 64-bit word products */
/* digit and word size */
/* *****************************************************************************/
/*
 equalities
*/
/* less than */
/* equal to */
/* greater than */
/* positive integer */
/* negative */
/* *****************************************************************************/
/*
 Various build options
*/
/* default (64) digits of allocation */
//bbox: was int16
//bbox	psPool_t	*pool;
/* *****************************************************************************/
/*
 Operations on large integers
*/
//made static:extern void pstm_set(pstm_int *a, pstm_digit b);
//made static:extern void pstm_zero(pstm_int * a);
//bbox: pool unused
//made static:extern int32 pstm_init(psPool_t *pool, pstm_int * a);
//bbox: pool unused
//bbox: pool unused
//made static:extern int32 pstm_init_copy(psPool_t *pool, pstm_int * a, pstm_int * b,
//made static:				int toSqr); //bbox: was int16 toSqr
//made static:extern int pstm_count_bits (pstm_int * a) FAST_FUNC; //bbox: was returning int16
//bbox: pool unused
//made static:extern void pstm_exch(pstm_int * a, pstm_int * b);
//bbox: was int16 size
//made static:extern void pstm_rshd(pstm_int *a, int x); //bbox: was int16 x
//made static:extern int32 pstm_lshd(pstm_int * a, int b); //bbox: was int16 b
//bbox: pool unused
//made static:extern int32 pstm_div(psPool_t *pool, pstm_int *a, pstm_int *b, pstm_int *c,
//made static:				pstm_int *d);
//bbox: pool unused
//made static:extern int32 pstm_div_2d(psPool_t *pool, pstm_int *a, int b, pstm_int *c,
//made static:				pstm_int *d); //bbox: was int16 b
//bbox: pool unused
//bbox: pool unused
//made static:extern int32 pstm_mod(psPool_t *pool, pstm_int *a, pstm_int *b, pstm_int *c);
//bbox: pool unused
//bbox: pool unused
//made static:extern int32 pstm_2expt(pstm_int *a, int b); //bbox: was int16 b
//bbox: pool unused
//bbox: pool unused
//made static:extern int32 pstm_montgomery_setup(pstm_int *a, pstm_digit *rho);
//bbox: pool unused
/*
 * Copyright (C) 2017 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* The file is taken almost verbatim from matrixssl-3-7-2b-open/crypto/math/.
 * Changes are flagged with //bbox
 */
/* *
 *	@file    pstm_montgomery_reduce.c
 *	@version 33ef80f (HEAD, tag: MATRIXSSL-3-7-2-OPEN, tag: MATRIXSSL-3-7-2-COMM, origin/master, origin/HEAD, master)
 *
 *	Multiprecision Montgomery Reduction.
 */
/*
 *	Copyright (c) 2013-2015 INSIDE Secure Corporation
 *	Copyright (c) PeerSec Networks, 2002-2011
 *	All Rights Reserved
 *
 *	The latest version of this code is available at http://www.matrixssl.org
 *
 *	This software is open source; you can redistribute it and/or modify
 *	it under the terms of the GNU General Public License as published by
 *	the Free Software Foundation; either version 2 of the License, or
 *	(at your option) any later version.
 *
 *	This General Public License does NOT permit incorporating this software
 *	into proprietary programs.  If you are unable to comply with the GPL, a
 *	commercial license for this software may be purchased from INSIDE at
 *	http://www.insidesecure.com/eng/Company/Locations
 *
 *	This program is distributed in WITHOUT ANY WARRANTY; without even the
 *	implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 *	See the GNU General Public License for more details.
 *
 *	You should have received a copy of the GNU General Public License
 *	along with this program; if not, write to the Free Software
 *	Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *	http://www.gnu.org/copyleft/gpl.html
 */
/* *****************************************************************************/
//bbox
//#include "../cryptoApi.h"
/* *****************************************************************************/
/* ISO C code */
/* *****************************************************************************/
/* computes x/R == x (mod N) via Montgomery Reduction */
#[no_mangle]
pub unsafe extern "C" fn pstm_montgomery_reduce(
  mut a: *mut pstm_int,
  mut m: *mut pstm_int,
  mut mp: pstm_digit,
  mut paD: *mut pstm_digit,
  mut paDlen: uint32,
) -> int32 {
  let mut c: *mut pstm_digit = 0 as *mut pstm_digit; //bbox: was int16
  let mut _c: *mut pstm_digit = 0 as *mut pstm_digit;
  let mut tmpm: *mut pstm_digit = 0 as *mut pstm_digit;
  let mut mu: pstm_digit = 0;
  let mut oldused: int32 = 0;
  let mut x: int32 = 0;
  let mut y: int32 = 0;
  let mut pa: libc::c_int = 0;
  pa = (*m).used;
  if pa > (*a).alloc {
    /* Sanity test for bad numbers.  This will confirm no buffer overruns */
    return -9i32;
  }
  if !paD.is_null()
    && paDlen
      >= (2i32 as uint32)
        .wrapping_mul(pa as libc::c_uint)
        .wrapping_add(1i32 as libc::c_uint)
  {
    c = paD;
    memset(c as *mut libc::c_void, 0i32, paDlen as libc::c_ulong);
  } else {
    c = xzalloc((2i32 * pa + 1i32) as size_t) as *mut pstm_digit
    //bbox
  }
  /* copy the input */
  oldused = (*a).used;
  x = 0i32;
  while x < oldused {
    *c.offset(x as isize) = *(*a).dp.offset(x as isize);
    x += 1
  }
  x = 0i32;
  while x < pa {
    let mut cy: pstm_digit = 0i32 as pstm_digit;
    /* get Mu for this round */
    mu = (*c.offset(x as isize)).wrapping_mul(mp);
    _c = c.offset(x as isize);
    tmpm = (*m).dp;
    y = 0i32;
    /* PSTM_X86_64 */
    while y < pa {
      let mut t: pstm_word = 0;
      let fresh0 = tmpm;
      tmpm = tmpm.offset(1);
      t = (*_c.offset(0) as pstm_word)
        .wrapping_add(cy as pstm_word)
        .wrapping_add((mu as pstm_word).wrapping_mul(*fresh0 as pstm_word));
      *_c.offset(0) = t as pstm_digit;
      cy = (t >> 32i32) as pstm_digit;
      _c = _c.offset(1);
      y += 1
    }
    while cy != 0 {
      let ref mut fresh1 = *_c.offset(0);
      *fresh1 = (*fresh1 as libc::c_uint).wrapping_add(cy) as pstm_digit as pstm_digit;
      let mut t_0: pstm_digit = *fresh1;
      cy = (t_0 < cy) as libc::c_int as pstm_digit;
      _c = _c.offset(1)
    }
    x += 1
  }
  /* now copy out */
  _c = c.offset(pa as isize);
  tmpm = (*a).dp;
  x = 0i32;
  while x < pa + 1i32 {
    let fresh2 = _c;
    _c = _c.offset(1);
    let fresh3 = tmpm;
    tmpm = tmpm.offset(1);
    *fresh3 = *fresh2;
    x += 1
  }
  while x < oldused {
    let fresh4 = tmpm;
    tmpm = tmpm.offset(1);
    *fresh4 = 0i32 as pstm_digit;
    x += 1
  }
  (*a).used = pa + 1i32;
  pstm_clamp(a);
  /* reuse x as return code */
  x = 0i32;
  /* if A >= m then A = A - m */
  if pstm_cmp_mag(a, m) != -1i32 {
    if s_pstm_sub(a, m, a) != 0i32 {
      x = -8i32
    }
  }
  if paDlen
    < (2i32 as uint32)
      .wrapping_mul(pa as libc::c_uint)
      .wrapping_add(1i32 as libc::c_uint)
  {
    free(c as *mut libc::c_void);
  }
  return x;
}
/* *****************************************************************************/
/* !DISABLE_PSTM */

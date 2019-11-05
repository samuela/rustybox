use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn pstm_grow(a: *mut pstm_int, size: libc::c_int) -> int32;
  #[no_mangle]
  fn pstm_clamp(a: *mut pstm_int);
}

use crate::librb::int32_t;
use crate::librb::size_t;
use crate::librb::uint32_t;
use crate::librb::uint64_t;
/* Failure to allocate requested memory */
/* Failure on sanity/limit tests */
pub type uint64 = uint64_t;
pub type uint32 = uint32_t;
pub type int32 = int32_t;
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
 *	@file    pstm_mul_comba.c
 *	@version 33ef80f (HEAD, tag: MATRIXSSL-3-7-2-OPEN, tag: MATRIXSSL-3-7-2-COMM, origin/master, origin/HEAD, master)
 *
 *	Multiprecision multiplication with Comba technique.
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
/* *****************************************************************************/
/* generic PxQ multiplier */
//bbox: pool unused
unsafe extern "C" fn pstm_mul_comba_gen(
  mut A: *mut pstm_int,
  mut B: *mut pstm_int,
  mut C: *mut pstm_int,
  mut paD: *mut pstm_digit,
  mut paDlen: uint32,
) -> int32 {
  let mut paDfail: libc::c_int = 0; //bbox: was int16
  let mut pa: libc::c_int = 0;
  let mut ix: int32 = 0;
  let mut iy: int32 = 0;
  let mut iz: int32 = 0;
  let mut tx: int32 = 0;
  let mut ty: int32 = 0;
  let mut c0: pstm_digit = 0;
  let mut c1: pstm_digit = 0;
  let mut c2: pstm_digit = 0;
  let mut tmpx: *mut pstm_digit = 0 as *mut pstm_digit;
  let mut tmpy: *mut pstm_digit = 0 as *mut pstm_digit;
  let mut dst: *mut pstm_digit = 0 as *mut pstm_digit;
  c2 = 0i32 as pstm_digit;
  c1 = c2;
  c0 = c1;
  paDfail = 0i32;
  /* get size of output and trim */
  pa = (*A).used + (*B).used;
  /*
    If c is not large enough grow it and continue
  */
  if (*C).alloc < pa {
    if pstm_grow(C, pa) != 0i32 {
      return -8i32;
    }
  } /* have a paD but it's not large enough */
  if !paD.is_null() {
    if (paDlen as libc::c_ulong)
      < (::std::mem::size_of::<pstm_digit>() as libc::c_ulong).wrapping_mul(pa as libc::c_ulong)
    {
      paDfail = 1i32;
      dst = xzalloc(
        (::std::mem::size_of::<pstm_digit>() as libc::c_ulong).wrapping_mul(pa as libc::c_ulong),
      ) as *mut pstm_digit
    //bbox
    } else {
      dst = paD;
      memset(dst as *mut libc::c_void, 0i32, paDlen as libc::c_ulong);
    }
  } else {
    dst = xzalloc(
      (::std::mem::size_of::<pstm_digit>() as libc::c_ulong).wrapping_mul(pa as libc::c_ulong),
    ) as *mut pstm_digit
    //bbox
  }
  ix = 0i32;
  while ix < pa {
    /* get offsets into the two bignums */
    ty = if ix < (*B).used - 1i32 {
      ix
    } else {
      ((*B).used) - 1i32
    };
    tx = ix - ty;
    /* setup temp aliases */
    tmpx = (*A).dp.offset(tx as isize);
    tmpy = (*B).dp.offset(ty as isize);
    /*
        This is the number of times the loop will iterate, essentially it's
          while (tx++ < a->used && ty-- >= 0) { ... }
    */
    iy = if (*A).used - tx < ty + 1i32 {
      ((*A).used) - tx
    } else {
      (ty) + 1i32
    };
    /* execute loop */
    c0 = c1;
    c1 = c2;
    c2 = 0i32 as pstm_digit;
    iz = 0i32;
    while iz < iy {
      let mut t: pstm_word = 0;
      let fresh0 = tmpx;
      tmpx = tmpx.offset(1);
      let fresh1 = tmpy;
      tmpy = tmpy.offset(-1);
      t = (c0 as pstm_word).wrapping_add((*fresh0 as pstm_word).wrapping_mul(*fresh1 as pstm_word));
      c0 = t as pstm_digit;
      t = (c1 as pstm_word).wrapping_add(t >> 32i32);
      c1 = t as pstm_digit;
      c2 =
        (c2 as libc::c_uint).wrapping_add((t >> 32i32) as pstm_digit) as pstm_digit as pstm_digit;
      iz += 1
    }
    /* store term */
    *dst.offset(ix as isize) = c0;
    ix += 1
  }
  /*
   setup dest
  */
  iy = (*C).used;
  (*C).used = pa;
  (*C).sign = (*A).sign ^ (*B).sign;
  let mut tmpc: *mut pstm_digit = 0 as *mut pstm_digit;
  tmpc = (*C).dp;
  ix = 0i32;
  while ix < pa {
    let fresh2 = tmpc;
    tmpc = tmpc.offset(1);
    *fresh2 = *dst.offset(ix as isize);
    ix += 1
  }
  /*
     clear unused digits [that existed in the old copy of c]
  */
  while ix < iy {
    let fresh3 = tmpc;
    tmpc = tmpc.offset(1);
    *fresh3 = 0i32 as pstm_digit;
    ix += 1
  }
  pstm_clamp(C);
  if paD.is_null() || paDfail == 1i32 {
    free(dst as *mut libc::c_void);
  }
  return 0i32;
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
/* *****************************************************************************/
/* USE_1024_KEY_SPEED_OPTIMIZATIONS */
/* USE_2048_KEY_SPEED_OPTIMIZATIONS */
/* *****************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pstm_mul_comba(
  mut A: *mut pstm_int,
  mut B: *mut pstm_int,
  mut C: *mut pstm_int,
  mut paD: *mut pstm_digit,
  mut paDlen: uint32,
) -> int32 {
  /* USE_2048_KEY_SPEED_OPTIMIZATIONS */
  return pstm_mul_comba_gen(A, B, C, paD, paDlen);
}
/* *****************************************************************************/
/* !DISABLE_PSTM */

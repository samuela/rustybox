use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;

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
/* from wolfssl-3.15.3/wolfcrypt/src/aes.c */
unsafe extern "C" fn RIGHTSHIFTX(mut x: *mut byte) {
  /* LITTLE_ENDIAN */
  // In order to use word-sized ops, little-endian needs to byteswap.
  // On x86, code size increase is ~10 bytes compared to byte-by-byte.
  let mut carryIn: u64 = if *x.offset(15) as libc::c_int & 0x1i32 != 0 {
    (0xe1i32 as u64) << 64i32 - 8i32
  } else {
    0 as u64
  };
  // 64-bit code: need to process only 2 words
  let mut tt: u64 = {
    let mut __v: u64 = 0; // zero, or 0x800..00
    let mut __x: u64 = *(x as *mut u64).offset(0);
    if false {
      __v = ((__x as libc::c_ulonglong & 0xff00000000000000u64) >> 56i32
        | (__x as libc::c_ulonglong & 0xff000000000000u64) >> 40i32
        | (__x as libc::c_ulonglong & 0xff0000000000u64) >> 24i32
        | (__x as libc::c_ulonglong & 0xff00000000u64) >> 8i32
        | (__x as libc::c_ulonglong & 0xff000000u64) << 8i32
        | (__x as libc::c_ulonglong & 0xff0000u64) << 24i32
        | (__x as libc::c_ulonglong & 0xff00u64) << 40i32
        | (__x as libc::c_ulonglong & 0xffu64) << 56i32) as u64
    } else {
      let fresh0 = &mut __v;
      let fresh1;
      let fresh2 = __x;
      llvm_asm!("bswap ${0:q}" : "=r" (fresh1) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
    }
    __v
  };
  let mut carryOut: u64 = tt << 64i32 - 1i32;
  tt = tt >> 1i32 ^ carryIn;
  *(x as *mut u64).offset(0) = {
    let mut __v: u64 = 0;
    let mut __x: u64 = tt;
    if false {
      __v = ((__x as libc::c_ulonglong & 0xff00000000000000u64) >> 56i32
        | (__x as libc::c_ulonglong & 0xff000000000000u64) >> 40i32
        | (__x as libc::c_ulonglong & 0xff0000000000u64) >> 24i32
        | (__x as libc::c_ulonglong & 0xff00000000u64) >> 8i32
        | (__x as libc::c_ulonglong & 0xff000000u64) << 8i32
        | (__x as libc::c_ulonglong & 0xff0000u64) << 24i32
        | (__x as libc::c_ulonglong & 0xff00u64) << 40i32
        | (__x as libc::c_ulonglong & 0xffu64) << 56i32) as u64
    } else {
      let fresh3 = &mut __v;
      let fresh4;
      let fresh5 = __x;
      llvm_asm!("bswap ${0:q}" : "=r" (fresh4) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
    }
    __v
  };
  tt = {
    let mut __v: u64 = 0;
    let mut __x: u64 = *(x as *mut u64).offset(1);
    if false {
      __v = ((__x as libc::c_ulonglong & 0xff00000000000000u64) >> 56i32
        | (__x as libc::c_ulonglong & 0xff000000000000u64) >> 40i32
        | (__x as libc::c_ulonglong & 0xff0000000000u64) >> 24i32
        | (__x as libc::c_ulonglong & 0xff00000000u64) >> 8i32
        | (__x as libc::c_ulonglong & 0xff000000u64) << 8i32
        | (__x as libc::c_ulonglong & 0xff0000u64) << 24i32
        | (__x as libc::c_ulonglong & 0xff00u64) << 40i32
        | (__x as libc::c_ulonglong & 0xffu64) << 56i32) as u64
    } else {
      let fresh6 = &mut __v;
      let fresh7;
      let fresh8 = __x;
      llvm_asm!("bswap ${0:q}" : "=r" (fresh7) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh6, fresh8)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
    }
    __v
  };
  tt = tt >> 1i32 ^ carryOut;
  *(x as *mut u64).offset(1) = {
    let mut __v: u64 = 0;
    let mut __x: u64 = tt;
    if false {
      __v = ((__x as libc::c_ulonglong & 0xff00000000000000u64) >> 56i32
        | (__x as libc::c_ulonglong & 0xff000000000000u64) >> 40i32
        | (__x as libc::c_ulonglong & 0xff0000000000u64) >> 24i32
        | (__x as libc::c_ulonglong & 0xff00000000u64) >> 8i32
        | (__x as libc::c_ulonglong & 0xff000000u64) << 8i32
        | (__x as libc::c_ulonglong & 0xff0000u64) << 24i32
        | (__x as libc::c_ulonglong & 0xff00u64) << 40i32
        | (__x as libc::c_ulonglong & 0xffu64) << 56i32) as u64
    } else {
      let fresh9 = &mut __v;
      let fresh10;
      let fresh11 = __x;
      llvm_asm!("bswap ${0:q}" : "=r" (fresh10) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh9, fresh11)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
    }
    __v
  };
  /* LITTLE_ENDIAN */
}
// Caller guarantees X is aligned
unsafe extern "C" fn GMULT(mut X: *mut byte, mut Y: *mut byte) {
  let mut Z: [byte; 16] = [0; 16];
  //byte V[AES_BLOCK_SIZE] ALIGNED_long;
  let mut i: libc::c_int = 0;
  memset(
    Z.as_mut_ptr() as *mut libc::c_void,
    0,
    16i32 as libc::c_ulong,
  );
  //XMEMCPY(V, X, AES_BLOCK_SIZE);
  i = 0;
  while i < 16i32 {
    let mut y: u32 = (0x800000i32 | *Y.offset(i as isize) as libc::c_int) as u32;
    loop {
      // for every bit in Y[i], from msb to lsb
      if y & 0x80i32 as libc::c_uint != 0 {
        crate::networking::tls::xorbuf_aligned_AES_BLOCK_SIZE(
          Z.as_mut_ptr() as *mut libc::c_void,
          X as *const libc::c_void,
        );
        // was V, not X
      } // was V, not X
      RIGHTSHIFTX(X);
      y = y << 1i32;
      if (y as i32) < 0 {
        break;
      }
    }
    i += 1
  }
  memcpy(
    X as *mut libc::c_void,
    Z.as_mut_ptr() as *const libc::c_void,
    16i32 as libc::c_ulong,
  );
}
/*
 * Copyright (C) 2018 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn aesgcm_GHASH(
  mut h: *mut byte,
  mut a: *const byte,
  mut c: *const byte,
  mut cSz: libc::c_uint,
  mut s: *mut byte,
)
//, unsigned sSz
{
  let mut x: [byte; 16] = [0; 16];
  //    byte scratch[AES_BLOCK_SIZE] ALIGNED_long;
  let mut blocks: libc::c_uint = 0;
  let mut partial: libc::c_uint = 0;
  //was: byte* h = aes->H;
  //XMEMSET(x, 0, AES_BLOCK_SIZE);
  /* Hash in A, the Additional Authentication Data */
  //    if (aSz != 0 && a != NULL) {
  //        blocks = aSz / AES_BLOCK_SIZE;
  //        partial = aSz % AES_BLOCK_SIZE;
  //        while (blocks--) {
  //xorbuf(x, a, AES_BLOCK_SIZE);
  memcpy(
    x.as_mut_ptr() as *mut libc::c_void,
    a as *const libc::c_void,
    16i32 as libc::c_ulong,
  ); // memcpy(x,a) = memset(x,0)+xorbuf(x,a)
  GMULT(x.as_mut_ptr(), h);
  //            a += AES_BLOCK_SIZE;
  //        }
  //        if (partial != 0) {
  //            XMEMSET(scratch, 0, AES_BLOCK_SIZE);
  //            XMEMCPY(scratch, a, partial);
  //            xorbuf(x, scratch, AES_BLOCK_SIZE);
  //            GMULT(x, h);
  //        }
  //    }
  /* Hash in C, the Ciphertext */
  if cSz != 0 as libc::c_uint {
    /*&& c != NULL*/
    blocks = cSz.wrapping_div(16i32 as libc::c_uint);
    partial = cSz.wrapping_rem(16i32 as libc::c_uint);
    loop {
      let fresh12 = blocks;
      blocks = blocks.wrapping_sub(1);
      if !(fresh12 != 0) {
        break;
      }
      // c is not guaranteed to be aligned
      crate::networking::tls::xorbuf_aligned_AES_BLOCK_SIZE(
        x.as_mut_ptr() as *mut libc::c_void,
        c as *const libc::c_void,
      );
      GMULT(x.as_mut_ptr(), h);
      c = c.offset(16)
    }
    if partial != 0 as libc::c_uint {
      //XMEMSET(scratch, 0, AES_BLOCK_SIZE);
      //XMEMCPY(scratch, c, partial);
      //xorbuf(x, scratch, AES_BLOCK_SIZE);
      crate::networking::tls::xorbuf(
        x.as_mut_ptr() as *mut libc::c_void,
        c as *const libc::c_void,
        partial,
      ); //same result as above
      GMULT(x.as_mut_ptr(), h);
    }
  }
  /* Hash in the lengths of A and C in bits */
  //FlattenSzInBits(&scratch[0], aSz);
  //FlattenSzInBits(&scratch[8], cSz);
  //xorbuf_aligned_AES_BLOCK_SIZE(x, scratch);
  // simpler:
  //P32(x)[0] ^= 0;
  let ref mut fresh16 = *(x.as_mut_ptr() as *mut u32).offset(1);
  *fresh16 ^= {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = (13i32 * 8i32) as libc::c_uint;
    if false {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh13 = &mut __v;
      let fresh14;
      let fresh15 = __x;
      llvm_asm!("bswap $0" : "=r" (fresh14) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh13, fresh15)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh13, fresh15, fresh14);
    }
    __v
  };
  //P32(x)[2] ^= 0;
  let ref mut fresh20 = *(x.as_mut_ptr() as *mut u32).offset(3);
  *fresh20 ^= {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = cSz.wrapping_mul(8i32 as libc::c_uint);
    if false {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh17 = &mut __v;
      let fresh18;
      let fresh19 = __x;
      llvm_asm!("bswap $0" : "=r" (fresh18) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh17, fresh19)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh17, fresh19, fresh18);
    }
    __v
  };
  GMULT(x.as_mut_ptr(), h);
  /* Copy the result into s. */
  memcpy(
    s as *mut libc::c_void,
    x.as_mut_ptr() as *const libc::c_void,
    16i32 as libc::c_ulong,
  );
}

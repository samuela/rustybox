use libc;
use libc::free;
extern "C" {
  #[no_mangle]
  fn pstm_mul_comba(
    A: *mut pstm_int,
    B: *mut pstm_int,
    C: *mut pstm_int,
    paD: *mut pstm_digit,
    paDlen: uint32,
  ) -> int32;
  #[no_mangle]
  fn pstm_to_unsigned_bin(a: *mut pstm_int, b: *mut libc::c_uchar) -> int32;
  #[no_mangle]
  fn pstm_add(a: *mut pstm_int, b: *mut pstm_int, c: *mut pstm_int) -> int32;
  #[no_mangle]
  fn pstm_exptmod(G: *mut pstm_int, X: *mut pstm_int, P: *mut pstm_int, Y: *mut pstm_int) -> int32;
  #[no_mangle]
  fn pstm_mulmod(a: *mut pstm_int, b: *mut pstm_int, c: *mut pstm_int, d: *mut pstm_int) -> int32;
  #[no_mangle]
  fn pstm_sub(a: *mut pstm_int, b: *mut pstm_int, c: *mut pstm_int) -> int32;
  #[no_mangle]
  fn pstm_cmp(a: *mut pstm_int, b: *mut pstm_int) -> int32;
  #[no_mangle]
  fn pstm_clear(a: *mut pstm_int);
  #[no_mangle]
  fn pstm_unsigned_bin_size(a: *mut pstm_int) -> int32;
  #[no_mangle]
  fn pstm_read_unsigned_bin(a: *mut pstm_int, b: *mut libc::c_uchar, c: int32) -> int32;
  #[no_mangle]
  fn pstm_init_for_read_unsigned_bin(a: *mut pstm_int, len: uint32) -> int32;
  #[no_mangle]
  fn pstm_init_size(a: *mut pstm_int, size: uint32) -> int32;
  #[no_mangle]
  fn tls_get_random(buf: *mut libc::c_void, len: libc::c_uint);
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
}



pub type uint32 = u32;
pub type int32 = i32;
pub type pstm_digit = uint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pstm_int {
  pub used: libc::c_int,
  pub alloc: libc::c_int,
  pub sign: libc::c_int,
  pub dp: *mut pstm_digit,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct psRsaKey_t {
  pub e: pstm_int,
  pub d: pstm_int,
  pub N: pstm_int,
  pub qP: pstm_int,
  pub dP: pstm_int,
  pub dQ: pstm_int,
  pub p: pstm_int,
  pub q: pstm_int,
  pub size: uint32,
  pub optimized: int32,
}
/*
 * Copyright (C) 2017 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* The code below is taken from parts of
 *  matrixssl-3-7-2b-open/crypto/pubkey/pkcs.c
 *  matrixssl-3-7-2b-open/crypto/pubkey/rsa.c
 * and (so far) almost not modified. Changes are flagged with //bbox
 */
unsafe extern "C" fn pkcs1Pad(
  mut in_0: *mut libc::c_uchar,
  mut inlen: uint32,
  mut out: *mut libc::c_uchar,
  mut outlen: uint32,
  mut cryptType: int32,
) -> int32 {
  let mut c: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
  let mut randomLen: int32 = 0;
  randomLen = outlen
    .wrapping_sub(3i32 as libc::c_uint)
    .wrapping_sub(inlen) as int32;
  if randomLen < 8i32 {
    bb_simple_error_msg_and_die(b"pkcs1Pad failure\n\x00" as *const u8 as *const libc::c_char);
  }
  c = out;
  *c = 0i32 as libc::c_uchar;
  c = c.offset(1);
  *c = cryptType as libc::c_uchar;
  c = c.offset(1);
  if cryptType == 0x1i32 {
    loop {
      let fresh0 = randomLen;
      randomLen = randomLen - 1;
      if !(fresh0 > 0i32) {
        break;
      }
      let fresh1 = c;
      c = c.offset(1);
      *fresh1 = 0xffi32 as libc::c_uchar
    }
  } else {
    tls_get_random(c as *mut libc::c_void, randomLen as uint32);
    if 0i32 < 0i32 {
      return -7i32;
    }
    loop
    /*
        SECURITY:  Read through the random data and change all 0x0 to 0x01.
        This is per spec that no random bytes should be 0
    */
    {
      let fresh2 = randomLen;
      randomLen = randomLen - 1;
      if !(fresh2 > 0i32) {
        break;
      }
      if *c as libc::c_int == 0i32 {
        *c = 0x1i32 as libc::c_uchar
      }
      c = c.offset(1)
    }
  }
  *c = 0i32 as libc::c_uchar;
  c = c.offset(1);
  memcpy(
    c as *mut libc::c_void,
    in_0 as *const libc::c_void,
    inlen as libc::c_ulong,
  );
  return outlen as int32;
}
unsafe extern "C" fn psRsaCrypt(
  mut in_0: *const libc::c_uchar,
  mut inlen: uint32,
  mut out: *mut libc::c_uchar,
  mut outlen: *mut uint32,
  mut key: *mut psRsaKey_t,
  mut type_0: int32,
) -> int32 {
  let mut current_block: u64;
  let mut tmp: pstm_int = pstm_int {
    used: 0,
    alloc: 0,
    sign: 0,
    dp: 0 as *mut pstm_digit,
  };
  let mut tmpa: pstm_int = pstm_int {
    used: 0,
    alloc: 0,
    sign: 0,
    dp: 0 as *mut pstm_digit,
  };
  let mut tmpb: pstm_int = pstm_int {
    used: 0,
    alloc: 0,
    sign: 0,
    dp: 0 as *mut pstm_digit,
  };
  let mut res: int32 = 0;
  let mut x: uint32 = 0;
  //bbox
  //	if (in == NULL || out == NULL || outlen == NULL || key == NULL) {
  //		psTraceCrypto("NULL parameter error in psRsaCrypt\n");
  //		return PS_ARG_FAIL;
  //	}
  tmpb.dp = 0 as *mut pstm_digit;
  tmpa.dp = tmpb.dp;
  tmp.dp = tmpa.dp;
  /* Init and copy into tmp */
  if pstm_init_for_read_unsigned_bin(
    &mut tmp,
    (inlen as libc::c_ulong).wrapping_add(::std::mem::size_of::<pstm_digit>() as libc::c_ulong)
      as uint32,
  ) != 0i32
  {
    return -1i32;
  }
  if pstm_read_unsigned_bin(&mut tmp, in_0 as *mut libc::c_uchar, inlen as int32) != 0i32 {
    pstm_clear(&mut tmp);
    return -1i32;
  }
  /* Sanity check on the input */
  if pstm_cmp(&mut (*key).N, &mut tmp) == -1i32 {
    res = -9i32
  } else {
    if type_0 == 0x2i32 {
      if (*key).optimized != 0 {
        if pstm_init_size(&mut tmpa, (*key).p.alloc as uint32) != 0i32 {
          res = -1i32;
          current_block = 6275814254496255747;
        } else if pstm_init_size(&mut tmpb, (*key).q.alloc as uint32) != 0i32 {
          pstm_clear(&mut tmpa);
          res = -1i32;
          current_block = 6275814254496255747;
        } else {
          if pstm_exptmod(&mut tmp, &mut (*key).dP, &mut (*key).p, &mut tmpa) != 0i32 {
            bb_simple_error_msg_and_die(
              b"decrypt error: pstm_exptmod dP, p\n\x00" as *const u8 as *const libc::c_char,
            );
          } else if pstm_exptmod(&mut tmp, &mut (*key).dQ, &mut (*key).q, &mut tmpb) != 0i32 {
            bb_simple_error_msg_and_die(
              b"decrypt error: pstm_exptmod dQ, q\n\x00" as *const u8 as *const libc::c_char,
            );
          } else if pstm_sub(&mut tmpa, &mut tmpb, &mut tmp) != 0i32 {
            bb_simple_error_msg_and_die(
              b"decrypt error: sub tmpb, tmp\n\x00" as *const u8 as *const libc::c_char,
            );
          } else if pstm_mulmod(&mut tmp, &mut (*key).qP, &mut (*key).p, &mut tmp) != 0i32 {
            bb_simple_error_msg_and_die(
              b"decrypt error: pstm_mulmod qP, p\n\x00" as *const u8 as *const libc::c_char,
            );
          } else if pstm_mul_comba(
            &mut tmp,
            &mut (*key).q,
            &mut tmp,
            0 as *mut pstm_digit,
            0i32 as uint32,
          ) != 0i32
          {
            bb_simple_error_msg_and_die(
              b"decrypt error: pstm_mul q \n\x00" as *const u8 as *const libc::c_char,
            );
          } else if pstm_add(&mut tmp, &mut tmpb, &mut tmp) != 0i32 {
            bb_simple_error_msg_and_die(
              b"decrypt error: pstm_add tmp \n\x00" as *const u8 as *const libc::c_char,
            );
          }
          current_block = 2480299350034459858;
        }
      } else if pstm_exptmod(&mut tmp, &mut (*key).d, &mut (*key).N, &mut tmp) != 0i32 {
        bb_simple_error_msg_and_die(
          b"psRsaCrypt error: pstm_exptmod\n\x00" as *const u8 as *const libc::c_char,
        );
      } else {
        current_block = 2480299350034459858;
      }
    } else {
      if type_0 == 0x1i32 {
        if pstm_exptmod(&mut tmp, &mut (*key).e, &mut (*key).N, &mut tmp) != 0i32 {
          bb_simple_error_msg_and_die(
            b"psRsaCrypt error: pstm_exptmod\n\x00" as *const u8 as *const libc::c_char,
          );
        }
      } else {
        bb_simple_error_msg_and_die(
          b"psRsaCrypt error: invalid type param\n\x00" as *const u8 as *const libc::c_char,
        );
      }
      current_block = 2480299350034459858;
    }
    match current_block {
      6275814254496255747 => {}
      _ => {
        /* Read it back */
        x = pstm_unsigned_bin_size(&mut (*key).N) as uint32;
        if x > *outlen {
          res = -1i32;
          bb_simple_error_msg_and_die(
            b"psRsaCrypt error: pstm_unsigned_bin_size\n\x00" as *const u8 as *const libc::c_char,
          );
        } else {
          /* We want the encrypted value to always be the key size.  Pad with 0x0 */
          while (x as libc::c_ulong) < (*key).size as libc::c_ulong {
            let fresh3 = out;
            out = out.offset(1);
            *fresh3 = 0i32 as libc::c_uchar;
            x = x.wrapping_add(1)
          }
          *outlen = x;
          /* Convert it */
          memset(out as *mut libc::c_void, 0i32, x as libc::c_ulong);
          if pstm_to_unsigned_bin(
            &mut tmp,
            out.offset(x.wrapping_sub(pstm_unsigned_bin_size(&mut tmp) as libc::c_uint) as isize),
          ) != 0i32
          {
            bb_simple_error_msg_and_die(
              b"psRsaCrypt error: pstm_to_unsigned_bin\n\x00" as *const u8 as *const libc::c_char,
            );
          } else {
            /* Clean up and return */
            res = 0i32
          }
        }
      }
    }
  }
  if type_0 == 0x2i32 && (*key).optimized != 0 {
    //pstm_clear_multi(&tmpa, &tmpb, NULL, NULL, NULL, NULL, NULL, NULL);
    pstm_clear(&mut tmpa);
    pstm_clear(&mut tmpb);
  }
  pstm_clear(&mut tmp);
  return res;
}
/*
 * Copyright (C) 2017 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 *
 * Selected few declarations for RSA.
 */
/* Size of the key in bytes */
/* 1 for optimized */
//bbox	psPool_t *pool;
#[no_mangle]
pub unsafe extern "C" fn psRsaEncryptPub(
  mut key: *mut psRsaKey_t,
  mut in_0: *mut libc::c_uchar,
  mut inlen: uint32,
  mut out: *mut libc::c_uchar,
  mut outlen: uint32,
) -> int32 {
  let mut err: int32 = 0;
  let mut size: uint32 = 0;
  size = (*key).size;
  if outlen < size {
    //bbox		psTraceCrypto("Error on bad outlen parameter to psRsaEncryptPub\n");
    bb_error_msg_and_die(
      b"RSA crypt outlen:%d < size:%d\x00" as *const u8 as *const libc::c_char,
      outlen,
      size,
    );
  }
  err = pkcs1Pad(in_0, inlen, out, size, 0x2i32);
  if err < 0i32 {
    bb_simple_error_msg_and_die(
      b"Error padding psRsaEncryptPub. Likely data too long\n\x00" as *const u8
        as *const libc::c_char,
    );
  }
  err = psRsaCrypt(out, size, out, &mut outlen as *mut uint32, key, 0x1i32);
  if err < 0i32 {
    bb_simple_error_msg_and_die(
      b"Error performing psRsaEncryptPub\n\x00" as *const u8 as *const libc::c_char,
    );
  }
  if outlen != size {
    bb_simple_error_msg_and_die(
      b"Encrypted size error in psRsaEncryptPub\n\x00" as *const u8 as *const libc::c_char,
    );
  }
  return size as int32;
}

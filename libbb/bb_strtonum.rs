use libc;
use libc::free;

extern "C" {
  #[no_mangle]
  fn strtol(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_long;

  #[no_mangle]
  fn strtoul(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_ulong;

  #[no_mangle]
  fn strtoll(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_longlong;

  #[no_mangle]
  fn strtoull(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_ulonglong;

  #[no_mangle]
  static bb_errno: *mut libc::c_int;
}

#[inline(always)]
unsafe extern "C" fn bb_ascii_isalnum(mut a: libc::c_uchar) -> libc::c_int {
  let mut b: libc::c_uchar = (a as libc::c_int - '0' as i32) as libc::c_uchar;
  if b as libc::c_int <= 9i32 {
    return (b as libc::c_int <= 9i32) as libc::c_int;
  }
  b = ((a as libc::c_int | 0x20i32) - 'a' as i32) as libc::c_uchar;
  return (b as libc::c_int <= 'z' as i32 - 'a' as i32) as libc::c_int;
}

/*
 * Utility routines.
 *
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* On exit: errno = 0 only if there was non-empty, '\0' terminated value
 * errno = EINVAL if value was not '\0' terminated, but otherwise ok
 *    Return value is still valid, caller should just check whether end[0]
 *    is a valid terminating char for particular case. OTOH, if caller
 *    requires '\0' terminated input, [s]he can just check errno == 0.
 * errno = ERANGE if value had alphanumeric terminating char ("1234abcg").
 * errno = ERANGE if value is out of range, missing, etc.
 * errno = ERANGE if value had minus sign for strtouXX (even "-0" is not ok )
 *    return value is all-ones in this case.
 *
 * Test code:
 * char *endptr;
 * const char *minus = "-";
 * errno = 0;
 * bb_strtoi(minus, &endptr, 0); // must set ERANGE
 * printf("minus:%p endptr:%p errno:%d EINVAL:%d\n", minus, endptr, errno, EINVAL);
 * errno = 0;
 * bb_strtoi("-0-", &endptr, 0); // must set EINVAL and point to second '-'
 * printf("endptr[0]:%c errno:%d EINVAL:%d\n", endptr[0], errno, EINVAL);
 */
unsafe extern "C" fn ret_ERANGE() -> libc::c_ulonglong {
  *bb_errno = 34i32; /* this ain't as small as it looks (on glibc) */
  return (9223372036854775807i64 as libc::c_ulonglong)
    .wrapping_mul(2u64)
    .wrapping_add(1u64);
}
unsafe extern "C" fn handle_errors(
  mut v: libc::c_ulonglong,
  mut endp: *mut *mut libc::c_char,
) -> libc::c_ulonglong {
  let mut next_ch: libc::c_char = **endp;
  /* errno is already set to ERANGE by strtoXXX if value overflowed */
  if next_ch != 0 {
    /* "1234abcg" or out-of-range? */
    if bb_ascii_isalnum(next_ch as libc::c_uchar) != 0 || *bb_errno != 0 {
      return ret_ERANGE();
    }
    /* good number, just suspicious terminator */
    *bb_errno = 22i32
  }
  return v;
}
#[no_mangle]
pub unsafe extern "C" fn bb_strtoull(
  mut arg: *const libc::c_char,
  mut endp: *mut *mut libc::c_char,
  mut base: libc::c_int,
) -> libc::c_ulonglong {
  let mut v: libc::c_ulonglong = 0;
  let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
  if endp.is_null() {
    endp = &mut endptr
  }
  *endp = arg as *mut libc::c_char;
  /* strtoul("  -4200000000") returns 94967296, errno 0 (!) */
  /* I don't think that this is right. Preventing this... */
  if bb_ascii_isalnum(*arg.offset(0) as libc::c_uchar) == 0 {
    return ret_ERANGE();
  }
  /* not 100% correct for lib func, but convenient for the caller */
  *bb_errno = 0i32;
  v = strtoull(arg, endp, base);
  return handle_errors(v, endp);
}
#[no_mangle]
pub unsafe extern "C" fn bb_strtoll(
  mut arg: *const libc::c_char,
  mut endp: *mut *mut libc::c_char,
  mut base: libc::c_int,
) -> libc::c_longlong {
  let mut v: libc::c_ulonglong = 0;
  let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut first: libc::c_char = 0;
  if endp.is_null() {
    endp = &mut endptr
  }
  *endp = arg as *mut libc::c_char;
  /* Check for the weird "feature":
   * a "-" string is apparently a valid "number" for strto[u]l[l]!
   * It returns zero and errno is 0! :( */
  first = if *arg.offset(0) as libc::c_int != '-' as i32 {
    *arg.offset(0) as libc::c_int
  } else {
    *arg.offset(1) as libc::c_int
  } as libc::c_char;
  if bb_ascii_isalnum(first as libc::c_uchar) == 0 {
    return ret_ERANGE() as libc::c_longlong;
  }
  *bb_errno = 0i32;
  v = strtoll(arg, endp, base) as libc::c_ulonglong;
  return handle_errors(v, endp) as libc::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn bb_strtou(
  mut arg: *const libc::c_char,
  mut endp: *mut *mut libc::c_char,
  mut base: libc::c_int,
) -> libc::c_uint {
  let mut v: libc::c_ulong = 0;
  let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
  if endp.is_null() {
    endp = &mut endptr
  }
  *endp = arg as *mut libc::c_char;
  if bb_ascii_isalnum(*arg.offset(0) as libc::c_uchar) == 0 {
    return ret_ERANGE() as libc::c_uint;
  }
  *bb_errno = 0i32;
  v = strtoul(arg, endp, base);
  if v
    > (2147483647i32 as libc::c_uint)
      .wrapping_mul(2u32)
      .wrapping_add(1u32) as libc::c_ulong
  {
    return ret_ERANGE() as libc::c_uint;
  }
  return handle_errors(v as libc::c_ulonglong, endp) as libc::c_uint;
}

/*
 * ascii-to-numbers implementations for busybox
 *
 * Copyright (C) 2003  Manuel Novoa III  <mjn3@codepoet.org>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* Provides extern declarations of functions */
/* Unsigned long long functions always exist */
/* Provides inline definitions of functions */
/* (useful for mapping them to the type of the same width) */
/* If long == long long, then just map them one-to-one */
/* Same for int -> [long] long */
/* Specialized */
/* Non-aborting kind of convertors: bb_strto[u][l]l */
/* On exit: errno = 0 only if there was non-empty, '\0' terminated value
 * errno = EINVAL if value was not '\0' terminated, but otherwise ok
 *    Return value is still valid, caller should just check whether end[0]
 *    is a valid terminating char for particular case. OTOH, if caller
 *    requires '\0' terminated input, [s]he can just check errno == 0.
 * errno = ERANGE if value had alphanumeric terminating char ("1234abcg").
 * errno = ERANGE if value is out of range, missing, etc.
 * errno = ERANGE if value had minus sign for strtouXX (even "-0" is not ok )
 *    return value is all-ones in this case.
 */
#[no_mangle]
pub unsafe extern "C" fn bb_strtoi(
  mut arg: *const libc::c_char,
  mut endp: *mut *mut libc::c_char,
  mut base: libc::c_int,
) -> libc::c_int {
  let mut v: libc::c_long = 0;
  let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut first: libc::c_char = 0;
  if endp.is_null() {
    endp = &mut endptr
  }
  *endp = arg as *mut libc::c_char;
  first = if *arg.offset(0) as libc::c_int != '-' as i32 {
    *arg.offset(0) as libc::c_int
  } else {
    *arg.offset(1) as libc::c_int
  } as libc::c_char;
  if bb_ascii_isalnum(first as libc::c_uchar) == 0 {
    return ret_ERANGE() as libc::c_int;
  }
  *bb_errno = 0i32;
  v = strtol(arg, endp, base);
  if v > 2147483647i32 as libc::c_long {
    return ret_ERANGE() as libc::c_int;
  }
  if v < (-2147483647i32 - 1i32) as libc::c_long {
    return ret_ERANGE() as libc::c_int;
  }
  return handle_errors(v as libc::c_ulonglong, endp) as libc::c_int;
}

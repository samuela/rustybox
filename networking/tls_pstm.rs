use libc;
extern "C" {
  #[no_mangle]
  fn pstm_sqr_comba(
    A: *mut pstm_int,
    B: *mut pstm_int,
    paD: *mut pstm_digit,
    paDlen: uint32,
  ) -> int32;
  #[no_mangle]
  fn pstm_montgomery_reduce(
    a: *mut pstm_int,
    m: *mut pstm_int,
    mp: pstm_digit,
    paD: *mut pstm_digit,
    paDlen: uint32,
  ) -> int32;
  #[no_mangle]
  fn pstm_mul_comba(
    A: *mut pstm_int,
    B: *mut pstm_int,
    C: *mut pstm_int,
    paD: *mut pstm_digit,
    paDlen: uint32,
  ) -> int32;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
}
pub type __int32_t = libc::c_int;
use crate::librb::__uint32_t;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
use crate::librb::uint32_t;
use crate::librb::uint64_t;
use crate::librb::size_t;
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
//bbox: was int16 b
/* *****************************************************************************/
/*
 init an pstm_int for a given size
*/
#[no_mangle]
pub unsafe extern "C" fn pstm_init_size(mut a: *mut pstm_int, mut size: uint32) -> int32 {
  //bbox
  //	uint16		x;
  /*
   alloc mem
  */
  (*a).dp = xzalloc(
    (::std::mem::size_of::<pstm_digit>() as libc::c_ulong).wrapping_mul(size as libc::c_ulong),
  ) as *mut pstm_digit; //bbox
                        //bbox	a->pool = pool;
  (*a).used = 0i32;
  (*a).alloc = size as libc::c_int;
  (*a).sign = 0i32;
  /*
   zero the digits
  */
  //bbox
  //	for (x = 0; x < size; x++) {
  //		a->dp[x] = 0;
  //	}
  return 0i32;
}
/* *****************************************************************************/
/*
  Init a new pstm_int.
*/
unsafe extern "C" fn pstm_init(mut a: *mut pstm_int) -> int32 {
  //bbox
  //	int32		i;
  /*
   allocate memory required and clear it
  */
  (*a).dp = xzalloc(
    (::std::mem::size_of::<pstm_digit>() as libc::c_ulong).wrapping_mul(64i32 as libc::c_ulong),
  ) as *mut pstm_digit; //bbox
                        /*
                         set the digits to zero
                        */
  //bbox
  //	for (i = 0; i < PSTM_DEFAULT_INIT; i++) {
  //		a->dp[i] = 0;
  //	}
  /*
   set the used to zero, allocated digits to the default precision and sign
   to positive
  */
  //bbox	a->pool = pool;
  (*a).used = 0i32;
  (*a).alloc = 64i32;
  (*a).sign = 0i32;
  return 0i32;
}
/* *****************************************************************************/
/*
 Grow as required
*/
#[no_mangle]
pub unsafe extern "C" fn pstm_grow(mut a: *mut pstm_int, mut size: libc::c_int) -> int32 {
  let mut i: libc::c_int = 0; //bbox: was int16
  let mut tmp: *mut pstm_digit = 0 as *mut pstm_digit;
  /*
   If the alloc size is smaller alloc more ram.
  */
  if (*a).alloc < size {
    /*
        Reallocate the array a->dp

        We store the return in a temporary variable in case the operation
        failed we don't want to overwrite the dp member of a.
    */
    tmp = xrealloc(
      (*a).dp as *mut libc::c_void,
      (::std::mem::size_of::<pstm_digit>() as libc::c_ulong).wrapping_mul(size as libc::c_ulong),
    ) as *mut pstm_digit; //bbox
                          /*
                             reallocation succeeded so set a->dp
                          */
    (*a).dp = tmp;
    /*
       zero excess digits
    */
    i = (*a).alloc;
    (*a).alloc = size;
    while i < (*a).alloc {
      *(*a).dp.offset(i as isize) = 0i32 as pstm_digit;
      i += 1
    }
  }
  return 0i32;
}
/* *****************************************************************************/
/*
 copy, b = a (b must be pre-allocated)
*/
#[no_mangle]
pub unsafe extern "C" fn pstm_copy(mut a: *mut pstm_int, mut b: *mut pstm_int) -> int32 {
  let mut res: int32 = 0;
  let mut n: int32 = 0;
  /*
   If dst == src do nothing
  */
  if a == b {
    return 0i32;
  }
  /*
   Grow dest
  */
  if (*b).alloc < (*a).used {
    pstm_grow(b, (*a).used);
    res = 0i32;
    if res != 0i32 {
      return res;
    }
  }
  /*
   Zero b and copy the parameters over
  */
  let mut tmpa: *mut pstm_digit = 0 as *mut pstm_digit;
  let mut tmpb: *mut pstm_digit = 0 as *mut pstm_digit;
  /* pointer aliases */
  /* source */
  tmpa = (*a).dp;
  /* destination */
  tmpb = (*b).dp;
  /* copy all the digits */
  n = 0i32;
  while n < (*a).used {
    let fresh0 = tmpa;
    tmpa = tmpa.offset(1);
    let fresh1 = tmpb;
    tmpb = tmpb.offset(1);
    *fresh1 = *fresh0;
    n += 1
  }
  /* clear high digits */
  while n < (*b).used {
    let fresh2 = tmpb;
    tmpb = tmpb.offset(1);
    *fresh2 = 0i32 as pstm_digit;
    n += 1
  }
  /*
   copy used count and sign
  */
  (*b).used = (*a).used;
  (*b).sign = (*a).sign;
  return 0i32;
}
/* *****************************************************************************/
/*
  Trim unused digits

  This is used to ensure that leading zero digits are trimed and the
  leading "used" digit will be non-zero. Typically very fast.  Also fixes
  the sign if there are no more leading digits
*/
#[no_mangle]
pub unsafe extern "C" fn pstm_clamp(mut a: *mut pstm_int) {
  /*	decrease used while the most significant digit is zero. */
  while (*a).used > 0i32 && *(*a).dp.offset(((*a).used - 1i32) as isize) == 0i32 as libc::c_uint {
    (*a).used -= 1
  }
  /*	reset the sign flag if used == 0 */
  if (*a).used == 0i32 {
    (*a).sign = 0i32
  };
}
/* *****************************************************************************/
/*
 clear one (frees).
*/
#[no_mangle]
pub unsafe extern "C" fn pstm_clear(mut a: *mut pstm_int) {
  let mut i: int32 = 0;
  /*
   only do anything if a hasn't been freed previously
  */
  if !a.is_null() && !(*a).dp.is_null() {
    /*
       first zero the digits
    */
    i = 0i32;
    while i < (*a).used {
      *(*a).dp.offset(i as isize) = 0i32 as pstm_digit;
      i += 1
    }
    free((*a).dp as *mut libc::c_void);
    /*
       reset members to make debugging easier
    */
    (*a).dp = 0 as *mut pstm_digit;
    (*a).used = 0i32;
    (*a).alloc = (*a).used;
    (*a).sign = 0i32
  };
}
/* *****************************************************************************/
/*
 clear many (frees).
*/
//UNUSED
/* *****************************************************************************/
/*
 Set to zero.
*/
unsafe extern "C" fn pstm_zero(mut a: *mut pstm_int) {
  let mut n: int32 = 0;
  let mut tmp: *mut pstm_digit = 0 as *mut pstm_digit;
  (*a).sign = 0i32;
  (*a).used = 0i32;
  tmp = (*a).dp;
  n = 0i32;
  while n < (*a).alloc {
    let fresh3 = tmp;
    tmp = tmp.offset(1);
    *fresh3 = 0i32 as pstm_digit;
    n += 1
  }
}
/* *****************************************************************************/
/*
 Compare maginitude of two ints (unsigned).
*/
#[no_mangle]
pub unsafe extern "C" fn pstm_cmp_mag(mut a: *mut pstm_int, mut b: *mut pstm_int) -> int32 {
  let mut n: libc::c_int = 0; //bbox: was int16
  let mut tmpa: *mut pstm_digit = 0 as *mut pstm_digit;
  let mut tmpb: *mut pstm_digit = 0 as *mut pstm_digit;
  /*
   compare based on # of non-zero digits
  */
  if (*a).used > (*b).used {
    return 1i32;
  }
  if (*a).used < (*b).used {
    return -1i32;
  }
  /* alias for a */
  tmpa = (*a).dp.offset(((*a).used - 1i32) as isize);
  /* alias for b */
  tmpb = (*b).dp.offset(((*a).used - 1i32) as isize);
  /*
   compare based on digits
  */
  n = 0i32;
  while n < (*a).used {
    if *tmpa > *tmpb {
      return 1i32;
    }
    if *tmpa < *tmpb {
      return -1i32;
    }
    n += 1;
    tmpa = tmpa.offset(-1);
    tmpb = tmpb.offset(-1)
  }
  return 0i32;
}
/* *****************************************************************************/
/*
 Compare two ints (signed)
*/
#[no_mangle]
pub unsafe extern "C" fn pstm_cmp(mut a: *mut pstm_int, mut b: *mut pstm_int) -> int32 {
  /*
   compare based on sign
  */
  if (*a).sign != (*b).sign {
    if (*a).sign == 1i32 {
      return -1i32;
    } else {
      return 1i32;
    }
  }
  /*
   compare digits
  */
  if (*a).sign == 1i32 {
    /* if negative compare opposite direction */
    return pstm_cmp_mag(b, a);
  } else {
    return pstm_cmp_mag(a, b);
  };
}
/* *****************************************************************************/
/*
  pstm_ints can be initialized more precisely when they will populated
  using pstm_read_unsigned_bin since the length of the byte stream is known
*/
#[no_mangle]
pub unsafe extern "C" fn pstm_init_for_read_unsigned_bin(
  mut a: *mut pstm_int,
  mut len: uint32,
) -> int32 {
  let mut size: int32 = 0;
  /*
    Need to set this based on how many words max it will take to store the bin.
    The magic + 2:
      1 to round up for the remainder of this integer math
      1 for the initial carry of '1' bits that fall between DIGIT_BIT and 8
  */
  size = (len as libc::c_ulong)
    .wrapping_div(::std::mem::size_of::<pstm_digit>() as libc::c_ulong)
    .wrapping_mul(
      (::std::mem::size_of::<pstm_digit>() as libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong),
    )
    .wrapping_div(32i32 as libc::c_ulong)
    .wrapping_add(2i32 as libc::c_ulong) as int32;
  pstm_init_size(a, size as uint32);
  return 0i32;
}
/* *****************************************************************************/
/*
  Reads a unsigned char array into pstm_int format.  User should have
  called pstm_init_for_read_unsigned_bin first.  There is some grow logic
  here if the default pstm_init was used but we don't really want to hit it.
*/
#[no_mangle]
pub unsafe extern "C" fn pstm_read_unsigned_bin(
  mut a: *mut pstm_int,
  mut b: *mut libc::c_uchar,
  mut c: int32,
) -> int32 {
  /* zero the int */
  pstm_zero(a);
  /*
    If we know the endianness of this architecture, and we're using
    32-bit pstm_digits, we can optimize this
  */
  /* But not for both simultaneously */
  let mut pd: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
  if c as libc::c_uint as libc::c_ulong
    > (4096i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<pstm_digit>() as libc::c_ulong)
  {
    let mut excess: uint32 = (c as libc::c_ulong).wrapping_sub(
      (4096i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<pstm_digit>() as libc::c_ulong),
    ) as uint32;
    c = (c as libc::c_uint).wrapping_sub(excess) as int32 as int32;
    b = b.offset(excess as isize)
  }
  (*a).used = (c as libc::c_ulong)
    .wrapping_add(::std::mem::size_of::<pstm_digit>() as libc::c_ulong)
    .wrapping_sub(1i32 as libc::c_ulong)
    .wrapping_div(::std::mem::size_of::<pstm_digit>() as libc::c_ulong)
    as libc::c_int;
  if (*a).alloc < (*a).used {
    pstm_grow(a, (*a).used);
    if 0i32 != 0i32 {
      return -8i32;
    }
  }
  pd = (*a).dp as *mut libc::c_uchar;
  /* read the bytes in */
  c -= 1i32;
  while c >= 0i32 {
    let fresh4 = b;
    b = b.offset(1);
    *pd.offset(c as isize) = *fresh4;
    c -= 1i32
  }
  pstm_clamp(a);
  return 0i32;
}
/* *****************************************************************************/
/*
*/
unsafe extern "C" fn pstm_count_bits(mut a: *mut pstm_int) -> libc::c_int {
  let mut r: libc::c_int = 0; //bbox: was int16
  let mut q: pstm_digit = 0;
  if (*a).used == 0i32 {
    return 0i32;
  }
  /* get number of digits and add that */
  r = ((*a).used - 1i32) * 32i32;
  /* take the last digit and count the bits in it */
  q = *(*a).dp.offset(((*a).used - 1i32) as isize);
  while q > 0i32 as pstm_digit {
    r += 1;
    q >>= 1i32 as pstm_digit
  }
  return r;
}
/* *****************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pstm_unsigned_bin_size(mut a: *mut pstm_int) -> int32 {
  let mut size: int32 = pstm_count_bits(a);
  return size / 8i32 + (if size & 7i32 != 0i32 { 1i32 } else { 0i32 });
}
/* *****************************************************************************/
unsafe extern "C" fn pstm_set(mut a: *mut pstm_int, mut b: pstm_digit) {
  pstm_zero(a);
  *(*a).dp.offset(0) = b;
  (*a).used = if *(*a).dp.offset(0) != 0 { 1i32 } else { 0i32 };
}
/* *****************************************************************************/
/*
  Right shift
*/
unsafe extern "C" fn pstm_rshd(mut a: *mut pstm_int, mut x: libc::c_int) {
  let mut y: libc::c_int = 0; //bbox: was int16
                              /* too many digits just zero and return */
  if x >= (*a).used {
    pstm_zero(a);
    return;
  }
  /* shift */
  y = 0i32;
  while y < (*a).used - x {
    *(*a).dp.offset(y as isize) = *(*a).dp.offset((y + x) as isize);
    y += 1
  }
  /* zero rest */
  while y < (*a).used {
    *(*a).dp.offset(y as isize) = 0i32 as pstm_digit;
    y += 1
  }
  /* decrement count */
  (*a).used -= x;
  pstm_clamp(a);
}
/* *****************************************************************************/
/*
 Shift left a certain amount of digits.
*/
unsafe extern "C" fn pstm_lshd(mut a: *mut pstm_int, mut b: libc::c_int) -> int32 {
  let mut x: libc::c_int = 0; //bbox: was int16
  let mut res: int32 = 0;
  /*
   If its less than zero return.
  */
  if b <= 0i32 {
    return 0i32;
  }
  /*
   Grow to fit the new digits.
  */
  if (*a).alloc < (*a).used + b {
    pstm_grow(a, (*a).used + b);
    res = 0i32;
    if res != 0i32 {
      return res;
    }
  }
  let mut top: *mut pstm_digit = 0 as *mut pstm_digit;
  let mut bottom: *mut pstm_digit = 0 as *mut pstm_digit;
  /*
     Increment the used by the shift amount then copy upwards.
  */
  (*a).used += b;
  /* top */
  top = (*a).dp.offset((*a).used as isize).offset(-1);
  /* base */
  bottom = (*a)
    .dp
    .offset((*a).used as isize)
    .offset(-1)
    .offset(-(b as isize));
  /*
     This is implemented using a sliding window except the window goes the
     other way around.  Copying from the bottom to the top.
  */
  x = (*a).used - 1i32;
  while x >= b {
    let fresh5 = bottom;
    bottom = bottom.offset(-1);
    let fresh6 = top;
    top = top.offset(-1);
    *fresh6 = *fresh5;
    x -= 1
  }
  /* zero the lower digits */
  top = (*a).dp;
  x = 0i32;
  while x < b {
    let fresh7 = top;
    top = top.offset(1);
    *fresh7 = 0i32 as pstm_digit;
    x += 1
  }
  return 0i32;
}
/* *****************************************************************************/
/*
  computes a = 2**b
*/
unsafe extern "C" fn pstm_2expt(mut a: *mut pstm_int, mut b: libc::c_int) -> int32 {
  let mut z: libc::c_int = 0; //bbox: was int16
                              /* zero a as per default */
  pstm_zero(a);
  if b < 0i32 {
    return 0i32;
  }
  z = b / 32i32;
  if z >= 4096i32 {
    return -9i32;
  }
  /* set the used count of where the bit will go */
  (*a).used = z + 1i32;
  if (*a).used > (*a).alloc {
    pstm_grow(a, (*a).used);
    if 0i32 != 0i32 {
      return -8i32;
    }
  }
  /* put the single bit in its place */
  *(*a).dp.offset(z as isize) = (1i32 as pstm_digit) << b % 32i32;
  return 0i32;
}
/* *****************************************************************************/
/*

*/
#[no_mangle]
pub unsafe extern "C" fn pstm_mul_2(mut a: *mut pstm_int, mut b: *mut pstm_int) -> int32 {
  let mut res: int32 = 0; //bbox: was int16
  let mut x: libc::c_int = 0;
  let mut oldused: libc::c_int = 0;
  /*
   grow to accomodate result
  */
  if (*b).alloc < (*a).used + 1i32 {
    pstm_grow(b, (*a).used + 1i32);
    res = 0i32;
    if res != 0i32 {
      return res;
    }
  }
  oldused = (*b).used;
  (*b).used = (*a).used;
  let mut r: pstm_digit = 0;
  let mut rr: pstm_digit = 0;
  let mut tmpa: *mut pstm_digit = 0 as *mut pstm_digit;
  let mut tmpb: *mut pstm_digit = 0 as *mut pstm_digit;
  /* alias for source */
  tmpa = (*a).dp;
  /* alias for dest */
  tmpb = (*b).dp;
  /* carry */
  r = 0i32 as pstm_digit;
  x = 0i32;
  while x < (*a).used {
    /*
          get what will be the *next* carry bit from the
          MSB of the current digit
    */
    rr = *tmpa >> (32i32 - 1i32) as pstm_digit;
    /*
          now shift up this digit, add in the carry [from the previous]
    */
    let fresh8 = tmpa;
    tmpa = tmpa.offset(1);
    let fresh9 = tmpb;
    tmpb = tmpb.offset(1);
    *fresh9 = *fresh8 << 1i32 as pstm_digit | r;
    /*
          copy the carry that would be from the source
          digit into the next iteration
    */
    r = rr;
    x += 1
  }
  /* new leading digit? */
  if r != 0i32 as libc::c_uint && (*b).used != 4096i32 - 1i32 {
    /* add a MSB which is always 1 at this point */
    *tmpb = 1i32 as pstm_digit;
    (*b).used += 1
  }
  /*
      now zero any excess digits on the destination that we didn't write to
  */
  tmpb = (*b).dp.offset((*b).used as isize);
  x = (*b).used;
  while x < oldused {
    let fresh10 = tmpb;
    tmpb = tmpb.offset(1);
    *fresh10 = 0i32 as pstm_digit;
    x += 1
  }
  (*b).sign = (*a).sign;
  return 0i32;
}
/* *****************************************************************************/
/*
  unsigned subtraction ||a|| >= ||b|| ALWAYS!
*/
#[no_mangle]
pub unsafe extern "C" fn s_pstm_sub(
  mut a: *mut pstm_int,
  mut b: *mut pstm_int,
  mut c: *mut pstm_int,
) -> int32 {
  let mut oldbused: libc::c_int = 0; //bbox: was int16
  let mut oldused: libc::c_int = 0;
  let mut x: int32 = 0;
  let mut t: pstm_word = 0;
  if (*b).used > (*a).used {
    return -9i32;
  }
  if (*c).alloc < (*a).used {
    pstm_grow(c, (*a).used);
    x = 0i32;
    if x != 0i32 {
      return x;
    }
  }
  oldused = (*c).used;
  oldbused = (*b).used;
  (*c).used = (*a).used;
  t = 0i32 as pstm_word;
  x = 0i32;
  while x < oldbused {
    t = (*(*a).dp.offset(x as isize) as pstm_word)
      .wrapping_sub((*(*b).dp.offset(x as isize) as pstm_word).wrapping_add(t));
    *(*c).dp.offset(x as isize) = t as pstm_digit;
    t = t >> 32i32 & 1i32 as libc::c_ulong;
    x += 1
  }
  while x < (*a).used {
    t = (*(*a).dp.offset(x as isize) as pstm_word).wrapping_sub(t);
    *(*c).dp.offset(x as isize) = t as pstm_digit;
    t = t >> 32i32;
    x += 1
  }
  while x < oldused {
    *(*c).dp.offset(x as isize) = 0i32 as pstm_digit;
    x += 1
  }
  pstm_clamp(c);
  return 0i32;
}
/* *****************************************************************************/
/*
  unsigned addition
*/
unsafe extern "C" fn s_pstm_add(
  mut a: *mut pstm_int,
  mut b: *mut pstm_int,
  mut c: *mut pstm_int,
) -> int32 {
  let mut x: libc::c_int = 0; //bbox: was int16
  let mut y: libc::c_int = 0;
  let mut oldused: libc::c_int = 0;
  let mut t: pstm_word = 0;
  let mut adp: pstm_word = 0;
  let mut bdp: pstm_word = 0;
  y = (*a).used;
  if (*b).used > y {
    y = (*b).used
  }
  oldused = (*c).used;
  (*c).used = y;
  if (*c).used > (*c).alloc {
    pstm_grow(c, (*c).used);
    if 0i32 != 0i32 {
      return -8i32;
    }
  }
  t = 0i32 as pstm_word;
  x = 0i32;
  while x < y {
    if (*a).used < x {
      adp = 0i32 as pstm_word
    } else {
      adp = *(*a).dp.offset(x as isize) as pstm_word
    }
    if (*b).used < x {
      bdp = 0i32 as pstm_word
    } else {
      bdp = *(*b).dp.offset(x as isize) as pstm_word
    }
    t = (t as libc::c_ulong).wrapping_add(adp.wrapping_add(bdp)) as pstm_word as pstm_word;
    *(*c).dp.offset(x as isize) = t as pstm_digit;
    t >>= 32i32;
    x += 1
  }
  if t != 0i32 as libc::c_ulong && x < 4096i32 {
    if (*c).used == (*c).alloc {
      pstm_grow(c, (*c).alloc + 1i32);
      if 0i32 != 0i32 {
        return -8i32;
      }
    }
    let fresh11 = (*c).used;
    (*c).used = (*c).used + 1;
    *(*c).dp.offset(fresh11 as isize) = t as pstm_digit;
    x += 1
  }
  (*c).used = x;
  while x < oldused {
    *(*c).dp.offset(x as isize) = 0i32 as pstm_digit;
    x += 1
  }
  pstm_clamp(c);
  return 0i32;
}
/* *****************************************************************************/
/*

*/
#[no_mangle]
pub unsafe extern "C" fn pstm_sub(
  mut a: *mut pstm_int,
  mut b: *mut pstm_int,
  mut c: *mut pstm_int,
) -> int32 {
  let mut res: int32 = 0; //bbox: was int16
  let mut sa: libc::c_int = 0;
  let mut sb: libc::c_int = 0;
  sa = (*a).sign;
  sb = (*b).sign;
  if sa != sb {
    /*
       subtract a negative from a positive, OR a positive from a negative.
       For both, ADD their magnitudes, and use the sign of the first number.
    */
    (*c).sign = sa;
    res = s_pstm_add(a, b, c);
    if res != 0i32 {
      return res;
    }
  } else if pstm_cmp_mag(a, b) != -1i32 {
    /*
       subtract a positive from a positive, OR a negative from a negative.
       First, take the difference between their magnitudes, then...
    */
    /* Copy the sign from the first */
    (*c).sign = sa;
    /* The first has a larger or equal magnitude */
    res = s_pstm_sub(a, b, c);
    if res != 0i32 {
      return res;
    }
  } else {
    /* The result has the _opposite_ sign from the first number. */
    (*c).sign = if sa == 0i32 { 1i32 } else { 0i32 };
    /* The second has a larger magnitude */
    res = s_pstm_sub(b, a, c);
    if res != 0i32 {
      return res;
    }
  }
  return 0i32;
}
/* *****************************************************************************/
/*
  c = a - b
*/
//UNUSED
/* *****************************************************************************/
/*
  setups the montgomery reduction
*/
unsafe extern "C" fn pstm_montgomery_setup(
  mut a: *mut pstm_int,
  mut rho: *mut pstm_digit,
) -> int32 {
  let mut x: pstm_digit = 0;
  let mut b: pstm_digit = 0;
  /*
   fast inversion mod 2**k
   Based on the fact that
   XA = 1 (mod 2**n)	=>  (X(2-XA)) A = 1 (mod 2**2n)
             =>  2*X*A - X*X*A*A = 1
             =>  2*(1) - (1)     = 1
  */
  b = *(*a).dp.offset(0); /* here x*a==1 mod 2**4 */
  if b & 1i32 as libc::c_uint == 0i32 as libc::c_uint {
    bb_simple_error_msg_and_die(
      b"pstm_montogomery_setup failure\n\x00" as *const u8 as *const libc::c_char,
    ); /* here x*a==1 mod 2**8 */
  } /* here x*a==1 mod 2**16 */
  x = ((b.wrapping_add(2i32 as libc::c_uint) & 4i32 as libc::c_uint) << 1i32).wrapping_add(b); /* here x*a==1 mod 2**32 */
  x = (x as libc::c_uint).wrapping_mul((2i32 as libc::c_uint).wrapping_sub(b.wrapping_mul(x)))
    as pstm_digit as pstm_digit;
  x = (x as libc::c_uint).wrapping_mul((2i32 as libc::c_uint).wrapping_sub(b.wrapping_mul(x)))
    as pstm_digit as pstm_digit;
  x = (x as libc::c_uint).wrapping_mul((2i32 as libc::c_uint).wrapping_sub(b.wrapping_mul(x)))
    as pstm_digit as pstm_digit;
  /* rho = -1/m mod b */
  *rho = ((1i32 as pstm_word) << 32i32 as pstm_word).wrapping_sub(x as pstm_word) as pstm_digit;
  return 0i32;
}
/* *****************************************************************************/
/*
 *	computes a = B**n mod b without division or multiplication useful for
 *	normalizing numbers in a Montgomery system.
 */
unsafe extern "C" fn pstm_montgomery_calc_normalization(
  mut a: *mut pstm_int,
  mut b: *mut pstm_int,
) -> int32 {
  let mut x: int32 = 0; //bbox: was int16
  let mut bits: libc::c_int = 0;
  /* how many bits of last digit does b use */
  bits = pstm_count_bits(b) % 32i32;
  if bits == 0 {
    bits = 32i32
  }
  /* compute A = B^(n-1) * 2^(bits-1) */
  if (*b).used > 1i32 {
    x = pstm_2expt(a, ((*b).used - 1i32) * 32i32 + bits - 1i32);
    if x != 0i32 {
      return x;
    }
  } else {
    pstm_set(a, 1i32 as pstm_digit);
    bits = 1i32
  }
  /* now compute C = A * B mod b */
  x = bits - 1i32;
  while x < 32i32 {
    if pstm_mul_2(a, a) != 0i32 {
      return -8i32;
    }
    if pstm_cmp_mag(a, b) != -1i32 {
      if s_pstm_sub(a, b, a) != 0i32 {
        return -8i32;
      }
    }
    x += 1
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
 *	@file    pstm.c
 *	@version 33ef80f (HEAD, tag: MATRIXSSL-3-7-2-OPEN, tag: MATRIXSSL-3-7-2-COMM, origin/master, origin/HEAD, master)
 *
 *	Multiprecision number implementation.
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
/*
  c = a * 2**d
*/
unsafe extern "C" fn pstm_mul_2d(
  mut a: *mut pstm_int,
  mut b: libc::c_int,
  mut c: *mut pstm_int,
) -> int32 {
  let mut carry: pstm_digit = 0; //bbox: was int16
  let mut carrytmp: pstm_digit = 0;
  let mut shift: pstm_digit = 0;
  let mut x: libc::c_int = 0;
  /* copy it */
  pstm_copy(a, c);
  if 0i32 != 0i32 {
    return -8i32;
  }
  /* handle whole digits */
  if b >= 32i32 {
    pstm_lshd(c, b / 32i32);
    if 0i32 != 0i32 {
      return -8i32;
    }
  }
  b %= 32i32;
  /* shift the digits */
  if b != 0i32 {
    carry = 0i32 as pstm_digit;
    shift = (32i32 - b) as pstm_digit;
    x = 0i32;
    while x < (*c).used {
      carrytmp = *(*c).dp.offset(x as isize) >> shift;
      *(*c).dp.offset(x as isize) = (*(*c).dp.offset(x as isize) << b).wrapping_add(carry);
      carry = carrytmp;
      x += 1
    }
    /* store last carry if room */
    if carry != 0 && x < 4096i32 {
      if (*c).used == (*c).alloc {
        pstm_grow(c, (*c).alloc + 1i32);
        if 0i32 != 0i32 {
          return -8i32;
        }
      }
      let fresh12 = (*c).used;
      (*c).used = (*c).used + 1;
      *(*c).dp.offset(fresh12 as isize) = carry
    }
  }
  pstm_clamp(c);
  return 0i32;
}
/* *****************************************************************************/
/*
  c = a mod 2**d
*/
unsafe extern "C" fn pstm_mod_2d(
  mut a: *mut pstm_int,
  mut b: libc::c_int,
  mut c: *mut pstm_int,
) -> int32
//bbox: was int16 b
{
  let mut x: libc::c_int = 0; //bbox: was int16
                              /* zero if count less than or equal to zero */
  if b <= 0i32 {
    pstm_zero(c);
    return 0i32;
  }
  /* get copy of input */
  pstm_copy(a, c);
  if 0i32 != 0i32 {
    return -8i32;
  }
  /* if 2**d is larger than we just return */
  if b >= 32i32 * (*a).used {
    return 0i32;
  }
  /* zero digits above the last digit of the modulus */
  x = b / 32i32 + (if b % 32i32 == 0i32 { 0i32 } else { 1i32 });
  while x < (*c).used {
    *(*c).dp.offset(x as isize) = 0i32 as pstm_digit;
    x += 1
  }
  /* clear the digit that is not completely outside/inside the modulus */
  let ref mut fresh13 = *(*c).dp.offset((b / 32i32) as isize);
  *fresh13 &= !(0i32 as pstm_digit) >> 32i32 - b;
  pstm_clamp(c);
  return 0i32;
}
/* *****************************************************************************/
/*
  c = a * b
*/
unsafe extern "C" fn pstm_mul_d(
  mut a: *mut pstm_int,
  mut b: pstm_digit,
  mut c: *mut pstm_int,
) -> int32 {
  let mut w: pstm_word = 0; //bbox: was int16
  let mut res: int32 = 0;
  let mut x: libc::c_int = 0;
  let mut oldused: libc::c_int = 0;
  if (*c).alloc < (*a).used + 1i32 {
    pstm_grow(c, (*a).used + 1i32);
    res = 0i32;
    if res != 0i32 {
      return res;
    }
  }
  oldused = (*c).used;
  (*c).used = (*a).used;
  (*c).sign = (*a).sign;
  w = 0i32 as pstm_word;
  x = 0i32;
  while x < (*a).used {
    w = (*(*a).dp.offset(x as isize) as pstm_word)
      .wrapping_mul(b as pstm_word)
      .wrapping_add(w);
    *(*c).dp.offset(x as isize) = w as pstm_digit;
    w = w >> 32i32;
    x += 1
  }
  if w != 0i32 as libc::c_ulong && (*a).used != 4096i32 {
    let fresh14 = (*c).used;
    (*c).used = (*c).used + 1;
    *(*c).dp.offset(fresh14 as isize) = w as pstm_digit;
    x += 1
  }
  while x < oldused {
    *(*c).dp.offset(x as isize) = 0i32 as pstm_digit;
    x += 1
  }
  pstm_clamp(c);
  return 0i32;
}
/* *****************************************************************************/
/*
  c = a / 2**b
*/
unsafe extern "C" fn pstm_div_2d(
  mut a: *mut pstm_int,
  mut b: libc::c_int,
  mut c: *mut pstm_int,
  mut d: *mut pstm_int,
) -> int32 {
  let mut current_block: u64; //bbox: was int16
  let mut D: pstm_digit = 0;
  let mut r: pstm_digit = 0;
  let mut rr: pstm_digit = 0;
  let mut res: int32 = 0;
  let mut x: libc::c_int = 0;
  let mut t: pstm_int = pstm_int {
    used: 0,
    alloc: 0,
    sign: 0,
    dp: 0 as *mut pstm_digit,
  };
  /* if the shift count is <= 0 then we do no work */
  if b <= 0i32 {
    pstm_copy(a, c);
    if 0i32 != 0i32 {
      return -8i32;
    }
    if !d.is_null() {
      pstm_zero(d);
    }
    return 0i32;
  }
  /* get the remainder */
  if !d.is_null() {
    pstm_init(&mut t);
    if 0i32 != 0i32 {
      return -8i32;
    }
    pstm_mod_2d(a, b, &mut t);
    if 0i32 != 0i32 {
      res = -8i32;
      current_block = 7090354130695220050;
    } else {
      current_block = 12039483399334584727;
    }
  } else {
    current_block = 12039483399334584727;
  }
  match current_block {
    12039483399334584727 =>
    /* copy */
    {
      pstm_copy(a, c);
      if 0i32 != 0i32 {
        res = -8i32
      } else {
        /* shift by as many digits in the bit count */
        if b >= 32i32 {
          pstm_rshd(c, b / 32i32);
        }
        /* shift any bit count < DIGIT_BIT */
        D = (b % 32i32) as pstm_digit;
        if D != 0i32 as libc::c_uint {
          let mut tmpc: *mut pstm_digit = 0 as *mut pstm_digit;
          let mut mask: pstm_digit = 0;
          let mut shift: pstm_digit = 0;
          /* mask */
          mask = ((1i32 as pstm_digit) << D).wrapping_sub(1i32 as libc::c_uint);
          /* shift for lsb */
          shift = (32i32 as libc::c_uint).wrapping_sub(D);
          /* alias */
          tmpc = (*c).dp.offset(((*c).used - 1i32) as isize);
          /* carry */
          r = 0i32 as pstm_digit;
          x = (*c).used - 1i32;
          while x >= 0i32 {
            /* get the lower  bits of this word in a temp */
            rr = *tmpc & mask;
            /* shift the current word and mix in the carry bits from previous */
            *tmpc = *tmpc >> D | r << shift;
            tmpc = tmpc.offset(-1);
            /* set the carry to the carry bits of the current word above */
            r = rr;
            x -= 1
          }
        }
        pstm_clamp(c);
        res = 0i32
      }
    }
    _ => {}
  }
  if !d.is_null() {
    pstm_copy(&mut t, d);
    if 0i32 != 0i32 {
      res = -8i32
    }
    pstm_clear(&mut t);
  }
  return res;
}
/* *****************************************************************************/
/*
  b = a/2
*/
//UNUSED
/* *****************************************************************************/
/*
 Creates "a" then copies b into it
*/
unsafe extern "C" fn pstm_init_copy(
  mut a: *mut pstm_int,
  mut b: *mut pstm_int,
  mut toSqr: libc::c_int,
) -> int32 {
  let mut x: libc::c_int = 0; //bbox: was int16
  let mut res: int32 = 0;
  if a == b {
    return 0i32;
  }
  x = (*b).alloc;
  if toSqr != 0 {
    /*
        Smart-size:  Increasing size of a if b->used is roughly half
        of b->alloc because usage has shown that a lot of these copies
        go on to be squared and need these extra digits
    */
    if (*b).used * 2i32 + 2i32 >= x {
      x = (*b).used * 2i32 + 3i32
    }
  }
  pstm_init_size(a, x as uint32);
  res = 0i32;
  if res != 0i32 {
    return res;
  }
  pstm_copy(b, a);
  return 0i32;
}
/* *****************************************************************************/
/*
  With some compilers, we have seen issues linking with the builtin
  64 bit division routine. The issues with either manifest in a failure
  to find 'udivdi3' at link time, or a runtime invalid instruction fault
  during an RSA operation.
  The routine below divides a 64 bit unsigned int by a 32 bit unsigned int
  explicitly, rather than using the division operation
    The 64 bit result is placed in the 'numerator' parameter
    The 32 bit mod (remainder) of the division is the return parameter
  Based on implementations by:
    Copyright (C) 2003 Bernardo Innocenti <bernie@develer.com>
    Copyright (C) 1999 Hewlett-Packard Co
    Copyright (C) 1999 David Mosberger-Tang <davidm@hpl.hp.com>
*/
/* USE_MATRIX_DIV64 */
/* USE_MATRIX_DIV128 */
/* *****************************************************************************/
/*
  a/b => cb + d == a
*/
unsafe extern "C" fn pstm_div(
  mut a: *mut pstm_int,
  mut b: *mut pstm_int,
  mut c: *mut pstm_int,
  mut d: *mut pstm_int,
) -> int32 {
  let mut current_block: u64; //bbox: was int16
  let mut q: pstm_int = pstm_int {
    used: 0,
    alloc: 0,
    sign: 0,
    dp: 0 as *mut pstm_digit,
  };
  let mut x: pstm_int = pstm_int {
    used: 0,
    alloc: 0,
    sign: 0,
    dp: 0 as *mut pstm_digit,
  };
  let mut y: pstm_int = pstm_int {
    used: 0,
    alloc: 0,
    sign: 0,
    dp: 0 as *mut pstm_digit,
  };
  let mut t1: pstm_int = pstm_int {
    used: 0,
    alloc: 0,
    sign: 0,
    dp: 0 as *mut pstm_digit,
  };
  let mut t2: pstm_int = pstm_int {
    used: 0,
    alloc: 0,
    sign: 0,
    dp: 0 as *mut pstm_digit,
  };
  let mut res: int32 = 0;
  let mut n: libc::c_int = 0;
  let mut t: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  let mut norm: libc::c_int = 0;
  let mut neg: libc::c_int = 0;
  /* is divisor zero ? */
  if (if (*b).used == 0i32 { 1i32 } else { 0i32 }) == 1i32 {
    return -9i32;
  }
  /* if a < b then q=0, r = a */
  if pstm_cmp_mag(a, b) == -1i32 {
    if !d.is_null() {
      pstm_copy(a, d);
      if 0i32 != 0i32 {
        return -8i32;
      }
    }
    if !c.is_null() {
      pstm_zero(c);
    }
    return 0i32;
  }
  /*
    Smart-size inits
  */
  pstm_init_size(&mut t1, (*a).alloc as uint32);
  res = 0i32;
  if res != 0i32 {
    return res;
  }
  pstm_init_size(&mut t2, 3i32 as uint32);
  res = 0i32;
  if !(res != 0i32) {
    pstm_init_copy(&mut x, a, 0i32);
    res = 0i32;
    if !(res != 0i32) {
      /*
        Used to be an init_copy on b but pstm_grow was always hit with triple size
      */
      pstm_init_size(&mut y, ((*b).used * 3i32) as uint32);
      res = 0i32;
      if !(res != 0i32) {
        pstm_copy(b, &mut y);
        res = 0i32;
        if !(res != 0i32) {
          /* fix the sign */
          neg = if (*a).sign == (*b).sign { 0i32 } else { 1i32 };
          y.sign = 0i32;
          x.sign = y.sign;
          /* normalize both x and y, ensure that y >= b/2, [b == 2**DIGIT_BIT] */
          norm = pstm_count_bits(&mut y) % 32i32;
          if norm < 32i32 - 1i32 {
            norm = 32i32 - 1i32 - norm;
            pstm_mul_2d(&mut x, norm, &mut x);
            res = 0i32;
            if res != 0i32 {
              current_block = 17810489416690824808;
            } else {
              pstm_mul_2d(&mut y, norm, &mut y);
              res = 0i32;
              if res != 0i32 {
                current_block = 17810489416690824808;
              } else {
                current_block = 8704759739624374314;
              }
            }
          } else {
            norm = 0i32;
            current_block = 8704759739624374314;
          }
          match current_block {
            17810489416690824808 => {}
            _ => {
              /* note hac does 0 based, so if used==5 then its 0,1,2,3,4, e.g. use 4 */
              n = x.used - 1i32;
              t = y.used - 1i32;
              pstm_init_size(&mut q, (n - t + 1i32) as uint32);
              res = 0i32;
              if !(res != 0i32) {
                q.used = n - t + 1i32;
                /* while (x >= y*b**n-t) do { q[n-t] += 1; x -= y*b**{n-t} } */
                pstm_lshd(&mut y, n - t);
                res = 0i32;
                if res != 0i32 {
                  current_block = 16660950639971353743;
                } else {
                  current_block = 11743904203796629665;
                }
                'c_11045: loop {
                  match current_block {
                    16660950639971353743 =>
                    /* y = y*b**{n-t} */
                    {
                      pstm_clear(&mut q);
                      break;
                    }
                    _ => {
                      if pstm_cmp(&mut x, &mut y) != -1i32 {
                        let ref mut fresh15 = *q.dp.offset((n - t) as isize);
                        *fresh15 = (*fresh15).wrapping_add(1);
                        res = pstm_sub(&mut x, &mut y, &mut x);
                        if res != 0i32 {
                          current_block = 16660950639971353743;
                        } else {
                          current_block = 11743904203796629665;
                        }
                      } else {
                        /* reset y by shifting it back down */
                        pstm_rshd(&mut y, n - t);
                        /* step 3. for i from n down to (t + 1) */
                        i = n;
                        while i >= t + 1i32 {
                          if !(i > x.used) {
                            /* step 3.1 if xi == yt then set q{i-t-1} to b-1,
                             * otherwise set q{i-t-1} to (xi*b + x{i-1})/yt */
                            if *x.dp.offset(i as isize) == *y.dp.offset(t as isize) {
                              *q.dp.offset((i - t - 1i32) as isize) = ((1i32 as pstm_word) << 32i32)
                                .wrapping_sub(1i32 as libc::c_ulong)
                                as pstm_digit
                            } else {
                              let mut tmp: pstm_word = 0;
                              tmp = (*x.dp.offset(i as isize) as pstm_word) << 32i32 as pstm_word;
                              tmp |= *x.dp.offset((i - 1i32) as isize) as pstm_word;
                              tmp = (tmp as libc::c_ulong)
                                .wrapping_div(*y.dp.offset(t as isize) as pstm_word)
                                as pstm_word as pstm_word;
                              /* USE_MATRIX_DIV64 */
                              *q.dp.offset((i - t - 1i32) as isize) = tmp as pstm_digit
                            }
                            /* while (q{i-t-1} * (yt * b + y{t-1})) >
                               xi * b**2 + xi-1 * b + xi-2

                              do q{i-t-1} -= 1;
                            */
                            *q.dp.offset((i - t - 1i32) as isize) =
                              (*q.dp.offset((i - t - 1i32) as isize))
                                .wrapping_add(1i32 as libc::c_uint);
                            loop {
                              *q.dp.offset((i - t - 1i32) as isize) =
                                (*q.dp.offset((i - t - 1i32) as isize))
                                  .wrapping_sub(1i32 as libc::c_uint);
                              /* find left hand */
                              pstm_zero(&mut t1);
                              *t1.dp.offset(0) = if t - 1i32 < 0i32 {
                                0i32 as libc::c_uint
                              } else {
                                *y.dp.offset((t - 1i32) as isize)
                              };
                              *t1.dp.offset(1) = *y.dp.offset(t as isize);
                              t1.used = 2i32;
                              pstm_mul_d(&mut t1, *q.dp.offset((i - t - 1i32) as isize), &mut t1);
                              res = 0i32;
                              if res != 0i32 {
                                current_block = 16660950639971353743;
                                continue 'c_11045;
                              }
                              /* find right hand */
                              *t2.dp.offset(0) = if i - 2i32 < 0i32 {
                                0i32 as libc::c_uint
                              } else {
                                *x.dp.offset((i - 2i32) as isize)
                              };
                              *t2.dp.offset(1) = if i - 1i32 < 0i32 {
                                0i32 as libc::c_uint
                              } else {
                                *x.dp.offset((i - 1i32) as isize)
                              };
                              *t2.dp.offset(2) = *x.dp.offset(i as isize);
                              t2.used = 3i32;
                              if !(pstm_cmp_mag(&mut t1, &mut t2) == 1i32) {
                                break;
                              }
                            }
                            /* step 3.3 x = x - q{i-t-1} * y * b**{i-t-1} */
                            pstm_mul_d(&mut y, *q.dp.offset((i - t - 1i32) as isize), &mut t1);
                            res = 0i32;
                            if res != 0i32 {
                              current_block = 16660950639971353743;
                              continue 'c_11045;
                            }
                            pstm_lshd(&mut t1, i - t - 1i32);
                            res = 0i32;
                            if res != 0i32 {
                              current_block = 16660950639971353743;
                              continue 'c_11045;
                            }
                            res = pstm_sub(&mut x, &mut t1, &mut x);
                            if res != 0i32 {
                              current_block = 16660950639971353743;
                              continue 'c_11045;
                            }
                            /* if x < 0 then { x = x + y*b**{i-t-1}; q{i-t-1} -= 1; } */
                            if x.sign == 1i32 {
                              pstm_copy(&mut y, &mut t1);
                              res = 0i32;
                              if res != 0i32 {
                                current_block = 16660950639971353743;
                                continue 'c_11045;
                              }
                              pstm_lshd(&mut t1, i - t - 1i32);
                              res = 0i32;
                              if res != 0i32 {
                                current_block = 16660950639971353743;
                                continue 'c_11045;
                              }
                              res = pstm_add(&mut x, &mut t1, &mut x);
                              if res != 0i32 {
                                current_block = 16660950639971353743;
                                continue 'c_11045;
                              }
                              *q.dp.offset((i - t - 1i32) as isize) =
                                (*q.dp.offset((i - t - 1i32) as isize))
                                  .wrapping_sub(1i32 as libc::c_uint)
                            }
                          }
                          i -= 1
                        }
                        /*
                          now q is the quotient and x is the remainder (which we have to normalize)
                        */
                        /* get sign before writing to c */
                        x.sign = if x.used == 0i32 { 0i32 } else { (*a).sign };
                        if !c.is_null() {
                          pstm_clamp(&mut q);
                          pstm_copy(&mut q, c);
                          if 0i32 != 0i32 {
                            res = -8i32;
                            current_block = 16660950639971353743;
                            continue;
                          } else {
                            (*c).sign = neg
                          }
                        }
                        if !d.is_null() {
                          pstm_div_2d(&mut x, norm, &mut x, 0 as *mut pstm_int);
                          res = 0i32;
                          if res != 0i32 {
                            current_block = 16660950639971353743;
                            continue;
                          }
                          /*
                             the following is a kludge, essentially we were seeing the right
                             remainder but with excess digits that should have been zero
                          */
                          i = (*b).used;
                          while i < x.used {
                            *x.dp.offset(i as isize) = 0i32 as pstm_digit;
                            i += 1
                          }
                          pstm_clamp(&mut x);
                          pstm_copy(&mut x, d);
                          if 0i32 != 0i32 {
                            res = -8i32;
                            current_block = 16660950639971353743;
                            continue;
                          }
                        }
                        res = 0i32;
                        current_block = 16660950639971353743;
                      }
                    }
                  }
                }
              }
            }
          }
        }
        pstm_clear(&mut y);
      }
      pstm_clear(&mut x);
    }
    pstm_clear(&mut t2);
  }
  pstm_clear(&mut t1);
  return res;
}
/* *****************************************************************************/
/*
  Swap the elements of two integers, for cases where you can't simply swap
  the pstm_int pointers around
*/
unsafe extern "C" fn pstm_exch(mut a: *mut pstm_int, mut b: *mut pstm_int) {
  let mut t: pstm_int = pstm_int {
    used: 0,
    alloc: 0,
    sign: 0,
    dp: 0 as *mut pstm_digit,
  };
  t = *a;
  *a = *b;
  *b = t;
}
/* *****************************************************************************/
/*
  c = a mod b, 0 <= c < b
*/
unsafe extern "C" fn pstm_mod(
  mut a: *mut pstm_int,
  mut b: *mut pstm_int,
  mut c: *mut pstm_int,
) -> int32 {
  let mut t: pstm_int = pstm_int {
    used: 0,
    alloc: 0,
    sign: 0,
    dp: 0 as *mut pstm_digit,
  };
  let mut err: int32 = 0;
  /*
    Smart-size
  */
  pstm_init_size(&mut t, (*b).alloc as uint32);
  err = 0i32;
  if err != 0i32 {
    return err;
  }
  err = pstm_div(a, b, 0 as *mut pstm_int, &mut t);
  if err != 0i32 {
    pstm_clear(&mut t);
    return err;
  }
  if t.sign != (*b).sign {
    err = pstm_add(&mut t, b, c)
  } else {
    pstm_exch(&mut t, c);
  }
  pstm_clear(&mut t);
  return err;
}
/* *****************************************************************************/
/*
  d = a * b (mod c)
*/
#[no_mangle]
pub unsafe extern "C" fn pstm_mulmod(
  mut a: *mut pstm_int,
  mut b: *mut pstm_int,
  mut c: *mut pstm_int,
  mut d: *mut pstm_int,
) -> int32 {
  let mut res: int32 = 0; //bbox: was int16
  let mut size: libc::c_int = 0;
  let mut tmp: pstm_int = pstm_int {
    used: 0,
    alloc: 0,
    sign: 0,
    dp: 0 as *mut pstm_digit,
  };
  /*
    Smart-size pstm_inits.  d is an output that is influenced by this local 't'
    so don't shrink 'd' if it wants to becuase this will lead to an pstm_grow
    in RSA operations
  */
  size = (*a).used + (*b).used + 1i32;
  if a == d && size < (*a).alloc {
    size = (*a).alloc
  }
  pstm_init_size(&mut tmp, size as uint32);
  res = 0i32;
  if res != 0i32 {
    return res;
  }
  res = pstm_mul_comba(a, b, &mut tmp, 0 as *mut pstm_digit, 0i32 as uint32);
  if res != 0i32 {
    pstm_clear(&mut tmp);
    return res;
  }
  res = pstm_mod(&mut tmp, c, d);
  pstm_clear(&mut tmp);
  return res;
}
/* *****************************************************************************/
/*
 *	y = g**x (mod b)
 *	Some restrictions... x must be positive and < b
 */
#[no_mangle]
pub unsafe extern "C" fn pstm_exptmod(
  mut G: *mut pstm_int,
  mut X: *mut pstm_int,
  mut P: *mut pstm_int,
  mut Y: *mut pstm_int,
) -> int32 {
  let mut current_block: u64; /* Keep this winsize based: (1 << max_winsize) */
  let mut M: [pstm_int; 32] = [pstm_int {
    used: 0,
    alloc: 0,
    sign: 0,
    dp: 0 as *mut pstm_digit,
  }; 32]; //bbox: was int16
  let mut res: pstm_int = pstm_int {
    used: 0,
    alloc: 0,
    sign: 0,
    dp: 0 as *mut pstm_digit,
  };
  let mut buf: pstm_digit = 0;
  let mut mp: pstm_digit = 0;
  let mut paD: *mut pstm_digit = 0 as *mut pstm_digit;
  let mut err: int32 = 0;
  let mut bitbuf: int32 = 0;
  let mut bitcpy: libc::c_int = 0;
  let mut bitcnt: libc::c_int = 0;
  let mut mode: libc::c_int = 0;
  let mut digidx: libc::c_int = 0;
  let mut x: libc::c_int = 0;
  let mut y: libc::c_int = 0;
  let mut winsize: libc::c_int = 0;
  let mut paDlen: uint32 = 0;
  /* set window size from what user set as optimization */
  x = pstm_count_bits(X);
  if x < 50i32 {
    winsize = 2i32
  } else {
    winsize = 3i32
  }
  /* now setup montgomery  */
  err = pstm_montgomery_setup(P, &mut mp);
  if err != 0i32 {
    return err;
  }
  /* setup result */
  pstm_init_size(&mut res, ((*P).used * 2i32 + 1i32) as uint32);
  err = 0i32;
  if err != 0i32 {
    return err;
  }
  /*
   create M table
   The M table contains powers of the input base, e.g. M[x] = G^x mod P
   The first half of the table is not computed though except for M[0] and M[1]
  */
  /* now we need R mod m */
  err = pstm_montgomery_calc_normalization(&mut res, P);
  if !(err != 0i32) {
    /*
     init M array
     init first cell
    */
    pstm_init_size(&mut *M.as_mut_ptr().offset(1), res.used as uint32);
    err = 0i32;
    if !(err != 0i32) {
      /* now set M[1] to G * R mod m */
      if pstm_cmp_mag(P, G) != 1i32 {
        /* G > P so we reduce it first */
        err = pstm_mod(G, P, &mut *M.as_mut_ptr().offset(1));
        if err != 0i32 {
          current_block = 1207487804900576449;
        } else {
          current_block = 5634871135123216486;
        }
      } else {
        pstm_copy(G, &mut *M.as_mut_ptr().offset(1));
        err = 0i32;
        if err != 0i32 {
          current_block = 1207487804900576449;
        } else {
          current_block = 5634871135123216486;
        }
      }
      match current_block {
        5634871135123216486 => {
          err = pstm_mulmod(
            &mut *M.as_mut_ptr().offset(1),
            &mut res,
            P,
            &mut *M.as_mut_ptr().offset(1),
          );
          if !(err != 0i32) {
            /*
              Pre-allocated digit.  Used for mul, sqr, AND reduce
            */
            paDlen = (((M[1].used + 3i32) * 2i32) as libc::c_ulong)
              .wrapping_mul(::std::mem::size_of::<pstm_digit>() as libc::c_ulong)
              as uint32; //bbox
            paD = xzalloc(paDlen as size_t) as *mut pstm_digit;
            /*
             compute the value at M[1<<(winsize-1)] by squaring M[1] (winsize-1) times
            */
            pstm_init_copy(
              &mut *M.as_mut_ptr().offset((1i32 << winsize - 1i32) as isize),
              &mut *M.as_mut_ptr().offset(1),
              1i32,
            );
            if 0i32 != 0i32 {
              err = -8i32
            } else {
              x = 0i32;
              loop {
                if !(x < winsize - 1i32) {
                  current_block = 2604890879466389055;
                  break;
                }
                err = pstm_sqr_comba(
                  &mut *M.as_mut_ptr().offset((1i32 << winsize - 1i32) as isize),
                  &mut *M.as_mut_ptr().offset((1i32 << winsize - 1i32) as isize),
                  paD,
                  paDlen,
                );
                if err != 0i32 {
                  current_block = 6164085797154605288;
                  break;
                }
                err = pstm_montgomery_reduce(
                  &mut *M.as_mut_ptr().offset((1i32 << winsize - 1i32) as isize),
                  P,
                  mp,
                  paD,
                  paDlen,
                );
                if err != 0i32 {
                  current_block = 6164085797154605288;
                  break;
                }
                x += 1
              }
              match current_block {
                6164085797154605288 => {}
                _ =>
                /*
                  now init the second half of the array
                */
                {
                  x = (1i32 << winsize - 1i32) + 1i32;
                  loop {
                    if !(x < 1i32 << winsize) {
                      current_block = 1622411330066726685;
                      break;
                    }
                    pstm_init_size(
                      &mut *M.as_mut_ptr().offset(x as isize),
                      (M[(1i32 << winsize - 1i32) as usize].alloc + 1i32) as uint32,
                    );
                    err = 0i32;
                    if err != 0i32 {
                      y = 1i32 << winsize - 1i32;
                      while y < x {
                        pstm_clear(&mut *M.as_mut_ptr().offset(y as isize));
                        y += 1
                      }
                      current_block = 6164085797154605288;
                      break;
                    } else {
                      x += 1
                    }
                  }
                  match current_block {
                    6164085797154605288 => {}
                    _ =>
                    /* create upper table */
                    {
                      x = (1i32 << winsize - 1i32) + 1i32;
                      loop {
                        if !(x < 1i32 << winsize) {
                          current_block = 5891011138178424807;
                          break;
                        }
                        err = pstm_mul_comba(
                          &mut *M.as_mut_ptr().offset((x - 1i32) as isize),
                          &mut *M.as_mut_ptr().offset(1),
                          &mut *M.as_mut_ptr().offset(x as isize),
                          paD,
                          paDlen,
                        );
                        if err != 0i32 {
                          current_block = 14289100601382369252;
                          break;
                        }
                        err = pstm_montgomery_reduce(
                          &mut *M.as_mut_ptr().offset(x as isize),
                          P,
                          mp,
                          paD,
                          paDlen,
                        );
                        if err != 0i32 {
                          current_block = 14289100601382369252;
                          break;
                        }
                        x += 1
                      }
                      match current_block {
                        5891011138178424807 => {
                          /* set initial mode and bit cnt */
                          mode = 0i32;
                          bitcnt = 1i32;
                          buf = 0i32 as pstm_digit;
                          digidx = (*X).used - 1i32;
                          bitcpy = 0i32;
                          bitbuf = 0i32;
                          's_285: loop
                          /* grab next digit as required */
                          {
                            bitcnt -= 1;
                            if bitcnt == 0i32 {
                              /* if digidx == -1 we are out of digits so break */
                              if digidx == -1i32 {
                                current_block = 10393716428851982524;
                                break;
                              }
                              /* read next digit and reset bitcnt */
                              let fresh16 = digidx;
                              digidx = digidx - 1;
                              buf = *(*X).dp.offset(fresh16 as isize);
                              bitcnt = 32i32
                            }
                            /* grab the next msb from the exponent */
                            y = (buf >> 32i32 - 1i32 & 1i32 as libc::c_uint) as libc::c_int;
                            buf <<= 1i32 as pstm_digit;
                            /*
                                 If the bit is zero and mode == 0 then we ignore it.
                                 These represent the leading zero bits before the first 1 bit
                                 in the exponent.  Technically this opt is not required but it
                                 does lower the # of trivial squaring/reductions used
                            */
                            if mode == 0i32 && y == 0i32 {
                              continue;
                            }
                            /* if the bit is zero and mode == 1 then we square */
                            if mode == 1i32 && y == 0i32 {
                              err = pstm_sqr_comba(&mut res, &mut res, paD, paDlen);
                              if err != 0i32 {
                                current_block = 14289100601382369252;
                                break;
                              }
                              err = pstm_montgomery_reduce(&mut res, P, mp, paD, paDlen);
                              if err != 0i32 {
                                current_block = 14289100601382369252;
                                break;
                              }
                            } else {
                              /* else we add it to the window */
                              bitcpy += 1;
                              bitbuf |= y << winsize - bitcpy;
                              mode = 2i32;
                              if !(bitcpy == winsize) {
                                continue;
                              }
                              /* ok window is filled so square as required and mul square first */
                              x = 0i32;
                              while x < winsize {
                                err = pstm_sqr_comba(&mut res, &mut res, paD, paDlen);
                                if err != 0i32 {
                                  current_block = 14289100601382369252;
                                  break 's_285;
                                }
                                err = pstm_montgomery_reduce(&mut res, P, mp, paD, paDlen);
                                if err != 0i32 {
                                  current_block = 14289100601382369252;
                                  break 's_285;
                                }
                                x += 1
                              }
                              /* then multiply */
                              err = pstm_mul_comba(
                                &mut res,
                                &mut *M.as_mut_ptr().offset(bitbuf as isize),
                                &mut res,
                                paD,
                                paDlen,
                              );
                              if err != 0i32 {
                                current_block = 14289100601382369252;
                                break;
                              }
                              err = pstm_montgomery_reduce(&mut res, P, mp, paD, paDlen);
                              if err != 0i32 {
                                current_block = 14289100601382369252;
                                break;
                              }
                              /* empty window and reset */
                              bitcpy = 0i32;
                              bitbuf = 0i32;
                              mode = 1i32
                            }
                          }
                          match current_block {
                            14289100601382369252 => {}
                            _ =>
                            /* if bits remain then square/multiply */
                            {
                              if mode == 2i32 && bitcpy > 0i32 {
                                /* square then multiply if the bit is set */
                                x = 0i32;
                                loop {
                                  if !(x < bitcpy) {
                                    current_block = 10301740260014665685;
                                    break;
                                  }
                                  err = pstm_sqr_comba(&mut res, &mut res, paD, paDlen);
                                  if err != 0i32 {
                                    current_block = 14289100601382369252;
                                    break;
                                  }
                                  err = pstm_montgomery_reduce(&mut res, P, mp, paD, paDlen);
                                  if err != 0i32 {
                                    current_block = 14289100601382369252;
                                    break;
                                  }
                                  /* get next bit of the window */
                                  bitbuf <<= 1i32;
                                  if bitbuf & 1i32 << winsize != 0i32 {
                                    /* then multiply */
                                    err = pstm_mul_comba(
                                      &mut res,
                                      &mut *M.as_mut_ptr().offset(1),
                                      &mut res,
                                      paD,
                                      paDlen,
                                    );
                                    if err != 0i32 {
                                      current_block = 14289100601382369252;
                                      break;
                                    }
                                    err = pstm_montgomery_reduce(&mut res, P, mp, paD, paDlen);
                                    if err != 0i32 {
                                      current_block = 14289100601382369252;
                                      break;
                                    }
                                  }
                                  x += 1
                                }
                              } else {
                                current_block = 10301740260014665685;
                              }
                              match current_block {
                                14289100601382369252 => {}
                                _ =>
                                /*
                                  Fix up result if Montgomery reduction is used recall that any value in a
                                  Montgomery system is actually multiplied by R mod n.  So we have to reduce
                                  one more time to cancel out the factor of R.
                                */
                                {
                                  err = pstm_montgomery_reduce(&mut res, P, mp, paD, paDlen);
                                  if !(err != 0i32) {
                                    /* swap res with Y */
                                    pstm_copy(&mut res, Y);
                                    err = 0i32;
                                    if !(err != 0i32) {
                                      err = 0i32
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                        _ => {}
                      }
                      x = 1i32 << winsize - 1i32;
                      while x < 1i32 << winsize {
                        pstm_clear(&mut *M.as_mut_ptr().offset(x as isize));
                        x += 1
                      }
                    }
                  }
                }
              }
            }
            free(paD as *mut libc::c_void);
          }
        }
        _ => {}
      }
      pstm_clear(&mut *M.as_mut_ptr().offset(1));
    }
  }
  pstm_clear(&mut res);
  return err;
}
/* *****************************************************************************/
/*

*/
#[no_mangle]
pub unsafe extern "C" fn pstm_add(
  mut a: *mut pstm_int,
  mut b: *mut pstm_int,
  mut c: *mut pstm_int,
) -> int32 {
  let mut res: int32 = 0; //bbox: was int16
  let mut sa: libc::c_int = 0;
  let mut sb: libc::c_int = 0;
  /* get sign of both inputs */
  sa = (*a).sign;
  sb = (*b).sign;
  /* handle two cases, not four */
  if sa == sb {
    /* both positive or both negative, add their mags, copy the sign */
    (*c).sign = sa;
    res = s_pstm_add(a, b, c);
    if res != 0i32 {
      return res;
    }
  } else if pstm_cmp_mag(a, b) == -1i32 {
    (*c).sign = sb;
    res = s_pstm_sub(b, a, c);
    if res != 0i32 {
      return res;
    }
  } else {
    (*c).sign = sa;
    res = s_pstm_sub(a, b, c);
    if res != 0i32 {
      return res;
    }
  }
  return 0i32;
}
/*
   one positive, the other negative
   subtract the one with the greater magnitude from the one of the lesser
   magnitude. The result gets the sign of the one with the greater mag.
*/
/* *****************************************************************************/
/*
  reverse an array, used for radix code
*/
unsafe extern "C" fn pstm_reverse(mut s: *mut libc::c_uchar, mut len: libc::c_int)
//bbox: was int16 len
{
  let mut ix: int32 = 0;
  let mut iy: int32 = 0;
  let mut t: libc::c_uchar = 0;
  ix = 0i32;
  iy = len - 1i32;
  while ix < iy {
    t = *s.offset(ix as isize);
    *s.offset(ix as isize) = *s.offset(iy as isize);
    *s.offset(iy as isize) = t;
    ix += 1;
    iy -= 1
  }
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
/* *****************************************************************************/
/*
  No reverse.  Useful in some of the EIP-154 PKA stuff where special byte
  order seems to come into play more often
*/
//UNUSED
/* *****************************************************************************/
/*

*/
#[no_mangle]
pub unsafe extern "C" fn pstm_to_unsigned_bin(
  mut a: *mut pstm_int,
  mut b: *mut libc::c_uchar,
) -> int32 {
  let mut res: int32 = 0; //bbox: was int16
  let mut x: libc::c_int = 0;
  let mut t: pstm_int = {
    let mut init = pstm_int {
      used: 0i32,
      alloc: 0,
      sign: 0,
      dp: 0 as *mut pstm_digit,
    };
    init
  };
  pstm_init_copy(&mut t, a, 0i32);
  res = 0i32;
  if res != 0i32 {
    return res;
  }
  x = 0i32;
  while (if t.used == 0i32 { 1i32 } else { 0i32 }) == 0i32 {
    let fresh17 = x;
    x = x + 1;
    *b.offset(fresh17 as isize) = (*t.dp.offset(0) & 255i32 as libc::c_uint) as libc::c_uchar;
    pstm_div_2d(&mut t, 8i32, &mut t, 0 as *mut pstm_int);
    res = 0i32;
    if res != 0i32 {
      pstm_clear(&mut t);
      return res;
    }
  }
  pstm_reverse(b, x);
  pstm_clear(&mut t);
  return 0i32;
}
/* *****************************************************************************/
/* !DISABLE_PSTM */
//UNUSED
//UNUSED

use libc;




extern "C" {
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
}

use crate::librb::size_t;

/* After v = xrealloc_vector(v, SHIFT, idx) it's ok to use
 * at least v[idx] and v[idx+1], for all idx values.
 * SHIFT specifies how many new elements are added (1:2, 2:4, ..., 8:256...)
 * when all elements are used up. New elements are zeroed out.
 * xrealloc_vector(v, SHIFT, idx) *MUST* be called with consecutive IDXs -
 * skipping an index is a bad bug - it may miss a realloc!
 */

/*
 * Utility routines.
 *
 * Copyright (C) 2008 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* Resize (grow) malloced vector.
 *
 *  #define magic packed two parameters into one:
 *  sizeof = sizeof_and_shift >> 8
 *  shift  = (sizeof_and_shift) & 0xff
 *
 * Lets say shift = 4. 1 << 4 == 0x10.
 * If idx == 0, 0x10, 0x20 etc, vector[] is resized to next higher
 * idx step, plus one: if idx == 0x20, vector[] is resized to 0x31,
 * thus last usable element is vector[0x30].
 *
 * In other words: after xrealloc_vector(v, 4, idx), with any idx,
 * it's ok to use at least v[idx] and v[idx+1].
 * v[idx+2] etc generally are not ok.
 *
 * New elements are zeroed out, but only if realloc was done
 * (not on every call). You can depend on v[idx] and v[idx+1] being
 * zeroed out if you use it like this:
 *  v = xrealloc_vector(v, 4, idx);
 *  v[idx].some_fields = ...; - the rest stays 0/NULL
 *  idx++;
 * If you do not advance idx like above, you should be more careful.
 * Next call to xrealloc_vector(v, 4, idx) may or may not zero out v[idx].
 */
#[no_mangle]
pub unsafe extern "C" fn xrealloc_vector_helper(
  mut vector: *mut libc::c_void,
  mut sizeof_and_shift: libc::c_uint,
  mut idx: libc::c_int,
) -> *mut libc::c_void {
  let mut mask: libc::c_int = 1i32 << sizeof_and_shift as u8 as libc::c_int; /* sizeof(vector[0]) */
  if idx & mask - 1i32 == 0 {
    sizeof_and_shift >>= 8i32;
    vector = xrealloc(
      vector,
      sizeof_and_shift.wrapping_mul((idx + mask + 1i32) as libc::c_uint) as size_t,
    );
    memset(
      (vector as *mut libc::c_char)
        .offset(sizeof_and_shift.wrapping_mul(idx as libc::c_uint) as isize)
        as *mut libc::c_void,
      0i32,
      sizeof_and_shift.wrapping_mul((mask + 1i32) as libc::c_uint) as libc::c_ulong,
    );
  }
  return vector;
}

use libc;


extern "C" {
  #[no_mangle]
  fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn skip_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
}
use crate::librb::size_t;

/*
 * Utility routines.
 *
 * Copyright (C) many different people.
 * If you wrote this, please acknowledge your work.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn trim(mut s: *mut libc::c_char) -> *mut libc::c_char {
  let mut len: size_t = strlen(s);
  let mut old: size_t = len;
  /* trim trailing whitespace */
  while len != 0
    && ({
      let mut bb__isspace: libc::c_uchar =
        (*s.offset(len.wrapping_sub(1i32 as libc::c_ulong) as isize) as libc::c_int - 9i32)
          as libc::c_uchar;
      (bb__isspace as libc::c_int == ' ' as i32 - 9i32
        || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
    }) != 0
  {
    len = len.wrapping_sub(1)
  }
  /* trim leading whitespace */
  if len != 0 {
    let mut nws: *mut libc::c_char = skip_whitespace(s);
    if nws.wrapping_offset_from(s) as libc::c_long !=0{
      len = (len as libc::c_ulong)
        .wrapping_sub(nws.wrapping_offset_from(s) as libc::c_long as libc::c_ulong)
        as size_t as size_t;
      memmove(s as *mut libc::c_void, nws as *const libc::c_void, len);
    }
  }
  s = s.offset(len as isize);
  /* If it was a "const char*" which does not need trimming,
   * avoid superfluous store */
  if old != len {
    *s = '\u{0}' as i32 as libc::c_char
  }
  return s;
}

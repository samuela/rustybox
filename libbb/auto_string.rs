use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
}
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
/* vi: set sw=4 ts=4: */
/*
 * Utility routines.
 *
 * Copyright (C) 2015 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//kbuild:lib-y += auto_string.o
#[no_mangle]
pub unsafe extern "C" fn auto_string(mut str: *mut libc::c_char) -> *mut libc::c_char {
  static mut saved: [*mut libc::c_char; 4] = [0 as *const libc::c_char as *mut libc::c_char; 4]; /* = 0 */
  static mut cur_saved: uint8_t = 0;
  free(saved[cur_saved as usize] as *mut libc::c_void);
  saved[cur_saved as usize] = str;
  cur_saved = ((cur_saved as libc::c_int + 1i32) as libc::c_uint
    & ((::std::mem::size_of::<[*mut libc::c_char; 4]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
      as libc::c_uint)
      .wrapping_sub(1i32 as libc::c_uint)) as uint8_t;
  return str;
}

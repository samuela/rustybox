use libc;
use libc::strstr;
extern "C" {

  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
}

use crate::librb::smallint;
/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn strip_unsafe_prefix(mut str: *const libc::c_char) -> *const libc::c_char {
  let mut cp: *const libc::c_char = str;
  loop {
    let mut cp2: *mut libc::c_char = 0 as *mut libc::c_char;
    if *cp as libc::c_int == '/' as i32 {
      cp = cp.offset(1)
    } else if !is_prefixed_with(
      cp,
      (b"/../\x00" as *const u8 as *const libc::c_char).offset(1),
    )
    .is_null()
    {
      cp = cp.offset(3)
    } else {
      cp2 = strstr(cp, b"/../\x00" as *const u8 as *const libc::c_char);
      if cp2.is_null() {
        break;
      }
      cp = cp2.offset(4)
    }
  }
  if cp != str {
    static mut warned: smallint = 0i32 as smallint;
    if warned == 0 {
      warned = 1i32 as smallint;
      bb_error_msg(
        b"removing leading \'%.*s\' from member names\x00" as *const u8 as *const libc::c_char,
        cp.wrapping_offset_from(str) as libc::c_long as libc::c_int,
        str,
      );
    }
  }
  return cp;
}

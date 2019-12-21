use crate::librb::smallint;
use libc;
use libc::strstr;

/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn strip_unsafe_prefix(mut str: *const libc::c_char) -> *const libc::c_char {
  let mut cp: *const libc::c_char = str;
  loop {
    let mut cp2: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    if *cp as libc::c_int == '/' as i32 {
      cp = cp.offset(1)
    } else if !crate::libbb::compare_string_array::is_prefixed_with(
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
      crate::libbb::verror_msg::bb_error_msg(
        b"removing leading \'%.*s\' from member names\x00" as *const u8 as *const libc::c_char,
        cp.wrapping_offset_from(str) as libc::c_long as libc::c_int,
        str,
      );
    }
  }
  return cp;
}

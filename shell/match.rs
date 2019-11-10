use libc;




































































extern "C" {
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn fnmatch(
    __pattern: *const libc::c_char,
    __name: *const libc::c_char,
    __flags: libc::c_int,
  ) -> libc::c_int;
}
use crate::librb::size_t;
/* match.h - interface to shell ##/%% matching code */
//TODO! Why ash.c still uses internal version?!
pub type C2RustUnnamed = libc::c_uint;
pub const SCAN_MATCH_RIGHT_HALF: C2RustUnnamed = 8;
pub const SCAN_MATCH_LEFT_HALF: C2RustUnnamed = 4;
pub const SCAN_MOVE_FROM_RIGHT: C2RustUnnamed = 2;
pub const SCAN_MOVE_FROM_LEFT: C2RustUnnamed = 1;
/*
 * ##/%% variable matching code ripped out of ash shell for code sharing
 *
 * This code is derived from software contributed to Berkeley by
 * Kenneth Almquist.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 *
 * Copyright (c) 1989, 1991, 1993, 1994
 *      The Regents of the University of California.  All rights reserved.
 *
 * Copyright (c) 1997-2005 Herbert Xu <herbert@gondor.apana.org.au>
 * was re-ported from NetBSD and debianized.
 */
#[no_mangle]
pub unsafe extern "C" fn scan_and_match(
  mut string: *mut libc::c_char,
  mut pattern: *const libc::c_char,
  mut flags: libc::c_uint,
) -> *mut libc::c_char {
  let mut loc: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut len: libc::c_uint = strlen(string) as libc::c_uint;
  let mut early_exit: libc::c_int = 0;
  /* We can stop the scan early only if the string part
   * we are matching against is shrinking, and the pattern has
   * an unquoted "star" at the corresponding end. There are two cases.
   * Case 1:
   * "qwerty" does not match against pattern "*zy",
   * no point in trying to match "werty", "erty" etc:
   */
  early_exit = (flags
    == (SCAN_MOVE_FROM_LEFT as libc::c_int + SCAN_MATCH_RIGHT_HALF as libc::c_int) as libc::c_uint
    && *pattern.offset(0) as libc::c_int == '*' as i32) as libc::c_int;
  if flags & SCAN_MOVE_FROM_LEFT as libc::c_int as libc::c_uint != 0 {
    loc = string;
    end = string.offset(len as isize).offset(1)
  } else {
    loc = string.offset(len as isize);
    end = string.offset(-1);
    if flags
      == (SCAN_MOVE_FROM_RIGHT as libc::c_int + SCAN_MATCH_LEFT_HALF as libc::c_int) as libc::c_uint
    {
      /* Case 2:
       * "qwerty" does not match against pattern "qz*",
       * no point in trying to match "qwert", "qwer" etc:
       */
      let mut p: *const libc::c_char = pattern.offset(strlen(pattern) as isize);
      p = p.offset(-1);
      if p >= pattern && *p as libc::c_int == '*' as i32 {
        early_exit = 1i32;
        loop {
          p = p.offset(-1);
          if !(p >= pattern && *p as libc::c_int == '\\' as i32) {
            break;
          }
          early_exit ^= 1i32
        }
      }
    }
  }
  while loc != end {
    let mut c: libc::c_char = 0;
    let mut r: libc::c_int = 0;
    c = *loc;
    if flags & SCAN_MATCH_LEFT_HALF as libc::c_int as libc::c_uint != 0 {
      *loc = '\u{0}' as i32 as libc::c_char;
      r = fnmatch(pattern, string, 0i32);
      //bb_error_msg("fnmatch('%s','%s',0):%d", pattern, string, r);
      *loc = c
    } else {
      r = fnmatch(pattern, loc, 0i32)
      //bb_error_msg("fnmatch('%s','%s',0):%d", pattern, loc, r);
    }
    if r == 0i32 {
      /* match found */
      return loc;
    }
    if early_exit != 0 {
      break;
    }
    if flags & SCAN_MOVE_FROM_LEFT as libc::c_int as libc::c_uint != 0 {
      loc = loc.offset(1)
    } else {
      loc = loc.offset(-1)
    }
  }
  return 0 as *mut libc::c_char;
}

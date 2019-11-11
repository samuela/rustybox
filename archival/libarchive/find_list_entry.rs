
use libc;
extern "C" {
  #[no_mangle]
  fn fnmatch(
    __pattern: *const libc::c_char,
    __name: *const libc::c_char,
    __flags: libc::c_int,
  ) -> libc::c_int;
}
use crate::libbb::llist::llist_t;
/*
 * Copyright (C) 2002 by Glenn McGrath
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Find a string in a shell pattern list */
#[no_mangle]
pub unsafe extern "C" fn find_list_entry(
  mut list: *const llist_t,
  mut filename: *const libc::c_char,
) -> *const llist_t {
  while !list.is_null() {
    if fnmatch((*list).data, filename, 0i32) == 0i32 {
      return list;
    }
    list = (*list).link
  }
  return 0 as *const llist_t;
}

/* Same, but compares only path components present in pattern
 * (extra trailing path components in filename are assumed to match)
 */
#[no_mangle]
pub unsafe extern "C" fn find_list_entry2(
  mut list: *const llist_t,
  mut filename: *const libc::c_char,
) -> *const llist_t {
  let mut buf: [libc::c_char; 4096] = [0; 4096];
  let mut pattern_slash_cnt: libc::c_int = 0;
  let mut c: *const libc::c_char = 0 as *const libc::c_char;
  let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
  while !list.is_null() {
    c = (*list).data;
    pattern_slash_cnt = 0i32;
    while *c != 0 {
      let fresh0 = c;
      c = c.offset(1);
      if *fresh0 as libc::c_int == '/' as i32 {
        pattern_slash_cnt += 1
      }
    }
    c = filename;
    d = buf.as_mut_ptr();
    /* paranoia is better than buffer overflows */
    while *c as libc::c_int != 0
      && d
        != buf
          .as_mut_ptr()
          .offset(::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as isize)
          .offset(-1)
    {
      if *c as libc::c_int == '/' as i32 && {
        pattern_slash_cnt -= 1;
        (pattern_slash_cnt) < 0i32
      } {
        break;
      }
      let fresh1 = c;
      c = c.offset(1);
      let fresh2 = d;
      d = d.offset(1);
      *fresh2 = *fresh1
    }
    *d = '\u{0}' as i32 as libc::c_char;
    if fnmatch((*list).data, buf.as_mut_ptr(), 0i32) == 0i32 {
      return list;
    }
    list = (*list).link
  }
  return 0 as *const llist_t;
}

use libc;
use libc::open;


use libc::free;
extern "C" {

  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
}
use crate::librb::size_t;
/* Based on ipsvd utilities written by Gerrit Pape <pape@smarden.org>
 * which are released into public domain by the author.
 * Homepage: http://smarden.sunsite.dk/ipsvd/
 *
 * Copyright (C) 2007 Denys Vlasenko.
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hcc {
  pub ip: *mut libc::c_char,
  pub pid: libc::c_int,
}
/* Based on ipsvd utilities written by Gerrit Pape <pape@smarden.org>
 * which are released into public domain by the author.
 * Homepage: http://smarden.sunsite.dk/ipsvd/
 *
 * Copyright (C) 2007 Denys Vlasenko.
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn ipsvd_perhost_init(mut c: libc::c_uint) -> *mut hcc {
  //	free(cc);
  let mut cc: *mut hcc = xzalloc(
    (c.wrapping_add(1i32 as libc::c_uint) as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<hcc>() as libc::c_ulong),
  ) as *mut hcc; /* "end" marker */
  (*cc.offset(c as isize)).pid = -1i32;
  return cc;
}
/* Returns number of already opened connects to this ips, including this one.
 * ip should be a malloc'ed ptr.
 * If return value is <= maxconn, ip is inserted into the table
 * and pointer to table entry if stored in *hccpp
 * (useful for storing pid later).
 * Else ip is NOT inserted (you must take care of it - free() etc) */
#[no_mangle]
pub unsafe extern "C" fn ipsvd_perhost_add(
  mut cc: *mut hcc,
  mut ip: *mut libc::c_char,
  mut maxconn: libc::c_uint,
  mut hccpp: *mut *mut hcc,
) -> libc::c_uint {
  let mut i: libc::c_uint = 0;
  let mut conn: libc::c_uint = 1i32 as libc::c_uint;
  let mut freepos: libc::c_int = -1i32;
  i = 0i32 as libc::c_uint;
  while (*cc.offset(i as isize)).pid >= 0i32 {
    if (*cc.offset(i as isize)).ip.is_null() {
      freepos = i as libc::c_int
    } else if strcmp((*cc.offset(i as isize)).ip, ip) == 0i32 {
      conn = conn.wrapping_add(1)
    }
    i = i.wrapping_add(1)
  }
  if freepos == -1i32 {
    return 0i32 as libc::c_uint;
  }
  if conn <= maxconn {
    let ref mut fresh0 = (*cc.offset(freepos as isize)).ip;
    *fresh0 = ip;
    *hccpp = &mut *cc.offset(freepos as isize) as *mut hcc
  }
  return conn;
}
/* Finds and frees element with pid */
#[no_mangle]
pub unsafe extern "C" fn ipsvd_perhost_remove(mut cc: *mut hcc, mut pid: libc::c_int) {
  let mut i: libc::c_uint = 0;
  i = 0i32 as libc::c_uint;
  while (*cc.offset(i as isize)).pid >= 0i32 {
    if (*cc.offset(i as isize)).pid == pid {
      free((*cc.offset(i as isize)).ip as *mut libc::c_void);
      let ref mut fresh1 = (*cc.offset(i as isize)).ip;
      *fresh1 = 0 as *mut libc::c_char;
      (*cc.offset(i as isize)).pid = 0i32;
      return;
    }
    i = i.wrapping_add(1)
  }
}
//void ipsvd_perhost_free(struct hcc *cc)
//{
//	free(cc);
//}

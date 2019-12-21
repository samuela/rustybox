use crate::libbb::xfuncs_printf::xmalloc;
use crate::librb::re_pattern_buffer;
use crate::librb::size_t;
use c2rust_bitfields;
use libc;
extern "C" {

  #[no_mangle]
  fn regcomp(
    __preg: *mut regex_t,
    __pattern: *const libc::c_char,
    __cflags: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn regerror(
    __errcode: libc::c_int,
    __preg: *const regex_t,
    __errbuf: *mut libc::c_char,
    __errbuf_size: size_t,
  ) -> size_t;
}

pub type regex_t = re_pattern_buffer;

/*
 * Busybox xregcomp utility routine.  This isn't in libbb.h because the
 * C library we're linking against may not support regex.h.
 *
 * Based in part on code from sash, Copyright (c) 1999 by David I. Bell
 * Permission has been granted to redistribute this code under GPL.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */

/*
 * Utility routines.
 *
 * Copyright (C) many different people.
 * If you wrote this, please acknowledge your work.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn regcomp_or_errmsg(
  mut preg: *mut regex_t,
  mut regex: *const libc::c_char,
  mut cflags: libc::c_int,
) -> *mut libc::c_char {
  let mut ret: libc::c_int = regcomp(preg, regex, cflags);
  if ret != 0 {
    let mut errmsgsz: libc::c_int = regerror(
      ret,
      preg,
      std::ptr::null_mut::<libc::c_char>(),
      0i32 as size_t,
    ) as libc::c_int;
    let mut errmsg: *mut libc::c_char = xmalloc(errmsgsz as size_t) as *mut libc::c_char;
    regerror(ret, preg, errmsg, errmsgsz as size_t);
    return errmsg;
  }
  return std::ptr::null_mut::<libc::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn xregcomp(
  mut preg: *mut regex_t,
  mut regex: *const libc::c_char,
  mut cflags: libc::c_int,
) {
  let mut errmsg: *mut libc::c_char = regcomp_or_errmsg(preg, regex, cflags);
  if !errmsg.is_null() {
    crate::libbb::verror_msg::bb_error_msg_and_die(
      b"bad regex \'%s\': %s\x00" as *const u8 as *const libc::c_char,
      regex,
      errmsg,
    );
  };
}

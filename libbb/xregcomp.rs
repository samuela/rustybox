use c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
use libc;




extern "C" {
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
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
use crate::librb::size_t;
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
  pub buffer: *mut libc::c_uchar,
  pub allocated: libc::c_ulong,
  pub used: libc::c_ulong,
  pub syntax: reg_syntax_t,
  pub fastmap: *mut libc::c_char,
  pub translate: *mut libc::c_uchar,
  pub re_nsub: size_t,
  #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
  #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
  #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
  #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
  #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
  #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
  #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
  pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
  #[bitfield(padding)]
  pub c2rust_padding: [u8; 7],
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
    let mut errmsgsz: libc::c_int =
      regerror(ret, preg, 0 as *mut libc::c_char, 0i32 as size_t) as libc::c_int;
    let mut errmsg: *mut libc::c_char = xmalloc(errmsgsz as size_t) as *mut libc::c_char;
    regerror(ret, preg, errmsg, errmsgsz as size_t);
    return errmsg;
  }
  return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn xregcomp(
  mut preg: *mut regex_t,
  mut regex: *const libc::c_char,
  mut cflags: libc::c_int,
) {
  let mut errmsg: *mut libc::c_char = regcomp_or_errmsg(preg, regex, cflags);
  if !errmsg.is_null() {
    bb_error_msg_and_die(
      b"bad regex \'%s\': %s\x00" as *const u8 as *const libc::c_char,
      regex,
      errmsg,
    );
  };
}

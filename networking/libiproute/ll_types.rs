use libc;
use libc::chdir;
use libc::chmod;
use libc::chown;
use libc::closelog;
use libc::dup2;
use libc::fstat;
use libc::getenv;
use libc::geteuid;
use libc::getopt;
use libc::getpid;
use libc::isatty;
use libc::kill;
use libc::openlog;
use libc::sigaddset;
use libc::sigemptyset;
use libc::sigprocmask;
use libc::sleep;
use libc::sscanf;
use libc::strcasecmp;
use libc::strcpy;
use libc::symlink;
use libc::syscall;
use libc::syslog;
use libc::time;
use libc::access;
use libc::atoi;
use libc::fclose;
use libc::fprintf;
use libc::lstat;
use libc::printf;
use libc::puts;
use libc::rename;
use libc::rmdir;
use libc::sprintf;
use libc::strchr;
use libc::strcmp;
use libc::strrchr;
use libc::strstr;
use libc::system;




extern "C" {

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
}
use crate::librb::size_t;



/*
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version
 * 2 of the License, or (at your option) any later version.
 *
 * Authors: Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
 */
/* linux/if_arp.h needs it on some systems */
#[no_mangle]
pub unsafe extern "C" fn ll_type_n2a(
  mut type_0: libc::c_int,
  mut buf: *mut libc::c_char,
) -> *const libc::c_char {
  static mut arphrd_name: [libc::c_char; 125] = [
    103, 101, 110, 101, 114, 105, 99, 0, 108, 111, 111, 112, 98, 97, 99, 107, 0, 101, 116, 104,
    101, 114, 0, 105, 110, 102, 105, 110, 105, 98, 97, 110, 100, 0, 105, 101, 101, 101, 56, 48, 50,
    0, 116, 114, 0, 105, 101, 101, 101, 56, 48, 50, 46, 49, 49, 0, 105, 101, 101, 101, 49, 51, 57,
    52, 0, 105, 114, 100, 97, 0, 115, 108, 105, 112, 0, 99, 115, 108, 105, 112, 0, 115, 108, 105,
    112, 54, 0, 99, 115, 108, 105, 112, 54, 0, 112, 112, 112, 0, 105, 112, 105, 112, 0, 116, 117,
    110, 110, 101, 108, 54, 0, 115, 105, 116, 0, 103, 114, 101, 0, 118, 111, 105, 100, 0, 0,
  ];
  /* FEATURE_IP_RARE_PROTOCOLS */
  /* Keep these arrays in sync! */
  static mut arphrd_type: [u16; 19] = [
    0i32 as u16,
    772i32 as u16,
    1i32 as u16,
    32i32 as u16,
    6i32 as u16,
    800i32 as u16,
    801i32 as u16,
    24i32 as u16,
    783i32 as u16,
    256i32 as u16,
    257i32 as u16,
    258i32 as u16,
    259i32 as u16,
    512i32 as u16,
    768i32 as u16,
    769i32 as u16,
    776i32 as u16,
    778i32 as u16,
    0xffffi32 as u16,
  ];
  let mut i: libc::c_uint = 0;
  let mut aname: *const libc::c_char = arphrd_name.as_ptr();
  i = 0i32 as libc::c_uint;
  while i
    < (::std::mem::size_of::<[u16; 19]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<u16>() as libc::c_ulong) as libc::c_uint
  {
    if arphrd_type[i as usize] as libc::c_int == type_0 {
      return aname;
    }
    aname = aname.offset(strlen(aname).wrapping_add(1i32 as libc::c_ulong) as isize);
    i = i.wrapping_add(1)
  }
  sprintf(buf, b"[%d]\x00" as *const u8 as *const libc::c_char, type_0);
  return buf;
}

use libc;


use libc::gid_t;
use libc::mode_t;
use crate::librb::off_t;
use libc::time_t;
use libc::uid_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_header_t {
  pub name: *mut libc::c_char,
  pub link_target: *mut libc::c_char,
  pub tar__uname: *mut libc::c_char,
  pub tar__gname: *mut libc::c_char,
  pub size: off_t,
  pub uid: uid_t,
  pub gid: gid_t,
  pub mode: mode_t,
  pub mtime: time_t,
  pub device: libc::dev_t,
}

/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn header_skip(mut _file_header: *const file_header_t) {}

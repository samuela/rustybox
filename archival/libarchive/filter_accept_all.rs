use libc;
extern "C" {
  pub type hardlinks_t;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
use crate::librb::smallint;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
pub type mode_t = __mode_t;
pub type dev_t = __dev_t;
pub type time_t = __time_t;
pub type uoff_t = libc::c_ulong;
use crate::libbb::llist::llist_t;
use crate::librb::bb_uidgid_t;
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
  pub device: dev_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_handle_t {
  pub ah_flags: libc::c_uint,
  pub src_fd: libc::c_int,
  pub filter: Option<unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char>,
  pub accept: *mut llist_t,
  pub reject: *mut llist_t,
  pub passed: *mut llist_t,
  pub file_header: *mut file_header_t,
  pub link_placeholders: *mut llist_t,
  pub action_header: Option<unsafe extern "C" fn(_: *const file_header_t) -> ()>,
  pub action_data: Option<unsafe extern "C" fn(_: *mut archive_handle_t) -> ()>,
  pub seek: Option<unsafe extern "C" fn(_: libc::c_int, _: off_t) -> ()>,
  pub offset: off_t,
  pub tar__strip_components: libc::c_uint,
  pub tar__end: smallint,
  pub tar__longname: *mut libc::c_char,
  pub tar__linkname: *mut libc::c_char,
  pub tar__to_command: *mut libc::c_char,
  pub tar__to_command_shell: *const libc::c_char,
  pub cpio__blocks: uoff_t,
  pub cpio__owner: bb_uidgid_t,
  pub cpio__hardlinks_to_create: *mut hardlinks_t,
  pub cpio__created_hardlinks: *mut hardlinks_t,
  pub dpkg__buffer: *mut libc::c_char,
  pub dpkg__action_data_subarchive:
    Option<unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char>,
  pub dpkg__sub_archive: *mut archive_handle_t,
}
/* vi: set sw=4 ts=4: */
/*
 * Copyright (C) 2002 by Glenn McGrath
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Accept any non-null name, its not really a filter at all */
#[no_mangle]
pub unsafe extern "C" fn filter_accept_all(
  mut archive_handle: *mut archive_handle_t,
) -> libc::c_char {
  if !(*(*archive_handle).file_header).name.is_null() {
    return 0i32 as libc::c_char;
  }
  return 1i32 as libc::c_char;
}

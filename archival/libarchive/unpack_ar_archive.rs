use libc;

extern "C" {
  pub type hardlinks_t;

  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn xread(fd: libc::c_int, buf: *mut libc::c_void, count: size_t);

  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;

  #[no_mangle]
  fn get_header_ar(archive_handle: *mut archive_handle_t) -> libc::c_char;
}

use crate::libbb::llist::llist_t;

use crate::librb::bb_uidgid_t;
use crate::librb::dev_t;
use libc::gid_t;
use crate::librb::mode_t;
use crate::librb::off_t;
use crate::librb::size_t;
use crate::librb::smallint;
use libc::time_t;
use libc::uid_t;
use crate::librb::uoff_t;

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

/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn unpack_ar_archive(mut ar_archive: *mut archive_handle_t) {
  let mut magic: [libc::c_char; 7] = [0; 7];
  xread(
    (*ar_archive).src_fd,
    magic.as_mut_ptr() as *mut libc::c_void,
    7i32 as size_t,
  );
  if is_prefixed_with(
    magic.as_mut_ptr(),
    b"!<arch>\x00" as *const u8 as *const libc::c_char,
  )
  .is_null()
  {
    bb_simple_error_msg_and_die(b"invalid ar magic\x00" as *const u8 as *const libc::c_char);
  }
  (*ar_archive).offset += 7i32 as libc::c_long;
  while get_header_ar(ar_archive) as libc::c_int == 0i32 {}
}

use libc;
extern "C" {
  pub type hardlinks_t;
  #[no_mangle]
  fn get_header_tar(archive_handle: *mut archive_handle_t) -> libc::c_char;
  #[no_mangle]
  fn seek_by_read(fd: libc::c_int, amount: off_t);
  #[no_mangle]
  fn unpack_gz_stream(xstate: *mut transformer_state_t) -> libc::c_longlong;
  #[no_mangle]
  fn fork_transformer(
    fd: libc::c_int,
    signature_skipped: libc::c_int,
    transformer: Option<unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong>,
  );
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
use crate::librb::smallint;
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct transformer_state_t {
  pub signature_skipped: smallint,
  pub xformer: Option<unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong>,
  pub src_fd: libc::c_int,
  pub dst_fd: libc::c_int,
  pub mem_output_size_max: size_t,
  pub mem_output_size: size_t,
  pub mem_output_buf: *mut libc::c_char,
  pub bytes_out: off_t,
  pub bytes_in: off_t,
  pub crc32: uint32_t,
  pub mtime: time_t,
  pub magic: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub b: [uint8_t; 8],
  pub b16: [uint16_t; 4],
  pub b32: [uint32_t; 2],
}
/* vi: set sw=4 ts=4: */
/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn get_header_tar_gz(
  mut archive_handle: *mut archive_handle_t,
) -> libc::c_char {
  /* Can't lseek over pipes */
  (*archive_handle).seek =
    Some(seek_by_read as unsafe extern "C" fn(_: libc::c_int, _: off_t) -> ());
  fork_transformer(
    (*archive_handle).src_fd,
    0i32,
    Some(unpack_gz_stream as unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong),
  );
  (*archive_handle).offset = 0i32 as off_t;
  while get_header_tar(archive_handle) as libc::c_int == 0i32 {}
  /* Can only do one file at a time */
  return 1i32 as libc::c_char;
}

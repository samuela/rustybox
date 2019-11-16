use libc;
use libc::off_t;
extern "C" {

  #[no_mangle]
  fn get_header_tar(archive_handle: *mut archive_handle_t) -> libc::c_char;

  #[no_mangle]
  fn seek_by_read(fd: libc::c_int, amount: off_t);

  #[no_mangle]
  fn unpack_bz2_stream(xstate: *mut transformer_state_t) -> libc::c_longlong;

  #[no_mangle]
  fn fork_transformer(
    fd: libc::c_int,
    signature_skipped: libc::c_int,
    transformer: Option<unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong>,
  );
}

use crate::archival::libarchive::bb_archive::archive_handle_t;
use crate::archival::libarchive::bb_archive::transformer_state_t;
/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn get_header_tar_bz2(
  mut archive_handle: *mut archive_handle_t,
) -> libc::c_char {
  /* Can't lseek over pipes */
  (*archive_handle).seek =
    Some(seek_by_read as unsafe extern "C" fn(_: libc::c_int, _: off_t) -> ());
  fork_transformer(
    (*archive_handle).src_fd,
    0i32,
    Some(
      unpack_bz2_stream as unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong,
    ),
  );
  (*archive_handle).offset = 0i32 as off_t;
  while get_header_tar(archive_handle) as libc::c_int == 0i32 {}
  /* Can only do one file at a time */
  return 1i32 as libc::c_char;
}

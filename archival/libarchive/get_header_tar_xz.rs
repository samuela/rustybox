use crate::archival::libarchive::bb_archive::archive_handle_t;
use crate::archival::libarchive::bb_archive::transformer_state_t;
use libc;
use libc::off_t;
/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn get_header_tar_xz(
  mut archive_handle: *mut archive_handle_t,
) -> libc::c_char {
  /* Can't lseek over pipes */
  (*archive_handle).seek = Some(
    crate::archival::libarchive::seek_by_read::seek_by_read
      as unsafe extern "C" fn(_: libc::c_int, _: off_t) -> (),
  );
  crate::archival::libarchive::open_transformer::fork_transformer(
    (*archive_handle).src_fd,
    0,
    Some(
      crate::archival::libarchive::decompress_unxz::unpack_xz_stream
        as unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong,
    ),
  );
  (*archive_handle).offset = 0 as off_t;
  while crate::archival::libarchive::get_header_tar::get_header_tar(archive_handle) as libc::c_int
    == 0
  {}
  /* Can only do one file at a time */
  return 1i32 as libc::c_char;
}

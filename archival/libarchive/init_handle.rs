use crate::archival::libarchive::bb_archive::archive_handle_t;
use crate::archival::libarchive::bb_archive::file_header_t;
use libc;
use libc::gid_t;
use libc::off_t;
use libc::uid_t;
/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn init_handle() -> *mut archive_handle_t {
  let mut archive_handle: *mut archive_handle_t = 0 as *mut archive_handle_t;
  /* Initialize default values */
  archive_handle = crate::libbb::xfuncs_printf::xzalloc(
    ::std::mem::size_of::<archive_handle_t>() as libc::c_ulong
  ) as *mut archive_handle_t;
  (*archive_handle).file_header = crate::libbb::xfuncs_printf::xzalloc(::std::mem::size_of::<
    file_header_t,
  >() as libc::c_ulong) as *mut file_header_t;
  (*archive_handle).action_header = Some(
    crate::archival::libarchive::header_skip::header_skip
      as unsafe extern "C" fn(_: *const file_header_t) -> (),
  );
  (*archive_handle).action_data = Some(
    crate::archival::libarchive::data_skip::data_skip
      as unsafe extern "C" fn(_: *mut archive_handle_t) -> (),
  );
  (*archive_handle).filter = Some(
    crate::archival::libarchive::filter_accept_all::filter_accept_all
      as unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char,
  );
  (*archive_handle).seek = Some(
    crate::archival::libarchive::seek_by_jump::seek_by_jump
      as unsafe extern "C" fn(_: libc::c_int, _: off_t) -> (),
  );
  (*archive_handle).cpio__owner.uid = -1i64 as uid_t;
  (*archive_handle).cpio__owner.gid = -1i64 as gid_t;
  return archive_handle;
}

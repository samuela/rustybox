use crate::archival::libarchive::bb_archive::archive_handle_t;
use crate::librb::size_t;
use libc;

/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn unpack_ar_archive(mut ar_archive: *mut archive_handle_t) {
  let mut magic: [libc::c_char; 7] = [0; 7];
  crate::libbb::read_printf::xread(
    (*ar_archive).src_fd,
    magic.as_mut_ptr() as *mut libc::c_void,
    7i32 as size_t,
  );
  if crate::libbb::compare_string_array::is_prefixed_with(
    magic.as_mut_ptr(),
    b"!<arch>\x00" as *const u8 as *const libc::c_char,
  )
  .is_null()
  {
    crate::libbb::verror_msg::bb_simple_error_msg_and_die(
      b"invalid ar magic\x00" as *const u8 as *const libc::c_char,
    );
  }
  (*ar_archive).offset += 7i32 as libc::c_long;
  while crate::archival::libarchive::get_header_ar::get_header_ar(ar_archive) as libc::c_int == 0 {}
}

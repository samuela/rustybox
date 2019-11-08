use crate::archival::libarchive::bb_archive::file_header_t;
use crate::archival::libarchive::bb_archive::hardlinks_t;
use crate::libbb::llist::llist_t;
use crate::librb::bb_uidgid_t;
use crate::librb::size_t;
use crate::librb::smallint;
use crate::librb::uoff_t;
use libc;
use libc::off_t;

extern "C" {

  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn xread(fd: libc::c_int, buf: *mut libc::c_void, count: size_t);

  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;

  #[no_mangle]
  fn get_header_ar(archive_handle: *mut archive_handle_t) -> libc::c_char;
}

use crate::archival::libarchive::bb_archive::archive_handle_t;

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

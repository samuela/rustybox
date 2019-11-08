use crate::archival::libarchive::bb_archive::file_header_t;
use crate::archival::libarchive::bb_archive::hardlinks_t;
use crate::libbb::llist::llist_t;
use crate::librb::bb_uidgid_t;
use crate::librb::smallint;
use crate::librb::uoff_t;
use libc;
use libc::off_t;

use crate::archival::libarchive::bb_archive::archive_handle_t;

/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn data_align(
  mut archive_handle: *mut archive_handle_t,
  mut boundary: libc::c_uint,
) {
  let mut skip_amount: libc::c_uint = ((boundary as libc::c_long
    - (*archive_handle).offset % boundary as libc::c_long)
    % boundary as libc::c_long) as libc::c_uint;
  (*archive_handle).seek.expect("non-null function pointer")(
    (*archive_handle).src_fd,
    skip_amount as off_t,
  );
  (*archive_handle).offset += skip_amount as libc::c_long;
}

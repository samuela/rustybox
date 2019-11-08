use crate::archival::libarchive::bb_archive::file_header_t;
use crate::archival::libarchive::bb_archive::hardlinks_t;
use crate::libbb::llist::llist_t;
use crate::librb::bb_uidgid_t;
use crate::librb::smallint;
use crate::librb::uoff_t;
use libc;
use libc::off_t;
use crate::archival::libarchive::bb_archive::archive_handle_t;

#[no_mangle]
pub unsafe extern "C" fn data_skip(mut archive_handle: *mut archive_handle_t) {
  (*archive_handle).seek.expect("non-null function pointer")(
    (*archive_handle).src_fd,
    (*(*archive_handle).file_header).size,
  );
}

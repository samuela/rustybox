use crate::archival::libarchive::bb_archive::archive_handle_t;

#[no_mangle]
pub unsafe extern "C" fn data_extract_to_stdout(mut archive_handle: *mut archive_handle_t) {
  crate::libbb::copyfd::bb_copyfd_exact_size(
    (*archive_handle).src_fd,
    1,
    (*(*archive_handle).file_header).size,
  );
}

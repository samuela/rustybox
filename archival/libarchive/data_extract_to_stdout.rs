use crate::archival::libarchive::bb_archive::archive_handle_t;
use libc;



use libc::off_t;

extern "C" {
  #[no_mangle]
  fn bb_copyfd_exact_size(fd1: libc::c_int, fd2: libc::c_int, size: off_t);
}

#[no_mangle]
pub unsafe extern "C" fn data_extract_to_stdout(mut archive_handle: *mut archive_handle_t) {
  bb_copyfd_exact_size(
    (*archive_handle).src_fd,
    1,
    (*(*archive_handle).file_header).size,
  );
}

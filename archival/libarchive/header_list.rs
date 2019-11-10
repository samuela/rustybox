use libc;






























use libc::puts;












extern "C" {

}







use crate::archival::libarchive::bb_archive::file_header_t;

/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn header_list(mut file_header: *const file_header_t) {
  //TODO: cpio -vp DIR should output "DIR/NAME", not just "NAME" */
  puts((*file_header).name);
}

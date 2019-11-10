use crate::archival::libarchive::bb_archive::file_header_t;
/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn header_skip(mut _file_header: *const file_header_t) {}

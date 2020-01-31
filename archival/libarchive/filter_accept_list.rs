use crate::archival::libarchive::bb_archive::archive_handle_t;
use libc;

/*
 * Copyright (C) 2002 by Glenn McGrath
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/*
 * Accept names that are in the accept list, ignoring reject list.
 */
pub unsafe fn filter_accept_list(mut archive_handle: *mut archive_handle_t) -> libc::c_char {
  if !crate::archival::libarchive::find_list_entry::find_list_entry(
    (*archive_handle).accept,
    (*(*archive_handle).file_header).name,
  )
  .is_null()
  {
    return 0 as libc::c_char;
  }
  return 1i32 as libc::c_char;
}

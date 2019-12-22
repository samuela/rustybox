use crate::libbb::llist::llist_t;
use libc;

use crate::archival::libarchive::bb_archive::archive_handle_t;
/*
 * Copyright (C) 2002 by Glenn McGrath
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/*
 * Accept names that are in the accept list and not in the reject list
 */
#[no_mangle]
pub unsafe extern "C" fn filter_accept_reject_list(
  mut archive_handle: *mut archive_handle_t,
) -> libc::c_char {
  let mut key: *const libc::c_char = std::ptr::null();
  let mut reject_entry: *const llist_t = std::ptr::null();
  let mut accept_entry: *const llist_t = std::ptr::null();
  key = (*(*archive_handle).file_header).name;
  /* If the key is in a reject list fail */
  reject_entry =
    crate::archival::libarchive::find_list_entry::find_list_entry2((*archive_handle).reject, key);
  if !reject_entry.is_null() {
    return 1i32 as libc::c_char;
  }
  /* Fail if an accept list was specified and the key wasnt in there */
  if !(*archive_handle).accept.is_null() {
    accept_entry =
      crate::archival::libarchive::find_list_entry::find_list_entry2((*archive_handle).accept, key);
    if accept_entry.is_null() {
      return 1i32 as libc::c_char;
    }
  }
  /* Accepted */
  return 0i32 as libc::c_char;
}

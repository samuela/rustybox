

use crate::libbb::llist::llist_t;



use libc;
use libc::chdir;
use libc::chmod;
use libc::chown;
use libc::closelog;
use libc::dup2;
use libc::fstat;
use libc::getenv;
use libc::geteuid;
use libc::getopt;
use libc::getpid;
use libc::isatty;
use libc::kill;
use libc::openlog;
use libc::sigaddset;
use libc::sigemptyset;
use libc::sigprocmask;
use libc::sleep;
use libc::sscanf;
use libc::strcasecmp;
use libc::strcpy;
use libc::symlink;
use libc::syscall;
use libc::syslog;
use libc::time;
use libc::access;
use libc::atoi;
use libc::fclose;
use libc::fprintf;
use libc::lstat;
use libc::printf;
use libc::puts;
use libc::rename;
use libc::rmdir;
use libc::sprintf;
use libc::strchr;
use libc::strcmp;
use libc::strrchr;
use libc::strstr;
use libc::system;






extern "C" {

  #[no_mangle]
  fn find_list_entry2(list: *const llist_t, filename: *const libc::c_char) -> *const llist_t;
}

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
  let mut key: *const libc::c_char = 0 as *const libc::c_char;
  let mut reject_entry: *const llist_t = 0 as *const llist_t;
  let mut accept_entry: *const llist_t = 0 as *const llist_t;
  key = (*(*archive_handle).file_header).name;
  /* If the key is in a reject list fail */
  reject_entry = find_list_entry2((*archive_handle).reject, key);
  if !reject_entry.is_null() {
    return 1i32 as libc::c_char;
  }
  /* Fail if an accept list was specified and the key wasnt in there */
  if !(*archive_handle).accept.is_null() {
    accept_entry = find_list_entry2((*archive_handle).accept, key);
    if accept_entry.is_null() {
      return 1i32 as libc::c_char;
    }
  }
  /* Accepted */
  return 0i32 as libc::c_char;
}

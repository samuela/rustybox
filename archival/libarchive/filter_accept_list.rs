

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
  fn find_list_entry(list: *const llist_t, filename: *const libc::c_char) -> *const llist_t;
}

use crate::archival::libarchive::bb_archive::archive_handle_t;

/*
 * Copyright (C) 2002 by Glenn McGrath
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/*
 * Accept names that are in the accept list, ignoring reject list.
 */
#[no_mangle]
pub unsafe extern "C" fn filter_accept_list(
  mut archive_handle: *mut archive_handle_t,
) -> libc::c_char {
  if !find_list_entry(
    (*archive_handle).accept,
    (*(*archive_handle).file_header).name,
  )
  .is_null()
  {
    return 0i32 as libc::c_char;
  }
  return 1i32 as libc::c_char;
}

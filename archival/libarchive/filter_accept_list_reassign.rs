use crate::libbb::llist::llist_t;

use libc;
use libc::strcmp;
use libc::strrchr;
extern "C" {

  #[no_mangle]
  fn get_header_tar(archive_handle: *mut archive_handle_t) -> libc::c_char;
  #[no_mangle]
  fn get_header_tar_gz(archive_handle: *mut archive_handle_t) -> libc::c_char;
  #[no_mangle]
  fn get_header_tar_xz(archive_handle: *mut archive_handle_t) -> libc::c_char;
  #[no_mangle]
  fn get_header_tar_bz2(archive_handle: *mut archive_handle_t) -> libc::c_char;
  #[no_mangle]
  fn get_header_tar_lzma(archive_handle: *mut archive_handle_t) -> libc::c_char;
  #[no_mangle]
  fn find_list_entry(list: *const llist_t, filename: *const libc::c_char) -> *const llist_t;
}

/* Busybox does not use threads, we can speed up stdio. */
/* Above functions are required by POSIX.1-2008, below ones are extensions */
/* musl <= 1.1.15 does not support fflush_unlocked(NULL) */
//# undef  fflush
//# define fflush(stream) fflush_unlocked(stream)
/* Make all declarations hidden (-fvisibility flag only affects definitions) */
/* (don't include system headers after this until corresponding pop!) */
/* Tested to work correctly with all int types (IIRC :]) */
/* Large file support */
/* Note that CONFIG_LFS=y forces bbox to be built with all common ops
 * (stat, lseek etc) mapped to "largefile" variants by libc.
 * Practically it means that open() automatically has O_LARGEFILE added
 * and all filesize/file_offset parameters and struct members are "large"
 * (in today's world - signed 64bit). For full support of large files,
 * we need a few helper #defines (below) and careful use of off_t
 * instead of int/ssize_t. No lseek64(), O_LARGEFILE etc necessary */
/* CONFIG_LFS is on */
/* "long" is long enough on this system */
/* BSD-derived getopt() functions require that optind be set to 1 in
 * order to reset getopt() state.  This used to be generally accepted
 * way of resetting getopt().  However, glibc's getopt()
 * has additional getopt() state beyond optind (specifically, glibc
 * extensions such as '+' and '-' at the start of the string), and requires
 * that optind be set to zero to reset its state.  BSD-derived versions
 * of getopt() misbehaved if optind is set to 0 in order to reset getopt(),
 * and glibc's getopt() used to coredump if optind is set 1 in order
 * to reset getopt().
 * Then BSD introduced additional variable "optreset" which should be
 * set to 1 in order to reset getopt().  Sigh.  Standards, anyone?
 *
 * By ~2008, OpenBSD 3.4 was changed to survive glibc-like optind = 0
 * (to interpret it as if optreset was set).
 */
/*def __GLIBC__*/
/* BSD style */
/* Having next pointer as a first member allows easy creation
 * of "llist-compatible" structs, and using llist_FOO functions
 * on them.
 */
use crate::archival::libarchive::bb_archive::archive_handle_t;
/*
 * Copyright (C) 2002 by Glenn McGrath
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Built and used only if ENABLE_DPKG || ENABLE_DPKG_DEB */
/*
 * Reassign the subarchive metadata parser based on the filename extension
 * e.g. if its a .tar.gz modify archive_handle->sub_archive to process a .tar.gz
 * or if its a .tar.bz2 make archive_handle->sub_archive handle that
 */
#[no_mangle]
pub unsafe extern "C" fn filter_accept_list_reassign(
  mut archive_handle: *mut archive_handle_t,
) -> libc::c_char {
  /* Check the file entry is in the accept list */
  if !find_list_entry(
    (*archive_handle).accept,
    (*(*archive_handle).file_header).name,
  )
  .is_null()
  {
    let mut name_ptr: *const libc::c_char = 0 as *const libc::c_char;
    /* Find extension */
    name_ptr = strrchr((*(*archive_handle).file_header).name, '.' as i32);
    if name_ptr.is_null() {
      return 1i32 as libc::c_char;
    }
    name_ptr = name_ptr.offset(1);
    /* Modify the subarchive handler based on the extension */
    if strcmp(name_ptr, b"tar\x00" as *const u8 as *const libc::c_char) == 0i32 {
      (*archive_handle).dpkg__action_data_subarchive =
        Some(get_header_tar as unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char);
      return 0i32 as libc::c_char;
    }
    if 1i32 != 0 && strcmp(name_ptr, b"gz\x00" as *const u8 as *const libc::c_char) == 0i32 {
      (*archive_handle).dpkg__action_data_subarchive =
        Some(get_header_tar_gz as unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char);
      return 0i32 as libc::c_char;
    }
    if 1i32 != 0 && strcmp(name_ptr, b"bz2\x00" as *const u8 as *const libc::c_char) == 0i32 {
      (*archive_handle).dpkg__action_data_subarchive =
        Some(get_header_tar_bz2 as unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char);
      return 0i32 as libc::c_char;
    }
    if 1i32 != 0 && strcmp(name_ptr, b"lzma\x00" as *const u8 as *const libc::c_char) == 0i32 {
      (*archive_handle).dpkg__action_data_subarchive =
        Some(get_header_tar_lzma as unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char);
      return 0i32 as libc::c_char;
    }
    if 1i32 != 0 && strcmp(name_ptr, b"xz\x00" as *const u8 as *const libc::c_char) == 0i32 {
      (*archive_handle).dpkg__action_data_subarchive =
        Some(get_header_tar_xz as unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char);
      return 0i32 as libc::c_char;
    }
  }
  return 1i32 as libc::c_char;
}

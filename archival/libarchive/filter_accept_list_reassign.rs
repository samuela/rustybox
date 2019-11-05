use libc;
extern "C" {
  pub type hardlinks_t;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
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
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type smallint = libc::c_schar;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
pub type mode_t = __mode_t;
pub type dev_t = __dev_t;
pub type time_t = __time_t;
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
pub type uoff_t = libc::c_ulong;
use crate::librb::bb_uidgid_t;
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
use crate::libbb::llist::llist_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_header_t {
  pub name: *mut libc::c_char,
  pub link_target: *mut libc::c_char,
  pub tar__uname: *mut libc::c_char,
  pub tar__gname: *mut libc::c_char,
  pub size: off_t,
  pub uid: uid_t,
  pub gid: gid_t,
  pub mode: mode_t,
  pub mtime: time_t,
  pub device: dev_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_handle_t {
  pub ah_flags: libc::c_uint,
  pub src_fd: libc::c_int,
  pub filter: Option<unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char>,
  pub accept: *mut llist_t,
  pub reject: *mut llist_t,
  pub passed: *mut llist_t,
  pub file_header: *mut file_header_t,
  pub link_placeholders: *mut llist_t,
  pub action_header: Option<unsafe extern "C" fn(_: *const file_header_t) -> ()>,
  pub action_data: Option<unsafe extern "C" fn(_: *mut archive_handle_t) -> ()>,
  pub seek: Option<unsafe extern "C" fn(_: libc::c_int, _: off_t) -> ()>,
  pub offset: off_t,
  pub tar__strip_components: libc::c_uint,
  pub tar__end: smallint,
  pub tar__longname: *mut libc::c_char,
  pub tar__linkname: *mut libc::c_char,
  pub tar__to_command: *mut libc::c_char,
  pub tar__to_command_shell: *const libc::c_char,
  pub cpio__blocks: uoff_t,
  pub cpio__owner: bb_uidgid_t,
  pub cpio__hardlinks_to_create: *mut hardlinks_t,
  pub cpio__created_hardlinks: *mut hardlinks_t,
  pub dpkg__buffer: *mut libc::c_char,
  pub dpkg__action_data_subarchive:
    Option<unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char>,
  pub dpkg__sub_archive: *mut archive_handle_t,
}
/* vi: set sw=4 ts=4: */
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

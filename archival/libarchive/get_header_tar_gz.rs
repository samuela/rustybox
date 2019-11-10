






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




use libc::off_t;


extern "C" {

  #[no_mangle]
  fn get_header_tar(archive_handle: *mut archive_handle_t) -> libc::c_char;
  #[no_mangle]
  fn seek_by_read(fd: libc::c_int, amount: off_t);
  #[no_mangle]
  fn unpack_gz_stream(xstate: *mut transformer_state_t) -> libc::c_longlong;
  #[no_mangle]
  fn fork_transformer(
    fd: libc::c_int,
    signature_skipped: libc::c_int,
    transformer: Option<unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong>,
  );
}

use crate::archival::libarchive::bb_archive::archive_handle_t;
use crate::archival::libarchive::bb_archive::transformer_state_t;


/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn get_header_tar_gz(
  mut archive_handle: *mut archive_handle_t,
) -> libc::c_char {
  /* Can't lseek over pipes */
  (*archive_handle).seek =
    Some(seek_by_read as unsafe extern "C" fn(_: libc::c_int, _: off_t) -> ());
  fork_transformer(
    (*archive_handle).src_fd,
    0i32,
    Some(unpack_gz_stream as unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong),
  );
  (*archive_handle).offset = 0i32 as off_t;
  while get_header_tar(archive_handle) as libc::c_int == 0i32 {}
  /* Can only do one file at a time */
  return 1i32 as libc::c_char;
}

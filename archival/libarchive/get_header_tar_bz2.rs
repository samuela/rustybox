



use crate::librb::size_t;
use crate::librb::smallint;

use libc;
use libc::off_t;
use libc::time_t;

extern "C" {

  #[no_mangle]
  fn get_header_tar(archive_handle: *mut archive_handle_t) -> libc::c_char;

  #[no_mangle]
  fn seek_by_read(fd: libc::c_int, amount: off_t);

  #[no_mangle]
  fn unpack_bz2_stream(xstate: *mut transformer_state_t) -> libc::c_longlong;

  #[no_mangle]
  fn fork_transformer(
    fd: libc::c_int,
    signature_skipped: libc::c_int,
    transformer: Option<unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong>,
  );
}

use crate::archival::libarchive::bb_archive::archive_handle_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct transformer_state_t {
  pub signature_skipped: smallint,
  pub xformer: Option<unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong>,
  pub src_fd: libc::c_int,
  pub dst_fd: libc::c_int,
  pub mem_output_size_max: size_t,
  pub mem_output_size: size_t,
  pub mem_output_buf: *mut libc::c_char,
  pub bytes_out: off_t,
  pub bytes_in: off_t,
  pub crc32: u32,
  pub mtime: time_t,
  pub magic: C2RustUnnamed,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub b: [u8; 8],
  pub b16: [u16; 4],
  pub b32: [u32; 2],
}

/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn get_header_tar_bz2(
  mut archive_handle: *mut archive_handle_t,
) -> libc::c_char {
  /* Can't lseek over pipes */
  (*archive_handle).seek =
    Some(seek_by_read as unsafe extern "C" fn(_: libc::c_int, _: off_t) -> ());
  fork_transformer(
    (*archive_handle).src_fd,
    0i32,
    Some(
      unpack_bz2_stream as unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong,
    ),
  );
  (*archive_handle).offset = 0i32 as off_t;
  while get_header_tar(archive_handle) as libc::c_int == 0i32 {}
  /* Can only do one file at a time */
  return 1i32 as libc::c_char;
}

use libc;
use libc::open;



extern "C" {
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
  //TODO: supply a pointer to char[11] buffer (avoid statics)?
  #[no_mangle]
  fn bb_mode_string(mode: mode_t) -> *const libc::c_char;
  /* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
   * But potentially slow, don't use in one-billion-times loops */
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn utoa(n: libc::c_uint) -> *mut libc::c_char;
}


use libc::mode_t;

use libc::time_t;


use libc::tm;
use crate::archival::libarchive::bb_archive::file_header_t;

/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn header_verbose_list(mut file_header: *const file_header_t) {
  let mut tm_time: tm =std::mem::zeroed(); //localtime(&file_header->mtime);
  let mut ptm: *mut tm = &mut tm_time;
  let mut uid: [libc::c_char; 14] = [0; 14];
  /*char gid[sizeof(int)*3 + 2];*/
  let mut user: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut group: *mut libc::c_char = 0 as *mut libc::c_char;
  localtime_r(&(*file_header).mtime, ptm);
  user = (*file_header).tar__uname;
  if user.is_null() {
    sprintf(
      uid.as_mut_ptr(),
      b"%u\x00" as *const u8 as *const libc::c_char,
      (*file_header).uid,
    );
    user = uid.as_mut_ptr()
  }
  group = (*file_header).tar__gname;
  if group.is_null() {
    /*sprintf(gid, "%u", (unsigned)file_header->gid);*/
    group = utoa((*file_header).gid)
  }
  printf(
    b"%s %s/%s %9lu %4u-%02u-%02u %02u:%02u:%02u %s\x00" as *const u8 as *const libc::c_char,
    bb_mode_string((*file_header).mode),
    user,
    group,
    (*file_header).size,
    1900i32 + (*ptm).tm_year,
    1i32 + (*ptm).tm_mon,
    (*ptm).tm_mday,
    (*ptm).tm_hour,
    (*ptm).tm_min,
    (*ptm).tm_sec,
    (*file_header).name,
  );
  /* !FEATURE_TAR_UNAME_GNAME */
  /* FEATURE_TAR_UNAME_GNAME */
  /* NB: GNU tar shows "->" for symlinks and "link to" for hardlinks */
  if !(*file_header).link_target.is_null() {
    printf(
      b" -> %s\x00" as *const u8 as *const libc::c_char,
      (*file_header).link_target,
    );
  }
  bb_putchar('\n' as i32);
}

use libc;
extern "C" {
  pub type hardlinks_t;
  #[no_mangle]
  fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
  #[no_mangle]
  fn lseek(__fd: libc::c_int, __offset: __off64_t, __whence: libc::c_int) -> __off64_t;
  #[no_mangle]
  fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
  #[no_mangle]
  fn xstrndup(s: *const libc::c_char, n: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn xread_char(fd: libc::c_int) -> libc::c_uchar;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn data_skip(archive_handle: *mut archive_handle_t);
  #[no_mangle]
  fn create_links_from_list(list: *mut llist_t);
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type smallint = libc::c_schar;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
pub type mode_t = __mode_t;
pub type dev_t = __dev_t;
pub type time_t = __time_t;
pub type uoff_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bb_uidgid_t {
  pub uid: uid_t,
  pub gid: gid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct llist_t {
  pub link: *mut llist_t,
  pub data: *mut libc::c_char,
}
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
/*
 * busybox ar archive data structures
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ar_header {
  pub name: [libc::c_char; 16],
  pub date: [libc::c_char; 12],
  pub uid: [libc::c_char; 6],
  pub gid: [libc::c_char; 6],
  pub mode: [libc::c_char; 8],
  pub size: [libc::c_char; 10],
  pub magic: [libc::c_char; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub raw: [libc::c_char; 60],
  pub formatted: ar_header,
}
/* vi: set sw=4 ts=4: */
/*
 * Copyright 2001 Glenn McGrath.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* WARNING: Clobbers str[len], so fields must be read in reverse order! */
unsafe extern "C" fn read_num(
  mut str: *mut libc::c_char,
  mut base: libc::c_int,
  mut len: libc::c_int,
) -> libc::c_uint {
  let mut err: libc::c_int = 0;
  /* ar fields are fixed length text strings (padded with spaces).
   * Ensure bb_strtou doesn't read past the field in case the full
   * width is used. */
  *str.offset(len as isize) = 0i32 as libc::c_char;
  /* This code works because
   * on misformatted numbers bb_strtou returns all-ones */
  err = bb_strtou(str, 0 as *mut *mut libc::c_char, base) as libc::c_int;
  if err == -1i32 {
    bb_simple_error_msg_and_die(b"invalid ar header\x00" as *const u8 as *const libc::c_char);
  }
  return err as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn get_header_ar(mut archive_handle: *mut archive_handle_t) -> libc::c_char {
  let mut typed: *mut file_header_t = (*archive_handle).file_header;
  let mut size: libc::c_uint = 0;
  let mut ar: C2RustUnnamed = C2RustUnnamed { raw: [0; 60] };
  /* dont use xread as we want to handle the error ourself */
  if read(
    (*archive_handle).src_fd,
    ar.raw.as_mut_ptr() as *mut libc::c_void,
    60i32 as size_t,
  ) != 60i32 as libc::c_long
  {
    /* End Of File */
    return 1i32 as libc::c_char;
  }
  /* ar header starts on an even byte (2 byte aligned)
   * '\n' is used for padding
   */
  if ar.raw[0] as libc::c_int == '\n' as i32 {
    /* fix up the header, we started reading 1 byte too early */
    memmove(
      ar.raw.as_mut_ptr() as *mut libc::c_void,
      &mut *ar.raw.as_mut_ptr().offset(1) as *mut libc::c_char as *const libc::c_void,
      59i32 as libc::c_ulong,
    );
    ar.raw[59] = xread_char((*archive_handle).src_fd) as libc::c_char;
    (*archive_handle).offset += 1
  }
  (*archive_handle).offset += 60i32 as libc::c_long;
  if ar.formatted.magic[0] as libc::c_int != '`' as i32
    || ar.formatted.magic[1] as libc::c_int != '\n' as i32
  {
    bb_simple_error_msg_and_die(b"invalid ar header\x00" as *const u8 as *const libc::c_char);
  }
  /*
   * Note that the fields MUST be read in reverse order as
   * read_num() clobbers the next byte after the field!
   * Order is: name, date, uid, gid, mode, size, magic.
   */
  size = read_num(
    ar.formatted.size.as_mut_ptr(),
    10i32,
    ::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as libc::c_int,
  );
  (*typed).size = size as off_t;
  /* special filenames have '/' as the first character */
  if ar.formatted.name[0] as libc::c_int == '/' as i32 {
    if ar.formatted.name[1] as libc::c_int == ' ' as i32 {
      /* This is the index of symbols in the file for compilers */
      data_skip(archive_handle);
      (*archive_handle).offset += size as libc::c_long;
      return get_header_ar(archive_handle);
      /* Return next header */
    }
    bb_simple_error_msg_and_die(
      b"long filenames not supported\x00" as *const u8 as *const libc::c_char,
    );
  }
  /* Only size is always present, the rest may be missing in
   * long filename pseudo file. Thus we decode the rest
   * after dealing with long filename pseudo file.
   */
  (*typed).mode = read_num(
    ar.formatted.mode.as_mut_ptr(),
    8i32,
    ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as libc::c_int,
  );
  (*typed).gid = read_num(
    ar.formatted.gid.as_mut_ptr(),
    10i32,
    ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as libc::c_int,
  );
  (*typed).uid = read_num(
    ar.formatted.uid.as_mut_ptr(),
    10i32,
    ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as libc::c_int,
  );
  (*typed).mtime = read_num(
    ar.formatted.date.as_mut_ptr(),
    10i32,
    ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as libc::c_int,
  ) as time_t;
  /* short filenames */
  (*typed).name = xstrndup(ar.formatted.name.as_mut_ptr(), 16i32);
  *(*typed)
    .name
    .offset(strcspn((*typed).name, b" /\x00" as *const u8 as *const libc::c_char) as isize) =
    '\u{0}' as i32 as libc::c_char;
  if (*archive_handle).filter.expect("non-null function pointer")(archive_handle) as libc::c_int
    == 0i32
  {
    (*archive_handle)
      .action_header
      .expect("non-null function pointer")(typed);
    if !(*archive_handle).dpkg__sub_archive.is_null() {
      let mut sa: *mut archive_handle_t = (*archive_handle).dpkg__sub_archive;
      while (*archive_handle)
        .dpkg__action_data_subarchive
        .expect("non-null function pointer")(sa) as libc::c_int
        == 0i32
      {}
      create_links_from_list((*sa).link_placeholders);
    } else {
      (*archive_handle)
        .action_data
        .expect("non-null function pointer")(archive_handle);
    }
  } else {
    data_skip(archive_handle);
  }
  (*archive_handle).offset += (*typed).size;
  /* Set the file pointer to the correct spot, we may have been reading a compressed file */
  lseek((*archive_handle).src_fd, (*archive_handle).offset, 0i32);
  return 0i32 as libc::c_char;
}

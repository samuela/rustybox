// Things that used to live in include/bb_archive.h.

#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_header_t {
  pub name: *mut libc::c_char,
  pub link_target: *mut libc::c_char,
  pub tar__uname: *mut libc::c_char,
  pub tar__gname: *mut libc::c_char,
  pub size: libc::off_t,
  pub uid: libc::uid_t,
  pub gid: libc::gid_t,
  pub mode: libc::mode_t,
  pub mtime: libc::time_t,
  pub device: libc::dev_t,
}

// Declared in bb_archive.h but defined in get_header_cpio.c.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hardlinks_t {
  pub next: *mut hardlinks_t,
  pub inode: libc::c_int,
  pub mode: libc::c_int,
  pub mtime: libc::c_int,
  pub uid: libc::c_int,
  pub gid: libc::c_int,
  pub name: [libc::c_char; 1],
}

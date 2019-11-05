use libc;
extern "C" {
  pub type __dirstream;
  #[no_mangle]
  fn opendir(__name: *const libc::c_char) -> *mut DIR;
  #[no_mangle]
  fn closedir(__dirp: *mut DIR) -> libc::c_int;
  #[no_mangle]
  fn readdir(__dirp: *mut DIR) -> *mut dirent;
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn putc_unlocked(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
  #[no_mangle]
  fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __ino64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
  pub d_ino: __ino64_t,
  pub d_off: __off64_t,
  pub d_reclen: libc::c_ushort,
  pub d_type: libc::c_uchar,
  pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
use crate::librb::stat;
use crate::librb::timespec;
use crate::librb::_IO_FILE;
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
  pub _next: *mut _IO_marker,
  pub _sbuf: *mut _IO_FILE,
  pub _pos: libc::c_int,
}
use crate::librb::FILE;
/* vi: set sw=4 ts=4: */
/*
 * See README for additional information
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
unsafe extern "C" fn close_silently(mut fd: libc::c_int) {
  let mut e: libc::c_int = *bb_errno;
  close(fd);
  *bb_errno = e;
}
/* Iterate a function on each entry of a directory */
#[no_mangle]
pub unsafe extern "C" fn iterate_on_dir(
  mut dir_name: *const libc::c_char,
  mut func: Option<
    unsafe extern "C" fn(
      _: *const libc::c_char,
      _: *mut dirent,
      _: *mut libc::c_void,
    ) -> libc::c_int,
  >,
  mut private: *mut libc::c_void,
) -> libc::c_int {
  let mut dir: *mut DIR = 0 as *mut DIR;
  let mut de: *mut dirent = 0 as *mut dirent;
  dir = opendir(dir_name);
  if dir.is_null() {
    return -1i32;
  }
  loop {
    de = readdir(dir);
    if de.is_null() {
      break;
    }
    func.expect("non-null function pointer")(dir_name, de, private);
  }
  closedir(dir);
  return 0i32;
}
/* Get/set a file version on an ext2 file system */
#[no_mangle]
pub unsafe extern "C" fn fgetsetversion(
  mut name: *const libc::c_char,
  mut get_version: *mut libc::c_ulong,
  mut set_version: libc::c_ulong,
) -> libc::c_int {
  let mut fd: libc::c_int = 0;
  let mut r: libc::c_int = 0;
  let mut ver: libc::c_int = 0;
  fd = open(name, 0i32 | 0o4000i32);
  if fd == -1i32 {
    return -1i32;
  }
  if get_version.is_null() {
    ver = set_version as libc::c_int;
    r = ioctl(
      fd,
      (1u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (('v' as i32) << 0i32 + 8i32) as libc::c_uint
        | (2i32 << 0i32) as libc::c_uint) as libc::c_ulong
        | (::std::mem::size_of::<libc::c_long>() as libc::c_ulong) << 0i32 + 8i32 + 8i32,
      &mut ver as *mut libc::c_int,
    )
  } else {
    r = ioctl(
      fd,
      (2u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (('v' as i32) << 0i32 + 8i32) as libc::c_uint
        | (1i32 << 0i32) as libc::c_uint) as libc::c_ulong
        | (::std::mem::size_of::<libc::c_long>() as libc::c_ulong) << 0i32 + 8i32 + 8i32,
      &mut ver as *mut libc::c_int,
    );
    *get_version = ver as libc::c_ulong
  }
  close_silently(fd);
  return r;
  /* ! HAVE_EXT2_IOCTLS */
  /* ! HAVE_EXT2_IOCTLS */
}
/* vi: set sw=4 ts=4: */
/*
 * See README for additional information
 *
 * This file can be redistributed under the terms of the GNU Library General
 * Public License
 */
/* Constants and structures */
/* Iterate a function on each entry of a directory */
/* Get/set a file version on an ext2 file system */
/* Get/set a file flags on an ext2 file system */
/* Get/set a file flags on an ext2 file system */
#[no_mangle]
pub unsafe extern "C" fn fgetsetflags(
  mut name: *const libc::c_char,
  mut get_flags: *mut libc::c_ulong,
  mut set_flags: libc::c_ulong,
) -> libc::c_int {
  let mut buf: stat = stat {
    st_dev: 0,
    st_ino: 0,
    st_nlink: 0,
    st_mode: 0,
    st_uid: 0,
    st_gid: 0,
    __pad0: 0,
    st_rdev: 0,
    st_size: 0,
    st_blksize: 0,
    st_blocks: 0,
    st_atim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_mtim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_ctim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    __glibc_reserved: [0; 3],
  };
  let mut fd: libc::c_int = 0;
  let mut r: libc::c_int = 0;
  let mut f: libc::c_int = 0;
  if stat(name, &mut buf) == 0i32
    && !(buf.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint)
    && !(buf.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint)
  {
    /* HAVE_EXT2_IOCTLS */
    *bb_errno = 95i32; /* neither read nor write asked for */
    return -1i32;
  } else {
    fd = open(name, 0i32 | 0o4000i32);
    if fd == -1i32 {
      return -1i32;
    }
    if get_flags.is_null() {
      f = set_flags as libc::c_int;
      r = ioctl(
        fd,
        (1u32 << 0i32 + 8i32 + 8i32 + 14i32
          | (('f' as i32) << 0i32 + 8i32) as libc::c_uint
          | (2i32 << 0i32) as libc::c_uint) as libc::c_ulong
          | (::std::mem::size_of::<libc::c_long>() as libc::c_ulong) << 0i32 + 8i32 + 8i32,
        &mut f as *mut libc::c_int,
      )
    } else {
      r = ioctl(
        fd,
        (2u32 << 0i32 + 8i32 + 8i32 + 14i32
          | (('f' as i32) << 0i32 + 8i32) as libc::c_uint
          | (1i32 << 0i32) as libc::c_uint) as libc::c_ulong
          | (::std::mem::size_of::<libc::c_long>() as libc::c_ulong) << 0i32 + 8i32 + 8i32,
        &mut f as *mut libc::c_int,
      );
      *get_flags = f as libc::c_ulong
    }
    close_silently(fd);
    return r;
  };
}
/* Print file attributes on an ext2 file system */
#[no_mangle]
pub static mut e2attr_flags_value: [uint32_t; 13] = [
  0x1000i32 as uint32_t,
  0x1i32 as uint32_t,
  0x2i32 as uint32_t,
  0x8i32 as uint32_t,
  0x10000i32 as uint32_t,
  0x10i32 as uint32_t,
  0x20i32 as uint32_t,
  0x40i32 as uint32_t,
  0x80i32 as uint32_t,
  0x4i32 as uint32_t,
  0x4000i32 as uint32_t,
  0x8000i32 as uint32_t,
  0x20000i32 as uint32_t,
];
#[no_mangle]
pub static mut e2attr_flags_sname: [libc::c_char; 14] =
  [73, 115, 117, 83, 68, 105, 97, 100, 65, 99, 106, 116, 84, 0];
static mut e2attr_flags_lname: [libc::c_char; 214] = [
  73, 110, 100, 101, 120, 101, 100, 95, 100, 105, 114, 101, 99, 116, 111, 114, 121, 0, 83, 101, 99,
  117, 114, 101, 95, 68, 101, 108, 101, 116, 105, 111, 110, 0, 85, 110, 100, 101, 108, 101, 116,
  101, 0, 83, 121, 110, 99, 104, 114, 111, 110, 111, 117, 115, 95, 85, 112, 100, 97, 116, 101, 115,
  0, 83, 121, 110, 99, 104, 114, 111, 110, 111, 117, 115, 95, 68, 105, 114, 101, 99, 116, 111, 114,
  121, 95, 85, 112, 100, 97, 116, 101, 115, 0, 73, 109, 109, 117, 116, 97, 98, 108, 101, 0, 65,
  112, 112, 101, 110, 100, 95, 79, 110, 108, 121, 0, 78, 111, 95, 68, 117, 109, 112, 0, 78, 111,
  95, 65, 116, 105, 109, 101, 0, 67, 111, 109, 112, 114, 101, 115, 115, 105, 111, 110, 95, 82, 101,
  113, 117, 101, 115, 116, 101, 100, 0, 74, 111, 117, 114, 110, 97, 108, 101, 100, 95, 68, 97, 116,
  97, 0, 78, 111, 95, 84, 97, 105, 108, 109, 101, 114, 103, 105, 110, 103, 0, 84, 111, 112, 95,
  111, 102, 95, 68, 105, 114, 101, 99, 116, 111, 114, 121, 95, 72, 105, 101, 114, 97, 114, 99, 104,
  105, 101, 115, 0, 0,
];
/* Print file attributes on an ext2 file system */
/* Another trailing NUL is added by compiler */
#[no_mangle]
pub unsafe extern "C" fn print_e2flags(
  mut f: *mut FILE,
  mut flags: libc::c_ulong,
  mut options: libc::c_uint,
) {
  let mut fv: *const uint32_t = 0 as *const uint32_t;
  let mut fn_0: *const libc::c_char = 0 as *const libc::c_char;
  fv = e2attr_flags_value.as_ptr();
  if options & 1i32 as libc::c_uint != 0 {
    let mut first: libc::c_int = 1i32;
    fn_0 = e2attr_flags_lname.as_ptr();
    loop {
      if flags & *fv as libc::c_ulong != 0 {
        if first == 0 {
          fputs_unlocked(b", \x00" as *const u8 as *const libc::c_char, f);
        }
        fputs_unlocked(fn_0, f);
        first = 0i32
      }
      fv = fv.offset(1);
      fn_0 = fn_0.offset(strlen(fn_0).wrapping_add(1i32 as libc::c_ulong) as isize);
      if !(*fn_0 != 0) {
        break;
      }
    }
    if first != 0 {
      fputs_unlocked(b"---\x00" as *const u8 as *const libc::c_char, f);
    }
  } else {
    fn_0 = e2attr_flags_sname.as_ptr();
    loop {
      let mut c: libc::c_char = '-' as i32 as libc::c_char;
      if flags & *fv as libc::c_ulong != 0 {
        c = *fn_0
      }
      putc_unlocked(c as libc::c_int, f);
      fv = fv.offset(1);
      fn_0 = fn_0.offset(1);
      if !(*fn_0 != 0) {
        break;
      }
    }
  };
}

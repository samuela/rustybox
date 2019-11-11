use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::appletlib::applet_name;
use libc;
use libc::close;
use libc::closedir;
use libc::ioctl;
use libc::open;
use libc::opendir;
use libc::readdir;
extern "C" {

  #[no_mangle]
  fn putc_unlocked(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;


}

use crate::librb::size_t;
use libc::dirent;
use libc::stat;
use libc::DIR;
use libc::FILE;
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
  let mut buf: stat = std::mem::zeroed();
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
pub static mut e2attr_flags_value: [u32; 13] = [
  0x1000i32 as u32,
  0x1i32 as u32,
  0x2i32 as u32,
  0x8i32 as u32,
  0x10000i32 as u32,
  0x10i32 as u32,
  0x20i32 as u32,
  0x40i32 as u32,
  0x80i32 as u32,
  0x4i32 as u32,
  0x4000i32 as u32,
  0x8000i32 as u32,
  0x20000i32 as u32,
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
  let mut fv: *const u32 = 0 as *const u32;
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

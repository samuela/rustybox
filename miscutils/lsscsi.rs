use crate::libbb::ptr_to_globals::bb_errno;
use crate::librb::size_t;
use libc;
use libc::chdir;
use libc::dirent;
use libc::printf;
use libc::readdir;
use libc::ssize_t;
use libc::strchr;
use libc::DIR;

/*
 * lsscsi implementation for busybox
 *
 * Copyright (C) 2017 Markus Gothe <nietzsche@lysator.liu.se>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config LSSCSI
//config:	bool "lsscsi (2.5 kb)"
//config:	default y
//config:	#select PLATFORM_LINUX
//config:	help
//config:	lsscsi is a utility for displaying information about SCSI buses in the
//config:	system and devices connected to them.
//config:
//config:	This version uses sysfs (/sys/bus/scsi/devices) only.
//applet:IF_LSSCSI(APPLET_NOEXEC(lsscsi, lsscsi, BB_DIR_USR_BIN, SUID_DROP, lsscsi))
//kbuild:lib-$(CONFIG_LSSCSI) += lsscsi.o
//usage:#define lsscsi_trivial_usage NOUSAGE_STR
//usage:#define lsscsi_full_usage ""
static mut scsi_dir: [libc::c_char; 22] = [
  47, 115, 121, 115, 47, 98, 117, 115, 47, 115, 99, 115, 105, 47, 100, 101, 118, 105, 99, 101, 115,
  0,
];
unsafe extern "C" fn get_line(
  mut filename: *const libc::c_char,
  mut buf: *mut libc::c_char,
  mut bufsize_p: *mut libc::c_uint,
) -> *mut libc::c_char {
  let mut bufsize: libc::c_uint = *bufsize_p;
  let mut sz: ssize_t = 0;
  if bufsize.wrapping_sub(2i32 as libc::c_uint) as libc::c_int <= 0 {
    return buf;
  }
  sz = crate::libbb::read::open_read_close(
    filename,
    buf as *mut libc::c_void,
    bufsize.wrapping_sub(2i32 as libc::c_uint) as size_t,
  );
  if sz < 0 {
    sz = 0
  }
  *buf.offset(sz as isize) = '\u{0}' as i32 as libc::c_char;
  sz = crate::libbb::trim::trim(buf).wrapping_offset_from(buf) + 1;
  bufsize -= sz as u32;
  buf = buf.offset(sz as isize);
  *buf.offset(0) = '\u{0}' as i32 as libc::c_char;
  *bufsize_p = bufsize;
  return buf;
}
pub unsafe fn lsscsi_main(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut de: *mut dirent = std::ptr::null_mut();
  let mut dir: *mut DIR = std::ptr::null_mut();
  crate::libbb::xfuncs_printf::xchdir(scsi_dir.as_ptr());
  dir = crate::libbb::xfuncs_printf::xopendir(b".\x00" as *const u8 as *const libc::c_char);
  loop {
    de = readdir(dir);
    if de.is_null() {
      break;
    }
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut ptr: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut bufsize: libc::c_uint = 0;
    let mut vendor: *const libc::c_char = std::ptr::null();
    let mut type_str: *const libc::c_char = std::ptr::null();
    let mut type_name: *const libc::c_char = std::ptr::null();
    let mut model: *const libc::c_char = std::ptr::null();
    let mut rev: *const libc::c_char = std::ptr::null();
    let mut type_0: libc::c_uint = 0;
    if !(((*de).d_name[0] as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32) {
      continue;
    }
    if strchr((*de).d_name.as_mut_ptr(), ':' as i32).is_null() {
      continue;
    }
    if chdir((*de).d_name.as_mut_ptr()) != 0 {
      continue;
    }
    bufsize = ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_uint;
    vendor = buf.as_mut_ptr();
    ptr = get_line(
      b"vendor\x00" as *const u8 as *const libc::c_char,
      buf.as_mut_ptr(),
      &mut bufsize,
    );
    type_str = ptr;
    ptr = get_line(
      b"type\x00" as *const u8 as *const libc::c_char,
      ptr,
      &mut bufsize,
    );
    model = ptr;
    ptr = get_line(
      b"model\x00" as *const u8 as *const libc::c_char,
      ptr,
      &mut bufsize,
    );
    rev = ptr;
    ptr = get_line(
      b"rev\x00" as *const u8 as *const libc::c_char,
      ptr,
      &mut bufsize,
    );
    printf(
      b"[%s]\t\x00" as *const u8 as *const libc::c_char,
      (*de).d_name.as_mut_ptr(),
    );
    type_0 = crate::libbb::bb_strtonum::bb_strtou(type_str, 0 as *mut *mut libc::c_char, 10i32);
    if *bb_errno != 0
      || type_0 >= 0x20i32 as libc::c_uint
      || {
        type_name =
                       crate::libbb::compare_string_array::nth_string(b"disk\x00tape\x00printer\x00process\x00worm\x00\x00scanner\x00optical\x00mediumx\x00comms\x00\x00\x00storage\x00enclosu\x00sim dsk\x00opti rd\x00bridge\x00osd\x00adi\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00wlun\x00no dev\x00"
                                      as *const u8 as *const libc::c_char,
                                  type_0 as libc::c_int);
        (*type_name.offset(0) as libc::c_int) == '\u{0}' as i32
      }
    {
      printf(b"(%s)\t\x00" as *const u8 as *const libc::c_char, type_str);
    } else {
      printf(b"%s\t\x00" as *const u8 as *const libc::c_char, type_name);
    }
    printf(
      b"%s\t%s\t%s\n\x00" as *const u8 as *const libc::c_char,
      vendor,
      model,
      rev,
    );
    /* TODO: also output device column, e.g. "/dev/sdX" */
    /* chdir("..") may not work as expected,
     * since we might have followed a symlink.
     */
    crate::libbb::xfuncs_printf::xchdir(scsi_dir.as_ptr());
  }
  return 0;
}

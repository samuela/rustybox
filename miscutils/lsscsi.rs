use crate::librb::size_t;
use libc;
use libc::dirent;
use libc::ssize_t;
use libc::DIR;

extern "C" {

  #[no_mangle]
  fn chdir(__path: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn closedir(__dirp: *mut DIR) -> libc::c_int;
  #[no_mangle]
  fn readdir(__dirp: *mut DIR) -> *mut dirent;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn trim(s: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xopendir(path: *const libc::c_char) -> *mut DIR;
  #[no_mangle]
  fn xchdir(path: *const libc::c_char);
  #[no_mangle]
  fn open_read_close(
    filename: *const libc::c_char,
    buf: *mut libc::c_void,
    maxsz: size_t,
  ) -> ssize_t;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  #[no_mangle]
  fn nth_string(strings: *const libc::c_char, n: libc::c_int) -> *const libc::c_char;
}

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
//applet:IF_LSSCSI(APPLET_NOEXEC(lsscsi, lsscsi, BB_DIR_USR_BIN, BB_SUID_DROP, lsscsi))
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
  if bufsize.wrapping_sub(2i32 as libc::c_uint) as libc::c_int <= 0i32 {
    return buf;
  }
  sz = open_read_close(
    filename,
    buf as *mut libc::c_void,
    bufsize.wrapping_sub(2i32 as libc::c_uint) as size_t,
  );
  if sz < 0 {
    sz = 0
  }
  *buf.offset(sz as isize) = '\u{0}' as i32 as libc::c_char;
  sz = trim(buf).wrapping_offset_from(buf) + 1;
  bufsize -= sz as u32;
  buf = buf.offset(sz as isize);
  *buf.offset(0) = '\u{0}' as i32 as libc::c_char;
  *bufsize_p = bufsize;
  return buf;
}
#[no_mangle]
pub unsafe extern "C" fn lsscsi_main(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut de: *mut dirent = 0 as *mut dirent;
  let mut dir: *mut DIR = 0 as *mut DIR;
  xchdir(scsi_dir.as_ptr());
  dir = xopendir(b".\x00" as *const u8 as *const libc::c_char);
  loop {
    de = readdir(dir);
    if de.is_null() {
      break;
    }
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bufsize: libc::c_uint = 0;
    let mut vendor: *const libc::c_char = 0 as *const libc::c_char;
    let mut type_str: *const libc::c_char = 0 as *const libc::c_char;
    let mut type_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut model: *const libc::c_char = 0 as *const libc::c_char;
    let mut rev: *const libc::c_char = 0 as *const libc::c_char;
    let mut type_0: libc::c_uint = 0;
    if !(((*de).d_name[0] as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32) {
      continue;
    }
    if strchr((*de).d_name.as_mut_ptr(), ':' as i32).is_null() {
      continue;
    }
    if chdir((*de).d_name.as_mut_ptr()) != 0i32 {
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
    type_0 = bb_strtou(type_str, 0 as *mut *mut libc::c_char, 10i32);
    if *bb_errno != 0
      || type_0 >= 0x20i32 as libc::c_uint
      || {
        type_name =
                       nth_string(b"disk\x00tape\x00printer\x00process\x00worm\x00\x00scanner\x00optical\x00mediumx\x00comms\x00\x00\x00storage\x00enclosu\x00sim dsk\x00opti rd\x00bridge\x00osd\x00adi\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00wlun\x00no dev\x00"
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
    xchdir(scsi_dir.as_ptr());
  }
  return 0i32;
}

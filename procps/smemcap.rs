use libc;
extern "C" {
  pub type __dirstream;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn closedir(__dirp: *mut DIR) -> libc::c_int;
  #[no_mangle]
  fn readdir(__dirp: *mut DIR) -> *mut dirent;
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xopendir(path: *const libc::c_char) -> *mut DIR;
  #[no_mangle]
  fn xchdir(path: *const libc::c_char);
  // NB: will return short read on error, not -1,
  // if some data was read before error occurred
  #[no_mangle]
  fn full_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn xwrite(fd: libc::c_int, buf: *const libc::c_void, count: size_t);
}

use libc::ino64_t;
use libc::mode_t;

use libc::off64_t;
use libc::off_t;

use crate::librb::size_t;
use crate::librb::ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
  pub d_ino: ino64_t,
  pub d_off: off64_t,
  pub d_reclen: libc::c_ushort,
  pub d_type: libc::c_uchar,
  pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;

use libc::stat;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tar_header_t {
  pub name: [libc::c_char; 100],
  pub mode: [libc::c_char; 8],
  pub uid: [libc::c_char; 8],
  pub gid: [libc::c_char; 8],
  pub size: [libc::c_char; 12],
  pub mtime: [libc::c_char; 12],
  pub chksum: [libc::c_char; 8],
  pub typeflag: libc::c_char,
  pub linkname: [libc::c_char; 100],
  pub magic: [libc::c_char; 8],
  pub uname: [libc::c_char; 32],
  pub gname: [libc::c_char; 32],
  pub devmajor: [libc::c_char; 8],
  pub devminor: [libc::c_char; 8],
  pub prefix: [libc::c_char; 155],
  pub padding: [libc::c_char; 12],
}
/*
 smemcap - a tool for meaningful memory reporting

 Copyright 2008-2009 Matt Mackall <mpm@selenic.com>

 This software may be used and distributed according to the terms of
 the GNU General Public License version 2 or later, incorporated
 herein by reference.
*/
//config:config SMEMCAP
//config:	bool "smemcap (2.5 kb)"
//config:	default y
//config:	help
//config:	smemcap is a tool for capturing process data for smem,
//config:	a memory usage statistic tool.
//applet:IF_SMEMCAP(APPLET(smemcap, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_SMEMCAP) += smemcap.o
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fileblock {
  pub next: *mut fileblock,
  pub data: [libc::c_char; 512],
}
unsafe extern "C" fn writeheader(
  mut path: *const libc::c_char,
  mut sb: *mut stat,
  mut type_0: libc::c_int,
) {
  let mut header: tar_header_t = tar_header_t {
    name: [0; 100],
    mode: [0; 8],
    uid: [0; 8],
    gid: [0; 8],
    size: [0; 12],
    mtime: [0; 12],
    chksum: [0; 8],
    typeflag: 0,
    linkname: [0; 100],
    magic: [0; 8],
    uname: [0; 32],
    gname: [0; 32],
    devmajor: [0; 8],
    devminor: [0; 8],
    prefix: [0; 155],
    padding: [0; 12],
  };
  let mut i: libc::c_int = 0;
  let mut sum: libc::c_int = 0;
  memset(
    &mut header as *mut tar_header_t as *mut libc::c_void,
    0i32,
    512i32 as libc::c_ulong,
  );
  strcpy(header.name.as_mut_ptr(), path);
  sprintf(
    header.mode.as_mut_ptr(),
    b"%o\x00" as *const u8 as *const libc::c_char,
    (*sb).st_mode & 0o777i32 as libc::c_uint,
  );
  /* careful to not overflow fields! */
  sprintf(
    header.uid.as_mut_ptr(),
    b"%o\x00" as *const u8 as *const libc::c_char,
    (*sb).st_uid & 0o7777777i32 as libc::c_uint,
  ); /* like GNU tar */
  sprintf(
    header.gid.as_mut_ptr(),
    b"%o\x00" as *const u8 as *const libc::c_char,
    (*sb).st_gid & 0o7777777i32 as libc::c_uint,
  );
  sprintf(
    header.size.as_mut_ptr(),
    b"%o\x00" as *const u8 as *const libc::c_char,
    (*sb).st_size as libc::c_uint,
  );
  sprintf(
    header.mtime.as_mut_ptr(),
    b"%llo\x00" as *const u8 as *const libc::c_char,
    (*sb).st_mtime as libc::c_longlong & 0o77777777777i64,
  );
  header.typeflag = type_0 as libc::c_char;
  strcpy(
    header.magic.as_mut_ptr(),
    b"ustar  \x00" as *const u8 as *const libc::c_char,
  );
  /* Calculate and store the checksum (the sum of all of the bytes of
   * the header). The checksum field must be filled with blanks for the
   * calculation. The checksum field is formatted differently from the
   * other fields: it has 6 digits, a NUL, then a space -- rather than
   * digits, followed by a NUL like the other fields... */
  header.chksum[7] = ' ' as i32 as libc::c_char;
  sum = ' ' as i32 * 7i32;
  i = 0i32;
  while i < 512i32 {
    sum +=
      *(&mut header as *mut tar_header_t as *mut libc::c_uchar).offset(i as isize) as libc::c_int;
    i += 1
  }
  sprintf(
    header.chksum.as_mut_ptr(),
    b"%06o\x00" as *const u8 as *const libc::c_char,
    sum,
  );
  xwrite(
    1i32,
    &mut header as *mut tar_header_t as *const libc::c_void,
    512i32 as size_t,
  );
}
unsafe extern "C" fn archivefile(mut path: *const libc::c_char) {
  let mut start: *mut fileblock = 0 as *mut fileblock;
  let mut cur: *mut fileblock = 0 as *mut fileblock;
  let mut prev: *mut *mut fileblock = &mut start;
  let mut fd: libc::c_int = 0;
  let mut r: libc::c_int = 0;
  let mut size: libc::c_uint = 0i32 as libc::c_uint;
  let mut s: stat = std::mem::zeroed();
  /* buffer the file */
  fd = open(path, 0i32);
  if fd == -1i32 {
    /* skip vanished processes between dir listing and traversal */
    return;
  }
  loop {
    cur = xzalloc(::std::mem::size_of::<fileblock>() as libc::c_ulong) as *mut fileblock;
    *prev = cur;
    prev = &mut (*cur).next;
    r = full_read(
      fd,
      (*cur).data.as_mut_ptr() as *mut libc::c_void,
      512i32 as size_t,
    ) as libc::c_int;
    if r > 0i32 {
      size = size.wrapping_add(r as libc::c_uint)
    }
    if !(r == 512i32) {
      break;
    }
  }
  /* write archive header */
  fstat(fd, &mut s);
  close(fd);
  s.st_size = size as off_t;
  writeheader(path, &mut s, '0' as i32);
  /* dump file contents */
  cur = start;
  while size as libc::c_int > 0i32 {
    xwrite(
      1i32,
      (*cur).data.as_mut_ptr() as *const libc::c_void,
      512i32 as size_t,
    );
    start = cur;
    cur = (*cur).next;
    free(start as *mut libc::c_void);
    size = size.wrapping_sub(512i32 as libc::c_uint)
  }
}
unsafe extern "C" fn archivejoin(mut sub: *const libc::c_char, mut name: *const libc::c_char) {
  let mut path: [libc::c_char; 33] = [0; 33];
  sprintf(
    path.as_mut_ptr(),
    b"%s/%s\x00" as *const u8 as *const libc::c_char,
    sub,
    name,
  );
  archivefile(path.as_mut_ptr());
}
//usage:#define smemcap_trivial_usage ">SMEMDATA.TAR"
//usage:#define smemcap_full_usage "\n\n"
//usage:       "Collect memory usage data in /proc and write it to stdout"
#[no_mangle]
pub unsafe extern "C" fn smemcap_main(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut d: *mut DIR = 0 as *mut DIR;
  let mut de: *mut dirent = 0 as *mut dirent;
  xchdir(b"/proc\x00" as *const u8 as *const libc::c_char);
  d = xopendir(b".\x00" as *const u8 as *const libc::c_char);
  archivefile(b"meminfo\x00" as *const u8 as *const libc::c_char);
  archivefile(b"version\x00" as *const u8 as *const libc::c_char);
  loop {
    de = readdir(d);
    if de.is_null() {
      break;
    }
    if ((*de).d_name[0] as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
      let mut s: stat = std::mem::zeroed();
      memset(
        &mut s as *mut stat as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<stat>() as libc::c_ulong,
      );
      s.st_mode = 0o555i32 as mode_t;
      writeheader((*de).d_name.as_mut_ptr(), &mut s, '5' as i32);
      archivejoin(
        (*de).d_name.as_mut_ptr(),
        b"smaps\x00" as *const u8 as *const libc::c_char,
      );
      archivejoin(
        (*de).d_name.as_mut_ptr(),
        b"cmdline\x00" as *const u8 as *const libc::c_char,
      );
      archivejoin(
        (*de).d_name.as_mut_ptr(),
        b"stat\x00" as *const u8 as *const libc::c_char,
      );
    }
  }
  return 0i32;
}

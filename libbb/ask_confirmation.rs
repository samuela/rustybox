use libc;

extern "C" {
  #[no_mangle]
  static mut stdin: *mut _IO_FILE;

  #[no_mangle]
  fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn fflush_all() -> libc::c_int;
}

pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
  pub _flags: libc::c_int,
  pub _IO_read_ptr: *mut libc::c_char,
  pub _IO_read_end: *mut libc::c_char,
  pub _IO_read_base: *mut libc::c_char,
  pub _IO_write_base: *mut libc::c_char,
  pub _IO_write_ptr: *mut libc::c_char,
  pub _IO_write_end: *mut libc::c_char,
  pub _IO_buf_base: *mut libc::c_char,
  pub _IO_buf_end: *mut libc::c_char,
  pub _IO_save_base: *mut libc::c_char,
  pub _IO_backup_base: *mut libc::c_char,
  pub _IO_save_end: *mut libc::c_char,
  pub _markers: *mut _IO_marker,
  pub _chain: *mut _IO_FILE,
  pub _fileno: libc::c_int,
  pub _flags2: libc::c_int,
  pub _old_offset: __off_t,
  pub _cur_column: libc::c_ushort,
  pub _vtable_offset: libc::c_schar,
  pub _shortbuf: [libc::c_char; 1],
  pub _lock: *mut libc::c_void,
  pub _offset: __off64_t,
  pub __pad1: *mut libc::c_void,
  pub __pad2: *mut libc::c_void,
  pub __pad3: *mut libc::c_void,
  pub __pad4: *mut libc::c_void,
  pub __pad5: size_t,
  pub _mode: libc::c_int,
  pub _unused2: [libc::c_char; 20],
}

pub type _IO_lock_t = ();

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
  pub _next: *mut _IO_marker,
  pub _sbuf: *mut _IO_FILE,
  pub _pos: libc::c_int,
}

pub type FILE = _IO_FILE;

/*
 * bb_ask_y_confirmation implementation for busybox
 *
 * Copyright (C) 2003  Manuel Novoa III  <mjn3@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */

/* Read a line from fp.  If the first non-whitespace char is 'y' or 'Y',
 * return 1.  Otherwise return 0.
 */
#[no_mangle]
pub unsafe extern "C" fn bb_ask_y_confirmation_FILE(mut fp: *mut FILE) -> libc::c_int {
  let mut first: libc::c_char = 0i32 as libc::c_char;
  let mut c: libc::c_int = 0;
  fflush_all();
  loop {
    c = getc_unlocked(fp);
    if !(c != -1i32 && c != '\n' as i32) {
      break;
    }
    if first as libc::c_int == 0i32
      && ({
        let mut bb__isblank: libc::c_uchar = c as libc::c_uchar;
        (bb__isblank as libc::c_int == ' ' as i32 || bb__isblank as libc::c_int == '\t' as i32)
          as libc::c_int
      }) == 0
    {
      first = (c | 0x20i32) as libc::c_char
    }
  }
  return (first as libc::c_int == 'y' as i32) as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn bb_ask_y_confirmation() -> libc::c_int {
  return bb_ask_y_confirmation_FILE(stdin);
}

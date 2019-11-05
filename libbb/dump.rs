use libc;
extern "C" {
  #[no_mangle]
  fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static mut stdin: *mut FILE;
  #[no_mangle]
  fn freopen(
    __filename: *const libc::c_char,
    __modes: *const libc::c_char,
    __stream: *mut FILE,
  ) -> *mut FILE;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t, __stream: *mut FILE) -> size_t;
  #[no_mangle]
  fn fseeko(__stream: *mut FILE, __off: __off64_t, __whence: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn ferror_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn skip_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrndup(s: *const libc::c_char, n: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strcpy_and_process_escape_sequences(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn xfstat(fd: libc::c_int, buf: *mut stat, errmsg: *const libc::c_char);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
}
pub type __uint8_t = libc::c_uchar;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type smallint = libc::c_schar;
pub type size_t = libc::c_ulong;
pub type off_t = __off64_t;
use crate::librb::stat;
use crate::librb::timespec;

pub type _IO_lock_t = ();

use crate::librb::FILE;
/* no conversions */
pub type dump_vflag_t = libc::c_uint;
pub const WAIT: dump_vflag_t = 3;
pub const FIRST: dump_vflag_t = 2;
pub const DUP: dump_vflag_t = 1;
pub const ALL: dump_vflag_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PR {
  pub nextpr: *mut PR,
  pub flags: libc::c_uint,
  pub bcnt: libc::c_int,
  pub cchar: *mut libc::c_char,
  pub fmt: *mut libc::c_char,
  pub nospace: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FU {
  pub nextfu: *mut FU,
  pub nextpr: *mut PR,
  pub flags: libc::c_uint,
  pub reps: libc::c_int,
  pub bcnt: libc::c_int,
  pub fmt: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FS {
  pub nextfs: *mut FS,
  pub nextfu: *mut FU,
  pub bcnt: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dumper_t {
  pub dump_skip: off_t,
  pub dump_length: libc::c_int,
  pub dump_vflag: smallint,
  pub fshead: *mut FS,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct priv_dumper_t {
  pub pub_0: dumper_t,
  pub argv: *mut *mut libc::c_char,
  pub endfu: *mut FU,
  pub savaddress: off_t,
  pub eaddress: off_t,
  pub address: off_t,
  pub blocksize: libc::c_int,
  pub exitval: smallint,
  pub next__done: smallint,
  pub get__ateof: smallint,
  pub get__curp: *mut libc::c_uchar,
  pub get__savp: *mut libc::c_uchar,
}
/* vi: set sw=4 ts=4: */
/*
 * Support code for the hexdump and od applets,
 * based on code from util-linux v 2.11l
 *
 * Copyright (c) 1989
 * The Regents of the University of California.  All rights reserved.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 *
 * Original copyright notice is retained at the end of this file.
 */
static mut dot_flags_width_chars: [libc::c_char; 16] = [
  46, 35, 45, 43, 32, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 0,
];
static mut size_conv_str: [libc::c_char; 25] = [
  1, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 8, 99, 100, 105, 111, 117, 120, 88, 101, 69, 102, 103, 71, 0,
];
static mut int_convs: [libc::c_char; 7] = [100, 105, 111, 117, 120, 88, 0];
#[no_mangle]
pub unsafe extern "C" fn alloc_dumper() -> *mut dumper_t {
  let mut dumper: *mut priv_dumper_t =
    xzalloc(::std::mem::size_of::<priv_dumper_t>() as libc::c_ulong) as *mut priv_dumper_t;
  (*dumper).pub_0.dump_length = -1i32;
  (*dumper).pub_0.dump_vflag = FIRST as libc::c_int as smallint;
  (*dumper).get__ateof = 1i32 as smallint;
  return &mut (*dumper).pub_0;
}
#[inline(never)]
unsafe extern "C" fn bb_dump_size(mut fs: *mut FS) -> libc::c_int {
  let mut fu: *mut FU = 0 as *mut FU;
  let mut bcnt: libc::c_int = 0;
  let mut cur_size: libc::c_int = 0;
  let mut fmt: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut p: *const libc::c_char = 0 as *const libc::c_char;
  let mut prec: libc::c_int = 0;
  /* figure out the data block size needed for each format unit */
  cur_size = 0i32;
  fu = (*fs).nextfu;
  while !fu.is_null() {
    if (*fu).bcnt != 0 {
      cur_size += (*fu).bcnt * (*fu).reps
    } else {
      prec = 0i32;
      bcnt = prec;
      fmt = (*fu).fmt;
      while *fmt != 0 {
        if !(*fmt as libc::c_int != '%' as i32) {
          loop
          /*
           * skip any special chars -- save precision in
           * case it's a %s format.
           */
          {
            fmt = fmt.offset(1);
            if strchr(
              dot_flags_width_chars.as_ptr().offset(1),
              *fmt as libc::c_int,
            )
            .is_null()
            {
              break;
            }
          }
          if *fmt as libc::c_int == '.' as i32 && {
            fmt = fmt.offset(1);
            ((*fmt as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int) <= 9i32
          } {
            prec = atoi(fmt);
            loop {
              fmt = fmt.offset(1);
              if !((*fmt as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32) {
                break;
              }
            }
          }
          p = strchr(size_conv_str.as_ptr().offset(12), *fmt as libc::c_int);
          if p.is_null() {
            if *fmt as libc::c_int == 's' as i32 {
              bcnt += prec
            }
            if *fmt as libc::c_int == '_' as i32 {
              fmt = fmt.offset(1);
              if *fmt as libc::c_int == 'c' as i32
                || *fmt as libc::c_int == 'p' as i32
                || *fmt as libc::c_int == 'u' as i32
              {
                bcnt += 1i32
              }
            }
          } else {
            bcnt += *p.offset(-12i32 as isize) as libc::c_int
          }
        }
        fmt = fmt.offset(1)
      }
      cur_size += bcnt * (*fu).reps
    }
    fu = (*fu).nextfu
  }
  return cur_size;
}
#[inline(never)]
unsafe extern "C" fn rewrite(mut dumper: *mut priv_dumper_t, mut fs: *mut FS) {
  let mut e: *const libc::c_char = 0 as *const libc::c_char;
  let mut current_block: u64;
  let mut fu: *mut FU = 0 as *mut FU;
  fu = (*fs).nextfu;
  while !fu.is_null() {
    let mut pr: *mut PR = 0 as *mut PR;
    let mut p1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fmtp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nconv: libc::c_int = 0i32;
    /*
     * break each format unit into print units; each
     * conversion character gets its own.
     */
    fmtp = (*fu).fmt;
    while *fmtp != 0 {
      let mut len: libc::c_uint = 0;
      let mut prec: *const libc::c_char = 0 as *const libc::c_char;
      let mut byte_count_str: *const libc::c_char = 0 as *const libc::c_char;
      /* DBU:[dvae@cray.com] zalloc so that forward ptrs start out NULL */
      pr = xzalloc(::std::mem::size_of::<PR>() as libc::c_ulong) as *mut PR;
      if (*fu).nextpr.is_null() {
        (*fu).nextpr = pr
      }
      /* skip preceding text and up to the next % sign */
      p1 = strchr(fmtp, '%' as i32);
      if p1.is_null() {
        /* only text in the string */
        (*pr).fmt = fmtp;
        (*pr).flags = 0x400i32 as libc::c_uint;
        break;
      } else {
        /*
         * get precision for %s -- if have a byte count, don't
         * need it.
         */
        prec = 0 as *const libc::c_char;
        if (*fu).bcnt != 0 {
          loop
          /* skip to conversion character */
          {
            p1 = p1.offset(1);
            if strchr(dot_flags_width_chars.as_ptr(), *p1 as libc::c_int).is_null() {
              break;
            }
          }
        } else {
          loop
          /* skip any special chars, field width */
          {
            p1 = p1.offset(1); /* set end pointer */
            if strchr(dot_flags_width_chars.as_ptr().offset(1), *p1 as libc::c_int).is_null() {
              break;
            }
          }
          if *p1 as libc::c_int == '.' as i32 && {
            p1 = p1.offset(1);
            ((*p1 as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int) <= 9i32
          } {
            prec = p1;
            loop {
              p1 = p1.offset(1);
              if !((*p1 as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32) {
                break;
              }
            }
          }
        }
        p2 = p1.offset(1);
        /*
         * figure out the byte count for each conversion;
         * rewrite the format as necessary, set up blank-
         * padding for end of data.
         */
        if *p1 as libc::c_int == 'c' as i32 {
          (*pr).flags = 0x8i32 as libc::c_uint;
          current_block = 8136744424551816548;
        } else {
          if *p1 as libc::c_int == 'l' as i32 {
            /* %ld etc */
            e = 0 as *const libc::c_char; /* "diouxX"? */
            p2 = p2.offset(1);
            p1 = p1.offset(1);
            current_block = 525212256790890415;
          } else if !strchr(int_convs.as_ptr(), *p1 as libc::c_int).is_null() {
            current_block = 525212256790890415;
          } else if !strchr(
            b"eEfgG\x00" as *const u8 as *const libc::c_char,
            *p1 as libc::c_int,
          )
          .is_null()
          {
            /* floating point */
            (*pr).flags = 0x10i32 as libc::c_uint; /* move past a in "%_a" */
            byte_count_str = b"\x08\x04\x00" as *const u8 as *const libc::c_char;
            current_block = 2106018012992170135;
          } else if *p1 as libc::c_int == 's' as i32 {
            (*pr).flags = 0x80i32 as libc::c_uint;
            (*pr).bcnt = (*fu).bcnt;
            if (*fu).bcnt == 0i32 {
              if prec.is_null() {
                bb_simple_error_msg_and_die(
                  b"%%s needs precision or byte count\x00" as *const u8 as *const libc::c_char,
                );
              }
              (*pr).bcnt = atoi(prec)
            }
            current_block = 2754258178208450300;
          } else if *p1 as libc::c_int == '_' as i32 {
            p2 = p2.offset(1);
            match *p1.offset(1) as libc::c_int {
              65 => {
                current_block = 1333274983171898033;
                match current_block {
                  1333274983171898033 => {
                    /* %_A[dox]: print address and the end */
                    (*dumper).endfu = fu;
                    (*fu).flags |= 0x1i32 as libc::c_uint;
                    current_block = 11784409961931904551;
                  }
                  9733961473582153922 => {
                    /* %_c: chars, \ooo, \n \r \t etc */
                    (*pr).flags = 0x4i32 as libc::c_uint;
                    /* *p1 = 'c';   set in conv_c */
                    current_block = 8136744424551816548;
                  }
                  17536441121804553876 => {
                    /* %_p: chars, dots for nonprintable */
                    (*pr).flags = 0x40i32 as libc::c_uint;
                    *p1 = 'c' as i32 as libc::c_char;
                    current_block = 8136744424551816548;
                  }
                  7133350962238418135 => {
                    /* %_p: chars, 'nul', 'esc' etc for nonprintable */
                    (*pr).flags = 0x100i32 as libc::c_uint;
                    current_block = 8136744424551816548;
                  }
                  _ => {}
                }
                match current_block {
                  8136744424551816548 => {}
                  _ =>
                  /* FALLTHROUGH */
                  /* %_a[dox]: current address */
                  {
                    (*pr).flags = 0x1i32 as libc::c_uint; /* move past x in "%_ax" */
                    p2 = p2.offset(1);
                    if *p1.offset(2) as libc::c_int != 'd' as i32
                      && *p1.offset(2) as libc::c_int != 'o' as i32
                      && *p1.offset(2) as libc::c_int != 'x' as i32
                    {
                      current_block = 17724882069067801719;
                    } else {
                      *p1 = *p1.offset(2);
                      current_block = 2754258178208450300;
                    }
                  }
                }
              }
              97 => {
                current_block = 11784409961931904551;
                match current_block {
                  1333274983171898033 => {
                    (*dumper).endfu = fu;
                    (*fu).flags |= 0x1i32 as libc::c_uint;
                    current_block = 11784409961931904551;
                  }
                  9733961473582153922 => {
                    (*pr).flags = 0x4i32 as libc::c_uint;
                    current_block = 8136744424551816548;
                  }
                  17536441121804553876 => {
                    (*pr).flags = 0x40i32 as libc::c_uint;
                    *p1 = 'c' as i32 as libc::c_char;
                    current_block = 8136744424551816548;
                  }
                  7133350962238418135 => {
                    (*pr).flags = 0x100i32 as libc::c_uint;
                    current_block = 8136744424551816548;
                  }
                  _ => {}
                }
                match current_block {
                  8136744424551816548 => {}
                  _ => {
                    (*pr).flags = 0x1i32 as libc::c_uint;
                    p2 = p2.offset(1);
                    if *p1.offset(2) as libc::c_int != 'd' as i32
                      && *p1.offset(2) as libc::c_int != 'o' as i32
                      && *p1.offset(2) as libc::c_int != 'x' as i32
                    {
                      current_block = 17724882069067801719;
                    } else {
                      *p1 = *p1.offset(2);
                      current_block = 2754258178208450300;
                    }
                  }
                }
              }
              99 => {
                current_block = 9733961473582153922;
                match current_block {
                  1333274983171898033 => {
                    (*dumper).endfu = fu;
                    (*fu).flags |= 0x1i32 as libc::c_uint;
                    current_block = 11784409961931904551;
                  }
                  9733961473582153922 => {
                    (*pr).flags = 0x4i32 as libc::c_uint;
                    current_block = 8136744424551816548;
                  }
                  17536441121804553876 => {
                    (*pr).flags = 0x40i32 as libc::c_uint;
                    *p1 = 'c' as i32 as libc::c_char;
                    current_block = 8136744424551816548;
                  }
                  7133350962238418135 => {
                    (*pr).flags = 0x100i32 as libc::c_uint;
                    current_block = 8136744424551816548;
                  }
                  _ => {}
                }
                match current_block {
                  8136744424551816548 => {}
                  _ => {
                    (*pr).flags = 0x1i32 as libc::c_uint;
                    p2 = p2.offset(1);
                    if *p1.offset(2) as libc::c_int != 'd' as i32
                      && *p1.offset(2) as libc::c_int != 'o' as i32
                      && *p1.offset(2) as libc::c_int != 'x' as i32
                    {
                      current_block = 17724882069067801719;
                    } else {
                      *p1 = *p1.offset(2);
                      current_block = 2754258178208450300;
                    }
                  }
                }
              }
              112 => {
                current_block = 17536441121804553876;
                match current_block {
                  1333274983171898033 => {
                    (*dumper).endfu = fu;
                    (*fu).flags |= 0x1i32 as libc::c_uint;
                    current_block = 11784409961931904551;
                  }
                  9733961473582153922 => {
                    (*pr).flags = 0x4i32 as libc::c_uint;
                    current_block = 8136744424551816548;
                  }
                  17536441121804553876 => {
                    (*pr).flags = 0x40i32 as libc::c_uint;
                    *p1 = 'c' as i32 as libc::c_char;
                    current_block = 8136744424551816548;
                  }
                  7133350962238418135 => {
                    (*pr).flags = 0x100i32 as libc::c_uint;
                    current_block = 8136744424551816548;
                  }
                  _ => {}
                }
                match current_block {
                  8136744424551816548 => {}
                  _ => {
                    (*pr).flags = 0x1i32 as libc::c_uint;
                    p2 = p2.offset(1);
                    if *p1.offset(2) as libc::c_int != 'd' as i32
                      && *p1.offset(2) as libc::c_int != 'o' as i32
                      && *p1.offset(2) as libc::c_int != 'x' as i32
                    {
                      current_block = 17724882069067801719;
                    } else {
                      *p1 = *p1.offset(2);
                      current_block = 2754258178208450300;
                    }
                  }
                }
              }
              117 => {
                current_block = 7133350962238418135;
                match current_block {
                  1333274983171898033 => {
                    (*dumper).endfu = fu;
                    (*fu).flags |= 0x1i32 as libc::c_uint;
                    current_block = 11784409961931904551;
                  }
                  9733961473582153922 => {
                    (*pr).flags = 0x4i32 as libc::c_uint;
                    current_block = 8136744424551816548;
                  }
                  17536441121804553876 => {
                    (*pr).flags = 0x40i32 as libc::c_uint;
                    *p1 = 'c' as i32 as libc::c_char;
                    current_block = 8136744424551816548;
                  }
                  7133350962238418135 => {
                    (*pr).flags = 0x100i32 as libc::c_uint;
                    current_block = 8136744424551816548;
                  }
                  _ => {}
                }
                match current_block {
                  8136744424551816548 => {}
                  _ => {
                    (*pr).flags = 0x1i32 as libc::c_uint;
                    p2 = p2.offset(1);
                    if *p1.offset(2) as libc::c_int != 'd' as i32
                      && *p1.offset(2) as libc::c_int != 'o' as i32
                      && *p1.offset(2) as libc::c_int != 'x' as i32
                    {
                      current_block = 17724882069067801719;
                    } else {
                      *p1 = *p1.offset(2);
                      current_block = 2754258178208450300;
                    }
                  }
                }
              }
              _ => {
                current_block = 17724882069067801719;
              }
            }
          } else {
            current_block = 17724882069067801719;
          }
          match current_block {
            8136744424551816548 => {}
            2754258178208450300 => {}
            2106018012992170135 => {}
            _ => {
              match current_block {
                525212256790890415 =>
                /* %d etc */
                {
                  e = strchr(int_convs.as_ptr(), *p1 as libc::c_int);
                  if e.is_null() {
                    current_block = 17724882069067801719;
                  } else {
                    (*pr).flags = 0x20i32 as libc::c_uint;
                    if e > int_convs.as_ptr().offset(1) {
                      /* not d or i? */
                      (*pr).flags = 0x200i32 as libc::c_uint
                    }
                    byte_count_str = b"\x04\x02\x01\x00" as *const u8 as *const libc::c_char;
                    current_block = 2106018012992170135;
                  }
                }
                _ => {}
              }
              match current_block {
                2106018012992170135 => {}
                _ => {
                  bb_error_msg_and_die(
                    b"bad conversion character %%%s\x00" as *const u8 as *const libc::c_char,
                    p1,
                  );
                }
              }
            }
          }
        }
        match current_block {
          8136744424551816548 =>
          /* *p1 = 'c';   set in conv_u */
          {
            byte_count_str = b"\x01\x00" as *const u8 as *const libc::c_char;
            current_block = 2106018012992170135;
          }
          _ => {}
        }
        match current_block {
          2106018012992170135 => {
            if (*fu).bcnt != 0 {
              while !((*fu).bcnt == *byte_count_str as libc::c_int) {
                byte_count_str = byte_count_str.offset(1);
                if *byte_count_str as libc::c_int == 0i32 {
                  bb_error_msg_and_die(
                    b"bad byte count for conversion character %s\x00" as *const u8
                      as *const libc::c_char,
                    p1,
                  );
                }
              }
            }
            /* Unlike the original, output the remainder of the format string. */
            (*pr).bcnt = *byte_count_str as libc::c_int
          }
          _ => {}
        }
        /*
         * copy to PR format string, set conversion character
         * pointer, update original.
         */
        len =
          (p1.wrapping_offset_from(fmtp) as libc::c_long + 1i32 as libc::c_long) as libc::c_uint;
        (*pr).fmt = xstrndup(fmtp, len as libc::c_int);
        /* DBU:[dave@cray.com] w/o this, trailing fmt text, space is lost.
         * Skip subsequent text and up to the next % sign and tack the
         * additional text onto fmt: eg. if fmt is "%x is a HEX number",
         * we lose the " is a HEX number" part of fmt.
         */
        p3 = p2;
        while *p3 as libc::c_int != 0 && *p3 as libc::c_int != '%' as i32 {
          p3 = p3.offset(1)
        }
        if p3.wrapping_offset_from(p2) as libc::c_long != 0i32 as libc::c_long {
          let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
          d = xrealloc(
            (*pr).fmt as *mut libc::c_void,
            (len as libc::c_long
              + p3.wrapping_offset_from(p2) as libc::c_long
              + 1i32 as libc::c_long) as size_t,
          ) as *mut libc::c_char;
          (*pr).fmt = d;
          d = d.offset(len as isize);
          loop {
            let fresh0 = p2;
            p2 = p2.offset(1);
            let fresh1 = d;
            d = d.offset(1);
            *fresh1 = *fresh0;
            if !(p2 != p3) {
              break;
            }
          }
          *d = '\u{0}' as i32 as libc::c_char
          /* now p2 = p3 */
        } /* must be after realloc! */
        (*pr).cchar = (*pr).fmt.offset(len as isize).offset(-1);
        fmtp = p2;
        /* only one conversion character if byte count */
        if (*pr).flags & 0x1i32 as libc::c_uint == 0 && (*fu).bcnt != 0 && {
          let fresh2 = nconv;
          nconv = nconv + 1;
          (fresh2) != 0
        } {
          bb_simple_error_msg_and_die(
            b"byte count with multiple conversion characters\x00" as *const u8
              as *const libc::c_char,
          );
        }
      }
    }
    /*
     * if format unit byte count not specified, figure it out
     * so can adjust rep count later.
     */
    if (*fu).bcnt == 0i32 {
      pr = (*fu).nextpr;
      while !pr.is_null() {
        (*fu).bcnt += (*pr).bcnt;
        pr = (*pr).nextpr
      }
    }
    fu = (*fu).nextfu
  }
  /*
   * if the format string interprets any data at all, and it's
   * not the same as the blocksize, and its last format unit
   * interprets any data at all, and has no iteration count,
   * repeat it as necessary.
   *
   * if rep count is greater than 1, no trailing whitespace
   * gets output from the last iteration of the format unit.
   */
  fu = (*fs).nextfu;
  while !fu.is_null() {
    if (*fu).nextfu.is_null()
      && (*fs).bcnt < (*dumper).blocksize
      && (*fu).flags & 0x2i32 as libc::c_uint == 0
      && (*fu).bcnt != 0
    {
      (*fu).reps += ((*dumper).blocksize - (*fs).bcnt) / (*fu).bcnt
    }
    if (*fu).reps > 1i32 && !(*fu).nextpr.is_null() {
      let mut pr_0: *mut PR = 0 as *mut PR;
      let mut p1_0: *mut libc::c_char = 0 as *mut libc::c_char;
      let mut p2_0: *mut libc::c_char = 0 as *mut libc::c_char;
      pr_0 = (*fu).nextpr;
      while !(*pr_0).nextpr.is_null() {
        pr_0 = (*pr_0).nextpr
      }
      p2_0 = 0 as *mut libc::c_char;
      p1_0 = (*pr_0).fmt;
      while *p1_0 != 0 {
        p2_0 = if ({
          let mut bb__isspace: libc::c_uchar = (*p1_0 as libc::c_int - 9i32) as libc::c_uchar;
          (bb__isspace as libc::c_int == ' ' as i32 - 9i32
            || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
        }) != 0
        {
          p1_0
        } else {
          0 as *mut libc::c_char
        };
        p1_0 = p1_0.offset(1)
      }
      if !p2_0.is_null() {
        (*pr_0).nospace = p2_0
      }
    }
    fu = (*fu).nextfu
  }
}
unsafe extern "C" fn do_skip(mut dumper: *mut priv_dumper_t, mut fname: *const libc::c_char) {
  let mut sbuf: stat = stat {
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
  xfstat(0i32, &mut sbuf, fname);
  if sbuf.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint
    && (*dumper).pub_0.dump_skip >= sbuf.st_size
  {
    /* If st_size is valid and pub.dump_skip >= st_size */
    (*dumper).pub_0.dump_skip -= sbuf.st_size;
    (*dumper).address += sbuf.st_size;
    return;
  }
  if fseeko(stdin, (*dumper).pub_0.dump_skip, 0i32) != 0 {
    bb_simple_perror_msg_and_die(fname);
  }
  (*dumper).address += (*dumper).pub_0.dump_skip;
  (*dumper).savaddress = (*dumper).address;
  (*dumper).pub_0.dump_skip = 0i32 as off_t;
}
#[inline(never)]
unsafe extern "C" fn next(mut dumper: *mut priv_dumper_t) -> libc::c_int {
  loop {
    let mut fname: *const libc::c_char = *(*dumper).argv;
    if !fname.is_null() {
      (*dumper).argv = (*dumper).argv.offset(1);
      if *fname.offset(0) as libc::c_int != '-' as i32 || *fname.offset(1) as libc::c_int != 0 {
        if freopen(fname, b"r\x00" as *const u8 as *const libc::c_char, stdin).is_null() {
          bb_simple_perror_msg(fname);
          (*dumper).exitval = 1i32 as smallint;
          continue;
        }
      }
    } else if (*dumper).next__done != 0 {
      return 0i32;
    }
    (*dumper).next__done = 1i32 as smallint;
    if (*dumper).pub_0.dump_skip != 0 {
      do_skip(
        dumper,
        if !fname.is_null() {
          fname
        } else {
          b"stdin\x00" as *const u8 as *const libc::c_char
        },
      );
    }
    if (*dumper).pub_0.dump_skip == 0i32 as libc::c_long {
      return 1i32;
    }
  }
  /* no next file */
  /* NOTREACHED */
}
unsafe extern "C" fn get(mut dumper: *mut priv_dumper_t) -> *mut libc::c_uchar {
  let mut n: libc::c_int = 0; /*DBU:[dave@cray.com] initialize,initialize..*/
  let mut need: libc::c_int = 0;
  let mut nread: libc::c_int = 0;
  let mut blocksize: libc::c_int = (*dumper).blocksize;
  if (*dumper).get__curp.is_null() {
    (*dumper).address = 0i32 as off_t;
    (*dumper).get__curp = xmalloc(blocksize as size_t) as *mut libc::c_uchar;
    (*dumper).get__savp = xzalloc(blocksize as size_t) as *mut libc::c_uchar
  /* need to be initialized */
  } else {
    let mut tmp: *mut libc::c_uchar = (*dumper).get__curp;
    (*dumper).get__curp = (*dumper).get__savp;
    (*dumper).get__savp = tmp;
    (*dumper).savaddress += blocksize as libc::c_long;
    (*dumper).address = (*dumper).savaddress
  }
  need = blocksize;
  nread = 0i32;
  loop {
    /*
     * if read the right number of bytes, or at EOF for one file,
     * and no other files are available, zero-pad the rest of the
     * block and set the end flag.
     */
    if (*dumper).pub_0.dump_length == 0
      || (*dumper).get__ateof as libc::c_int != 0 && next(dumper) == 0
    {
      if need == blocksize {
        return 0 as *mut libc::c_uchar;
      }
      if (*dumper).pub_0.dump_vflag as libc::c_int != ALL as libc::c_int
        && (*dumper).pub_0.dump_vflag as libc::c_int != FIRST as libc::c_int
        && memcmp(
          (*dumper).get__curp as *const libc::c_void,
          (*dumper).get__savp as *const libc::c_void,
          nread as libc::c_ulong,
        ) == 0i32
      {
        /* same data? */
        if (*dumper).pub_0.dump_vflag as libc::c_int != DUP as libc::c_int {
          puts(b"*\x00" as *const u8 as *const libc::c_char);
        }
        return 0 as *mut libc::c_uchar;
      }
      memset(
        (*dumper).get__curp.offset(nread as isize) as *mut libc::c_void,
        0i32,
        need as libc::c_ulong,
      );
      (*dumper).eaddress = (*dumper).address + nread as libc::c_long;
      return (*dumper).get__curp;
    }
    n = fread(
      (*dumper).get__curp.offset(nread as isize) as *mut libc::c_void,
      ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
      if (*dumper).pub_0.dump_length == -1i32 {
        need
      } else if (*dumper).pub_0.dump_length < need {
        (*dumper).pub_0.dump_length
      } else {
        need
      } as size_t,
      stdin,
    ) as libc::c_int;
    if n == 0i32 {
      if ferror_unlocked(stdin) != 0 {
        bb_simple_perror_msg(*(*dumper).argv.offset(-1i32 as isize));
      }
      (*dumper).get__ateof = 1i32 as smallint
    } else {
      (*dumper).get__ateof = 0i32 as smallint;
      if (*dumper).pub_0.dump_length != -1i32 {
        (*dumper).pub_0.dump_length -= n
      }
      need -= n;
      if need == 0i32 {
        if (*dumper).pub_0.dump_vflag as libc::c_int == ALL as libc::c_int
          || (*dumper).pub_0.dump_vflag as libc::c_int == FIRST as libc::c_int
          || memcmp(
            (*dumper).get__curp as *const libc::c_void,
            (*dumper).get__savp as *const libc::c_void,
            blocksize as libc::c_ulong,
          ) != 0i32
        {
          /* not same data? */
          if (*dumper).pub_0.dump_vflag as libc::c_int == DUP as libc::c_int
            || (*dumper).pub_0.dump_vflag as libc::c_int == FIRST as libc::c_int
          {
            (*dumper).pub_0.dump_vflag = WAIT as libc::c_int as smallint
          }
          return (*dumper).get__curp;
        }
        if (*dumper).pub_0.dump_vflag as libc::c_int == WAIT as libc::c_int {
          puts(b"*\x00" as *const u8 as *const libc::c_char);
        }
        (*dumper).pub_0.dump_vflag = DUP as libc::c_int as smallint;
        (*dumper).savaddress += blocksize as libc::c_long;
        (*dumper).address = (*dumper).savaddress;
        need = blocksize;
        nread = 0i32
      } else {
        nread += n
      }
    }
  }
}
unsafe extern "C" fn bpad(mut pr: *mut PR) {
  let mut p1: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
  /*
   * remove all conversion flags; '-' is the only one valid
   * with %s, and it's not useful here.
   */
  (*pr).flags = 0x2i32 as libc::c_uint;
  *(*pr).cchar = 's' as i32 as libc::c_char;
  p1 = (*pr).fmt;
  while *p1 as libc::c_int != '%' as i32 {
    p1 = p1.offset(1)
  }
  p1 = p1.offset(1);
  p2 = p1;
  while *p1 as libc::c_int != 0
    && !strchr(
      b" -0+#\x00" as *const u8 as *const libc::c_char,
      *p1 as libc::c_int,
    )
    .is_null()
  {
    if !(*pr).nospace.is_null() {
      (*pr).nospace = (*pr).nospace.offset(-1)
    }
    p1 = p1.offset(1)
  }
  loop {
    let fresh3 = p1;
    p1 = p1.offset(1);
    let fresh4 = p2;
    p2 = p2.offset(1);
    *fresh4 = *fresh3;
    if !(*fresh4 as libc::c_int != 0i32) {
      break;
    }
  }
}
static mut conv_str: [libc::c_char; 33] = [
  0, 92, 48, 0, 7, 92, 97, 0, 8, 92, 98, 0, 12, 92, 102, 0, 10, 92, 110, 0, 13, 92, 114, 0, 9, 92,
  116, 0, 11, 92, 118, 0, 0,
];
unsafe extern "C" fn conv_c(mut pr: *mut PR, mut p: *mut libc::c_uchar) {
  let mut buf: [libc::c_char; 4] = [0; 4];
  let mut current_block: u64;
  let mut str: *const libc::c_char = conv_str.as_ptr();
  loop {
    if *p as libc::c_int == *str as libc::c_int {
      str = str.offset(1);
      current_block = 14281039486041038171;
      break;
    /* map e.g. '\n' to "\\n" */
    } else {
      str = str.offset(4);
      if !(*str != 0) {
        current_block = 15427931788582360902;
        break;
      }
    }
  }
  match current_block {
    15427931788582360902 => {
      if (*p as libc::c_int - 0x20i32) as libc::c_uint <= (0x7ei32 - 0x20i32) as libc::c_uint {
        *(*pr).cchar = 'c' as i32 as libc::c_char;
        printf((*pr).fmt, *p as libc::c_int);
        current_block = 3512920355445576850;
      } else {
        buf = [0; 4];
        /* gcc-8.0.1 needs lots of casts to shut up */
        sprintf(
          buf.as_mut_ptr(),
          b"%03o\x00" as *const u8 as *const libc::c_char,
          *p as libc::c_uint,
        );
        str = buf.as_mut_ptr();
        current_block = 14281039486041038171;
      }
    }
    _ => {}
  }
  match current_block {
    14281039486041038171 => {
      *(*pr).cchar = 's' as i32 as libc::c_char;
      printf((*pr).fmt, str);
    }
    _ => {}
  };
}
unsafe extern "C" fn conv_u(mut pr: *mut PR, mut p: *mut libc::c_uchar) {
  static mut list: [libc::c_char; 127] = [
    110, 117, 108, 0, 115, 111, 104, 0, 115, 116, 120, 0, 101, 116, 120, 0, 101, 111, 116, 0, 101,
    110, 113, 0, 97, 99, 107, 0, 98, 101, 108, 0, 98, 115, 0, 95, 104, 116, 0, 95, 108, 102, 0, 95,
    118, 116, 0, 95, 102, 102, 0, 95, 99, 114, 0, 95, 115, 111, 0, 95, 115, 105, 0, 95, 100, 108,
    101, 0, 100, 99, 108, 0, 100, 99, 50, 0, 100, 99, 51, 0, 100, 99, 52, 0, 110, 97, 107, 0, 115,
    121, 110, 0, 101, 116, 98, 0, 99, 97, 110, 0, 101, 109, 0, 95, 115, 117, 98, 0, 101, 115, 99,
    0, 102, 115, 0, 95, 103, 115, 0, 95, 114, 115, 0, 95, 117, 115, 0,
  ];
  /* od used nl, not lf */
  if *p as libc::c_int <= 0x1fi32 {
    *(*pr).cchar = 's' as i32 as libc::c_char;
    printf(
      (*pr).fmt,
      list.as_ptr().offset((4i32 * *p as libc::c_int) as isize),
    );
  } else if *p as libc::c_int == 0x7fi32 {
    *(*pr).cchar = 's' as i32 as libc::c_char;
    printf((*pr).fmt, b"del\x00" as *const u8 as *const libc::c_char);
  } else if (*p as libc::c_int) < 0x7fi32 {
    /* isprint() */
    *(*pr).cchar = 'c' as i32 as libc::c_char;
    printf((*pr).fmt, *p as libc::c_int);
  } else {
    *(*pr).cchar = 'x' as i32 as libc::c_char;
    printf((*pr).fmt, *p as libc::c_int);
  };
}
unsafe extern "C" fn display(mut dumper: *mut priv_dumper_t) {
  let mut fs: *mut FS = 0 as *mut FS;
  let mut fu: *mut FU = 0 as *mut FU;
  let mut pr: *mut PR = 0 as *mut PR;
  let mut cnt: libc::c_int = 0;
  let mut bp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
  let mut savebp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
  let mut saveaddress: off_t = 0;
  let mut savech: libc::c_uchar = '\u{0}' as i32 as libc::c_uchar;
  loop {
    bp = get(dumper);
    if bp.is_null() {
      break;
    }
    fs = (*dumper).pub_0.fshead;
    savebp = bp;
    saveaddress = (*dumper).address;
    while !fs.is_null() {
      fu = (*fs).nextfu;
      while !fu.is_null() {
        if (*fu).flags & 0x1i32 as libc::c_uint != 0 {
          break;
        }
        cnt = (*fu).reps;
        while cnt != 0 {
          pr = (*fu).nextpr;
          while !pr.is_null() {
            if (*dumper).eaddress != 0
              && (*dumper).address >= (*dumper).eaddress
              && (*pr).flags & (0x400i32 | 0x2i32) as libc::c_uint == 0
            {
              bpad(pr);
            }
            if cnt == 1i32 && !(*pr).nospace.is_null() {
              savech = *(*pr).nospace as libc::c_uchar;
              *(*pr).nospace = '\u{0}' as i32 as libc::c_char
            }
            /*                      PRINT; */
            match (*pr).flags {
              1 => {
                printf((*pr).fmt, (*dumper).address as libc::c_uint);
              }
              2 => {
                printf((*pr).fmt, b"\x00" as *const u8 as *const libc::c_char);
              }
              4 => {
                conv_c(pr, bp);
              }
              8 => {
                printf((*pr).fmt, *bp as libc::c_int);
              }
              16 => {
                let mut dval: libc::c_double = 0.;
                let mut fval: libc::c_float = 0.;
                match (*pr).bcnt {
                  4 => {
                    memcpy(
                      &mut fval as *mut libc::c_float as *mut libc::c_void,
                      bp as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
                    );
                    printf((*pr).fmt, fval as libc::c_double);
                  }
                  8 => {
                    memcpy(
                      &mut dval as *mut libc::c_double as *mut libc::c_void,
                      bp as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    );
                    printf((*pr).fmt, dval);
                  }
                  _ => {}
                }
              }
              32 => {
                let mut ival: libc::c_int = 0;
                let mut sval: libc::c_short = 0;
                match (*pr).bcnt {
                  1 => {
                    printf((*pr).fmt, *bp as libc::c_int);
                  }
                  2 => {
                    memcpy(
                      &mut sval as *mut libc::c_short as *mut libc::c_void,
                      bp as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    );
                    printf((*pr).fmt, sval as libc::c_int);
                  }
                  4 => {
                    memcpy(
                      &mut ival as *mut libc::c_int as *mut libc::c_void,
                      bp as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    );
                    printf((*pr).fmt, ival);
                  }
                  _ => {}
                }
              }
              64 => {
                printf(
                  (*pr).fmt,
                  if (*bp as libc::c_int - 0x20i32) as libc::c_uint
                    <= (0x7ei32 - 0x20i32) as libc::c_uint
                  {
                    *bp as libc::c_int
                  } else {
                    '.' as i32
                  },
                );
              }
              128 => {
                printf((*pr).fmt, bp as *mut libc::c_char);
              }
              1024 => {
                printf((*pr).fmt);
              }
              256 => {
                conv_u(pr, bp);
              }
              512 => {
                let mut ival_0: libc::c_uint = 0;
                let mut sval_0: libc::c_ushort = 0;
                match (*pr).bcnt {
                  1 => {
                    printf((*pr).fmt, *bp as libc::c_uint);
                  }
                  2 => {
                    memcpy(
                      &mut sval_0 as *mut libc::c_ushort as *mut libc::c_void,
                      bp as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong,
                    );
                    printf((*pr).fmt, sval_0 as libc::c_uint);
                  }
                  4 => {
                    memcpy(
                      &mut ival_0 as *mut libc::c_uint as *mut libc::c_void,
                      bp as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                    );
                    printf((*pr).fmt, ival_0);
                  }
                  _ => {}
                }
              }
              _ => {}
            }
            if cnt == 1i32 && !(*pr).nospace.is_null() {
              *(*pr).nospace = savech as libc::c_char
            }
            (*dumper).address += (*pr).bcnt as libc::c_long;
            bp = bp.offset((*pr).bcnt as isize);
            pr = (*pr).nextpr
          }
          cnt -= 1
        }
        fu = (*fu).nextfu
      }
      fs = (*fs).nextfs;
      bp = savebp;
      (*dumper).address = saveaddress
    }
  }
  if !(*dumper).endfu.is_null() {
    /*
     * if eaddress not set, error or file size was multiple
     * of blocksize, and no partial block ever found.
     */
    if (*dumper).eaddress == 0 {
      if (*dumper).address == 0 {
        return;
      }
      (*dumper).eaddress = (*dumper).address
    }
    pr = (*(*dumper).endfu).nextpr;
    while !pr.is_null() {
      match (*pr).flags {
        1 => {
          printf((*pr).fmt, (*dumper).eaddress as libc::c_uint);
        }
        1024 => {
          printf((*pr).fmt);
        }
        _ => {}
      }
      pr = (*pr).nextpr
    }
  };
}
#[no_mangle]
pub unsafe extern "C" fn bb_dump_dump(
  mut pub_dumper: *mut dumper_t,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut tfs: *mut FS = 0 as *mut FS;
  let mut blocksize: libc::c_int = 0;
  /* figure out the data block size */
  blocksize = 0i32;
  tfs = (*(pub_dumper as *mut priv_dumper_t)).pub_0.fshead;
  while !tfs.is_null() {
    (*tfs).bcnt = bb_dump_size(tfs);
    if blocksize < (*tfs).bcnt {
      blocksize = (*tfs).bcnt
    }
    tfs = (*tfs).nextfs
  }
  (*(pub_dumper as *mut priv_dumper_t)).blocksize = blocksize;
  /* rewrite the rules, do syntax checking */
  tfs = (*(pub_dumper as *mut priv_dumper_t)).pub_0.fshead;
  while !tfs.is_null() {
    rewrite(pub_dumper as *mut priv_dumper_t, tfs);
    tfs = (*tfs).nextfs
  }
  let ref mut fresh5 = (*(pub_dumper as *mut priv_dumper_t)).argv;
  *fresh5 = argv;
  display(pub_dumper as *mut priv_dumper_t);
  return (*(pub_dumper as *mut priv_dumper_t)).exitval as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bb_dump_add(mut pub_dumper: *mut dumper_t, mut fmt: *const libc::c_char) {
  let mut p: *const libc::c_char = 0 as *const libc::c_char;
  let mut tfs: *mut FS = 0 as *mut FS;
  let mut tfu: *mut FU = 0 as *mut FU;
  let mut nextfupp: *mut *mut FU = 0 as *mut *mut FU;
  let mut savep: *const libc::c_char = 0 as *const libc::c_char;
  /* start new linked list of format units */
  tfs = xzalloc(::std::mem::size_of::<FS>() as libc::c_ulong) as *mut FS; /*DBU:[dave@cray.com] start out NULL */
  if (*(pub_dumper as *mut priv_dumper_t)).pub_0.fshead.is_null() {
    let ref mut fresh6 = (*(pub_dumper as *mut priv_dumper_t)).pub_0.fshead;
    *fresh6 = tfs
  } else {
    let mut fslast: *mut FS = (*(pub_dumper as *mut priv_dumper_t)).pub_0.fshead;
    while !(*fslast).nextfs.is_null() {
      fslast = (*fslast).nextfs
    }
    (*fslast).nextfs = tfs
  }
  nextfupp = &mut (*tfs).nextfu;
  /* take the format string and break it up into format units */
  p = fmt;
  loop {
    p = skip_whitespace(p);
    if *p as libc::c_int == '\u{0}' as i32 {
      break;
    }
    /* allocate a new format unit and link it in */
    /* NOSTRICT */
    /* DBU:[dave@cray.com] zalloc so that forward pointers start out NULL */
    tfu = xzalloc(::std::mem::size_of::<FU>() as libc::c_ulong) as *mut FU;
    *nextfupp = tfu;
    nextfupp = &mut (*tfu).nextfu;
    (*tfu).reps = 1i32;
    /* if leading digit, repetition count */
    if (*p as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
      savep = p;
      while (*p as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
        p = p.offset(1)
      }
      if ({
        let mut bb__isspace: libc::c_uchar = (*p as libc::c_int - 9i32) as libc::c_uchar;
        (bb__isspace as libc::c_int == ' ' as i32 - 9i32
          || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
      }) == 0
        && *p as libc::c_int != '/' as i32
      {
        bb_error_msg_and_die(
          b"bad format {%s}\x00" as *const u8 as *const libc::c_char,
          fmt,
        );
      }
      /* may overwrite either white space or slash */
      (*tfu).reps = atoi(savep);
      (*tfu).flags = 0x2i32 as libc::c_uint;
      /* skip trailing white space */
      p = p.offset(1);
      p = skip_whitespace(p)
    }
    /* skip slash and trailing white space */
    if *p as libc::c_int == '/' as i32 {
      p = skip_whitespace(p.offset(1))
    }
    /* byte count */
    if (*p as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
      // TODO: use bb_strtou
      savep = p;
      loop {
        p = p.offset(1);
        if !((*p as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32) {
          break;
        }
      }
      if ({
        let mut bb__isspace: libc::c_uchar = (*p as libc::c_int - 9i32) as libc::c_uchar;
        (bb__isspace as libc::c_int == ' ' as i32 - 9i32
          || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
      }) == 0
      {
        bb_error_msg_and_die(
          b"bad format {%s}\x00" as *const u8 as *const libc::c_char,
          fmt,
        );
      }
      // Above check prohibits formats such as '/1"%02x"' - it requires space after 1.
      // Other than this, formats can be pretty much jammed together:
      // "%07_ax:"8/2 "%04x|""\n"
      // but this space is required. The check *can* be removed, but
      // keeping it to stay compat with util-linux hexdump.
      (*tfu).bcnt = atoi(savep);
      /* skip trailing white space */
      p = skip_whitespace(p.offset(1))
    }
    /* format */
    if *p as libc::c_int != '\"' as i32 {
      bb_error_msg_and_die(
        b"bad format {%s}\x00" as *const u8 as *const libc::c_char,
        fmt,
      );
    }
    p = p.offset(1);
    savep = p;
    while *p as libc::c_int != '\"' as i32 {
      let fresh7 = p;
      p = p.offset(1);
      if *fresh7 as libc::c_int == '\u{0}' as i32 {
        bb_error_msg_and_die(
          b"bad format {%s}\x00" as *const u8 as *const libc::c_char,
          fmt,
        );
      }
    }
    (*tfu).fmt = xstrndup(
      savep,
      p.wrapping_offset_from(savep) as libc::c_long as libc::c_int,
    );
    /* alphabetic escape sequences have to be done in place */
    strcpy_and_process_escape_sequences((*tfu).fmt, (*tfu).fmt);
    /* unknown mappings are not changed: "\z" -> '\\' 'z' */
    /* trailing backslash, if any, is preserved */
    p = p.offset(1)
  }
}
/*
 * Copyright (c) 1989 The Regents of the University of California.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ''AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

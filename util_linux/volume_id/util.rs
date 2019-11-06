use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn lseek(__fd: libc::c_int, __offset: __off64_t, __whence: libc::c_int) -> __off64_t;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  // NB: will return short read on error, not -1,
  // if some data was read before error occurred
  #[no_mangle]
  fn full_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn strnlen(__string: *const libc::c_char, __maxlen: size_t) -> size_t;
}

use crate::librb::__off64_t;

use crate::librb::size_t;
use crate::librb::ssize_t;


#[derive(Copy, Clone)]
#[repr(C)]
pub struct volume_id {
  pub fd: libc::c_int,
  pub error: libc::c_int,
  pub sbbuf_len: size_t,
  pub seekbuf_len: size_t,
  pub sbbuf: *mut u8,
  pub seekbuf: *mut u8,
  pub seekbuf_off: u64,
  pub label: [libc::c_char; 65],
  pub uuid: [libc::c_char; 37],
  pub type_0: *const libc::c_char,
}

pub type uuid_format = libc::c_uint;
pub const UUID_DCE_STRING: uuid_format = 3;
// pub const UUID_DCE: uuid_format = 2;
// pub const UUID_NTFS: uuid_format = 1;
// pub const UUID_DOS: uuid_format = 0;

pub type endian = libc::c_uint;
// pub const BE: endian = 1;
pub const LE: endian = 0;

/*
 * volume_id - reads filesystem label and uuid
 *
 * Copyright (C) 2005 Kay Sievers <kay.sievers@vrfy.org>
 *
 *	This library is free software; you can redistribute it and/or
 *	modify it under the terms of the GNU Lesser General Public
 *	License as published by the Free Software Foundation; either
 *	version 2.1 of the License, or (at your option) any later version.
 *
 *	This library is distributed in the hope that it will be useful,
 *	but WITHOUT ANY WARRANTY; without even the implied warranty of
 *	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 *	Lesser General Public License for more details.
 *
 *	You should have received a copy of the GNU Lesser General Public
 *	License along with this library; if not, write to the Free Software
 *	Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA
 */
#[no_mangle]
pub unsafe extern "C" fn volume_id_set_unicode16(
  mut str: *mut libc::c_char,
  mut len: size_t,
  mut buf: *const u8,
  mut endianess: endian,
  mut count: size_t,
) {
  let mut i: libc::c_uint = 0;
  let mut j: libc::c_uint = 0;
  let mut c: libc::c_uint = 0;
  j = 0i32 as libc::c_uint;
  i = 0i32 as libc::c_uint;
  while i.wrapping_add(2i32 as libc::c_uint) as libc::c_ulong <= count {
    if endianess as libc::c_uint == LE as libc::c_int as libc::c_uint {
      c = ((*buf.offset(i.wrapping_add(1i32 as libc::c_uint) as isize) as libc::c_int) << 8i32
        | *buf.offset(i as isize) as libc::c_int) as libc::c_uint
    } else {
      c = ((*buf.offset(i as isize) as libc::c_int) << 8i32
        | *buf.offset(i.wrapping_add(1i32 as libc::c_uint) as isize) as libc::c_int)
        as libc::c_uint
    }
    if c == 0i32 as libc::c_uint {
      break;
    }
    if j.wrapping_add(1i32 as libc::c_uint) as libc::c_ulong >= len {
      break;
    }
    if !(c < 0x80i32 as libc::c_uint) {
      let mut topbits: u8 = 0xc0i32 as u8;
      if j.wrapping_add(2i32 as libc::c_uint) as libc::c_ulong >= len {
        break;
      }
      if !(c < 0x800i32 as libc::c_uint) {
        if j.wrapping_add(3i32 as libc::c_uint) as libc::c_ulong >= len {
          break;
        }
        /* 1110yyyy 10yyyyxx 10xxxxxx */
        let fresh0 = j;
        j = j.wrapping_add(1);
        *str.offset(fresh0 as isize) =
          (0xe0i32 as libc::c_uint | c >> 12i32) as u8 as libc::c_char;
        topbits = 0x80i32 as u8
      }
      /* 110yyyxx 10xxxxxx */
      let fresh1 = j;
      j = j.wrapping_add(1);
      *str.offset(fresh1 as isize) =
        (topbits as libc::c_uint | c >> 6i32 & 0x3fi32 as libc::c_uint) as u8 as libc::c_char;
      c = 0x80i32 as libc::c_uint | c & 0x3fi32 as libc::c_uint
    }
    /* 0xxxxxxx */
    let fresh2 = j;
    j = j.wrapping_add(1);
    *str.offset(fresh2 as isize) = c as u8 as libc::c_char;
    i = i.wrapping_add(2i32 as libc::c_uint)
  }
  *str.offset(j as isize) = '\u{0}' as i32 as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn volume_id_set_label_string(
  mut id: *mut volume_id,
  mut buf: *const u8,
  mut count: size_t,
) {
  let mut i: libc::c_uint = 0;
  memcpy(
    (*id).label.as_mut_ptr() as *mut libc::c_void,
    buf as *const libc::c_void,
    count,
  );
  /* remove trailing whitespace */
  i = strnlen((*id).label.as_mut_ptr(), count) as libc::c_uint;
  loop {
    let fresh3 = i;
    i = i.wrapping_sub(1);
    if !(fresh3 != 0) {
      break;
    }
    if ({
      let mut bb__isspace: libc::c_uchar =
        ((*id).label[i as usize] as libc::c_int - 9i32) as libc::c_uchar;
      (bb__isspace as libc::c_int == ' ' as i32 - 9i32
        || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
    }) == 0
    {
      break;
    }
  }
  (*id).label[i.wrapping_add(1i32 as libc::c_uint) as usize] = '\u{0}' as i32 as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn volume_id_set_label_unicode16(
  mut id: *mut volume_id,
  mut buf: *const u8,
  mut endianess: endian,
  mut count: size_t,
) {
  volume_id_set_unicode16(
    (*id).label.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 65]>() as libc::c_ulong,
    buf,
    endianess,
    count,
  );
}
#[no_mangle]
pub unsafe extern "C" fn volume_id_set_uuid(
  mut id: *mut volume_id,
  mut buf: *const u8,
  mut format: uuid_format,
) {
  let mut current_block: u64;
  let mut i: libc::c_uint = 0;
  let mut count: libc::c_uint =
    if format as libc::c_uint == UUID_DCE_STRING as libc::c_int as libc::c_uint {
      36i32
    } else {
      (4i32) << format as libc::c_uint
    } as libc::c_uint;
  //	memcpy(id->uuid_raw, buf, count);
  //	id->uuid_raw_len = count;
  /* if set, create string in the same format, the native platform uses */
  i = 0i32 as libc::c_uint; /* all bytes are zero, leave it empty ("") */
  loop {
    if !(i < count) {
      current_block = 17179679302217393232;
      break;
    }
    if *buf.offset(i as isize) as libc::c_int != 0i32 {
      current_block = 11319177375723085162;
      break;
    }
    i = i.wrapping_add(1)
  }
  match current_block {
    17179679302217393232 => return,
    _ => {
      match format as libc::c_uint {
        0 => {
          sprintf(
            (*id).uuid.as_mut_ptr(),
            b"%02X%02X-%02X%02X\x00" as *const u8 as *const libc::c_char,
            *buf.offset(3) as libc::c_int,
            *buf.offset(2) as libc::c_int,
            *buf.offset(1) as libc::c_int,
            *buf.offset(0) as libc::c_int,
          );
        }
        1 => {
          sprintf(
            (*id).uuid.as_mut_ptr(),
            b"%02X%02X%02X%02X%02X%02X%02X%02X\x00" as *const u8 as *const libc::c_char,
            *buf.offset(7) as libc::c_int,
            *buf.offset(6) as libc::c_int,
            *buf.offset(5) as libc::c_int,
            *buf.offset(4) as libc::c_int,
            *buf.offset(3) as libc::c_int,
            *buf.offset(2) as libc::c_int,
            *buf.offset(1) as libc::c_int,
            *buf.offset(0) as libc::c_int,
          );
        }
        2 => {
          sprintf(
            (*id).uuid.as_mut_ptr(),
            b"%02x%02x%02x%02x-%02x%02x-%02x%02x-%02x%02x-%02x%02x%02x%02x%02x%02x\x00" as *const u8
              as *const libc::c_char,
            *buf.offset(0) as libc::c_int,
            *buf.offset(1) as libc::c_int,
            *buf.offset(2) as libc::c_int,
            *buf.offset(3) as libc::c_int,
            *buf.offset(4) as libc::c_int,
            *buf.offset(5) as libc::c_int,
            *buf.offset(6) as libc::c_int,
            *buf.offset(7) as libc::c_int,
            *buf.offset(8) as libc::c_int,
            *buf.offset(9) as libc::c_int,
            *buf.offset(10) as libc::c_int,
            *buf.offset(11) as libc::c_int,
            *buf.offset(12) as libc::c_int,
            *buf.offset(13) as libc::c_int,
            *buf.offset(14) as libc::c_int,
            *buf.offset(15) as libc::c_int,
          );
        }
        3 => {
          memcpy(
            (*id).uuid.as_mut_ptr() as *mut libc::c_void,
            buf as *const libc::c_void,
            count as libc::c_ulong,
          );
          (*id).uuid[count as usize] = '\u{0}' as i32 as libc::c_char
        }
        _ => {}
      }
      return;
    }
  };
}
/* Do not use xlseek here. With it, single corrupted filesystem
 * may result in attempt to seek past device -> exit.
 * It's better to ignore such fs and continue.  */
#[no_mangle]
pub unsafe extern "C" fn volume_id_get_buffer(
  mut id: *mut volume_id,
  mut off: u64,
  mut len: size_t,
) -> *mut libc::c_void {
  let mut current_block: u64;
  let mut dst: *mut u8 = 0 as *mut u8;
  let mut small_off: libc::c_uint = 0;
  let mut read_len: ssize_t = 0;
  /* check if requested area fits in superblock buffer */
  if off.wrapping_add(len) <= 0x11000i32 as libc::c_ulong {
    /* && off <= SB_BUFFER_SIZE - want this paranoid overflow check? */
    if (*id).sbbuf.is_null() {
      (*id).sbbuf = xmalloc(0x11000i32 as size_t) as *mut u8
    }
    small_off = off as libc::c_uint;
    dst = (*id).sbbuf;
    /* check if we need to read */
    len = (len as libc::c_ulong).wrapping_add(off) as size_t as size_t; /* we already have it */
    if len <= (*id).sbbuf_len {
      current_block = 3815402658071396482;
    } else {
      (*id).sbbuf_len = len;
      off = 0i32 as u64;
      current_block = 16164644963279819311;
    }
  } else {
    if len > 0x10000i32 as libc::c_ulong {
      return 0 as *mut libc::c_void;
    }
    dst = (*id).seekbuf;
    /* check if we need to read */
    if off >= (*id).seekbuf_off
      && off.wrapping_add(len) <= (*id).seekbuf_off.wrapping_add((*id).seekbuf_len)
    {
      small_off = off.wrapping_sub((*id).seekbuf_off) as libc::c_uint;
      current_block = 3815402658071396482; /* can't overflow */
    /* we already have it */
    } else {
      (*id).seekbuf_off = off;
      (*id).seekbuf_len = len;
      (*id).seekbuf = xrealloc((*id).seekbuf as *mut libc::c_void, len) as *mut u8;
      small_off = 0i32 as libc::c_uint;
      dst = (*id).seekbuf;
      current_block = 16164644963279819311;
    }
  }
  match current_block {
    16164644963279819311 => {
      if lseek((*id).fd, off as __off64_t, 0i32) as libc::c_ulong != off {
        current_block = 1886147455329398701;
      } else {
        read_len = full_read((*id).fd, dst as *mut libc::c_void, len);
        if read_len as libc::c_ulong != len {
          current_block = 1886147455329398701;
        } else {
          current_block = 3815402658071396482;
        }
      }
      match current_block {
        3815402658071396482 => {}
        _ => {
          /* No filesystem can be this tiny. It's most likely
           * non-associated loop device, empty drive and so on.
           * Flag it, making it possible to short circuit future
           * accesses. Rationale:
           * users complained of slow blkid due to empty floppy drives.
           */
          if off < (64i32 * 1024i32) as libc::c_ulong {
            (*id).error = 1i32
          }
          /* id->seekbuf_len or id->sbbuf_len is wrong now! Fixing. */
          volume_id_free_buffer(id);
          return 0 as *mut libc::c_void;
        }
      }
    }
    _ => {}
  }
  return dst.offset(small_off as isize) as *mut libc::c_void;
}
/*
 * volume_id - reads filesystem label and uuid
 *
 * Copyright (C) 2005 Kay Sievers <kay.sievers@vrfy.org>
 *
 *	This library is free software; you can redistribute it and/or
 *	modify it under the terms of the GNU Lesser General Public
 *	License as published by the Free Software Foundation; either
 *	version 2.1 of the License, or (at your option) any later version.
 *
 *	This library is distributed in the hope that it will be useful,
 *	but WITHOUT ANY WARRANTY; without even the implied warranty of
 *	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 *	Lesser General Public License for more details.
 *
 *	You should have received a copy of the GNU Lesser General Public
 *	License along with this library; if not, write to the Free Software
 *	Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA
 */
/* #define dbg(...) bb_error_msg(__VA_ARGS__) */
/* volume_id.h */
//	int		fd_close:1;
//	u8		label_raw[VOLUME_ID_LABEL_SIZE];
//	size_t		label_raw_len;
//	u8		uuid_raw[VOLUME_ID_UUID_SIZE];
//	size_t		uuid_raw_len;
/* uuid is stored in ASCII (not binary) form here: */
//	char		type_version[VOLUME_ID_FORMAT_SIZE];
//	smallint	usage_id;
//	const char	*usage;
/*u64 off,*/
/* util.h */
/* size of superblock buffer, reiserfs block is at 64k */
/* size of seek buffer, FAT cluster is 32k max */
/* volume_id_set_uuid(id,buf,fmt) assumes size of uuid buf
 * by shifting: 4 << fmt, except for fmt == UUID_DCE_STRING.
 * The constants below should match sizes.
 */
/* 4 bytes */
/* 8 bytes */
/* 16 bytes */
/* 36 bytes (VOLUME_ID_UUID_SIZE) */
//void volume_id_set_usage(struct volume_id *id, enum volume_id_usage usage_id);
//void volume_id_set_usage_part(struct volume_id_partition *part, enum volume_id_usage usage_id);
//void volume_id_set_label_raw(struct volume_id *id, const u8 *buf, size_t count);
#[no_mangle]
pub unsafe extern "C" fn volume_id_free_buffer(mut id: *mut volume_id) {
  free((*id).sbbuf as *mut libc::c_void);
  (*id).sbbuf = 0 as *mut u8;
  (*id).sbbuf_len = 0i32 as size_t;
  free((*id).seekbuf as *mut libc::c_void);
  (*id).seekbuf = 0 as *mut u8;
  (*id).seekbuf_len = 0i32 as size_t;
  (*id).seekbuf_off = 0i32 as u64;
  /* paranoia */
}

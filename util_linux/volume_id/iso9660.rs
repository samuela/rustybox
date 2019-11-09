use libc;
use libc::open;




extern "C" {
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;

  #[no_mangle]
  fn volume_id_get_buffer(id: *mut volume_id, off_0: u64, len: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn volume_id_set_label_string(id: *mut volume_id, buf: *const u8, count: size_t);

  #[no_mangle]
  fn volume_id_set_unicode16(
    str: *mut libc::c_char,
    len: size_t,
    buf: *const u8,
    endianess: endian,
    count: size_t,
  );
}

use crate::librb::size_t;



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

pub type endian = libc::c_uint;
pub const BE: endian = 1;
// pub const LE: endian = 0;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct high_sierra_volume_descriptor {
  pub foo: [u8; 8],
  pub type_0: u8,
  pub id: [u8; 4],
  pub version: u8,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct iso_volume_descriptor {
  pub vd_type: u8,
  pub vd_id: [u8; 5],
  pub vd_version: u8,
  pub flags: u8,
  pub system_id: [u8; 32],
  pub volume_id: [u8; 32],
  pub unused: [u8; 8],
  pub space_size: [u8; 8],
  pub escape_sequences: [u8; 8],
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
/* Probe routines */
/* RAID */
//int FAST_FUNC volume_id_probe_highpoint_37x_raid(struct volume_id *id /*,u64 off*/);
//int FAST_FUNC volume_id_probe_highpoint_45x_raid(struct volume_id *id /*,u64 off*/, u64 size);
//int FAST_FUNC volume_id_probe_intel_software_raid(struct volume_id *id /*,u64 off*/, u64 size);
/*,u64 off*/
//int FAST_FUNC volume_id_probe_lsi_mega_raid(struct volume_id *id /*,u64 off*/, u64 size);
//int FAST_FUNC volume_id_probe_nvidia_raid(struct volume_id *id /*,u64 off*/, u64 size);
//int FAST_FUNC volume_id_probe_promise_fasttrack_raid(struct volume_id *id /*,u64 off*/, u64 size);
//int FAST_FUNC volume_id_probe_silicon_medley_raid(struct volume_id *id /*,u64 off*/, u64 size);
//int FAST_FUNC volume_id_probe_via_raid(struct volume_id *id /*,u64 off*/, u64 size);
//int FAST_FUNC volume_id_probe_lvm1(struct volume_id *id /*,u64 off*/);
//int FAST_FUNC volume_id_probe_lvm2(struct volume_id *id /*,u64 off*/);
/* FS */
/*,u64 off*/
/*,u64 off*/
/*,u64 off*/
/*,u64 off*/
/*,u64 off*/
/*,u64 off*/
//int FAST_FUNC volume_id_probe_hpfs(struct volume_id *id /*,u64 off*/);
#[no_mangle]
pub unsafe extern "C" fn volume_id_probe_iso9660(mut id: *mut volume_id) -> libc::c_int
/*,u64 off*/ {
  let mut buf: *mut u8 = 0 as *mut u8;
  let mut is: *mut iso_volume_descriptor = 0 as *mut iso_volume_descriptor;
  let mut hs: *mut high_sierra_volume_descriptor = 0 as *mut high_sierra_volume_descriptor;
  buf = volume_id_get_buffer(
    id,
    (0i32 as u64).wrapping_add(0x8000i32 as libc::c_ulong),
    0x200i32 as size_t,
  ) as *mut u8;
  if buf.is_null() {
    return -1i32;
  }
  is = buf as *mut iso_volume_descriptor;
  if memcmp(
    (*is).vd_id.as_mut_ptr() as *const libc::c_void,
    b"CD001\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
    5i32 as libc::c_ulong,
  ) == 0i32
  {
    let mut vd_offset: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    //		volume_id_set_label_raw(id, is->volume_id, 32);
    volume_id_set_label_string(id, (*is).volume_id.as_mut_ptr(), 32i32 as size_t);
    vd_offset = 0x8000i32 + 0x800i32;
    i = 0i32;
    while i < 16i32 {
      let mut svd_label: [u8; 64] = [0; 64];
      is = volume_id_get_buffer(
        id,
        (0i32 as u64).wrapping_add(vd_offset as libc::c_ulong),
        0x200i32 as size_t,
      ) as *mut iso_volume_descriptor;
      if is.is_null() || (*is).vd_type as libc::c_int == 0xffi32 {
        break;
      }
      if !((*is).vd_type as libc::c_int != 0x2i32) {
        if memcmp(
          (*is).escape_sequences.as_mut_ptr() as *const libc::c_void,
          b"%/@\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
          3i32 as libc::c_ulong,
        ) == 0i32
          || memcmp(
            (*is).escape_sequences.as_mut_ptr() as *const libc::c_void,
            b"%/C\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            3i32 as libc::c_ulong,
          ) == 0i32
          || memcmp(
            (*is).escape_sequences.as_mut_ptr() as *const libc::c_void,
            b"%/E\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            3i32 as libc::c_ulong,
          ) == 0i32
        {
          volume_id_set_unicode16(
            svd_label.as_mut_ptr() as *mut libc::c_char,
            ::std::mem::size_of::<[u8; 64]>() as libc::c_ulong,
            (*is).volume_id.as_mut_ptr(),
            BE,
            32i32 as size_t,
          );
          if memcmp(
            (*id).label.as_mut_ptr() as *const libc::c_void,
            svd_label.as_mut_ptr() as *const libc::c_void,
            16i32 as libc::c_ulong,
          ) == 0i32
          {
            break;
          }
          //				volume_id_set_label_raw(id, is->volume_id, 32);
          volume_id_set_label_string(id, svd_label.as_mut_ptr(), 32i32 as size_t);
          //				strcpy(id->type_version, "Joliet Extension");
          break;
        } else {
          vd_offset += 0x800i32
        }
      }
      i += 1
    }
  } else {
    hs = buf as *mut high_sierra_volume_descriptor;
    if !(memcmp(
      (*hs).id.as_mut_ptr() as *const libc::c_void,
      b"CDROM\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
      5i32 as libc::c_ulong,
    ) == 0i32)
    {
      return -1i32;
    }
  }
  //		strcpy(id->type_version, "High Sierra");
  //	volume_id_set_usage(id, VOLUME_ID_FILESYSTEM);
  (*id).type_0 = b"iso9660\x00" as *const u8 as *const libc::c_char;
  return 0i32;
}

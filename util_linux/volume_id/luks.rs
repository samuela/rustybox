use libc;
use libc::unlink;



extern "C" {
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;

  #[no_mangle]
  fn volume_id_set_uuid(id: *mut volume_id, buf: *const u8, format: uuid_format);

  #[no_mangle]
  fn volume_id_get_buffer(id: *mut volume_id, off_0: u64, len: size_t) -> *mut libc::c_void;
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

pub type uuid_format = libc::c_uint;
pub const UUID_DCE_STRING: uuid_format = 3;
// pub const UUID_DCE: uuid_format = 2;
// pub const UUID_NTFS: uuid_format = 1;
// pub const UUID_DOS: uuid_format = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct luks_phdr {
  pub magic: [u8; 6],
  pub version: u16,
  pub cipherName: [u8; 32],
  pub cipherMode: [u8; 32],
  pub hashSpec: [u8; 32],
  pub payloadOffset: u32,
  pub keyBytes: u32,
  pub mkDigest: [u8; 20],
  pub mkDigestSalt: [u8; 32],
  pub mkDigestIterations: u32,
  pub uuid: [u8; 40],
  pub keyblock: [C2RustUnnamed; 8],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
  pub active: u32,
  pub passwordIterations: u32,
  pub passwordSalt: [u8; 32],
  pub keyMaterialOffset: u32,
  pub stripes: u32,
}

static mut LUKS_MAGIC: [u8; 6] = [
  'L' as i32 as u8,
  'U' as i32 as u8,
  'K' as i32 as u8,
  'S' as i32 as u8,
  0xbai32 as u8,
  0xbei32 as u8,
];

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
/*,u64 off*/
/*,u64 off*/
/*,u64 off*/
/*,u64 off*/
#[no_mangle]
pub unsafe extern "C" fn volume_id_probe_luks(mut id: *mut volume_id) -> libc::c_int
/*,u64 off*/ {
  let mut header: *mut luks_phdr = 0 as *mut luks_phdr;
  header = volume_id_get_buffer(
    id,
    0i32 as u64,
    ::std::mem::size_of::<luks_phdr>() as libc::c_ulong,
  ) as *mut luks_phdr;
  if header.is_null() {
    return -1i32;
  }
  if memcmp(
    (*header).magic.as_mut_ptr() as *const libc::c_void,
    LUKS_MAGIC.as_ptr() as *const libc::c_void,
    6i32 as libc::c_ulong,
  ) != 0
  {
    return -1i32;
  }
  //	volume_id_set_usage(id, VOLUME_ID_CRYPTO);
  volume_id_set_uuid(id, (*header).uuid.as_mut_ptr(), UUID_DCE_STRING);
  (*id).type_0 = b"crypto_LUKS\x00" as *const u8 as *const libc::c_char;
  return 0i32;
}

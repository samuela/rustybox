use crate::librb::size_t;

use libc;
use libc::free;

extern "C" {
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;

  #[no_mangle]
  fn volume_id_set_label_string(id: *mut volume_id, buf: *const u8, count: size_t);

  #[no_mangle]
  fn volume_id_set_uuid(id: *mut volume_id, buf: *const u8, format: uuid_format);

  #[no_mangle]
  fn volume_id_get_buffer(id: *mut volume_id, off: u64, len: size_t) -> *mut libc::c_void;
}

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
// pub const UUID_DCE_STRING: uuid_format = 3;
pub const UUID_DCE: uuid_format = 2;
// pub const UUID_NTFS: uuid_format = 1;
// pub const UUID_DOS: uuid_format = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct bcache_super_block {
  pub csum: u64,
  pub offset: u64,
  pub version: u64,
  pub magic: [u8; 16],
  pub uuid: [u8; 16],
  pub c2rust_unnamed: C2RustUnnamed_3,
  pub label: [u8; 32],
  pub flags: u64,
  pub seq: u64,
  pub pad: [u64; 8],
  pub c2rust_unnamed_0: C2RustUnnamed_0,
  pub last_mount: u32, /* time_t */
  pub first_bucket: u16,
  pub c2rust_unnamed_1: C2RustUnnamed,
  pub d: [u64; 256], /* journal buckets */
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub njournal_buckets: u16,
  pub keys: u16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
  pub c2rust_unnamed: C2RustUnnamed_2,
  pub c2rust_unnamed_0: C2RustUnnamed_1,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
  pub data_offset: u64,
  /*
   * block_size from the cache device section is still used by
   * backing devices, so don't add anything here until we fix
   * things to not need it for backing devices anymore
   */
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
  pub nbuckets: u64,
  pub block_size: u16,
  pub bucket_size: u16,
  pub nr_in_set: u16,
  pub nr_this_dev: u16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
  pub set_uuid: [u8; 16],
  pub set_magic: u64,
}
static mut bcache_magic: [libc::c_char; 16] = [
  0xc6i32 as libc::c_char,
  0x85i32 as libc::c_char,
  0x73i32 as libc::c_char,
  0xf6i32 as libc::c_char,
  0x4ei32 as libc::c_char,
  0x1ai32 as libc::c_char,
  0x45i32 as libc::c_char,
  0xcai32 as libc::c_char,
  0x82i32 as libc::c_char,
  0x65i32 as libc::c_char,
  0xf5i32 as libc::c_char,
  0x7fi32 as libc::c_char,
  0x48i32 as libc::c_char,
  0xbai32 as libc::c_char,
  0x6di32 as libc::c_char,
  0x81i32 as libc::c_char,
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
/*uint64_t off,*/
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
/*,uint64_t off*/
//int FAST_FUNC volume_id_probe_lsi_mega_raid(struct volume_id *id /*,u64 off*/, u64 size);
//int FAST_FUNC volume_id_probe_nvidia_raid(struct volume_id *id /*,u64 off*/, u64 size);
//int FAST_FUNC volume_id_probe_promise_fasttrack_raid(struct volume_id *id /*,u64 off*/, u64 size);
//int FAST_FUNC volume_id_probe_silicon_medley_raid(struct volume_id *id /*,u64 off*/, u64 size);
//int FAST_FUNC volume_id_probe_via_raid(struct volume_id *id /*,u64 off*/, u64 size);
//int FAST_FUNC volume_id_probe_lvm1(struct volume_id *id /*,u64 off*/);
//int FAST_FUNC volume_id_probe_lvm2(struct volume_id *id /*,u64 off*/);
/* FS */
/* supper block offset in kB */
/* magic string offset within super block */
#[no_mangle]
pub unsafe extern "C" fn volume_id_probe_bcache(mut id: *mut volume_id) -> libc::c_int
/*,uint64_t off*/ {
  let mut sb: *mut bcache_super_block = 0 as *mut bcache_super_block;
  sb = volume_id_get_buffer(
    id,
    0x1000i32 as u64,
    ::std::mem::size_of::<bcache_super_block>() as libc::c_ulong,
  ) as *mut bcache_super_block;
  if sb.is_null() {
    return -1i32;
  }
  if memcmp(
    (*sb).magic.as_mut_ptr() as *const libc::c_void,
    bcache_magic.as_ptr() as *const libc::c_void,
    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
  ) != 0i32
  {
    return -1i32;
  }
  volume_id_set_label_string(id, (*sb).label.as_mut_ptr(), 32i32 as size_t);
  volume_id_set_uuid(id, (*sb).uuid.as_mut_ptr(), UUID_DCE);
  (*id).type_0 = b"bcache\x00" as *const u8 as *const libc::c_char;
  return 0i32;
}

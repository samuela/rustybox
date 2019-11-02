use libc;
extern "C" {
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn volume_id_set_label_string(id: *mut volume_id, buf: *const uint8_t, count: size_t);
  #[no_mangle]
  fn volume_id_set_uuid(id: *mut volume_id, buf: *const uint8_t, format: uuid_format);
  #[no_mangle]
  fn volume_id_get_buffer(id: *mut volume_id, off: uint64_t, len: size_t) -> *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct volume_id {
  pub fd: libc::c_int,
  pub error: libc::c_int,
  pub sbbuf_len: size_t,
  pub seekbuf_len: size_t,
  pub sbbuf: *mut uint8_t,
  pub seekbuf: *mut uint8_t,
  pub seekbuf_off: uint64_t,
  pub label: [libc::c_char; 65],
  pub uuid: [libc::c_char; 37],
  pub type_0: *const libc::c_char,
}
pub type uuid_format = libc::c_uint;
pub const UUID_DCE_STRING: uuid_format = 3;
pub const UUID_DCE: uuid_format = 2;
pub const UUID_NTFS: uuid_format = 1;
pub const UUID_DOS: uuid_format = 0;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct btrfs_super_block {
  pub csum: [uint8_t; 32],
  pub fsid: [uint8_t; 16],
  pub bytenr: uint64_t,
  pub flags: uint64_t,
  pub magic: [uint8_t; 8],
  pub generation: uint64_t,
  pub root: uint64_t,
  pub chunk_root: uint64_t,
  pub log_root: uint64_t,
  pub log_root_transid: uint64_t,
  pub total_bytes: uint64_t,
  pub bytes_used: uint64_t,
  pub root_dir_objectid: uint64_t,
  pub num_devices: uint64_t,
  pub sectorsize: uint32_t,
  pub nodesize: uint32_t,
  pub leafsize: uint32_t,
  pub stripesize: uint32_t,
  pub sys_chunk_array_size: uint32_t,
  pub chunk_root_generation: uint64_t,
  pub compat_flags: uint64_t,
  pub compat_ro_flags: uint64_t,
  pub incompat_flags: uint64_t,
  pub csum_type: uint16_t,
  pub root_level: uint8_t,
  pub chunk_root_level: uint8_t,
  pub log_root_level: uint8_t,
  pub dev_item: btrfs_dev_item,
  pub label: [uint8_t; 256],
  // LABEL
  // ...
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct btrfs_dev_item {
  pub devid: uint64_t,
  pub total_bytes: uint64_t,
  pub bytes_used: uint64_t,
  pub io_align: uint32_t,
  pub io_width: uint32_t,
  pub sector_size: uint32_t,
  pub type_0: uint64_t,
  pub generation: uint64_t,
  pub start_offset: uint64_t,
  pub dev_group: uint32_t,
  pub seek_speed: uint8_t,
  pub bandwidth: uint8_t,
  pub uuid: [uint8_t; 16],
  pub fsid: [uint8_t; 16],
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
//	uint8_t		label_raw[VOLUME_ID_LABEL_SIZE];
//	size_t		label_raw_len;
//	uint8_t		uuid_raw[VOLUME_ID_UUID_SIZE];
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
//void volume_id_set_label_raw(struct volume_id *id, const uint8_t *buf, size_t count);
/* Probe routines */
/* RAID */
//int FAST_FUNC volume_id_probe_highpoint_37x_raid(struct volume_id *id /*,uint64_t off*/);
//int FAST_FUNC volume_id_probe_highpoint_45x_raid(struct volume_id *id /*,uint64_t off*/, uint64_t size);
//int FAST_FUNC volume_id_probe_intel_software_raid(struct volume_id *id /*,uint64_t off*/, uint64_t size);
/*,uint64_t off*/
//int FAST_FUNC volume_id_probe_lsi_mega_raid(struct volume_id *id /*,uint64_t off*/, uint64_t size);
//int FAST_FUNC volume_id_probe_nvidia_raid(struct volume_id *id /*,uint64_t off*/, uint64_t size);
//int FAST_FUNC volume_id_probe_promise_fasttrack_raid(struct volume_id *id /*,uint64_t off*/, uint64_t size);
//int FAST_FUNC volume_id_probe_silicon_medley_raid(struct volume_id *id /*,uint64_t off*/, uint64_t size);
//int FAST_FUNC volume_id_probe_via_raid(struct volume_id *id /*,uint64_t off*/, uint64_t size);
//int FAST_FUNC volume_id_probe_lvm1(struct volume_id *id /*,uint64_t off*/);
//int FAST_FUNC volume_id_probe_lvm2(struct volume_id *id /*,uint64_t off*/);
/* FS */
/*,uint64_t off*/
#[no_mangle]
pub unsafe extern "C" fn volume_id_probe_btrfs(mut id: *mut volume_id) -> libc::c_int
/*,uint64_t off*/ {
  // btrfs has superblocks at 64K, 64M and 256G
  // minimum btrfs size is 256M
  // so we never step out the device if we analyze
  // the first and the second superblocks
  let mut sb: *mut btrfs_super_block = 0 as *mut btrfs_super_block;
  let mut off: libc::c_uint = 64i32 as libc::c_uint;
  while off < (64i32 * 1024i32 * 1024i32) as libc::c_uint {
    off = off.wrapping_mul(1024i32 as libc::c_uint);
    sb = volume_id_get_buffer(
      id,
      off as uint64_t,
      ::std::mem::size_of::<btrfs_super_block>() as libc::c_ulong,
    ) as *mut btrfs_super_block;
    if sb.is_null() {
      return -1i32;
    }
    if memcmp(
      (*sb).magic.as_mut_ptr() as *const libc::c_void,
      b"_BHRfS_M\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
      8i32 as libc::c_ulong,
    ) != 0i32
    {
      return -1i32;
    }
  }
  // N.B.: btrfs natively supports 256 (>VOLUME_ID_LABEL_SIZE) size labels
  volume_id_set_label_string(id, (*sb).label.as_mut_ptr(), 64i32 as size_t);
  volume_id_set_uuid(id, (*sb).fsid.as_mut_ptr(), UUID_DCE);
  (*id).type_0 = b"btrfs\x00" as *const u8 as *const libc::c_char;
  return 0i32;
}

use crate::libbb::ptr_to_globals::bb_errno;
use libc;
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
// pub const UUID_DCE_STRING: uuid_format = 3;
pub const UUID_DCE: uuid_format = 2;
// pub const UUID_NTFS: uuid_format = 1;
// pub const UUID_DOS: uuid_format = 0;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct btrfs_super_block {
  pub csum: [u8; 32],
  pub fsid: [u8; 16],
  pub bytenr: u64,
  pub flags: u64,
  pub magic: [u8; 8],
  pub generation: u64,
  pub root: u64,
  pub chunk_root: u64,
  pub log_root: u64,
  pub log_root_transid: u64,
  pub total_bytes: u64,
  pub bytes_used: u64,
  pub root_dir_objectid: u64,
  pub num_devices: u64,
  pub sectorsize: u32,
  pub nodesize: u32,
  pub leafsize: u32,
  pub stripesize: u32,
  pub sys_chunk_array_size: u32,
  pub chunk_root_generation: u64,
  pub compat_flags: u64,
  pub compat_ro_flags: u64,
  pub incompat_flags: u64,
  pub csum_type: u16,
  pub root_level: u8,
  pub chunk_root_level: u8,
  pub log_root_level: u8,
  pub dev_item: btrfs_dev_item,
  pub label: [u8; 256],
  // LABEL
  // ...
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct btrfs_dev_item {
  pub devid: u64,
  pub total_bytes: u64,
  pub bytes_used: u64,
  pub io_align: u32,
  pub io_width: u32,
  pub sector_size: u32,
  pub type_0: u64,
  pub generation: u64,
  pub start_offset: u64,
  pub dev_group: u32,
  pub seek_speed: u8,
  pub bandwidth: u8,
  pub uuid: [u8; 16],
  pub fsid: [u8; 16],
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

#[no_mangle]
pub unsafe extern "C" fn volume_id_probe_btrfs(mut id: *mut volume_id) -> libc::c_int
/*,u64 off*/ {
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
      off as u64,
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

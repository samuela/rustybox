use libc;

extern "C" {
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;

  #[no_mangle]
  fn volume_id_set_label_string(id: *mut volume_id, buf: *const uint8_t, count: size_t);

  #[no_mangle]
  fn volume_id_set_uuid(id: *mut volume_id, buf: *const uint8_t, format: uuid_format);

  #[no_mangle]
  fn volume_id_get_buffer(id: *mut volume_id, off_0: uint64_t, len: size_t) -> *mut libc::c_void;
}

use crate::librb::__int16_t;
use crate::librb::__uint16_t;

use crate::librb::__uint64_t;
use crate::librb::int16_t;
use crate::librb::size_t;
use crate::librb::uint16_t;
use crate::librb::uint32_t;
use crate::librb::uint64_t;
use crate::librb::uint8_t;

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
// pub const UUID_DCE_STRING: uuid_format = 3;
pub const UUID_DCE: uuid_format = 2;
// pub const UUID_NTFS: uuid_format = 1;
// pub const UUID_DOS: uuid_format = 0;

/* This is the superblock. The OCFS2 header files have structs in structs.
This is one has been simplified since we only care about the superblock.
*/
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ocfs2_super_block {
  pub i_signature: [uint8_t; 8],
  pub i_generation: uint32_t,
  pub i_suballoc_slot: int16_t,
  pub i_suballoc_bit: uint16_t,
  pub i_reserved0: uint32_t,
  pub i_clusters: uint32_t,
  pub i_uid: uint32_t,
  pub i_gid: uint32_t,
  pub i_size: uint64_t,
  pub i_mode: uint16_t,
  pub i_links_count: uint16_t,
  pub i_flags: uint32_t,
  pub i_atime: uint64_t,
  pub i_ctime: uint64_t,
  pub i_mtime: uint64_t,
  pub i_dtime: uint64_t,
  pub i_blkno: uint64_t,
  pub i_last_eb_blk: uint64_t,
  pub i_fs_generation: uint32_t,
  pub i_atime_nsec: uint32_t,
  pub i_ctime_nsec: uint32_t,
  pub i_mtime_nsec: uint32_t,
  pub i_reserved1: [uint64_t; 9],
  pub i_pad1: uint64_t,
  pub s_major_rev_level: uint16_t,
  pub s_minor_rev_level: uint16_t,
  pub s_mnt_count: uint16_t,
  pub s_max_mnt_count: int16_t,
  pub s_state: uint16_t,
  pub s_errors: uint16_t,
  pub s_checkinterval: uint32_t,
  pub s_lastcheck: uint64_t,
  pub s_creator_os: uint32_t,
  pub s_feature_compat: uint32_t,
  pub s_feature_incompat: uint32_t,
  pub s_feature_ro_compat: uint32_t,
  pub s_root_blkno: uint64_t,
  pub s_system_dir_blkno: uint64_t,
  pub s_blocksize_bits: uint32_t,
  pub s_clustersize_bits: uint32_t,
  pub s_max_slots: uint16_t,
  pub s_reserved1: uint16_t,
  pub s_reserved2: uint32_t,
  pub s_first_cluster_group: uint64_t,
  pub s_label: [uint8_t; 64],
  pub s_uuid: [uint8_t; 16],
  /* 128-bit uuid */
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
/*,uint64_t off*/
/*,uint64_t off*/
/*,uint64_t off*/
/*,uint64_t off*/
/*,uint64_t off*/
//int FAST_FUNC volume_id_probe_hpfs(struct volume_id *id /*,uint64_t off*/);
/*,uint64_t off*/
/*,uint64_t off*/
/*,uint64_t off*/
/*,uint64_t off*/
/*,uint64_t off*/
//int FAST_FUNC volume_id_probe_mac_partition_map(struct volume_id *id /*,uint64_t off*/);
/*, uint64_t off*/
//int FAST_FUNC volume_id_probe_msdos_part_table(struct volume_id *id /*,uint64_t off*/);
/*,uint64_t off*/
/*,uint64_t off*/
/*,uint64_t off*/
/*,uint64_t off*/
#[no_mangle]
pub unsafe extern "C" fn volume_id_probe_ocfs2(mut id: *mut volume_id) -> libc::c_int
/*,uint64_t off*/ {
  let mut os: *mut ocfs2_super_block = 0 as *mut ocfs2_super_block;
  os = volume_id_get_buffer(
    id,
    (0i32 as uint64_t).wrapping_add(0x2000i32 as libc::c_ulong),
    0x200i32 as size_t,
  ) as *mut ocfs2_super_block;
  if os.is_null() {
    return -1i32;
  }
  if memcmp(
    (*os).i_signature.as_mut_ptr() as *const libc::c_void,
    b"OCFSV2\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
    6i32 as libc::c_ulong,
  ) != 0i32
  {
    return -1i32;
  }
  //	volume_id_set_usage(id, VOLUME_ID_FILESYSTEM);
  //	volume_id_set_label_raw(id, os->s_label, OCFS2_MAX_VOL_LABEL_LEN < VOLUME_ID_LABEL_SIZE ?
  //					OCFS2_MAX_VOL_LABEL_LEN : VOLUME_ID_LABEL_SIZE);
  volume_id_set_label_string(
    id,
    (*os).s_label.as_mut_ptr(),
    if 64i32 < 64i32 { 64i32 } else { 64i32 } as size_t,
  );
  volume_id_set_uuid(id, (*os).s_uuid.as_mut_ptr(), UUID_DCE);
  (*id).type_0 = b"ocfs2\x00" as *const u8 as *const libc::c_char;
  return 0i32;
}

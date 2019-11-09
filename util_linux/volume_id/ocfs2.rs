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
  fn volume_id_get_buffer(id: *mut volume_id, off_0: u64, len: size_t) -> *mut libc::c_void;
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

/* This is the superblock. The OCFS2 header files have structs in structs.
This is one has been simplified since we only care about the superblock.
*/
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ocfs2_super_block {
  pub i_signature: [u8; 8],
  pub i_generation: u32,
  pub i_suballoc_slot: i16,
  pub i_suballoc_bit: u16,
  pub i_reserved0: u32,
  pub i_clusters: u32,
  pub i_uid: u32,
  pub i_gid: u32,
  pub i_size: u64,
  pub i_mode: u16,
  pub i_links_count: u16,
  pub i_flags: u32,
  pub i_atime: u64,
  pub i_ctime: u64,
  pub i_mtime: u64,
  pub i_dtime: u64,
  pub i_blkno: u64,
  pub i_last_eb_blk: u64,
  pub i_fs_generation: u32,
  pub i_atime_nsec: u32,
  pub i_ctime_nsec: u32,
  pub i_mtime_nsec: u32,
  pub i_reserved1: [u64; 9],
  pub i_pad1: u64,
  pub s_major_rev_level: u16,
  pub s_minor_rev_level: u16,
  pub s_mnt_count: u16,
  pub s_max_mnt_count: i16,
  pub s_state: u16,
  pub s_errors: u16,
  pub s_checkinterval: u32,
  pub s_lastcheck: u64,
  pub s_creator_os: u32,
  pub s_feature_compat: u32,
  pub s_feature_incompat: u32,
  pub s_feature_ro_compat: u32,
  pub s_root_blkno: u64,
  pub s_system_dir_blkno: u64,
  pub s_blocksize_bits: u32,
  pub s_clustersize_bits: u32,
  pub s_max_slots: u16,
  pub s_reserved1: u16,
  pub s_reserved2: u32,
  pub s_first_cluster_group: u64,
  pub s_label: [u8; 64],
  pub s_uuid: [u8; 16],
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
/*,u64 off*/
//int FAST_FUNC volume_id_probe_mac_partition_map(struct volume_id *id /*,u64 off*/);
/*, u64 off*/
//int FAST_FUNC volume_id_probe_msdos_part_table(struct volume_id *id /*,u64 off*/);
/*,u64 off*/
/*,u64 off*/
/*,u64 off*/
/*,u64 off*/
#[no_mangle]
pub unsafe extern "C" fn volume_id_probe_ocfs2(mut id: *mut volume_id) -> libc::c_int
/*,u64 off*/ {
  let mut os: *mut ocfs2_super_block = 0 as *mut ocfs2_super_block;
  os = volume_id_get_buffer(
    id,
    (0i32 as u64).wrapping_add(0x2000i32 as libc::c_ulong),
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

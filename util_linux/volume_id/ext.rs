use libc;

extern "C" {
  #[no_mangle]
  fn volume_id_set_label_string(id: *mut volume_id, buf: *const uint8_t, count: size_t);

  #[no_mangle]
  fn volume_id_set_uuid(id: *mut volume_id, buf: *const uint8_t, format: uuid_format);

  #[no_mangle]
  fn volume_id_get_buffer(id: *mut volume_id, off_0: uint64_t, len: size_t) -> *mut libc::c_void;
}






use crate::librb::int16_t;
use crate::librb::int32_t;
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

/*
 * File system states
 */
/* Unmounted cleanly */
/* Errors detected */
/*
 * Mount flags
 */
/* Do mount-time checks */
/* Create files with directory's group */
/* Some debugging messages */
/* Continue on errors */
/* Remount fs ro on errors */
/* Panic on errors */
/* Mimics the Minix statfs */
/* Disable 32-bit UIDs */
/*
 * Maximal mount counts between two filesystem checks
 */
/* Allow 20 mounts */
/* Don't use interval check */
/*
 * Behaviour when detecting errors
 */
/* Continue execution */
/* Remount fs read-only */
/* Panic */
/*
 * Structure of the super block
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ext2_super_block {
  pub s_inodes_count: uint32_t,
  pub s_blocks_count: uint32_t,
  pub s_r_blocks_count: uint32_t,
  pub s_free_blocks_count: uint32_t,
  pub s_free_inodes_count: uint32_t,
  pub s_first_data_block: uint32_t,
  pub s_log_block_size: uint32_t,
  pub s_log_frag_size: int32_t,
  pub s_blocks_per_group: uint32_t,
  pub s_frags_per_group: uint32_t,
  pub s_inodes_per_group: uint32_t,
  pub s_mtime: uint32_t,
  pub s_wtime: uint32_t,
  pub s_mnt_count: uint16_t,
  pub s_max_mnt_count: int16_t,
  pub s_magic: uint16_t,
  pub s_state: uint16_t,
  pub s_errors: uint16_t,
  pub s_minor_rev_level: uint16_t,
  pub s_lastcheck: uint32_t,
  pub s_checkinterval: uint32_t,
  pub s_creator_os: uint32_t,
  pub s_rev_level: uint32_t,
  pub s_def_resuid: uint16_t,
  pub s_def_resgid: uint16_t,
  pub s_first_ino: uint32_t,
  pub s_inode_size: uint16_t,
  pub s_block_group_nr: uint16_t,
  pub s_feature_compat: uint32_t,
  pub s_feature_incompat: uint32_t,
  pub s_feature_ro_compat: uint32_t,
  pub s_uuid: [uint8_t; 16],
  pub s_volume_name: [libc::c_char; 16],
  pub s_last_mounted: [libc::c_char; 64],
  pub s_algorithm_usage_bitmap: uint32_t,
  pub s_prealloc_blocks: uint8_t,
  pub s_prealloc_dir_blocks: uint8_t,
  pub s_reserved_gdt_blocks: uint16_t,
  pub s_journal_uuid: [uint8_t; 16],
  pub s_journal_inum: uint32_t,
  pub s_journal_dev: uint32_t,
  pub s_last_orphan: uint32_t,
  pub s_hash_seed: [uint32_t; 4],
  pub s_def_hash_version: uint8_t,
  pub s_jnl_backup_type: uint8_t,
  pub s_reserved_word_pad: uint16_t,
  pub s_default_mount_opts: uint32_t,
  pub s_first_meta_bg: uint32_t,
  pub s_mkfs_time: uint32_t,
  pub s_jnl_blocks: [uint32_t; 17],
  pub s_blocks_count_hi: uint32_t,
  pub s_r_blocks_count_hi: uint32_t,
  pub s_free_blocks_count_hi: uint32_t,
  pub s_min_extra_isize: uint16_t,
  pub s_want_extra_isize: uint16_t,
  pub s_flags: uint32_t,
  pub s_raid_stride: uint16_t,
  pub s_mmp_interval: uint16_t,
  pub s_mmp_block: uint64_t,
  pub s_raid_stripe_width: uint32_t,
  pub s_log_groups_per_flex: uint8_t,
  pub s_reserved_char_pad2: uint8_t,
  pub s_reserved_pad: uint16_t,
  pub s_reserved: [uint32_t; 162],
  /* Padding to the end of the block */
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
#[no_mangle]
pub unsafe extern "C" fn volume_id_probe_ext(mut id: *mut volume_id) -> libc::c_int
/*,uint64_t off*/ {
  let mut es: *mut ext2_super_block = 0 as *mut ext2_super_block;
  es = volume_id_get_buffer(
    id,
    (0i32 as uint64_t).wrapping_add(0x400i32 as libc::c_ulong),
    0x200i32 as size_t,
  ) as *mut ext2_super_block;
  if es.is_null() {
    return -1i32;
  }
  if (*es).s_magic as libc::c_int != 0xef53i32 as uint16_t as libc::c_int {
    return -1i32;
  }
  //	volume_id_set_usage(id, VOLUME_ID_FILESYSTEM);
  //	volume_id_set_label_raw(id, es->volume_name, 16);
  volume_id_set_label_string(
    id,
    (*es).s_volume_name.as_mut_ptr() as *mut libc::c_void as *const uint8_t,
    16i32 as size_t,
  );
  volume_id_set_uuid(id, (*es).s_uuid.as_mut_ptr(), UUID_DCE);
  if (*es).s_feature_ro_compat & (0x8i32 | 0x20i32) as uint32_t != 0
    || (*es).s_feature_incompat & (0x40i32 | 0x80i32) as uint32_t != 0
  {
    (*id).type_0 = b"ext4\x00" as *const u8 as *const libc::c_char
  } else if (*es).s_feature_compat & 0x4i32 as uint32_t != 0 {
    (*id).type_0 = b"ext3\x00" as *const u8 as *const libc::c_char
  } else {
    (*id).type_0 = b"ext2\x00" as *const u8 as *const libc::c_char
  }
  return 0i32;
}

use libc;

extern "C" {
  #[no_mangle]
  fn volume_id_set_label_unicode16(
    id: *mut volume_id,
    buf: *const uint8_t,
    endianess: endian,
    count: size_t,
  );

  #[no_mangle]
  fn volume_id_set_uuid(id: *mut volume_id, buf: *const uint8_t, format: uuid_format);

  #[no_mangle]
  fn volume_id_get_buffer(id: *mut volume_id, off: uint64_t, len: size_t) -> *mut libc::c_void;
}

pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
use crate::librb::uint8_t;
pub type uint16_t = __uint16_t;
use crate::librb::uint32_t;
use crate::librb::uint64_t;
use crate::librb::size_t;

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

pub type endian = libc::c_uint;
// pub const BE: endian = 1;
pub const LE: endian = 0;

// offset for 1:st super block
/*
#define F2FS_SB2_OFFSET		0x1400		// offset for 2:nd super block
*/
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct f2fs_super_block {
  pub magic: uint32_t,
  pub major_ver: uint16_t,
  pub minor_ver: uint16_t,
  pub log_sectorsize: uint32_t,
  pub log_sectors_per_block: uint32_t,
  pub log_blocksize: uint32_t,
  pub log_blocks_per_seg: uint32_t,
  pub segs_per_sec: uint32_t,
  pub secs_per_zone: uint32_t,
  pub checksum_offset: uint32_t,
  pub block_count: uint64_t,
  pub section_count: uint32_t,
  pub segment_count: uint32_t,
  pub segment_count_ckpt: uint32_t,
  pub segment_count_sit: uint32_t,
  pub segment_count_nat: uint32_t,
  pub segment_count_ssa: uint32_t,
  pub segment_count_main: uint32_t,
  pub segment0_blkaddr: uint32_t,
  pub cp_blkaddr: uint32_t,
  pub sit_blkaddr: uint32_t,
  pub nat_blkaddr: uint32_t,
  pub ssa_blkaddr: uint32_t,
  pub main_blkaddr: uint32_t,
  pub root_ino: uint32_t,
  pub node_ino: uint32_t,
  pub meta_ino: uint32_t,
  pub uuid: [uint8_t; 16],
  pub volume_name: [uint16_t; 512],
  // volume name
  // /* 0x47C */	uint32_t	extension_count;	// # of extensions below
  // /* 0x480 */	uint8_t		extension_list[64][8];	// extension array
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
#[no_mangle]
pub unsafe extern "C" fn volume_id_probe_f2fs(mut id: *mut volume_id) -> libc::c_int
/*,uint64_t off*/ {
  let mut sb: *mut f2fs_super_block = 0 as *mut f2fs_super_block;
  // Go for primary super block (ignore second sb)
  sb = volume_id_get_buffer(
    id,
    0x400i32 as uint64_t,
    ::std::mem::size_of::<f2fs_super_block>() as libc::c_ulong,
  ) as *mut f2fs_super_block;
  if sb.is_null() {
    return -1i32;
  }
  if (*sb).magic != 0xf2f52010u32 {
    return -1i32;
  }
  (*id).type_0 = b"f2fs\x00" as *const u8 as *const libc::c_char;
  // For version 1.0 we don't know sb structure and can't set label/uuid
  if (*sb).major_ver as libc::c_int == 1i32 as uint16_t as libc::c_int
    && (*sb).minor_ver as libc::c_int == 0i32 as uint16_t as libc::c_int
  {
    return 0i32;
  }
  volume_id_set_label_unicode16(
    id,
    (*sb).volume_name.as_mut_ptr() as *mut uint8_t,
    LE,
    if 1024i32 < 64i32 { 1024i32 } else { 64i32 } as size_t,
  );
  volume_id_set_uuid(id, (*sb).uuid.as_mut_ptr(), UUID_DCE);
  return 0i32;
}

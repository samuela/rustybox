use libc;




extern "C" {
  #[no_mangle]
  fn volume_id_set_label_unicode16(
    id: *mut volume_id,
    buf: *const u8,
    endianess: endian,
    count: size_t,
  );

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
  pub magic: u32,
  pub major_ver: u16,
  pub minor_ver: u16,
  pub log_sectorsize: u32,
  pub log_sectors_per_block: u32,
  pub log_blocksize: u32,
  pub log_blocks_per_seg: u32,
  pub segs_per_sec: u32,
  pub secs_per_zone: u32,
  pub checksum_offset: u32,
  pub block_count: u64,
  pub section_count: u32,
  pub segment_count: u32,
  pub segment_count_ckpt: u32,
  pub segment_count_sit: u32,
  pub segment_count_nat: u32,
  pub segment_count_ssa: u32,
  pub segment_count_main: u32,
  pub segment0_blkaddr: u32,
  pub cp_blkaddr: u32,
  pub sit_blkaddr: u32,
  pub nat_blkaddr: u32,
  pub ssa_blkaddr: u32,
  pub main_blkaddr: u32,
  pub root_ino: u32,
  pub node_ino: u32,
  pub meta_ino: u32,
  pub uuid: [u8; 16],
  pub volume_name: [u16; 512],
  // volume name
  // /* 0x47C */	u32	extension_count;	// # of extensions below
  // /* 0x480 */	u8		extension_list[64][8];	// extension array
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
#[no_mangle]
pub unsafe extern "C" fn volume_id_probe_f2fs(mut id: *mut volume_id) -> libc::c_int
/*,u64 off*/ {
  let mut sb: *mut f2fs_super_block = 0 as *mut f2fs_super_block;
  // Go for primary super block (ignore second sb)
  sb = volume_id_get_buffer(
    id,
    0x400i32 as u64,
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
  if (*sb).major_ver as libc::c_int == 1i32 as u16 as libc::c_int
    && (*sb).minor_ver as libc::c_int == 0i32 as u16 as libc::c_int
  {
    return 0i32;
  }
  volume_id_set_label_unicode16(
    id,
    (*sb).volume_name.as_mut_ptr() as *mut u8,
    LE,
    if 1024i32 < 64i32 { 1024i32 } else { 64i32 } as size_t,
  );
  volume_id_set_uuid(id, (*sb).uuid.as_mut_ptr(), UUID_DCE);
  return 0i32;
}

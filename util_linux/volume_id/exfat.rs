use libc;
use libc::unlink;



extern "C" {
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;

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
pub const UUID_DOS: uuid_format = 0;
pub type endian = libc::c_uint;
// pub const BE: endian = 1;
pub const LE: endian = 0;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct volume_guid {
  pub sec_count: u8,
  pub set_checksum: u16,
  pub flags: u16,
  pub vol_guid: [u8; 16],
  pub reserved: [u8; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub label: volume_label,
  pub guid: volume_guid,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct volume_label {
  pub char_count: u8,
  pub vol_label: [u16; 11],
  pub reserved: [u8; 8],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct exfat_dir_entry {
  pub entry_type: u8,
  pub type_0: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct exfat_super_block {
  pub boot_jump: [u8; 3],
  pub fs_name: [u8; 8],
  pub must_be_zero: [u8; 53],
  pub partition_offset: u64,
  pub volume_length: u64,
  pub fat_offset: u32,
  pub fat_size: u32,
  pub cluster_heap_offset: u32,
  pub cluster_count: u32,
  pub root_dir: u32,
  pub vol_serial_nr: [u8; 4],
  pub fs_revision: u16,
  pub vol_flags: u16,
  pub bytes_per_sector: u8,
  pub sectors_per_cluster: u8,
  pub nr_of_fats: u8,
  // 2 for TexFAT
  /* 0x6F */
  // ...
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
#[no_mangle]
pub unsafe extern "C" fn volume_id_probe_exfat(mut id: *mut volume_id) -> libc::c_int
/*,u64 off*/ {
  let mut sb: *mut exfat_super_block = 0 as *mut exfat_super_block;
  let mut de: *mut exfat_dir_entry = 0 as *mut exfat_dir_entry;
  let mut sector_sz: libc::c_uint = 0;
  let mut cluster_sz: libc::c_uint = 0;
  let mut root_dir_off: u64 = 0;
  let mut count: libc::c_uint = 0;
  let mut need_lbl_guid: libc::c_uint = 0;
  // Primary super block
  sb = volume_id_get_buffer(
    id,
    0i32 as u64,
    ::std::mem::size_of::<exfat_super_block>() as libc::c_ulong,
  ) as *mut exfat_super_block;
  if sb.is_null() {
    return -1i32;
  }
  if memcmp(
    (*sb).fs_name.as_mut_ptr() as *const libc::c_void,
    b"EXFAT   \x00" as *const u8 as *const libc::c_char as *const libc::c_void,
    8i32 as libc::c_ulong,
  ) != 0i32
  {
    return -1i32;
  }
  sector_sz = (1i32 << (*sb).bytes_per_sector as libc::c_int) as libc::c_uint;
  cluster_sz = sector_sz << (*sb).sectors_per_cluster as libc::c_int;
  // There are no clusters 0 and 1, so the first cluster is 2.
  root_dir_off = (0i32 as u64)
    .wrapping_add((*sb).cluster_heap_offset.wrapping_mul(sector_sz) as libc::c_ulong)
    .wrapping_add(
      (*sb)
        .root_dir
        .wrapping_sub(2i32 as libc::c_uint)
        .wrapping_mul(cluster_sz) as libc::c_ulong,
    );
  // Use DOS uuid as fallback, if no GUID set
  volume_id_set_uuid(id, (*sb).vol_serial_nr.as_mut_ptr(), UUID_DOS);
  // EXFAT_MAX_DIR_ENTRIES is used as a safety belt.
  // The Root Directory may hold an unlimited number of entries,
  // so we do not want to check all. Usually label and GUID
  // are in the beginning, but there are no guarantees.
  need_lbl_guid = (1i32 << 0i32 | 1i32 << 1i32) as libc::c_uint;
  count = 0i32 as libc::c_uint;
  while count < 100i32 as libc::c_uint {
    de = volume_id_get_buffer(
      id,
      root_dir_off.wrapping_add(count.wrapping_mul(32i32 as libc::c_uint) as libc::c_ulong),
      32i32 as size_t,
    ) as *mut exfat_dir_entry;
    if de.is_null() {
      break;
    }
    if (*de).entry_type as libc::c_int == 0i32 {
      break;
    }
    if (*de).entry_type as libc::c_int == 0x83i32 {
      // Volume Label Directory Entry
      volume_id_set_label_unicode16(
        id,
        (*de).type_0.label.vol_label.as_mut_ptr() as *mut u8,
        LE,
        (2i32 * (*de).type_0.label.char_count as libc::c_int) as size_t,
      );
      need_lbl_guid &= !(1i32 << 0i32) as libc::c_uint
    }
    if (*de).entry_type as libc::c_int == 0xa0i32 {
      // Volume GUID Directory Entry
      volume_id_set_uuid(id, (*de).type_0.guid.vol_guid.as_mut_ptr(), UUID_DCE);
      need_lbl_guid &= !(1i32 << 1i32) as libc::c_uint
    }
    if need_lbl_guid == 0 {
      break;
    }
    count = count.wrapping_add(1)
  }
  (*id).type_0 = b"exfat\x00" as *const u8 as *const libc::c_char;
  return 0i32;
}

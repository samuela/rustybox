use libc;

extern "C" {
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;

  #[no_mangle]
  fn volume_id_get_buffer(id: *mut volume_id, off_0: uint64_t, len: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn volume_id_set_uuid(id: *mut volume_id, buf: *const uint8_t, format: uuid_format);

  #[no_mangle]
  fn volume_id_set_label_unicode16(
    id: *mut volume_id,
    buf: *const uint8_t,
    endianess: endian,
    count: size_t,
  );
}

pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
use crate::librb::uint8_t;
pub type uint16_t = __uint16_t;
use crate::librb::uint32_t;
use crate::librb::uint64_t;
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
// pub const UUID_DCE_STRING: uuid_format = 3;
// pub const UUID_DCE: uuid_format = 2;
pub const UUID_NTFS: uuid_format = 1;
// pub const UUID_DOS: uuid_format = 0;

pub type endian = libc::c_uint;
// pub const BE: endian = 1;
pub const LE: endian = 0;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct file_attribute {
  pub type_0: uint32_t,
  pub len: uint32_t,
  pub non_resident: uint8_t,
  pub name_len: uint8_t,
  pub name_offset: uint16_t,
  pub flags: uint16_t,
  pub instance: uint16_t,
  pub value_len: uint32_t,
  pub value_offset: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct master_file_table_record {
  pub magic: [uint8_t; 4],
  pub usa_ofs: uint16_t,
  pub usa_count: uint16_t,
  pub lsn: uint64_t,
  pub sequence_number: uint16_t,
  pub link_count: uint16_t,
  pub attrs_offset: uint16_t,
  pub flags: uint16_t,
  pub bytes_in_use: uint32_t,
  pub bytes_allocated: uint32_t,
}
/*
 * volume_id - reads filesystem label and uuid
 *
 * Copyright (C) 2004 Kay Sievers <kay.sievers@vrfy.org>
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
//config:config FEATURE_VOLUMEID_NTFS
//config:	bool "ntfs filesystem"
//config:	default y
//config:	depends on VOLUMEID
//kbuild:lib-$(CONFIG_FEATURE_VOLUMEID_NTFS) += ntfs.o
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ntfs_super_block {
  pub jump: [uint8_t; 3],
  pub oem_id: [uint8_t; 8],
  pub bytes_per_sector: uint16_t,
  pub sectors_per_cluster: uint8_t,
  pub reserved_sectors: uint16_t,
  pub fats: uint8_t,
  pub root_entries: uint16_t,
  pub sectors: uint16_t,
  pub media_type: uint8_t,
  pub sectors_per_fat: uint16_t,
  pub sectors_per_track: uint16_t,
  pub heads: uint16_t,
  pub hidden_sectors: uint32_t,
  pub large_sectors: uint32_t,
  pub unused: [uint16_t; 2],
  pub number_of_sectors: uint64_t,
  pub mft_cluster_location: uint64_t,
  pub mft_mirror_cluster_location: uint64_t,
  pub cluster_per_mft_record: int8_t,
  pub reserved1: [uint8_t; 3],
  pub cluster_per_index_record: int8_t,
  pub reserved2: [uint8_t; 3],
  pub volume_serial: [uint8_t; 8],
  pub checksum: uint16_t,
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
#[no_mangle]
pub unsafe extern "C" fn volume_id_probe_ntfs(mut id: *mut volume_id) -> libc::c_int
/*,uint64_t off*/ {
  let mut sector_size: libc::c_uint = 0;
  let mut cluster_size: libc::c_uint = 0;
  let mut mft_cluster: uint64_t = 0;
  let mut mft_off: uint64_t = 0;
  let mut mft_record_size: libc::c_uint = 0;
  let mut attr_type: libc::c_uint = 0;
  let mut attr_off: libc::c_uint = 0;
  let mut attr_len: libc::c_uint = 0;
  let mut val_off: libc::c_uint = 0;
  let mut val_len: libc::c_uint = 0;
  let mut mftr: *mut master_file_table_record = 0 as *mut master_file_table_record;
  let mut ns: *mut ntfs_super_block = 0 as *mut ntfs_super_block;
  let mut buf: *const uint8_t = 0 as *const uint8_t;
  let mut val: *const uint8_t = 0 as *const uint8_t;
  ns = volume_id_get_buffer(id, 0i32 as uint64_t, 0x200i32 as size_t) as *mut ntfs_super_block;
  if ns.is_null() {
    return -1i32;
  }
  if memcmp(
    (*ns).oem_id.as_mut_ptr() as *const libc::c_void,
    b"NTFS\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
    4i32 as libc::c_ulong,
  ) != 0i32
  {
    return -1i32;
  }
  volume_id_set_uuid(id, (*ns).volume_serial.as_mut_ptr(), UUID_NTFS);
  sector_size = (*ns).bytes_per_sector as libc::c_uint;
  cluster_size = ((*ns).sectors_per_cluster as libc::c_uint).wrapping_mul(sector_size);
  mft_cluster = (*ns).mft_cluster_location;
  mft_off = mft_cluster.wrapping_mul(cluster_size as libc::c_ulong);
  if ((*ns).cluster_per_mft_record as libc::c_int) < 0i32 {
    /* size = -log2(mft_record_size); normally 1024 Bytes */
    mft_record_size = (1i32 << -((*ns).cluster_per_mft_record as libc::c_int)) as libc::c_uint
  } else {
    mft_record_size = ((*ns).cluster_per_mft_record as libc::c_uint).wrapping_mul(cluster_size)
  }
  buf = volume_id_get_buffer(
    id,
    (0i32 as uint64_t)
      .wrapping_add(mft_off)
      .wrapping_add((3i32 as libc::c_uint).wrapping_mul(mft_record_size) as libc::c_ulong),
    mft_record_size as size_t,
  ) as *const uint8_t;
  if !buf.is_null() {
    mftr = buf as *mut master_file_table_record;
    if !(memcmp(
      (*mftr).magic.as_mut_ptr() as *const libc::c_void,
      b"FILE\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
      4i32 as libc::c_ulong,
    ) != 0i32)
    {
      attr_off = (*mftr).attrs_offset as libc::c_uint;
      loop {
        let mut attr: *mut file_attribute = 0 as *mut file_attribute;
        attr = &*buf.offset(attr_off as isize) as *const uint8_t as *mut file_attribute;
        attr_type = (*attr).type_0;
        attr_len = (*attr).len;
        val_off = (*attr).value_offset as libc::c_uint;
        val_len = (*attr).value_len;
        attr_off = attr_off.wrapping_add(attr_len);
        if attr_len == 0i32 as libc::c_uint {
          break;
        }
        if attr_off >= mft_record_size {
          break;
        }
        if attr_type == 0xffffffffu32 {
          break;
        }
        //		if (attr_type == MFT_RECORD_ATTR_VOLUME_INFO) {
        //			struct volume_info *info;
        //			dbg("found info, len %i", val_len);
        //			info = (struct volume_info*) (((uint8_t *) attr) + val_off);
        //			snprintf(id->type_version, sizeof(id->type_version)-1,
        //				 "%u.%u", info->major_ver, info->minor_ver);
        //		}
        if attr_type == 0x60i32 as libc::c_uint {
          if val_len > 64i32 as libc::c_uint {
            val_len = 64i32 as libc::c_uint
          }
          val = (attr as *mut uint8_t).offset(val_off as isize);
          //			volume_id_set_label_raw(id, val, val_len);
          volume_id_set_label_unicode16(id, val, LE, val_len as size_t);
        }
      }
    }
  }
  //	volume_id_set_usage(id, VOLUME_ID_FILESYSTEM);
  (*id).type_0 = b"ntfs\x00" as *const u8 as *const libc::c_char;
  return 0i32;
}

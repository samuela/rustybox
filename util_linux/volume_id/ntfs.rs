use libc;
extern "C" {
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;

}

use crate::librb::size_t;

use crate::util_linux::volume_id::volume_id::volume_id;

pub type uuid_format = libc::c_uint;
// pub const UUID_DCE_STRING: uuid_format = 3;
// pub const UUID_DCE: uuid_format = 2;
pub const UUID_NTFS: uuid_format = 1;
// pub const UUID_DOS: uuid_format = 0;

pub type endian = libc::c_uint;
// pub const BE: endian = 1;
pub const LE: endian = 0;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct file_attribute {
  pub type_0: u32,
  pub len: u32,
  pub non_resident: u8,
  pub name_len: u8,
  pub name_offset: u16,
  pub flags: u16,
  pub instance: u16,
  pub value_len: u32,
  pub value_offset: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct master_file_table_record {
  pub magic: [u8; 4],
  pub usa_ofs: u16,
  pub usa_count: u16,
  pub lsn: u64,
  pub sequence_number: u16,
  pub link_count: u16,
  pub attrs_offset: u16,
  pub flags: u16,
  pub bytes_in_use: u32,
  pub bytes_allocated: u32,
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

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ntfs_super_block {
  pub jump: [u8; 3],
  pub oem_id: [u8; 8],
  pub bytes_per_sector: u16,
  pub sectors_per_cluster: u8,
  pub reserved_sectors: u16,
  pub fats: u8,
  pub root_entries: u16,
  pub sectors: u16,
  pub media_type: u8,
  pub sectors_per_fat: u16,
  pub sectors_per_track: u16,
  pub heads: u16,
  pub hidden_sectors: u32,
  pub large_sectors: u32,
  pub unused: [u16; 2],
  pub number_of_sectors: u64,
  pub mft_cluster_location: u64,
  pub mft_mirror_cluster_location: u64,
  pub cluster_per_mft_record: i8,
  pub reserved1: [u8; 3],
  pub cluster_per_index_record: i8,
  pub reserved2: [u8; 3],
  pub volume_serial: [u8; 8],
  pub checksum: u16,
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
#[no_mangle]
pub unsafe extern "C" fn volume_id_probe_ntfs(mut id: *mut volume_id) -> libc::c_int
/*,u64 off*/ {
  let mut sector_size: libc::c_uint = 0;
  let mut cluster_size: libc::c_uint = 0;
  let mut mft_cluster: u64 = 0;
  let mut mft_off: u64 = 0;
  let mut mft_record_size: libc::c_uint = 0;
  let mut attr_type: libc::c_uint = 0;
  let mut attr_off: libc::c_uint = 0;
  let mut attr_len: libc::c_uint = 0;
  let mut val_off: libc::c_uint = 0;
  let mut val_len: libc::c_uint = 0;
  let mut mftr: *mut master_file_table_record = std::ptr::null_mut();
  let mut ns: *mut ntfs_super_block = std::ptr::null_mut();
  let mut buf: *const u8 = 0 as *const u8;
  let mut val: *const u8 = 0 as *const u8;
  ns = crate::util_linux::volume_id::util::volume_id_get_buffer(id, 0i32 as u64, 0x200i32 as size_t)
    as *mut ntfs_super_block;
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
  crate::util_linux::volume_id::util::volume_id_set_uuid(
    id,
    (*ns).volume_serial.as_mut_ptr(),
    UUID_NTFS,
  );
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
  buf = crate::util_linux::volume_id::util::volume_id_get_buffer(
    id,
    (0i32 as u64)
      .wrapping_add(mft_off)
      .wrapping_add((3i32 as libc::c_uint).wrapping_mul(mft_record_size) as libc::c_ulong),
    mft_record_size as size_t,
  ) as *const u8;
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
        let mut attr: *mut file_attribute = std::ptr::null_mut();
        attr = &*buf.offset(attr_off as isize) as *const u8 as *mut file_attribute;
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
        //			info = (struct volume_info*) (((u8 *) attr) + val_off);
        //			snprintf(id->type_version, sizeof(id->type_version)-1,
        //				 "%u.%u", info->major_ver, info->minor_ver);
        //		}
        if attr_type == 0x60i32 as libc::c_uint {
          if val_len > 64i32 as libc::c_uint {
            val_len = 64i32 as libc::c_uint
          }
          val = (attr as *mut u8).offset(val_off as isize);
          //			volume_id_set_label_raw(id, val, val_len);
          crate::util_linux::volume_id::util::volume_id_set_label_unicode16(
            id,
            val,
            LE,
            val_len as size_t,
          );
        }
      }
    }
  }
  //	volume_id_set_usage(id, VOLUME_ID_FILESYSTEM);
  (*id).type_0 = b"ntfs\x00" as *const u8 as *const libc::c_char;
  return 0i32;
}

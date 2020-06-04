use libc;

use crate::librb::size_t;

use crate::util_linux::volume_id::volume_id::volume_id;
/* V3 minix super-block data on disk */

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct minix3_super_block {
  pub s_ninodes: u32,
  pub s_pad0: u16,
  pub s_imap_blocks: u16,
  pub s_zmap_blocks: u16,
  pub s_firstdatazone: u16,
  pub s_log_zone_size: u16,
  pub s_pad1: u16,
  pub s_max_size: u32,
  pub s_zones: u32,
  pub s_magic: u16,
  pub s_pad2: u16,
  pub s_blocksize: u16,
  pub s_disk_version: u8,
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
//config:config FEATURE_VOLUMEID_MINIX
//config:	bool "minix filesystem"
//config:	default y
//config:	depends on VOLUMEID
//kbuild:lib-$(CONFIG_FEATURE_VOLUMEID_MINIX) += minix.o

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct minix_super_block {
  pub s_ninodes: u16,
  pub s_nzones: u16,
  pub s_imap_blocks: u16,
  pub s_zmap_blocks: u16,
  pub s_firstdatazone: u16,
  pub s_log_zone_size: u16,
  pub s_max_size: u32,
  pub s_magic: u16,
  pub s_state: u16,
  pub s_zones: u32,
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
pub unsafe fn volume_id_probe_minix(mut id: *mut volume_id) -> libc::c_int
/*, u64 off*/ {
  let mut ms: *mut minix_super_block = std::ptr::null_mut();
  let mut ms3: *mut minix3_super_block = std::ptr::null_mut();
  ms = crate::util_linux::volume_id::util::volume_id_get_buffer(
    id,
    (0i32 as u64).wrapping_add(0x400i32 as u64),
    0x200i32 as size_t,
  ) as *mut minix_super_block;
  if ms.is_null() {
    return -1i32;
  }
  if !((*ms).s_magic as libc::c_int == 0x137fi32 as u16 as libc::c_int) {
    if !((*ms).s_magic as libc::c_int == 0x138fi32 as u16 as libc::c_int) {
      if !((*ms).s_magic as libc::c_int == 0x2468i32 as u16 as libc::c_int) {
        if !((*ms).s_magic as libc::c_int == 0x2478i32 as u16 as libc::c_int) {
          ms3 = ms as *mut libc::c_void as *mut minix3_super_block;
          if !((*ms3).s_magic as libc::c_int == 0x4d5ai32 as u16 as libc::c_int) {
            return -1i32;
          }
        }
      }
    }
  }
  /* minix V3 fs, 60 char names */
  //	volume_id_set_usage(id, VOLUME_ID_FILESYSTEM);
  (*id).type_0 = b"minix\x00" as *const u8 as *const libc::c_char;
  return 0;
}

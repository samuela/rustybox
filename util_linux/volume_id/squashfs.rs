

use libc;
extern "C" {
  #[no_mangle]
  fn volume_id_get_buffer(id: *mut volume_id, off_0: u64, len: size_t) -> *mut libc::c_void;
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
/*
 * volume_id - reads filesystem label and uuid
 *
 * Copyright (C) 2012 S-G Bergh <sgb@systemasis.org>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config FEATURE_VOLUMEID_SQUASHFS
//config:	bool "SquashFS filesystem"
//config:	default y
//config:	depends on VOLUMEID && FEATURE_BLKID_TYPE
//config:	help
//config:	Squashfs is a compressed read-only filesystem for Linux. Squashfs is
//config:	intended for general read-only filesystem use and in constrained block
//config:	device/memory systems (e.g. embedded systems) where low overhead is
//config:	needed.
//kbuild:lib-$(CONFIG_FEATURE_VOLUMEID_SQUASHFS) += squashfs.o
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct squashfs_superblock {
  pub magic: u32,
  /*
    u32	dummy[6];
    u16        major;
    u16        minor;
  */
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
/*,u64 off*/
/*,u64 off*/
/*,u64 off*/
#[no_mangle]
pub unsafe extern "C" fn volume_id_probe_squashfs(mut id: *mut volume_id) -> libc::c_int
/*,u64 off*/ {
  let mut sb: *mut squashfs_superblock = 0 as *mut squashfs_superblock;
  sb = volume_id_get_buffer(id, 0i32 as u64, 0x200i32 as size_t) as *mut squashfs_superblock;
  if sb.is_null() {
    return -1i32;
  }
  // Old SquashFS (pre 4.0) can be both big and little endian, so test for both.
  // Likewise, it is commonly used in firwmare with some non-standard signatures.
  if (*sb).magic
    == ((('s' as i32 * 256i32 + 'q' as i32) * 256i32 + 's' as i32) as u32)
      .wrapping_mul(256i32 as libc::c_uint)
      .wrapping_add('h' as i32 as libc::c_uint)
    || (*sb).magic
      == ((('h' as i32 * 256i32 + 's' as i32) * 256i32 + 'q' as i32) as u32)
        .wrapping_mul(256i32 as libc::c_uint)
        .wrapping_add('s' as i32 as libc::c_uint)
    || (*sb).magic
      == ((('s' as i32 * 256i32 + 'h' as i32) * 256i32 + 's' as i32) as u32)
        .wrapping_mul(256i32 as libc::c_uint)
        .wrapping_add('q' as i32 as libc::c_uint)
    || (*sb).magic
      == ((('q' as i32 * 256i32 + 's' as i32) * 256i32 + 'h' as i32) as u32)
        .wrapping_mul(256i32 as libc::c_uint)
        .wrapping_add('s' as i32 as libc::c_uint)
  {
    (*id).type_0 = b"squashfs\x00" as *const u8 as *const libc::c_char;
    return 0i32;
  }
  return -1i32;
}

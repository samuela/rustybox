use libc;
extern "C" {

  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}

use crate::librb::size_t;

use crate::util_linux::volume_id::volume_id::volume_id;

pub type uuid_format = libc::c_uint;
// pub const UUID_DCE_STRING: uuid_format = 3;
pub const UUID_DCE: uuid_format = 2;
// pub const UUID_NTFS: uuid_format = 1;
// pub const UUID_DOS: uuid_format = 0;

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
//config:config FEATURE_VOLUMEID_LINUXRAID
//config:	bool "linuxraid"
//config:	default y
//config:	depends on VOLUMEID
//kbuild:lib-$(CONFIG_FEATURE_VOLUMEID_LINUXRAID) += linux_raid.o

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct mdp_super_block {
  pub md_magic: u32,
  pub major_version: u32,
  pub minor_version: u32,
  pub patch_version: u32,
  pub gvalid_words: u32,
  pub set_uuid0: u32,
  pub ctime: u32,
  pub level: u32,
  pub size: u32,
  pub nr_disks: u32,
  pub raid_disks: u32,
  pub md_minor: u32,
  pub not_persistent: u32,
  pub set_uuid1: u32,
  pub set_uuid2: u32,
  pub set_uuid3: u32,
}
pub type aliased_u32 = u32;
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
#[no_mangle]
pub unsafe extern "C" fn volume_id_probe_linux_raid(
  mut id: *mut volume_id,
  mut size: u64,
) -> libc::c_int {
  let mut sboff: u64 = 0;
  let mut uuid: [u8; 16] = [0; 16];
  let mut mdp: *mut mdp_super_block = std::ptr::null_mut();
  if size < 0x10000i32 as libc::c_ulong {
    return -1i32;
  }
  sboff = (size & !(0x10000i32 - 1i32) as libc::c_ulong).wrapping_sub(0x10000i32 as libc::c_ulong);
  mdp = crate::util_linux::volume_id::util::volume_id_get_buffer(
    id,
    (0i32 as u64).wrapping_add(sboff),
    0x800i32 as size_t,
  ) as *mut mdp_super_block;
  if mdp.is_null() {
    return -1i32;
  }
  if (*mdp).md_magic != 0xa92b4efcu32 {
    return -1i32;
  }
  *(uuid.as_mut_ptr() as *mut aliased_u32) = (*mdp).set_uuid0;
  memcpy(
    &mut *uuid.as_mut_ptr().offset(4) as *mut u8 as *mut libc::c_void,
    &mut (*mdp).set_uuid1 as *mut u32 as *const libc::c_void,
    12i32 as libc::c_ulong,
  );
  crate::util_linux::volume_id::util::volume_id_set_uuid(id, uuid.as_mut_ptr(), UUID_DCE);
  //	snprintf(id->type_version, sizeof(id->type_version)-1, "%u.%u.%u",
  //		le32_to_cpu(mdp->major_version),
  //		le32_to_cpu(mdp->minor_version),
  //		le32_to_cpu(mdp->patch_version));
  //	volume_id_set_usage(id, VOLUME_ID_RAID);
  (*id).type_0 = b"linux_raid_member\x00" as *const u8 as *const libc::c_char;
  return 0i32;
}

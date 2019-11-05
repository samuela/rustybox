use libc;

extern "C" {
  #[no_mangle]
  fn volume_id_get_buffer(id: *mut volume_id, off_0: uint64_t, len: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn volume_id_set_uuid(id: *mut volume_id, buf: *const uint8_t, format: uuid_format);

  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}

use crate::librb::__uint8_t;
use crate::librb::__uint32_t;
pub type __uint64_t = libc::c_ulong;
use crate::librb::uint8_t;
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
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct mdp_super_block {
  pub md_magic: uint32_t,
  pub major_version: uint32_t,
  pub minor_version: uint32_t,
  pub patch_version: uint32_t,
  pub gvalid_words: uint32_t,
  pub set_uuid0: uint32_t,
  pub ctime: uint32_t,
  pub level: uint32_t,
  pub size: uint32_t,
  pub nr_disks: uint32_t,
  pub raid_disks: uint32_t,
  pub md_minor: uint32_t,
  pub not_persistent: uint32_t,
  pub set_uuid1: uint32_t,
  pub set_uuid2: uint32_t,
  pub set_uuid3: uint32_t,
}
pub type aliased_uint32_t = uint32_t;
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
#[no_mangle]
pub unsafe extern "C" fn volume_id_probe_linux_raid(
  mut id: *mut volume_id,
  mut size: uint64_t,
) -> libc::c_int {
  let mut sboff: uint64_t = 0;
  let mut uuid: [uint8_t; 16] = [0; 16];
  let mut mdp: *mut mdp_super_block = 0 as *mut mdp_super_block;
  if size < 0x10000i32 as libc::c_ulong {
    return -1i32;
  }
  sboff = (size & !(0x10000i32 - 1i32) as libc::c_ulong).wrapping_sub(0x10000i32 as libc::c_ulong);
  mdp = volume_id_get_buffer(
    id,
    (0i32 as uint64_t).wrapping_add(sboff),
    0x800i32 as size_t,
  ) as *mut mdp_super_block;
  if mdp.is_null() {
    return -1i32;
  }
  if (*mdp).md_magic != 0xa92b4efcu32 {
    return -1i32;
  }
  *(uuid.as_mut_ptr() as *mut aliased_uint32_t) = (*mdp).set_uuid0;
  memcpy(
    &mut *uuid.as_mut_ptr().offset(4) as *mut uint8_t as *mut libc::c_void,
    &mut (*mdp).set_uuid1 as *mut uint32_t as *const libc::c_void,
    12i32 as libc::c_ulong,
  );
  volume_id_set_uuid(id, uuid.as_mut_ptr(), UUID_DCE);
  //	snprintf(id->type_version, sizeof(id->type_version)-1, "%u.%u.%u",
  //		le32_to_cpu(mdp->major_version),
  //		le32_to_cpu(mdp->minor_version),
  //		le32_to_cpu(mdp->patch_version));
  //	volume_id_set_usage(id, VOLUME_ID_RAID);
  (*id).type_0 = b"linux_raid_member\x00" as *const u8 as *const libc::c_char;
  return 0i32;
}

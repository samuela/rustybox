use libc;
extern "C" {
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;

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
 * Copyright (C) 2005 Tobias Klauser <tklauser@access.unizh.ch>
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
//config:config FEATURE_VOLUMEID_REISERFS
//config:	bool "Reiser filesystem"
//config:	default y
//config:	depends on VOLUMEID
//kbuild:lib-$(CONFIG_FEATURE_VOLUMEID_REISERFS) += reiserfs.o

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct reiserfs_super_block {
  pub blocks_count: u32,
  pub free_blocks: u32,
  pub root_block: u32,
  pub journal_block: u32,
  pub journal_dev: u32,
  pub orig_journal_size: u32,
  pub dummy2: [u32; 5],
  pub blocksize: u16,
  pub dummy3: [u16; 3],
  pub magic: [u8; 12],
  pub dummy4: [u32; 5],
  pub uuid: [u8; 16],
  pub label: [u8; 16],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct reiser4_super_block {
  pub magic: [u8; 16],
  pub dummy: [u16; 2],
  pub uuid: [u8; 16],
  pub label: [u8; 16],
  pub dummy2: u64,
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
pub unsafe fn volume_id_probe_reiserfs(mut id: *mut volume_id) -> libc::c_int
/*,u64 off*/ {
  let mut current_block: u64;
  let mut rs: *mut reiserfs_super_block = std::ptr::null_mut();
  let mut rs4: *mut reiser4_super_block = std::ptr::null_mut();
  rs = crate::util_linux::volume_id::util::volume_id_get_buffer(
    id,
    (0i32 as u64).wrapping_add(0x10000i32 as u64),
    0x200i32 as size_t,
  ) as *mut reiserfs_super_block;
  if rs.is_null() {
    return -1i32;
  }
  if !(memcmp(
    (*rs).magic.as_mut_ptr() as *const libc::c_void,
    b"ReIsErFs\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
    8i32 as libc::c_ulong,
  ) == 0)
  {
    if memcmp(
      (*rs).magic.as_mut_ptr() as *const libc::c_void,
      b"ReIsEr2Fs\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
      9i32 as libc::c_ulong,
    ) == 0
    {
      //		strcpy(id->type_version, "3.6");
      current_block = 14461063366507817781;
    } else if memcmp(
      (*rs).magic.as_mut_ptr() as *const libc::c_void,
      b"ReIsEr3Fs\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
      9i32 as libc::c_ulong,
    ) == 0
    {
      current_block = 14461063366507817781;
    } else {
      rs4 = rs as *mut reiser4_super_block;
      if memcmp(
        (*rs4).magic.as_mut_ptr() as *const libc::c_void,
        b"ReIsEr4\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        7i32 as libc::c_ulong,
      ) == 0
      {
        //		strcpy(id->type_version, "4");
        //		volume_id_set_label_raw(id, rs4->label, 16);
        crate::util_linux::volume_id::util::volume_id_set_label_string(
          id,
          (*rs4).label.as_mut_ptr(),
          16i32 as size_t,
        );
        crate::util_linux::volume_id::util::volume_id_set_uuid(
          id,
          (*rs4).uuid.as_mut_ptr(),
          UUID_DCE,
        );
      } else {
        rs = crate::util_linux::volume_id::util::volume_id_get_buffer(
          id,
          (0i32 as u64).wrapping_add(0x2000i32 as u64),
          0x200i32 as size_t,
        ) as *mut reiserfs_super_block;
        if rs.is_null() {
          return -1i32;
        }
        if !(memcmp(
          (*rs).magic.as_mut_ptr() as *const libc::c_void,
          b"ReIsErFs\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
          8i32 as libc::c_ulong,
        ) == 0)
        {
          return -1i32;
        }
      }
      current_block = 12579958577185958249;
    }
    match current_block {
      12579958577185958249 => {}
      _ =>
      //		strcpy(id->type_version, "JR");
      //	volume_id_set_label_raw(id, rs->label, 16);
      {
        crate::util_linux::volume_id::util::volume_id_set_label_string(
          id,
          (*rs).label.as_mut_ptr(),
          16i32 as size_t,
        );
        crate::util_linux::volume_id::util::volume_id_set_uuid(
          id,
          (*rs).uuid.as_mut_ptr(),
          UUID_DCE,
        );
      }
    }
  }
  //		strcpy(id->type_version, "3.5");
  //	volume_id_set_usage(id, VOLUME_ID_FILESYSTEM);
  (*id).type_0 = b"reiserfs\x00" as *const u8 as *const libc::c_char;
  return 0;
}

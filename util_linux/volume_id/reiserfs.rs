use libc;

extern "C" {
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;

  #[no_mangle]
  fn volume_id_set_label_string(id: *mut volume_id, buf: *const uint8_t, count: size_t);

  #[no_mangle]
  fn volume_id_set_uuid(id: *mut volume_id, buf: *const uint8_t, format: uuid_format);

  #[no_mangle]
  fn volume_id_get_buffer(id: *mut volume_id, off_0: uint64_t, len: size_t) -> *mut libc::c_void;
}

pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct reiserfs_super_block {
  pub blocks_count: uint32_t,
  pub free_blocks: uint32_t,
  pub root_block: uint32_t,
  pub journal_block: uint32_t,
  pub journal_dev: uint32_t,
  pub orig_journal_size: uint32_t,
  pub dummy2: [uint32_t; 5],
  pub blocksize: uint16_t,
  pub dummy3: [uint16_t; 3],
  pub magic: [uint8_t; 12],
  pub dummy4: [uint32_t; 5],
  pub uuid: [uint8_t; 16],
  pub label: [uint8_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct reiser4_super_block {
  pub magic: [uint8_t; 16],
  pub dummy: [uint16_t; 2],
  pub uuid: [uint8_t; 16],
  pub label: [uint8_t; 16],
  pub dummy2: uint64_t,
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
/*,uint64_t off*/
/*,uint64_t off*/
/*,uint64_t off*/
#[no_mangle]
pub unsafe extern "C" fn volume_id_probe_reiserfs(mut id: *mut volume_id) -> libc::c_int
/*,uint64_t off*/ {
  let mut current_block: u64;
  let mut rs: *mut reiserfs_super_block = 0 as *mut reiserfs_super_block;
  let mut rs4: *mut reiser4_super_block = 0 as *mut reiser4_super_block;
  rs = volume_id_get_buffer(
    id,
    (0i32 as uint64_t).wrapping_add(0x10000i32 as libc::c_ulong),
    0x200i32 as size_t,
  ) as *mut reiserfs_super_block;
  if rs.is_null() {
    return -1i32;
  }
  if !(memcmp(
    (*rs).magic.as_mut_ptr() as *const libc::c_void,
    b"ReIsErFs\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
    8i32 as libc::c_ulong,
  ) == 0i32)
  {
    if memcmp(
      (*rs).magic.as_mut_ptr() as *const libc::c_void,
      b"ReIsEr2Fs\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
      9i32 as libc::c_ulong,
    ) == 0i32
    {
      //		strcpy(id->type_version, "3.6");
      current_block = 14461063366507817781;
    } else if memcmp(
      (*rs).magic.as_mut_ptr() as *const libc::c_void,
      b"ReIsEr3Fs\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
      9i32 as libc::c_ulong,
    ) == 0i32
    {
      current_block = 14461063366507817781;
    } else {
      rs4 = rs as *mut reiser4_super_block;
      if memcmp(
        (*rs4).magic.as_mut_ptr() as *const libc::c_void,
        b"ReIsEr4\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        7i32 as libc::c_ulong,
      ) == 0i32
      {
        //		strcpy(id->type_version, "4");
        //		volume_id_set_label_raw(id, rs4->label, 16);
        volume_id_set_label_string(id, (*rs4).label.as_mut_ptr(), 16i32 as size_t);
        volume_id_set_uuid(id, (*rs4).uuid.as_mut_ptr(), UUID_DCE);
      } else {
        rs = volume_id_get_buffer(
          id,
          (0i32 as uint64_t).wrapping_add(0x2000i32 as libc::c_ulong),
          0x200i32 as size_t,
        ) as *mut reiserfs_super_block;
        if rs.is_null() {
          return -1i32;
        }
        if !(memcmp(
          (*rs).magic.as_mut_ptr() as *const libc::c_void,
          b"ReIsErFs\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
          8i32 as libc::c_ulong,
        ) == 0i32)
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
        volume_id_set_label_string(id, (*rs).label.as_mut_ptr(), 16i32 as size_t);
        volume_id_set_uuid(id, (*rs).uuid.as_mut_ptr(), UUID_DCE);
      }
    }
  }
  //		strcpy(id->type_version, "3.5");
  //	volume_id_set_usage(id, VOLUME_ID_FILESYSTEM);
  (*id).type_0 = b"reiserfs\x00" as *const u8 as *const libc::c_char;
  return 0i32;
}

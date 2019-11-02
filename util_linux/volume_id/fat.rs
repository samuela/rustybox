use libc;
extern "C" {
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn volume_id_get_buffer(id: *mut volume_id, off: uint64_t, len: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn volume_id_set_uuid(id: *mut volume_id, buf: *const uint8_t, format: uuid_format);
  #[no_mangle]
  fn volume_id_set_label_string(id: *mut volume_id, buf: *const uint8_t, count: size_t);
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
pub const UUID_DCE_STRING: uuid_format = 3;
pub const UUID_DCE: uuid_format = 2;
pub const UUID_NTFS: uuid_format = 1;
pub const UUID_DOS: uuid_format = 0;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct fat32_super_block {
  pub fat32_length: uint32_t,
  pub flags: uint16_t,
  pub version: [uint8_t; 2],
  pub root_cluster: uint32_t,
  pub insfo_sector: uint16_t,
  pub backup_boot: uint16_t,
  pub reserved2: [uint16_t; 6],
  pub unknown: [uint8_t; 3],
  pub serno: [uint8_t; 4],
  pub label: [uint8_t; 11],
  pub magic: [uint8_t; 8],
  pub dummy2: [uint8_t; 164],
  pub pmagic: [uint8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub fat: fat_super_block,
  pub fat32: fat32_super_block,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct fat_super_block {
  pub unknown: [uint8_t; 3],
  pub serno: [uint8_t; 4],
  pub label: [uint8_t; 11],
  pub magic: [uint8_t; 8],
  pub dummy2: [uint8_t; 192],
  pub pmagic: [uint8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct vfat_super_block {
  pub boot_jump: [uint8_t; 3],
  pub sysid: [uint8_t; 8],
  pub sector_size_bytes: uint16_t,
  pub sectors_per_cluster: uint8_t,
  pub reserved_sct: uint16_t,
  pub fats: uint8_t,
  pub dir_entries: uint16_t,
  pub sectors: uint16_t,
  pub media: uint8_t,
  pub fat_length: uint16_t,
  pub secs_track: uint16_t,
  pub heads: uint16_t,
  pub hidden: uint32_t,
  pub total_sect: uint32_t,
  pub type_0: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct vfat_dir_entry {
  pub name: [uint8_t; 11],
  pub attr: uint8_t,
  pub time_creat: uint16_t,
  pub date_creat: uint16_t,
  pub time_acc: uint16_t,
  pub date_acc: uint16_t,
  pub cluster_high: uint16_t,
  pub time_write: uint16_t,
  pub date_write: uint16_t,
  pub cluster_low: uint16_t,
  pub size: uint32_t,
}
unsafe extern "C" fn get_attr_volume_id(
  mut dir: *mut vfat_dir_entry,
  mut count: libc::c_int,
) -> *mut uint8_t {
  loop {
    count -= 1;
    if !(count >= 0i32) {
      break;
    }
    /* end marker */
    if (*dir).name[0] as libc::c_int == 0i32 {
      break;
    }
    /* empty entry */
    if !((*dir).name[0] as libc::c_int == 0xe5i32) {
      /* long name */
      if !((*dir).attr as libc::c_int & 0x3fi32 == 0xfi32) {
        if (*dir).attr as libc::c_int & (0x8i32 | 0x10i32) == 0x8i32 {
          /* labels do not have file data */
          if !((*dir).cluster_high as libc::c_int != 0i32
            || (*dir).cluster_low as libc::c_int != 0i32)
          {
            return (*dir).name.as_mut_ptr();
          }
        }
      }
    }
    dir = dir.offset(1)
  }
  return 0 as *mut uint8_t;
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
#[no_mangle]
pub unsafe extern "C" fn volume_id_probe_vfat(mut id: *mut volume_id) -> libc::c_int
/*,uint64_t fat_partition_off*/ {
  let mut current_block: u64;
  let mut vs: *mut vfat_super_block = 0 as *mut vfat_super_block;
  let mut dir: *mut vfat_dir_entry = 0 as *mut vfat_dir_entry;
  let mut sector_size_bytes: uint16_t = 0;
  let mut dir_entries: uint16_t = 0;
  let mut sect_count: uint32_t = 0;
  let mut reserved_sct: uint16_t = 0;
  let mut fat_size_sct: uint32_t = 0;
  let mut root_cluster: uint32_t = 0;
  let mut dir_size_sct: uint32_t = 0;
  let mut cluster_count: uint32_t = 0;
  let mut root_start_off: uint64_t = 0;
  let mut start_data_sct: uint32_t = 0;
  let mut buf: *mut uint8_t = 0 as *mut uint8_t;
  let mut buf_size: uint32_t = 0;
  let mut label: *mut uint8_t = 0 as *mut uint8_t;
  let mut next_cluster: uint32_t = 0;
  let mut maxloop: libc::c_int = 0;
  vs = volume_id_get_buffer(id, 0i32 as uint64_t, 0x200i32 as size_t) as *mut vfat_super_block;
  if vs.is_null() {
    return -1i32;
  }
  /* believe only that's fat, don't trust the version
   * the cluster_count will tell us
   */
  if memcmp(
    (*vs).sysid.as_mut_ptr() as *const libc::c_void,
    b"NTFS\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
    4i32 as libc::c_ulong,
  ) == 0i32
  {
    return -1i32;
  }
  if !(memcmp(
    (*vs).type_0.fat32.magic.as_mut_ptr() as *const libc::c_void,
    b"MSWIN\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
    5i32 as libc::c_ulong,
  ) == 0i32)
  {
    if !(memcmp(
      (*vs).type_0.fat32.magic.as_mut_ptr() as *const libc::c_void,
      b"FAT32   \x00" as *const u8 as *const libc::c_char as *const libc::c_void,
      8i32 as libc::c_ulong,
    ) == 0i32)
    {
      if !(memcmp(
        (*vs).type_0.fat.magic.as_mut_ptr() as *const libc::c_void,
        b"FAT16   \x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        8i32 as libc::c_ulong,
      ) == 0i32)
      {
        if !(memcmp(
          (*vs).type_0.fat.magic.as_mut_ptr() as *const libc::c_void,
          b"MSDOS\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
          5i32 as libc::c_ulong,
        ) == 0i32)
        {
          if !(memcmp(
            (*vs).type_0.fat.magic.as_mut_ptr() as *const libc::c_void,
            b"FAT12   \x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            8i32 as libc::c_ulong,
          ) == 0i32)
          {
            /*
             * There are old floppies out there without a magic, so we check
             * for well known values and guess if it's a fat volume
             */
            /* boot jump address check */
            if ((*vs).boot_jump[0] as libc::c_int != 0xebi32
              || (*vs).boot_jump[2] as libc::c_int != 0x90i32)
              && (*vs).boot_jump[0] as libc::c_int != 0xe9i32
            {
              return -1i32;
            }
            /* heads check */
            if (*vs).heads as libc::c_int == 0i32 {
              return -1i32;
            }
            /* cluster size check */
            if (*vs).sectors_per_cluster as libc::c_int == 0i32
              || (*vs).sectors_per_cluster as libc::c_int
                & (*vs).sectors_per_cluster as libc::c_int - 1i32
                != 0
            {
              return -1i32;
            }
            /* media check */
            if ((*vs).media as libc::c_int) < 0xf8i32 && (*vs).media as libc::c_int != 0xf0i32 {
              return -1i32;
            }
            /* fat count*/
            if (*vs).fats as libc::c_int != 2i32 {
              return -1i32;
            }
          }
        }
      }
    }
  }
  /* sector size check */
  sector_size_bytes = (*vs).sector_size_bytes;
  if sector_size_bytes as libc::c_int != 0x200i32
    && sector_size_bytes as libc::c_int != 0x400i32
    && sector_size_bytes as libc::c_int != 0x800i32
    && sector_size_bytes as libc::c_int != 0x1000i32
  {
    return -1i32;
  }
  reserved_sct = (*vs).reserved_sct;
  sect_count = (*vs).sectors as uint32_t;
  if sect_count == 0i32 as libc::c_uint {
    sect_count = (*vs).total_sect
  }
  fat_size_sct = (*vs).fat_length as uint32_t;
  if fat_size_sct == 0i32 as libc::c_uint {
    fat_size_sct = (*vs).type_0.fat32.fat32_length
  }
  fat_size_sct =
    (fat_size_sct as libc::c_uint).wrapping_mul((*vs).fats as libc::c_uint) as uint32_t as uint32_t;
  dir_entries = (*vs).dir_entries;
  dir_size_sct = (dir_entries as libc::c_ulong)
    .wrapping_mul(::std::mem::size_of::<vfat_dir_entry>() as libc::c_ulong)
    .wrapping_add((sector_size_bytes as libc::c_int - 1i32) as libc::c_ulong)
    .wrapping_div(sector_size_bytes as libc::c_ulong) as uint32_t;
  cluster_count = sect_count.wrapping_sub(
    (reserved_sct as libc::c_uint)
      .wrapping_add(fat_size_sct)
      .wrapping_add(dir_size_sct),
  );
  cluster_count = (cluster_count as libc::c_uint)
    .wrapping_div((*vs).sectors_per_cluster as libc::c_uint) as uint32_t
    as uint32_t;
  //	if (cluster_count < FAT12_MAX) {
  //		strcpy(id->type_version, "FAT12");
  //	} else if (cluster_count < FAT16_MAX) {
  //		strcpy(id->type_version, "FAT16");
  //	} else {
  //		strcpy(id->type_version, "FAT32");
  //		goto fat32;
  //	}
  if cluster_count > 0xfff4i32 as libc::c_uint {
    /* FAT32 root dir is a cluster chain like any other directory */
    buf_size =
      ((*vs).sectors_per_cluster as libc::c_int * sector_size_bytes as libc::c_int) as uint32_t;
    root_cluster = (*vs).type_0.fat32.root_cluster;
    start_data_sct = (reserved_sct as libc::c_uint).wrapping_add(fat_size_sct);
    next_cluster = root_cluster;
    maxloop = 100i32;
    loop {
      maxloop -= 1;
      if !(maxloop != 0) {
        current_block = 11869735117417356968;
        break;
      }
      let mut next_off_sct: uint64_t = 0;
      let mut next_off: uint64_t = 0;
      let mut fat_entry_off: uint64_t = 0;
      let mut count: libc::c_int = 0;
      next_off_sct = (next_cluster.wrapping_sub(2i32 as libc::c_uint) as uint64_t)
        .wrapping_mul((*vs).sectors_per_cluster as libc::c_ulong);
      next_off = (start_data_sct as libc::c_ulong)
        .wrapping_add(next_off_sct)
        .wrapping_mul(sector_size_bytes as libc::c_ulong);
      /* get cluster */
      buf = volume_id_get_buffer(
        id,
        (0i32 as uint64_t).wrapping_add(next_off),
        buf_size as size_t,
      ) as *mut uint8_t;
      if buf.is_null() {
        current_block = 7641562720398393250;
        break;
      }
      dir = buf as *mut vfat_dir_entry;
      count = (buf_size as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<vfat_dir_entry>() as libc::c_ulong)
        as libc::c_int;
      label = get_attr_volume_id(dir, count);
      if !label.is_null() {
        current_block = 11869735117417356968;
        break;
      }
      /* get FAT entry */
      fat_entry_off = ((reserved_sct as libc::c_int * sector_size_bytes as libc::c_int)
        as libc::c_ulong)
        .wrapping_add(
          (next_cluster as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        );
      buf = volume_id_get_buffer(
        id,
        (0i32 as uint64_t).wrapping_add(fat_entry_off),
        buf_size as size_t,
      ) as *mut uint8_t;
      if buf.is_null() {
        current_block = 7641562720398393250;
        break;
      }
      /* set next cluster */
      next_cluster = *(buf as *mut uint32_t) & 0xfffffffi32 as libc::c_uint;
      if next_cluster < 2i32 as libc::c_uint || next_cluster > 0xffffff6i32 as libc::c_uint {
        current_block = 11869735117417356968;
        break;
      }
    }
    match current_block {
      7641562720398393250 => {}
      _ => {
        // TODO: why was this translated this way?
        // (maxloop) == 0i32;
        vs =
          volume_id_get_buffer(id, 0i32 as uint64_t, 0x200i32 as size_t) as *mut vfat_super_block;
        if vs.is_null() {
          return -1i32;
        }
        if !label.is_null()
          && memcmp(
            label as *const libc::c_void,
            b"NO NAME    \x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            11i32 as libc::c_ulong,
          ) != 0i32
        {
          //		volume_id_set_label_raw(id, label, 11);
          volume_id_set_label_string(id, label, 11i32 as size_t);
        } else if memcmp(
          (*vs).type_0.fat32.label.as_mut_ptr() as *const libc::c_void,
          b"NO NAME    \x00" as *const u8 as *const libc::c_char as *const libc::c_void,
          11i32 as libc::c_ulong,
        ) != 0i32
        {
          //		volume_id_set_label_raw(id, vs->type.fat32.label, 11);
          volume_id_set_label_string(id, (*vs).type_0.fat32.label.as_mut_ptr(), 11i32 as size_t);
        }
        volume_id_set_uuid(id, (*vs).type_0.fat32.serno.as_mut_ptr(), UUID_DOS);
      }
    }
  } else {
    /* the label may be an attribute in the root directory */
    root_start_off = (reserved_sct as libc::c_uint)
      .wrapping_add(fat_size_sct)
      .wrapping_mul(sector_size_bytes as libc::c_uint) as uint64_t;
    buf_size = (dir_entries as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<vfat_dir_entry>() as libc::c_ulong)
      as uint32_t;
    buf = volume_id_get_buffer(
      id,
      (0i32 as uint64_t).wrapping_add(root_start_off),
      buf_size as size_t,
    ) as *mut uint8_t;
    if !buf.is_null() {
      label = get_attr_volume_id(buf as *mut vfat_dir_entry, dir_entries as libc::c_int);
      vs = volume_id_get_buffer(id, 0i32 as uint64_t, 0x200i32 as size_t) as *mut vfat_super_block;
      if vs.is_null() {
        return -1i32;
      }
      if !label.is_null()
        && memcmp(
          label as *const libc::c_void,
          b"NO NAME    \x00" as *const u8 as *const libc::c_char as *const libc::c_void,
          11i32 as libc::c_ulong,
        ) != 0i32
      {
        //		volume_id_set_label_raw(id, label, 11);
        volume_id_set_label_string(id, label, 11i32 as size_t);
      } else if memcmp(
        (*vs).type_0.fat.label.as_mut_ptr() as *const libc::c_void,
        b"NO NAME    \x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        11i32 as libc::c_ulong,
      ) != 0i32
      {
        //		volume_id_set_label_raw(id, vs->type.fat.label, 11);
        volume_id_set_label_string(id, (*vs).type_0.fat.label.as_mut_ptr(), 11i32 as size_t);
      }
      volume_id_set_uuid(id, (*vs).type_0.fat.serno.as_mut_ptr(), UUID_DOS);
    }
  }
  //	volume_id_set_usage(id, VOLUME_ID_FILESYSTEM);
  (*id).type_0 = b"vfat\x00" as *const u8 as *const libc::c_char;
  return 0i32;
}

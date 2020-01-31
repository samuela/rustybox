use crate::librb::size_t;
use crate::util_linux::volume_id::volume_id::volume_id;
use libc;
extern "C" {
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
}

pub type endian = libc::c_uint;
pub const BE: endian = 1;
// pub const LE: endian = 0;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct dstring {
  pub clen: u8,
  pub c: [u8; 31],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct primary_descriptor {
  pub seq_num: u32,
  pub desc_num: u32,
  pub ident: dstring,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed {
  pub anchor: anchor_descriptor,
  pub primary: primary_descriptor,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct anchor_descriptor {
  pub length: u32,
  pub location: u32,
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
//config:config FEATURE_VOLUMEID_UDF
//config:	bool "udf filesystem"
//config:	default y
//config:	depends on VOLUMEID
//kbuild:lib-$(CONFIG_FEATURE_VOLUMEID_UDF) += udf.o

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct volume_descriptor {
  pub tag: descriptor_tag,
  pub type_0: C2RustUnnamed,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct descriptor_tag {
  pub id: u16,
  pub version: u16,
  pub checksum: u8,
  pub reserved: u8,
  pub serial: u16,
  pub crc: u16,
  pub crc_len: u16,
  pub location: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct volume_structure_descriptor {
  pub type_0: u8,
  pub id: [u8; 5],
  pub version: u8,
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
/*,u64 off*/
/*,u64 off*/
pub unsafe fn volume_id_probe_udf(mut id: *mut volume_id) -> libc::c_int
/*,u64 off*/ {
  let mut current_block: u64;
  let mut vd: *mut volume_descriptor = std::ptr::null_mut();
  let mut vsd: *mut volume_structure_descriptor = std::ptr::null_mut();
  let mut bs: libc::c_uint = 0;
  let mut b: libc::c_uint = 0;
  let mut type_0: libc::c_uint = 0;
  let mut count: libc::c_uint = 0;
  let mut loc: libc::c_uint = 0;
  let mut clen: libc::c_uint = 0;
  vsd = crate::util_linux::volume_id::util::volume_id_get_buffer(
    id,
    (0i32 as u64).wrapping_add(0x8000i32 as libc::c_ulong),
    0x200i32 as size_t,
  ) as *mut volume_structure_descriptor;
  if vsd.is_null() {
    return -1i32;
  }
  if !(memcmp(
    (*vsd).id.as_mut_ptr() as *const libc::c_void,
    b"NSR02\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
    5i32 as libc::c_ulong,
  ) == 0)
  {
    if !(memcmp(
      (*vsd).id.as_mut_ptr() as *const libc::c_void,
      b"NSR03\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
      5i32 as libc::c_ulong,
    ) == 0)
    {
      if !(memcmp(
        (*vsd).id.as_mut_ptr() as *const libc::c_void,
        b"BEA01\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        5i32 as libc::c_ulong,
      ) == 0)
      {
        if !(memcmp(
          (*vsd).id.as_mut_ptr() as *const libc::c_void,
          b"BOOT2\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
          5i32 as libc::c_ulong,
        ) == 0)
        {
          if !(memcmp(
            (*vsd).id.as_mut_ptr() as *const libc::c_void,
            b"CD001\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            5i32 as libc::c_ulong,
          ) == 0)
          {
            if !(memcmp(
              (*vsd).id.as_mut_ptr() as *const libc::c_void,
              b"CDW02\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
              5i32 as libc::c_ulong,
            ) == 0)
            {
              if !(memcmp(
                (*vsd).id.as_mut_ptr() as *const libc::c_void,
                b"TEA03\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                5i32 as libc::c_ulong,
              ) == 0)
              {
                return -1i32;
              }
            }
          }
        }
      }
    }
  }
  /* search the next VSD to get the logical block size of the volume */
  bs = 0x800i32 as libc::c_uint;
  loop {
    if !(bs < 0x8000i32 as libc::c_uint) {
      current_block = 13472856163611868459;
      break;
    }
    vsd = crate::util_linux::volume_id::util::volume_id_get_buffer(
      id,
      (0i32 as u64)
        .wrapping_add(0x8000i32 as libc::c_ulong)
        .wrapping_add(bs as libc::c_ulong),
      0x800i32 as size_t,
    ) as *mut volume_structure_descriptor;
    if vsd.is_null() {
      return -1i32;
    }
    if (*vsd).id[0] as libc::c_int != '\u{0}' as i32 {
      current_block = 16203760046146113240;
      break;
    }
    bs = bs.wrapping_add(0x800i32 as libc::c_uint)
  }
  match current_block {
    13472856163611868459 => return -1i32,
    _ =>
    /* search the list of VSDs for a NSR descriptor */
    {
      b = 0 as libc::c_uint;
      loop {
        if !(b < 64i32 as libc::c_uint) {
          current_block = 7245201122033322888;
          break;
        }
        vsd = crate::util_linux::volume_id::util::volume_id_get_buffer(
          id,
          (0i32 as u64)
            .wrapping_add(0x8000i32 as libc::c_ulong)
            .wrapping_add(b.wrapping_mul(bs) as libc::c_ulong),
          0x800i32 as size_t,
        ) as *mut volume_structure_descriptor;
        if vsd.is_null() {
          return -1i32;
        }
        if (*vsd).id[0] as libc::c_int == '\u{0}' as i32 {
          return -1i32;
        }
        if memcmp(
          (*vsd).id.as_mut_ptr() as *const libc::c_void,
          b"NSR02\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
          5i32 as libc::c_ulong,
        ) == 0
        {
          current_block = 1900569641276003734;
          break;
        }
        if memcmp(
          (*vsd).id.as_mut_ptr() as *const libc::c_void,
          b"NSR03\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
          5i32 as libc::c_ulong,
        ) == 0
        {
          current_block = 1900569641276003734;
          break;
        }
        b = b.wrapping_add(1)
      }
      match current_block {
        7245201122033322888 => return -1i32,
        _ => {
          /* read anchor volume descriptor */
          vd = crate::util_linux::volume_id::util::volume_id_get_buffer(
            id,
            (0i32 as u64).wrapping_add((256i32 as libc::c_uint).wrapping_mul(bs) as libc::c_ulong),
            0x200i32 as size_t,
          ) as *mut volume_descriptor;
          if vd.is_null() {
            return -1i32;
          }
          type_0 = (*vd).tag.id as libc::c_uint;
          if !(type_0 != 2i32 as libc::c_uint) {
            /* get descriptor list address and block count */
            count = (*vd).type_0.anchor.length.wrapping_div(bs);
            loc = (*vd).type_0.anchor.location;
            /* pick the primary descriptor from the list */
            b = 0 as libc::c_uint;
            loop {
              if !(b < count) {
                current_block = 1843993682127799951;
                break;
              }
              vd = crate::util_linux::volume_id::util::volume_id_get_buffer(
                id,
                (0i32 as u64).wrapping_add(loc.wrapping_add(b).wrapping_mul(bs) as libc::c_ulong),
                0x200i32 as size_t,
              ) as *mut volume_descriptor;
              if vd.is_null() {
                return -1i32;
              }
              type_0 = (*vd).tag.id as libc::c_uint;
              /* check validity */
              if type_0 == 0 as libc::c_uint {
                current_block = 1843993682127799951;
                break;
              }
              if (*vd).tag.location != loc.wrapping_add(b) {
                current_block = 1843993682127799951;
                break;
              }
              if type_0 == 1i32 as libc::c_uint {
                current_block = 18232118978385979440;
                break;
              }
              b = b.wrapping_add(1)
            }
            match current_block {
              1843993682127799951 => {}
              _ =>
              /* TAG_ID_PVD */
              //	volume_id_set_label_raw(id, &(vd->type.primary.ident.clen), 32);
              {
                clen = (*vd).type_0.primary.ident.clen as libc::c_uint;
                if clen == 8i32 as libc::c_uint {
                  crate::util_linux::volume_id::util::volume_id_set_label_string(
                    id,
                    (*vd).type_0.primary.ident.c.as_mut_ptr(),
                    31i32 as size_t,
                  );
                } else if clen == 16i32 as libc::c_uint {
                  crate::util_linux::volume_id::util::volume_id_set_label_unicode16(
                    id,
                    (*vd).type_0.primary.ident.c.as_mut_ptr(),
                    BE,
                    31i32 as size_t,
                  );
                }
              }
            }
          }
          /* TAG_ID_AVDP */
          //	volume_id_set_usage(id, VOLUME_ID_FILESYSTEM);
          (*id).type_0 = b"udf\x00" as *const u8 as *const libc::c_char;
          return 0;
        }
      }
    }
  };
}

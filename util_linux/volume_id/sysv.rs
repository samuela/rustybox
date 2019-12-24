use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;

use libc;

use crate::librb::size_t;

use crate::util_linux::volume_id::volume_id::volume_id;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct xenix_super {
  pub s_isize: u16,
  pub s_fsize: u32,
  pub s_nfree: u16,
  pub s_free: [u32; 100],
  pub s_ninode: u16,
  pub s_inode: [u16; 100],
  pub s_flock: u8,
  pub s_ilock: u8,
  pub s_fmod: u8,
  pub s_ronly: u8,
  pub s_time: u32,
  pub s_tfree: u32,
  pub s_tinode: u16,
  pub s_dinfo: [u16; 4],
  pub s_fname: [u8; 6],
  pub s_fpack: [u8; 6],
  pub s_clean: u8,
  pub s_fill: [u8; 371],
  pub s_magic: u32,
  pub s_type: u32,
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
//config:config FEATURE_VOLUMEID_SYSV
//config:	bool "sysv filesystem"
//config:	default y
//config:	depends on VOLUMEID
//kbuild:lib-$(CONFIG_FEATURE_VOLUMEID_SYSV) += sysv.o

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct sysv_super {
  pub s_isize: u16,
  pub s_pad0: u16,
  pub s_fsize: u32,
  pub s_nfree: u16,
  pub s_pad1: u16,
  pub s_free: [u32; 50],
  pub s_ninode: u16,
  pub s_pad2: u16,
  pub s_inode: [u16; 100],
  pub s_flock: u8,
  pub s_ilock: u8,
  pub s_fmod: u8,
  pub s_ronly: u8,
  pub s_time: u32,
  pub s_dinfo: [u16; 4],
  pub s_tfree: u32,
  pub s_tinode: u16,
  pub s_pad3: u16,
  pub s_fname: [u8; 6],
  pub s_fpack: [u8; 6],
  pub s_fill: [u32; 12],
  pub s_state: u32,
  pub s_magic: u32,
  pub s_type: u32,
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
#[no_mangle]
pub unsafe extern "C" fn volume_id_probe_sysv(mut id: *mut volume_id) -> libc::c_int
/*,u64 off*/ {
  let mut current_block: u64;
  let mut vs: *mut sysv_super = std::ptr::null_mut();
  let mut xs: *mut xenix_super = std::ptr::null_mut();
  let mut boff: libc::c_uint = 0;
  boff = 0x200i32 as libc::c_uint;
  loop {
    if !(boff <= 0x800i32 as libc::c_uint) {
      current_block = 7746791466490516765;
      break;
    }
    vs = crate::util_linux::volume_id::util::volume_id_get_buffer(
      id,
      (0i32 as u64).wrapping_add(boff.wrapping_mul(0x1i32 as libc::c_uint) as libc::c_ulong),
      0x200i32 as size_t,
    ) as *mut sysv_super;
    if vs.is_null() {
      return -1i32;
    }
    if (*vs).s_magic == 0xfd187e20u32
      || (*vs).s_magic
        == ({
          let mut __v: libc::c_uint = 0;
          let mut __x: libc::c_uint = 0xfd187e20u32;
          if false {
            __v = (__x & 0xff000000u32) >> 24i32
              | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
              | (__x & 0xff00i32 as libc::c_uint) << 8i32
              | (__x & 0xffi32 as libc::c_uint) << 24i32
          } else {
            let fresh0 = &mut __v;
            let fresh1;
            let fresh2 = __x;
            asm!("bswap $0" : "=r" (fresh1) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2)) :);
            c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
          }
          __v
        })
    {
      //			volume_id_set_label_raw(id, vs->s_fname, 6);
      crate::util_linux::volume_id::util::volume_id_set_label_string(
        id,
        (*vs).s_fname.as_mut_ptr(),
        6i32 as size_t,
      );
      (*id).type_0 = b"sysv\x00" as *const u8 as *const libc::c_char;
      current_block = 8260290716299312918;
      break;
    } else {
      boff <<= 1i32
    }
  }
  match current_block {
    7746791466490516765 => {
      boff = 0x200i32 as libc::c_uint;
      loop {
        if !(boff <= 0x800i32 as libc::c_uint) {
          current_block = 12124785117276362961;
          break;
        }
        xs = crate::util_linux::volume_id::util::volume_id_get_buffer(
          id,
          (0i32 as u64).wrapping_add(boff.wrapping_add(0x18i32 as libc::c_uint) as libc::c_ulong),
          0x200i32 as size_t,
        ) as *mut xenix_super;
        if xs.is_null() {
          return -1i32;
        }
        if (*xs).s_magic == 0x2b5544i32 as u32
          || (*xs).s_magic
            == ({
              let mut __v: libc::c_uint = 0;
              let mut __x: libc::c_uint = 0x2b5544i32 as libc::c_uint;
              if false {
                __v = (__x & 0xff000000u32) >> 24i32
                  | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
                  | (__x & 0xff00i32 as libc::c_uint) << 8i32
                  | (__x & 0xffi32 as libc::c_uint) << 24i32
              } else {
                let fresh3 = &mut __v;
                let fresh4;
                let fresh5 = __x;
                asm!("bswap $0" : "=r" (fresh4) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5)) :);
                c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
              }
              __v
            })
        {
          //			volume_id_set_label_raw(id, xs->s_fname, 6);
          crate::util_linux::volume_id::util::volume_id_set_label_string(
            id,
            (*xs).s_fname.as_mut_ptr(),
            6i32 as size_t,
          );
          (*id).type_0 = b"xenix\x00" as *const u8 as *const libc::c_char;
          current_block = 8260290716299312918;
          break;
        } else {
          boff <<= 1i32
        }
      }
      match current_block {
        8260290716299312918 => {}
        _ => return -1i32,
      }
    }
    _ => {}
  }
  //	volume_id_set_usage(id, VOLUME_ID_FILESYSTEM);
  return 0;
}

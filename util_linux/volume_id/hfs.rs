use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use crate::libbb::ptr_to_globals::bb_errno;
use libc;
extern "C" {
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn md5_begin(ctx: *mut md5_ctx_t);

  #[no_mangle]
  fn md5_hash(ctx: *mut md5_ctx_t, buffer: *const libc::c_void, len: size_t);

  #[no_mangle]
  fn md5_end(ctx: *mut md5_ctx_t, resbuf: *mut libc::c_void) -> libc::c_uint;

  #[no_mangle]
  fn volume_id_set_label_string(id: *mut volume_id, buf: *const u8, count: size_t);

  #[no_mangle]
  fn volume_id_set_label_unicode16(
    id: *mut volume_id,
    buf: *const u8,
    endianess: endian,
    count: size_t,
  );

  #[no_mangle]
  fn volume_id_set_uuid(id: *mut volume_id, buf: *const u8, format: uuid_format);

  #[no_mangle]
  fn volume_id_get_buffer(id: *mut volume_id, off: u64, len: size_t) -> *mut libc::c_void;
}

use crate::librb::size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md5_ctx_t {
  pub wbuffer: [u8; 64],
  pub process_block: Option<unsafe extern "C" fn(_: *mut md5_ctx_t) -> ()>,
  pub total64: u64,
  pub hash: [u32; 8],
  /* 4 elements for md5, 5 for sha1, 8 for sha256 */
}

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

pub type uuid_format = libc::c_uint;
// pub const UUID_DCE_STRING: uuid_format = 3;
pub const UUID_DCE: uuid_format = 2;
// pub const UUID_NTFS: uuid_format = 1;
// pub const UUID_DOS: uuid_format = 0;

pub type endian = libc::c_uint;
pub const BE: endian = 1;
// pub const LE: endian = 0;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct hfsplus_catalog_key {
  pub key_len: u16,
  pub parent_id: u32,
  pub unicode_len: u16,
  pub unicode: [u8; 510],
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct hfsplus_bnode_descriptor {
  pub next: u32,
  pub prev: u32,
  pub type_0: u8,
  pub height: u8,
  pub num_recs: u16,
  pub reserved: u16,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct hfsplus_extent {
  pub start_block: u32,
  pub block_count: u32,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct hfsplus_bheader_record {
  pub depth: u16,
  pub root: u32,
  pub leaf_count: u32,
  pub leaf_head: u32,
  pub leaf_tail: u32,
  pub node_size: u16,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct hfsplus_fork {
  pub total_size: u64,
  pub clump_size: u32,
  pub total_blocks: u32,
  pub extents: [hfsplus_extent; 8],
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct hfsplus_vol_header {
  pub signature: [u8; 2],
  pub version: u16,
  pub attributes: u32,
  pub last_mount_vers: u32,
  pub reserved: u32,
  pub create_date: u32,
  pub modify_date: u32,
  pub backup_date: u32,
  pub checked_date: u32,
  pub file_count: u32,
  pub folder_count: u32,
  pub blocksize: u32,
  pub total_blocks: u32,
  pub free_blocks: u32,
  pub next_alloc: u32,
  pub rsrc_clump_sz: u32,
  pub data_clump_sz: u32,
  pub next_cnid: u32,
  pub write_count: u32,
  pub encodings_bmp: u64,
  pub finder_info: hfs_finder_info,
  pub alloc_file: hfsplus_fork,
  pub ext_file: hfsplus_fork,
  pub cat_file: hfsplus_fork,
  pub attr_file: hfsplus_fork,
  pub start_file: hfsplus_fork,
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
//config:config FEATURE_VOLUMEID_HFS
//config:	bool "hfs filesystem"
//config:	default y
//config:	depends on VOLUMEID
//kbuild:lib-$(CONFIG_FEATURE_VOLUMEID_HFS) += hfs.o
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct hfs_finder_info {
  pub boot_folder: u32,
  pub start_app: u32,
  pub open_folder: u32,
  pub os9_folder: u32,
  pub reserved: u32,
  pub osx_folder: u32,
  pub id: [u8; 8],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct hfs_mdb {
  pub signature: [u8; 2],
  pub cr_date: u32,
  pub ls_Mod: u32,
  pub atrb: u16,
  pub nm_fls: u16,
  pub vbm_st: u16,
  pub alloc_ptr: u16,
  pub nm_al_blks: u16,
  pub al_blk_size: u32,
  pub clp_size: u32,
  pub al_bl_st: u16,
  pub nxt_cnid: u32,
  pub free_bks: u16,
  pub label_len: u8,
  pub label: [u8; 27],
  pub vol_bkup: u32,
  pub vol_seq_num: u16,
  pub wr_cnt: u32,
  pub xt_clump_size: u32,
  pub ct_clump_size: u32,
  pub num_root_dirs: u16,
  pub file_count: u32,
  pub dir_count: u32,
  pub finder_info: hfs_finder_info,
  pub embed_sig: [u8; 2],
  pub embed_startblock: u16,
  pub embed_blockcount: u16,
}
unsafe extern "C" fn hfs_set_uuid(mut id: *mut volume_id, mut hfs_id: *const u8) {
  let mut current_block: u64;
  let mut md5c: md5_ctx_t = md5_ctx_t {
    wbuffer: [0; 64],
    process_block: None,
    total64: 0,
    hash: [0; 8],
  };
  let mut uuid: [u8; 16] = [0; 16];
  let mut i: libc::c_uint = 0;
  i = 0i32 as libc::c_uint;
  loop {
    if !(i < 8i32 as libc::c_uint) {
      current_block = 7502529970979898288;
      break;
    }
    if *hfs_id.offset(i as isize) as libc::c_int != 0i32 {
      current_block = 10177439978398584590;
      break;
    }
    i = i.wrapping_add(1)
  }
  match current_block {
    7502529970979898288 => return,
    _ => {
      md5_begin(&mut md5c);
      md5_hash(
        &mut md5c,
        b"\xb3\xe2\x0f9\xf2\x92\x11\xd6\x97\xa4\x000eC\xec\xac\x00" as *const u8
          as *const libc::c_char as *const libc::c_void,
        16i32 as size_t,
      );
      md5_hash(&mut md5c, hfs_id as *const libc::c_void, 8i32 as size_t);
      md5_end(&mut md5c, uuid.as_mut_ptr() as *mut libc::c_void);
      uuid[6] = (0x30i32 | uuid[6] as libc::c_int & 0xfi32) as u8;
      uuid[8] = (0x80i32 | uuid[8] as libc::c_int & 0x3fi32) as u8;
      volume_id_set_uuid(id, uuid.as_mut_ptr(), UUID_DCE);
      return;
    }
  };
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
#[no_mangle]
pub unsafe extern "C" fn volume_id_probe_hfs_hfsplus(mut id: *mut volume_id) -> libc::c_int
/*,u64 off*/ {
  let mut current_block: u64;
  let mut off: u64 = 0i32 as u64;
  let mut blocksize: libc::c_uint = 0;
  let mut cat_block: libc::c_uint = 0;
  let mut ext_block_start: libc::c_uint = 0;
  let mut ext_block_count: libc::c_uint = 0;
  let mut ext: libc::c_int = 0;
  let mut leaf_node_head: libc::c_uint = 0;
  let mut leaf_node_count: libc::c_uint = 0;
  let mut leaf_node_size: libc::c_uint = 0;
  let mut leaf_block: libc::c_uint = 0;
  let mut leaf_off: u64 = 0;
  let mut alloc_block_size: libc::c_uint = 0;
  let mut alloc_first_block: libc::c_uint = 0;
  let mut embed_first_block: libc::c_uint = 0;
  let mut record_count: libc::c_uint = 0;
  let mut hfsplus: *mut hfsplus_vol_header = 0 as *mut hfsplus_vol_header;
  let mut descr: *mut hfsplus_bnode_descriptor = 0 as *mut hfsplus_bnode_descriptor;
  let mut bnode: *mut hfsplus_bheader_record = 0 as *mut hfsplus_bheader_record;
  let mut key: *mut hfsplus_catalog_key = 0 as *mut hfsplus_catalog_key;
  let mut label_len: libc::c_uint = 0;
  let mut extents: [hfsplus_extent; 8] = [hfsplus_extent {
    start_block: 0,
    block_count: 0,
  }; 8];
  let mut hfs: *mut hfs_mdb = 0 as *mut hfs_mdb;
  let mut buf: *const u8 = 0 as *const u8;
  buf = volume_id_get_buffer(
    id,
    off.wrapping_add(0x400i32 as libc::c_ulong),
    0x200i32 as size_t,
  ) as *const u8;
  if buf.is_null() {
    return -1i32;
  }
  hfs = buf as *mut hfs_mdb;
  if !((*hfs).signature[0] as libc::c_int != 'B' as i32
    || (*hfs).signature[1] as libc::c_int != 'D' as i32)
  {
    /* it may be just a hfs wrapper for hfs+ */
    if (*hfs).embed_sig[0] as libc::c_int == 'H' as i32
      && (*hfs).embed_sig[1] as libc::c_int == '+' as i32
    {
      alloc_block_size = {
        let mut __v: libc::c_uint = 0;
        let mut __x: libc::c_uint = (*hfs).al_blk_size;
        if 0 != 0 {
          __v = (__x & 0xff000000u32) >> 24i32
            | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
            | (__x & 0xff00i32 as libc::c_uint) << 8i32
            | (__x & 0xffi32 as libc::c_uint) << 24i32
        } else {
          let fresh0 = &mut __v;
          let fresh1;
          let fresh2 = __x;
          asm!("bswap $0" : "=r" (fresh1) : "0"
                              (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2))
                              :);
          c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
        }
        __v
      };
      alloc_first_block = ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = (*hfs).al_bl_st;
        if 0 != 0 {
          __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
            as libc::c_ushort
        } else {
          let fresh3 = &mut __v;
          let fresh4;
          let fresh5 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh4) : "0"
                              (c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5))
                              : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
        }
        __v
      }) as libc::c_uint;
      embed_first_block = ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = (*hfs).embed_startblock;
        if 0 != 0 {
          __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
            as libc::c_ushort
        } else {
          let fresh6 = &mut __v;
          let fresh7;
          let fresh8 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh7) : "0"
                              (c2rust_asm_casts::AsmCast::cast_in(fresh6, fresh8))
                              : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
        }
        __v
      }) as libc::c_uint;
      off = (off as libc::c_ulong).wrapping_add(
        alloc_first_block
          .wrapping_mul(512i32 as libc::c_uint)
          .wrapping_add(embed_first_block.wrapping_mul(alloc_block_size)) as libc::c_ulong,
      ) as u64 as u64;
      buf = volume_id_get_buffer(
        id,
        off.wrapping_add(0x400i32 as libc::c_ulong),
        0x200i32 as size_t,
      ) as *const u8;
      if buf.is_null() {
        return -1i32;
      }
    } else {
      if (*hfs).label_len as libc::c_int > 0i32 && ((*hfs).label_len as libc::c_int) < 28i32 {
        //		volume_id_set_label_raw(id, hfs->label, hfs->label_len);
        volume_id_set_label_string(id, (*hfs).label.as_mut_ptr(), (*hfs).label_len as size_t);
      }
      hfs_set_uuid(id, (*hfs).finder_info.id.as_mut_ptr());
      //	volume_id_set_usage(id, VOLUME_ID_FILESYSTEM);
      (*id).type_0 = b"hfs\x00" as *const u8 as *const libc::c_char;
      return 0i32;
    }
  }
  hfsplus = buf as *mut hfsplus_vol_header;
  if (*hfs).signature[0] as libc::c_int == 'H' as i32 {
    if (*hfs).signature[1] as libc::c_int == '+' as i32
      || (*hfs).signature[1] as libc::c_int == 'X' as i32
    {
      hfs_set_uuid(id, (*hfsplus).finder_info.id.as_mut_ptr());
      blocksize = {
        let mut __v: libc::c_uint = 0;
        let mut __x: libc::c_uint = (*hfsplus).blocksize;
        if 0 != 0 {
          __v = (__x & 0xff000000u32) >> 24i32
            | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
            | (__x & 0xff00i32 as libc::c_uint) << 8i32
            | (__x & 0xffi32 as libc::c_uint) << 24i32
        } else {
          let fresh9 = &mut __v;
          let fresh10;
          let fresh11 = __x;
          asm!("bswap $0" : "=r" (fresh10) : "0"
                              (c2rust_asm_casts::AsmCast::cast_in(fresh9, fresh11))
                              :);
          c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
        }
        __v
      };
      memcpy(
        extents.as_mut_ptr() as *mut libc::c_void,
        (*hfsplus).cat_file.extents.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[hfsplus_extent; 8]>() as libc::c_ulong,
      );
      cat_block = {
        let mut __v: libc::c_uint = 0;
        let mut __x: libc::c_uint = extents[0].start_block;
        if 0 != 0 {
          __v = (__x & 0xff000000u32) >> 24i32
            | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
            | (__x & 0xff00i32 as libc::c_uint) << 8i32
            | (__x & 0xffi32 as libc::c_uint) << 24i32
        } else {
          let fresh12 = &mut __v;
          let fresh13;
          let fresh14 = __x;
          asm!("bswap $0" : "=r" (fresh13) : "0"
                              (c2rust_asm_casts::AsmCast::cast_in(fresh12, fresh14))
                              :);
          c2rust_asm_casts::AsmCast::cast_out(fresh12, fresh14, fresh13);
        }
        __v
      };
      buf = volume_id_get_buffer(
        id,
        off.wrapping_add(cat_block.wrapping_mul(blocksize) as libc::c_ulong),
        0x2000i32 as size_t,
      ) as *const u8;
      if !buf.is_null() {
        bnode = &*buf
          .offset(::std::mem::size_of::<hfsplus_bnode_descriptor>() as libc::c_ulong as isize)
          as *const u8 as *mut hfsplus_bheader_record;
        leaf_node_head = {
          let mut __v: libc::c_uint = 0;
          let mut __x: libc::c_uint = (*bnode).leaf_head;
          if 0 != 0 {
            __v = (__x & 0xff000000u32) >> 24i32
              | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
              | (__x & 0xff00i32 as libc::c_uint) << 8i32
              | (__x & 0xffi32 as libc::c_uint) << 24i32
          } else {
            let fresh15 = &mut __v;
            let fresh16;
            let fresh17 = __x;
            asm!("bswap $0" : "=r" (fresh16) : "0"
                                  (c2rust_asm_casts::AsmCast::cast_in(fresh15, fresh17))
                                  :);
            c2rust_asm_casts::AsmCast::cast_out(fresh15, fresh17, fresh16);
          }
          __v
        };
        leaf_node_size = ({
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = (*bnode).node_size;
          if 0 != 0 {
            __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
              as libc::c_ushort
          } else {
            let fresh18 = &mut __v;
            let fresh19;
            let fresh20 = __x;
            asm!("rorw $$8, ${0:w}" : "=r" (fresh19) : "0"
                                  (c2rust_asm_casts::AsmCast::cast_in(fresh18, fresh20))
                                  : "cc");
            c2rust_asm_casts::AsmCast::cast_out(fresh18, fresh20, fresh19);
          }
          __v
        }) as libc::c_uint;
        leaf_node_count = {
          let mut __v: libc::c_uint = 0;
          let mut __x: libc::c_uint = (*bnode).leaf_count;
          if 0 != 0 {
            __v = (__x & 0xff000000u32) >> 24i32
              | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
              | (__x & 0xff00i32 as libc::c_uint) << 8i32
              | (__x & 0xffi32 as libc::c_uint) << 24i32
          } else {
            let fresh21 = &mut __v;
            let fresh22;
            let fresh23 = __x;
            asm!("bswap $0" : "=r" (fresh22) : "0"
                                  (c2rust_asm_casts::AsmCast::cast_in(fresh21, fresh23))
                                  :);
            c2rust_asm_casts::AsmCast::cast_out(fresh21, fresh23, fresh22);
          }
          __v
        };
        if !(leaf_node_count == 0i32 as libc::c_uint) {
          leaf_block = leaf_node_head
            .wrapping_mul(leaf_node_size)
            .wrapping_div(blocksize);
          /* get physical location */
          ext = 0i32;
          loop {
            if !(ext < 8i32) {
              current_block = 16203797167131938757;
              break;
            }
            ext_block_start = {
              let mut __v: libc::c_uint = 0;
              let mut __x: libc::c_uint = extents[ext as usize].start_block;
              if 0 != 0 {
                __v = (__x & 0xff000000u32) >> 24i32
                  | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
                  | (__x & 0xff00i32 as libc::c_uint) << 8i32
                  | (__x & 0xffi32 as libc::c_uint) << 24i32
              } else {
                let fresh24 = &mut __v;
                let fresh25;
                let fresh26 = __x;
                asm!("bswap $0" : "=r" (fresh25) : "0"
                                          (c2rust_asm_casts::AsmCast::cast_in(fresh24, fresh26))
                                          :);
                c2rust_asm_casts::AsmCast::cast_out(fresh24, fresh26, fresh25);
              }
              __v
            };
            ext_block_count = {
              let mut __v: libc::c_uint = 0;
              let mut __x: libc::c_uint = extents[ext as usize].block_count;
              if 0 != 0 {
                __v = (__x & 0xff000000u32) >> 24i32
                  | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
                  | (__x & 0xff00i32 as libc::c_uint) << 8i32
                  | (__x & 0xffi32 as libc::c_uint) << 24i32
              } else {
                let fresh27 = &mut __v;
                let fresh28;
                let fresh29 = __x;
                asm!("bswap $0" : "=r" (fresh28) : "0"
                                          (c2rust_asm_casts::AsmCast::cast_in(fresh27, fresh29))
                                          :);
                c2rust_asm_casts::AsmCast::cast_out(fresh27, fresh29, fresh28);
              }
              __v
            };
            if ext_block_count == 0i32 as libc::c_uint {
              current_block = 7082531977902607404;
              break;
            }
            /* this is our extent */
            if leaf_block < ext_block_count {
              current_block = 16203797167131938757;
              break;
            }
            leaf_block = leaf_block.wrapping_sub(ext_block_count);
            ext += 1
          }
          match current_block {
            7082531977902607404 => {}
            _ => {
              if !(ext == 8i32) {
                leaf_off = ext_block_start
                  .wrapping_add(leaf_block)
                  .wrapping_mul(blocksize) as u64;
                buf = volume_id_get_buffer(id, off.wrapping_add(leaf_off), leaf_node_size as size_t)
                  as *const u8;
                if !buf.is_null() {
                  descr = buf as *mut hfsplus_bnode_descriptor;
                  record_count = ({
                    let mut __v: libc::c_ushort = 0;
                    let mut __x: libc::c_ushort = (*descr).num_recs;
                    if 0 != 0 {
                      __v = (__x as libc::c_int >> 8i32 & 0xffi32
                        | (__x as libc::c_int & 0xffi32) << 8i32)
                        as libc::c_ushort
                    } else {
                      let fresh30 = &mut __v;
                      let fresh31;
                      let fresh32 = __x;
                      asm!("rorw $$8, ${0:w}" :
                                                      "=r" (fresh31) : "0"
                                                      (c2rust_asm_casts::AsmCast::cast_in(fresh30, fresh32))
                                                      : "cc");
                      c2rust_asm_casts::AsmCast::cast_out(fresh30, fresh32, fresh31);
                    }
                    __v
                  }) as libc::c_uint;
                  if !(record_count == 0i32 as libc::c_uint) {
                    if !((*descr).type_0 as libc::c_int != 0xffi32) {
                      key =
                        &*buf.offset(::std::mem::size_of::<hfsplus_bnode_descriptor>()
                          as libc::c_ulong as isize) as *const u8
                          as *mut hfsplus_catalog_key;
                      if !((*key).parent_id
                        != ({
                          let mut __v: libc::c_uint = 0;
                          let mut __x: libc::c_uint = 1i32 as libc::c_uint;
                          if 0 != 0 {
                            __v = (__x & 0xff000000u32) >> 24i32
                              | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
                              | (__x & 0xff00i32 as libc::c_uint) << 8i32
                              | (__x & 0xffi32 as libc::c_uint) << 24i32
                          } else {
                            let fresh33 = &mut __v;
                            let fresh34;
                            let fresh35 = __x;
                            asm!("bswap $0"
                                                                   : "=r"
                                                                   (fresh34) :
                                                                   "0"
                                                                   (c2rust_asm_casts::AsmCast::cast_in(fresh33, fresh35))
                                                                   :);
                            c2rust_asm_casts::AsmCast::cast_out(fresh33, fresh35, fresh34);
                          }
                          __v
                        }))
                      {
                        label_len = (({
                          let mut __v: libc::c_ushort = 0;
                          let mut __x: libc::c_ushort = (*key).unicode_len;
                          if 0 != 0 {
                            __v = (__x as libc::c_int >> 8i32 & 0xffi32
                              | (__x as libc::c_int & 0xffi32) << 8i32)
                              as libc::c_ushort
                          } else {
                            let fresh36 = &mut __v;
                            let fresh37;
                            let fresh38 = __x;
                            asm!("rorw $$8, ${0:w}"
                                                                   : "=r"
                                                                   (fresh37) :
                                                                   "0"
                                                                   (c2rust_asm_casts::AsmCast::cast_in(fresh36, fresh38))
                                                                   : "cc");
                            c2rust_asm_casts::AsmCast::cast_out(fresh36, fresh38, fresh37);
                          }
                          __v
                        }) as libc::c_int
                          * 2i32) as libc::c_uint;
                        //	volume_id_set_label_raw(id, key->unicode, label_len);
                        volume_id_set_label_unicode16(
                          id,
                          (*key).unicode.as_mut_ptr(),
                          BE,
                          label_len as size_t,
                        );
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
      //	volume_id_set_usage(id, VOLUME_ID_FILESYSTEM);
      (*id).type_0 = b"hfsplus\x00" as *const u8 as *const libc::c_char;
      return 0i32;
    }
  }
  return -1i32;
}

use crate::librb::size_t;
use libc;
use libc::close;
use libc::free;

#[repr(C)]
#[derive(Copy, Clone)]
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

pub type probe_fptr = Option<unsafe fn(_: *mut volume_id) -> libc::c_int>;
/*, u64 off*/
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
//kbuild:lib-$(CONFIG_VOLUMEID) += volume_id.o util.o
/* Some detection routines do not set label or uuid anyway,
 * so they are disabled. */
/* Looks for partitions, we don't use it: */
/* #define ENABLE_FEATURE_VOLUMEID_MSDOS      0 - NB: this one
 * was not properly added to probe table anyway - ??! */
/* None of RAIDs have label or uuid, except LinuxRAID: */
/* These filesystems also have no label or uuid: */
pub type raid_probe_fptr = Option<unsafe fn(_: *mut volume_id, _: u64) -> libc::c_int>;
static mut raid1: [raid_probe_fptr; 1] = {
  [Some(
    crate::util_linux::volume_id::linux_raid::volume_id_probe_linux_raid,
  )]
};
static mut raid2: [probe_fptr; 1] = {
  [Some(
    crate::util_linux::volume_id::luks::volume_id_probe_luks,
  )]
};
/*u64 off,*/
/* signature in the first block, only small buffer needed */
static mut fs1: [probe_fptr; 6] = {
  [
    Some(crate::util_linux::volume_id::fat::volume_id_probe_vfat),
    Some(crate::util_linux::volume_id::exfat::volume_id_probe_exfat),
    Some(crate::util_linux::volume_id::lfs::volume_id_probe_lfs),
    Some(crate::util_linux::volume_id::squashfs::volume_id_probe_squashfs),
    Some(crate::util_linux::volume_id::xfs::volume_id_probe_xfs),
    Some(crate::util_linux::volume_id::bcache::volume_id_probe_bcache),
  ]
};
/* fill buffer with maximum */
static mut fs2: [probe_fptr; 17] = {
  [
    Some(crate::util_linux::volume_id::linux_swap::volume_id_probe_linux_swap),
    Some(crate::util_linux::volume_id::ext::volume_id_probe_ext),
    Some(crate::util_linux::volume_id::btrfs::volume_id_probe_btrfs),
    Some(crate::util_linux::volume_id::reiserfs::volume_id_probe_reiserfs),
    Some(crate::util_linux::volume_id::jfs::volume_id_probe_jfs),
    Some(crate::util_linux::volume_id::udf::volume_id_probe_udf),
    Some(crate::util_linux::volume_id::iso9660::volume_id_probe_iso9660),
    Some(crate::util_linux::volume_id::hfs::volume_id_probe_hfs_hfsplus),
    Some(crate::util_linux::volume_id::f2fs::volume_id_probe_f2fs),
    Some(crate::util_linux::volume_id::nilfs::volume_id_probe_nilfs),
    Some(crate::util_linux::volume_id::ntfs::volume_id_probe_ntfs),
    Some(crate::util_linux::volume_id::cramfs::volume_id_probe_cramfs),
    Some(crate::util_linux::volume_id::romfs::volume_id_probe_romfs),
    Some(crate::util_linux::volume_id::sysv::volume_id_probe_sysv),
    Some(crate::util_linux::volume_id::minix::volume_id_probe_minix),
    Some(crate::util_linux::volume_id::ocfs2::volume_id_probe_ocfs2),
    Some(crate::util_linux::volume_id::ubifs::volume_id_probe_ubifs),
  ]
};
pub unsafe fn volume_id_probe_all(mut id: *mut volume_id, mut size: u64) -> libc::c_int {
  let mut current_block: u64;
  let mut i: libc::c_uint = 0;
  /* probe for raid first, cause fs probes may be successful on raid members */
  if size != 0 {
    i = 0 as libc::c_uint;
    loop {
      if !(i
        < (::std::mem::size_of::<[raid_probe_fptr; 1]>() as libc::c_ulong)
          .wrapping_div(::std::mem::size_of::<raid_probe_fptr>() as libc::c_ulong)
          as libc::c_uint)
      {
        current_block = 7815301370352969686;
        break;
      }
      if raid1[i as usize].expect("non-null function pointer")(id, size) == 0 {
        current_block = 13835600803501426168;
        break;
      }
      if (*id).error != 0 {
        current_block = 13835600803501426168;
        break;
      }
      i = i.wrapping_add(1)
    }
  } else {
    current_block = 7815301370352969686;
  }
  match current_block {
    7815301370352969686 => {
      i = 0 as libc::c_uint;
      loop {
        if !(i
          < (::std::mem::size_of::<[probe_fptr; 1]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<probe_fptr>() as libc::c_ulong)
            as libc::c_uint)
        {
          current_block = 2979737022853876585;
          break;
        }
        if raid2[i as usize].expect("non-null function pointer")(id) == 0 {
          current_block = 13835600803501426168;
          break;
        }
        if (*id).error != 0 {
          current_block = 13835600803501426168;
          break;
        }
        i = i.wrapping_add(1)
      }
      match current_block {
        13835600803501426168 => {}
        _ =>
        /* signature in the first block, only small buffer needed */
        {
          i = 0 as libc::c_uint;
          loop {
            if !(i
              < (::std::mem::size_of::<[probe_fptr; 6]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<probe_fptr>() as libc::c_ulong)
                as libc::c_uint)
            {
              current_block = 17833034027772472439;
              break;
            }
            if fs1[i as usize].expect("non-null function pointer")(id) == 0 {
              current_block = 13835600803501426168;
              break;
            }
            if (*id).error != 0 {
              current_block = 13835600803501426168;
              break;
            }
            i = i.wrapping_add(1)
          }
          match current_block {
            13835600803501426168 => {}
            _ => {
              /* fill buffer with maximum */
              crate::util_linux::volume_id::util::volume_id_get_buffer(
                id,
                0 as u64,
                0x11000i32 as size_t,
              );
              i = 0 as libc::c_uint;
              while i
                < (::std::mem::size_of::<[probe_fptr; 17]>() as libc::c_ulong)
                  .wrapping_div(::std::mem::size_of::<probe_fptr>() as libc::c_ulong)
                  as libc::c_uint
              {
                if fs2[i as usize].expect("non-null function pointer")(id) == 0 {
                  break;
                }
                if (*id).error != 0 {
                  break;
                }
                i = i.wrapping_add(1)
              }
            }
          }
        }
      }
    }
    _ => {}
  }
  crate::util_linux::volume_id::util::volume_id_free_buffer(id);
  return -(*id).error;
  /* 0 or -1 */
}
/* open volume by device node */
pub unsafe fn volume_id_open_node(mut fd: libc::c_int) -> *mut volume_id {
  let mut id: *mut volume_id = std::ptr::null_mut();
  id = crate::libbb::xfuncs_printf::xzalloc(::std::mem::size_of::<volume_id>() as libc::c_ulong)
    as *mut volume_id;
  (*id).fd = fd;
  // /* close fd on device close */
  //id->fd_close = 1;
  return id;
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
pub unsafe fn free_volume_id(mut id: *mut volume_id) {
  if id.is_null() {
    return;
  }
  //if (id->fd_close != 0) - always true
  close((*id).fd);
  crate::util_linux::volume_id::util::volume_id_free_buffer(id);
  free(id as *mut libc::c_void);
}

use libc;
use libc::ioctl;
use libc::open;
use libc::printf;
use libc::strcasecmp;
use libc::strcmp;
extern "C" {

  #[no_mangle]
  fn gnu_dev_major(__dev: libc::dev_t) -> libc::c_uint;

  #[no_mangle]
  fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;

}

use crate::librb::size_t;
use libc::stat;
pub type C2RustUnnamed = libc::c_uint;
// pub const ACTION_DANGLING_OK: C2RustUnnamed = 64;
// pub const ACTION_QUIET: C2RustUnnamed = 32;
// pub const ACTION_DEPTHFIRST: C2RustUnnamed = 8;
// pub const ACTION_FOLLOWLINKS_L0: C2RustUnnamed = 4;
// pub const ACTION_FOLLOWLINKS: C2RustUnnamed = 2;
pub const ACTION_RECURSE: C2RustUnnamed = 1;

/*
 * Support functions for mounting devices by label/uuid
 *
 * Copyright (C) 2006 by Jason Schoon <floydpink@gmail.com>
 * Some portions cribbed from e2fsprogs, util-linux, dosfstools
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//kbuild:lib-$(CONFIG_BLKID) += get_devname.o
//kbuild:lib-$(CONFIG_FINDFS) += get_devname.o
//kbuild:lib-$(CONFIG_FEATURE_MOUNT_LABEL) += get_devname.o
//kbuild:lib-$(CONFIG_FEATURE_SWAPONOFF_LABEL) += get_devname.o
/* BLKGETSIZE64 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uuidCache_s {
  pub next: *mut uuidCache_s,
  pub device: *mut libc::c_char,
  pub label: *mut libc::c_char,
  pub uc_uuid: *mut libc::c_char,
  pub type_0: *const libc::c_char,
}

use crate::util_linux::volume_id::volume_id::volume_id;
static mut uuidCache: *mut uuidCache_s = std::ptr::null_mut();
/* Returns !0 on error.
 * Otherwise, returns malloc'ed strings for label and uuid
 * (and they can't be NULL, although they can be "").
 * NB: closes fd. */
unsafe extern "C" fn get_label_uuid(
  mut fd: libc::c_int,
  mut label: *mut *mut libc::c_char,
  mut uuid: *mut *mut libc::c_char,
  mut type_0: *mut *const libc::c_char,
) -> libc::c_int {
  let mut rv: libc::c_int = 1i32;
  let mut size: u64 = 0;
  let mut vid: *mut volume_id = std::ptr::null_mut();
  /* fd is owned by vid now */
  vid = crate::util_linux::volume_id::volume_id::volume_id_open_node(fd); /* also closes fd */
  if ioctl(
    fd,
    (2u32 << 0 + 8i32 + 8i32 + 14i32
      | (0x12i32 << 0 + 8i32) as libc::c_uint
      | (114i32 << 0) as libc::c_uint) as libc::c_ulong
      | (::std::mem::size_of::<size_t>() as libc::c_ulong) << 0 + 8i32 + 8i32,
    &mut size as *mut u64,
  ) != 0
  {
    size = 0 as u64
  }
  if !(crate::util_linux::volume_id::volume_id::volume_id_probe_all(vid, size) != 0) {
    if (*vid).label[0] as libc::c_int != '\u{0}' as i32
      || (*vid).uuid[0] as libc::c_int != '\u{0}' as i32
      || !(*vid).type_0.is_null()
    {
      *label = crate::libbb::xfuncs_printf::xstrndup(
        (*vid).label.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 65]>() as libc::c_ulong as libc::c_int,
      );
      *uuid = crate::libbb::xfuncs_printf::xstrndup(
        (*vid).uuid.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 37]>() as libc::c_ulong as libc::c_int,
      );
      *type_0 = (*vid).type_0;
      rv = 0
    }
  }
  crate::util_linux::volume_id::volume_id::free_volume_id(vid);
  return rv;
}
/* NB: we take ownership of (malloc'ed) label and uuid */
unsafe extern "C" fn uuidcache_addentry(
  mut device: *mut libc::c_char,
  mut label: *mut libc::c_char,
  mut uuid: *mut libc::c_char,
  mut type_0: *const libc::c_char,
) {
  let mut last: *mut uuidCache_s = std::ptr::null_mut();
  if uuidCache.is_null() {
    uuidCache =
      crate::libbb::xfuncs_printf::xzalloc(::std::mem::size_of::<uuidCache_s>() as libc::c_ulong)
        as *mut uuidCache_s;
    last = uuidCache
  } else {
    last = uuidCache;
    while !(*last).next.is_null() {
      last = (*last).next
    }
    (*last).next =
      crate::libbb::xfuncs_printf::xzalloc(::std::mem::size_of::<uuidCache_s>() as libc::c_ulong)
        as *mut uuidCache_s;
    last = (*last).next
  }
  /*last->next = NULL; - xzalloc did it*/
  //	last->major = major;
  //	last->minor = minor;
  (*last).device = device;
  (*last).label = label;
  (*last).uc_uuid = uuid;
  (*last).type_0 = type_0;
}
/* If get_label_uuid() on device_name returns success,
 * add a cache entry for this device.
 * If device node does not exist, it will be temporarily created. */
unsafe extern "C" fn uuidcache_check_device(
  mut device: *const libc::c_char,
  mut statbuf: *mut stat,
  mut _userData: *mut libc::c_void,
  mut _depth: libc::c_int,
) -> libc::c_int {
  /* note: this check rejects links to devices, among other nodes */
  if !((*statbuf).st_mode & 0o170000i32 as libc::c_uint == 0o60000i32 as libc::c_uint)
    && !((*statbuf).st_mode & 0o170000i32 as libc::c_uint == 0o20000i32 as libc::c_uint
      && strncmp(
        crate::libbb::get_last_path_component::bb_basename(device),
        b"ubi\x00" as *const u8 as *const libc::c_char,
        3i32 as libc::c_ulong,
      ) == 0)
  {
    return 1i32;
  }
  /* Users report that mucking with floppies (especially non-present
   * ones) is significant PITA. This is a horribly dirty hack,
   * but it is very useful in real world.
   * If this will ever need to be enabled, consider using O_NONBLOCK.
   */
  if gnu_dev_major((*statbuf).st_rdev) == 2i32 as libc::c_uint {
    return 1i32;
  }
  add_to_uuid_cache(device);
  return 1i32;
}
unsafe extern "C" fn uuidcache_init(mut scan_devices: libc::c_int) -> *mut uuidCache_s {
  if !uuidCache.is_null() {
    return uuidCache;
  }
  /* We were scanning /proc/partitions
   * and /proc/sys/dev/cdrom/info here.
   * Missed volume managers. I see that "standard" blkid uses these:
   * /dev/mapper/control
   * /proc/devices
   * /proc/evms/volumes
   * /proc/lvm/VGs
   * This is unacceptably complex. Let's just scan /dev.
   * (Maybe add scanning of /sys/block/XXX/dev for devices
   * somehow not having their /dev/XXX entries created?) */
  if scan_devices != 0 {
    crate::libbb::recursive_action::recursive_action(
      b"/dev\x00" as *const u8 as *const libc::c_char,
      ACTION_RECURSE as libc::c_int as libc::c_uint,
      Some(
        uuidcache_check_device
          as unsafe extern "C" fn(
            _: *const libc::c_char,
            _: *mut stat,
            _: *mut libc::c_void,
            _: libc::c_int,
          ) -> libc::c_int,
      ),
      None,
      0 as *mut libc::c_void,
      0 as libc::c_uint,
    );
  }
  return uuidCache;
}
// UNUSED
/* Used by blkid */
#[no_mangle]
pub unsafe extern "C" fn display_uuid_cache(mut scan_devices: libc::c_int) {
  let mut uc: *mut uuidCache_s = std::ptr::null_mut(); /* for compiler */
  uc = uuidcache_init(scan_devices);
  while !uc.is_null() {
    printf(b"%s:\x00" as *const u8 as *const libc::c_char, (*uc).device);
    if *(*uc).label.offset(0) != 0 {
      printf(
        b" LABEL=\"%s\"\x00" as *const u8 as *const libc::c_char,
        (*uc).label,
      );
    }
    if *(*uc).uc_uuid.offset(0) != 0 {
      printf(
        b" UUID=\"%s\"\x00" as *const u8 as *const libc::c_char,
        (*uc).uc_uuid,
      );
    }
    if !(*uc).type_0.is_null() {
      printf(
        b" TYPE=\"%s\"\x00" as *const u8 as *const libc::c_char,
        (*uc).type_0,
      );
    }
    crate::libbb::xfuncs_printf::bb_putchar('\n' as i32);
    uc = (*uc).next
  }
}
#[no_mangle]
pub unsafe extern "C" fn add_to_uuid_cache(mut device: *const libc::c_char) -> libc::c_int {
  let mut uuid: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  uuid = uuid;
  let mut label: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  label = label;
  let mut type_0: *const libc::c_char = std::ptr::null();
  type_0 = type_0;
  let mut fd: libc::c_int = 0;
  fd = open(device, 0);
  if fd < 0 {
    return 0;
  }
  /* get_label_uuid() closes fd in all cases (success & failure) */
  if get_label_uuid(fd, &mut label, &mut uuid, &mut type_0) == 0 {
    /* uuidcache_addentry() takes ownership of all four params */
    uuidcache_addentry(
      crate::libbb::xfuncs_printf::xstrdup(device),
      label,
      uuid,
      type_0,
    );
    return 1i32;
  }
  return 0;
}
/* Used by mount and findfs */
#[no_mangle]
pub unsafe extern "C" fn get_devname_from_label(
  mut spec: *const libc::c_char,
) -> *mut libc::c_char {
  let mut uc: *mut uuidCache_s = std::ptr::null_mut();
  uc = uuidcache_init(1i32);
  while !uc.is_null() {
    if *(*uc).label.offset(0) as libc::c_int != 0 && strcmp(spec, (*uc).label) == 0 {
      return crate::libbb::xfuncs_printf::xstrdup((*uc).device);
    }
    uc = (*uc).next
  }
  return std::ptr::null_mut::<libc::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn get_devname_from_uuid(mut spec: *const libc::c_char) -> *mut libc::c_char {
  let mut uc: *mut uuidCache_s = std::ptr::null_mut();
  uc = uuidcache_init(1i32);
  while !uc.is_null() {
    /* case of hex numbers doesn't matter */
    if strcasecmp(spec, (*uc).uc_uuid) == 0 {
      return crate::libbb::xfuncs_printf::xstrdup((*uc).device);
    }
    uc = (*uc).next
  }
  return std::ptr::null_mut::<libc::c_char>();
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
/* Returns:
 * 0: no UUID= or LABEL= prefix found
 * 1: UUID= or LABEL= prefix found. In this case,
 *    *fsname is replaced if device with such UUID or LABEL is found
 */
#[no_mangle]
pub unsafe extern "C" fn resolve_mount_spec(mut fsname: *mut *mut libc::c_char) -> libc::c_int {
  let mut tmp: *mut libc::c_char = *fsname; /* no UUID= or LABEL= prefix found */
  if !crate::libbb::compare_string_array::is_prefixed_with(
    *fsname,
    b"UUID=\x00" as *const u8 as *const libc::c_char,
  )
  .is_null()
  {
    tmp = get_devname_from_uuid((*fsname).offset(5))
  } else if !crate::libbb::compare_string_array::is_prefixed_with(
    *fsname,
    b"LABEL=\x00" as *const u8 as *const libc::c_char,
  )
  .is_null()
  {
    tmp = get_devname_from_label((*fsname).offset(6))
  }
  if tmp == *fsname {
    return 0;
  }
  if !tmp.is_null() {
    *fsname = tmp
  }
  return 1i32;
}

use libc;
extern "C" {
  #[no_mangle]
  fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
  #[no_mangle]
  fn volume_id_open_node(fd: libc::c_int) -> *mut volume_id;
  #[no_mangle]
  fn volume_id_probe_all(id: *mut volume_id, size: uint64_t) -> libc::c_int;
  #[no_mangle]
  fn free_volume_id(id: *mut volume_id);
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn gnu_dev_major(__dev: __dev_t) -> libc::c_uint;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xstrndup(s: *const libc::c_char, n: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn recursive_action(
    fileName: *const libc::c_char,
    flags: libc::c_uint,
    fileAction: Option<
      unsafe extern "C" fn(
        _: *const libc::c_char,
        _: *mut stat,
        _: *mut libc::c_void,
        _: libc::c_int,
      ) -> libc::c_int,
    >,
    dirAction: Option<
      unsafe extern "C" fn(
        _: *const libc::c_char,
        _: *mut stat,
        _: *mut libc::c_void,
        _: libc::c_int,
      ) -> libc::c_int,
    >,
    userData: *mut libc::c_void,
    depth: libc::c_uint,
  ) -> libc::c_int;
  #[no_mangle]
  fn bb_basename(name: *const libc::c_char) -> *const libc::c_char;
  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
  pub tv_sec: __time_t,
  pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
  pub st_dev: __dev_t,
  pub st_ino: __ino_t,
  pub st_nlink: __nlink_t,
  pub st_mode: __mode_t,
  pub st_uid: __uid_t,
  pub st_gid: __gid_t,
  pub __pad0: libc::c_int,
  pub st_rdev: __dev_t,
  pub st_size: __off_t,
  pub st_blksize: __blksize_t,
  pub st_blocks: __blkcnt_t,
  pub st_atim: timespec,
  pub st_mtim: timespec,
  pub st_ctim: timespec,
  pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type C2RustUnnamed = libc::c_uint;
pub const ACTION_DANGLING_OK: C2RustUnnamed = 64;
pub const ACTION_QUIET: C2RustUnnamed = 32;
pub const ACTION_DEPTHFIRST: C2RustUnnamed = 8;
pub const ACTION_FOLLOWLINKS_L0: C2RustUnnamed = 4;
pub const ACTION_FOLLOWLINKS: C2RustUnnamed = 2;
pub const ACTION_RECURSE: C2RustUnnamed = 1;
/* vi: set sw=4 ts=4: */
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuidCache_s {
  pub next: *mut uuidCache_s,
  pub device: *mut libc::c_char,
  pub label: *mut libc::c_char,
  pub uc_uuid: *mut libc::c_char,
  pub type_0: *const libc::c_char,
}
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
static mut uuidCache: *mut uuidCache_s = 0 as *const uuidCache_s as *mut uuidCache_s;
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
  let mut size: uint64_t = 0;
  let mut vid: *mut volume_id = 0 as *mut volume_id;
  /* fd is owned by vid now */
  vid = volume_id_open_node(fd); /* also closes fd */
  if ioctl(
    fd,
    (2u32 << 0i32 + 8i32 + 8i32 + 14i32
      | (0x12i32 << 0i32 + 8i32) as libc::c_uint
      | (114i32 << 0i32) as libc::c_uint) as libc::c_ulong
      | (::std::mem::size_of::<size_t>() as libc::c_ulong) << 0i32 + 8i32 + 8i32,
    &mut size as *mut uint64_t,
  ) != 0i32
  {
    size = 0i32 as uint64_t
  }
  if !(volume_id_probe_all(vid, size) != 0i32) {
    if (*vid).label[0] as libc::c_int != '\u{0}' as i32
      || (*vid).uuid[0] as libc::c_int != '\u{0}' as i32
      || !(*vid).type_0.is_null()
    {
      *label = xstrndup(
        (*vid).label.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 65]>() as libc::c_ulong as libc::c_int,
      );
      *uuid = xstrndup(
        (*vid).uuid.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 37]>() as libc::c_ulong as libc::c_int,
      );
      *type_0 = (*vid).type_0;
      rv = 0i32
    }
  }
  free_volume_id(vid);
  return rv;
}
/* NB: we take ownership of (malloc'ed) label and uuid */
unsafe extern "C" fn uuidcache_addentry(
  mut device: *mut libc::c_char,
  mut label: *mut libc::c_char,
  mut uuid: *mut libc::c_char,
  mut type_0: *const libc::c_char,
) {
  let mut last: *mut uuidCache_s = 0 as *mut uuidCache_s;
  if uuidCache.is_null() {
    uuidCache = xzalloc(::std::mem::size_of::<uuidCache_s>() as libc::c_ulong) as *mut uuidCache_s;
    last = uuidCache
  } else {
    last = uuidCache;
    while !(*last).next.is_null() {
      last = (*last).next
    }
    (*last).next =
      xzalloc(::std::mem::size_of::<uuidCache_s>() as libc::c_ulong) as *mut uuidCache_s;
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
        bb_basename(device),
        b"ubi\x00" as *const u8 as *const libc::c_char,
        3i32 as libc::c_ulong,
      ) == 0i32)
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
    recursive_action(
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
      0i32 as libc::c_uint,
    );
  }
  return uuidCache;
}
// UNUSED
/* Used by blkid */
#[no_mangle]
pub unsafe extern "C" fn display_uuid_cache(mut scan_devices: libc::c_int) {
  let mut uc: *mut uuidCache_s = 0 as *mut uuidCache_s; /* for compiler */
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
    bb_putchar('\n' as i32);
    uc = (*uc).next
  }
}
#[no_mangle]
pub unsafe extern "C" fn add_to_uuid_cache(mut device: *const libc::c_char) -> libc::c_int {
  let mut uuid: *mut libc::c_char = 0 as *mut libc::c_char;
  uuid = uuid;
  let mut label: *mut libc::c_char = 0 as *mut libc::c_char;
  label = label;
  let mut type_0: *const libc::c_char = 0 as *const libc::c_char;
  type_0 = type_0;
  let mut fd: libc::c_int = 0;
  fd = open(device, 0i32);
  if fd < 0i32 {
    return 0i32;
  }
  /* get_label_uuid() closes fd in all cases (success & failure) */
  if get_label_uuid(fd, &mut label, &mut uuid, &mut type_0) == 0i32 {
    /* uuidcache_addentry() takes ownership of all four params */
    uuidcache_addentry(xstrdup(device), label, uuid, type_0);
    return 1i32;
  }
  return 0i32;
}
/* Used by mount and findfs */
#[no_mangle]
pub unsafe extern "C" fn get_devname_from_label(
  mut spec: *const libc::c_char,
) -> *mut libc::c_char {
  let mut uc: *mut uuidCache_s = 0 as *mut uuidCache_s;
  uc = uuidcache_init(1i32);
  while !uc.is_null() {
    if *(*uc).label.offset(0) as libc::c_int != 0 && strcmp(spec, (*uc).label) == 0i32 {
      return xstrdup((*uc).device);
    }
    uc = (*uc).next
  }
  return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn get_devname_from_uuid(mut spec: *const libc::c_char) -> *mut libc::c_char {
  let mut uc: *mut uuidCache_s = 0 as *mut uuidCache_s;
  uc = uuidcache_init(1i32);
  while !uc.is_null() {
    /* case of hex numbers doesn't matter */
    if strcasecmp(spec, (*uc).uc_uuid) == 0i32 {
      return xstrdup((*uc).device);
    }
    uc = (*uc).next
  }
  return 0 as *mut libc::c_char;
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
  if !is_prefixed_with(*fsname, b"UUID=\x00" as *const u8 as *const libc::c_char).is_null() {
    tmp = get_devname_from_uuid((*fsname).offset(5))
  } else if !is_prefixed_with(*fsname, b"LABEL=\x00" as *const u8 as *const libc::c_char).is_null()
  {
    tmp = get_devname_from_label((*fsname).offset(6))
  }
  if tmp == *fsname {
    return 0i32;
  }
  if !tmp.is_null() {
    *fsname = tmp
  }
  return 1i32;
}

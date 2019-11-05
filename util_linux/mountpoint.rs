use libc;
extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn gnu_dev_major(__dev: __dev_t) -> libc::c_uint;
  #[no_mangle]
  fn gnu_dev_minor(__dev: __dev_t) -> libc::c_uint;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn find_block_device(path: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
}
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __ino64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type ino_t = __ino64_t;
use crate::librb::timespec;
use crate::librb::stat;
pub type dev_t = __dev_t;
/* vi: set sw=4 ts=4: */
/*
 * mountpoint implementation for busybox
 *
 * Copyright (C) 2005 Bernhard Reutner-Fischer
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 *
 * Based on sysvinit's mountpoint
 */
//config:config MOUNTPOINT
//config:	bool "mountpoint (4.9 kb)"
//config:	default y
//config:	help
//config:	mountpoint checks if the directory is a mountpoint.
//applet:IF_MOUNTPOINT(APPLET_NOEXEC(mountpoint, mountpoint, BB_DIR_BIN, BB_SUID_DROP, mountpoint))
//kbuild:lib-$(CONFIG_MOUNTPOINT) += mountpoint.o
//usage:#define mountpoint_trivial_usage
//usage:       "[-q] <[-dn] DIR | -x DEVICE>"
//usage:#define mountpoint_full_usage "\n\n"
//usage:       "Check if the directory is a mountpoint\n"
//usage:     "\n	-q	Quiet"
//usage:     "\n	-d	Print major/minor device number of the filesystem"
//usage:     "\n	-n	Print device name of the filesystem"
//usage:     "\n	-x	Print major/minor device number of the blockdevice"
//usage:
//usage:#define mountpoint_example_usage
//usage:       "$ mountpoint /proc\n"
//usage:       "/proc is not a mountpoint\n"
//usage:       "$ mountpoint /sys\n"
//usage:       "/sys is a mountpoint\n"
#[no_mangle]
pub unsafe extern "C" fn mountpoint_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut st: stat = stat {
    st_dev: 0,
    st_ino: 0,
    st_nlink: 0,
    st_mode: 0,
    st_uid: 0,
    st_gid: 0,
    __pad0: 0,
    st_rdev: 0,
    st_size: 0,
    st_blksize: 0,
    st_blocks: 0,
    st_atim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_mtim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_ctim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    __glibc_reserved: [0; 3],
  }; /* make perror_msg work as error_msg */
  let mut msg: *const libc::c_char = 0 as *const libc::c_char;
  let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut rc: libc::c_int = 0;
  let mut opt: libc::c_int = 0;
  opt = getopt32(argv, b"^qdxn\x00=1\x00" as *const u8 as *const libc::c_char) as libc::c_int;
  arg = *argv.offset(optind as isize);
  msg = b"%s\x00" as *const u8 as *const libc::c_char;
  rc = if opt & 4i32 != 0 {
    stat(arg, &mut st)
  } else {
    lstat(arg, &mut st)
  };
  if !(rc != 0i32) {
    if opt & 4i32 != 0 {
      if st.st_mode & 0o170000i32 as libc::c_uint == 0o60000i32 as libc::c_uint {
        printf(
          b"%u:%u\n\x00" as *const u8 as *const libc::c_char,
          gnu_dev_major(st.st_rdev),
          gnu_dev_minor(st.st_rdev),
        );
        return 0i32;
      }
      *bb_errno = 0i32;
      msg = b"%s: not a block device\x00" as *const u8 as *const libc::c_char
    } else {
      *bb_errno = 20i32;
      if st.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint {
        let mut st_dev: dev_t = st.st_dev;
        let mut st_ino: ino_t = st.st_ino;
        let mut p: *mut libc::c_char =
          xasprintf(b"%s/..\x00" as *const u8 as *const libc::c_char, arg);
        if stat(p, &mut st) == 0i32 {
          /* else: stat had set errno, just fall through */
          //int is_mnt = (st_dev != st.st_dev) || (st_dev == st.st_dev && st_ino == st.st_ino);
          let mut is_not_mnt: libc::c_int =
            (st_dev == st.st_dev && st_ino != st.st_ino) as libc::c_int;
          if opt & 2i32 != 0 {
            printf(
              b"%u:%u\n\x00" as *const u8 as *const libc::c_char,
              gnu_dev_major(st_dev),
              gnu_dev_minor(st_dev),
            );
          }
          if opt & 8i32 != 0 {
            let mut d: *const libc::c_char = find_block_device(arg);
            /* name is undefined, but device is mounted -> anonymous superblock! */
            /* happens with btrfs */
            if d.is_null() {
              d = b"UNKNOWN\x00" as *const u8 as *const libc::c_char
              /* TODO: iterate /proc/mounts, or /proc/self/mountinfo
               * to find out the device name */
            }
            printf(b"%s %s\n\x00" as *const u8 as *const libc::c_char, d, arg);
          }
          if opt & (1i32 | 2i32 | 8i32) == 0 {
            printf(
              b"%s is %sa mountpoint\n\x00" as *const u8 as *const libc::c_char,
              arg,
              if is_not_mnt != 0 {
                b"not \x00" as *const u8 as *const libc::c_char
              } else {
                b"\x00" as *const u8 as *const libc::c_char
              },
            );
          }
          return is_not_mnt;
        }
        arg = p
      }
    }
  }
  if opt & 1i32 == 0 {
    bb_perror_msg(msg, arg);
  }
  return 1i32;
}

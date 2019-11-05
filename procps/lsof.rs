use libc;
extern "C" {
  pub type __dirstream;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn getpid() -> __pid_t;
  #[no_mangle]
  fn opendir(__name: *const libc::c_char) -> *mut DIR;
  #[no_mangle]
  fn closedir(__dirp: *mut DIR) -> libc::c_int;
  #[no_mangle]
  fn readdir(__dirp: *mut DIR) -> *mut dirent;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn xmalloc_readlink(path: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn safe_strncpy(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    size: size_t,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn procps_scan(sp: *mut procps_status_t, flags: libc::c_int) -> *mut procps_status_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __ino64_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
use crate::librb::uint8_t;
pub type uint16_t = __uint16_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
  pub d_ino: __ino64_t,
  pub d_off: __off64_t,
  pub d_reclen: libc::c_ushort,
  pub d_type: libc::c_uchar,
  pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct smaprec {
  pub mapped_rw: libc::c_ulong,
  pub mapped_ro: libc::c_ulong,
  pub shared_clean: libc::c_ulong,
  pub shared_dirty: libc::c_ulong,
  pub private_clean: libc::c_ulong,
  pub private_dirty: libc::c_ulong,
  pub stack: libc::c_ulong,
  pub smap_pss: libc::c_ulong,
  pub smap_swap: libc::c_ulong,
  pub smap_size: libc::c_ulong,
  pub smap_start: libc::c_ulonglong,
  pub smap_mode: [libc::c_char; 5],
  pub smap_name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct procps_status_t {
  pub dir: *mut DIR,
  pub task_dir: *mut DIR,
  pub shift_pages_to_bytes: uint8_t,
  pub shift_pages_to_kb: uint8_t,
  pub argv_len: uint16_t,
  pub argv0: *mut libc::c_char,
  pub exe: *mut libc::c_char,
  pub main_thread_pid: libc::c_uint,
  pub vsz: libc::c_ulong,
  pub rss: libc::c_ulong,
  pub stime: libc::c_ulong,
  pub utime: libc::c_ulong,
  pub start_time: libc::c_ulong,
  pub pid: libc::c_uint,
  pub ppid: libc::c_uint,
  pub pgid: libc::c_uint,
  pub sid: libc::c_uint,
  pub uid: libc::c_uint,
  pub gid: libc::c_uint,
  pub ruid: libc::c_uint,
  pub rgid: libc::c_uint,
  pub niceness: libc::c_int,
  pub tty_major: libc::c_uint,
  pub tty_minor: libc::c_uint,
  pub smaps: smaprec,
  pub state: [libc::c_char; 4],
  pub comm: [libc::c_char; 16],
  pub last_seen_on_cpu: libc::c_int,
}
pub type C2RustUnnamed = libc::c_uint;
pub const PSSCAN_TASKS: C2RustUnnamed = 4194304;
pub const PSSCAN_RUIDGID: C2RustUnnamed = 2097152;
pub const PSSCAN_NICE: C2RustUnnamed = 1048576;
pub const PSSCAN_CPU: C2RustUnnamed = 524288;
pub const PSSCAN_START_TIME: C2RustUnnamed = 262144;
pub const PSSCAN_CONTEXT: C2RustUnnamed = 0;
pub const PSSCAN_ARGVN: C2RustUnnamed = 65536;
pub const PSSCAN_SMAPS: C2RustUnnamed = 32768;
pub const PSSCAN_TTY: C2RustUnnamed = 16384;
pub const PSSCAN_UTIME: C2RustUnnamed = 8192;
pub const PSSCAN_STIME: C2RustUnnamed = 4096;
pub const PSSCAN_RSS: C2RustUnnamed = 2048;
pub const PSSCAN_VSZ: C2RustUnnamed = 1024;
pub const PSSCAN_STATE: C2RustUnnamed = 512;
pub const PSSCAN_EXE: C2RustUnnamed = 256;
pub const PSSCAN_ARGV0: C2RustUnnamed = 128;
pub const PSSCAN_COMM: C2RustUnnamed = 32;
pub const PSSCAN_UIDGID: C2RustUnnamed = 16;
pub const PSSCAN_SID: C2RustUnnamed = 8;
pub const PSSCAN_PGID: C2RustUnnamed = 4;
pub const PSSCAN_PPID: C2RustUnnamed = 2;
pub const PSSCAN_PID: C2RustUnnamed = 1;
/* vi: set sw=4 ts=4: */
/*
 * Mini lsof implementation for busybox
 *
 * Copyright (C) 2012 by Sven Oliver 'SvOlli' Moll <svolli@svolli.de>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config LSOF
//config:	bool "lsof (3.4 kb)"
//config:	default y
//config:	help
//config:	Show open files in the format of:
//config:	PID <TAB> /path/to/executable <TAB> /path/to/opened/file
//applet:IF_LSOF(APPLET(lsof, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_LSOF) += lsof.o
//usage:#define lsof_trivial_usage
//usage:       ""
//usage:#define lsof_full_usage "\n\n"
//usage:       "Show all open files"
/*
 * Examples of "standard" lsof output:
 *
 * COMMAND    PID USER   FD   TYPE             DEVICE     SIZE       NODE NAME
 * init         1 root  cwd    DIR                8,5     4096          2 /
 * init         1 root  rtd    DIR                8,5     4096          2 /
 * init         1 root  txt    REG                8,5   872400      63408 /app/busybox-1.19.2/busybox
 * rpc.portm 1064 root  mem    REG                8,5    43494      47451 /app/glibc-2.11/lib/libnss_files-2.11.so
 * rpc.portm 1064 root    3u  IPv4               2178                 UDP *:111
 * rpc.portm 1064 root    4u  IPv4               1244                 TCP *:111 (LISTEN)
 * runsvdir  1116 root    0r   CHR                1,3                1214 /dev/null
 * runsvdir  1116 root    1w   CHR                1,3                1214 /dev/null
 * runsvdir  1116 root    2w   CHR                1,3                1214 /dev/null
 * runsvdir  1116 root    3r   DIR                8,6     1560      58359 /.local/var/service
 * gpm       1128 root    4u  unix 0xffff88007c09ccc0                1302 /dev/gpmctl
 */
#[no_mangle]
pub unsafe extern "C" fn lsof_main(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut proc_0: *mut procps_status_t = 0 as *mut procps_status_t;
  loop {
    proc_0 = procps_scan(
      proc_0,
      PSSCAN_PID as libc::c_int | PSSCAN_EXE as libc::c_int,
    );
    if proc_0.is_null() {
      break;
    }
    let mut name: [libc::c_char; 35] = [0; 35];
    let mut baseofs: libc::c_uint = 0;
    let mut d_fd: *mut DIR = 0 as *mut DIR;
    let mut fdlink: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut entry: *mut dirent = 0 as *mut dirent;
    if getpid() as libc::c_uint == (*proc_0).pid {
      continue;
    }
    baseofs = sprintf(
      name.as_mut_ptr(),
      b"/proc/%u/fd/\x00" as *const u8 as *const libc::c_char,
      (*proc_0).pid,
    ) as libc::c_uint;
    d_fd = opendir(name.as_mut_ptr());
    if !d_fd.is_null() {
      loop {
        entry = readdir(d_fd);
        if entry.is_null() {
          break;
        }
        /* Skip entries '.' and '..' (and any hidden file) */
        if (*entry).d_name[0] as libc::c_int == '.' as i32 {
          continue;
        }
        safe_strncpy(
          name.as_mut_ptr().offset(baseofs as isize),
          (*entry).d_name.as_mut_ptr(),
          10i32 as size_t,
        );
        fdlink = xmalloc_readlink(name.as_mut_ptr());
        if !fdlink.is_null() {
          printf(
            b"%d\t%s\t%s\n\x00" as *const u8 as *const libc::c_char,
            (*proc_0).pid,
            (*proc_0).exe,
            fdlink,
          );
          free(fdlink as *mut libc::c_void);
        }
      }
      closedir(d_fd);
    }
  }
  return 0i32;
}

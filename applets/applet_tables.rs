use libc;
extern "C" {
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strlen(_: *const libc::c_char) -> libc::c_ulong;
  #[no_mangle]
  static mut stdout: *mut _IO_FILE;
  #[no_mangle]
  static mut stderr: *mut _IO_FILE;
  #[no_mangle]
  fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn snprintf(
    _: *mut libc::c_char,
    _: libc::c_ulong,
    _: *const libc::c_char,
    _: ...
  ) -> libc::c_int;
  #[no_mangle]
  fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn getpid() -> __pid_t;
  #[no_mangle]
  fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type size_t = libc::c_ulong;
pub type __compar_fn_t =
  Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
  pub _flags: libc::c_int,
  pub _IO_read_ptr: *mut libc::c_char,
  pub _IO_read_end: *mut libc::c_char,
  pub _IO_read_base: *mut libc::c_char,
  pub _IO_write_base: *mut libc::c_char,
  pub _IO_write_ptr: *mut libc::c_char,
  pub _IO_write_end: *mut libc::c_char,
  pub _IO_buf_base: *mut libc::c_char,
  pub _IO_buf_end: *mut libc::c_char,
  pub _IO_save_base: *mut libc::c_char,
  pub _IO_backup_base: *mut libc::c_char,
  pub _IO_save_end: *mut libc::c_char,
  pub _markers: *mut _IO_marker,
  pub _chain: *mut _IO_FILE,
  pub _fileno: libc::c_int,
  pub _flags2: libc::c_int,
  pub _old_offset: __off_t,
  pub _cur_column: libc::c_ushort,
  pub _vtable_offset: libc::c_schar,
  pub _shortbuf: [libc::c_char; 1],
  pub _lock: *mut libc::c_void,
  pub _offset: __off64_t,
  pub __pad1: *mut libc::c_void,
  pub __pad2: *mut libc::c_void,
  pub __pad3: *mut libc::c_void,
  pub __pad4: *mut libc::c_void,
  pub __pad5: size_t,
  pub _mode: libc::c_int,
  pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
  pub _next: *mut _IO_marker,
  pub _sbuf: *mut _IO_FILE,
  pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type bb_install_loc_t = libc::c_uint;
pub const BB_DIR_USR_SBIN: bb_install_loc_t = 4;
pub const BB_DIR_USR_BIN: bb_install_loc_t = 3;
pub const BB_DIR_SBIN: bb_install_loc_t = 2;
pub const BB_DIR_BIN: bb_install_loc_t = 1;
pub const BB_DIR_ROOT: bb_install_loc_t = 0;
pub type bb_suid_t = libc::c_uint;
pub const BB_SUID_REQUIRE: bb_suid_t = 2;
pub const BB_SUID_MAYBE: bb_suid_t = 1;
pub const BB_SUID_DROP: bb_suid_t = 0;
/* vi: set sw=4 ts=4: */
/*
 * Applet table generator.
 * Runs on host and produces include/applet_tables.h
 *
 * Copyright (C) 2007 Denys Vlasenko <vda.linux@googlemail.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bb_applet {
  pub name: *const libc::c_char,
  pub main: *const libc::c_char,
  pub install_loc: bb_install_loc_t,
  pub need_suid: bb_suid_t,
  pub noexec: libc::c_uchar,
  pub nofork: libc::c_uchar,
}
/* Define struct bb_applet applets[] */
pub const NUM_APPLETS: libc::c_uint = 396;
/* DO NOT EDIT. This file is generated from applets.src.h */
/* vi: set sw=4 ts=4: */
/*
 * applets.h - a listing of all busybox applets.
 *
 * If you write a new applet, you need to add an entry to this list to make
 * busybox aware of it.
 */
/*
name  - applet name as it is typed on command line
help  - applet name, converted to C (ether-wake: help = ether_wake)
main  - corresponding <applet>_main to call (bzcat: main = bunzip2)
l     - location to install link to: [/usr]/[s]bin
s     - suid type:
        BB_SUID_REQUIRE: will complain if busybox isn't suid
        and is run by non-root (applet_main() will not be called at all)
        BB_SUID_DROP: will drop suid prior to applet_main()
        BB_SUID_MAYBE: neither of the above
        (every instance of BB_SUID_REQUIRE and BB_SUID_MAYBE
        needs to be justified in comment)
        NB: please update FEATURE_SUID help text whenever you add/remove
        BB_SUID_REQUIRE or BB_SUID_MAYBE applet.
*/
static mut applets: [bb_applet; 396] = [
  {
    let mut init = bb_applet {
      name: b"gunzip\x00" as *const u8 as *const libc::c_char,
      main: b"gunzip\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"zcat\x00" as *const u8 as *const libc::c_char,
      main: b"gunzip\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"bunzip2\x00" as *const u8 as *const libc::c_char,
      main: b"bunzip2\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"bzcat\x00" as *const u8 as *const libc::c_char,
      main: b"bunzip2\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"unlzma\x00" as *const u8 as *const libc::c_char,
      main: b"unlzma\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"lzcat\x00" as *const u8 as *const libc::c_char,
      main: b"unlzma\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"lzma\x00" as *const u8 as *const libc::c_char,
      main: b"unlzma\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"unxz\x00" as *const u8 as *const libc::c_char,
      main: b"unxz\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"xzcat\x00" as *const u8 as *const libc::c_char,
      main: b"unxz\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"xz\x00" as *const u8 as *const libc::c_char,
      main: b"unxz\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"bzip2\x00" as *const u8 as *const libc::c_char,
      main: b"bzip2\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"cpio\x00" as *const u8 as *const libc::c_char,
      main: b"cpio\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"dpkg\x00" as *const u8 as *const libc::c_char,
      main: b"dpkg\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"dpkg-deb\x00" as *const u8 as *const libc::c_char,
      main: b"dpkg_deb\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"gzip\x00" as *const u8 as *const libc::c_char,
      main: b"gzip\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"lzop\x00" as *const u8 as *const libc::c_char,
      main: b"lzop\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"rpm\x00" as *const u8 as *const libc::c_char,
      main: b"rpm\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"rpm2cpio\x00" as *const u8 as *const libc::c_char,
      main: b"rpm2cpio\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"tar\x00" as *const u8 as *const libc::c_char,
      main: b"tar\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"unzip\x00" as *const u8 as *const libc::c_char,
      main: b"unzip\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"chvt\x00" as *const u8 as *const libc::c_char,
      main: b"chvt\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"clear\x00" as *const u8 as *const libc::c_char,
      main: b"clear\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"deallocvt\x00" as *const u8 as *const libc::c_char,
      main: b"deallocvt\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"dumpkmap\x00" as *const u8 as *const libc::c_char,
      main: b"dumpkmap\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"fgconsole\x00" as *const u8 as *const libc::c_char,
      main: b"fgconsole\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"kbd_mode\x00" as *const u8 as *const libc::c_char,
      main: b"kbd_mode\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"loadfont\x00" as *const u8 as *const libc::c_char,
      main: b"loadfont\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"setfont\x00" as *const u8 as *const libc::c_char,
      main: b"setfont\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"loadkmap\x00" as *const u8 as *const libc::c_char,
      main: b"loadkmap\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"openvt\x00" as *const u8 as *const libc::c_char,
      main: b"openvt\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"reset\x00" as *const u8 as *const libc::c_char,
      main: b"reset\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"resize\x00" as *const u8 as *const libc::c_char,
      main: b"resize\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"setconsole\x00" as *const u8 as *const libc::c_char,
      main: b"setconsole\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"setkeycodes\x00" as *const u8 as *const libc::c_char,
      main: b"setkeycodes\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"setlogcons\x00" as *const u8 as *const libc::c_char,
      main: b"setlogcons\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"showkey\x00" as *const u8 as *const libc::c_char,
      main: b"showkey\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"basename\x00" as *const u8 as *const libc::c_char,
      main: b"basename\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"cat\x00" as *const u8 as *const libc::c_char,
      main: b"cat\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"chgrp\x00" as *const u8 as *const libc::c_char,
      main: b"chgrp\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"chmod\x00" as *const u8 as *const libc::c_char,
      main: b"chmod\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"chown\x00" as *const u8 as *const libc::c_char,
      main: b"chown\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"chroot\x00" as *const u8 as *const libc::c_char,
      main: b"chroot\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"cksum\x00" as *const u8 as *const libc::c_char,
      main: b"cksum\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"comm\x00" as *const u8 as *const libc::c_char,
      main: b"comm\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"cp\x00" as *const u8 as *const libc::c_char,
      main: b"cp\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"cut\x00" as *const u8 as *const libc::c_char,
      main: b"cut\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"date\x00" as *const u8 as *const libc::c_char,
      main: b"date\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"dd\x00" as *const u8 as *const libc::c_char,
      main: b"dd\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"df\x00" as *const u8 as *const libc::c_char,
      main: b"df\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"dirname\x00" as *const u8 as *const libc::c_char,
      main: b"dirname\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"dos2unix\x00" as *const u8 as *const libc::c_char,
      main: b"dos2unix\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"unix2dos\x00" as *const u8 as *const libc::c_char,
      main: b"dos2unix\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"du\x00" as *const u8 as *const libc::c_char,
      main: b"du\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"echo\x00" as *const u8 as *const libc::c_char,
      main: b"echo\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"env\x00" as *const u8 as *const libc::c_char,
      main: b"env\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"expand\x00" as *const u8 as *const libc::c_char,
      main: b"expand\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"unexpand\x00" as *const u8 as *const libc::c_char,
      main: b"expand\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"expr\x00" as *const u8 as *const libc::c_char,
      main: b"expr\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"factor\x00" as *const u8 as *const libc::c_char,
      main: b"factor\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"false\x00" as *const u8 as *const libc::c_char,
      main: b"false\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"fold\x00" as *const u8 as *const libc::c_char,
      main: b"fold\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"head\x00" as *const u8 as *const libc::c_char,
      main: b"head\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"hostid\x00" as *const u8 as *const libc::c_char,
      main: b"hostid\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"groups\x00" as *const u8 as *const libc::c_char,
      main: b"id\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"id\x00" as *const u8 as *const libc::c_char,
      main: b"id\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"install\x00" as *const u8 as *const libc::c_char,
      main: b"install\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"link\x00" as *const u8 as *const libc::c_char,
      main: b"link\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ln\x00" as *const u8 as *const libc::c_char,
      main: b"ln\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"logname\x00" as *const u8 as *const libc::c_char,
      main: b"logname\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ls\x00" as *const u8 as *const libc::c_char,
      main: b"ls\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"md5sum\x00" as *const u8 as *const libc::c_char,
      main: b"md5_sha1_sum\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"sha1sum\x00" as *const u8 as *const libc::c_char,
      main: b"md5_sha1_sum\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"sha3sum\x00" as *const u8 as *const libc::c_char,
      main: b"md5_sha1_sum\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"sha256sum\x00" as *const u8 as *const libc::c_char,
      main: b"md5_sha1_sum\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"sha512sum\x00" as *const u8 as *const libc::c_char,
      main: b"md5_sha1_sum\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"mkdir\x00" as *const u8 as *const libc::c_char,
      main: b"mkdir\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"mkfifo\x00" as *const u8 as *const libc::c_char,
      main: b"mkfifo\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"mknod\x00" as *const u8 as *const libc::c_char,
      main: b"mknod\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"mktemp\x00" as *const u8 as *const libc::c_char,
      main: b"mktemp\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"mv\x00" as *const u8 as *const libc::c_char,
      main: b"mv\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"nice\x00" as *const u8 as *const libc::c_char,
      main: b"nice\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"nl\x00" as *const u8 as *const libc::c_char,
      main: b"nl\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"nohup\x00" as *const u8 as *const libc::c_char,
      main: b"nohup\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"nproc\x00" as *const u8 as *const libc::c_char,
      main: b"nproc\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"od\x00" as *const u8 as *const libc::c_char,
      main: b"od\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"paste\x00" as *const u8 as *const libc::c_char,
      main: b"paste\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"printenv\x00" as *const u8 as *const libc::c_char,
      main: b"printenv\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"printf\x00" as *const u8 as *const libc::c_char,
      main: b"printf\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"pwd\x00" as *const u8 as *const libc::c_char,
      main: b"pwd\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"readlink\x00" as *const u8 as *const libc::c_char,
      main: b"readlink\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"realpath\x00" as *const u8 as *const libc::c_char,
      main: b"realpath\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"rm\x00" as *const u8 as *const libc::c_char,
      main: b"rm\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"rmdir\x00" as *const u8 as *const libc::c_char,
      main: b"rmdir\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"seq\x00" as *const u8 as *const libc::c_char,
      main: b"seq\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"shred\x00" as *const u8 as *const libc::c_char,
      main: b"shred\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"shuf\x00" as *const u8 as *const libc::c_char,
      main: b"shuf\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"sleep\x00" as *const u8 as *const libc::c_char,
      main: b"sleep\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"sort\x00" as *const u8 as *const libc::c_char,
      main: b"sort\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"split\x00" as *const u8 as *const libc::c_char,
      main: b"split\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"stat\x00" as *const u8 as *const libc::c_char,
      main: b"stat\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"stty\x00" as *const u8 as *const libc::c_char,
      main: b"stty\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"sum\x00" as *const u8 as *const libc::c_char,
      main: b"sum\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"sync\x00" as *const u8 as *const libc::c_char,
      main: b"sync\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"fsync\x00" as *const u8 as *const libc::c_char,
      main: b"fsync\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"tac\x00" as *const u8 as *const libc::c_char,
      main: b"tac\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"tail\x00" as *const u8 as *const libc::c_char,
      main: b"tail\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"tee\x00" as *const u8 as *const libc::c_char,
      main: b"tee\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"test\x00" as *const u8 as *const libc::c_char,
      main: b"test\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"[\x00" as *const u8 as *const libc::c_char,
      main: b"test\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"[[\x00" as *const u8 as *const libc::c_char,
      main: b"test\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"timeout\x00" as *const u8 as *const libc::c_char,
      main: b"timeout\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"touch\x00" as *const u8 as *const libc::c_char,
      main: b"touch\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"tr\x00" as *const u8 as *const libc::c_char,
      main: b"tr\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"true\x00" as *const u8 as *const libc::c_char,
      main: b"true\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"truncate\x00" as *const u8 as *const libc::c_char,
      main: b"truncate\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"tty\x00" as *const u8 as *const libc::c_char,
      main: b"tty\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"uname\x00" as *const u8 as *const libc::c_char,
      main: b"uname\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"arch\x00" as *const u8 as *const libc::c_char,
      main: b"uname\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"uniq\x00" as *const u8 as *const libc::c_char,
      main: b"uniq\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"unlink\x00" as *const u8 as *const libc::c_char,
      main: b"unlink\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"usleep\x00" as *const u8 as *const libc::c_char,
      main: b"usleep\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"uudecode\x00" as *const u8 as *const libc::c_char,
      main: b"uudecode\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"base64\x00" as *const u8 as *const libc::c_char,
      main: b"base64\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"uuencode\x00" as *const u8 as *const libc::c_char,
      main: b"uuencode\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"wc\x00" as *const u8 as *const libc::c_char,
      main: b"wc\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"users\x00" as *const u8 as *const libc::c_char,
      main: b"who\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"w\x00" as *const u8 as *const libc::c_char,
      main: b"who\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"who\x00" as *const u8 as *const libc::c_char,
      main: b"who\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"whoami\x00" as *const u8 as *const libc::c_char,
      main: b"whoami\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"yes\x00" as *const u8 as *const libc::c_char,
      main: b"yes\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"pipe_progress\x00" as *const u8 as *const libc::c_char,
      main: b"pipe_progress\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"run-parts\x00" as *const u8 as *const libc::c_char,
      main: b"run_parts\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"start-stop-daemon\x00" as *const u8 as *const libc::c_char,
      main: b"start_stop_daemon\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"which\x00" as *const u8 as *const libc::c_char,
      main: b"which\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"chattr\x00" as *const u8 as *const libc::c_char,
      main: b"chattr\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"fsck\x00" as *const u8 as *const libc::c_char,
      main: b"fsck\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"lsattr\x00" as *const u8 as *const libc::c_char,
      main: b"lsattr\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"awk\x00" as *const u8 as *const libc::c_char,
      main: b"awk\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"cmp\x00" as *const u8 as *const libc::c_char,
      main: b"cmp\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"diff\x00" as *const u8 as *const libc::c_char,
      main: b"diff\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ed\x00" as *const u8 as *const libc::c_char,
      main: b"ed\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"patch\x00" as *const u8 as *const libc::c_char,
      main: b"patch\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"sed\x00" as *const u8 as *const libc::c_char,
      main: b"sed\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"vi\x00" as *const u8 as *const libc::c_char,
      main: b"vi\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"find\x00" as *const u8 as *const libc::c_char,
      main: b"find\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"grep\x00" as *const u8 as *const libc::c_char,
      main: b"grep\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"egrep\x00" as *const u8 as *const libc::c_char,
      main: b"grep\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"fgrep\x00" as *const u8 as *const libc::c_char,
      main: b"grep\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"xargs\x00" as *const u8 as *const libc::c_char,
      main: b"xargs\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"bootchartd\x00" as *const u8 as *const libc::c_char,
      main: b"bootchartd\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"halt\x00" as *const u8 as *const libc::c_char,
      main: b"halt\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"poweroff\x00" as *const u8 as *const libc::c_char,
      main: b"halt\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"reboot\x00" as *const u8 as *const libc::c_char,
      main: b"halt\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"init\x00" as *const u8 as *const libc::c_char,
      main: b"init\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"linuxrc\x00" as *const u8 as *const libc::c_char,
      main: b"init\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_ROOT,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"nuke\x00" as *const u8 as *const libc::c_char,
      main: b"nuke\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"resume\x00" as *const u8 as *const libc::c_char,
      main: b"resume\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"add-shell\x00" as *const u8 as *const libc::c_char,
      main: b"add_remove_shell\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"remove-shell\x00" as *const u8 as *const libc::c_char,
      main: b"add_remove_shell\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"addgroup\x00" as *const u8 as *const libc::c_char,
      main: b"addgroup\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"adduser\x00" as *const u8 as *const libc::c_char,
      main: b"adduser\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"chpasswd\x00" as *const u8 as *const libc::c_char,
      main: b"chpasswd\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"cryptpw\x00" as *const u8 as *const libc::c_char,
      main: b"cryptpw\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"mkpasswd\x00" as *const u8 as *const libc::c_char,
      main: b"cryptpw\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"deluser\x00" as *const u8 as *const libc::c_char,
      main: b"deluser\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"delgroup\x00" as *const u8 as *const libc::c_char,
      main: b"deluser\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"getty\x00" as *const u8 as *const libc::c_char,
      main: b"getty\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"login\x00" as *const u8 as *const libc::c_char,
      main: b"login\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_REQUIRE,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"passwd\x00" as *const u8 as *const libc::c_char,
      main: b"passwd\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_REQUIRE,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"su\x00" as *const u8 as *const libc::c_char,
      main: b"su\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_REQUIRE,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"sulogin\x00" as *const u8 as *const libc::c_char,
      main: b"sulogin\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"vlock\x00" as *const u8 as *const libc::c_char,
      main: b"vlock\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_REQUIRE,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"makemime\x00" as *const u8 as *const libc::c_char,
      main: b"makemime\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"popmaildir\x00" as *const u8 as *const libc::c_char,
      main: b"popmaildir\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"reformime\x00" as *const u8 as *const libc::c_char,
      main: b"reformime\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"sendmail\x00" as *const u8 as *const libc::c_char,
      main: b"sendmail\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"adjtimex\x00" as *const u8 as *const libc::c_char,
      main: b"adjtimex\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"bc\x00" as *const u8 as *const libc::c_char,
      main: b"bc\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"dc\x00" as *const u8 as *const libc::c_char,
      main: b"dc\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"beep\x00" as *const u8 as *const libc::c_char,
      main: b"beep\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"chat\x00" as *const u8 as *const libc::c_char,
      main: b"chat\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"conspy\x00" as *const u8 as *const libc::c_char,
      main: b"conspy\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"crond\x00" as *const u8 as *const libc::c_char,
      main: b"crond\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"crontab\x00" as *const u8 as *const libc::c_char,
      main: b"crontab\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_REQUIRE,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"devmem\x00" as *const u8 as *const libc::c_char,
      main: b"devmem\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"fbsplash\x00" as *const u8 as *const libc::c_char,
      main: b"fbsplash\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"hdparm\x00" as *const u8 as *const libc::c_char,
      main: b"hdparm\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"hexedit\x00" as *const u8 as *const libc::c_char,
      main: b"hexedit\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"i2cget\x00" as *const u8 as *const libc::c_char,
      main: b"i2cget\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"i2cset\x00" as *const u8 as *const libc::c_char,
      main: b"i2cset\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"i2cdump\x00" as *const u8 as *const libc::c_char,
      main: b"i2cdump\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"i2cdetect\x00" as *const u8 as *const libc::c_char,
      main: b"i2cdetect\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"i2ctransfer\x00" as *const u8 as *const libc::c_char,
      main: b"i2ctransfer\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"less\x00" as *const u8 as *const libc::c_char,
      main: b"less\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"lsscsi\x00" as *const u8 as *const libc::c_char,
      main: b"lsscsi\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"makedevs\x00" as *const u8 as *const libc::c_char,
      main: b"makedevs\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"man\x00" as *const u8 as *const libc::c_char,
      main: b"man\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"microcom\x00" as *const u8 as *const libc::c_char,
      main: b"microcom\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"mt\x00" as *const u8 as *const libc::c_char,
      main: b"mt\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"nandwrite\x00" as *const u8 as *const libc::c_char,
      main: b"nandwrite\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"nanddump\x00" as *const u8 as *const libc::c_char,
      main: b"nandwrite\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"partprobe\x00" as *const u8 as *const libc::c_char,
      main: b"partprobe\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"raidautorun\x00" as *const u8 as *const libc::c_char,
      main: b"raidautorun\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"readahead\x00" as *const u8 as *const libc::c_char,
      main: b"readahead\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"runlevel\x00" as *const u8 as *const libc::c_char,
      main: b"runlevel\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"rx\x00" as *const u8 as *const libc::c_char,
      main: b"rx\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"setfattr\x00" as *const u8 as *const libc::c_char,
      main: b"setfattr\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"setserial\x00" as *const u8 as *const libc::c_char,
      main: b"setserial\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"strings\x00" as *const u8 as *const libc::c_char,
      main: b"strings\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"time\x00" as *const u8 as *const libc::c_char,
      main: b"time\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ts\x00" as *const u8 as *const libc::c_char,
      main: b"ts\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ttysize\x00" as *const u8 as *const libc::c_char,
      main: b"ttysize\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ubiattach\x00" as *const u8 as *const libc::c_char,
      main: b"ubi_tools\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ubidetach\x00" as *const u8 as *const libc::c_char,
      main: b"ubi_tools\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ubimkvol\x00" as *const u8 as *const libc::c_char,
      main: b"ubi_tools\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ubirmvol\x00" as *const u8 as *const libc::c_char,
      main: b"ubi_tools\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ubirsvol\x00" as *const u8 as *const libc::c_char,
      main: b"ubi_tools\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ubiupdatevol\x00" as *const u8 as *const libc::c_char,
      main: b"ubi_tools\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ubirename\x00" as *const u8 as *const libc::c_char,
      main: b"ubirename\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"volname\x00" as *const u8 as *const libc::c_char,
      main: b"volname\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"watchdog\x00" as *const u8 as *const libc::c_char,
      main: b"watchdog\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"modinfo\x00" as *const u8 as *const libc::c_char,
      main: b"modinfo\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"lsmod\x00" as *const u8 as *const libc::c_char,
      main: b"lsmod\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"modprobe\x00" as *const u8 as *const libc::c_char,
      main: b"modprobe\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"depmod\x00" as *const u8 as *const libc::c_char,
      main: b"modprobe\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"insmod\x00" as *const u8 as *const libc::c_char,
      main: b"modprobe\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"rmmod\x00" as *const u8 as *const libc::c_char,
      main: b"modprobe\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"arp\x00" as *const u8 as *const libc::c_char,
      main: b"arp\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"arping\x00" as *const u8 as *const libc::c_char,
      main: b"arping\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"brctl\x00" as *const u8 as *const libc::c_char,
      main: b"brctl\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"dnsd\x00" as *const u8 as *const libc::c_char,
      main: b"dnsd\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ether-wake\x00" as *const u8 as *const libc::c_char,
      main: b"ether_wake\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ftpd\x00" as *const u8 as *const libc::c_char,
      main: b"ftpd\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ftpget\x00" as *const u8 as *const libc::c_char,
      main: b"ftpgetput\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ftpput\x00" as *const u8 as *const libc::c_char,
      main: b"ftpgetput\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"dnsdomainname\x00" as *const u8 as *const libc::c_char,
      main: b"hostname\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"hostname\x00" as *const u8 as *const libc::c_char,
      main: b"hostname\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"httpd\x00" as *const u8 as *const libc::c_char,
      main: b"httpd\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ifconfig\x00" as *const u8 as *const libc::c_char,
      main: b"ifconfig\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ifenslave\x00" as *const u8 as *const libc::c_char,
      main: b"ifenslave\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ifplugd\x00" as *const u8 as *const libc::c_char,
      main: b"ifplugd\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ifup\x00" as *const u8 as *const libc::c_char,
      main: b"ifupdown\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ifdown\x00" as *const u8 as *const libc::c_char,
      main: b"ifupdown\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"inetd\x00" as *const u8 as *const libc::c_char,
      main: b"inetd\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ip\x00" as *const u8 as *const libc::c_char,
      main: b"ip\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ipaddr\x00" as *const u8 as *const libc::c_char,
      main: b"ipaddr\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"iplink\x00" as *const u8 as *const libc::c_char,
      main: b"iplink\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"iproute\x00" as *const u8 as *const libc::c_char,
      main: b"iproute\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"iprule\x00" as *const u8 as *const libc::c_char,
      main: b"iprule\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"iptunnel\x00" as *const u8 as *const libc::c_char,
      main: b"iptunnel\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ipneigh\x00" as *const u8 as *const libc::c_char,
      main: b"ipneigh\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ipcalc\x00" as *const u8 as *const libc::c_char,
      main: b"ipcalc\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"fakeidentd\x00" as *const u8 as *const libc::c_char,
      main: b"fakeidentd\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"nameif\x00" as *const u8 as *const libc::c_char,
      main: b"nameif\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"nbd-client\x00" as *const u8 as *const libc::c_char,
      main: b"nbdclient\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"nc\x00" as *const u8 as *const libc::c_char,
      main: b"nc\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"netstat\x00" as *const u8 as *const libc::c_char,
      main: b"netstat\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"nslookup\x00" as *const u8 as *const libc::c_char,
      main: b"nslookup\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ntpd\x00" as *const u8 as *const libc::c_char,
      main: b"ntpd\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ping\x00" as *const u8 as *const libc::c_char,
      main: b"ping\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_MAYBE,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ping6\x00" as *const u8 as *const libc::c_char,
      main: b"ping6\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_MAYBE,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"pscan\x00" as *const u8 as *const libc::c_char,
      main: b"pscan\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"route\x00" as *const u8 as *const libc::c_char,
      main: b"route\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"slattach\x00" as *const u8 as *const libc::c_char,
      main: b"slattach\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ssl_client\x00" as *const u8 as *const libc::c_char,
      main: b"ssl_client\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"tc\x00" as *const u8 as *const libc::c_char,
      main: b"tc\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"tcpsvd\x00" as *const u8 as *const libc::c_char,
      main: b"tcpudpsvd\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"udpsvd\x00" as *const u8 as *const libc::c_char,
      main: b"tcpudpsvd\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"telnet\x00" as *const u8 as *const libc::c_char,
      main: b"telnet\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"telnetd\x00" as *const u8 as *const libc::c_char,
      main: b"telnetd\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"tftp\x00" as *const u8 as *const libc::c_char,
      main: b"tftp\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"tftpd\x00" as *const u8 as *const libc::c_char,
      main: b"tftpd\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"traceroute\x00" as *const u8 as *const libc::c_char,
      main: b"traceroute\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_MAYBE,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"traceroute6\x00" as *const u8 as *const libc::c_char,
      main: b"traceroute6\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_MAYBE,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"tunctl\x00" as *const u8 as *const libc::c_char,
      main: b"tunctl\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"vconfig\x00" as *const u8 as *const libc::c_char,
      main: b"vconfig\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"wget\x00" as *const u8 as *const libc::c_char,
      main: b"wget\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"whois\x00" as *const u8 as *const libc::c_char,
      main: b"whois\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"zcip\x00" as *const u8 as *const libc::c_char,
      main: b"zcip\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"lpd\x00" as *const u8 as *const libc::c_char,
      main: b"lpd\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"lpq\x00" as *const u8 as *const libc::c_char,
      main: b"lpqr\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"lpr\x00" as *const u8 as *const libc::c_char,
      main: b"lpqr\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"free\x00" as *const u8 as *const libc::c_char,
      main: b"free\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"fuser\x00" as *const u8 as *const libc::c_char,
      main: b"fuser\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"iostat\x00" as *const u8 as *const libc::c_char,
      main: b"iostat\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"kill\x00" as *const u8 as *const libc::c_char,
      main: b"kill\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"killall\x00" as *const u8 as *const libc::c_char,
      main: b"kill\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"killall5\x00" as *const u8 as *const libc::c_char,
      main: b"kill\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"lsof\x00" as *const u8 as *const libc::c_char,
      main: b"lsof\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"mpstat\x00" as *const u8 as *const libc::c_char,
      main: b"mpstat\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"nmeter\x00" as *const u8 as *const libc::c_char,
      main: b"nmeter\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"pgrep\x00" as *const u8 as *const libc::c_char,
      main: b"pgrep\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"pkill\x00" as *const u8 as *const libc::c_char,
      main: b"pgrep\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"pidof\x00" as *const u8 as *const libc::c_char,
      main: b"pidof\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"pmap\x00" as *const u8 as *const libc::c_char,
      main: b"pmap\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"powertop\x00" as *const u8 as *const libc::c_char,
      main: b"powertop\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ps\x00" as *const u8 as *const libc::c_char,
      main: b"ps\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"pstree\x00" as *const u8 as *const libc::c_char,
      main: b"pstree\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"pwdx\x00" as *const u8 as *const libc::c_char,
      main: b"pwdx\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"smemcap\x00" as *const u8 as *const libc::c_char,
      main: b"smemcap\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"sysctl\x00" as *const u8 as *const libc::c_char,
      main: b"sysctl\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"top\x00" as *const u8 as *const libc::c_char,
      main: b"top\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"uptime\x00" as *const u8 as *const libc::c_char,
      main: b"uptime\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"watch\x00" as *const u8 as *const libc::c_char,
      main: b"watch\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"chpst\x00" as *const u8 as *const libc::c_char,
      main: b"chpst\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"envdir\x00" as *const u8 as *const libc::c_char,
      main: b"chpst\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"envuidgid\x00" as *const u8 as *const libc::c_char,
      main: b"chpst\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"setuidgid\x00" as *const u8 as *const libc::c_char,
      main: b"chpst\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"softlimit\x00" as *const u8 as *const libc::c_char,
      main: b"chpst\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"runsv\x00" as *const u8 as *const libc::c_char,
      main: b"runsv\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"runsvdir\x00" as *const u8 as *const libc::c_char,
      main: b"runsvdir\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"sv\x00" as *const u8 as *const libc::c_char,
      main: b"sv\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"svc\x00" as *const u8 as *const libc::c_char,
      main: b"svc\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"svok\x00" as *const u8 as *const libc::c_char,
      main: b"svok\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"svlogd\x00" as *const u8 as *const libc::c_char,
      main: b"svlogd\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ash\x00" as *const u8 as *const libc::c_char,
      main: b"ash\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"sh\x00" as *const u8 as *const libc::c_char,
      main: b"ash\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"cttyhack\x00" as *const u8 as *const libc::c_char,
      main: b"cttyhack\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"hush\x00" as *const u8 as *const libc::c_char,
      main: b"hush\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"klogd\x00" as *const u8 as *const libc::c_char,
      main: b"klogd\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"logger\x00" as *const u8 as *const libc::c_char,
      main: b"logger\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"logread\x00" as *const u8 as *const libc::c_char,
      main: b"logread\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"syslogd\x00" as *const u8 as *const libc::c_char,
      main: b"syslogd\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"acpid\x00" as *const u8 as *const libc::c_char,
      main: b"acpid\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"blkdiscard\x00" as *const u8 as *const libc::c_char,
      main: b"blkdiscard\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"blkid\x00" as *const u8 as *const libc::c_char,
      main: b"blkid\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"blockdev\x00" as *const u8 as *const libc::c_char,
      main: b"blockdev\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"cal\x00" as *const u8 as *const libc::c_char,
      main: b"cal\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"chrt\x00" as *const u8 as *const libc::c_char,
      main: b"chrt\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"dmesg\x00" as *const u8 as *const libc::c_char,
      main: b"dmesg\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"eject\x00" as *const u8 as *const libc::c_char,
      main: b"eject\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"fallocate\x00" as *const u8 as *const libc::c_char,
      main: b"fallocate\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"fatattr\x00" as *const u8 as *const libc::c_char,
      main: b"fatattr\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"fbset\x00" as *const u8 as *const libc::c_char,
      main: b"fbset\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"fdformat\x00" as *const u8 as *const libc::c_char,
      main: b"fdformat\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"fdisk\x00" as *const u8 as *const libc::c_char,
      main: b"fdisk\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"findfs\x00" as *const u8 as *const libc::c_char,
      main: b"findfs\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_MAYBE,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"flock\x00" as *const u8 as *const libc::c_char,
      main: b"flock\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"fdflush\x00" as *const u8 as *const libc::c_char,
      main: b"freeramdisk\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"freeramdisk\x00" as *const u8 as *const libc::c_char,
      main: b"freeramdisk\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"fsck.minix\x00" as *const u8 as *const libc::c_char,
      main: b"fsck_minix\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"fsfreeze\x00" as *const u8 as *const libc::c_char,
      main: b"fsfreeze\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"fstrim\x00" as *const u8 as *const libc::c_char,
      main: b"fstrim\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"getopt\x00" as *const u8 as *const libc::c_char,
      main: b"getopt\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"hexdump\x00" as *const u8 as *const libc::c_char,
      main: b"hexdump\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"hd\x00" as *const u8 as *const libc::c_char,
      main: b"hexdump\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"xxd\x00" as *const u8 as *const libc::c_char,
      main: b"xxd\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"hwclock\x00" as *const u8 as *const libc::c_char,
      main: b"hwclock\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ionice\x00" as *const u8 as *const libc::c_char,
      main: b"ionice\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ipcrm\x00" as *const u8 as *const libc::c_char,
      main: b"ipcrm\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"ipcs\x00" as *const u8 as *const libc::c_char,
      main: b"ipcs\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"last\x00" as *const u8 as *const libc::c_char,
      main: b"last\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"losetup\x00" as *const u8 as *const libc::c_char,
      main: b"losetup\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"lspci\x00" as *const u8 as *const libc::c_char,
      main: b"lspci\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"lsusb\x00" as *const u8 as *const libc::c_char,
      main: b"lsusb\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"mdev\x00" as *const u8 as *const libc::c_char,
      main: b"mdev\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"mesg\x00" as *const u8 as *const libc::c_char,
      main: b"mesg\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"mke2fs\x00" as *const u8 as *const libc::c_char,
      main: b"mkfs_ext2\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"mkfs.ext2\x00" as *const u8 as *const libc::c_char,
      main: b"mkfs_ext2\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"mkfs.minix\x00" as *const u8 as *const libc::c_char,
      main: b"mkfs_minix\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"mkdosfs\x00" as *const u8 as *const libc::c_char,
      main: b"mkfs_vfat\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"mkfs.vfat\x00" as *const u8 as *const libc::c_char,
      main: b"mkfs_vfat\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"mkswap\x00" as *const u8 as *const libc::c_char,
      main: b"mkswap\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"more\x00" as *const u8 as *const libc::c_char,
      main: b"more\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"mount\x00" as *const u8 as *const libc::c_char,
      main: b"mount\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_MAYBE,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"mountpoint\x00" as *const u8 as *const libc::c_char,
      main: b"mountpoint\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"nologin\x00" as *const u8 as *const libc::c_char,
      main: b"scripted\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"nsenter\x00" as *const u8 as *const libc::c_char,
      main: b"nsenter\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"pivot_root\x00" as *const u8 as *const libc::c_char,
      main: b"pivot_root\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 1i32 as libc::c_uchar,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"rdate\x00" as *const u8 as *const libc::c_char,
      main: b"rdate\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"rdev\x00" as *const u8 as *const libc::c_char,
      main: b"rdev\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"readprofile\x00" as *const u8 as *const libc::c_char,
      main: b"readprofile\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"renice\x00" as *const u8 as *const libc::c_char,
      main: b"renice\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"rev\x00" as *const u8 as *const libc::c_char,
      main: b"rev\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"rtcwake\x00" as *const u8 as *const libc::c_char,
      main: b"rtcwake\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"script\x00" as *const u8 as *const libc::c_char,
      main: b"script\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"scriptreplay\x00" as *const u8 as *const libc::c_char,
      main: b"scriptreplay\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"setarch\x00" as *const u8 as *const libc::c_char,
      main: b"setarch\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"linux32\x00" as *const u8 as *const libc::c_char,
      main: b"setarch\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"linux64\x00" as *const u8 as *const libc::c_char,
      main: b"setarch\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"setpriv\x00" as *const u8 as *const libc::c_char,
      main: b"setpriv\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"setsid\x00" as *const u8 as *const libc::c_char,
      main: b"setsid\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"swapon\x00" as *const u8 as *const libc::c_char,
      main: b"swap_on_off\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"swapoff\x00" as *const u8 as *const libc::c_char,
      main: b"swap_on_off\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"switch_root\x00" as *const u8 as *const libc::c_char,
      main: b"switch_root\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"run-init\x00" as *const u8 as *const libc::c_char,
      main: b"switch_root\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"taskset\x00" as *const u8 as *const libc::c_char,
      main: b"taskset\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"uevent\x00" as *const u8 as *const libc::c_char,
      main: b"uevent\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"umount\x00" as *const u8 as *const libc::c_char,
      main: b"umount\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"unshare\x00" as *const u8 as *const libc::c_char,
      main: b"unshare\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"wall\x00" as *const u8 as *const libc::c_char,
      main: b"wall\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_REQUIRE,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"udhcpc6\x00" as *const u8 as *const libc::c_char,
      main: b"udhcpc6\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"udhcpc\x00" as *const u8 as *const libc::c_char,
      main: b"udhcpc\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"udhcpd\x00" as *const u8 as *const libc::c_char,
      main: b"udhcpd\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"dhcprelay\x00" as *const u8 as *const libc::c_char,
      main: b"dhcprelay\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_SBIN,
      need_suid: BB_SUID_DROP,
      noexec: 0,
      nofork: 0,
    };
    init
  },
  {
    let mut init = bb_applet {
      name: b"dumpleases\x00" as *const u8 as *const libc::c_char,
      main: b"dumpleases\x00" as *const u8 as *const libc::c_char,
      install_loc: BB_DIR_USR_BIN,
      need_suid: BB_SUID_DROP,
      noexec: 1i32 as libc::c_uchar,
      nofork: 0,
    };
    init
  },
];
unsafe extern "C" fn cmp_name(
  mut a: *const libc::c_void,
  mut b: *const libc::c_void,
) -> libc::c_int {
  let mut aa: *const bb_applet = a as *const bb_applet;
  let mut bb: *const bb_applet = b as *const bb_applet;
  return strcmp((*aa).name, (*bb).name);
}
unsafe extern "C" fn str_isalnum_(mut s: *const libc::c_char) -> libc::c_int {
  while *s != 0 {
    if *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
      & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
      == 0
      && *s as libc::c_int != '_' as i32
    {
      return 0i32;
    }
    s = s.offset(1)
  }
  return 1i32;
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  let mut tmp1: [libc::c_char; 4096] = [0; 4096];
  let mut tmp2: [libc::c_char; 4096] = [0; 4096];
  // In find_applet_by_name(), before linear search, narrow it down
  // by looking at N "equidistant" names. With ~350 applets:
  // KNOWN_APPNAME_OFFSETS  cycles
  //                     0    9057
  //                     2    4604 + ~100 bytes of code
  //                     4    2407 + 4 bytes
  //                     8    1342 + 8 bytes
  //                    16     908 + 16 bytes
  //                    32     884 + 32 bytes
  // With 8, int16_t applet_nameofs[] table has 7 elements.
  let mut KNOWN_APPNAME_OFFSETS: libc::c_int = 8i32;
  // With 128 applets we do two linear searches, with 1..7 strcmp's in the first one
  // and 1..16 strcmp's in the second. With 256 apps, second search does 1..32 strcmp's.
  if (NUM_APPLETS as libc::c_int) < 128i32 {
    KNOWN_APPNAME_OFFSETS = 4i32
  }
  if (NUM_APPLETS as libc::c_int) < 32i32 {
    KNOWN_APPNAME_OFFSETS = 0i32
  }
  qsort(
    applets.as_mut_ptr() as *mut libc::c_void,
    NUM_APPLETS as libc::c_int as size_t,
    ::std::mem::size_of::<bb_applet>() as libc::c_ulong,
    Some(
      cmp_name
        as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int,
    ),
  );
  j = 0i32;
  i = j;
  while i < NUM_APPLETS as libc::c_int - 1i32 {
    if cmp_name(
      applets.as_mut_ptr().offset(i as isize) as *const libc::c_void,
      applets.as_mut_ptr().offset(i as isize).offset(1) as *const libc::c_void,
    ) == 0i32
    {
      fprintf(
        stderr,
        b"%s: duplicate applet name \'%s\'\n\x00" as *const u8 as *const libc::c_char,
        *argv.offset(0),
        applets[i as usize].name,
      );
      j = 1i32
    }
    i += 1
  }
  if j != 0i32 || (*argv.offset(1)).is_null() {
    return 1i32;
  }
  snprintf(
    tmp1.as_mut_ptr(),
    4096i32 as libc::c_ulong,
    b"%s.%u.new\x00" as *const u8 as *const libc::c_char,
    *argv.offset(1),
    getpid(),
  );
  i = open(tmp1.as_mut_ptr(), 0o1i32 | 0o1000i32 | 0o100i32, 0o666i32);
  if i < 0i32 {
    return 1i32;
  }
  dup2(i, 1i32);
  /* Keep in sync with include/busybox.h! */
  printf(
    b"/* This is a generated file, don\'t edit */\n\n\x00" as *const u8 as *const libc::c_char,
  );
  printf(
    b"#define NUM_APPLETS %u\n\x00" as *const u8 as *const libc::c_char,
    NUM_APPLETS as libc::c_int,
  );
  if NUM_APPLETS as libc::c_int == 1i32 {
    printf(
      b"#define SINGLE_APPLET_STR \"%s\"\n\x00" as *const u8 as *const libc::c_char,
      applets[0].name,
    );
    printf(
      b"#define SINGLE_APPLET_MAIN %s_main\n\x00" as *const u8 as *const libc::c_char,
      applets[0].main,
    );
  }
  printf(
    b"#define KNOWN_APPNAME_OFFSETS %u\n\n\x00" as *const u8 as *const libc::c_char,
    KNOWN_APPNAME_OFFSETS,
  );
  if KNOWN_APPNAME_OFFSETS > 0i32 {
    let mut ofs: libc::c_int = 0;
    let vla = KNOWN_APPNAME_OFFSETS as usize;
    let mut offset: Vec<libc::c_int> = ::std::vec::from_elem(0, vla);
    let vla_0 = KNOWN_APPNAME_OFFSETS as usize;
    let mut index: Vec<libc::c_int> = ::std::vec::from_elem(0, vla_0);
    i = 0i32;
    while i < KNOWN_APPNAME_OFFSETS {
      *index.as_mut_ptr().offset(i as isize) =
        i * NUM_APPLETS as libc::c_int / KNOWN_APPNAME_OFFSETS;
      i += 1
    }
    ofs = 0i32;
    i = 0i32;
    while i < NUM_APPLETS as libc::c_int {
      j = 0i32;
      while j < KNOWN_APPNAME_OFFSETS {
        if i == *index.as_mut_ptr().offset(j as isize) {
          *offset.as_mut_ptr().offset(j as isize) = ofs
        }
        j += 1
      }
      ofs = (ofs as libc::c_ulong)
        .wrapping_add(strlen(applets[i as usize].name).wrapping_add(1i32 as libc::c_ulong))
        as libc::c_int as libc::c_int;
      i += 1
    }
    /* If the list of names is too long refuse to proceed */
    if ofs > 0xffffi32 {
      return 1i32;
    }
    printf(b"const uint16_t applet_nameofs[] ALIGN2 = {\n\x00" as *const u8 as *const libc::c_char);
    i = 1i32;
    while i < KNOWN_APPNAME_OFFSETS {
      printf(
        b"%d,\n\x00" as *const u8 as *const libc::c_char,
        *offset.as_mut_ptr().offset(i as isize),
      );
      i += 1
    }
    printf(b"};\n\n\x00" as *const u8 as *const libc::c_char);
  }
  //printf("#ifndef SKIP_definitions\n");
  printf(b"const char applet_names[] ALIGN1 = \"\"\n\x00" as *const u8 as *const libc::c_char);
  i = 0i32;
  while i < NUM_APPLETS as libc::c_int {
    printf(
      b"\"%s\" \"\\0\"\n\x00" as *const u8 as *const libc::c_char,
      applets[i as usize].name,
    );
    i += 1
    //		if (MAX_APPLET_NAME_LEN < strlen(applets[i].name))
    //			MAX_APPLET_NAME_LEN = strlen(applets[i].name);
  } /* 2 bits */
  printf(b";\n\n\x00" as *const u8 as *const libc::c_char); /* 3 bits */
  i = 0i32; /* 3 bits */
  while i < NUM_APPLETS as libc::c_int {
    if str_isalnum_(applets[i as usize].name) != 0 {
      printf(
        b"#define APPLET_NO_%s %d\n\x00" as *const u8 as *const libc::c_char,
        applets[i as usize].name,
        i,
      );
    }
    i += 1
  }
  printf(b"\n\x00" as *const u8 as *const libc::c_char);
  printf(b"#ifndef SKIP_applet_main\n\x00" as *const u8 as *const libc::c_char);
  printf(
    b"int (*const applet_main[])(int argc, char **argv) = {\n\x00" as *const u8
      as *const libc::c_char,
  );
  i = 0i32;
  while i < NUM_APPLETS as libc::c_int {
    printf(
      b"%s_main,\n\x00" as *const u8 as *const libc::c_char,
      applets[i as usize].main,
    );
    i += 1
  }
  printf(b"};\n\x00" as *const u8 as *const libc::c_char);
  printf(b"#endif\n\n\x00" as *const u8 as *const libc::c_char);
  printf(b"const uint8_t applet_suid[] ALIGN1 = {\n\x00" as *const u8 as *const libc::c_char);
  i = 0i32;
  while i < NUM_APPLETS as libc::c_int {
    let mut v: libc::c_int = applets[i as usize].need_suid as libc::c_int;
    i += 1;
    if i < NUM_APPLETS as libc::c_int {
      v =
        (v as libc::c_uint | (applets[i as usize].need_suid as libc::c_uint) << 2i32) as libc::c_int
    }
    i += 1;
    if i < NUM_APPLETS as libc::c_int {
      v =
        (v as libc::c_uint | (applets[i as usize].need_suid as libc::c_uint) << 4i32) as libc::c_int
    }
    i += 1;
    if i < NUM_APPLETS as libc::c_int {
      v =
        (v as libc::c_uint | (applets[i as usize].need_suid as libc::c_uint) << 6i32) as libc::c_int
    }
    printf(b"0x%02x,\n\x00" as *const u8 as *const libc::c_char, v);
    i += 1
  }
  printf(b"};\n\n\x00" as *const u8 as *const libc::c_char);
  printf(
    b"const uint8_t applet_install_loc[] ALIGN1 = {\n\x00" as *const u8 as *const libc::c_char,
  );
  i = 0i32;
  while i < NUM_APPLETS as libc::c_int {
    let mut v_0: libc::c_int = applets[i as usize].install_loc as libc::c_int;
    i += 1;
    if i < NUM_APPLETS as libc::c_int {
      v_0 = (v_0 as libc::c_uint | (applets[i as usize].install_loc as libc::c_uint) << 4i32)
        as libc::c_int
    }
    printf(b"0x%02x,\n\x00" as *const u8 as *const libc::c_char, v_0);
    i += 1
  }
  printf(b"};\n\x00" as *const u8 as *const libc::c_char);
  //printf("#endif /* SKIP_definitions */\n");
  //	printf("\n");
  //	printf("#define MAX_APPLET_NAME_LEN %u\n", MAX_APPLET_NAME_LEN);
  if !(*argv.offset(2)).is_null() {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut line_new: [libc::c_char; 80] = [0; 80];
    //		}
    sprintf(
      line_new.as_mut_ptr(),
      b"#define NUM_APPLETS %u\n\x00" as *const u8 as *const libc::c_char,
      NUM_APPLETS as libc::c_int,
    );
    snprintf(
      tmp2.as_mut_ptr(),
      4096i32 as libc::c_ulong,
      b"%s.%u.new\x00" as *const u8 as *const libc::c_char,
      *argv.offset(2),
      getpid(),
    );
    fp = fopen(
      tmp2.as_mut_ptr(),
      b"w\x00" as *const u8 as *const libc::c_char,
    );
    if fp.is_null() {
      return 1i32;
    }
    fputs(line_new.as_mut_ptr(), fp);
    if fclose(fp) != 0 {
      return 1i32;
    }
  }
  if fclose(stdout) != 0 {
    return 1i32;
  }
  if rename(tmp1.as_mut_ptr(), *argv.offset(1)) != 0 {
    return 1i32;
  }
  if rename(tmp2.as_mut_ptr(), *argv.offset(2)) != 0 {
    return 1i32;
  }
  return 0i32;
}

pub fn main() {
  let mut args: Vec<*mut libc::c_char> = Vec::new();
  for arg in ::std::env::args() {
    args.push(
      ::std::ffi::CString::new(arg)
        .expect("Failed to convert argument into CString.")
        .into_raw(),
    );
  }
  args.push(::std::ptr::null_mut());
  unsafe {
    ::std::process::exit(main_0(
      (args.len() - 1) as libc::c_int,
      args.as_mut_ptr() as *mut *mut libc::c_char,
    ) as i32)
  }
}
//		char line_old[80];
//		line_old[0] = 0;
//		fp = fopen(argv[2], "r");
//		if (fp) {
//			fgets(line_old, sizeof(line_old), fp);
//			fclose(fp);
//		}
//		if (strcmp(line_old, line_new) != 0) {

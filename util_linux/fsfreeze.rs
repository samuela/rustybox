use libc;
use libc::putenv;
use libc::umask;
use libc::mknod;
use libc::fchmod;
use libc::fscanf;
use libc::alarm;
use libc::sync;
use libc::setsid;
use libc::ioctl;
use libc::statfs;
use libc::mount;
use libc::prctl;
use libc::opendir;
use libc::closedir;
use libc::readdir;
use libc::strtok;
use libc::putchar_unlocked;
use libc::endmntent;
use libc::setmntent;
use libc::umount2;
use libc::getegid;
use libc::getuid;
use libc::getgid;
use libc::setutxent;
use libc::endutxent;











































extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> u32;
  #[no_mangle]
  fn bb_xioctl(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
}



/*
 * Copyright (C) 2017 Denys Vlasenko <vda.linux@googlemail.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config FSFREEZE
//config:	bool "fsfreeze (3.5 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	select LONG_OPTS
//config:	help
//config:	Halt new accesses and flush writes on a mounted filesystem.
//applet:IF_FSFREEZE(APPLET_NOEXEC(fsfreeze, fsfreeze, BB_DIR_USR_SBIN, BB_SUID_DROP, fsfreeze))
//kbuild:lib-$(CONFIG_FSFREEZE) += fsfreeze.o
//usage:#define fsfreeze_trivial_usage
//usage:       "--[un]freeze MOUNTPOINT"
//usage:#define fsfreeze_full_usage "\n\n"
//usage:	"Flush and halt writes to MOUNTPOINT"
#[no_mangle]
pub unsafe extern "C" fn fsfreeze_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opts: libc::c_uint = 0;
  let mut fd: libc::c_int = 0;
  /* exactly one non-option arg: the mountpoint */
  /* one of opts is required */
  /* opts are mutually exclusive */
  opts = getopt32long(
    argv,
    b"^\x00=1:\xff:\xfe:\xff--\xfe:\xfe--\xff\x00" as *const u8 as *const libc::c_char,
    b"freeze\x00\x00\xffunfreeze\x00\x00\xfe\x00" as *const u8 as *const libc::c_char,
  );
  fd = xopen(*argv.offset(optind as isize), 0i32);
  /* Works with NULL arg on linux-4.8.0 */
  bb_xioctl(
    fd,
    if opts & 1i32 as libc::c_uint != 0 {
      (((2u32 | 1u32) << 0i32 + 8i32 + 8i32 + 14i32
        | (('X' as i32) << 0i32 + 8i32) as libc::c_uint
        | (119i32 << 0i32) as libc::c_uint) as libc::c_ulong)
        | (::std::mem::size_of::<libc::c_int>() as libc::c_ulong) << 0i32 + 8i32 + 8i32
    } else {
      (((2u32 | 1u32) << 0i32 + 8i32 + 8i32 + 14i32
        | (('X' as i32) << 0i32 + 8i32) as libc::c_uint
        | (120i32 << 0i32) as libc::c_uint) as libc::c_ulong)
        | (::std::mem::size_of::<libc::c_int>() as libc::c_ulong) << 0i32 + 8i32 + 8i32
    } as libc::c_uint,
    0 as *mut libc::c_void,
    b"(opts & 1) ? FIFREEZE : FITHAW\x00" as *const u8 as *const libc::c_char,
  );
  return 0i32;
}

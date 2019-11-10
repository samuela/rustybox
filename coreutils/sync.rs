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

use libc::close;

extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn fsync(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn syncfs(__fd: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn fdatasync(__fildes: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn open_or_warn(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
}

pub const OPT_DATASYNC: C2RustUnnamed = 1;
pub const OPT_SYNCFS: C2RustUnnamed = 2;
pub type C2RustUnnamed = libc::c_uint;

/*
 * Mini sync implementation for busybox
 *
 * Copyright (C) 1995, 1996 by Bruce Perens <bruce@pixar.com>.
 * Copyright (C) 2015 by Ari Sundholm <ari@tuxera.com>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config SYNC
//config:	bool "sync (3.8 kb)"
//config:	default y
//config:	help
//config:	sync is used to flush filesystem buffers.
//config:config FEATURE_SYNC_FANCY
//config:	bool "Enable -d and -f flags (requires syncfs(2) in libc)"
//config:	default y
//config:	depends on SYNC
//config:	help
//config:	sync -d FILE... executes fdatasync() on each FILE.
//config:	sync -f FILE... executes syncfs() on each FILE.
//               APPLET_NOFORK:name  main  location    suid_type     help
//applet:IF_SYNC(APPLET_NOFORK(sync, sync, BB_DIR_BIN, BB_SUID_DROP, sync))
//kbuild:lib-$(CONFIG_SYNC) += sync.o
/* BB_AUDIT SUSv3 N/A -- Matches GNU behavior. */
//usage:#define sync_trivial_usage
//usage:       ""IF_FEATURE_SYNC_FANCY("[-df] [FILE]...")
//usage:#define sync_full_usage "\n\n"
//usage:    IF_NOT_FEATURE_SYNC_FANCY(
//usage:       "Write all buffered blocks to disk"
//usage:    )
//usage:    IF_FEATURE_SYNC_FANCY(
//usage:       "Write all buffered blocks (in FILEs) to disk"
//usage:     "\n	-d	Avoid syncing metadata"
//usage:     "\n	-f	Sync filesystems underlying FILEs"
//usage:    )
/* This is a NOFORK applet. Be very careful! */
unsafe extern "C" fn sync_common(
  mut opts: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut ret: libc::c_int = 0;
  ret = 0i32;
  loop {
    /* GNU "sync FILE" uses O_NONBLOCK open */
    let mut fd: libc::c_int = open_or_warn(*argv, 0o400i32 | 0i32 | 0o4000i32);
    /* open(NOATIME) can only be used by owner or root, don't use NOATIME here */
    if fd < 0i32 {
      ret = 1i32
    } else {
      if opts & OPT_SYNCFS as libc::c_int != 0 {
        /*
         * syncfs is documented to only fail with EBADF,
         * which can't happen here. So, no error checks.
         */
        syncfs(fd);
      } else if (if opts & OPT_DATASYNC as libc::c_int != 0 {
        fdatasync(fd)
      } else {
        fsync(fd)
      }) != 0i32
      {
        bb_simple_perror_msg(*argv);
        ret = 1i32
      }
      close(fd);
    }
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  return ret;
}
#[no_mangle]
pub unsafe extern "C" fn sync_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opts: libc::c_uint = getopt32(
    argv,
    b"^df\x00d--f:f--d\x00" as *const u8 as *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  if (*argv.offset(0)).is_null() {
    sync();
    return 0i32;
  }
  return sync_common(opts as libc::c_int, argv);
}
/*
 * Mini fsync implementation for busybox
 *
 * Copyright (C) 2008 Nokia Corporation. All rights reserved.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config FSYNC
//config:	bool "fsync (3.6 kb)"
//config:	default y
//config:	help
//config:	fsync is used to flush file-related cached blocks to disk.
//                APPLET_NOFORK:name   main   location    suid_type     help
//applet:IF_FSYNC(APPLET_NOFORK(fsync, fsync, BB_DIR_BIN, BB_SUID_DROP, fsync))
//kbuild:lib-$(CONFIG_FSYNC) += sync.o
//usage:#define fsync_trivial_usage
//usage:       "[-d] FILE..."
//usage:#define fsync_full_usage "\n\n"
//usage:       "Write all buffered blocks in FILEs to disk\n"
//usage:     "\n	-d	Avoid syncing metadata"
#[no_mangle]
pub unsafe extern "C" fn fsync_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opts: libc::c_int =
    getopt32(argv, b"^d\x00-1\x00" as *const u8 as *const libc::c_char) as libc::c_int;
  argv = argv.offset(optind as isize);
  return sync_common(opts, argv);
}

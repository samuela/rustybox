use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn dirname(__path: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn copy_file(
    source: *const libc::c_char,
    dest: *const libc::c_char,
    flags: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn bb_get_last_path_component_strip(path: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> uint32_t;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn concat_path_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_make_directory(
    path: *mut libc::c_char,
    mode: libc::c_long,
    flags: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn cp_mv_stat2(fn_0: *const libc::c_char, fn_stat: *mut stat, sf: stat_func) -> libc::c_int;
  #[no_mangle]
  fn cp_mv_stat(fn_0: *const libc::c_char, fn_stat: *mut stat) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
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
pub type uint32_t = __uint32_t;
use crate::librb::timespec;
use crate::librb::stat;
pub type C2RustUnnamed = libc::c_int;
pub const FILEUTILS_IGNORE_CHMOD_ERR: C2RustUnnamed = -2147483648;
pub const FILEUTILS_REFLINK_ALWAYS: C2RustUnnamed = 262144;
pub const FILEUTILS_REFLINK: C2RustUnnamed = 131072;
pub const FILEUTILS_RMDEST: C2RustUnnamed = 32768;
pub const FILEUTILS_NO_TARGET_DIR: C2RustUnnamed = 16384;
pub const FILEUTILS_UPDATE: C2RustUnnamed = 8192;
pub const FILEUTILS_VERBOSE: C2RustUnnamed = 4096;
pub const FILEUTILS_DEREFERENCE_L0: C2RustUnnamed = 256;
pub const FILEUTILS_DEREF_SOFTLINK: C2RustUnnamed = 128;
pub const FILEUTILS_MAKE_SOFTLINK: C2RustUnnamed = 64;
pub const FILEUTILS_MAKE_HARDLINK: C2RustUnnamed = 32;
pub const FILEUTILS_INTERACTIVE: C2RustUnnamed = 16;
pub const FILEUTILS_FORCE: C2RustUnnamed = 8;
pub const FILEUTILS_RECUR: C2RustUnnamed = 4;
pub const FILEUTILS_DEREFERENCE: C2RustUnnamed = 2;
pub const FILEUTILS_PRESERVE_STATUS: C2RustUnnamed = 1;
/* vi: set sw=4 ts=4: */
/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
pub type stat_func =
  Option<unsafe extern "C" fn(_: *const libc::c_char, _: *mut stat) -> libc::c_int>;
/*OPT_rmdest  = FILEUTILS_RMDEST = 1 << FILEUTILS_CP_OPTNUM */
pub const OPT_parents: C2RustUnnamed_0 = 65536;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const OPT_reflink: C2RustUnnamed_0 = 131072;
pub const FILEUTILS_CP_OPTNUM: C2RustUnnamed_0 = 15;
/* vi: set sw=4 ts=4: */
/*
 * Mini cp implementation for busybox
 *
 * Copyright (C) 2000 by Matt Kraai <kraai@alumni.carnegiemellon.edu>
 * SELinux support by Yuichi Nakamura <ynakam@hitachisoft.jp>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Mar 16, 2003      Manuel Novoa III   (mjn3@codepoet.org)
 *
 * Size reduction.
 */
//config:config CP
//config:	bool "cp (10 kb)"
//config:	default y
//config:	help
//config:	cp is used to copy files and directories.
//config:
//config:config FEATURE_CP_LONG_OPTIONS
//config:	bool "Enable long options"
//config:	default y
//config:	depends on CP && LONG_OPTS
//config:	help
//config:	Enable long options.
//config:	Also add support for --parents option.
//config:
//config:config FEATURE_CP_REFLINK
//config:	bool "Enable --reflink[=auto]"
//config:	default y
//config:	depends on FEATURE_CP_LONG_OPTIONS
//applet:IF_CP(APPLET_NOEXEC(cp, cp, BB_DIR_BIN, BB_SUID_DROP, cp))
/* NOEXEC despite cases when it can be a "runner" (cp -r LARGE_DIR NEW_DIR) */
//kbuild:lib-$(CONFIG_CP) += cp.o
/* http://www.opengroup.org/onlinepubs/007904975/utilities/cp.html */
//usage:#define cp_trivial_usage
//usage:       "[OPTIONS] SOURCE... DEST"
//usage:#define cp_full_usage "\n\n"
//usage:       "Copy SOURCE(s) to DEST\n"
//usage:     "\n	-a	Same as -dpR"
//usage:	IF_SELINUX(
//usage:     "\n	-c	Preserve security context"
//usage:	)
//usage:     "\n	-R,-r	Recurse"
//usage:     "\n	-d,-P	Preserve symlinks (default if -R)"
//usage:     "\n	-L	Follow all symlinks"
//usage:     "\n	-H	Follow symlinks on command line"
//usage:     "\n	-p	Preserve file attributes if possible"
//usage:     "\n	-f	Overwrite"
//usage:     "\n	-i	Prompt before overwrite"
//usage:     "\n	-l,-s	Create (sym)links"
//usage:     "\n	-T	Treat DEST as a normal file"
//usage:     "\n	-u	Copy only newer files"
/* This is a NOEXEC applet. Be very careful! */
#[no_mangle]
pub unsafe extern "C" fn cp_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut source_stat: stat = stat {
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
  };
  let mut dest_stat: stat = stat {
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
  };
  let mut last: *const libc::c_char = 0 as *const libc::c_char;
  let mut dest: *const libc::c_char = 0 as *const libc::c_char;
  let mut s_flags: libc::c_int = 0;
  let mut d_flags: libc::c_int = 0;
  let mut flags: libc::c_int = 0;
  let mut status: libc::c_int = 0;
  let mut reflink: *mut libc::c_char = 0 as *mut libc::c_char;
  flags =
        getopt32long(argv,
                     b"^pdRfilsLHarPvuT\x00-2:l--s:s--l:Pd:rRd:Rd:apdR\x00" as
                         *const u8 as *const libc::c_char,
                     b"archive\x00\x00aforce\x00\x00finteractive\x00\x00ilink\x00\x00ldereference\x00\x00Lno-dereference\x00\x00Precursive\x00\x00Rsymbolic-link\x00\x00sno-target-directory\x00\x00Tverbose\x00\x00vupdate\x00\x00uremove-destination\x00\x00\xffparents\x00\x00\xfereflink\x00\x02\xfd\x00"
                         as *const u8 as *const libc::c_char,
                     &mut reflink as *mut *mut libc::c_char) as libc::c_int;
  if flags & FILEUTILS_REFLINK as libc::c_int != 0 {
    if reflink.is_null() {
      flags |= FILEUTILS_REFLINK_ALWAYS as libc::c_int
    } else if strcmp(reflink, b"always\x00" as *const u8 as *const libc::c_char) == 0i32 {
      flags |= FILEUTILS_REFLINK_ALWAYS as libc::c_int
    } else if strcmp(reflink, b"auto\x00" as *const u8 as *const libc::c_char) != 0i32 {
      bb_show_usage();
    }
  }
  /* Options of cp from GNU coreutils 6.10:
   * -a, --archive
   * -f, --force
   * -i, --interactive
   * -l, --link
   * -L, --dereference
   * -P, --no-dereference
   * -R, -r, --recursive
   * -s, --symbolic-link
   * -v, --verbose
   * -H	follow command-line symbolic links in SOURCE
   * -d	same as --no-dereference --preserve=links
   * -p	same as --preserve=mode,ownership,timestamps
   * -c	same as --preserve=context
   * -u, --update
   *	copy only when the SOURCE file is newer than the destination
   *	file or when the destination file is missing
   * --remove-destination
   *	remove each existing destination file before attempting to open
   * --parents
   *	use full source file name under DIRECTORY
   * -T, --no-target-directory
   *	treat DEST as a normal file
   * NOT SUPPORTED IN BBOX:
   * --backup[=CONTROL]
   *	make a backup of each existing destination file
   * -b	like --backup but does not accept an argument
   * --copy-contents
   *	copy contents of special files when recursive
   * --preserve[=ATTR_LIST]
   *	preserve attributes (default: mode,ownership,timestamps),
   *	if possible additional attributes: security context,links,all
   * --no-preserve=ATTR_LIST
   * --sparse=WHEN
   *	control creation of sparse files
   * --strip-trailing-slashes
   *	remove any trailing slashes from each SOURCE argument
   * -S, --suffix=SUFFIX
   *	override the usual backup suffix
   * -t, --target-directory=DIRECTORY
   *	copy all SOURCE arguments into DIRECTORY
   * -x, --one-file-system
   *	stay on this file system
   * -Z, --context=CONTEXT
   *	(SELinux) set SELinux security context of copy to CONTEXT
   */
  argc -= optind;
  argv = argv.offset(optind as isize);
  /* Reverse this bit. If there is -d, bit is not set: */
  flags ^= FILEUTILS_DEREFERENCE as libc::c_int;
  /* coreutils 6.9 compat:
   * by default, "cp" derefs symlinks (creates regular dest files),
   * but "cp -R" does not. We switch off deref if -r or -R (see above).
   * However, "cp -RL" must still deref symlinks: */
  if flags & FILEUTILS_DEREF_SOFTLINK as libc::c_int != 0 {
    /* -L */
    flags |= FILEUTILS_DEREFERENCE as libc::c_int
  }
  status = 0i32;
  last = *argv.offset((argc - 1i32) as isize);
  /* If there are only two arguments and...  */
  if argc == 2i32 {
    s_flags = cp_mv_stat2(
      *argv,
      &mut source_stat,
      if flags & FILEUTILS_DEREFERENCE as libc::c_int != 0 {
        Some(stat as unsafe extern "C" fn(_: *const libc::c_char, _: *mut stat) -> libc::c_int)
      } else {
        Some(lstat as unsafe extern "C" fn(_: *const libc::c_char, _: *mut stat) -> libc::c_int)
      },
    );
    if s_flags < 0i32 {
      return 1i32;
    }
    d_flags = cp_mv_stat(last, &mut dest_stat);
    if d_flags < 0i32 {
      return 1i32;
    }
    if flags & FILEUTILS_NO_TARGET_DIR as libc::c_int != 0 {
      /* -T */
      if s_flags & 2i32 == 0 && d_flags & 2i32 != 0 {
        /* cp -T NOTDIR DIR */
        bb_error_msg_and_die(
          b"\'%s\' is a directory\x00" as *const u8 as *const libc::c_char,
          last,
        );
      }
    }
    //bb_error_msg("flags:%x FILEUTILS_RMDEST:%x OPT_parents:%x",
    //	flags, FILEUTILS_RMDEST, OPT_parents);
    if flags & OPT_parents as libc::c_int != 0 {
      if d_flags & 2i32 == 0 {
        bb_simple_error_msg_and_die(
          b"with --parents, the destination must be a directory\x00" as *const u8
            as *const libc::c_char,
        );
      }
    }
    if flags & FILEUTILS_RMDEST as libc::c_int != 0 {
      flags |= FILEUTILS_FORCE as libc::c_int
    }
    /* ...if neither is a directory...  */
    if (s_flags | d_flags) & 2i32 == 0
      || flags & FILEUTILS_RECUR as libc::c_int != 0 && s_flags & 2i32 != 0 && d_flags == 0
      || flags & FILEUTILS_NO_TARGET_DIR as libc::c_int != 0
    {
      /* Do a simple copy */
      dest = last;
      current_block = 1243268177428749716;
    /* NB: argc==2 -> *++argv==last */
    } else {
      current_block = 790185930182612747;
    }
  } else {
    if flags & FILEUTILS_NO_TARGET_DIR as libc::c_int != 0 {
      bb_simple_error_msg_and_die(b"too many arguments\x00" as *const u8 as *const libc::c_char);
    }
    current_block = 790185930182612747;
  }
  loop {
    match current_block {
      1243268177428749716 => {
        if copy_file(*argv, dest, flags) < 0i32 {
          status = 1i32
        }
        argv = argv.offset(1);
        if *argv == last as *mut libc::c_char {
          break;
        }
        /* don't move up: dest may be == last and not malloced! */
        free(dest as *mut libc::c_void);
        current_block = 790185930182612747;
      }
      _ => {
        if flags & OPT_parents as libc::c_int != 0 {
          let mut dest_dup: *mut libc::c_char = 0 as *mut libc::c_char;
          let mut dest_dir: *mut libc::c_char = 0 as *mut libc::c_char;
          dest = concat_path_file(last, *argv);
          dest_dup = xstrdup(dest);
          dest_dir = dirname(dest_dup);
          if bb_make_directory(
            dest_dir,
            -1i32 as libc::c_long,
            FILEUTILS_RECUR as libc::c_int,
          ) != 0
          {
            return 1i32;
          }
          free(dest_dup as *mut libc::c_void);
          current_block = 1243268177428749716;
        } else {
          dest = concat_path_file(last, bb_get_last_path_component_strip(*argv));
          current_block = 1243268177428749716;
        }
      }
    }
  }
  /* possibly leaking dest... */
  /* Exit. We are NOEXEC, not NOFORK. We do exit at the end of main() */
  return status;
}

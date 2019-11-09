use libc;
use libc::unlink;

use libc::free;
use libc::gid_t;
use libc::stat;
use libc::uid_t;

use libc::mode_t;

extern "C" {

  #[no_mangle]
  fn lchown(__file: *const libc::c_char, __owner: uid_t, __group: gid_t) -> libc::c_int;
  #[no_mangle]
  fn getuid() -> uid_t;
  #[no_mangle]
  fn getgid() -> gid_t;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn dirname(__path: *mut libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn chmod(__file: *const libc::c_char, __mode: mode_t) -> libc::c_int;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn copy_file(
    source: *const libc::c_char,
    dest: *const libc::c_char,
    flags: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn bb_basename(name: *const libc::c_char) -> *const libc::c_char;
  #[no_mangle]
  fn xuname2uid(name: *const libc::c_char) -> libc::c_long;
  #[no_mangle]
  fn xgroup2gid(name: *const libc::c_char) -> libc::c_long;
  #[no_mangle]
  fn get_ug_id(
    s: *const libc::c_char,
    xname2id: Option<unsafe extern "C" fn(_: *const libc::c_char) -> libc::c_long>,
  ) -> libc::c_ulong;
  #[no_mangle]
  fn spawn_and_wait(argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> u32;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_parse_mode(s: *const libc::c_char, cur_mode: libc::c_uint) -> libc::c_int;
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
}

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
pub const OPT_GROUP: C2RustUnnamed_0 = 128;
pub const OPT_OWNER: C2RustUnnamed_0 = 512;
pub const OPT_STRIP: C2RustUnnamed_0 = 64;
pub const OPT_TARGET: C2RustUnnamed_0 = 1024;
pub const OPT_MKDIR_LEADING: C2RustUnnamed_0 = 8;
pub const OPT_DIRECTORY: C2RustUnnamed_0 = 16;
pub const OPT_MODE: C2RustUnnamed_0 = 256;
pub const OPT_PRESERVE_TIME: C2RustUnnamed_0 = 32;
pub const OPT_v: C2RustUnnamed_0 = 2;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const OPT_b: C2RustUnnamed_0 = 4;
pub const OPT_c: C2RustUnnamed_0 = 1;

/*
 * Copyright (C) 2003 by Glenn McGrath
 * SELinux support: by Yuichi Nakamura <ynakam@hitachisoft.jp>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config INSTALL
//config:	bool "install (12 kb)"
//config:	default y
//config:	help
//config:	Copy files and set attributes.
//config:
//config:config FEATURE_INSTALL_LONG_OPTIONS
//config:	bool "Enable long options"
//config:	default y
//config:	depends on INSTALL && LONG_OPTS
//applet:IF_INSTALL(APPLET(install, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_INSTALL) += install.o
/* -v, -b, -c are ignored */
//usage:#define install_trivial_usage
//usage:	"[-cdDsp] [-o USER] [-g GRP] [-m MODE] [-t DIR] [SOURCE]... DEST"
//usage:#define install_full_usage "\n\n"
//usage:       "Copy files and set attributes\n"
//usage:     "\n	-c	Just copy (default)"
//usage:     "\n	-d	Create directories"
//usage:     "\n	-D	Create leading target directories"
//usage:     "\n	-s	Strip symbol table"
//usage:     "\n	-p	Preserve date"
//usage:     "\n	-o USER	Set ownership"
//usage:     "\n	-g GRP	Set group ownership"
//usage:     "\n	-m MODE	Set permissions"
//usage:     "\n	-t DIR	Install to DIR"
//usage:	IF_SELINUX(
//usage:     "\n	-Z	Set security context"
//usage:	)
static mut install_longopts: [libc::c_char; 95] = [
  118, 101, 114, 98, 111, 115, 101, 0, 0, 118, 100, 105, 114, 101, 99, 116, 111, 114, 121, 0, 0,
  100, 112, 114, 101, 115, 101, 114, 118, 101, 45, 116, 105, 109, 101, 115, 116, 97, 109, 112, 115,
  0, 0, 112, 115, 116, 114, 105, 112, 0, 0, 115, 103, 114, 111, 117, 112, 0, 1, 103, 109, 111, 100,
  101, 0, 1, 109, 111, 119, 110, 101, 114, 0, 1, 111, 116, 97, 114, 103, 101, 116, 45, 100, 105,
  114, 101, 99, 116, 111, 114, 121, 0, 1, 116, 0,
];
#[no_mangle]
pub unsafe extern "C" fn install_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut statbuf: stat = std::mem::zeroed();
  let mut mode: mode_t = 0;
  let mut uid: uid_t = 0;
  let mut gid: gid_t = 0;
  let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut last: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut gid_str: *const libc::c_char = 0 as *const libc::c_char;
  let mut uid_str: *const libc::c_char = 0 as *const libc::c_char;
  let mut mode_str: *const libc::c_char = 0 as *const libc::c_char;
  let mut mkdir_flags: libc::c_int = FILEUTILS_RECUR as libc::c_int;
  let mut copy_flags: libc::c_int =
    FILEUTILS_DEREFERENCE as libc::c_int | FILEUTILS_FORCE as libc::c_int;
  let mut opts: libc::c_int = 0;
  let mut ret: libc::c_int = 0i32;
  let mut isdir: libc::c_int = 0;
  /* -c exists for backwards compatibility, it's needed */
  /* -b is ignored ("make a backup of each existing destination file") */
  opts = getopt32long(
    argv,
    b"^cvbDdpsg:m:o:t:\x00t--d:d--t:s--d:d--s\x00" as *const u8 as *const libc::c_char,
    install_longopts.as_ptr(),
    &mut gid_str as *mut *const libc::c_char,
    &mut mode_str as *mut *const libc::c_char,
    &mut uid_str as *mut *const libc::c_char,
    &mut last as *mut *mut libc::c_char,
  ) as libc::c_int;
  argc -= optind;
  argv = argv.offset(optind as isize);
  if opts & OPT_v as libc::c_int != 0 && FILEUTILS_VERBOSE as libc::c_int != 0 {
    mkdir_flags |= FILEUTILS_VERBOSE as libc::c_int;
    copy_flags |= FILEUTILS_VERBOSE as libc::c_int
  }
  /* preserve access and modification time, this is GNU behaviour,
   * BSD only preserves modification time */
  if opts & OPT_PRESERVE_TIME as libc::c_int != 0 {
    copy_flags |= FILEUTILS_PRESERVE_STATUS as libc::c_int
  } /* GNU coreutils 6.10 compat */
  mode = 0o755i32 as mode_t;
  if opts & OPT_MODE as libc::c_int != 0 {
    mode = bb_parse_mode(mode_str, mode) as mode_t
  }
  uid = if opts & OPT_OWNER as libc::c_int != 0 {
    get_ug_id(
      uid_str,
      Some(xuname2uid as unsafe extern "C" fn(_: *const libc::c_char) -> libc::c_long),
    )
  } else {
    getuid() as libc::c_ulong
  } as uid_t;
  gid = if opts & OPT_GROUP as libc::c_int != 0 {
    get_ug_id(
      gid_str,
      Some(xgroup2gid as unsafe extern "C" fn(_: *const libc::c_char) -> libc::c_long),
    )
  } else {
    getgid() as libc::c_ulong
  } as gid_t;
  /* If -t DIR is in use, then isdir=true, last="DIR" */
  isdir = opts & OPT_TARGET as libc::c_int;
  if opts & (OPT_TARGET as libc::c_int | OPT_DIRECTORY as libc::c_int) == 0 {
    /* Neither -t DIR nor -d is in use */
    argc -= 1;
    last = *argv.offset(argc as isize);
    let ref mut fresh0 = *argv.offset(argc as isize);
    *fresh0 = 0 as *mut libc::c_char;
    /* coreutils install resolves link in this case, don't use lstat */
    isdir = if stat(last, &mut statbuf) < 0i32 {
      0i32
    } else {
      (statbuf.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint) as libc::c_int
    }
  }
  if argc < 1i32 {
    bb_show_usage();
  }
  loop {
    let fresh1 = argv;
    argv = argv.offset(1);
    arg = *fresh1;
    if arg.is_null() {
      break;
    }
    let mut dest: *mut libc::c_char = 0 as *mut libc::c_char;
    if opts & OPT_DIRECTORY as libc::c_int != 0 {
      dest = arg;
      /* GNU coreutils 6.9 does not set uid:gid
       * on intermediate created directories
       * (only on last one) */
      if bb_make_directory(dest, 0o755i32 as libc::c_long, mkdir_flags) != 0 {
        ret = 1i32;
        current_block = 11038358249373066665;
      } else {
        current_block = 9353995356876505083;
      }
    } else {
      dest = last;
      if opts & OPT_MKDIR_LEADING as libc::c_int != 0 {
        let mut ddir: *mut libc::c_char = xstrdup(dest);
        /*
         * -D -t DIR1/DIR2/F3 FILE: create DIR1/DIR2/F3, copy FILE there
         * -D FILE DIR1/DIR2/F3: create DIR1/DIR2, copy FILE there as F3
         */
        bb_make_directory(
          if opts & OPT_TARGET as libc::c_int != 0 {
            ddir
          } else {
            dirname(ddir)
          },
          0o755i32 as libc::c_long,
          mkdir_flags,
        );
        /* errors are not checked. copy_file
         * will fail if dir is not created.
         */
        free(ddir as *mut libc::c_void);
      }
      if isdir != 0 {
        dest = concat_path_file(last, bb_basename(arg))
      }
      if copy_file(arg, dest, copy_flags) != 0i32 {
        /* copy is not made */
        ret = 1i32; /* -p --preserve-dates */
        current_block = 11038358249373066665;
      } else {
        if opts & OPT_STRIP as libc::c_int != 0 {
          let mut args: [*mut libc::c_char; 4] = [0 as *mut libc::c_char; 4];
          args[0] = b"strip\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
          args[1] = b"-p\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
          args[2] = dest;
          args[3] = 0 as *mut libc::c_char;
          if spawn_and_wait(args.as_mut_ptr()) != 0 {
            bb_simple_perror_msg(b"strip\x00" as *const u8 as *const libc::c_char);
            ret = 1i32
          }
        }
        current_block = 9353995356876505083;
      }
    }
    match current_block {
      9353995356876505083 => {
        /* Set the file mode (always, not only with -m).
         * GNU coreutils 6.10 is not affected by umask. */
        if chmod(dest, mode) == -1i32 {
          bb_perror_msg(
            b"can\'t change %s of %s\x00" as *const u8 as *const libc::c_char,
            b"permissions\x00" as *const u8 as *const libc::c_char,
            dest,
          );
          ret = 1i32
        }
        /* Set the user and group id */
        if opts & (OPT_OWNER as libc::c_int | OPT_GROUP as libc::c_int) != 0
          && lchown(dest, uid, gid) == -1i32
        {
          bb_perror_msg(
            b"can\'t change %s of %s\x00" as *const u8 as *const libc::c_char,
            b"ownership\x00" as *const u8 as *const libc::c_char,
            dest,
          );
          ret = 1i32
        }
      }
      _ => {}
    }
    if 0i32 != 0 && isdir != 0 {
      free(dest as *mut libc::c_void);
    }
  }
  return ret;
}

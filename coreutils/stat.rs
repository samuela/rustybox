use crate::librb::size_t;

use crate::libbb::appletlib::applet_name;
use libc;
use libc::free;
use libc::gid_t;
use libc::group;
use libc::lstat;
use libc::mode_t;
use libc::passwd;
use libc::printf;
use libc::puts;
use libc::sprintf;
use libc::stat;
use libc::statfs;
use libc::strchr;
use libc::time_t;
use libc::tm;
use libc::uid_t;
use libc::FILE;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn gnu_dev_major(__dev: libc::dev_t) -> libc::c_uint;
  #[no_mangle]
  fn gnu_dev_minor(__dev: libc::dev_t) -> libc::c_uint;
  #[no_mangle]
  static mut stdout: *mut FILE;

  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn strftime(
    __s: *mut libc::c_char,
    __maxsize: size_t,
    __format: *const libc::c_char,
    __tp: *const tm,
  ) -> size_t;
  #[no_mangle]
  fn localtime(_: *const time_t) -> *mut tm;

  /* Search for an entry with a matching user ID.  */
  #[no_mangle]
  fn bb_internal_getpwuid(__uid: uid_t) -> *mut passwd;
  /* Search for an entry with a matching group ID.  */
  #[no_mangle]
  fn bb_internal_getgrgid(__gid: gid_t) -> *mut group;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_mode_string(mode: mode_t) -> *const libc::c_char;
  #[no_mangle]
  fn xmalloc_readlink_or_warn(path: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn nth_string(strings: *const libc::c_char, n: libc::c_int) -> *const libc::c_char;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

pub type __fsblkcnt64_t = libc::c_ulong;
pub type __fsfilcnt64_t = libc::c_ulong;
pub type __fsword_t = libc::c_long;

pub type C2RustUnnamed = libc::c_uint;
pub const COMMON_BUFSIZE: C2RustUnnamed = 1024;

/*
 * stat -- display file or file system status
 *
 * Copyright (C) 2001, 2002, 2003, 2004, 2005 Free Software Foundation.
 * Copyright (C) 2005 by Erik Andersen <andersen@codepoet.org>
 * Copyright (C) 2005 by Mike Frysinger <vapier@gentoo.org>
 * Copyright (C) 2006 by Yoshinori Sato <ysato@users.sourceforge.jp>
 *
 * Written by Michael Meskes
 * Taken from coreutils and turned into a busybox applet by Mike Frysinger
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config STAT
//config:	bool "stat (11 kb)"
//config:	default y
//config:	help
//config:	display file or filesystem status.
//config:
//config:config FEATURE_STAT_FORMAT
//config:	bool "Enable custom formats (-c)"
//config:	default y
//config:	depends on STAT
//config:	help
//config:	Without this, stat will not support the '-c format' option where
//config:	users can pass a custom format string for output. This adds about
//config:	7k to a nonstatic build on amd64.
//config:
//config:config FEATURE_STAT_FILESYSTEM
//config:	bool "Enable display of filesystem status (-f)"
//config:	default y
//config:	depends on STAT
//config:	select PLATFORM_LINUX # statfs()
//config:	help
//config:	Without this, stat will not support the '-f' option to display
//config:	information about filesystem status.
//applet:IF_STAT(APPLET_NOEXEC(stat, stat, BB_DIR_BIN, BB_SUID_DROP, stat))
//kbuild:lib-$(CONFIG_STAT) += stat.o
//usage:#define stat_trivial_usage
//usage:       "[OPTIONS] FILE..."
//usage:#define stat_full_usage "\n\n"
//usage:       "Display file"
//usage:            IF_FEATURE_STAT_FILESYSTEM(" (default) or filesystem")
//usage:            " status\n"
//usage:	IF_FEATURE_STAT_FORMAT(
//usage:     "\n	-c FMT	Use the specified format"
//usage:	)
//usage:	IF_FEATURE_STAT_FILESYSTEM(
//usage:     "\n	-f	Display filesystem status"
//usage:	)
//usage:     "\n	-L	Follow links"
//usage:     "\n	-t	Terse display"
//usage:	IF_SELINUX(
//usage:     "\n	-Z	Print security context"
//usage:	)
//usage:	IF_FEATURE_STAT_FORMAT(
//usage:       "\n\nFMT sequences"IF_FEATURE_STAT_FILESYSTEM(" for files")":\n"
//usage:       " %a	Access rights in octal\n"
//usage:       " %A	Access rights in human readable form\n"
//usage:       " %b	Number of blocks allocated (see %B)\n"
//usage:       " %B	Size in bytes of each block reported by %b\n"
//usage:       " %d	Device number in decimal\n"
//usage:       " %D	Device number in hex\n"
//usage:       " %f	Raw mode in hex\n"
//usage:       " %F	File type\n"
//usage:       " %g	Group ID\n"
//usage:       " %G	Group name\n"
//usage:       " %h	Number of hard links\n"
//usage:       " %i	Inode number\n"
//usage:       " %n	File name\n"
//usage:       " %N	File name, with -> TARGET if symlink\n"
//usage:       " %o	I/O block size\n"
//usage:       " %s	Total size in bytes\n"
//usage:       " %t	Major device type in hex\n"
//usage:       " %T	Minor device type in hex\n"
//usage:       " %u	User ID\n"
//usage:       " %U	User name\n"
//usage:       " %x	Time of last access\n"
//usage:       " %X	Time of last access as seconds since Epoch\n"
//usage:       " %y	Time of last modification\n"
//usage:       " %Y	Time of last modification as seconds since Epoch\n"
//usage:       " %z	Time of last change\n"
//usage:       " %Z	Time of last change as seconds since Epoch\n"
//usage:	IF_FEATURE_STAT_FILESYSTEM(
//usage:       "\nFMT sequences for file systems:\n"
//usage:       " %a	Free blocks available to non-superuser\n"
//usage:       " %b	Total data blocks\n"
//usage:       " %c	Total file nodes\n"
//usage:       " %d	Free file nodes\n"
//usage:       " %f	Free blocks\n"
//usage:	IF_SELINUX(
//usage:       " %C	Security context in selinux\n"
//usage:	)
//usage:       " %i	File System ID in hex\n"
//usage:       " %l	Maximum length of filenames\n"
//usage:       " %n	File name\n"
//usage:       " %s	Block size (for faster transfer)\n"
//usage:       " %S	Fundamental block size (for block counts)\n"
//usage:       " %t	Type in hex\n"
//usage:       " %T	Type in human readable form"
//usage:	)
//usage:	)
pub type C2RustUnnamed_0 = libc::c_uint;
pub const OPT_SELINUX: C2RustUnnamed_0 = 0;
pub const OPT_FILESYS: C2RustUnnamed_0 = 4;
pub const OPT_DEREFERENCE: C2RustUnnamed_0 = 2;
pub const OPT_TERSE: C2RustUnnamed_0 = 1;

pub type statfunc_ptr =
  Option<unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> bool>;

unsafe extern "C" fn file_type(mut st: *const stat) -> *const libc::c_char {
  /* See POSIX 1003.1-2001 XCU Table 4-8 lines 17093-17107
   * for some of these formats.
   * To keep diagnostics grammatical in English, the
   * returned string must start with a consonant.
   */
  if (*st).st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint {
    return if (*st).st_size == 0 {
      b"regular empty file\x00" as *const u8 as *const libc::c_char
    } else {
      b"regular file\x00" as *const u8 as *const libc::c_char
    };
  }
  if (*st).st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint {
    return b"directory\x00" as *const u8 as *const libc::c_char;
  }
  if (*st).st_mode & 0o170000i32 as libc::c_uint == 0o60000i32 as libc::c_uint {
    return b"block special file\x00" as *const u8 as *const libc::c_char;
  }
  if (*st).st_mode & 0o170000i32 as libc::c_uint == 0o20000i32 as libc::c_uint {
    return b"character special file\x00" as *const u8 as *const libc::c_char;
  }
  if (*st).st_mode & 0o170000i32 as libc::c_uint == 0o10000i32 as libc::c_uint {
    return b"fifo\x00" as *const u8 as *const libc::c_char;
  }
  if (*st).st_mode & 0o170000i32 as libc::c_uint == 0o120000i32 as libc::c_uint {
    return b"symbolic link\x00" as *const u8 as *const libc::c_char;
  }
  if (*st).st_mode & 0o170000i32 as libc::c_uint == 0o140000i32 as libc::c_uint {
    return b"socket\x00" as *const u8 as *const libc::c_char;
  }
  if (*st).st_mode.wrapping_sub((*st).st_mode) != 0 {
    return b"message queue\x00" as *const u8 as *const libc::c_char;
  }
  if (*st).st_mode.wrapping_sub((*st).st_mode) != 0 {
    return b"semaphore\x00" as *const u8 as *const libc::c_char;
  }
  if (*st).st_mode.wrapping_sub((*st).st_mode) != 0 {
    return b"shared memory object\x00" as *const u8 as *const libc::c_char;
  }
  return b"weird file\x00" as *const u8 as *const libc::c_char;
}

unsafe fn human_time(ts: &libc::timespec) -> *const libc::c_char {
  let mut fmt: [libc::c_char; 39] = [0; 39];
  sprintf(
    stpcpy(
      fmt.as_mut_ptr(),
      b"%Y-%m-%d %H:%M:%S\x00" as *const u8 as *const libc::c_char,
    ),
    b".%09u %%z\x00" as *const u8 as *const libc::c_char,
    ts.tv_nsec as libc::c_uint,
  );
  strftime(
    bb_common_bufsiz1.as_mut_ptr(),
    COMMON_BUFSIZE as libc::c_int as size_t,
    fmt.as_mut_ptr(),
    localtime(&ts.tv_sec),
  );
  return bb_common_bufsiz1.as_mut_ptr();
}

/* Return the type of the specified file system.
 * Some systems have statfvs.f_basetype[FSTYPSZ]. (AIX, HP-UX, and Solaris)
 * Others have statfs.f_fstypename[MFSNAMELEN]. (NetBSD 1.5.2)
 * Still others have neither and have to get by with f_type (Linux).
 */
unsafe extern "C" fn human_fstype(mut f_type: u32) -> *const libc::c_char {
  static mut fstype: [u32; 35] = [
    0xadffi32 as u32,
    0x1cd1i32 as u32,
    0x137di32 as u32,
    0xef51i32 as u32,
    0xef53i32 as u32,
    0x3153464ai32 as u32,
    0x58465342i32 as u32,
    0xf995e849u32,
    0x9660i32 as u32,
    0x4000i32 as u32,
    0x4004i32 as u32,
    0x137fi32 as u32,
    0x138fi32 as u32,
    0x2468i32 as u32,
    0x2478i32 as u32,
    0x4d44i32 as u32,
    0x4006i32 as u32,
    0x564ci32 as u32,
    0x6969i32 as u32,
    0x9fa0i32 as u32,
    0x517bi32 as u32,
    0x12ff7b4i32 as u32,
    0x12ff7b5i32 as u32,
    0x12ff7b6i32 as u32,
    0x12ff7b7i32 as u32,
    0x11954i32 as u32,
    0x12fd16di32 as u32,
    0x5346544ei32 as u32,
    0x1021994i32 as u32,
    0x52654973i32 as u32,
    0x28cd3d45i32 as u32,
    0x7275i32 as u32,
    0x858458f6u32,
    0x73717368i32 as u32,
    0x62656572i32 as u32,
  ];
  static mut humanname: [libc::c_char; 236] = [
    97, 102, 102, 115, 0, 100, 101, 118, 112, 116, 115, 0, 101, 120, 116, 0, 101, 120, 116, 50, 0,
    101, 120, 116, 50, 47, 101, 120, 116, 51, 0, 106, 102, 115, 0, 120, 102, 115, 0, 104, 112, 102,
    115, 0, 105, 115, 111, 102, 115, 0, 105, 115, 111, 102, 115, 0, 105, 115, 111, 102, 115, 0,
    109, 105, 110, 105, 120, 0, 109, 105, 110, 105, 120, 32, 40, 51, 48, 32, 99, 104, 97, 114, 46,
    41, 0, 109, 105, 110, 105, 120, 32, 118, 50, 0, 109, 105, 110, 105, 120, 32, 118, 50, 32, 40,
    51, 48, 32, 99, 104, 97, 114, 46, 41, 0, 109, 115, 100, 111, 115, 0, 102, 97, 116, 0, 110, 111,
    118, 101, 108, 108, 0, 110, 102, 115, 0, 112, 114, 111, 99, 0, 115, 109, 98, 0, 120, 101, 110,
    105, 120, 0, 115, 121, 115, 118, 52, 0, 115, 121, 115, 118, 50, 0, 99, 111, 104, 0, 117, 102,
    115, 0, 120, 105, 97, 0, 110, 116, 102, 115, 0, 116, 109, 112, 102, 115, 0, 114, 101, 105, 115,
    101, 114, 102, 115, 0, 99, 114, 97, 109, 102, 115, 0, 114, 111, 109, 102, 115, 0, 114, 97, 109,
    102, 115, 0, 115, 113, 117, 97, 115, 104, 102, 115, 0, 115, 121, 115, 102, 115, 0, 85, 78, 75,
    78, 79, 87, 78, 0,
  ];
  let mut i: libc::c_int = 0;
  i = 0i32;
  while (i as libc::c_uint)
    < (::std::mem::size_of::<[u32; 35]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<u32>() as libc::c_ulong) as libc::c_uint
  {
    if fstype[i as usize] == f_type {
      break;
    }
    i += 1
  }
  return nth_string(humanname.as_ptr(), i);
}

/* "man statfs" says that statfsbuf->f_fsid is a mess */
/* coreutils treats it as an array of ints, most significant first */
unsafe extern "C" fn get_f_fsid(mut statfsbuf: *const statfs) -> libc::c_ulonglong {
  let mut p: *const libc::c_uint =
    &(*statfsbuf).f_fsid as *const libc::fsid_t as *const libc::c_void as *const libc::c_uint;
  let mut sz: libc::c_uint = (::std::mem::size_of::<libc::fsid_t>())
    .wrapping_div(::std::mem::size_of::<libc::c_uint>())
    as libc::c_uint;
  let mut r: libc::c_ulonglong = 0;
  loop {
    let fresh0 = p;
    p = p.offset(1);
    r = r << (::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8) | *fresh0 as libc::c_ulonglong;
    sz = sz.wrapping_sub(1);
    if !(sz > 0) {
      break;
    }
  }
  return r;
}

/* FEATURE_STAT_FILESYSTEM */
unsafe extern "C" fn strcatc(mut str: *mut libc::c_char, mut c: libc::c_char) {
  let mut len: libc::c_int = strlen(str) as libc::c_int;
  let fresh1 = len;
  len = len + 1;
  *str.offset(fresh1 as isize) = c;
  *str.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
}
unsafe extern "C" fn printfs(mut pformat: *mut libc::c_char, mut msg: *const libc::c_char) {
  strcatc(pformat, 's' as i32 as libc::c_char);
  printf(pformat, msg);
}
/* print statfs info */
unsafe extern "C" fn print_statfs(
  mut pformat: *mut libc::c_char,
  m: libc::c_char,
  filename: *const libc::c_char,
  mut data: *const libc::c_void,
) {
  let mut statfsbuf: *const statfs = data as *const statfs;
  if m as libc::c_int == 'n' as i32 {
    printfs(pformat, filename);
  } else if m as libc::c_int == 'i' as i32 {
    strcat(pformat, b"llx\x00" as *const u8 as *const libc::c_char);
    printf(pformat, get_f_fsid(statfsbuf));
  } else if m as libc::c_int == 'l' as i32 {
    strcat(pformat, b"lu\x00" as *const u8 as *const libc::c_char);
    printf(pformat, (*statfsbuf).f_namelen as libc::c_ulong);
  } else if m as libc::c_int == 't' as i32 {
    strcat(pformat, b"lx\x00" as *const u8 as *const libc::c_char);
    printf(pformat, (*statfsbuf).f_type as libc::c_ulong);
  /* no equiv */
  } else if m as libc::c_int == 'T' as i32 {
    printfs(pformat, human_fstype((*statfsbuf).f_type as u32));
  } else if m as libc::c_int == 'b' as i32 {
    strcat(pformat, b"llu\x00" as *const u8 as *const libc::c_char);
    printf(pformat, (*statfsbuf).f_blocks as libc::c_ulonglong);
  } else if m as libc::c_int == 'f' as i32 {
    strcat(pformat, b"llu\x00" as *const u8 as *const libc::c_char);
    printf(pformat, (*statfsbuf).f_bfree as libc::c_ulonglong);
  } else if m as libc::c_int == 'a' as i32 {
    strcat(pformat, b"llu\x00" as *const u8 as *const libc::c_char);
    printf(pformat, (*statfsbuf).f_bavail as libc::c_ulonglong);
  } else if m as libc::c_int == 's' as i32 || m as libc::c_int == 'S' as i32 {
    strcat(pformat, b"lu\x00" as *const u8 as *const libc::c_char);
    printf(pformat, (*statfsbuf).f_bsize as libc::c_ulong);
  } else if m as libc::c_int == 'c' as i32 {
    strcat(pformat, b"llu\x00" as *const u8 as *const libc::c_char);
    printf(pformat, (*statfsbuf).f_files as libc::c_ulonglong);
  } else if m as libc::c_int == 'd' as i32 {
    strcat(pformat, b"llu\x00" as *const u8 as *const libc::c_char);
    printf(pformat, (*statfsbuf).f_ffree as libc::c_ulonglong);
  } else {
    strcatc(pformat, 'c' as i32 as libc::c_char);
    printf(pformat, m as libc::c_int);
  };
}
/* print stat info */
unsafe extern "C" fn print_stat(
  mut pformat: *mut libc::c_char,
  m: libc::c_char,
  filename: *const libc::c_char,
  mut data: *const libc::c_void,
) {
  let mut statbuf: *mut stat = data as *mut stat;
  let mut pw_ent: *mut passwd = 0 as *mut passwd;
  let mut gw_ent: *mut group = 0 as *mut group;
  if m as libc::c_int == 'n' as i32 {
    printfs(pformat, filename);
  } else if m as libc::c_int == 'N' as i32 {
    strcatc(pformat, 's' as i32 as libc::c_char);
    if (*statbuf).st_mode & 0o170000i32 as libc::c_uint == 0o120000i32 as libc::c_uint {
      let mut linkname: *mut libc::c_char = xmalloc_readlink_or_warn(filename);
      if linkname.is_null() {
        return;
      }
      printf(
        b"\'%s\' -> \'%s\'\x00" as *const u8 as *const libc::c_char,
        filename,
        linkname,
      );
      free(linkname as *mut libc::c_void);
    } else {
      printf(pformat, filename);
    }
  } else if m as libc::c_int == 'd' as i32 {
    strcat(pformat, b"llu\x00" as *const u8 as *const libc::c_char);
    printf(pformat, (*statbuf).st_dev as libc::c_ulonglong);
  } else if m as libc::c_int == 'D' as i32 {
    strcat(pformat, b"llx\x00" as *const u8 as *const libc::c_char);
    printf(pformat, (*statbuf).st_dev as libc::c_ulonglong);
  } else if m as libc::c_int == 'i' as i32 {
    strcat(pformat, b"llu\x00" as *const u8 as *const libc::c_char);
    printf(pformat, (*statbuf).st_ino as libc::c_ulonglong);
  } else if m as libc::c_int == 'a' as i32 {
    strcat(pformat, b"lo\x00" as *const u8 as *const libc::c_char);
    printf(
      pformat,
      ((*statbuf).st_mode
        & (0o4000i32
          | 0o2000i32
          | 0o1000i32
          | (0o400i32 | 0o200i32 | 0o100i32)
          | (0o400i32 | 0o200i32 | 0o100i32) >> 3i32
          | (0o400i32 | 0o200i32 | 0o100i32) >> 3i32 >> 3i32) as libc::c_uint)
        as libc::c_ulong,
    );
  } else if m as libc::c_int == 'A' as i32 {
    printfs(pformat, bb_mode_string((*statbuf).st_mode));
  } else if m as libc::c_int == 'f' as i32 {
    strcat(pformat, b"lx\x00" as *const u8 as *const libc::c_char);
    printf(pformat, (*statbuf).st_mode as libc::c_ulong);
  } else if m as libc::c_int == 'F' as i32 {
    printfs(pformat, file_type(statbuf));
  } else if m as libc::c_int == 'h' as i32 {
    strcat(pformat, b"lu\x00" as *const u8 as *const libc::c_char);
    printf(pformat, (*statbuf).st_nlink);
  } else if m as libc::c_int == 'u' as i32 {
    strcat(pformat, b"lu\x00" as *const u8 as *const libc::c_char);
    printf(pformat, (*statbuf).st_uid as libc::c_ulong);
  } else if m as libc::c_int == 'U' as i32 {
    pw_ent = bb_internal_getpwuid((*statbuf).st_uid);
    printfs(
      pformat,
      if !pw_ent.is_null() {
        (*pw_ent).pw_name
      } else {
        b"UNKNOWN\x00" as *const u8 as *const libc::c_char
      },
    );
  } else if m as libc::c_int == 'g' as i32 {
    strcat(pformat, b"lu\x00" as *const u8 as *const libc::c_char);
    printf(pformat, (*statbuf).st_gid as libc::c_ulong);
  } else if m as libc::c_int == 'G' as i32 {
    gw_ent = bb_internal_getgrgid((*statbuf).st_gid);
    printfs(
      pformat,
      if !gw_ent.is_null() {
        (*gw_ent).gr_name
      } else {
        b"UNKNOWN\x00" as *const u8 as *const libc::c_char
      },
    );
  } else if m as libc::c_int == 't' as i32 {
    strcat(pformat, b"lx\x00" as *const u8 as *const libc::c_char);
    printf(pformat, gnu_dev_major((*statbuf).st_rdev) as libc::c_ulong);
  } else if m as libc::c_int == 'T' as i32 {
    strcat(pformat, b"lx\x00" as *const u8 as *const libc::c_char);
    printf(pformat, gnu_dev_minor((*statbuf).st_rdev) as libc::c_ulong);
  } else if m as libc::c_int == 's' as i32 {
    strcat(pformat, b"llu\x00" as *const u8 as *const libc::c_char);
    printf(pformat, (*statbuf).st_size as libc::c_ulonglong);
  } else if m as libc::c_int == 'B' as i32 {
    strcat(pformat, b"lu\x00" as *const u8 as *const libc::c_char);
    printf(pformat, 512i32 as libc::c_ulong);
  //ST_NBLOCKSIZE
  } else if m as libc::c_int == 'b' as i32 {
    strcat(pformat, b"llu\x00" as *const u8 as *const libc::c_char);
    printf(pformat, (*statbuf).st_blocks as libc::c_ulonglong);
  } else if m as libc::c_int == 'o' as i32 {
    strcat(pformat, b"lu\x00" as *const u8 as *const libc::c_char);
    printf(pformat, (*statbuf).st_blksize as libc::c_ulong);
  } else if m as libc::c_int == 'x' as i32 {
    printfs(
      pformat,
      human_time(&libc::timespec {
        tv_sec: (*statbuf).st_atime,
        tv_nsec: (*statbuf).st_atime_nsec,
      }),
    );
  } else if m as libc::c_int == 'X' as i32 {
    strcat(
      pformat,
      if !((0i32 as time_t) < -1i32 as time_t) {
        b"ld\x00" as *const u8 as *const libc::c_char
      } else {
        b"lu\x00" as *const u8 as *const libc::c_char
      },
    );
    /* note: (unsigned long) would be wrong:
     * imagine (unsigned long64)int32 */
    printf(pformat, (*statbuf).st_atime);
  } else if m as libc::c_int == 'y' as i32 {
    printfs(
      pformat,
      human_time(&libc::timespec {
        tv_sec: (*statbuf).st_mtime,
        tv_nsec: (*statbuf).st_mtime_nsec,
      }),
    );
  } else if m as libc::c_int == 'Y' as i32 {
    strcat(
      pformat,
      if !((0i32 as time_t) < -1i32 as time_t) {
        b"ld\x00" as *const u8 as *const libc::c_char
      } else {
        b"lu\x00" as *const u8 as *const libc::c_char
      },
    );
    printf(pformat, (*statbuf).st_mtime);
  } else if m as libc::c_int == 'z' as i32 {
    printfs(
      pformat,
      human_time(&libc::timespec {
        tv_sec: (*statbuf).st_ctime,
        tv_nsec: (*statbuf).st_ctime_nsec,
      }),
    );
  } else if m as libc::c_int == 'Z' as i32 {
    strcat(
      pformat,
      if !((0i32 as time_t) < -1i32 as time_t) {
        b"ld\x00" as *const u8 as *const libc::c_char
      } else {
        b"lu\x00" as *const u8 as *const libc::c_char
      },
    );
    printf(pformat, (*statbuf).st_ctime);
  } else {
    strcatc(pformat, 'c' as i32 as libc::c_char);
    printf(pformat, m as libc::c_int);
  };
}
unsafe extern "C" fn print_it(
  mut masterformat: *const libc::c_char,
  mut filename: *const libc::c_char,
  mut print_func: Option<
    unsafe extern "C" fn(
      _: *mut libc::c_char,
      _: libc::c_char,
      _: *const libc::c_char,
      _: *const libc::c_void,
    ) -> (),
  >,
  mut data: *const libc::c_void,
) {
  /* Create a working copy of the format string */
  let mut format: *mut libc::c_char = xstrdup(masterformat);
  /* Add 2 to accommodate our conversion of the stat '%s' format string
   * to the printf '%llu' one.  */
  let mut dest: *mut libc::c_char = xmalloc(
    strlen(format)
      .wrapping_add(2i32 as libc::c_ulong)
      .wrapping_add(1i32 as libc::c_ulong),
  ) as *mut libc::c_char;
  let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
  b = format;
  while !b.is_null() {
    /* Each iteration finds next %spec,
     * prints preceding string and handles found %spec
     */
    let mut len: size_t = 0;
    let mut p: *mut libc::c_char = strchr(b, '%' as i32);
    if p.is_null() {
      /* coreutils 6.3 always prints newline at the end */
      /*fputs(b, stdout);*/
      puts(b);
      break;
    } else {
      /* dest = "%<modifiers>" */
      len = (1i32 as libc::c_ulong).wrapping_add(strspn(
        p.offset(1),
        b"#-+.I 0123456789\x00" as *const u8 as *const libc::c_char,
      ));
      memcpy(dest as *mut libc::c_void, p as *const libc::c_void, len);
      *dest.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
      /* print preceding string */
      *p = '\u{0}' as i32 as libc::c_char;
      fputs_unlocked(b, stdout);
      p = p.offset(len as isize);
      b = p.offset(1);
      let mut current_block_11: u64;
      match *p as libc::c_int {
        0 => {
          b = 0 as *mut libc::c_char;
          current_block_11 = 14977033353768631501;
        }
        37 => {
          current_block_11 = 14977033353768631501;
        }
        _ => {
          /* Completes "%<modifiers>" with specifier and printfs */
          print_func.expect("non-null function pointer")(dest, *p, filename, data);
          current_block_11 = 5948590327928692120;
        }
      }
      match current_block_11 {
        14977033353768631501 =>
        /* fall through */
        {
          bb_putchar('%' as i32);
        }
        _ => {}
      }
    }
  }
  free(format as *mut libc::c_void);
  free(dest as *mut libc::c_void);
}
/* FEATURE_STAT_FORMAT */
/* Stat the file system and print what we find.  */
unsafe extern "C" fn do_statfs(
  mut filename: *const libc::c_char,
  mut format: *const libc::c_char,
) -> bool {
  let mut statfsbuf: statfs = std::mem::zeroed();
  if statfs(filename, &mut statfsbuf) != 0i32 {
    bb_perror_msg(
      b"can\'t read file system information for \'%s\'\x00" as *const u8 as *const libc::c_char,
      filename,
    );
    return 0i32 != 0;
  }
  if format.is_null() {
    format = if option_mask32 & OPT_TERSE as libc::c_int as libc::c_uint != 0 {
      b"%n %i %l %t %s %b %f %a %c %d\x00" as *const u8 as *const libc::c_char
    } else {
      b"  File: \"%n\"\n    ID: %-8i Namelen: %-7l Type: %T\nBlock size: %-10s\nBlocks: Total: %-10b Free: %-10f Available: %a\nInodes: Total: %-10c Free: %d\x00"
                    as *const u8 as *const libc::c_char
    }
    /* SELINUX */
  }
  print_it(
    format,
    filename,
    Some(
      print_statfs
        as unsafe extern "C" fn(
          _: *mut libc::c_char,
          _: libc::c_char,
          _: *const libc::c_char,
          _: *const libc::c_void,
        ) -> (),
    ),
    &mut statfsbuf as *mut statfs as *const libc::c_void,
  );
  /* !FEATURE_STAT_FORMAT */
  /* FEATURE_STAT_FORMAT */
  return 1i32 != 0;
}
/* FEATURE_STAT_FILESYSTEM */
/* stat the file and print what we find */
unsafe extern "C" fn do_stat(
  mut filename: *const libc::c_char,
  mut format: *const libc::c_char,
) -> bool {
  let mut statbuf: stat = std::mem::zeroed();
  if (if option_mask32 & OPT_DEREFERENCE as libc::c_int as libc::c_uint != 0 {
    Some(stat as unsafe extern "C" fn(_: *const libc::c_char, _: *mut stat) -> libc::c_int)
  } else {
    Some(lstat as unsafe extern "C" fn(_: *const libc::c_char, _: *mut stat) -> libc::c_int)
  })
  .expect("non-null function pointer")(filename, &mut statbuf)
    != 0i32
  {
    bb_perror_msg(
      b"can\'t stat \'%s\'\x00" as *const u8 as *const libc::c_char,
      filename,
    );
    return 0i32 != 0;
  }
  if format.is_null() {
    if option_mask32 & OPT_TERSE as libc::c_int as libc::c_uint != 0 {
      format =
        b"%n %s %b %f %u %g %D %i %h %t %T %X %Y %Z %o\x00" as *const u8 as *const libc::c_char
    } else if statbuf.st_mode & 0o170000i32 as libc::c_uint == 0o60000i32 as libc::c_uint
      || statbuf.st_mode & 0o170000i32 as libc::c_uint == 0o20000i32 as libc::c_uint
    {
      format =
                b"  File: %N\n  Size: %-10s\tBlocks: %-10b IO Block: %-6o %F\nDevice: %Dh/%dd\tInode: %-10i  Links: %-5h Device type: %t,%T\nAccess: (%04a/%10.10A)  Uid: (%5u/%8U)   Gid: (%5g/%8G)\nAccess: %x\nModify: %y\nChange: %z\x00"
                    as *const u8 as *const libc::c_char
    } else {
      format =
                b"  File: %N\n  Size: %-10s\tBlocks: %-10b IO Block: %-6o %F\nDevice: %Dh/%dd\tInode: %-10i  Links: %h\nAccess: (%04a/%10.10A)  Uid: (%5u/%8U)   Gid: (%5g/%8G)\nAccess: %x\nModify: %y\nChange: %z\x00"
                    as *const u8 as *const libc::c_char
    }
  }
  print_it(
    format,
    filename,
    Some(
      print_stat
        as unsafe extern "C" fn(
          _: *mut libc::c_char,
          _: libc::c_char,
          _: *const libc::c_char,
          _: *const libc::c_void,
        ) -> (),
    ),
    &mut statbuf as *mut stat as *const libc::c_void,
  );
  /* FEATURE_STAT_FORMAT */
  /* FEATURE_STAT_FORMAT */
  return 1i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn stat_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut format: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut i: libc::c_int = 0;
  let mut ok: libc::c_int = 0;
  let mut statfunc: statfunc_ptr =
    Some(do_stat as unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> bool);
  let mut opts: libc::c_uint = 0;
  opts = getopt32(
    argv,
    b"^tLfc:\x00-1\x00" as *const u8 as *const libc::c_char,
    &mut format as *mut *mut libc::c_char,
  );
  if opts & OPT_FILESYS as libc::c_int as libc::c_uint != 0 {
    /* -f */
    statfunc = Some(
      do_statfs as unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> bool,
    )
  }
  ok = 1i32;
  argv = argv.offset(optind as isize);
  i = 0i32;
  while !(*argv.offset(i as isize)).is_null() {
    ok &=
      statfunc.expect("non-null function pointer")(*argv.offset(i as isize), format) as libc::c_int;
    i += 1
  }
  return if ok != 0 { 0i32 } else { 1i32 };
}

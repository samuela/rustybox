use libc;
extern "C" {
  pub type __dirstream;

  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);

  #[no_mangle]
  fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);

  #[no_mangle]
  fn isatty(__fd: libc::c_int) -> libc::c_int;

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn closedir(__dirp: *mut DIR) -> libc::c_int;

  #[no_mangle]
  fn readdir(__dirp: *mut DIR) -> *mut dirent;

  #[no_mangle]
  fn gnu_dev_major(__dev: __dev_t) -> libc::c_uint;

  #[no_mangle]
  fn gnu_dev_minor(__dev: __dev_t) -> libc::c_uint;

  #[no_mangle]
  static mut stdout: *mut _IO_FILE;

  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn strcoll(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  fn strverscmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;

  #[no_mangle]
  fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;

  #[no_mangle]
  fn time(__timer: *mut time_t) -> time_t;

  #[no_mangle]
  fn strftime(
    __s: *mut libc::c_char,
    __maxsize: size_t,
    __format: *const libc::c_char,
    __tp: *const tm,
  ) -> size_t;

  #[no_mangle]
  fn localtime(__timer: *const time_t) -> *mut tm;

  #[no_mangle]
  fn ctime(__timer: *const time_t) -> *mut libc::c_char;

  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn bb_mode_string(mode: mode_t) -> *const libc::c_char;

  #[no_mangle]
  fn bb_basename(name: *const libc::c_char) -> *const libc::c_char;

  #[no_mangle]
  fn warn_opendir(path: *const libc::c_char) -> *mut DIR;

  #[no_mangle]
  fn xmalloc_readlink_or_warn(path: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn printable_string2(stats: *mut uni_stat_t, str: *const libc::c_char) -> *const libc::c_char;

  #[no_mangle]
  fn make_human_readable_str(
    size: libc::c_ulonglong,
    block_size: libc::c_ulong,
    display_unit: libc::c_ulong,
  ) -> *const libc::c_char;

  #[no_mangle]
  fn get_cached_username(uid: uid_t) -> *const libc::c_char;

  #[no_mangle]
  fn get_cached_groupname(gid: gid_t) -> *const libc::c_char;

  #[no_mangle]
  static mut option_mask32: uint32_t;

  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> uint32_t;

  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);

  #[no_mangle]
  fn concat_path_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;

  #[no_mangle]
  static mut applet_name: *const libc::c_char;

  #[no_mangle]
  fn index_in_substrings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn get_terminal_width(fd: libc::c_int) -> libc::c_int;

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
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
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __blkcnt64_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type smallint = libc::c_schar;
pub type size_t = libc::c_ulong;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
pub type ino_t = __ino64_t;
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
pub type mode_t = __mode_t;
use crate::librb::stat;
use crate::librb::timespec;
pub type nlink_t = __nlink_t;
pub type time_t = __time_t;
pub type blkcnt_t = __blkcnt64_t;
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
pub type __compar_fn_t =
  Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
  pub tm_sec: libc::c_int,
  pub tm_min: libc::c_int,
  pub tm_hour: libc::c_int,
  pub tm_mday: libc::c_int,
  pub tm_mon: libc::c_int,
  pub tm_year: libc::c_int,
  pub tm_wday: libc::c_int,
  pub tm_yday: libc::c_int,
  pub tm_isdst: libc::c_int,
  pub tm_gmtoff: libc::c_long,
  pub tm_zone: *const libc::c_char,
}
pub type uoff_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uni_stat_t {
  pub byte_count: libc::c_uint,
  pub unicode_count: libc::c_uint,
  pub unicode_width: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub show_color: smallint,
  pub exit_code: smallint,
  pub show_dirname: smallint,
  pub terminal_width: libc::c_uint,
  pub current_time_t: time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dnode {
  pub name: *const libc::c_char,
  pub fullname: *const libc::c_char,
  pub dn_next: *mut dnode,
  pub fname_allocated: smallint,
  pub dn_mode_lstat: mode_t,
  pub dn_mode_stat: mode_t,
  pub dn_mode: mode_t,
  pub dn_size: off_t,
  pub dn_time: time_t,
  pub dn_ino: ino_t,
  pub dn_blocks: blkcnt_t,
  pub dn_nlink: nlink_t,
  pub dn_uid: uid_t,
  pub dn_gid: gid_t,
  pub dn_rdev_maj: libc::c_int,
  pub dn_rdev_min: libc::c_int,
}
pub const OPT_r: C2RustUnnamed_1 = 2097152;
pub const BITS_TO_SHIFT: libc::c_uint = 32;
pub const OPT_X: C2RustUnnamed_1 = 1048576;
pub const OPT_v: C2RustUnnamed_1 = 4194304;
pub const OPT_t: C2RustUnnamed_1 = 131072;
pub const OPT_S: C2RustUnnamed_1 = 524288;
pub const OPT_dirs_first: C2RustUnnamed_1 = 536870912;
pub const SPLIT_SUBDIR: C2RustUnnamed_0 = 2;
pub const SPLIT_FILE: C2RustUnnamed_0 = 0;
pub const SPLIT_DIR: C2RustUnnamed_0 = 1;
pub const OPT_R: C2RustUnnamed_1 = 16384;
pub const OPT_F: C2RustUnnamed_1 = 4096;
pub const OPT_p: C2RustUnnamed_1 = 8192;
pub const OPT_Q: C2RustUnnamed_1 = 32768;
pub const OPT_full_time: C2RustUnnamed_1 = 268435456;
pub const OPT_h: C2RustUnnamed_1 = 33554432;
pub const OPT_g: C2RustUnnamed_1 = 64;
pub const OPT_n: C2RustUnnamed_1 = 128;
pub const OPT_l: C2RustUnnamed_1 = 32;
pub const OPT_s: C2RustUnnamed_1 = 256;
pub const OPT_i: C2RustUnnamed_1 = 8;
pub const OPT_x: C2RustUnnamed_1 = 512;
pub const OPT_Z: C2RustUnnamed_1 = 0;
pub const OPT_1: C2RustUnnamed_1 = 16;
pub const OPT_c: C2RustUnnamed_1 = 65536;
pub const OPT_u: C2RustUnnamed_1 = 262144;
pub const OPT_L: C2RustUnnamed_1 = 8388608;
pub const OPT_a: C2RustUnnamed_1 = 2;
pub const OPT_A: C2RustUnnamed_1 = 1024;
pub const OPT_d: C2RustUnnamed_1 = 4;
pub const OPT_H: C2RustUnnamed_1 = 16777216;
pub const OPT_C: C2RustUnnamed_1 = 1;
pub const OPT_color: C2RustUnnamed_1 = 1073741824;
pub const TERMINAL_WIDTH: C2RustUnnamed_0 = 80;
pub type C2RustUnnamed_0 = libc::c_uint;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const OPT_w: C2RustUnnamed_1 = 134217728;
pub const OPT_T: C2RustUnnamed_1 = 67108864;
pub const OPTBIT_color: C2RustUnnamed_1 = 30;
pub const OPTBIT_dirs_first: C2RustUnnamed_1 = 29;
pub const OPTBIT_full_time: C2RustUnnamed_1 = 28;
pub const OPTBIT_w: C2RustUnnamed_1 = 27;
pub const OPTBIT_T: C2RustUnnamed_1 = 26;
pub const OPTBIT_h: C2RustUnnamed_1 = 25;
pub const OPTBIT_H: C2RustUnnamed_1 = 24;
pub const OPTBIT_L: C2RustUnnamed_1 = 23;
pub const OPTBIT_v: C2RustUnnamed_1 = 22;
pub const OPTBIT_r: C2RustUnnamed_1 = 21;
pub const OPTBIT_X: C2RustUnnamed_1 = 20;
pub const OPTBIT_S: C2RustUnnamed_1 = 19;
pub const OPTBIT_u: C2RustUnnamed_1 = 18;
pub const OPTBIT_t: C2RustUnnamed_1 = 17;
pub const OPTBIT_c: C2RustUnnamed_1 = 16;
pub const OPTBIT_Q: C2RustUnnamed_1 = 15;
pub const OPTBIT_Z: C2RustUnnamed_1 = 15;
pub const OPTBIT_R: C2RustUnnamed_1 = 14;
pub const OPTBIT_p: C2RustUnnamed_1 = 13;
pub const OPTBIT_F: C2RustUnnamed_1 = 12;
/* we have to zero it out because of NOEXEC */
/* ** Output code ***/
/* FYI type values: 1:fifo 2:char 4:dir 6:blk 8:file 10:link 12:socket
 * (various wacky OSes: 13:Sun door 14:BSD whiteout 5:XENIX named file
 *  3/7:multiplexed char/block device)
 * and we use 0 for unknown and 15 for executables (see below) */
/*                       un  fi chr -   dir -  blk  -  file -  link - sock -   - exe */
/* 036 black foreground              050 black background
   037 red foreground                051 red background
   040 green foreground              052 green background
   041 brown foreground              053 brown background
   042 blue foreground               054 blue background
   043 magenta (purple) foreground   055 magenta background
   044 cyan (light blue) foreground  056 cyan background
   045 gray foreground               057 white background
*/
/*un  fi  chr  -  dir  -  blk  -  file -  link -  sock -   -  exe */
/* Select normal (0) [actually "reset all"] or bold (1)
 * (other attributes are 2:dim 4:underline 5:blink 7:reverse,
 *  let's use 7 for "impossible" types, just for fun)
 * Note: coreutils 6.9 uses inverted red for setuid binaries.
 */
/*un fi chr - dir - blk - file- link- sock- -  exe */
/* mode of zero is interpreted as "unknown" (stat failed) */
unsafe extern "C" fn fgcolor(mut mode: mode_t) -> libc::c_char {
  if mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint
    && mode & (0o100i32 | 0o100i32 >> 3i32 | 0o100i32 >> 3i32 >> 3i32) as libc::c_uint != 0
  {
    return (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(
      b"\x1f##%\"%##\x00%$%#%% \x00",
    ))[(0xf000i32 >> 12i32 & 0xfi32) as usize];
  } /* File is executable ... */
  return (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(
    b"\x1f##%\"%##\x00%$%#%% \x00",
  ))[(mode >> 12i32 & 0xfi32 as libc::c_uint) as usize]; /* File is executable ... */
}
unsafe extern "C" fn bold(mut mode: mode_t) -> libc::c_char {
  if mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint
    && mode & (0o100i32 | 0o100i32 >> 3i32 | 0o100i32 >> 3i32 >> 3i32) as libc::c_uint != 0
  {
    return (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(
      b"\x01\x00\x01\x07\x01\x07\x01\x07\x00\x07\x01\x07\x01\x07\x07\x01\x00",
    ))[(0xf000i32 >> 12i32 & 0xfi32) as usize];
  }
  return (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(
    b"\x01\x00\x01\x07\x01\x07\x01\x07\x00\x07\x01\x07\x01\x07\x07\x01\x00",
  ))[(mode >> 12i32 & 0xfi32 as libc::c_uint) as usize];
}
unsafe extern "C" fn append_char(mut mode: mode_t) -> libc::c_char {
  if option_mask32 & (OPT_F as libc::c_int | OPT_p as libc::c_int) as libc::c_uint == 0 {
    return '\u{0}' as i32 as libc::c_char;
  }
  if mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint {
    return '/' as i32 as libc::c_char;
  }
  if option_mask32 & OPT_F as libc::c_int as libc::c_uint == 0 {
    return '\u{0}' as i32 as libc::c_char;
  }
  if mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint
    && mode & (0o100i32 | 0o100i32 >> 3i32 | 0o100i32 >> 3i32 >> 3i32) as libc::c_uint != 0
  {
    return '*' as i32 as libc::c_char;
  }
  return (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(
    b"\x00|\x00\x00/\x00\x00\x00\x00\x00@\x00=\x00\x00\x00\x00",
  ))[(mode >> 12i32 & 0xfi32 as libc::c_uint) as usize];
}
unsafe extern "C" fn calc_name_len(mut name: *const libc::c_char) -> libc::c_uint {
  let mut len: libc::c_uint = 0;
  let mut uni_stat: uni_stat_t = uni_stat_t {
    byte_count: 0,
    unicode_count: 0,
    unicode_width: 0,
  };
  // TODO: quote tab as \t, etc, if -Q
  name = printable_string2(&mut uni_stat, name);
  if option_mask32 & OPT_Q as libc::c_int as libc::c_uint == 0 {
    return uni_stat.unicode_width;
  }
  len = (2i32 as libc::c_uint).wrapping_add(uni_stat.unicode_width);
  while *name != 0 {
    if *name as libc::c_int == '\"' as i32 || *name as libc::c_int == '\\' as i32 {
      len = len.wrapping_add(1)
    }
    name = name.offset(1)
  }
  return len;
}
/* Return the number of used columns.
 * Note that only columnar output uses return value.
 * -l and -1 modes don't care.
 * coreutils 7.2 also supports:
 * ls -b (--escape) = octal escapes (although it doesn't look like working)
 * ls -N (--literal) = not escape at all
 */
unsafe extern "C" fn print_name(mut name: *const libc::c_char) -> libc::c_uint {
  let mut len: libc::c_uint = 0;
  let mut uni_stat: uni_stat_t = uni_stat_t {
    byte_count: 0,
    unicode_count: 0,
    unicode_width: 0,
  };
  // TODO: quote tab as \t, etc, if -Q
  name = printable_string2(&mut uni_stat, name);
  if option_mask32 & OPT_Q as libc::c_int as libc::c_uint == 0 {
    fputs_unlocked(name, stdout);
    return uni_stat.unicode_width;
  }
  len = (2i32 as libc::c_uint).wrapping_add(uni_stat.unicode_width);
  putchar_unlocked('\"' as i32);
  while *name != 0 {
    if *name as libc::c_int == '\"' as i32 || *name as libc::c_int == '\\' as i32 {
      putchar_unlocked('\\' as i32);
      len = len.wrapping_add(1)
    }
    putchar_unlocked(*name as libc::c_int);
    name = name.offset(1)
  }
  putchar_unlocked('\"' as i32);
  return len;
}
/* Return the number of used columns.
 * Note that only columnar output uses return value,
 * -l and -1 modes don't care.
 */
#[inline(never)]
unsafe extern "C" fn display_single(mut dn: *const dnode) -> libc::c_uint {
  let mut column: libc::c_uint = 0i32 as libc::c_uint;
  let mut lpath: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opt: libc::c_int = 0;
  let mut statbuf: stat = stat {
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
  let mut append: libc::c_char = append_char((*dn).dn_mode);
  opt = option_mask32 as libc::c_int;
  /* Do readlink early, so that if it fails, error message
   * does not appear *inside* the "ls -l" line */
  lpath = 0 as *mut libc::c_char;
  if opt & OPT_l as libc::c_int != 0 {
    if (*dn).dn_mode & 0o170000i32 as libc::c_uint == 0o120000i32 as libc::c_uint {
      lpath = xmalloc_readlink_or_warn((*dn).fullname)
    }
  }
  if opt & OPT_i as libc::c_int != 0 {
    /* show inode# */
    column = column.wrapping_add(printf(
      b"%7llu \x00" as *const u8 as *const libc::c_char,
      (*dn).dn_ino as libc::c_longlong,
    ) as libc::c_uint)
  }
  //TODO: -h should affect -s too:
  if opt & OPT_s as libc::c_int != 0 {
    /* show allocated blocks */
    column = column.wrapping_add(printf(
      b"%6lu \x00" as *const u8 as *const libc::c_char,
      (*dn).dn_blocks >> 1i32,
    ) as libc::c_uint)
  }
  if opt & OPT_l as libc::c_int != 0 {
    /* long listing: show mode */
    column = column.wrapping_add(printf(
      b"%-10s \x00" as *const u8 as *const libc::c_char,
      bb_mode_string((*dn).dn_mode) as *mut libc::c_char,
    ) as libc::c_uint);
    /* long listing: show number of links */
    column = column.wrapping_add(printf(
      b"%4lu \x00" as *const u8 as *const libc::c_char,
      (*dn).dn_nlink as libc::c_long,
    ) as libc::c_uint);
    /* long listing: show user/group */
    if opt & OPT_n as libc::c_int != 0 {
      if opt & OPT_g as libc::c_int != 0 {
        column = column.wrapping_add(printf(
          b"%-8u \x00" as *const u8 as *const libc::c_char,
          (*dn).dn_gid as libc::c_int,
        ) as libc::c_uint)
      } else {
        column = column.wrapping_add(printf(
          b"%-8u %-8u \x00" as *const u8 as *const libc::c_char,
          (*dn).dn_uid as libc::c_int,
          (*dn).dn_gid as libc::c_int,
        ) as libc::c_uint)
      }
    } else if opt & OPT_g as libc::c_int != 0 {
      column = column.wrapping_add(printf(
        b"%-8.8s \x00" as *const u8 as *const libc::c_char,
        get_cached_groupname((*dn).dn_gid),
      ) as libc::c_uint)
    } else {
      column = column.wrapping_add(printf(
        b"%-8.8s %-8.8s \x00" as *const u8 as *const libc::c_char,
        get_cached_username((*dn).dn_uid),
        get_cached_groupname((*dn).dn_gid),
      ) as libc::c_uint)
    }
    /* long listing: show size */
    if (*dn).dn_mode & 0o170000i32 as libc::c_uint == 0o60000i32 as libc::c_uint
      || (*dn).dn_mode & 0o170000i32 as libc::c_uint == 0o20000i32 as libc::c_uint
    {
      column = column.wrapping_add(printf(
        b"%4u, %3u \x00" as *const u8 as *const libc::c_char,
        (*dn).dn_rdev_maj,
        (*dn).dn_rdev_min,
      ) as libc::c_uint)
    } else if opt & OPT_h as libc::c_int != 0 {
      column = column.wrapping_add(printf(
        b"%7s \x00" as *const u8 as *const libc::c_char,
        make_human_readable_str(
          (*dn).dn_size as libc::c_ulonglong,
          1i32 as libc::c_ulong,
          0i32 as libc::c_ulong,
        ),
      ) as libc::c_uint)
    } else {
      column = column.wrapping_add(printf(
        b"%9lu \x00" as *const u8 as *const libc::c_char,
        (*dn).dn_size,
      ) as libc::c_uint)
    }
    /* long listing: show {m,c,a}time */
    if opt & OPT_full_time as libc::c_int != 0 {
      let mut buf: [libc::c_char; 29] = [0; 29]; /* ordinary time format */
      strftime(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong,
        b"%Y-%m-%d %H:%M:%S %z\x00" as *const u8 as *const libc::c_char,
        localtime(&(*dn).dn_time),
      );
      column = column.wrapping_add(printf(
        b"%s \x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
      ) as libc::c_uint)
    /* --full-time */
    /* coreutils 8.4 ls --full-time prints:
     * 2009-07-13 17:49:27.000000000 +0200
     * we don't show fractional seconds.
     */
    } else {
      /* G.current_time_t is ~== time(NULL) */
      let mut filetime: *mut libc::c_char = ctime(&(*dn).dn_time);
      /* filetime's format: "Wed Jun 30 21:49:08 1993\n" */
      let mut age: time_t =
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).current_time_t - (*dn).dn_time;
      if age < 3600i64 * 24i32 as libc::c_long * 365i32 as libc::c_long / 2i32 as libc::c_long
        && age > (-15i32 * 60i32) as libc::c_long
      {
        /* less than 6 months old */
        /* "mmm dd hh:mm " */
        printf(
          b"%.12s \x00" as *const u8 as *const libc::c_char,
          filetime.offset(4),
        );
      } else {
        /* "mmm dd  yyyy " */
        /* "mmm dd yyyyy " after year 9999 :) */
        *strchr(filetime.offset(20), '\n' as i32).offset(0) = ' ' as i32 as libc::c_char; /* used only by coulmnal output */
        printf(
          b"%.7s%6s\x00" as *const u8 as *const libc::c_char,
          filetime.offset(4),
          filetime.offset(20),
        );
      }
      column = column.wrapping_add(13i32 as libc::c_uint)
    }
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).show_color != 0 {
    let mut mode: mode_t = (*dn).dn_mode_lstat;
    if mode == 0 {
      if lstat((*dn).fullname, &mut statbuf) == 0i32 {
        mode = statbuf.st_mode
      }
    }
    printf(
      b"\x1b[%u;%um\x00" as *const u8 as *const libc::c_char,
      bold(mode) as libc::c_int,
      fgcolor(mode) as libc::c_int,
    );
  }
  column = column.wrapping_add(print_name((*dn).name));
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).show_color != 0 {
    printf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
  }
  if !lpath.is_null() {
    printf(b" -> \x00" as *const u8 as *const libc::c_char);
    if opt & (OPT_F as libc::c_int | OPT_p as libc::c_int) != 0
      || (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).show_color as libc::c_int != 0
    {
      let mut mode_0: mode_t = (*dn).dn_mode_stat;
      if mode_0 == 0 {
        if stat((*dn).fullname, &mut statbuf) == 0i32 {
          mode_0 = statbuf.st_mode
        }
      }
      append = append_char(mode_0);
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).show_color != 0 {
        printf(
          b"\x1b[%u;%um\x00" as *const u8 as *const libc::c_char,
          bold(mode_0) as libc::c_int,
          fgcolor(mode_0) as libc::c_int,
        );
      }
    }
    column = column.wrapping_add(print_name(lpath).wrapping_add(4i32 as libc::c_uint));
    free(lpath as *mut libc::c_void);
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).show_color != 0 {
      printf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
    }
  }
  if opt & (OPT_F as libc::c_int | OPT_p as libc::c_int) != 0 {
    if append != 0 {
      putchar_unlocked(append as libc::c_int);
      column = column.wrapping_add(1)
    }
  }
  return column;
}
unsafe extern "C" fn display_files(mut dn: *mut *mut dnode, mut nfiles: libc::c_uint) {
  let mut i: libc::c_uint = 0;
  let mut ncols: libc::c_uint = 0;
  let mut nrows: libc::c_uint = 0;
  let mut row: libc::c_uint = 0;
  let mut nc: libc::c_uint = 0;
  let mut column: libc::c_uint = 0;
  let mut nexttab: libc::c_uint = 0;
  let mut column_width: libc::c_uint = 0i32 as libc::c_uint;
  if option_mask32 & (OPT_l as libc::c_int | OPT_1 as libc::c_int) as libc::c_uint != 0 {
    ncols = 1i32 as libc::c_uint
  } else {
    /* find the longest file name, use that as the column width */
    i = 0i32 as libc::c_uint; /* "alloc block" width */
    while !(*dn.offset(i as isize)).is_null() {
      let mut len: libc::c_int = calc_name_len((**dn.offset(i as isize)).name) as libc::c_int;
      if column_width < len as libc::c_uint {
        column_width = len as libc::c_uint
      }
      i = i.wrapping_add(1)
    }
    column_width = column_width.wrapping_add(
      (2i32
        + (if option_mask32 & OPT_Z as libc::c_int as libc::c_uint != 0 {
          33i32
        } else {
          0i32
        })
        + (if option_mask32 & OPT_i as libc::c_int as libc::c_uint != 0 {
          8i32
        } else {
          0i32
        })
        + (if option_mask32 & OPT_s as libc::c_int as libc::c_uint != 0 {
          5i32
        } else {
          0i32
        })) as libc::c_uint,
    );
    ncols = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .terminal_width
      .wrapping_div(column_width)
  }
  if ncols > 1i32 as libc::c_uint {
    nrows = nfiles.wrapping_div(ncols);
    if nrows.wrapping_mul(ncols) < nfiles {
      nrows = nrows.wrapping_add(1)
    }
  /* round up fractionals */
  } else {
    nrows = nfiles;
    ncols = 1i32 as libc::c_uint
  }
  column = 0i32 as libc::c_uint;
  nexttab = 0i32 as libc::c_uint;
  row = 0i32 as libc::c_uint;
  while row < nrows {
    nc = 0i32 as libc::c_uint;
    while nc < ncols {
      /* reach into the array based on the column and row */
      if option_mask32 & OPT_x as libc::c_int as libc::c_uint != 0 {
        /* display by column */
        i = row.wrapping_mul(ncols).wrapping_add(nc)
      } else {
        i = nc.wrapping_mul(nrows).wrapping_add(row)
      } /* display across row */
      if i < nfiles {
        if column > 0i32 as libc::c_uint {
          nexttab = nexttab.wrapping_sub(column);
          printf(
            b"%*s\x00" as *const u8 as *const libc::c_char,
            nexttab,
            b"\x00" as *const u8 as *const libc::c_char,
          );
          column = column.wrapping_add(nexttab)
        }
        nexttab = column.wrapping_add(column_width);
        column = column.wrapping_add(display_single(*dn.offset(i as isize)))
      }
      nc = nc.wrapping_add(1)
    }
    putchar_unlocked('\n' as i32);
    column = 0i32 as libc::c_uint;
    row = row.wrapping_add(1)
  }
}
/* ** Dir scanning code ***/
unsafe extern "C" fn my_stat(
  mut fullname: *const libc::c_char,
  mut name: *const libc::c_char,
  mut force_follow: libc::c_int,
) -> *mut dnode {
  let mut statbuf: stat = stat {
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
  let mut cur: *mut dnode = 0 as *mut dnode;
  cur = xzalloc(::std::mem::size_of::<dnode>() as libc::c_ulong) as *mut dnode;
  (*cur).fullname = fullname;
  (*cur).name = name;
  if option_mask32 & OPT_L as libc::c_int as libc::c_uint != 0 || force_follow != 0 {
    if stat(fullname, &mut statbuf) != 0 {
      bb_simple_perror_msg(fullname);
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).exit_code = 1i32 as smallint;
      free(cur as *mut libc::c_void);
      return 0 as *mut dnode;
    }
    (*cur).dn_mode_stat = statbuf.st_mode
  } else {
    if lstat(fullname, &mut statbuf) != 0 {
      bb_simple_perror_msg(fullname);
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).exit_code = 1i32 as smallint;
      free(cur as *mut libc::c_void);
      return 0 as *mut dnode;
    }
    (*cur).dn_mode_lstat = statbuf.st_mode
  }
  /* cur->dstat = statbuf: */
  (*cur).dn_mode = statbuf.st_mode;
  (*cur).dn_size = statbuf.st_size;
  (*cur).dn_time = statbuf.st_mtim.tv_sec;
  if option_mask32 & OPT_u as libc::c_int as libc::c_uint != 0 {
    (*cur).dn_time = statbuf.st_atim.tv_sec
  }
  if option_mask32 & OPT_c as libc::c_int as libc::c_uint != 0 {
    (*cur).dn_time = statbuf.st_ctim.tv_sec
  }
  (*cur).dn_ino = statbuf.st_ino;
  (*cur).dn_blocks = statbuf.st_blocks;
  (*cur).dn_nlink = statbuf.st_nlink;
  (*cur).dn_uid = statbuf.st_uid;
  (*cur).dn_gid = statbuf.st_gid;
  (*cur).dn_rdev_maj = gnu_dev_major(statbuf.st_rdev) as libc::c_int;
  (*cur).dn_rdev_min = gnu_dev_minor(statbuf.st_rdev) as libc::c_int;
  return cur;
}
unsafe extern "C" fn count_dirs(mut dn: *mut *mut dnode, mut which: libc::c_int) -> libc::c_uint {
  let mut dirs: libc::c_uint = 0;
  let mut all: libc::c_uint = 0;
  if dn.is_null() {
    return 0i32 as libc::c_uint;
  }
  all = 0i32 as libc::c_uint;
  dirs = all;
  while !(*dn).is_null() {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    all = all.wrapping_add(1);
    if (**dn).dn_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint {
      name = (**dn).name;
      if which != SPLIT_SUBDIR as libc::c_int
        || *name.offset(0) as libc::c_int != '.' as i32
        || *name.offset(1) as libc::c_int != 0
          && (*name.offset(1) as libc::c_int != '.' as i32 || *name.offset(2) as libc::c_int != 0)
      {
        dirs = dirs.wrapping_add(1)
      }
    }
    dn = dn.offset(1)
  }
  return if which != SPLIT_FILE as libc::c_int {
    dirs
  } else {
    all.wrapping_sub(dirs)
  };
}
/* get memory to hold an array of pointers */
unsafe extern "C" fn dnalloc(mut num: libc::c_uint) -> *mut *mut dnode {
  if num < 1i32 as libc::c_uint {
    return 0 as *mut *mut dnode;
  } /* so that we have terminating NULL */
  num = num.wrapping_add(1);
  return xzalloc(
    (num as libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut dnode>() as libc::c_ulong),
  ) as *mut *mut dnode;
}
unsafe extern "C" fn dfree(mut dnp: *mut *mut dnode) {
  let mut i: libc::c_uint = 0;
  if dnp.is_null() {
    return;
  }
  i = 0i32 as libc::c_uint;
  while !(*dnp.offset(i as isize)).is_null() {
    let mut cur: *mut dnode = *dnp.offset(i as isize);
    if (*cur).fname_allocated != 0 {
      free((*cur).fullname as *mut libc::c_char as *mut libc::c_void);
    }
    free(cur as *mut libc::c_void);
    i = i.wrapping_add(1)
  }
  free(dnp as *mut libc::c_void);
}
/* Returns NULL-terminated malloced vector of pointers (or NULL) */
unsafe extern "C" fn splitdnarray(
  mut dn: *mut *mut dnode,
  mut which: libc::c_int,
) -> *mut *mut dnode {
  let mut dncnt: libc::c_uint = 0;
  let mut d: libc::c_uint = 0;
  let mut dnp: *mut *mut dnode = 0 as *mut *mut dnode;
  if dn.is_null() {
    return 0 as *mut *mut dnode;
  }
  /* count how many dirs or files there are */
  dncnt = count_dirs(dn, which);
  /* allocate a file array and a dir array */
  dnp = dnalloc(dncnt);
  /* copy the entrys into the file or dir array */
  d = 0i32 as libc::c_uint; /* assume sort by name */
  while !(*dn).is_null() {
    if (**dn).dn_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint {
      let mut name: *const libc::c_char = 0 as *const libc::c_char;
      if !(which == SPLIT_FILE as libc::c_int) {
        name = (**dn).name;
        if which & SPLIT_DIR as libc::c_int != 0
          || *name.offset(0) as libc::c_int != '.' as i32
          || *name.offset(1) as libc::c_int != 0
            && (*name.offset(1) as libc::c_int != '.' as i32 || *name.offset(2) as libc::c_int != 0)
        {
          let fresh0 = d;
          d = d.wrapping_add(1);
          let ref mut fresh1 = *dnp.offset(fresh0 as isize);
          *fresh1 = *dn
        }
      }
    } else if which == SPLIT_FILE as libc::c_int {
      let fresh2 = d;
      d = d.wrapping_add(1);
      let ref mut fresh3 = *dnp.offset(fresh2 as isize);
      *fresh3 = *dn
    }
    dn = dn.offset(1)
  }
  return dnp;
}
unsafe extern "C" fn sortcmp(
  mut a: *const libc::c_void,
  mut b: *const libc::c_void,
) -> libc::c_int {
  let mut current_block: u64;
  let mut d1: *mut dnode = *(a as *mut *mut dnode);
  let mut d2: *mut dnode = *(b as *mut *mut dnode);
  let mut opt: libc::c_uint = option_mask32;
  let mut dif: off_t = 0;
  dif = 0i32 as off_t;
  // TODO: use pre-initialized function pointer
  // instead of branch forest
  if opt & OPT_dirs_first as libc::c_int as libc::c_uint != 0 {
    dif = (((*d2).dn_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint)
      as libc::c_int
      - ((*d1).dn_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint) as libc::c_int)
      as off_t;
    if dif != 0i32 as libc::c_long {
      current_block = 17197688095750354635;
    } else {
      current_block = 2473556513754201174;
    }
  } else {
    current_block = 2473556513754201174;
  }
  match current_block {
    2473556513754201174 => {
      if opt & OPT_S as libc::c_int as libc::c_uint != 0 {
        /* sort by size */
        dif = (*d2).dn_size - (*d1).dn_size
      } else if opt & OPT_t as libc::c_int as libc::c_uint != 0 {
        /* sort by time */
        dif = (*d2).dn_time - (*d1).dn_time
      } else if opt & OPT_v as libc::c_int as libc::c_uint != 0 {
        /* sort by version */
        dif = strverscmp((*d1).name, (*d2).name) as off_t
      } else if opt & OPT_X as libc::c_int as libc::c_uint != 0 {
        /* sort by extension */
        dif = strcmp(
          strchrnul((*d1).name, '.' as i32),
          strchrnul((*d2).name, '.' as i32),
        ) as off_t
      }
      if dif == 0i32 as libc::c_long {
        /* sort by name, use as tie breaker for other sorts */
        dif = strcmp((*d1).name, (*d2).name) as off_t
      } else if ::std::mem::size_of::<off_t>() as libc::c_ulong
        > ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
      {
        /* Make dif fit into an int */
        /* shift leaving only "int" worth of bits */
        /* (this requires dif != 0, and here it is nonzero) */
        dif = (1i32 | (dif as uoff_t >> BITS_TO_SHIFT as libc::c_int) as libc::c_int) as off_t
      }
    }
    _ => {}
  }
  return if opt & OPT_r as libc::c_int as libc::c_uint != 0 {
    -(dif as libc::c_int)
  } else {
    dif as libc::c_int
  };
}
unsafe extern "C" fn dnsort(mut dn: *mut *mut dnode, mut size: libc::c_int) {
  qsort(
    dn as *mut libc::c_void,
    size as size_t,
    ::std::mem::size_of::<*mut dnode>() as libc::c_ulong,
    Some(
      sortcmp
        as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int,
    ),
  );
}
unsafe extern "C" fn sort_and_display_files(mut dn: *mut *mut dnode, mut nfiles: libc::c_uint) {
  dnsort(dn, nfiles as libc::c_int);
  display_files(dn, nfiles);
}
/* Returns NULL-terminated malloced vector of pointers (or NULL) */
unsafe extern "C" fn scan_one_dir(
  mut path: *const libc::c_char,
  mut nfiles_p: *mut libc::c_uint,
) -> *mut *mut dnode {
  let mut dn: *mut dnode = 0 as *mut dnode;
  let mut cur: *mut dnode = 0 as *mut dnode;
  let mut dnp: *mut *mut dnode = 0 as *mut *mut dnode;
  let mut entry: *mut dirent = 0 as *mut dirent;
  let mut dir: *mut DIR = 0 as *mut DIR;
  let mut i: libc::c_uint = 0;
  let mut nfiles: libc::c_uint = 0;
  *nfiles_p = 0i32 as libc::c_uint;
  dir = warn_opendir(path);
  if dir.is_null() {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).exit_code = 1i32 as smallint;
    return 0 as *mut *mut dnode;
    /* could not open the dir */
  }
  dn = 0 as *mut dnode;
  nfiles = 0i32 as libc::c_uint;
  loop {
    entry = readdir(dir);
    if entry.is_null() {
      break;
      /* if only -A, skip . and .. but show other dotfiles */
    }
    let mut fullname: *mut libc::c_char = 0 as *mut libc::c_char;
    /* are we going to list the file- it may be . or .. or a hidden file */
    if (*entry).d_name[0] as libc::c_int == '.' as i32 {
      if option_mask32 & (OPT_a as libc::c_int | OPT_A as libc::c_int) as libc::c_uint == 0 {
        continue; /* skip all dotfiles if no -a/-A */
      }
      if option_mask32 & OPT_a as libc::c_int as libc::c_uint == 0
        && ((*entry).d_name[1] == 0
          || (*entry).d_name[1] as libc::c_int == '.' as i32 && (*entry).d_name[2] == 0)
      {
        continue;
      }
    }
    fullname = concat_path_file(path, (*entry).d_name.as_mut_ptr());
    cur = my_stat(fullname, bb_basename(fullname), 0i32);
    if cur.is_null() {
      free(fullname as *mut libc::c_void);
    } else {
      (*cur).fname_allocated = 1i32 as smallint;
      (*cur).dn_next = dn;
      dn = cur;
      nfiles = nfiles.wrapping_add(1)
    }
  }
  closedir(dir);
  if dn.is_null() {
    return 0 as *mut *mut dnode;
  }
  /* now that we know how many files there are
   * allocate memory for an array to hold dnode pointers
   */
  *nfiles_p = nfiles; /* save pointer to node in array */
  dnp = dnalloc(nfiles);
  i = 0i32 as libc::c_uint;
  loop {
    let ref mut fresh4 = *dnp.offset(i as isize);
    *fresh4 = dn;
    dn = (*dn).dn_next;
    if dn.is_null() {
      break;
    }
    i = i.wrapping_add(1)
  }
  return dnp;
}
/* http://www.opengroup.org/onlinepubs/9699919799/utilities/ls.html
 * If any of the -l, -n, -s options is specified, each list
 * of files within the directory shall be preceded by a
 * status line indicating the number of file system blocks
 * occupied by files in the directory in 512-byte units if
 * the -k option is not specified, or 1024-byte units if the
 * -k option is specified, rounded up to the next integral
 * number of units.
 */
/* by Jorgen Overgaard (jorgen AT antistaten.se) */
unsafe extern "C" fn calculate_blocks(mut dn: *mut *mut dnode) -> off_t {
  let mut blocks: uoff_t = 1i32 as uoff_t;
  if !dn.is_null() {
    while !(*dn).is_null() {
      /* st_blocks is in 512 byte blocks */
      blocks = (blocks as libc::c_ulong).wrapping_add((**dn).dn_blocks as libc::c_ulong) as uoff_t
        as uoff_t;
      dn = dn.offset(1)
    }
  }
  /* Even though standard says use 512 byte blocks, coreutils use 1k */
  /* Actually, we round up by calculating (blocks + 1) / 2,
   * "+ 1" was done when we initialized blocks to 1 */
  return (blocks >> 1i32) as off_t;
}
unsafe extern "C" fn scan_and_display_dirs_recur(mut dn: *mut *mut dnode, mut first: libc::c_int) {
  let mut nfiles: libc::c_uint = 0;
  let mut subdnp: *mut *mut dnode = 0 as *mut *mut dnode;
  while !(*dn).is_null() {
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).show_dirname as libc::c_int != 0
      || option_mask32 & OPT_R as libc::c_int as libc::c_uint != 0
    {
      if first == 0 {
        bb_putchar('\n' as i32);
      }
      first = 0i32;
      printf(
        b"%s:\n\x00" as *const u8 as *const libc::c_char,
        (**dn).fullname,
      );
    }
    subdnp = scan_one_dir((**dn).fullname, &mut nfiles);
    if option_mask32 & (OPT_s as libc::c_int | OPT_l as libc::c_int) as libc::c_uint != 0 {
      if option_mask32 & OPT_h as libc::c_int as libc::c_uint != 0 {
        printf(
          b"total %-7s\n\x00" as *const u8 as *const libc::c_char,
          make_human_readable_str(
            (calculate_blocks(subdnp) * 1024i32 as libc::c_long) as libc::c_ulonglong,
            0i32 as libc::c_ulong,
            0i32 as libc::c_ulong,
          ),
        );
      } else {
        printf(
          b"total %lu\n\x00" as *const u8 as *const libc::c_char,
          calculate_blocks(subdnp),
        );
      }
    }
    if nfiles > 0i32 as libc::c_uint {
      /* list all files at this level */
      sort_and_display_files(subdnp, nfiles);
      if 1i32 != 0 && option_mask32 & OPT_R as libc::c_int as libc::c_uint != 0 {
        let mut dnd: *mut *mut dnode = 0 as *mut *mut dnode;
        let mut dndirs: libc::c_uint = 0;
        /* recursive - list the sub-dirs */
        dnd = splitdnarray(subdnp, SPLIT_SUBDIR as libc::c_int);
        dndirs = count_dirs(subdnp, SPLIT_SUBDIR as libc::c_int);
        if dndirs > 0i32 as libc::c_uint {
          dnsort(dnd, dndirs as libc::c_int);
          scan_and_display_dirs_recur(dnd, 0i32);
          /* free the array of dnode pointers to the dirs */
          free(dnd as *mut libc::c_void);
        }
      }
      /* free the dnodes and the fullname mem */
      dfree(subdnp);
    }
    dn = dn.offset(1)
  }
}
/* vi: set sw=4 ts=4: */
/*
 * Busybox main internal header file
 *
 * Based in part on code from sash, Copyright (c) 1999 by David I. Bell
 * Permission has been granted to redistribute this code under GPL.
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* TODO: and glibc? */
/* There are two incompatible basename's, let's not use them! */
/* See the dirname/basename man page for details */
/* dirname,basename */
/* Don't do this here:
 * #include <sys/sysinfo.h>
 * Some linux/ includes pull in conflicting definition
 * of struct sysinfo (only in some toolchanins), which breaks build.
 * Include sys/sysinfo.h only in those files which need it.
 */
/* Just in case libc doesn't define some of these... */
//This breaks on bionic:
//# if !defined(__socklen_t_defined) && !defined(_SOCKLEN_T_DECLARED)
// /* We #define socklen_t *after* includes, otherwise we get
// * typedef redefinition errors from system headers
// * (in case "is it defined already" detection above failed)
// */
//#  define socklen_t bb_socklen_t
//   typedef unsigned socklen_t;
//# endif
//if this is still needed, add a fix along the lines of
//  ifdef SPECIFIC_BROKEN_LIBC_CHECK / typedef socklen_t / endif
//in platform.h instead!
/*
 * Use '%m' to append error string on platforms that support it,
 * '%s' and strerror() on those that don't.
 */
/*nothing*/
/* Some libc's forget to declare these, do it ourself */
/* klogctl is in libc's klog.h, but we cheat and not #include that */
/* Busybox does not use threads, we can speed up stdio. */
/* Above functions are required by POSIX.1-2008, below ones are extensions */
/* musl <= 1.1.15 does not support fflush_unlocked(NULL) */
//# undef  fflush
//# define fflush(stream) fflush_unlocked(stream)
/* Make all declarations hidden (-fvisibility flag only affects definitions) */
/* (don't include system headers after this until corresponding pop!) */
/* Tested to work correctly with all int types (IIRC :]) */
/* Large file support */
/* Note that CONFIG_LFS=y forces bbox to be built with all common ops
 * (stat, lseek etc) mapped to "largefile" variants by libc.
 * Practically it means that open() automatically has O_LARGEFILE added
 * and all filesize/file_offset parameters and struct members are "large"
 * (in today's world - signed 64bit). For full support of large files,
 * we need a few helper #defines (below) and careful use of off_t
 * instead of int/ssize_t. No lseek64(), O_LARGEFILE etc necessary */
/* CONFIG_LFS is on */
/* "long" is long enough on this system */
/* usage: sz = BB_STRTOOFF(s, NULL, 10); if (errno || sz < 0) die(); */
/* usage: printf("size: %"OFF_FMT"d (%"OFF_FMT"x)\n", sz, sz); */
/* scary. better ideas? (but do *test* them first!) */
/* Users report bionic to use 32-bit off_t even if LARGEFILE support is requested.
 * We misdetected that. Don't let it build:
 */
/* Some useful definitions */
/* Macros for min/max.  */
/* buffer allocation schemes */
/* glibc uses __errno_location() to get a ptr to errno */
/* We can just memorize it once - no multithreading in busybox :) */
/* dmalloc will redefine these to it's own implementation. It is safe
 * to have the prototypes here unconditionally.  */
/* After v = xrealloc_vector(v, SHIFT, idx) it's ok to use
 * at least v[idx] and v[idx+1], for all idx values.
 * SHIFT specifies how many new elements are added (1:2, 2:4, ..., 8:256...)
 * when all elements are used up. New elements are zeroed out.
 * xrealloc_vector(v, SHIFT, idx) *MUST* be called with consecutive IDXs -
 * skipping an index is a bad bug - it may miss a realloc!
 */
//TODO: supply a pointer to char[11] buffer (avoid statics)?
/* cp.c, mv.c, install.c depend on these values. CAREFUL when changing them! */
/* -p */
/* !-d */
/* -R */
/* -f */
/* -i */
/* -l */
/* -s */
/* -L */
/* -H */
/* -a = -pdR (mapped in cp.c) */
/* -r = -dR  (mapped in cp.c) */
/* -P = -d   (mapped in cp.c) */
/* -v */
/* -u */
/* -T */
/* --remove-destination */
/* bit 17 skipped for "cp --parents" */
/* cp --reflink=auto */
/* cp --reflink[=always] */
/*
 * Hole. cp may have some bits set here,
 * they should not affect remove_file()/copy_file()
 */
/* NB: without FILEUTILS_RECUR in flags, it will basically "cat"
 * the source, not copy (unless "source" is a directory).
 * This makes "cp /dev/null file" and "install /dev/null file" (!!!)
 * work coreutils-compatibly. */
/*ACTION_REVERSE      = (1 << 4), - unused */
/* more than enough for "/dev/ttyXXX" */
/* bb_copyfd_XX print read/write errors and return -1 if they occur */
/* "short" copy can be detected by return value < size */
/* this helper yells "short read!" if param is not -1 */
/* xxxx_strip version can modify its parameter:
 * "/"        -> "/"
 * "abc"      -> "abc"
 * "abc/def"  -> "def"
 * "abc/def/" -> "def" !!
 */
/* "abc/def/" -> "" and it never modifies 'path' */
/* Simpler version: does not special case "/" string */
/* NB: can violate const-ness (similarly to strchr) */
/* !RETURNS_MALLOC: it's a realloc-like function */
/* bb_signals(BB_FATAL_SIGS, handler) catches all signals which
 * otherwise would kill us, except for those resulting from bugs:
 * SIGSEGV, SIGILL, SIGFPE.
 * Other fatal signals not included (TODO?):
 * SIGBUS   Bus error (bad memory access)
 * SIGPOLL  Pollable event. Synonym of SIGIO
 * SIGPROF  Profiling timer expired
 * SIGSYS   Bad argument to routine
 * SIGTRAP  Trace/breakpoint trap
 *
 * The only known arch with some of these sigs not fitting
 * into 32 bits is parisc (SIGXCPU=33, SIGXFSZ=34, SIGSTKFLT=36).
 * Dance around with long long to guard against that...
 */
// Write to pipe with no readers
// Quit from keyboard
// Abort signal from abort(3)
// Timer signal from alarm(2)
// Virtual alarm clock
// CPU time limit exceeded
// File size limit exceeded
// Yes kids, these are also fatal!
/* Unlike signal() and bb_signals, sets handler with sigaction()
 * and in a way that while signal handler is run, no other signals
 * will be blocked; syscalls will not be restarted: */
/* syscalls like read() will be interrupted with EINTR: */
/* syscalls like read() won't be interrupted (though select/poll will be): */
/* Will do sigaction(signum, act, NULL): */
/* SIG_BLOCK/SIG_UNBLOCK all signals: */
/* Return old set in the same set: */
/* Standard handler which just records signo */
/* not FAST_FUNC! */
/* In this form code with pipes is much more readable */
/* Useful for having small structure members/global variables */
/* | AF_DECnet */
/* | AF_IPX */
/* SO_REUSEADDR allows a server to rebind to an address that is already
 * "in use" by old connections to e.g. previous server instance which is
 * killed or crashed. Without it bind will fail until all such connections
 * time out. Linux does not allow multiple live binds on same ip:port
 * regardless of SO_REUSEADDR (unlike some other flavors of Unix).
 * Turn it on before you call bind(). */
/* On Linux this never fails. */
/* NB: returns port in host byte order */
/* Create stream socket, and allocate suitable lsa.
 * (lsa of correct size and lsa->sa.sa_family (AF_INET/AF_INET6))
 * af == AF_UNSPEC will result in trying to create IPv6 socket,
 * and if kernel doesn't support it, fall back to IPv4.
 * This is useful if you plan to bind to resulting local lsa.
 */
/* Create server socket bound to bindaddr:port. bindaddr can be NULL,
 * numeric IP ("N.N.N.N") or numeric IPv6 address,
 * and can have ":PORT" suffix (for IPv6 use "[X:X:...:X]:PORT").
 * Only if there is no suffix, port argument is used */
/* NB: these set SO_REUSEADDR before bind */
/* Create client TCP socket connected to peer:port. Peer cannot be NULL.
 * Peer can be numeric IP ("N.N.N.N"), numeric IPv6 address or hostname,
 * and can have ":PORT" suffix (for IPv6 use "[X:X:...:X]:PORT").
 * If there is no suffix, port argument is used */
/* Connect to peer identified by lsa */
/* Get local address of bound or accepted socket */
/* Get remote address of connected or accepted socket */
/* Return malloc'ed len_and_sockaddr with socket address of host:port
 * Currently will return IPv4 or IPv6 sockaddrs only
 * (depending on host), but in theory nothing prevents e.g.
 * UNIX socket address being returned, IPX sockaddr etc...
 * On error does bb_error_msg and returns NULL */
/* Version which dies on error */
/* Same, useful if you want to force family (e.g. IPv6) */
/* Assign sin[6]_port member if the socket is an AF_INET[6] one,
 * otherwise no-op. Useful for ftp.
 * NB: does NOT do htons() internally, just direct assignment. */
/* Retrieve sin[6]_port or return -1 for non-INET[6] lsa's */
/* Reverse DNS. Returns NULL on failure. */
/* This one doesn't append :PORTNUM */
/* This one also doesn't fall back to dotted IP (returns NULL) */
/* inet_[ap]ton on steroids */
// "old" (ipv4 only) API
// users: traceroute.c hostname.c - use _list_ of all IPs
// Also mount.c and inetd.c are using gethostbyname(),
// + inet_common.c has additional IPv4-only stuff
/* opaque */
// RFC 5246
// sequence number
//   Each connection state contains a sequence number, which is
//   maintained separately for read and write states.  The sequence
//   number MUST be set to zero whenever a connection state is made the
//   active state.  Sequence numbers are of type uint64 and may not
//   exceed 2^64-1.
/*uint64_t read_seq64_be;*/
/*uint8_t *server_write_MAC_key;*/
//used by AES_GCM
/* 0 if argv[0] is NULL: */
/* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
 * But potentially slow, don't use in one-billion-times loops */
/* Note: does not use stdio, writes to fd 2 directly */
// gcc-4.1.1 still isn't good enough at optimizing it
// (+200 bytes compared to macro)
//static ALWAYS_INLINE
//int LONE_DASH(const char *s) { return s[0] == '-' && !s[1]; }
//static ALWAYS_INLINE
//int NOT_LONE_DASH(const char *s) { return s[0] != '-' || s[1]; }
/* Returns a string with unprintable chars replaced by '?' or
 * SUBST_WCHAR. This function is unicode-aware. */
/* Prints unprintable char ch as ^C or M-c to file
 * (M-c is used only if ch is ORed with PRINTABLE_META),
 * else it is printed as-is (except for ch = 0x9b) */
/* Return a string that is the printable representation of character ch.
 * Buffer must hold at least four characters. */
// NB: will return short read on error, not -1,
// if some data was read before error occurred
// Reads one line a-la fgets (but doesn't save terminating '\n').
// Reads byte-by-byte. Useful when it is important to not read ahead.
// Bytes are appended to pfx (which must be malloced, or NULL).
/* Reads block up to *maxsz_p (default: INT_MAX - 4095) */
/* Returns NULL if file can't be opened (default max size: INT_MAX - 4095) */
/* Never returns NULL */
/* Else use variable one (a bit more expensive) */
/* Autodetects gzip/bzip2 formats. fd may be in the middle of the file! */
/* Autodetects .gz etc */
/* lzma has no signature, need a little helper. NB: exist only for ENABLE_FEATURE_SEAMLESS_LZMA=y */
// NB: will return short write on error, not -1,
// if some data was written before error occurred
/* Close fd, but check for failures (some types of write errors) */
/* Reads and prints to stdout till eof, then closes FILE. Exits on error: */
/* Reads a line from a text file, up to a newline or NUL byte, inclusive.
 * Returns malloc'ed char*. If end is NULL '\n' isn't considered
 * end of line. If end isn't NULL, length of the chunk is stored in it.
 * Returns NULL if EOF/error.
 */
/* Reads up to (and including) TERMINATING_STRING: */
/* Same, with limited max size, and returns the length (excluding NUL): */
/* Chops off TERMINATING_STRING from the end: */
/* Reads up to (and including) "\n" or NUL byte: */
/* Chops off '\n' from the end, unlike fgets: */
/* Same, but doesn't try to conserve space (may have some slack after the end) */
/* extern char *xmalloc_fgetline_fast(FILE *file) FAST_FUNC RETURNS_MALLOC; */
/* Prints warning to stderr and returns NULL on failure: */
/* "Opens" stdin if filename is special, else just opens file: */
/* not FAST_FUNC! */
/* Wrapper which restarts poll on EINTR or ENOMEM.
 * On other errors complains [perror("poll")] and returns.
 * Warning! May take (much) longer than timeout_ms to return!
 * If this is a problem, use bare poll and open-code EINTR/ENOMEM handling */
/* Convert each alpha char in str to lower-case */
/* Returns a pointer past the formatted number, does NOT null-terminate */
/* Intelligent formatters of bignums */
/* If block_size == 0, display size without fractional part,
 * else display (size * block_size) with one decimal digit.
 * If display_unit == 0, show value no bigger than 1024 with suffix (K,M,G...),
 * else divide by display_unit and do not use suffix. */
/* "1024.0G" */
//TODO: provide pointer to buf (avoid statics)?
/* Put a string of hex bytes ("1b2e66fe"...), return advanced pointer */
/* Reverse */
/* Generate a UUID */
/* Last element is marked by mult == 0 */
/* Specialized: */
/* Using xatoi() instead of naive atoi() is not always convenient -
 * in many places people want *non-negative* values, but store them
 * in signed int. Therefore we need this one:
 * dies if input is not in [0, INT_MAX] range. Also will reject '-0' etc.
 * It should really be named xatoi_nonnegative (since it allows 0),
 * but that would be too long.
 */
/* Useful for reading port numbers */
/* These parse entries in /etc/passwd and /etc/group.  This is desirable
 * for BusyBox since we want to avoid using the glibc NSS stuff, which
 * increases target size and is often not needed on embedded systems.  */
/* wrapper: allows string to contain numeric uid or gid */
/* always sets uid and gid; returns 0 on failure */
/* always sets uid and gid; exits on failure */
/* chown-like handling of "user[:[group]" */
/* versions which cache results (useful for ps, ls etc) */
/* internally usernames are saved in fixed-sized char[] buffers */
/*
 * Returns (-1) terminated malloced result of getgroups().
 * Reallocs group_array (useful for repeated calls).
 * ngroups is an initial size of array. It is rounded up to 32 for realloc.
 * ngroups is updated on return.
 * ngroups can be NULL: bb_getgroups(NULL, NULL) is valid usage.
 * Dies on errors (on Linux, only xrealloc can cause this, not internal getgroups call).
 */
/* BB_EXECxx always execs (it's not doing NOFORK/NOEXEC stuff),
 * but it may exec busybox and call applet instead of searching PATH.
 */
/* xvfork() can't be a _function_, return after vfork in child mangles stack
 * in the parent. It must be a macro. */
/* NOMMU friendy fork+exec: */
/* wait4pid: unlike waitpid, waits ONLY for one process.
 * Returns sig + 0x180 if child is killed by signal.
 * It's safe to pass negative 'pids' from failed [v]fork -
 * wait4pid will return -1 (and will not clobber [v]fork's errno).
 * IOW: rc = wait4pid(spawn(argv));
 *      if (rc < 0) bb_perror_msg("%s", argv[0]);
 *      if (rc > 0) bb_error_msg("exit code: %d", rc & 0xff);
 */
/* ***********************************************************************/
/* spawn_and_wait/run_nofork_applet/run_applet_no_and_exit need to work */
/* carefully together to reinit some global state while not disturbing  */
/* other. Be careful if you change them. Consult docs/nofork_noexec.txt */
/* ***********************************************************************/
/* Same as wait4pid(spawn(argv)), but with NOFORK/NOEXEC if configured: */
/* Does NOT check that applet is NOFORK, just blindly runs it */
/* Helpers for daemonization.
 *
 * bb_daemonize(flags) = daemonize, does not compile on NOMMU
 *
 * bb_daemonize_or_rexec(flags, argv) = daemonizes on MMU (and ignores argv),
 *      rexec's itself on NOMMU with argv passed as command line.
 * Thus bb_daemonize_or_rexec may cause your <applet>_main() to be re-executed
 * from the start. (It will detect it and not reexec again second time).
 * You have to audit carefully that you don't do something twice as a result
 * (opening files/sockets, parsing config files etc...)!
 *
 * Both of the above will redirect fd 0,1,2 to /dev/null and drop ctty
 * (will do setsid()).
 *
 * fork_or_rexec(argv) = bare-bones fork on MMU,
 *      "vfork + re-exec ourself" on NOMMU. No fd redirection, no setsid().
 *      On MMU ignores argv.
 *
 * Helper for network daemons in foreground mode:
 *
 * bb_sanitize_stdio() = make sure that fd 0,1,2 are opened by opening them
 * to /dev/null if they are not.
 */
/* internal use */
//DAEMON_DOUBLE_FORK     = 1 << 4, /* double fork to avoid controlling tty */
/* Clear dangerous stuff, set PATH. Return 1 if was run by different user. */
/* For top, ps. Some argv[i] are replaced by malloced "-opt" strings */
/* { "-", NULL } */
/* BSD-derived getopt() functions require that optind be set to 1 in
 * order to reset getopt() state.  This used to be generally accepted
 * way of resetting getopt().  However, glibc's getopt()
 * has additional getopt() state beyond optind (specifically, glibc
 * extensions such as '+' and '-' at the start of the string), and requires
 * that optind be set to zero to reset its state.  BSD-derived versions
 * of getopt() misbehaved if optind is set to 0 in order to reset getopt(),
 * and glibc's getopt() used to coredump if optind is set 1 in order
 * to reset getopt().
 * Then BSD introduced additional variable "optreset" which should be
 * set to 1 in order to reset getopt().  Sigh.  Standards, anyone?
 *
 * By ~2008, OpenBSD 3.4 was changed to survive glibc-like optind = 0
 * (to interpret it as if optreset was set).
 */
/*def __GLIBC__*/
/* BSD style */
/* Having next pointer as a first member allows easy creation
 * of "llist-compatible" structs, and using llist_FOO functions
 * on them.
 */
/* BTW, surprisingly, changing API to
 *   llist_t *llist_add_to(llist_t *old_head, void *data)
 * etc does not result in smaller code... */
/* start_stop_daemon and udhcpc are special - they want
 * to create pidfiles regardless of FEATURE_PIDFILE */
/* True only if we created pidfile which is *file*, not /dev/null etc */
/* We need to export XXX_main from libbusybox
 * only if we build "individual" binaries
 */
/* Embedded script support */
/* Applets which are useful from another applets */
/* If shell needs them, they exist even if not enabled as applets */
/* Similar, but used by chgrp, not shell */
/* Used by ftpd */
#[no_mangle]
pub unsafe extern "C" fn ls_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  /*      ^^^^^^^^^^^^^^^^^ note: if FTPD, argc can be wrong, see ftpd.c */
  let mut dnd: *mut *mut dnode = 0 as *mut *mut dnode;
  let mut dnf: *mut *mut dnode = 0 as *mut *mut dnode;
  let mut dnp: *mut *mut dnode = 0 as *mut *mut dnode;
  let mut dn: *mut dnode = 0 as *mut dnode;
  let mut cur: *mut dnode = 0 as *mut dnode;
  let mut opt: libc::c_uint = 0;
  let mut nfiles: libc::c_uint = 0;
  let mut dnfiles: libc::c_uint = 0;
  let mut dndirs: libc::c_uint = 0;
  let mut i: libc::c_uint = 0;
  /* colored LS support by JaWi, janwillem.janssen@lxtreme.nl */
  /* coreutils 6.10:
   * # ls --color=BOGUS
   * ls: invalid argument 'BOGUS' for '--color'
   * Valid arguments are:
   * 'always', 'yes', 'force'
   * 'never', 'no', 'none'
   * 'auto', 'tty', 'if-tty'
   * (and substrings: "--color=alwa" work too)
   */
  static mut color_str: [libc::c_char; 34] = [
    97, 108, 119, 97, 121, 115, 0, 121, 101, 115, 0, 102, 111, 114, 99, 101, 0, 97, 117, 116, 111,
    0, 116, 116, 121, 0, 105, 102, 45, 116, 116, 121, 0, 0,
  ];
  /* need to initialize since --color has _an optional_ argument */
  let mut color_opt: *const libc::c_char = color_str.as_ptr(); /* "always" */
  static mut ls_longopts: [libc::c_char; 47] = [
    102, 117, 108, 108, 45, 116, 105, 109, 101, 0, 0, -1, 103, 114, 111, 117, 112, 45, 100, 105,
    114, 101, 99, 116, 111, 114, 105, 101, 115, 45, 102, 105, 114, 115, 116, 0, 0, -2, 99, 111,
    108, 111, 114, 0, 2, -3, 0,
  ];
  memset(
    bb_common_bufsiz1.as_mut_ptr() as *mut globals as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<globals>() as libc::c_ulong,
  );
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).terminal_width =
    TERMINAL_WIDTH as libc::c_int as libc::c_uint;
  time(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).current_time_t);
  /* obtain the terminal width */
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).terminal_width =
    get_terminal_width(0i32) as libc::c_uint;
  /* go one less... */
  let ref mut fresh5 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).terminal_width;
  *fresh5 = (*fresh5).wrapping_sub(1);
  /* process options */
  opt =
        getopt32long(argv,
                     b"^Cadi1lgnsxAkFpRQctuSXrvLHhT:w:\x00nl:gl:\xffl:t-S:S-t:H-L:L-H:C-xl:x-Cl:l-xC:C-1:1-C:x-1:1-x:c-u:u-c:w+\x00"
                         as *const u8 as *const libc::c_char,
                     ls_longopts.as_ptr(), 0 as *mut libc::c_void,
                     &mut (*(bb_common_bufsiz1.as_mut_ptr() as
                                 *mut globals)).terminal_width as
                         *mut libc::c_uint,
                     &mut color_opt as *mut *const libc::c_char);
  /* option bits debug */
  /* set G_show_color = 1/0 */
  if 1i32 != 0 && isatty(1i32) != 0 {
    let mut p: *mut libc::c_char = getenv(b"LS_COLORS\x00" as *const u8 as *const libc::c_char);
    /* LS_COLORS is unset, or (not empty && not "none") ? */
    if p.is_null()
      || *p.offset(0) as libc::c_int != 0
        && strcmp(p, b"none\x00" as *const u8 as *const libc::c_char) != 0i32
    {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).show_color = 1i32 as smallint
    }
  }
  if opt & OPT_color as libc::c_int as libc::c_uint != 0 {
    if *color_opt.offset(0) as libc::c_int == 'n' as i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).show_color = 0i32 as smallint
    } else {
      let mut current_block_15: u64;
      match index_in_substrings(color_str.as_ptr(), color_opt) {
        3 | 4 | 5 => {
          if isatty(1i32) != 0 {
            current_block_15 = 1012356763440963274;
          } else {
            current_block_15 = 14359455889292382949;
          }
        }
        0 | 1 | 2 => {
          current_block_15 = 1012356763440963274;
        }
        _ => {
          current_block_15 = 14359455889292382949;
        }
      }
      match current_block_15 {
        1012356763440963274 => {
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).show_color = 1i32 as smallint
        }
        _ => {}
      }
    }
  }
  /* sort out which command line options take precedence */
  if 1i32 != 0 && opt & OPT_d as libc::c_int as libc::c_uint != 0 {
    option_mask32 &= !(OPT_R as libc::c_int) as libc::c_uint
  } /* no recurse if listing only dir */
  if opt & OPT_l as libc::c_int as libc::c_uint == 0 {
    /* not -l? */
    if 1i32 != 0 && 1i32 != 0 {
      /* when to sort by time? -t[cu] sorts by time even with -l */
      /* (this is achieved by opt_flags[] element for -t) */
      /* without -l, bare -c or -u enable sort too */
      /* (with -l, bare -c or -u just select which time to show) */
      if opt & (OPT_c as libc::c_int | OPT_u as libc::c_int) as libc::c_uint != 0 {
        option_mask32 |= OPT_t as libc::c_int as libc::c_uint
      }
    }
  }
  /* choose a display format if one was not already specified by an option */
  if option_mask32
    & (OPT_l as libc::c_int | OPT_1 as libc::c_int | OPT_x as libc::c_int | OPT_C as libc::c_int)
      as libc::c_uint
    == 0
  {
    option_mask32 |= if isatty(1i32) != 0 {
      OPT_C as libc::c_int
    } else {
      OPT_1 as libc::c_int
    } as libc::c_uint
  }
  if 1i32 != 0 && *applet_name.offset(0) as libc::c_int == 'f' as i32 {
    /* ftpd secret backdoor. dirs first are much nicer */
    option_mask32 |= OPT_dirs_first as libc::c_int as libc::c_uint
  } /* 2 or more items? label directories */
  argv = argv.offset(optind as isize);
  if (*argv.offset(0)).is_null() {
    argv = argv.offset(-1);
    *argv = b".\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }
  if !(*argv.offset(1)).is_null() {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).show_dirname = 1i32 as smallint
  }
  /* stuff the command line file names into a dnode array */
  dn = 0 as *mut dnode;
  nfiles = 0i32 as libc::c_uint;
  loop {
    cur = my_stat(
      *argv,
      *argv,
      (option_mask32
        & (OPT_l as libc::c_int
          | OPT_i as libc::c_int
          | OPT_s as libc::c_int
          | OPT_F as libc::c_int) as libc::c_uint
        == 0
        || option_mask32 & OPT_H as libc::c_int as libc::c_uint != 0) as libc::c_int,
    );
    argv = argv.offset(1);
    if !cur.is_null() {
      /*cur->fname_allocated = 0; - already is */
      (*cur).dn_next = dn;
      dn = cur;
      nfiles = nfiles.wrapping_add(1)
    }
    if (*argv).is_null() {
      break;
    }
  }
  /* nfiles _may_ be 0 here - try "ls doesnt_exist" */
  if nfiles == 0i32 as libc::c_uint {
    return (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).exit_code as libc::c_int;
  }
  /* now that we know how many files there are
   * allocate memory for an array to hold dnode pointers
   */
  dnp = dnalloc(nfiles); /* save pointer to node in array */
  i = 0i32 as libc::c_uint;
  loop {
    let ref mut fresh6 = *dnp.offset(i as isize);
    *fresh6 = dn;
    dn = (*dn).dn_next;
    if dn.is_null() {
      break;
    }
    i = i.wrapping_add(1)
  }
  if option_mask32 & OPT_d as libc::c_int as libc::c_uint != 0 {
    sort_and_display_files(dnp, nfiles);
  } else {
    dnd = splitdnarray(dnp, SPLIT_DIR as libc::c_int);
    dnf = splitdnarray(dnp, SPLIT_FILE as libc::c_int);
    dndirs = count_dirs(dnp, SPLIT_DIR as libc::c_int);
    dnfiles = nfiles.wrapping_sub(dndirs);
    if dnfiles > 0i32 as libc::c_uint {
      sort_and_display_files(dnf, dnfiles);
    }
    if dndirs > 0i32 as libc::c_uint {
      dnsort(dnd, dndirs as libc::c_int);
      scan_and_display_dirs_recur(dnd, (dnfiles == 0i32 as libc::c_uint) as libc::c_int);
    }
  }
  return (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).exit_code as libc::c_int;
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
    ::std::process::exit(ls_main(
      (args.len() - 1) as libc::c_int,
      args.as_mut_ptr() as *mut *mut libc::c_char,
    ) as i32)
  }
}

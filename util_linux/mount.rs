use libc;

extern "C" {
  #[no_mangle]
  fn setmntent(__file: *const libc::c_char, __mode: *const libc::c_char) -> *mut FILE;

  #[no_mangle]
  fn getmntent_r(
    __stream: *mut FILE,
    __result: *mut mntent,
    __buffer: *mut libc::c_char,
    __bufsize: libc::c_int,
  ) -> *mut mntent;

  #[no_mangle]
  fn addmntent(__stream: *mut FILE, __mnt: *const mntent) -> libc::c_int;

  #[no_mangle]
  fn endmntent(__stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn mount(
    __special_file: *const libc::c_char,
    __dir: *const libc::c_char,
    __fstype: *const libc::c_char,
    __rwflag: libc::c_ulong,
    __data: *const libc::c_void,
  ) -> libc::c_int;

  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);

  #[no_mangle]
  fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> libc::c_int;

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;

  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;

  #[no_mangle]
  fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;

  #[no_mangle]
  static bb_errno: *mut libc::c_int;

  #[no_mangle]
  fn skip_whitespace(_: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn host2sockaddr(host: *const libc::c_char, port: libc::c_int) -> *mut len_and_sockaddr;

  #[no_mangle]
  fn xhost2sockaddr(host: *const libc::c_char, port: libc::c_int) -> *mut len_and_sockaddr;

  #[no_mangle]
  fn xmalloc_sockaddr2dotted_noport(sa: *const sockaddr) -> *mut libc::c_char;

  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;

  #[no_mangle]
  fn xmalloc_fgetline(file: *mut FILE) -> *mut libc::c_char;

  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;

  #[no_mangle]
  fn spawn_and_wait(argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn sanitize_env_if_suid() -> libc::c_int;

  #[no_mangle]
  static mut option_mask32: uint32_t;

  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;

  #[no_mangle]
  fn llist_add_to_end(list_head: *mut *mut llist_t, data: *mut libc::c_void);

  #[no_mangle]
  fn llist_pop(elm: *mut *mut llist_t) -> *mut libc::c_void;

  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);

  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);

  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;

  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;

  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);

  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);

  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;

  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;

  #[no_mangle]
  fn fstype_matches(fstype: *const libc::c_char, comma_list: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn find_mount_point(name: *const libc::c_char, subdir_too: libc::c_int) -> *mut mntent;

  #[no_mangle]
  fn del_loop(device: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn set_loop(
    devname: *mut *mut libc::c_char,
    file: *const libc::c_char,
    offset: libc::c_ulonglong,
    flags: libc::c_uint,
  ) -> libc::c_int;

  #[no_mangle]
  fn bb_simplify_path(path: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  static bb_msg_perm_denied_are_you_root: [libc::c_char; 0];

  #[no_mangle]
  static bb_msg_you_must_be_root: [libc::c_char; 0];

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];

  /* Returns:
   * 0: no UUID= or LABEL= prefix found
   * 1: UUID= or LABEL= prefix found. In this case,
   *    *fsname is replaced if device with such UUID or LABEL is found
   */
  #[no_mangle]
  fn resolve_mount_spec(fsname: *mut *mut libc::c_char) -> libc::c_int;
}



pub type size_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;


use crate::librb::FILE;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mntent {
  pub mnt_fsname: *mut libc::c_char,
  pub mnt_dir: *mut libc::c_char,
  pub mnt_type: *mut libc::c_char,
  pub mnt_opts: *mut libc::c_char,
  pub mnt_freq: libc::c_int,
  pub mnt_passno: libc::c_int,
}

pub type C2RustUnnamed = libc::c_int;
// pub const MS_NOUSER: C2RustUnnamed = -2147483648;
// pub const MS_ACTIVE: C2RustUnnamed = 1073741824;
// pub const MS_LAZYTIME: C2RustUnnamed = 33554432;
pub const MS_STRICTATIME: C2RustUnnamed = 16777216;
// pub const MS_I_VERSION: C2RustUnnamed = 8388608;
// pub const MS_KERNMOUNT: C2RustUnnamed = 4194304;
pub const MS_RELATIME: C2RustUnnamed = 2097152;
pub const MS_SHARED: C2RustUnnamed = 1048576;
pub const MS_SLAVE: C2RustUnnamed = 524288;
pub const MS_PRIVATE: C2RustUnnamed = 262144;
pub const MS_UNBINDABLE: C2RustUnnamed = 131072;
// pub const MS_POSIXACL: C2RustUnnamed = 65536;
pub const MS_SILENT: C2RustUnnamed = 32768;
// pub const MS_REC: C2RustUnnamed = 16384;
pub const MS_MOVE: C2RustUnnamed = 8192;
pub const MS_BIND: C2RustUnnamed = 4096;
pub const MS_NODIRATIME: C2RustUnnamed = 2048;
pub const MS_NOATIME: C2RustUnnamed = 1024;
pub const MS_DIRSYNC: C2RustUnnamed = 128;
pub const MS_MANDLOCK: C2RustUnnamed = 64;
pub const MS_REMOUNT: C2RustUnnamed = 32;
pub const MS_SYNCHRONOUS: C2RustUnnamed = 16;
pub const MS_NOEXEC: C2RustUnnamed = 8;
pub const MS_NODEV: C2RustUnnamed = 4;
pub const MS_NOSUID: C2RustUnnamed = 2;
pub const MS_RDONLY: C2RustUnnamed = 1;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type socklen_t = __socklen_t;

use crate::librb::timespec;

use crate::librb::stat;

pub type sa_family_t = libc::c_ushort;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
  pub sa_family: sa_family_t,
  pub sa_data: [libc::c_char; 14],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
  pub sin6_family: sa_family_t,
  pub sin6_port: in_port_t,
  pub sin6_flowinfo: uint32_t,
  pub sin6_addr: in6_addr,
  pub sin6_scope_id: uint32_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
  pub __in6_u: C2RustUnnamed_0,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
  pub __u6_addr8: [uint8_t; 16],
  pub __u6_addr16: [uint16_t; 8],
  pub __u6_addr32: [uint32_t; 4],
}
pub type in_port_t = uint16_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
  pub sin_family: sa_family_t,
  pub sin_port: in_port_t,
  pub sin_addr: in_addr,
  pub sin_zero: [libc::c_uchar; 8],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
  pub s_addr: in_addr_t,
}

pub type in_addr_t = uint32_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct len_and_sockaddr {
  pub len: socklen_t,
  pub u: C2RustUnnamed_1,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
  pub sa: sockaddr,
  pub sin: sockaddr_in,
  pub sin6: sockaddr_in6,
}

use crate::libbb::llist::llist_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub verbose: libc::c_uint,
  pub fslist: *mut llist_t,
  pub getmntent_buf: [libc::c_char; 1],
}

pub type C2RustUnnamed_2 = libc::c_uint;
pub const MOUNT_FAKEFLAGS: C2RustUnnamed_2 = 2013265920;
pub const MOUNT_SWAP: C2RustUnnamed_2 = 1073741824;
pub const MOUNT_NOAUTO: C2RustUnnamed_2 = 536870912;
pub const MOUNT_NOFAIL: C2RustUnnamed_2 = 268435456;
pub const MOUNT_USERS: C2RustUnnamed_2 = 134217728;

pub type C2RustUnnamed_3 = libc::c_uint;
// pub const OPT_T: C2RustUnnamed_3 = 2048;
// pub const OPT_O: C2RustUnnamed_3 = 1024;
// pub const OPT_i: C2RustUnnamed_3 = 512;
// pub const OPT_s: C2RustUnnamed_3 = 256;
// pub const OPT_v: C2RustUnnamed_3 = 128;
pub const OPT_f: C2RustUnnamed_3 = 64;
// pub const OPT_n: C2RustUnnamed_3 = 32;
pub const OPT_a: C2RustUnnamed_3 = 16;
pub const OPT_w: C2RustUnnamed_3 = 8;
pub const OPT_r: C2RustUnnamed_3 = 4;
// pub const OPT_t: C2RustUnnamed_3 = 2;
// pub const OPT_o: C2RustUnnamed_3 = 1;

pub type C2RustUnnamed_4 = libc::c_uint;
pub const GETMNTENT_BUFSIZE: C2RustUnnamed_4 = 1008;

static mut mount_options: [int32_t; 44] = [
  0i32,
  0i32,
  MOUNT_NOAUTO as libc::c_int,
  MOUNT_SWAP as libc::c_int,
  MOUNT_SWAP as libc::c_int,
  MOUNT_USERS as libc::c_int,
  MOUNT_USERS as libc::c_int,
  MOUNT_NOFAIL as libc::c_int,
  0i32,
  0i32,
  MS_NOSUID as libc::c_int,
  !(MS_NOSUID as libc::c_int),
  !(MS_NODEV as libc::c_int),
  MS_NODEV as libc::c_int,
  !(MS_NOEXEC as libc::c_int),
  MS_NOEXEC as libc::c_int,
  MS_SYNCHRONOUS as libc::c_int,
  MS_DIRSYNC as libc::c_int,
  !(MS_SYNCHRONOUS as libc::c_int),
  !(MS_NOATIME as libc::c_int),
  MS_NOATIME as libc::c_int,
  !(MS_NODIRATIME as libc::c_int),
  MS_NODIRATIME as libc::c_int,
  MS_MANDLOCK as libc::c_int,
  !(MS_MANDLOCK as libc::c_int),
  MS_RELATIME as libc::c_int,
  !(MS_RELATIME as libc::c_int),
  MS_STRICTATIME as libc::c_int,
  !(MS_SILENT as libc::c_int),
  MS_BIND as libc::c_int | 1i32 << 14i32,
  1i32 << 8i32,
  MS_BIND as libc::c_int,
  MS_MOVE as libc::c_int,
  MS_SHARED as libc::c_int,
  MS_SLAVE as libc::c_int,
  MS_PRIVATE as libc::c_int,
  MS_UNBINDABLE as libc::c_int,
  MS_SHARED as libc::c_int | 1i32 << 14i32,
  MS_SLAVE as libc::c_int | 1i32 << 14i32,
  MS_PRIVATE as libc::c_int | 1i32 << 14i32,
  MS_UNBINDABLE as libc::c_int | 1i32 << 14i32,
  MS_RDONLY as libc::c_int,
  !(MS_RDONLY as libc::c_int),
  MS_REMOUNT as libc::c_int,
];
static mut mount_option_str: [libc::c_char; 345] = [
  108, 111, 111, 112, 0, 100, 101, 102, 97, 117, 108, 116, 115, 0, 110, 111, 97, 117, 116, 111, 0,
  115, 119, 0, 115, 119, 97, 112, 0, 117, 115, 101, 114, 0, 117, 115, 101, 114, 115, 0, 110, 111,
  102, 97, 105, 108, 0, 95, 110, 101, 116, 100, 101, 118, 0, 99, 111, 109, 109, 101, 110, 116, 61,
  0, 110, 111, 115, 117, 105, 100, 0, 115, 117, 105, 100, 0, 100, 101, 118, 0, 110, 111, 100, 101,
  118, 0, 101, 120, 101, 99, 0, 110, 111, 101, 120, 101, 99, 0, 115, 121, 110, 99, 0, 100, 105,
  114, 115, 121, 110, 99, 0, 97, 115, 121, 110, 99, 0, 97, 116, 105, 109, 101, 0, 110, 111, 97,
  116, 105, 109, 101, 0, 100, 105, 114, 97, 116, 105, 109, 101, 0, 110, 111, 100, 105, 114, 97,
  116, 105, 109, 101, 0, 109, 97, 110, 100, 0, 110, 111, 109, 97, 110, 100, 0, 114, 101, 108, 97,
  116, 105, 109, 101, 0, 110, 111, 114, 101, 108, 97, 116, 105, 109, 101, 0, 115, 116, 114, 105,
  99, 116, 97, 116, 105, 109, 101, 0, 108, 111, 117, 100, 0, 114, 98, 105, 110, 100, 0, 117, 110,
  105, 111, 110, 0, 98, 105, 110, 100, 0, 109, 111, 118, 101, 0, 109, 97, 107, 101, 45, 115, 104,
  97, 114, 101, 100, 0, 109, 97, 107, 101, 45, 115, 108, 97, 118, 101, 0, 109, 97, 107, 101, 45,
  112, 114, 105, 118, 97, 116, 101, 0, 109, 97, 107, 101, 45, 117, 110, 98, 105, 110, 100, 97, 98,
  108, 101, 0, 109, 97, 107, 101, 45, 114, 115, 104, 97, 114, 101, 100, 0, 109, 97, 107, 101, 45,
  114, 115, 108, 97, 118, 101, 0, 109, 97, 107, 101, 45, 114, 112, 114, 105, 118, 97, 116, 101, 0,
  109, 97, 107, 101, 45, 114, 117, 110, 98, 105, 110, 100, 97, 98, 108, 101, 0, 114, 111, 0, 114,
  119, 0, 114, 101, 109, 111, 117, 110, 116, 0, 0,
];
unsafe extern "C" fn verbose_mount(
  mut source: *const libc::c_char,
  mut target: *const libc::c_char,
  mut filesystemtype: *const libc::c_char,
  mut mountflags: libc::c_ulong,
  mut data: *const libc::c_void,
) -> libc::c_int {
  let mut rc: libc::c_int = 0;
  *bb_errno = 0i32;
  rc = mount(source, target, filesystemtype, mountflags, data);
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose >= 2i32 as libc::c_uint {
    bb_perror_msg(
      b"mount(\'%s\',\'%s\',\'%s\',0x%08lx,\'%s\'):%d\x00" as *const u8 as *const libc::c_char,
      source,
      target,
      filesystemtype,
      mountflags,
      data as *mut libc::c_char,
      rc,
    );
  }
  return rc;
}
// Append mount options to string
unsafe extern "C" fn append_mount_options(
  mut oldopts: *mut *mut libc::c_char,
  mut newopts: *const libc::c_char,
) {
  if !(*oldopts).is_null() && **oldopts as libc::c_int != 0 {
    // Do not insert options which are already there
    while *newopts.offset(0) != 0 {
      let mut current_block_11: u64;
      let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
      let mut len: libc::c_int = strlen(newopts) as libc::c_int;
      p = strchr(newopts, ',' as i32);
      if !p.is_null() {
        len = p.wrapping_offset_from(newopts) as libc::c_long as libc::c_int
      }
      p = *oldopts;
      loop {
        if strncmp(p, newopts, len as libc::c_ulong) == 0
          && (*p.offset(len as isize) as libc::c_int == ',' as i32
            || *p.offset(len as isize) as libc::c_int == '\u{0}' as i32)
        {
          current_block_11 = 8801671410852752138;
          break;
        }
        p = strchr(p, ',' as i32);
        if p.is_null() {
          current_block_11 = 5399440093318478209;
          break;
        }
        p = p.offset(1)
      }
      match current_block_11 {
        5399440093318478209 => {
          p = xasprintf(
            b"%s,%.*s\x00" as *const u8 as *const libc::c_char,
            *oldopts,
            len,
            newopts,
          );
          free(*oldopts as *mut libc::c_void);
          *oldopts = p
        }
        _ => {}
      }
      newopts = newopts.offset(len as isize);
      while *newopts.offset(0) as libc::c_int == ',' as i32 {
        newopts = newopts.offset(1)
      }
    }
  } else {
    *oldopts = xstrdup(newopts)
  };
}
// Use the mount_options list to parse options into flags.
// Also update list of unrecognized options if unrecognized != NULL
unsafe extern "C" fn parse_mount_options(
  mut options: *mut libc::c_char,
  mut unrecognized: *mut *mut libc::c_char,
) -> libc::c_ulong {
  let mut flags: libc::c_ulong = MS_SILENT as libc::c_int as libc::c_ulong;
  let mut current_block_14: u64;
  loop
  // Loop through options
  {
    let mut i: libc::c_uint = 0;
    let mut comma: *mut libc::c_char = strchr(options, ',' as i32);
    let mut option_str: *const libc::c_char = mount_option_str.as_ptr();
    if !comma.is_null() {
      *comma = '\u{0}' as i32 as libc::c_char
    }
    // FIXME: use hasmntopt()
    // Find this option in mount_options
    i = 0i32 as libc::c_uint;
    loop {
      if !(i
        < (::std::mem::size_of::<[int32_t; 44]>() as libc::c_ulong)
          .wrapping_div(::std::mem::size_of::<int32_t>() as libc::c_ulong)
          as libc::c_uint)
      {
        current_block_14 = 10048703153582371463;
        break;
      }
      let mut opt_len: libc::c_uint = strlen(option_str) as libc::c_uint;
      if strncasecmp(option_str, options, opt_len as libc::c_ulong) == 0i32
        && (*options.offset(opt_len as isize) as libc::c_int == '\u{0}' as i32
          || *option_str.offset(opt_len.wrapping_sub(1i32 as libc::c_uint) as isize) as libc::c_int
            == '=' as i32)
      {
        let mut fl: libc::c_ulong = mount_options[i as usize] as libc::c_ulong;
        if fl & (1u32 << 31i32) as libc::c_ulong != 0 {
          flags &= fl
        } else {
          flags |= fl
        }
        current_block_14 = 10695954142028877493;
        break;
      } else {
        option_str = option_str.offset(opt_len.wrapping_add(1i32 as libc::c_uint) as isize);
        i = i.wrapping_add(1)
      }
    }
    match current_block_14 {
      10048703153582371463 => {
        // We did not recognize this option.
        // If "unrecognized" is not NULL, append option there.
        // Note that we should not append *empty* option -
        // in this case we want to pass NULL, not "", to "data"
        // parameter of mount(2) syscall.
        // This is crucial for filesystems that don't accept
        // any arbitrary mount options, like cgroup fs:
        // "mount -t cgroup none /mnt"
        if *options.offset(0) as libc::c_int != 0 && !unrecognized.is_null() {
          // Add it to strflags, to pass on to kernel
          let mut p: *mut libc::c_char = *unrecognized;
          let mut len: libc::c_uint = if !p.is_null() {
            strlen(p)
          } else {
            0i32 as libc::c_ulong
          } as libc::c_uint;
          p = xrealloc(
            p as *mut libc::c_void,
            (len as libc::c_ulong)
              .wrapping_add(strlen(options))
              .wrapping_add(2i32 as libc::c_ulong),
          ) as *mut libc::c_char;
          *unrecognized = p;
          // Comma separated if it's not the first one
          if len != 0 {
            let fresh0 = len;
            len = len.wrapping_add(1);
            *p.offset(fresh0 as isize) = ',' as i32 as libc::c_char
          }
          strcpy(p.offset(len as isize), options);
        }
      }
      _ => {}
    }
    if comma.is_null() {
      break;
    }
    // Advance to next option
    *comma = ',' as i32 as libc::c_char;
    comma = comma.offset(1);
    options = comma
  }
  return flags;
}
// Return a list of all block device backed filesystems
unsafe extern "C" fn get_block_backed_filesystems() -> *mut llist_t {
  static mut filesystems: [[libc::c_char; 18]; 2] = [
    [
      47, 101, 116, 99, 47, 102, 105, 108, 101, 115, 121, 115, 116, 101, 109, 115, 0, 0,
    ],
    [
      47, 112, 114, 111, 99, 47, 102, 105, 108, 101, 115, 121, 115, 116, 101, 109, 115, 0,
    ],
  ];
  let mut fs: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut list: *mut llist_t = 0 as *mut llist_t;
  let mut i: libc::c_int = 0;
  let mut f: *mut FILE = 0 as *mut FILE;
  i = 0i32;
  while i < 2i32 {
    f = fopen_for_read(filesystems[i as usize].as_ptr());
    if !f.is_null() {
      loop {
        buf = xmalloc_fgetline(f);
        if buf.is_null() {
          break;
        }
        if !(!is_prefixed_with(buf, b"nodev\x00" as *const u8 as *const libc::c_char).is_null()
          && ({
            let mut bb__isspace: libc::c_uchar =
              (*buf.offset(5) as libc::c_int - 9i32) as libc::c_uchar;
            (bb__isspace as libc::c_int == ' ' as i32 - 9i32
              || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
          }) != 0)
        {
          fs = skip_whitespace(buf);
          if !(*fs as libc::c_int == '#' as i32 || *fs as libc::c_int == '*' as i32 || *fs == 0) {
            llist_add_to_end(&mut list, xstrdup(fs) as *mut libc::c_void);
          }
        }
        free(buf as *mut libc::c_void);
      }
    }
    i += 1
  }
  return list;
}
// Perform actual mount of specific filesystem at specific location.
// NB: mp->xxx fields may be trashed on exit
unsafe extern "C" fn mount_it_now(
  mut mp: *mut mntent,
  mut vfsflags: libc::c_ulong,
  mut filteropts: *mut libc::c_char,
) -> libc::c_int {
  let mut rc: libc::c_int = 0i32;
  vfsflags &= !(MOUNT_FAKEFLAGS as libc::c_int as libc::c_ulong);
  if option_mask32 & OPT_f as libc::c_int as libc::c_uint != 0 {
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose >= 2i32 as libc::c_uint {
      bb_error_msg(
        b"would do mount(\'%s\',\'%s\',\'%s\',0x%08lx,\'%s\')\x00" as *const u8
          as *const libc::c_char,
        (*mp).mnt_fsname,
        (*mp).mnt_dir,
        (*mp).mnt_type,
        vfsflags,
        filteropts,
      );
    }
  } else {
    loop
    // Mount, with fallback to read-only if necessary.
    {
      *bb_errno = 0i32;
      rc = verbose_mount(
        (*mp).mnt_fsname,
        (*mp).mnt_dir,
        (*mp).mnt_type,
        vfsflags,
        filteropts as *const libc::c_void,
      );
      // If mount failed, try
      // helper program mount.<mnt_type>
      if 0i32 != 0 && rc != 0 && !(*mp).mnt_type.is_null() {
        let mut args: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
        let mut errno_save: libc::c_int = *bb_errno;
        args[0] = xasprintf(
          b"mount.%s\x00" as *const u8 as *const libc::c_char,
          (*mp).mnt_type,
        );
        rc = 1i32;
        if option_mask32 & OPT_f as libc::c_int as libc::c_uint != 0 {
          let fresh1 = rc;
          rc = rc + 1;
          args[fresh1 as usize] = b"-f\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        if 0i32 != 0 && 0i32 == 0 {
          let fresh2 = rc;
          rc = rc + 1;
          args[fresh2 as usize] = b"-n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        let fresh3 = rc;
        rc = rc + 1;
        args[fresh3 as usize] = (*mp).mnt_fsname;
        let fresh4 = rc;
        rc = rc + 1;
        args[fresh4 as usize] = (*mp).mnt_dir;
        if !filteropts.is_null() {
          let fresh5 = rc;
          rc = rc + 1;
          args[fresh5 as usize] =
            b"-o\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
          let fresh6 = rc;
          rc = rc + 1;
          args[fresh6 as usize] = filteropts
        }
        args[rc as usize] = 0 as *mut libc::c_char;
        rc = spawn_and_wait(args.as_mut_ptr());
        free(args[0] as *mut libc::c_void);
        if rc == 0 {
          break;
        }
        *bb_errno = errno_save
      }
      if rc == 0
        || vfsflags & MS_RDONLY as libc::c_int as libc::c_ulong != 0
        || *bb_errno != 13i32 && *bb_errno != 30i32
      {
        break;
      }
      if vfsflags & MS_SILENT as libc::c_int as libc::c_ulong == 0 {
        bb_error_msg(
          b"%s is write-protected, mounting read-only\x00" as *const u8 as *const libc::c_char,
          (*mp).mnt_fsname,
        );
      }
      vfsflags |= MS_RDONLY as libc::c_int as libc::c_ulong
    }
    // Abort entirely if permission denied.
    if rc != 0 && *bb_errno == 1i32 {
      bb_simple_error_msg_and_die(bb_msg_perm_denied_are_you_root.as_ptr());
    }
  }
  // If the mount was successful, and we're maintaining an old-style
  // mtab file by hand, add the new entry to it now.
  if 0i32 != 0 && rc == 0 && vfsflags & MS_REMOUNT as libc::c_int as libc::c_ulong == 0 {
    let mut fsname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mountTable: *mut FILE = setmntent(
      b"/proc/mounts\x00" as *const u8 as *const libc::c_char,
      b"a+\x00" as *const u8 as *const libc::c_char,
    );
    let mut option_str: *const libc::c_char = mount_option_str.as_ptr();
    let mut i: libc::c_int = 0;
    if mountTable.is_null() {
      bb_simple_perror_msg(b"/proc/mounts\x00" as *const u8 as *const libc::c_char);
    } else {
      // Add vfs string flags
      i = 0i32;
      while mount_options[i as usize] != MS_REMOUNT as libc::c_int {
        if mount_options[i as usize] > 0i32
          && mount_options[i as usize] as libc::c_ulong & vfsflags != 0
        {
          append_mount_options(&mut (*mp).mnt_opts, option_str);
        }
        option_str =
          option_str.offset(strlen(option_str).wrapping_add(1i32 as libc::c_ulong) as isize);
        i += 1
      }
      // Remove trailing / (if any) from directory we mounted on
      i = strlen((*mp).mnt_dir).wrapping_sub(1i32 as libc::c_ulong) as libc::c_int;
      while i > 0i32 && *(*mp).mnt_dir.offset(i as isize) as libc::c_int == '/' as i32 {
        let fresh7 = i;
        i = i - 1;
        *(*mp).mnt_dir.offset(fresh7 as isize) = '\u{0}' as i32 as libc::c_char
      }
      // Convert to canonical pathnames as needed
      (*mp).mnt_dir = bb_simplify_path((*mp).mnt_dir);
      fsname = 0 as *mut libc::c_char;
      if (*mp).mnt_type.is_null() || *(*mp).mnt_type == 0 {
        // bind mount
        fsname = bb_simplify_path((*mp).mnt_fsname);
        (*mp).mnt_fsname = fsname;
        (*mp).mnt_type = b"bind\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      }
      (*mp).mnt_passno = 0i32;
      (*mp).mnt_freq = (*mp).mnt_passno;
      // Write and close
      addmntent(mountTable, mp);
      endmntent(mountTable);
    }
  }
  return rc;
}
// !ENABLE_FEATURE_MOUNT_NFS
/* Linux 2.6.23+ supports nfs mounts with options passed as a string.
 * For older kernels, you must build busybox with ENABLE_FEATURE_MOUNT_NFS.
 * (However, note that then you lose any chances that NFS over IPv6 would work).
 */
unsafe extern "C" fn nfsmount(
  mut mp: *mut mntent,
  mut vfsflags: libc::c_ulong,
  mut filteropts: *mut libc::c_char,
) -> libc::c_int {
  let mut lsa: *mut len_and_sockaddr = 0 as *mut len_and_sockaddr;
  let mut opts: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut dotted: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut ret: libc::c_int = 0;
  end = strchr((*mp).mnt_fsname, ']' as i32);
  if !end.is_null() && *end.offset(1) as libc::c_int == ':' as i32 {
    end = end.offset(1)
  } else {
    /* mount_main() guarantees that ':' is there */
    end = strchr((*mp).mnt_fsname, ':' as i32)
  }
  *end = '\u{0}' as i32 as libc::c_char;
  lsa = xhost2sockaddr((*mp).mnt_fsname, 0i32);
  *end = ':' as i32 as libc::c_char;
  dotted = xmalloc_sockaddr2dotted_noport(&mut (*lsa).u.sa);
  opts = xasprintf(
    b"%s%saddr=%s\x00" as *const u8 as *const libc::c_char,
    if !filteropts.is_null() {
      filteropts
    } else {
      b"\x00" as *const u8 as *const libc::c_char
    },
    if !filteropts.is_null() {
      b",\x00" as *const u8 as *const libc::c_char
    } else {
      b"\x00" as *const u8 as *const libc::c_char
    },
    dotted,
  );
  ret = mount_it_now(mp, vfsflags, opts);
  return ret;
}
// !ENABLE_FEATURE_MOUNT_NFS
// Mount one directory.  Handles CIFS, NFS, loopback, autobind, and filesystem
// type detection.  Returns 0 for success, nonzero for failure.
// NB: mp->xxx fields may be trashed on exit
unsafe extern "C" fn singlemount(mut mp: *mut mntent, mut ignore_busy: libc::c_int) -> libc::c_int {
  let mut loopfd: libc::c_int = -1i32;
  let mut rc: libc::c_int = -1i32;
  let mut vfsflags: libc::c_ulong = 0;
  let mut loopFile: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut filteropts: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut fl: *mut llist_t = 0 as *mut llist_t;
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
  };
  *bb_errno = 0i32;
  vfsflags = parse_mount_options((*mp).mnt_opts, &mut filteropts);
  // Treat fstype "auto" as unspecified
  if !(*mp).mnt_type.is_null()
    && strcmp(
      (*mp).mnt_type,
      b"auto\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
  {
    (*mp).mnt_type = 0 as *mut libc::c_char
  }
  // Might this be a virtual filesystem?
  if 0i32 != 0 && !strchr((*mp).mnt_fsname, '#' as i32).is_null() {
    let mut args: [*mut libc::c_char; 35] = [0 as *mut libc::c_char; 35];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    // fsname: "cmd#arg1#arg2..."
    // WARNING: allows execution of arbitrary commands!
    // Try "mount 'sh#-c#sh' bogus_dir".
    // It is safe ONLY because non-root
    // cannot use two-argument mount command
    // and using one-argument "mount 'sh#-c#sh'" doesn't work:
    // "mount: can't find sh#-c#sh in /etc/fstab"
    // (if /etc/fstab has it, it's ok: root sets up /etc/fstab).
    s = (*mp).mnt_fsname;
    n = 0i32;
    let fresh8 = n;
    n = n + 1;
    args[fresh8 as usize] = s;
    while *s as libc::c_int != 0 && n < 35i32 - 2i32 {
      let fresh9 = s;
      s = s.offset(1);
      if *fresh9 as libc::c_int == '#' as i32 && *s as libc::c_int != '#' as i32 {
        *s.offset(-1i32 as isize) = '\u{0}' as i32 as libc::c_char;
        let fresh10 = n;
        n = n + 1;
        args[fresh10 as usize] = s
      }
    }
    let fresh11 = n;
    n = n + 1;
    args[fresh11 as usize] = (*mp).mnt_dir;
    args[n as usize] = 0 as *mut libc::c_char;
    rc = spawn_and_wait(args.as_mut_ptr())
  } else if 1i32 != 0
    && ((*mp).mnt_type.is_null()
      || strcmp(
        (*mp).mnt_type,
        b"cifs\x00" as *const u8 as *const libc::c_char,
      ) == 0i32)
    && (*(*mp).mnt_fsname.offset(0) as libc::c_int == '/' as i32
      || *(*mp).mnt_fsname.offset(0) as libc::c_int == '\\' as i32)
    && *(*mp).mnt_fsname.offset(0) as libc::c_int == *(*mp).mnt_fsname.offset(1) as libc::c_int
  {
    let mut len: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    let mut hostname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut share: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lsa: *mut len_and_sockaddr = 0 as *mut len_and_sockaddr;
    // Might this be an CIFS filesystem?
    // Parse mp->mnt_fsname of the form "//hostname/share[/dir1/dir2]"
    hostname = (*mp).mnt_fsname.offset(2);
    len = strcspn(hostname, b"/\\\x00" as *const u8 as *const libc::c_char) as libc::c_int;
    share = hostname.offset(len as isize).offset(1);
    if !(len == 0i32
      || *share.offset(-1i32 as isize) as libc::c_int == '\u{0}' as i32
      || *share.offset(0) as libc::c_int == '\u{0}' as i32)
    {
      c = *share.offset(-1i32 as isize);
      *share.offset(-1i32 as isize) = '\u{0}' as i32 as libc::c_char;
      len = strcspn(share, b"/\\\x00" as *const u8 as *const libc::c_char) as libc::c_int;
      // "unc=\\hostname\share" option is mandatory
      // after CIFS option parsing was rewritten in Linux 3.4.
      // Must use backslashes.
      // If /dir1/dir2 is present, also add "prefixpath=dir1/dir2"
      let mut unc: *mut libc::c_char = xasprintf(
        if *share.offset(len as isize) as libc::c_int != '\u{0}' as i32 {
          b"unc=\\\\%s\\%.*s,prefixpath=%s\x00" as *const u8 as *const libc::c_char
        } else {
          b"unc=\\\\%s\\%.*s\x00" as *const u8 as *const libc::c_char
        },
        hostname,
        len,
        share,
        share.offset(len as isize).offset(1),
      );
      parse_mount_options(unc, &mut filteropts);
      lsa = host2sockaddr(hostname, 0i32);
      *share.offset(-1i32 as isize) = c;
      if !lsa.is_null() {
        // If there is no "ip=..." option yet
        if is_prefixed_with(
          filteropts,
          (b",ip=\x00" as *const u8 as *const libc::c_char).offset(1),
        )
        .is_null()
          && strstr(filteropts, b",ip=\x00" as *const u8 as *const libc::c_char).is_null()
        {
          let mut dotted: *mut libc::c_char = 0 as *mut libc::c_char;
          let mut ip: *mut libc::c_char = 0 as *mut libc::c_char;
          // Insert "ip=..." option into options
          dotted = xmalloc_sockaddr2dotted_noport(&mut (*lsa).u.sa);
          ip = xasprintf(b"ip=%s\x00" as *const u8 as *const libc::c_char, dotted);
          // Note: IPv6 scoped addresses ("host%iface", see RFC 4007) should be
          // handled by libc in getnameinfo() (inside xmalloc_sockaddr2dotted_noport()).
          // Currently, glibc does not support that (has no NI_NUMERICSCOPE),
          // musl apparently does. This results in "ip=numericIPv6%iface_name"
          // (instead of _numeric_ iface_id) with glibc.
          // This probably should be fixed in glibc, not here.
          // The workaround is to manually specify correct "ip=ADDR%n" option.
          parse_mount_options(ip, &mut filteropts);
        }
        (*mp).mnt_type = b"cifs\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        rc = mount_it_now(mp, vfsflags, filteropts)
      }
    }
  } else if ((*mp).mnt_type.is_null()
    || !is_prefixed_with(
      (*mp).mnt_type,
      b"nfs\x00" as *const u8 as *const libc::c_char,
    )
    .is_null())
    && !strchr((*mp).mnt_fsname, ':' as i32).is_null()
  {
    if (*mp).mnt_type.is_null() {
      (*mp).mnt_type = b"nfs\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    rc = nfsmount(mp, vfsflags, filteropts)
  } else {
    // Might this be an NFS filesystem?
    // Look at the file.  (Not found isn't a failure for remount, or for
    // a synthetic filesystem like proc or sysfs.)
    // (We use stat, not lstat, in order to allow
    // mount symlink_to_file_or_blkdev dir)
    if stat((*mp).mnt_fsname, &mut st) == 0
      && vfsflags
        & (MS_REMOUNT as libc::c_int | MS_BIND as libc::c_int | MS_MOVE as libc::c_int)
          as libc::c_ulong
        == 0
    {
      // Do we need to allocate a loopback device for it?
      if 1i32 != 0 && st.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint {
        loopFile = bb_simplify_path((*mp).mnt_fsname);
        // Autodetect bind mounts
        (*mp).mnt_fsname = 0 as *mut libc::c_char; // will receive malloced loop dev name
        loopfd = set_loop(
          &mut (*mp).mnt_fsname,
          loopFile,
          0i32 as libc::c_ulonglong,
          ((if vfsflags & MS_RDONLY as libc::c_int as libc::c_ulong != 0 {
            1i32
          } else {
            0i32
          }) | 4i32) as libc::c_uint,
        );
        if loopfd < 0i32 {
          if *bb_errno == 1i32 || *bb_errno == 13i32 {
            bb_simple_error_msg(bb_msg_perm_denied_are_you_root.as_ptr());
          } else {
            bb_simple_perror_msg(
              b"can\'t setup loop device\x00" as *const u8 as *const libc::c_char,
            );
          }
          return *bb_errno;
        }
      } else if st.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint
        && (*mp).mnt_type.is_null()
      {
        vfsflags |= MS_BIND as libc::c_int as libc::c_ulong
      }
    }
    // mount always creates AUTOCLEARed loopdevs, so that umounting
    // drops them without any code in the userspace.
    // This happens since circa linux-2.6.25:
    // commit 96c5865559cee0f9cbc5173f3c949f6ce3525581
    // Date:    Wed Feb 6 01:36:27 2008 -0800
    // Subject: Allow auto-destruction of loop devices
    // If we know the fstype (or don't need to), jump straight
    // to the actual mount.
    if !(*mp).mnt_type.is_null()
      || vfsflags
        & (MS_REMOUNT as libc::c_int | MS_BIND as libc::c_int | MS_MOVE as libc::c_int)
          as libc::c_ulong
        != 0
    {
      let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
      loop {
        next = if !(*mp).mnt_type.is_null() {
          strchr((*mp).mnt_type, ',' as i32)
        } else {
          0 as *mut libc::c_char
        };
        if !next.is_null() {
          *next = '\u{0}' as i32 as libc::c_char
        }
        rc = mount_it_now(mp, vfsflags, filteropts);
        if rc == 0i32 || next.is_null() {
          break;
        }
        (*mp).mnt_type = next.offset(1)
      }
    } else {
      // Loop through filesystem types until mount succeeds
      // or we run out
      // Initialize list of block backed filesystems.
      // This has to be done here so that during "mount -a",
      // mounts after /proc shows up can autodetect.
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .fslist
        .is_null()
      {
        let ref mut fresh12 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).fslist;
        *fresh12 = get_block_backed_filesystems();

        // This was originally guarded by
        //    if (ENABLE_FEATURE_CLEAN_UP && fslist)
        // in the C code. Not sure what ENABLE_FEATURE_CLEAN_UP is, but it's not
        // in the default config.
        // if 0i32 != 0
        //   && !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        //     .fslist
        //     .is_null()
        // {
        //   atexit(Some(
        //     delete_block_backed_filesystems as unsafe extern "C" fn() -> (),
        //   ));
        // }
      }
      fl = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).fslist;
      while !fl.is_null() {
        (*mp).mnt_type = (*fl).data;
        rc = mount_it_now(mp, vfsflags, filteropts);
        if rc == 0i32 {
          break;
        }
        fl = (*fl).link
      }
    }
    // If mount failed, clean up loop file (if any).
    // (Newer kernels which support LO_FLAGS_AUTOCLEAR should not need this,
    // merely "close(loopfd)" should do it?)
    if 1i32 != 0 && rc != 0 && !loopFile.is_null() {
      del_loop((*mp).mnt_fsname);
    }
  }
  // empty share name
  if loopfd >= 0i32 {
    close(loopfd);
  }
  if *bb_errno == 16i32 && ignore_busy != 0 {
    return 0i32;
  }
  if *bb_errno == 2i32 && vfsflags & MOUNT_NOFAIL as libc::c_int as libc::c_ulong != 0 {
    return 0i32;
  }
  if rc != 0i32 {
    bb_perror_msg(
      b"mounting %s on %s failed\x00" as *const u8 as *const libc::c_char,
      (*mp).mnt_fsname,
      (*mp).mnt_dir,
    );
  }
  return rc;
}
// -O support
//    -O interprets a list of filter options which select whether a mount
// point will be mounted: only mounts with options matching *all* filtering
// options will be selected.
//    By default each -O filter option must be present in the list of mount
// options, but if it is prefixed by "no" then it must be absent.
// For example,
//  -O a,nob,c  matches  -o a,c  but fails to match  -o a,b,c
//              (and also fails to match  -o a  because  -o c  is absent).
//
// It is different from -t in that each option is matched exactly; a leading
// "no" at the beginning of one option does not negate the rest.
unsafe extern "C" fn match_opt(
  mut fs_opt_in: *const libc::c_char,
  mut O_opt: *const libc::c_char,
) -> libc::c_int {
  if O_opt.is_null() {
    return 1i32;
  }
  while *O_opt != 0 {
    let mut fs_opt: *const libc::c_char = fs_opt_in;
    let mut O_len: libc::c_int = 0;
    let mut match_0: libc::c_int = 0;
    // If option begins with "no" then treat as an inverted match:
    // matching is a failure
    match_0 = 0i32;
    if *O_opt.offset(0) as libc::c_int == 'n' as i32
      && *O_opt.offset(1) as libc::c_int == 'o' as i32
    {
      match_0 = 1i32;
      O_opt = O_opt.offset(2)
    }
    // Isolate the current O option
    O_len = strchrnul(O_opt, ',' as i32).wrapping_offset_from(O_opt) as libc::c_long as libc::c_int;
    loop
    // Check for a match against existing options
    {
      if strncmp(fs_opt, O_opt, O_len as libc::c_ulong) == 0i32
        && (*fs_opt.offset(O_len as isize) as libc::c_int == '\u{0}' as i32
          || *fs_opt.offset(O_len as isize) as libc::c_int == ',' as i32)
      {
        if match_0 != 0 {
          return 0i32;
        } // "no" prefix, but option found
        match_0 = 1i32; // current O option found, go check next one
        break; // match wanted but not found
      } else {
        fs_opt = strchr(fs_opt, ',' as i32);
        if fs_opt.is_null() {
          break;
        }
        fs_opt = fs_opt.offset(1)
      }
    }
    if match_0 == 0i32 {
      return 0i32;
    }
    if *O_opt.offset(O_len as isize) as libc::c_int == '\u{0}' as i32 {
      break;
    }
    // Step to the next O option
    O_opt = O_opt.offset((O_len + 1i32) as isize)
  }
  // If we get here then everything matched
  return 1i32;
}
// Parse options, if necessary parse fstab/mtab, and call singlemount for
// each directory to be mounted.
#[no_mangle]
pub unsafe extern "C" fn mount_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut cmdopts: *mut libc::c_char = xzalloc(1i32 as size_t) as *mut libc::c_char;
  let mut fstype: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut O_optmatch: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut storage_path: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut lst_o: *mut llist_t = 0 as *mut llist_t;
  let mut fstabname: *const libc::c_char = b"/etc/fstab\x00" as *const u8 as *const libc::c_char;
  let mut fstab: *mut FILE = 0 as *mut FILE;
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  let mut rc: libc::c_int = 0i32;
  let mut cmdopt_flags: libc::c_ulong = 0;
  let mut opt: libc::c_uint = 0;
  let mut mtpair: [mntent; 2] = [mntent {
    mnt_fsname: 0 as *mut libc::c_char,
    mnt_dir: 0 as *mut libc::c_char,
    mnt_type: 0 as *mut libc::c_char,
    mnt_opts: 0 as *mut libc::c_char,
    mnt_freq: 0,
    mnt_passno: 0,
  }; 2];
  let mut mtcur: *mut mntent = mtpair.as_mut_ptr();
  let mut nonroot: libc::c_int = sanitize_env_if_suid();
  // Parse long options, like --bind and --move.  Note that -o option
  // and --option are synonymous.  Yes, this means --remount,rw works.
  j = 1i32;
  i = j;
  while !(*argv.offset(i as isize)).is_null() {
    if *(*argv.offset(i as isize)).offset(0) as libc::c_int == '-' as i32
      && *(*argv.offset(i as isize)).offset(1) as libc::c_int == '-' as i32
    {
      append_mount_options(&mut cmdopts, (*argv.offset(i as isize)).offset(2));
    } else {
      let fresh13 = j;
      j = j + 1;
      let ref mut fresh14 = *argv.offset(fresh13 as isize);
      *fresh14 = *argv.offset(i as isize)
    }
    i += 1
  }
  let ref mut fresh15 = *argv.offset(j as isize);
  *fresh15 = 0 as *mut libc::c_char;
  // Parse remaining options
  // Max 2 params; -o is a list, -v is a counter
  opt = getopt32(
    argv,
    b"^o:*t:rwanfvsiO:T:\x00?2vv\x00" as *const u8 as *const libc::c_char,
    &mut lst_o as *mut *mut llist_t,
    &mut fstype as *mut *mut libc::c_char,
    &mut O_optmatch as *mut *mut libc::c_char,
    &mut fstabname as *mut *const libc::c_char,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose as *mut libc::c_uint,
  ); // -o
  while !lst_o.is_null() {
    append_mount_options(&mut cmdopts, llist_pop(&mut lst_o) as *const libc::c_char);
    // -r
  } // -w
  if opt & OPT_r as libc::c_int as libc::c_uint != 0 {
    append_mount_options(&mut cmdopts, b"ro\x00" as *const u8 as *const libc::c_char);
  }
  if opt & OPT_w as libc::c_int as libc::c_uint != 0 {
    append_mount_options(&mut cmdopts, b"rw\x00" as *const u8 as *const libc::c_char);
  }
  argv = argv.offset(optind as isize);
  // If we have no arguments, show currently mounted filesystems
  if (*argv.offset(0)).is_null() {
    if opt & OPT_a as libc::c_int as libc::c_uint == 0 {
      let mut mountTable: *mut FILE = setmntent(
        b"/proc/mounts\x00" as *const u8 as *const libc::c_char,
        b"r\x00" as *const u8 as *const libc::c_char,
      );
      if mountTable.is_null() {
        bb_error_msg_and_die(
          b"no %s\x00" as *const u8 as *const libc::c_char,
          b"/proc/mounts\x00" as *const u8 as *const libc::c_char,
        );
      }
      while !getmntent_r(
        mountTable,
        &mut *mtpair.as_mut_ptr().offset(0),
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .getmntent_buf
          .as_mut_ptr(),
        GETMNTENT_BUFSIZE as libc::c_int,
      )
      .is_null()
      {
        // Don't show rootfs. FIXME: why??
        // util-linux 2.12a happily shows rootfs...
        //if (strcmp(mtpair->mnt_fsname, "rootfs") == 0) continue;
        if fstype.is_null() || strcmp((*mtpair.as_mut_ptr()).mnt_type, fstype) == 0i32 {
          printf(
            b"%s on %s type %s (%s)\n\x00" as *const u8 as *const libc::c_char,
            (*mtpair.as_mut_ptr()).mnt_fsname,
            (*mtpair.as_mut_ptr()).mnt_dir,
            (*mtpair.as_mut_ptr()).mnt_type,
            (*mtpair.as_mut_ptr()).mnt_opts,
          );
        }
      }
      return 0i32;
    }
    storage_path = 0 as *mut libc::c_char
  } else {
    // When we have two arguments, the second is the directory and we can
    // skip looking at fstab entirely.  We can always abspath() the directory
    // argument when we get it.
    if !(*argv.offset(1)).is_null() {
      if nonroot != 0 {
        bb_simple_error_msg_and_die(bb_msg_you_must_be_root.as_ptr());
      }
      let ref mut fresh16 = (*mtpair.as_mut_ptr()).mnt_fsname;
      *fresh16 = *argv.offset(0);
      let ref mut fresh17 = (*mtpair.as_mut_ptr()).mnt_dir;
      *fresh17 = *argv.offset(1);
      let ref mut fresh18 = (*mtpair.as_mut_ptr()).mnt_type;
      *fresh18 = fstype;
      let ref mut fresh19 = (*mtpair.as_mut_ptr()).mnt_opts;
      *fresh19 = cmdopts;
      resolve_mount_spec(&mut (*mtpair.as_mut_ptr()).mnt_fsname);
      rc = singlemount(mtpair.as_mut_ptr(), 0i32);
      return rc;
    }
    storage_path = bb_simplify_path(*argv.offset(0))
    // malloced
  }
  // Past this point, we are handling either "mount -a [opts]"
  // or "mount [opts] single_param"
  cmdopt_flags = parse_mount_options(cmdopts, 0 as *mut *mut libc::c_char);
  if nonroot != 0 && cmdopt_flags & !(MS_SILENT as libc::c_int) as libc::c_ulong != 0 {
    // Non-root users cannot specify flags
    bb_simple_error_msg_and_die(bb_msg_you_must_be_root.as_ptr());
  }
  // If we have a shared subtree flag, don't worry about fstab or mtab.
  if 1i32 != 0
    && cmdopt_flags
      & (MS_SHARED as libc::c_int
        | MS_PRIVATE as libc::c_int
        | MS_SLAVE as libc::c_int
        | MS_UNBINDABLE as libc::c_int) as libc::c_ulong
      != 0
  {
    // verbose_mount(source, target, type, flags, data)
    rc = verbose_mount(
      b"\x00" as *const u8 as *const libc::c_char,
      *argv.offset(0),
      b"\x00" as *const u8 as *const libc::c_char,
      cmdopt_flags,
      b"\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
    );
    if rc != 0 {
      bb_simple_perror_msg_and_die(*argv.offset(0));
    }
    return rc;
  }
  // A malicious user could overmount /usr without this.
  if 1i32 != 0 && nonroot != 0 {
    fstabname = b"/etc/fstab\x00" as *const u8 as *const libc::c_char
  }
  // Open either fstab or mtab
  if cmdopt_flags & MS_REMOUNT as libc::c_int as libc::c_ulong != 0 {
    // WARNING. I am not sure this matches util-linux's
    // behavior. It's possible util-linux does not
    // take -o opts from mtab (takes only mount source).
    fstabname = b"/proc/mounts\x00" as *const u8 as *const libc::c_char
  }
  fstab = setmntent(fstabname, b"r\x00" as *const u8 as *const libc::c_char);
  if fstab.is_null() {
    bb_perror_msg_and_die(
      b"can\'t read \'%s\'\x00" as *const u8 as *const libc::c_char,
      fstabname,
    );
  }
  // Loop through entries until we find what we're looking for
  memset(
    mtpair.as_mut_ptr() as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<[mntent; 2]>() as libc::c_ulong,
  );
  loop {
    let mut mtother: *mut mntent = if mtcur == mtpair.as_mut_ptr() {
      mtpair.as_mut_ptr().offset(1)
    } else {
      mtpair.as_mut_ptr()
    };
    // no
    if getmntent_r(
      fstab,
      mtcur,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .getmntent_buf
        .as_mut_ptr()
        .offset(
          (if mtcur == mtpair.as_mut_ptr() {
            (GETMNTENT_BUFSIZE as libc::c_int) / 2i32
          } else {
            0i32
          }) as isize,
        ),
      GETMNTENT_BUFSIZE as libc::c_int / 2i32,
    )
    .is_null()
    {
      // Get next fstab entry
      // End of fstab/mtab is reached
      mtcur = mtother; // the thing we found last time
      break;
    } else if !(*argv.offset(0)).is_null() {
      // If we're trying to mount something specific and this isn't it,
      // skip it.  Note we must match the exact text in fstab (ala
      // "proc") or a full path from root
      // Is this what we're looking for?
      if strcmp(*argv.offset(0), (*mtcur).mnt_fsname) != 0i32
        && strcmp(storage_path, (*mtcur).mnt_fsname) != 0i32
        && strcmp(*argv.offset(0), (*mtcur).mnt_dir) != 0i32
        && strcmp(storage_path, (*mtcur).mnt_dir) != 0i32
      {
        continue;
      }
      // Remember this entry.  Something later may have
      // overmounted it, and we want the _last_ match.
      mtcur = mtother
    // If we're mounting all
    } else {
      let mut mp: *mut mntent = 0 as *mut mntent;
      // No, mount -a won't mount anything,
      // even user mounts, for mere humans
      if nonroot != 0 {
        bb_simple_error_msg_and_die(bb_msg_you_must_be_root.as_ptr());
      }
      // Does type match? (NULL matches always)
      if fstype_matches((*mtcur).mnt_type, fstype) == 0 {
        continue;
      }
      // Skip noauto and swap anyway
      if parse_mount_options((*mtcur).mnt_opts, 0 as *mut *mut libc::c_char)
        & (MOUNT_NOAUTO as libc::c_int | MOUNT_SWAP as libc::c_int) as libc::c_ulong
        != 0
        || strcasecmp(
          (*mtcur).mnt_type,
          b"swap\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
      {
        continue;
      }
      // Does (at least one) option match?
      // (NULL matches always)
      if match_opt((*mtcur).mnt_opts, O_optmatch) == 0 {
        continue;
      }
      resolve_mount_spec(&mut (*mtcur).mnt_fsname);
      // NFS mounts want this to be xrealloc-able
      (*mtcur).mnt_opts = xstrdup((*mtcur).mnt_opts);
      // If nothing is mounted on this directory...
      // (otherwise repeated "mount -a" mounts everything again)
      mp = find_mount_point((*mtcur).mnt_dir, 0i32);
      // We do not check fsname match of found mount point -
      // "/" may have fsname of "/dev/root" while fstab
      // says "/dev/something_else".
      if !mp.is_null() {
        if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose != 0 {
          bb_error_msg(
            b"according to %s, %s is already mounted on %s\x00" as *const u8 as *const libc::c_char,
            b"/proc/mounts\x00" as *const u8 as *const libc::c_char,
            (*mp).mnt_fsname,
            (*mp).mnt_dir,
          );
        }
      } else if singlemount(mtcur, 1i32) != 0 {
        // ...mount this thing
        // Count number of failed mounts
        rc += 1
      }
      free((*mtcur).mnt_opts as *mut libc::c_void);
    }
  }
  // End of fstab/mtab is reached.
  // Were we looking for something specific?
  if !(*argv.offset(0)).is_null() {
    // yes
    let mut l: libc::c_ulong = 0;
    // If we didn't find anything, complain
    if (*mtcur).mnt_fsname.is_null() {
      bb_error_msg_and_die(
        b"can\'t find %s in %s\x00" as *const u8 as *const libc::c_char,
        *argv.offset(0),
        fstabname,
      );
    }
    // What happens when we try to "mount swap_partition"?
    // (fstab containts "swap_partition swap swap defaults 0 0")
    // util-linux-ng 2.13.1 does this:
    // stat("/sbin/mount.swap", 0x7fff62a3a350) = -1 ENOENT (No such file or directory)
    // mount("swap_partition", "swap", "swap", MS_MGC_VAL, NULL) = -1 ENOENT (No such file or directory)
    // lstat("swap", 0x7fff62a3a640)           = -1 ENOENT (No such file or directory)
    // write(2, "mount: mount point swap does not exist\n", 39) = 39
    // exit_group(32)                          = ?
    if nonroot != 0 {
      // fstab must have "users" or "user"
      l = parse_mount_options((*mtcur).mnt_opts, 0 as *mut *mut libc::c_char);
      if l & MOUNT_USERS as libc::c_int as libc::c_ulong == 0 {
        bb_simple_error_msg_and_die(bb_msg_you_must_be_root.as_ptr());
      }
    }
    //util-linux-2.12 does not do this check.
    // // If nothing is mounted on this directory...
    // // (otherwise repeated "mount FOO" mounts FOO again)
    //mp = find_mount_point(mtcur->mnt_dir, /*subdir_too:*/ 0);
    //if (mp) {
    //	bb_error_msg("according to %s, "
    //		"%s is already mounted on %s",
    //		bb_path_mtab_file,
    //		mp->mnt_fsname, mp->mnt_dir);
    //} else {
    // ...mount the last thing we found
    (*mtcur).mnt_opts = xstrdup((*mtcur).mnt_opts);
    append_mount_options(&mut (*mtcur).mnt_opts, cmdopts);
    resolve_mount_spec(&mut (*mtpair.as_mut_ptr()).mnt_fsname);
    rc = singlemount(mtcur, 0i32)
  }
  //ret:
  //TODO: exitcode should be ORed mask of (from "man mount"):
  // 0 success
  // 1 incorrect invocation or permissions
  // 2 system error (out of memory, cannot fork, no more loop devices)
  // 4 internal mount bug or missing nfs support in mount
  // 8 user interrupt
  //16 problems writing or locking /etc/mtab
  //32 mount failure
  //64 some mount succeeded
  return rc;
}

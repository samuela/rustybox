use libc;
extern "C" {
  pub type __dirstream;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn getpid() -> __pid_t;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn opendir(__name: *const libc::c_char) -> *mut DIR;
  #[no_mangle]
  fn closedir(__dirp: *mut DIR) -> libc::c_int;
  #[no_mangle]
  fn readdir(__dirp: *mut DIR) -> *mut dirent;
  #[no_mangle]
  fn socket(__domain: libc::c_int, __type: libc::c_int, __protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut FILE,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstat(pathname: *const libc::c_char, buf: *mut stat);
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  #[no_mangle]
  static mut option_mask32: uint32_t;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn concat_subpath_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn get_signum(name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn index_in_substrings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn bb_makedev(major: libc::c_uint, minor: libc::c_uint) -> libc::c_ulonglong;
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
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type smallint = libc::c_schar;
pub type size_t = libc::c_ulong;
pub type pid_t = __pid_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
  pub tv_sec: __time_t,
  pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
  pub st_dev: __dev_t,
  pub st_ino: __ino_t,
  pub st_nlink: __nlink_t,
  pub st_mode: __mode_t,
  pub st_uid: __uid_t,
  pub st_gid: __gid_t,
  pub __pad0: libc::c_int,
  pub st_rdev: __dev_t,
  pub st_size: __off_t,
  pub st_blksize: __blksize_t,
  pub st_blocks: __blkcnt_t,
  pub st_atim: timespec,
  pub st_mtim: timespec,
  pub st_ctim: timespec,
  pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type dev_t = __dev_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub recursion_depth: libc::c_int,
  pub mypid: pid_t,
  pub inode_list_head: *mut inode_list,
  pub kill_failed: smallint,
  pub killsig: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inode_list {
  pub next: *mut inode_list,
  pub inode: ino_t,
  pub dev: dev_t,
}
pub type C2RustUnnamed = libc::c_uint;
pub const OPT_IP4: C2RustUnnamed = 16;
pub const OPT_IP6: C2RustUnnamed = 8;
pub const OPT_SILENT: C2RustUnnamed = 4;
pub const OPT_KILL: C2RustUnnamed = 2;
pub const OPT_MOUNT: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PROC_SUBDIR_LINKS: C2RustUnnamed_0 = 3;
pub const PROC_DIR_LINKS: C2RustUnnamed_0 = 2;
pub const PROC_DIR: C2RustUnnamed_0 = 1;
pub const PROC_NET: C2RustUnnamed_0 = 0;
pub const MAPS: C2RustUnnamed_1 = 6;
pub const MMAP_DIR_LINKS: C2RustUnnamed_1 = 5;
pub const LIB_DIR_LINKS: C2RustUnnamed_1 = 4;
pub const FD_DIR_LINKS: C2RustUnnamed_1 = 3;
pub const ROOT_LINK: C2RustUnnamed_1 = 2;
pub const EXE_LINK: C2RustUnnamed_1 = 1;
pub const CWD_LINK: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
unsafe extern "C" fn add_inode(mut st: *const stat) {
  let mut curr: *mut *mut inode_list =
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).inode_list_head;
  while !(*curr).is_null() {
    if (**curr).dev == (*st).st_dev && (**curr).inode == (*st).st_ino {
      return;
    }
    curr = &mut (**curr).next
  }
  *curr = xzalloc(::std::mem::size_of::<inode_list>() as libc::c_ulong) as *mut inode_list;
  (**curr).dev = (*st).st_dev;
  (**curr).inode = (*st).st_ino;
}
unsafe extern "C" fn search_dev_inode(mut st: *const stat) -> smallint {
  let mut ilist: *mut inode_list =
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).inode_list_head;
  while !ilist.is_null() {
    if (*ilist).dev == (*st).st_dev {
      if option_mask32 & OPT_MOUNT as libc::c_int as libc::c_uint != 0 {
        return 1i32 as smallint;
      }
      if (*ilist).inode == (*st).st_ino {
        return 1i32 as smallint;
      }
    }
    ilist = (*ilist).next
  }
  return 0i32 as smallint;
}
unsafe extern "C" fn scan_proc_net_or_maps(
  mut path: *const libc::c_char,
  mut port: libc::c_uint,
) -> smallint {
  let mut f: *mut FILE = 0 as *mut FILE;
  let mut line: [libc::c_char; 256] = [0; 256];
  let mut addr: [libc::c_char; 68] = [0; 68];
  let mut major: libc::c_int = 0;
  let mut minor: libc::c_int = 0;
  let mut r: libc::c_int = 0;
  let mut uint64_inode: libc::c_longlong = 0;
  let mut tmp_port: libc::c_uint = 0;
  let mut retval: smallint = 0;
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
  let mut fmt: *const libc::c_char = 0 as *const libc::c_char;
  let mut fag: *mut libc::c_void = 0 as *mut libc::c_void;
  let mut sag: *mut libc::c_void = 0 as *mut libc::c_void;
  f = fopen_for_read(path);
  if f.is_null() {
    return 0i32 as smallint;
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).recursion_depth == PROC_NET as libc::c_int
  {
    let mut fd: libc::c_int = 0;
    /* find socket dev */
    statbuf.st_dev = 0i32 as __dev_t; /* . or .. */
    fd = socket(2i32, SOCK_DGRAM as libc::c_int, 0i32);
    if fd >= 0i32 {
      fstat(fd, &mut statbuf);
      close(fd);
    }
    fmt = b"%*d: %64[0-9A-Fa-f]:%x %*x:%*x %*x %*x:%*x %*x:%*x %*x %*d %*d %llu\x00" as *const u8
      as *const libc::c_char;
    fag = addr.as_mut_ptr() as *mut libc::c_void;
    sag = &mut tmp_port as *mut libc::c_uint as *mut libc::c_void
  } else {
    fmt = b"%*s %*s %*s %x:%x %llu\x00" as *const u8 as *const libc::c_char;
    fag = &mut major as *mut libc::c_int as *mut libc::c_void;
    sag = &mut minor as *mut libc::c_int as *mut libc::c_void
  }
  retval = 0i32 as smallint;
  while !fgets_unlocked(line.as_mut_ptr(), 255i32, f).is_null() {
    r = sscanf(
      line.as_mut_ptr(),
      fmt,
      fag,
      sag,
      &mut uint64_inode as *mut libc::c_longlong,
    );
    if r != 3i32 {
      continue;
    }
    statbuf.st_ino = uint64_inode as __ino_t;
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).recursion_depth
      == PROC_NET as libc::c_int
    {
      r = strlen(addr.as_mut_ptr()) as libc::c_int;
      if r == 8i32 && option_mask32 & OPT_IP6 as libc::c_int as libc::c_uint != 0 {
        continue;
      }
      if r > 8i32 && option_mask32 & OPT_IP4 as libc::c_int as libc::c_uint != 0 {
        continue;
      }
      if tmp_port == port {
        add_inode(&mut statbuf);
      }
    } else {
      if !(major != 0i32 && minor != 0i32 && statbuf.st_ino != 0i32 as libc::c_ulong) {
        continue;
      }
      statbuf.st_dev = bb_makedev(major as libc::c_uint, minor as libc::c_uint) as __dev_t;
      retval = search_dev_inode(&mut statbuf);
      if retval != 0 {
        break;
      }
    }
  }
  fclose(f);
  return retval;
}
unsafe extern "C" fn scan_recursive(mut path: *const libc::c_char) -> smallint {
  let mut d: *mut DIR = 0 as *mut DIR;
  let mut d_ent: *mut dirent = 0 as *mut dirent;
  let mut stop_scan: smallint = 0;
  let mut retval: smallint = 0;
  d = opendir(path);
  if d.is_null() {
    return 0i32 as smallint;
  }
  let ref mut fresh0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).recursion_depth;
  *fresh0 += 1;
  retval = 0i32 as smallint;
  stop_scan = 0i32 as smallint;
  while stop_scan == 0 && {
    d_ent = readdir(d);
    !d_ent.is_null()
  } {
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
    let mut pid: pid_t = 0;
    let mut subpath: *mut libc::c_char = 0 as *mut libc::c_char;
    subpath = concat_subpath_file(path, (*d_ent).d_name.as_mut_ptr());
    if subpath.is_null() {
      continue;
    }
    let mut current_block_25: u64;
    match (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).recursion_depth {
      1 => {
        pid = bb_strtou(
          (*d_ent).d_name.as_mut_ptr(),
          0 as *mut *mut libc::c_char,
          10i32,
        ) as pid_t;
        if *bb_errno != 0i32
          || pid == (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).mypid
          || scan_recursive(subpath) as libc::c_int == 0i32
        {
          current_block_25 = 17784502470059252271;
        } else {
          if option_mask32 & OPT_KILL as libc::c_int as libc::c_uint != 0 {
            if kill(
              pid,
              (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).killsig,
            ) != 0i32
            {
              bb_perror_msg(
                b"kill pid %s\x00" as *const u8 as *const libc::c_char,
                (*d_ent).d_name.as_mut_ptr(),
              );
              (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).kill_failed = 1i32 as smallint
            }
          }
          if option_mask32 & OPT_SILENT as libc::c_int as libc::c_uint == 0 {
            printf(
              b"%s \x00" as *const u8 as *const libc::c_char,
              (*d_ent).d_name.as_mut_ptr(),
            );
          }
          retval = 1i32 as smallint;
          current_block_25 = 17784502470059252271;
        }
      }
      2 => {
        match index_in_substrings(
          b"cwd\x00exe\x00root\x00fd\x00lib\x00mmap\x00maps\x00\x00" as *const u8
            as *const libc::c_char,
          (*d_ent).d_name.as_mut_ptr(),
        ) {
          0 | 1 | 2 => {
            current_block_25 = 16160708752528353120;
          }
          3 | 4 | 5 => {
            current_block_25 = 1653815207292225316;
            match current_block_25 {
              1653815207292225316 => {
                stop_scan = scan_recursive(subpath);
                if stop_scan != 0 {
                  retval = stop_scan
                }
              }
              _ => {
                stop_scan = scan_proc_net_or_maps(subpath, 0i32 as libc::c_uint);
                if stop_scan != 0 {
                  retval = stop_scan
                }
              }
            }
            current_block_25 = 17784502470059252271;
          }
          6 => {
            current_block_25 = 7597295434072776329;
            match current_block_25 {
              1653815207292225316 => {
                stop_scan = scan_recursive(subpath);
                if stop_scan != 0 {
                  retval = stop_scan
                }
              }
              _ => {
                stop_scan = scan_proc_net_or_maps(subpath, 0i32 as libc::c_uint);
                if stop_scan != 0 {
                  retval = stop_scan
                }
              }
            }
            current_block_25 = 17784502470059252271;
          }
          _ => {
            current_block_25 = 17784502470059252271;
          }
        }
      }
      3 => {
        current_block_25 = 16160708752528353120;
      }
      _ => {
        current_block_25 = 17784502470059252271;
      }
    }
    match current_block_25 {
      16160708752528353120 => {
        if !(stat(subpath, &mut statbuf) < 0i32) {
          stop_scan = search_dev_inode(&mut statbuf);
          if stop_scan != 0 {
            retval = stop_scan
          }
        }
      }
      _ => {}
    }
    free(subpath as *mut libc::c_void);
  }
  closedir(d);
  let ref mut fresh1 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).recursion_depth;
  *fresh1 -= 1;
  return retval;
}
#[no_mangle]
pub unsafe extern "C" fn fuser_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut pp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).mypid = getpid();
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).killsig = 9i32;
  /* Handle -SIGNAL. Oh my... */
  pp = argv; /* it's "-4" or "-6" */
  loop {
    pp = pp.offset(1);
    if (*pp).is_null() {
      break;
    }
    let mut sig: libc::c_int = 0;
    let mut arg: *mut libc::c_char = *pp;
    if *arg.offset(0) as libc::c_int != '-' as i32 {
      continue;
    }
    if *arg.offset(1) as libc::c_int == '-' as i32
      && *arg.offset(2) as libc::c_int == '\u{0}' as i32
    {
      break;
    }
    if (*arg.offset(1) as libc::c_int == '4' as i32 || *arg.offset(1) as libc::c_int == '6' as i32)
      && *arg.offset(2) as libc::c_int == '\u{0}' as i32
    {
      continue;
    }
    sig = get_signum(&mut *arg.offset(1));
    if sig < 0i32 {
      continue;
    }
    /* "-SIGNAL" option found. Remove it and bail out */
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).killsig = sig;
    loop {
      arg = *pp.offset(1);
      let ref mut fresh2 = *pp.offset(0);
      *fresh2 = arg;
      pp = pp.offset(1);
      if arg.is_null() {
        break;
      }
    }
    break;
  }
  getopt32(
    argv,
    b"^mks64\x00-1\x00" as *const u8 as *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  pp = argv;
  while !(*pp).is_null() {
    /* parse net arg */
    let mut port: libc::c_uint = 0;
    let mut path: [libc::c_char; 15] = [0; 15];
    strcpy(
      path.as_mut_ptr(),
      b"/proc/net/\x00" as *const u8 as *const libc::c_char,
    );
    if sscanf(
      *pp,
      b"%u/%4s\x00" as *const u8 as *const libc::c_char,
      &mut port as *mut libc::c_uint,
      path
        .as_mut_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as isize)
        .offset(-1),
    ) == 2i32
      && access(path.as_mut_ptr(), 4i32) == 0i32
    {
      /* PORT/PROTO */
      scan_proc_net_or_maps(path.as_mut_ptr(), port);
    } else {
      /* FILE */
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
      xstat(*pp, &mut statbuf);
      add_inode(&mut statbuf);
    }
    pp = pp.offset(1)
  }
  if scan_recursive(b"/proc\x00" as *const u8 as *const libc::c_char) != 0 {
    if option_mask32 & OPT_SILENT as libc::c_int as libc::c_uint == 0 {
      bb_putchar('\n' as i32);
    }
    return (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).kill_failed as libc::c_int;
  }
  return 1i32;
}

use libc;
extern "C" {
  pub type __dirstream;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn closedir(__dirp: *mut DIR) -> libc::c_int;
  #[no_mangle]
  fn readdir(__dirp: *mut DIR) -> *mut dirent;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn warn_opendir(path: *const libc::c_char) -> *mut DIR;
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
  #[no_mangle]
  fn make_human_readable_str(
    size: libc::c_ulonglong,
    block_size: libc::c_ulong,
    display_unit: libc::c_ulong,
  ) -> *const libc::c_char;
  #[no_mangle]
  static mut option_mask32: uint32_t;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn concat_subpath_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn is_in_ino_dev_hashtable(statbuf: *const stat) -> *mut libc::c_char;
  #[no_mangle]
  fn add_to_ino_dev_hashtable(statbuf: *const stat, name: *const libc::c_char);
  #[no_mangle]
  fn reset_ino_dev_hashtable();
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
pub type __syscall_slong_t = libc::c_long;
pub type uint32_t = __uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub disp_unit: libc::c_ulong,
  pub max_print_depth: libc::c_int,
  pub status: bool,
  pub slink_depth: libc::c_int,
  pub du_depth: libc::c_int,
  pub dir_dev: dev_t,
}
pub type C2RustUnnamed = libc::c_uint;
pub const OPT_m_mbytes: C2RustUnnamed = 1024;
pub const OPT_h_for_humans: C2RustUnnamed = 512;
pub const OPT_c_total: C2RustUnnamed = 256;
pub const OPT_l_hardlinks: C2RustUnnamed = 128;
pub const OPT_d_maxdepth: C2RustUnnamed = 64;
pub const OPT_x_one_FS: C2RustUnnamed = 32;
pub const OPT_s_total_norecurse: C2RustUnnamed = 16;
pub const OPT_L_follow_links: C2RustUnnamed = 8;
pub const OPT_k_kbytes: C2RustUnnamed = 4;
pub const OPT_H_follow_links: C2RustUnnamed = 2;
pub const OPT_a_files_too: C2RustUnnamed = 1;
unsafe extern "C" fn print(mut size: libc::c_ulonglong, mut filename: *const libc::c_char) {
  /* TODO - May not want to defer error checking here. */
  /* ~30 bytes of code for extra comtat:
   * coreutils' du rounds sizes up:
   * for example,  1025k file is shown as "2" by du -m.
   * We round to nearest if human-readable [too hard to fix],
   * else (fixed scale such as -m), we round up. To that end,
   * add yet another half of the unit before displaying:
   */
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).disp_unit != 0 {
    size = size.wrapping_add(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .disp_unit
        .wrapping_sub(1i32 as libc::c_ulong)
        .wrapping_div((512i32 * 2i32) as libc::c_uint as libc::c_ulong) as libc::c_ulonglong,
    )
  }
  printf(
    b"%s\t%s\n\x00" as *const u8 as *const libc::c_char,
    make_human_readable_str(
      size,
      512i32 as libc::c_ulong,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).disp_unit,
    ),
    filename,
  );
}
/* tiny recursive du */
unsafe extern "C" fn du(mut filename: *const libc::c_char) -> libc::c_ulonglong {
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
  let mut sum: libc::c_ulonglong = 0;
  if lstat(filename, &mut statbuf) != 0i32 {
    bb_simple_perror_msg(filename);
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).status = 1i32 != 0;
    return 0i32 as libc::c_ulonglong;
  }
  if option_mask32 & OPT_x_one_FS as libc::c_int as libc::c_uint != 0 {
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).du_depth == 0i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).dir_dev = statbuf.st_dev
    } else if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).dir_dev != statbuf.st_dev {
      return 0i32 as libc::c_ulonglong;
    }
  }
  sum = statbuf.st_blocks as libc::c_ulonglong;
  if statbuf.st_mode & 0o170000i32 as libc::c_uint == 0o120000i32 as libc::c_uint {
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).slink_depth
      > (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).du_depth
    {
      /* -H or -L */
      if stat(filename, &mut statbuf) != 0i32 {
        bb_simple_perror_msg(filename);
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).status = 1i32 != 0;
        return 0i32 as libc::c_ulonglong;
      }
      sum = statbuf.st_blocks as libc::c_ulonglong;
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).slink_depth == 1i32 {
        /* Convert -H to -L */
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).slink_depth = 2147483647i32
      }
    }
  }
  if option_mask32 & OPT_l_hardlinks as libc::c_int as libc::c_uint == 0
    && statbuf.st_nlink > 1i32 as libc::c_ulong
  {
    /* Add files/directories with links only once */
    if !is_in_ino_dev_hashtable(&mut statbuf).is_null() {
      return 0i32 as libc::c_ulonglong;
    }
    add_to_ino_dev_hashtable(&mut statbuf, 0 as *const libc::c_char);
  }
  if statbuf.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint {
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut entry: *mut dirent = 0 as *mut dirent;
    let mut newfile: *mut libc::c_char = 0 as *mut libc::c_char;
    dir = warn_opendir(filename);
    if dir.is_null() {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).status = 1i32 != 0;
      return sum;
    }
    loop {
      entry = readdir(dir);
      if entry.is_null() {
        break;
      }
      newfile = concat_subpath_file(filename, (*entry).d_name.as_mut_ptr());
      if newfile.is_null() {
        continue;
      }
      let ref mut fresh0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).du_depth;
      *fresh0 += 1;
      sum = sum.wrapping_add(du(newfile));
      let ref mut fresh1 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).du_depth;
      *fresh1 -= 1;
      free(newfile as *mut libc::c_void);
    }
    closedir(dir);
  } else if option_mask32 & OPT_a_files_too as libc::c_int as libc::c_uint == 0
    && (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).du_depth != 0i32
  {
    return sum;
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).du_depth
    <= (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_print_depth
  {
    print(sum, filename);
  }
  return sum;
}
#[no_mangle]
pub unsafe extern "C" fn du_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut total: libc::c_ulonglong = 0;
  let mut slink_depth_save: libc::c_int = 0;
  let mut opt: libc::c_uint = 0;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).disp_unit = 1024i32 as libc::c_ulong;
  if !getenv(b"POSIXLY_CORRECT\x00" as *const u8 as *const libc::c_char).is_null() {
    /* TODO - a new libbb function? */
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).disp_unit = 512i32 as libc::c_ulong
  }
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_print_depth = 2147483647i32;
  /* Note: SUSv3 specifies that -a and -s options cannot be used together
   * in strictly conforming applications.  However, it also says that some
   * du implementations may produce output when -a and -s are used together.
   * gnu du exits with an error code in this case.  We choose to simply
   * ignore -a.  This is consistent with -s being equivalent to -d 0.
   */
  opt = getopt32(
    argv,
    b"^aHkLsxd:+lchm\x00h-km:k-hm:m-hk:H-L:L-H:s-d:d-s\x00" as *const u8 as *const libc::c_char,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_print_depth as *mut libc::c_int,
  );
  argv = argv.offset(optind as isize);
  if opt & OPT_h_for_humans as libc::c_int as libc::c_uint != 0 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).disp_unit = 0i32 as libc::c_ulong
  }
  if opt & OPT_m_mbytes as libc::c_int as libc::c_uint != 0 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).disp_unit =
      (1024i32 * 1024i32) as libc::c_ulong
  }
  if opt & OPT_k_kbytes as libc::c_int as libc::c_uint != 0 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).disp_unit = 1024i32 as libc::c_ulong
  }
  if opt & OPT_H_follow_links as libc::c_int as libc::c_uint != 0 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).slink_depth = 1i32
  }
  if opt & OPT_L_follow_links as libc::c_int as libc::c_uint != 0 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).slink_depth = 2147483647i32
  }
  if opt & OPT_s_total_norecurse as libc::c_int as libc::c_uint != 0 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_print_depth = 0i32
  }
  /* go through remaining args (if any) */
  if (*argv).is_null() {
    argv = argv.offset(-1);
    *argv = b".\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).slink_depth == 1i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).slink_depth = 0i32
    }
  }
  slink_depth_save = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).slink_depth;
  total = 0i32 as libc::c_ulonglong;
  loop {
    total = total.wrapping_add(du(*argv));
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).slink_depth = slink_depth_save;
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  if opt & OPT_c_total as libc::c_int as libc::c_uint != 0 {
    print(total, b"total\x00" as *const u8 as *const libc::c_char);
  }
  fflush_stdout_and_exit((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).status as libc::c_int);
}

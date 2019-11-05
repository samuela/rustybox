use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  static mut stdout: *mut FILE;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn fileno_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn xmalloc_follow_symlinks(path: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xunlink(pathname: *const libc::c_char);
  #[no_mangle]
  fn xfstat(fd: libc::c_int, buf: *mut stat, errmsg: *const libc::c_char);
  #[no_mangle]
  fn xopen3(pathname: *const libc::c_char, flags: libc::c_int, mode: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xrename(oldpath: *const libc::c_char, newpath: *const libc::c_char);
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc_fgetline(file: *mut FILE) -> *mut libc::c_char;
  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
use crate::librb::stat;
use crate::librb::timespec;

pub type _IO_lock_t = ();

use crate::librb::FILE;
#[no_mangle]
pub unsafe extern "C" fn add_remove_shell_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut orig_fp: *mut FILE = 0 as *mut FILE;
  let mut orig_fn: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut new_fn: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut sb: stat = stat {
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
  sb.st_mode = 0o666i32 as __mode_t;
  argv = argv.offset(1);
  orig_fn = xmalloc_follow_symlinks(b"/etc/shells\x00" as *const u8 as *const libc::c_char);
  if orig_fn.is_null() {
    return 1i32;
  }
  orig_fp = fopen_for_read(orig_fn);
  if !orig_fp.is_null() {
    xfstat(fileno_unlocked(orig_fp), &mut sb, orig_fn);
  }
  new_fn = xasprintf(b"%s.tmp\x00" as *const u8 as *const libc::c_char, orig_fn);
  /*
   * O_TRUNC or O_EXCL? At the first glance, O_EXCL looks better,
   * since it prevents races. But: (1) it requires a retry loop,
   * (2) if /etc/shells.tmp is *stale*, then retry loop
   * with O_EXCL will never succeed - it should have a timeout,
   * after which it should revert to O_TRUNC.
   * For now, I settle for O_TRUNC instead.
   */
  xmove_fd(
    xopen3(
      new_fn,
      0o1i32 | 0o100i32 | 0o1000i32,
      sb.st_mode as libc::c_int,
    ),
    1i32,
  );
  /* TODO?
  xfchown(STDOUT_FILENO, sb.st_uid, sb.st_gid);
  */
  if !orig_fp.is_null() {
    /* Copy old file, possibly skipping removed shell names */
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
      line = xmalloc_fgetline(orig_fp);
      if line.is_null() {
        break;
      }
      let mut current_block_13: u64;
      let mut cpp: *mut *mut libc::c_char = argv;
      loop {
        if (*cpp).is_null() {
          current_block_13 = 4808432441040389987;
          break;
        }
        if *cpp != 1i32 as uintptr_t as *mut libc::c_char && strcmp(*cpp, line) == 0i32 {
          /* Old file has this shell name */
          if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(0) as libc::c_int == 'r' as i32) {
            current_block_13 = 221764114354898702;
            break;
          }
          /* we are add-shell */
          /* mark this name as "do not add" */
          *cpp = 1i32 as uintptr_t as *mut libc::c_char
        }
        cpp = cpp.offset(1)
      }
      match current_block_13 {
        4808432441040389987 => {
          /* copy shell name from old to new file */
          puts(line);
        }
        _ => {}
      }
      /* we are remove-shell */
      /* delete this name by not copying it */
      free(line as *mut libc::c_void);
    }
  }
  if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(0) as libc::c_int == 'a' as i32) {
    let mut cpp_0: *mut *mut libc::c_char = argv;
    while !(*cpp_0).is_null() {
      if *cpp_0 != 1i32 as uintptr_t as *mut libc::c_char {
        puts(*cpp_0);
      }
      cpp_0 = cpp_0.offset(1)
    }
  }
  /* Ensure we wrote out everything */
  if fclose(stdout) != 0i32 {
    xunlink(new_fn);
    bb_perror_msg_and_die(
      b"%s: write error\x00" as *const u8 as *const libc::c_char,
      new_fn,
    );
  }
  /* Small hole: if rename fails, /etc/shells.tmp is not removed */
  xrename(new_fn, orig_fn);
  return 0i32;
}

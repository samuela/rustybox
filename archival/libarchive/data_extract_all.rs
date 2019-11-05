use libc;
extern "C" {
  pub type hardlinks_t;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn getpid() -> __pid_t;
  #[no_mangle]
  fn unlink(__name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn chown(__file: *const libc::c_char, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
  #[no_mangle]
  fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
  #[no_mangle]
  fn mknod(__path: *const libc::c_char, __mode: __mode_t, __dev: __dev_t) -> libc::c_int;
  #[no_mangle]
  fn utimes(__file: *const libc::c_char, __tvp: *const timeval) -> libc::c_int;
  /* Search for an entry with a matching username.  */
  #[no_mangle]
  fn bb_internal_getpwnam(__name: *const libc::c_char) -> *mut passwd;
  /* Search for an entry with a matching group name.  */
  #[no_mangle]
  fn bb_internal_getgrnam(__name: *const libc::c_char) -> *mut group;
  /* Some useful definitions */
  /* Macros for min/max.  */
  /* buffer allocation schemes */
  /* glibc uses __errno_location() to get a ptr to errno */
  /* We can just memorize it once - no multithreading in busybox :) */
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn bb_copyfd_exact_size(fd1: libc::c_int, fd2: libc::c_int, size: off_t);
  #[no_mangle]
  fn xopen3(pathname: *const libc::c_char, flags: libc::c_int, mode: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xrename(oldpath: *const libc::c_char, newpath: *const libc::c_char);
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_make_directory(
    path: *mut libc::c_char,
    mode: libc::c_long,
    flags: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn data_skip(archive_handle: *mut archive_handle_t);
  #[no_mangle]
  fn create_or_remember_link(
    link_placeholders: *mut *mut llist_t,
    target: *const libc::c_char,
    linkname: *const libc::c_char,
    hard_link: libc::c_int,
  );
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type smallint = libc::c_schar;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
pub type mode_t = __mode_t;
use crate::librb::timespec;
use crate::librb::stat;
pub type dev_t = __dev_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
  pub tv_sec: __time_t,
  pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
  pub pw_name: *mut libc::c_char,
  pub pw_passwd: *mut libc::c_char,
  pub pw_uid: __uid_t,
  pub pw_gid: __gid_t,
  pub pw_gecos: *mut libc::c_char,
  pub pw_dir: *mut libc::c_char,
  pub pw_shell: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
  pub gr_name: *mut libc::c_char,
  pub gr_passwd: *mut libc::c_char,
  pub gr_gid: __gid_t,
  pub gr_mem: *mut *mut libc::c_char,
}
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
pub type uoff_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_int;
/* cp --reflink[=always] */
/*
 * Hole. cp may have some bits set here,
 * they should not affect remove_file()/copy_file()
 */
pub const FILEUTILS_IGNORE_CHMOD_ERR: C2RustUnnamed = -2147483648;
/* cp --reflink=auto */
pub const FILEUTILS_REFLINK_ALWAYS: C2RustUnnamed = 262144;
/* --remove-destination */
/* bit 17 skipped for "cp --parents" */
pub const FILEUTILS_REFLINK: C2RustUnnamed = 131072;
/* -T */
pub const FILEUTILS_RMDEST: C2RustUnnamed = 32768;
/* -u */
pub const FILEUTILS_NO_TARGET_DIR: C2RustUnnamed = 16384;
/* -v */
pub const FILEUTILS_UPDATE: C2RustUnnamed = 8192;
/* -H */
/* -a = -pdR (mapped in cp.c) */
/* -r = -dR  (mapped in cp.c) */
/* -P = -d   (mapped in cp.c) */
pub const FILEUTILS_VERBOSE: C2RustUnnamed = 4096;
/* -L */
pub const FILEUTILS_DEREFERENCE_L0: C2RustUnnamed = 256;
/* -s */
pub const FILEUTILS_DEREF_SOFTLINK: C2RustUnnamed = 128;
/* -l */
pub const FILEUTILS_MAKE_SOFTLINK: C2RustUnnamed = 64;
/* -i */
pub const FILEUTILS_MAKE_HARDLINK: C2RustUnnamed = 32;
/* -f */
pub const FILEUTILS_INTERACTIVE: C2RustUnnamed = 16;
/* -R */
pub const FILEUTILS_FORCE: C2RustUnnamed = 8;
/* !-d */
pub const FILEUTILS_RECUR: C2RustUnnamed = 4;
/* -p */
pub const FILEUTILS_DEREFERENCE: C2RustUnnamed = 2;
/* cp.c, mv.c, install.c depend on these values. CAREFUL when changing them! */
pub const FILEUTILS_PRESERVE_STATUS: C2RustUnnamed = 1;
use crate::librb::bb_uidgid_t;
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
use crate::libbb::llist::llist_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_header_t {
  pub name: *mut libc::c_char,
  pub link_target: *mut libc::c_char,
  pub tar__uname: *mut libc::c_char,
  pub tar__gname: *mut libc::c_char,
  pub size: off_t,
  pub uid: uid_t,
  pub gid: gid_t,
  pub mode: mode_t,
  pub mtime: time_t,
  pub device: dev_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_handle_t {
  pub ah_flags: libc::c_uint,
  pub src_fd: libc::c_int,
  pub filter: Option<unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char>,
  pub accept: *mut llist_t,
  pub reject: *mut llist_t,
  pub passed: *mut llist_t,
  pub file_header: *mut file_header_t,
  pub link_placeholders: *mut llist_t,
  pub action_header: Option<unsafe extern "C" fn(_: *const file_header_t) -> ()>,
  pub action_data: Option<unsafe extern "C" fn(_: *mut archive_handle_t) -> ()>,
  pub seek: Option<unsafe extern "C" fn(_: libc::c_int, _: off_t) -> ()>,
  pub offset: off_t,
  pub tar__strip_components: libc::c_uint,
  pub tar__end: smallint,
  pub tar__longname: *mut libc::c_char,
  pub tar__linkname: *mut libc::c_char,
  pub tar__to_command: *mut libc::c_char,
  pub tar__to_command_shell: *const libc::c_char,
  pub cpio__blocks: uoff_t,
  pub cpio__owner: bb_uidgid_t,
  pub cpio__hardlinks_to_create: *mut hardlinks_t,
  pub cpio__created_hardlinks: *mut hardlinks_t,
  pub dpkg__buffer: *mut libc::c_char,
  pub dpkg__action_data_subarchive:
    Option<unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char>,
  pub dpkg__sub_archive: *mut archive_handle_t,
}
/* vi: set sw=4 ts=4: */
/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn data_extract_all(mut archive_handle: *mut archive_handle_t) {
  let mut current_block: u64;
  let mut file_header: *mut file_header_t = (*archive_handle).file_header;
  let mut dst_fd: libc::c_int = 0;
  let mut res: libc::c_int = 0;
  let mut hard_link: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut dst_name: *mut libc::c_char = 0 as *mut libc::c_char;
  /* Hard links are encoded as regular files of size 0
   * with a nonempty link field */
  hard_link = 0 as *mut libc::c_char;
  if (*file_header).mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint
    && (*file_header).size == 0i32 as libc::c_long
  {
    hard_link = (*file_header).link_target
  }
  dst_name = (*file_header).name;
  if (*archive_handle).tar__strip_components != 0 {
    let mut n: libc::c_uint = (*archive_handle).tar__strip_components;
    loop {
      dst_name = strchr(dst_name, '/' as i32);
      if dst_name.is_null() || *dst_name.offset(1) as libc::c_int == '\u{0}' as i32 {
        data_skip(archive_handle);
        current_block = 9521147444787763968;
        break;
      } else {
        dst_name = dst_name.offset(1);
        /*
         * Link target is shortened only for hardlinks:
         * softlinks restored unchanged.
         */
        if !hard_link.is_null() {
          // GNU tar 1.26 does not check that we reached end of link name:
          // if "dir/hardlink" is hardlinked to "file",
          // tar xvf a.tar --strip-components=1 says:
          //  tar: hardlink: Cannot hard link to '': No such file or directory
          // and continues processing. We silently skip such entries.
          hard_link = strchr(hard_link, '/' as i32);
          if hard_link.is_null() || *hard_link.offset(1) as libc::c_int == '\u{0}' as i32 {
            data_skip(archive_handle);
            current_block = 9521147444787763968;
            break;
          } else {
            hard_link = hard_link.offset(1)
          }
        }
        n = n.wrapping_sub(1);
        if !(n != 0i32 as libc::c_uint) {
          current_block = 4808432441040389987;
          break;
        }
      }
    }
  } else {
    current_block = 4808432441040389987;
  }
  match current_block {
    4808432441040389987 => {
      if (*archive_handle).ah_flags & (1i32 << 1i32) as libc::c_uint != 0 {
        let mut slash: *mut libc::c_char = strrchr(dst_name, '/' as i32);
        if !slash.is_null() {
          *slash = '\u{0}' as i32 as libc::c_char;
          bb_make_directory(
            dst_name,
            -1i32 as libc::c_long,
            FILEUTILS_RECUR as libc::c_int,
          );
          *slash = '/' as i32 as libc::c_char
        }
      }
      if (*archive_handle).ah_flags & (1i32 << 2i32) as libc::c_uint != 0 {
        /* Remove the entry if it exists */
        if !((*file_header).mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint) {
          if !hard_link.is_null() {
            /* Ugly special case:
             * tar cf t.tar hardlink1 hardlink2 hardlink1
             * results in this tarball structure:
             * hardlink1
             * hardlink2 -> hardlink1
             * hardlink1 -> hardlink1 <== !!!
             */
            if strcmp(hard_link, dst_name) == 0i32 {
              current_block = 9521147444787763968;
            } else {
              current_block = 5494826135382683477;
            }
          } else {
            current_block = 5494826135382683477;
          }
          match current_block {
            9521147444787763968 => {}
            _ => {
              /* Proceed with deleting */
              if unlink(dst_name) == -1i32 && *bb_errno != 2i32 {
                bb_perror_msg_and_die(
                  b"can\'t remove old file %s\x00" as *const u8 as *const libc::c_char,
                  dst_name,
                );
              }
              current_block = 3689906465960840878;
            }
          }
        } else {
          current_block = 3689906465960840878;
        }
      } else if (*archive_handle).ah_flags & (1i32 << 3i32) as libc::c_uint != 0 {
        /* Remove the existing entry if its older than the extracted entry */
        let mut existing_sb: stat = stat {
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
        if lstat(dst_name, &mut existing_sb) == -1i32 {
          if *bb_errno != 2i32 {
            bb_simple_perror_msg_and_die(
              b"can\'t stat old file\x00" as *const u8 as *const libc::c_char,
            );
          }
          current_block = 3689906465960840878;
        } else if existing_sb.st_mtim.tv_sec >= (*file_header).mtime {
          if !((*file_header).mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint) {
            bb_error_msg(
              b"%s not created: newer or same age file exists\x00" as *const u8
                as *const libc::c_char,
              dst_name,
            );
          }
          data_skip(archive_handle);
          current_block = 9521147444787763968;
        } else {
          if unlink(dst_name) == -1i32 && *bb_errno != 21i32 {
            bb_perror_msg_and_die(
              b"can\'t remove old file %s\x00" as *const u8 as *const libc::c_char,
              dst_name,
            );
          }
          current_block = 3689906465960840878;
        }
      } else {
        current_block = 3689906465960840878;
      }
      match current_block {
        9521147444787763968 => {}
        _ =>
        /* Handle hard links separately */
        {
          if !hard_link.is_null() {
            create_or_remember_link(
              &mut (*archive_handle).link_placeholders,
              hard_link,
              dst_name,
              1i32,
            );
          } else {
            /* Create the filesystem entry */
            match (*file_header).mode & 0o170000i32 as libc::c_uint {
              32768 => {
                /* Regular file */
                let mut dst_nameN: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut flags: libc::c_int = 0o1i32 | 0o100i32 | 0o200i32;
                if (*archive_handle).ah_flags & (1i32 << 7i32) as libc::c_uint != 0 {
                  flags = 0o1i32 | 0o100i32 | 0o1000i32
                }
                dst_nameN = dst_name;
                if (*archive_handle).ah_flags & (1i32 << 9i32) as libc::c_uint != 0 {
                  /* rpm-style temp file name */
                  dst_nameN = xasprintf(
                    b"%s;%x\x00" as *const u8 as *const libc::c_char,
                    dst_name,
                    getpid(),
                  )
                }
                dst_fd = xopen3(dst_nameN, flags, (*file_header).mode as libc::c_int);
                bb_copyfd_exact_size((*archive_handle).src_fd, dst_fd, (*file_header).size);
                close(dst_fd);
                if (*archive_handle).ah_flags & (1i32 << 9i32) as libc::c_uint != 0 {
                  xrename(dst_nameN, dst_name);
                  free(dst_nameN as *mut libc::c_void);
                }
              }
              16384 => {
                res = mkdir(dst_name, (*file_header).mode);
                if res != 0i32 && *bb_errno != 21i32 && *bb_errno != 17i32 {
                  bb_perror_msg(
                    b"can\'t make dir %s\x00" as *const u8 as *const libc::c_char,
                    dst_name,
                  );
                }
              }
              40960 => {
                /* Symlink */
                //TODO: what if file_header->link_target == NULL (say, corrupted tarball?)
                /* To avoid a directory traversal attack via symlinks,
                 * do not restore symlinks with ".." components
                 * or symlinks starting with "/", unless a magic
                 * envvar is set.
                 *
                 * For example, consider a .tar created via:
                 *  $ tar cvf bug.tar anything.txt
                 *  $ ln -s /tmp symlink
                 *  $ tar --append -f bug.tar symlink
                 *  $ rm symlink
                 *  $ mkdir symlink
                 *  $ tar --append -f bug.tar symlink/evil.py
                 *
                 * This will result in an archive that contains:
                 *  $ tar --list -f bug.tar
                 *  anything.txt
                 *  symlink [-> /tmp]
                 *  symlink/evil.py
                 *
                 * Untarring bug.tar would otherwise place evil.py in '/tmp'.
                 */
                create_or_remember_link(
                  &mut (*archive_handle).link_placeholders,
                  (*file_header).link_target,
                  dst_name,
                  0i32,
                );
              }
              49152 | 24576 | 8192 | 4096 => {
                res = mknod(dst_name, (*file_header).mode, (*file_header).device);
                if res != 0i32 {
                  bb_perror_msg(
                    b"can\'t create node %s\x00" as *const u8 as *const libc::c_char,
                    dst_name,
                  );
                }
              }
              _ => {
                bb_simple_error_msg_and_die(
                  b"unrecognized file type\x00" as *const u8 as *const libc::c_char,
                );
              }
            }
            if !((*file_header).mode & 0o170000i32 as libc::c_uint == 0o120000i32 as libc::c_uint) {
              if (*archive_handle).ah_flags & (1i32 << 4i32) as libc::c_uint == 0 {
                let mut uid: uid_t = (*file_header).uid;
                let mut gid: gid_t = (*file_header).gid;
                if (*archive_handle).ah_flags & (1i32 << 6i32) as libc::c_uint == 0 {
                  if !(*file_header).tar__uname.is_null() {
                    //TODO: cache last name/id pair?
                    let mut pwd: *mut passwd = bb_internal_getpwnam((*file_header).tar__uname);
                    if !pwd.is_null() {
                      uid = (*pwd).pw_uid
                    }
                  }
                  if !(*file_header).tar__gname.is_null() {
                    let mut grp: *mut group = bb_internal_getgrnam((*file_header).tar__gname);
                    if !grp.is_null() {
                      gid = (*grp).gr_gid
                    }
                  }
                }
                /* GNU tar 1.15.1 uses chown, not lchown */
                chown(dst_name, uid, gid);
              }
              /* uclibc has no lchmod, glibc is even stranger -
               * it has lchmod which seems to do nothing!
               * so we use chmod... */
              if (*archive_handle).ah_flags & (1i32 << 5i32) as libc::c_uint == 0 {
                chmod(dst_name, (*file_header).mode);
              }
              if (*archive_handle).ah_flags & (1i32 << 0i32) as libc::c_uint != 0 {
                let mut t: [timeval; 2] = [timeval {
                  tv_sec: 0,
                  tv_usec: 0,
                }; 2];
                t[0].tv_sec = (*file_header).mtime;
                t[1].tv_sec = t[0].tv_sec;
                t[0].tv_usec = 0i32 as __suseconds_t;
                t[1].tv_usec = t[0].tv_usec;
                utimes(dst_name, t.as_mut_ptr() as *const timeval);
              }
            }
          }
        }
      }
    }
    _ => {}
  };
}

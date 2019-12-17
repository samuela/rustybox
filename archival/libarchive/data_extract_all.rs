use crate::archival::libarchive::bb_archive::archive_handle_t;
use crate::archival::libarchive::bb_archive::file_header_t;
use crate::libbb::ptr_to_globals::bb_errno;
use crate::libpwdgrp::pwd_grp::bb_internal_getpwnam;
use libc;
use libc::chmod;
use libc::chown;
use libc::close;
use libc::free;
use libc::getpid;
use libc::gid_t;
use libc::group;
use libc::lstat;
use libc::mknod;
use libc::mode_t;
use libc::passwd;
use libc::stat;
use libc::strchr;
use libc::strcmp;
use libc::strrchr;
use libc::suseconds_t;
use libc::timeval;
use libc::uid_t;
use libc::unlink;
extern "C" {

  #[no_mangle]
  fn mkdir(__path: *const libc::c_char, __mode: mode_t) -> libc::c_int;

  #[no_mangle]
  fn utimes(__file: *const libc::c_char, __tvp: *const timeval) -> libc::c_int;

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

/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn data_extract_all(mut archive_handle: *mut archive_handle_t) {
  let mut current_block: u64;
  let mut file_header: *mut file_header_t = (*archive_handle).file_header;
  let mut dst_fd: libc::c_int = 0;
  let mut res: libc::c_int = 0;
  let mut hard_link: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut dst_name: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  /* Hard links are encoded as regular files of size 0
   * with a nonempty link field */
  hard_link = std::ptr::null_mut::<libc::c_char>();
  if (*file_header).mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint
    && (*file_header).size == 0
  {
    hard_link = (*file_header).link_target
  }
  dst_name = (*file_header).name;
  if (*archive_handle).tar__strip_components != 0 {
    let mut n: libc::c_uint = (*archive_handle).tar__strip_components;
    loop {
      dst_name = strchr(dst_name, '/' as i32);
      if dst_name.is_null() || *dst_name.offset(1) as libc::c_int == '\u{0}' as i32 {
        crate::archival::libarchive::data_skip::data_skip(archive_handle);
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
            crate::archival::libarchive::data_skip::data_skip(archive_handle);
            current_block = 9521147444787763968;
            break;
          } else {
            hard_link = hard_link.offset(1)
          }
        }
        n = n.wrapping_sub(1);
        if !(n != 0 as libc::c_uint) {
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
          crate::libbb::make_directory::bb_make_directory(
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
            if strcmp(hard_link, dst_name) == 0 {
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
                crate::libbb::perror_msg::bb_perror_msg_and_die(
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
        let mut existing_sb: stat = std::mem::zeroed();
        if lstat(dst_name, &mut existing_sb) == -1i32 {
          if *bb_errno != 2i32 {
            crate::libbb::perror_msg::bb_simple_perror_msg_and_die(
              b"can\'t stat old file\x00" as *const u8 as *const libc::c_char,
            );
          }
          current_block = 3689906465960840878;
        } else if existing_sb.st_mtime >= (*file_header).mtime {
          if !((*file_header).mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint) {
            crate::libbb::verror_msg::bb_error_msg(
              b"%s not created: newer or same age file exists\x00" as *const u8
                as *const libc::c_char,
              dst_name,
            );
          }
          crate::archival::libarchive::data_skip::data_skip(archive_handle);
          current_block = 9521147444787763968;
        } else {
          if unlink(dst_name) == -1i32 && *bb_errno != 21i32 {
            crate::libbb::perror_msg::bb_perror_msg_and_die(
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
            crate::archival::libarchive::unsafe_symlink_target::create_or_remember_link(
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
                let mut dst_nameN: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
                let mut flags: libc::c_int = 0o1i32 | 0o100i32 | 0o200i32;
                if (*archive_handle).ah_flags & (1i32 << 7i32) as libc::c_uint != 0 {
                  flags = 0o1i32 | 0o100i32 | 0o1000i32
                }
                dst_nameN = dst_name;
                if (*archive_handle).ah_flags & (1i32 << 9i32) as libc::c_uint != 0 {
                  /* rpm-style temp file name */
                  dst_nameN = crate::libbb::xfuncs_printf::xasprintf(
                    b"%s;%x\x00" as *const u8 as *const libc::c_char,
                    dst_name,
                    getpid(),
                  )
                }
                dst_fd = crate::libbb::xfuncs_printf::xopen3(
                  dst_nameN,
                  flags,
                  (*file_header).mode as libc::c_int,
                );
                crate::libbb::copyfd::bb_copyfd_exact_size(
                  (*archive_handle).src_fd,
                  dst_fd,
                  (*file_header).size,
                );
                close(dst_fd);
                if (*archive_handle).ah_flags & (1i32 << 9i32) as libc::c_uint != 0 {
                  crate::libbb::xfuncs_printf::xrename(dst_nameN, dst_name);
                  free(dst_nameN as *mut libc::c_void);
                }
              }
              16384 => {
                res = mkdir(dst_name, (*file_header).mode);
                if res != 0 && *bb_errno != 21i32 && *bb_errno != 17i32 {
                  crate::libbb::perror_msg::bb_perror_msg(
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
                crate::archival::libarchive::unsafe_symlink_target::create_or_remember_link(
                  &mut (*archive_handle).link_placeholders,
                  (*file_header).link_target,
                  dst_name,
                  0,
                );
              }
              49152 | 24576 | 8192 | 4096 => {
                res = mknod(dst_name, (*file_header).mode, (*file_header).device);
                if res != 0 {
                  crate::libbb::perror_msg::bb_perror_msg(
                    b"can\'t create node %s\x00" as *const u8 as *const libc::c_char,
                    dst_name,
                  );
                }
              }
              _ => {
                crate::libbb::verror_msg::bb_simple_error_msg_and_die(
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
                    let mut grp: *mut group =
                      crate::libpwdgrp::pwd_grp::bb_internal_getgrnam((*file_header).tar__gname);
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
              if (*archive_handle).ah_flags & (1i32 << 0) as libc::c_uint != 0 {
                let mut t: [timeval; 2] = [timeval {
                  tv_sec: 0,
                  tv_usec: 0,
                }; 2];
                t[0].tv_sec = (*file_header).mtime;
                t[1].tv_sec = t[0].tv_sec;
                t[0].tv_usec = 0 as suseconds_t;
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

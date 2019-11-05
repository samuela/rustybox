use libc;
extern "C" {
  pub type __dirstream;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn lchown(__file: *const libc::c_char, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
  #[no_mangle]
  fn link(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn symlink(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn unlink(__name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn opendir(__name: *const libc::c_char) -> *mut DIR;
  #[no_mangle]
  fn closedir(__dirp: *mut DIR) -> libc::c_int;
  #[no_mangle]
  fn readdir(__dirp: *mut DIR) -> *mut dirent;
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  static mut stderr: *mut FILE;
  #[no_mangle]
  fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn chown(__file: *const libc::c_char, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
  #[no_mangle]
  fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
  #[no_mangle]
  fn umask(__mask: __mode_t) -> __mode_t;
  #[no_mangle]
  fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
  #[no_mangle]
  fn mknod(__path: *const libc::c_char, __mode: __mode_t, __dev: __dev_t) -> libc::c_int;
  #[no_mangle]
  fn utimes(__file: *const libc::c_char, __tvp: *const timeval) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
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
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn xmalloc_readlink_or_warn(path: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_ask_y_confirmation() -> libc::c_int;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  fn bb_copyfd_eof(fd1: libc::c_int, fd2: libc::c_int) -> off_t;
  #[no_mangle]
  fn open3_or_warn(
    pathname: *const libc::c_char,
    flags: libc::c_int,
    mode: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn open_or_warn(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn add_to_ino_dev_hashtable(statbuf: *const stat, name: *const libc::c_char);
  #[no_mangle]
  fn is_in_ino_dev_hashtable(statbuf: *const stat) -> *mut libc::c_char;
  #[no_mangle]
  fn concat_path_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn concat_subpath_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;
}
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
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
use crate::librb::smallint;

use crate::librb::off_t;
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
use crate::librb::mode_t;
use crate::librb::stat;
use crate::librb::timespec;
use crate::librb::timeval;



use crate::librb::FILE;
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
 * Mini copy_file implementation for busybox
 *
 * Copyright (C) 2001 by Matt Kraai <kraai@alumni.carnegiemellon.edu>
 * SELinux support by Yuichi Nakamura <ynakam@hitachisoft.jp>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
// FEATURE_NON_POSIX_CP:
//
// POSIX: if exists and -i, ask (w/o -i assume yes).
// Then open w/o EXCL (yes, not unlink!).
// If open still fails and -f, try unlink, then try open again.
// Result: a mess:
// If dest is a (sym)link, we overwrite link destination!
// (or fail, if it points to dir/nonexistent location/etc).
// This is strange, but POSIX-correct.
// coreutils cp has --remove-destination to override this...
/* Called if open of destination, link creation etc fails.
 * errno must be set to relevant value ("why we cannot create dest?")
 * to give reasonable error message */
unsafe extern "C" fn ask_and_unlink(
  mut dest: *const libc::c_char,
  mut flags: libc::c_int,
) -> libc::c_int {
  let mut e: libc::c_int = *bb_errno;
  // else: act as if -f is always in effect.
  // We don't want "can't create" msg, we want unlink to be done
  // (silently unless -i). Why? POSIX cp usually succeeds with
  // O_TRUNC open of existing file, and user is left ignorantly happy.
  // With above block unconditionally enabled, non-POSIX cp
  // will complain a lot more than POSIX one.
  /* TODO: maybe we should do it only if ctty is present? */
  if flags & FILEUTILS_INTERACTIVE as libc::c_int != 0 {
    // We would not do POSIX insanity. -i asks,
    // then _unlinks_ the offender. Presto.
    // (No "opening without O_EXCL", no "unlink only if -f")
    // Or else we will end up having 3 open()s!
    fprintf(
      stderr,
      b"%s: overwrite \'%s\'? \x00" as *const u8 as *const libc::c_char,
      applet_name,
      dest,
    );
    if bb_ask_y_confirmation() == 0 {
      return 0i32;
    }
    /* not allowed to overwrite */
  } /* do not use errno from unlink */
  if unlink(dest) < 0i32 {
    *bb_errno = e;
    bb_perror_msg(
      b"can\'t create \'%s\'\x00" as *const u8 as *const libc::c_char,
      dest,
    );
    return -1i32;
    /* error */
  }
  if flags & FILEUTILS_RMDEST as libc::c_int != 0 {
    if flags & FILEUTILS_VERBOSE as libc::c_int != 0 {
      printf(
        b"removed \'%s\'\n\x00" as *const u8 as *const libc::c_char,
        dest,
      );
    }
  }
  return 1i32;
  /* ok (to try again) */
}
/* Return:
 * -1 error, copy not made
 *  0 copy is made or user answered "no" in interactive mode
 *    (failures to preserve mode/owner/times are not reported in exit code)
 */
#[no_mangle]
pub unsafe extern "C" fn copy_file(
  mut source: *const libc::c_char,
  mut dest: *const libc::c_char,
  mut flags: libc::c_int,
) -> libc::c_int {
  let mut lf: Option<
    unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int,
  > = None;
  let mut current_block: u64;
  /* This is a recursive function, try to minimize stack usage */
  /* NB: each struct stat is ~100 bytes */
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
  let mut retval: smallint = 0i32 as smallint;
  let mut dest_exists: smallint = 0i32 as smallint;
  let mut ovr: smallint = 0;
  /* Inverse of cp -d ("cp without -d") */
  if (if flags & FILEUTILS_DEREFERENCE as libc::c_int + FILEUTILS_DEREFERENCE_L0 as libc::c_int != 0
  {
    Some(stat as unsafe extern "C" fn(_: *const libc::c_char, _: *mut stat) -> libc::c_int)
  } else {
    Some(lstat as unsafe extern "C" fn(_: *const libc::c_char, _: *mut stat) -> libc::c_int)
  })
  .expect("non-null function pointer")(source, &mut source_stat)
    < 0i32
  {
    /* This may be a dangling symlink.
     * Making [sym]links to dangling symlinks works, so... */
    if !(flags & (FILEUTILS_MAKE_SOFTLINK as libc::c_int | FILEUTILS_MAKE_HARDLINK as libc::c_int)
      != 0)
    {
      bb_perror_msg(
        b"can\'t stat \'%s\'\x00" as *const u8 as *const libc::c_char,
        source,
      );
      return -1i32;
    }
  } else {
    if lstat(dest, &mut dest_stat) < 0i32 {
      if *bb_errno != 2i32 {
        bb_perror_msg(
          b"can\'t stat \'%s\'\x00" as *const u8 as *const libc::c_char,
          dest,
        );
        return -1i32;
      }
    } else {
      if source_stat.st_dev == dest_stat.st_dev && source_stat.st_ino == dest_stat.st_ino {
        bb_error_msg(
          b"\'%s\' and \'%s\' are the same file\x00" as *const u8 as *const libc::c_char,
          source,
          dest,
        );
        return -1i32;
      }
      dest_exists = 1i32 as smallint
    }
    if source_stat.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint {
      let mut dp: *mut DIR = 0 as *mut DIR;
      let mut tp: *const libc::c_char = 0 as *const libc::c_char;
      let mut d: *mut dirent = 0 as *mut dirent;
      let mut saved_umask: mode_t = 0i32 as mode_t;
      if flags & FILEUTILS_RECUR as libc::c_int == 0 {
        bb_error_msg(
          b"omitting directory \'%s\'\x00" as *const u8 as *const libc::c_char,
          source,
        );
        return -1i32;
      }
      /* Did we ever create source ourself before? */
      tp = is_in_ino_dev_hashtable(&mut source_stat);
      if !tp.is_null() {
        /* We did! it's a recursion! man the lifeboats... */
        bb_error_msg(
          b"recursion detected, omitting directory \'%s\'\x00" as *const u8 as *const libc::c_char,
          source,
        );
        return -1i32;
      }
      if dest_exists != 0 {
        if !(dest_stat.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint) {
          bb_error_msg(
            b"target \'%s\' is not a directory\x00" as *const u8 as *const libc::c_char,
            dest,
          );
          return -1i32;
        }
      /* race here: user can substitute a symlink between
       * this check and actual creation of files inside dest */
      } else {
        /* Create DEST */
        let mut mode: mode_t = 0;
        saved_umask = umask(0i32 as __mode_t);
        mode = source_stat.st_mode;
        if flags & FILEUTILS_PRESERVE_STATUS as libc::c_int == 0 {
          mode = source_stat.st_mode & !saved_umask
        }
        /* Allow owner to access new dir (at least for now) */
        mode |= (0o400i32 | 0o200i32 | 0o100i32) as libc::c_uint;
        if mkdir(dest, mode) < 0i32 {
          umask(saved_umask);
          bb_perror_msg(
            b"can\'t create directory \'%s\'\x00" as *const u8 as *const libc::c_char,
            dest,
          );
          return -1i32;
        }
        umask(saved_umask);
        /* need stat info for add_to_ino_dev_hashtable */
        if lstat(dest, &mut dest_stat) < 0i32 {
          bb_perror_msg(
            b"can\'t stat \'%s\'\x00" as *const u8 as *const libc::c_char,
            dest,
          );
          return -1i32;
        }
      }
      /* remember (dev,inode) of each created dir.
       * NULL: name is not remembered */
      add_to_ino_dev_hashtable(&mut dest_stat, 0 as *const libc::c_char);
      /* Recursively copy files in SOURCE */
      dp = opendir(source);
      if dp.is_null() {
        retval = -1i32 as smallint
      } else {
        loop {
          d = readdir(dp);
          if d.is_null() {
            break;
          }
          let mut new_source: *mut libc::c_char = 0 as *mut libc::c_char;
          let mut new_dest: *mut libc::c_char = 0 as *mut libc::c_char;
          new_source = concat_subpath_file(source, (*d).d_name.as_mut_ptr());
          if new_source.is_null() {
            continue;
          }
          new_dest = concat_path_file(dest, (*d).d_name.as_mut_ptr());
          if copy_file(
            new_source,
            new_dest,
            flags & !(FILEUTILS_DEREFERENCE_L0 as libc::c_int),
          ) < 0i32
          {
            retval = -1i32 as smallint
          }
          free(new_source as *mut libc::c_void);
          free(new_dest as *mut libc::c_void);
        }
        closedir(dp);
        if dest_exists == 0 && chmod(dest, source_stat.st_mode & !saved_umask) < 0i32 {
          bb_perror_msg(
            b"can\'t preserve %s of \'%s\'\x00" as *const u8 as *const libc::c_char,
            b"permissions\x00" as *const u8 as *const libc::c_char,
            dest,
          );
          /* retval = -1; - WRONG! copy *WAS* made */
        }
      }
      current_block = 16682393968998137680;
    } else {
      if dest_exists != 0 {
        if flags & FILEUTILS_UPDATE as libc::c_int != 0 {
          if source_stat.st_mtim.tv_sec <= dest_stat.st_mtim.tv_sec {
            return 0i32;
            /* source file must be newer */
          }
        }
        if flags & FILEUTILS_RMDEST as libc::c_int != 0 {
          ovr = ask_and_unlink(dest, flags) as smallint;
          if ovr as libc::c_int <= 0i32 {
            return ovr as libc::c_int;
          }
          dest_exists = 0i32 as smallint
        }
      }
      if flags & (FILEUTILS_MAKE_SOFTLINK as libc::c_int | FILEUTILS_MAKE_HARDLINK as libc::c_int)
        != 0
      {
        lf = None;
        current_block = 8153042539075492440;
      } else {
        if flags & FILEUTILS_RECUR as libc::c_int == 0
          || source_stat.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint
        {
          /* DEREF uses stat, which never returns S_ISLNK() == true.
           * So the below is never true: */
          /* || (FLAGS_DEREF && S_ISLNK(source_stat.st_mode)) */
          let mut src_fd: libc::c_int = 0;
          let mut dst_fd: libc::c_int = 0;
          let mut new_mode: mode_t = 0;
          if flags & FILEUTILS_DEREFERENCE as libc::c_int + FILEUTILS_DEREFERENCE_L0 as libc::c_int
            == 0
            && source_stat.st_mode & 0o170000i32 as libc::c_uint == 0o120000i32 as libc::c_uint
          {
            current_block = 10550084591834429986;
          } else {
            if 1i32 != 0
              && flags
                & FILEUTILS_DEREFERENCE as libc::c_int + FILEUTILS_DEREFERENCE_L0 as libc::c_int
                == 0
            {
              let mut link_target: *const libc::c_char = 0 as *const libc::c_char;
              link_target = is_in_ino_dev_hashtable(&mut source_stat);
              if !link_target.is_null() {
                if link(link_target, dest) < 0i32 {
                  ovr = ask_and_unlink(dest, flags) as smallint;
                  if ovr as libc::c_int <= 0i32 {
                    return ovr as libc::c_int;
                  }
                  if link(link_target, dest) < 0i32 {
                    bb_perror_msg(
                      b"can\'t create link \'%s\'\x00" as *const u8 as *const libc::c_char,
                      dest,
                    );
                    return -1i32;
                  }
                }
                return 0i32;
              }
              add_to_ino_dev_hashtable(&mut source_stat, dest);
            }
            src_fd = open_or_warn(source, 0i32);
            if src_fd < 0i32 {
              return -1i32;
            }
            /* Do not try to open with weird mode fields */
            new_mode = source_stat.st_mode; /* POSIX, and not "cp -i" */
            if !(source_stat.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint) {
              new_mode = 0o666i32 as mode_t
            }
            if 1i32 != 0 || flags & FILEUTILS_INTERACTIVE as libc::c_int != 0 {
              /*
               * O_CREAT|O_EXCL: require that file did not exist before creation
               */
              dst_fd = open(dest, 0o1i32 | 0o100i32 | 0o200i32, new_mode)
            } else {
              /*
               * O_CREAT|O_TRUNC: create, or truncate (security problem versus (sym)link attacks)
               */
              dst_fd = open(dest, 0o1i32 | 0o100i32 | 0o1000i32, new_mode)
            }
            if dst_fd == -1i32 {
              ovr = ask_and_unlink(dest, flags) as smallint;
              if ovr as libc::c_int <= 0i32 {
                close(src_fd);
                return ovr as libc::c_int;
              }
              /* It shouldn't exist. If it exists, do not open (symlink attack?) */
              dst_fd = open3_or_warn(dest, 0o1i32 | 0o100i32 | 0o200i32, new_mode as libc::c_int);
              if dst_fd < 0i32 {
                close(src_fd);
                return -1i32;
              }
            }
            if flags & FILEUTILS_REFLINK as libc::c_int != 0 {
              retval = ioctl(
                dst_fd,
                (1u32 << 0i32 + 8i32 + 8i32 + 14i32
                  | (0x94i32 << 0i32 + 8i32) as libc::c_uint
                  | (9i32 << 0i32) as libc::c_uint) as libc::c_ulong
                  | (::std::mem::size_of::<libc::c_int>() as libc::c_ulong) << 0i32 + 8i32 + 8i32,
                src_fd,
              ) as smallint;
              if retval as libc::c_int == 0i32 {
                current_block = 10903821241939442503;
              } else if flags & FILEUTILS_REFLINK_ALWAYS as libc::c_int != 0 {
                bb_perror_msg(
                  b"failed to clone \'%s\' from \'%s\'\x00" as *const u8 as *const libc::c_char,
                  dest,
                  source,
                );
                current_block = 10903821241939442503;
              } else {
                /* reflink did not work */
                /* fall through to standard copy */
                retval = 0i32 as smallint;
                current_block = 3921975509081277429;
              }
            } else {
              current_block = 3921975509081277429;
            }
            match current_block {
              3921975509081277429 => {
                if bb_copyfd_eof(src_fd, dst_fd) == -1i32 as libc::c_long {
                  retval = -1i32 as smallint
                }
              }
              _ => {}
            }
            /* Careful with writing... */
            if close(dst_fd) < 0i32 {
              bb_perror_msg(
                b"error writing to \'%s\'\x00" as *const u8 as *const libc::c_char,
                dest,
              );
              retval = -1i32 as smallint
            }
            /* ...but read size is already checked by bb_copyfd_eof */
            close(src_fd);
            /* "cp /dev/something new_file" should not
             * copy mode of /dev/something */
            if !(source_stat.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint) {
              return retval as libc::c_int;
            }
            current_block = 16682393968998137680;
          }
        } else {
          current_block = 10550084591834429986;
        }
        match current_block {
          16682393968998137680 => {}
          _ =>
          /* "cp -d symlink dst": create a link */
          /* Source is a symlink or a special file */
          /* We are lazy here, a bit lax with races... */
          {
            if dest_exists != 0 {
              *bb_errno = 17i32;
              ovr = ask_and_unlink(dest, flags) as smallint;
              if ovr as libc::c_int <= 0i32 {
                return ovr as libc::c_int;
              }
            }
            if source_stat.st_mode & 0o170000i32 as libc::c_uint == 0o120000i32 as libc::c_uint {
              let mut lpath: *mut libc::c_char = xmalloc_readlink_or_warn(source);
              if !lpath.is_null() {
                let mut r: libc::c_int = symlink(lpath, dest);
                if r < 0i32 {
                  /* shared message */
                  bb_perror_msg(
                    b"can\'t create %slink \'%s\' to \'%s\'\x00" as *const u8
                      as *const libc::c_char,
                    b"sym\x00" as *const u8 as *const libc::c_char,
                    dest,
                    lpath,
                  );
                  free(lpath as *mut libc::c_void);
                  return -1i32;
                }
                free(lpath as *mut libc::c_void);
                if flags & FILEUTILS_PRESERVE_STATUS as libc::c_int != 0 {
                  if lchown(dest, source_stat.st_uid, source_stat.st_gid) < 0i32 {
                    bb_perror_msg(
                      b"can\'t preserve %s of \'%s\'\x00" as *const u8 as *const libc::c_char,
                      b"ownership\x00" as *const u8 as *const libc::c_char,
                      dest,
                    );
                  }
                }
              }
              current_block = 7773253009131997152;
            } else {
              if source_stat.st_mode & 0o170000i32 as libc::c_uint == 0o60000i32 as libc::c_uint
                || source_stat.st_mode & 0o170000i32 as libc::c_uint == 0o20000i32 as libc::c_uint
                || source_stat.st_mode & 0o170000i32 as libc::c_uint == 0o140000i32 as libc::c_uint
                || source_stat.st_mode & 0o170000i32 as libc::c_uint == 0o10000i32 as libc::c_uint
              {
                if mknod(dest, source_stat.st_mode, source_stat.st_rdev) < 0i32 {
                  bb_perror_msg(
                    b"can\'t create \'%s\'\x00" as *const u8 as *const libc::c_char,
                    dest,
                  );
                  return -1i32;
                }
              } else {
                bb_error_msg(
                  b"unrecognized file \'%s\' with mode %x\x00" as *const u8 as *const libc::c_char,
                  source,
                  source_stat.st_mode,
                );
                return -1i32;
              }
              current_block = 16682393968998137680;
            }
          }
        }
      }
    }
    match current_block {
      8153042539075492440 => {}
      _ => {
        match current_block {
          16682393968998137680 => {
            if flags & FILEUTILS_PRESERVE_STATUS as libc::c_int != 0 {
              /* Cannot happen: */
              /* && !(flags & (FILEUTILS_MAKE_SOFTLINK|FILEUTILS_MAKE_HARDLINK)) */
              let mut times: [timeval; 2] = [timeval {
                tv_sec: 0,
                tv_usec: 0,
              }; 2];
              times[0].tv_sec = source_stat.st_mtim.tv_sec;
              times[1].tv_sec = times[0].tv_sec;
              times[0].tv_usec = 0i32 as __suseconds_t;
              times[1].tv_usec = times[0].tv_usec;
              /* BTW, utimes sets usec-precision time - just FYI */
              if utimes(dest, times.as_mut_ptr() as *const timeval) < 0i32 {
                bb_perror_msg(
                  b"can\'t preserve %s of \'%s\'\x00" as *const u8 as *const libc::c_char,
                  b"times\x00" as *const u8 as *const libc::c_char,
                  dest,
                );
              }
              if chown(dest, source_stat.st_uid, source_stat.st_gid) < 0i32 {
                source_stat.st_mode &= !(0o4000i32 | 0o2000i32) as libc::c_uint;
                bb_perror_msg(
                  b"can\'t preserve %s of \'%s\'\x00" as *const u8 as *const libc::c_char,
                  b"ownership\x00" as *const u8 as *const libc::c_char,
                  dest,
                );
              }
              if chmod(dest, source_stat.st_mode) < 0i32 {
                bb_perror_msg(
                  b"can\'t preserve %s of \'%s\'\x00" as *const u8 as *const libc::c_char,
                  b"permissions\x00" as *const u8 as *const libc::c_char,
                  dest,
                );
              }
            }
          }
          _ => {}
        }
        /* _Not_ jumping to preserve_mode_ugid_time:
         * symlinks don't have those */
        if flags & FILEUTILS_VERBOSE as libc::c_int != 0 {
          printf(
            b"\'%s\' -> \'%s\'\n\x00" as *const u8 as *const libc::c_char,
            source,
            dest,
          );
        }
        return retval as libc::c_int;
      }
    }
  }
  /* Hmm... maybe
   * if (DEREF && MAKE_SOFTLINK) source = realpath(source) ?
   * (but realpath returns NULL on dangling symlinks...) */
  lf = if flags & FILEUTILS_MAKE_SOFTLINK as libc::c_int != 0 {
    Some(
      symlink
        as unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int,
    )
  } else {
    Some(
      link as unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int,
    )
  };
  if lf.expect("non-null function pointer")(source, dest) < 0i32 {
    ovr = ask_and_unlink(dest, flags) as smallint;
    if ovr as libc::c_int <= 0i32 {
      return ovr as libc::c_int;
    }
    if lf.expect("non-null function pointer")(source, dest) < 0i32 {
      bb_perror_msg(
        b"can\'t create link \'%s\'\x00" as *const u8 as *const libc::c_char,
        dest,
      );
      return -1i32;
    }
  }
  /* _Not_ jumping to preserve_mode_ugid_time:
   * (sym)links don't have those */
  return 0i32;
}

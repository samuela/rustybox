use crate::archival::libarchive::bb_archive::file_header_t;
use crate::archival::libarchive::bb_archive::hardlinks_t;
use crate::libbb::llist::llist_t;
use crate::librb::bb_uidgid_t;
use crate::librb::signal::__sighandler_t;
use crate::librb::smallint;
use crate::librb::uoff_t;
use libc;
use libc::off_t;
use libc::pid_t;

extern "C" {

  #[no_mangle]
  fn putenv(__string: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn vfork() -> libc::c_int;
  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
  #[no_mangle]
  fn execl(__path: *const libc::c_char, __arg: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_copyfd_exact_size(fd1: libc::c_int, fd2: libc::c_int, size: off_t);
  #[no_mangle]
  fn xdup2(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn bb_unsetenv_and_free(key: *mut libc::c_char);
  #[no_mangle]
  fn xpipe(filedes: *mut libc::c_int);
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn xfork() -> pid_t;
  #[no_mangle]
  fn wait_for_exitstatus(pid: pid_t) -> libc::c_int;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_die_memory_exhausted() -> !;
}

use crate::archival::libarchive::bb_archive::archive_handle_t;

pub type C2RustUnnamed = libc::c_uint;
pub const TAR_MAX: C2RustUnnamed = 8;
pub const TAR_GID: C2RustUnnamed = 7;
pub const TAR_UID: C2RustUnnamed = 6;
pub const TAR_SIZE: C2RustUnnamed = 5;
pub const TAR_GNAME: C2RustUnnamed = 4;
pub const TAR_UNAME: C2RustUnnamed = 3;
pub const TAR_REALNAME: C2RustUnnamed = 2;
pub const TAR_FILENAME: C2RustUnnamed = 1;
//TAR_FILETYPE,
pub const TAR_MODE: C2RustUnnamed = 0;

/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
static mut tar_var: [*const libc::c_char; 8] = [
  b"MODE\x00" as *const u8 as *const libc::c_char,
  b"FILENAME\x00" as *const u8 as *const libc::c_char,
  b"REALNAME\x00" as *const u8 as *const libc::c_char,
  b"UNAME\x00" as *const u8 as *const libc::c_char,
  b"GNAME\x00" as *const u8 as *const libc::c_char,
  b"SIZE\x00" as *const u8 as *const libc::c_char,
  b"UID\x00" as *const u8 as *const libc::c_char,
  b"GID\x00" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn xputenv(mut str: *mut libc::c_char) {
  if putenv(str) != 0 {
    bb_die_memory_exhausted();
  };
}
unsafe extern "C" fn str2env(
  mut env: *mut *mut libc::c_char,
  mut idx: libc::c_int,
  mut str: *const libc::c_char,
) {
  let ref mut fresh0 = *env.offset(idx as isize);
  *fresh0 = xasprintf(
    b"TAR_%s=%s\x00" as *const u8 as *const libc::c_char,
    tar_var[idx as usize],
    str,
  );
  xputenv(*env.offset(idx as isize));
}
unsafe extern "C" fn dec2env(
  mut env: *mut *mut libc::c_char,
  mut idx: libc::c_int,
  mut val: libc::c_ulonglong,
) {
  let ref mut fresh1 = *env.offset(idx as isize);
  *fresh1 = xasprintf(
    b"TAR_%s=%llu\x00" as *const u8 as *const libc::c_char,
    tar_var[idx as usize],
    val,
  );
  xputenv(*env.offset(idx as isize));
}
unsafe extern "C" fn oct2env(
  mut env: *mut *mut libc::c_char,
  mut idx: libc::c_int,
  mut val: libc::c_ulong,
) {
  let ref mut fresh2 = *env.offset(idx as isize);
  *fresh2 = xasprintf(
    b"TAR_%s=%lo\x00" as *const u8 as *const libc::c_char,
    tar_var[idx as usize],
    val,
  );
  xputenv(*env.offset(idx as isize));
}
#[no_mangle]
pub unsafe extern "C" fn data_extract_to_command(mut archive_handle: *mut archive_handle_t) {
  let mut file_header: *mut file_header_t = (*archive_handle).file_header;
  /* do we need this? ENABLE_FEATURE_TAR_SELINUX */
  if (*file_header).mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint {
    let mut pid: pid_t = 0;
    let mut p: [libc::c_int; 2] = [0; 2];
    let mut status: libc::c_int = 0;
    let mut tar_env: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    memset(
      tar_env.as_mut_ptr() as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<[*mut libc::c_char; 8]>() as libc::c_ulong,
    );
    xpipe(p.as_mut_ptr());
    pid = if 1i32 != 0 {
      xfork()
    } else {
      ({
        let mut bb__xvfork_pid: pid_t = vfork();
        if bb__xvfork_pid < 0i32 {
          bb_simple_perror_msg_and_die(b"vfork\x00" as *const u8 as *const libc::c_char);
        }
        bb__xvfork_pid
      })
    };
    if pid == 0i32 {
      /* Child */
      /* str2env(tar_env, TAR_FILETYPE, "f"); - parent should do it once */
      oct2env(
        tar_env.as_mut_ptr(),
        TAR_MODE as libc::c_int,
        (*file_header).mode as libc::c_ulong,
      );
      str2env(
        tar_env.as_mut_ptr(),
        TAR_FILENAME as libc::c_int,
        (*file_header).name,
      );
      str2env(
        tar_env.as_mut_ptr(),
        TAR_REALNAME as libc::c_int,
        (*file_header).name,
      );
      str2env(
        tar_env.as_mut_ptr(),
        TAR_UNAME as libc::c_int,
        (*file_header).tar__uname,
      );
      str2env(
        tar_env.as_mut_ptr(),
        TAR_GNAME as libc::c_int,
        (*file_header).tar__gname,
      );
      dec2env(
        tar_env.as_mut_ptr(),
        TAR_SIZE as libc::c_int,
        (*file_header).size as libc::c_ulonglong,
      );
      dec2env(
        tar_env.as_mut_ptr(),
        TAR_UID as libc::c_int,
        (*file_header).uid as libc::c_ulonglong,
      );
      dec2env(
        tar_env.as_mut_ptr(),
        TAR_GID as libc::c_int,
        (*file_header).gid as libc::c_ulonglong,
      );
      close(p[1]);
      xdup2(p[0], 0i32);
      signal(13i32, None);
      execl(
        (*archive_handle).tar__to_command_shell,
        (*archive_handle).tar__to_command_shell,
        b"-c\x00" as *const u8 as *const libc::c_char,
        (*archive_handle).tar__to_command,
        0 as *mut libc::c_char,
      );
      bb_perror_msg_and_die(
        b"can\'t execute \'%s\'\x00" as *const u8 as *const libc::c_char,
        (*archive_handle).tar__to_command_shell,
      );
    }
    close(p[0]);
    /* Our caller is expected to do signal(SIGPIPE, SIG_IGN)
     * so that we don't die if child don't read all the input: */
    bb_copyfd_exact_size((*archive_handle).src_fd, p[1], -(*file_header).size);
    close(p[1]);
    status = wait_for_exitstatus(pid);
    if status & 0x7fi32 == 0i32 && (status & 0xff00i32) >> 8i32 != 0 {
      bb_error_msg_and_die(
        b"\'%s\' returned status %d\x00" as *const u8 as *const libc::c_char,
        (*archive_handle).tar__to_command,
        (status & 0xff00i32) >> 8i32,
      );
    }
    if ((status & 0x7fi32) + 1i32) as libc::c_schar as libc::c_int >> 1i32 > 0i32 {
      bb_error_msg_and_die(
        b"\'%s\' terminated by signal %d\x00" as *const u8 as *const libc::c_char,
        (*archive_handle).tar__to_command,
        status & 0x7fi32,
      );
    }
    if 1i32 == 0 {
      let mut i: libc::c_int = 0;
      i = 0i32;
      while i < TAR_MAX as libc::c_int {
        if !tar_env[i as usize].is_null() {
          bb_unsetenv_and_free(tar_env[i as usize]);
        }
        i += 1
      }
    }
  };
  /* ENABLE_FEATURE_TAR_SELINUX */
}

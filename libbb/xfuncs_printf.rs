use libc;
extern "C" {
  pub type __dirstream;
  pub type sockaddr_x25;
  pub type sockaddr_un;
  pub type sockaddr_ns;
  pub type sockaddr_iso;
  pub type sockaddr_ipx;
  pub type sockaddr_inarp;
  pub type sockaddr_eon;
  pub type sockaddr_dl;
  pub type sockaddr_ax25;
  pub type sockaddr_at;
  #[no_mangle]
  fn rand() -> libc::c_int;
  #[no_mangle]
  fn srand(__seed: libc::c_uint);
  #[no_mangle]
  fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn setenv(
    __name: *const libc::c_char,
    __value: *const libc::c_char,
    __replace: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn _exit(_: libc::c_int) -> !;
  #[no_mangle]
  fn getpid() -> __pid_t;
  #[no_mangle]
  fn geteuid() -> __uid_t;
  #[no_mangle]
  fn getegid() -> __gid_t;
  #[no_mangle]
  fn setuid(__uid: __uid_t) -> libc::c_int;
  #[no_mangle]
  fn seteuid(__uid: __uid_t) -> libc::c_int;
  #[no_mangle]
  fn setgid(__gid: __gid_t) -> libc::c_int;
  #[no_mangle]
  fn setegid(__gid: __gid_t) -> libc::c_int;
  #[no_mangle]
  fn fork() -> __pid_t;
  #[no_mangle]
  fn vfork() -> libc::c_int;
  #[no_mangle]
  fn ttyname_r(__fd: libc::c_int, __buf: *mut libc::c_char, __buflen: size_t) -> libc::c_int;
  #[no_mangle]
  fn unlink(__name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn chroot(__path: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn opendir(__name: *const libc::c_char) -> *mut DIR;
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn socket(__domain: libc::c_int, __type: libc::c_int, __protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn bind(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG, __len: socklen_t) -> libc::c_int;
  #[no_mangle]
  fn sendto(
    __fd: libc::c_int,
    __buf: *const libc::c_void,
    __n: size_t,
    __flags: libc::c_int,
    __addr: __CONST_SOCKADDR_ARG,
    __addr_len: socklen_t,
  ) -> ssize_t;
  #[no_mangle]
  fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
  #[no_mangle]
  static mut stdout: *mut _IO_FILE;
  #[no_mangle]
  fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fflush(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn vasprintf(
    __ptr: *mut *mut libc::c_char,
    __f: *const libc::c_char,
    __arg: ::std::ffi::VaList,
  ) -> libc::c_int;
  #[no_mangle]
  fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn ferror_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fileno_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn fchdir(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn chdir(__path: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn lseek(__fd: libc::c_int, __offset: __off64_t, __whence: libc::c_int) -> __off64_t;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn mempcpy(
    __dest: *mut libc::c_void,
    __src: *const libc::c_void,
    __n: size_t,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
  #[no_mangle]
  fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn monotonic_us() -> libc::c_ulonglong;
  /* must be directly before hash[] */
  /* always correctly aligned for uint64_t */
  /* TLS benefits from knowing that sha1 and sha256 share these. Give them "agnostic" names too */
  /*unsigned last_eta;*/
  /* Some older linkers don't perform string merging, we used to have common strings
   * as global arrays to do it by hand. But:
   * (1) newer linkers do it themselves,
   * (2) however, they DONT merge string constants with global arrays,
   * even if the value is the same (!). Thus global arrays actually
   * increased size a bit: for example, "/etc/passwd" string from libc
   * wasn't merged with bb_path_passwd_file[] array!
   * Therefore now we use #defines.
   */
  /* "BusyBox vN.N.N (timestamp or extra_version)" */
  #[no_mangle]
  static bb_msg_memory_exhausted: [libc::c_char; 0];
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_copyfd_eof(fd1: libc::c_int, fd2: libc::c_int) -> off_t;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn kill_myself_with_sig(sig: libc::c_int) -> !;
  #[no_mangle]
  fn strerror(_: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn xfunc_die() -> !;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  static bb_msg_standard_output: [libc::c_char; 0];
  #[no_mangle]
  fn wait_for_exitstatus(pid: pid_t) -> libc::c_int;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_verror_msg(s: *const libc::c_char, p: ::std::ffi::VaList, strerr: *const libc::c_char);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
  pub gp_offset: libc::c_uint,
  pub fp_offset: libc::c_uint,
  pub overflow_arg_area: *mut libc::c_void,
  pub reg_save_area: *mut libc::c_void,
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
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
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
pub type pid_t = __pid_t;
pub type socklen_t = __socklen_t;
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
  pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
pub union __CONST_SOCKADDR_ARG {
  pub __sockaddr__: *const sockaddr,
  pub __sockaddr_at__: *const sockaddr_at,
  pub __sockaddr_ax25__: *const sockaddr_ax25,
  pub __sockaddr_dl__: *const sockaddr_dl,
  pub __sockaddr_eon__: *const sockaddr_eon,
  pub __sockaddr_in__: *const sockaddr_in,
  pub __sockaddr_in6__: *const sockaddr_in6,
  pub __sockaddr_inarp__: *const sockaddr_inarp,
  pub __sockaddr_ipx__: *const sockaddr_ipx,
  pub __sockaddr_iso__: *const sockaddr_iso,
  pub __sockaddr_ns__: *const sockaddr_ns,
  pub __sockaddr_un__: *const sockaddr_un,
  pub __sockaddr_x25__: *const sockaddr_x25,
}
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
pub type va_list = __builtin_va_list;
/* vi: set sw=4 ts=4: */
/*
 * Utility routines.
 *
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 * Copyright (C) 2006 Rob Landley
 * Copyright (C) 2006 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* We need to have separate xfuncs.c and xfuncs_printf.c because
 * with current linkers, even with section garbage collection,
 * if *.o module references any of XXXprintf functions, you pull in
 * entire printf machinery. Even if you do not use the function
 * which uses XXXprintf.
 *
 * xfuncs.c contains functions (not necessarily xfuncs)
 * which do not pull in printf, directly or indirectly.
 * xfunc_printf.c contains those which do.
 */
/* All the functions starting with "x" call bb_error_msg_and_die() if they
 * fail, so callers never need to check for errors.  If it returned, it
 * succeeded. */
#[no_mangle]
pub unsafe extern "C" fn bb_die_memory_exhausted() -> ! {
  bb_simple_error_msg_and_die(bb_msg_memory_exhausted.as_ptr());
}
/* dmalloc provides variants of these that do abort() on failure.
 * Since dmalloc's prototypes overwrite the impls here as they are
 * included after these prototypes in libbb.h, all is well.
 */
// Warn if we can't allocate size bytes of memory.
#[no_mangle]
pub unsafe extern "C" fn malloc_or_warn(mut size: size_t) -> *mut libc::c_void {
  let mut ptr: *mut libc::c_void = malloc(size);
  if ptr.is_null() && size != 0i32 as libc::c_ulong {
    bb_simple_error_msg(bb_msg_memory_exhausted.as_ptr());
  }
  return ptr;
}
// Die if we can't allocate size bytes of memory.
#[no_mangle]
pub unsafe extern "C" fn xmalloc(mut size: size_t) -> *mut libc::c_void {
  let mut ptr: *mut libc::c_void = malloc(size);
  if ptr.is_null() && size != 0i32 as libc::c_ulong {
    bb_die_memory_exhausted();
  }
  return ptr;
}
// Die if we can't resize previously allocated memory.  (This returns a pointer
// to the new memory, which may or may not be the same as the old memory.
// It'll copy the contents to a new chunk and free the old one if necessary.)
#[no_mangle]
pub unsafe extern "C" fn xrealloc(
  mut ptr: *mut libc::c_void,
  mut size: size_t,
) -> *mut libc::c_void {
  ptr = realloc(ptr, size);
  if ptr.is_null() && size != 0i32 as libc::c_ulong {
    bb_die_memory_exhausted();
  }
  return ptr;
}
/* DMALLOC */
// Die if we can't allocate and zero size bytes of memory.
#[no_mangle]
pub unsafe extern "C" fn xzalloc(mut size: size_t) -> *mut libc::c_void {
  let mut ptr: *mut libc::c_void = xmalloc(size);
  memset(ptr, 0i32, size);
  return ptr;
}
// Die if we can't copy a string to freshly allocated memory.
#[no_mangle]
pub unsafe extern "C" fn xstrdup(mut s: *const libc::c_char) -> *mut libc::c_char {
  let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
  if s.is_null() {
    return 0 as *mut libc::c_char;
  }
  t = strdup(s);
  if t.is_null() {
    bb_die_memory_exhausted();
  }
  return t;
}
// Die if we can't allocate n+1 bytes (space for the null terminator) and copy
// the (possibly truncated to length n) string into it.
#[no_mangle]
pub unsafe extern "C" fn xstrndup(
  mut s: *const libc::c_char,
  mut n: libc::c_int,
) -> *mut libc::c_char {
  let mut m: libc::c_int = 0;
  let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
  if 0i32 != 0 && s.is_null() {
    bb_simple_error_msg_and_die(b"xstrndup bug\x00" as *const u8 as *const libc::c_char);
  }
  /* We can just xmalloc(n+1) and strncpy into it, */
  /* but think about xstrndup("abc", 10000) wastage! */
  m = n;
  t = s as *mut libc::c_char;
  while m != 0 {
    if *t == 0 {
      break;
    }
    m -= 1;
    t = t.offset(1)
  }
  n -= m;
  t = xmalloc((n + 1i32) as size_t) as *mut libc::c_char;
  *t.offset(n as isize) = '\u{0}' as i32 as libc::c_char;
  return memcpy(
    t as *mut libc::c_void,
    s as *const libc::c_void,
    n as libc::c_ulong,
  ) as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn xmemdup(
  mut s: *const libc::c_void,
  mut n: libc::c_int,
) -> *mut libc::c_void {
  return memcpy(xmalloc(n as size_t), s, n as libc::c_ulong);
}
// Die if we can't open a file and return a FILE* to it.
// Notice we haven't got xfread(), This is for use with fscanf() and friends.
#[no_mangle]
pub unsafe extern "C" fn xfopen(
  mut path: *const libc::c_char,
  mut mode: *const libc::c_char,
) -> *mut FILE {
  let mut fp: *mut FILE = fopen(path, mode);
  if fp.is_null() {
    bb_perror_msg_and_die(
      b"can\'t open \'%s\'\x00" as *const u8 as *const libc::c_char,
      path,
    );
  }
  return fp;
}
// Die if we can't open a file and return a fd.
#[no_mangle]
pub unsafe extern "C" fn xopen3(
  mut pathname: *const libc::c_char,
  mut flags: libc::c_int,
  mut mode: libc::c_int,
) -> libc::c_int {
  let mut ret: libc::c_int = 0;
  ret = open(pathname, flags, mode);
  if ret < 0i32 {
    bb_perror_msg_and_die(
      b"can\'t open \'%s\'\x00" as *const u8 as *const libc::c_char,
      pathname,
    );
  }
  return ret;
}
// Die if we can't open a file and return a fd.
#[no_mangle]
pub unsafe extern "C" fn xopen(
  mut pathname: *const libc::c_char,
  mut flags: libc::c_int,
) -> libc::c_int {
  return xopen3(pathname, flags, 0o666i32);
}
// Warn if we can't open a file and return a fd.
#[no_mangle]
pub unsafe extern "C" fn open3_or_warn(
  mut pathname: *const libc::c_char,
  mut flags: libc::c_int,
  mut mode: libc::c_int,
) -> libc::c_int {
  let mut ret: libc::c_int = 0;
  ret = open(pathname, flags, mode);
  if ret < 0i32 {
    bb_perror_msg(
      b"can\'t open \'%s\'\x00" as *const u8 as *const libc::c_char,
      pathname,
    );
  }
  return ret;
}
// Warn if we can't open a file and return a fd.
#[no_mangle]
pub unsafe extern "C" fn open_or_warn(
  mut pathname: *const libc::c_char,
  mut flags: libc::c_int,
) -> libc::c_int {
  return open3_or_warn(pathname, flags, 0o666i32);
}
/* Die if we can't open an existing file readonly with O_NONBLOCK
 * and return the fd.
 * Note that for ioctl O_RDONLY is sufficient.
 */
#[no_mangle]
pub unsafe extern "C" fn xopen_nonblocking(mut pathname: *const libc::c_char) -> libc::c_int {
  return xopen(pathname, 0i32 | 0o4000i32);
}
#[no_mangle]
pub unsafe extern "C" fn xopen_as_uid_gid(
  mut pathname: *const libc::c_char,
  mut flags: libc::c_int,
  mut u: uid_t,
  mut g: gid_t,
) -> libc::c_int {
  let mut fd: libc::c_int = 0;
  let mut old_euid: uid_t = geteuid();
  let mut old_egid: gid_t = getegid();
  xsetegid(g);
  xseteuid(u);
  fd = xopen(pathname, flags);
  xseteuid(old_euid);
  xsetegid(old_egid);
  return fd;
}
#[no_mangle]
pub unsafe extern "C" fn xunlink(mut pathname: *const libc::c_char) {
  if unlink(pathname) != 0 {
    bb_perror_msg_and_die(
      b"can\'t remove file \'%s\'\x00" as *const u8 as *const libc::c_char,
      pathname,
    );
  };
}
#[no_mangle]
pub unsafe extern "C" fn xrename(
  mut oldpath: *const libc::c_char,
  mut newpath: *const libc::c_char,
) {
  if rename(oldpath, newpath) != 0 {
    bb_perror_msg_and_die(
      b"can\'t move \'%s\' to \'%s\'\x00" as *const u8 as *const libc::c_char,
      oldpath,
      newpath,
    );
  };
}
#[no_mangle]
pub unsafe extern "C" fn rename_or_warn(
  mut oldpath: *const libc::c_char,
  mut newpath: *const libc::c_char,
) -> libc::c_int {
  let mut n: libc::c_int = rename(oldpath, newpath);
  if n != 0 {
    bb_perror_msg(
      b"can\'t move \'%s\' to \'%s\'\x00" as *const u8 as *const libc::c_char,
      oldpath,
      newpath,
    );
  }
  return n;
}
#[no_mangle]
pub unsafe extern "C" fn xpipe(mut filedes: *mut libc::c_int) {
  if pipe(filedes) != 0 {
    bb_simple_perror_msg_and_die(b"can\'t create pipe\x00" as *const u8 as *const libc::c_char);
  };
}
#[no_mangle]
pub unsafe extern "C" fn xdup2(mut from: libc::c_int, mut to: libc::c_int) {
  if dup2(from, to) != to {
    bb_simple_perror_msg_and_die(
      b"can\'t duplicate file descriptor\x00" as *const u8 as *const libc::c_char,
    );
  };
  //		" %d to %d", from, to);
}
// "Renumber" opened fd
#[no_mangle]
pub unsafe extern "C" fn xmove_fd(mut from: libc::c_int, mut to: libc::c_int) {
  if from == to {
    return;
  }
  xdup2(from, to);
  close(from);
}
// Die with an error message if we can't write the entire buffer.
#[no_mangle]
pub unsafe extern "C" fn xwrite(
  mut fd: libc::c_int,
  mut buf: *const libc::c_void,
  mut count: size_t,
) {
  if count != 0 {
    let mut size: ssize_t = full_write(fd, buf, count);
    if size as size_t != count {
      /*
       * Two cases: write error immediately;
       * or some writes succeeded, then we hit an error.
       * In either case, errno is set.
       */
      bb_simple_perror_msg_and_die(if size >= 0i32 as libc::c_long {
        b"short write\x00" as *const u8 as *const libc::c_char
      } else {
        b"write error\x00" as *const u8 as *const libc::c_char
      });
    }
  };
}
#[no_mangle]
pub unsafe extern "C" fn xwrite_str(mut fd: libc::c_int, mut str: *const libc::c_char) {
  xwrite(fd, str as *const libc::c_void, strlen(str));
}
#[no_mangle]
pub unsafe extern "C" fn xclose(mut fd: libc::c_int) {
  if close(fd) != 0 {
    bb_simple_perror_msg_and_die(b"close failed\x00" as *const u8 as *const libc::c_char);
  };
}
// Die with an error message if we can't lseek to the right spot.
#[no_mangle]
pub unsafe extern "C" fn xlseek(
  mut fd: libc::c_int,
  mut offset: off_t,
  mut whence: libc::c_int,
) -> off_t {
  let mut off: off_t = lseek(fd, offset, whence);
  if off == -1i32 as off_t {
    bb_perror_msg_and_die(
      b"lseek(%lu, %d)\x00" as *const u8 as *const libc::c_char,
      offset,
      whence,
    );
  }
  return off;
}
#[no_mangle]
pub unsafe extern "C" fn xmkstemp(mut template: *mut libc::c_char) -> libc::c_int {
  let mut fd: libc::c_int = mkstemp(template);
  if fd < 0i32 {
    bb_perror_msg_and_die(
      b"can\'t create temp file \'%s\'\x00" as *const u8 as *const libc::c_char,
      template,
    );
  }
  return fd;
}
// Die with supplied filename if this FILE* has ferror set.
#[no_mangle]
pub unsafe extern "C" fn die_if_ferror(mut fp: *mut FILE, mut fn_0: *const libc::c_char) {
  if ferror_unlocked(fp) != 0 {
    /* ferror doesn't set useful errno */
    bb_error_msg_and_die(
      b"%s: I/O error\x00" as *const u8 as *const libc::c_char,
      fn_0,
    );
  };
}
// Die with an error message if stdout has ferror set.
#[no_mangle]
pub unsafe extern "C" fn die_if_ferror_stdout() {
  die_if_ferror(stdout, bb_msg_standard_output.as_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn fflush_all() -> libc::c_int {
  return fflush(0 as *mut FILE);
}
#[no_mangle]
pub unsafe extern "C" fn bb_putchar(mut ch: libc::c_int) -> libc::c_int {
  return putchar_unlocked(ch);
}
/* Die with an error message if we can't copy an entire FILE* to stdout,
 * then close that file. */
#[no_mangle]
pub unsafe extern "C" fn xprint_and_close_file(mut file: *mut FILE) {
  fflush_all();
  // copyfd outputs error messages for us.
  if bb_copyfd_eof(fileno_unlocked(file), 1i32) == -1i32 as libc::c_long {
    xfunc_die();
  }
  fclose(file);
}
// Die with an error message if we can't malloc() enough space and do an
// sprintf() into that space.
#[no_mangle]
pub unsafe extern "C" fn xasprintf(
  mut format: *const libc::c_char,
  mut args: ...
) -> *mut libc::c_char {
  let mut p: ::std::ffi::VaListImpl;
  let mut r: libc::c_int = 0;
  let mut string_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
  p = args.clone();
  r = vasprintf(&mut string_ptr, format, p.as_va_list());
  if r < 0i32 {
    bb_die_memory_exhausted();
  }
  return string_ptr;
}
#[no_mangle]
pub unsafe extern "C" fn xsetenv(mut key: *const libc::c_char, mut value: *const libc::c_char) {
  if setenv(key, value, 1i32) != 0 {
    bb_die_memory_exhausted();
  };
}
/* Handles "VAR=VAL" strings, even those which are part of environ
 * _right now_
 */
#[no_mangle]
pub unsafe extern "C" fn bb_unsetenv(mut var: *const libc::c_char) {
  let mut onstack: [libc::c_char; 112] = [0; 112]; /* smaller stack setup code on x86 */
  let mut tp: *mut libc::c_char = 0 as *mut libc::c_char;
  tp = strchr(var, '=' as i32);
  if !tp.is_null() {
    /* In case var was putenv'ed, we can't replace '='
     * with NUL and unsetenv(var) - it won't work,
     * env is modified by the replacement, unsetenv
     * sees "VAR" instead of "VAR=VAL" and does not remove it!
     * Horror :(
     */
    let mut sz: libc::c_uint = tp.wrapping_offset_from(var) as libc::c_long as libc::c_uint;
    if (sz as libc::c_ulong) < ::std::mem::size_of::<[libc::c_char; 112]>() as libc::c_ulong {
      *(mempcpy(
        onstack.as_mut_ptr() as *mut libc::c_void,
        var as *const libc::c_void,
        sz as size_t,
      ) as *mut libc::c_char)
        .offset(0) = '\u{0}' as i32 as libc::c_char;
      tp = 0 as *mut libc::c_char;
      var = onstack.as_mut_ptr()
    } else {
      /* unlikely: very long var name */
      tp = xstrndup(var, sz as libc::c_int);
      var = tp
    }
  }
  unsetenv(var);
  free(tp as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn bb_unsetenv_and_free(mut var: *mut libc::c_char) {
  bb_unsetenv(var);
  free(var as *mut libc::c_void);
}
// Die with an error message if we can't set gid.  (Because resource limits may
// limit this user to a given number of processes, and if that fills up the
// setgid() will fail and we'll _still_be_root_, which is bad.)
#[no_mangle]
pub unsafe extern "C" fn xsetgid(mut gid: gid_t) {
  if setgid(gid) != 0 {
    bb_simple_perror_msg_and_die(b"setgid\x00" as *const u8 as *const libc::c_char);
  };
}
// Die with an error message if we can't set uid.  (See xsetgid() for why.)
#[no_mangle]
pub unsafe extern "C" fn xsetuid(mut uid: uid_t) {
  if setuid(uid) != 0 {
    bb_simple_perror_msg_and_die(b"setuid\x00" as *const u8 as *const libc::c_char);
  };
}
#[no_mangle]
pub unsafe extern "C" fn xsetegid(mut egid: gid_t) {
  if setegid(egid) != 0 {
    bb_simple_perror_msg_and_die(b"setegid\x00" as *const u8 as *const libc::c_char);
  };
}
#[no_mangle]
pub unsafe extern "C" fn xseteuid(mut euid: uid_t) {
  if seteuid(euid) != 0 {
    bb_simple_perror_msg_and_die(b"seteuid\x00" as *const u8 as *const libc::c_char);
  };
}
// Die if we can't chdir to a new path.
#[no_mangle]
pub unsafe extern "C" fn xchdir(mut path: *const libc::c_char) {
  if chdir(path) != 0 {
    bb_perror_msg_and_die(
      b"can\'t change directory to \'%s\'\x00" as *const u8 as *const libc::c_char,
      path,
    );
  };
}
#[no_mangle]
pub unsafe extern "C" fn xfchdir(mut fd: libc::c_int) {
  if fchdir(fd) != 0 {
    bb_simple_perror_msg_and_die(b"fchdir\x00" as *const u8 as *const libc::c_char);
  };
}
#[no_mangle]
pub unsafe extern "C" fn xchroot(mut path: *const libc::c_char) {
  if chroot(path) != 0 {
    bb_perror_msg_and_die(
      b"can\'t change root directory to \'%s\'\x00" as *const u8 as *const libc::c_char,
      path,
    );
  }
  xchdir(b"/\x00" as *const u8 as *const libc::c_char);
}
// Print a warning message if opendir() fails, but don't die.
#[no_mangle]
pub unsafe extern "C" fn warn_opendir(mut path: *const libc::c_char) -> *mut DIR {
  let mut dp: *mut DIR = 0 as *mut DIR;
  dp = opendir(path);
  if dp.is_null() {
    bb_perror_msg(
      b"can\'t open \'%s\'\x00" as *const u8 as *const libc::c_char,
      path,
    );
  }
  return dp;
}
// Die with an error message if opendir() fails.
#[no_mangle]
pub unsafe extern "C" fn xopendir(mut path: *const libc::c_char) -> *mut DIR {
  let mut dp: *mut DIR = 0 as *mut DIR;
  dp = opendir(path);
  if dp.is_null() {
    bb_perror_msg_and_die(
      b"can\'t open \'%s\'\x00" as *const u8 as *const libc::c_char,
      path,
    );
  }
  return dp;
}
// Die with an error message if we can't open a new socket.
#[no_mangle]
pub unsafe extern "C" fn xsocket(
  mut domain: libc::c_int,
  mut type_0: libc::c_int,
  mut protocol: libc::c_int,
) -> libc::c_int {
  let mut r: libc::c_int = socket(domain, type_0, protocol);
  if r < 0i32 {
    /* Hijack vaguely related config option */
    bb_simple_perror_msg_and_die(b"socket\x00" as *const u8 as *const libc::c_char);
  }
  return r;
}
// Die with an error message if we can't bind a socket to an address.
#[no_mangle]
pub unsafe extern "C" fn xbind(
  mut sockfd: libc::c_int,
  mut my_addr: *mut sockaddr,
  mut addrlen: socklen_t,
) {
  if bind(
    sockfd,
    __CONST_SOCKADDR_ARG {
      __sockaddr__: my_addr,
    },
    addrlen,
  ) != 0
  {
    bb_simple_perror_msg_and_die(b"bind\x00" as *const u8 as *const libc::c_char);
  };
}
// Die with an error message if we can't listen for connections on a socket.
#[no_mangle]
pub unsafe extern "C" fn xlisten(mut s: libc::c_int, mut backlog: libc::c_int) {
  if listen(s, backlog) != 0 {
    bb_simple_perror_msg_and_die(b"listen\x00" as *const u8 as *const libc::c_char);
  };
}
/* Die with an error message if sendto failed.
 * Return bytes sent otherwise  */
#[no_mangle]
pub unsafe extern "C" fn xsendto(
  mut s: libc::c_int,
  mut buf: *const libc::c_void,
  mut len: size_t,
  mut to: *const sockaddr,
  mut tolen: socklen_t,
) -> ssize_t {
  let mut ret: ssize_t = sendto(
    s,
    buf,
    len,
    0i32,
    __CONST_SOCKADDR_ARG { __sockaddr__: to },
    tolen,
  );
  if ret < 0i32 as libc::c_long {
    bb_simple_perror_msg_and_die(b"sendto\x00" as *const u8 as *const libc::c_char);
  }
  return ret;
}
// xstat() - a stat() which dies on failure with meaningful error message
#[no_mangle]
pub unsafe extern "C" fn xstat(mut name: *const libc::c_char, mut stat_buf: *mut stat) {
  if stat(name, stat_buf) != 0 {
    bb_perror_msg_and_die(
      b"can\'t stat \'%s\'\x00" as *const u8 as *const libc::c_char,
      name,
    );
  };
}
#[no_mangle]
pub unsafe extern "C" fn xfstat(
  mut fd: libc::c_int,
  mut stat_buf: *mut stat,
  mut errmsg: *const libc::c_char,
) {
  /* errmsg is usually a file name, but not always:
   * xfstat may be called in a spot where file name is no longer
   * available, and caller may give e.g. "can't stat input file" string.
   */
  if fstat(fd, stat_buf) != 0 {
    bb_simple_perror_msg_and_die(errmsg);
  };
}
// selinux_or_die() - die if SELinux is disabled.
#[no_mangle]
pub unsafe extern "C" fn selinux_or_die() {
  bb_simple_error_msg_and_die(
    b"SELinux support is disabled\x00" as *const u8 as *const libc::c_char,
  );
}
#[no_mangle]
pub unsafe extern "C" fn ioctl_or_perror_and_die(
  mut fd: libc::c_int,
  mut request: libc::c_uint,
  mut argp: *mut libc::c_void,
  mut fmt: *const libc::c_char,
  mut args: ...
) -> libc::c_int {
  let mut ret: libc::c_int = 0;
  let mut p: ::std::ffi::VaListImpl;
  ret = ioctl(fd, request as libc::c_ulong, argp);
  if ret < 0i32 {
    p = args.clone();
    bb_verror_msg(fmt, p.as_va_list(), strerror(*bb_errno));
    /* xfunc_die can actually longjmp, so be nice */
    xfunc_die();
  }
  return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ioctl_or_perror(
  mut fd: libc::c_int,
  mut request: libc::c_uint,
  mut argp: *mut libc::c_void,
  mut fmt: *const libc::c_char,
  mut args: ...
) -> libc::c_int {
  let mut p: ::std::ffi::VaListImpl;
  let mut ret: libc::c_int = ioctl(fd, request as libc::c_ulong, argp);
  if ret < 0i32 {
    p = args.clone();
    bb_verror_msg(fmt, p.as_va_list(), strerror(*bb_errno));
  }
  return ret;
}
#[no_mangle]
pub unsafe extern "C" fn bb_ioctl_or_warn(
  mut fd: libc::c_int,
  mut request: libc::c_uint,
  mut argp: *mut libc::c_void,
  mut ioctl_name: *const libc::c_char,
) -> libc::c_int {
  let mut ret: libc::c_int = 0;
  ret = ioctl(fd, request as libc::c_ulong, argp);
  if ret < 0i32 {
    bb_simple_perror_msg(ioctl_name);
  }
  return ret;
}
/* NB: typically you want to pass fd 0, not 1. Think 'applet | grep something' */
/* NB: "unsigned request" is crucial! "int request" will break some arches! */
#[no_mangle]
pub unsafe extern "C" fn bb_xioctl(
  mut fd: libc::c_int,
  mut request: libc::c_uint,
  mut argp: *mut libc::c_void,
  mut ioctl_name: *const libc::c_char,
) -> libc::c_int {
  let mut ret: libc::c_int = 0;
  ret = ioctl(fd, request as libc::c_ulong, argp);
  if ret < 0i32 {
    bb_simple_perror_msg_and_die(ioctl_name);
  }
  return ret;
}
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
/* Don't need IF_xxx() guard for these */
/* Networking */
/* This structure defines protocol families and their handlers. */
/*int type,*/
/* may modify src */
/* This structure defines hardware protocols and their handlers. */
/*
 * If *devname is not NULL, use that name, otherwise try to find free one,
 * malloc and return it in *devname.
 * return value is the opened fd to the loop device, or < on error
 */
/* These constants match linux/loop.h (without BB_ prefix): */
/* Returns malloced str */
/* Like bb_ask_noecho, but asks on stdin with no timeout.  */
/* Returns -1 if input is invalid. current_mode is a base for e.g. "u+rw" */
/*
 * Config file parser
 */
// treat consecutive delimiters as one
// trim leading and trailing delimiters
// TODO: COLLAPSE and TRIM seem to always go in pair
// last token takes entire remainder of the line
// die if < min tokens found
// keep a copy of current line
// comments are recognized even if they aren't the first char
// delim[0] and delim[1] are two different allowed comment chars
// (so far, delim[0] will only work as comment char for full-line comment)
// (IOW: it works as if PARSE_EOL_COMMENTS is not set. sysctl applet is okay with this)
// comments are recognized even if there is whitespace before
// ("line start><space><tab><space>#comment" is also comment, not only "line start>#comment")
// NORMAL is:
// * remove leading and trailing delimiters and collapse
//   multiple delimiters into one
// * warn and continue if less than mintokens delimiters found
// * grab everything into last token
// * comments are recognized even if they aren't the first char
/* delims[0] is a comment char (use '\0' to disable), the rest are token delimiters */
/* Concatenate path and filename to new allocated buffer.
 * Add "/" only as needed (no duplicate "//" are produced).
 * If path is NULL, it is assumed to be "/".
 * filename should not be NULL. */
/* Returns NULL on . and .. */
/* Returns ptr to NUL */
/* Returns $SHELL, getpwuid(getuid())->pw_shell, or DEFAULT_SHELL.
 * Note that getpwuid result might need xstrdup'ing
 * if there is a possibility of intervening getpwxxx() calls.
 */
/* Structures inside "struct caps" are Linux-specific and libcap-specific: */
/* setup_environment:
 * if chdir pw->pw_dir: ok: else if to_tmp == 1: goto /tmp else: goto / or die
 * if clear_env = 1: cd(pw->pw_dir), clear environment, then set
 *   TERM=(old value)
 *   USER=pw->pw_name, LOGNAME=pw->pw_name
 *   PATH=bb_default_[root_]path
 *   HOME=pw->pw_dir
 *   SHELL=shell
 * else if change_env = 1:
 *   if not root (if pw->pw_uid != 0):
 *     USER=pw->pw_name, LOGNAME=pw->pw_name
 *   HOME=pw->pw_dir
 *   SHELL=shell
 * else does nothing
 *
 * NB: CHANGEENV and CLEARENV use setenv() - this leaks memory!
 * If setup_environment() is used is vforked child, this leaks memory _in parent too_!
 */
/* Returns a malloced string */
/*
 * rnd is additional random input. New one is returned.
 * Useful if you call crypt_make_salt many times in a row:
 * rnd = crypt_make_salt(buf1, 4, 0);
 * rnd = crypt_make_salt(buf2, 4, rnd);
 * rnd = crypt_make_salt(buf3, 4, rnd);
 * (otherwise we risk having same salt generated)
 */
/*, int rnd*/
/* "$N$" + sha_salt_16_bytes + NUL */
/* Returns number of lines changed, or -1 on error */
#[no_mangle]
pub unsafe extern "C" fn xmalloc_ttyname(mut fd: libc::c_int) -> *mut libc::c_char {
  let mut buf: [libc::c_char; 128] = [0; 128];
  let mut r: libc::c_int = ttyname_r(
    fd,
    buf.as_mut_ptr(),
    (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong),
  );
  if r != 0 {
    return 0 as *mut libc::c_char;
  }
  return xstrdup(buf.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn generate_uuid(mut buf: *mut uint8_t) {
  /* http://www.ietf.org/rfc/rfc4122.txt
   *  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
   * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   * |                          time_low                             |
   * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   * |       time_mid                |         time_hi_and_version   |
   * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   * |clk_seq_and_variant            |         node (0-1)            |
   * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   * |                         node (2-5)                            |
   * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   * IOW, uuid has this layout:
   * uint32_t time_low (big endian)
   * uint16_t time_mid (big endian)
   * uint16_t time_hi_and_version (big endian)
   *  version is a 4-bit field:
   *   1 Time-based
   *   2 DCE Security, with embedded POSIX UIDs
   *   3 Name-based (MD5)
   *   4 Randomly generated
   *   5 Name-based (SHA-1)
   * uint16_t clk_seq_and_variant (big endian)
   *  variant is a 3-bit field:
   *   0xx Reserved, NCS backward compatibility
   *   10x The variant specified in rfc4122
   *   110 Reserved, Microsoft backward compatibility
   *   111 Reserved for future definition
   * uint8_t node[6]
   *
   * For version 4, these bits are set/cleared:
   * time_hi_and_version & 0x0fff | 0x4000
   * clk_seq_and_variant & 0x3fff | 0x8000
   */
  let mut pid: pid_t = 0;
  let mut i: libc::c_int = 0;
  i = open(
    b"/dev/urandom\x00" as *const u8 as *const libc::c_char,
    0i32,
  );
  if i >= 0i32 {
    read(i, buf as *mut libc::c_void, 16i32 as size_t);
    close(i);
  }
  /* Paranoia. /dev/urandom may be missing.
   * rand() is guaranteed to generate at least [0, 2^15) range,
   * but lowest bits in some libc are not so "random".  */
  srand(monotonic_us() as libc::c_uint); /* pulls in printf */
  pid = getpid();
  loop {
    i = 0i32;
    while i < 16i32 {
      let ref mut fresh0 = *buf.offset(i as isize);
      *fresh0 = (*fresh0 as libc::c_int ^ rand() >> 5i32) as uint8_t;
      i += 1
    }
    if pid == 0i32 {
      break;
    }
    srand(pid as libc::c_uint);
    pid = 0i32
  }
  /* version = 4 */
  *buf.offset((4i32 + 2i32) as isize) =
    (*buf.offset((4i32 + 2i32) as isize) as libc::c_int & 0xfi32 | 0x40i32) as uint8_t;
  /* variant = 10x */
  *buf.offset((4i32 + 2i32 + 2i32) as isize) =
    (*buf.offset((4i32 + 2i32 + 2i32) as isize) as libc::c_int & 0x3fi32 | 0x80i32) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn xfork() -> pid_t {
  let mut pid: pid_t = 0;
  pid = fork();
  if pid < 0i32 {
    /* wtf? */
    bb_simple_perror_msg_and_die((b"vfork\x00" as *const u8 as *const libc::c_char).offset(1));
  }
  return pid;
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
#[no_mangle]
pub unsafe extern "C" fn xvfork_parent_waits_and_exits() {
  let mut pid: pid_t = 0;
  fflush_all();
  pid = ({
    let mut bb__xvfork_pid: pid_t = vfork();
    if bb__xvfork_pid < 0i32 {
      bb_simple_perror_msg_and_die(b"vfork\x00" as *const u8 as *const libc::c_char);
    }
    bb__xvfork_pid
  });
  if pid > 0i32 {
    /* Parent */
    let mut exit_status: libc::c_int = wait_for_exitstatus(pid);
    if ((exit_status & 0x7fi32) + 1i32) as libc::c_schar as libc::c_int >> 1i32 > 0i32 {
      kill_myself_with_sig(exit_status & 0x7fi32);
    }
    _exit((exit_status & 0xff00i32) >> 8i32);
  };
  /* Child continues */
}

use libc;
extern "C" {
  pub type __dirstream;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn opendir(__name: *const libc::c_char) -> *mut DIR;
  #[no_mangle]
  fn closedir(__dirp: *mut DIR) -> libc::c_int;
  #[no_mangle]
  fn readdir(__dirp: *mut DIR) -> *mut dirent;
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn snprintf(
    _: *mut libc::c_char,
    _: libc::c_ulong,
    _: *const libc::c_char,
    _: ...
  ) -> libc::c_int;
  #[no_mangle]
  fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut FILE,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn skip_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn skip_non_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc_vector_helper(
    vector: *mut libc::c_void,
    sizeof_and_shift: libc::c_uint,
    idx: libc::c_int,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xmemdup(s: *const libc::c_void, n: libc::c_int) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_basename(name: *const libc::c_char) -> *const libc::c_char;
  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xopendir(path: *const libc::c_char) -> *mut DIR;
  #[no_mangle]
  fn xmalloc_readlink(path: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn safe_strncpy(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    size: size_t,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn open_read_close(
    filename: *const libc::c_char,
    buf: *mut libc::c_void,
    maxsz: size_t,
  ) -> ssize_t;
  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  #[no_mangle]
  fn uid2uname_utoa(uid: uid_t) -> *mut libc::c_char;
  #[no_mangle]
  fn gid2group_utoa(gid: gid_t) -> *mut libc::c_char;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
  #[no_mangle]
  fn getpagesize() -> libc::c_int;
}

use crate::librb::__ino64_t;

use crate::librb::__off64_t;

use crate::librb::gid_t;
use crate::librb::pid_t;
use crate::librb::size_t;
use crate::librb::ssize_t;
use crate::librb::uid_t;
use crate::librb::uint16_t;
use crate::librb::uint8_t;
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
use crate::librb::stat;
use crate::librb::timespec;

use crate::librb::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cache_t {
  pub cache: *mut id_to_name_map_t,
  pub size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct id_to_name_map_t {
  pub id: uid_t,
  pub name: [libc::c_char; 28],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct smaprec {
  pub mapped_rw: libc::c_ulong,
  pub mapped_ro: libc::c_ulong,
  pub shared_clean: libc::c_ulong,
  pub shared_dirty: libc::c_ulong,
  pub private_clean: libc::c_ulong,
  pub private_dirty: libc::c_ulong,
  pub stack: libc::c_ulong,
  pub smap_pss: libc::c_ulong,
  pub smap_swap: libc::c_ulong,
  pub smap_size: libc::c_ulong,
  pub smap_start: libc::c_ulonglong,
  pub smap_mode: [libc::c_char; 5],
  pub smap_name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct procps_status_t {
  pub dir: *mut DIR,
  pub task_dir: *mut DIR,
  pub shift_pages_to_bytes: uint8_t,
  pub shift_pages_to_kb: uint8_t,
  pub argv_len: uint16_t,
  pub argv0: *mut libc::c_char,
  pub exe: *mut libc::c_char,
  pub main_thread_pid: libc::c_uint,
  pub vsz: libc::c_ulong,
  pub rss: libc::c_ulong,
  pub stime: libc::c_ulong,
  pub utime: libc::c_ulong,
  pub start_time: libc::c_ulong,
  pub pid: libc::c_uint,
  pub ppid: libc::c_uint,
  pub pgid: libc::c_uint,
  pub sid: libc::c_uint,
  pub uid: libc::c_uint,
  pub gid: libc::c_uint,
  pub ruid: libc::c_uint,
  pub rgid: libc::c_uint,
  pub niceness: libc::c_int,
  pub tty_major: libc::c_uint,
  pub tty_minor: libc::c_uint,
  pub smaps: smaprec,
  pub state: [libc::c_char; 4],
  pub comm: [libc::c_char; 16],
  pub last_seen_on_cpu: libc::c_int,
}
pub type C2RustUnnamed = libc::c_uint;
pub const PSSCAN_TASKS: C2RustUnnamed = 4194304;
pub const PSSCAN_RUIDGID: C2RustUnnamed = 2097152;
pub const PSSCAN_NICE: C2RustUnnamed = 1048576;
pub const PSSCAN_CPU: C2RustUnnamed = 524288;
pub const PSSCAN_START_TIME: C2RustUnnamed = 262144;
pub const PSSCAN_CONTEXT: C2RustUnnamed = 0;
pub const PSSCAN_ARGVN: C2RustUnnamed = 65536;
pub const PSSCAN_SMAPS: C2RustUnnamed = 32768;
pub const PSSCAN_TTY: C2RustUnnamed = 16384;
pub const PSSCAN_UTIME: C2RustUnnamed = 8192;
pub const PSSCAN_STIME: C2RustUnnamed = 4096;
pub const PSSCAN_RSS: C2RustUnnamed = 2048;
pub const PSSCAN_VSZ: C2RustUnnamed = 1024;
pub const PSSCAN_STATE: C2RustUnnamed = 512;
pub const PSSCAN_EXE: C2RustUnnamed = 256;
pub const PSSCAN_ARGV0: C2RustUnnamed = 128;
pub const PSSCAN_COMM: C2RustUnnamed = 32;
pub const PSSCAN_UIDGID: C2RustUnnamed = 16;
pub const PSSCAN_SID: C2RustUnnamed = 8;
pub const PSSCAN_PGID: C2RustUnnamed = 4;
pub const PSSCAN_PPID: C2RustUnnamed = 2;
pub const PSSCAN_PID: C2RustUnnamed = 1;
static mut username: cache_t = cache_t {
  cache: 0 as *const id_to_name_map_t as *mut id_to_name_map_t,
  size: 0,
};
static mut groupname: cache_t = cache_t {
  cache: 0 as *const id_to_name_map_t as *mut id_to_name_map_t,
  size: 0,
};
unsafe extern "C" fn clear_cache(mut cp: *mut cache_t) {
  free((*cp).cache as *mut libc::c_void);
  (*cp).cache = 0 as *mut id_to_name_map_t;
  (*cp).size = 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn clear_username_cache() {
  clear_cache(&mut username);
  clear_cache(&mut groupname);
}
/* more generic, but we don't need that yet */
unsafe extern "C" fn get_cached(
  mut cp: *mut cache_t,
  mut id: uid_t,
  mut x2x_utoa: Option<unsafe extern "C" fn(_: uid_t) -> *mut libc::c_char>,
) -> *mut libc::c_char {
  let mut i: libc::c_int = 0;
  i = 0i32;
  while i < (*cp).size {
    if (*(*cp).cache.offset(i as isize)).id == id {
      return (*(*cp).cache.offset(i as isize)).name.as_mut_ptr();
    }
    i += 1
  }
  let fresh0 = (*cp).size;
  (*cp).size = (*cp).size + 1;
  i = fresh0;
  (*cp).cache = xrealloc_vector_helper(
    (*cp).cache as *mut libc::c_void,
    ((::std::mem::size_of::<id_to_name_map_t>() as libc::c_ulong) << 8i32)
      .wrapping_add(2i32 as libc::c_ulong) as libc::c_uint,
    i,
  ) as *mut id_to_name_map_t;
  (*(*cp).cache.offset(i as isize)).id = id;
  /* Never fails. Generates numeric string if name isn't found */
  safe_strncpy(
    (*(*cp).cache.offset(i as isize)).name.as_mut_ptr(),
    x2x_utoa.expect("non-null function pointer")(id),
    ::std::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong,
  );
  return (*(*cp).cache.offset(i as isize)).name.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn get_cached_username(mut uid: uid_t) -> *const libc::c_char {
  return get_cached(
    &mut username,
    uid,
    Some(uid2uname_utoa as unsafe extern "C" fn(_: uid_t) -> *mut libc::c_char),
  );
}
#[no_mangle]
pub unsafe extern "C" fn get_cached_groupname(mut gid: gid_t) -> *const libc::c_char {
  return get_cached(
    &mut groupname,
    gid,
    Some(gid2group_utoa as unsafe extern "C" fn(_: gid_t) -> *mut libc::c_char),
  );
}
unsafe extern "C" fn read_to_buf(
  mut filename: *const libc::c_char,
  mut buf: *mut libc::c_void,
) -> libc::c_int {
  let mut fd: libc::c_int = 0;
  /* open_read_close() would do two reads, checking for EOF.
   * When you have 10000 /proc/$NUM/stat to read, it isn't desirable */
  let mut ret: ssize_t = -1i32 as ssize_t;
  fd = open(filename, 0i32);
  if fd >= 0i32 {
    ret = read(fd, buf, (1024i32 - 1i32) as size_t);
    close(fd);
  }
  *(buf as *mut libc::c_char).offset(if ret > 0i32 as libc::c_long {
    ret
  } else {
    0i32 as libc::c_long
  } as isize) = '\u{0}' as i32 as libc::c_char;
  return ret as libc::c_int;
}
unsafe extern "C" fn alloc_procps_scan() -> *mut procps_status_t {
  let mut n: libc::c_uint = getpagesize() as libc::c_uint;
  let mut sp: *mut procps_status_t =
    xzalloc(::std::mem::size_of::<procps_status_t>() as libc::c_ulong) as *mut procps_status_t;
  (*sp).dir = xopendir(b"/proc\x00" as *const u8 as *const libc::c_char);
  loop {
    n >>= 1i32;
    if n == 0 {
      break;
    }
    (*sp).shift_pages_to_bytes = (*sp).shift_pages_to_bytes.wrapping_add(1)
  }
  (*sp).shift_pages_to_kb = ((*sp).shift_pages_to_bytes as libc::c_int - 10i32) as uint8_t;
  return sp;
}
#[no_mangle]
pub unsafe extern "C" fn free_procps_scan(mut sp: *mut procps_status_t) {
  closedir((*sp).dir);
  if !(*sp).task_dir.is_null() {
    closedir((*sp).task_dir);
  }
  free((*sp).argv0 as *mut libc::c_void);
  free((*sp).exe as *mut libc::c_void);
  free(sp as *mut libc::c_void);
}
unsafe extern "C" fn fast_strtoull_16(mut endptr: *mut *mut libc::c_char) -> libc::c_ulonglong {
  let mut c: libc::c_uchar = 0;
  let mut str: *mut libc::c_char = *endptr;
  let mut n: libc::c_ulonglong = 0i32 as libc::c_ulonglong;
  loop
  /* Need to stop on both ' ' and '\n' */
  {
    let fresh1 = str;
    str = str.offset(1);
    c = *fresh1 as libc::c_uchar;
    if !(c as libc::c_int > ' ' as i32) {
      break;
    }
    c = ((c as libc::c_int | 0x20i32) - '0' as i32) as libc::c_uchar;
    if c as libc::c_int > 9i32 {
      /* c = c + '0' - 'a' + 10: */
      c = (c as libc::c_int - ('a' as i32 - '0' as i32 - 10i32)) as libc::c_uchar
    } /* We skip trailing space! */
    n = n
      .wrapping_mul(16i32 as libc::c_ulonglong)
      .wrapping_add(c as libc::c_ulonglong)
  }
  *endptr = str;
  return n;
}
/* We cut a lot of corners here for speed */
unsafe extern "C" fn fast_strtoul_10(mut endptr: *mut *mut libc::c_char) -> libc::c_ulong {
  let mut c: libc::c_uchar = 0;
  let mut str: *mut libc::c_char = *endptr;
  let mut n: libc::c_ulong = (*str as libc::c_int - '0' as i32) as libc::c_ulong;
  loop
  /* Need to stop on both ' ' and '\n' */
  {
    str = str.offset(1); /* We skip trailing space! */
    c = *str as libc::c_uchar;
    if !(c as libc::c_int > ' ' as i32) {
      break;
    }
    n = n
      .wrapping_mul(10i32 as libc::c_ulong)
      .wrapping_add((c as libc::c_int - '0' as i32) as libc::c_ulong)
  }
  *endptr = str.offset(1);
  return n;
}
unsafe extern "C" fn skip_fields(
  mut str: *mut libc::c_char,
  mut count: libc::c_int,
) -> *mut libc::c_char {
  loop {
    loop {
      let fresh2 = str;
      str = str.offset(1);
      if !(*fresh2 as libc::c_int != ' ' as i32) {
        break;
      }
    }
    count -= 1;
    if !(count != 0) {
      break;
    }
    /* we found a space char, str points after it */
  }
  return str;
}
#[no_mangle]
pub unsafe extern "C" fn procps_read_smaps(
  mut pid: pid_t,
  mut total: *mut smaprec,
  mut cb: Option<unsafe extern "C" fn(_: *mut smaprec, _: *mut libc::c_void) -> ()>,
  mut data: *mut libc::c_void,
) -> libc::c_int {
  let mut file: *mut FILE = 0 as *mut FILE;
  let mut currec: smaprec = smaprec {
    mapped_rw: 0,
    mapped_ro: 0,
    shared_clean: 0,
    shared_dirty: 0,
    private_clean: 0,
    private_dirty: 0,
    stack: 0,
    smap_pss: 0,
    smap_swap: 0,
    smap_size: 0,
    smap_start: 0,
    smap_mode: [0; 5],
    smap_name: 0 as *mut libc::c_char,
  };
  let mut filename: [libc::c_char; 27] = [0; 27];
  let mut buf: [libc::c_char; 1024] = [0; 1024];
  sprintf(
    filename.as_mut_ptr(),
    b"/proc/%u/smaps\x00" as *const u8 as *const libc::c_char,
    pid,
  );
  file = fopen_for_read(filename.as_mut_ptr());
  if file.is_null() {
    return 1i32;
  }
  memset(
    &mut currec as *mut smaprec as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<smaprec>() as libc::c_ulong,
  );
  while !fgets_unlocked(buf.as_mut_ptr(), 1024i32, file).is_null() {
    // Each mapping datum has this form:
    // f7d29000-f7d39000 rw-s FILEOFS M:m INODE FILENAME
    // Size:                nnn kB
    // Rss:                 nnn kB
    // .....
    let mut tp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if cb.is_some() {
      tp = is_prefixed_with(
        buf.as_mut_ptr(),
        b"Pss:\x00" as *const u8 as *const libc::c_char,
      );
      if !tp.is_null() {
        tp = skip_whitespace(tp);
        currec.smap_pss = fast_strtoul_10(&mut tp);
        (*total).smap_pss = (*total).smap_pss.wrapping_add(currec.smap_pss);
        continue;
      } else {
        tp = is_prefixed_with(
          buf.as_mut_ptr(),
          b"Swap:\x00" as *const u8 as *const libc::c_char,
        );
        if !tp.is_null() {
          tp = skip_whitespace(tp);
          currec.smap_swap = fast_strtoul_10(&mut tp);
          (*total).smap_swap = (*total).smap_swap.wrapping_add(currec.smap_swap);
          continue;
        }
      }
    }
    tp = is_prefixed_with(
      buf.as_mut_ptr(),
      b"Private_Dirty:\x00" as *const u8 as *const libc::c_char,
    );
    if !tp.is_null() {
      tp = skip_whitespace(tp);
      currec.private_dirty = fast_strtoul_10(&mut tp);
      (*total).private_dirty = (*total).private_dirty.wrapping_add(currec.private_dirty)
    } else {
      tp = is_prefixed_with(
        buf.as_mut_ptr(),
        b"Private_Clean:\x00" as *const u8 as *const libc::c_char,
      );
      if !tp.is_null() {
        tp = skip_whitespace(tp);
        currec.private_clean = fast_strtoul_10(&mut tp);
        (*total).private_clean = (*total).private_clean.wrapping_add(currec.private_clean)
      } else {
        tp = is_prefixed_with(
          buf.as_mut_ptr(),
          b"Shared_Dirty:\x00" as *const u8 as *const libc::c_char,
        );
        if !tp.is_null() {
          tp = skip_whitespace(tp);
          currec.shared_dirty = fast_strtoul_10(&mut tp);
          (*total).shared_dirty = (*total).shared_dirty.wrapping_add(currec.shared_dirty)
        } else {
          tp = is_prefixed_with(
            buf.as_mut_ptr(),
            b"Shared_Clean:\x00" as *const u8 as *const libc::c_char,
          );
          if !tp.is_null() {
            tp = skip_whitespace(tp);
            currec.shared_clean = fast_strtoul_10(&mut tp);
            (*total).shared_clean = (*total).shared_clean.wrapping_add(currec.shared_clean)
          } else {
            tp = strchr(buf.as_mut_ptr(), '-' as i32);
            if !tp.is_null() {
              // We reached next mapping - the line of this form:
              // f7d29000-f7d39000 rw-s FILEOFS M:m INODE FILENAME
              if cb.is_some() {
                /* If we have a previous record, there's nothing more
                 * for it, call the callback and clear currec
                 */
                if currec.smap_size != 0 {
                  cb.expect("non-null function pointer")(&mut currec, data);
                }
                free(currec.smap_name as *mut libc::c_void);
              }
              memset(
                &mut currec as *mut smaprec as *mut libc::c_void,
                0i32,
                ::std::mem::size_of::<smaprec>() as libc::c_ulong,
              );
              *tp = ' ' as i32 as libc::c_char;
              tp = buf.as_mut_ptr();
              currec.smap_start = fast_strtoull_16(&mut tp);
              currec.smap_size = (fast_strtoull_16(&mut tp).wrapping_sub(currec.smap_start)
                >> 10i32) as libc::c_ulong;
              strncpy(
                currec.smap_mode.as_mut_ptr(),
                tp,
                (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                  .wrapping_sub(1i32 as libc::c_ulong),
              );
              // skipping "rw-s FILEOFS M:m INODE "
              tp = skip_whitespace(skip_fields(tp, 4i32));
              // filter out /dev/something (something != zero)
              if is_prefixed_with(tp, b"/dev/\x00" as *const u8 as *const libc::c_char).is_null()
                || strcmp(tp, b"/dev/zero\n\x00" as *const u8 as *const libc::c_char) == 0i32
              {
                if currec.smap_mode[1] as libc::c_int == 'w' as i32 {
                  currec.mapped_rw = currec.smap_size; /* for (;;) */
                  (*total).mapped_rw = (*total).mapped_rw.wrapping_add(currec.smap_size)
                } else if currec.smap_mode[1] as libc::c_int == '-' as i32 {
                  currec.mapped_ro = currec.smap_size;
                  (*total).mapped_ro = (*total).mapped_ro.wrapping_add(currec.smap_size)
                }
              }
              if strcmp(tp, b"[stack]\n\x00" as *const u8 as *const libc::c_char) == 0i32 {
                (*total).stack = (*total).stack.wrapping_add(currec.smap_size)
              }
              if cb.is_some() {
                p = skip_non_whitespace(tp);
                if p == tp {
                  currec.smap_name = xstrdup(b"  [ anon ]\x00" as *const u8 as *const libc::c_char)
                } else {
                  *p = '\u{0}' as i32 as libc::c_char;
                  currec.smap_name = xstrdup(tp)
                }
              }
              (*total).smap_size = (*total).smap_size.wrapping_add(currec.smap_size)
            }
          }
        }
      }
    }
  }
  fclose(file);
  if cb.is_some() {
    if currec.smap_size != 0 {
      cb.expect("non-null function pointer")(&mut currec, data);
    }
    free(currec.smap_name as *mut libc::c_void);
  }
  return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn procps_scan(
  mut sp: *mut procps_status_t,
  mut flags: libc::c_int,
) -> *mut procps_status_t {
  if sp.is_null() {
    sp = alloc_procps_scan()
  }
  let mut current_block_79: u64;
  loop {
    let mut entry: *mut dirent = 0 as *mut dirent;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut tasknice: libc::c_long = 0;
    let mut pid: libc::c_uint = 0;
    let mut n: libc::c_int = 0;
    let mut filename: [libc::c_char; 49] = [0; 49];
    let mut filename_tail: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(*sp).task_dir.is_null() {
      entry = readdir((*sp).task_dir);
      if !entry.is_null() {
        current_block_79 = 13839692391726842101;
      } else {
        closedir((*sp).task_dir);
        (*sp).task_dir = 0 as *mut DIR;
        current_block_79 = 1841672684692190573;
      }
    } else {
      current_block_79 = 1841672684692190573;
    }
    match current_block_79 {
      1841672684692190573 => {
        entry = readdir((*sp).dir);
        if entry.is_null() {
          free_procps_scan(sp);
          return 0 as *mut procps_status_t;
        }
      }
      _ => {}
    }
    pid = bb_strtou(
      (*entry).d_name.as_mut_ptr(),
      0 as *mut *mut libc::c_char,
      10i32,
    );
    if *bb_errno != 0 {
      continue;
    }
    if flags & PSSCAN_TASKS as libc::c_int != 0 && (*sp).task_dir.is_null() {
      /* We found another /proc/PID. Do not use it,
       * there will be /proc/PID/task/PID (same PID!),
       * so just go ahead and dive into /proc/PID/task. */
      sprintf(
        filename.as_mut_ptr(),
        b"/proc/%u/task\x00" as *const u8 as *const libc::c_char,
        pid,
      );
      /* Note: if opendir fails, we just go to next /proc/XXX */
      (*sp).task_dir = opendir(filename.as_mut_ptr());
      (*sp).main_thread_pid = pid
    } else {
      /* After this point we can:
       * "break": stop parsing, return the data
       * "continue": try next /proc/XXX
       */
      memset(
        &mut (*sp).vsz as *mut libc::c_ulong as *mut libc::c_void,
        0i32,
        (::std::mem::size_of::<procps_status_t>() as libc::c_ulong).wrapping_sub(48u64),
      ); /* we needed only pid, we got it */
      (*sp).pid = pid; /* process probably exited */
      if flags & !(PSSCAN_PID as libc::c_int) == 0 {
        break;
      }
      if !(*sp).task_dir.is_null() {
        filename_tail = filename.as_mut_ptr().offset(sprintf(
          filename.as_mut_ptr(),
          b"/proc/%u/task/%u/\x00" as *const u8 as *const libc::c_char,
          (*sp).main_thread_pid,
          pid,
        ) as isize)
      } else {
        filename_tail = filename.as_mut_ptr().offset(sprintf(
          filename.as_mut_ptr(),
          b"/proc/%u/\x00" as *const u8 as *const libc::c_char,
          pid,
        ) as isize)
      }
      if flags & PSSCAN_UIDGID as libc::c_int != 0 {
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
        if stat(filename.as_mut_ptr(), &mut sb) != 0 {
          continue;
        }
        /* Effective UID/GID, not real */
        (*sp).uid = sb.st_uid;
        (*sp).gid = sb.st_gid
      }
      /* These are all retrieved from proc/NN/stat in one go: */
      if flags
        & (PSSCAN_PPID as libc::c_int
          | PSSCAN_PGID as libc::c_int
          | PSSCAN_SID as libc::c_int
          | PSSCAN_COMM as libc::c_int
          | PSSCAN_STATE as libc::c_int
          | PSSCAN_VSZ as libc::c_int
          | PSSCAN_RSS as libc::c_int
          | PSSCAN_STIME as libc::c_int
          | PSSCAN_UTIME as libc::c_int
          | PSSCAN_START_TIME as libc::c_int
          | PSSCAN_TTY as libc::c_int
          | PSSCAN_NICE as libc::c_int
          | PSSCAN_CPU as libc::c_int)
        != 0
      {
        let mut s_idx: libc::c_int = 0;
        let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut comm1: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut tty: libc::c_int = 0;
        let mut vsz: libc::c_ulong = 0;
        let mut rss: libc::c_ulong = 0;
        /* see proc(5) for some details on this */
        strcpy(
          filename_tail,
          b"stat\x00" as *const u8 as *const libc::c_char,
        ); /* process probably exited */
        n = read_to_buf(filename.as_mut_ptr(), buf.as_mut_ptr() as *mut libc::c_void); /* split into "PID (cmd" and "<rest>" */
        if n < 0i32 {
          continue;
        }
        cp = strrchr(buf.as_mut_ptr(), ')' as i32);
        /*if (!cp || cp[1] != ' ')
        continue;*/
        *cp.offset(0) = '\u{0}' as i32 as libc::c_char;
        comm1 = strchr(buf.as_mut_ptr(), '(' as i32);
        /*if (comm1)*/
        safe_strncpy(
          (*sp).comm.as_mut_ptr(),
          comm1.offset(1),
          ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        ); /* bogus data, get next /proc/XXX */
        n =
                    sscanf(cp.offset(2),
                           b"%c %u %u %u %d %*s %*s %*s %*s %*s %*s %lu %lu %*s %*s %*s %ld %*s %*s %lu %lu %lu %*s %*s %*s %*s %*s %*s %*s %*s %*s %*s %*s %*s %*s %*s %d\x00"
                               as *const u8 as *const libc::c_char,
                           (*sp).state.as_mut_ptr(),
                           &mut (*sp).ppid as *mut libc::c_uint,
                           &mut (*sp).pgid as *mut libc::c_uint,
                           &mut (*sp).sid as *mut libc::c_uint,
                           &mut tty as *mut libc::c_int,
                           &mut (*sp).utime as *mut libc::c_ulong,
                           &mut (*sp).stime as *mut libc::c_ulong,
                           &mut tasknice as *mut libc::c_long,
                           &mut (*sp).start_time as *mut libc::c_ulong,
                           &mut vsz as *mut libc::c_ulong,
                           &mut rss as *mut libc::c_ulong,
                           &mut (*sp).last_seen_on_cpu as *mut libc::c_int);
        if n < 11i32 {
          continue;
        }
        if n == 11i32 {
          (*sp).last_seen_on_cpu = 0i32
        }
        /* vsz is in bytes and we want kb */
        (*sp).vsz = vsz >> 10i32;
        /* vsz is in bytes but rss is in *PAGES*! Can you believe that? */
        (*sp).rss = rss << (*sp).shift_pages_to_kb as libc::c_int;
        (*sp).tty_major = (tty >> 8i32 & 0xfffi32) as libc::c_uint;
        (*sp).tty_minor = (tty & 0xffi32 | tty >> 12i32 & 0xfff00i32) as libc::c_uint;
        /* FEATURE_FAST_TOP */
        (*sp).niceness = tasknice as libc::c_int;
        (*sp).state[1] = ' ' as i32 as libc::c_char;
        (*sp).state[2] = ' ' as i32 as libc::c_char;
        s_idx = 1i32;
        if (*sp).vsz == 0i32 as libc::c_ulong && (*sp).state[0] as libc::c_int != 'Z' as i32 {
          /* not sure what the purpose of this flag */
          (*sp).state[1] = 'W' as i32 as libc::c_char;
          s_idx = 2i32
        }
        if tasknice != 0i32 as libc::c_long {
          if tasknice < 0i32 as libc::c_long {
            (*sp).state[s_idx as usize] = '<' as i32 as libc::c_char
          } else {
            /* > 0 */
            (*sp).state[s_idx as usize] = 'N' as i32 as libc::c_char
          }
        }
      }
      if flags & PSSCAN_SMAPS as libc::c_int != 0 {
        procps_read_smaps(pid as pid_t, &mut (*sp).smaps, None, 0 as *mut libc::c_void);
      }
      /* TOPMEM */
      if flags & PSSCAN_RUIDGID as libc::c_int != 0 {
        let mut file: *mut FILE = 0 as *mut FILE;
        strcpy(
          filename_tail,
          b"status\x00" as *const u8 as *const libc::c_char,
        );
        file = fopen_for_read(filename.as_mut_ptr());
        if !file.is_null() {
          while !fgets_unlocked(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            file,
          )
          .is_null()
          {
            let mut tp: *mut libc::c_char = 0 as *mut libc::c_char;
            tp = is_prefixed_with(
              buf.as_mut_ptr(),
              b"Uid:\x00" as *const u8 as *const libc::c_char,
            );
            if !tp.is_null() {
              tp = skip_whitespace(tp);
              sscanf(
                tp,
                b"%u\x00" as *const u8 as *const libc::c_char,
                &mut (*sp).ruid as *mut libc::c_uint,
              );
            } else {
              tp = is_prefixed_with(
                buf.as_mut_ptr(),
                b"Gid:\x00" as *const u8 as *const libc::c_char,
              );
              if tp.is_null() {
                continue;
              }
              tp = skip_whitespace(tp);
              sscanf(
                tp,
                b"%u\x00" as *const u8 as *const libc::c_char,
                &mut (*sp).rgid as *mut libc::c_uint,
              );
              break;
            }
          }
          fclose(file);
        }
      }
      /* PS_ADDITIONAL_COLUMNS */
      if flags & PSSCAN_EXE as libc::c_int != 0 {
        strcpy(
          filename_tail,
          b"exe\x00" as *const u8 as *const libc::c_char,
        );
        free((*sp).exe as *mut libc::c_void);
        (*sp).exe = xmalloc_readlink(filename.as_mut_ptr())
      }
      /* Note: if /proc/PID/cmdline is empty,
       * code below "breaks". Therefore it must be
       * the last code to parse /proc/PID/xxx data
       * (we used to have /proc/PID/exe parsing after it
       * and were getting stale sp->exe).
       */
      /* PSSCAN_CMD is not used */
      if !(flags & (PSSCAN_ARGV0 as libc::c_int | PSSCAN_ARGVN as libc::c_int) != 0) {
        break;
      }
      free((*sp).argv0 as *mut libc::c_void);
      (*sp).argv0 = 0 as *mut libc::c_char;
      strcpy(
        filename_tail,
        b"cmdline\x00" as *const u8 as *const libc::c_char,
      );
      n = read_to_buf(filename.as_mut_ptr(), buf.as_mut_ptr() as *mut libc::c_void);
      if n <= 0i32 {
        break;
      }
      if flags & PSSCAN_ARGVN as libc::c_int != 0 {
        (*sp).argv_len = n as uint16_t;
        (*sp).argv0 =
          xmemdup(buf.as_mut_ptr() as *const libc::c_void, n + 1i32) as *mut libc::c_char
      /* sp->argv0[n] = '\0'; - buf has it */
      } else {
        (*sp).argv_len = 0i32 as uint16_t;
        (*sp).argv0 = xstrdup(buf.as_mut_ptr())
      }
      break;
    }
  }
  return sp;
}

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
/* NB: typically you want to pass fd 0, not 1. Think 'applet | grep something' */
/* NB: "unsigned request" is crucial! "int request" will break some arches! */
/* At least glibc has horrendously large inline for this, so wrap it */
/* "Keycodes" that report an escape sequence.
 * We use something which fits into signed char,
 * yet doesn't represent any valid Unicode character.
 * Also, -1 is reserved for error indication and we don't use it. */
/* Used only if Alt/Ctrl/Shifted */
/* Used only if Alted */
/* ^^^^^ Be sure that last defined value is small enough.
 * Current read_key() code allows going up to -32 (0xfff..fffe0).
 * This gives three upper bits in LSB to play with:
 * KEYCODE_foo values are 0xfff..fffXX, lowest XX bits are: scavvvvv,
 * s=0 if SHIFT, c=0 if CTRL, a=0 if ALT,
 * vvvvv bits are the same for same key regardless of "shift bits".
 */
//KEYCODE_SHIFT_...   = KEYCODE_...   & ~0x80,
/* 0xfff..fff00 */
/* How long is the longest ESC sequence we know?
 * We want it big enough to be able to contain
 * cursor position sequence "ESC [ 9999 ; 9999 R"
 */
/* Note: fd may be in blocking or non-blocking mode, both make sense.
 * For one, less uses non-blocking mode.
 * Only the first read syscall inside read_key may block indefinitely
 * (unless fd is in non-blocking mode),
 * subsequent reads will time out after a few milliseconds.
 * Return of -1 means EOF or error (errno == 0 on EOF).
 * buffer[0] is used as a counter of buffered chars and must be 0
 * on first call.
 * timeout:
 * -2: do not poll(-1) for input - read() it, return on EAGAIN at once
 * -1: poll(-1) (i.e. block even on NONBLOCKed fd)
 * >=0: poll() for TIMEOUT milliseconds, return -1/EAGAIN on timeout
 */
/* It's NOT just ENABLEd or disabled. It's a number: */
/* must never be <= 0 */
/* meaning of this field depends on FEATURE_EDITING_SAVE_ON_EXIT:
 * if !FEATURE_EDITING_SAVE_ON_EXIT: "how many lines are
 * in on-disk history"
 * if FEATURE_EDITING_SAVE_ON_EXIT: "how many in-memory lines are
 * also in on-disk history (and thus need to be skipped on save)"
 */
/*
 * maxsize must be >= 2.
 * Returns:
 * -1 on read errors or EOF, or on bare Ctrl-D,
 * 0  on ctrl-C (the line entered is still returned in 'command'),
 * >0 length of input string, including terminating '\n'
 */
/* synchronize with sizeof(task_struct.comm) in /usr/include/linux/sched.h */
// For mixed 32/64 userspace, 32-bit pmap still needs
// 64-bit field here to correctly show 64-bit processes:
// (strictly speaking, other fields need to be wider too,
// but they are in kbytes, not bytes, and they hold sizes,
// not start addresses, sizes tend to be less than 4 terabytes)
/* Fields are set to 0/NULL if failed to determine (or not requested) */
/* Everything below must contain no ptrs to malloc'ed data:
 * it is memset(0) for each process in procps_scan() */
/* we round it to kbytes */
/* basename of executable in exec(2), read from /proc/N/stat
 * (if executable is symlink or script, it is NOT replaced
 * by link target or interpreter name) */
/* user/group? - use passwd/group parsing functions */
/* flag bits for procps_scan(xx, flags) calls */
/* PSSCAN_CMD      = 1 << 6, - use read_cmdline instead */
/* NB: used by find_pid_by_name(). Any applet using it
 * needs to be mentioned here. */
//procps_status_t* alloc_procps_scan(void) FAST_FUNC;
/* Format cmdline (up to col chars) into char buf[size] */
/* Puts [comm] if cmdline is empty (-> process is a kernel thread) */
#[no_mangle]
pub unsafe extern "C" fn read_cmdline(
  mut buf: *mut libc::c_char,
  mut col: libc::c_int,
  mut pid: libc::c_uint,
  mut comm: *const libc::c_char,
) {
  let mut sz: libc::c_int = 0;
  let mut filename: [libc::c_char; 29] = [0; 29];
  sprintf(
    filename.as_mut_ptr(),
    b"/proc/%u/cmdline\x00" as *const u8 as *const libc::c_char,
    pid,
  );
  sz = open_read_close(
    filename.as_mut_ptr(),
    buf as *mut libc::c_void,
    (col - 1i32) as size_t,
  ) as libc::c_int;
  if sz > 0i32 {
    let mut base: *const libc::c_char = 0 as *const libc::c_char;
    let mut comm_len: libc::c_int = 0;
    *buf.offset(sz as isize) = '\u{0}' as i32 as libc::c_char;
    loop {
      sz -= 1;
      if !(sz >= 0i32 && *buf.offset(sz as isize) as libc::c_int == '\u{0}' as i32) {
        break;
      }
    }
    /* Prevent basename("process foo/bar") = "bar" */
    *strchrnul(buf, ' ' as i32).offset(0) = '\u{0}' as i32 as libc::c_char; /* before we replace argv0's NUL with space */
    base = bb_basename(buf);
    while sz >= 0i32 {
      if (*buf.offset(sz as isize) as libc::c_uchar as libc::c_int) < ' ' as i32 {
        *buf.offset(sz as isize) = ' ' as i32 as libc::c_char
      }
      sz -= 1
    }
    if *base.offset(0) as libc::c_int == '-' as i32 {
      /* "-sh" (login shell)? */
      base = base.offset(1)
    }
    /* If comm differs from argv0, prepend "{comm} ".
     * It allows to see thread names set by prctl(PR_SET_NAME).
     */
    if comm.is_null() {
      return;
    }
    comm_len = strlen(comm) as libc::c_int;
    /* Why compare up to comm_len, not COMM_LEN-1?
     * Well, some processes rewrite argv, and use _spaces_ there
     * while rewriting. (KDE is observed to do it).
     * I prefer to still treat argv0 "process foo bar"
     * as 'equal' to comm "process".
     */
    if strncmp(base, comm, comm_len as libc::c_ulong) != 0i32 {
      comm_len += 3i32;
      if col > comm_len {
        memmove(
          buf.offset(comm_len as isize) as *mut libc::c_void,
          buf as *const libc::c_void,
          (col - comm_len) as libc::c_ulong,
        );
      }
      snprintf(
        buf,
        col as libc::c_ulong,
        b"{%s}\x00" as *const u8 as *const libc::c_char,
        comm,
      );
      if col <= comm_len {
        return;
      }
      *buf.offset((comm_len - 1i32) as isize) = ' ' as i32 as libc::c_char;
      *buf.offset((col - 1i32) as isize) = '\u{0}' as i32 as libc::c_char
    }
  } else {
    snprintf(
      buf,
      col as libc::c_ulong,
      b"[%s]\x00" as *const u8 as *const libc::c_char,
      if !comm.is_null() {
        comm
      } else {
        b"?\x00" as *const u8 as *const libc::c_char
      },
    );
  };
}
/* from kernel:
  //             pid comm S ppid pgid sid tty_nr tty_pgrp flg
  sprintf(buffer,"%d (%s) %c %d  %d   %d  %d     %d       %lu %lu \
%lu %lu %lu %lu %lu %ld %ld %ld %ld %d 0 %llu %lu %ld %lu %lu %lu %lu %lu \
%lu %lu %lu %lu %lu %lu %lu %lu %d %d %lu %lu %llu\n",
    task->pid,
    tcomm,
    state,
    ppid,
    pgid,
    sid,
    tty_nr,
    tty_pgrp,
    task->flags,
    min_flt,
    cmin_flt,
    maj_flt,
    cmaj_flt,
    cputime_to_clock_t(utime),
    cputime_to_clock_t(stime),
    cputime_to_clock_t(cutime),
    cputime_to_clock_t(cstime),
    priority,
    nice,
    num_threads,
    // 0,
    start_time,
    vsize,
    mm ? get_mm_rss(mm) : 0,
    rsslim,
    mm ? mm->start_code : 0,
    mm ? mm->end_code : 0,
    mm ? mm->start_stack : 0,
    esp,
    eip,
the rest is some obsolete cruft
*/

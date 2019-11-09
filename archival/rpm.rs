use crate::archival::libarchive::bb_archive::archive_handle_t;
use crate::librb::__compar_fn_t;
use crate::librb::size_t;
use crate::librb::smallint;
use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
use libc::close;
use libc::free;
use libc::gid_t;
use libc::group;
use libc::off64_t;
use libc::off_t;
use libc::passwd;
use libc::stat;
use libc::time_t;
use libc::uid_t;

extern "C" {


  #[no_mangle]
  fn chown(__file: *const libc::c_char, __owner: uid_t, __group: gid_t) -> libc::c_int;
  #[no_mangle]
  fn getuid() -> uid_t;
  #[no_mangle]
  fn getgid() -> gid_t;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn getopt(
    ___argc: libc::c_int,
    ___argv: *const *mut libc::c_char,
    __shortopts: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  fn getpagesize() -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn bsearch(
    __key: *const libc::c_void,
    __base: *const libc::c_void,
    __nmemb: size_t,
    __size: size_t,
    __compar: __compar_fn_t,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn mmap(
    __addr: *mut libc::c_void,
    __len: size_t,
    __prot: libc::c_int,
    __flags: libc::c_int,
    __fd: libc::c_int,
    __offset: off64_t,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
  #[no_mangle]
  fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn strftime(
    __s: *mut libc::c_char,
    __maxsize: size_t,
    __format: *const libc::c_char,
    __tp: *const tm,
  ) -> size_t;
  #[no_mangle]
  fn localtime(__timer: *const time_t) -> *mut tm;
  /* Search for an entry with a matching username.  */
  #[no_mangle]
  fn bb_internal_getpwnam(__name: *const libc::c_char) -> *mut passwd;
  /* Search for an entry with a matching group name.  */
  #[no_mangle]
  fn bb_internal_getgrnam(__name: *const libc::c_char) -> *mut group;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn remove_file(path: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn copy_file(
    source: *const libc::c_char,
    dest: *const libc::c_char,
    flags: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn bb_copyfd_eof(fd1: libc::c_int, fd2: libc::c_int) -> off_t;
  #[no_mangle]
  static mut bb_got_signal: smallint;
  #[no_mangle]
  fn xchdir(path: *const libc::c_char);
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xlseek(fd: libc::c_int, offset: off_t, whence: libc::c_int) -> off_t;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn xread(fd: libc::c_int, buf: *mut libc::c_void, count: size_t);
  /* Autodetects gzip/bzip2 formats. fd may be in the middle of the file! */
  #[no_mangle]
  fn setup_unzip_on_fd(fd: libc::c_int, fail_if_not_compressed: libc::c_int) -> libc::c_int;
  /* lzma has no signature, need a little helper. NB: exist only for ENABLE_FEATURE_SEAMLESS_LZMA=y */
  #[no_mangle]
  fn setup_lzma_on_fd(fd: libc::c_int);
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  static bb_msg_standard_output: [libc::c_char; 0];
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
  #[no_mangle]
  fn init_handle() -> *mut archive_handle_t;
  #[no_mangle]
  fn data_extract_all(archive_handle: *mut archive_handle_t);
  #[no_mangle]
  fn get_header_cpio(archive_handle: *mut archive_handle_t) -> libc::c_char;
  #[no_mangle]
  fn seek_by_read(fd: libc::c_int, amount: off_t);
  #[no_mangle]
  fn check_errors_in_children(signo: libc::c_int);
}

/* NB: unaligned parameter should be a pointer, aligned one -
 * a lvalue. This makes it more likely to not swap them by mistake
 */
/* #elif ... - add your favorite arch today! */
/* Unaligned, fixed-endian accessors */
/* unxz needs an aligned fixed-endian accessor.
 * (however, the compiler does not realize it's aligned, the cast is still necessary)
 */
/* ---- Size-saving "small" ints (arch-dependent) ----------- */
/* add other arches which benefit from this... */
use libc::tm;
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
//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub map: *mut libc::c_void,
  pub mytags: *mut rpm_index,
  pub tagcount: libc::c_int,
  pub mapsize: libc::c_uint,
  pub pagesize: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rpm_index {
  pub tag: u32,
  pub type_0: u32,
  pub offset: u32,
  pub count: u32,
}

/* Then follows the header: */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rpm_header {
  pub magic_and_ver: u32,
  pub reserved: u32,
  pub entries: u32,
  pub size: u32,
  /* Size of store (4 bytes) */
}
pub type rpm_functions_e = libc::c_uint;
pub const rpm_query_list_config: rpm_functions_e = 64;
pub const rpm_query_list_doc: rpm_functions_e = 32;
pub const rpm_query_list: rpm_functions_e = 16;
pub const rpm_query_package: rpm_functions_e = 8;
pub const rpm_query_info: rpm_functions_e = 4;
pub const rpm_install: rpm_functions_e = 2;
pub const rpm_query: rpm_functions_e = 1;
unsafe extern "C" fn rpm_gettags(mut filename: *const libc::c_char) -> libc::c_int {
  let mut tags: *mut rpm_index = 0 as *mut rpm_index;
  let mut fd: libc::c_int = 0;
  let mut pass: libc::c_uint = 0;
  let mut idx: libc::c_uint = 0;
  let mut storepos: libc::c_uint = 0;
  if filename.is_null() {
    /* rpm2cpio w/o filename? */
    filename = bb_msg_standard_output.as_ptr(); /* Seek past the unused lead */
    fd = 0i32
  } else {
    fd = xopen(filename, 0i32)
  }
  storepos = xlseek(fd, 96i32 as off_t, 1i32) as libc::c_uint;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tagcount = 0i32;
  tags = 0 as *mut rpm_index;
  idx = 0i32 as libc::c_uint;
  /* 1st pass is the signature headers, 2nd is the main stuff */
  pass = 0i32 as libc::c_uint;
  while pass < 2i32 as libc::c_uint {
    let mut header: rpm_header = rpm_header {
      magic_and_ver: 0,
      reserved: 0,
      entries: 0,
      size: 0,
    };
    let mut cnt: libc::c_uint = 0;
    xread(
      fd,
      &mut header as *mut rpm_header as *mut libc::c_void,
      ::std::mem::size_of::<rpm_header>() as libc::c_ulong,
    );
    if header.magic_and_ver
      != ({
        let mut __v: libc::c_uint = 0;
        let mut __x: libc::c_uint = 0x8eade801u32;
        if 0 != 0 {
          __v = (__x & 0xff000000u32) >> 24i32
            | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
            | (__x & 0xff00i32 as libc::c_uint) << 8i32
            | (__x & 0xffi32 as libc::c_uint) << 24i32
        } else {
          let fresh0 = &mut __v;
          let fresh1;
          let fresh2 = __x;
          asm!("bswap $0" : "=r" (fresh1) : "0"
                             (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2))
                             :);
          c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
        }
        __v
      })
    {
      bb_error_msg_and_die(
        b"invalid RPM header magic in \'%s\'\x00" as *const u8 as *const libc::c_char,
        filename,
      );
    }
    header.size = {
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = header.size;
      if 0 != 0 {
        __v = (__x & 0xff000000u32) >> 24i32
          | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
          | (__x & 0xff00i32 as libc::c_uint) << 8i32
          | (__x & 0xffi32 as libc::c_uint) << 24i32
      } else {
        let fresh3 = &mut __v;
        let fresh4;
        let fresh5 = __x;
        asm!("bswap $0" : "=r" (fresh4) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5))
                          :);
        c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
      }
      __v
    };
    cnt = {
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = header.entries;
      if 0 != 0 {
        __v = (__x & 0xff000000u32) >> 24i32
          | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
          | (__x & 0xff00i32 as libc::c_uint) << 8i32
          | (__x & 0xffi32 as libc::c_uint) << 24i32
      } else {
        let fresh6 = &mut __v;
        let fresh7;
        let fresh8 = __x;
        asm!("bswap $0" : "=r" (fresh7) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh6, fresh8))
                          :);
        c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
      }
      __v
    };
    storepos = (storepos as libc::c_ulong).wrapping_add(
      (::std::mem::size_of::<rpm_header>() as libc::c_ulong)
        .wrapping_add(cnt.wrapping_mul(16i32 as libc::c_uint) as libc::c_ulong),
    ) as libc::c_uint as libc::c_uint;
    let ref mut fresh9 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tagcount;
    *fresh9 = (*fresh9 as libc::c_uint).wrapping_add(cnt) as libc::c_int as libc::c_int;
    tags = xrealloc(
      tags as *mut libc::c_void,
      (::std::mem::size_of::<rpm_index>() as libc::c_ulong).wrapping_mul(
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tagcount as libc::c_ulong,
      ),
    ) as *mut rpm_index;
    xread(
      fd,
      &mut *tags.offset(idx as isize) as *mut rpm_index as *mut libc::c_void,
      (::std::mem::size_of::<rpm_index>() as libc::c_ulong).wrapping_mul(cnt as libc::c_ulong),
    );
    loop {
      let fresh10 = cnt;
      cnt = cnt.wrapping_sub(1);
      if !(fresh10 != 0) {
        break;
      }
      let mut tag: *mut rpm_index = &mut *tags.offset(idx as isize) as *mut rpm_index;
      (*tag).tag = {
        let mut __v: libc::c_uint = 0;
        let mut __x: libc::c_uint = (*tag).tag;
        if 0 != 0 {
          __v = (__x & 0xff000000u32) >> 24i32
            | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
            | (__x & 0xff00i32 as libc::c_uint) << 8i32
            | (__x & 0xffi32 as libc::c_uint) << 24i32
        } else {
          let fresh11 = &mut __v;
          let fresh12;
          let fresh13 = __x;
          asm!("bswap $0" : "=r" (fresh12) : "0"
                              (c2rust_asm_casts::AsmCast::cast_in(fresh11, fresh13))
                              :);
          c2rust_asm_casts::AsmCast::cast_out(fresh11, fresh13, fresh12);
        }
        __v
      };
      (*tag).type_0 = {
        let mut __v: libc::c_uint = 0;
        let mut __x: libc::c_uint = (*tag).type_0;
        if 0 != 0 {
          __v = (__x & 0xff000000u32) >> 24i32
            | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
            | (__x & 0xff00i32 as libc::c_uint) << 8i32
            | (__x & 0xffi32 as libc::c_uint) << 24i32
        } else {
          let fresh14 = &mut __v;
          let fresh15;
          let fresh16 = __x;
          asm!("bswap $0" : "=r" (fresh15) : "0"
                              (c2rust_asm_casts::AsmCast::cast_in(fresh14, fresh16))
                              :);
          c2rust_asm_casts::AsmCast::cast_out(fresh14, fresh16, fresh15);
        }
        __v
      };
      (*tag).count = {
        let mut __v: libc::c_uint = 0;
        let mut __x: libc::c_uint = (*tag).count;
        if 0 != 0 {
          __v = (__x & 0xff000000u32) >> 24i32
            | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
            | (__x & 0xff00i32 as libc::c_uint) << 8i32
            | (__x & 0xffi32 as libc::c_uint) << 24i32
        } else {
          let fresh17 = &mut __v;
          let fresh18;
          let fresh19 = __x;
          asm!("bswap $0" : "=r" (fresh18) : "0"
                              (c2rust_asm_casts::AsmCast::cast_in(fresh17, fresh19))
                              :);
          c2rust_asm_casts::AsmCast::cast_out(fresh17, fresh19, fresh18);
        }
        __v
      };
      (*tag).offset =
                storepos.wrapping_add({
                                           let mut __v: libc::c_uint = 0;
                                           let mut __x: libc::c_uint =
                                               (*tag).offset;
                                           if 0 != 0 {
                                               __v =
                                                   (__x & 0xff000000u32) >>
                                                       24i32 |
                                                       (__x &
                                                            0xff0000i32 as
                                                                libc::c_uint)
                                                           >> 8i32 |
                                                       (__x &
                                                            0xff00i32 as
                                                                libc::c_uint)
                                                           << 8i32 |
                                                       (__x &
                                                            0xffi32 as
                                                                libc::c_uint)
                                                           << 24i32
                                           } else {
                                               let fresh20 = &mut __v;
                                               let fresh21;
                                               let fresh22 = __x;
                                               asm!("bswap $0" : "=r"
                                                    (fresh21) : "0"
                                                    (c2rust_asm_casts::AsmCast::cast_in(fresh20, fresh22))
                                                    :);
                                               c2rust_asm_casts::AsmCast::cast_out(fresh20,
                                                                                   fresh22,
                                                                                   fresh21);
                                           }
                                           __v
                                       });
      if pass == 0i32 as libc::c_uint {
        (*tag).tag = ((*tag).tag as libc::c_uint).wrapping_sub(743i32 as libc::c_uint) as u32 as u32
      }
      idx = idx.wrapping_add(1)
    }
    /* Skip padding to 8 byte boundary after reading signature headers */
    if pass == 0i32 as libc::c_uint {
      while header.size & 7i32 as libc::c_uint != 0 {
        header.size = header.size.wrapping_add(1)
      }
    }
    /* Seek past store */
    storepos = xlseek(fd, header.size as off_t, 1i32) as libc::c_uint;
    pass = pass.wrapping_add(1)
  }
  let ref mut fresh23 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).mytags;
  *fresh23 = tags;
  /* Map the store */
  storepos = storepos.wrapping_add((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pagesize)
    & -((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pagesize as libc::c_int)
      as libc::c_uint;
  /* remember size for munmap */
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).mapsize = storepos;
  /* some NOMMU systems prefer MAP_PRIVATE over MAP_SHARED */
  let ref mut fresh24 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).map;
  *fresh24 = mmap(
    0 as *mut libc::c_void,
    storepos as size_t,
    0x1i32,
    0x2i32,
    fd,
    0i32 as off64_t,
  );
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).map == -1i32 as *mut libc::c_void {
    bb_perror_msg_and_die(
      b"mmap \'%s\'\x00" as *const u8 as *const libc::c_char,
      filename,
    );
  }
  return fd;
}
unsafe extern "C" fn bsearch_rpmtag(
  mut key: *const libc::c_void,
  mut item: *const libc::c_void,
) -> libc::c_int {
  let mut tag: *mut libc::c_int = key as *mut libc::c_int;
  let mut tmp: *mut rpm_index = item as *mut rpm_index;
  return (*tag as libc::c_uint).wrapping_sub((*tmp).tag) as libc::c_int;
}
unsafe extern "C" fn rpm_getstr(
  mut tag: libc::c_int,
  mut itemindex: libc::c_int,
) -> *mut libc::c_char {
  let mut found: *mut rpm_index = 0 as *mut rpm_index;
  found = bsearch(
    &mut tag as *mut libc::c_int as *const libc::c_void,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).mytags as *const libc::c_void,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tagcount as size_t,
    ::std::mem::size_of::<rpm_index>() as libc::c_ulong,
    Some(
      bsearch_rpmtag
        as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int,
    ),
  ) as *mut rpm_index;
  if found.is_null() || itemindex as libc::c_uint >= (*found).count {
    return 0 as *mut libc::c_char;
  }
  if (*found).type_0 == 6i32 as libc::c_uint
    || (*found).type_0 == 9i32 as libc::c_uint
    || (*found).type_0 == 8i32 as libc::c_uint
  {
    let mut n: libc::c_int = 0;
    let mut tmpstr: *mut libc::c_char = ((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).map
      as *mut libc::c_char)
      .offset((*found).offset as isize);
    n = 0i32;
    while n < itemindex {
      tmpstr = tmpstr.offset(strlen(tmpstr) as isize).offset(1);
      n += 1
    }
    return tmpstr;
  }
  return 0 as *mut libc::c_char;
}
unsafe extern "C" fn rpm_getstr0(mut tag: libc::c_int) -> *mut libc::c_char {
  return rpm_getstr(tag, 0i32);
}
unsafe extern "C" fn rpm_getint(mut tag: libc::c_int, mut itemindex: libc::c_int) -> libc::c_int {
  let mut found: *mut rpm_index = 0 as *mut rpm_index;
  let mut tmpint: *mut libc::c_char = 0 as *mut libc::c_char;
  /* gcc throws warnings here when sizeof(void*)!=sizeof(int) ...
   * it's ok to ignore it because tag won't be used as a pointer */
  found = bsearch(
    &mut tag as *mut libc::c_int as *const libc::c_void,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).mytags as *const libc::c_void,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tagcount as size_t,
    ::std::mem::size_of::<rpm_index>() as libc::c_ulong,
    Some(
      bsearch_rpmtag
        as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int,
    ),
  ) as *mut rpm_index;
  if found.is_null() || itemindex as libc::c_uint >= (*found).count {
    return -1i32;
  }
  tmpint = ((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).map as *mut libc::c_char)
    .offset((*found).offset as isize);
  if (*found).type_0 == 4i32 as libc::c_uint {
    tmpint = tmpint.offset((itemindex * 4i32) as isize);
    return ({
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = *(tmpint as *mut i32) as libc::c_uint;
      if 0 != 0 {
        __v = (__x & 0xff000000u32) >> 24i32
          | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
          | (__x & 0xff00i32 as libc::c_uint) << 8i32
          | (__x & 0xffi32 as libc::c_uint) << 24i32
      } else {
        let fresh25 = &mut __v;
        let fresh26;
        let fresh27 = __x;
        asm!("bswap $0" : "=r" (fresh26) : "0"
                             (c2rust_asm_casts::AsmCast::cast_in(fresh25, fresh27))
                             :);
        c2rust_asm_casts::AsmCast::cast_out(fresh25, fresh27, fresh26);
      }
      __v
    }) as libc::c_int;
  }
  if (*found).type_0 == 3i32 as libc::c_uint {
    tmpint = tmpint.offset((itemindex * 2i32) as isize);
    return ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = *(tmpint as *mut i16) as libc::c_ushort;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh28 = &mut __v;
        let fresh29;
        let fresh30 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh29) : "0"
                             (c2rust_asm_casts::AsmCast::cast_in(fresh28, fresh30))
                             : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh28, fresh30, fresh29);
      }
      __v
    }) as libc::c_int;
  }
  if (*found).type_0 == 2i32 as libc::c_uint {
    tmpint = tmpint.offset(itemindex as isize);
    return *(tmpint as *mut i8) as libc::c_int;
  }
  return -1i32;
}
unsafe extern "C" fn rpm_getcount(mut tag: libc::c_int) -> libc::c_int {
  let mut found: *mut rpm_index = 0 as *mut rpm_index;
  found = bsearch(
    &mut tag as *mut libc::c_int as *const libc::c_void,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).mytags as *const libc::c_void,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tagcount as size_t,
    ::std::mem::size_of::<rpm_index>() as libc::c_ulong,
    Some(
      bsearch_rpmtag
        as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int,
    ),
  ) as *mut rpm_index;
  if found.is_null() {
    return 0i32;
  }
  return (*found).count as libc::c_int;
}
unsafe extern "C" fn fileaction_dobackup(
  mut filename: *mut libc::c_char,
  mut fileref: libc::c_int,
) {
  let mut oldfile: stat = std::mem::zeroed();
  let mut stat_res: libc::c_int = 0;
  let mut newname: *mut libc::c_char = 0 as *mut libc::c_char;
  if rpm_getint(1037i32, fileref) & 1i32 << 0i32 != 0 {
    /* Only need to backup config files */
    stat_res = lstat(filename, &mut oldfile);
    if stat_res == 0i32
      && oldfile.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint
    {
      /* File already exists  - really should check MD5's etc to see if different */
      newname = xasprintf(
        b"%s.rpmorig\x00" as *const u8 as *const libc::c_char,
        filename,
      );
      copy_file(
        filename,
        newname,
        FILEUTILS_RECUR as libc::c_int | FILEUTILS_PRESERVE_STATUS as libc::c_int,
      );
      remove_file(
        filename,
        FILEUTILS_RECUR as libc::c_int | FILEUTILS_FORCE as libc::c_int,
      );
      free(newname as *mut libc::c_void);
    }
  };
}
unsafe extern "C" fn fileaction_setowngrp(
  mut filename: *mut libc::c_char,
  mut fileref: libc::c_int,
) {
  /* real rpm warns: "user foo does not exist - using <you>" */
  let mut pw: *mut passwd = bb_internal_getpwnam(rpm_getstr(1039i32, fileref)); /* or euid? */
  let mut uid: libc::c_int = if !pw.is_null() {
    (*pw).pw_uid
  } else {
    getuid()
  } as libc::c_int;
  let mut gr: *mut group = bb_internal_getgrnam(rpm_getstr(1040i32, fileref));
  let mut gid: libc::c_int = if !gr.is_null() {
    (*gr).gr_gid
  } else {
    getgid()
  } as libc::c_int;
  chown(filename, uid as uid_t, gid as gid_t);
}
unsafe extern "C" fn loop_through_files(
  mut filetag: libc::c_int,
  mut fileaction: Option<unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int) -> ()>,
) {
  let mut count: libc::c_int = 0i32;
  while !rpm_getstr(filetag, count).is_null() {
    let mut filename: *mut libc::c_char = xasprintf(
      b"%s%s\x00" as *const u8 as *const libc::c_char,
      rpm_getstr(1118i32, rpm_getint(1116i32, count)),
      rpm_getstr(1117i32, count),
    );
    let fresh31 = count;
    count = count + 1;
    fileaction.expect("non-null function pointer")(filename, fresh31);
    free(filename as *mut libc::c_void);
  }
}
//DEBUG
unsafe extern "C" fn extract_cpio(mut fd: libc::c_int, mut source_rpm: *const libc::c_char) {
  let mut archive_handle: *mut archive_handle_t = 0 as *mut archive_handle_t; /* else: SRPM, install to current dir */
  if !source_rpm.is_null() {
    /* Binary rpm (it was built from some SRPM), install to root */
    xchdir(b"/\x00" as *const u8 as *const libc::c_char);
  }
  /* Initialize */
  archive_handle = init_handle();
  (*archive_handle).seek =
    Some(seek_by_read as unsafe extern "C" fn(_: libc::c_int, _: off_t) -> ());
  (*archive_handle).action_data =
    Some(data_extract_all as unsafe extern "C" fn(_: *mut archive_handle_t) -> ());
  /* For testing (rpm -i only lists the files in internal cpio): */
  (*archive_handle).ah_flags = (1i32 << 0i32 | 1i32 << 1i32 | 1i32 << 9i32) as libc::c_uint;
  (*archive_handle).src_fd = fd;
  /*archive_handle->offset = 0; - init_handle() did it */
  setup_unzip_on_fd((*archive_handle).src_fd, 1i32);
  while get_header_cpio(archive_handle) as libc::c_int == 0i32 {}
}
//usage:#define rpm_trivial_usage
//usage:       "-i PACKAGE.rpm; rpm -qp[ildc] PACKAGE.rpm"
//usage:#define rpm_full_usage "\n\n"
//usage:       "Manipulate RPM packages\n"
//usage:     "\nCommands:"
//usage:     "\n	-i	Install package"
//usage:     "\n	-qp	Query package"
//usage:     "\n	-qpi	Show information"
//usage:     "\n	-qpl	List contents"
//usage:     "\n	-qpd	List documents"
//usage:     "\n	-qpc	List config files"
/* RPM version 4.13.0.1:
 * Unlike -q, -i seems to imply -p: -i, -ip and -pi work the same.
 * OTOH, with -q order is important: "-piq FILE.rpm" works as -qp, not -qpi
 * (IOW: shows only package name, not package info).
 * "-iq ARG" works as -q: treats ARG as package name, not a file.
 *
 * "man rpm" on -l option and options implying it:
 * -l, --list		List files in package.
 * -c, --configfiles	List only configuration files (implies -l).
 * -d, --docfiles	List only documentation files (implies -l).
 * -L, --licensefiles	List only license files (implies -l).
 * --dump	Dump file information as follows (implies -l):
 *		path size mtime digest mode owner group isconfig isdoc rdev symlink
 * -s, --state	Display the states of files in the package (implies -l).
 *		The state of each file is one of normal, not installed, or replaced.
 *
 * Looks like we can switch to getopt32 here: in practice, people
 * do place -q first if they intend to use it (misinterpreting "-piq" wouldn't matter).
 */
#[no_mangle]
pub unsafe extern "C" fn rpm_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opt: libc::c_int = 0;
  let mut func: libc::c_int = 0i32;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pagesize = getpagesize() as libc::c_uint;
  loop {
    opt = getopt(
      argc,
      argv,
      b"iqpldc\x00" as *const u8 as *const libc::c_char,
    );
    if !(opt != -1i32) {
      break;
    }
    match opt {
      105 => {
        /* First arg: Install mode, with q: Information */
        if func == 0 {
          func = rpm_install as libc::c_int
        } else {
          func |= rpm_query_info as libc::c_int
        }
      }
      113 => {
        /* First arg: Query mode */
        if func != 0 {
          bb_show_usage();
        }
        func = rpm_query as libc::c_int
      }
      112 => {
        /* Query a package (IOW: .rpm file, we are not querying RPMDB) */
        func |= rpm_query_package as libc::c_int
      }
      108 => {
        /* List files in a package */
        func |= rpm_query_list as libc::c_int
      }
      100 => {
        /* List doc files in a package (implies -l) */
        func |= rpm_query_list as libc::c_int;
        func |= rpm_query_list_doc as libc::c_int
      }
      99 => {
        /* List config files in a package (implies -l) */
        func |= rpm_query_list as libc::c_int;
        func |= rpm_query_list_config as libc::c_int
      }
      _ => {
        bb_show_usage();
      }
    }
  }
  argv = argv.offset(optind as isize);
  //argc -= optind;
  if (*argv.offset(0)).is_null() {
    bb_show_usage();
  }
  loop {
    let mut rpm_fd: libc::c_int = 0;
    let mut source_rpm: *const libc::c_char = 0 as *const libc::c_char;
    rpm_fd = rpm_gettags(*argv);
    source_rpm = rpm_getstr0(1044i32);
    if func & rpm_install as libc::c_int != 0 {
      /* -i (and not -qi) */
      /* Backup any config files */
      loop_through_files(
        1117i32,
        Some(
          fileaction_dobackup as unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int) -> (),
        ),
      );
      /* Extact the archive */
      extract_cpio(rpm_fd, source_rpm);
      /* Set the correct file uid/gid's */
      loop_through_files(
        1117i32,
        Some(
          fileaction_setowngrp as unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int) -> (),
        ),
      );
    } else if func & (rpm_query as libc::c_int | rpm_query_package as libc::c_int)
      == rpm_query as libc::c_int | rpm_query_package as libc::c_int
    {
      /* -qp */
      if func & (rpm_query_info as libc::c_int | rpm_query_list as libc::c_int) == 0 {
        /* If just a straight query, just give package name */
        printf(
          b"%s-%s-%s\n\x00" as *const u8 as *const libc::c_char,
          rpm_getstr0(1000i32),
          rpm_getstr0(1001i32),
          rpm_getstr0(1002i32),
        );
      }
      if func & rpm_query_info as libc::c_int != 0 {
        /* Do the nice printout */
        let mut bdate_time: time_t = 0;
        let mut bdate_ptm: *mut tm = 0 as *mut tm;
        let mut bdatestring: [libc::c_char; 50] = [0; 50];
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        printf(
          b"%-12s: %s\n\x00" as *const u8 as *const libc::c_char,
          b"Name\x00" as *const u8 as *const libc::c_char,
          rpm_getstr0(1000i32),
        );
        /* TODO compat: add "Epoch" here */
        printf(
          b"%-12s: %s\n\x00" as *const u8 as *const libc::c_char,
          b"Version\x00" as *const u8 as *const libc::c_char,
          rpm_getstr0(1001i32),
        );
        printf(
          b"%-12s: %s\n\x00" as *const u8 as *const libc::c_char,
          b"Release\x00" as *const u8 as *const libc::c_char,
          rpm_getstr0(1002i32),
        );
        /* add "Architecture" */
        /* printf("%-12s: %s\n", "Install Date", "(not installed)"); - we don't know */
        printf(
          b"%-12s: %s\n\x00" as *const u8 as *const libc::c_char,
          b"Group\x00" as *const u8 as *const libc::c_char,
          rpm_getstr0(1016i32),
        );
        printf(
          b"%-12s: %d\n\x00" as *const u8 as *const libc::c_char,
          b"Size\x00" as *const u8 as *const libc::c_char,
          rpm_getint(1009i32, 0i32),
        );
        printf(
          b"%-12s: %s\n\x00" as *const u8 as *const libc::c_char,
          b"License\x00" as *const u8 as *const libc::c_char,
          rpm_getstr0(1014i32),
        );
        /* add "Signature" */
        printf(
          b"%-12s: %s\n\x00" as *const u8 as *const libc::c_char,
          b"Source RPM\x00" as *const u8 as *const libc::c_char,
          if !source_rpm.is_null() {
            source_rpm
          } else {
            b"(none)\x00" as *const u8 as *const libc::c_char
          },
        );
        bdate_time = rpm_getint(1006i32, 0i32) as time_t;
        bdate_ptm = localtime(&mut bdate_time);
        strftime(
          bdatestring.as_mut_ptr(),
          50i32 as size_t,
          b"%a %d %b %Y %T %Z\x00" as *const u8 as *const libc::c_char,
          bdate_ptm,
        );
        printf(
          b"%-12s: %s\n\x00" as *const u8 as *const libc::c_char,
          b"Build Date\x00" as *const u8 as *const libc::c_char,
          bdatestring.as_mut_ptr(),
        );
        printf(
          b"%-12s: %s\n\x00" as *const u8 as *const libc::c_char,
          b"Build Host\x00" as *const u8 as *const libc::c_char,
          rpm_getstr0(1007i32),
        );
        p = rpm_getstr0(1098i32);
        printf(
          b"%-12s: %s\n\x00" as *const u8 as *const libc::c_char,
          b"Relocations\x00" as *const u8 as *const libc::c_char,
          if !p.is_null() {
            p
          } else {
            b"(not relocatable)\x00" as *const u8 as *const libc::c_char
          },
        );
        /* add "Packager" */
        p = rpm_getstr0(1011i32);
        if !p.is_null() {
          /* rpm 4.13.0.1 does not show "(none)" for Vendor: */
          printf(
            b"%-12s: %s\n\x00" as *const u8 as *const libc::c_char,
            b"Vendor\x00" as *const u8 as *const libc::c_char,
            p,
          );
        }
        p = rpm_getstr0(1020i32);
        if !p.is_null() {
          /* rpm 4.13.0.1 does not show "(none)"/"(null)" for URL: */
          printf(
            b"%-12s: %s\n\x00" as *const u8 as *const libc::c_char,
            b"URL\x00" as *const u8 as *const libc::c_char,
            p,
          );
        }
        printf(
          b"%-12s: %s\n\x00" as *const u8 as *const libc::c_char,
          b"Summary\x00" as *const u8 as *const libc::c_char,
          rpm_getstr0(1004i32),
        );
        printf(
          b"Description :\n%s\n\x00" as *const u8 as *const libc::c_char,
          rpm_getstr0(1005i32),
        );
      }
      if func & rpm_query_list as libc::c_int != 0 {
        let mut count: libc::c_int = 0;
        let mut it: libc::c_int = 0;
        let mut flags: libc::c_int = 0;
        count = rpm_getcount(1117i32);
        let mut current_block_61: u64;
        it = 0i32;
        while it < count {
          flags = rpm_getint(1037i32, it);
          match func & (rpm_query_list_doc as libc::c_int | rpm_query_list_config as libc::c_int) {
            32 => {
              if flags & 1i32 << 1i32 == 0 {
                current_block_61 = 1134115459065347084;
              } else {
                current_block_61 = 3229571381435211107;
              }
            }
            64 => {
              if flags & 1i32 << 0i32 == 0 {
                current_block_61 = 1134115459065347084;
              } else {
                current_block_61 = 3229571381435211107;
              }
            }
            96 => {
              if flags & (1i32 << 0i32 | 1i32 << 1i32) == 0 {
                current_block_61 = 1134115459065347084;
              } else {
                current_block_61 = 3229571381435211107;
              }
            }
            _ => {
              current_block_61 = 3229571381435211107;
            }
          }
          match current_block_61 {
            3229571381435211107 => {
              printf(
                b"%s%s\n\x00" as *const u8 as *const libc::c_char,
                rpm_getstr(1118i32, rpm_getint(1116i32, it)),
                rpm_getstr(1117i32, it),
              );
            }
            _ => {}
          }
          it += 1
        }
      }
    } else {
      /* Unsupported (help text shows what we support) */
      bb_show_usage();
    }
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
    munmap(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).map,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).mapsize as size_t,
    );
    free((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).mytags as *mut libc::c_void);
    close(rpm_fd);
  }
  return 0i32;
}
/* RPM */
/*
 * Mini rpm2cpio implementation for busybox
 *
 * Copyright (C) 2001 by Laurence Anderson
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config RPM2CPIO
//config:	bool "rpm2cpio (21 kb)"
//config:	default y
//config:	help
//config:	Converts a RPM file into a CPIO archive.
//applet:IF_RPM2CPIO(APPLET(rpm2cpio, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_RPM2CPIO) += rpm.o
//usage:#define rpm2cpio_trivial_usage
//usage:       "PACKAGE.rpm"
//usage:#define rpm2cpio_full_usage "\n\n"
//usage:       "Output a cpio archive of the rpm file"
/* No getopt required */
#[no_mangle]
pub unsafe extern "C" fn rpm2cpio_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut str: *const libc::c_char = 0 as *const libc::c_char;
  let mut rpm_fd: libc::c_int = 0;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pagesize = getpagesize() as libc::c_uint;
  rpm_fd = rpm_gettags(*argv.offset(1));
  //if (SEAMLESS_COMPRESSION) - we do this at the end instead.
  //	/* We need to know whether child (gzip/bzip/etc) exits abnormally */
  //	signal(SIGCHLD, check_errors_in_children);
  if 1i32 != 0
    && {
      str = rpm_getstr0(1125i32);
      !str.is_null()
    }
    && strcmp(str, b"lzma\x00" as *const u8 as *const libc::c_char) == 0i32
  {
    // lzma compression can't be detected
    // set up decompressor without detection
    setup_lzma_on_fd(rpm_fd);
  } else {
    setup_unzip_on_fd(rpm_fd, 1i32);
  }
  if bb_copyfd_eof(rpm_fd, 1i32) < 0 {
    bb_simple_error_msg_and_die(b"error unpacking\x00" as *const u8 as *const libc::c_char);
  }
  if 0i32 != 0 || 1i32 != 0 || 1i32 != 0 || 1i32 != 0 || 1i32 != 0 || 0i32 != 0 {
    check_errors_in_children(0i32);
    return bb_got_signal as libc::c_int;
  }
  return 0i32;
}
/* RPM2CPIO */

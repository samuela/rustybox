use crate::archival::libarchive::bb_archive::transformer_state_t;
use crate::libbb::llist::llist_t;
use crate::librb::size_t;
use crate::librb::smallint;
use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::appletlib::applet_name;
use libc;
use libc::close;
use libc::free;
use libc::getopt;
use libc::lstat;
use libc::mode_t;
use libc::off64_t;
use libc::off_t;
use libc::open;
use libc::printf;
use libc::puts;
use libc::sprintf;
use libc::ssize_t;
use libc::stat;
use libc::strcpy;
use libc::FILE;
extern "C" {

  #[no_mangle]
  static mut optarg: *mut libc::c_char;

  #[no_mangle]
  static mut stdin: *mut FILE;

  #[no_mangle]
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut FILE,
  ) -> *mut libc::c_char;

  #[no_mangle]
  fn lseek(__fd: libc::c_int, __offset: off64_t, __whence: libc::c_int) -> off64_t;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn dirname(__path: *mut libc::c_char) -> *mut libc::c_char;

  /* Some useful definitions */
  /* Macros for min/max.  */
  /* buffer allocation schemes */
  /* glibc uses __errno_location() to get a ptr to errno */
  /* We can just memorize it once - no multithreading in busybox :) */

  #[no_mangle]
  fn chomp(s: *mut libc::c_char);
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_copyfd_exact_size(fd1: libc::c_int, fd2: libc::c_int, size: off_t);
  /* Simpler version: does not special case "/" string */
  #[no_mangle]
  fn bb_basename(name: *const libc::c_char) -> *const libc::c_char;
  /* NB: can violate const-ness (similarly to strchr) */
  #[no_mangle]
  fn last_char_is(s: *const libc::c_char, c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn xdup2(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn xchdir(path: *const libc::c_char);
  #[no_mangle]
  fn xopen3(pathname: *const libc::c_char, flags: libc::c_int, mode: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xlseek(fd: libc::c_int, offset: off_t, whence: libc::c_int) -> off_t;
  #[no_mangle]
  fn overlapping_strcpy(dst: *mut libc::c_char, src: *const libc::c_char);
  /* Returns a string with unprintable chars replaced by '?' or
   * SUBST_WCHAR. This function is unicode-aware. */
  #[no_mangle]
  fn printable_string(str: *const libc::c_char) -> *const libc::c_char;
  // NB: will return short read on error, not -1,
  // if some data was read before error occurred
  #[no_mangle]
  fn full_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn xread(fd: libc::c_int, buf: *mut libc::c_void, count: size_t);
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn llist_add_to(old_head: *mut *mut llist_t, data: *mut libc::c_void);
  #[no_mangle]
  fn xfunc_die() -> !;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
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
  fn strip_unsafe_prefix(str: *const libc::c_char) -> *const libc::c_char;
  #[no_mangle]
  fn create_or_remember_link(
    link_placeholders: *mut *mut llist_t,
    target: *const libc::c_char,
    linkname: *const libc::c_char,
    hard_link: libc::c_int,
  );
  #[no_mangle]
  fn create_links_from_list(list: *mut llist_t);
  #[no_mangle]
  fn find_list_entry(list: *const llist_t, filename: *const libc::c_char) -> *const llist_t;
  #[no_mangle]
  fn init_transformer_state(xstate: *mut transformer_state_t);
  #[no_mangle]
  fn inflate_unzip(xstate: *mut transformer_state_t) -> libc::c_longlong;
  #[no_mangle]
  fn unpack_bz2_stream(xstate: *mut transformer_state_t) -> libc::c_longlong;
  #[no_mangle]
  fn unpack_lzma_stream(xstate: *mut transformer_state_t) -> libc::c_longlong;
  #[no_mangle]
  fn unpack_xz_stream(xstate: *mut transformer_state_t) -> libc::c_longlong;
}

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
/* Having next pointer as a first member allows easy creation
 * of "llist-compatible" structs, and using llist_FOO functions
 * on them.
 */

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
  pub b: [u8; 8],
  pub b16: [u16; 4],
  pub b32: [u32; 2],
}

/*
 * Mini unzip implementation for busybox
 *
 * Copyright (C) 2004 by Ed Clark
 *
 * Loosely based on original busybox unzip applet by Laurence Anderson.
 * All options and features should work in this version.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* For reference see
 * http://www.pkware.com/company/standards/appnote/
 * http://www.info-zip.org/pub/infozip/doc/appnote-iz-latest.zip
 *
 * TODO
 * Zip64 + other methods
 */
//config:config UNZIP
//config:	bool "unzip (26 kb)"
//config:	default y
//config:	help
//config:	unzip will list or extract files from a ZIP archive,
//config:	commonly found on DOS/WIN systems. The default behavior
//config:	(with no options) is to extract the archive into the
//config:	current directory.
//config:
//config:config FEATURE_UNZIP_CDF
//config:	bool "Read and use Central Directory data"
//config:	default y
//config:	depends on UNZIP
//config:	help
//config:	If you know that you only need to deal with simple
//config:	ZIP files without deleted/updated files, SFX archives etc,
//config:	you can reduce code size by unselecting this option.
//config:	To support less trivial ZIPs, say Y.
//config:
//config:config FEATURE_UNZIP_BZIP2
//config:	bool "Support compression method 12 (bzip2)"
//config:	default y
//config:	depends on FEATURE_UNZIP_CDF && DESKTOP
// FEATURE_UNZIP_CDF is needed, otherwise we can't find start of next file
// DESKTOP is needed to get back uncompressed length
//config:
//config:config FEATURE_UNZIP_LZMA
//config:	bool "Support compression method 14 (lzma)"
//config:	default y
//config:	depends on FEATURE_UNZIP_CDF && DESKTOP
//config:
//config:config FEATURE_UNZIP_XZ
//config:	bool "Support compression method 95 (xz)"
//config:	default y
//config:	depends on FEATURE_UNZIP_CDF && DESKTOP
//applet:IF_UNZIP(APPLET(unzip, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_UNZIP) += unzip.o
//usage:#define unzip_trivial_usage
//usage:       "[-lnojpq] FILE[.zip] [FILE]... [-x FILE...] [-d DIR]"
//usage:#define unzip_full_usage "\n\n"
//usage:       "Extract FILEs from ZIP archive\n"
//usage:     "\n	-l	List contents (with -q for short form)"
//usage:     "\n	-n	Never overwrite files (default: ask)"
//usage:     "\n	-o	Overwrite"
//usage:     "\n	-j	Do not restore paths"
//usage:     "\n	-p	Print to stdout"
//usage:     "\n	-q	Quiet"
//usage:     "\n	-x FILE	Exclude FILEs"
//usage:     "\n	-d DIR	Extract into DIR"
pub type C2RustUnnamed_1 = libc::c_uint;
pub const ZIP_DD_MAGIC: C2RustUnnamed_1 = 134695760;
pub const ZIP_CDE_MAGIC: C2RustUnnamed_1 = 101010256;
pub const ZIP_CDF_MAGIC: C2RustUnnamed_1 = 33639248;
pub const ZIP_FILEHEADER_MAGIC: C2RustUnnamed_1 = 67324752;
#[derive(Copy, Clone)]
#[repr(C)]
pub union zip_header_t {
  pub raw: [u8; 26],
  pub fmt: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
  pub version: u16,
  pub zip_flags: u16,
  pub method: u16,
  pub modtime: u16,
  pub moddate: u16,
  pub crc32: u32,
  pub cmpsize: u32,
  pub ucmpsize: u32,
  pub filename_len: u16,
  pub extra_len: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union cdf_header_t {
  pub raw: [u8; 42],
  pub fmt: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
  pub version_made_by: u16,
  pub version_needed: u16,
  pub cdf_flags: u16,
  pub method: u16,
  pub modtime: u16,
  pub moddate: u16,
  pub crc32: u32,
  pub cmpsize: u32,
  pub ucmpsize: u32,
  pub filename_len: u16,
  pub extra_len: u16,
  pub file_comment_length: u16,
  pub disk_number_start: u16,
  pub internal_attributes: u16,
  pub external_attributes: u32,
  pub relative_offset_of_local_header: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union cde_t {
  pub raw: [u8; 16],
  pub fmt: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
  pub this_disk_no: u16,
  pub disk_with_cdf_no: u16,
  pub cdf_entries_on_this_disk: u16,
  pub cdf_entries_total: u16,
  pub cdf_size: u32,
  pub cdf_offset: u32,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const zip_fd: C2RustUnnamed_5 = 3;
pub const OPT_l: C2RustUnnamed_7 = 1;
pub const O_ALWAYS: C2RustUnnamed_6 = 2;
pub const O_PROMPT: C2RustUnnamed_6 = 0;
pub const O_NEVER: C2RustUnnamed_6 = 1;
pub const OPT_j: C2RustUnnamed_7 = 4;
pub const OPT_x: C2RustUnnamed_7 = 2;
pub type C2RustUnnamed_6 = libc::c_uint;
pub type C2RustUnnamed_7 = libc::c_uint;
/* NB: does not preserve file position! */
unsafe extern "C" fn find_cdf_offset() -> u32 {
  let mut cde: cde_t = cde_t { raw: [0; 16] };
  let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
  let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
  let mut end: off_t = 0;
  let mut found: u32 = 0;
  end = lseek(zip_fd as libc::c_int, 0i32 as off64_t, 2i32);
  if end == -1i32 as off_t {
    return 0xffffffffu32;
  }
  end -= (64i32 * 1024i32) as libc::c_long;
  if end < 0 {
    end = 0i32 as off_t
  }
  xlseek(zip_fd as libc::c_int, end, 0i32);
  buf = xzalloc((64i32 * 1024i32) as size_t) as *mut libc::c_uchar;
  full_read(
    zip_fd as libc::c_int,
    buf as *mut libc::c_void,
    (64i32 * 1024i32) as size_t,
  );
  found = 0xffffffffu32;
  p = buf;
  while p
    <= buf
      .offset((64i32 * 1024i32) as isize)
      .offset(-16)
      .offset(-4)
  {
    if *p as libc::c_int != 'P' as i32 {
      p = p.offset(1)
    } else {
      p = p.offset(1);
      if *p as libc::c_int != 'K' as i32 {
        continue;
      }
      p = p.offset(1);
      if *p as libc::c_int != 5i32 {
        continue;
      }
      p = p.offset(1);
      if *p as libc::c_int != 6i32 {
        continue;
      }
      /* we found CDE! */
      memcpy(
        cde.raw.as_mut_ptr() as *mut libc::c_void,
        p.offset(1) as *const libc::c_void,
        16i32 as libc::c_ulong,
      );
      /*
       * I've seen .ZIP files with seemingly valid CDEs
       * where cdf_offset points past EOF - ??
       * This check ignores such CDEs:
       */
      if (cde.fmt.cdf_offset as libc::c_long) < end + p.wrapping_offset_from(buf) as libc::c_long {
        found = cde.fmt.cdf_offset
        /*
         * We do not "break" here because only the last CDE is valid.
         * I've seen a .zip archive which contained a .zip file,
         * uncompressed, and taking the first CDE was using
         * the CDE inside that file!
         */
      }
    }
  }
  free(buf as *mut libc::c_void);
  return found;
}
unsafe extern "C" fn read_next_cdf(mut cdf_offset: u32, mut cdf: *mut cdf_header_t) -> u32 {
  let mut magic: u32 = 0;
  if cdf_offset == 0xffffffffu32 {
    return cdf_offset;
  }
  xlseek(zip_fd as libc::c_int, cdf_offset as off_t, 0i32);
  xread(
    zip_fd as libc::c_int,
    &mut magic as *mut u32 as *mut libc::c_void,
    4i32 as size_t,
  );
  /* Central Directory End? Assume CDF has ended.
   * (more correct method is to use cde.cdf_entries_total counter)
   */
  if magic == ZIP_CDE_MAGIC as libc::c_int as libc::c_uint {
    return 0i32 as u32;
    /* EOF */
  }
  xread(
    zip_fd as libc::c_int,
    (*cdf).raw.as_mut_ptr() as *mut libc::c_void,
    42i32 as size_t,
  );
  cdf_offset = (cdf_offset as libc::c_uint).wrapping_add(
    (4i32
      + 42i32
      + (*cdf).fmt.filename_len as libc::c_int
      + (*cdf).fmt.extra_len as libc::c_int
      + (*cdf).fmt.file_comment_length as libc::c_int) as libc::c_uint,
  ) as u32 as u32;
  return cdf_offset;
}
unsafe extern "C" fn die_if_bad_fnamesize(mut sz: libc::c_uint) {
  if sz > 0xfffi32 as libc::c_uint {
    /* more than 4k?! no funny business please */
    bb_simple_error_msg_and_die(b"bad archive\x00" as *const u8 as *const libc::c_char);
  };
}
unsafe extern "C" fn unzip_skip(mut skip: off_t) {
  if skip != 0 {
    if lseek(zip_fd as libc::c_int, skip, 1i32) == -1i32 as off_t {
      bb_copyfd_exact_size(zip_fd as libc::c_int, -1i32, skip);
    }
  };
}
unsafe extern "C" fn unzip_create_leading_dirs(mut fn_0: *const libc::c_char) {
  /* Create all leading directories */
  let mut name: *mut libc::c_char = xstrdup(fn_0);
  /* mode of -1: set mode according to umask */
  if bb_make_directory(
    dirname(name),
    -1i32 as libc::c_long,
    FILEUTILS_RECUR as libc::c_int,
  ) != 0
  {
    xfunc_die();
    /* bb_make_directory is noisy */
  }
  free(name as *mut libc::c_void);
}
unsafe extern "C" fn unzip_extract_symlink(
  mut symlink_placeholders: *mut *mut llist_t,
  mut zip: *mut zip_header_t,
  mut dst_fn: *const libc::c_char,
) {
  let mut target: *mut libc::c_char = 0 as *mut libc::c_char;
  die_if_bad_fnamesize((*zip).fmt.ucmpsize);
  if (*zip).fmt.method as libc::c_int == 0i32 {
    /* Method 0 - stored (not compressed) */
    target = xzalloc((*zip).fmt.ucmpsize.wrapping_add(1i32 as libc::c_uint) as size_t)
      as *mut libc::c_char;
    xread(
      zip_fd as libc::c_int,
      target as *mut libc::c_void,
      (*zip).fmt.ucmpsize as size_t,
    );
  } else {
    bb_simple_error_msg_and_die(
      b"compressed symlink is not supported\x00" as *const u8 as *const libc::c_char,
    );
  }
  create_or_remember_link(symlink_placeholders, target, dst_fn, 0i32);
  free(target as *mut libc::c_void);
}
unsafe extern "C" fn unzip_extract(mut zip: *mut zip_header_t, mut dst_fd: libc::c_int) {
  let mut xstate: transformer_state_t = std::mem::zeroed();
  if (*zip).fmt.method as libc::c_int == 0i32 {
    /* Method 0 - stored (not compressed) */
    let mut size: off_t = (*zip).fmt.ucmpsize as off_t;
    if size != 0 {
      bb_copyfd_exact_size(zip_fd as libc::c_int, dst_fd, size);
    }
    return;
  }
  init_transformer_state(&mut xstate);
  xstate.bytes_in = (*zip).fmt.cmpsize as off_t;
  xstate.src_fd = zip_fd as libc::c_int;
  xstate.dst_fd = dst_fd;
  if (*zip).fmt.method as libc::c_int == 8i32 {
    /* Method 8 - inflate */
    if inflate_unzip(&mut xstate) < 0i32 as libc::c_longlong {
      bb_simple_error_msg_and_die(b"inflate error\x00" as *const u8 as *const libc::c_char);
    }
    /* Validate decompression - crc */
    if (*zip).fmt.crc32 as libc::c_long != xstate.crc32 as libc::c_long ^ 0xffffffffi64 {
      bb_simple_error_msg_and_die(b"crc error\x00" as *const u8 as *const libc::c_char);
    }
  } else if (*zip).fmt.method as libc::c_int == 12i32 {
    /* Tested. Unpacker reads too much, but we use CDF
     * and will seek to the correct beginning of next file.
     */
    xstate.bytes_out = unpack_bz2_stream(&mut xstate) as off_t;
    if xstate.bytes_out < 0 {
      bb_simple_error_msg_and_die(b"inflate error\x00" as *const u8 as *const libc::c_char);
    }
  } else if (*zip).fmt.method as libc::c_int == 14i32 {
    /* Not tested yet */
    xstate.bytes_out = unpack_lzma_stream(&mut xstate) as off_t;
    if xstate.bytes_out < 0 {
      bb_simple_error_msg_and_die(b"inflate error\x00" as *const u8 as *const libc::c_char);
    }
  } else if (*zip).fmt.method as libc::c_int == 95i32 {
    /* Not tested yet */
    xstate.bytes_out = unpack_xz_stream(&mut xstate) as off_t;
    if xstate.bytes_out < 0 {
      bb_simple_error_msg_and_die(b"inflate error\x00" as *const u8 as *const libc::c_char);
    }
  } else {
    bb_error_msg_and_die(
      b"unsupported method %u\x00" as *const u8 as *const libc::c_char,
      (*zip).fmt.method as libc::c_int,
    );
  }
  /* Validate decompression - size */
  if (*zip).fmt.ucmpsize as libc::c_long != xstate.bytes_out {
    /* Don't die. Who knows, maybe len calculation
     * was botched somewhere. After all, crc matched! */
    bb_simple_error_msg(b"bad length\x00" as *const u8 as *const libc::c_char);
  };
}
unsafe extern "C" fn my_fgets80(mut buf80: *mut libc::c_char) {
  fflush_all();
  if fgets_unlocked(buf80, 80i32, stdin).is_null() {
    bb_simple_perror_msg_and_die(
      b"can\'t read standard input\x00" as *const u8 as *const libc::c_char,
    );
  };
}
unsafe extern "C" fn get_lstat_mode(mut dst_fn: *const libc::c_char) -> libc::c_int {
  let mut stat_buf: stat = std::mem::zeroed();
  if lstat(dst_fn, &mut stat_buf) == -1i32 {
    if *bb_errno != 2i32 {
      bb_perror_msg_and_die(
        b"can\'t stat \'%s\'\x00" as *const u8 as *const libc::c_char,
        dst_fn,
      );
    }
    /* File does not exist */
    return -1i32;
  } /* must match size used by my_fgets80 */
  return stat_buf.st_mode as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn unzip_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut mode_0: libc::c_int = 0;
  let mut current_block: u64;
  let mut opts: libc::c_uint = 0;
  let mut quiet: smallint = 0i32 as smallint;
  let mut verbose: smallint = 0i32 as smallint;
  let mut overwrite: smallint = O_PROMPT as libc::c_int as smallint;
  let mut cdf_offset: u32 = 0;
  let mut total_usize: libc::c_ulong = 0;
  let mut total_size: libc::c_ulong = 0;
  let mut total_entries: libc::c_uint = 0;
  let mut dst_fd: libc::c_int = -1i32;
  let mut src_fn: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut dst_fn: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut zaccept: *mut llist_t = 0 as *mut llist_t;
  let mut zreject: *mut llist_t = 0 as *mut llist_t;
  let mut base_dir: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut symlink_placeholders: *mut llist_t = 0 as *mut llist_t;
  let mut i: libc::c_int = 0;
  let mut key_buf: [libc::c_char; 80] = [0; 80];
  /* -q, -l and -v: UnZip 5.52 of 28 February 2005, by Info-ZIP:
   *
   * # /usr/bin/unzip -qq -v decompress_unlzma.i.zip
   *   204372  Defl:N    35278  83%  09-06-09 14:23  0d056252  decompress_unlzma.i
   * # /usr/bin/unzip -q -v decompress_unlzma.i.zip
   *  Length   Method    Size  Ratio   Date   Time   CRC-32    Name
   * --------  ------  ------- -----   ----   ----   ------    ----
   *   204372  Defl:N    35278  83%  09-06-09 14:23  0d056252  decompress_unlzma.i
   * --------          -------  ---                            -------
   *   204372            35278  83%                            1 file
   * # /usr/bin/unzip -v decompress_unlzma.i.zip
   * Archive:  decompress_unlzma.i.zip
   *  Length   Method    Size  Ratio   Date   Time   CRC-32    Name
   * --------  ------  ------- -----   ----   ----   ------    ----
   *   204372  Defl:N    35278  83%  09-06-09 14:23  0d056252  decompress_unlzma.i
   * --------          -------  ---                            -------
   *   204372            35278  83%                            1 file
   * # unzip -v decompress_unlzma.i.zip
   * Archive:  decompress_unlzma.i.zip
   *   Length     Date   Time    Name
   *  --------    ----   ----    ----
   *    204372  09-06-09 14:23   decompress_unlzma.i
   *  --------                   -------
   *    204372                   1 files
   * # /usr/bin/unzip -l -qq decompress_unlzma.i.zip
   *    204372  09-06-09 14:23   decompress_unlzma.i
   * # /usr/bin/unzip -l -q decompress_unlzma.i.zip
   *   Length     Date   Time    Name
   *  --------    ----   ----    ----
   *    204372  09-06-09 14:23   decompress_unlzma.i
   *  --------                   -------
   *    204372                   1 file
   * # /usr/bin/unzip -l decompress_unlzma.i.zip
   * Archive:  decompress_unlzma.i.zip
   *   Length     Date   Time    Name
   *  --------    ----   ----    ----
   *    204372  09-06-09 14:23   decompress_unlzma.i
   *  --------                   -------
   *    204372                   1 file
   */
  opts = 0i32 as libc::c_uint;
  loop
  /* '-' makes getopt return 1 for non-options */
  {
    i = getopt(
      argc,
      argv,
      b"-d:lnopqxjv\x00" as *const u8 as *const libc::c_char,
    );
    if !(i != -1i32) {
      break;
    }
    let mut current_block_20: u64;
    match i {
      100 => {
        /* Extract to base directory */
        base_dir = optarg;
        current_block_20 = 7746103178988627676;
      }
      108 => {
        /* List */
        opts |= OPT_l as libc::c_int as libc::c_uint;
        current_block_20 = 7746103178988627676;
      }
      110 => {
        /* Never overwrite existing files */
        overwrite = O_NEVER as libc::c_int as smallint;
        current_block_20 = 7746103178988627676;
      }
      111 => {
        /* Always overwrite existing files */
        overwrite = O_ALWAYS as libc::c_int as smallint;
        current_block_20 = 7746103178988627676;
      }
      112 => {
        /* Extract files to stdout and fall through to set verbosity */
        dst_fd = 1i32;
        current_block_20 = 11893559095843931121;
      }
      113 => {
        current_block_20 = 11893559095843931121;
      }
      118 => {
        /* Verbose list */
        verbose += 1;
        opts |= OPT_l as libc::c_int as libc::c_uint;
        current_block_20 = 7746103178988627676;
      }
      120 => {
        opts |= OPT_x as libc::c_int as libc::c_uint;
        current_block_20 = 7746103178988627676;
      }
      106 => {
        opts |= OPT_j as libc::c_int as libc::c_uint;
        current_block_20 = 7746103178988627676;
      }
      1 => {
        if src_fn.is_null() {
          /* The zip file */
          /* +5: space for ".zip" and NUL */
          src_fn = xmalloc(strlen(optarg).wrapping_add(5i32 as libc::c_ulong)) as *mut libc::c_char;
          strcpy(src_fn, optarg);
        } else if opts & OPT_x as libc::c_int as libc::c_uint == 0 {
          /* Include files */
          llist_add_to(&mut zaccept, optarg as *mut libc::c_void);
        } else {
          /* Exclude files */
          llist_add_to(&mut zreject, optarg as *mut libc::c_void);
        }
        current_block_20 = 7746103178988627676;
      }
      _ => {
        bb_show_usage();
      }
    }
    match current_block_20 {
      11893559095843931121 => {
        /* Be quiet */
        quiet += 1
      }
      _ => {}
    }
  }
  if src_fn.is_null() {
    bb_show_usage();
  }
  /* Open input file */
  if *src_fn.offset(0) as libc::c_int == '-' as i32 && *src_fn.offset(1) == 0 {
    xdup2(0i32, zip_fd as libc::c_int);
    /* Cannot use prompt mode since zip data is arriving on STDIN */
    if overwrite as libc::c_int == O_PROMPT as libc::c_int {
      overwrite = O_NEVER as libc::c_int as smallint
    }
  } else {
    static mut extn: [[libc::c_char; 5]; 2] = [[46, 122, 105, 112, 0], [46, 90, 73, 80, 0]];
    let mut ext: *mut libc::c_char = src_fn.offset(strlen(src_fn) as isize);
    let mut src_fd: libc::c_int = 0;
    i = 0i32;
    loop {
      src_fd = open(src_fn, 0i32);
      if src_fd >= 0i32 {
        break;
      }
      i += 1;
      if i > 2i32 {
        *ext = '\u{0}' as i32 as libc::c_char;
        bb_error_msg_and_die(
          b"can\'t open %s[.zip]\x00" as *const u8 as *const libc::c_char,
          src_fn,
        );
      }
      strcpy(ext, extn[(i - 1i32) as usize].as_ptr());
    }
    xmove_fd(src_fd, zip_fd as libc::c_int);
  }
  /* Change dir if necessary */
  if !base_dir.is_null() {
    xchdir(base_dir);
  }
  if quiet as libc::c_int <= 1i32 {
    /* not -qq */
    if quiet as libc::c_int == 0i32 {
      printf(
        b"Archive:  %s\n\x00" as *const u8 as *const libc::c_char,
        printable_string(src_fn),
      );
    }
    if opts & OPT_l as libc::c_int as libc::c_uint != 0 {
      puts(if verbose as libc::c_int != 0 {
        b" Length   Method    Size  Cmpr    Date    Time   CRC-32   Name\n--------  ------  ------- ---- ---------- ----- --------  ----\x00"
                         as *const u8 as *const libc::c_char
      } else {
        b"  Length      Date    Time    Name\n---------  ---------- -----   ----\x00" as *const u8
          as *const libc::c_char
      });
    }
  }
  /* Example of an archive with one 0-byte long file named 'z'
   * created by Zip 2.31 on Unix:
   * 0000 [50 4b]03 04 0a 00 00 00 00 00 42 1a b8 3c 00 00 |PK........B..<..|
   *       sig........ vneed flags compr mtime mdate crc32>
   * 0010  00 00 00 00 00 00 00 00 00 00 01 00 15 00 7a 55 |..............zU|
   *      >..... csize...... usize...... fnlen exlen fn ex>
   * 0020  54 09 00 03 cc d3 f9 4b cc d3 f9 4b 55 78 04 00 |T......K...KUx..|
   *      >tra_field......................................
   * 0030  00 00 00 00[50 4b]01 02 17 03 0a 00 00 00 00 00 |....PK..........|
   *       ........... sig........ vmade vneed flags compr
   * 0040  42 1a b8 3c 00 00 00 00 00 00 00 00 00 00 00 00 |B..<............|
   *       mtime mdate crc32...... csize...... usize......
   * 0050  01 00 0d 00 00 00 00 00 00 00 00 00 a4 81 00 00 |................|
   *       fnlen exlen clen. dnum. iattr eattr...... relofs> (eattr = rw-r--r--)
   * 0060  00 00 7a 55 54 05 00 03 cc d3 f9 4b 55 78 00 00 |..zUT......KUx..|
   *      >..... fn extra_field...........................
   * 0070 [50 4b]05 06 00 00 00 00 01 00 01 00 3c 00 00 00 |PK..........<...|
   * 0080  34 00 00 00 00 00                               |4.....|
   */
  total_usize = 0i32 as libc::c_ulong; /* try to seek to the end, find CDE and CDF start */
  total_size = 0i32 as libc::c_ulong;
  total_entries = 0i32 as libc::c_uint;
  cdf_offset = find_cdf_offset();
  loop {
    let mut zip: zip_header_t = zip_header_t { raw: [0; 26] };
    let mut dir_mode: mode_t = 0o777i32 as mode_t;
    let mut file_mode: mode_t = 0o666i32 as mode_t;
    if 1i32 == 0 || cdf_offset == 0xffffffffu32 {
      /* Normally happens when input is unseekable.
       *
       * Valid ZIP file has Central Directory at the end
       * with central directory file headers (CDFs).
       * After it, there is a Central Directory End structure.
       * CDFs identify what files are in the ZIP and where
       * they are located. This allows ZIP readers to load
       * the list of files without reading the entire ZIP archive.
       * ZIP files may be appended to, only files specified in
       * the CD are valid. Scanning for local file headers is
       * not a correct algorithm.
       *
       * We try to do the above, and resort to "linear" reading
       * of ZIP file only if seek failed or CDE wasn't found.
       */
      let mut magic: u32 = 0;
      /* Check magic number */
      xread(
        zip_fd as libc::c_int,
        &mut magic as *mut u32 as *mut libc::c_void,
        4i32 as size_t,
      );
      /* CDF item? Assume there are no more files, exit */
      if magic == ZIP_CDF_MAGIC as libc::c_int as libc::c_uint {
        break;
      }
      /* Data descriptor? It was a streaming file, go on */
      if magic == ZIP_DD_MAGIC as libc::c_int as libc::c_uint {
        /* skip over duplicate crc32, cmpsize and ucmpsize */
        unzip_skip((3i32 * 4i32) as off_t);
        continue;
      } else {
        if magic != ZIP_FILEHEADER_MAGIC as libc::c_int as libc::c_uint {
          bb_error_msg_and_die(
            b"invalid zip magic %08X\x00" as *const u8 as *const libc::c_char,
            magic as libc::c_int,
          );
        }
        xread(
          zip_fd as libc::c_int,
          zip.raw.as_mut_ptr() as *mut libc::c_void,
          26i32 as size_t,
        );
        if zip.fmt.zip_flags as libc::c_int & 0x8i32 != 0 {
          bb_error_msg_and_die(
            b"zip flag %s is not supported\x00" as *const u8 as *const libc::c_char,
            b"8 (streaming)\x00" as *const u8 as *const libc::c_char,
          );
        }
      }
    } else {
      /* cdf_offset is valid (and we know the file is seekable) */
      let mut cdf: cdf_header_t = cdf_header_t { raw: [0; 42] };
      cdf_offset = read_next_cdf(cdf_offset, &mut cdf);
      if cdf_offset == 0i32 as libc::c_uint {
        break;
      }
      xlseek(
        zip_fd as libc::c_int,
        cdf
          .fmt
          .relative_offset_of_local_header
          .wrapping_add(4i32 as libc::c_uint) as off_t,
        0i32,
      );
      xread(
        zip_fd as libc::c_int,
        zip.raw.as_mut_ptr() as *mut libc::c_void,
        26i32 as size_t,
      );
      if zip.fmt.zip_flags as libc::c_int & 0x8i32 != 0 {
        /* 0x0008 - streaming. [u]cmpsize can be reliably gotten
         * only from Central Directory.
         */
        zip.fmt.crc32 = cdf.fmt.crc32;
        zip.fmt.cmpsize = cdf.fmt.cmpsize;
        zip.fmt.ucmpsize = cdf.fmt.ucmpsize
      }
      // Seen in some zipfiles: central directory 9 byte extra field contains
      // a subfield with ID 0x5455 and 5 data bytes, which is a Unix-style UTC mtime.
      // Local header version:
      //  u16 0x5455 ("UT")
      //  u16 size (1 + 4 * n)
      //  u8  flags: bit 0:mtime is present, bit 1:atime is present, bit 2:ctime is present
      //  u32 mtime
      //  u32 atime
      //  u32 ctime
      // Central header version:
      //  u16 0x5455 ("UT")
      //  u16 size (5 (or 1?))
      //  u8  flags: bit 0:mtime is present, bit 1:atime is present, bit 2:ctime is present
      //  u32 mtime (CDF does not store atime/ctime)
      if cdf.fmt.version_made_by as libc::c_int >> 8i32 == 3i32 {
        /* This archive is created on Unix */
        file_mode = cdf.fmt.external_attributes >> 16i32;
        dir_mode = file_mode
      }
    }
    if zip.fmt.zip_flags as libc::c_int & 0x1i32 != 0 {
      /* 0x0001 - encrypted */
      bb_error_msg_and_die(
        b"zip flag %s is not supported\x00" as *const u8 as *const libc::c_char,
        b"1 (encryption)\x00" as *const u8 as *const libc::c_char,
      );
    }
    /* Read filename */
    free(dst_fn as *mut libc::c_void);
    die_if_bad_fnamesize(zip.fmt.filename_len as libc::c_uint);
    dst_fn = xzalloc((zip.fmt.filename_len as libc::c_int + 1i32) as size_t) as *mut libc::c_char;
    xread(
      zip_fd as libc::c_int,
      dst_fn as *mut libc::c_void,
      zip.fmt.filename_len as size_t,
    );
    /* Skip extra header bytes */
    unzip_skip(zip.fmt.extra_len as off_t);
    /* Guard against "/abspath", "/../" and similar attacks */
    overlapping_strcpy(dst_fn, strip_unsafe_prefix(dst_fn));
    /* Filter zip entries */
    if !find_list_entry(zreject, dst_fn).is_null()
      || !zaccept.is_null() && find_list_entry(zaccept, dst_fn).is_null()
    {
      /* Skip entry */
      current_block = 1883191908174573312;
    } else if opts & OPT_l as libc::c_int as libc::c_uint != 0 {
      /* List entry */
      let mut dtbuf: [libc::c_char; 17] = [0; 17];
      sprintf(
        dtbuf.as_mut_ptr(),
        b"%02u-%02u-%04u %02u:%02u\x00" as *const u8 as *const libc::c_char,
        zip.fmt.moddate as libc::c_int >> 5i32 & 0xfi32,
        zip.fmt.moddate as libc::c_int & 0x1fi32,
        (zip.fmt.moddate as libc::c_int >> 9i32) + 1980i32,
        zip.fmt.modtime as libc::c_int >> 11i32,
        zip.fmt.modtime as libc::c_int >> 5i32 & 0x3fi32,
      );
      if verbose == 0 {
        //      "  Length      Date    Time    Name\n"
        //      "---------  ---------- -----   ----"
        printf(
          b"%9u  %s   %s\n\x00" as *const u8 as *const libc::c_char,
          zip.fmt.ucmpsize,
          dtbuf.as_mut_ptr(),
          printable_string(dst_fn),
        );
      } else {
        let mut method6: [libc::c_char; 7] = [0; 7];
        let mut percents: libc::c_ulong = 0;
        sprintf(
          method6.as_mut_ptr(),
          b"%6u\x00" as *const u8 as *const libc::c_char,
          zip.fmt.method as libc::c_int,
        );
        if zip.fmt.method as libc::c_int == 0i32 {
          strcpy(
            method6.as_mut_ptr(),
            b"Stored\x00" as *const u8 as *const libc::c_char,
          );
        }
        if zip.fmt.method as libc::c_int == 8i32 {
          strcpy(
            method6.as_mut_ptr(),
            b"Defl:N\x00" as *const u8 as *const libc::c_char,
          );
          /* normal, maximum, fast, superfast */
          method6[5] = (*::std::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"NXFS\x00"))
            [(zip.fmt.zip_flags as libc::c_int >> 1i32 & 3i32) as usize]
        } /* happens if ucmpsize < cmpsize */
        percents = zip.fmt.ucmpsize.wrapping_sub(zip.fmt.cmpsize) as libc::c_ulong;
        if (percents as i32) < 0i32 {
          percents = 0i32 as libc::c_ulong
        }
        percents = percents.wrapping_mul(100i32 as libc::c_ulong);
        if zip.fmt.ucmpsize != 0 {
          percents = percents.wrapping_div(zip.fmt.ucmpsize as libc::c_ulong)
        }
        //      " Length   Method    Size  Cmpr    Date    Time   CRC-32   Name\n"
        //      "--------  ------  ------- ---- ---------- ----- --------  ----"
        printf(
          b"%8u  %s%9u%4u%% %s %08x  %s\n\x00" as *const u8 as *const libc::c_char,
          zip.fmt.ucmpsize,
          method6.as_mut_ptr(),
          zip.fmt.cmpsize,
          percents as libc::c_uint,
          dtbuf.as_mut_ptr(),
          zip.fmt.crc32,
          printable_string(dst_fn),
        );
        total_size = total_size.wrapping_add(zip.fmt.cmpsize as libc::c_ulong)
      }
      total_usize = total_usize.wrapping_add(zip.fmt.ucmpsize as libc::c_ulong);
      current_block = 1883191908174573312;
    } else {
      if dst_fd == 1i32 {
        current_block = 3132383208977154520;
      } else {
        /* Strip paths (after -l: unzip -lj a.zip lists full names) */
        if opts & OPT_j as libc::c_int as libc::c_uint != 0 {
          overlapping_strcpy(dst_fn, bb_basename(dst_fn));
        }
        /* Did this strip everything ("DIR/" case)? Then skip */
        if *dst_fn.offset(0) == 0 {
          current_block = 1883191908174573312;
        } else if !last_char_is(dst_fn, '/' as i32).is_null() {
          let mut mode: libc::c_int = 0;
          /* Extract directory */
          mode = get_lstat_mode(dst_fn);
          if mode == -1i32 {
            /* ENOENT */
            if quiet == 0 {
              printf(
                b"   creating: %s\n\x00" as *const u8 as *const libc::c_char,
                printable_string(dst_fn),
              );
            }
            unzip_create_leading_dirs(dst_fn);
            if bb_make_directory(
              dst_fn,
              dir_mode as libc::c_long,
              FILEUTILS_IGNORE_CHMOD_ERR as libc::c_int,
            ) != 0
            {
              xfunc_die();
            }
          } else if !(mode & 0o170000i32 == 0o40000i32) {
            bb_error_msg_and_die(
              b"\'%s\' exists but is not a %s\x00" as *const u8 as *const libc::c_char,
              printable_string(dst_fn),
              b"directory\x00" as *const u8 as *const libc::c_char,
            );
          }
          current_block = 1883191908174573312;
        } else {
          loop
          /* Does target file already exist? */
          {
            mode_0 = get_lstat_mode(dst_fn);
            if mode_0 == -1i32 {
              /* ENOENT: does not exist */
              current_block = 13644292938706749517;
              break;
            } else {
              if overwrite as libc::c_int == O_NEVER as libc::c_int {
                current_block = 1883191908174573312;
                break;
              }
              if !(mode_0 & 0o170000i32 == 0o100000i32) {
                current_block = 8916208649952454714;
                break;
              }
              if overwrite as libc::c_int == O_ALWAYS as libc::c_int {
                current_block = 13644292938706749517;
                break;
              }
              printf(
                b"replace %s? [y]es, [n]o, [A]ll, [N]one, [r]ename: \x00" as *const u8
                  as *const libc::c_char,
                printable_string(dst_fn),
              );
              my_fgets80(key_buf.as_mut_ptr());
              /* User input could take a long time. Is it still a regular file? */
              mode_0 = get_lstat_mode(dst_fn);
              if !(mode_0 & 0o170000i32 == 0o100000i32) {
                current_block = 8916208649952454714;
                break;
              }
              /* Extract (or skip) it */
              match key_buf[0] as libc::c_int {
                65 => {
                  overwrite = O_ALWAYS as libc::c_int as smallint;
                  current_block = 13644292938706749517;
                  break;
                }
                121 => {
                  current_block = 13644292938706749517;
                  break;
                }
                78 => {
                  overwrite = O_NEVER as libc::c_int as smallint;
                  current_block = 1883191908174573312;
                  break;
                }
                110 => {
                  current_block = 1883191908174573312;
                  break;
                }
                114 => {
                  /* Prompt for new name */
                  printf(b"new name: \x00" as *const u8 as *const libc::c_char);
                  my_fgets80(key_buf.as_mut_ptr());
                  free(dst_fn as *mut libc::c_void);
                  dst_fn = xstrdup(key_buf.as_mut_ptr());
                  chomp(dst_fn);
                }
                _ => {
                  printf(
                    b"error: invalid response [%c]\n\x00" as *const u8 as *const libc::c_char,
                    key_buf[0] as libc::c_int,
                  );
                }
              }
            }
          }
          match current_block {
            1883191908174573312 => {}
            _ => {
              match current_block {
                8916208649952454714 => {
                  bb_error_msg_and_die(
                    b"\'%s\' exists but is not a %s\x00" as *const u8 as *const libc::c_char,
                    printable_string(dst_fn),
                    b"regular file\x00" as *const u8 as *const libc::c_char,
                  );
                }
                _ =>
                /* Open file and fall into unzip */
                {
                  unzip_create_leading_dirs(dst_fn);
                  dst_fd = -1i32;
                  if !(file_mode & 0o170000i32 as libc::c_uint == 0o120000i32 as libc::c_uint) {
                    dst_fd = xopen3(
                      dst_fn,
                      0o1i32 | 0o100i32 | 0o1000i32 | 0o400000i32,
                      file_mode as libc::c_int,
                    )
                  }
                }
              }
              current_block = 3132383208977154520;
            }
          }
        }
      }
      match current_block {
        1883191908174573312 => {}
        _ =>
        /* Extracting to STDOUT */
        {
          if quiet == 0 {
            printf(
              b"  inflating: %s\n\x00" as *const u8 as *const libc::c_char,
              printable_string(dst_fn),
            );
          }
          if file_mode & 0o170000i32 as libc::c_uint == 0o120000i32 as libc::c_uint {
            if dst_fd != 1i32 {
              /* not -p? */
              unzip_extract_symlink(&mut symlink_placeholders, &mut zip, dst_fn);
            }
          } else {
            unzip_extract(&mut zip, dst_fd);
            if dst_fd != 1i32 {
              /* closing STDOUT is potentially bad for future business */
              close(dst_fd);
            }
          }
          current_block = 6497888915984600225;
        }
      }
    }
    match current_block {
      1883191908174573312 =>
      /* Skip entry data */
      {
        unzip_skip(zip.fmt.cmpsize as off_t);
      }
      _ => {}
    }
    total_entries = total_entries.wrapping_add(1)
  }
  /* EOF? */
  create_links_from_list(symlink_placeholders);
  if opts & OPT_l as libc::c_int as libc::c_uint != 0 && quiet as libc::c_int <= 1i32 {
    if verbose == 0 {
      //	"  Length      Date    Time    Name\n"
      //	"---------  ---------- -----   ----"
      printf(
        b" --------%21s-------\n%9lu%21s%u files\n\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        total_usize,
        b"\x00" as *const u8 as *const libc::c_char,
        total_entries,
      ); /* happens if usize < size */
    } else {
      let mut percents_0: libc::c_ulong = total_usize.wrapping_sub(total_size);
      if (percents_0 as libc::c_long) < 0 {
        percents_0 = 0i32 as libc::c_ulong
      }
      percents_0 = percents_0.wrapping_mul(100i32 as libc::c_ulong);
      if total_usize != 0 {
        percents_0 = percents_0.wrapping_div(total_usize)
      }
      //	" Length   Method    Size  Cmpr    Date    Time   CRC-32   Name\n"
      //	"--------  ------  ------- ---- ---------- ----- --------  ----"
      printf(
        b"--------          ------- ----%28s----\n%8lu%17lu%4u%%%28s%u files\n\x00" as *const u8
          as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        total_usize,
        total_size,
        percents_0 as libc::c_uint,
        b"\x00" as *const u8 as *const libc::c_char,
        total_entries,
      );
    }
  }
  return 0i32;
}

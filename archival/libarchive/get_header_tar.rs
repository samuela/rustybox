use libc;
extern "C" {
  pub type hardlinks_t;
  #[no_mangle]
  fn strtoull(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_ulonglong;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn lseek(__fd: libc::c_int, __offset: __off64_t, __whence: libc::c_int) -> __off64_t;
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
  /* Some useful definitions */
  /* Macros for min/max.  */
  /* buffer allocation schemes */
  /* glibc uses __errno_location() to get a ptr to errno */
  /* We can just memorize it once - no multithreading in busybox :) */
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xstrndup(s: *const libc::c_char, n: libc::c_int) -> *mut libc::c_char;
  /* NB: can violate const-ness (similarly to strchr) */
  #[no_mangle]
  fn last_char_is(s: *const libc::c_char, c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn overlapping_strcpy(dst: *mut libc::c_char, src: *const libc::c_char);
  // NB: will return short read on error, not -1,
  // if some data was read before error occurred
  #[no_mangle]
  fn full_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn xread(fd: libc::c_int, buf: *mut libc::c_void, count: size_t);
  /* Autodetects gzip/bzip2 formats. fd may be in the middle of the file! */
  #[no_mangle]
  fn setup_unzip_on_fd(fd: libc::c_int, fail_if_not_compressed: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  #[no_mangle]
  fn llist_add_to(old_head: *mut *mut llist_t, data: *mut libc::c_void);
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn concat_path_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_makedev(major: libc::c_uint, minor: libc::c_uint) -> libc::c_ulonglong;
  #[no_mangle]
  fn data_skip(archive_handle: *mut archive_handle_t);
  #[no_mangle]
  fn strip_unsafe_prefix(str: *const libc::c_char) -> *const libc::c_char;
  #[no_mangle]
  fn data_align(archive_handle: *mut archive_handle_t, boundary: libc::c_uint);
}

use crate::libbb::llist::llist_t;

use crate::librb::__off64_t;

use crate::librb::bb_uidgid_t;
use crate::librb::dev_t;
use crate::librb::gid_t;
use crate::librb::int8_t;
use crate::librb::mode_t;
use crate::librb::off_t;
use crate::librb::size_t;
use crate::librb::smallint;
use crate::librb::ssize_t;
use libc::time_t;
use libc::uid_t;
use libc::uint32_t;
 use libc::uint8_t;
use crate::librb::uoff_t;

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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tar_header_t {
  pub name: [libc::c_char; 100],
  pub mode: [libc::c_char; 8],
  pub uid: [libc::c_char; 8],
  pub gid: [libc::c_char; 8],
  pub size: [libc::c_char; 12],
  pub mtime: [libc::c_char; 12],
  pub chksum: [libc::c_char; 8],
  pub typeflag: libc::c_char,
  pub linkname: [libc::c_char; 100],
  pub magic: [libc::c_char; 8],
  pub uname: [libc::c_char; 32],
  pub gname: [libc::c_char; 32],
  pub devmajor: [libc::c_char; 8],
  pub devminor: [libc::c_char; 8],
  pub prefix: [libc::c_char; 155],
  pub padding: [libc::c_char; 12],
}

/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 *
 * FIXME:
 *    In privileged mode if uname and gname map to a uid and gid then use the
 *    mapped value instead of the uid/gid values in tar header
 *
 * References:
 *    GNU tar and star man pages,
 *    Opengroup's ustar interchange format,
 *    http://www.opengroup.org/onlinepubs/007904975/utilities/pax.html
 */
pub type aliased_uint32_t = uint32_t;
/* NB: _DESTROYS_ str[len] character! */
unsafe extern "C" fn getOctal(
  mut str: *mut libc::c_char,
  mut len: libc::c_int,
) -> libc::c_ulonglong {
  let mut v: libc::c_ulonglong = 0;
  let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
  /* NB: leading spaces are allowed. Using strtoull to handle that.
   * The downside is that we accept e.g. "-123" too :(
   */
  *str.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
  v = strtoull(str, &mut end, 8i32);
  /* std: "Each numeric field is terminated by one or more
   * <space> or NUL characters". We must support ' '! */
  if *end as libc::c_int != '\u{0}' as i32 && *end as libc::c_int != ' ' as i32 {
    let mut first: int8_t = *str.offset(0) as int8_t;
    if first as libc::c_int & 0x80i32 == 0 {
      bb_simple_error_msg_and_die(
        b"corrupted octal value in tar header\x00" as *const u8 as *const libc::c_char,
      );
    }
    /*
     * GNU tar uses "base-256 encoding" for very large numbers.
     * Encoding is binary, with highest bit always set as a marker
     * and sign in next-highest bit:
     * 80 00 .. 00 - zero
     * bf ff .. ff - largest positive number
     * ff ff .. ff - minus 1
     * c0 00 .. 00 - smallest negative number
     *
     * Example of tar file with 8914993153 (0x213600001) byte file.
     * Field starts at offset 7c:
     * 00070  30 30 30 00 30 30 30 30  30 30 30 00 80 00 00 00  |000.0000000.....|
     * 00080  00 00 00 02 13 60 00 01  31 31 31 32 30 33 33 36  |.....`..11120336|
     *
     * NB: tarballs with NEGATIVE unix times encoded that way were seen!
     */
    /* Sign-extend 7bit 'first' to 64bit 'v' (that is, using 6th bit as sign): */
    first = ((first as libc::c_int) << 1i32) as int8_t; /* now 7th bit = 6th bit */
    first = (first as libc::c_int >> 1i32) as int8_t; /* sign-extend 8 bits to 64 */
    v = first as libc::c_ulonglong;
    loop {
      len -= 1;
      if !(len != 0i32) {
        break;
      }
      str = str.offset(1);
      v = (v << 8i32).wrapping_add(*str as uint8_t as libc::c_ulonglong)
    }
  }
  return v;
}
/* "global" is 0 or 1 */
unsafe extern "C" fn process_pax_hdr(
  mut archive_handle: *mut archive_handle_t,
  mut sz: libc::c_uint,
  mut global: libc::c_int,
) {
  let mut blk_sz: libc::c_uint = sz.wrapping_add(511i32 as libc::c_uint) & !511i32 as libc::c_uint;
  let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  buf = xmalloc(blk_sz.wrapping_add(1i32 as libc::c_uint) as size_t) as *mut libc::c_char;
  p = buf;
  xread(
    (*archive_handle).src_fd,
    buf as *mut libc::c_void,
    blk_sz as size_t,
  );
  (*archive_handle).offset += blk_sz as libc::c_long;
  /* prevent bb_strtou from running off the buffer */
  *buf.offset(sz as isize) = '\u{0}' as i32 as libc::c_char;
  while sz != 0i32 as libc::c_uint {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_uint = 0;
    /* Every record has this format: "LEN NAME=VALUE\n" */
    len = bb_strtou(p, &mut end, 10i32);
    /* expect errno to be EINVAL, because the character
     * following the digits should be a space
     */
    p = p.offset(len as isize);
    sz = sz.wrapping_sub(len);
    if ((sz | len) as libc::c_int) < 0i32
      || len == 0i32 as libc::c_uint
      || *bb_errno != 22i32
      || *end as libc::c_int != ' ' as i32
    {
      bb_simple_error_msg(
        b"malformed extended header, skipped\x00" as *const u8 as *const libc::c_char,
      );
      break;
    } else {
      /* overwrite the terminating newline with NUL
       * (we do not bother to check that it *was* a newline)
       */
      *p.offset(-1i32 as isize) = '\u{0}' as i32 as libc::c_char; /* can be "char", "int" seems give smaller code */
      value = end.offset(1);
      if !(global == 0) {
        continue;
      }
      if !is_prefixed_with(value, b"path=\x00" as *const u8 as *const libc::c_char).is_null() {
        value = value.offset(
          (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as isize,
        );
        free((*archive_handle).tar__longname as *mut libc::c_void);
        (*archive_handle).tar__longname = xstrdup(value)
      } else {
        if is_prefixed_with(value, b"linkpath=\x00" as *const u8 as *const libc::c_char).is_null() {
          continue;
        }
        value = value.offset(
          (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as isize,
        );
        free((*archive_handle).tar__linkname as *mut libc::c_void);
        (*archive_handle).tar__linkname = xstrdup(value)
      }
    }
  }
  free(buf as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn get_header_tar(mut archive_handle: *mut archive_handle_t) -> libc::c_char {
  let mut current_block: u64;
  let mut file_header: *mut file_header_t = (*archive_handle).file_header;
  let mut tar: tar_header_t = tar_header_t {
    name: [0; 100],
    mode: [0; 8],
    uid: [0; 8],
    gid: [0; 8],
    size: [0; 12],
    mtime: [0; 12],
    chksum: [0; 8],
    typeflag: 0,
    linkname: [0; 100],
    magic: [0; 8],
    uname: [0; 32],
    gname: [0; 32],
    devmajor: [0; 8],
    devminor: [0; 8],
    prefix: [0; 155],
    padding: [0; 12],
  };
  let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut tar_typeflag: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  let mut sum_u: libc::c_int = 0;
  let mut sum: libc::c_int = 0;
  let mut sum_s: libc::c_int = 0;
  let mut parse_names: libc::c_int = 0;
  /* Our "private data" */
  'c_8845: loop
  /* return get_header_tar(archive_handle); */
  /* Align header */
  {
    data_align(archive_handle, 512i32 as libc::c_uint);
    loop
    /* return get_header_tar(archive_handle); */
    /* to prevent misdetection of bz2 sig */
    {
      *(&mut tar as *mut tar_header_t as *mut aliased_uint32_t) = 0i32 as aliased_uint32_t;
      i = full_read(
        (*archive_handle).src_fd,
        &mut tar as *mut tar_header_t as *mut libc::c_void,
        512i32 as size_t,
      ) as libc::c_int;
      /* If GNU tar sees EOF in above read, it says:
       * "tar: A lone zero block at N", where N = kilobyte
       * where EOF was met (not EOF block, actual EOF!),
       * and exits with EXIT_SUCCESS.
       * We will mimic exit(EXIT_SUCCESS), although we will not mimic
       * the message and we don't check whether we indeed
       * saw zero block directly before this. */
      if i == 0i32 {
        /* GNU tar 1.29 will be silent if tar archive ends abruptly
         * (if there are no zero blocks at all, and last read returns zero,
         * not short read 0 < len < 512). Complain only if
         * the very first read fails. Grrr.
         */
        if (*archive_handle).offset == 0i32 as libc::c_long {
          bb_simple_error_msg(b"short read\x00" as *const u8 as *const libc::c_char);
        }
        /* this merely signals end of archive, not exit(1): */
        return 1i32 as libc::c_char;
      }
      if !(i != 512i32) {
        (*archive_handle).offset += i as libc::c_long;
        /* If there is no filename its an empty header */
        if tar.name[0] as libc::c_int == 0i32
          && tar.prefix[0] as libc::c_int == 0i32
          && (*archive_handle).tar__longname.is_null()
        {
          if (*archive_handle).tar__end != 0 {
            /* Second consecutive empty header - end of archive.
             * Read until the end to empty the pipe from gz or bz2
             */
            while full_read(
              (*archive_handle).src_fd,
              &mut tar as *mut tar_header_t as *mut libc::c_void,
              512i32 as size_t,
            ) == 512i32 as libc::c_long
            {}
            return 1i32 as libc::c_char;
            /* "end of archive" */
          }
          (*archive_handle).tar__end = 1i32 as smallint;
          return 0i32 as libc::c_char;
          /* "decoded one header" */
        }
        (*archive_handle).tar__end = 0i32 as smallint;
        /* Check header has valid magic, "ustar" is for the proper tar,
         * five NULs are for the old tar format  */
        if !(is_prefixed_with(
          tar.magic.as_mut_ptr(),
          b"ustar\x00" as *const u8 as *const libc::c_char,
        )
        .is_null()
          && (1i32 == 0
            || memcmp(
              tar.magic.as_mut_ptr() as *const libc::c_void,
              b"\x00\x00\x00\x00\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
              5i32 as libc::c_ulong,
            ) != 0i32))
        {
          /* Do checksum on headers.
           * POSIX says that checksum is done on unsigned bytes, but
           * Sun and HP-UX gets it wrong... more details in
           * GNU tar source. */
          sum_u = (' ' as i32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            as libc::c_int;
          sum_s = sum_u;
          i = 0i32;
          while i < 148i32 {
            sum_u += *(&mut tar as *mut tar_header_t as *mut libc::c_uchar).offset(i as isize)
              as libc::c_int;
            sum_s += *(&mut tar as *mut tar_header_t as *mut libc::c_schar).offset(i as isize)
              as libc::c_int;
            i += 1
          }
          i = 156i32;
          while i < 512i32 {
            sum_u += *(&mut tar as *mut tar_header_t as *mut libc::c_uchar).offset(i as isize)
              as libc::c_int;
            sum_s += *(&mut tar as *mut tar_header_t as *mut libc::c_schar).offset(i as isize)
              as libc::c_int;
            i += 1
          }
          /* Most tarfiles have tar.chksum NUL or space terminated, but
           * github.com decided to be "special" and have unterminated field:
           * 0090: 30343300 30303031 33323731 30000000 |043.000132710...|
           *                                                ^^^^^^^^|
           * Need to use GET_OCTAL. This overwrites tar.typeflag ---+
           * (the '0' char immediately after chksum in example above) with NUL.
           */
          tar_typeflag = tar.typeflag as uint8_t as libc::c_int; /* save it */
          sum = getOctal(
            tar.chksum.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as libc::c_int,
          ) as libc::c_int;
          if sum_u != sum && sum_s != sum {
            bb_simple_error_msg_and_die(
              b"invalid tar header checksum\x00" as *const u8 as *const libc::c_char,
            );
          }
          /* GET_OCTAL trashes subsequent field, therefore we call it
           * on fields in reverse order */
          if tar.devmajor[0] != 0 {
            let mut t: libc::c_char = tar.prefix[0];
            /* we trash prefix[0] here, but we DO need it later! */
            let mut minor: libc::c_uint = getOctal(
              tar.devminor.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as libc::c_int,
            ) as libc::c_uint;
            let mut major: libc::c_uint = getOctal(
              tar.devmajor.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as libc::c_int,
            ) as libc::c_uint;
            (*file_header).device = bb_makedev(major, minor) as dev_t;
            tar.prefix[0] = t
          }
          /* 0 is reserved for high perf file, treat as normal file */
          if tar_typeflag == '\u{0}' as i32 {
            tar_typeflag = '0' as i32
          }
          parse_names = (tar_typeflag >= '0' as i32 && tar_typeflag <= '7' as i32) as libc::c_int;
          (*file_header).link_target = 0 as *mut libc::c_char;
          if (*archive_handle).tar__linkname.is_null()
            && parse_names != 0
            && tar.linkname[0] as libc::c_int != 0
          {
            (*file_header).link_target = xstrndup(
              tar.linkname.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong as libc::c_int,
            )
            /* FIXME: what if we have non-link object with link_target? */
            /* Will link_target be free()ed? */
          }
          (*file_header).tar__uname = if tar.uname[0] as libc::c_int != 0 {
            xstrndup(
              tar.uname.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
            )
          } else {
            0 as *mut libc::c_char
          };
          (*file_header).tar__gname = if tar.gname[0] as libc::c_int != 0 {
            xstrndup(
              tar.gname.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
            )
          } else {
            0 as *mut libc::c_char
          };
          (*file_header).mtime = getOctal(
            tar.mtime.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as libc::c_int,
          ) as time_t;
          (*file_header).size = getOctal(
            tar.size.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as libc::c_int,
          ) as off_t;
          (*file_header).gid = getOctal(
            tar.gid.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as libc::c_int,
          ) as gid_t;
          (*file_header).uid = getOctal(
            tar.uid.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as libc::c_int,
          ) as uid_t;
          /* Set bits 0-11 of the files mode */
          (*file_header).mode = (0o7777i32 as libc::c_ulonglong
            & getOctal(
              tar.mode.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as libc::c_int,
            )) as mode_t;
          (*file_header).name = 0 as *mut libc::c_char;
          if (*archive_handle).tar__longname.is_null() && parse_names != 0 {
            /* we trash mode[0] here, it's ok */
            //tar.name[sizeof(tar.name)] = '\0'; - gcc 4.3.0 would complain
            tar.mode[0] = '\u{0}' as i32 as libc::c_char;
            if tar.prefix[0] != 0 {
              /* and padding[0] */
              //tar.prefix[sizeof(tar.prefix)] = '\0'; - gcc 4.3.0 would complain
              tar.padding[0] = '\u{0}' as i32 as libc::c_char;
              (*file_header).name = concat_path_file(tar.prefix.as_mut_ptr(), tar.name.as_mut_ptr())
            } else {
              (*file_header).name = xstrdup(tar.name.as_mut_ptr())
            }
          }
          /* Set bits 12-15 of the files mode */
          /* (typeflag was not trashed because chksum does not use getOctal) */
          match tar_typeflag {
            49 => {
              /* hardlink */
              /* we mark hardlinks as regular files with zero size and a link name */
              (*file_header).mode |= 0o100000i32 as libc::c_uint;
              /* on size of link fields from star(4)
               * ... For tar archives written by pre POSIX.1-1988
               * implementations, the size field usually contains the size of
               * the file and needs to be ignored as no data may follow this
               * header type.  For POSIX.1- 1988 compliant archives, the size
               * field needs to be 0.  For POSIX.1-2001 compliant archives,
               * the size field may be non zero, indicating that file data is
               * included in the archive.
               * i.e; always assume this is zero for safety.
               */
              current_block = 8559883396898198220;
              break 'c_8845;
            }
            55 | 48 => {
              /* case 0: */
              if !last_char_is((*file_header).name, '/' as i32).is_null() {
                current_block = 7455596087214207069;
                break 'c_8845;
              } else {
                current_block = 17239133558811367971;
                break 'c_8845;
              }
            }
            50 => {
              (*file_header).mode |= 0o120000i32 as libc::c_uint;
              /* have seen tarballs with size field containing
               * the size of the link target's name */
              current_block = 8559883396898198220;
              break 'c_8845;
            }
            51 => {
              (*file_header).mode |= 0o20000i32 as libc::c_uint;
              current_block = 8559883396898198220;
              break 'c_8845;
            }
            52 => {
              (*file_header).mode |= 0o60000i32 as libc::c_uint;
              current_block = 8559883396898198220;
              break 'c_8845;
            }
            53 => {
              current_block = 7455596087214207069;
              break 'c_8845;
            }
            54 => {
              (*file_header).mode |= 0o10000i32 as libc::c_uint;
              current_block = 8559883396898198220;
              break 'c_8845;
            }
            103 => {
              /* pax global header */
              current_block = 4689751644202276831;
            }
            120 => {
              current_block = 4689751644202276831;
            }
            76 => {
              /* See http://www.gnu.org/software/tar/manual/html_node/Extensions.html */
              /* free: paranoia: tar with several consecutive longnames */
              free((*archive_handle).tar__longname as *mut libc::c_void);
              /* For paranoia reasons we allocate extra NUL char */
              (*archive_handle).tar__longname =
                xzalloc(((*file_header).size + 1i32 as libc::c_long) as size_t)
                  as *mut libc::c_char;
              /* We read ASCIZ string, including NUL */
              xread(
                (*archive_handle).src_fd,
                (*archive_handle).tar__longname as *mut libc::c_void,
                (*file_header).size as size_t,
              );
              (*archive_handle).offset += (*file_header).size;
              /* return get_header_tar(archive_handle); */
              /* gcc 4.1.1 didn't optimize it into jump */
              /* so we will do it ourself, this also saves stack */
              break;
            }
            75 => {
              free((*archive_handle).tar__linkname as *mut libc::c_void);
              (*archive_handle).tar__linkname =
                xzalloc(((*file_header).size + 1i32 as libc::c_long) as size_t)
                  as *mut libc::c_char;
              xread(
                (*archive_handle).src_fd,
                (*archive_handle).tar__linkname as *mut libc::c_void,
                (*file_header).size as size_t,
              );
              (*archive_handle).offset += (*file_header).size;
              break;
            }
            86 => {
              current_block = 4488496028633655612;
            }
            _ => {
              bb_error_msg_and_die(
                b"unknown typeflag: 0x%x\x00" as *const u8 as *const libc::c_char,
                tar_typeflag,
              );
            }
          }
          match current_block {
            4689751644202276831 => {
              /* pax extended header */
              if !((*file_header).size as uoff_t > 0xfffffi32 as libc::c_ulong) {
                process_pax_hdr(
                  archive_handle,
                  (*file_header).size as libc::c_uint,
                  (tar_typeflag == 'g' as i32) as libc::c_int,
                );
                continue;
              }
              /* Fall through to skip it */
            }
            _ => {}
          }
          /* Volume header */
          let mut sz: off_t = 0; /* sz /= 512 but w/o contortions for signed div */
          bb_error_msg(
            b"warning: skipping header \'%c\'\x00" as *const u8 as *const libc::c_char,
            tar_typeflag,
          );
          sz = (*file_header).size + 511i32 as libc::c_long & !(511i32 as off_t);
          (*archive_handle).offset += sz;
          sz >>= 9i32;
          loop {
            let fresh0 = sz;
            sz = sz - 1;
            if !(fresh0 != 0) {
              break;
            }
            xread(
              (*archive_handle).src_fd,
              &mut tar as *mut tar_header_t as *mut libc::c_void,
              512i32 as size_t,
            );
          }
          continue;
        }
      }
      /* Two different causes for lseek() != 0:
       * unseekable fd (would like to support that too, but...),
       * or not first block (false positive, it's not .gz/.bz2!) */
      if lseek((*archive_handle).src_fd, -i as __off64_t, 1i32) != 0i32 as libc::c_long {
        current_block = 25209135276526723; /* paranoia */
        break 'c_8845;
      }
      if setup_unzip_on_fd((*archive_handle).src_fd, 0i32) != 0i32 {
        current_block = 25209135276526723;
        break 'c_8845;
      }
      (*archive_handle).offset = 0i32 as off_t
    }
  }
  match current_block {
    25209135276526723 => {
      bb_simple_error_msg_and_die(b"invalid tar magic\x00" as *const u8 as *const libc::c_char);
    }
    7455596087214207069 => {
      (*file_header).mode |= 0o40000i32 as libc::c_uint;
      current_block = 8559883396898198220;
    }
    17239133558811367971 => {
      (*file_header).mode |= 0o100000i32 as libc::c_uint;
      current_block = 3024367268842933116;
    }
    _ => {}
  }
  match current_block {
    8559883396898198220 => (*file_header).size = 0i32 as off_t,
    _ => {}
  }
  if !(*archive_handle).tar__longname.is_null() {
    (*file_header).name = (*archive_handle).tar__longname;
    (*archive_handle).tar__longname = 0 as *mut libc::c_char
  }
  if !(*archive_handle).tar__linkname.is_null() {
    (*file_header).link_target = (*archive_handle).tar__linkname;
    (*archive_handle).tar__linkname = 0 as *mut libc::c_char
  }
  /* Everything up to and including last ".." component is stripped */
  overlapping_strcpy(
    (*file_header).name,
    strip_unsafe_prefix((*file_header).name),
  );
  //TODO: do the same for file_header->link_target?
  /* Strip trailing '/' in directories */
  /* Must be done after mode is set as '/' is used to check if it's a directory */
  cp = last_char_is((*file_header).name, '/' as i32);
  if (*archive_handle).filter.expect("non-null function pointer")(archive_handle) as libc::c_int
    == 0i32
  {
    (*archive_handle)
      .action_header
      .expect("non-null function pointer")(file_header);
    /* Note that we kill the '/' only after action_header() */
    /* (like GNU tar 1.15.1: verbose mode outputs "dir/dir/") */
    if !cp.is_null() {
      *cp = '\u{0}' as i32 as libc::c_char
    } /* Caller isn't interested in list of unpacked files */
    (*archive_handle)
      .action_data
      .expect("non-null function pointer")(archive_handle);
    if !(*archive_handle).accept.is_null()
      || !(*archive_handle).reject.is_null()
      || (*archive_handle).ah_flags & (1i32 << 8i32) as libc::c_uint != 0
    {
      llist_add_to(
        &mut (*archive_handle).passed,
        (*file_header).name as *mut libc::c_void,
      );
    } else {
      free((*file_header).name as *mut libc::c_void);
    }
  } else {
    data_skip(archive_handle);
    free((*file_header).name as *mut libc::c_void);
  }
  (*archive_handle).offset += (*file_header).size;
  free((*file_header).link_target as *mut libc::c_void);
  /* Do not free(file_header->name)!
   * It might be inserted in archive_handle->passed - see above */
  free((*file_header).tar__uname as *mut libc::c_void);
  free((*file_header).tar__gname as *mut libc::c_void);
  return 0i32 as libc::c_char;
  /* "decoded one header" */
}

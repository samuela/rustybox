use crate::archival::libarchive::bb_archive::archive_handle_t;
use crate::archival::libarchive::bb_archive::file_header_t;
use crate::archival::libarchive::bb_archive::transformer_state_t;
use crate::libbb::llist::llist_t;
use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::xfuncs_printf::xmalloc;
use crate::librb::fd_pair;
use crate::librb::signal::__sighandler_t;
use crate::librb::size_t;
use crate::librb::smallint;
use crate::librb::uoff_t;
use libc;
use libc::close;
use libc::fclose;
use libc::fprintf;
use libc::free;
use libc::getuid;
use libc::ino_t;
use libc::off_t;
use libc::pid_t;
use libc::putenv;
use libc::puts;
use libc::sprintf;
use libc::stat;
use libc::strchr;
use libc::strcmp;
use libc::strcpy;
use libc::FILE;
extern "C" {

  #[no_mangle]
  fn fnmatch(
    __pattern: *const libc::c_char,
    __name: *const libc::c_char,
    __flags: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn dup(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn execlp(__file: *const libc::c_char, __arg: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn _exit(_: libc::c_int) -> !;

  #[no_mangle]
  fn vfork() -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn gnu_dev_major(__dev: libc::dev_t) -> libc::c_uint;
  #[no_mangle]
  fn gnu_dev_minor(__dev: libc::dev_t) -> libc::c_uint;
  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
  #[no_mangle]
  static mut stdout: *mut FILE;
  #[no_mangle]
  static mut stderr: *mut FILE;

  #[no_mangle]
  fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  static mut bb_got_signal: smallint;

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];

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

pub type C2RustUnnamed = libc::c_uint;
pub const ACTION_DANGLING_OK: C2RustUnnamed = 64;
pub const ACTION_QUIET: C2RustUnnamed = 32;
pub const ACTION_DEPTHFIRST: C2RustUnnamed = 8;
pub const ACTION_FOLLOWLINKS_L0: C2RustUnnamed = 4;
pub const ACTION_FOLLOWLINKS: C2RustUnnamed = 2;
pub const ACTION_RECURSE: C2RustUnnamed = 1;

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_0 {
  pub b: [u8; 8],
  pub b16: [u16; 4],
  pub b32: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HardLinkInfo {
  pub next: *mut HardLinkInfo,
  pub dev: libc::dev_t,
  pub ino: ino_t,
  pub name: [libc::c_char; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TarBallInfo {
  pub tarFd: libc::c_int,
  pub verboseFlag: libc::c_int,
  pub excludeList: *const llist_t,
  pub hlInfoHead: *mut HardLinkInfo,
  pub hlInfo: *mut HardLinkInfo,
  pub tarFileStatBuf: stat,
}
/* A nice enum with all the possible tar file content types */
pub type C2RustUnnamed_1 = libc::c_uint;
/* GNU long (>100 chars) file name */
/* GNU long (>100 chars) link name */
pub const GNULONGNAME: C2RustUnnamed_1 = 76;
/* reserved */
pub const GNULONGLINK: C2RustUnnamed_1 = 75;
/* FIFO special */
pub const CONTTYPE: C2RustUnnamed_1 = 55;
/* directory */
pub const FIFOTYPE: C2RustUnnamed_1 = 54;
/* block special */
pub const DIRTYPE: C2RustUnnamed_1 = 53;
/* character special */
pub const BLKTYPE: C2RustUnnamed_1 = 52;
/* symbolic link */
pub const CHRTYPE: C2RustUnnamed_1 = 51;
/* hard link */
pub const SYMTYPE: C2RustUnnamed_1 = 50;
/* regular file (ancient bug compat) */
pub const LNKTYPE: C2RustUnnamed_1 = 49;
/* regular file */
pub const REGTYPE0: C2RustUnnamed_1 = 0;
pub const REGTYPE: C2RustUnnamed_1 = 48;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prefilled {
  pub mode: [libc::c_char; 8],
  pub uid: [libc::c_char; 8],
  pub gid: [libc::c_char; 8],
  pub size: [libc::c_char; 12],
  pub mtime: [libc::c_char; 12],
  /* 136-147 */
}
//usage:#define tar_trivial_usage
//usage:	IF_FEATURE_TAR_CREATE("c|") "x|t [-"
//usage:	IF_FEATURE_SEAMLESS_Z("Z")
//usage:	IF_FEATURE_SEAMLESS_GZ("z")
//usage:	IF_FEATURE_SEAMLESS_XZ("J")
//usage:	IF_FEATURE_SEAMLESS_BZ2("j")
//usage:	"a"
//usage:	IF_FEATURE_TAR_CREATE("h")
//usage:	IF_FEATURE_TAR_NOPRESERVE_TIME("m")
//usage:	"vokO] "
//usage:	"[-f TARFILE] [-C DIR] "
//usage:	IF_FEATURE_TAR_FROM("[-T FILE] [-X FILE] "IF_FEATURE_TAR_LONG_OPTIONS("[--exclude PATTERN]... "))
//usage:	"[FILE]..."
//usage:#define tar_full_usage "\n\n"
//usage:	IF_FEATURE_TAR_CREATE("Create, extract, ")
//usage:	IF_NOT_FEATURE_TAR_CREATE("Extract ")
//usage:	"or list files from a tar file"
//usage:     "\n"
//usage:	IF_FEATURE_TAR_CREATE(
//usage:     "\n	c	Create"
//usage:	)
//usage:     "\n	x	Extract"
//usage:     "\n	t	List"
//usage:     "\n	-f FILE	Name of TARFILE ('-' for stdin/out)"
//usage:     "\n	-C DIR	Change to DIR before operation"
//usage:     "\n	-v	Verbose"
//usage:     "\n	-O	Extract to stdout"
//usage:	IF_FEATURE_TAR_NOPRESERVE_TIME(
//usage:     "\n	-m	Don't restore mtime"
//usage:	)
//usage:     "\n	-o	Don't restore user:group"
// /////:-p - accepted but ignored, restores mode (aliases in GNU tar: --preserve-permissions, --same-permissions)
//usage:     "\n	-k	Don't replace existing files"
//usage:	IF_FEATURE_SEAMLESS_Z(
//usage:     "\n	-Z	(De)compress using compress"
//usage:	)
//usage:	IF_FEATURE_SEAMLESS_GZ(
//usage:     "\n	-z	(De)compress using gzip"
//usage:	)
//usage:	IF_FEATURE_SEAMLESS_XZ(
//usage:     "\n	-J	(De)compress using xz"
//usage:	)
//usage:	IF_FEATURE_SEAMLESS_BZ2(
//usage:     "\n	-j	(De)compress using bzip2"
//usage:	)
//usage:     "\n	-a	(De)compress based on extension"
//usage:	IF_FEATURE_TAR_CREATE(
//usage:     "\n	-h	Follow symlinks"
//usage:	)
//usage:	IF_FEATURE_TAR_FROM(
//usage:     "\n	-T FILE	File with names to include"
//usage:     "\n	-X FILE	File with glob patterns to exclude"
//usage:	IF_FEATURE_TAR_LONG_OPTIONS(
//usage:     "\n	--exclude PATTERN	Glob pattern to exclude"
//usage:	)
//usage:	)
//usage:
//usage:#define tar_example_usage
//usage:       "$ zcat /tmp/tarball.tar.gz | tar -xf -\n"
//usage:       "$ tar -cf /tmp/tarball.tar /usr/local\n"
// Supported but aren't in --help:
//	lzma
//	no-recursion
//	numeric-owner
//	no-same-permissions
//	overwrite
//IF_FEATURE_TAR_TO_COMMAND(
//	to-command
//)
pub type C2RustUnnamed_2 = libc::c_uint;
// overwrite
pub const OPT_ANY_COMPRESS: C2RustUnnamed_2 = 575488;
// no-same-permissions
pub const OPT_OVERWRITE: C2RustUnnamed_2 = 16777216;
// numeric-owner
pub const OPT_NOPRESERVE_PERM: C2RustUnnamed_2 = 8388608;
// to-command
pub const OPT_NUMERIC_OWNER: C2RustUnnamed_2 = 4194304;
// no-recursion
pub const OPT_2COMMAND: C2RustUnnamed_2 = 2097152;
// lzma
pub const OPT_NORECURSION: C2RustUnnamed_2 = 1048576;
// strip-components
pub const OPT_LZMA: C2RustUnnamed_2 = 524288;
// m
pub const OPT_STRIP_COMPONENTS: C2RustUnnamed_2 = 262144;
// a
pub const OPT_NOPRESERVE_TIME: C2RustUnnamed_2 = 131072;
// Z
pub const OPT_AUTOCOMPRESS_BY_EXT: C2RustUnnamed_2 = 65536;
// J
pub const OPT_COMPRESS: C2RustUnnamed_2 = 0;
// z
pub const OPT_XZ: C2RustUnnamed_2 = 32768;
// X
pub const OPT_GZIP: C2RustUnnamed_2 = 16384;
// T
pub const OPT_EXCLUDE_FROM: C2RustUnnamed_2 = 8192;
// j
pub const OPT_INCLUDE_FROM: C2RustUnnamed_2 = 4096;
// h
pub const OPT_BZIP2: C2RustUnnamed_2 = 2048;
// c
pub const OPT_DEREFERENCE: C2RustUnnamed_2 = 1024;
// k
pub const OPT_CREATE: C2RustUnnamed_2 = 512;
// v
pub const OPT_KEEP_OLD: C2RustUnnamed_2 = 256;
// p
pub const OPT_VERBOSE: C2RustUnnamed_2 = 128;
// o == no-same-owner
pub const OPT_P: C2RustUnnamed_2 = 64;
// O
pub const OPT_NOPRESERVE_OWNER: C2RustUnnamed_2 = 32;
// f
pub const OPT_2STDOUT: C2RustUnnamed_2 = 16;
// C
pub const OPT_TARNAME: C2RustUnnamed_2 = 8;
// x
pub const OPT_BASEDIR: C2RustUnnamed_2 = 4;
// t
pub const OPT_EXTRACT: C2RustUnnamed_2 = 2;
pub const OPT_TEST: C2RustUnnamed_2 = 1;
pub const OPTBIT_OVERWRITE: C2RustUnnamed_2 = 24;
pub const OPTBIT_NOPRESERVE_PERM: C2RustUnnamed_2 = 23;
pub const OPTBIT_NUMERIC_OWNER: C2RustUnnamed_2 = 22;
pub const OPTBIT_2COMMAND: C2RustUnnamed_2 = 21;
pub const OPTBIT_NORECURSION: C2RustUnnamed_2 = 20;
pub const OPTBIT_LZMA: C2RustUnnamed_2 = 19;
pub const OPTBIT_STRIP_COMPONENTS: C2RustUnnamed_2 = 18;
pub const OPTBIT_NOPRESERVE_TIME: C2RustUnnamed_2 = 17;
// 16th bit
pub const OPTBIT_AUTOCOMPRESS_BY_EXT: C2RustUnnamed_2 = 16;
pub const OPTBIT_XZ: C2RustUnnamed_2 = 15;
pub const OPTBIT_GZIP: C2RustUnnamed_2 = 14;
pub const OPTBIT_EXCLUDE_FROM: C2RustUnnamed_2 = 13;
pub const OPTBIT_INCLUDE_FROM: C2RustUnnamed_2 = 12;
pub const OPTBIT_BZIP2: C2RustUnnamed_2 = 11;
pub const OPTBIT_DEREFERENCE: C2RustUnnamed_2 = 10;
pub const OPTBIT_CREATE: C2RustUnnamed_2 = 9;
pub const OPTBIT_KEEP_OLD: C2RustUnnamed_2 = 8;
/* Might be faster (and bigger) if the dev/ino were stored in numeric order;) */
unsafe fn addHardLinkInfo(
  mut hlInfoHeadPtr: *mut *mut HardLinkInfo,
  mut statbuf: *mut stat,
  mut fileName: *const libc::c_char,
) {
  /* Note: hlInfoHeadPtr can never be NULL! */
  let mut hlInfo: *mut HardLinkInfo = std::ptr::null_mut();
  hlInfo = xmalloc(
    (::std::mem::size_of::<HardLinkInfo>() as libc::c_ulong).wrapping_add(strlen(fileName)),
  ) as *mut HardLinkInfo;
  (*hlInfo).next = *hlInfoHeadPtr;
  *hlInfoHeadPtr = hlInfo;
  (*hlInfo).dev = (*statbuf).st_dev;
  (*hlInfo).ino = (*statbuf).st_ino;
  //	hlInfo->linkCount = statbuf->st_nlink;
  strcpy((*hlInfo).name.as_mut_ptr(), fileName);
}

/* Might be faster (and bigger) if the dev/ino were stored in numeric order ;) */
unsafe fn findHardLinkInfo(
  mut hlInfo: *mut HardLinkInfo,
  mut statbuf: *mut stat,
) -> *mut HardLinkInfo {
  while !hlInfo.is_null() {
    if (*statbuf).st_ino == (*hlInfo).ino && (*statbuf).st_dev == (*hlInfo).dev {
      break;
    }
    hlInfo = (*hlInfo).next
  }
  return hlInfo;
}
/* Put an octal string into the specified buffer.
 * The number is zero padded and possibly NUL terminated.
 * Stores low-order bits only if whole value does not fit. */
unsafe fn putOctal(mut cp: *mut libc::c_char, mut len: libc::c_int, mut value: off_t) {
  let mut tempBuffer: [libc::c_char; 25] = [0; 25];
  let mut tempString: *mut libc::c_char = tempBuffer.as_mut_ptr();
  let mut width: libc::c_int = 0;
  width = sprintf(
    tempBuffer.as_mut_ptr(),
    b"%0*lo\x00" as *const u8 as *const libc::c_char,
    len,
    value,
  );
  tempString = tempString.offset((width - len) as isize);
  /* If string has leading zeroes, we can drop one */
  /* and field will have trailing '\0' */
  /* (increases chances of compat with other tars) */
  if *tempString.offset(0) as libc::c_int == '0' as i32 {
    tempString = tempString.offset(1)
  }
  /* Copy the string to the field */
  memcpy(
    cp as *mut libc::c_void,
    tempString as *const libc::c_void,
    len as libc::c_ulong,
  );
}
unsafe fn chksum_and_xwrite(mut fd: libc::c_int, mut hp: *mut tar_header_t) {
  /* POSIX says that checksum is done on unsigned bytes
   * (Sun and HP-UX gets it wrong... more details in
   * GNU tar source) */
  let mut cp: *const libc::c_uchar = std::ptr::null();
  let mut chksum: libc::c_int = 0;
  let mut size: libc::c_int = 0;
  strcpy(
    (*hp).magic.as_mut_ptr(),
    b"ustar  \x00" as *const u8 as *const libc::c_char,
  );
  /* Calculate and store the checksum (i.e., the sum of all of the bytes of
   * the header).  The checksum field must be filled with blanks for the
   * calculation.  The checksum field is formatted differently from the
   * other fields: it has 6 digits, a null, then a space -- rather than
   * digits, followed by a null like the other fields... */
  memset(
    (*hp).chksum.as_mut_ptr() as *mut libc::c_void,
    ' ' as i32,
    ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
  );
  cp = hp as *const libc::c_uchar;
  chksum = 0;
  size = ::std::mem::size_of::<tar_header_t>() as libc::c_ulong as libc::c_int;
  loop {
    let fresh0 = cp;
    cp = cp.offset(1);
    chksum += *fresh0 as libc::c_int;
    size -= 1;
    if !(size != 0) {
      break;
    }
  }
  putOctal(
    (*hp).chksum.as_mut_ptr(),
    (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int,
    chksum as off_t,
  );
  /* Now write the header out to disk */
  crate::libbb::xfuncs_printf::xwrite(
    fd,
    hp as *const libc::c_void,
    ::std::mem::size_of::<tar_header_t>() as libc::c_ulong,
  );
}
unsafe fn writeLongname(
  mut fd: libc::c_int,
  mut type_0: libc::c_int,
  mut name: *const libc::c_char,
  mut dir: libc::c_int,
) {
  let mut header: tar_header_t = tar_header_t {
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
  let mut size: libc::c_int = 0;
  memset(
    &mut header as *mut tar_header_t as *mut libc::c_void,
    0,
    ::std::mem::size_of::<tar_header_t>() as libc::c_ulong,
  );
  header.typeflag = type_0 as libc::c_char;
  strcpy(
    header.name.as_mut_ptr(),
    b"././@LongLink\x00" as *const u8 as *const libc::c_char,
  );
  /* This sets mode/uid/gid/mtime to "00...00<NUL>" strings */
  memset(
    header.mode.as_mut_ptr() as *mut libc::c_void,
    '0' as i32,
    ::std::mem::size_of::<prefilled>() as libc::c_ulong,
  );
  header.mode[(::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
    .wrapping_sub(1i32 as libc::c_ulong) as usize] = '\u{0}' as i32 as libc::c_char;
  header.uid[(::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
    .wrapping_sub(1i32 as libc::c_ulong) as usize] = '\u{0}' as i32 as libc::c_char;
  header.gid[(::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
    .wrapping_sub(1i32 as libc::c_ulong) as usize] = '\u{0}' as i32 as libc::c_char;
  /* header.size is filled by '0' now, will be corrected below */
  header.mtime[(::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
    .wrapping_sub(1i32 as libc::c_ulong) as usize] = '\u{0}' as i32 as libc::c_char; /* normalize: 0/1 */
  dir = (dir != 0) as libc::c_int; /* GNU tar uses strlen+1 */
  size = strlen(name)
    .wrapping_add(1i32 as libc::c_ulong)
    .wrapping_add(dir as libc::c_ulong) as libc::c_int;
  /* + dir: account for possible '/' */
  putOctal(
    header.size.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as libc::c_int,
    size as off_t,
  );
  chksum_and_xwrite(fd, &mut header);
  /* Write filename[/] and pad the block. */
  /* dir=0: writes 'name<NUL>', pads */
  /* dir=1: writes 'name', writes '/<NUL>', pads */
  dir *= 2i32;
  crate::libbb::xfuncs_printf::xwrite(fd, name as *const libc::c_void, (size - dir) as size_t);
  crate::libbb::xfuncs_printf::xwrite(
    fd,
    b"/\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
    dir as size_t,
  );
  size = -size & 512i32 - 1i32;
  memset(
    &mut header as *mut tar_header_t as *mut libc::c_void,
    0,
    size as libc::c_ulong,
  );
  crate::libbb::xfuncs_printf::xwrite(
    fd,
    &mut header as *mut tar_header_t as *const libc::c_void,
    size as size_t,
  );
}
/* Write out a tar header for the specified file/directory/whatever */
unsafe fn writeTarHeader(
  mut tbInfo: *mut TarBallInfo,
  mut header_name: *const libc::c_char,
  mut fileName: *const libc::c_char,
  mut statbuf: *mut stat,
) -> libc::c_int {
  let mut header: tar_header_t = tar_header_t {
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
  memset(
    &mut header as *mut tar_header_t as *mut libc::c_void,
    0,
    ::std::mem::size_of::<tar_header_t>() as libc::c_ulong,
  );
  strncpy(
    header.name.as_mut_ptr(),
    header_name,
    ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
  );
  /* POSIX says to mask mode with 07777. */
  putOctal(
    header.mode.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as libc::c_int,
    ((*statbuf).st_mode & 0o7777i32 as libc::c_uint) as off_t,
  ); /* Regular file size is handled later */
  putOctal(
    header.uid.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as libc::c_int,
    (*statbuf).st_uid as off_t,
  );
  putOctal(
    header.gid.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as libc::c_int,
    (*statbuf).st_gid as off_t,
  );
  memset(
    header.size.as_mut_ptr() as *mut libc::c_void,
    '0' as i32,
    (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong),
  );
  /* users report that files with negative st_mtime cause trouble, so: */
  putOctal(
    header.mtime.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as libc::c_int,
    if (*statbuf).st_mtime >= 0 {
      (*statbuf).st_mtime
    } else {
      0
    },
  );
  /* Enter the user and group names */
  crate::libbb::safe_strncpy::safe_strncpy(
    header.uname.as_mut_ptr(),
    crate::libbb::procps::get_cached_username((*statbuf).st_uid),
    ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
  );
  crate::libbb::safe_strncpy::safe_strncpy(
    header.gname.as_mut_ptr(),
    crate::libbb::procps::get_cached_groupname((*statbuf).st_gid),
    ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
  );
  if !(*tbInfo).hlInfo.is_null() {
    /* This is a hard link */
    header.typeflag = LNKTYPE as libc::c_int as libc::c_char;
    strncpy(
      header.linkname.as_mut_ptr(),
      (*(*tbInfo).hlInfo).name.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
    );
    /* Write out long linkname if needed */
    if header.linkname[(::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong) as usize]
      != 0
    {
      writeLongname(
        (*tbInfo).tarFd,
        GNULONGLINK as libc::c_int,
        (*(*tbInfo).hlInfo).name.as_mut_ptr(),
        0,
      );
    }
  } else if (*statbuf).st_mode & 0o170000i32 as libc::c_uint == 0o120000i32 as libc::c_uint {
    let mut lpath: *mut libc::c_char = crate::libbb::xreadlink::xmalloc_readlink_or_warn(fileName);
    if lpath.is_null() {
      return 0;
    }
    header.typeflag = SYMTYPE as libc::c_int as libc::c_char;
    strncpy(
      header.linkname.as_mut_ptr(),
      lpath,
      ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
    );
    /* Write out long linkname if needed */
    if header.linkname[(::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong) as usize]
      != 0
    {
      writeLongname((*tbInfo).tarFd, GNULONGLINK as libc::c_int, lpath, 0);
    }
    free(lpath as *mut libc::c_void);
  } else if (*statbuf).st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint {
    header.typeflag = DIRTYPE as libc::c_int as libc::c_char;
    /* Append '/' only if there is a space for it */
    if header.name[(::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong) as usize]
      == 0
    {
      header.name[strlen(header.name.as_mut_ptr()) as usize] = '/' as i32 as libc::c_char
    }
  } else if (*statbuf).st_mode & 0o170000i32 as libc::c_uint == 0o20000i32 as libc::c_uint {
    header.typeflag = CHRTYPE as libc::c_int as libc::c_char;
    putOctal(
      header.devmajor.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as libc::c_int,
      gnu_dev_major((*statbuf).st_rdev) as off_t,
    );
    putOctal(
      header.devminor.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as libc::c_int,
      gnu_dev_minor((*statbuf).st_rdev) as off_t,
    );
  } else if (*statbuf).st_mode & 0o170000i32 as libc::c_uint == 0o60000i32 as libc::c_uint {
    header.typeflag = BLKTYPE as libc::c_int as libc::c_char;
    putOctal(
      header.devmajor.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as libc::c_int,
      gnu_dev_major((*statbuf).st_rdev) as off_t,
    );
    putOctal(
      header.devminor.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as libc::c_int,
      gnu_dev_minor((*statbuf).st_rdev) as off_t,
    );
  } else if (*statbuf).st_mode & 0o170000i32 as libc::c_uint == 0o10000i32 as libc::c_uint {
    header.typeflag = FIFOTYPE as libc::c_int as libc::c_char
  } else if (*statbuf).st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint {
    /* header.size field is 12 bytes long */
    /* Does octal-encoded size fit? */
    let mut filesize: uoff_t = (*statbuf).st_size as uoff_t;
    if ::std::mem::size_of::<uoff_t>() as libc::c_ulong <= 4i32 as libc::c_ulong
      || filesize <= 0o777777777777i64 as uoff_t
    {
      putOctal(
        header.size.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as libc::c_int,
        filesize as off_t,
      );
    } else {
      /* Does base256-encoded size fit?
       * It always does unless off_t is wider than 64 bits.
       */
      /* 2^64-1 */
      /* GNU tar uses "base-256 encoding" for very large numbers.
       * Encoding is binary, with highest bit always set as a marker
       * and sign in next-highest bit:
       * 80 00 .. 00 - zero
       * bf ff .. ff - largest positive number
       * ff ff .. ff - minus 1
       * c0 00 .. 00 - smallest negative number
       */
      let mut p8: *mut libc::c_char = header
        .size
        .as_mut_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as isize);
      loop {
        p8 = p8.offset(-1);
        *p8 = filesize as u8 as libc::c_char;
        filesize >>= 8i32;
        if !(p8 != header.size.as_mut_ptr()) {
          break;
        }
      }
      *p8 = (*p8 as libc::c_int | 0x80i32) as libc::c_char
    }
    header.typeflag = REGTYPE as libc::c_int as libc::c_char
  } else {
    crate::libbb::verror_msg::bb_error_msg(
      b"%s: unknown file type\x00" as *const u8 as *const libc::c_char,
      fileName,
    );
    return 0;
  }
  /* Write out long name if needed */
  /* (we, like GNU tar, output long linkname *before* long name) */
  if header.name[(::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong)
    .wrapping_sub(1i32 as libc::c_ulong) as usize]
    != 0
  {
    writeLongname(
      (*tbInfo).tarFd,
      GNULONGNAME as libc::c_int,
      header_name,
      ((*statbuf).st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint)
        as libc::c_int,
    );
  }
  /* Now write the header out to disk */
  chksum_and_xwrite((*tbInfo).tarFd, &mut header);
  /* Now do the verbose thing (or not) */
  if (*tbInfo).verboseFlag != 0 {
    let mut vbFd: *mut FILE = stdout;
    /* If archive goes to stdout, verbose goes to stderr */
    if (*tbInfo).tarFd == 1i32 {
      vbFd = stderr
    }
    /* GNU "tar cvvf" prints "extended" listing a-la "ls -l" */
    /* We don't have such excesses here: for us "v" == "vv" */
    /* '/' is probably a GNUism */
    fprintf(
      vbFd,
      b"%s%s\n\x00" as *const u8 as *const libc::c_char,
      header_name,
      if (*statbuf).st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint {
        b"/\x00" as *const u8 as *const libc::c_char
      } else {
        b"\x00" as *const u8 as *const libc::c_char
      },
    );
  }
  return 1i32;
}
unsafe fn exclude_file(
  mut excluded_files: *const llist_t,
  mut file: *const libc::c_char,
) -> libc::c_int {
  while !excluded_files.is_null() {
    if *(*excluded_files).data.offset(0) as libc::c_int == '/' as i32 {
      if fnmatch((*excluded_files).data, file, 1i32 << 0 | 1i32 << 3i32) == 0 {
        return 1i32;
      }
    } else {
      let mut p: *const libc::c_char = std::ptr::null();
      p = file;
      while *p.offset(0) as libc::c_int != '\u{0}' as i32 {
        if (p == file || *p.offset(-1i32 as isize) as libc::c_int == '/' as i32)
          && *p.offset(0) as libc::c_int != '/' as i32
          && fnmatch((*excluded_files).data, p, 1i32 << 0 | 1i32 << 3i32) == 0
        {
          return 1i32;
        }
        p = p.offset(1)
      }
    }
    excluded_files = (*excluded_files).link
  }
  return 0;
}
unsafe fn writeFileToTarball(
  mut fileName: *const libc::c_char,
  mut statbuf: *mut stat,
  mut userData: *mut libc::c_void,
  mut _depth: libc::c_int,
) -> libc::c_int {
  let mut tbInfo: *mut TarBallInfo = userData as *mut TarBallInfo;
  let mut header_name: *const libc::c_char = std::ptr::null();
  let mut inputFileFd: libc::c_int = -1i32;
  /* Strip leading '/' and such (must be before memorizing hardlink's name) */
  header_name = crate::archival::libarchive::unsafe_prefix::strip_unsafe_prefix(fileName);
  if *header_name.offset(0) as libc::c_int == '\u{0}' as i32 {
    return 1i32;
  }
  /* It is against the rules to archive a socket */
  if (*statbuf).st_mode & 0o170000i32 as libc::c_uint == 0o140000i32 as libc::c_uint {
    crate::libbb::verror_msg::bb_error_msg(
      b"%s: socket ignored\x00" as *const u8 as *const libc::c_char,
      fileName,
    );
    return 1i32;
  }
  /*
   * Check to see if we are dealing with a hard link.
   * If so -
   * Treat the first occurrence of a given dev/inode as a file while
   * treating any additional occurrences as hard links.  This is done
   * by adding the file information to the HardLinkInfo linked list.
   */
  (*tbInfo).hlInfo = std::ptr::null_mut();
  if !((*statbuf).st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint)
    && (*statbuf).st_nlink > 1i32 as libc::c_ulong
  {
    (*tbInfo).hlInfo = findHardLinkInfo((*tbInfo).hlInfoHead, statbuf);
    if (*tbInfo).hlInfo.is_null() {
      addHardLinkInfo(&mut (*tbInfo).hlInfoHead, statbuf, header_name);
    }
  }
  /* It is a bad idea to store the archive we are in the process of creating,
   * so check the device and inode to be sure that this particular file isn't
   * the new tarball */
  if (*tbInfo).tarFileStatBuf.st_dev == (*statbuf).st_dev
    && (*tbInfo).tarFileStatBuf.st_ino == (*statbuf).st_ino
  {
    crate::libbb::verror_msg::bb_error_msg(
      b"%s: file is the archive; skipping\x00" as *const u8 as *const libc::c_char,
      fileName,
    );
    return 1i32;
  }
  if exclude_file((*tbInfo).excludeList, header_name) != 0 {
    return 2i32;
  }
  /* Is this a regular file? */
  if (*tbInfo).hlInfo.is_null()
    && (*statbuf).st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint
  {
    /* open the file we want to archive, and make sure all is well */
    inputFileFd = crate::libbb::xfuncs_printf::open_or_warn(fileName, 0);
    if inputFileFd < 0 {
      return 0;
    }
  }
  /* Add an entry to the tarball */
  if writeTarHeader(tbInfo, header_name, fileName, statbuf) == 0 {
    return 0;
  }
  /* If it was a regular file, write out the body */
  if inputFileFd >= 0 {
    let mut readSize: size_t = 0;
    /* Write the file to the archive. */
    /* We record size into header first, */
    /* and then write out file. If file shrinks in between, */
    /* tar will be corrupted. So we don't allow for that. */
    /* NB: GNU tar 1.16 warns and pads with zeroes */
    /* or even seeks back and updates header */
    crate::libbb::copyfd::bb_copyfd_exact_size(inputFileFd, (*tbInfo).tarFd, (*statbuf).st_size);
    // //off_t readSize;
    // //readSize = bb_copyfd_size(inputFileFd, tbInfo->tarFd, statbuf->st_size);
    // //if (readSize != statbuf->st_size && readSize >= 0) {
    // //	bb_error_msg_and_die("short read from %s, aborting", fileName);
    // //}
    /* Check that file did not grow in between? */
    /* if (safe_read(inputFileFd, 1) == 1) warn but continue? */
    close(inputFileFd);
    /* Pad the file up to the tar block size */
    /* (a few tricks here in the name of code size) */
    readSize = (-((*statbuf).st_size as libc::c_int) & 512i32 - 1i32) as size_t;
    memset(
      bb_common_bufsiz1.as_mut_ptr() as *mut libc::c_void,
      0,
      readSize,
    );
    crate::libbb::xfuncs_printf::xwrite(
      (*tbInfo).tarFd,
      bb_common_bufsiz1.as_mut_ptr() as *const libc::c_void,
      readSize,
    );
  }
  return 1i32;
}
/* Don't inline: vfork scares gcc and pessimizes code */
#[inline(never)]
unsafe fn vfork_compressor(mut tar_fd: libc::c_int, mut gzip: *const libc::c_char) {
  // On Linux, vfork never unpauses parent early, although standard
  // allows for that. Do we want to waste bytes checking for it?
  let mut vfork_exec_errno: libc::c_int = 0; /* we only want EPIPE on errors */
  let mut data: fd_pair = fd_pair { rd: 0, wr: 0 };
  crate::libbb::xfuncs_printf::xpipe(&mut data.rd);
  signal(
    13i32,
    ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
  );
  if ({
    let mut bb__xvfork_pid: pid_t = vfork();
    if bb__xvfork_pid < 0 {
      crate::libbb::perror_msg::bb_simple_perror_msg_and_die(
        b"vfork\x00" as *const u8 as *const libc::c_char,
      );
    }
    bb__xvfork_pid
  }) == 0
  {
    /* child */
    let mut tfd: libc::c_int = 0;
    /* NB: close _first_, then move fds! */
    close(data.wr);
    /* copy it: parent's tar_fd variable must not change */
    tfd = tar_fd;
    if tfd == 0 {
      /* Output tar fd may be zero.
       * xmove_fd(data.rd, 0) would destroy it.
       * Reproducer:
       *  exec 0>&-
       *  exec 1>&-
       *  tar czf Z.tar.gz FILE
       * Swapping move_fd's order wouldn't work:
       * data.rd is 1 and _it_ would be destroyed.
       */
      tfd = dup(tfd)
    }
    crate::libbb::xfuncs_printf::xmove_fd(data.rd, 0);
    crate::libbb::xfuncs_printf::xmove_fd(tfd, 1i32);
    /* exec gzip/bzip2/... program */
    //BB_EXECLP(gzip, gzip, "-f", (char *)0); - WRONG for "xz",
    // if xz is an enabled applet, it'll be a version which
    // can only decompress. We do need to execute external
    // program, not applet.
    execlp(
      gzip,
      gzip,
      b"-f\x00" as *const u8 as *const libc::c_char,
      std::ptr::null_mut::<libc::c_char>(),
    );
    ::std::ptr::write_volatile(&mut vfork_exec_errno as *mut libc::c_int, *bb_errno);
    _exit(1i32);
  }
  /* parent */
  crate::libbb::xfuncs_printf::xmove_fd(data.wr, tar_fd);
  close(data.rd);
  if vfork_exec_errno != 0 {
    *bb_errno = vfork_exec_errno;
    crate::libbb::perror_msg::bb_perror_msg_and_die(
      b"can\'t execute \'%s\'\x00" as *const u8 as *const libc::c_char,
      gzip,
    );
  };
}
/* SEAMLESS_COMPRESSION */
/* gcc 4.2.1 inlines it, making code bigger */
#[inline(never)]
unsafe fn writeTarFile(
  mut tbInfo: *mut TarBallInfo,
  mut recurseFlags: libc::c_int,
  mut filelist: *const llist_t,
  mut gzip: *const libc::c_char,
) -> libc::c_int {
  let mut errorFlag: libc::c_int = 0;
  /*tbInfo->hlInfoHead = NULL; - already is */
  /* Store the stat info for the tarball's file, so
   * can avoid including the tarball into itself....  */
  crate::libbb::xfuncs_printf::xfstat(
    (*tbInfo).tarFd,
    &mut (*tbInfo).tarFileStatBuf,
    b"can\'t stat tar file\x00" as *const u8 as *const libc::c_char,
  );
  if !gzip.is_null() {
    vfork_compressor((*tbInfo).tarFd, gzip);
  }
  /* Read the directory/files and iterate over them one at a time */
  while !filelist.is_null() {
    if crate::libbb::recursive_action::recursive_action(
      (*filelist).data,
      recurseFlags as libc::c_uint,
      Some(
        writeFileToTarball
          as unsafe fn(
            _: *const libc::c_char,
            _: *mut stat,
            _: *mut libc::c_void,
            _: libc::c_int,
          ) -> libc::c_int,
      ),
      Some(
        writeFileToTarball
          as unsafe fn(
            _: *const libc::c_char,
            _: *mut stat,
            _: *mut libc::c_void,
            _: libc::c_int,
          ) -> libc::c_int,
      ),
      tbInfo as *mut libc::c_void,
      0 as libc::c_uint,
    ) == 0
    {
      errorFlag = 1i32
    }
    filelist = (*filelist).link
  }
  /* Write two empty blocks to the end of the archive */
  memset(
    bb_common_bufsiz1.as_mut_ptr() as *mut libc::c_void,
    0,
    (2i32 * 512i32) as libc::c_ulong,
  );
  crate::libbb::xfuncs_printf::xwrite(
    (*tbInfo).tarFd,
    bb_common_bufsiz1.as_mut_ptr() as *const libc::c_void,
    (2i32 * 512i32) as size_t,
  );
  /* To be pedantically correct, we would check if the tarball
   * is smaller than 20 tar blocks, and pad it if it was smaller,
   * but that isn't necessary for GNU tar interoperability, and
   * so is considered a waste of space */
  /* Close so the child process (if any) will exit */
  close((*tbInfo).tarFd);
  /* Hang up the tools, close up shop, head home */
  if errorFlag != 0 {
    crate::libbb::verror_msg::bb_simple_error_msg(
      b"error exit delayed from previous errors\x00" as *const u8 as *const libc::c_char,
    );
  }
  if !gzip.is_null() {
    let mut status: libc::c_int = 0;
    if crate::libbb::xfuncs::safe_waitpid(-1i32, &mut status, 0) == -1i32 {
      crate::libbb::perror_msg::bb_simple_perror_msg(
        b"waitpid\x00" as *const u8 as *const libc::c_char,
      );
    } else if !(status & 0x7fi32 == 0) || (status & 0xff00i32) >> 8i32 != 0 {
      /* gzip was killed or has exited with nonzero! */
      errorFlag = 1i32
    }
  }
  return errorFlag;
}
/* FEATURE_TAR_CREATE */
unsafe fn append_file_list_to_list(mut list: *mut llist_t) -> *mut llist_t {
  let mut newlist: *mut llist_t = std::ptr::null_mut();
  while !list.is_null() {
    let mut src_stream: *mut FILE = std::ptr::null_mut();
    let mut line: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    src_stream = crate::libbb::wfopen_input::xfopen_stdin(
      crate::libbb::llist::llist_pop(&mut list) as *const libc::c_char,
    );
    loop {
      line = crate::libbb::get_line_from_file::xmalloc_fgetline(src_stream);
      if line.is_null() {
        break;
      }
      /* kill trailing '/' unless the string is just "/" */
      let mut cp: *mut libc::c_char = crate::libbb::last_char_is::last_char_is(line, '/' as i32);
      if cp > line {
        *cp = '\u{0}' as i32 as libc::c_char
      }
      crate::libbb::llist::llist_add_to_end(&mut newlist, line as *mut libc::c_void);
    }
    fclose(src_stream);
  }
  return newlist;
}
static mut tar_longopts: [i8; 314] = [
  108, 105, 115, 116, 0, 0, 116, 101, 120, 116, 114, 97, 99, 116, 0, 0, 120, 100, 105, 114, 101,
  99, 116, 111, 114, 121, 0, 1, 67, 102, 105, 108, 101, 0, 1, 102, 116, 111, 45, 115, 116, 100,
  111, 117, 116, 0, 0, 79, 110, 111, 45, 115, 97, 109, 101, 45, 111, 119, 110, 101, 114, 0, 0, 111,
  115, 97, 109, 101, 45, 112, 101, 114, 109, 105, 115, 115, 105, 111, 110, 115, 0, 0, 112, 118,
  101, 114, 98, 111, 115, 101, 0, 0, 118, 107, 101, 101, 112, 45, 111, 108, 100, 0, 0, 107, 99,
  114, 101, 97, 116, 101, 0, 0, 99, 100, 101, 114, 101, 102, 101, 114, 101, 110, 99, 101, 0, 0,
  104, 98, 122, 105, 112, 50, 0, 0, 106, 102, 105, 108, 101, 115, 45, 102, 114, 111, 109, 0, 1, 84,
  101, 120, 99, 108, 117, 100, 101, 45, 102, 114, 111, 109, 0, 1, 88, 103, 122, 105, 112, 0, 0,
  122, 120, 122, 0, 0, 74, 97, 117, 116, 111, 45, 99, 111, 109, 112, 114, 101, 115, 115, 0, 0, 97,
  116, 111, 117, 99, 104, 0, 0, 109, 115, 116, 114, 105, 112, 45, 99, 111, 109, 112, 111, 110, 101,
  110, 116, 115, 0, 1, -8, 108, 122, 109, 97, 0, 0, -7, 110, 111, 45, 114, 101, 99, 117, 114, 115,
  105, 111, 110, 0, 0, -6, 116, 111, 45, 99, 111, 109, 109, 97, 110, 100, 0, 1, -5, 110, 117, 109,
  101, 114, 105, 99, 45, 111, 119, 110, 101, 114, 0, 0, -4, 110, 111, 45, 115, 97, 109, 101, 45,
  112, 101, 114, 109, 105, 115, 115, 105, 111, 110, 115, 0, 0, -3, 111, 118, 101, 114, 119, 114,
  105, 116, 101, 0, 0, -2, 101, 120, 99, 108, 117, 100, 101, 0, 1, -1, 0,
];
pub unsafe fn tar_main(mut _argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut tar_handle: *mut archive_handle_t = std::ptr::null_mut();
  let mut base_dir: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut tar_filename: *const libc::c_char = b"-\x00" as *const u8 as *const libc::c_char;
  let mut opt: libc::c_uint = 0;
  let mut verboseFlag: libc::c_int = 0;
  let mut excludes: *mut llist_t = std::ptr::null_mut();
  /* Initialise default values */
  tar_handle = crate::archival::libarchive::init_handle::init_handle();
  (*tar_handle).ah_flags = (1i32 << 1i32 | 1i32 << 0 | 1i32 << 2i32) as libc::c_uint;
  /* Apparently only root's tar preserves perms (see bug 3844) */
  if getuid() != 0 as libc::c_uint {
    (*tar_handle).ah_flags |= (1i32 << 5i32) as libc::c_uint
  }
  /* Lie to buildroot when it starts asking stupid questions. */
  if !(*argv.offset(1)).is_null()
    && strcmp(
      *argv.offset(1),
      b"--version\x00" as *const u8 as *const libc::c_char,
    ) == 0
  {
    // Output of 'tar --version' examples:
    // tar (GNU tar) 1.15.1
    // tar (GNU tar) 1.25
    // bsdtar 2.8.3 - libarchive 2.8.3
    puts(b"tar (busybox) 1.32.0.git\x00" as *const u8 as *const libc::c_char);
    return 0;
  }
  if !(*argv.offset(1)).is_null()
    && *(*argv.offset(1)).offset(0) as libc::c_int != '-' as i32
    && *(*argv.offset(1)).offset(0) as libc::c_int != '\u{0}' as i32
  {
    /* Compat:
     * 1st argument without dash handles options with parameters
     * differently from dashed one: it takes *next argv[i]*
     * as parameter even if there are more chars in 1st argument:
     *  "tar fx TARFILE" - "x" is not taken as f's param
     *  but is interpreted as -x option
     *  "tar -xf TARFILE" - dashed equivalent of the above
     *  "tar -fx ..." - "x" is taken as f's param
     * getopt32 wouldn't handle 1st command correctly.
     * Unfortunately, people do use such commands.
     * We massage argv[1] to work around it by moving 'f'
     * to the end of the string.
     * More contrived "tar fCx TARFILE DIR" still fails,
     * but such commands are much less likely to be used.
     */
    let mut f: *mut libc::c_char = strchr(*argv.offset(1), 'f' as i32);
    if !f.is_null() {
      while *f.offset(1) as libc::c_int != '\u{0}' as i32 {
        *f = *f.offset(1);
        f = f.offset(1)
      }
      *f = 'f' as i32 as libc::c_char
    }
    /* Prepend '-' to the first argument  */
    let ref mut fresh1 = *argv.offset(1);
    *fresh1 = crate::libbb::xfuncs_printf::xasprintf(
      b"-%s\x00" as *const u8 as *const libc::c_char,
      *argv.offset(1),
    )
  }
  opt = crate::libbb::getopt32::getopt32long(
    argv,
    b"^txC:f:OopvkchjT:*X:*zJam\xf8:\x00tt:vv:\xff::c:t:x:c--tx:t--cx:x--ct:\xf8+\x00" as *const u8
      as *const libc::c_char,
    tar_longopts.as_ptr(),
    &mut base_dir as *mut *mut libc::c_char,
    &mut tar_filename as *mut *const libc::c_char,
    &mut (*tar_handle).accept as *mut *mut llist_t,
    &mut (*tar_handle).reject as *mut *mut llist_t,
    &mut (*tar_handle).tar__strip_components as *mut libc::c_uint,
    &mut (*tar_handle).tar__to_command as *mut *mut libc::c_char,
    &mut excludes as *mut *mut llist_t,
    &mut verboseFlag as *mut libc::c_int,
    &mut verboseFlag as *mut libc::c_int,
  );
  argv = argv.offset(optind as isize);
  if verboseFlag != 0 {
    (*tar_handle).action_header = Some(
      crate::archival::libarchive::header_verbose_list::header_verbose_list
        as unsafe fn(_: *const file_header_t) -> (),
    )
  }
  if verboseFlag == 1i32 {
    (*tar_handle).action_header = Some(
      crate::archival::libarchive::header_list::header_list
        as unsafe fn(_: *const file_header_t) -> (),
    )
  }
  if opt & OPT_EXTRACT as libc::c_int as libc::c_uint != 0 {
    (*tar_handle).action_data = Some(
      crate::archival::libarchive::data_extract_all::data_extract_all
        as unsafe fn(_: *mut archive_handle_t) -> (),
    )
  }
  if opt & OPT_2STDOUT as libc::c_int as libc::c_uint != 0 {
    (*tar_handle).action_data = Some(
      crate::archival::libarchive::data_extract_to_stdout::data_extract_to_stdout
        as unsafe fn(_: *mut archive_handle_t) -> (),
    )
  }
  if opt & OPT_2COMMAND as libc::c_int as libc::c_uint != 0 {
    putenv(b"TAR_FILETYPE=f\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
    signal(
      13i32,
      ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
    );
    (*tar_handle).action_data = Some(
      crate::archival::libarchive::data_extract_to_command::data_extract_to_command
        as unsafe fn(_: *mut archive_handle_t) -> (),
    );
    (*tar_handle).tar__to_command_shell =
      crate::libbb::xfuncs_printf::xstrdup(crate::libbb::get_shell_name::get_shell_name())
  }
  if opt & OPT_KEEP_OLD as libc::c_int as libc::c_uint != 0 {
    (*tar_handle).ah_flags &= !(1i32 << 2i32) as libc::c_uint
  }
  if opt & OPT_NUMERIC_OWNER as libc::c_int as libc::c_uint != 0 {
    (*tar_handle).ah_flags |= (1i32 << 6i32) as libc::c_uint
  }
  if opt & OPT_NOPRESERVE_OWNER as libc::c_int as libc::c_uint != 0 {
    (*tar_handle).ah_flags |= (1i32 << 4i32) as libc::c_uint
  }
  if opt & OPT_NOPRESERVE_PERM as libc::c_int as libc::c_uint != 0 {
    (*tar_handle).ah_flags |= (1i32 << 5i32) as libc::c_uint
  }
  if opt & OPT_OVERWRITE as libc::c_int as libc::c_uint != 0 {
    (*tar_handle).ah_flags &= !(1i32 << 2i32) as libc::c_uint;
    (*tar_handle).ah_flags |= (1i32 << 7i32) as libc::c_uint
  }
  if opt & OPT_NOPRESERVE_TIME as libc::c_int as libc::c_uint != 0 {
    (*tar_handle).ah_flags &= !(1i32 << 0) as libc::c_uint
  }
  (*tar_handle).reject = append_file_list_to_list((*tar_handle).reject);
  /* Append excludes to reject */
  while !excludes.is_null() {
    let mut next: *mut llist_t = (*excludes).link;
    (*excludes).link = (*tar_handle).reject;
    (*tar_handle).reject = excludes;
    excludes = next
  }
  (*tar_handle).accept = append_file_list_to_list((*tar_handle).accept);
  /* Setup an array of filenames to work with */
  /* TODO: This is the same as in ar, make a separate function? */
  while !(*argv).is_null() {
    /* kill trailing '/' unless the string is just "/" */
    let mut cp: *mut libc::c_char = crate::libbb::last_char_is::last_char_is(*argv, '/' as i32);
    if cp > *argv {
      *cp = '\u{0}' as i32 as libc::c_char
    }
    crate::libbb::llist::llist_add_to_end(&mut (*tar_handle).accept, *argv as *mut libc::c_void);
    argv = argv.offset(1)
  }
  if !(*tar_handle).accept.is_null() || !(*tar_handle).reject.is_null() {
    (*tar_handle).filter = Some(
      crate::archival::libarchive::filter_accept_reject_list::filter_accept_reject_list
        as unsafe fn(_: *mut archive_handle_t) -> libc::c_char,
    )
  }
  /* Open the tar file */
  let mut tar_fd: libc::c_int = 0;
  let mut flags: libc::c_int = 0;
  if opt & OPT_CREATE as libc::c_int as libc::c_uint != 0 {
    /* Make sure there is at least one file to tar up */
    if (*tar_handle).accept.is_null() {
      crate::libbb::verror_msg::bb_simple_error_msg_and_die(
        b"empty archive\x00" as *const u8 as *const libc::c_char,
      );
    }
    tar_fd = 1i32;
    /* Mimicking GNU tar 1.15.1: */
    flags = 0o1i32 | 0o100i32 | 0o1000i32
  }
  if *tar_filename.offset(0) as libc::c_int == '-' as i32 && *tar_filename.offset(1) == 0 {
    (*tar_handle).src_fd = tar_fd;
    (*tar_handle).seek = Some(
      crate::archival::libarchive::seek_by_read::seek_by_read
        as unsafe fn(_: libc::c_int, _: off_t) -> (),
    )
  } else if 1i32 != 0
    && 1i32 != 0
    && flags == 0
    && opt & OPT_ANY_COMPRESS as libc::c_int as libc::c_uint == 0
    && !crate::libbb::compare_string_array::is_suffixed_with(
      tar_filename,
      b".lzma\x00" as *const u8 as *const libc::c_char,
    )
    .is_null()
  {
    /* We do this only for .lzma files, they have no signature.
     * All other compression formats are recognized in
     * get_header_tar() when first tar block has invalid format.
     * Doing it here for all filenames would falsely trigger
     * on e.g. tarball with 1st file named "BZh5".
     */
    (*tar_handle).src_fd =
      crate::archival::libarchive::open_transformer::open_zipped(tar_filename, 0);
    if (*tar_handle).src_fd < 0 {
      crate::libbb::perror_msg::bb_perror_msg_and_die(
        b"can\'t open \'%s\'\x00" as *const u8 as *const libc::c_char,
        tar_filename,
      );
    }
  } else {
    (*tar_handle).src_fd = crate::libbb::xfuncs_printf::xopen(tar_filename, flags);
    if OPT_GZIP as libc::c_int
      | OPT_BZIP2 as libc::c_int
      | OPT_XZ as libc::c_int
      | OPT_LZMA as libc::c_int
      != 0
      && opt & OPT_AUTOCOMPRESS_BY_EXT as libc::c_int as libc::c_uint != 0
      && flags != 0
    {
      if OPT_GZIP as libc::c_int != 0
        && !crate::libbb::compare_string_array::is_suffixed_with(
          tar_filename,
          b"gz\x00" as *const u8 as *const libc::c_char,
        )
        .is_null()
      {
        opt |= OPT_GZIP as libc::c_int as libc::c_uint
      }
      if OPT_BZIP2 as libc::c_int != 0
        && !crate::libbb::compare_string_array::is_suffixed_with(
          tar_filename,
          b"bz2\x00" as *const u8 as *const libc::c_char,
        )
        .is_null()
      {
        opt |= OPT_BZIP2 as libc::c_int as libc::c_uint
      }
      if OPT_XZ as libc::c_int != 0
        && !crate::libbb::compare_string_array::is_suffixed_with(
          tar_filename,
          b"xz\x00" as *const u8 as *const libc::c_char,
        )
        .is_null()
      {
        opt |= OPT_XZ as libc::c_int as libc::c_uint
      }
      if OPT_LZMA as libc::c_int != 0
        && !crate::libbb::compare_string_array::is_suffixed_with(
          tar_filename,
          b"lzma\x00" as *const u8 as *const libc::c_char,
        )
        .is_null()
      {
        opt |= OPT_LZMA as libc::c_int as libc::c_uint
      }
    }
  }
  if !base_dir.is_null() {
    crate::libbb::xfuncs_printf::xchdir(base_dir);
  }
  /* Create an archive */
  if opt & OPT_CREATE as libc::c_int as libc::c_uint != 0 {
    let mut tbInfo: *mut TarBallInfo = std::ptr::null_mut();
    let mut zipMode: *const libc::c_char = std::ptr::null();
    if opt & OPT_COMPRESS as libc::c_int as libc::c_uint != 0 {
      zipMode = b"compress\x00" as *const u8 as *const libc::c_char
    }
    if opt & OPT_GZIP as libc::c_int as libc::c_uint != 0 {
      zipMode = b"gzip\x00" as *const u8 as *const libc::c_char
    }
    if opt & OPT_BZIP2 as libc::c_int as libc::c_uint != 0 {
      zipMode = b"bzip2\x00" as *const u8 as *const libc::c_char
    }
    if opt & OPT_LZMA as libc::c_int as libc::c_uint != 0 {
      zipMode = b"lzma\x00" as *const u8 as *const libc::c_char
    }
    if opt & OPT_XZ as libc::c_int as libc::c_uint != 0 {
      zipMode = b"xz\x00" as *const u8 as *const libc::c_char
    }
    tbInfo =
      crate::libbb::xfuncs_printf::xzalloc(::std::mem::size_of::<TarBallInfo>() as libc::c_ulong)
        as *mut TarBallInfo;
    (*tbInfo).tarFd = (*tar_handle).src_fd;
    (*tbInfo).verboseFlag = verboseFlag;
    (*tbInfo).excludeList = (*tar_handle).reject;
    /* NB: writeTarFile() closes tar_handle->src_fd */
    return writeTarFile(
      tbInfo,
      (if opt & OPT_DEREFERENCE as libc::c_int as libc::c_uint != 0 {
        ACTION_FOLLOWLINKS as libc::c_int
      } else {
        0
      }) | (if opt & OPT_NORECURSION as libc::c_int as libc::c_uint != 0 {
        0
      } else {
        ACTION_RECURSE as libc::c_int
      }),
      (*tar_handle).accept,
      zipMode,
    );
  }
  if opt & OPT_ANY_COMPRESS as libc::c_int as libc::c_uint != 0 {
    let mut xformer: Option<unsafe fn(_: *mut transformer_state_t) -> libc::c_longlong> = None;

    // TODO: why was this translated this way?
    // (opt & OPT_COMPRESS as libc::c_int as libc::c_uint) != 0;

    if opt & OPT_GZIP as libc::c_int as libc::c_uint != 0 {
      xformer = Some(
        crate::archival::libarchive::decompress_gunzip::unpack_gz_stream
          as unsafe fn(_: *mut transformer_state_t) -> libc::c_longlong,
      )
    }
    if opt & OPT_BZIP2 as libc::c_int as libc::c_uint != 0 {
      xformer = Some(crate::archival::libarchive::decompress_bunzip2::unpack_bz2_stream)
    }
    if opt & OPT_LZMA as libc::c_int as libc::c_uint != 0 {
      xformer = Some(
        crate::archival::libarchive::decompress_unlzma::unpack_lzma_stream
          as unsafe fn(_: *mut transformer_state_t) -> libc::c_longlong,
      )
    }
    if opt & OPT_XZ as libc::c_int as libc::c_uint != 0 {
      xformer = Some(
        crate::archival::libarchive::decompress_unxz::unpack_xz_stream
          as unsafe fn(_: *mut transformer_state_t) -> libc::c_longlong,
      )
    }
    crate::archival::libarchive::open_transformer::fork_transformer(
      (*tar_handle).src_fd,
      0,
      xformer,
    );
    /*tar_handle->offset = 0; - already is */
    (*tar_handle).seek = Some(
      crate::archival::libarchive::seek_by_read::seek_by_read
        as unsafe fn(_: libc::c_int, _: off_t) -> (),
    )
  }
  /* Can't lseek over pipes */
  /* Zero processed headers (== empty file) is not a valid tarball.
   * We (ab)use bb_got_signal as exitcode here,
   * because check_errors_in_children() uses _it_ as error indicator.
   */
  bb_got_signal = 1i32 as smallint; /* saw at least one header, good */
  while crate::archival::libarchive::get_header_tar::get_header_tar(tar_handle) as libc::c_int == 0
  {
    bb_got_signal = 0 as smallint
  }
  crate::archival::libarchive::unsafe_symlink_target::create_links_from_list(
    (*tar_handle).link_placeholders,
  );
  /* Check that every file that should have been extracted was */
  while !(*tar_handle).accept.is_null() {
    if crate::archival::libarchive::find_list_entry::find_list_entry(
      (*tar_handle).reject,
      (*(*tar_handle).accept).data,
    )
    .is_null()
      && crate::archival::libarchive::find_list_entry::find_list_entry(
        (*tar_handle).passed,
        (*(*tar_handle).accept).data,
      )
      .is_null()
    {
      crate::libbb::verror_msg::bb_error_msg_and_die(
        b"%s: not found in archive\x00" as *const u8 as *const libc::c_char,
        (*(*tar_handle).accept).data,
      );
    }
    (*tar_handle).accept = (*(*tar_handle).accept).link
  }
  if 0 != 0
    || 1i32 != 0
    || 1i32 != 0
    || 1i32 != 0
    || 1i32 != 0
    || 0 != 0
    || OPT_COMPRESS as libc::c_int != 0
  {
    /* Set bb_got_signal to 1 if a child died with !0 exitcode */
    crate::archival::libarchive::open_transformer::check_errors_in_children(0i32);
  }
  return bb_got_signal as libc::c_int;
}

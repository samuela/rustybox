use crate::librb::size_t;
use crate::librb::smallint;
use crate::librb::uoff_t;
use libc;
















use libc::sleep;
use libc::sscanf;








use libc::fclose;


use libc::printf;
use libc::puts;


use libc::sprintf;

use libc::strcmp;



use libc::open;

use libc::close;

use libc::off64_t;
use libc::off_t;
use libc::sigset_t;
use libc::ssize_t;
use libc::stat;
use libc::FILE;

extern "C" {
  #[no_mangle]
  fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;

  #[no_mangle]
  fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;



  #[no_mangle]
  fn lseek(__fd: libc::c_int, __offset: off64_t, __whence: libc::c_int) -> off64_t;





  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn sync();



  #[no_mangle]
  fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;

  #[no_mangle]
  fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;







  #[no_mangle]
  fn snprintf(
    _: *mut libc::c_char,
    _: libc::c_ulong,
    _: *const libc::c_char,
    _: ...
  ) -> libc::c_int;



  #[no_mangle]
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut FILE,
  ) -> *mut libc::c_char;



  #[no_mangle]
  fn strtoul(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_ulong;

  #[no_mangle]
  fn strtoull(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_ulonglong;

  #[no_mangle]
  static ptr_to_globals: *mut globals;

  #[no_mangle]
  fn read_line_input(
    st: *mut line_input_t,
    prompt: *const libc::c_char,
    command: *mut libc::c_char,
    maxsize: libc::c_int,
  ) -> libc::c_int;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  static bb_errno: *mut libc::c_int;

  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);

  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn auto_string(str: *mut libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn full_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;

  #[no_mangle]
  fn xwrite(fd: libc::c_int, buf: *const libc::c_void, count: size_t);

  #[no_mangle]
  fn fopen_or_warn(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;

  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;

  #[no_mangle]
  fn smart_ulltoa5(
    ul: libc::c_ulonglong,
    buf: *mut libc::c_char,
    scale: *const libc::c_char,
  ) -> *mut libc::c_char;

  /* Non-aborting kind of convertors: bb_strto[u][l]l */
  /* On exit: errno = 0 only if there was non-empty, '\0' terminated value
   * errno = EINVAL if value was not '\0' terminated, but otherwise ok
   *    Return value is still valid, caller should just check whether end[0]
   *    is a valid terminating char for particular case. OTOH, if caller
   *    requires '\0' terminated input, [s]he can just check errno == 0.
   * errno = ERANGE if value had alphanumeric terminating char ("1234abcg").
   * errno = ERANGE if value is out of range, missing, etc.
   * errno = ERANGE if value had minus sign for strtouXX (even "-0" is not ok )
   *    return value is all-ones in this case.
   */
  #[no_mangle]
  fn bb_strtoull(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_ulonglong;

  #[no_mangle]
  static mut option_mask32: u32;

  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;

  #[no_mangle]
  fn bb_show_usage() -> !;

  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);

  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;

  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);

  #[no_mangle]
  fn ioctl_or_perror(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    fmt: *const libc::c_char,
    _: ...
  ) -> libc::c_int;

  #[no_mangle]
  fn exit(_: libc::c_int) -> !;

  #[no_mangle]
  fn bsd_select();

  #[no_mangle]
  fn xbsd_print_disklabel(_: libc::c_int);

  #[no_mangle]
  fn gpt_list_table(xtra: libc::c_int);

  #[no_mangle]
  static sgi_sys_types: [*const libc::c_char; 0];

  #[no_mangle]
  fn sgi_delete_partition(i: libc::c_int);

  #[no_mangle]
  fn sgi_change_sysid(i: libc::c_int, sys: libc::c_int);

  #[no_mangle]
  fn sgi_list_table(xtra: libc::c_int);

  #[no_mangle]
  fn sgi_set_xcyl();

  #[no_mangle]
  fn verify_sgi(verbose: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn sgi_add_partition(n: libc::c_int, sys: libc::c_int);

  #[no_mangle]
  fn sgi_set_swappartition(i: libc::c_int);

  #[no_mangle]
  fn sgi_get_bootfile() -> *const libc::c_char;

  #[no_mangle]
  fn sgi_set_bootfile(aFile: *const libc::c_char);

  #[no_mangle]
  fn create_sgiinfo();

  #[no_mangle]
  fn sgi_write_table();

  #[no_mangle]
  fn sgi_set_bootpartition(i: libc::c_int);

  #[no_mangle]
  fn sun_delete_partition(i: libc::c_int);

  #[no_mangle]
  fn sun_change_sysid(i: libc::c_int, sys: libc::c_int);

  #[no_mangle]
  fn sun_list_table(xtra: libc::c_int);

  #[no_mangle]
  fn add_sun_partition(n: libc::c_int, sys: libc::c_int);

  #[no_mangle]
  fn sun_set_alt_cyl();

  #[no_mangle]
  fn sun_set_ncyl(cyl: libc::c_int);

  #[no_mangle]
  fn sun_set_xcyl();

  #[no_mangle]
  fn sun_set_ilfact();

  #[no_mangle]
  fn sun_set_rspeed();

  #[no_mangle]
  fn sun_set_pcylcount();

  #[no_mangle]
  fn toggle_sunflags(i: libc::c_int, mask: libc::c_uchar);

  #[no_mangle]
  fn verify_sun();

  #[no_mangle]
  fn sun_write_table();
}

pub type bb__aliased_u32 = u32;

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

pub type __jmp_buf = [libc::c_long; 8];

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
  pub __jmpbuf: __jmp_buf,
  pub __mask_was_saved: libc::c_int,
  pub __saved_mask: sigset_t,
}

pub type jmp_buf = [__jmp_buf_tag; 1];

#[derive(Copy, Clone)]
#[repr(C)]
pub struct line_input_t {
  pub flags: libc::c_int,
  pub timeout: libc::c_int,
  pub path_lookup: *const libc::c_char,
  pub cnt_history: libc::c_int,
  pub cur_history: libc::c_int,
  pub max_history: libc::c_int,
  pub cnt_history_in_file: libc::c_uint,
  pub hist_file: *const libc::c_char,
  pub history: [*mut libc::c_char; 256],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub line_ptr: *mut libc::c_char,
  pub disk_device: *const libc::c_char,
  pub g_partitions: libc::c_int,
  pub units_per_sector: libc::c_uint,
  pub sector_size: libc::c_uint,
  pub user_set_sector_size: libc::c_uint,
  pub sector_offset: libc::c_uint,
  pub g_heads: libc::c_uint,
  pub g_sectors: libc::c_uint,
  pub g_cylinders: libc::c_uint,
  pub current_label_type: smallint,
  pub display_in_cyl_units: smallint,
  pub listing: smallint,
  pub dos_compatible_flag: smallint,
  pub nowarn: smallint,
  pub ext_index: libc::c_int,
  pub user_cylinders: libc::c_uint,
  pub user_heads: libc::c_uint,
  pub user_sectors: libc::c_uint,
  pub pt_heads: libc::c_uint,
  pub pt_sectors: libc::c_uint,
  pub kern_heads: libc::c_uint,
  pub kern_sectors: libc::c_uint,
  pub extended_offset: sector_t,
  pub total_number_of_sectors: sector_t,
  pub listingbuf: jmp_buf,
  pub line_buffer: [libc::c_char; 80],
  pub MBRbuffer: [libc::c_char; 2048],
  pub ptes: [pte; 60],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct pte {
  pub part_table: *mut partition,
  pub ext_pointer: *mut partition,
  pub offset_from_dev_start: sector_t,
  pub sectorbuffer: *mut libc::c_char,
  pub changed: libc::c_char,
}

pub type sector_t = u32;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct partition {
  pub boot_ind: libc::c_uchar,
  pub head: libc::c_uchar,
  pub sector: libc::c_uchar,
  pub cyl: libc::c_uchar,
  pub sys_ind: libc::c_uchar,
  pub end_head: libc::c_uchar,
  pub end_sector: libc::c_uchar,
  pub end_cyl: libc::c_uchar,
  pub start4: [libc::c_uchar; 4],
  pub size4: [libc::c_uchar; 4],
}

pub type C2RustUnnamed = libc::c_uint;
// pub const OPT_s: C2RustUnnamed = 0;
pub const OPT_u: C2RustUnnamed = 32;
// pub const OPT_S: C2RustUnnamed = 16;
pub const OPT_l: C2RustUnnamed = 8;
// pub const OPT_H: C2RustUnnamed = 4;
// pub const OPT_C: C2RustUnnamed = 2;
pub const OPT_b: C2RustUnnamed = 1;

pub type ullong = libc::c_ulonglong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct hd_geometry {
  pub heads: libc::c_uchar,
  pub sectors: libc::c_uchar,
  pub cylinders: libc::c_ushort,
  pub start: libc::c_ulong,
}

pub type label_type = libc::c_uint;
// pub const LABEL_GPT: label_type = 5;
// pub const LABEL_OSF: label_type = 4;
// pub const LABEL_AIX: label_type = 3;
// pub const LABEL_SGI: label_type = 2;
// pub const LABEL_SUN: label_type = 1;
pub const LABEL_DOS: label_type = 0;

pub type action = libc::c_uint;
// pub const CREATE_EMPTY_SUN: action = 3;
pub const CREATE_EMPTY_DOS: action = 2;
pub const TRY_ONLY: action = 1;
pub const OPEN_MAIN: action = 0;
pub const dev_fd: C2RustUnnamed_1 = 3;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sun_info {
  pub spare1: libc::c_uchar,
  pub id: libc::c_uchar,
  pub spare2: libc::c_uchar,
  pub flags: libc::c_uchar,
}

/* FEATURE_FDISK_WRITABLE */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sun_partition {
  pub info: [libc::c_uchar; 128],
  pub spare0: [libc::c_uchar; 14],
  pub infos: [sun_info; 8],
  pub spare1: [libc::c_uchar; 246],
  pub rspeed: libc::c_ushort,
  pub pcylcount: libc::c_ushort,
  pub sparecyl: libc::c_ushort,
  pub spare2: [libc::c_uchar; 4],
  pub ilfact: libc::c_ushort,
  pub ncyl: libc::c_ushort,
  pub nacyl: libc::c_ushort,
  pub ntrks: libc::c_ushort,
  pub nsect: libc::c_ushort,
  pub spare3: [libc::c_uchar; 4],
  pub partitions: [sun_partinfo; 8],
  pub magic: libc::c_ushort,
  pub csum: libc::c_ushort,
  /* Label xor'd checksum */
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sun_partinfo {
  pub start_cylinder: u32,
  pub num_sectors: u32,
}

pub const COLS: C2RustUnnamed_0 = 3;
pub type C2RustUnnamed_0 = libc::c_uint;
pub type C2RustUnnamed_1 = libc::c_uint;

#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}

#[inline(always)]
unsafe extern "C" fn bb_strtoul(
  mut arg: *const libc::c_char,
  mut endp: *mut *mut libc::c_char,
  mut base: libc::c_int,
) -> libc::c_ulong {
  return bb_strtoull(arg, endp, base) as libc::c_ulong;
}

static mut msg_building_new_label: [libc::c_char; 143] = [
  66, 117, 105, 108, 100, 105, 110, 103, 32, 97, 32, 110, 101, 119, 32, 37, 115, 46, 32, 67, 104,
  97, 110, 103, 101, 115, 32, 119, 105, 108, 108, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110,
  32, 109, 101, 109, 111, 114, 121, 32, 111, 110, 108, 121, 44, 10, 117, 110, 116, 105, 108, 32,
  121, 111, 117, 32, 100, 101, 99, 105, 100, 101, 32, 116, 111, 32, 119, 114, 105, 116, 101, 32,
  116, 104, 101, 109, 46, 32, 65, 102, 116, 101, 114, 32, 116, 104, 97, 116, 32, 116, 104, 101, 32,
  112, 114, 101, 118, 105, 111, 117, 115, 32, 99, 111, 110, 116, 101, 110, 116, 10, 119, 111, 110,
  39, 116, 32, 98, 101, 32, 114, 101, 99, 111, 118, 101, 114, 97, 98, 108, 101, 46, 10, 10, 0,
];
static mut msg_part_already_defined: [libc::c_char; 61] = [
  80, 97, 114, 116, 105, 116, 105, 111, 110, 32, 37, 117, 32, 105, 115, 32, 97, 108, 114, 101, 97,
  100, 121, 32, 100, 101, 102, 105, 110, 101, 100, 44, 32, 100, 101, 108, 101, 116, 101, 32, 105,
  116, 32, 98, 101, 102, 111, 114, 101, 32, 114, 101, 45, 97, 100, 100, 105, 110, 103, 10, 0,
];
static mut i386_sys_types: [*const libc::c_char; 49] = [
  b"\x00Empty\x00" as *const u8 as *const libc::c_char,
  b"\x01FAT12\x00" as *const u8 as *const libc::c_char,
  b"\x04FAT16 <32M\x00" as *const u8 as *const libc::c_char,
  b"\x05Extended\x00" as *const u8 as *const libc::c_char,
  b"\x06FAT16\x00" as *const u8 as *const libc::c_char,
  b"\x07HPFS/NTFS\x00" as *const u8 as *const libc::c_char,
  b"\nOS/2 Boot Manager\x00" as *const u8 as *const libc::c_char,
  b"\x0bWin95 FAT32\x00" as *const u8 as *const libc::c_char,
  b"\x0cWin95 FAT32 (LBA)\x00" as *const u8 as *const libc::c_char,
  b"\x0eWin95 FAT16 (LBA)\x00" as *const u8 as *const libc::c_char,
  b"\x0fWin95 Ext\'d (LBA)\x00" as *const u8 as *const libc::c_char,
  b"\x11Hidden FAT12\x00" as *const u8 as *const libc::c_char,
  b"\x12Compaq diagnostics\x00" as *const u8 as *const libc::c_char,
  b"\x14Hidden FAT16 <32M\x00" as *const u8 as *const libc::c_char,
  b"\x16Hidden FAT16\x00" as *const u8 as *const libc::c_char,
  b"\x17Hidden HPFS/NTFS\x00" as *const u8 as *const libc::c_char,
  b"\x1bHidden Win95 FAT32\x00" as *const u8 as *const libc::c_char,
  b"\x1cHidden W95 FAT32 (LBA)\x00" as *const u8 as *const libc::c_char,
  b"\x1eHidden W95 FAT16 (LBA)\x00" as *const u8 as *const libc::c_char,
  b"<Part.Magic recovery\x00" as *const u8 as *const libc::c_char,
  b"APPC PReP Boot\x00" as *const u8 as *const libc::c_char,
  b"BSFS\x00" as *const u8 as *const libc::c_char,
  b"cGNU HURD or SysV\x00" as *const u8 as *const libc::c_char,
  b"\x80Old Minix\x00" as *const u8 as *const libc::c_char,
  b"\x81Minix / old Linux\x00" as *const u8 as *const libc::c_char,
  b"\x82Linux swap\x00" as *const u8 as *const libc::c_char,
  b"\x83Linux\x00" as *const u8 as *const libc::c_char,
  b"\x84OS/2 hidden C: drive\x00" as *const u8 as *const libc::c_char,
  b"\x85Linux extended\x00" as *const u8 as *const libc::c_char,
  b"\x86NTFS volume set\x00" as *const u8 as *const libc::c_char,
  b"\x87NTFS volume set\x00" as *const u8 as *const libc::c_char,
  b"\x8eLinux LVM\x00" as *const u8 as *const libc::c_char,
  b"\x9fBSD/OS\x00" as *const u8 as *const libc::c_char,
  b"\xa0Thinkpad hibernation\x00" as *const u8 as *const libc::c_char,
  b"\xa5FreeBSD\x00" as *const u8 as *const libc::c_char,
  b"\xa6OpenBSD\x00" as *const u8 as *const libc::c_char,
  b"\xa8Darwin UFS\x00" as *const u8 as *const libc::c_char,
  b"\xa9NetBSD\x00" as *const u8 as *const libc::c_char,
  b"\xabDarwin boot\x00" as *const u8 as *const libc::c_char,
  b"\xb7BSDI fs\x00" as *const u8 as *const libc::c_char,
  b"\xb8BSDI swap\x00" as *const u8 as *const libc::c_char,
  b"\xbeSolaris boot\x00" as *const u8 as *const libc::c_char,
  b"\xebBeOS fs\x00" as *const u8 as *const libc::c_char,
  b"\xeeEFI GPT\x00" as *const u8 as *const libc::c_char,
  b"\xefEFI (FAT-12/16/32)\x00" as *const u8 as *const libc::c_char,
  b"\xf0Linux/PA-RISC boot\x00" as *const u8 as *const libc::c_char,
  b"\xf2DOS secondary\x00" as *const u8 as *const libc::c_char,
  b"\xfdLinux raid autodetect\x00" as *const u8 as *const libc::c_char,
  0 as *const libc::c_char,
];

/* TODO: move to libbb? */
/* TODO: return unsigned long long, FEATURE_FDISK_BLKSIZE _can_ handle
 * disks > 2^32 sectors
 */
unsafe extern "C" fn bb_BLKGETSIZE_sectors(mut fd: libc::c_int) -> sector_t {
  let mut current_block: u64;
  let mut v64: u64 = 0;
  let mut longsectors: libc::c_ulong = 0;
  if ioctl(
    fd,
    (2u32 << 0i32 + 8i32 + 8i32 + 14i32
      | (0x12i32 << 0i32 + 8i32) as libc::c_uint
      | (114i32 << 0i32) as libc::c_uint) as libc::c_ulong
      | (::std::mem::size_of::<size_t>() as libc::c_ulong) << 0i32 + 8i32 + 8i32,
    &mut v64 as *mut u64,
  ) == 0i32
  {
    /* Got bytes, convert to 512 byte sectors */
    v64 >>= 9i32;
    if v64 != v64 as sector_t as libc::c_ulong {
      current_block = 4774164167766903100;
    } else {
      current_block = 15619007995458559411;
    }
  } else {
    /* Needs temp of type long */
    if ioctl(
      fd,
      (0u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (0x12i32 << 0i32 + 8i32) as libc::c_uint
        | (96i32 << 0i32) as libc::c_uint
        | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint) as libc::c_ulong,
      &mut longsectors as *mut libc::c_ulong,
    ) != 0
    {
      /* Perhaps this is a disk image */
      let mut sz: off_t = lseek(fd, 0i32 as off64_t, 2i32);
      longsectors = 0i32 as libc::c_ulong;
      if sz > 0 {
        longsectors = (sz as uoff_t).wrapping_div((*ptr_to_globals).sector_size as libc::c_ulong)
      }
      lseek(fd, 0i32 as off64_t, 0i32);
    }
    if ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
      > ::std::mem::size_of::<sector_t>() as libc::c_ulong
      && longsectors != longsectors as sector_t as libc::c_ulong
    {
      current_block = 4774164167766903100;
    } else {
      return longsectors as sector_t;
    }
  }
  match current_block {
    4774164167766903100 => {
      /* Not only DOS, but all other partition tables
       * we support can't record more than 32 bit
       * sector counts or offsets
       */
      bb_simple_error_msg(
        b"device has more than 2^32 sectors, can\'t use all of them\x00" as *const u8
          as *const libc::c_char,
      );
      v64 = -1i64 as u32 as u64
    }
    _ => {}
  }
  return v64 as sector_t;
}
unsafe extern "C" fn close_dev_fd() {
  /* Not really closing, but making sure it is open, and to harmless place */
  xmove_fd(
    xopen(b"/dev/null\x00" as *const u8 as *const libc::c_char, 0i32),
    dev_fd as libc::c_int,
  );
}
/* Return partition name */
unsafe extern "C" fn partname(
  mut dev: *const libc::c_char,
  mut pno: libc::c_int,
  mut lth: libc::c_int,
) -> *const libc::c_char {
  let mut p: *const libc::c_char = 0 as *const libc::c_char;
  let mut w: libc::c_int = 0;
  let mut wp: libc::c_int = 0;
  let mut bufsiz: libc::c_int = 0;
  let mut bufp: *mut libc::c_char = 0 as *mut libc::c_char;
  bufp = auto_string(xzalloc(80i32 as size_t) as *mut libc::c_char);
  bufsiz = 80i32;
  w = strlen(dev) as libc::c_int;
  p = b"\x00" as *const u8 as *const libc::c_char;
  if (*dev.offset((w - 1i32) as isize) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int
    <= 9i32
  {
    p = b"p\x00" as *const u8 as *const libc::c_char
  }
  /* devfs kludge - note: fdisk partition names are not supposed
  to equal kernel names, so there is no reason to do this */
  if strcmp(
    dev.offset(w as isize).offset(-4),
    b"disc\x00" as *const u8 as *const libc::c_char,
  ) == 0i32
  {
    w -= 4i32;
    p = b"part\x00" as *const u8 as *const libc::c_char
  }
  wp = strlen(p) as libc::c_int;
  if lth != 0 {
    snprintf(
      bufp,
      bufsiz as libc::c_ulong,
      b"%*.*s%s%-2u\x00" as *const u8 as *const libc::c_char,
      lth - wp - 2i32,
      w,
      dev,
      p,
      pno,
    );
  } else {
    snprintf(
      bufp,
      bufsiz as libc::c_ulong,
      b"%.*s%s%-2u\x00" as *const u8 as *const libc::c_char,
      w,
      dev,
      p,
      pno,
    );
  }
  return bufp;
}
#[inline(always)]
unsafe extern "C" fn str_units() -> *const libc::c_char {
  return if (*ptr_to_globals).display_in_cyl_units as libc::c_int != 0 {
    b"cylinder\x00" as *const u8 as *const libc::c_char
  } else {
    b"sector\x00" as *const u8 as *const libc::c_char
  };
}
unsafe extern "C" fn valid_part_table_flag(mut mbuffer: *const libc::c_char) -> libc::c_int {
  return (*mbuffer.offset(510) as libc::c_int == 0x55i32
    && *mbuffer.offset(511) as u8 as libc::c_int == 0xaai32) as libc::c_int;
}
unsafe extern "C" fn fdisk_fatal(mut why: *const libc::c_char) {
  if (*ptr_to_globals).listing != 0 {
    close_dev_fd();
    longjmp((*ptr_to_globals).listingbuf.as_mut_ptr(), 1i32);
  }
  bb_error_msg_and_die(why, (*ptr_to_globals).disk_device);
}
unsafe extern "C" fn seek_sector(mut secno: sector_t) {
  let mut off: u64 = (secno as u64).wrapping_mul((*ptr_to_globals).sector_size as libc::c_ulong);
  if off
    > (if -1i32 as off_t > 0 {
      -1i32 as off_t
    } else {
      !((1i32 as off_t)
        << (::std::mem::size_of::<off_t>() as libc::c_ulong)
          .wrapping_mul(8i32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong))
    }) as libc::c_ulong
    || lseek(dev_fd as libc::c_int, off as off_t, 0i32) == -1i32 as off_t
  {
    fdisk_fatal(b"can\'t seek on %s\x00" as *const u8 as *const libc::c_char);
  };
}
unsafe extern "C" fn set_all_unchanged() {
  let mut i: libc::c_int = 0;
  i = 0i32;
  while i < 60i32 {
    (*ptr_to_globals).ptes[i as usize].changed = 0i32 as libc::c_char;
    i += 1
  }
}
#[inline(always)]
unsafe extern "C" fn set_changed(mut i: libc::c_int) {
  (*ptr_to_globals).ptes[i as usize].changed = 1i32 as libc::c_char;
}
#[inline(always)]
unsafe extern "C" fn write_part_table_flag(mut b: *mut libc::c_char) {
  *b.offset(510) = 0x55i32 as libc::c_char;
  *b.offset(511) = 0xaai32 as libc::c_char;
}
/* Read line; return 0 or first printable non-space char */
unsafe extern "C" fn read_line(mut prompt: *const libc::c_char) -> libc::c_int {
  let mut sz: libc::c_int = 0; /* Ctrl-D or Ctrl-C */
  sz = read_line_input(
    0 as *mut line_input_t,
    prompt,
    (*ptr_to_globals).line_buffer.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong as libc::c_int,
  );
  if sz <= 0i32 {
    exit(0i32);
  }
  if (*ptr_to_globals).line_buffer[(sz - 1i32) as usize] as libc::c_int == '\n' as i32 {
    sz -= 1;
    (*ptr_to_globals).line_buffer[sz as usize] = '\u{0}' as i32 as libc::c_char
  }
  (*ptr_to_globals).line_ptr = (*ptr_to_globals).line_buffer.as_mut_ptr();
  while *(*ptr_to_globals).line_ptr as libc::c_int != '\u{0}' as i32
    && *(*ptr_to_globals).line_ptr as libc::c_uchar as libc::c_int <= ' ' as i32
  {
    (*ptr_to_globals).line_ptr = (*ptr_to_globals).line_ptr.offset(1)
  }
  return *(*ptr_to_globals).line_ptr as libc::c_int;
}
unsafe extern "C" fn read_nonempty(mut mesg: *const libc::c_char) -> libc::c_char {
  while read_line(mesg) == 0 {}
  return *(*ptr_to_globals).line_ptr;
}
unsafe extern "C" fn read_maybe_empty(mut mesg: *const libc::c_char) -> libc::c_char {
  if read_line(mesg) == 0 {
    (*ptr_to_globals).line_ptr = (*ptr_to_globals).line_buffer.as_mut_ptr();
    *(*ptr_to_globals).line_ptr.offset(0) = '\n' as i32 as libc::c_char;
    *(*ptr_to_globals).line_ptr.offset(1) = '\u{0}' as i32 as libc::c_char
  }
  return *(*ptr_to_globals).line_ptr.offset(0);
}
unsafe extern "C" fn read_hex(mut sys: *const *const libc::c_char) -> libc::c_int {
  let mut v: libc::c_ulong = 0;
  loop {
    read_nonempty(b"Hex code (type L to list codes): \x00" as *const u8 as *const libc::c_char);
    if *(*ptr_to_globals).line_ptr.offset(0) as libc::c_int | 0x20i32 == 'l' as i32 {
      list_types(sys);
    } else {
      v = bb_strtoul(
        (*ptr_to_globals).line_ptr,
        0 as *mut *mut libc::c_char,
        16i32,
      );
      if v <= 0xffi32 as libc::c_ulong {
        return v as libc::c_int;
      }
    }
  }
}
unsafe extern "C" fn write_sector(mut secno: sector_t, mut buf: *const libc::c_void) {
  seek_sector(secno);
  xwrite(
    dev_fd as libc::c_int,
    buf,
    (*ptr_to_globals).sector_size as size_t,
  );
}
#[inline(always)]
unsafe extern "C" fn read4_little_endian(mut cp: *const libc::c_uchar) -> libc::c_uint {
  let mut v: u32 = 0;
  v = *(cp as *mut bb__aliased_u32);
  return v;
}
unsafe extern "C" fn get_start_sect(mut p: *const partition) -> sector_t {
  return read4_little_endian((*p).start4.as_ptr());
}
unsafe extern "C" fn get_nr_sects(mut p: *const partition) -> sector_t {
  return read4_little_endian((*p).size4.as_ptr());
}
/* start_sect and nr_sects are stored little endian on all machines */
/* moreover, they are not aligned correctly */
#[inline(always)]
unsafe extern "C" fn store4_little_endian(mut cp: *mut libc::c_uchar, mut val: libc::c_uint) {
  let mut v: u32 = val;
  *(cp as *mut bb__aliased_u32) = v;
}
unsafe extern "C" fn set_start_sect(mut p: *mut partition, mut start_sect: libc::c_uint) {
  store4_little_endian((*p).start4.as_mut_ptr(), start_sect);
}
unsafe extern "C" fn set_nr_sects(mut p: *mut partition, mut nr_sects: libc::c_uint) {
  store4_little_endian((*p).size4.as_mut_ptr(), nr_sects);
}
/* Allocate a buffer and read a partition table sector */
unsafe extern "C" fn read_pte(mut pe: *mut pte, mut offset: sector_t) {
  (*pe).offset_from_dev_start = offset;
  (*pe).sectorbuffer = xzalloc((*ptr_to_globals).sector_size as size_t) as *mut libc::c_char;
  seek_sector(offset);
  /* xread would make us abort - bad for fdisk -l */
  if full_read(
    dev_fd as libc::c_int,
    (*pe).sectorbuffer as *mut libc::c_void,
    (*ptr_to_globals).sector_size as size_t,
  ) != (*ptr_to_globals).sector_size as isize
  {
    fdisk_fatal(b"can\'t read from %s\x00" as *const u8 as *const libc::c_char);
  }
  (*pe).changed = 0i32 as libc::c_char;
  (*pe).ext_pointer = 0 as *mut partition;
  (*pe).part_table = (*pe).ext_pointer;
}
unsafe extern "C" fn get_partition_start_from_dev_start(mut pe: *const pte) -> sector_t {
  return (*pe)
    .offset_from_dev_start
    .wrapping_add(get_start_sect((*pe).part_table));
}
/*
 * Avoid warning about DOS partitions when no DOS partition was changed.
 * Here a heuristic "is probably dos partition".
 * We might also do the opposite and warn in all cases except
 * for "is probably nondos partition".
 */
unsafe extern "C" fn menu() {
  puts(b"Command Action\x00" as *const u8 as *const libc::c_char); /* sun */
  puts(b"a\ttoggle a bootable flag\x00" as *const u8 as *const libc::c_char);
  puts(b"b\tedit bsd disklabel\x00" as *const u8 as *const libc::c_char);
  puts(b"c\ttoggle the dos compatibility flag\x00" as *const u8 as *const libc::c_char);
  puts(b"d\tdelete a partition\x00" as *const u8 as *const libc::c_char);
  puts(b"l\tlist known partition types\x00" as *const u8 as *const libc::c_char);
  puts(b"n\tadd a new partition\x00" as *const u8 as *const libc::c_char);
  puts(b"o\tcreate a new empty DOS partition table\x00" as *const u8 as *const libc::c_char);
  puts(b"p\tprint the partition table\x00" as *const u8 as *const libc::c_char);
  puts(b"q\tquit without saving changes\x00" as *const u8 as *const libc::c_char);
  puts(b"s\tcreate a new empty Sun disklabel\x00" as *const u8 as *const libc::c_char);
  puts(b"t\tchange a partition\'s system id\x00" as *const u8 as *const libc::c_char);
  puts(b"u\tchange display/entry units\x00" as *const u8 as *const libc::c_char);
  puts(b"v\tverify the partition table\x00" as *const u8 as *const libc::c_char);
  puts(b"w\twrite table to disk and exit\x00" as *const u8 as *const libc::c_char);
  puts(b"x\textra functionality (experts only)\x00" as *const u8 as *const libc::c_char);
}
/* FEATURE_FDISK_WRITABLE */
unsafe extern "C" fn xmenu() {
  puts(b"Command Action\x00" as *const u8 as *const libc::c_char); /* !sun */
  puts(b"b\tmove beginning of data in a partition\x00" as *const u8 as *const libc::c_char); /* !sun */
  puts(b"c\tchange number of cylinders\x00" as *const u8 as *const libc::c_char); /* !sun, !aix, !sgi */
  puts(b"d\tprint the raw data in the partition table\x00" as *const u8 as *const libc::c_char);
  puts(b"e\tlist extended partitions\x00" as *const u8 as *const libc::c_char);
  puts(b"f\tfix partition order\x00" as *const u8 as *const libc::c_char);
  puts(b"h\tchange number of heads\x00" as *const u8 as *const libc::c_char);
  puts(b"p\tprint the partition table\x00" as *const u8 as *const libc::c_char);
  puts(b"q\tquit without saving changes\x00" as *const u8 as *const libc::c_char);
  puts(b"r\treturn to main menu\x00" as *const u8 as *const libc::c_char);
  puts(b"s\tchange number of sectors/track\x00" as *const u8 as *const libc::c_char);
  puts(b"v\tverify the partition table\x00" as *const u8 as *const libc::c_char);
  puts(b"w\twrite table to disk and exit\x00" as *const u8 as *const libc::c_char);
}
/* ADVANCED mode */
unsafe extern "C" fn get_sys_types() -> *const *const libc::c_char {
  // This was originally
  // return (
  //   LABEL_IS_SUN ? sun_sys_types :
  //   LABEL_IS_SGI ? sgi_sys_types :
  //   i386_sys_types);
  return i386_sys_types.as_ptr();
}
unsafe extern "C" fn partition_type(mut type_0: libc::c_uchar) -> *const libc::c_char {
  let mut i: libc::c_int = 0;
  let mut types: *const *const libc::c_char = get_sys_types();
  i = 0i32;
  while !(*types.offset(i as isize)).is_null() {
    if *(*types.offset(i as isize)).offset(0) as libc::c_uchar as libc::c_int
      == type_0 as libc::c_int
    {
      return (*types.offset(i as isize)).offset(1);
    }
    i += 1
  }
  return b"Unknown\x00" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn is_cleared_partition(mut p: *const partition) -> libc::c_int {
  /* We consider partition "cleared" only if it has only zeros */
  let mut cp: *const libc::c_char = p as *const libc::c_char;
  let mut cnt: libc::c_int = ::std::mem::size_of::<partition>() as libc::c_ulong as libc::c_int;
  let mut bits: libc::c_char = 0i32 as libc::c_char;
  loop {
    cnt -= 1;
    if !(cnt >= 0i32) {
      break;
    }
    let fresh0 = cp;
    cp = cp.offset(1);
    bits = (bits as libc::c_int | *fresh0 as libc::c_int) as libc::c_char
  }
  return (bits as libc::c_int == 0i32) as libc::c_int;
}
unsafe extern "C" fn clear_partition(mut p: *mut partition) {
  if !p.is_null() {
    memset(
      p as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<partition>() as libc::c_ulong,
    );
  };
}
unsafe extern "C" fn get_sysid(mut i: libc::c_int) -> libc::c_int {
  // This was originally
  // return LABEL_IS_SUN ? sunlabel->infos[i].id :
  //         (LABEL_IS_SGI ? sgi_get_sysid(i) :
  //           ptes[i].part_table->sys_ind);
  return (*(*ptr_to_globals).ptes[i as usize].part_table).sys_ind as libc::c_int;
}
unsafe extern "C" fn list_types(mut sys: *const *const libc::c_char) {
  let mut last: [libc::c_uint; 3] = [0; 3];
  let mut done: libc::c_uint = 0;
  let mut next: libc::c_uint = 0;
  let mut size: libc::c_uint = 0;
  let mut i: libc::c_int = 0;
  size = 0i32 as libc::c_uint;
  while !(*sys.offset(size as isize)).is_null() {
    size = size.wrapping_add(1)
  }
  done = 0i32 as libc::c_uint;
  i = COLS as libc::c_int - 1i32;
  while i >= 0i32 {
    done = done.wrapping_add(
      size
        .wrapping_add(i as libc::c_uint)
        .wrapping_sub(done)
        .wrapping_div((i + 1i32) as libc::c_uint),
    );
    last[(COLS as libc::c_int - 1i32 - i) as usize] = done;
    i -= 1
  }
  next = 0i32 as libc::c_uint;
  done = next;
  i = done as libc::c_int;
  loop {
    printf(
      b"%c%2x %-22.22s\x00" as *const u8 as *const libc::c_char,
      if i != 0 { ' ' as i32 } else { '\n' as i32 },
      *(*sys.offset(next as isize)).offset(0) as libc::c_uchar as libc::c_int,
      (*sys.offset(next as isize)).offset(1),
    );
    let fresh1 = i;
    i = i + 1;
    next = last[fresh1 as usize].wrapping_add(done);
    if i >= COLS as libc::c_int || next >= last[i as usize] {
      i = 0i32;
      done = done.wrapping_add(1);
      next = done
    }
    if !(done < last[0]) {
      break;
    }
  }
  bb_putchar('\n' as i32);
}
unsafe extern "C" fn set_hsc_start_end(
  mut p: *mut partition,
  mut start: sector_t,
  mut stop: sector_t,
) {
  if (*ptr_to_globals).dos_compatible_flag as libc::c_int != 0
    && start.wrapping_div(
      (*ptr_to_globals)
        .g_sectors
        .wrapping_mul((*ptr_to_globals).g_heads),
    ) > 1023i32 as libc::c_uint
  {
    start = (*ptr_to_globals)
      .g_heads
      .wrapping_mul((*ptr_to_globals).g_sectors)
      .wrapping_mul(1024i32 as libc::c_uint)
      .wrapping_sub(1i32 as libc::c_uint)
  }
  (*p).sector = start
    .wrapping_rem((*ptr_to_globals).g_sectors)
    .wrapping_add(1i32 as libc::c_uint) as libc::c_uchar;
  start = (start as libc::c_uint).wrapping_div((*ptr_to_globals).g_sectors) as sector_t as sector_t;
  (*p).head = start.wrapping_rem((*ptr_to_globals).g_heads) as libc::c_uchar;
  start = (start as libc::c_uint).wrapping_div((*ptr_to_globals).g_heads) as sector_t as sector_t;
  (*p).cyl = (start & 0xffi32 as libc::c_uint) as libc::c_uchar;
  (*p).sector =
    ((*p).sector as libc::c_uint | start >> 2i32 & 0xc0i32 as libc::c_uint) as libc::c_uchar;
  if (*ptr_to_globals).dos_compatible_flag as libc::c_int != 0
    && stop.wrapping_div(
      (*ptr_to_globals)
        .g_sectors
        .wrapping_mul((*ptr_to_globals).g_heads),
    ) > 1023i32 as libc::c_uint
  {
    stop = (*ptr_to_globals)
      .g_heads
      .wrapping_mul((*ptr_to_globals).g_sectors)
      .wrapping_mul(1024i32 as libc::c_uint)
      .wrapping_sub(1i32 as libc::c_uint)
  }
  (*p).end_sector = stop
    .wrapping_rem((*ptr_to_globals).g_sectors)
    .wrapping_add(1i32 as libc::c_uint) as libc::c_uchar;
  stop = (stop as libc::c_uint).wrapping_div((*ptr_to_globals).g_sectors) as sector_t as sector_t;
  (*p).end_head = stop.wrapping_rem((*ptr_to_globals).g_heads) as libc::c_uchar;
  stop = (stop as libc::c_uint).wrapping_div((*ptr_to_globals).g_heads) as sector_t as sector_t;
  (*p).end_cyl = (stop & 0xffi32 as libc::c_uint) as libc::c_uchar;
  (*p).end_sector =
    ((*p).end_sector as libc::c_uint | stop >> 2i32 & 0xc0i32 as libc::c_uint) as libc::c_uchar;
}
unsafe extern "C" fn set_partition(
  mut i: libc::c_int,
  mut doext: libc::c_int,
  mut start: sector_t,
  mut stop: sector_t,
  mut sysid: libc::c_int,
) {
  let mut p: *mut partition = 0 as *mut partition;
  let mut offset: sector_t = 0;
  if doext != 0 {
    p = (*ptr_to_globals).ptes[i as usize].ext_pointer;
    offset = (*ptr_to_globals).extended_offset
  } else {
    p = (*ptr_to_globals).ptes[i as usize].part_table;
    offset = (*ptr_to_globals).ptes[i as usize].offset_from_dev_start
  }
  (*p).boot_ind = 0i32 as libc::c_uchar;
  (*p).sys_ind = sysid as libc::c_uchar;
  set_start_sect(p, start.wrapping_sub(offset));
  set_nr_sects(
    p,
    stop.wrapping_sub(start).wrapping_add(1i32 as libc::c_uint),
  );
  set_hsc_start_end(p, start, stop);
  (*ptr_to_globals).ptes[i as usize].changed = 1i32 as libc::c_char;
}
unsafe extern "C" fn warn_geometry() -> libc::c_int {
  if (*ptr_to_globals).g_heads != 0
    && (*ptr_to_globals).g_sectors != 0
    && (*ptr_to_globals).g_cylinders != 0
  {
    return 0i32;
  }
  printf(b"Unknown value(s) for:\x00" as *const u8 as *const libc::c_char);
  if (*ptr_to_globals).g_heads == 0 {
    printf(b" heads\x00" as *const u8 as *const libc::c_char);
  }
  if (*ptr_to_globals).g_sectors == 0 {
    printf(b" sectors\x00" as *const u8 as *const libc::c_char);
  }
  if (*ptr_to_globals).g_cylinders == 0 {
    printf(b" cylinders\x00" as *const u8 as *const libc::c_char);
  }
  puts(b" (settable in the extra functions menu)\x00" as *const u8 as *const libc::c_char);
  return 1i32;
}
unsafe extern "C" fn update_units() {
  let mut cyl_units: libc::c_int = (*ptr_to_globals)
    .g_heads
    .wrapping_mul((*ptr_to_globals).g_sectors) as libc::c_int;
  if (*ptr_to_globals).display_in_cyl_units as libc::c_int != 0 && cyl_units != 0 {
    (*ptr_to_globals).units_per_sector = cyl_units as libc::c_uint
  } else {
    (*ptr_to_globals).units_per_sector = 1i32 as libc::c_uint
  };
  /* in sectors */
}
unsafe extern "C" fn warn_cylinders() {
  if LABEL_DOS as libc::c_int == (*ptr_to_globals).current_label_type as libc::c_int
    && (*ptr_to_globals).g_cylinders > 1024i32 as libc::c_uint
    && (*ptr_to_globals).nowarn == 0
  {
    printf(b"\nThe number of cylinders for this disk is set to %u.\nThere is nothing wrong with that, but this is larger than 1024,\nand could in certain setups cause problems with:\n1) software that runs at boot time (e.g., old versions of LILO)\n2) booting and partitioning software from other OSs\n   (e.g., DOS FDISK, OS/2 FDISK)\n\x00"
                   as *const u8 as *const libc::c_char,
               (*ptr_to_globals).g_cylinders);
  };
}
unsafe extern "C" fn read_extended(mut ext: libc::c_int) {
  let mut i: libc::c_int = 0;
  let mut pex: *mut pte = 0 as *mut pte;
  let mut p: *mut partition = 0 as *mut partition;
  let mut q: *mut partition = 0 as *mut partition;
  (*ptr_to_globals).ext_index = ext;
  pex = &mut *(*ptr_to_globals).ptes.as_mut_ptr().offset(ext as isize) as *mut pte;
  (*pex).ext_pointer = (*pex).part_table;
  p = (*pex).part_table;
  if get_start_sect(p) == 0 {
    puts(b"Bad offset in primary extended partition\x00" as *const u8 as *const libc::c_char);
    return;
  }
  while (*p).sys_ind as libc::c_int == 0x5i32
    || (*p).sys_ind as libc::c_int == 0xfi32
    || (*p).sys_ind as libc::c_int == 0x85i32
  {
    let mut pe: *mut pte = &mut *(*ptr_to_globals)
      .ptes
      .as_mut_ptr()
      .offset((*ptr_to_globals).g_partitions as isize) as *mut pte;
    if (*ptr_to_globals).g_partitions >= 60i32 {
      /* This is not a Linux restriction, but
      this program uses arrays of size MAXIMUM_PARTS.
      Do not try to 'improve' this test. */
      let mut pre: *mut pte = &mut *(*ptr_to_globals)
        .ptes
        .as_mut_ptr()
        .offset(((*ptr_to_globals).g_partitions - 1i32) as isize)
        as *mut pte;
      printf(
        b"Warning: deleting partitions after %u\n\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).g_partitions,
      );
      (*pre).changed = 1i32 as libc::c_char;
      clear_partition((*pre).ext_pointer);
      return;
    }
    read_pte(
      pe,
      (*ptr_to_globals)
        .extended_offset
        .wrapping_add(get_start_sect(p)),
    );
    if (*ptr_to_globals).extended_offset == 0 {
      (*ptr_to_globals).extended_offset = get_start_sect(p)
    }
    p = (*pe).sectorbuffer.offset(0x1bei32 as isize).offset(
      (0i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<partition>() as libc::c_ulong)
        as isize,
    ) as *mut partition;
    q = p;
    i = 0i32;
    while i < 4i32 {
      if get_nr_sects(p) != 0 {
        if (*p).sys_ind as libc::c_int == 0x5i32
          || (*p).sys_ind as libc::c_int == 0xfi32
          || (*p).sys_ind as libc::c_int == 0x85i32
        {
          if !(*pe).ext_pointer.is_null() {
            printf(
              b"Warning: extra link pointer in partition table %u\n\x00" as *const u8
                as *const libc::c_char,
              (*ptr_to_globals).g_partitions + 1i32,
            );
          } else {
            (*pe).ext_pointer = p
          }
        } else if (*p).sys_ind != 0 {
          if !(*pe).part_table.is_null() {
            printf(
              b"Warning: ignoring extra data in partition table %u\n\x00" as *const u8
                as *const libc::c_char,
              (*ptr_to_globals).g_partitions + 1i32,
            );
          } else {
            (*pe).part_table = p
          }
        }
      }
      i += 1;
      p = p.offset(1)
    }
    /* very strange code here... */
    if (*pe).part_table.is_null() {
      if q != (*pe).ext_pointer {
        (*pe).part_table = q
      } else {
        (*pe).part_table = q.offset(1)
      }
    }
    if (*pe).ext_pointer.is_null() {
      if q != (*pe).part_table {
        (*pe).ext_pointer = q
      } else {
        (*pe).ext_pointer = q.offset(1)
      }
    }
    p = (*pe).ext_pointer;
    (*ptr_to_globals).g_partitions += 1
  }
  's_222: loop
  /* remove empty links */
  {
    i = 4i32;
    loop {
      if !(i < (*ptr_to_globals).g_partitions) {
        break 's_222;
      }
      let mut pe_0: *mut pte =
        &mut *(*ptr_to_globals).ptes.as_mut_ptr().offset(i as isize) as *mut pte;
      if get_nr_sects((*pe_0).part_table) == 0
        && ((*ptr_to_globals).g_partitions > 5i32
          || (*(*ptr_to_globals).ptes[4].part_table).sys_ind as libc::c_int != 0)
      {
        printf(
          b"Omitting empty partition (%u)\n\x00" as *const u8 as *const libc::c_char,
          i + 1i32,
        );
        delete_partition(i);
        break;
      /* numbering changed */
      } else {
        i += 1
      }
    }
  }
}
unsafe extern "C" fn create_doslabel() {
  printf(
    msg_building_new_label.as_ptr(),
    b"DOS disklabel\x00" as *const u8 as *const libc::c_char,
  );
  (*ptr_to_globals).current_label_type = LABEL_DOS as libc::c_int as smallint;
  (*ptr_to_globals).g_partitions = 4i32;
  memset(
    &mut *(*ptr_to_globals)
      .MBRbuffer
      .as_mut_ptr()
      .offset((510i32 - 4i32 * 16i32) as isize) as *mut libc::c_char as *mut libc::c_void,
    0i32,
    (4i32 * 16i32) as libc::c_ulong,
  );
  write_part_table_flag((*ptr_to_globals).MBRbuffer.as_mut_ptr());
  (*ptr_to_globals).extended_offset = 0i32 as sector_t;
  set_all_unchanged();
  set_changed(0i32);
  get_boot(CREATE_EMPTY_DOS);
}
unsafe extern "C" fn get_sectorsize() {
  if (*ptr_to_globals).user_set_sector_size == 0 {
    let mut arg: libc::c_int = 0;
    if ioctl(
      dev_fd as libc::c_int,
      (0u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (0x12i32 << 0i32 + 8i32) as libc::c_uint
        | (104i32 << 0i32) as libc::c_uint
        | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint) as libc::c_ulong,
      &mut arg as *mut libc::c_int,
    ) == 0i32
    {
      (*ptr_to_globals).sector_size = arg as libc::c_uint
    }
    if (*ptr_to_globals).sector_size != 512i32 as libc::c_uint {
      printf(
        b"Note: sector size is %u (not 512)\n\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).sector_size,
      );
    }
  };
}
unsafe extern "C" fn get_kernel_geometry() {
  let mut geometry: hd_geometry = hd_geometry {
    heads: 0,
    sectors: 0,
    cylinders: 0,
    start: 0,
  };
  if ioctl(
    dev_fd as libc::c_int,
    0x301i32 as libc::c_ulong,
    &mut geometry as *mut hd_geometry,
  ) == 0
  {
    (*ptr_to_globals).kern_heads = geometry.heads as libc::c_uint;
    (*ptr_to_globals).kern_sectors = geometry.sectors as libc::c_uint
    /* never use geometry.cylinders - it is truncated */
  };
}
unsafe extern "C" fn get_partition_table_geometry() {
  let mut bufp: *const libc::c_uchar =
    (*ptr_to_globals).MBRbuffer.as_mut_ptr() as *const libc::c_uchar;
  let mut p: *mut partition = 0 as *mut partition;
  let mut i: libc::c_int = 0;
  let mut h: libc::c_int = 0;
  let mut s: libc::c_int = 0;
  let mut hh: libc::c_int = 0;
  let mut ss: libc::c_int = 0;
  let mut first: libc::c_int = 1i32;
  let mut bad: libc::c_int = 0i32;
  if valid_part_table_flag(bufp as *mut libc::c_char) == 0 {
    return;
  }
  ss = 0i32;
  hh = ss;
  i = 0i32;
  while i < 4i32 {
    p = bufp.offset(0x1bei32 as isize).offset(
      (i as libc::c_ulong).wrapping_mul(::std::mem::size_of::<partition>() as libc::c_ulong)
        as isize,
    ) as *mut partition;
    if (*p).sys_ind as libc::c_int != 0i32 {
      h = (*p).end_head as libc::c_int + 1i32;
      s = (*p).end_sector as libc::c_int & 0o77i32;
      if first != 0 {
        hh = h;
        ss = s;
        first = 0i32
      } else if hh != h || ss != s {
        bad = 1i32
      }
    }
    i += 1
  }
  if first == 0 && bad == 0 {
    (*ptr_to_globals).pt_heads = hh as libc::c_uint;
    (*ptr_to_globals).pt_sectors = ss as libc::c_uint
  };
}
unsafe extern "C" fn get_geometry() {
  let mut sec_fac: libc::c_int = 0;
  get_sectorsize();
  sec_fac = (*ptr_to_globals)
    .sector_size
    .wrapping_div(512i32 as libc::c_uint) as libc::c_int;
  (*ptr_to_globals).g_sectors = 0i32 as libc::c_uint;
  (*ptr_to_globals).g_cylinders = (*ptr_to_globals).g_sectors;
  (*ptr_to_globals).g_heads = (*ptr_to_globals).g_cylinders;
  (*ptr_to_globals).kern_sectors = 0i32 as libc::c_uint;
  (*ptr_to_globals).kern_heads = (*ptr_to_globals).kern_sectors;
  (*ptr_to_globals).pt_sectors = 0i32 as libc::c_uint;
  (*ptr_to_globals).pt_heads = (*ptr_to_globals).pt_sectors;
  get_kernel_geometry();
  get_partition_table_geometry();
  (*ptr_to_globals).g_heads = if (*ptr_to_globals).user_heads != 0 {
    (*ptr_to_globals).user_heads
  } else if (*ptr_to_globals).pt_heads != 0 {
    (*ptr_to_globals).pt_heads
  } else if (*ptr_to_globals).kern_heads != 0 {
    (*ptr_to_globals).kern_heads
  } else {
    255i32 as libc::c_uint
  };
  (*ptr_to_globals).g_sectors = if (*ptr_to_globals).user_sectors != 0 {
    (*ptr_to_globals).user_sectors
  } else if (*ptr_to_globals).pt_sectors != 0 {
    (*ptr_to_globals).pt_sectors
  } else if (*ptr_to_globals).kern_sectors != 0 {
    (*ptr_to_globals).kern_sectors
  } else {
    63i32 as libc::c_uint
  };
  (*ptr_to_globals).total_number_of_sectors = bb_BLKGETSIZE_sectors(dev_fd as libc::c_int);
  (*ptr_to_globals).sector_offset = 1i32 as libc::c_uint;
  if (*ptr_to_globals).dos_compatible_flag != 0 {
    (*ptr_to_globals).sector_offset = (*ptr_to_globals).g_sectors
  }
  (*ptr_to_globals).g_cylinders = (*ptr_to_globals).total_number_of_sectors.wrapping_div(
    (*ptr_to_globals)
      .g_heads
      .wrapping_mul((*ptr_to_globals).g_sectors)
      .wrapping_mul(sec_fac as libc::c_uint),
  );
  if (*ptr_to_globals).g_cylinders == 0 {
    (*ptr_to_globals).g_cylinders = (*ptr_to_globals).user_cylinders
  };
}
/*
 * Opens disk_device and optionally reads MBR.
 *    If what == OPEN_MAIN:
 *      Open device, read MBR.  Abort program on short read.  Create empty
 *      disklabel if the on-disk structure is invalid (WRITABLE mode).
 *    If what == TRY_ONLY:
 *      Open device, read MBR.  Return an error if anything is out of place.
 *      Do not create an empty disklabel.  This is used for the "list"
 *      operations: "fdisk -l /dev/sda" and "fdisk -l" (all devices).
 *    If what == CREATE_EMPTY_*:
 *      This means that get_boot() was called recursively from create_*label().
 *      Do not re-open the device; just set up the ptes array and print
 *      geometry warnings.
 *
 * Returns:
 *   -1: no 0xaa55 flag present (possibly entire disk BSD)
 *    0: found or created label
 *    1: I/O error
 */
unsafe extern "C" fn get_boot(mut what: action) -> libc::c_int {
  let mut i: libc::c_int = 0;
  let mut fd: libc::c_int = 0;
  (*ptr_to_globals).g_partitions = 4i32;
  i = 0i32;
  while i < 4i32 {
    let mut pe: *mut pte = &mut *(*ptr_to_globals).ptes.as_mut_ptr().offset(i as isize) as *mut pte;
    (*pe).part_table = (*ptr_to_globals)
      .MBRbuffer
      .as_mut_ptr()
      .offset(0x1bei32 as isize)
      .offset(
        (i as libc::c_ulong).wrapping_mul(::std::mem::size_of::<partition>() as libc::c_ulong)
          as isize,
      ) as *mut partition;
    (*pe).ext_pointer = 0 as *mut partition;
    (*pe).offset_from_dev_start = 0i32 as sector_t;
    (*pe).sectorbuffer = (*ptr_to_globals).MBRbuffer.as_mut_ptr();
    (*pe).changed = (what as libc::c_uint == CREATE_EMPTY_DOS as libc::c_int as libc::c_uint)
      as libc::c_int as libc::c_char;
    i += 1
  }
  // ALERT! highly idiotic design!
  // We end up here when we call get_boot() recursively
  // via get_boot() [table is bad] -> create_doslabel() -> get_boot(CREATE_EMPTY_DOS).
  // or get_boot() [table is bad] -> create_sunlabel() -> get_boot(CREATE_EMPTY_SUN).
  // (just factor out re-init of ptes[0,1,2,3] in a separate fn instead?)
  // So skip opening device _again_...
  if !(what as libc::c_uint == CREATE_EMPTY_DOS as libc::c_int as libc::c_uint) {
    fd = open(
      (*ptr_to_globals).disk_device,
      if option_mask32 & OPT_l as libc::c_int as libc::c_uint != 0 {
        0i32
      } else {
        0o2i32
      },
    );
    if fd < 0i32 {
      fd = open((*ptr_to_globals).disk_device, 0i32);
      if fd < 0i32 {
        if what as libc::c_uint == TRY_ONLY as libc::c_int as libc::c_uint {
          return 1i32;
        }
        fdisk_fatal(b"can\'t open \'%s\'\x00" as *const u8 as *const libc::c_char);
      }
      printf(
        b"\'%s\' is opened for read only\n\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).disk_device,
      );
    }
    xmove_fd(fd, dev_fd as libc::c_int);
    if 512i32 as libc::c_long
      != full_read(
        dev_fd as libc::c_int,
        (*ptr_to_globals).MBRbuffer.as_mut_ptr() as *mut libc::c_void,
        512i32 as size_t,
      ) as i64
    {
      if what as libc::c_uint == TRY_ONLY as libc::c_int as libc::c_uint {
        close_dev_fd();
        return 1i32;
      }
      fdisk_fatal(b"can\'t read from %s\x00" as *const u8 as *const libc::c_char);
    }
    get_geometry();
    update_units();
    if valid_part_table_flag((*ptr_to_globals).MBRbuffer.as_mut_ptr()) == 0 {
      if what as libc::c_uint == OPEN_MAIN as libc::c_int as libc::c_uint {
        puts(b"Device contains neither a valid DOS partition table, nor Sun, SGI, OSF or GPT disklabel\x00"
                         as *const u8 as *const libc::c_char);
        create_doslabel();
        return 0i32;
      }
      /* TRY_ONLY: */
      return -1i32;
    }
  }
  /* FEATURE_FDISK_WRITABLE */
  warn_cylinders();
  warn_geometry();
  i = 0i32;
  while i < 4i32 {
    if (*(*ptr_to_globals).ptes[i as usize].part_table).sys_ind as libc::c_int == 0x5i32
      || (*(*ptr_to_globals).ptes[i as usize].part_table).sys_ind as libc::c_int == 0xfi32
      || (*(*ptr_to_globals).ptes[i as usize].part_table).sys_ind as libc::c_int == 0x85i32
    {
      if (*ptr_to_globals).g_partitions != 4i32 {
        printf(
          b"Ignoring extra extended partition %u\n\x00" as *const u8 as *const libc::c_char,
          i + 1i32,
        );
      } else {
        read_extended(i);
      }
    }
    i += 1
  }
  i = 3i32;
  while i < (*ptr_to_globals).g_partitions {
    let mut pe_0: *mut pte =
      &mut *(*ptr_to_globals).ptes.as_mut_ptr().offset(i as isize) as *mut pte;
    if valid_part_table_flag((*pe_0).sectorbuffer) == 0 {
      printf(b"Warning: invalid flag 0x%02x,0x%02x of partition table %u will be corrected by w(rite)\n\x00"
                       as *const u8 as *const libc::c_char,
                   *(*pe_0).sectorbuffer.offset(510) as libc::c_int,
                   *(*pe_0).sectorbuffer.offset(511) as libc::c_int,
                   i + 1i32);
      (*pe_0).changed = 1i32 as libc::c_char
    }
    i += 1
  }
  return 0i32;
}
/*
 * Print the message MESG, then read an integer between LOW and HIGH (inclusive).
 * If the user hits Enter, DFLT is returned.
 * Answers like +10 are interpreted as offsets from BASE.
 *
 * There is no default if DFLT is not between LOW and HIGH.
 */
unsafe extern "C" fn read_int(
  mut low: sector_t,
  mut dflt: sector_t,
  mut high: sector_t,
  mut base: sector_t,
  mut mesg: *const libc::c_char,
) -> sector_t {
  let mut value: sector_t = 0;
  let mut default_ok: libc::c_int = 1i32;
  let mut fmt: *const libc::c_char =
    b"%s (%u-%u, default %u): \x00" as *const u8 as *const libc::c_char;
  if dflt < low || dflt > high {
    fmt = b"%s (%u-%u): \x00" as *const u8 as *const libc::c_char;
    default_ok = 0i32
  }
  loop {
    let mut use_default: libc::c_int = default_ok;
    loop
    /* ask question and read answer */
    {
      printf(fmt, mesg, low, high, dflt);
      read_maybe_empty(b"\x00" as *const u8 as *const libc::c_char);
      if !(*(*ptr_to_globals).line_ptr as libc::c_int != '\n' as i32
        && !((*(*ptr_to_globals).line_ptr as libc::c_int - '0' as i32) as libc::c_uchar
          as libc::c_int
          <= 9i32)
        && *(*ptr_to_globals).line_ptr as libc::c_int != '-' as i32
        && *(*ptr_to_globals).line_ptr as libc::c_int != '+' as i32)
      {
        break;
      }
    }
    if *(*ptr_to_globals).line_ptr as libc::c_int == '+' as i32
      || *(*ptr_to_globals).line_ptr as libc::c_int == '-' as i32
    {
      let mut minus: libc::c_int =
        (*(*ptr_to_globals).line_ptr as libc::c_int == '-' as i32) as libc::c_int;
      let mut scale_shift: libc::c_uint = 0;
      if ::std::mem::size_of::<sector_t>() as libc::c_ulong
        <= ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
      {
        value = strtoul(
          (*ptr_to_globals).line_ptr.offset(1),
          0 as *mut *mut libc::c_char,
          10i32,
        ) as sector_t
      } else {
        value = strtoull(
          (*ptr_to_globals).line_ptr.offset(1),
          0 as *mut *mut libc::c_char,
          10i32,
        ) as sector_t
      }
      loop
      /* (1) if 2nd char is digit, use_default = 0.
       * (2) move line_ptr to first non-digit.
       */
      {
        (*ptr_to_globals).line_ptr = (*ptr_to_globals).line_ptr.offset(1); /* 1024 */
        if !((*(*ptr_to_globals).line_ptr as libc::c_int - '0' as i32) as libc::c_uchar
          as libc::c_int
          <= 9i32)
        {
          break;
        }
        use_default = 0i32
      }
      scale_shift = 0i32 as libc::c_uint;
      match *(*ptr_to_globals).line_ptr as libc::c_int | 0x20i32 {
        107 => scale_shift = 10i32 as libc::c_uint,
        109 => {
          /*
           * fdisk from util-linux 2.31 seems to round '+NNNk' and '+NNNK' to megabytes,
           * (512-byte) sector count of the partition does not equal NNN*2:
           *
           * Last sector, +sectors or +size{K,M,G,T,P} (1953792-1000215215, default 1000215215): +9727k
           *   Device     Boot   Start     End Sectors  Size Id Type
           *   /dev/sdaN       1953792 1972223   18432    9M 83 Linux   <-- size exactly 9*1024*1024 bytes
           *
           * Last sector, +sectors or +size{K,M,G,T,P} (1953792-1000215215, default 1000215215): +9728k
           *   /dev/sdaN       1953792 1974271   20480   10M 83 Linux   <-- size exactly 10*1024*1024 bytes
           *
           * If 'k' means 1000 bytes (not 1024), then 9728k = 9728*1000 = 9500*1024,
           * exactly halfway from 9000 to 10000, which explains why it jumps to next mbyte
           * at this value.
           *
           * 'm' does not seem to behave this way: it means 1024*1024 bytes.
           *
           * Not sure we want to copy this. If user says he wants 1234kbyte partition,
           * we do _exactly that_: 1234kbytes = 2468 sectors.
           */
          scale_shift = 20i32 as libc::c_uint
        }
        103 => {
          /* 1024*1024 */
          scale_shift = 30i32 as libc::c_uint
        }
        116 => {
          /* 1024*1024*1024 */
          scale_shift = 40i32 as libc::c_uint
        }
        _ => {}
      } /* 1024*1024*1024*1024 */
      if scale_shift != 0 {
        let mut bytes: ullong = 0; /* round */
        let mut unit: libc::c_ulong = 0;
        bytes = (value as ullong) << scale_shift;
        unit = (*ptr_to_globals)
          .sector_size
          .wrapping_mul((*ptr_to_globals).units_per_sector) as libc::c_ulong;
        bytes = (bytes as libc::c_ulonglong)
          .wrapping_add(unit.wrapping_div(2i32 as libc::c_ulong) as libc::c_ulonglong)
          as ullong as ullong;
        bytes =
          (bytes as libc::c_ulonglong).wrapping_div(unit as libc::c_ulonglong) as ullong as ullong;
        value = if bytes != 0i32 as libc::c_ulonglong {
          bytes.wrapping_sub(1i32 as libc::c_ulonglong)
        } else {
          0i32 as libc::c_ulonglong
        } as sector_t
      }
      if minus != 0 {
        value = value.wrapping_neg()
      }
      value = (value as libc::c_uint).wrapping_add(base) as sector_t as sector_t
    } else {
      if ::std::mem::size_of::<sector_t>() as libc::c_ulong
        <= ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
      {
        value = strtoul(
          (*ptr_to_globals).line_ptr,
          0 as *mut *mut libc::c_char,
          10i32,
        ) as sector_t
      } else {
        value = strtoull(
          (*ptr_to_globals).line_ptr,
          0 as *mut *mut libc::c_char,
          10i32,
        ) as sector_t
      }
      while (*(*ptr_to_globals).line_ptr as libc::c_int - '0' as i32) as libc::c_uchar
        as libc::c_int
        <= 9i32
      {
        (*ptr_to_globals).line_ptr = (*ptr_to_globals).line_ptr.offset(1);
        use_default = 0i32
      }
    }
    if use_default != 0 {
      value = dflt;
      printf(
        b"Using default value %u\n\x00" as *const u8 as *const libc::c_char,
        value,
      );
    }
    if value >= low && value <= high {
      break;
    }
    puts(b"Value is out of range\x00" as *const u8 as *const libc::c_char);
  }
  return value;
}
unsafe extern "C" fn get_partition(mut warn: libc::c_int, mut max: libc::c_uint) -> libc::c_uint {
  let mut pe: *mut pte = 0 as *mut pte;
  let mut i: libc::c_uint = 0;
  i = read_int(
    1i32 as sector_t,
    0i32 as sector_t,
    max,
    0i32 as sector_t,
    b"Partition number\x00" as *const u8 as *const libc::c_char,
  )
  .wrapping_sub(1i32 as libc::c_uint);
  pe = &mut *(*ptr_to_globals).ptes.as_mut_ptr().offset(i as isize) as *mut pte;

  // This was originally
  // if (warn) {
  //   if ((!LABEL_IS_SUN && !LABEL_IS_SGI && !pe->part_table->sys_ind)
  //   || (LABEL_IS_SUN && (!sunlabel->partitions[i].num_sectors || !sunlabel->infos[i].id))
  //   || (LABEL_IS_SGI && !sgi_get_num_sectors(i))
  //   ) {
  //     printf("Warning: partition %u has empty type\n", i+1);
  //   }
  // }
  if warn != 0 {
    if (*(*pe).part_table).sys_ind == 0 {
      printf(
        b"Warning: partition %u has empty type\n\x00" as *const u8 as *const libc::c_char,
        i.wrapping_add(1i32 as libc::c_uint),
      );
    }
  }
  return i;
}
unsafe extern "C" fn get_existing_partition(
  mut warn: libc::c_int,
  mut max: libc::c_uint,
) -> libc::c_int {
  let mut current_block: u64;
  let mut pno: libc::c_int = -1i32;
  let mut i: libc::c_uint = 0;
  i = 0i32 as libc::c_uint;
  loop {
    if !(i < max) {
      current_block = 8515828400728868193;
      break;
    }
    let mut pe: *mut pte = &mut *(*ptr_to_globals).ptes.as_mut_ptr().offset(i as isize) as *mut pte;
    let mut p: *mut partition = (*pe).part_table;
    if !p.is_null() && is_cleared_partition(p) == 0 {
      if pno >= 0i32 {
        current_block = 325519450371384539;
        break;
      }
      pno = i as libc::c_int
    }
    i = i.wrapping_add(1)
  }
  match current_block {
    325519450371384539 => return get_partition(warn, max) as libc::c_int,
    _ => {
      if pno >= 0i32 {
        printf(
          b"Selected partition %u\n\x00" as *const u8 as *const libc::c_char,
          pno + 1i32,
        );
        return pno;
      }
      puts(b"No partition is defined yet!\x00" as *const u8 as *const libc::c_char);
      return -1i32;
    }
  };
}
unsafe extern "C" fn get_nonexisting_partition() -> libc::c_int {
  let mut current_block: u64;
  let max: libc::c_int = 4i32;
  let mut pno: libc::c_int = -1i32;
  let mut i: libc::c_uint = 0;
  i = 0i32 as libc::c_uint;
  loop {
    if !(i < max as libc::c_uint) {
      current_block = 3276175668257526147;
      break;
    }
    let mut pe: *mut pte = &mut *(*ptr_to_globals).ptes.as_mut_ptr().offset(i as isize) as *mut pte;
    let mut p: *mut partition = (*pe).part_table;
    if !p.is_null() && is_cleared_partition(p) != 0 {
      if pno >= 0i32 {
        current_block = 788725619030147294;
        break;
      }
      pno = i as libc::c_int
    }
    i = i.wrapping_add(1)
  }
  match current_block {
    788725619030147294 => return get_partition(0i32, max as libc::c_uint) as libc::c_int,
    _ => {
      if pno >= 0i32 {
        printf(
          b"Selected partition %u\n\x00" as *const u8 as *const libc::c_char,
          pno + 1i32,
        );
        return pno;
      }
      puts(
        b"All primary partitions have been defined already!\x00" as *const u8
          as *const libc::c_char,
      );
      return -1i32;
    }
  };
}
unsafe extern "C" fn change_units() {
  (*ptr_to_globals).display_in_cyl_units =
    ((*ptr_to_globals).display_in_cyl_units == 0) as libc::c_int as smallint;
  update_units();
  printf(
    b"Changing display/entry units to %ss\n\x00" as *const u8 as *const libc::c_char,
    str_units(),
  );
}
unsafe extern "C" fn toggle_active(mut i: libc::c_int) {
  let mut pe: *mut pte = &mut *(*ptr_to_globals).ptes.as_mut_ptr().offset(i as isize) as *mut pte;
  let mut p: *mut partition = (*pe).part_table;
  if ((*p).sys_ind as libc::c_int == 0x5i32
    || (*p).sys_ind as libc::c_int == 0xfi32
    || (*p).sys_ind as libc::c_int == 0x85i32)
    && (*p).boot_ind == 0
  {
    printf(
      b"WARNING: Partition %u is an extended partition\n\x00" as *const u8 as *const libc::c_char,
      i + 1i32,
    );
  }
  (*p).boot_ind = if (*p).boot_ind as libc::c_int != 0 {
    0i32
  } else {
    0x80i32
  } as libc::c_uchar;
  (*pe).changed = 1i32 as libc::c_char;
}
unsafe extern "C" fn toggle_dos_compatibility_flag() {
  (*ptr_to_globals).dos_compatible_flag =
    (1i32 - (*ptr_to_globals).dos_compatible_flag as libc::c_int) as smallint;
  if (*ptr_to_globals).dos_compatible_flag != 0 {
    (*ptr_to_globals).sector_offset = (*ptr_to_globals).g_sectors;
    printf(
      b"DOS Compatibility flag is %sset\n\x00" as *const u8 as *const libc::c_char,
      b"\x00" as *const u8 as *const libc::c_char,
    );
  } else {
    (*ptr_to_globals).sector_offset = 1i32 as libc::c_uint;
    printf(
      b"DOS Compatibility flag is %sset\n\x00" as *const u8 as *const libc::c_char,
      b"not \x00" as *const u8 as *const libc::c_char,
    );
  };
}
unsafe extern "C" fn delete_partition(mut i: libc::c_int) {
  let mut pe: *mut pte = &mut *(*ptr_to_globals).ptes.as_mut_ptr().offset(i as isize) as *mut pte;
  let mut p: *mut partition = (*pe).part_table;
  let mut q: *mut partition = (*pe).ext_pointer;
  /* Note that for the fifth partition (i == 4) we don't actually
   * decrement partitions.
   */
  if warn_geometry() != 0 {
    return;
  } /* C/H/S not set */
  (*pe).changed = 1i32 as libc::c_char;
  if i < 4i32 {
    if ((*p).sys_ind as libc::c_int == 0x5i32
      || (*p).sys_ind as libc::c_int == 0xfi32
      || (*p).sys_ind as libc::c_int == 0x85i32)
      && i == (*ptr_to_globals).ext_index
    {
      (*ptr_to_globals).g_partitions = 4i32;
      (*ptr_to_globals).ptes[(*ptr_to_globals).ext_index as usize].ext_pointer =
        0 as *mut partition;
      (*ptr_to_globals).extended_offset = 0i32 as sector_t
    }
    clear_partition(p);
    return;
  }
  if (*q).sys_ind == 0 && i > 4i32 {
    /* the last one in the chain - just delete */
    (*ptr_to_globals).g_partitions -= 1;
    i -= 1;
    clear_partition((*ptr_to_globals).ptes[i as usize].ext_pointer);
    (*ptr_to_globals).ptes[i as usize].changed = 1i32 as libc::c_char
  } else {
    /* not the last one - further ones will be moved down */
    if i > 4i32 {
      /* delete this link in the chain */
      p = (*ptr_to_globals).ptes[(i - 1i32) as usize].ext_pointer;
      *p = *q;
      set_start_sect(p, get_start_sect(q));
      set_nr_sects(p, get_nr_sects(q));
      (*ptr_to_globals).ptes[(i - 1i32) as usize].changed = 1i32 as libc::c_char
    } else if (*ptr_to_globals).g_partitions > 5i32 {
      /* 5 will be moved to 4 */
      /* the first logical in a longer chain */
      pe = &mut *(*ptr_to_globals).ptes.as_mut_ptr().offset(5) as *mut pte;
      if !(*pe).part_table.is_null() {
        /* prevent SEGFAULT */
        set_start_sect(
          (*pe).part_table,
          get_partition_start_from_dev_start(pe).wrapping_sub((*ptr_to_globals).extended_offset),
        );
      }
      (*pe).offset_from_dev_start = (*ptr_to_globals).extended_offset;
      (*pe).changed = 1i32 as libc::c_char
    }
    if (*ptr_to_globals).g_partitions > 5i32 {
      (*ptr_to_globals).g_partitions -= 1;
      while i < (*ptr_to_globals).g_partitions {
        (*ptr_to_globals).ptes[i as usize] = (*ptr_to_globals).ptes[(i + 1i32) as usize];
        i += 1
      }
    } else {
      /* the only logical: clear only */
      clear_partition((*ptr_to_globals).ptes[i as usize].part_table);
    }
  };
}
unsafe extern "C" fn change_sysid() {
  let mut i: libc::c_int = 0;
  let mut sys: libc::c_int = 0;
  let mut origsys: libc::c_int = 0;
  let mut p: *mut partition = 0 as *mut partition;
  /* If sgi_label then don't use get_existing_partition,
  let the user select a partition, since get_existing_partition()
  only works for Linux like partition tables. */
  if 0i32 == 0 {
    i = get_existing_partition(0i32, (*ptr_to_globals).g_partitions as libc::c_uint)
  } else {
    i = get_partition(0i32, (*ptr_to_globals).g_partitions as libc::c_uint) as libc::c_int
  }
  if i == -1i32 {
    return;
  }
  p = (*ptr_to_globals).ptes[i as usize].part_table;
  sys = get_sysid(i);
  origsys = sys;
  /* if changing types T to 0 is allowed, then
  the reverse change must be allowed, too */
  if sys == 0 && 0i32 == 0 && 0i32 == 0 && get_nr_sects(p) == 0 {
    printf(
      b"Partition %u does not exist yet!\n\x00" as *const u8 as *const libc::c_char,
      i + 1i32,
    );
    return;
  }
  loop {
    sys = read_hex(get_sys_types());
    if sys == 0 && 0i32 == 0 && 0i32 == 0 {
      puts(b"Type 0 means free space to many systems\n(but not to Linux). Having partitions of\ntype 0 is probably unwise.\x00"
                     as *const u8 as *const libc::c_char);
      /* break; */
    }
    if 0i32 == 0 && 0i32 == 0 {
      if (sys == 0x5i32 || sys == 0xfi32 || sys == 0x85i32) as libc::c_int
        != ((*p).sys_ind as libc::c_int == 0x5i32
          || (*p).sys_ind as libc::c_int == 0xfi32
          || (*p).sys_ind as libc::c_int == 0x85i32) as libc::c_int
      {
        puts(
          b"You cannot change a partition into an extended one or vice versa\x00" as *const u8
            as *const libc::c_char,
        );
        break;
      }
    }
    if !(sys < 256i32) {
      continue;
    }
    if sys == origsys {
      break;
    }
    (*p).sys_ind = sys as libc::c_uchar;
    printf(
      b"Changed system type of partition %u to %x (%s)\n\x00" as *const u8 as *const libc::c_char,
      i + 1i32,
      sys,
      partition_type(sys as libc::c_uchar),
    );
    (*ptr_to_globals).ptes[i as usize].changed = 1i32 as libc::c_char;
    break;
  }
}
/* FEATURE_FDISK_WRITABLE */
/* check_consistency() and linear2chs() added Sat Mar 6 12:28:16 1993,
 * faith@cs.unc.edu, based on code fragments from pfdisk by Gordon W. Ross,
 * Jan.  1990 (version 1.2.1 by Gordon W. Ross Aug. 1990; Modified by S.
 * Lubkin Oct.  1991). */
unsafe extern "C" fn linear2chs(
  mut ls: libc::c_uint,
  mut c: *mut libc::c_uint,
  mut h: *mut libc::c_uint,
  mut s: *mut libc::c_uint,
) {
  let mut spc: libc::c_int = (*ptr_to_globals)
    .g_heads
    .wrapping_mul((*ptr_to_globals).g_sectors) as libc::c_int;
  *c = ls.wrapping_div(spc as libc::c_uint);
  ls = ls.wrapping_rem(spc as libc::c_uint);
  *h = ls.wrapping_div((*ptr_to_globals).g_sectors);
  *s = ls
    .wrapping_rem((*ptr_to_globals).g_sectors)
    .wrapping_add(1i32 as libc::c_uint);
  /* sectors count from 1 */
}
unsafe extern "C" fn check_consistency(mut p: *const partition, mut partition: libc::c_int) {
  let mut pbc: libc::c_uint = 0; /* physical beginning c, h, s */
  let mut pbh: libc::c_uint = 0; /* physical ending c, h, s */
  let mut pbs: libc::c_uint = 0; /* logical beginning c, h, s */
  let mut pec: libc::c_uint = 0; /* logical ending c, h, s */
  let mut peh: libc::c_uint = 0; /* do not check extended partitions */
  let mut pes: libc::c_uint = 0;
  let mut lbc: libc::c_uint = 0;
  let mut lbh: libc::c_uint = 0;
  let mut lbs: libc::c_uint = 0;
  let mut lec: libc::c_uint = 0;
  let mut leh: libc::c_uint = 0;
  let mut les: libc::c_uint = 0;
  if (*ptr_to_globals).g_heads == 0 || (*ptr_to_globals).g_sectors == 0 || partition >= 4i32 {
    return;
  }
  /* physical beginning c, h, s */
  pbc = ((*p).cyl as libc::c_int | ((*p).sector as libc::c_int & 0xc0i32) << 2i32) as libc::c_uint;
  pbh = (*p).head as libc::c_uint;
  pbs = ((*p).sector as libc::c_int & 0x3fi32) as libc::c_uint;
  /* physical ending c, h, s */
  pec = ((*p).end_cyl as libc::c_int | ((*p).end_sector as libc::c_int & 0xc0i32) << 2i32)
    as libc::c_uint;
  peh = (*p).end_head as libc::c_uint;
  pes = ((*p).end_sector as libc::c_int & 0x3fi32) as libc::c_uint;
  /* compute logical beginning (c, h, s) */
  linear2chs(get_start_sect(p), &mut lbc, &mut lbh, &mut lbs);
  /* compute logical ending (c, h, s) */
  linear2chs(
    get_start_sect(p)
      .wrapping_add(get_nr_sects(p))
      .wrapping_sub(1i32 as libc::c_uint),
    &mut lec,
    &mut leh,
    &mut les,
  );
  /* Same physical / logical beginning? */
  if (*ptr_to_globals).g_cylinders <= 1024i32 as libc::c_uint
    && (pbc != lbc || pbh != lbh || pbs != lbs)
  {
    printf(
      b"Partition %u has different physical/logical start (non-Linux?):\n\x00" as *const u8
        as *const libc::c_char,
      partition + 1i32,
    );
    printf(
      b"     phys=(%u,%u,%u) \x00" as *const u8 as *const libc::c_char,
      pbc,
      pbh,
      pbs,
    );
    printf(
      b"logical=(%u,%u,%u)\n\x00" as *const u8 as *const libc::c_char,
      lbc,
      lbh,
      lbs,
    );
  }
  /* Same physical / logical ending? */
  if (*ptr_to_globals).g_cylinders <= 1024i32 as libc::c_uint
    && (pec != lec || peh != leh || pes != les)
  {
    printf(
      b"Partition %u has different physical/logical end:\n\x00" as *const u8 as *const libc::c_char,
      partition + 1i32,
    ); /* fdisk util-linux 2.28 does this */
    printf(
      b"     phys=(%u,%u,%u) \x00" as *const u8 as *const libc::c_char,
      pec,
      peh,
      pes,
    );
    printf(
      b"logical=(%u,%u,%u)\n\x00" as *const u8 as *const libc::c_char,
      lec,
      leh,
      les,
    );
  };
}
unsafe extern "C" fn list_disk_geometry() {
  let mut xbytes: ullong = (*ptr_to_globals)
    .total_number_of_sectors
    .wrapping_div((1024i32 * 1024i32 / 512i32) as libc::c_uint)
    as ullong;
  let mut x: libc::c_char = 'M' as i32 as libc::c_char;
  if xbytes >= 10000i32 as libc::c_ulonglong {
    xbytes =
      (xbytes as libc::c_ulonglong).wrapping_add(512i32 as libc::c_ulonglong) as ullong as ullong;
    xbytes =
      (xbytes as libc::c_ulonglong).wrapping_div(1024i32 as libc::c_ulonglong) as ullong as ullong;
    x = 'G' as i32 as libc::c_char
  }
  printf(b"Disk %s: %llu %cB, %llu bytes, %u sectors\n%u cylinders, %u heads, %u sectors/track\nUnits: %ss of %u * %u = %u bytes\n\n\x00"
               as *const u8 as *const libc::c_char,
           (*ptr_to_globals).disk_device, xbytes, x as libc::c_int,
           ((*ptr_to_globals).total_number_of_sectors as
                ullong).wrapping_mul(512i32 as libc::c_ulonglong),
           (*ptr_to_globals).total_number_of_sectors,
           (*ptr_to_globals).g_cylinders, (*ptr_to_globals).g_heads,
           (*ptr_to_globals).g_sectors, str_units(),
           (*ptr_to_globals).units_per_sector, (*ptr_to_globals).sector_size,
           (*ptr_to_globals).units_per_sector.wrapping_mul((*ptr_to_globals).sector_size));
}
/*
 * Check whether partition entries are ordered by their starting positions.
 * Return 0 if OK. Return i if partition i should have been earlier.
 * Two separate checks: primary and logical partitions.
 */
unsafe extern "C" fn wrong_p_order(mut prev: *mut libc::c_int) -> libc::c_int {
  let mut pe: *const pte = 0 as *const pte;
  let mut p: *const partition = 0 as *const partition;
  let mut last_p_start_pos: sector_t = 0i32 as sector_t;
  let mut p_start_pos: sector_t = 0;
  let mut i: libc::c_uint = 0;
  let mut last_i: libc::c_uint = 0i32 as libc::c_uint;
  i = 0i32 as libc::c_uint;
  while i < (*ptr_to_globals).g_partitions as libc::c_uint {
    if i == 4i32 as libc::c_uint {
      last_i = 4i32 as libc::c_uint;
      last_p_start_pos = 0i32 as sector_t
    }
    pe = &mut *(*ptr_to_globals).ptes.as_mut_ptr().offset(i as isize) as *mut pte;
    p = (*pe).part_table;
    if (*p).sys_ind != 0 {
      p_start_pos = get_partition_start_from_dev_start(pe);
      if last_p_start_pos > p_start_pos {
        if !prev.is_null() {
          *prev = last_i as libc::c_int
        }
        return i as libc::c_int;
      }
      last_p_start_pos = p_start_pos;
      last_i = i
    }
    i = i.wrapping_add(1)
  }
  return 0i32;
}
/*
 * Fix the chain of logicals.
 * extended_offset is unchanged, the set of sectors used is unchanged
 * The chain is sorted so that sectors increase, and so that
 * starting sectors increase.
 *
 * After this it may still be that cfdisk doesnt like the table.
 * (This is because cfdisk considers expanded parts, from link to
 * end of partition, and these may still overlap.)
 * Now
 *   sfdisk /dev/hda > ohda; sfdisk /dev/hda < ohda
 * may help.
 */
unsafe extern "C" fn fix_chain_of_logicals() {
  let mut j: libc::c_int = 0;
  let mut oj: libc::c_int = 0;
  let mut ojj: libc::c_int = 0;
  let mut sj: libc::c_int = 0;
  let mut sjj: libc::c_int = 0;
  let mut pj: *mut partition = 0 as *mut partition;
  let mut pjj: *mut partition = 0 as *mut partition;
  let mut tmp: partition = partition {
    boot_ind: 0,
    head: 0,
    sector: 0,
    cyl: 0,
    sys_ind: 0,
    end_head: 0,
    end_sector: 0,
    end_cyl: 0,
    start4: [0; 4],
    size4: [0; 4],
  };
  's_8: loop
  /* Stage 1: sort sectors but leave sector of part 4 */
  /* (Its sector is the global extended_offset.) */
  {
    j = 5i32;
    loop {
      if !(j < (*ptr_to_globals).g_partitions - 1i32) {
        break 's_8;
      }
      oj = (*ptr_to_globals).ptes[j as usize].offset_from_dev_start as libc::c_int;
      ojj = (*ptr_to_globals).ptes[(j + 1i32) as usize].offset_from_dev_start as libc::c_int;
      if oj > ojj {
        (*ptr_to_globals).ptes[j as usize].offset_from_dev_start = ojj as sector_t;
        (*ptr_to_globals).ptes[(j + 1i32) as usize].offset_from_dev_start = oj as sector_t;
        pj = (*ptr_to_globals).ptes[j as usize].part_table;
        set_start_sect(
          pj,
          get_start_sect(pj)
            .wrapping_add(oj as libc::c_uint)
            .wrapping_sub(ojj as libc::c_uint),
        );
        pjj = (*ptr_to_globals).ptes[(j + 1i32) as usize].part_table;
        set_start_sect(
          pjj,
          get_start_sect(pjj)
            .wrapping_add(ojj as libc::c_uint)
            .wrapping_sub(oj as libc::c_uint),
        );
        set_start_sect(
          (*ptr_to_globals).ptes[(j - 1i32) as usize].ext_pointer,
          (ojj as libc::c_uint).wrapping_sub((*ptr_to_globals).extended_offset),
        );
        set_start_sect(
          (*ptr_to_globals).ptes[j as usize].ext_pointer,
          (oj as libc::c_uint).wrapping_sub((*ptr_to_globals).extended_offset),
        );
        break;
      } else {
        j += 1
      }
    }
  }
  's_74: loop
  /* Stage 2: sort starting sectors */
  {
    j = 4i32;
    loop {
      if !(j < (*ptr_to_globals).g_partitions - 1i32) {
        break 's_74;
      }
      pj = (*ptr_to_globals).ptes[j as usize].part_table;
      pjj = (*ptr_to_globals).ptes[(j + 1i32) as usize].part_table;
      sj = get_start_sect(pj) as libc::c_int;
      sjj = get_start_sect(pjj) as libc::c_int;
      oj = (*ptr_to_globals).ptes[j as usize].offset_from_dev_start as libc::c_int;
      ojj = (*ptr_to_globals).ptes[(j + 1i32) as usize].offset_from_dev_start as libc::c_int;
      if oj + sj > ojj + sjj {
        tmp = *pj;
        *pj = *pjj;
        *pjj = tmp;
        set_start_sect(pj, (ojj + sjj - oj) as libc::c_uint);
        set_start_sect(pjj, (oj + sj - ojj) as libc::c_uint);
        break;
      } else {
        j += 1
      }
    }
  }
  /* Probably something was changed */
  j = 4i32;
  while j < (*ptr_to_globals).g_partitions {
    (*ptr_to_globals).ptes[j as usize].changed = 1i32 as libc::c_char;
    j += 1
  }
}
unsafe extern "C" fn fix_partition_table_order() {
  let mut pei: *mut pte = 0 as *mut pte;
  let mut pek: *mut pte = 0 as *mut pte;
  let mut i: libc::c_int = 0;
  let mut k: libc::c_int = 0;
  if wrong_p_order(0 as *mut libc::c_int) == 0 {
    puts(b"Ordering is already correct\n\x00" as *const u8 as *const libc::c_char);
    return;
  }
  loop {
    i = wrong_p_order(&mut k);
    if !(i != 0i32 && i < 4i32) {
      break;
    }
    /* partition i should have come earlier, move it */
    /* We have to move data in the MBR */
    let mut pi: *mut partition = 0 as *mut partition;
    let mut pk: *mut partition = 0 as *mut partition;
    let mut pe: *mut partition = 0 as *mut partition;
    let mut pbuf: partition = partition {
      boot_ind: 0,
      head: 0,
      sector: 0,
      cyl: 0,
      sys_ind: 0,
      end_head: 0,
      end_sector: 0,
      end_cyl: 0,
      start4: [0; 4],
      size4: [0; 4],
    };
    pei = &mut *(*ptr_to_globals).ptes.as_mut_ptr().offset(i as isize) as *mut pte;
    pek = &mut *(*ptr_to_globals).ptes.as_mut_ptr().offset(k as isize) as *mut pte;
    pe = (*pei).ext_pointer;
    (*pei).ext_pointer = (*pek).ext_pointer;
    (*pek).ext_pointer = pe;
    pi = (*pei).part_table;
    pk = (*pek).part_table;
    memmove(
      &mut pbuf as *mut partition as *mut libc::c_void,
      pi as *const libc::c_void,
      ::std::mem::size_of::<partition>() as libc::c_ulong,
    );
    memmove(
      pi as *mut libc::c_void,
      pk as *const libc::c_void,
      ::std::mem::size_of::<partition>() as libc::c_ulong,
    );
    memmove(
      pk as *mut libc::c_void,
      &mut pbuf as *mut partition as *const libc::c_void,
      ::std::mem::size_of::<partition>() as libc::c_ulong,
    );
    (*pek).changed = 1i32 as libc::c_char;
    (*pei).changed = (*pek).changed
  }
  if i != 0 {
    fix_chain_of_logicals();
  }
  puts(b"Done\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn chs_string11(
  mut cyl: libc::c_uint,
  mut head: libc::c_uint,
  mut sect: libc::c_uint,
) -> *const libc::c_char {
  let mut buf: *mut libc::c_char = auto_string(xzalloc(
    (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
      .wrapping_mul(3i32 as libc::c_ulong)
      .wrapping_mul(3i32 as libc::c_ulong),
  ) as *mut libc::c_char);
  sprintf(
    buf,
    b"%u,%u,%u\x00" as *const u8 as *const libc::c_char,
    cyl | (sect & 0xc0i32 as libc::c_uint) << 2i32,
    head,
    sect & 0x3fi32 as libc::c_uint,
  );
  return buf;
}
unsafe extern "C" fn list_table(mut _xtra: libc::c_int) {
  let mut i: libc::c_int = 0;
  let mut w: libc::c_int = 0;
  list_disk_geometry();
  /* Heuristic: we list partition 3 of /dev/foo as /dev/foo3,
   * but if the device name ends in a digit, say /dev/foo1,
   * then the partition is called /dev/foo1p3.
   */
  w = strlen((*ptr_to_globals).disk_device) as libc::c_int;
  if w != 0
    && (*(*ptr_to_globals).disk_device.offset((w - 1i32) as isize) as libc::c_int - '0' as i32)
      as libc::c_uchar as libc::c_int
      <= 9i32
  {
    w += 1
  }
  if w < 7i32 {
    w = 7i32
  }
  printf(
    b"%-*s Boot StartCHS    EndCHS        StartLBA     EndLBA    Sectors  Size Id Type\n\x00"
      as *const u8 as *const libc::c_char,
    w - 1i32,
    b"Device\x00" as *const u8 as *const libc::c_char,
  );
  i = 0i32;
  while i < (*ptr_to_globals).g_partitions {
    let mut p: *const partition = 0 as *const partition;
    let mut pe: *const pte =
      &mut *(*ptr_to_globals).ptes.as_mut_ptr().offset(i as isize) as *mut pte;
    let mut boot4: [libc::c_char; 4] = [0; 4];
    let mut numstr6: [libc::c_char; 6] = [0; 6];
    let mut start_sect: sector_t = 0;
    let mut end_sect: sector_t = 0;
    let mut nr_sects: sector_t = 0;
    p = (*pe).part_table;
    if !(p.is_null() || is_cleared_partition(p) != 0) {
      sprintf(
        boot4.as_mut_ptr(),
        b"%02x\x00" as *const u8 as *const libc::c_char,
        (*p).boot_ind as libc::c_int,
      );
      if (*p).boot_ind as libc::c_int & 0x7fi32 == 0i32 {
        /* 0x80 shown as '*', 0x00 is ' ' */
        boot4[0] = if (*p).boot_ind as libc::c_int != 0 {
          '*' as i32
        } else {
          ' ' as i32
        } as libc::c_char;
        boot4[1] = ' ' as i32 as libc::c_char
      }
      start_sect = get_partition_start_from_dev_start(pe);
      end_sect = start_sect;
      nr_sects = get_nr_sects(p);
      if nr_sects != 0i32 as libc::c_uint {
        end_sect = (end_sect as libc::c_uint)
          .wrapping_add(nr_sects.wrapping_sub(1i32 as libc::c_uint)) as sector_t
          as sector_t
      }
      *smart_ulltoa5(
        (nr_sects as ullong).wrapping_mul((*ptr_to_globals).sector_size as libc::c_ulonglong),
        numstr6.as_mut_ptr(),
        b" KMGTPEZY\x00" as *const u8 as *const libc::c_char,
      )
      .offset(0) = '\u{0}' as i32 as libc::c_char;
      //      Boot StartCHS    EndCHS        StartLBA     EndLBA    Sectors  Size Id Type
      printf(
        b"%s%s %-11s %-11s %10u %10u %10u %s %2x %s\n\x00" as *const u8 as *const libc::c_char,
        partname((*ptr_to_globals).disk_device, i + 1i32, w + 2i32),
        boot4.as_mut_ptr(),
        chs_string11(
          (*p).cyl as libc::c_uint,
          (*p).head as libc::c_uint,
          (*p).sector as libc::c_uint,
        ),
        chs_string11(
          (*p).end_cyl as libc::c_uint,
          (*p).end_head as libc::c_uint,
          (*p).end_sector as libc::c_uint,
        ),
        start_sect,
        end_sect,
        nr_sects,
        numstr6.as_mut_ptr(),
        (*p).sys_ind as libc::c_int,
        partition_type((*p).sys_ind),
      );
      check_consistency(p, i);
    }
    i += 1
  }
  /* Is partition table in disk order? It need not be, but... */
  /* partition table entries are not checked for correct order
   * if this is a sgi, sun or aix labeled disk... */
  if LABEL_DOS as libc::c_int == (*ptr_to_globals).current_label_type as libc::c_int
    && wrong_p_order(0 as *mut libc::c_int) != 0
  {
    /* FIXME */
    puts(
      b"\nPartition table entries are not in disk order\x00" as *const u8 as *const libc::c_char,
    ); /* 48 does not suffice in Japanese */
  };
}
unsafe extern "C" fn x_list_table(mut extend: libc::c_int) {
  let mut pe: *const pte = 0 as *const pte;
  let mut p: *const partition = 0 as *const partition;
  let mut i: libc::c_int = 0;
  printf(
    b"\nDisk %s: %u heads, %u sectors, %u cylinders\n\n\x00" as *const u8 as *const libc::c_char,
    (*ptr_to_globals).disk_device,
    (*ptr_to_globals).g_heads,
    (*ptr_to_globals).g_sectors,
    (*ptr_to_globals).g_cylinders,
  );
  puts(
    b"Nr AF  Hd Sec  Cyl  Hd Sec  Cyl      Start       Size ID\x00" as *const u8
      as *const libc::c_char,
  );
  i = 0i32;
  while i < (*ptr_to_globals).g_partitions {
    pe = &mut *(*ptr_to_globals).ptes.as_mut_ptr().offset(i as isize) as *mut pte;
    p = if extend != 0 {
      (*pe).ext_pointer
    } else {
      (*pe).part_table
    };
    if !p.is_null() {
      printf(
        b"%2u %02x%4u%4u%5u%4u%4u%5u%11u%11u %02x\n\x00" as *const u8 as *const libc::c_char,
        i + 1i32,
        (*p).boot_ind as libc::c_int,
        (*p).head as libc::c_int,
        (*p).sector as libc::c_int & 0x3fi32,
        (*p).cyl as libc::c_int | ((*p).sector as libc::c_int & 0xc0i32) << 2i32,
        (*p).end_head as libc::c_int,
        (*p).end_sector as libc::c_int & 0x3fi32,
        (*p).end_cyl as libc::c_int | ((*p).end_sector as libc::c_int & 0xc0i32) << 2i32,
        get_start_sect(p),
        get_nr_sects(p),
        (*p).sys_ind as libc::c_int,
      );
      if (*p).sys_ind != 0 {
        check_consistency(p, i);
      }
    }
    i += 1
  }
}
unsafe extern "C" fn fill_bounds(mut first: *mut sector_t, mut last: *mut sector_t) {
  let mut i: libc::c_uint = 0;
  let mut pe: *const pte = &mut *(*ptr_to_globals).ptes.as_mut_ptr().offset(0) as *mut pte;
  let mut p: *const partition = 0 as *const partition;
  i = 0i32 as libc::c_uint;
  while i < (*ptr_to_globals).g_partitions as libc::c_uint {
    p = (*pe).part_table;
    if (*p).sys_ind == 0
      || ((*p).sys_ind as libc::c_int == 0x5i32
        || (*p).sys_ind as libc::c_int == 0xfi32
        || (*p).sys_ind as libc::c_int == 0x85i32)
    {
      *first.offset(i as isize) = 0xffffffffu32;
      *last.offset(i as isize) = 0i32 as sector_t
    } else {
      *first.offset(i as isize) = get_partition_start_from_dev_start(pe);
      *last.offset(i as isize) = (*first.offset(i as isize))
        .wrapping_add(get_nr_sects(p))
        .wrapping_sub(1i32 as libc::c_uint)
    }
    pe = pe.offset(1);
    i = i.wrapping_add(1)
  }
}
unsafe extern "C" fn check(
  mut n: libc::c_int,
  mut h: libc::c_uint,
  mut s: libc::c_uint,
  mut c: libc::c_uint,
  mut start: sector_t,
) {
  let mut total: sector_t = 0;
  let mut real_s: sector_t = 0;
  let mut real_c: sector_t = 0;
  real_s = (s & 0x3fi32 as libc::c_uint).wrapping_sub(1i32 as libc::c_uint);
  real_c = c | (s & 0xc0i32 as libc::c_uint) << 2i32;
  total = real_c
    .wrapping_mul((*ptr_to_globals).g_sectors)
    .wrapping_add(real_s)
    .wrapping_mul((*ptr_to_globals).g_heads)
    .wrapping_add(h);
  if total == 0 {
    printf(
      b"Partition %u contains sector 0\n\x00" as *const u8 as *const libc::c_char,
      n,
    );
  }
  if h >= (*ptr_to_globals).g_heads {
    printf(
      b"Partition %u: head %u greater than maximum %u\n\x00" as *const u8 as *const libc::c_char,
      n,
      h.wrapping_add(1i32 as libc::c_uint),
      (*ptr_to_globals).g_heads,
    );
  }
  if real_s >= (*ptr_to_globals).g_sectors {
    printf(
      b"Partition %u: sector %u greater than maximum %u\n\x00" as *const u8 as *const libc::c_char,
      n,
      s,
      (*ptr_to_globals).g_sectors,
    );
  }
  if real_c >= (*ptr_to_globals).g_cylinders {
    printf(
      b"Partition %u: cylinder %u greater than maximum %u\n\x00" as *const u8
        as *const libc::c_char,
      n,
      real_c.wrapping_add(1i32 as libc::c_uint),
      (*ptr_to_globals).g_cylinders,
    );
  }
  if (*ptr_to_globals).g_cylinders <= 1024i32 as libc::c_uint && start != total {
    printf(
      b"Partition %u: previous sectors %u disagrees with total %u\n\x00" as *const u8
        as *const libc::c_char,
      n,
      start,
      total,
    );
  };
}
unsafe extern "C" fn verify() {
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  let mut total: sector_t = 1i32 as sector_t;
  let mut chs_size: sector_t = 0;
  let vla = (*ptr_to_globals).g_partitions as usize;
  let mut first: Vec<sector_t> = ::std::vec::from_elem(0, vla);
  let vla_0 = (*ptr_to_globals).g_partitions as usize;
  let mut last: Vec<sector_t> = ::std::vec::from_elem(0, vla_0);
  let mut p: *mut partition = 0 as *mut partition;
  if warn_geometry() != 0 {
    return;
  }
  fill_bounds(first.as_mut_ptr(), last.as_mut_ptr());
  i = 0i32;
  while i < (*ptr_to_globals).g_partitions {
    let mut pe: *mut pte = &mut *(*ptr_to_globals).ptes.as_mut_ptr().offset(i as isize) as *mut pte;
    p = (*pe).part_table;
    if (*p).sys_ind as libc::c_int != 0
      && !((*p).sys_ind as libc::c_int == 0x5i32
        || (*p).sys_ind as libc::c_int == 0xfi32
        || (*p).sys_ind as libc::c_int == 0x85i32)
    {
      check_consistency(p, i);
      if get_partition_start_from_dev_start(pe) < *first.as_mut_ptr().offset(i as isize) {
        printf(
          b"Warning: bad start-of-data in partition %u\n\x00" as *const u8 as *const libc::c_char,
          i + 1i32,
        );
      }
      check(
        i + 1i32,
        (*p).end_head as libc::c_uint,
        (*p).end_sector as libc::c_uint,
        (*p).end_cyl as libc::c_uint,
        *last.as_mut_ptr().offset(i as isize),
      );
      total = (total as libc::c_uint).wrapping_add(
        (*last.as_mut_ptr().offset(i as isize))
          .wrapping_add(1i32 as libc::c_uint)
          .wrapping_sub(*first.as_mut_ptr().offset(i as isize)),
      ) as sector_t as sector_t;
      j = 0i32;
      while j < i {
        if *first.as_mut_ptr().offset(i as isize) >= *first.as_mut_ptr().offset(j as isize)
          && *first.as_mut_ptr().offset(i as isize) <= *last.as_mut_ptr().offset(j as isize)
          || *last.as_mut_ptr().offset(i as isize) <= *last.as_mut_ptr().offset(j as isize)
            && *last.as_mut_ptr().offset(i as isize) >= *first.as_mut_ptr().offset(j as isize)
        {
          printf(
            b"Warning: partition %u overlaps partition %u\n\x00" as *const u8
              as *const libc::c_char,
            j + 1i32,
            i + 1i32,
          );
          total = (total as libc::c_uint).wrapping_add(
            if *first.as_mut_ptr().offset(i as isize) >= *first.as_mut_ptr().offset(j as isize) {
              *first.as_mut_ptr().offset(i as isize)
            } else {
              *first.as_mut_ptr().offset(j as isize)
            },
          ) as sector_t as sector_t;
          total = (total as libc::c_uint).wrapping_sub(
            if *last.as_mut_ptr().offset(i as isize) <= *last.as_mut_ptr().offset(j as isize) {
              *last.as_mut_ptr().offset(i as isize)
            } else {
              *last.as_mut_ptr().offset(j as isize)
            },
          ) as sector_t as sector_t
        }
        j += 1
      }
    }
    i += 1
  }
  if (*ptr_to_globals).extended_offset != 0 {
    let mut pex: *mut pte = &mut *(*ptr_to_globals)
      .ptes
      .as_mut_ptr()
      .offset((*ptr_to_globals).ext_index as isize) as *mut pte;
    let mut e_last: sector_t = get_start_sect((*pex).part_table)
      .wrapping_add(get_nr_sects((*pex).part_table))
      .wrapping_sub(1i32 as libc::c_uint);
    i = 4i32;
    while i < (*ptr_to_globals).g_partitions {
      total = total.wrapping_add(1);
      p = (*ptr_to_globals).ptes[i as usize].part_table;
      if (*p).sys_ind == 0 {
        if i != 4i32 || i + 1i32 < (*ptr_to_globals).g_partitions {
          printf(
            b"Warning: partition %u is empty\n\x00" as *const u8 as *const libc::c_char,
            i + 1i32,
          );
        }
      } else if *first.as_mut_ptr().offset(i as isize) < (*ptr_to_globals).extended_offset
        || *last.as_mut_ptr().offset(i as isize) > e_last
      {
        printf(
          b"Logical partition %u not entirely in partition %u\n\x00" as *const u8
            as *const libc::c_char,
          i + 1i32,
          (*ptr_to_globals).ext_index + 1i32,
        );
      }
      i += 1
    }
  }
  chs_size = (*ptr_to_globals)
    .g_heads
    .wrapping_mul((*ptr_to_globals).g_sectors)
    .wrapping_mul((*ptr_to_globals).g_cylinders);
  if total > chs_size {
    printf(
      b"Total allocated sectors %u greater than CHS size %u\n\x00" as *const u8
        as *const libc::c_char,
      total,
      chs_size,
    );
  } else {
    total = chs_size.wrapping_sub(total);
    if total != 0i32 as libc::c_uint {
      printf(
        b"%u unallocated sectors\n\x00" as *const u8 as *const libc::c_char,
        total,
      );
    }
  };
}
unsafe extern "C" fn add_partition(mut n: libc::c_int, mut sys: libc::c_int) {
  let mut mesg: [libc::c_char; 256] = [0; 256];
  let mut i: libc::c_int = 0;
  let mut num_read: libc::c_int = 0i32;
  let mut p: *mut partition = (*ptr_to_globals).ptes[n as usize].part_table;
  let mut q: *mut partition =
    (*ptr_to_globals).ptes[(*ptr_to_globals).ext_index as usize].part_table;
  let mut limit: sector_t = 0;
  let mut temp: sector_t = 0;
  let mut start: sector_t = 0;
  let mut stop: sector_t = 0i32 as sector_t;
  let vla = (*ptr_to_globals).g_partitions as usize;
  let mut first: Vec<sector_t> = ::std::vec::from_elem(0, vla);
  let vla_0 = (*ptr_to_globals).g_partitions as usize;
  let mut last: Vec<sector_t> = ::std::vec::from_elem(0, vla_0);
  if !p.is_null() && (*p).sys_ind as libc::c_int != 0 {
    printf(msg_part_already_defined.as_ptr(), n + 1i32);
    return;
  }
  fill_bounds(first.as_mut_ptr(), last.as_mut_ptr());
  if n < 4i32 {
    start = (*ptr_to_globals).sector_offset;
    if (*ptr_to_globals).display_in_cyl_units as libc::c_int != 0
      || (*ptr_to_globals).total_number_of_sectors == 0
    {
      limit = (*ptr_to_globals)
        .g_heads
        .wrapping_mul((*ptr_to_globals).g_sectors)
        .wrapping_mul((*ptr_to_globals).g_cylinders)
        .wrapping_sub(1i32 as libc::c_uint)
    } else {
      limit = (*ptr_to_globals)
        .total_number_of_sectors
        .wrapping_sub(1i32 as libc::c_uint)
    }
    if (*ptr_to_globals).extended_offset != 0 {
      *first
        .as_mut_ptr()
        .offset((*ptr_to_globals).ext_index as isize) = (*ptr_to_globals).extended_offset;
      *last
        .as_mut_ptr()
        .offset((*ptr_to_globals).ext_index as isize) = get_start_sect(q)
        .wrapping_add(get_nr_sects(q))
        .wrapping_sub(1i32 as libc::c_uint)
    }
  } else {
    start = (*ptr_to_globals)
      .extended_offset
      .wrapping_add((*ptr_to_globals).sector_offset);
    limit = get_start_sect(q)
      .wrapping_add(get_nr_sects(q))
      .wrapping_sub(1i32 as libc::c_uint)
  }
  if (*ptr_to_globals).display_in_cyl_units != 0 {
    i = 0i32;
    while i < (*ptr_to_globals).g_partitions {
      *first.as_mut_ptr().offset(i as isize) =
        (if (*ptr_to_globals).display_in_cyl_units as libc::c_int != 0 {
          (*first.as_mut_ptr().offset(i as isize))
            .wrapping_div((*ptr_to_globals).units_per_sector)
            .wrapping_add(1i32 as libc::c_uint)
        } else {
          *first.as_mut_ptr().offset(i as isize)
        })
        .wrapping_sub(1i32 as libc::c_uint)
        .wrapping_mul((*ptr_to_globals).units_per_sector);
      i += 1
    }
  }
  snprintf(
    mesg.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    b"First %s\x00" as *const u8 as *const libc::c_char,
    str_units(),
  );
  loop {
    temp = start;
    i = 0i32;
    while i < (*ptr_to_globals).g_partitions {
      let mut lastplusoff: libc::c_int = 0;
      if start == (*ptr_to_globals).ptes[i as usize].offset_from_dev_start {
        start = (start as libc::c_uint).wrapping_add((*ptr_to_globals).sector_offset) as sector_t
          as sector_t
      }
      lastplusoff = (*last.as_mut_ptr().offset(i as isize)).wrapping_add(if n < 4i32 {
        0i32 as libc::c_uint
      } else {
        (*ptr_to_globals).sector_offset
      }) as libc::c_int;
      if start >= *first.as_mut_ptr().offset(i as isize) && start <= lastplusoff as libc::c_uint {
        start = (lastplusoff + 1i32) as sector_t
      }
      i += 1
    }
    if start > limit {
      break;
    }
    if start >= temp.wrapping_add((*ptr_to_globals).units_per_sector) && num_read != 0 {
      printf(
        b"Sector %u is already allocated\n\x00" as *const u8 as *const libc::c_char,
        temp,
      );
      temp = start;
      num_read = 0i32
    }
    if num_read == 0 && start == temp {
      let mut saved_start: sector_t = 0;
      saved_start = start;
      start = read_int(
        if (*ptr_to_globals).display_in_cyl_units as libc::c_int != 0 {
          saved_start
            .wrapping_div((*ptr_to_globals).units_per_sector)
            .wrapping_add(1i32 as libc::c_uint)
        } else {
          saved_start
        },
        if (*ptr_to_globals).display_in_cyl_units as libc::c_int != 0 {
          saved_start
            .wrapping_div((*ptr_to_globals).units_per_sector)
            .wrapping_add(1i32 as libc::c_uint)
        } else {
          saved_start
        },
        if (*ptr_to_globals).display_in_cyl_units as libc::c_int != 0 {
          limit
            .wrapping_div((*ptr_to_globals).units_per_sector)
            .wrapping_add(1i32 as libc::c_uint)
        } else {
          limit
        },
        0i32 as sector_t,
        mesg.as_mut_ptr(),
      );
      if (*ptr_to_globals).display_in_cyl_units != 0 {
        start = start
          .wrapping_sub(1i32 as libc::c_uint)
          .wrapping_mul((*ptr_to_globals).units_per_sector);
        if start < saved_start {
          start = saved_start
        }
      }
      num_read = 1i32
    }
    if !(start != temp || num_read == 0) {
      break;
    }
  }
  if n > 4i32 {
    /* NOT for fifth partition */
    let mut pe: *mut pte = &mut *(*ptr_to_globals).ptes.as_mut_ptr().offset(n as isize) as *mut pte;
    (*pe).offset_from_dev_start = start.wrapping_sub((*ptr_to_globals).sector_offset);
    if (*pe).offset_from_dev_start == (*ptr_to_globals).extended_offset {
      /* must be corrected */
      (*pe).offset_from_dev_start = (*pe).offset_from_dev_start.wrapping_add(1); /* lowercase */
      if (*ptr_to_globals).sector_offset == 1i32 as libc::c_uint {
        start = start.wrapping_add(1)
      }
    }
  }
  i = 0i32;
  while i < (*ptr_to_globals).g_partitions {
    let mut pe_0: *mut pte =
      &mut *(*ptr_to_globals).ptes.as_mut_ptr().offset(i as isize) as *mut pte;
    if start < (*pe_0).offset_from_dev_start && limit >= (*pe_0).offset_from_dev_start {
      limit = (*pe_0)
        .offset_from_dev_start
        .wrapping_sub(1i32 as libc::c_uint)
    }
    if start < *first.as_mut_ptr().offset(i as isize)
      && limit >= *first.as_mut_ptr().offset(i as isize)
    {
      limit = (*first.as_mut_ptr().offset(i as isize)).wrapping_sub(1i32 as libc::c_uint)
    }
    i += 1
  }
  if start > limit {
    puts(b"No free sectors available\x00" as *const u8 as *const libc::c_char);
    if n > 4i32 {
      (*ptr_to_globals).g_partitions -= 1
    }
    return;
  }
  if (if (*ptr_to_globals).display_in_cyl_units as libc::c_int != 0 {
    start
      .wrapping_div((*ptr_to_globals).units_per_sector)
      .wrapping_add(1i32 as libc::c_uint)
  } else {
    start
  }) == (if (*ptr_to_globals).display_in_cyl_units as libc::c_int != 0 {
    limit
      .wrapping_div((*ptr_to_globals).units_per_sector)
      .wrapping_add(1i32 as libc::c_uint)
  } else {
    limit
  }) {
    stop = limit
  } else {
    snprintf(
      mesg.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
      b"Last %s or +size{,K,M,G,T}\x00" as *const u8 as *const libc::c_char,
      str_units(),
    );
    stop = read_int(
      if (*ptr_to_globals).display_in_cyl_units as libc::c_int != 0 {
        start
          .wrapping_div((*ptr_to_globals).units_per_sector)
          .wrapping_add(1i32 as libc::c_uint)
      } else {
        start
      },
      if (*ptr_to_globals).display_in_cyl_units as libc::c_int != 0 {
        limit
          .wrapping_div((*ptr_to_globals).units_per_sector)
          .wrapping_add(1i32 as libc::c_uint)
      } else {
        limit
      },
      if (*ptr_to_globals).display_in_cyl_units as libc::c_int != 0 {
        limit
          .wrapping_div((*ptr_to_globals).units_per_sector)
          .wrapping_add(1i32 as libc::c_uint)
      } else {
        limit
      },
      if (*ptr_to_globals).display_in_cyl_units as libc::c_int != 0 {
        start
          .wrapping_div((*ptr_to_globals).units_per_sector)
          .wrapping_add(1i32 as libc::c_uint)
      } else {
        start
      },
      mesg.as_mut_ptr(),
    );
    if (*ptr_to_globals).display_in_cyl_units != 0 {
      stop = stop
        .wrapping_mul((*ptr_to_globals).units_per_sector)
        .wrapping_sub(1i32 as libc::c_uint);
      if stop > limit {
        stop = limit
      }
    }
  }
  set_partition(n, 0i32, start, stop, sys);
  if n > 4i32 {
    set_partition(
      n - 1i32,
      1i32,
      (*ptr_to_globals).ptes[n as usize].offset_from_dev_start,
      stop,
      0x5i32,
    );
  }
  if sys == 0x5i32 || sys == 0xfi32 || sys == 0x85i32 {
    let mut pe4: *mut pte = &mut *(*ptr_to_globals).ptes.as_mut_ptr().offset(4) as *mut pte;
    let mut pen: *mut pte =
      &mut *(*ptr_to_globals).ptes.as_mut_ptr().offset(n as isize) as *mut pte;
    (*ptr_to_globals).ext_index = n;
    (*pen).ext_pointer = p;
    (*ptr_to_globals).extended_offset = start;
    (*pe4).offset_from_dev_start = (*ptr_to_globals).extended_offset;
    (*pe4).sectorbuffer = xzalloc((*ptr_to_globals).sector_size as size_t) as *mut libc::c_char;
    (*pe4).part_table = (*pe4).sectorbuffer.offset(0x1bei32 as isize).offset(
      (0i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<partition>() as libc::c_ulong)
        as isize,
    ) as *mut partition;
    (*pe4).ext_pointer = (*pe4).part_table.offset(1);
    (*pe4).changed = 1i32 as libc::c_char;
    (*ptr_to_globals).g_partitions = 5i32
  };
}
unsafe extern "C" fn add_logical() {
  if (*ptr_to_globals).g_partitions > 5i32
    || (*(*ptr_to_globals).ptes[4].part_table).sys_ind as libc::c_int != 0
  {
    let mut pe: *mut pte = &mut *(*ptr_to_globals)
      .ptes
      .as_mut_ptr()
      .offset((*ptr_to_globals).g_partitions as isize) as *mut pte;
    (*pe).sectorbuffer = xzalloc((*ptr_to_globals).sector_size as size_t) as *mut libc::c_char;
    (*pe).part_table = (*pe).sectorbuffer.offset(0x1bei32 as isize).offset(
      (0i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<partition>() as libc::c_ulong)
        as isize,
    ) as *mut partition;
    (*pe).ext_pointer = (*pe).part_table.offset(1);
    (*pe).offset_from_dev_start = 0i32 as sector_t;
    (*pe).changed = 1i32 as libc::c_char;
    (*ptr_to_globals).g_partitions += 1
  }
  add_partition((*ptr_to_globals).g_partitions - 1i32, 0x83i32);
}
unsafe extern "C" fn new_partition() {
  let mut i: libc::c_int = 0;
  let mut free_primary: libc::c_int = 0i32;
  if warn_geometry() != 0 {
    return;
  }
  i = 0i32;
  while i < 4i32 {
    free_primary += ((*(*ptr_to_globals).ptes[i as usize].part_table).sys_ind == 0) as libc::c_int;
    i += 1
  }
  if free_primary == 0 && (*ptr_to_globals).g_partitions >= 60i32 {
    puts(
      b"The maximum number of partitions has been created\x00" as *const u8 as *const libc::c_char,
    );
    return;
  }
  if free_primary == 0 {
    if (*ptr_to_globals).extended_offset != 0 {
      add_logical();
    } else {
      puts(
        b"You must delete some partition and add an extended partition first\x00" as *const u8
          as *const libc::c_char,
      );
    }
  } else {
    let mut c: libc::c_char = 0;
    let mut line: [libc::c_char; 80] = [0; 80];
    snprintf(
      line.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong,
      b"Partition type\n   p   primary partition (1-4)\n   %s\n\x00" as *const u8
        as *const libc::c_char,
      if (*ptr_to_globals).extended_offset != 0 {
        b"l   logical (5 or over)\x00" as *const u8 as *const libc::c_char
      } else {
        b"e   extended\x00" as *const u8 as *const libc::c_char
      },
    );
    loop {
      c = read_nonempty(line.as_mut_ptr());
      c = (c as libc::c_int | 0x20i32) as libc::c_char;
      if c as libc::c_int == 'p' as i32 {
        i = get_nonexisting_partition();
        if i >= 0i32 {
          add_partition(i, 0x83i32);
        }
        return;
      }
      if c as libc::c_int == 'l' as i32 && (*ptr_to_globals).extended_offset != 0 {
        add_logical();
        return;
      }
      if c as libc::c_int == 'e' as i32 && (*ptr_to_globals).extended_offset == 0 {
        i = get_nonexisting_partition();
        if i >= 0i32 {
          add_partition(i, 0x5i32);
        }
        return;
      }
      printf(
        b"Invalid partition number for type \'%c\'\n\x00" as *const u8 as *const libc::c_char,
        c as libc::c_int,
      );
    }
  };
}
unsafe extern "C" fn reread_partition_table(mut leave: libc::c_int) {
  let mut i: libc::c_int = 0;
  puts(b"Calling ioctl() to re-read partition table\x00" as *const u8 as *const libc::c_char);
  sync();
  /* Users with slow external USB disks on a 320MHz ARM system (year 2011)
   * report that sleep is needed, otherwise BLKRRPART may fail with -EIO:
   */
  sleep(1i32 as libc::c_uint); /* == pe->offset_from_dev_start + get_start_sect(p) */
  i = ioctl_or_perror(
    dev_fd as libc::c_int,
    0u32 << 0i32 + 8i32 + 8i32 + 14i32
      | (0x12i32 << 0i32 + 8i32) as libc::c_uint
      | (95i32 << 0i32) as libc::c_uint
      | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint,
    0 as *mut libc::c_void,
    b"WARNING: rereading partition table failed, kernel still uses old table\x00" as *const u8
      as *const libc::c_char,
  ); /* does not return */
  if leave != 0 {
    exit((i != 0i32) as libc::c_int);
  };
}
unsafe extern "C" fn write_table() {
  let mut i: libc::c_int = 0;
  if LABEL_DOS as libc::c_int == (*ptr_to_globals).current_label_type as libc::c_int {
    i = 0i32;
    while i < 3i32 {
      if (*ptr_to_globals).ptes[i as usize].changed != 0 {
        (*ptr_to_globals).ptes[3].changed = 1i32 as libc::c_char
      }
      i += 1
    }
    i = 3i32;
    while i < (*ptr_to_globals).g_partitions {
      let mut pe: *mut pte =
        &mut *(*ptr_to_globals).ptes.as_mut_ptr().offset(i as isize) as *mut pte;
      if (*pe).changed != 0 {
        write_part_table_flag((*pe).sectorbuffer);
        write_sector(
          (*pe).offset_from_dev_start,
          (*pe).sectorbuffer as *const libc::c_void,
        );
      }
      i += 1
    }
  }
  puts(b"The partition table has been altered.\x00" as *const u8 as *const libc::c_char);
  reread_partition_table(1i32);
}
unsafe extern "C" fn print_buffer(mut pbuffer: *mut libc::c_char) {
  let mut i: libc::c_int = 0;
  let mut l: libc::c_int = 0;
  i = 0i32;
  l = 0i32;
  while (i as libc::c_uint) < (*ptr_to_globals).sector_size {
    if l == 0i32 {
      printf(b"0x%03X:\x00" as *const u8 as *const libc::c_char, i);
    }
    printf(
      b" %02X\x00" as *const u8 as *const libc::c_char,
      *pbuffer.offset(i as isize) as libc::c_uchar as libc::c_int,
    );
    if l == 16i32 - 1i32 {
      bb_putchar('\n' as i32);
      l = -1i32
    }
    i += 1;
    l += 1
  }
  if l > 0i32 {
    bb_putchar('\n' as i32);
  }
  bb_putchar('\n' as i32);
}
unsafe extern "C" fn print_raw() {
  let mut i: libc::c_int = 0;
  printf(
    b"Device: %s\n\x00" as *const u8 as *const libc::c_char,
    (*ptr_to_globals).disk_device,
  );
  if 0i32 != 0 || 0i32 != 0 {
    print_buffer((*ptr_to_globals).MBRbuffer.as_mut_ptr());
  } else {
    i = 3i32;
    while i < (*ptr_to_globals).g_partitions {
      print_buffer((*ptr_to_globals).ptes[i as usize].sectorbuffer);
      i += 1
    }
  };
}
unsafe extern "C" fn move_begin(mut i: libc::c_uint) {
  let mut pe: *mut pte = &mut *(*ptr_to_globals).ptes.as_mut_ptr().offset(i as isize) as *mut pte;
  let mut p: *mut partition = (*pe).part_table;
  let mut new: sector_t = 0;
  let mut first: sector_t = 0;
  let mut nr_sects: sector_t = 0;
  if warn_geometry() != 0 {
    return;
  }
  nr_sects = get_nr_sects(p);
  if (*p).sys_ind == 0
    || nr_sects == 0
    || ((*p).sys_ind as libc::c_int == 0x5i32
      || (*p).sys_ind as libc::c_int == 0xfi32
      || (*p).sys_ind as libc::c_int == 0x85i32)
  {
    printf(
      b"Partition %u has no data area\n\x00" as *const u8 as *const libc::c_char,
      i.wrapping_add(1i32 as libc::c_uint),
    );
    return;
  }
  first = get_partition_start_from_dev_start(pe);
  new = read_int(
    0i32 as sector_t,
    first,
    first
      .wrapping_add(nr_sects)
      .wrapping_sub(1i32 as libc::c_uint),
    first,
    b"New beginning of data\x00" as *const u8 as *const libc::c_char,
  );
  if new != first {
    let mut new_relative: sector_t = new.wrapping_sub((*pe).offset_from_dev_start);
    nr_sects = (nr_sects as libc::c_uint).wrapping_add(get_start_sect(p).wrapping_sub(new_relative))
      as sector_t as sector_t;
    set_start_sect(p, new_relative);
    set_nr_sects(p, nr_sects);
    read_nonempty(b"Recalculate C/H/S values? (Y/N): \x00" as *const u8 as *const libc::c_char);
    if *(*ptr_to_globals).line_ptr.offset(0) as libc::c_int | 0x20i32 == 'y' as i32 {
      set_hsc_start_end(
        p,
        new,
        new
          .wrapping_add(nr_sects)
          .wrapping_sub(1i32 as libc::c_uint),
      );
    }
    (*pe).changed = 1i32 as libc::c_char
  };
}
unsafe extern "C" fn xselect() {
  let mut c: libc::c_char = 0;
  loop {
    bb_putchar('\n' as i32);
    c = (0x20i32
      | read_nonempty(b"Expert command (m for help): \x00" as *const u8 as *const libc::c_char)
        as libc::c_int) as libc::c_char;
    match c as libc::c_int {
      98 => {
        if LABEL_DOS as libc::c_int == (*ptr_to_globals).current_label_type as libc::c_int {
          move_begin(get_partition(
            0i32,
            (*ptr_to_globals).g_partitions as libc::c_uint,
          ));
        }
      }
      99 => {
        (*ptr_to_globals).g_cylinders = read_int(
          1i32 as sector_t,
          (*ptr_to_globals).g_cylinders,
          1048576i32 as sector_t,
          0i32 as sector_t,
          b"Number of cylinders\x00" as *const u8 as *const libc::c_char,
        );
        (*ptr_to_globals).user_cylinders = (*ptr_to_globals).g_cylinders;
        if LABEL_DOS as libc::c_int == (*ptr_to_globals).current_label_type as libc::c_int {
          warn_cylinders();
        }
      }
      100 => {
        print_raw();
      }
      101 => {
        if LABEL_DOS as libc::c_int == (*ptr_to_globals).current_label_type as libc::c_int {
          x_list_table(1i32);
        }
      }
      102 => {
        if LABEL_DOS as libc::c_int == (*ptr_to_globals).current_label_type as libc::c_int {
          fix_partition_table_order();
        }
      }
      104 => {
        (*ptr_to_globals).g_heads = read_int(
          1i32 as sector_t,
          (*ptr_to_globals).g_heads,
          256i32 as sector_t,
          0i32 as sector_t,
          b"Number of heads\x00" as *const u8 as *const libc::c_char,
        );
        (*ptr_to_globals).user_heads = (*ptr_to_globals).g_heads;
        update_units();
      }
      112 => {
        x_list_table(0i32);
      }
      113 => {
        bb_putchar('\n' as i32);
        exit(0i32);
      }
      114 => return,
      115 => {
        (*ptr_to_globals).g_sectors = read_int(
          1i32 as sector_t,
          (*ptr_to_globals).g_sectors,
          63i32 as sector_t,
          0i32 as sector_t,
          b"Number of sectors\x00" as *const u8 as *const libc::c_char,
        );
        (*ptr_to_globals).user_sectors = (*ptr_to_globals).g_sectors;
        if (*ptr_to_globals).dos_compatible_flag != 0 {
          (*ptr_to_globals).sector_offset = (*ptr_to_globals).g_sectors;
          puts(
            b"Warning: setting sector offset for DOS compatibility\x00" as *const u8
              as *const libc::c_char,
          );
        }
        update_units();
      }
      118 => {
        verify();
      }
      119 => {
        write_table();
      }
      97 | 103 | 105 | 111 | 121 => {}
      _ => {
        xmenu();
      }
    }
  }
}
/* ADVANCED mode */
unsafe extern "C" fn is_ide_cdrom_or_tape(mut device: *const libc::c_char) -> libc::c_int {
  let mut procf: *mut FILE = 0 as *mut FILE;
  let mut buf: [libc::c_char; 100] = [0; 100];
  let mut statbuf: stat = std::mem::zeroed();
  let mut is_ide: libc::c_int = 0i32;
  /* No device was given explicitly, and we are trying some
  likely things.  But opening /dev/hdc may produce errors like
  "hdc: tray open or drive not ready"
  if it happens to be a CD-ROM drive. It even happens that
  the process hangs on the attempt to read a music CD.
  So try to be careful. This only works since 2.1.73. */
  if is_prefixed_with(device, b"/dev/hd\x00" as *const u8 as *const libc::c_char).is_null() {
    return 0i32;
  }
  snprintf(
    buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
    b"/proc/ide/%s/media\x00" as *const u8 as *const libc::c_char,
    device.offset(5),
  );
  procf = fopen_for_read(buf.as_mut_ptr());
  if !procf.is_null()
    && !fgets_unlocked(
      buf.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong as libc::c_int,
      procf,
    )
    .is_null()
  {
    is_ide = (!is_prefixed_with(
      buf.as_mut_ptr(),
      b"cdrom\x00" as *const u8 as *const libc::c_char,
    )
    .is_null()
      || !is_prefixed_with(
        buf.as_mut_ptr(),
        b"tape\x00" as *const u8 as *const libc::c_char,
      )
      .is_null()) as libc::c_int
  } else if stat(device, &mut statbuf) == 0i32 {
    is_ide = (statbuf.st_mode & 0o222i32 as libc::c_uint == 0i32 as libc::c_uint) as libc::c_int
  }
  if !procf.is_null() {
    fclose(procf);
  }
  return is_ide;
}
unsafe extern "C" fn open_list_and_close(
  mut device: *const libc::c_char,
  mut user_specified: libc::c_int,
) {
  let mut gb: libc::c_int = 0;
  (*ptr_to_globals).disk_device = device;
  if _setjmp((*ptr_to_globals).listingbuf.as_mut_ptr()) != 0 {
    return;
  }
  if user_specified == 0 {
    if is_ide_cdrom_or_tape(device) != 0 {
      return;
    }
  }
  /* Now when this proc file does not exist, skip the
  device when it is read-only. */
  /* Open disk_device, save file descriptor to dev_fd */
  *bb_errno = 0i32;
  gb = get_boot(TRY_ONLY);
  if gb > 0i32 {
    /* I/O error */
    /* Ignore other errors, since we try IDE
    and SCSI hard disks which may not be
    installed on the system. */
    if user_specified != 0 || *bb_errno == 13i32 {
      bb_perror_msg(
        b"can\'t open \'%s\'\x00" as *const u8 as *const libc::c_char,
        device,
      );
    }
    return;
  }
  if gb < 0i32 {
    /* no DOS signature */
    list_disk_geometry();
    printf(
      b"Disk %s doesn\'t contain a valid partition table\n\x00" as *const u8 as *const libc::c_char,
      device,
    );
  } else {
    list_table(0i32);
    if 0i32 == 0 && (*ptr_to_globals).g_partitions > 4i32 {
      delete_partition((*ptr_to_globals).ext_index);
    }
  }
  close_dev_fd();
}
/* Is it a whole disk? The digit check is still useful
for Xen devices for example. */
unsafe extern "C" fn is_whole_disk(mut disk: *const libc::c_char) -> libc::c_int {
  let mut len: libc::c_uint = 0;
  let mut fd: libc::c_int = open(disk, 0i32);
  if fd != -1i32 {
    let mut geometry: hd_geometry = hd_geometry {
      heads: 0,
      sectors: 0,
      cylinders: 0,
      start: 0,
    };
    let mut err: libc::c_int = ioctl(
      fd,
      0x301i32 as libc::c_ulong,
      &mut geometry as *mut hd_geometry,
    );
    close(fd);
    if err == 0 {
      return (geometry.start == 0i32 as libc::c_ulong) as libc::c_int;
    }
  }
  /* Treat "nameN" as a partition name, not whole disk */
  /* note: mmcblk0 should work from the geometry check above */
  len = strlen(disk) as libc::c_uint;
  if len != 0i32 as libc::c_uint
    && (*disk.offset(len.wrapping_sub(1i32 as libc::c_uint) as isize) as libc::c_int - '0' as i32)
      as libc::c_uchar as libc::c_int
      <= 9i32
  {
    return 0i32;
  }
  return 1i32;
}
/* for fdisk -l: try all things in /proc/partitions
that look like a partition name (do not end in a digit) */
unsafe extern "C" fn list_devs_in_proc_partititons() {
  let mut procpt: *mut FILE = 0 as *mut FILE;
  let mut line: [libc::c_char; 100] = [0; 100];
  let mut ptname: [libc::c_char; 100] = [0; 100];
  let mut devname: [libc::c_char; 120] = [0; 120];
  let mut ma: libc::c_int = 0;
  let mut mi: libc::c_int = 0;
  let mut sz: libc::c_int = 0;
  procpt = fopen_or_warn(
    b"/proc/partitions\x00" as *const u8 as *const libc::c_char,
    b"r\x00" as *const u8 as *const libc::c_char,
  );
  while !fgets_unlocked(
    line.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong as libc::c_int,
    procpt,
  )
  .is_null()
  {
    if sscanf(
      line.as_mut_ptr(),
      b" %u %u %u %[^\n ]\x00" as *const u8 as *const libc::c_char,
      &mut ma as *mut libc::c_int,
      &mut mi as *mut libc::c_int,
      &mut sz as *mut libc::c_int,
      ptname.as_mut_ptr(),
    ) != 4i32
    {
      continue;
    }
    sprintf(
      devname.as_mut_ptr(),
      b"/dev/%s\x00" as *const u8 as *const libc::c_char,
      ptname.as_mut_ptr(),
    );
    if is_whole_disk(devname.as_mut_ptr()) != 0 {
      open_list_and_close(devname.as_mut_ptr(), 0i32);
    }
  }
}
unsafe extern "C" fn unknown_command(mut c: libc::c_int) {
  printf(
    b"%c: unknown command\n\x00" as *const u8 as *const libc::c_char,
    c,
  );
}
#[no_mangle]
pub unsafe extern "C" fn fdisk_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opt: libc::c_uint = 0;
  /*
   *  fdisk -v
   *  fdisk -l [-b sectorsize] [-u] device ...
   *  fdisk -s [partition] ...
   *  fdisk [-b sectorsize] [-u] device
   *
   * Options -C, -H, -S set the geometry.
   */
  let ref mut fresh2 = *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
    as *mut *mut globals); /* needed: fd 3 must not stay closed */
  *fresh2 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  (*ptr_to_globals).sector_size = 512i32 as libc::c_uint;
  (*ptr_to_globals).sector_offset = 1i32 as libc::c_uint;
  (*ptr_to_globals).g_partitions = 4i32;
  (*ptr_to_globals).units_per_sector = 1i32 as libc::c_uint;
  (*ptr_to_globals).dos_compatible_flag = 1i32 as smallint;
  close_dev_fd();
  opt = getopt32(
    argv,
    b"b:+C:+H:+lS:+u\x00" as *const u8 as *const libc::c_char,
    &mut (*ptr_to_globals).sector_size as *mut libc::c_uint,
    &mut (*ptr_to_globals).user_cylinders as *mut libc::c_uint,
    &mut (*ptr_to_globals).user_heads as *mut libc::c_uint,
    &mut (*ptr_to_globals).user_sectors as *mut libc::c_uint,
  );
  argv = argv.offset(optind as isize);
  if opt & OPT_b as libc::c_int as libc::c_uint != 0 {
    /* Ugly: this sector size is really per device,
     * so cannot be combined with multiple disks,
     * and the same goes for the C/H/S options.
     */
    if (*ptr_to_globals).sector_size < 512i32 as libc::c_uint
      || (*ptr_to_globals).sector_size > 0x10000i32 as libc::c_uint
      || (*ptr_to_globals).sector_size
        & (*ptr_to_globals)
          .sector_size
          .wrapping_sub(1i32 as libc::c_uint)
        != 0
    {
      /* not power of 2 */
      bb_show_usage(); // -u
    }
    (*ptr_to_globals).sector_offset = 2i32 as libc::c_uint;
    (*ptr_to_globals).user_set_sector_size = 1i32 as libc::c_uint
  }
  if (*ptr_to_globals).user_heads <= 0i32 as libc::c_uint
    || (*ptr_to_globals).user_heads >= 256i32 as libc::c_uint
  {
    (*ptr_to_globals).user_heads = 0i32 as libc::c_uint
  }
  if (*ptr_to_globals).user_sectors <= 0i32 as libc::c_uint
    || (*ptr_to_globals).user_sectors >= 64i32 as libc::c_uint
  {
    (*ptr_to_globals).user_sectors = 0i32 as libc::c_uint
  }
  if opt & OPT_u as libc::c_int as libc::c_uint != 0 {
    (*ptr_to_globals).display_in_cyl_units = 0i32 as smallint
  }
  if opt & OPT_l as libc::c_int as libc::c_uint != 0 {
    (*ptr_to_globals).nowarn = 1i32 as smallint;
    if !(*argv).is_null() {
      (*ptr_to_globals).listing = 1i32 as smallint;
      loop {
        open_list_and_close(*argv, 1i32);
        argv = argv.offset(1);
        if (*argv).is_null() {
          break;
        }
      }
    } else {
      /* we don't have device names, */
      /* use /proc/partitions instead */
      list_devs_in_proc_partititons();
    }
    return 0i32;
  }
  if (*argv.offset(0)).is_null() || !(*argv.offset(1)).is_null() {
    bb_show_usage();
  }
  (*ptr_to_globals).disk_device = *argv.offset(0);
  get_boot(OPEN_MAIN);
  loop {
    let mut c: libc::c_int = 0;
    bb_putchar('\n' as i32);
    c = 0x20i32
      | read_nonempty(b"Command (m for help): \x00" as *const u8 as *const libc::c_char)
        as libc::c_int;
    let mut current_block_99: u64;
    match c {
      97 => {
        if LABEL_DOS as libc::c_int == (*ptr_to_globals).current_label_type as libc::c_int {
          toggle_active(
            get_partition(1i32, (*ptr_to_globals).g_partitions as libc::c_uint) as libc::c_int,
          );
        } else {
          unknown_command(c);
        }
        current_block_99 = 11995618668192240200;
      }
      99 => {
        if LABEL_DOS as libc::c_int == (*ptr_to_globals).current_label_type as libc::c_int {
          toggle_dos_compatibility_flag();
        } else {
          unknown_command(c);
        }
        current_block_99 = 11995618668192240200;
      }
      100 => {
        let mut j: libc::c_int = 0;
        /* If sgi_label then don't use get_existing_partition,
        let the user select a partition, since
        get_existing_partition() only works for Linux-like
        partition tables */
        if 0i32 == 0 {
          j = get_existing_partition(1i32, (*ptr_to_globals).g_partitions as libc::c_uint)
        } else {
          j = get_partition(1i32, (*ptr_to_globals).g_partitions as libc::c_uint) as libc::c_int
        } /* does not return */
        if j >= 0i32 {
          delete_partition(j);
        }
        current_block_99 = 11995618668192240200;
      }
      105 => {
        unknown_command(c);
        current_block_99 = 11200608247783221354;
      }
      108 => {
        current_block_99 = 11200608247783221354;
      }
      109 => {
        menu();
        current_block_99 = 11995618668192240200;
      }
      110 => {
        new_partition();
        current_block_99 = 11995618668192240200;
      }
      111 => {
        create_doslabel();
        current_block_99 = 11995618668192240200;
      }
      112 => {
        list_table(0i32);
        current_block_99 = 11995618668192240200;
      }
      113 => {
        bb_putchar('\n' as i32);
        return 0i32;
      }
      98 | 115 => {
        current_block_99 = 11995618668192240200;
      }
      116 => {
        change_sysid();
        current_block_99 = 11995618668192240200;
      }
      117 => {
        change_units();
        current_block_99 = 11995618668192240200;
      }
      118 => {
        verify();
        current_block_99 = 11995618668192240200;
      }
      119 => {
        write_table();
        current_block_99 = 11995618668192240200;
      }
      120 => {
        xselect();
        current_block_99 = 11995618668192240200;
      }
      _ => {
        unknown_command(c);
        menu();
        current_block_99 = 11995618668192240200;
      }
    }
    match current_block_99 {
      11200608247783221354 => {
        list_types(get_sys_types());
      }
      _ => {}
    }
  }
  /* FEATURE_FDISK_WRITABLE */
}

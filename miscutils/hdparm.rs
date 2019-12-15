use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::xfuncs_printf::xmalloc;
use libc;
use libc::close;
use libc::free;
use libc::getopt;
use libc::ioctl;
use libc::isatty;
use libc::printf;
use libc::puts;
use libc::sleep;
use libc::sync;
extern "C" {

  #[no_mangle]
  static mut optarg: *mut libc::c_char;
  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn fsync(__fd: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn exit(_: libc::c_int) -> !;
  #[no_mangle]
  fn abs(_: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn mlock(__addr: *const libc::c_void, __len: size_t) -> libc::c_int;
  #[no_mangle]
  fn munlock(__addr: *const libc::c_void, __len: size_t) -> libc::c_int;

  /* Some useful definitions */
  /* Macros for min/max.  */
  /* buffer allocation schemes */
  /* glibc uses __errno_location() to get a ptr to errno */
  /* We can just memorize it once - no multithreading in busybox :) */

  #[no_mangle]
  fn monotonic_us() -> libc::c_ulonglong;

  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn xopen_nonblocking(pathname: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xlseek(fd_0: libc::c_int, offset: off_t, whence: libc::c_int) -> off_t;
  /* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
   * But potentially slow, don't use in one-billion-times loops */
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xread(fd_0: libc::c_int, buf: *mut libc::c_void, count: size_t);
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn xatoll_range(
    str: *const libc::c_char,
    l: libc::c_longlong,
    u: libc::c_longlong,
  ) -> libc::c_longlong;
  #[no_mangle]
  fn bb_strtoi(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_int;
  /* Specialized: */
  /* Using xatoi() instead of naive atoi() is not always convenient -
   * in many places people want *non-negative* values, but store them
   * in signed int. Therefore we need this one:
   * dies if input is not in [0, INT_MAX] range. Also will reject '-0' etc.
   * It should really be named xatoi_nonnegative (since it allows 0),
   * but that would be too long.
   */
  #[no_mangle]
  fn xatoi_positive(numstr: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn nth_string(strings: *const libc::c_char, n: libc::c_int) -> *const libc::c_char;
  #[no_mangle]
  fn bb_ioctl_or_warn(
    fd_0: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  fn bb_xioctl(
    fd_0: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  fn print_flags_separated(
    masks: *const libc::c_int,
    labels: *const libc::c_char,
    flags: libc::c_int,
    separator: *const libc::c_char,
  ) -> libc::c_int;

  // #[no_mangle]
  // fn print_flags(ml: *const masks_labels_t, flags: libc::c_int)
  //  -> libc::c_int;

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
use crate::librb::size_t;
use crate::librb::smallint;
use libc::off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct masks_labels_t {
  pub labels: *const libc::c_char,
  pub masks: [libc::c_int; 0],
}
//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub get_identity: smallint,
  pub get_geom: smallint,
  pub do_flush: smallint,
  pub do_ctimings: smallint,
  pub do_timings: smallint,
  pub reread_partn: smallint,
  pub set_piomode: smallint,
  pub noisy_piomode: smallint,
  pub getset_readahead: smallint,
  pub getset_readonly: smallint,
  pub getset_unmask: smallint,
  pub getset_mult: smallint,
  pub getset_dma_q: smallint,
  pub getset_nowerr: smallint,
  pub getset_keep: smallint,
  pub getset_io32bit: smallint,
  pub piomode: libc::c_int,
  pub Xreadahead: libc::c_ulong,
  pub readonly: libc::c_ulong,
  pub unmask: libc::c_ulong,
  pub mult: libc::c_ulong,
  pub dma_q: libc::c_ulong,
  pub nowerr: libc::c_ulong,
  pub keep: libc::c_ulong,
  pub io32bit: libc::c_ulong,
  pub dma: libc::c_ulong,
  pub getset_dma: smallint,
  pub set_xfermode: smallint,
  pub get_xfermode: smallint,
  pub getset_dkeep: smallint,
  pub getset_standby: smallint,
  pub getset_lookahead: smallint,
  pub getset_prefetch: smallint,
  pub getset_defects: smallint,
  pub getset_wcache: smallint,
  pub getset_doorlock: smallint,
  pub set_seagate: smallint,
  pub set_standbynow: smallint,
  pub set_sleepnow: smallint,
  pub get_powermode: smallint,
  pub getset_apmmode: smallint,
  pub xfermode_requested: libc::c_int,
  pub dkeep: libc::c_ulong,
  pub standby_requested: libc::c_ulong,
  pub lookahead: libc::c_ulong,
  pub prefetch: libc::c_ulong,
  pub defects: libc::c_ulong,
  pub wcache: libc::c_ulong,
  pub doorlock: libc::c_ulong,
  pub apmmode: libc::c_ulong,
  pub get_IDentity: smallint,
  pub getset_busstate: smallint,
  pub perform_reset: smallint,
  pub perform_tristate: smallint,
  pub unregister_hwif: smallint,
  pub scan_hwif: smallint,
  pub busstate: libc::c_ulong,
  pub tristate: libc::c_ulong,
  pub hwif: libc::c_ulong,
  pub hwif_data: libc::c_ulong,
  pub hwif_ctrl: libc::c_ulong,
  pub hwif_irq: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hd_geometry {
  pub heads: libc::c_uchar,
  pub sectors: libc::c_uchar,
  pub cylinders: libc::c_ushort,
  pub start: libc::c_ulong,
}
pub type C2RustUnnamed = libc::c_uint;
pub const BUSSTATE_TRISTATE: C2RustUnnamed = 2;
pub const BUSSTATE_ON: C2RustUnnamed = 1;
pub const BUSSTATE_OFF: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hd_driveid {
  pub config: libc::c_ushort,
  pub cyls: libc::c_ushort,
  pub reserved2: libc::c_ushort,
  pub heads: libc::c_ushort,
  pub track_bytes: libc::c_ushort,
  pub sector_bytes: libc::c_ushort,
  pub sectors: libc::c_ushort,
  pub vendor0: libc::c_ushort,
  pub vendor1: libc::c_ushort,
  pub vendor2: libc::c_ushort,
  pub serial_no: [libc::c_uchar; 20],
  pub buf_type: libc::c_ushort,
  pub buf_size: libc::c_ushort,
  pub ecc_bytes: libc::c_ushort,
  pub fw_rev: [libc::c_uchar; 8],
  pub model: [libc::c_uchar; 40],
  pub max_multsect: libc::c_uchar,
  pub vendor3: libc::c_uchar,
  pub dword_io: libc::c_ushort,
  pub vendor4: libc::c_uchar,
  pub capability: libc::c_uchar,
  pub reserved50: libc::c_ushort,
  pub vendor5: libc::c_uchar,
  pub tPIO: libc::c_uchar,
  pub vendor6: libc::c_uchar,
  pub tDMA: libc::c_uchar,
  pub field_valid: libc::c_ushort,
  pub cur_cyls: libc::c_ushort,
  pub cur_heads: libc::c_ushort,
  pub cur_sectors: libc::c_ushort,
  pub cur_capacity0: libc::c_ushort,
  pub cur_capacity1: libc::c_ushort,
  pub multsect: libc::c_uchar,
  pub multsect_valid: libc::c_uchar,
  pub lba_capacity: libc::c_uint,
  pub dma_1word: libc::c_ushort,
  pub dma_mword: libc::c_ushort,
  pub eide_pio_modes: libc::c_ushort,
  pub eide_dma_min: libc::c_ushort,
  pub eide_dma_time: libc::c_ushort,
  pub eide_pio: libc::c_ushort,
  pub eide_pio_iordy: libc::c_ushort,
  pub words69_70: [libc::c_ushort; 2],
  pub words71_74: [libc::c_ushort; 4],
  pub queue_depth: libc::c_ushort,
  pub words76_79: [libc::c_ushort; 4],
  pub major_rev_num: libc::c_ushort,
  pub minor_rev_num: libc::c_ushort,
  pub command_set_1: libc::c_ushort,
  pub command_set_2: libc::c_ushort,
  pub cfsse: libc::c_ushort,
  pub cfs_enable_1: libc::c_ushort,
  pub cfs_enable_2: libc::c_ushort,
  pub csf_default: libc::c_ushort,
  pub dma_ultra: libc::c_ushort,
  pub trseuc: libc::c_ushort,
  pub trsEuc: libc::c_ushort,
  pub CurAPMvalues: libc::c_ushort,
  pub mprc: libc::c_ushort,
  pub hw_config: libc::c_ushort,
  pub acoustic: libc::c_ushort,
  pub msrqs: libc::c_ushort,
  pub sxfert: libc::c_ushort,
  pub sal: libc::c_ushort,
  pub spg: libc::c_uint,
  pub lba_capacity_2: libc::c_ulonglong,
  pub words104_125: [libc::c_ushort; 22],
  pub last_lun: libc::c_ushort,
  pub word127: libc::c_ushort,
  pub dlf: libc::c_ushort,
  pub csfo: libc::c_ushort,
  pub words130_155: [libc::c_ushort; 26],
  pub word156: libc::c_ushort,
  pub words157_159: [libc::c_ushort; 3],
  pub cfa_power: libc::c_ushort,
  pub words161_175: [libc::c_ushort; 15],
  pub words176_205: [libc::c_ushort; 30],
  pub words206_254: [libc::c_ushort; 49],
  pub integrity_word: libc::c_ushort,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const fd: C2RustUnnamed_0 = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
  pub blksize64: libc::c_ulonglong,
  pub blksize32: libc::c_uint,
}
#[inline(always)]
unsafe extern "C" fn xatol_range(
  mut str: *const libc::c_char,
  mut l: libc::c_long,
  mut u: libc::c_long,
) -> libc::c_long {
  return xatoll_range(str, l as libc::c_longlong, u as libc::c_longlong) as libc::c_long;
}
/* Busybox messages and functions */
unsafe extern "C" fn ioctl_alt_func(
  mut cmd: libc::c_int,
  mut args: *mut libc::c_uchar,
  mut alt: libc::c_int,
  mut string: *const libc::c_char,
) -> libc::c_int {
  if ioctl(fd as libc::c_int, cmd as libc::c_ulong, args) == 0 {
    return 0i32;
  }
  *args.offset(0) = alt as libc::c_uchar;
  return bb_ioctl_or_warn(
    fd as libc::c_int,
    cmd as libc::c_uint,
    args as *mut libc::c_void,
    string,
  );
}
unsafe extern "C" fn on_off(mut value: libc::c_int) {
  puts(if value != 0 {
    b" (on)\x00" as *const u8 as *const libc::c_char
  } else {
    b" (off)\x00" as *const u8 as *const libc::c_char
  });
}
unsafe extern "C" fn print_flag_on_off(
  mut get_arg: libc::c_int,
  mut s: *const libc::c_char,
  mut arg: libc::c_ulong,
) {
  if get_arg != 0 {
    printf(
      b" setting %s to %lu\x00" as *const u8 as *const libc::c_char,
      s,
      arg,
    );
    on_off(arg as libc::c_int);
  };
}
unsafe extern "C" fn print_value_on_off(mut str: *const libc::c_char, mut argp: libc::c_ulong) {
  printf(
    b" %s\t= %2lu\x00" as *const u8 as *const libc::c_char,
    str,
    argp,
  );
  on_off((argp != 0i32 as libc::c_ulong) as libc::c_int);
}
unsafe extern "C" fn print_ascii(mut p: *const libc::c_char, mut length: libc::c_int) {
  /* every 16bit word is big-endian (i.e. inverted) */
  /* accessing bytes in 1,0, 3,2, 5,4... sequence */
  let mut ofs: libc::c_int = 1i32;
  length *= 2i32;
  /* find first non-space & print it */
  while length != 0 && *p.offset(ofs as isize) as libc::c_int != ' ' as i32 {
    p = p.offset(1);
    ofs = -ofs;
    length -= 1
  }
  while length != 0 && *p.offset(ofs as isize) as libc::c_int != 0 {
    bb_putchar(*p.offset(ofs as isize) as libc::c_int);
    p = p.offset(1);
    ofs = -ofs;
    length -= 1
  }
  bb_putchar('\n' as i32);
}
unsafe extern "C" fn xprint_ascii(
  mut val: *mut u16,
  mut i: libc::c_int,
  mut string: *const libc::c_char,
  mut n: libc::c_int,
) {
  if *val.offset(i as isize) != 0 {
    printf(b"\t%-20s\x00" as *const u8 as *const libc::c_char, string);
    print_ascii(
      &mut *val.offset(i as isize) as *mut u16 as *mut libc::c_void as *const libc::c_char,
      n,
    );
  };
}
unsafe extern "C" fn mode_loop(
  mut mode_sup: u16,
  mut mode_sel: u16,
  mut cc: libc::c_int,
  mut have_mode: *mut u8,
) -> u8 {
  let mut ii: u16 = 0;
  let mut err_dma: u8 = 0i32 as u8;
  ii = 0i32 as u16;
  while ii as libc::c_int <= 7i32 {
    if mode_sel as libc::c_int & 0x1i32 != 0 {
      printf(
        b"*%cdma%u \x00" as *const u8 as *const libc::c_char,
        cc,
        ii as libc::c_int,
      );
      if *have_mode != 0 {
        err_dma = 1i32 as u8
      }
      *have_mode = 1i32 as u8
    } else if mode_sup as libc::c_int & 0x1i32 != 0 {
      printf(
        b"%cdma%u \x00" as *const u8 as *const libc::c_char,
        cc,
        ii as libc::c_int,
      );
    }
    mode_sup = (mode_sup as libc::c_int >> 1i32) as u16;
    mode_sel = (mode_sel as libc::c_int >> 1i32) as u16;
    ii = ii.wrapping_add(1)
  }
  return err_dma;
}
static mut pkt_str: [libc::c_char; 272] = [
  68, 105, 114, 101, 99, 116, 45, 97, 99, 99, 101, 115, 115, 32, 100, 101, 118, 105, 99, 101, 0,
  83, 101, 113, 117, 101, 110, 116, 105, 97, 108, 45, 97, 99, 99, 101, 115, 115, 32, 100, 101, 118,
  105, 99, 101, 0, 80, 114, 105, 110, 116, 101, 114, 0, 80, 114, 111, 99, 101, 115, 115, 111, 114,
  0, 87, 114, 105, 116, 101, 45, 111, 110, 99, 101, 32, 100, 101, 118, 105, 99, 101, 0, 67, 68, 45,
  82, 79, 77, 0, 83, 99, 97, 110, 110, 101, 114, 0, 79, 112, 116, 105, 99, 97, 108, 32, 109, 101,
  109, 111, 114, 121, 0, 77, 101, 100, 105, 117, 109, 32, 99, 104, 97, 110, 103, 101, 114, 0, 67,
  111, 109, 109, 117, 110, 105, 99, 97, 116, 105, 111, 110, 115, 32, 100, 101, 118, 105, 99, 101,
  0, 65, 67, 83, 45, 73, 84, 56, 32, 100, 101, 118, 105, 99, 101, 0, 65, 67, 83, 45, 73, 84, 56,
  32, 100, 101, 118, 105, 99, 101, 0, 65, 114, 114, 97, 121, 32, 99, 111, 110, 116, 114, 111, 108,
  108, 101, 114, 0, 69, 110, 99, 108, 111, 115, 117, 114, 101, 32, 115, 101, 114, 118, 105, 99,
  101, 115, 0, 82, 101, 100, 117, 99, 101, 100, 32, 98, 108, 111, 99, 107, 32, 99, 111, 109, 109,
  97, 110, 100, 32, 100, 101, 118, 105, 99, 101, 0, 79, 112, 116, 105, 99, 97, 108, 32, 99, 97,
  114, 100, 32, 114, 101, 97, 100, 101, 114, 47, 119, 114, 105, 116, 101, 114, 0, 0,
];
/* word 0, bits 12-8 = 0f */
static mut ata1_cfg_str: [libc::c_char; 318] = [
  114, 101, 115, 101, 114, 118, 101, 100, 0, 104, 97, 114, 100, 32, 115, 101, 99, 116, 111, 114,
  101, 100, 0, 115, 111, 102, 116, 32, 115, 101, 99, 116, 111, 114, 101, 100, 0, 110, 111, 116, 32,
  77, 70, 77, 32, 101, 110, 99, 111, 100, 101, 100, 32, 0, 104, 101, 97, 100, 32, 115, 119, 105,
  116, 99, 104, 32, 116, 105, 109, 101, 32, 62, 32, 49, 53, 117, 115, 0, 115, 112, 105, 110, 100,
  108, 101, 32, 109, 111, 116, 111, 114, 32, 99, 111, 110, 116, 114, 111, 108, 32, 111, 112, 116,
  105, 111, 110, 0, 102, 105, 120, 101, 100, 32, 100, 114, 105, 118, 101, 0, 114, 101, 109, 111,
  118, 97, 98, 108, 101, 32, 100, 114, 105, 118, 101, 0, 100, 105, 115, 107, 32, 120, 102, 101,
  114, 32, 114, 97, 116, 101, 32, 60, 61, 32, 53, 77, 98, 115, 0, 100, 105, 115, 107, 32, 120, 102,
  101, 114, 32, 114, 97, 116, 101, 32, 62, 32, 53, 77, 98, 115, 44, 32, 60, 61, 32, 49, 48, 77, 98,
  115, 0, 100, 105, 115, 107, 32, 120, 102, 101, 114, 32, 114, 97, 116, 101, 32, 62, 32, 53, 77,
  98, 115, 0, 114, 111, 116, 97, 116, 105, 111, 110, 97, 108, 32, 115, 112, 101, 101, 100, 32, 116,
  111, 108, 46, 0, 100, 97, 116, 97, 32, 115, 116, 114, 111, 98, 101, 32, 111, 102, 102, 115, 101,
  116, 32, 111, 112, 116, 105, 111, 110, 0, 116, 114, 97, 99, 107, 32, 111, 102, 102, 115, 101,
  116, 32, 111, 112, 116, 105, 111, 110, 0, 102, 111, 114, 109, 97, 116, 32, 115, 112, 101, 101,
  100, 32, 116, 111, 108, 101, 114, 97, 110, 99, 101, 32, 103, 97, 112, 32, 114, 101, 113, 100, 0,
  65, 84, 65, 80, 73, 0,
];
/* bit 14 */
static mut minor_str: [libc::c_char; 1016] = [
  85, 110, 115, 112, 101, 99, 105, 102, 105, 101, 100, 0, 65, 84, 65, 45, 49, 32, 88, 51, 84, 57,
  46, 50, 32, 55, 56, 49, 68, 32, 112, 114, 105, 111, 114, 32, 116, 111, 32, 114, 101, 118, 46, 52,
  0, 65, 84, 65, 45, 49, 32, 112, 117, 98, 108, 105, 115, 104, 101, 100, 44, 32, 65, 78, 83, 73,
  32, 88, 51, 46, 50, 50, 49, 45, 49, 57, 57, 52, 0, 65, 84, 65, 45, 49, 32, 88, 51, 84, 57, 46,
  50, 32, 55, 56, 49, 68, 32, 114, 101, 118, 46, 52, 0, 65, 84, 65, 45, 50, 32, 112, 117, 98, 108,
  105, 115, 104, 101, 100, 44, 32, 65, 78, 83, 73, 32, 88, 51, 46, 50, 55, 57, 45, 49, 57, 57, 54,
  0, 65, 84, 65, 45, 50, 32, 88, 51, 84, 49, 48, 32, 57, 52, 56, 68, 32, 112, 114, 105, 111, 114,
  32, 116, 111, 32, 114, 101, 118, 46, 50, 107, 0, 65, 84, 65, 45, 51, 32, 88, 51, 84, 49, 48, 32,
  50, 48, 48, 56, 68, 32, 114, 101, 118, 46, 49, 0, 65, 84, 65, 45, 50, 32, 88, 51, 84, 49, 48, 32,
  57, 52, 56, 68, 32, 114, 101, 118, 46, 50, 107, 0, 65, 84, 65, 45, 51, 32, 88, 51, 84, 49, 48,
  32, 50, 48, 48, 56, 68, 32, 114, 101, 118, 46, 48, 0, 65, 84, 65, 45, 50, 32, 88, 51, 84, 49, 48,
  32, 57, 52, 56, 68, 32, 114, 101, 118, 46, 51, 0, 65, 84, 65, 45, 51, 32, 112, 117, 98, 108, 105,
  115, 104, 101, 100, 44, 32, 65, 78, 83, 73, 32, 88, 51, 46, 50, 57, 56, 45, 49, 57, 57, 120, 0,
  65, 84, 65, 45, 51, 32, 88, 51, 84, 49, 48, 32, 50, 48, 48, 56, 68, 32, 114, 101, 118, 46, 54, 0,
  65, 84, 65, 45, 51, 32, 88, 51, 84, 49, 51, 32, 50, 48, 48, 56, 68, 32, 114, 101, 118, 46, 55,
  32, 97, 110, 100, 32, 55, 97, 0, 65, 84, 65, 47, 65, 84, 65, 80, 73, 45, 52, 32, 88, 51, 84, 49,
  51, 32, 49, 49, 53, 51, 68, 32, 114, 101, 118, 46, 54, 0, 65, 84, 65, 47, 65, 84, 65, 80, 73, 45,
  52, 32, 84, 49, 51, 32, 49, 49, 53, 51, 68, 32, 114, 101, 118, 46, 49, 51, 0, 65, 84, 65, 47, 65,
  84, 65, 80, 73, 45, 52, 32, 88, 51, 84, 49, 51, 32, 49, 49, 53, 51, 68, 32, 114, 101, 118, 46,
  55, 0, 65, 84, 65, 47, 65, 84, 65, 80, 73, 45, 52, 32, 84, 49, 51, 32, 49, 49, 53, 51, 68, 32,
  114, 101, 118, 46, 49, 56, 0, 65, 84, 65, 47, 65, 84, 65, 80, 73, 45, 52, 32, 84, 49, 51, 32, 49,
  49, 53, 51, 68, 32, 114, 101, 118, 46, 49, 53, 0, 65, 84, 65, 47, 65, 84, 65, 80, 73, 45, 52, 32,
  112, 117, 98, 108, 105, 115, 104, 101, 100, 44, 32, 65, 78, 83, 73, 32, 73, 78, 67, 73, 84, 83,
  32, 51, 49, 55, 45, 49, 57, 57, 56, 0, 65, 84, 65, 47, 65, 84, 65, 80, 73, 45, 53, 32, 84, 49,
  51, 32, 49, 51, 50, 49, 68, 32, 114, 101, 118, 46, 51, 0, 65, 84, 65, 47, 65, 84, 65, 80, 73, 45,
  52, 32, 84, 49, 51, 32, 49, 49, 53, 51, 68, 32, 114, 101, 118, 46, 49, 52, 0, 65, 84, 65, 47, 65,
  84, 65, 80, 73, 45, 53, 32, 84, 49, 51, 32, 49, 51, 50, 49, 68, 32, 114, 101, 118, 46, 49, 0, 65,
  84, 65, 47, 65, 84, 65, 80, 73, 45, 53, 32, 112, 117, 98, 108, 105, 115, 104, 101, 100, 44, 32,
  65, 78, 83, 73, 32, 73, 78, 67, 73, 84, 83, 32, 51, 52, 48, 45, 50, 48, 48, 48, 0, 65, 84, 65,
  47, 65, 84, 65, 80, 73, 45, 52, 32, 84, 49, 51, 32, 49, 49, 53, 51, 68, 32, 114, 101, 118, 46,
  49, 55, 0, 65, 84, 65, 47, 65, 84, 65, 80, 73, 45, 54, 32, 84, 49, 51, 32, 49, 52, 49, 48, 68,
  32, 114, 101, 118, 46, 48, 0, 65, 84, 65, 47, 65, 84, 65, 80, 73, 45, 54, 32, 84, 49, 51, 32, 49,
  52, 49, 48, 68, 32, 114, 101, 118, 46, 51, 97, 0, 65, 84, 65, 47, 65, 84, 65, 80, 73, 45, 55, 32,
  84, 49, 51, 32, 49, 53, 51, 50, 68, 32, 114, 101, 118, 46, 49, 0, 65, 84, 65, 47, 65, 84, 65, 80,
  73, 45, 54, 32, 84, 49, 51, 32, 49, 52, 49, 48, 68, 32, 114, 101, 118, 46, 50, 0, 65, 84, 65, 47,
  65, 84, 65, 80, 73, 45, 54, 32, 84, 49, 51, 32, 49, 52, 49, 48, 68, 32, 114, 101, 118, 46, 49, 0,
  65, 84, 65, 47, 65, 84, 65, 80, 73, 45, 55, 32, 112, 117, 98, 108, 105, 115, 104, 101, 100, 44,
  32, 65, 78, 83, 73, 32, 73, 78, 67, 73, 84, 83, 32, 51, 57, 55, 45, 50, 48, 48, 53, 0, 65, 84,
  65, 47, 65, 84, 65, 80, 73, 45, 55, 32, 84, 49, 51, 32, 49, 53, 51, 50, 68, 32, 114, 101, 118,
  46, 48, 0, 114, 101, 115, 101, 114, 118, 101, 100, 0, 114, 101, 115, 101, 114, 118, 101, 100, 0,
  65, 84, 65, 47, 65, 84, 65, 80, 73, 45, 55, 32, 84, 49, 51, 32, 49, 53, 51, 50, 68, 32, 114, 101,
  118, 46, 52, 97, 0, 65, 84, 65, 47, 65, 84, 65, 80, 73, 45, 54, 32, 112, 117, 98, 108, 105, 115,
  104, 101, 100, 44, 32, 65, 78, 83, 73, 32, 73, 78, 67, 73, 84, 83, 32, 51, 54, 49, 45, 50, 48,
  48, 50, 0, 114, 101, 115, 101, 114, 118, 101, 100, 0,
];
/* 0x0023-0xfffe */
static mut actual_ver: [libc::c_char; 36] = [
  0i32 as libc::c_char,
  1i32 as libc::c_char,
  1i32 as libc::c_char,
  1i32 as libc::c_char,
  2i32 as libc::c_char,
  2i32 as libc::c_char,
  3i32 as libc::c_char,
  2i32 as libc::c_char,
  3i32 as libc::c_char,
  2i32 as libc::c_char,
  3i32 as libc::c_char,
  3i32 as libc::c_char,
  3i32 as libc::c_char,
  4i32 as libc::c_char,
  4i32 as libc::c_char,
  4i32 as libc::c_char,
  4i32 as libc::c_char,
  4i32 as libc::c_char,
  4i32 as libc::c_char,
  5i32 as libc::c_char,
  4i32 as libc::c_char,
  5i32 as libc::c_char,
  5i32 as libc::c_char,
  4i32 as libc::c_char,
  6i32 as libc::c_char,
  6i32 as libc::c_char,
  7i32 as libc::c_char,
  6i32 as libc::c_char,
  6i32 as libc::c_char,
  7i32 as libc::c_char,
  7i32 as libc::c_char,
  0i32 as libc::c_char,
  0i32 as libc::c_char,
  7i32 as libc::c_char,
  6i32 as libc::c_char,
  0i32 as libc::c_char,
];
static mut cmd_feat_str: [libc::c_char; 844] = [
  0, 78, 79, 80, 32, 99, 109, 100, 0, 82, 69, 65, 68, 32, 66, 85, 70, 70, 69, 82, 32, 99, 109, 100,
  0, 87, 82, 73, 84, 69, 32, 66, 85, 70, 70, 69, 82, 32, 99, 109, 100, 0, 0, 72, 111, 115, 116, 32,
  80, 114, 111, 116, 101, 99, 116, 101, 100, 32, 65, 114, 101, 97, 32, 102, 101, 97, 116, 117, 114,
  101, 32, 115, 101, 116, 0, 68, 69, 86, 73, 67, 69, 32, 82, 69, 83, 69, 84, 32, 99, 109, 100, 0,
  83, 69, 82, 86, 73, 67, 69, 32, 105, 110, 116, 101, 114, 114, 117, 112, 116, 0, 82, 101, 108,
  101, 97, 115, 101, 32, 105, 110, 116, 101, 114, 114, 117, 112, 116, 0, 76, 111, 111, 107, 45, 97,
  104, 101, 97, 100, 0, 87, 114, 105, 116, 101, 32, 99, 97, 99, 104, 101, 0, 80, 65, 67, 75, 69,
  84, 32, 99, 111, 109, 109, 97, 110, 100, 32, 102, 101, 97, 116, 117, 114, 101, 32, 115, 101, 116,
  0, 80, 111, 119, 101, 114, 32, 77, 97, 110, 97, 103, 101, 109, 101, 110, 116, 32, 102, 101, 97,
  116, 117, 114, 101, 32, 115, 101, 116, 0, 82, 101, 109, 111, 118, 97, 98, 108, 101, 32, 77, 101,
  100, 105, 97, 32, 102, 101, 97, 116, 117, 114, 101, 32, 115, 101, 116, 0, 83, 101, 99, 117, 114,
  105, 116, 121, 32, 77, 111, 100, 101, 32, 102, 101, 97, 116, 117, 114, 101, 32, 115, 101, 116, 0,
  83, 77, 65, 82, 84, 32, 102, 101, 97, 116, 117, 114, 101, 32, 115, 101, 116, 0, 0, 0, 70, 76, 85,
  83, 72, 32, 67, 65, 67, 72, 69, 32, 69, 88, 84, 32, 99, 109, 100, 0, 77, 97, 110, 100, 97, 116,
  111, 114, 121, 32, 70, 76, 85, 83, 72, 32, 67, 65, 67, 72, 69, 32, 99, 109, 100, 32, 0, 68, 101,
  118, 105, 99, 101, 32, 67, 111, 110, 102, 105, 103, 117, 114, 97, 116, 105, 111, 110, 32, 79,
  118, 101, 114, 108, 97, 121, 32, 102, 101, 97, 116, 117, 114, 101, 32, 115, 101, 116, 32, 0, 52,
  56, 45, 98, 105, 116, 32, 65, 100, 100, 114, 101, 115, 115, 32, 102, 101, 97, 116, 117, 114, 101,
  32, 115, 101, 116, 32, 0, 0, 83, 69, 84, 32, 77, 65, 88, 32, 115, 101, 99, 117, 114, 105, 116,
  121, 32, 101, 120, 116, 101, 110, 115, 105, 111, 110, 0, 65, 100, 100, 114, 101, 115, 115, 32,
  79, 102, 102, 115, 101, 116, 32, 82, 101, 115, 101, 114, 118, 101, 100, 32, 65, 114, 101, 97, 32,
  66, 111, 111, 116, 0, 83, 69, 84, 32, 70, 69, 65, 84, 85, 82, 69, 83, 32, 115, 117, 98, 99, 111,
  109, 109, 97, 110, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 116, 111, 32, 115, 112,
  105, 110, 117, 112, 32, 97, 102, 116, 101, 114, 32, 112, 111, 119, 101, 114, 32, 117, 112, 0, 80,
  111, 119, 101, 114, 45, 85, 112, 32, 73, 110, 32, 83, 116, 97, 110, 100, 98, 121, 32, 102, 101,
  97, 116, 117, 114, 101, 32, 115, 101, 116, 0, 82, 101, 109, 111, 118, 97, 98, 108, 101, 32, 77,
  101, 100, 105, 97, 32, 83, 116, 97, 116, 117, 115, 32, 78, 111, 116, 105, 102, 105, 99, 97, 116,
  105, 111, 110, 32, 102, 101, 97, 116, 117, 114, 101, 32, 115, 101, 116, 0, 65, 100, 118, 46, 32,
  80, 111, 119, 101, 114, 32, 77, 97, 110, 97, 103, 101, 109, 101, 110, 116, 32, 102, 101, 97, 116,
  117, 114, 101, 32, 115, 101, 116, 0, 67, 70, 65, 32, 102, 101, 97, 116, 117, 114, 101, 32, 115,
  101, 116, 0, 82, 69, 65, 68, 47, 87, 82, 73, 84, 69, 32, 68, 77, 65, 32, 81, 85, 69, 85, 69, 68,
  0, 68, 79, 87, 78, 76, 79, 65, 68, 32, 77, 73, 67, 82, 79, 67, 79, 68, 69, 32, 99, 109, 100, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 101, 110, 101, 114, 97, 108, 32, 80, 117, 114, 112, 111, 115,
  101, 32, 76, 111, 103, 103, 105, 110, 103, 32, 102, 101, 97, 116, 117, 114, 101, 32, 115, 101,
  116, 0, 0, 77, 101, 100, 105, 97, 32, 67, 97, 114, 100, 32, 80, 97, 115, 115, 32, 84, 104, 114,
  111, 117, 103, 104, 32, 67, 111, 109, 109, 97, 110, 100, 32, 102, 101, 97, 116, 117, 114, 101,
  32, 115, 101, 116, 32, 0, 77, 101, 100, 105, 97, 32, 115, 101, 114, 105, 97, 108, 32, 110, 117,
  109, 98, 101, 114, 32, 0, 83, 77, 65, 82, 84, 32, 115, 101, 108, 102, 45, 116, 101, 115, 116, 32,
  0, 83, 77, 65, 82, 84, 32, 101, 114, 114, 111, 114, 32, 108, 111, 103, 103, 105, 110, 103, 32, 0,
];
/* word 84 bit  0 */
static mut secu_str: [libc::c_char; 82] = [
  115, 117, 112, 112, 111, 114, 116, 101, 100, 0, 101, 110, 97, 98, 108, 101, 100, 0, 108, 111, 99,
  107, 101, 100, 0, 102, 114, 111, 122, 101, 110, 0, 101, 120, 112, 105, 114, 101, 100, 58, 32,
  115, 101, 99, 117, 114, 105, 116, 121, 32, 99, 111, 117, 110, 116, 0, 115, 117, 112, 112, 111,
  114, 116, 101, 100, 58, 32, 101, 110, 104, 97, 110, 99, 101, 100, 32, 101, 114, 97, 115, 101, 0,
];
/* word 128, bit 5 */
// Parse 512 byte disk identification block and print much crap.
unsafe extern "C" fn identify(mut val: *mut u16) -> ! {
  let mut ii: u16 = 0; /* (:) */
  let mut jj: u16 = 0;
  let mut kk: u16 = 0;
  let mut like_std: u16 = 1i32 as u16;
  let mut std: u16 = 0i32 as u16;
  let mut min_std: u16 = 0xffffi32 as u16;
  let mut dev: u16 = 0xffffi32 as u16;
  let mut eqpt: u16 = 0xffffi32 as u16;
  let mut have_mode: u8 = 0i32 as u8;
  let mut err_dma: u8 = 0i32 as u8;
  let mut chksum: u8 = 0i32 as u8;
  let mut ll: u32 = 0;
  let mut mm: u32 = 0;
  let mut nn: u32 = 0;
  let mut oo: u32 = 0;
  let mut bbbig: u64 = 0;
  let mut strng: *const libc::c_char = std::ptr::null();
  /* check if we recognize the device type */
  bb_putchar('\n' as i32);
  if *val.offset(0) as libc::c_int & 0x8000i32 == 0 {
    dev = 0i32 as u16;
    printf(b"ATA device, with \x00" as *const u8 as *const libc::c_char);
  } else if *val.offset(0) as libc::c_int == 0x848ai32 {
    dev = 0i32 as u16;
    like_std = 4i32 as u16;
    printf(b"CompactFlash ATA device, with \x00" as *const u8 as *const libc::c_char);
  } else if *val.offset(0) as libc::c_int & 0x4000i32 == 0 {
    dev = 0x1i32 as u16;
    eqpt = ((*val.offset(0) as libc::c_int & 0x1f00i32) >> 8i32) as u16;
    printf(
      b"ATAPI %s, with \x00" as *const u8 as *const libc::c_char,
      if eqpt as libc::c_int <= 0xfi32 {
        nth_string(pkt_str.as_ptr(), eqpt as libc::c_int)
      } else {
        b"unknown\x00" as *const u8 as *const libc::c_char
      },
    );
    like_std = 3i32 as u16
  } else {
    /* "Unknown device type:\n\tbits 15&14 of general configuration word 0 both set to 1.\n" */
    bb_simple_error_msg_and_die(b"unknown device type\x00" as *const u8 as *const libc::c_char);
  }
  printf(
    b"%sremovable media\n\x00" as *const u8 as *const libc::c_char,
    if *val.offset(0) as libc::c_int & 0x80i32 == 0 {
      b"non-\x00" as *const u8 as *const libc::c_char
    } else {
      b"\x00" as *const u8 as *const libc::c_char
    },
  );
  /* Info from the specific configuration word says whether or not the
   * ID command completed correctly.  It is only defined, however in
   * ATA/ATAPI-5 & 6; it is reserved (value theoretically 0) in prior
   * standards.  Since the values allowed for this word are extremely
   * specific, it should be safe to check it now, even though we don't
   * know yet what standard this device is using.
   */
  if *val.offset(2) as libc::c_int == 0x37c8i32
    || *val.offset(2) as libc::c_int == 0x738ci32
    || *val.offset(2) as libc::c_int == 0x8c73i32
    || *val.offset(2) as libc::c_int == 0xc837i32
  {
    like_std = 5i32 as u16;
    if *val.offset(2) as libc::c_int == 0x37c8i32 || *val.offset(2) as libc::c_int == 0x738ci32 {
      puts(
        b"powers-up in standby; SET FEATURES subcmd spins-up.\x00" as *const u8
          as *const libc::c_char,
      );
    }
    if (*val.offset(2) as libc::c_int == 0x37c8i32 || *val.offset(2) as libc::c_int == 0x8c73i32)
      && *val.offset(0) as libc::c_int & 0x4i32 != 0
    {
      puts(
        b"\n\tWARNING: ID response incomplete.\n\tFollowing data may be incorrect.\n\x00"
          as *const u8 as *const libc::c_char,
      );
    }
  }
  /* output the model and serial numbers and the fw revision */
  xprint_ascii(
    val,
    27i32,
    b"Model Number:\x00" as *const u8 as *const libc::c_char,
    20i32,
  );
  xprint_ascii(
    val,
    10i32,
    b"Serial Number:\x00" as *const u8 as *const libc::c_char,
    10i32,
  );
  xprint_ascii(
    val,
    23i32,
    b"Firmware Revision:\x00" as *const u8 as *const libc::c_char,
    4i32,
  );
  xprint_ascii(
    val,
    176i32,
    b"Media Serial Num:\x00" as *const u8 as *const libc::c_char,
    20i32,
  );
  xprint_ascii(
    val,
    196i32,
    b"Media Manufacturer:\x00" as *const u8 as *const libc::c_char,
    10i32,
  );
  /* major & minor standards version number (Note: these words were not
   * defined until ATA-3 & the CDROM std uses different words.) */
  printf(b"Standards:\x00" as *const u8 as *const libc::c_char);
  if eqpt as libc::c_int != 0x5i32 {
    if *val.offset(81) as libc::c_int != 0 && *val.offset(81) as libc::c_int <= 0x22i32 {
      if (like_std as libc::c_int) < 3i32 {
        like_std = 3i32 as u16
      }
      std = actual_ver[*val.offset(81) as usize] as u16;
      if std != 0 {
        printf(
          b"\n\tUsed: %s \x00" as *const u8 as *const libc::c_char,
          nth_string(minor_str.as_ptr(), *val.offset(81) as libc::c_int),
        );
      }
    }
    /* looks like when they up-issue the std, they obsolete one;
     * thus, only the newest 4 issues need be supported. (That's
     * what "kk" and "min_std" are all about.) */
    if *val.offset(80) as libc::c_int != 0 && *val.offset(80) as libc::c_int != 0xffffi32 {
      printf(b"\n\tSupported: \x00" as *const u8 as *const libc::c_char);
      jj = ((*val.offset(80) as libc::c_int) << 1i32) as u16;
      kk = if like_std as libc::c_int > 4i32 {
        (like_std as libc::c_int) - 4i32
      } else {
        0i32
      } as u16;
      ii = 14i32 as u16;
      while ii as libc::c_int > 0i32 && ii as libc::c_int > kk as libc::c_int {
        if jj as libc::c_int & 0x8000i32 != 0 {
          printf(
            b"%u \x00" as *const u8 as *const libc::c_char,
            ii as libc::c_int,
          );
          if (like_std as libc::c_int) < ii as libc::c_int {
            like_std = ii;
            kk = if like_std as libc::c_int > 4i32 {
              (like_std as libc::c_int) - 4i32
            } else {
              0i32
            } as u16
          }
          if min_std as libc::c_int > ii as libc::c_int {
            min_std = ii
          }
        }
        jj = ((jj as libc::c_int) << 1i32) as u16;
        ii = ii.wrapping_sub(1)
      }
      if (like_std as libc::c_int) < 3i32 {
        like_std = 3i32 as u16
      }
    }
    /* Figure out what standard the device is using if it hasn't told
     * us.  If we know the std, check if the device is using any of
     * the words from the next level up.  It happens.
     */
    if (like_std as libc::c_int) < std as libc::c_int {
      like_std = std
    }
    if (std as libc::c_int == 5i32 || std == 0 && (like_std as libc::c_int) < 6i32)
      && (*val.offset(83) as libc::c_int & 0xc000i32 == 0x4000i32
        && *val.offset(83) as libc::c_int & 0x3fffi32 > 0xffi32
        || *val.offset(84) as libc::c_int & 0xc000i32 == 0x4000i32
          && *val.offset(84) as libc::c_int & 0x2fi32 != 0)
    {
      like_std = 6i32 as u16
    } else if (std as libc::c_int == 4i32 || std == 0 && (like_std as libc::c_int) < 5i32)
      && (*val.offset(255) as libc::c_int & 0xffi32 == 0xa5i32 && chksum == 0
        || *val.offset(93) as libc::c_int & 0xc000i32 == 0x4000i32
        || *val.offset(83) as libc::c_int & 0xc000i32 == 0x4000i32
          && *val.offset(83) as libc::c_int & 0x3fffi32 > 0x1fi32)
    {
      like_std = 5i32 as u16
    } else if (std as libc::c_int == 3i32 || std == 0 && (like_std as libc::c_int) < 4i32)
      && (*val.offset(83) as libc::c_int & 0xc000i32 == 0x4000i32
        && (*val.offset(83) as libc::c_int & 0x3fffi32 > 0i32
          || *val.offset(82) as libc::c_int & 0x77ffi32 > 0xfi32)
        || *val.offset(50) as libc::c_int & 0xc000i32 == 0x4000i32
        || *val.offset(53) as libc::c_int & 0x4i32 != 0 && *val.offset(88) as libc::c_int != 0
        || *val.offset(127) as libc::c_int & 0x3i32 == 0x1i32)
    {
      like_std = 4i32 as u16
    } else if (std as libc::c_int == 2i32 || std == 0 && (like_std as libc::c_int) < 3i32)
      && *val.offset(83) as libc::c_int & 0xc000i32 == 0x4000i32
    {
      like_std = 3i32 as u16
    } else if (std as libc::c_int == 1i32 || std == 0 && (like_std as libc::c_int) < 2i32)
      && (*val.offset(49) as libc::c_int & (0x800i32 | 0x400i32) != 0
        || *val.offset(53) as libc::c_int & 0x2i32 != 0)
    {
      like_std = 2i32 as u16
    }
    if std == 0 {
      printf(
        b"\n\tLikely used: %u\n\x00" as *const u8 as *const libc::c_char,
        like_std as libc::c_int,
      );
    } else if like_std as libc::c_int > std as libc::c_int {
      printf(
        b"& some of %u\n\x00" as *const u8 as *const libc::c_char,
        like_std as libc::c_int,
      );
    } else {
      bb_putchar('\n' as i32);
    }
  } else {
    /* TBD: do CDROM stuff more thoroughly.  For now... */
    kk = 0i32 as u16;
    if *val.offset(74) as libc::c_int == 9i32 {
      kk = 1i32 as u16;
      printf(
        b"\n\tUsed: ATAPI for CD-ROMs, SFF-8020i, r2.5\x00" as *const u8 as *const libc::c_char,
      );
    }
    if *val.offset(73) as libc::c_int != 0 && *val.offset(73) as libc::c_int != 0xffffi32 {
      kk = 1i32 as u16;
      printf(b"\n\tSupported: CD-ROM ATAPI\x00" as *const u8 as *const libc::c_char);
      jj = (*val.offset(73) as libc::c_int >> 1i32) as u16;
      ii = 1i32 as u16;
      while (ii as libc::c_int) < 15i32 {
        if jj as libc::c_int & 0x1i32 != 0 {
          printf(
            b"-%u \x00" as *const u8 as *const libc::c_char,
            ii as libc::c_int,
          );
        }
        jj = (jj as libc::c_int >> 1i32) as u16;
        ii = ii.wrapping_add(1)
      }
    }
    puts(if kk as libc::c_int != 0 {
      b"\x00" as *const u8 as *const libc::c_char
    } else {
      b"\n\tLikely used CD-ROM ATAPI-1\x00" as *const u8 as *const libc::c_char
    });
    /* the cdrom stuff is more like ATA-2 than anything else, so: */
    like_std = 2i32 as u16
  }
  if min_std as libc::c_int == 0xffffi32 {
    min_std = if like_std as libc::c_int > 4i32 {
      (like_std as libc::c_int) - 3i32
    } else {
      1i32
    } as u16
  }
  puts(b"Configuration:\x00" as *const u8 as *const libc::c_char);
  /* more info from the general configuration word */
  if eqpt as libc::c_int != 0x5i32 && like_std as libc::c_int == 1i32 {
    jj = (*val.offset(0) as libc::c_int >> 1i32) as u16; /* Data Request (DRQ) */
    ii = 1i32 as u16;
    while (ii as libc::c_int) < 15i32 {
      if jj as libc::c_int & 0x1i32 != 0 {
        printf(
          b"\t%s\n\x00" as *const u8 as *const libc::c_char,
          nth_string(ata1_cfg_str.as_ptr(), ii as libc::c_int),
        );
      }
      jj = (jj as libc::c_int >> 1i32) as u16;
      ii = ii.wrapping_add(1)
    }
  }
  if dev as libc::c_int == 0x1i32 {
    if *val.offset(0) as libc::c_int & 0x60i32 == 0i32 {
      strng = b"3ms\x00" as *const u8 as *const libc::c_char
    } else if *val.offset(0) as libc::c_int & 0x60i32 == 0x20i32 {
      strng = b"<=10ms with INTRQ\x00" as *const u8 as *const libc::c_char
    } else if *val.offset(0) as libc::c_int & 0x60i32 == 0x40i32 {
      strng = b"50us\x00" as *const u8 as *const libc::c_char
    } else {
      strng = b"unknown\x00" as *const u8 as *const libc::c_char
    }
    printf(
      b"\tDRQ response: %s\n\tPacket size: \x00" as *const u8 as *const libc::c_char,
      strng,
    );
    if *val.offset(0) as libc::c_int & 0x3i32 == 0i32 {
      strng = b"12 bytes\x00" as *const u8 as *const libc::c_char
    } else if *val.offset(0) as libc::c_int & 0x3i32 == 0x1i32 {
      strng = b"16 bytes\x00" as *const u8 as *const libc::c_char
    } else {
      strng = b"unknown\x00" as *const u8 as *const libc::c_char
    }
    puts(strng);
  } else {
    /* addressing...CHS? See section 6.2 of ATA specs 4 or 5 */
    ll = (*val.offset(61) as u32) << 16i32 | *val.offset(60) as libc::c_uint;
    mm = 0i32 as u32;
    bbbig = 0i32 as u64;
    if ll > 0xfbfc10i32 as libc::c_uint && *val.offset(1) == 0 {
      puts(b"\tCHS addressing not supported\x00" as *const u8 as *const libc::c_char);
    } else {
      jj = (*val.offset(53) as libc::c_int & 0x1i32) as u16;
      printf(b"\tLogical\t\tmax\tcurrent\n\tcylinders\t%u\t%u\n\theads\t\t%u\t%u\n\tsectors/track\t%u\t%u\n\t--\n\x00"
                       as *const u8 as *const libc::c_char,
                   *val.offset(1) as libc::c_int,
                   if jj as libc::c_int != 0 {
                       *val.offset(54) as libc::c_int
                   } else { 0i32 }, *val.offset(3) as libc::c_int,
                   if jj as libc::c_int != 0 {
                       *val.offset(55) as libc::c_int
                   } else { 0i32 }, *val.offset(6) as libc::c_int,
                   if jj as libc::c_int != 0 {
                       *val.offset(56) as libc::c_int
                   } else { 0i32 });
      if min_std as libc::c_int == 1i32
        && (*val.offset(4) as libc::c_int != 0 || *val.offset(5) as libc::c_int != 0)
      {
        printf(
          b"\tbytes/track: %u\tbytes/sector: %u\n\x00" as *const u8 as *const libc::c_char,
          *val.offset(4) as libc::c_int,
          *val.offset(5) as libc::c_int,
        );
      }
      if jj != 0 {
        mm = (*val.offset(58) as u32) << 16i32 | *val.offset(57) as libc::c_uint;
        if (like_std as libc::c_int) < 3i32 {
          /* check Endian of capacity bytes */
          nn = (*val.offset(54) as libc::c_int
            * *val.offset(55) as libc::c_int
            * *val.offset(56) as libc::c_int) as u32;
          oo = (*val.offset(57) as u32) << 16i32 | *val.offset(58) as libc::c_uint;
          if abs(mm.wrapping_sub(nn) as libc::c_int) > abs(oo.wrapping_sub(nn) as libc::c_int) {
            mm = oo
          }
        }
        printf(
          b"\tCHS current addressable sectors:%11u\n\x00" as *const u8 as *const libc::c_char,
          mm,
        );
      }
    }
    /* LBA addressing */
    printf(
      b"\tLBA    user addressable sectors:%11u\n\x00" as *const u8 as *const libc::c_char,
      ll,
    ); /* # 512 byte blocks */
    if *val.offset(83) as libc::c_int & 0xc000i32 == 0x4000i32
      && *val.offset(83) as libc::c_int & 0x400i32 != 0
    {
      bbbig = (*val.offset(103) as u64) << 48i32
        | (*val.offset(102) as u64) << 32i32
        | (*val.offset(101) as u64) << 16i32
        | *val.offset(100) as libc::c_ulong;
      printf(
        b"\tLBA48  user addressable sectors:%11lu\n\x00" as *const u8 as *const libc::c_char,
        bbbig,
      );
    }
    if bbbig == 0 {
      bbbig = if ll > mm { ll } else { mm } as u64
    }
    printf(
      b"\tdevice size with M = 1024*1024: %11lu MBytes\n\x00" as *const u8 as *const libc::c_char,
      bbbig >> 11i32,
    );
    bbbig = (bbbig << 9i32).wrapping_div(1000000i32 as libc::c_ulong);
    printf(
      b"\tdevice size with M = 1000*1000: %11lu MBytes \x00" as *const u8 as *const libc::c_char,
      bbbig,
    );
    if bbbig > 1000i32 as libc::c_ulong {
      printf(
        b"(%lu GB)\n\x00" as *const u8 as *const libc::c_char,
        bbbig.wrapping_div(1000i32 as libc::c_ulong),
      );
    } else {
      bb_putchar('\n' as i32);
    }
  }
  /* hw support of commands (capabilities) */
  printf(b"Capabilities:\n\t\x00" as *const u8 as *const libc::c_char);
  if dev as libc::c_int == 0x1i32 {
    if eqpt as libc::c_int != 0x5i32 && *val.offset(49) as libc::c_int & 0x4000i32 != 0 {
      printf(b"Cmd queuing, \x00" as *const u8 as *const libc::c_char);
    }
    if *val.offset(49) as libc::c_int & 0x2000i32 != 0 {
      printf(b"Cmd overlap, \x00" as *const u8 as *const libc::c_char);
    }
  }
  if *val.offset(49) as libc::c_int & 0x200i32 != 0 {
    printf(b"LBA, \x00" as *const u8 as *const libc::c_char);
  }
  if like_std as libc::c_int != 1i32 {
    printf(
      b"IORDY%s(can%s be disabled)\n\x00" as *const u8 as *const libc::c_char,
      if *val.offset(49) as libc::c_int & 0x800i32 == 0 {
        b"(may be)\x00" as *const u8 as *const libc::c_char
      } else {
        b"\x00" as *const u8 as *const libc::c_char
      },
      if *val.offset(49) as libc::c_int & 0x400i32 != 0 {
        b"\x00" as *const u8 as *const libc::c_char
      } else {
        b"not\x00" as *const u8 as *const libc::c_char
      },
    );
  } else {
    puts(b"no IORDY\x00" as *const u8 as *const libc::c_char);
  }
  if like_std as libc::c_int == 1i32 && *val.offset(20) as libc::c_int != 0 {
    printf(
      b"\tBuffer type: %04x: %s%s\n\x00" as *const u8 as *const libc::c_char,
      *val.offset(20) as libc::c_int,
      if (*val.offset(20) as libc::c_int) < 2i32 {
        b"single port, single-sector\x00" as *const u8 as *const libc::c_char
      } else {
        b"dual port, multi-sector\x00" as *const u8 as *const libc::c_char
      },
      if *val.offset(20) as libc::c_int > 2i32 {
        b" with read caching ability\x00" as *const u8 as *const libc::c_char
      } else {
        b"\x00" as *const u8 as *const libc::c_char
      },
    );
  }
  if min_std as libc::c_int == 1i32
    && (*val.offset(21) as libc::c_int != 0 && *val.offset(21) as libc::c_int != 0xffffi32)
  {
    printf(
      b"\tBuffer size: %.1fkB\n\x00" as *const u8 as *const libc::c_char,
      (*val.offset(21) as libc::c_float / 2i32 as libc::c_float) as libc::c_double,
    );
  }
  if (min_std as libc::c_int) < 4i32 && *val.offset(22) as libc::c_int != 0 {
    printf(
      b"\tbytes avail on r/w long: %u\n\x00" as *const u8 as *const libc::c_char,
      *val.offset(22) as libc::c_int,
    );
  }
  if eqpt as libc::c_int != 0x5i32 && like_std as libc::c_int > 3i32 {
    printf(
      b"\tQueue depth: %u\n\x00" as *const u8 as *const libc::c_char,
      (*val.offset(75) as libc::c_int & 0x1fi32) + 1i32,
    );
  }
  if dev as libc::c_int == 0i32 {
    if like_std as libc::c_int == 1i32 {
      printf(
        b"\tCan%s perform double-word IO\n\x00" as *const u8 as *const libc::c_char,
        if *val.offset(48) == 0 {
          b"not\x00" as *const u8 as *const libc::c_char
        } else {
          b"\x00" as *const u8 as *const libc::c_char
        },
      );
    } else {
      printf(
        b"\tStandby timer values: spec\'d by %s\x00" as *const u8 as *const libc::c_char,
        if *val.offset(49) as libc::c_int & 0x2000i32 != 0 {
          b"standard\x00" as *const u8 as *const libc::c_char
        } else {
          b"vendor\x00" as *const u8 as *const libc::c_char
        },
      );
      if like_std as libc::c_int > 3i32 && *val.offset(50) as libc::c_int & 0xc000i32 == 0x4000i32 {
        printf(
          b", %s device specific minimum\n\x00" as *const u8 as *const libc::c_char,
          if *val.offset(50) as libc::c_int & 0x1i32 != 0 {
            b"with\x00" as *const u8 as *const libc::c_char
          } else {
            b"no\x00" as *const u8 as *const libc::c_char
          },
        );
      } else {
        bb_putchar('\n' as i32);
      }
    }
    printf(b"\tR/W multiple sector transfer: \x00" as *const u8 as *const libc::c_char);
    if (like_std as libc::c_int) < 3i32 && *val.offset(47) as libc::c_int & 0xffi32 == 0 {
      puts(b"not supported\x00" as *const u8 as *const libc::c_char);
    } else {
      printf(
        b"Max = %u\tCurrent = \x00" as *const u8 as *const libc::c_char,
        *val.offset(47) as libc::c_int & 0xffi32,
      );
      if *val.offset(59) as libc::c_int & 0x100i32 != 0 {
        printf(
          b"%u\n\x00" as *const u8 as *const libc::c_char,
          *val.offset(59) as libc::c_int & 0xffi32,
        );
      } else {
        puts(b"?\x00" as *const u8 as *const libc::c_char);
      }
    }
    if like_std as libc::c_int > 3i32 && *val.offset(83) as libc::c_int & 0x8i32 != 0 {
      /* We print out elsewhere whether the APM feature is enabled or
       * not.  If it's not enabled, let's not repeat the info; just print
       * nothing here. */
      printf(b"\tAdvancedPM level: \x00" as *const u8 as *const libc::c_char);
      if *val.offset(91) as libc::c_int & 0xff00i32 == 0x4000i32 {
        let mut apm_level: u8 = (*val.offset(91) as libc::c_int & 0xffi32) as u8;
        printf(
          b"%u (0x%x)\n\x00" as *const u8 as *const libc::c_char,
          apm_level as libc::c_int,
          apm_level as libc::c_int,
        );
      } else {
        printf(
          b"unknown setting (0x%04x)\n\x00" as *const u8 as *const libc::c_char,
          *val.offset(91) as libc::c_int,
        );
      }
    }
    if like_std as libc::c_int > 5i32 && *val.offset(94) as libc::c_int != 0 {
      printf(
        b"\tRecommended acoustic management value: %u, current value: %u\n\x00" as *const u8
          as *const libc::c_char,
        *val.offset(94) as libc::c_int >> 8i32 & 0xffi32,
        *val.offset(94) as libc::c_int & 0xffi32,
      );
    }
  } else {
    /* ATAPI */
    if eqpt as libc::c_int != 0x5i32 && *val.offset(49) as libc::c_int & 0x1000i32 != 0 {
      puts(b"\tATA sw reset required\x00" as *const u8 as *const libc::c_char);
    }
    if *val.offset(71) as libc::c_int != 0 || *val.offset(72) as libc::c_int != 0 {
      printf(b"\tOverlap support:\x00" as *const u8 as *const libc::c_char);
      if *val.offset(71) != 0 {
        printf(
          b" %uus to release bus.\x00" as *const u8 as *const libc::c_char,
          *val.offset(71) as libc::c_int,
        );
      }
      if *val.offset(72) != 0 {
        printf(
          b" %uus to clear BSY after SERVICE cmd.\x00" as *const u8 as *const libc::c_char,
          *val.offset(72) as libc::c_int,
        );
      }
      bb_putchar('\n' as i32);
    }
  }
  /* DMA stuff. Check that only one DMA mode is selected. */
  printf(b"\tDMA: \x00" as *const u8 as *const libc::c_char);
  if *val.offset(49) as libc::c_int & 0x100i32 == 0 {
    puts(b"not supported\x00" as *const u8 as *const libc::c_char);
  } else {
    if *val.offset(52) as libc::c_int != 0 && *val.offset(62) == 0 && *val.offset(63) == 0 {
      printf(
        b" sdma%u\n\x00" as *const u8 as *const libc::c_char,
        (*val.offset(52) as libc::c_int & 0xff00i32) >> 8i32,
      );
    }
    if *val.offset(62) != 0 {
      jj = *val.offset(62);
      kk = (*val.offset(62) as libc::c_int >> 8i32) as u16;
      err_dma = (err_dma as libc::c_int
        + mode_loop(jj, kk, 's' as i32, &mut have_mode) as libc::c_int) as u8
    }
    if *val.offset(63) != 0 {
      jj = *val.offset(63);
      kk = (*val.offset(63) as libc::c_int >> 8i32) as u16;
      err_dma = (err_dma as libc::c_int
        + mode_loop(jj, kk, 'm' as i32, &mut have_mode) as libc::c_int) as u8
    }
    if *val.offset(53) as libc::c_int & 0x4i32 != 0 && *val.offset(88) as libc::c_int != 0 {
      jj = *val.offset(88);
      kk = (*val.offset(88) as libc::c_int >> 8i32) as u16;
      err_dma = (err_dma as libc::c_int
        + mode_loop(jj, kk, 'u' as i32, &mut have_mode) as libc::c_int) as u8
    }
    if err_dma as libc::c_int != 0 || have_mode == 0 {
      printf(b"(?)\x00" as *const u8 as *const libc::c_char);
    }
    bb_putchar('\n' as i32);
    if dev as libc::c_int == 0x1i32
      && eqpt as libc::c_int != 0x5i32
      && *val.offset(49) as libc::c_int & 0x8000i32 != 0
    {
      puts(b"\t\tInterleaved DMA support\x00" as *const u8 as *const libc::c_char);
    }
    if *val.offset(53) as libc::c_int & 0x2i32 != 0
      && (*val.offset(65) as libc::c_int != 0 || *val.offset(66) as libc::c_int != 0)
    {
      printf(b"\t\tCycle time:\x00" as *const u8 as *const libc::c_char);
      if *val.offset(65) != 0 {
        printf(
          b" min=%uns\x00" as *const u8 as *const libc::c_char,
          *val.offset(65) as libc::c_int,
        );
      }
      if *val.offset(66) != 0 {
        printf(
          b" recommended=%uns\x00" as *const u8 as *const libc::c_char,
          *val.offset(66) as libc::c_int,
        );
      }
      bb_putchar('\n' as i32);
    }
  }
  /* Programmed IO stuff */
  printf(b"\tPIO: \x00" as *const u8 as *const libc::c_char);
  /* If a drive supports mode n (e.g. 3), it also supports all modes less
   * than n (e.g. 3, 2, 1 and 0).  Print all the modes. */
  if *val.offset(53) as libc::c_int & 0x2i32 != 0 && *val.offset(64) as libc::c_int & 0xffi32 != 0 {
    jj = ((*val.offset(64) as libc::c_int & 0xffi32) << 3i32 | 0x7i32) as u16;
    ii = 0i32 as u16;
    while ii as libc::c_int <= 8i32 {
      if jj as libc::c_int & 0x1i32 != 0 {
        printf(
          b"pio%d \x00" as *const u8 as *const libc::c_char,
          ii as libc::c_int,
        );
      }
      jj = (jj as libc::c_int >> 1i32) as u16;
      ii = ii.wrapping_add(1)
    }
    bb_putchar('\n' as i32);
  } else if ((min_std as libc::c_int) < 5i32 || eqpt as libc::c_int == 0x5i32)
    && *val.offset(51) as libc::c_int & 0xff00i32 != 0
  {
    ii = 0i32 as u16;
    while ii as libc::c_int <= *val.offset(51) as libc::c_int >> 8i32 {
      printf(
        b"pio%d \x00" as *const u8 as *const libc::c_char,
        ii as libc::c_int,
      );
      ii = ii.wrapping_add(1)
    }
    bb_putchar('\n' as i32);
  } else {
    puts(b"unknown\x00" as *const u8 as *const libc::c_char);
  }
  if *val.offset(53) as libc::c_int & 0x2i32 != 0 {
    if *val.offset(67) as libc::c_int != 0 || *val.offset(68) as libc::c_int != 0 {
      printf(b"\t\tCycle time:\x00" as *const u8 as *const libc::c_char);
      if *val.offset(67) != 0 {
        printf(
          b" no flow control=%uns\x00" as *const u8 as *const libc::c_char,
          *val.offset(67) as libc::c_int,
        );
      }
      if *val.offset(68) != 0 {
        printf(
          b"  IORDY flow control=%uns\x00" as *const u8 as *const libc::c_char,
          *val.offset(68) as libc::c_int,
        );
      }
      bb_putchar('\n' as i32);
    }
  }
  if *val.offset(83) as libc::c_int & 0xc000i32 == 0x4000i32 {
    puts(b"Commands/features:\n\tEnabled\tSupported:\x00" as *const u8 as *const libc::c_char);
    jj = *val.offset(82);
    kk = *val.offset(85);
    ii = 0i32 as u16;
    while (ii as libc::c_int) < 48i32 {
      let mut feat_str: *const libc::c_char = nth_string(cmd_feat_str.as_ptr(), ii as libc::c_int);
      if jj as libc::c_int & 0x8000i32 != 0 && *feat_str as libc::c_int != '\u{0}' as i32 {
        printf(
          b"\t%s\t%s\n\x00" as *const u8 as *const libc::c_char,
          if kk as libc::c_int & 0x8000i32 != 0 {
            b"   *\x00" as *const u8 as *const libc::c_char
          } else {
            b"\x00" as *const u8 as *const libc::c_char
          },
          feat_str,
        );
      }
      jj = ((jj as libc::c_int) << 1i32) as u16;
      kk = ((kk as libc::c_int) << 1i32) as u16;
      if ii as libc::c_int % 16i32 == 15i32 {
        jj = *val.offset((82i32 + 1i32 + ii as libc::c_int / 16i32) as isize);
        kk = *val.offset((85i32 + 1i32 + ii as libc::c_int / 16i32) as isize)
      }
      if ii as libc::c_int == 31i32 {
        if *val.offset(84) as libc::c_int & 0xc000i32 != 0x4000i32 {
          ii = (ii as libc::c_int + 16i32) as u16
        }
      }
      ii = ii.wrapping_add(1)
    }
  }
  /* Removable Media Status Notification feature set */
  if *val.offset(127) as libc::c_int & 0x3i32 == 0x1i32 {
    printf(
      b"\t%s supported\n\x00" as *const u8 as *const libc::c_char,
      nth_string(cmd_feat_str.as_ptr(), 27i32),
    );
  }
  /* security */
  if eqpt as libc::c_int != 0x5i32
    && like_std as libc::c_int > 3i32
    && (*val.offset(128) as libc::c_int != 0
      || *val.offset(89) as libc::c_int != 0
      || *val.offset(90) as libc::c_int != 0)
  {
    puts(b"Security:\x00" as *const u8 as *const libc::c_char);
    if *val.offset(92) as libc::c_int != 0 && *val.offset(92) as libc::c_int != 0xffffi32 {
      printf(
        b"\tMaster password revision code = %u\n\x00" as *const u8 as *const libc::c_char,
        *val.offset(92) as libc::c_int,
      );
    }
    jj = *val.offset(128);
    if jj != 0 {
      ii = 0i32 as u16;
      while (ii as libc::c_int) < 6i32 {
        printf(
          b"\t%s\t%s\n\x00" as *const u8 as *const libc::c_char,
          if jj as libc::c_int & 0x1i32 == 0 {
            b"not\x00" as *const u8 as *const libc::c_char
          } else {
            b"\x00" as *const u8 as *const libc::c_char
          },
          nth_string(secu_str.as_ptr(), ii as libc::c_int),
        );
        jj = (jj as libc::c_int >> 1i32) as u16;
        ii = ii.wrapping_add(1)
      }
      if *val.offset(128) as libc::c_int & 0x2i32 != 0 {
        printf(
          b"\tSecurity level %s\n\x00" as *const u8 as *const libc::c_char,
          if *val.offset(128) as libc::c_int & 0x10i32 != 0 {
            b"maximum\x00" as *const u8 as *const libc::c_char
          } else {
            b"high\x00" as *const u8 as *const libc::c_char
          },
        );
      }
    }
    jj = (*val.offset(89) as libc::c_int & 0xffi32) as u16;
    kk = (*val.offset(90) as libc::c_int & 0xffi32) as u16;
    if jj as libc::c_int != 0 || kk as libc::c_int != 0 {
      bb_putchar('\t' as i32);
      if jj != 0 {
        printf(
          b"%umin for %sSECURITY ERASE UNIT. \x00" as *const u8 as *const libc::c_char,
          if jj as libc::c_int == 0xffi32 {
            508i32
          } else {
            (jj as libc::c_int) << 1i32
          },
          b"\x00" as *const u8 as *const libc::c_char,
        );
      }
      if kk != 0 {
        printf(
          b"%umin for %sSECURITY ERASE UNIT. \x00" as *const u8 as *const libc::c_char,
          if kk as libc::c_int == 0xffi32 {
            508i32
          } else {
            (kk as libc::c_int) << 1i32
          },
          b"ENHANCED \x00" as *const u8 as *const libc::c_char,
        );
      }
      bb_putchar('\n' as i32);
    }
  }
  /* reset result */
  jj = *val.offset(93);
  if jj as libc::c_int & 0xc000i32 == 0x4000i32 {
    oo = (jj as libc::c_int & 0x1i32) as u32;
    if oo == 0 {
      jj = (jj as libc::c_int >> 8i32) as u16
    }
    if jj as libc::c_int & 0x6i32 == 0x2i32 {
      strng = b" determined by the jumper\x00" as *const u8 as *const libc::c_char
    } else if jj as libc::c_int & 0x6i32 == 0x4i32 {
      strng = b" determined by CSEL\x00" as *const u8 as *const libc::c_char
    } else {
      strng = b"\x00" as *const u8 as *const libc::c_char
    }
    printf(
      b"HW reset results:\n\tCBLID- %s Vih\n\tDevice num = %i%s\n\x00" as *const u8
        as *const libc::c_char,
      if *val.offset(93) as libc::c_int & 0x2000i32 != 0 {
        b"above\x00" as *const u8 as *const libc::c_char
      } else {
        b"below\x00" as *const u8 as *const libc::c_char
      },
      (oo == 0) as libc::c_int,
      strng,
    );
  }
  /* more stuff from std 5 */
  if like_std as libc::c_int > 4i32 && eqpt as libc::c_int != 0x5i32 {
    if *val.offset(160) as libc::c_int & 0x8000i32 != 0 {
      printf(
        b"CFA power mode 1:\n\t%s%s\n\x00" as *const u8 as *const libc::c_char,
        if *val.offset(160) as libc::c_int & 0x1000i32 != 0 {
          b"disabled\x00" as *const u8 as *const libc::c_char
        } else {
          b"enabled\x00" as *const u8 as *const libc::c_char
        },
        if *val.offset(160) as libc::c_int & 0x2000i32 != 0 {
          b" and required by some commands\x00" as *const u8 as *const libc::c_char
        } else {
          b"\x00" as *const u8 as *const libc::c_char
        },
      );
      if *val.offset(160) as libc::c_int & 0xfffi32 != 0 {
        printf(
          b"\tMaximum current = %uma\n\x00" as *const u8 as *const libc::c_char,
          *val.offset(160) as libc::c_int & 0xfffi32,
        );
      }
    }
    if *val.offset(255) as libc::c_int & 0xffi32 == 0xa5i32 {
      printf(
        b"Checksum: %scorrect\n\x00" as *const u8 as *const libc::c_char,
        if chksum as libc::c_int != 0 {
          b"in\x00" as *const u8 as *const libc::c_char
        } else {
          b"\x00" as *const u8 as *const libc::c_char
        },
      );
    }
  }
  exit(0i32);
}
// Historically, if there was no HDIO_OBSOLETE_IDENTITY, then
// then the HDIO_GET_IDENTITY only returned 142 bytes.
// Otherwise, HDIO_OBSOLETE_IDENTITY returns 142 bytes,
// and HDIO_GET_IDENTITY returns 512 bytes.  But the latest
// 2.5.xx kernels no longer define HDIO_OBSOLETE_IDENTITY
// (which they should, but they should just return -EINVAL).
//
// So.. we must now assume that HDIO_GET_IDENTITY returns 512 bytes.
// On a really old system, it will not, and we will be confused.
// Too bad, really.
static mut cfg_str: [libc::c_char; 146] = [
  0, 72, 97, 114, 100, 83, 101, 99, 116, 0, 83, 111, 102, 116, 83, 101, 99, 116, 0, 78, 111, 116,
  77, 70, 77, 0, 72, 100, 83, 119, 62, 49, 53, 117, 83, 101, 99, 0, 83, 112, 105, 110, 77, 111,
  116, 67, 116, 108, 0, 70, 105, 120, 101, 100, 0, 82, 101, 109, 111, 118, 101, 97, 98, 108, 101,
  0, 68, 84, 82, 60, 61, 53, 77, 98, 115, 0, 68, 84, 82, 62, 53, 77, 98, 115, 0, 68, 84, 82, 62,
  49, 48, 77, 98, 115, 0, 82, 111, 116, 83, 112, 100, 84, 111, 108, 62, 46, 53, 37, 0, 100, 83,
  116, 98, 79, 102, 102, 0, 84, 114, 107, 79, 102, 102, 0, 70, 109, 116, 71, 97, 112, 82, 101, 113,
  0, 110, 111, 110, 77, 97, 103, 110, 101, 116, 105, 99, 0,
];
static mut BuffType: [libc::c_char; 37] = [
  117, 110, 107, 110, 111, 119, 110, 0, 49, 83, 101, 99, 116, 0, 68, 117, 97, 108, 80, 111, 114,
  116, 0, 68, 117, 97, 108, 80, 111, 114, 116, 67, 97, 99, 104, 101, 0,
];
#[inline(never)]
unsafe extern "C" fn dump_identity(mut id: *const hd_driveid) {
  let mut i: libc::c_int = 0;
  let mut id_regs: *const libc::c_ushort = id as *const libc::c_void as *const libc::c_ushort;
  printf(
    b"\n Model=%.40s, FwRev=%.8s, SerialNo=%.20s\n Config={\x00" as *const u8
      as *const libc::c_char,
    (*id).model.as_ptr(),
    (*id).fw_rev.as_ptr(),
    (*id).serial_no.as_ptr(),
  );
  i = 0i32;
  while i <= 15i32 {
    if (*id).config as libc::c_int & 1i32 << i != 0 {
      printf(
        b" %s\x00" as *const u8 as *const libc::c_char,
        nth_string(cfg_str.as_ptr(), i),
      );
    }
    i += 1
  }
  printf(b" }\n RawCHS=%u/%u/%u, TrkSize=%u, SectSize=%u, ECCbytes=%u\n BuffType=(%u) %s, BuffSize=%ukB, MaxMultSect=%u\x00"
               as *const u8 as *const libc::c_char, (*id).cyls as libc::c_int,
           (*id).heads as libc::c_int, (*id).sectors as libc::c_int,
           (*id).track_bytes as libc::c_int,
           (*id).sector_bytes as libc::c_int, (*id).ecc_bytes as libc::c_int,
           (*id).buf_type as libc::c_int,
           nth_string(BuffType.as_ptr(),
                      if (*id).buf_type as libc::c_int > 3i32 {
                          0i32
                      } else { (*id).buf_type as libc::c_int }),
           (*id).buf_size as libc::c_int / 2i32,
           (*id).max_multsect as libc::c_int);
  if (*id).max_multsect != 0 {
    printf(b", MultSect=\x00" as *const u8 as *const libc::c_char);
    if (*id).multsect_valid as libc::c_int & 1i32 == 0 {
      printf(
        b"?%u?\x00" as *const u8 as *const libc::c_char,
        (*id).multsect as libc::c_int,
      );
    } else if (*id).multsect != 0 {
      printf(
        b"%u\x00" as *const u8 as *const libc::c_char,
        (*id).multsect as libc::c_int,
      );
    } else {
      printf(b"off\x00" as *const u8 as *const libc::c_char);
    }
  }
  bb_putchar('\n' as i32);
  if (*id).field_valid as libc::c_int & 1i32 == 0 {
    printf(b" (maybe):\x00" as *const u8 as *const libc::c_char);
  }
  printf(
    b" CurCHS=%u/%u/%u, CurSects=%lu, LBA=%s\x00" as *const u8 as *const libc::c_char,
    (*id).cur_cyls as libc::c_int,
    (*id).cur_heads as libc::c_int,
    (*id).cur_sectors as libc::c_int,
    if false {
      ((((*id).cur_capacity0 as libc::c_int) << 16i32) as libc::c_ulong)
        | (*id).cur_capacity1 as libc::c_ulong
    } else {
      ((((*id).cur_capacity1 as libc::c_int) << 16i32) as libc::c_ulong)
        | (*id).cur_capacity0 as libc::c_ulong
    },
    if (*id).capability as libc::c_int & 2i32 == 0i32 {
      b"no\x00" as *const u8 as *const libc::c_char
    } else {
      b"yes\x00" as *const u8 as *const libc::c_char
    },
  );
  if (*id).capability as libc::c_int & 2i32 != 0 {
    printf(
      b", LBAsects=%u\x00" as *const u8 as *const libc::c_char,
      (*id).lba_capacity,
    );
  }
  printf(
    b"\n IORDY=%s\x00" as *const u8 as *const libc::c_char,
    if (*id).capability as libc::c_int & 8i32 != 0 {
      if (*id).capability as libc::c_int & 4i32 != 0 {
        b"on/off\x00" as *const u8 as *const libc::c_char
      } else {
        b"yes\x00" as *const u8 as *const libc::c_char
      }
    } else {
      b"no\x00" as *const u8 as *const libc::c_char
    },
  );
  if ((*id).capability as libc::c_int & 8i32 != 0 || (*id).field_valid as libc::c_int & 2i32 != 0)
    && (*id).field_valid as libc::c_int & 2i32 != 0
  {
    printf(
      b", tPIO={min:%u,w/IORDY:%u}\x00" as *const u8 as *const libc::c_char,
      (*id).eide_pio as libc::c_int,
      (*id).eide_pio_iordy as libc::c_int,
    );
  }
  if (*id).capability as libc::c_int & 1i32 != 0 && (*id).field_valid as libc::c_int & 2i32 != 0 {
    printf(
      b", tDMA={min:%u,rec:%u}\x00" as *const u8 as *const libc::c_char,
      (*id).eide_dma_min as libc::c_int,
      (*id).eide_dma_time as libc::c_int,
    );
  }

  printf(b"\n PIO modes:  \x00" as *const u8 as *const libc::c_char);
  if (*id).tPIO as libc::c_int <= 5i32 {
    printf(b"pio0 \x00" as *const u8 as *const libc::c_char);
    if (*id).tPIO as libc::c_int >= 1i32 {
      printf(b"pio1 \x00" as *const u8 as *const libc::c_char);
    }
    if (*id).tPIO as libc::c_int >= 2i32 {
      printf(b"pio2 \x00" as *const u8 as *const libc::c_char);
    }
  }
  if (*id).field_valid as libc::c_int & 2i32 != 0 {
    // static mut pio_modes: masks_labels_t = {
    //   let mut init = masks_labels_t {
    //     labels: b"pio3 \x00pio4 \x00pio? \x00\x00" as *const u8 as *const libc::c_char,
    //     masks: [1i32, 2i32, !3i32],
    //   };
    //   init
    // };
    // print_flags(&pio_modes, (*id).eide_pio_modes as libc::c_int);

    static pio_modes_masks: [libc::c_int; 3] = [1i32, 2i32, !3i32];

    print_flags_separated(
      pio_modes_masks.as_ptr(),
      b"pio3 \x00pio4 \x00pio? \x00\x00" as *const u8 as *const libc::c_char,
      (*id).eide_pio_modes as libc::c_int,
      0 as *const libc::c_char,
    );
  }
  if (*id).capability as libc::c_int & 1i32 != 0 {
    if (*id).dma_1word as libc::c_int | (*id).dma_mword as libc::c_int != 0 {
      static mut dma_wmode_masks: [libc::c_int; 8] = [
        0x100i32, 1i32, 0x200i32, 2i32, 0x400i32, 4i32, 0xf800i32, 0xf8i32,
      ];
      printf(b"\n DMA modes:  \x00" as *const u8 as *const libc::c_char);
      print_flags_separated(
        dma_wmode_masks.as_ptr(),
        b"*\x00sdma0 \x00*\x00sdma1 \x00*\x00sdma2 \x00*\x00sdma? \x00\x00" as *const u8
          as *const libc::c_char,
        (*id).dma_1word as libc::c_int,
        0 as *const libc::c_char,
      );
      print_flags_separated(
        dma_wmode_masks.as_ptr(),
        b"*\x00mdma0 \x00*\x00mdma1 \x00*\x00mdma2 \x00*\x00mdma? \x00\x00" as *const u8
          as *const libc::c_char,
        (*id).dma_mword as libc::c_int,
        0 as *const libc::c_char,
      );
    }
  }
  if ((*id).capability as libc::c_int & 8i32 != 0 || (*id).field_valid as libc::c_int & 2i32 != 0)
    && (*id).field_valid as libc::c_int & 4i32 != 0
  {
    printf(b"\n UDMA modes: \x00" as *const u8 as *const libc::c_char);

    // static mut ultra_modes1: masks_labels_t = {
    //   let mut init = masks_labels_t {
    //     labels: b"*\x00udma0 \x00*\x00udma1 \x00*\x00udma2 \x00\x00" as *const u8
    //       as *const libc::c_char,
    //     masks: [0x100i32, 0x1i32, 0x200i32, 0x2i32, 0x400i32, 0x4i32],
    //   };
    //   init
    // };
    // print_flags(&ultra_modes1, (*id).dma_ultra as libc::c_int);

    static ultra_modes1_masks: [libc::c_int; 6] =
      [0x100i32, 0x1i32, 0x200i32, 0x2i32, 0x400i32, 0x4i32];
    print_flags_separated(
      ultra_modes1_masks.as_ptr(),
      b"*\x00udma0 \x00*\x00udma1 \x00*\x00udma2 \x00\x00" as *const u8 as *const libc::c_char,
      (*id).dma_ultra as libc::c_int,
      0 as *const libc::c_char,
    );

    if (*id).hw_config as libc::c_int & 0x2000i32 != 0 {
      /* !__NEW_HD_DRIVE_ID */
      /* __NEW_HD_DRIVE_ID */
      //   static mut ultra_modes2: masks_labels_t = {
      //     let mut init = masks_labels_t {
      //   labels: b"*\x00udma3 \x00*\x00udma4 \x00*\x00udma5 \x00*\x00udma6 \x00*\x00udma7 \x00\x00"
      //     as *const u8 as *const libc::c_char,
      // masks: [
      //   0x800i32, 0x8i32, 0x1000i32, 0x10i32, 0x2000i32, 0x20i32, 0x4000i32, 0x40i32,
      //   0x8000i32, 0x80i32,
      // ],
      //     };
      //     init
      //   };
      // print_flags(&ultra_modes2, (*id).dma_ultra as libc::c_int);

      static ultra_modes2_masks: [libc::c_int; 10] = [
        0x800i32, 0x8i32, 0x1000i32, 0x10i32, 0x2000i32, 0x20i32, 0x4000i32, 0x40i32, 0x8000i32,
        0x80i32,
      ];
      print_flags_separated(
        ultra_modes2_masks.as_ptr(),
        b"*\x00udma3 \x00*\x00udma4 \x00*\x00udma5 \x00*\x00udma6 \x00*\x00udma7 \x00\x00"
          as *const u8 as *const libc::c_char,
        (*id).dma_ultra as libc::c_int,
        0 as *const libc::c_char,
      );
    }
  }
  printf(
    b"\n AdvancedPM=%s\x00" as *const u8 as *const libc::c_char,
    if *id_regs.offset(83) as libc::c_int & 8i32 == 0 {
      b"no\x00" as *const u8 as *const libc::c_char
    } else {
      b"yes\x00" as *const u8 as *const libc::c_char
    },
  );
  if *id_regs.offset(83) as libc::c_int & 8i32 != 0 {
    if *id_regs.offset(86) as libc::c_int & 8i32 == 0 {
      printf(b": disabled (255)\x00" as *const u8 as *const libc::c_char);
    } else if *id_regs.offset(91) as libc::c_int & 0xff00i32 != 0x4000i32 {
      printf(b": unknown setting\x00" as *const u8 as *const libc::c_char);
    } else {
      printf(
        b": mode=0x%02X (%u)\x00" as *const u8 as *const libc::c_char,
        *id_regs.offset(91) as libc::c_int & 0xffi32,
        *id_regs.offset(91) as libc::c_int & 0xffi32,
      );
    }
  }
  if *id_regs.offset(82) as libc::c_int & 0x20i32 != 0 {
    printf(
      b" WriteCache=%s\x00" as *const u8 as *const libc::c_char,
      if *id_regs.offset(85) as libc::c_int & 0x20i32 != 0 {
        b"enabled\x00" as *const u8 as *const libc::c_char
      } else {
        b"disabled\x00" as *const u8 as *const libc::c_char
      },
    );
  }
  if (*id).minor_rev_num as libc::c_int != 0 && (*id).minor_rev_num as libc::c_int <= 31i32
    || (*id).major_rev_num as libc::c_int != 0 && (*id).minor_rev_num as libc::c_int <= 31i32
  {
    printf(
      b"\n Drive conforms to: %s: \x00" as *const u8 as *const libc::c_char,
      if (*id).minor_rev_num as libc::c_int <= 31i32 {
        nth_string(minor_str.as_ptr(), (*id).minor_rev_num as libc::c_int)
      } else {
        b"unknown\x00" as *const u8 as *const libc::c_char
      },
    );
    if (*id).major_rev_num as libc::c_int != 0i32 && (*id).major_rev_num as libc::c_int != 0xffffi32
    {
      /* NOVAL_1 */
      i = 0i32;
      while i <= 15i32 {
        if (*id).major_rev_num as libc::c_int & 1i32 << i != 0 {
          printf(b" ATA/ATAPI-%u\x00" as *const u8 as *const libc::c_char, i);
        }
        i += 1
      }
    }
  }
  /* __NEW_HD_DRIVE_ID */
  puts(b"\n\n * current active mode\n\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn flush_buffer_cache()
/*int fd*/
{
  fsync(fd as libc::c_int); /* flush buffers */
  bb_ioctl_or_warn(
    fd as libc::c_int,
    0u32 << 0i32 + 8i32 + 8i32 + 14i32
      | (0x12i32 << 0i32 + 8i32) as libc::c_uint
      | (97i32 << 0i32) as libc::c_uint
      | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint,
    0 as *mut libc::c_void,
    b"BLKFLSBUF\x00" as *const u8 as *const libc::c_char,
  ); /* do it again, big time */
  sleep(1i32 as libc::c_uint);
  if ioctl(
    fd as libc::c_int,
    0x31fi32 as libc::c_ulong,
    0 as *mut libc::c_void,
  ) != 0
    && *bb_errno != 22i32
  {
    /* await completion */
    /* To be coherent with ioctl_or_warn */
    bb_simple_perror_msg(b"HDIO_DRIVE_CMD\x00" as *const u8 as *const libc::c_char);
  };
}
unsafe extern "C" fn seek_to_zero()
/*int fd*/
{
  xlseek(fd as libc::c_int, 0i32 as off_t, 0i32);
}
unsafe extern "C" fn read_big_block(mut buf: *mut libc::c_char) {
  let mut i: libc::c_int = 0;
  xread(
    fd as libc::c_int,
    buf as *mut libc::c_void,
    (1i32 * 1024i32 * 1024i32) as size_t,
  );
  /* access all sectors of buf to ensure the read fully completed */
  i = 0i32;
  while i < 1i32 * 1024i32 * 1024i32 {
    let ref mut fresh0 = *buf.offset(i as isize);
    *fresh0 = (*fresh0 as libc::c_int & 1i32) as libc::c_char;
    i += 512i32
  }
}
unsafe extern "C" fn dev_size_mb() -> libc::c_uint
/*int fd*/ {
  let mut u: C2RustUnnamed_1 = C2RustUnnamed_1 { blksize64: 0 };
  if 0i32
    == ioctl(
      fd as libc::c_int,
      (2u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (0x12i32 << 0i32 + 8i32) as libc::c_uint
        | (114i32 << 0i32) as libc::c_uint) as libc::c_ulong
        | (::std::mem::size_of::<size_t>() as libc::c_ulong) << 0i32 + 8i32 + 8i32,
      &mut u.blksize64 as *mut libc::c_ulonglong,
    )
  {
    // bytes
    u.blksize64 = u
      .blksize64
      .wrapping_div((1024i32 * 1024i32) as libc::c_ulonglong)
  } else {
    bb_xioctl(
      fd as libc::c_int,
      0u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (0x12i32 << 0i32 + 8i32) as libc::c_uint
        | (96i32 << 0i32) as libc::c_uint
        | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint,
      &mut u.blksize32 as *mut libc::c_uint as *mut libc::c_void,
      b"BLKGETSIZE\x00" as *const u8 as *const libc::c_char,
    ); // sectors
    u.blksize64 = u.blksize32.wrapping_div((2i32 * 1024i32) as libc::c_uint) as libc::c_ulonglong
  }
  if u.blksize64
    > (2147483647i32 as libc::c_uint)
      .wrapping_mul(2u32)
      .wrapping_add(1u32) as libc::c_ulonglong
  {
    return (2147483647i32 as libc::c_uint)
      .wrapping_mul(2u32)
      .wrapping_add(1u32);
  }
  return u.blksize64 as libc::c_uint;
}
unsafe extern "C" fn print_timing(mut m: libc::c_uint, mut elapsed_us: libc::c_uint) {
  let mut sec: libc::c_uint = elapsed_us.wrapping_div(1000000i32 as libc::c_uint);
  let mut hs: libc::c_uint = elapsed_us
    .wrapping_rem(1000000i32 as libc::c_uint)
    .wrapping_div(10000i32 as libc::c_uint);
  printf(
    b"%5u MB in %u.%02u seconds = %u kB/s\n\x00" as *const u8 as *const libc::c_char,
    m,
    sec,
    hs,
    (m as libc::c_ulonglong)
      .wrapping_mul((1024i32 * 1000000i32) as libc::c_ulonglong)
      .wrapping_div((elapsed_us | 1i32 as libc::c_uint) as libc::c_ulonglong) as libc::c_uint,
  );
}
unsafe extern "C" fn do_time(mut cache: libc::c_int)
/*,int fd*/
/* cache=1: time cache: repeatedly read N MB at offset 0
 * cache=0: time device: linear read, starting at offset 0
 */
{
  let mut max_iterations: libc::c_uint = 0; /* doesn't need to be long long */
  let mut iterations: libc::c_uint = 0;
  let mut start: libc::c_uint = 0;
  let mut elapsed: libc::c_uint = 0;
  let mut elapsed2: libc::c_uint = 0;
  let mut total_MB: libc::c_uint = 0;
  let mut buf: *mut libc::c_char =
    xmalloc((1i32 * 1024i32 * 1024i32) as size_t) as *mut libc::c_char;
  if mlock(
    buf as *const libc::c_void,
    (1i32 * 1024i32 * 1024i32) as size_t,
  ) != 0
  {
    bb_simple_perror_msg_and_die(b"mlock\x00" as *const u8 as *const libc::c_char);
  }
  /* Clear out the device request queues & give them time to complete.
   * NB: *small* delay. User is expected to have a clue and to not run
   * heavy io in parallel with measurements. */
  sync(); /* Time device */
  sleep(1i32 as libc::c_uint);
  if cache != 0 {
    /* Time cache */
    seek_to_zero();
    read_big_block(buf);
    printf(b"Timing buffer-cache reads: \x00" as *const u8 as *const libc::c_char);
  } else {
    printf(b"Timing buffered disk reads:\x00" as *const u8 as *const libc::c_char);
  }
  fflush_all();
  /* Now do the timing */
  iterations = 0i32 as libc::c_uint;
  /* Max time to run (small for cache, avoids getting
   * huge total_MB which can overlow unsigned type) */
  elapsed2 = 510000i32 as libc::c_uint; /* cache */
  max_iterations = (2147483647i32 as libc::c_uint)
    .wrapping_mul(2u32)
    .wrapping_add(1u32); /* not cache */
  if cache == 0 {
    elapsed2 = 3000000i32 as libc::c_uint;
    /* Don't want to read past the end! */
    max_iterations = dev_size_mb().wrapping_div(1i32 as libc::c_uint)
  }
  start = monotonic_us() as libc::c_uint;
  loop {
    if cache != 0 {
      seek_to_zero();
    }
    read_big_block(buf);
    elapsed = (monotonic_us() as libc::c_uint).wrapping_sub(start);
    iterations = iterations.wrapping_add(1);
    if !(elapsed < elapsed2 && iterations < max_iterations) {
      break;
    }
  }
  total_MB = iterations.wrapping_mul(1i32 as libc::c_uint);
  //printf(" elapsed:%u iterations:%u ", elapsed, iterations);
  if cache != 0 {
    /* Cache: remove lseek() and monotonic_us() overheads
     * from elapsed */
    start = monotonic_us() as libc::c_uint;
    loop {
      seek_to_zero();
      elapsed2 = (monotonic_us() as libc::c_uint).wrapping_sub(start);
      iterations = iterations.wrapping_sub(1);
      if !(iterations != 0) {
        break;
      }
    }
    //printf(" elapsed2:%u ", elapsed2);
    elapsed = elapsed.wrapping_sub(elapsed2); // BUFCACHE_FACTOR (why?)
    total_MB = total_MB.wrapping_mul(2i32 as libc::c_uint);
    flush_buffer_cache();
  }
  print_timing(total_MB, elapsed);
  munlock(
    buf as *const libc::c_void,
    (1i32 * 1024i32 * 1024i32) as size_t,
  );
  free(buf as *mut libc::c_void);
}
unsafe extern "C" fn bus_state_value(mut value: libc::c_uint) {
  if value == BUSSTATE_ON as libc::c_int as libc::c_uint {
    on_off(1i32);
  } else if value == BUSSTATE_OFF as libc::c_int as libc::c_uint {
    on_off(0i32);
  } else if value == BUSSTATE_TRISTATE as libc::c_int as libc::c_uint {
    puts(b" (tristate)\x00" as *const u8 as *const libc::c_char);
  } else {
    printf(
      b" (unknown: %u)\n\x00" as *const u8 as *const libc::c_char,
      value,
    );
  };
}
unsafe extern "C" fn interpret_standby(mut standby: u8) {
  printf(b" (\x00" as *const u8 as *const libc::c_char);
  if standby as libc::c_int == 0i32 {
    printf(b"off\x00" as *const u8 as *const libc::c_char);
  } else if standby as libc::c_int <= 240i32
    || standby as libc::c_int == 252i32
    || standby as libc::c_int == 255i32
  {
    /* standby is in 5 sec units */
    let mut t: libc::c_uint = (standby as libc::c_int * 5i32) as libc::c_uint;
    printf(
      b"%u minutes %u seconds\x00" as *const u8 as *const libc::c_char,
      t.wrapping_div(60i32 as libc::c_uint),
      t.wrapping_rem(60i32 as libc::c_uint),
    );
  } else if standby as libc::c_int <= 251i32 {
    let mut t_0: libc::c_uint = (standby as libc::c_int - 240i32) as libc::c_uint;
    /* t is in 30 min units */
    printf(
      b"%u.%c hours\x00" as *const u8 as *const libc::c_char,
      t_0.wrapping_div(2i32 as libc::c_uint),
      if t_0 & 1i32 as libc::c_uint != 0 {
        '5' as i32
      } else {
        '0' as i32
      },
    );
  }
  if standby as libc::c_int == 253i32 {
    printf(b"vendor-specific\x00" as *const u8 as *const libc::c_char);
  }
  if standby as libc::c_int == 254i32 {
    printf(b"reserved\x00" as *const u8 as *const libc::c_char);
  }
  puts(b")\x00" as *const u8 as *const libc::c_char);
}
static mut xfermode_val: [u8; 32] = [
  8i32 as u8,
  9i32 as u8,
  10i32 as u8,
  11i32 as u8,
  12i32 as u8,
  13i32 as u8,
  14i32 as u8,
  15i32 as u8,
  16i32 as u8,
  17i32 as u8,
  18i32 as u8,
  19i32 as u8,
  20i32 as u8,
  21i32 as u8,
  22i32 as u8,
  23i32 as u8,
  32i32 as u8,
  33i32 as u8,
  34i32 as u8,
  35i32 as u8,
  36i32 as u8,
  37i32 as u8,
  38i32 as u8,
  39i32 as u8,
  64i32 as u8,
  65i32 as u8,
  66i32 as u8,
  67i32 as u8,
  68i32 as u8,
  69i32 as u8,
  70i32 as u8,
  71i32 as u8,
];
/* NB: we save size by _not_ storing terninating NUL! */
static mut xfermode_name: [[libc::c_char; 5]; 32] = [
  [112, 105, 111, 48, 0],
  [112, 105, 111, 49, 0],
  [112, 105, 111, 50, 0],
  [112, 105, 111, 51, 0],
  [112, 105, 111, 52, 0],
  [112, 105, 111, 53, 0],
  [112, 105, 111, 54, 0],
  [112, 105, 111, 55, 0],
  [115, 100, 109, 97, 48],
  [115, 100, 109, 97, 49],
  [115, 100, 109, 97, 50],
  [115, 100, 109, 97, 51],
  [115, 100, 109, 97, 52],
  [115, 100, 109, 97, 53],
  [115, 100, 109, 97, 54],
  [115, 100, 109, 97, 55],
  [109, 100, 109, 97, 48],
  [109, 100, 109, 97, 49],
  [109, 100, 109, 97, 50],
  [109, 100, 109, 97, 51],
  [109, 100, 109, 97, 52],
  [109, 100, 109, 97, 53],
  [109, 100, 109, 97, 54],
  [109, 100, 109, 97, 55],
  [117, 100, 109, 97, 48],
  [117, 100, 109, 97, 49],
  [117, 100, 109, 97, 50],
  [117, 100, 109, 97, 51],
  [117, 100, 109, 97, 52],
  [117, 100, 109, 97, 53],
  [117, 100, 109, 97, 54],
  [117, 100, 109, 97, 55],
];
unsafe extern "C" fn translate_xfermode(mut name: *const libc::c_char) -> libc::c_int {
  let mut val: libc::c_int = 0;
  let mut i: libc::c_uint = 0;
  i = 0i32 as libc::c_uint;
  while i
    < (::std::mem::size_of::<[u8; 32]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<u8>() as libc::c_ulong) as libc::c_uint
  {
    if strncmp(
      name,
      xfermode_name[i as usize].as_ptr(),
      5i32 as libc::c_ulong,
    ) == 0
    {
      if strlen(name) <= 5i32 as libc::c_ulong {
        return xfermode_val[i as usize] as libc::c_int;
      }
    }
    i = i.wrapping_add(1)
  }
  /* Negative numbers are invalid and are caught later */
  val = bb_strtoi(name, 0 as *mut *mut libc::c_char, 10i32);
  if *bb_errno == 0 {
    return val;
  }
  return -1i32;
}
unsafe extern "C" fn interpret_xfermode(mut xfermode: libc::c_uint) {
  printf(b" (\x00" as *const u8 as *const libc::c_char);
  if xfermode == 0i32 as libc::c_uint {
    printf(b"default PIO mode\x00" as *const u8 as *const libc::c_char);
  } else if xfermode == 1i32 as libc::c_uint {
    printf(b"default PIO mode, disable IORDY\x00" as *const u8 as *const libc::c_char);
  } else if xfermode >= 8i32 as libc::c_uint && xfermode <= 15i32 as libc::c_uint {
    printf(
      b"PIO flow control mode%u\x00" as *const u8 as *const libc::c_char,
      xfermode.wrapping_sub(8i32 as libc::c_uint),
    );
  } else if xfermode >= 16i32 as libc::c_uint && xfermode <= 23i32 as libc::c_uint {
    printf(
      b"singleword DMA mode%u\x00" as *const u8 as *const libc::c_char,
      xfermode.wrapping_sub(16i32 as libc::c_uint),
    );
  } else if xfermode >= 32i32 as libc::c_uint && xfermode <= 39i32 as libc::c_uint {
    printf(
      b"multiword DMA mode%u\x00" as *const u8 as *const libc::c_char,
      xfermode.wrapping_sub(32i32 as libc::c_uint),
    );
  } else if xfermode >= 64i32 as libc::c_uint && xfermode <= 71i32 as libc::c_uint {
    printf(
      b"UltraDMA mode%u\x00" as *const u8 as *const libc::c_char,
      xfermode.wrapping_sub(64i32 as libc::c_uint),
    );
  } else {
    printf(b"unknown\x00" as *const u8 as *const libc::c_char);
  }
  puts(b")\x00" as *const u8 as *const libc::c_char);
}
/* HDIO_DRIVE_CMD */
unsafe extern "C" fn print_flag(
  mut flag: libc::c_int,
  mut s: *const libc::c_char,
  mut value: libc::c_ulong,
) {
  if flag != 0 {
    printf(
      b" setting %s to %lu\n\x00" as *const u8 as *const libc::c_char,
      s,
      value,
    );
  };
}
unsafe extern "C" fn process_dev(mut devname: *mut libc::c_char) {
  /*int fd;*/
  let mut parm: libc::c_long = 0;
  let mut multcount: libc::c_long = 0;
  /* Please restore args[n] to these values after each ioctl
  except for args[2] */
  let mut args: [libc::c_uchar; 4] = [
    0xefi32 as libc::c_uchar,
    0i32 as libc::c_uchar,
    0i32 as libc::c_uchar,
    0i32 as libc::c_uchar,
  ];
  let mut fmt: *const libc::c_char = b" %s\t= %2ld\x00" as *const u8 as *const libc::c_char;
  /*fd = xopen_nonblocking(devname);*/
  xmove_fd(xopen_nonblocking(devname), fd as libc::c_int);
  printf(b"\n%s:\n\x00" as *const u8 as *const libc::c_char, devname);
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_readahead as libc::c_int == 2i32 {
    print_flag(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_readahead as libc::c_int,
      b"fs readahead\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).Xreadahead,
    );
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (0x12i32 << 0i32 + 8i32) as libc::c_uint
        | (98i32 << 0i32) as libc::c_uint
        | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).Xreadahead as *mut libc::c_int
        as *mut libc::c_void,
      b"BLKRASET\x00" as *const u8 as *const libc::c_char,
    );
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).unregister_hwif != 0 {
    printf(
      b" attempting to unregister hwif#%lu\n\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hwif,
    );
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0x32ai32 as libc::c_uint,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hwif as *mut libc::c_int
        as *mut libc::c_void,
      b"HDIO_UNREGISTER_HWIF\x00" as *const u8 as *const libc::c_char,
    );
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).scan_hwif as libc::c_int == 2i32 {
    printf(
      b" attempting to scan hwif (0x%lx, 0x%lx, %lu)\n\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hwif_data,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hwif_ctrl,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hwif_irq,
    );
    args[0] = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hwif_data as libc::c_uchar;
    args[1] = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hwif_ctrl as libc::c_uchar;
    args[2] = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hwif_irq as libc::c_uchar;
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0x328i32 as libc::c_uint,
      args.as_mut_ptr() as *mut libc::c_void,
      b"HDIO_SCAN_HWIF\x00" as *const u8 as *const libc::c_char,
    );
    args[0] = 0xefi32 as libc::c_uchar;
    args[1] = 0i32 as libc::c_uchar
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).set_piomode != 0 {
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).noisy_piomode != 0 {
      printf(b" attempting to \x00" as *const u8 as *const libc::c_char);
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).piomode == 255i32 {
        puts(b"auto-tune PIO mode\x00" as *const u8 as *const libc::c_char);
      } else if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).piomode < 100i32 {
        printf(
          b"set PIO mode to %d\n\x00" as *const u8 as *const libc::c_char,
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).piomode,
        );
      } else if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).piomode < 200i32 {
        printf(
          b"set MDMA mode to %d\n\x00" as *const u8 as *const libc::c_char,
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).piomode - 100i32,
        );
      } else {
        printf(
          b"set UDMA mode to %d\n\x00" as *const u8 as *const libc::c_char,
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).piomode - 200i32,
        );
      }
    }
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0x327i32 as libc::c_uint,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).piomode as libc::c_ulong
        as *mut libc::c_int as *mut libc::c_void,
      b"HDIO_SET_PIO_MODE\x00" as *const u8 as *const libc::c_char,
    );
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_io32bit as libc::c_int == 2i32 {
    print_flag(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_io32bit as libc::c_int,
      b"32-bit IO_support flag\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).io32bit,
    );
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0x324i32 as libc::c_uint,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).io32bit as *mut libc::c_int
        as *mut libc::c_void,
      b"HDIO_SET_32BIT\x00" as *const u8 as *const libc::c_char,
    );
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_mult as libc::c_int == 2i32 {
    print_flag(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_mult as libc::c_int,
      b"multcount\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).mult,
    );
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0x321i32 as libc::c_uint,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).mult as *mut libc::c_void,
      b"HDIO_SET_MULTCOUNT\x00" as *const u8 as *const libc::c_char,
    );
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_readonly as libc::c_int == 2i32 {
    print_flag_on_off(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_readonly as libc::c_int,
      b"readonly\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).readonly,
    );
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (0x12i32 << 0i32 + 8i32) as libc::c_uint
        | (93i32 << 0i32) as libc::c_uint
        | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint,
      &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).readonly as *mut libc::c_ulong
        as *mut libc::c_void,
      b"BLKROSET\x00" as *const u8 as *const libc::c_char,
    );
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_unmask as libc::c_int == 2i32 {
    print_flag_on_off(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_unmask as libc::c_int,
      b"unmaskirq\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).unmask,
    );
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0x322i32 as libc::c_uint,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).unmask as *mut libc::c_int
        as *mut libc::c_void,
      b"HDIO_SET_UNMASKINTR\x00" as *const u8 as *const libc::c_char,
    );
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_dma as libc::c_int == 2i32 {
    print_flag_on_off(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_dma as libc::c_int,
      b"using_dma\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).dma,
    );
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0x326i32 as libc::c_uint,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).dma as *mut libc::c_int
        as *mut libc::c_void,
      b"HDIO_SET_DMA\x00" as *const u8 as *const libc::c_char,
    );
  }
  /* FEATURE_HDPARM_HDIO_GETSET_DMA */
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_dma_q as libc::c_int == 2i32 {
    print_flag_on_off(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_dma_q as libc::c_int,
      b"DMA queue_depth\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).dma_q,
    );
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0x32ei32 as libc::c_uint,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).dma_q as *mut libc::c_int
        as *mut libc::c_void,
      b"HDIO_SET_QDMA\x00" as *const u8 as *const libc::c_char,
    );
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_nowerr as libc::c_int == 2i32 {
    print_flag_on_off(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_nowerr as libc::c_int,
      b"nowerr\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nowerr,
    );
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0x325i32 as libc::c_uint,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nowerr as *mut libc::c_int
        as *mut libc::c_void,
      b"HDIO_SET_NOWERR\x00" as *const u8 as *const libc::c_char,
    );
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_keep as libc::c_int == 2i32 {
    print_flag_on_off(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_keep as libc::c_int,
      b"keep_settings\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).keep,
    );
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0x323i32 as libc::c_uint,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).keep as *mut libc::c_int
        as *mut libc::c_void,
      b"HDIO_SET_KEEPSETTINGS\x00" as *const u8 as *const libc::c_char,
    );
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_doorlock as libc::c_int == 2i32 {
    args[0] = if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).doorlock != 0 {
      0xdei32
    } else {
      0xdfi32
    } as libc::c_uchar;
    args[2] = 0i32 as libc::c_uchar;
    print_flag_on_off(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_doorlock as libc::c_int,
      b"drive doorlock\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).doorlock,
    );
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0x31fi32 as libc::c_uint,
      &mut args as *mut [libc::c_uchar; 4] as *mut libc::c_void,
      b"HDIO_DRIVE_CMD\x00" as *const u8 as *const libc::c_char,
    );
    args[0] = 0xefi32 as libc::c_uchar
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_dkeep as libc::c_int == 2i32 {
    /* lock/unlock the drive's "feature" settings */
    print_flag_on_off(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_dkeep as libc::c_int,
      b"drive keep features\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).dkeep,
    );
    args[2] = if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).dkeep != 0 {
      0x66i32
    } else {
      0xcci32
    } as libc::c_uchar;
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0x31fi32 as libc::c_uint,
      &mut args as *mut [libc::c_uchar; 4] as *mut libc::c_void,
      b"HDIO_DRIVE_CMD\x00" as *const u8 as *const libc::c_char,
    );
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_defects as libc::c_int == 2i32 {
    args[2] = if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).defects != 0 {
      0x4i32
    } else {
      0x84i32
    } as libc::c_uchar;
    print_flag(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_defects as libc::c_int,
      b"drive defect-mgmt\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).defects,
    );
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0x31fi32 as libc::c_uint,
      &mut args as *mut [libc::c_uchar; 4] as *mut libc::c_void,
      b"HDIO_DRIVE_CMD\x00" as *const u8 as *const libc::c_char,
    );
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_prefetch as libc::c_int == 2i32 {
    args[1] = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).prefetch as libc::c_uchar;
    args[2] = 0xabi32 as libc::c_uchar;
    print_flag(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_prefetch as libc::c_int,
      b"drive prefetch\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).prefetch,
    );
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0x31fi32 as libc::c_uint,
      &mut args as *mut [libc::c_uchar; 4] as *mut libc::c_void,
      b"HDIO_DRIVE_CMD\x00" as *const u8 as *const libc::c_char,
    );
    args[1] = 0i32 as libc::c_uchar
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).set_xfermode != 0 {
    args[1] =
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).xfermode_requested as libc::c_uchar;
    args[2] = 3i32 as libc::c_uchar;
    print_flag(
      1i32,
      b"xfermode\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).xfermode_requested as libc::c_ulong,
    );
    interpret_xfermode(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).xfermode_requested as libc::c_uint,
    );
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0x31fi32 as libc::c_uint,
      &mut args as *mut [libc::c_uchar; 4] as *mut libc::c_void,
      b"HDIO_DRIVE_CMD\x00" as *const u8 as *const libc::c_char,
    );
    args[1] = 0i32 as libc::c_uchar
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_lookahead as libc::c_int == 2i32 {
    args[2] = if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lookahead != 0 {
      0xaai32
    } else {
      0x55i32
    } as libc::c_uchar;
    print_flag_on_off(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_lookahead as libc::c_int,
      b"drive read-lookahead\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lookahead,
    );
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0x31fi32 as libc::c_uint,
      &mut args as *mut [libc::c_uchar; 4] as *mut libc::c_void,
      b"HDIO_DRIVE_CMD\x00" as *const u8 as *const libc::c_char,
    );
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_apmmode as libc::c_int == 2i32 {
    /* feature register */
    args[2] =
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).apmmode == 255i32 as libc::c_ulong {
        0x85i32
      } else {
        0x5i32
      } as libc::c_uchar; /* set */
    args[1] = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).apmmode as libc::c_uchar; /* sector count register 1-255 */
    printf(
      b" setting APM level to %s 0x%02lX (%ld)\n\x00" as *const u8 as *const libc::c_char,
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).apmmode == 255i32 as libc::c_ulong {
        b"disabled\x00" as *const u8 as *const libc::c_char
      } else {
        b"\x00" as *const u8 as *const libc::c_char
      },
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).apmmode,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).apmmode,
    );
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0x31fi32 as libc::c_uint,
      &mut args as *mut [libc::c_uchar; 4] as *mut libc::c_void,
      b"HDIO_DRIVE_CMD\x00" as *const u8 as *const libc::c_char,
    );
    args[1] = 0i32 as libc::c_uchar
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_wcache as libc::c_int == 2i32 {
    /* DO_FLUSHCACHE */
    args[2] = if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).wcache != 0 {
      0x2i32
    } else {
      0x82i32
    } as libc::c_uchar;
    print_flag_on_off(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_wcache as libc::c_int,
      b"drive write-caching\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).wcache,
    );
    /* DO_FLUSHCACHE */
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0x31fi32 as libc::c_uint,
      &mut args as *mut [libc::c_uchar; 4] as *mut libc::c_void,
      b"HDIO_DRIVE_CMD\x00" as *const u8 as *const libc::c_char,
    );
  }
  /* DO_FLUSHCACHE */
  /* In code below, we do not preserve args[0], but the rest
  is preserved, including args[2] */
  args[2] = 0i32 as libc::c_uchar;
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).set_standbynow != 0 {
    puts(b" issuing standby command\x00" as *const u8 as *const libc::c_char);
    args[0] = 0xe0i32 as libc::c_uchar;
    ioctl_alt_func(
      0x31fi32,
      args.as_mut_ptr(),
      0x94i32,
      b"HDIO_DRIVE_CMD\x00" as *const u8 as *const libc::c_char,
    );
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).set_sleepnow != 0 {
    puts(b" issuing sleep command\x00" as *const u8 as *const libc::c_char);
    args[0] = 0xe6i32 as libc::c_uchar;
    ioctl_alt_func(
      0x31fi32,
      args.as_mut_ptr(),
      0x99i32,
      b"HDIO_DRIVE_CMD\x00" as *const u8 as *const libc::c_char,
    );
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).set_seagate != 0 {
    args[0] = 0xfbi32 as libc::c_uchar;
    puts(b" disabling Seagate auto powersaving mode\x00" as *const u8 as *const libc::c_char);
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0x31fi32 as libc::c_uint,
      &mut args as *mut [libc::c_uchar; 4] as *mut libc::c_void,
      b"HDIO_DRIVE_CMD\x00" as *const u8 as *const libc::c_char,
    );
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_standby as libc::c_int == 2i32 {
    args[0] = 0xe3i32 as libc::c_uchar;
    args[1] =
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).standby_requested as libc::c_uchar;
    print_flag(
      1i32,
      b"standby\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).standby_requested,
    );
    interpret_standby((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).standby_requested as u8);
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0x31fi32 as libc::c_uint,
      &mut args as *mut [libc::c_uchar; 4] as *mut libc::c_void,
      b"HDIO_DRIVE_CMD\x00" as *const u8 as *const libc::c_char,
    );
    args[1] = 0i32 as libc::c_uchar
  }
  /* HDIO_DRIVE_CMD */
  /* HDIO_DRIVE_CMD */
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_mult as libc::c_int != 0
    || (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).get_identity as libc::c_int != 0
  {
    multcount = -1i32 as libc::c_long;
    if ioctl(
      fd as libc::c_int,
      0x304i32 as libc::c_ulong,
      &mut multcount as *mut libc::c_long,
    ) != 0
    {
      /* To be coherent with ioctl_or_warn. */
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_mult as libc::c_int != 0
        && 1i32 != 0
      {
        bb_simple_perror_msg(b"HDIO_GET_MULTCOUNT\x00" as *const u8 as *const libc::c_char);
      } else {
        bb_perror_msg(
          b"ioctl %#x failed\x00" as *const u8 as *const libc::c_char,
          0x304i32,
        );
      }
    } else if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_mult != 0 {
      printf(
        fmt,
        b"multcount\x00" as *const u8 as *const libc::c_char,
        multcount,
      );
      on_off((multcount != 0) as libc::c_int);
    }
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_io32bit != 0 {
    if bb_ioctl_or_warn(
      fd as libc::c_int,
      0x309i32 as libc::c_uint,
      &mut parm as *mut libc::c_long as *mut libc::c_void,
      b"HDIO_GET_32BIT\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
      printf(
        b" IO_support\t=%3ld (\x00" as *const u8 as *const libc::c_char,
        parm,
      );
      if parm == 0 {
        puts(b"default 16-bit)\x00" as *const u8 as *const libc::c_char);
      } else if parm == 2i32 as libc::c_long {
        puts(b"16-bit)\x00" as *const u8 as *const libc::c_char);
      } else if parm == 1 {
        puts(b"32-bit)\x00" as *const u8 as *const libc::c_char);
      } else if parm == 3i32 as libc::c_long {
        puts(b"32-bit w/sync)\x00" as *const u8 as *const libc::c_char);
      } else if parm == 8i32 as libc::c_long {
        puts(b"Request-Queue-Bypass)\x00" as *const u8 as *const libc::c_char);
      } else {
        puts(b"???)\x00" as *const u8 as *const libc::c_char);
      }
    }
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_unmask != 0 {
    if bb_ioctl_or_warn(
      fd as libc::c_int,
      0x302i32 as libc::c_uint,
      &mut parm as *mut libc::c_long as *mut libc::c_void,
      b"HDIO_GET_UNMASKINTR\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
      print_value_on_off(
        b"unmaskirq\x00" as *const u8 as *const libc::c_char,
        parm as libc::c_ulong,
      );
    }
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_dma != 0 {
    if bb_ioctl_or_warn(
      fd as libc::c_int,
      0x30bi32 as libc::c_uint,
      &mut parm as *mut libc::c_long as *mut libc::c_void,
      b"HDIO_GET_DMA\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
      printf(
        fmt,
        b"using_dma\x00" as *const u8 as *const libc::c_char,
        parm,
      );
      if parm == 8i32 as libc::c_long {
        puts(b" (DMA-Assisted-PIO)\x00" as *const u8 as *const libc::c_char);
      } else {
        on_off((parm != 0) as libc::c_int);
      }
    }
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_dma_q != 0 {
    if bb_ioctl_or_warn(
      fd as libc::c_int,
      0x305i32 as libc::c_uint,
      &mut parm as *mut libc::c_long as *mut libc::c_void,
      b"HDIO_GET_QDMA\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
      print_value_on_off(
        b"queue_depth\x00" as *const u8 as *const libc::c_char,
        parm as libc::c_ulong,
      );
    }
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_keep != 0 {
    if bb_ioctl_or_warn(
      fd as libc::c_int,
      0x308i32 as libc::c_uint,
      &mut parm as *mut libc::c_long as *mut libc::c_void,
      b"HDIO_GET_KEEPSETTINGS\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
      print_value_on_off(
        b"keepsettings\x00" as *const u8 as *const libc::c_char,
        parm as libc::c_ulong,
      );
    }
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_nowerr != 0 {
    if bb_ioctl_or_warn(
      fd as libc::c_int,
      0x30ai32 as libc::c_uint,
      &mut parm as *mut libc::c_long as *mut libc::c_void,
      b"HDIO_GET_NOWERR\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
      print_value_on_off(
        b"nowerr\x00" as *const u8 as *const libc::c_char,
        parm as libc::c_ulong,
      );
    }
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_readonly != 0 {
    if bb_ioctl_or_warn(
      fd as libc::c_int,
      0u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (0x12i32 << 0i32 + 8i32) as libc::c_uint
        | (94i32 << 0i32) as libc::c_uint
        | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint,
      &mut parm as *mut libc::c_long as *mut libc::c_void,
      b"BLKROGET\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
      print_value_on_off(
        b"readonly\x00" as *const u8 as *const libc::c_char,
        parm as libc::c_ulong,
      );
    }
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_readahead != 0 {
    if bb_ioctl_or_warn(
      fd as libc::c_int,
      0u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (0x12i32 << 0i32 + 8i32) as libc::c_uint
        | (99i32 << 0i32) as libc::c_uint
        | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint,
      &mut parm as *mut libc::c_long as *mut libc::c_void,
      b"BLKRAGET\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
      print_value_on_off(
        b"readahead\x00" as *const u8 as *const libc::c_char,
        parm as libc::c_ulong,
      );
    }
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).get_geom != 0 {
    if bb_ioctl_or_warn(
      fd as libc::c_int,
      0u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (0x12i32 << 0i32 + 8i32) as libc::c_uint
        | (96i32 << 0i32) as libc::c_uint
        | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint,
      &mut parm as *mut libc::c_long as *mut libc::c_void,
      b"BLKGETSIZE\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
      let mut g: hd_geometry = hd_geometry {
        heads: 0,
        sectors: 0,
        cylinders: 0,
        start: 0,
      };
      if bb_ioctl_or_warn(
        fd as libc::c_int,
        0x301i32 as libc::c_uint,
        &mut g as *mut hd_geometry as *mut libc::c_void,
        b"HDIO_GETGEO\x00" as *const u8 as *const libc::c_char,
      ) == 0
      {
        printf(
          b" geometry\t= %u/%u/%u, sectors = %ld, start = %ld\n\x00" as *const u8
            as *const libc::c_char,
          g.cylinders as libc::c_int,
          g.heads as libc::c_int,
          g.sectors as libc::c_int,
          parm,
          g.start,
        );
      }
    }
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).get_powermode != 0 {
    let mut state: *const libc::c_char = std::ptr::null();
    args[0] = 0xe5i32 as libc::c_uchar;
    if ioctl_alt_func(
      0x31fi32,
      args.as_mut_ptr(),
      0x98i32,
      b"HDIO_DRIVE_CMD\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
      if *bb_errno != 5i32 || args[0] as libc::c_int != 0i32 || args[1] as libc::c_int != 0i32 {
        state = b"unknown\x00" as *const u8 as *const libc::c_char
      } else {
        state = b"sleeping\x00" as *const u8 as *const libc::c_char
      }
    } else {
      state = if args[2] as libc::c_int == 255i32 {
        b"active/idle\x00" as *const u8 as *const libc::c_char
      } else {
        b"standby\x00" as *const u8 as *const libc::c_char
      }
    }
    args[2] = 0i32 as libc::c_uchar;
    args[1] = args[2];
    printf(
      b" drive state is:  %s\n\x00" as *const u8 as *const libc::c_char,
      state,
    );
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).perform_reset != 0 {
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0x31ci32 as libc::c_uint,
      0 as *mut libc::c_void,
      b"HDIO_DRIVE_RESET\x00" as *const u8 as *const libc::c_char,
    );
  }
  /* FEATURE_HDPARM_HDIO_DRIVE_RESET */
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).perform_tristate != 0 {
    args[0] = 0i32 as libc::c_uchar;
    args[1] = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tristate as libc::c_uchar;
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0x31bi32 as libc::c_uint,
      &mut args as *mut [libc::c_uchar; 4] as *mut libc::c_void,
      b"HDIO_TRISTATE_HWIF\x00" as *const u8 as *const libc::c_char,
    );
  }
  /* FEATURE_HDPARM_HDIO_TRISTATE_HWIF */
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).get_identity != 0 {
    let mut id: hd_driveid = hd_driveid {
      config: 0,
      cyls: 0,
      reserved2: 0,
      heads: 0,
      track_bytes: 0,
      sector_bytes: 0,
      sectors: 0,
      vendor0: 0,
      vendor1: 0,
      vendor2: 0,
      serial_no: [0; 20],
      buf_type: 0,
      buf_size: 0,
      ecc_bytes: 0,
      fw_rev: [0; 8],
      model: [0; 40],
      max_multsect: 0,
      vendor3: 0,
      dword_io: 0,
      vendor4: 0,
      capability: 0,
      reserved50: 0,
      vendor5: 0,
      tPIO: 0,
      vendor6: 0,
      tDMA: 0,
      field_valid: 0,
      cur_cyls: 0,
      cur_heads: 0,
      cur_sectors: 0,
      cur_capacity0: 0,
      cur_capacity1: 0,
      multsect: 0,
      multsect_valid: 0,
      lba_capacity: 0,
      dma_1word: 0,
      dma_mword: 0,
      eide_pio_modes: 0,
      eide_dma_min: 0,
      eide_dma_time: 0,
      eide_pio: 0,
      eide_pio_iordy: 0,
      words69_70: [0; 2],
      words71_74: [0; 4],
      queue_depth: 0,
      words76_79: [0; 4],
      major_rev_num: 0,
      minor_rev_num: 0,
      command_set_1: 0,
      command_set_2: 0,
      cfsse: 0,
      cfs_enable_1: 0,
      cfs_enable_2: 0,
      csf_default: 0,
      dma_ultra: 0,
      trseuc: 0,
      trsEuc: 0,
      CurAPMvalues: 0,
      mprc: 0,
      hw_config: 0,
      acoustic: 0,
      msrqs: 0,
      sxfert: 0,
      sal: 0,
      spg: 0,
      lba_capacity_2: 0,
      words104_125: [0; 22],
      last_lun: 0,
      word127: 0,
      dlf: 0,
      csfo: 0,
      words130_155: [0; 26],
      word156: 0,
      words157_159: [0; 3],
      cfa_power: 0,
      words161_175: [0; 15],
      words176_205: [0; 30],
      words206_254: [0; 49],
      integrity_word: 0,
    };
    if ioctl(
      fd as libc::c_int,
      0x30di32 as libc::c_ulong,
      &mut id as *mut hd_driveid,
    ) == 0
    {
      if multcount != -1i32 as libc::c_long {
        id.multsect = multcount as libc::c_uchar;
        id.multsect_valid = (id.multsect_valid as libc::c_int | 1i32) as libc::c_uchar
      } else {
        id.multsect_valid = (id.multsect_valid as libc::c_int & !1i32) as libc::c_uchar
      }
      dump_identity(&mut id);
    } else if *bb_errno == -42i32 {
      puts(b" no identification info available\x00" as *const u8 as *const libc::c_char);
    } else {
      /* To be coherent with ioctl_or_warn */
      bb_simple_perror_msg(b"HDIO_GET_IDENTITY\x00" as *const u8 as *const libc::c_char);
      /* = { ... } will eat 0.5k of rodata! */
    }
  } /* time cache */
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).get_IDentity != 0 {
    let mut args1: [libc::c_uchar; 516] = [0; 516]; /* time device */
    memset(
      args1.as_mut_ptr() as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<[libc::c_uchar; 516]>() as libc::c_ulong,
    );
    args1[0] = 0xeci32 as libc::c_uchar;
    args1[3] = 1i32 as libc::c_uchar;
    if ioctl_alt_func(
      0x31fi32,
      args1.as_mut_ptr(),
      0xa1i32,
      b"HDIO_DRIVE_CMD\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
      identify(args1.as_mut_ptr().offset(4) as *mut libc::c_void as *mut u16);
    }
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_busstate as libc::c_int == 2i32 {
    print_flag(
      1i32,
      b"bus state\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).busstate,
    );
    bus_state_value((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).busstate as libc::c_uint);
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0x32di32 as libc::c_uint,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).busstate as *mut libc::c_int
        as *mut libc::c_void,
      b"HDIO_SET_BUSSTATE\x00" as *const u8 as *const libc::c_char,
    );
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_busstate != 0 {
    if bb_ioctl_or_warn(
      fd as libc::c_int,
      0x31ai32 as libc::c_uint,
      &mut parm as *mut libc::c_long as *mut libc::c_void,
      b"HDIO_GET_BUSSTATE\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
      printf(
        fmt,
        b"bus state\x00" as *const u8 as *const libc::c_char,
        parm,
      );
      bus_state_value(parm as libc::c_uint);
    }
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).reread_partn != 0 {
    bb_ioctl_or_warn(
      fd as libc::c_int,
      0u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (0x12i32 << 0i32 + 8i32) as libc::c_uint
        | (95i32 << 0i32) as libc::c_uint
        | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint,
      0 as *mut libc::c_void,
      b"BLKRRPART\x00" as *const u8 as *const libc::c_char,
    );
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).do_ctimings != 0 {
    do_time(1i32);
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).do_timings != 0 {
    do_time(0i32);
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).do_flush != 0 {
    flush_buffer_cache();
  }
  close(fd as libc::c_int);
}
unsafe extern "C" fn fromhex(mut c: libc::c_uchar) -> libc::c_int {
  if (c as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
    return c as libc::c_int - '0' as i32;
  }
  if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'f' as i32 {
    return c as libc::c_int - ('a' as i32 - 10i32);
  }
  bb_error_msg_and_die(
    b"bad char: \'%c\' 0x%02x\x00" as *const u8 as *const libc::c_char,
    c as libc::c_int,
    c as libc::c_int,
  );
}
unsafe extern "C" fn identify_from_stdin() -> ! {
  let mut sbuf: [u16; 256] = [0; 256];
  let mut buf: [libc::c_uchar; 1280] = [0; 1280];
  let mut b: *mut libc::c_uchar = buf.as_mut_ptr();
  let mut i: libc::c_int = 0;
  xread(
    0i32,
    buf.as_mut_ptr() as *mut libc::c_void,
    1280i32 as size_t,
  );
  // Convert the newline-separated hex data into an identify block.
  i = 0i32;
  while i < 256i32 {
    let mut j: libc::c_int = 0;
    j = 0i32;
    while j < 4i32 {
      let fresh1 = b;
      b = b.offset(1);
      sbuf[i as usize] = (((sbuf[i as usize] as libc::c_int) << 4i32) + fromhex(*fresh1)) as u16;
      j += 1
    }
    i += 1
  }
  // Parse the data.
  identify(sbuf.as_mut_ptr());
}
/* busybox specific stuff */
unsafe extern "C" fn parse_opts(
  mut value: *mut libc::c_ulong,
  mut min: libc::c_int,
  mut max: libc::c_int,
) -> libc::c_int {
  if !optarg.is_null() {
    *value = xatol_range(optarg, min as libc::c_long, max as libc::c_long) as libc::c_ulong;
    return 2i32;
  }
  return 1i32;
}
unsafe extern "C" fn parse_opts_0_max(
  mut value: *mut libc::c_ulong,
  mut max: libc::c_int,
) -> libc::c_int {
  return parse_opts(value, 0i32, max);
}
unsafe extern "C" fn parse_opts_0_1(mut value: *mut libc::c_ulong) -> libc::c_int {
  return parse_opts(value, 0i32, 1i32);
}
unsafe extern "C" fn parse_opts_0_INTMAX(mut value: *mut libc::c_ulong) -> libc::c_int {
  return parse_opts(value, 0i32, 2147483647i32);
}
unsafe extern "C" fn parse_xfermode(
  mut flag: libc::c_int,
  mut get: *mut smallint,
  mut set: *mut smallint,
  mut value: *mut libc::c_int,
) {
  if flag != 0 {
    *get = 1i32 as smallint;
    if !optarg.is_null() {
      *value = translate_xfermode(optarg);
      *set = (*value > -1i32) as libc::c_int as smallint
    }
  };
}
/*------- getopt short options --------*/
static mut hdparm_options: [libc::c_char; 68] = [
  103, 102, 117, 58, 58, 110, 58, 58, 112, 58, 114, 58, 58, 109, 58, 58, 99, 58, 58, 107, 58, 58,
  97, 58, 58, 66, 58, 116, 84, 105, 73, 100, 58, 58, 83, 58, 68, 58, 80, 58, 88, 58, 75, 58, 65,
  58, 76, 58, 87, 58, 67, 121, 89, 122, 90, 85, 58, 81, 58, 119, 120, 58, 58, 98, 58, 82, 58, 0,
];
/*-------------------------------------*/
/* our main() routine: */
#[no_mangle]
pub unsafe extern "C" fn hdparm_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut c: libc::c_int = 0;
  let mut flagcount: libc::c_int = 0i32;
  loop {
    c = getopt(argc, argv, hdparm_options.as_ptr());
    if !(c >= 0i32) {
      break;
    }
    flagcount += 1;
    let ref mut fresh2 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).get_IDentity;
    *fresh2 = (*fresh2 as libc::c_int | (c == 'I' as i32) as libc::c_int) as smallint;
    let ref mut fresh3 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).get_identity;
    *fresh3 = (*fresh3 as libc::c_int | (c == 'i' as i32) as libc::c_int) as smallint;
    let ref mut fresh4 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).get_geom;
    *fresh4 = (*fresh4 as libc::c_int | (c == 'g' as i32) as libc::c_int) as smallint;
    let ref mut fresh5 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).do_flush;
    *fresh5 = (*fresh5 as libc::c_int | (c == 'f' as i32) as libc::c_int) as smallint;
    if c == 'u' as i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_unmask =
        parse_opts_0_1(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).unmask) as smallint
    }
    if c == 'd' as i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_dma = parse_opts_0_max(
        &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).dma,
        9i32,
      ) as smallint
    }
    if c == 'n' as i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_nowerr =
        parse_opts_0_1(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nowerr) as smallint
    }
    parse_xfermode(
      (c == 'p' as i32) as libc::c_int,
      &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).noisy_piomode,
      &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).set_piomode,
      &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).piomode,
    );
    if c == 'r' as i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_readonly =
        parse_opts_0_1(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).readonly)
          as smallint
    }
    if c == 'm' as i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_mult =
        parse_opts_0_INTMAX(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).mult)
          as smallint
    }
    if c == 'c' as i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_io32bit =
        parse_opts_0_INTMAX(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).io32bit)
          as smallint
    }
    if c == 'k' as i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_keep =
        parse_opts_0_1(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).keep) as smallint
    }
    if c == 'a' as i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_readahead =
        parse_opts_0_INTMAX(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).Xreadahead)
          as smallint
    }
    if c == 'B' as i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_apmmode = parse_opts(
        &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).apmmode,
        1i32,
        255i32,
      ) as smallint
    }
    let ref mut fresh6 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).do_timings;
    *fresh6 = (*fresh6 as libc::c_int | (c == 't' as i32) as libc::c_int) as smallint;
    let ref mut fresh7 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).do_flush;
    *fresh7 = (*fresh7 as libc::c_int | *fresh6 as libc::c_int) as smallint;
    let ref mut fresh8 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).do_ctimings;
    *fresh8 = (*fresh8 as libc::c_int | (c == 'T' as i32) as libc::c_int) as smallint;
    let ref mut fresh9 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).do_flush;
    *fresh9 = (*fresh9 as libc::c_int | *fresh8 as libc::c_int) as smallint;
    if c == 'S' as i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_standby = parse_opts_0_max(
        &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).standby_requested,
        255i32,
      ) as smallint
    }
    if c == 'D' as i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_defects =
        parse_opts_0_INTMAX(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).defects)
          as smallint
    }
    if c == 'P' as i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_prefetch =
        parse_opts_0_INTMAX(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).prefetch)
          as smallint
    }
    parse_xfermode(
      (c == 'X' as i32) as libc::c_int,
      &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).get_xfermode,
      &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).set_xfermode,
      &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).xfermode_requested,
    );
    if c == 'K' as i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_dkeep =
        parse_opts_0_1(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).prefetch)
          as smallint
    }
    if c == 'A' as i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_lookahead =
        parse_opts_0_1(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lookahead)
          as smallint
    }
    if c == 'L' as i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_doorlock =
        parse_opts_0_1(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).doorlock)
          as smallint
    }
    if c == 'W' as i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_wcache =
        parse_opts_0_1(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).wcache) as smallint
    }
    let ref mut fresh10 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).get_powermode;
    *fresh10 = (*fresh10 as libc::c_int | (c == 'C' as i32) as libc::c_int) as smallint;
    let ref mut fresh11 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).set_standbynow;
    *fresh11 = (*fresh11 as libc::c_int | (c == 'y' as i32) as libc::c_int) as smallint;
    let ref mut fresh12 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).set_sleepnow;
    *fresh12 = (*fresh12 as libc::c_int | (c == 'Y' as i32) as libc::c_int) as smallint;
    let ref mut fresh13 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).reread_partn;
    *fresh13 = (*fresh13 as libc::c_int | (c == 'z' as i32) as libc::c_int) as smallint;
    let ref mut fresh14 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).set_seagate;
    *fresh14 = (*fresh14 as libc::c_int | (c == 'Z' as i32) as libc::c_int) as smallint;
    if c == 'U' as i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).unregister_hwif =
        parse_opts_0_INTMAX(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hwif)
          as smallint
    }
    if c == 'Q' as i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_dma_q =
        parse_opts_0_INTMAX(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).dma_q)
          as smallint
    }
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).perform_reset =
      (c == 'r' as i32) as libc::c_int as smallint;
    if c == 'x' as i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).perform_tristate =
        parse_opts_0_1(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tristate)
          as smallint
    }
    if c == 'b' as i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_busstate = parse_opts_0_max(
        &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).busstate,
        2i32,
      ) as smallint
    }
    if c == 'R' as i32 {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).scan_hwif =
        parse_opts_0_INTMAX(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hwif_data)
          as smallint;
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hwif_ctrl =
        xatoi_positive(if !(*argv.offset(optind as isize)).is_null() {
          *argv.offset(optind as isize)
        } else {
          b"\x00" as *const u8 as *const libc::c_char
        }) as libc::c_ulong;
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hwif_irq =
        xatoi_positive(if !(*argv.offset((optind + 1i32) as isize)).is_null() {
          *argv.offset((optind + 1i32) as isize)
        } else {
          b"\x00" as *const u8 as *const libc::c_char
        }) as libc::c_ulong;
      /* Move past the 2 additional arguments */
      argv = argv.offset(2);
      argc -= 2i32
    }
  }
  /* When no flags are given (flagcount = 0), -acdgkmnru is assumed. */
  if flagcount == 0 {
    let ref mut fresh15 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).get_geom; /* EXIT */
    *fresh15 = 1i32 as smallint;
    let ref mut fresh16 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_readahead;
    *fresh16 = *fresh15;
    let ref mut fresh17 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_readonly;
    *fresh17 = *fresh16;
    let ref mut fresh18 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_keep;
    *fresh18 = *fresh17;
    let ref mut fresh19 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_unmask;
    *fresh19 = *fresh18;
    let ref mut fresh20 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_io32bit;
    *fresh20 = *fresh19;
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_mult = *fresh20;
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).getset_dma = 1i32 as smallint
  }
  argv = argv.offset(optind as isize);
  if (*argv).is_null() {
    if 1i32 != 0 && isatty(0i32) == 0 {
      identify_from_stdin();
    }
    bb_show_usage();
  }
  loop {
    let fresh21 = argv;
    argv = argv.offset(1);
    process_dev(*fresh21);
    if (*argv).is_null() {
      break;
    }
  }
  return 0i32;
}

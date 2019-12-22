use libc;
use libc::printf;
extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  static kmg_i_suffixes: [suffix_mult; 0];

}

use crate::librb::suffix_mult;
pub type __u64 = libc::c_ulonglong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fstrim_range {
  pub start: __u64,
  pub len: __u64,
  pub minlen: __u64,
}
pub const OPT_v: C2RustUnnamed = 8;
pub const OPT_m: C2RustUnnamed = 4;
pub const OPT_l: C2RustUnnamed = 2;
pub const OPT_o: C2RustUnnamed = 1;
pub type C2RustUnnamed = libc::c_uint;

/*
 * fstrim.c - discard the part (or whole) of mounted filesystem.
 *
 * 03 March 2012 - Malek Degachi <malek-degachi@laposte.net>
 * Adapted for busybox from util-linux-2.12a.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config FSTRIM
//config:	bool "fstrim (4.4 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	Discard unused blocks on a mounted filesystem.
//applet:IF_FSTRIM(APPLET_NOEXEC(fstrim, fstrim, BB_DIR_SBIN, SUID_DROP, fstrim))
//kbuild:lib-$(CONFIG_FSTRIM) += fstrim.o
//usage:#define fstrim_trivial_usage
//usage:       "[OPTIONS] MOUNTPOINT"
//usage:#define fstrim_full_usage "\n\n"
//usage:	IF_LONG_OPTS(
//usage:       "	-o,--offset OFFSET	Offset in bytes to discard from"
//usage:     "\n	-l,--length LEN		Bytes to discard"
//usage:     "\n	-m,--minimum MIN	Minimum extent length"
//usage:     "\n	-v,--verbose		Print number of discarded bytes"
//usage:	)
//usage:	IF_NOT_LONG_OPTS(
//usage:       "	-o OFFSET	Offset in bytes to discard from"
//usage:     "\n	-l LEN		Bytes to discard"
//usage:     "\n	-m MIN		Minimum extent length"
//usage:     "\n	-v		Print number of discarded bytes"
//usage:	)
#[no_mangle]
pub unsafe extern "C" fn fstrim_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut range: fstrim_range = fstrim_range {
    start: 0,
    len: 0,
    minlen: 0,
  };
  let mut arg_o: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut arg_l: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut arg_m: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut mp: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut opts: libc::c_uint = 0;
  let mut fd: libc::c_int = 0;
  static mut fstrim_longopts: [libc::c_char; 39] = [
    111, 102, 102, 115, 101, 116, 0, 1, 111, 108, 101, 110, 103, 116, 104, 0, 1, 108, 109, 105,
    110, 105, 109, 117, 109, 0, 1, 109, 118, 101, 114, 98, 111, 115, 101, 0, 0, 118, 0,
  ];
  opts = crate::libbb::getopt32::getopt32long(
    argv,
    b"^o:l:m:v\x00=1\x00" as *const u8 as *const libc::c_char,
    fstrim_longopts.as_ptr(),
    &mut arg_o as *mut *mut libc::c_char,
    &mut arg_l as *mut *mut libc::c_char,
    &mut arg_m as *mut *mut libc::c_char,
  );
  memset(
    &mut range as *mut fstrim_range as *mut libc::c_void,
    0,
    ::std::mem::size_of::<fstrim_range>() as libc::c_ulong,
  );
  range.len = (9223372036854775807i64 as libc::c_ulonglong)
    .wrapping_mul(2u64)
    .wrapping_add(1u64);
  if opts & OPT_o as libc::c_int as libc::c_uint != 0 {
    range.start = crate::libbb::xatonum::xatoull_sfx(arg_o, kmg_i_suffixes.as_ptr())
  }
  if opts & OPT_l as libc::c_int as libc::c_uint != 0 {
    range.len = crate::libbb::xatonum::xatoull_sfx(arg_l, kmg_i_suffixes.as_ptr())
  }
  if opts & OPT_m as libc::c_int as libc::c_uint != 0 {
    range.minlen = crate::libbb::xatonum::xatoull_sfx(arg_m, kmg_i_suffixes.as_ptr())
  }
  mp = *argv.offset(optind as isize);
  //Wwhy bother checking that it's a blockdev?
  //	if (find_block_device(mp)) {
  fd = crate::libbb::xfuncs_printf::xopen_nonblocking(mp);
  /* On ENOTTY error, util-linux 2.31 says:
   * "fstrim: FILE: the discard operation is not supported"
   */
  crate::libbb::xfuncs_printf::bb_xioctl(
    fd,
    (((2u32 | 1u32) << 0 + 8i32 + 8i32 + 14i32
      | (('X' as i32) << 0 + 8i32) as libc::c_uint
      | (121i32 << 0) as libc::c_uint) as libc::c_ulong
      | (::std::mem::size_of::<fstrim_range>() as libc::c_ulong) << 0 + 8i32 + 8i32)
      as libc::c_uint,
    &mut range as *mut fstrim_range as *mut libc::c_void,
    b"FITRIM\x00" as *const u8 as *const libc::c_char,
  );
  if opts & OPT_v as libc::c_int as libc::c_uint != 0 {
    printf(
      b"%s: %llu bytes trimmed\n\x00" as *const u8 as *const libc::c_char,
      mp,
      range.len,
    );
  }
  return 0;
}

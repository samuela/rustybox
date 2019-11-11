use c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
use crate::libbb::ptr_to_globals::bb_errno;
use libc;
use libc::printf;
extern "C" {

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn xatoll(str: *const libc::c_char) -> libc::c_longlong;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_perror_nomsg_and_die() -> !;
  #[no_mangle]
  fn nth_string(strings: *const libc::c_char, n: libc::c_int) -> *const libc::c_char;
  #[no_mangle]
  fn adjtimex(__ntx: *mut timex) -> libc::c_int;
}

use crate::librb::__syscall_slong_t;
use crate::librb::size_t;
use libc::timeval;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct timex {
  pub modes: libc::c_uint,
  pub offset: __syscall_slong_t,
  pub freq: __syscall_slong_t,
  pub maxerror: __syscall_slong_t,
  pub esterror: __syscall_slong_t,
  pub status: libc::c_int,
  pub constant: __syscall_slong_t,
  pub precision: __syscall_slong_t,
  pub tolerance: __syscall_slong_t,
  pub time: timeval,
  pub tick: __syscall_slong_t,
  pub ppsfreq: __syscall_slong_t,
  pub jitter: __syscall_slong_t,
  pub shift: libc::c_int,
  pub stabil: __syscall_slong_t,
  pub jitcnt: __syscall_slong_t,
  pub calcnt: __syscall_slong_t,
  pub errcnt: __syscall_slong_t,
  pub stbcnt: __syscall_slong_t,
  pub tai: libc::c_int,
  #[bitfield(name = "c2rust_unnamed", ty = "libc::c_int", bits = "0..=31")]
  #[bitfield(name = "c2rust_unnamed_0", ty = "libc::c_int", bits = "32..=63")]
  #[bitfield(name = "c2rust_unnamed_1", ty = "libc::c_int", bits = "64..=95")]
  #[bitfield(name = "c2rust_unnamed_2", ty = "libc::c_int", bits = "96..=127")]
  #[bitfield(name = "c2rust_unnamed_3", ty = "libc::c_int", bits = "128..=159")]
  #[bitfield(name = "c2rust_unnamed_4", ty = "libc::c_int", bits = "160..=191")]
  #[bitfield(name = "c2rust_unnamed_5", ty = "libc::c_int", bits = "192..=223")]
  #[bitfield(name = "c2rust_unnamed_6", ty = "libc::c_int", bits = "224..=255")]
  #[bitfield(name = "c2rust_unnamed_7", ty = "libc::c_int", bits = "256..=287")]
  #[bitfield(name = "c2rust_unnamed_8", ty = "libc::c_int", bits = "288..=319")]
  #[bitfield(name = "c2rust_unnamed_9", ty = "libc::c_int", bits = "320..=351")]
  pub c2rust_unnamed_c2rust_unnamed_0_c2rust_unnamed_1_c2rust_unnamed_2_c2rust_unnamed_3_c2rust_unnamed_4_c2rust_unnamed_5_c2rust_unnamed_6_c2rust_unnamed_7_c2rust_unnamed_8_c2rust_unnamed_9:
    [u8; 44],
}
pub const OPT_quiet: C2RustUnnamed = 1;
pub type C2RustUnnamed = libc::c_uint;
#[inline(always)]
unsafe extern "C" fn xatol(mut str: *const libc::c_char) -> libc::c_long {
  return xatoll(str) as libc::c_long;
}

/*
 * adjtimex.c - read, and possibly modify, the Linux kernel 'timex' variables.
 *
 * Originally written: October 1997
 * Last hack: March 2001
 * Copyright 1997, 2000, 2001 Larry Doolittle <LRDoolittle@lbl.gov>
 *
 * busyboxed 20 March 2001, Larry Doolittle <ldoolitt@recycle.lbl.gov>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config ADJTIMEX
//config:	bool "adjtimex (4.7 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	Adjtimex reads and optionally sets adjustment parameters for
//config:	the Linux clock adjustment algorithm.
//applet:IF_ADJTIMEX(APPLET_NOFORK(adjtimex, adjtimex, BB_DIR_SBIN, BB_SUID_DROP, adjtimex))
//kbuild:lib-$(CONFIG_ADJTIMEX) += adjtimex.o
//usage:#define adjtimex_trivial_usage
//usage:       "[-q] [-o OFF] [-f FREQ] [-p TCONST] [-t TICK]"
//usage:#define adjtimex_full_usage "\n\n"
//usage:       "Read or set kernel time variables. See adjtimex(2)\n"
//usage:     "\n	-q	Quiet"
//usage:     "\n	-o OFF	Time offset, microseconds"
//usage:     "\n	-f FREQ	Frequency adjust, integer kernel units (65536 is 1ppm)"
//usage:     "\n	-t TICK	Microseconds per tick, usually 10000"
//usage:     "\n		(positive -t or -f values make clock run faster)"
//usage:     "\n	-p TCONST"
static mut statlist_bit: [u16; 14] = [
  0x1i32 as u16,
  0x2i32 as u16,
  0x4i32 as u16,
  0x8i32 as u16,
  0x10i32 as u16,
  0x20i32 as u16,
  0x40i32 as u16,
  0x80i32 as u16,
  0x100i32 as u16,
  0x200i32 as u16,
  0x400i32 as u16,
  0x800i32 as u16,
  0x1000i32 as u16,
  0i32 as u16,
];
static mut statlist_name: [libc::c_char; 96] = [
  80, 76, 76, 0, 80, 80, 83, 70, 82, 69, 81, 0, 80, 80, 83, 84, 73, 77, 69, 0, 70, 70, 76, 0, 73,
  78, 83, 0, 68, 69, 76, 0, 85, 78, 83, 89, 78, 67, 0, 70, 82, 69, 81, 72, 79, 76, 68, 0, 80, 80,
  83, 83, 73, 71, 78, 65, 76, 0, 80, 80, 83, 74, 73, 84, 84, 69, 82, 0, 80, 80, 83, 87, 65, 78, 68,
  69, 82, 0, 80, 80, 83, 69, 82, 82, 79, 82, 0, 67, 76, 79, 67, 75, 69, 82, 82, 0,
];
static mut ret_code_descript: [libc::c_char; 129] = [
  99, 108, 111, 99, 107, 32, 115, 121, 110, 99, 104, 114, 111, 110, 105, 122, 101, 100, 0, 105,
  110, 115, 101, 114, 116, 32, 108, 101, 97, 112, 32, 115, 101, 99, 111, 110, 100, 0, 100, 101,
  108, 101, 116, 101, 32, 108, 101, 97, 112, 32, 115, 101, 99, 111, 110, 100, 0, 108, 101, 97, 112,
  32, 115, 101, 99, 111, 110, 100, 32, 105, 110, 32, 112, 114, 111, 103, 114, 101, 115, 115, 0,
  108, 101, 97, 112, 32, 115, 101, 99, 111, 110, 100, 32, 104, 97, 115, 32, 111, 99, 99, 117, 114,
  114, 101, 100, 0, 99, 108, 111, 99, 107, 32, 110, 111, 116, 32, 115, 121, 110, 99, 104, 114, 111,
  110, 105, 122, 101, 100, 0,
];
#[no_mangle]
pub unsafe extern "C" fn adjtimex_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opt: libc::c_uint = 0;
  let mut opt_o: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opt_f: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opt_p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opt_t: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut txc: timex =
        timex{modes: 0,
              offset: 0,
              freq: 0,
              maxerror: 0,
              esterror: 0,
              status: 0,
              constant: 0,
              precision: 0,
              tolerance: 0,
              time: timeval{tv_sec: 0, tv_usec: 0,},
              tick: 0,
              ppsfreq: 0,
              jitter: 0,
              shift: 0,
              stabil: 0,
              jitcnt: 0,
              calcnt: 0,
              errcnt: 0,
              stbcnt: 0,
              tai: 0,
              c2rust_unnamed_c2rust_unnamed_0_c2rust_unnamed_1_c2rust_unnamed_2_c2rust_unnamed_3_c2rust_unnamed_4_c2rust_unnamed_5_c2rust_unnamed_6_c2rust_unnamed_7_c2rust_unnamed_8_c2rust_unnamed_9:
                  [0; 44],};
  let mut ret: libc::c_int = 0;
  let mut descript: *const libc::c_char = 0 as *const libc::c_char;
  memset(
    &mut txc as *mut timex as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<timex>() as libc::c_ulong,
  );
  opt = getopt32(
    argv,
    b"^qo:f:p:t:\x00=0\x00" as *const u8 as *const libc::c_char,
    &mut opt_o as *mut *mut libc::c_char,
    &mut opt_f as *mut *mut libc::c_char,
    &mut opt_p as *mut *mut libc::c_char,
    &mut opt_t as *mut *mut libc::c_char,
  );
  //if (opt & 0x1) // -q
  if opt & 0x2i32 as libc::c_uint != 0 {
    // -o
    txc.offset = xatol(opt_o);
    txc.modes |= 0x8001i32 as libc::c_uint
  }
  if opt & 0x4i32 as libc::c_uint != 0 {
    // -f
    txc.freq = xatol(opt_f);
    txc.modes |= 0x2i32 as libc::c_uint
  }
  if opt & 0x8i32 as libc::c_uint != 0 {
    // -p
    txc.constant = xatol(opt_p);
    txc.modes |= 0x20i32 as libc::c_uint
  }
  if opt & 0x10i32 as libc::c_uint != 0 {
    // -t
    txc.tick = xatol(opt_t);
    txc.modes |= 0x4000i32 as libc::c_uint
  }
  /* It's NOFORK applet because the code is very simple:
   * just some printf. No opens, no allocs.
   * If you need to make it more complex, feel free to downgrade to NOEXEC
   */
  ret = adjtimex(&mut txc);
  if ret < 0i32 {
    bb_perror_nomsg_and_die();
  }
  if opt & OPT_quiet as libc::c_int as libc::c_uint == 0 {
    let mut sep: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    printf(b"    mode:         %d\n-o  offset:       %ld us\n-f  freq.adjust:  %ld (65536 = 1ppm)\n    maxerror:     %ld\n    esterror:     %ld\n    status:       %d (\x00"
                   as *const u8 as *const libc::c_char, txc.modes, txc.offset,
               txc.freq, txc.maxerror, txc.esterror, txc.status);
    /* representative output of next code fragment:
     * "PLL | PPSTIME"
     */
    name = statlist_name.as_ptr();
    sep = b"\x00" as *const u8 as *const libc::c_char;
    i = 0i32;
    while statlist_bit[i as usize] != 0 {
      if txc.status & statlist_bit[i as usize] as libc::c_int != 0 {
        printf(b"%s%s\x00" as *const u8 as *const libc::c_char, sep, name);
        sep = b" | \x00" as *const u8 as *const libc::c_char
      }
      name = name.offset(strlen(name).wrapping_add(1i32 as libc::c_ulong) as isize);
      i += 1
    }
    descript = b"error\x00" as *const u8 as *const libc::c_char;
    if ret <= 5i32 {
      descript = nth_string(ret_code_descript.as_ptr(), ret)
    }
    printf(b")\n-p  timeconstant: %ld\n    precision:    %ld us\n    tolerance:    %ld\n-t  tick:         %ld us\n    time.tv_sec:  %ld\n    time.tv_usec: %ld\n    return value: %d (%s)\n\x00"
                   as *const u8 as *const libc::c_char, txc.constant,
               txc.precision, txc.tolerance, txc.tick, txc.time.tv_sec,
               txc.time.tv_usec, ret, descript);
  }
  return 0i32;
}

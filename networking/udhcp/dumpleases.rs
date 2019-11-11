use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;


use libc;
use libc::free;
use libc::printf;
use libc::puts;
use libc::time;
extern "C" {

  #[no_mangle]
  static mut stdout: *mut FILE;

  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn ctime(__timer: *const time_t) -> *mut libc::c_char;
  #[no_mangle]
  fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  // NB: will return short read on error, not -1,
  // if some data was read before error occurred
  #[no_mangle]
  fn full_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn xread(fd: libc::c_int, buf: *mut libc::c_void, count: size_t);
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> u32;
  //UNUSED: char* FAST_FUNC unicode_conv_to_printable_maxwidth(uni_stat_t *stats, const char *src, unsigned maxwidth);
  #[no_mangle]
  fn unicode_conv_to_printable_fixedwidth(
    src: *const libc::c_char,
    width: libc::c_uint,
  ) -> *mut libc::c_char;
}

pub type __int64_t = libc::c_long;

pub type int64_t = __int64_t;
use crate::librb::size_t;
use libc::ssize_t;
use libc::time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
  pub s_addr: in_addr_t,
}
pub type in_addr_t = u32;

use libc::FILE;
/* client_data sits in 2nd half of bb_common_bufsiz1 */
pub type leasetime_t = u32;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct dyn_lease {
  pub expires: leasetime_t,
  pub lease_nip: u32,
  pub lease_mac: [u8; 6],
  pub hostname: [libc::c_char; 20],
  pub pad: [u8; 2],
  /* total size is a multiply of 4 */
}
pub const OPT_a: C2RustUnnamed = 1;
// -f
pub const OPT_d: C2RustUnnamed = 8;
pub type C2RustUnnamed = libc::c_uint;
// -d
// -r
pub const OPT_f: C2RustUnnamed = 4;
// -a
pub const OPT_r: C2RustUnnamed = 2;

/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//applet:IF_DUMPLEASES(APPLET_NOEXEC(dumpleases, dumpleases, BB_DIR_USR_BIN, BB_SUID_DROP, dumpleases))
//kbuild:lib-$(CONFIG_DUMPLEASES) += dumpleases.o
//usage:#define dumpleases_trivial_usage
//usage:       "[-r|-a] [-d] [-f LEASEFILE]"
//usage:#define dumpleases_full_usage "\n\n"
//usage:       "Display DHCP leases granted by udhcpd\n"
//usage:	IF_LONG_OPTS(
//usage:     "\n	-f,--file FILE	Lease file"
//usage:     "\n	-r,--remaining	Show remaining time"
//usage:     "\n	-a,--absolute	Show expiration time"
//usage:     "\n	-d,--decimal	Show time in seconds"
//usage:	)
//usage:	IF_NOT_LONG_OPTS(
//usage:     "\n	-f FILE	Lease file"
//usage:     "\n	-r	Show remaining time"
//usage:     "\n	-a	Show expiration time"
//usage:     "\n	-d	Show time in seconds"
//usage:	)
#[no_mangle]
pub unsafe extern "C" fn dumpleases_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut fd: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  let mut opt: libc::c_uint = 0;
  let mut written_at: int64_t = 0;
  let mut curr: int64_t = 0;
  let mut file: *const libc::c_char =
    b"/var/lib/misc/udhcpd.leases\x00" as *const u8 as *const libc::c_char;
  let mut lease: dyn_lease = dyn_lease {
    expires: 0,
    lease_nip: 0,
    lease_mac: [0; 6],
    hostname: [0; 20],
    pad: [0; 2],
  };
  static mut dumpleases_longopts: [libc::c_char; 41] = [
    97, 98, 115, 111, 108, 117, 116, 101, 0, 0, 97, 114, 101, 109, 97, 105, 110, 105, 110, 103, 0,
    0, 114, 102, 105, 108, 101, 0, 1, 102, 100, 101, 99, 105, 109, 97, 108, 0, 0, 100, 0,
  ];
  opt = getopt32long(
    argv,
    b"^arf:d\x00=0:a--r:r--a\x00" as *const u8 as *const libc::c_char,
    dumpleases_longopts.as_ptr(),
    &mut file as *mut *const libc::c_char,
  );
  fd = xopen(file, 0i32);
  /*     "123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 */
  /*     "00:00:00:00:00:00 255.255.255.255 ABCDEFGHIJKLMNOPQRS Wed Jun 30 21:49:08 1993" */

  printf(
    b"Mac %-14sIP %-13sHost %-15sExpires %s\n\x00" as *const u8 as *const libc::c_char,
    b"Address\x00" as *const u8 as *const libc::c_char,
    b"Address\x00" as *const u8 as *const libc::c_char,
    b"Name\x00" as *const u8 as *const libc::c_char,
    if opt & OPT_a as libc::c_int as libc::c_uint != 0 {
      b"at\x00" as *const u8 as *const libc::c_char
    } else {
      b"in\x00" as *const u8 as *const libc::c_char
    },
  ); /* lease file from future! :) */
  xread(
    fd,
    &mut written_at as *mut int64_t as *mut libc::c_void,
    ::std::mem::size_of::<int64_t>() as libc::c_ulong,
  );
  written_at = ({
    let mut __v: u64 = 0;
    let mut __x: u64 = written_at as u64;
    if 0 != 0 {
      __v = ((__x as libc::c_ulonglong & 0xff00000000000000u64) >> 56i32
        | (__x as libc::c_ulonglong & 0xff000000000000u64) >> 40i32
        | (__x as libc::c_ulonglong & 0xff0000000000u64) >> 24i32
        | (__x as libc::c_ulonglong & 0xff00000000u64) >> 8i32
        | (__x as libc::c_ulonglong & 0xff000000u64) << 8i32
        | (__x as libc::c_ulonglong & 0xff0000u64) << 24i32
        | (__x as libc::c_ulonglong & 0xff00u64) << 40i32
        | (__x as libc::c_ulonglong & 0xffu64) << 56i32) as u64
    } else {
      let fresh0 = &mut __v;
      let fresh1;
      let fresh2 = __x;
      asm!("bswap ${0:q}" : "=r" (fresh1) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
    }
    __v
  }) as int64_t;
  curr = time(0 as *mut time_t);
  if curr < written_at {
    written_at = curr
  }
  while full_read(
    fd,
    &mut lease as *mut dyn_lease as *mut libc::c_void,
    ::std::mem::size_of::<dyn_lease>() as libc::c_ulong,
  ) as libc::c_ulong
    == ::std::mem::size_of::<dyn_lease>() as libc::c_ulong
  {
    let mut addr: in_addr = in_addr { s_addr: 0 };
    let mut expires_abs: int64_t = 0;
    let mut fmt: *const libc::c_char = (b":%02x\x00" as *const u8 as *const libc::c_char).offset(1);
    i = 0i32;
    while i < 6i32 {
      printf(fmt, lease.lease_mac[i as usize] as libc::c_int);
      fmt = b":%02x\x00" as *const u8 as *const libc::c_char;
      i += 1
    }
    addr.s_addr = lease.lease_nip;
    let mut uni_name: *mut libc::c_char =
      unicode_conv_to_printable_fixedwidth(lease.hostname.as_mut_ptr(), 19i32 as libc::c_uint);
    printf(
      b" %-16s%s \x00" as *const u8 as *const libc::c_char,
      inet_ntoa(addr),
      uni_name,
    );
    free(uni_name as *mut libc::c_void);
    expires_abs = ({
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = lease.expires;
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
    }) as libc::c_long
      + written_at;
    if expires_abs <= curr {
      puts(b"expired\x00" as *const u8 as *const libc::c_char);
    } else if opt & OPT_d as libc::c_int as libc::c_uint != 0 {
      /* -d: decimal time */
      if opt & OPT_a as libc::c_int as libc::c_uint == 0 {
        expires_abs -= curr
      } /* -a */
      printf(
        b"%llu\n\x00" as *const u8 as *const libc::c_char,
        expires_abs as libc::c_ulonglong,
      );
    } else if opt & OPT_a as libc::c_int as libc::c_uint == 0 {
      let mut d: libc::c_uint = 0;
      let mut h: libc::c_uint = 0;
      let mut m: libc::c_uint = 0;
      let mut expires: libc::c_uint = (expires_abs - curr) as libc::c_uint;
      d = expires.wrapping_div((24i32 * 60i32 * 60i32) as libc::c_uint);
      expires = expires.wrapping_rem((24i32 * 60i32 * 60i32) as libc::c_uint);
      h = expires.wrapping_div((60i32 * 60i32) as libc::c_uint);
      expires = expires.wrapping_rem((60i32 * 60i32) as libc::c_uint);
      m = expires.wrapping_div(60i32 as libc::c_uint);
      expires = expires.wrapping_rem(60i32 as libc::c_uint);
      if d != 0 {
        printf(b"%u days \x00" as *const u8 as *const libc::c_char, d);
      }
      printf(
        b"%02u:%02u:%02u\n\x00" as *const u8 as *const libc::c_char,
        h,
        m,
        expires,
      );
    /* no -a */
    } else {
      let mut t: time_t = expires_abs;
      fputs_unlocked(ctime(&mut t), stdout);
    }
  }
  /* close(fd); */
  return 0i32;
}

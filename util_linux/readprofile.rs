use crate::librb::size_t;
use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;

use libc;
use libc::close;
use libc::printf;
use libc::sscanf;
use libc::ssize_t;
use libc::strcmp;
use libc::strcpy;
use libc::FILE;
extern "C" {

  #[no_mangle]
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut FILE,
  ) -> *mut libc::c_char;

  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn xmalloc_xopen_read_close(
    filename: *const libc::c_char,
    maxsz_p: *mut size_t,
  ) -> *mut libc::c_void;

  #[no_mangle]
  fn xwrite(fd: libc::c_int, buf: *const libc::c_void, count: size_t);

  #[no_mangle]
  fn xfopen_for_read(path: *const libc::c_char) -> *mut FILE;

  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;

  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);

  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;

  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;

}

pub type C2RustUnnamed = libc::c_uint;
pub const OPT_v: C2RustUnnamed = 512;
pub const OPT_s: C2RustUnnamed = 64;
pub const OPT_a: C2RustUnnamed = 16;
pub const OPT_b: C2RustUnnamed = 32;
pub const OPT_i: C2RustUnnamed = 128;
pub const OPT_n: C2RustUnnamed = 8;
pub const OPT_M: C2RustUnnamed = 1;
pub const OPT_r: C2RustUnnamed = 256;
// pub const OPT_p: C2RustUnnamed = 4;
// pub const OPT_m: C2RustUnnamed = 2;

/*
 * readprofile.c - used to read /proc/profile
 *
 * Copyright (C) 1994,1996 Alessandro Rubini (rubini@ipvvis.unipv.it)
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/*
 * 1999-02-22 Arkadiusz Mickiewicz <misiek@pld.ORG.PL>
 * - added Native Language Support
 * 1999-09-01 Stephane Eranian <eranian@cello.hpl.hp.com>
 * - 64bit clean patch
 * 3Feb2001 Andrew Morton <andrewm@uow.edu.au>
 * - -M option to write profile multiplier.
 * 2001-11-07 Werner Almesberger <wa@almesberger.net>
 * - byte order auto-detection and -n option
 * 2001-11-09 Werner Almesberger <wa@almesberger.net>
 * - skip step size (index 0)
 * 2002-03-09 John Levon <moz@compsoc.man.ac.uk>
 * - make maplineno do something
 * 2002-11-28 Mads Martin Joergensen +
 * - also try /boot/System.map-`uname -r`
 * 2003-04-09 Werner Almesberger <wa@almesberger.net>
 * - fixed off-by eight error and improved heuristics in byte order detection
 * 2003-08-12 Nikita Danilov <Nikita@Namesys.COM>
 * - added -s option; example of use:
 * "readprofile -s -m /boot/System.map-test | grep __d_lookup | sort -n -k3"
 *
 * Taken from util-linux and adapted for busybox by
 * Paul Mundt <lethal@linux-sh.org>.
 */
//config:config READPROFILE
//config:	bool "readprofile (7.1 kb)"
//config:	default y
//config:	#select PLATFORM_LINUX
//config:	help
//config:	This allows you to parse /proc/profile for basic profiling.
//applet:IF_READPROFILE(APPLET(readprofile, BB_DIR_USR_SBIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_READPROFILE) += readprofile.o
//usage:#define readprofile_trivial_usage
//usage:       "[OPTIONS]"
//usage:#define readprofile_full_usage "\n\n"
//usage:       "	-m mapfile	(Default: /boot/System.map)"
//usage:     "\n	-p profile	(Default: /proc/profile)"
//usage:     "\n	-M NUM		Set the profiling multiplier to NUM"
//usage:     "\n	-i		Print only info about the sampling step"
//usage:     "\n	-v		Verbose"
//usage:     "\n	-a		Print all symbols, even if count is 0"
//usage:     "\n	-b		Print individual histogram-bin counts"
//usage:     "\n	-s		Print individual counters within functions"
//usage:     "\n	-r		Reset all the counters (root only)"
//usage:     "\n	-n		Disable byte order auto-detection"
#[no_mangle]
pub unsafe extern "C" fn readprofile_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut map: *mut FILE = 0 as *mut FILE; /* current and next address */
  let mut mapFile: *const libc::c_char = 0 as *const libc::c_char; /* current and next name */
  let mut proFile: *const libc::c_char = 0 as *const libc::c_char;
  let mut indx: libc::c_ulong = 0;
  let mut len: size_t = 0;
  let mut add0: u64 = 0;
  let mut step: libc::c_uint = 0;
  let mut buf: *mut libc::c_uint = 0 as *mut libc::c_uint;
  let mut total: libc::c_uint = 0;
  let mut fn_len: libc::c_uint = 0;
  let mut fn_add: libc::c_ulonglong = 0;
  let mut next_add: libc::c_ulonglong = 0;
  let mut fn_name: [libc::c_char; 128] = [0; 128];
  let mut next_name: [libc::c_char; 128] = [0; 128];
  let mut mapline: [libc::c_char; 128] = [0; 128];
  let mut mode: [libc::c_char; 8] = [0; 8];
  let mut maplineno: libc::c_int = 0;
  let mut multiplier: libc::c_int = 0;
  let mut opt: libc::c_uint = 0;
  proFile = b"/proc/profile\x00" as *const u8 as *const libc::c_char;
  mapFile = b"/boot/System.map\x00" as *const u8 as *const libc::c_char;
  multiplier = 0i32;
  opt = getopt32(
    argv,
    b"M:+m:p:nabsirv\x00" as *const u8 as *const libc::c_char,
    &mut multiplier as *mut libc::c_int,
    &mut mapFile as *mut *const libc::c_char,
    &mut proFile as *mut *const libc::c_char,
  );
  if opt & (OPT_M as libc::c_int | OPT_r as libc::c_int) as libc::c_uint != 0 {
    /* mult or reset, or both */
    let mut fd: libc::c_int = 0;
    let mut to_write: libc::c_int = 0;
    /*
     * When writing the multiplier, if the length of the write is
     * not sizeof(int), the multiplier is not changed
     */
    to_write = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int; /* sth different from sizeof(int) */
    if opt & OPT_M as libc::c_int as libc::c_uint == 0 {
      to_write = 1i32
    }
    fd = xopen(
      b"/proc/profile\x00" as *const u8 as *const libc::c_char,
      0o1i32,
    );
    xwrite(
      fd,
      &mut multiplier as *mut libc::c_int as *const libc::c_void,
      to_write as size_t,
    );
    close(fd);
    return 0i32;
  }
  /*
   * Use an fd for the profiling buffer, to skip stdio overhead
   */
  len = if -1i32 as ssize_t > 0 {
    -1i32 as ssize_t
  } else {
    !((1i32 as ssize_t)
      << (::std::mem::size_of::<ssize_t>() as libc::c_ulong)
        .wrapping_mul(8i32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong))
  } as size_t;
  buf = xmalloc_xopen_read_close(proFile, &mut len) as *mut libc::c_uint;
  len = (len as libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
    as size_t as size_t;
  if opt & OPT_n as libc::c_int as libc::c_uint == 0 {
    let mut big: libc::c_int = 0i32;
    let mut small: libc::c_int = 0i32;
    let mut p: *mut libc::c_uint = 0 as *mut libc::c_uint;
    p = buf.offset(1);
    while p < buf.offset(len as isize) {
      if *p
        & !0u32
          << (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(4i32 as libc::c_ulong)
        != 0
      {
        big += 1
      }
      if *p
        & ((1i32
          << (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(4i32 as libc::c_ulong))
          - 1i32) as libc::c_uint
        != 0
      {
        small += 1
      }
      p = p.offset(1)
    }
    if big > small {
      bb_simple_error_msg(
        b"assuming reversed byte order, use -n to force native byte order\x00" as *const u8
          as *const libc::c_char,
      );
      p = buf;
      while p < buf.offset(len as isize) {
        if ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong == 2i32 as libc::c_ulong {
          *p = ({
            let mut __v: libc::c_ushort = 0;
            let mut __x: libc::c_ushort = *p as libc::c_ushort;
            if 0 != 0 {
              __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
                as libc::c_ushort
            } else {
              let fresh0 = &mut __v;
              let fresh1;
              let fresh2 = __x;
              asm!("rorw $$8, ${0:w}" : "=r" (fresh1) : "0"
                                      (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2))
                                      : "cc");
              c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
            }
            __v
          }) as libc::c_uint
        }
        if ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong == 4i32 as libc::c_ulong {
          *p = {
            let mut __v: libc::c_uint = 0;
            let mut __x: libc::c_uint = *p;
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
          }
        }
        if ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong == 8i32 as libc::c_ulong {
          *p = ({
            let mut __v: u64 = 0;
            let mut __x: u64 = *p as u64;
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
              let fresh6 = &mut __v;
              let fresh7;
              let fresh8 = __x;
              asm!("bswap ${0:q}" : "=r" (fresh7) : "0"
                                      (c2rust_asm_casts::AsmCast::cast_in(fresh6, fresh8))
                                      :);
              c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
            }
            __v
          }) as libc::c_uint
        }
        p = p.offset(1)
      }
    }
  }
  step = *buf.offset(0);
  if opt & OPT_i as libc::c_int as libc::c_uint != 0 {
    printf(
      b"Sampling_step: %u\n\x00" as *const u8 as *const libc::c_char,
      step,
    );
    return 0i32;
  }
  map = xfopen_for_read(mapFile);
  add0 = 0i32 as u64;
  maplineno = 1i32;
  while !fgets_unlocked(mapline.as_mut_ptr(), 128i32, map).is_null() {
    if sscanf(
      mapline.as_mut_ptr(),
      b"%llx %s %s\x00" as *const u8 as *const libc::c_char,
      &mut fn_add as *mut libc::c_ulonglong,
      mode.as_mut_ptr(),
      fn_name.as_mut_ptr(),
    ) != 3i32
    {
      bb_error_msg_and_die(
        b"%s(%i): wrong map line\x00" as *const u8 as *const libc::c_char,
        mapFile,
        maplineno,
      );
    }
    if strcmp(
      fn_name.as_mut_ptr(),
      b"_stext\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
      /* only elf works like this */
      add0 = fn_add as u64;
      break;
    } else {
      maplineno += 1
    }
  }
  if add0 == 0 {
    bb_error_msg_and_die(
      b"can\'t find \"_stext\" in %s\x00" as *const u8 as *const libc::c_char,
      mapFile,
    );
  }
  /*
   * Main loop.
   */
  total = 0i32 as libc::c_uint;
  indx = 1i32 as libc::c_ulong;
  while !fgets_unlocked(mapline.as_mut_ptr(), 128i32, map).is_null() {
    let mut header_printed: bool = false;
    let mut this: libc::c_uint = 0;
    if sscanf(
      mapline.as_mut_ptr(),
      b"%llx %s %s\x00" as *const u8 as *const libc::c_char,
      &mut next_add as *mut libc::c_ulonglong,
      mode.as_mut_ptr(),
      next_name.as_mut_ptr(),
    ) != 3i32
    {
      bb_error_msg_and_die(
        b"%s(%i): wrong map line\x00" as *const u8 as *const libc::c_char,
        mapFile,
        maplineno,
      );
    }
    header_printed = 0i32 != 0;
    /* ignore any LEADING (before a '[tT]' symbol is found)
    Absolute symbols */
    if (mode[0] as libc::c_int == 'A' as i32 || mode[0] as libc::c_int == '?' as i32)
      && total == 0i32 as libc::c_uint
    {
      continue;
    }
    if mode[0] as libc::c_int | 0x20i32 != 't' as i32
      && mode[0] as libc::c_int | 0x20i32 != 'w' as i32
    {
      break;
    }
    if indx >= len {
      bb_simple_error_msg_and_die(
        b"profile address out of range. Wrong map file?\x00" as *const u8 as *const libc::c_char,
      );
    }
    this = 0i32 as libc::c_uint;
    while (indx as libc::c_ulonglong)
      < next_add
        .wrapping_sub(add0 as libc::c_ulonglong)
        .wrapping_div(step as libc::c_ulonglong)
    {
      if opt & OPT_b as libc::c_int as libc::c_uint != 0
        && (*buf.offset(indx as isize) != 0 || opt & OPT_a as libc::c_int as libc::c_uint != 0)
      {
        if !header_printed {
          printf(
            b"%s:\n\x00" as *const u8 as *const libc::c_char,
            fn_name.as_mut_ptr(),
          );
          header_printed = 1i32 != 0
        }
        printf(
          b"\t%lx\t%u\n\x00" as *const u8 as *const libc::c_char,
          indx
            .wrapping_sub(1i32 as libc::c_ulong)
            .wrapping_mul(step as libc::c_ulong)
            .wrapping_add(add0),
          *buf.offset(indx as isize),
        );
      }
      let fresh9 = indx;
      indx = indx.wrapping_add(1);
      this = this.wrapping_add(*buf.offset(fresh9 as isize))
    }
    total = total.wrapping_add(this);
    if opt & OPT_b as libc::c_int as libc::c_uint != 0 {
      if opt & OPT_v as libc::c_int as libc::c_uint != 0 || this > 0i32 as libc::c_uint {
        printf(
          b"  total\t\t\t\t%u\n\x00" as *const u8 as *const libc::c_char,
          this,
        );
      }
    } else if (this != 0 || opt & OPT_a as libc::c_int as libc::c_uint != 0) && {
      fn_len = next_add.wrapping_sub(fn_add) as libc::c_uint;
      (fn_len) != 0i32 as libc::c_uint
    } {
      if opt & OPT_v as libc::c_int as libc::c_uint != 0 {
        printf(
          b"%016llx %-40s %6u %8.4f\n\x00" as *const u8 as *const libc::c_char,
          fn_add,
          fn_name.as_mut_ptr(),
          this,
          this as libc::c_double / fn_len as libc::c_double,
        );
      } else {
        printf(
          b"%6u %-40s %8.4f\n\x00" as *const u8 as *const libc::c_char,
          this,
          fn_name.as_mut_ptr(),
          this as libc::c_double / fn_len as libc::c_double,
        );
      }
      if opt & OPT_s as libc::c_int as libc::c_uint != 0 {
        let mut scan: libc::c_ulonglong = 0;
        scan = fn_add
          .wrapping_sub(add0 as libc::c_ulonglong)
          .wrapping_div(step as libc::c_ulonglong)
          .wrapping_add(1i32 as libc::c_ulonglong);
        while scan
          < next_add
            .wrapping_sub(add0 as libc::c_ulonglong)
            .wrapping_div(step as libc::c_ulonglong)
        {
          let mut addr: libc::c_ulonglong = 0;
          addr = scan
            .wrapping_sub(1i32 as libc::c_ulonglong)
            .wrapping_mul(step as libc::c_ulonglong)
            .wrapping_add(add0 as libc::c_ulonglong);
          printf(
            b"\t%#llx\t%s+%#llx\t%u\n\x00" as *const u8 as *const libc::c_char,
            addr,
            fn_name.as_mut_ptr(),
            addr.wrapping_sub(fn_add),
            *buf.offset(scan as isize),
          );
          scan = scan.wrapping_add(1)
        }
      }
    }
    fn_add = next_add;
    strcpy(fn_name.as_mut_ptr(), next_name.as_mut_ptr());
    maplineno += 1
  }
  /* clock ticks, out of kernel text - probably modules */
  printf(
    b"%6u *unknown*\n\x00" as *const u8 as *const libc::c_char,
    *buf.offset(len.wrapping_sub(1i32 as libc::c_ulong) as isize),
  );
  /* trailer */
  if opt & OPT_v as libc::c_int as libc::c_uint != 0 {
    printf(
      b"%016x %-40s %6u %8.4f\n\x00" as *const u8 as *const libc::c_char,
      0i32,
      b"total\x00" as *const u8 as *const libc::c_char,
      total,
      total as libc::c_double / fn_add.wrapping_sub(add0 as libc::c_ulonglong) as libc::c_double,
    );
  } else {
    printf(
      b"%6u %-40s %8.4f\n\x00" as *const u8 as *const libc::c_char,
      total,
      b"total\x00" as *const u8 as *const libc::c_char,
      total as libc::c_double / fn_add.wrapping_sub(add0 as libc::c_ulonglong) as libc::c_double,
    );
  }
  return 0i32;
}

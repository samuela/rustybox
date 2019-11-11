use crate::libbb::ptr_to_globals::bb_errno;
use libc;
use libc::printf;
use libc::puts;
extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn xatoi_positive(numstr: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn procps_read_smaps(
    pid: pid_t,
    total: *mut smaprec,
    cb: Option<unsafe extern "C" fn(_: *mut smaprec, _: *mut libc::c_void) -> ()>,
    data: *mut libc::c_void,
  ) -> libc::c_int;
  #[no_mangle]
  fn read_cmdline(
    buf: *mut libc::c_char,
    size: libc::c_int,
    pid: libc::c_uint,
    comm: *const libc::c_char,
  );
}

pub type uintptr_t = libc::c_ulong;
use libc::pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct smaprec {
  pub mapped_rw: libc::c_ulong,
  pub mapped_ro: libc::c_ulong,
  pub shared_clean: libc::c_ulong,
  pub shared_dirty: libc::c_ulong,
  pub private_clean: libc::c_ulong,
  pub private_dirty: libc::c_ulong,
  pub stack: libc::c_ulong,
  pub smap_pss: libc::c_ulong,
  pub smap_swap: libc::c_ulong,
  pub smap_size: libc::c_ulong,
  pub smap_start: libc::c_ulonglong,
  pub smap_mode: [libc::c_char; 5],
  pub smap_name: *mut libc::c_char,
}
/*
 * pmap implementation for busybox
 *
 * Copyright (C) 2010 Nokia Corporation. All rights reserved.
 * Written by Alexander Shishkin <virtuoso@slind.org>
 *
 * Licensed under GPLv2 or later, see the LICENSE file in this source tree
 * for details.
 */
//config:config PMAP
//config:	bool "pmap (6 kb)"
//config:	default y
//config:	help
//config:	Display processes' memory mappings.
//applet:IF_PMAP(APPLET(pmap, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_PMAP) += pmap.o
//usage:#define pmap_trivial_usage
//usage:       "[-xq] PID..."
//usage:#define pmap_full_usage "\n\n"
//usage:       "Display process memory usage"
//usage:     "\n"
//usage:     "\n	-x	Show details"
//usage:     "\n	-q	Quiet"
pub type C2RustUnnamed = libc::c_uint;
pub const OPT_q: C2RustUnnamed = 2;
pub const OPT_x: C2RustUnnamed = 1;
unsafe extern "C" fn print_smaprec(mut currec: *mut smaprec, mut data: *mut libc::c_void) {
  let mut opt: libc::c_uint = data as uintptr_t as libc::c_uint; /* min one arg */
  printf(
    b"%016llx \x00" as *const u8 as *const libc::c_char,
    (*currec).smap_start,
  );
  if opt & OPT_x as libc::c_int as libc::c_uint != 0 {
    printf(
      b"%7lu %7lu %7lu %7lu \x00" as *const u8 as *const libc::c_char,
      (*currec).smap_size,
      (*currec).smap_pss,
      (*currec).private_dirty,
      (*currec).smap_swap,
    );
  } else {
    printf(
      b"%7luK\x00" as *const u8 as *const libc::c_char,
      (*currec).smap_size,
    );
  }
  printf(
    b" %.4s  %s\n\x00" as *const u8 as *const libc::c_char,
    (*currec).smap_mode.as_mut_ptr(),
    (*currec).smap_name,
  );
}
unsafe extern "C" fn procps_get_maps(mut pid: pid_t, mut opt: libc::c_uint) -> libc::c_int {
  let mut total: smaprec = smaprec {
    mapped_rw: 0,
    mapped_ro: 0,
    shared_clean: 0,
    shared_dirty: 0,
    private_clean: 0,
    private_dirty: 0,
    stack: 0,
    smap_pss: 0,
    smap_swap: 0,
    smap_size: 0,
    smap_start: 0,
    smap_mode: [0; 5],
    smap_name: 0 as *mut libc::c_char,
  };
  let mut ret: libc::c_int = 0;
  let mut buf: [libc::c_char; 256] = [0; 256];
  read_cmdline(
    buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    pid as libc::c_uint,
    0 as *const libc::c_char,
  );
  printf(
    b"%u: %s\n\x00" as *const u8 as *const libc::c_char,
    pid,
    buf.as_mut_ptr(),
  );
  if opt & OPT_q as libc::c_int as libc::c_uint == 0
    && opt & OPT_x as libc::c_int as libc::c_uint != 0
  {
    puts(
      b"Address\t\t  Kbytes     PSS   Dirty    Swap  Mode  Mapping\x00" as *const u8
        as *const libc::c_char,
    );
  }
  memset(
    &mut total as *mut smaprec as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<smaprec>() as libc::c_ulong,
  );
  ret = procps_read_smaps(
    pid,
    &mut total,
    Some(print_smaprec as unsafe extern "C" fn(_: *mut smaprec, _: *mut libc::c_void) -> ()),
    opt as uintptr_t as *mut libc::c_void,
  );
  if ret != 0 {
    return ret;
  }
  if opt & OPT_q as libc::c_int as libc::c_uint == 0 {
    if opt & OPT_x as libc::c_int as libc::c_uint != 0 {
      printf(
        b"----------------  ------  ------  ------  ------\ntotal\t\t %7lu %7lu %7lu %7lu\n\x00"
          as *const u8 as *const libc::c_char,
        total.smap_size,
        total.smap_pss,
        total.private_dirty,
        total.smap_swap,
      );
    } else {
      printf(
        b"mapped: %luK\n\x00" as *const u8 as *const libc::c_char,
        total.smap_size,
      );
    }
  }
  return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn pmap_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opts: libc::c_uint = 0;
  let mut ret: libc::c_int = 0;
  opts = getopt32(argv, b"^xq\x00-1\x00" as *const u8 as *const libc::c_char);
  argv = argv.offset(optind as isize);
  ret = 0i32;
  while !(*argv).is_null() {
    let fresh0 = argv;
    argv = argv.offset(1);
    let mut pid: pid_t = xatoi_positive(*fresh0);
    /* GNU pmap returns 42 if any of the pids failed */
    if procps_get_maps(pid, opts) != 0i32 {
      ret = 42i32
    }
  }
  return ret;
}

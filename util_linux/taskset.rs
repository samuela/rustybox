use crate::librb::size_t;
use libc;
use libc::pid_t;

extern "C" {
  #[no_mangle]
  fn sched_getaffinity(__pid: pid_t, __cpusetsize: size_t, __cpuset: *mut cpu_set_t)
    -> libc::c_int;
  #[no_mangle]
  fn sched_setaffinity(
    __pid: pid_t,
    __cpusetsize: size_t,
    __cpuset: *const cpu_set_t,
  ) -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrtoull(str: *const libc::c_char, b: libc::c_int) -> libc::c_ulonglong;
  #[no_mangle]
  fn xatoull_range(
    str: *const libc::c_char,
    l: libc::c_ulonglong,
    u: libc::c_ulonglong,
  ) -> libc::c_ulonglong;
  #[no_mangle]
  fn BB_EXECVP_or_die(argv: *mut *mut libc::c_char) -> !;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
}

pub type __cpu_mask = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpu_set_t {
  pub __bits: [__cpu_mask; 16],
}

/*
 * taskset - retrieve or set a processes' CPU affinity
 * Copyright (c) 2006 Bernhard Reutner-Fischer
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config TASKSET
//config:	bool "taskset (4.2 kb)"
//config:	default y
//config:	help
//config:	Retrieve or set a processes's CPU affinity.
//config:	This requires sched_{g,s}etaffinity support in your libc.
//config:
//config:config FEATURE_TASKSET_FANCY
//config:	bool "Fancy output"
//config:	default y
//config:	depends on TASKSET
//config:	help
//config:	Needed for machines with more than 32-64 CPUs:
//config:	affinity parameter 0xHHHHHHHHHHHHHHHHHHHH can be arbitrarily long
//config:	in this case. Otherwise, it is limited to sizeof(long).
//applet:IF_TASKSET(APPLET_NOEXEC(taskset, taskset, BB_DIR_USR_BIN, BB_SUID_DROP, taskset))
//kbuild:lib-$(CONFIG_TASKSET) += taskset.o
//usage:#define taskset_trivial_usage
//usage:       "[-p] [HEXMASK] PID | PROG ARGS"
//usage:#define taskset_full_usage "\n\n"
//usage:       "Set or get CPU affinity\n"
//usage:     "\n	-p	Operate on an existing PID"
//usage:
//usage:#define taskset_example_usage
//usage:       "$ taskset 0x7 ./dgemm_test&\n"
//usage:       "$ taskset -p 0x1 $!\n"
//usage:       "pid 4790's current affinity mask: 7\n"
//usage:       "pid 4790's new affinity mask: 1\n"
//usage:       "$ taskset 0x7 /bin/sh -c './taskset -p 0x1 $$'\n"
//usage:       "pid 6671's current affinity mask: 1\n"
//usage:       "pid 6671's new affinity mask: 1\n"
//usage:       "$ taskset -p 1\n"
//usage:       "pid 1's current affinity mask: 3\n"
/*
 * Not yet implemented:
 * -a/--all-tasks (affect all threads)
 *	needs to get TIDs from /proc/PID/task/ and use _them_ as "pid" in sched_setaffinity(pid)
 * -c/--cpu-list  (specify CPUs via "1,3,5-7")
 */
pub type ul = libc::c_ulong;
#[inline(always)]
unsafe extern "C" fn xstrtoul(mut str: *const libc::c_char, mut b: libc::c_int) -> libc::c_ulong {
  return xstrtoull(str, b) as libc::c_ulong;
}
#[inline(always)]
unsafe extern "C" fn xatoul_range(
  mut str: *const libc::c_char,
  mut l: libc::c_ulong,
  mut u: libc::c_ulong,
) -> libc::c_ulong {
  return xatoull_range(str, l as libc::c_ulonglong, u as libc::c_ulonglong) as libc::c_ulong;
}
/* craft a string from the mask */
unsafe extern "C" fn from_mask(
  mut mask: *const ul,
  mut sz_in_bytes: libc::c_uint,
) -> *mut libc::c_char {
  let mut str: *mut libc::c_char = xzalloc(
    sz_in_bytes
      .wrapping_add(1i32 as libc::c_uint)
      .wrapping_mul(2i32 as libc::c_uint) as size_t,
  ) as *mut libc::c_char; /* we will leak it */
  let mut p: *mut libc::c_char = str; /* :) */
  loop {
    let fresh0 = mask;
    mask = mask.offset(1);
    let mut v: ul = *fresh0;
    if ::std::mem::size_of::<ul>() as libc::c_ulong as libc::c_uint == 4i32 as libc::c_uint {
      p = p.offset(sprintf(p, b"%08lx\x00" as *const u8 as *const libc::c_char, v) as isize)
    }
    if ::std::mem::size_of::<ul>() as libc::c_ulong as libc::c_uint == 8i32 as libc::c_uint {
      p = p.offset(sprintf(p, b"%016lx\x00" as *const u8 as *const libc::c_char, v) as isize)
    }
    if ::std::mem::size_of::<ul>() as libc::c_ulong as libc::c_uint == 16i32 as libc::c_uint {
      p = p.offset(sprintf(p, b"%032lx\x00" as *const u8 as *const libc::c_char, v) as isize)
    }
    sz_in_bytes =
      sz_in_bytes.wrapping_sub(::std::mem::size_of::<ul>() as libc::c_ulong as libc::c_uint);
    if sz_in_bytes as libc::c_int <= 0i32 {
      break;
    }
  }
  while *str.offset(0) as libc::c_int == '0' as i32 && *str.offset(1) as libc::c_int != 0 {
    str = str.offset(1)
  }
  return str;
}
unsafe extern "C" fn get_aff(
  mut pid: libc::c_int,
  mut sz: *mut libc::c_uint,
) -> *mut libc::c_ulong {
  let mut r: libc::c_int = 0;
  let mut mask: *mut libc::c_ulong = 0 as *mut libc::c_ulong;
  let mut sz_in_bytes: libc::c_uint = *sz;
  loop {
    mask = xrealloc(mask as *mut libc::c_void, sz_in_bytes as size_t) as *mut libc::c_ulong;
    r = sched_getaffinity(
      pid,
      sz_in_bytes as size_t,
      mask as *mut libc::c_void as *mut cpu_set_t,
    );
    if r == 0i32 {
      break;
    }
    sz_in_bytes = sz_in_bytes.wrapping_mul(2i32 as libc::c_uint);
    if *bb_errno == 22i32 && sz_in_bytes as libc::c_int > 0i32 {
      continue;
    }
    bb_perror_msg_and_die(
      b"can\'t %cet pid %d\'s affinity\x00" as *const u8 as *const libc::c_char,
      'g' as i32,
      pid,
    );
  }
  //bb_error_msg("get mask[0]:%lx sz_in_bytes:%d", mask[0], sz_in_bytes);
  *sz = sz_in_bytes;
  return mask;
}
#[no_mangle]
pub unsafe extern "C" fn taskset_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut mask: *mut ul = 0 as *mut ul;
  let mut mask_size_in_bytes: libc::c_uint = 0;
  let mut pid: pid_t = 0i32;
  let mut opt_p: libc::c_uint = 0;
  let mut current_new: *const libc::c_char = 0 as *const libc::c_char;
  let mut aff: *mut libc::c_char = 0 as *mut libc::c_char;
  /* NB: we mimic util-linux's taskset: -p does not take
   * an argument, i.e., "-pN" is NOT valid, only "-p N"!
   * Indeed, util-linux-2.13-pre7 uses:
   * getopt_long(argc, argv, "+pchV", ...), not "...p:..." */
  opt_p = getopt32(argv, b"^+p\x00-1\x00" as *const u8 as *const libc::c_char);
  argv = argv.offset(optind as isize);
  let fresh1 = argv;
  argv = argv.offset(1);
  aff = *fresh1;
  if opt_p != 0 {
    let mut pid_str: *mut libc::c_char = aff;
    if !(*argv).is_null() {
      /* "-p <aff> <pid> ...rest.is.ignored..." */
      pid_str = *argv
      /* NB: *argv != NULL in this case */
    }
    /* else it was just "-p <pid>", and *argv == NULL */
    pid = xatoul_range(
      pid_str,
      1i32 as libc::c_ulong,
      ((9223372036854775807i64 as libc::c_ulong)
        .wrapping_mul(2u64)
        .wrapping_add(1u64) as pid_t as libc::c_uint
        >> 1i32) as libc::c_ulong,
    ) as pid_t
  } else if (*argv).is_null() {
    bb_show_usage();
  }
  mask_size_in_bytes = ::std::mem::size_of::<ul>() as libc::c_ulong as libc::c_uint;
  current_new = b"current\x00" as *const u8 as *const libc::c_char;
  loop
  /* <aff> <cmd...> */
  /* "-p <aff> <pid> [...ignored...]" */
  {
    mask = get_aff(pid, &mut mask_size_in_bytes);
    if opt_p != 0 {
      printf(
        b"pid %d\'s %s affinity mask: %s\n\x00" as *const u8 as *const libc::c_char,
        pid,
        current_new,
        from_mask(mask, mask_size_in_bytes),
      );
      if (*argv).is_null() {
        /* Either it was just "-p <pid>",
         * or it was "-p <aff> <pid>" and we came here
         * for the second time (see goto below) */
        return 0i32;
      }
      *argv = 0 as *mut libc::c_char;
      current_new = b"new\x00" as *const u8 as *const libc::c_char
    }
    memset(
      mask as *mut libc::c_void,
      0i32,
      mask_size_in_bytes as libc::c_ulong,
    );
    /* Affinity was specified, translate it into mask */
    /* it is always in hex, skip "0x" if it exists */
    if *aff.offset(0) as libc::c_int == '0' as i32
      && *aff.offset(1) as libc::c_int | 0x20i32 == 'x' as i32
    {
      aff = aff.offset(2)
    } /* bit pos in mask[] */
    if 1i32 == 0 {
      *mask.offset(0) = xstrtoul(aff, 16i32)
    } else {
      let mut i: libc::c_uint = 0;
      let mut last_char: *mut libc::c_char = 0 as *mut libc::c_char;
      i = 0i32 as libc::c_uint;
      /* aff is ASCII hex string, accept very long masks in this form.
       * Process hex string AABBCCDD... to ulong mask[]
       * from the rightmost nibble, which is least-significant.
       * Bits not fitting into mask[] are ignored: (example: 1234
       * in 12340000000000000000000000000000000000000ff)
       */
      last_char = strchrnul(aff, '\u{0}' as i32);
      while last_char > aff {
        let mut c: libc::c_char = 0;
        let mut val: ul = 0;
        last_char = last_char.offset(-1);
        c = *last_char;
        if (c as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
          val = (c as libc::c_int - '0' as i32) as ul
        } else if c as libc::c_int | 0x20i32 >= 'a' as i32
          && c as libc::c_int | 0x20i32 <= 'f' as i32
        {
          val = ((c as libc::c_int | 0x20i32) - ('a' as i32 - 10i32)) as ul
        } else {
          bb_error_msg_and_die(
            b"bad affinity \'%s\'\x00" as *const u8 as *const libc::c_char,
            aff,
          );
        }
        if i < mask_size_in_bytes.wrapping_mul(8i32 as libc::c_uint) {
          let ref mut fresh2 = *mask.offset(i.wrapping_div(
            (::std::mem::size_of::<ul>() as libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong)
              as libc::c_uint,
          ) as isize);
          *fresh2 |= val
            << (i
              & (::std::mem::size_of::<ul>() as libc::c_ulong)
                .wrapping_mul(8i32 as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong) as libc::c_uint)
          //bb_error_msg("bit %d set", i);
        }
        /* else:
         * We can error out here, but we don't.
         * For one, kernel itself ignores bits in mask[]
         * which do not map to any CPUs:
         * if mask[] has one 32-bit long element,
         * but you have only 8 CPUs, all bits beyond first 8
         * are ignored, silently.
         * No point in making bits past 31th to be errors.
         */
        i = i.wrapping_add(4i32 as libc::c_uint)
      }
    }
    /* Set pid's or our own (pid==0) affinity */
    if sched_setaffinity(
      pid,
      mask_size_in_bytes as size_t,
      mask as *mut libc::c_void as *const cpu_set_t,
    ) != 0
    {
      bb_perror_msg_and_die(
        b"can\'t %cet pid %d\'s affinity\x00" as *const u8 as *const libc::c_char,
        's' as i32,
        pid,
      );
    }
    //bb_error_msg("set mask[0]:%lx", mask[0]);
    if !(*argv.offset(0)).is_null() {
      break; /* print new affinity and exit */
    }
  }
  BB_EXECVP_or_die(argv);
}

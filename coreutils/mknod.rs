use libc;
extern "C" {
  #[no_mangle]
  fn gnu_dev_major(__dev: __dev_t) -> libc::c_uint;
  #[no_mangle]
  fn gnu_dev_minor(__dev: __dev_t) -> libc::c_uint;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn mknod(__path: *const libc::c_char, __mode: __mode_t, __dev: __dev_t) -> libc::c_int;
  #[no_mangle]
  fn xatoull_range(
    str: *const libc::c_char,
    l: libc::c_ulonglong,
    u: libc::c_ulonglong,
  ) -> libc::c_ulonglong;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  /* At least glibc has horrendously large inline for this, so wrap it */
  #[no_mangle]
  fn bb_makedev(major: libc::c_uint, minor: libc::c_uint) -> libc::c_ulonglong;
  #[no_mangle]
  fn getopt_mk_fifo_nod(argv: *mut *mut libc::c_char) -> mode_t;
}
use crate::librb::__dev_t;
use crate::librb::__mode_t;
use crate::librb::mode_t;
use crate::librb::dev_t;
#[inline(always)]
unsafe extern "C" fn xatoul_range(
  mut str: *const libc::c_char,
  mut l: libc::c_ulong,
  mut u: libc::c_ulong,
) -> libc::c_ulong {
  return xatoull_range(str, l as libc::c_ulonglong, u as libc::c_ulonglong) as libc::c_ulong;
}

/*
 * mknod implementation for busybox
 *
 * Copyright (C) 2003  Manuel Novoa III  <mjn3@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config MKNOD
//config:	bool "mknod (4.5 kb)"
//config:	default y
//config:	help
//config:	mknod is used to create FIFOs or block/character special
//config:	files with the specified names.
//applet:IF_MKNOD(APPLET_NOEXEC(mknod, mknod, BB_DIR_BIN, BB_SUID_DROP, mknod))
//kbuild:lib-$(CONFIG_MKNOD) += mknod.o
/* BB_AUDIT SUSv3 N/A -- Matches GNU behavior. */
//usage:#define mknod_trivial_usage
//usage:       "[-m MODE] " IF_SELINUX("[-Z] ") "NAME TYPE [MAJOR MINOR]"
//usage:#define mknod_full_usage "\n\n"
//usage:       "Create a special file (block, character, or pipe)\n"
//usage:     "\n	-m MODE	Creation mode (default a=rw)"
//usage:	IF_SELINUX(
//usage:     "\n	-Z	Set security context"
//usage:	)
//usage:     "\nTYPE:"
//usage:     "\n	b	Block device"
//usage:     "\n	c or u	Character device"
//usage:     "\n	p	Named pipe (MAJOR MINOR must be omitted)"
//usage:
//usage:#define mknod_example_usage
//usage:       "$ mknod /dev/fd0 b 2 0\n"
//usage:       "$ mknod -m 644 /tmp/pipe p\n"
// For makedev
/* This is a NOEXEC applet. Be very careful! */
static mut modes_chars: [libc::c_char; 8] = [
  'p' as i32 as libc::c_char,
  'c' as i32 as libc::c_char,
  'u' as i32 as libc::c_char,
  'b' as i32 as libc::c_char,
  0i32 as libc::c_char,
  1i32 as libc::c_char,
  1i32 as libc::c_char,
  2i32 as libc::c_char,
];
static mut modes_cubp: [mode_t; 3] = [
  0o10000i32 as mode_t,
  0o20000i32 as mode_t,
  0o60000i32 as mode_t,
];
#[no_mangle]
pub unsafe extern "C" fn mknod_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut mode: mode_t = 0;
  let mut dev: dev_t = 0;
  let mut type_0: *const libc::c_char = 0 as *const libc::c_char;
  let mut arg: *const libc::c_char = 0 as *const libc::c_char;
  mode = getopt_mk_fifo_nod(argv);
  argv = argv.offset(optind as isize);
  //argc -= optind;
  if (*argv.offset(0)).is_null() || (*argv.offset(1)).is_null() {
    bb_show_usage();
  }
  type_0 = strchr(
    modes_chars.as_ptr(),
    *(*argv.offset(1)).offset(0) as libc::c_int,
  );
  if type_0.is_null() {
    bb_show_usage();
  }
  mode |= modes_cubp[*type_0.offset(4) as libc::c_int as usize];
  dev = 0i32 as dev_t;
  arg = *argv.offset(2);
  if *type_0 as libc::c_int != 'p' as i32 {
    if (*argv.offset(2)).is_null() || (*argv.offset(3)).is_null() {
      bb_show_usage();
    }
    /* Autodetect what the system supports; these macros should
     * optimize out to two constants. */
    dev = bb_makedev(
      xatoul_range(
        *argv.offset(2),
        0i32 as libc::c_ulong,
        gnu_dev_major(
          (2147483647i32 as libc::c_uint)
            .wrapping_mul(2u32)
            .wrapping_add(1u32) as __dev_t,
        ) as libc::c_ulong,
      ) as libc::c_uint,
      xatoul_range(
        *argv.offset(3),
        0i32 as libc::c_ulong,
        gnu_dev_minor(
          (2147483647i32 as libc::c_uint)
            .wrapping_mul(2u32)
            .wrapping_add(1u32) as __dev_t,
        ) as libc::c_ulong,
      ) as libc::c_uint,
    ) as dev_t;
    arg = *argv.offset(4)
  }
  if !arg.is_null() {
    bb_show_usage();
  }
  if mknod(*argv.offset(0), mode, dev) != 0i32 {
    bb_simple_perror_msg_and_die(*argv.offset(0));
  }
  return 0i32;
}

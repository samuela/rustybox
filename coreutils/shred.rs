use libc;
use libc::chmod;
use libc::fstat;
use libc::mode_t;
use libc::off64_t;
use libc::off_t;
use libc::open;
use libc::stat;
extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn ftruncate(__fd: libc::c_int, __length: off64_t) -> libc::c_int;
  #[no_mangle]
  fn fdatasync(__fildes: libc::c_int) -> libc::c_int;

}

pub const OPT_u: C2RustUnnamed = 2;
pub const OPT_z: C2RustUnnamed = 4;
pub const OPT_f: C2RustUnnamed = 1;
pub type C2RustUnnamed = libc::c_uint;
pub const OPT_x: C2RustUnnamed = 32;
pub const OPT_v: C2RustUnnamed = 16;
pub const OPT_n: C2RustUnnamed = 8;

/*
 * Copyright (C) 2017 Denys Vlasenko <vda.linux@googlemail.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config SHRED
//config:	bool "shred (4.9 kb)"
//config:	default y
//config:	help
//config:	Overwrite a file to hide its contents, and optionally delete it
//applet:IF_SHRED(APPLET(shred, BB_DIR_USR_BIN, SUID_DROP))
//kbuild:lib-$(CONFIG_SHRED) += shred.o
//usage:#define shred_trivial_usage
//usage:       "FILE..."
//usage:#define shred_full_usage "\n\n"
//usage:       "Overwrite/delete FILEs\n"
//usage:     "\n	-f	Chmod to ensure writability"
//usage:     "\n	-n N	Overwrite N times (default 3)"
//usage:     "\n	-z	Final overwrite with zeros"
//usage:     "\n	-u	Remove file"
//-x and -v are accepted but have no effect
/* shred (GNU coreutils) 8.25:
-f, --force		change permissions to allow writing if necessary
-u			truncate and remove file after overwriting
-z, --zero		add a final overwrite with zeros to hide shredding
-n, --iterations=N	overwrite N times instead of the default (3)
-v, --verbose		show progress
-x, --exact		do not round file sizes up to the next full block; this is the default for non-regular files
--random-source=FILE	get random bytes from FILE
-s, --size=N		shred this many bytes (suffixes like K, M, G accepted)
--remove[=HOW]		like -u but give control on HOW to delete;  See below
*/
#[no_mangle]
pub unsafe extern "C" fn shred_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut rand_fd: libc::c_int = 0; /* for compiler */
  rand_fd = rand_fd;
  let mut zero_fd: libc::c_int = 0;
  let mut num_iter: libc::c_uint = 3i32 as libc::c_uint;
  let mut opt: libc::c_uint = 0;
  opt = crate::libbb::getopt32::getopt32(
    argv,
    b"fuzn:+vx\x00" as *const u8 as *const libc::c_char,
    &mut num_iter as *mut libc::c_uint,
  );
  argv = argv.offset(optind as isize);
  zero_fd =
    crate::libbb::xfuncs_printf::xopen(b"/dev/zero\x00" as *const u8 as *const libc::c_char, 0i32);
  if num_iter != 0i32 as libc::c_uint {
    rand_fd = crate::libbb::xfuncs_printf::xopen(
      b"/dev/urandom\x00" as *const u8 as *const libc::c_char,
      0i32,
    )
  }
  if (*argv).is_null() {
    crate::libbb::appletlib::bb_show_usage();
  }
  loop {
    let mut sb: stat = std::mem::zeroed();
    let mut fname: *const libc::c_char = std::ptr::null();
    let mut i: libc::c_uint = 0;
    let mut fd: libc::c_int = 0;
    let fresh0 = argv;
    argv = argv.offset(1);
    fname = *fresh0;
    if fname.is_null() {
      break;
    }
    fd = -1i32;
    if opt & OPT_f as libc::c_int as libc::c_uint != 0 {
      fd = open(fname, 0o1i32);
      if fd < 0i32 {
        chmod(fname, 0o666i32 as mode_t);
      }
    }
    if fd < 0i32 {
      fd = crate::libbb::xfuncs_printf::xopen(fname, 0o1i32)
    }
    if fstat(fd, &mut sb) == 0i32 && sb.st_size > 0 {
      let mut size: off_t = sb.st_size;
      i = 0i32 as libc::c_uint;
      while i < num_iter {
        crate::libbb::copyfd::bb_copyfd_size(rand_fd, fd, size);
        fdatasync(fd);
        crate::libbb::xfuncs_printf::xlseek(fd, 0i32 as off_t, 0i32);
        i = i.wrapping_add(1)
      }
      if opt & OPT_z as libc::c_int as libc::c_uint != 0 {
        crate::libbb::copyfd::bb_copyfd_size(zero_fd, fd, size);
        fdatasync(fd);
      }
      if opt & OPT_u as libc::c_int as libc::c_uint != 0 {
        ftruncate(fd, 0i32 as off64_t);
        crate::libbb::xfuncs_printf::xunlink(fname);
      }
      crate::libbb::xfuncs_printf::xclose(fd);
    }
  }
  return 0i32;
}

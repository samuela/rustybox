use libc;
use libc::access;
use libc::atoi;
use libc::fclose;
use libc::fprintf;
use libc::lstat;
use libc::printf;
use libc::puts;
use libc::rename;
use libc::rmdir;
use libc::sprintf;
use libc::strchr;
use libc::strcmp;
use libc::strrchr;
use libc::strstr;
use libc::system;




extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;



  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstat(pathname: *const libc::c_char, buf: *mut stat);
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_xioctl(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
}

use crate::librb::size_t;
use libc::ssize_t;


use libc::stat;

/*
 * fdformat.c  -  Low-level formats a floppy disk - Werner Almesberger
 * 5 July 2003 -- modified for Busybox by Erik Andersen
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config FDFORMAT
//config:	bool "fdformat (4.4 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	fdformat is used to low-level format a floppy disk.
//applet:IF_FDFORMAT(APPLET(fdformat, BB_DIR_USR_SBIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_FDFORMAT) += fdformat.o
//usage:#define fdformat_trivial_usage
//usage:       "[-n] DEVICE"
//usage:#define fdformat_full_usage "\n\n"
//usage:       "Format floppy disk\n"
//usage:     "\n	-n	Don't verify after format"
/* Stuff extracted from linux/fd.h */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floppy_struct {
  pub size: libc::c_uint,
  pub sect: libc::c_uint,
  pub head: libc::c_uint,
  pub track: libc::c_uint,
  pub stretch: libc::c_uint,
  pub gap: libc::c_uchar,
  pub rate: libc::c_uchar,
  pub spec1: libc::c_uchar,
  pub fmt_gap: libc::c_uchar,
  pub name: *const libc::c_char,
  /* used only for predefined formats */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct format_descr {
  pub device: libc::c_uint,
  pub head: libc::c_uint,
  pub track: libc::c_uint,
}
/* format fill byte. */
#[no_mangle]
pub unsafe extern "C" fn fdformat_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut fd: libc::c_int = 0;
  let mut n: libc::c_int = 0;
  let mut cyl: libc::c_int = 0;
  let mut read_bytes: libc::c_int = 0;
  let mut verify: libc::c_int = 0;
  let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
  let mut st: stat = std::mem::zeroed();
  let mut param: floppy_struct = floppy_struct {
    size: 0,
    sect: 0,
    head: 0,
    track: 0,
    stretch: 0,
    gap: 0,
    rate: 0,
    spec1: 0,
    fmt_gap: 0,
    name: 0 as *const libc::c_char,
  };
  let mut descr: format_descr = format_descr {
    device: 0,
    head: 0,
    track: 0,
  };
  verify =
    (getopt32(argv, b"^n\x00=1\x00" as *const u8 as *const libc::c_char) == 0) as libc::c_int;
  argv = argv.offset(optind as isize);
  xstat(*argv, &mut st);
  if !(st.st_mode & 0o170000i32 as libc::c_uint == 0o60000i32 as libc::c_uint) {
    bb_error_msg_and_die(
      b"%s: not a block device\x00" as *const u8 as *const libc::c_char,
      *argv,
    );
    /* do not test major - perhaps this was an USB floppy */
  }
  /* O_RDWR for formatting and verifying */
  fd = xopen(*argv, 0o2i32);
  /* original message was: "Could not determine current format type" */
  bb_xioctl(
    fd,
    ((2u32 << 0i32 + 8i32 + 8i32 + 14i32
      | (2i32 << 0i32 + 8i32) as libc::c_uint
      | (0x4i32 << 0i32) as libc::c_uint) as libc::c_ulong
      | (::std::mem::size_of::<floppy_struct>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
      as libc::c_uint,
    &mut param as *mut floppy_struct as *mut libc::c_void,
    b"FDGETPRM\x00" as *const u8 as *const libc::c_char,
  );
  printf(
    b"%s-sided, %u tracks, %u sec/track. Total capacity %d kB\n\x00" as *const u8
      as *const libc::c_char,
    if param.head == 2i32 as libc::c_uint {
      b"Double\x00" as *const u8 as *const libc::c_char
    } else {
      b"Single\x00" as *const u8 as *const libc::c_char
    },
    param.track,
    param.sect,
    param.size >> 1i32,
  );
  /* FORMAT */
  printf(b"Formatting... \x00" as *const u8 as *const libc::c_char);
  bb_xioctl(
    fd,
    0u32 << 0i32 + 8i32 + 8i32 + 14i32
      | (2i32 << 0i32 + 8i32) as libc::c_uint
      | (0x47i32 << 0i32) as libc::c_uint
      | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint,
    0 as *mut libc::c_void,
    b"FDFMTBEG\x00" as *const u8 as *const libc::c_char,
  );
  /* n == track */
  n = 0i32;
  while (n as libc::c_uint) < param.track {
    descr.head = 0i32 as libc::c_uint;
    descr.track = n as libc::c_uint;
    bb_xioctl(
      fd,
      ((1u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (2i32 << 0i32 + 8i32) as libc::c_uint
        | (0x48i32 << 0i32) as libc::c_uint) as libc::c_ulong
        | (::std::mem::size_of::<format_descr>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
        as libc::c_uint,
      &mut descr as *mut format_descr as *mut libc::c_void,
      b"FDFMTTRK\x00" as *const u8 as *const libc::c_char,
    );
    printf(
      b"%3d\x08\x08\x08\x00" as *const u8 as *const libc::c_char,
      n,
    );
    if param.head == 2i32 as libc::c_uint {
      descr.head = 1i32 as libc::c_uint;
      bb_xioctl(
        fd,
        ((1u32 << 0i32 + 8i32 + 8i32 + 14i32
          | (2i32 << 0i32 + 8i32) as libc::c_uint
          | (0x48i32 << 0i32) as libc::c_uint) as libc::c_ulong
          | (::std::mem::size_of::<format_descr>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
          as libc::c_uint,
        &mut descr as *mut format_descr as *mut libc::c_void,
        b"FDFMTTRK\x00" as *const u8 as *const libc::c_char,
      );
    }
    n += 1
  }
  bb_xioctl(
    fd,
    0u32 << 0i32 + 8i32 + 8i32 + 14i32
      | (2i32 << 0i32 + 8i32) as libc::c_uint
      | (0x49i32 << 0i32) as libc::c_uint
      | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint,
    0 as *mut libc::c_void,
    b"FDFMTEND\x00" as *const u8 as *const libc::c_char,
  );
  puts(b"Done\x00" as *const u8 as *const libc::c_char);
  /* VERIFY */
  if verify != 0 {
    /* n == cyl_size */
    n = param
      .sect
      .wrapping_mul(param.head)
      .wrapping_mul(512i32 as libc::c_uint) as libc::c_int;
    data = xmalloc(n as size_t) as *mut libc::c_uchar;
    printf(b"Verifying... \x00" as *const u8 as *const libc::c_char);
    cyl = 0i32;
    while (cyl as libc::c_uint) < param.track {
      printf(
        b"%3d\x08\x08\x08\x00" as *const u8 as *const libc::c_char,
        cyl,
      );
      read_bytes = safe_read(fd, data as *mut libc::c_void, n as size_t) as libc::c_int;
      if read_bytes != n {
        if read_bytes < 0i32 {
          bb_simple_perror_msg(b"read error\x00" as *const u8 as *const libc::c_char);
        }
        bb_error_msg_and_die(
          b"problem reading cylinder %d, expected %d, read %d\x00" as *const u8
            as *const libc::c_char,
          cyl,
          n,
          read_bytes,
        );
        // FIXME: maybe better seek & continue??
      }
      loop
      /* Check backwards so we don't need a counter */
      {
        read_bytes -= 1;
        if !(read_bytes >= 0i32) {
          break;
        }
        if *data.offset(read_bytes as isize) as libc::c_int != 0xf6i32 {
          printf(
            b"bad data in cyl %d\nContinuing... \x00" as *const u8 as *const libc::c_char,
            cyl,
          );
        }
      }
      cyl += 1
    }
    /* There is no point in freeing blocks at the end of a program, because
    all of the program's space is given back to the system when the process
    terminates.*/
    puts(b"Done\x00" as *const u8 as *const libc::c_char);
  }
  /* Don't bother closing.  Exit does
   * that, so we can save a few bytes */
  return 0i32;
}

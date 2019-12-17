use crate::libbb::appletlib::applet_name;
use crate::libbb::skip_whitespace::skip_whitespace;
use libc;
use libc::fclose;
use libc::free;
use libc::getopt;
use libc::putchar_unlocked;
use libc::sscanf;
use libc::strchr;
extern "C" {

  #[no_mangle]
  static mut optarg: *mut libc::c_char;

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  static mut stdin: *mut FILE;

  #[no_mangle]
  fn xmalloc_fgetline(file: *mut FILE) -> *mut libc::c_char;

  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;

  #[no_mangle]
  fn xfopen_for_read(path: *const libc::c_char) -> *mut FILE;

  #[no_mangle]
  static bkm_suffixes: [suffix_mult; 0];

  /*
   * ascii-to-numbers implementations for busybox
   *
   * Copyright (C) 2003  Manuel Novoa III  <mjn3@codepoet.org>
   *
   * Licensed under GPLv2, see file LICENSE in this source tree.
   */
  /* Provides extern declarations of functions */
  /* Unsigned long long functions always exist */
  #[no_mangle]
  fn xstrtoull_range_sfx(
    str: *const libc::c_char,
    b: libc::c_int,
    l: libc::c_ulonglong,
    u: libc::c_ulonglong,
    sfx: *const suffix_mult,
  ) -> libc::c_ulonglong;

  #[no_mangle]
  fn xatoi_positive(numstr: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn bb_show_usage() -> !;

  #[no_mangle]
  fn alloc_dumper() -> *mut dumper_t;

  #[no_mangle]
  fn bb_dump_add(dumper: *mut dumper_t, fmt: *const libc::c_char);

  #[no_mangle]
  fn bb_dump_dump(dumper: *mut dumper_t, argv: *mut *mut libc::c_char) -> libc::c_int;
}

use crate::librb::smallint;
use libc::off_t;
use libc::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct suffix_mult {
  pub suffix: [libc::c_char; 4],
  pub mult: libc::c_uint,
}

/* %_A */
/* rep count set, not default */
/* print offset */
/* blank pad */
/* %_c */
/* %c */
/* %[EefGf] */
/* %[di] */
/* %_p */
/* %s */
/* %_u */
/* %[ouXx] */
/* no conversions */

pub type dump_vflag_t = libc::c_uint;
// pub const WAIT: dump_vflag_t = 3;
// pub const FIRST: dump_vflag_t = 2;
// pub const DUP: dump_vflag_t = 1;
pub const ALL: dump_vflag_t = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PR {
  pub nextpr: *mut PR,
  pub flags: libc::c_uint,
  pub bcnt: libc::c_int,
  pub cchar: *mut libc::c_char,
  pub fmt: *mut libc::c_char,
  pub nospace: *mut libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FU {
  pub nextfu: *mut FU,
  pub nextpr: *mut PR,
  pub flags: libc::c_uint,
  pub reps: libc::c_int,
  pub bcnt: libc::c_int,
  pub fmt: *mut libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FS {
  pub nextfs: *mut FS,
  pub nextfu: *mut FU,
  pub bcnt: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dumper_t {
  pub dump_skip: off_t,
  pub dump_length: libc::c_int,
  pub dump_vflag: smallint,
  pub fshead: *mut FS,
}

#[inline(always)]
unsafe extern "C" fn bb_ascii_isxdigit(mut a: libc::c_uchar) -> libc::c_int {
  let mut b: libc::c_uchar = (a as libc::c_int - '0' as i32) as libc::c_uchar;
  if b as libc::c_int <= 9i32 {
    return (b as libc::c_int <= 9i32) as libc::c_int;
  }
  b = ((a as libc::c_int | 0x20i32) - 'a' as i32) as libc::c_uchar;
  return (b as libc::c_int <= 'f' as i32 - 'a' as i32) as libc::c_int;
}

/*
 * hexdump implementation for busybox
 * Based on code from util-linux v 2.11l
 *
 * Copyright (c) 1989
 * The Regents of the University of California.  All rights reserved.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config HEXDUMP
//config:	bool "hexdump (8.6 kb)"
//config:	default y
//config:	help
//config:	The hexdump utility is used to display binary data in a readable
//config:	way that is comparable to the output from most hex editors.
//config:
//config:config FEATURE_HEXDUMP_REVERSE
//config:	bool "Support -R, reverse of 'hexdump -Cv'"
//config:	default y
//config:	depends on HEXDUMP
//config:	help
//config:	The hexdump utility is used to display binary data in an ascii
//config:	readable way. This option creates binary data from an ascii input.
//config:	NB: this option is non-standard. It's unwise to use it in scripts
//config:	aimed to be portable.
//config:
//config:config HD
//config:	bool "hd (7.8 kb)"
//config:	default y
//config:	help
//config:	hd is an alias to hexdump -C.
//applet:IF_HEXDUMP(APPLET_NOEXEC(hexdump, hexdump, BB_DIR_USR_BIN, SUID_DROP, hexdump))
//applet:IF_HD(APPLET_NOEXEC(hd, hexdump, BB_DIR_USR_BIN, SUID_DROP, hd))
//kbuild:lib-$(CONFIG_HEXDUMP) += hexdump.o
//kbuild:lib-$(CONFIG_HD) += hexdump.o
//usage:#define hexdump_trivial_usage
//usage:       "[-bcCdefnosvx" IF_FEATURE_HEXDUMP_REVERSE("R") "] [FILE]..."
//usage:#define hexdump_full_usage "\n\n"
//usage:       "Display FILEs (or stdin) in a user specified format\n"
//usage:     "\n	-b		1-byte octal display"
//usage:     "\n	-c		1-byte character display"
//usage:     "\n	-d		2-byte decimal display"
//usage:     "\n	-o		2-byte octal display"
//usage:     "\n	-x		2-byte hex display"
//usage:     "\n	-C		hex+ASCII 16 bytes per line"
//usage:     "\n	-v		Show all (no dup folding)"
//usage:     "\n	-e FORMAT_STR	Example: '16/1 \"%02x|\"\"\\n\"'"
//usage:     "\n	-f FORMAT_FILE"
// exactly the same help text lines in hexdump and xxd:
//usage:     "\n	-n LENGTH	Show only first LENGTH bytes"
//usage:     "\n	-s OFFSET	Skip OFFSET bytes"
//usage:	IF_FEATURE_HEXDUMP_REVERSE(
//usage:     "\n	-R		Reverse of 'hexdump -Cv'")
// TODO: NONCOMPAT!!! move -R to xxd -r
//usage:
//usage:#define hd_trivial_usage
//usage:       "FILE..."
//usage:#define hd_full_usage "\n\n"
//usage:       "hd is an alias for hexdump -C"
/* This is a NOEXEC applet. Be very careful! */
unsafe extern "C" fn bb_dump_addfile(mut dumper: *mut dumper_t, mut name: *mut libc::c_char) {
  let mut p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut fp: *mut FILE = std::ptr::null_mut();
  let mut buf: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  fp = xfopen_for_read(name);
  loop {
    buf = xmalloc_fgetline(fp);
    if buf.is_null() {
      break;
    }
    p = skip_whitespace(buf);
    if *p as libc::c_int != 0 && *p as libc::c_int != '#' as i32 {
      bb_dump_add(dumper, p);
    }
    free(buf as *mut libc::c_void);
  }
  fclose(fp);
}
static mut add_strings: [*const libc::c_char; 5] = [
  b"\"%07.7_ax \"16/1 \"%03o \"\"\n\"\x00" as *const u8 as *const libc::c_char,
  b"\"%07.7_ax \"16/1 \"%3_c \"\"\n\"\x00" as *const u8 as *const libc::c_char,
  b"\"%07.7_ax \"8/2 \"  %05u \"\"\n\"\x00" as *const u8 as *const libc::c_char,
  b"\"%07.7_ax \"8/2 \" %06o \"\"\n\"\x00" as *const u8 as *const libc::c_char,
  b"\"%07.7_ax \"8/2 \"   %04x \"\"\n\"\x00" as *const u8 as *const libc::c_char,
];
static mut add_first: [libc::c_char; 12] = [34, 37, 48, 55, 46, 55, 95, 65, 120, 10, 34, 0];
static mut hexdump_opts: [libc::c_char; 17] = [
  98, 99, 100, 111, 120, 67, 101, 58, 102, 58, 110, 58, 115, 58, 118, 82, 0,
];
#[no_mangle]
pub unsafe extern "C" fn hexdump_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut buf: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut current_block: u64;
  let mut dumper: *mut dumper_t = alloc_dumper();
  let mut p: *const libc::c_char = std::ptr::null();
  let mut ch: libc::c_int = 0;
  let mut fp: *mut FILE = std::ptr::null_mut();
  let mut rdump: smallint = 0 as smallint;
  if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(2) == 0) {
    /* we are "hd" */
    ch = 'C' as i32;
    current_block = 9286514861260803080;
  } else {
    current_block = 6873731126896040597;
  }
  loop {
    match current_block {
      6873731126896040597 =>
      /* We cannot use getopt32: in hexdump options are cumulative.
       * E.g. "hexdump -C -C file" should dump each line twice */
      {
        ch = getopt(argc, argv, hexdump_opts.as_ptr());
        if !(ch > 0) {
          break;
        }
        p = strchr(hexdump_opts.as_ptr(), ch);
        if p.is_null() {
          bb_show_usage();
        }
        if (p.wrapping_offset_from(hexdump_opts.as_ptr()) as libc::c_long) < 5i32 as libc::c_long {
          bb_dump_add(dumper, add_first.as_ptr());
          bb_dump_add(
            dumper,
            add_strings[p.wrapping_offset_from(hexdump_opts.as_ptr()) as libc::c_long as libc::c_int
              as usize],
          );
        }
        /* Save a little bit of space below by omitting the 'else's. */
        if ch == 'C' as i32 {
          current_block = 9286514861260803080; // final address line after dump
          continue;
        }
      }
      _ => {
        bb_dump_add(
          dumper,
          b"\"%08.8_Ax\n\"\x00" as *const u8 as *const libc::c_char,
        );
        //------------------- "address  "   8 * "xx "    "  "  8 * "xx "
        bb_dump_add(
          dumper,
          b"\"%08.8_ax  \"8/1 \"%02x \"\"  \"8/1 \"%02x \"\x00" as *const u8 as *const libc::c_char,
        );
        //------------------- "  |ASCII...........|\n"
        bb_dump_add(
          dumper,
          b"\"  |\"16/1 \"%_p\"\"|\n\"\x00" as *const u8 as *const libc::c_char,
        ); /* else */
      }
    } /* else */
    if ch == 'e' as i32 {
      bb_dump_add(dumper, optarg); /* else */
    } /* else */
    if ch == 'f' as i32 {
      bb_dump_addfile(dumper, optarg);
    }
    if ch == 'n' as i32 {
      (*dumper).dump_length = xatoi_positive(optarg)
    }
    if ch == 's' as i32 {
      /* compat: -s accepts hex numbers too */
      (*dumper).dump_skip = xstrtoull_range_sfx(
        optarg,
        0,
        0 as libc::c_ulonglong,
        !((1i32 as off_t)
          << (::std::mem::size_of::<off_t>() as libc::c_ulong)
            .wrapping_mul(8i32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)) as libc::c_ulonglong,
        bkm_suffixes.as_ptr(),
      ) as off_t
    }
    if ch == 'v' as i32 {
      (*dumper).dump_vflag = ALL as libc::c_int as smallint
    }
    if ch == 'R' as i32 {
      rdump = 1i32 as smallint
    }
    current_block = 6873731126896040597;
  }
  if (*dumper).fshead.is_null() {
    bb_dump_add(dumper, add_first.as_ptr());
    bb_dump_add(
      dumper,
      b"\"%07.7_ax \"8/2 \"%04x \"\"\n\"\x00" as *const u8 as *const libc::c_char,
    );
  }
  argv = argv.offset(optind as isize);
  if rdump == 0 {
    return bb_dump_dump(dumper, argv);
  }
  /* -R: reverse of 'hexdump -Cv' */
  fp = stdin;
  if (*argv).is_null() {
    argv = argv.offset(-1);
    current_block = 6590038594373520143;
  } else {
    current_block = 6717214610478484138;
  }
  loop {
    match current_block {
      6590038594373520143 => {
        loop {
          buf = xmalloc_fgetline(fp);
          if buf.is_null() {
            break;
          }
          p = buf;
          loop {
            /* skip address or previous byte */
            while bb_ascii_isxdigit(*p as libc::c_uchar) != 0 {
              p = p.offset(1)
            }
            while *p as libc::c_int == ' ' as i32 {
              p = p.offset(1)
            }
            /* '|' char will break the line */
            if bb_ascii_isxdigit(*p as libc::c_uchar) == 0
              || sscanf(
                p,
                b"%x \x00" as *const u8 as *const libc::c_char,
                &mut ch as *mut libc::c_int,
              ) != 1i32
            {
              break;
            }
            putchar_unlocked(ch);
          }
          free(buf as *mut libc::c_void);
        }
        fclose(fp);
        argv = argv.offset(1);
        if !(*argv).is_null() {
          current_block = 6717214610478484138;
        } else {
          break;
        }
      }
      _ => {
        buf = std::ptr::null_mut::<libc::c_char>();
        fp = xfopen_for_read(*argv);
        current_block = 6590038594373520143;
      }
    }
  }
  fflush_stdout_and_exit(0i32);
}

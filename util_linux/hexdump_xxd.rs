use crate::librb::size_t;
use crate::librb::smallint;

use libc;
use libc::free;
use libc::off_t;
use libc::sprintf;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn xstrtoull_range(
    str: *const libc::c_char,
    b: libc::c_int,
    l: libc::c_ulonglong,
    u: libc::c_ulonglong,
  ) -> libc::c_ulonglong;

  #[no_mangle]
  fn xstrtou_range(
    str: *const libc::c_char,
    b: libc::c_int,
    l: libc::c_uint,
    u: libc::c_uint,
  ) -> libc::c_uint;

  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;

  #[no_mangle]
  fn alloc_dumper() -> *mut dumper_t;

  #[no_mangle]
  fn bb_dump_add(dumper: *mut dumper_t, fmt: *const libc::c_char);

  #[no_mangle]
  fn bb_dump_dump(dumper: *mut dumper_t, argv: *mut *mut libc::c_char) -> libc::c_int;
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

/*
 * xxd implementation for busybox
 *
 * Copyright (c) 2017 Denys Vlasenko <vda.linux@gmail.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config XXD
//config:	bool "xxd (8.9 kb)"
//config:	default y
//config:	help
//config:	The xxd utility is used to display binary data in a readable
//config:	way that is comparable to the output from most hex editors.
//applet:IF_XXD(APPLET_NOEXEC(xxd, xxd, BB_DIR_USR_BIN, BB_SUID_DROP, xxd))
//kbuild:lib-$(CONFIG_XXD) += hexdump_xxd.o
// $ xxd --version
// xxd V1.10 27oct98 by Juergen Weigert
// $ xxd --help
// Usage:
//       xxd [options] [infile [outfile]]
//    or
//       xxd -r [-s [-]offset] [-c cols] [-ps] [infile [outfile]]
// Options:
//    -a          toggle autoskip: A single '*' replaces nul-lines. Default off.
//    -b          binary digit dump (incompatible with -ps,-i,-r). Default hex.
//    -c cols     format <cols> octets per line. Default 16 (-i: 12, -ps: 30).
//    -E          show characters in EBCDIC. Default ASCII.
//    -e          little-endian dump (incompatible with -ps,-i,-r).
//    -g          number of octets per group in normal output. Default 2 (-e: 4).
//    -i          output in C include file style.
//    -l len      stop after <len> octets.
//    -o off      add <off> to the displayed file position.
//    -ps         output in postscript plain hexdump style.
//    -r          reverse operation: convert (or patch) hexdump into binary.
//    -r -s off   revert with <off> added to file positions found in hexdump.
//    -s [+][-]seek  start at <seek> bytes abs. (or +: rel.) infile offset.
//    -u          use upper case hex letters.
//usage:#define xxd_trivial_usage
//usage:       "[OPTIONS] [FILE]"
//usage:#define xxd_full_usage "\n\n"
//usage:       "Hex dump FILE (or stdin)\n"
//usage:     "\n	-g N		Bytes per group"
//usage:     "\n	-c N		Bytes per line"
//usage:     "\n	-p		Show only hex bytes, assumes -c30"
// exactly the same help text lines in hexdump and xxd:
//usage:     "\n	-l LENGTH	Show only first LENGTH bytes"
//usage:     "\n	-s OFFSET	Skip OFFSET bytes"
// TODO: implement -r (see hexdump -R)
/* This is a NOEXEC applet. Be very careful! */
#[no_mangle]
pub unsafe extern "C" fn xxd_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut buf: [libc::c_char; 80] = [0; 80];
  let mut dumper: *mut dumper_t = 0 as *mut dumper_t;
  let mut opt_l: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opt_s: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut bytes: libc::c_uint = 2i32 as libc::c_uint;
  let mut cols: libc::c_uint = 0i32 as libc::c_uint;
  let mut opt: libc::c_uint = 0;
  dumper = alloc_dumper();
  opt = getopt32(
    argv,
    b"^l:s:apg:+c:+\x00?1\x00" as *const u8 as *const libc::c_char,
    &mut opt_l as *mut *mut libc::c_char,
    &mut opt_s as *mut *mut libc::c_char,
    &mut bytes as *mut libc::c_uint,
    &mut cols as *mut libc::c_uint,
  );
  argv = argv.offset(optind as isize);
  (*dumper).dump_vflag = ALL as libc::c_int as smallint;
  //	if (opt & OPT_a)
  //		dumper->dump_vflag = SKIPNUL; ..does not exist
  if opt & (1i32 << 0i32) as libc::c_uint != 0 {
    (*dumper).dump_length = xstrtou_range(
      opt_l,
      0i32,
      0i32 as libc::c_uint,
      2147483647i32 as libc::c_uint,
    ) as libc::c_int
  }
  if opt & (1i32 << 1i32) as libc::c_uint != 0 {
    (*dumper).dump_skip = xstrtoull_range(
      opt_s,
      0i32,
      0i32 as libc::c_ulonglong,
      !((1i32 as off_t)
        << (::std::mem::size_of::<off_t>() as libc::c_ulong)
          .wrapping_mul(8i32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)) as libc::c_ulonglong,
    ) as off_t
    //BUGGY for /proc/version (unseekable?)
  }
  if opt & (1i32 << 3i32) as libc::c_uint != 0 {
    if cols == 0i32 as libc::c_uint {
      cols = 30i32 as libc::c_uint
    }
    bytes = cols
  /* -p ignores -gN */
  } else {
    if cols == 0i32 as libc::c_uint {
      cols = 16i32 as libc::c_uint
    }
    bb_dump_add(
      dumper,
      b"\"%08.8_ax: \"\x00" as *const u8 as *const libc::c_char,
    );
    // "address: "
  } // cols * "xx"
  if bytes < 1i32 as libc::c_uint || bytes >= cols {
    sprintf(
      buf.as_mut_ptr(),
      b"%u/1 \"%%02x\"\x00" as *const u8 as *const libc::c_char,
      cols,
    ); // cols * "xx "
    bb_dump_add(dumper, buf.as_mut_ptr());
  } else if bytes == 1i32 as libc::c_uint {
    sprintf(
      buf.as_mut_ptr(),
      b"%u/1 \"%%02x \"\x00" as *const u8 as *const libc::c_char,
      cols,
    );
    bb_dump_add(dumper, buf.as_mut_ptr());
  } else {
    /* Format "print byte" with and without trailing space */
    let mut i: libc::c_uint = 0;
    let mut bigbuf: *mut libc::c_char = xmalloc(
      (cols as libc::c_ulong).wrapping_mul(
        (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong),
      ),
    ) as *mut libc::c_char;
    let mut p: *mut libc::c_char = bigbuf;
    i = 1i32 as libc::c_uint;
    while i <= cols {
      if i == cols || i.wrapping_rem(bytes) != 0 {
        p = stpcpy(p, b"/1 \"%02x\"\x00" as *const u8 as *const libc::c_char)
      } else {
        p = stpcpy(p, b"/1 \"%02x \"\x00" as *const u8 as *const libc::c_char)
      }
      i = i.wrapping_add(1)
    }
    // for -g3, this results in B B BS B B BS... B = "xxxxxx xxxxxx .....xx"
    // todo: can be more clever and use
    // one 'bytes-1/1 "%02x"' format instead of many "B B B..." formats
    //bb_error_msg("ADDED:'%s'", bigbuf);
    bb_dump_add(dumper, bigbuf); // "  ASCII\n"
    free(bigbuf as *mut libc::c_void);
  }
  if opt & (1i32 << 3i32) as libc::c_uint == 0 {
    sprintf(
      buf.as_mut_ptr(),
      b"\"  \"%u/1 \"%%_p\"\"\n\"\x00" as *const u8 as *const libc::c_char,
      cols,
    );
    bb_dump_add(dumper, buf.as_mut_ptr());
  } else {
    bb_dump_add(dumper, b"\"\n\"\x00" as *const u8 as *const libc::c_char);
  }
  return bb_dump_dump(dumper, argv);
}

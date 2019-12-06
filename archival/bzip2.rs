use crate::archival::libarchive::bb_archive::transformer_state_t;
use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::xfuncs_printf::xmalloc;
use crate::librb::size_t;
use libc;
use libc::free;
use libc::ssize_t;
extern "C" {
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn full_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;

  #[no_mangle]
  fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;

  #[no_mangle]
  static mut option_mask32: u32;

  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;

  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;

  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);

  #[no_mangle]
  fn bunzip2_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn crc32_filltable(tbl256: *mut u32, endian: libc::c_int) -> *mut u32;

  #[no_mangle]
  fn append_ext(
    filename: *mut libc::c_char,
    expected_ext: *const libc::c_char,
  ) -> *mut libc::c_char;

  #[no_mangle]
  fn bbunpack(
    argv: *mut *mut libc::c_char,
    unpacker: Option<unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong>,
    make_new_name: Option<
      unsafe extern "C" fn(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char,
    >,
    expected_ext: *const libc::c_char,
  ) -> libc::c_int;
}

pub type bb__aliased_u32 = u32;
pub type bb__aliased_u64 = u64;

/* NB: unaligned parameter should be a pointer, aligned one -
 * a lvalue. This makes it more likely to not swap them by mistake
 */
/* #elif ... - add your favorite arch today! */
/* Unaligned, fixed-endian accessors */
/* unxz needs an aligned fixed-endian accessor.
 * (however, the compiler does not realize it's aligned, the cast is still necessary)
 */
/* ---- Size-saving "small" ints (arch-dependent) ----------- */
/* add other arches which benefit from this... */

pub type C2RustUnnamed_0 = libc::c_int;
pub const BBUNPK_SEAMLESS_MAGIC: C2RustUnnamed_0 = -2147483648;
pub const BBUNPK_OPT_TEST: C2RustUnnamed_0 = 64;
/* not included in BBUNPK_OPTSTR: */
pub const BBUNPK_OPT_DECOMPRESS: C2RustUnnamed_0 = 32;
pub const BBUNPK_OPT_QUIET: C2RustUnnamed_0 = 16;
pub const BBUNPK_OPT_VERBOSE: C2RustUnnamed_0 = 8;
/* only some decompressors: */
pub const BBUNPK_OPT_KEEP: C2RustUnnamed_0 = 4;
pub const BBUNPK_OPT_FORCE: C2RustUnnamed_0 = 2;
pub const BBUNPK_OPT_STDOUT: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bz_stream {
  pub state: *mut libc::c_void,
  pub next_in: *mut libc::c_char,
  pub next_out: *mut libc::c_char,
  pub avail_in: libc::c_uint,
  pub avail_out: libc::c_uint,
  pub total_out: libc::c_ulonglong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EState {
  pub strm: *mut bz_stream,
  pub mode: u8,
  pub state: u8,
  pub blockSize100k: u8,
  pub arr1: *mut u32,
  pub arr2: *mut u32,
  pub ftab: *mut u32,
  pub quadrant: *mut u16,
  pub budget: i32,
  pub ptr: *mut u32,
  pub block: *mut u8,
  pub mtfv: *mut u16,
  pub zbits: *mut u8,
  pub state_in_ch: u32,
  pub state_in_len: i32,
  pub nblock: i32,
  pub nblockMAX: i32,
  pub posZ: *mut u8,
  pub state_out_pos: *mut u8,
  pub bsBuff: u32,
  pub bsLive: i32,
  pub crc32table: *mut u32,
  pub blockCRC: u32,
  pub combinedCRC: u32,
  pub blockNo: i32,
  pub nMTF: i32,
  pub nInUse: i32,
  pub inUse: [Bool; 256],
  pub unseqToSeq: [u8; 256],
  pub mtfFreq: [i32; 258],
  pub selector: [u8; 18002],
  pub selectorMtf: [u8; 18002],
  pub len: [[u8; 258]; 6],
  pub sendMTFValues__code: [[i32; 258]; 6],
  pub sendMTFValues__rfreq: [[i32; 258]; 6],
  pub BZ2_hbMakeCodeLengths__heap: [i32; 260],
  pub BZ2_hbMakeCodeLengths__weight: [i32; 516],
  pub BZ2_hbMakeCodeLengths__parent: [i32; 516],
  pub mainSort__copyStart: [i32; 256],
  pub mainSort__copyEnd: [i32; 256],
}
/*
 * bzip2 is written by Julian Seward <jseward@bzip.org>.
 * Adapted for busybox by Denys Vlasenko <vda.linux@googlemail.com>.
 * See README and LICENSE files in this directory for more information.
 */
/*-------------------------------------------------------------*/
/*--- Private header file for the library.                  ---*/
/*---                                       bzlib_private.h ---*/
/*-------------------------------------------------------------*/
/* ------------------------------------------------------------------
This file is part of bzip2/libbzip2, a program and library for
lossless, block-sorting data compression.

bzip2/libbzip2 version 1.0.4 of 20 December 2006
Copyright (C) 1996-2006 Julian Seward <jseward@bzip.org>

Please read the WARNING, DISCLAIMER and PATENTS sections in the
README file.

This program is released under the terms of the license contained
in the file LICENSE.
------------------------------------------------------------------ */
/* #include "bzlib.h" */
/*-- General stuff. --*/
pub type Bool = libc::c_uchar;
pub const dSt: C2RustUnnamed_1 = 2;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const wfact: C2RustUnnamed_2 = 30;
pub type C2RustUnnamed_2 = libc::c_uint;
/*
 * Copyright (C) 2007 Denys Vlasenko <vda.linux@googlemail.com>
 *
 * This file uses bzip2 library code which is written
 * by Julian Seward <jseward@bzip.org>.
 * See README and LICENSE files in bz/ directory for more information
 * about bzip2 library code.
 */
//config:config BZIP2
//config:	bool "bzip2 (16 kb)"
//config:	default y
//config:	help
//config:	bzip2 is a compression utility using the Burrows-Wheeler block
//config:	sorting text compression algorithm, and Huffman coding. Compression
//config:	is generally considerably better than that achieved by more
//config:	conventional LZ77/LZ78-based compressors, and approaches the
//config:	performance of the PPM family of statistical compressors.
//config:
//config:	Unless you have a specific application which requires bzip2, you
//config:	should probably say N here.
//config:
//config:config BZIP2_SMALL
//config:	int "Trade bytes for speed (0:fast, 9:small)"
//config:	default 8  # all "fast or small" options default to small
//config:	range 0 9
//config:	depends on BZIP2
//config:	help
//config:	Trade code size versus speed.
//config:	Approximate values with gcc-6.3.0 "bzip -9" compressing
//config:	linux-4.15.tar were:
//config:	value         time (sec)  code size (386)
//config:	9 (smallest)       70.11             7687
//config:	8                  67.93             8091
//config:	7                  67.88             8405
//config:	6                  67.78             8624
//config:	5                  67.05             9427
//config:	4-0 (fastest)      64.14            12083
//config:
//config:config FEATURE_BZIP2_DECOMPRESS
//config:	bool "Enable decompression"
//config:	default y
//config:	depends on BZIP2 || BUNZIP2 || BZCAT
//config:	help
//config:	Enable -d (--decompress) and -t (--test) options for bzip2.
//config:	This will be automatically selected if bunzip2 or bzcat is
//config:	enabled.
//applet:IF_BZIP2(APPLET(bzip2, BB_DIR_USR_BIN, SUID_DROP))
//kbuild:lib-$(CONFIG_BZIP2) += bzip2.o
//usage:#define bzip2_trivial_usage
//usage:       "[OPTIONS] [FILE]..."
//usage:#define bzip2_full_usage "\n\n"
//usage:       "Compress FILEs (or stdin) with bzip2 algorithm\n"
//usage:     "\n	-1..9	Compression level"
//usage:	IF_FEATURE_BZIP2_DECOMPRESS(
//usage:     "\n	-d	Decompress"
//usage:     "\n	-t	Test file integrity"
//usage:	)
//usage:     "\n	-c	Write to stdout"
//usage:     "\n	-f	Force"
//usage:     "\n	-k	Keep input files"
/* Speed test:
 * Compiled with gcc 4.2.1, run on Athlon 64 1800 MHz (512K L2 cache).
 * Stock bzip2 is 26.4% slower than bbox bzip2 at SPEED 1
 * (time to compress gcc-4.2.1.tar is 126.4% compared to bbox).
 * At SPEED 5 difference is 32.7%.
 *
 * Test run of all BZIP2_SPEED values on a 11Mb text file:
 *     Size   Time (3 runs)
 * 0:  10828  4.145 4.146 4.148
 * 1:  11097  3.845 3.860 3.861
 * 2:  11392  3.763 3.767 3.768
 * 3:  11892  3.722 3.724 3.727
 * 4:  12740  3.637 3.640 3.644
 * 5:  17273  3.497 3.509 3.509
 */
/* Takes ~300 bytes, detects corruption caused by bad RAM etc */
/* No point in being shy and having very small buffer here.
 * bzip2 internal buffers are much bigger anyway, hundreds of kbytes.
 * If iobuf is several pages long, malloc() may use mmap,
 * making iobuf page aligned and thus (maybe) have one memcpy less
 * if kernel is clever enough.
 */
pub type C2RustUnnamed_3 = libc::c_uint;
pub const IOBUF_SIZE: C2RustUnnamed_3 = 8192;
/*
 * bzip2 is written by Julian Seward <jseward@bzip.org>.
 * Adapted for busybox by Denys Vlasenko <vda.linux@googlemail.com>.
 * See README and LICENSE files in this directory for more information.
 */
/*-------------------------------------------------------------*/
/*--- Block sorting machinery                               ---*/
/*---                                           blocksort.c ---*/
/*-------------------------------------------------------------*/
/* ------------------------------------------------------------------
This file is part of bzip2/libbzip2, a program and library for
lossless, block-sorting data compression.

bzip2/libbzip2 version 1.0.4 of 20 December 2006
Copyright (C) 1996-2006 Julian Seward <jseward@bzip.org>

Please read the WARNING, DISCLAIMER and PATENTS sections in the
README file.

This program is released under the terms of the license contained
in the file LICENSE.
------------------------------------------------------------------ */
/* #include "bzlib_private.h" */
unsafe extern "C" fn mvswap(mut ptr: *mut u32, mut zzp1: i32, mut zzp2: i32, mut zzn: i32) {
  while zzn > 0i32 {
    let mut zztmp: i32 = *ptr.offset(zzp1 as isize) as i32;
    *ptr.offset(zzp1 as isize) = *ptr.offset(zzp2 as isize);
    *ptr.offset(zzp2 as isize) = zztmp as u32;
    zzp1 += 1;
    zzp2 += 1;
    zzn -= 1
  }
}
#[inline(always)]
unsafe extern "C" fn mmin(mut a: i32, mut b: i32) -> i32 {
  return if a < b { a } else { b };
}
/*---------------------------------------------*/
/*--- Fallback O(N log(N)^2) sorting        ---*/
/*--- algorithm, for repetitive blocks      ---*/
/*---------------------------------------------*/
/*---------------------------------------------*/
#[inline]
unsafe extern "C" fn fallbackSimpleSort(
  mut fmap: *mut u32,
  mut eclass: *mut u32,
  mut lo: i32,
  mut hi: i32,
) {
  let mut i: i32 = 0;
  let mut j: i32 = 0;
  let mut tmp: i32 = 0;
  let mut ec_tmp: u32 = 0;
  if lo == hi {
    return;
  }
  if hi - lo > 3i32 {
    i = hi - 4i32;
    while i >= lo {
      tmp = *fmap.offset(i as isize) as i32;
      ec_tmp = *eclass.offset(tmp as isize);
      j = i + 4i32;
      while j <= hi && ec_tmp > *eclass.offset(*fmap.offset(j as isize) as isize) {
        *fmap.offset((j - 4i32) as isize) = *fmap.offset(j as isize);
        j += 4i32
      }
      *fmap.offset((j - 4i32) as isize) = tmp as u32;
      i -= 1
    }
  }
  i = hi - 1i32;
  while i >= lo {
    tmp = *fmap.offset(i as isize) as i32;
    ec_tmp = *eclass.offset(tmp as isize);
    j = i + 1i32;
    while j <= hi && ec_tmp > *eclass.offset(*fmap.offset(j as isize) as isize) {
      *fmap.offset((j - 1i32) as isize) = *fmap.offset(j as isize);
      j += 1
    }
    *fmap.offset((j - 1i32) as isize) = tmp as u32;
    i -= 1
  }
}
unsafe extern "C" fn fallbackQSort3(
  mut fmap: *mut u32,
  mut eclass: *mut u32,
  mut loSt: i32,
  mut hiSt: i32,
) {
  let mut sp: i32 = 0;
  let mut r: u32 = 0;
  let mut stackLo: [i32; 100] = [0; 100];
  let mut stackHi: [i32; 100] = [0; 100];
  r = 0i32 as u32;
  sp = 0i32;
  stackLo[sp as usize] = loSt;
  stackHi[sp as usize] = hiSt;
  sp += 1;
  while sp > 0i32 {
    let mut unLo: i32 = 0;
    let mut unHi: i32 = 0;
    let mut ltLo: i32 = 0;
    let mut gtHi: i32 = 0;
    let mut n: i32 = 0;
    let mut m: i32 = 0;
    let mut lo: i32 = 0;
    let mut hi: i32 = 0;
    let mut med: u32 = 0;
    let mut r3: u32 = 0;
    sp -= 1;
    lo = stackLo[sp as usize];
    hi = stackHi[sp as usize];
    if hi - lo < 10i32 {
      fallbackSimpleSort(fmap, eclass, lo, hi);
    } else {
      /* Random partitioning.  Median of 3 sometimes fails to
       * avoid bad cases.  Median of 9 seems to help but
       * looks rather expensive.  This too seems to work but
       * is cheaper.  Guidance for the magic constants
       * 7621 and 32768 is taken from Sedgewick's algorithms
       * book, chapter 35.
       */
      r = r
        .wrapping_mul(7621i32 as libc::c_uint)
        .wrapping_add(1i32 as libc::c_uint)
        .wrapping_rem(32768i32 as libc::c_uint);
      r3 = r.wrapping_rem(3i32 as libc::c_uint);
      if r3 == 0i32 as libc::c_uint {
        med = *eclass.offset(*fmap.offset(lo as isize) as isize)
      } else if r3 == 1i32 as libc::c_uint {
        med = *eclass.offset(*fmap.offset((lo + hi >> 1i32) as isize) as isize)
      } else {
        med = *eclass.offset(*fmap.offset(hi as isize) as isize)
      }
      ltLo = lo;
      unLo = ltLo;
      gtHi = hi;
      unHi = gtHi;
      loop {
        while !(unLo > unHi) {
          n = *eclass.offset(*fmap.offset(unLo as isize) as isize) as i32 - med as i32;
          if n == 0i32 {
            let mut zztmp: i32 = *fmap.offset(unLo as isize) as i32;
            *fmap.offset(unLo as isize) = *fmap.offset(ltLo as isize);
            *fmap.offset(ltLo as isize) = zztmp as u32;
            ltLo += 1;
            unLo += 1
          } else {
            if n > 0i32 {
              break;
            }
            unLo += 1
          }
        }
        while !(unLo > unHi) {
          n = *eclass.offset(*fmap.offset(unHi as isize) as isize) as i32 - med as i32;
          if n == 0i32 {
            let mut zztmp_0: i32 = *fmap.offset(unHi as isize) as i32;
            *fmap.offset(unHi as isize) = *fmap.offset(gtHi as isize);
            *fmap.offset(gtHi as isize) = zztmp_0 as u32;
            gtHi -= 1;
            unHi -= 1
          } else {
            if n < 0i32 {
              break;
            }
            unHi -= 1
          }
        }
        if unLo > unHi {
          break;
        }
        let mut zztmp_1: i32 = *fmap.offset(unLo as isize) as i32;
        *fmap.offset(unLo as isize) = *fmap.offset(unHi as isize);
        *fmap.offset(unHi as isize) = zztmp_1 as u32;
        unLo += 1;
        unHi -= 1
      }
      if gtHi < ltLo {
        continue;
      }
      n = mmin(ltLo - lo, unLo - ltLo);
      mvswap(fmap, lo, unLo - n, n);
      m = mmin(hi - gtHi, gtHi - unHi);
      mvswap(fmap, unLo, hi - m + 1i32, m);
      n = lo + unLo - ltLo - 1i32;
      m = hi - (gtHi - unHi) + 1i32;
      if n - lo > hi - m {
        stackLo[sp as usize] = lo;
        stackHi[sp as usize] = n;
        sp += 1;
        stackLo[sp as usize] = m;
        stackHi[sp as usize] = hi;
        sp += 1
      } else {
        stackLo[sp as usize] = m;
        stackHi[sp as usize] = hi;
        sp += 1;
        stackLo[sp as usize] = lo;
        stackHi[sp as usize] = n;
        sp += 1
      }
    }
  }
}
/*---------------------------------------------*/
/* Pre:
 *	nblock > 0
 *	eclass exists for [0 .. nblock-1]
 *	((u8*)eclass) [0 .. nblock-1] holds block
 *	ptr exists for [0 .. nblock-1]
 *
 * Post:
 *	((u8*)eclass) [0 .. nblock-1] holds block
 *	All other areas of eclass destroyed
 *	fmap [0 .. nblock-1] holds sorted order
 *	bhtab[0 .. 2+(nblock/32)] destroyed
*/
unsafe extern "C" fn fallbackSort(mut state: *mut EState) {
  let mut ftab: [i32; 257] = [0; 257];
  let mut ftabCopy: [i32; 256] = [0; 256];
  let mut H: i32 = 0;
  let mut i: i32 = 0;
  let mut j: i32 = 0;
  let mut k: i32 = 0;
  let mut l: i32 = 0;
  let mut r: i32 = 0;
  let mut cc: i32 = 0;
  let mut cc1: i32 = 0;
  let mut nNotDone: i32 = 0;
  let mut nBhtab: i32 = 0;
  /* params */
  let fmap: *mut u32 = (*state).arr1;
  let eclass: *mut u32 = (*state).arr2;
  let bhtab: *mut u32 = (*state).ftab;
  let nblock: i32 = (*state).nblock;
  /*
   * Initial 1-char radix sort to generate
   * initial fmap and initial BH bits.
   */
  i = 0i32; /* bbox: optimized */
  while i < 257i32 {
    ftab[i as usize] = 0i32; /* bbox: unsigned div is easier */
    i += 1
  }
  i = 0i32;
  while i < nblock {
    ftab[*(eclass as *mut u8).offset(i as isize) as usize] += 1;
    i += 1
  }
  i = 0i32;
  while i < 256i32 {
    ftabCopy[i as usize] = ftab[i as usize];
    i += 1
  }
  j = ftab[0];
  i = 1i32;
  while i < 257i32 {
    j += ftab[i as usize];
    ftab[i as usize] = j;
    i += 1
  }
  i = 0i32;
  while i < nblock {
    j = *(eclass as *mut u8).offset(i as isize) as i32;
    k = ftab[j as usize] - 1i32;
    ftab[j as usize] = k;
    *fmap.offset(k as isize) = i as u32;
    i += 1
  }
  nBhtab =
    (2i32 as libc::c_uint).wrapping_add((nblock as u32).wrapping_div(32i32 as libc::c_uint)) as i32;
  i = 0i32;
  while i < nBhtab {
    *bhtab.offset(i as isize) = 0i32 as u32;
    i += 1
  }
  i = 0i32;
  while i < 256i32 {
    let ref mut fresh0 = *bhtab.offset((ftab[i as usize] >> 5i32) as isize);
    *fresh0 |= (1i32 << (ftab[i as usize] & 31i32)) as libc::c_uint;
    i += 1
  }
  /*
   * Inductively refine the buckets.  Kind-of an
   * "exponential radix sort" (!), inspired by the
   * Manber-Myers suffix array construction algorithm.
   */
  /*-- set sentinel bits for block-end detection --*/
  i = 0i32;
  while i < 32i32 {
    let ref mut fresh1 = *bhtab.offset((nblock + 2i32 * i >> 5i32) as isize);
    *fresh1 |= (1i32 << (nblock + 2i32 * i & 31i32)) as libc::c_uint;
    let ref mut fresh2 = *bhtab.offset((nblock + 2i32 * i + 1i32 >> 5i32) as isize);
    *fresh2 &= !(1i32 << (nblock + 2i32 * i + 1i32 & 31i32)) as libc::c_uint;
    i += 1
  }
  /*-- the log(N) loop --*/
  H = 1i32;
  loop {
    j = 0i32;
    i = 0i32;
    while i < nblock {
      if *bhtab.offset((i >> 5i32) as isize) & (1i32 << (i & 31i32)) as libc::c_uint != 0 {
        j = i
      }
      k = (*fmap.offset(i as isize)).wrapping_sub(H as libc::c_uint) as i32;
      if k < 0i32 {
        k += nblock
      }
      *eclass.offset(k as isize) = j as u32;
      i += 1
    }
    nNotDone = 0i32;
    r = -1i32;
    loop {
      /*-- find the next non-singleton bucket --*/
      k = r + 1i32;
      while *bhtab.offset((k >> 5i32) as isize) & (1i32 << (k & 31i32)) as libc::c_uint != 0
        && k & 0x1fi32 != 0
      {
        k += 1
      }
      if *bhtab.offset((k >> 5i32) as isize) & (1i32 << (k & 31i32)) as libc::c_uint != 0 {
        while *bhtab.offset((k >> 5i32) as isize) == 0xffffffffu32 {
          k += 32i32
        }
        while *bhtab.offset((k >> 5i32) as isize) & (1i32 << (k & 31i32)) as libc::c_uint != 0 {
          k += 1
        }
      }
      l = k - 1i32;
      if l >= nblock {
        break;
      }
      while *bhtab.offset((k >> 5i32) as isize) & (1i32 << (k & 31i32)) as libc::c_uint == 0
        && k & 0x1fi32 != 0
      {
        k += 1
      }
      if *bhtab.offset((k >> 5i32) as isize) & (1i32 << (k & 31i32)) as libc::c_uint == 0 {
        while *bhtab.offset((k >> 5i32) as isize) == 0i32 as libc::c_uint {
          k += 32i32
        }
        while *bhtab.offset((k >> 5i32) as isize) & (1i32 << (k & 31i32)) as libc::c_uint == 0 {
          k += 1
        }
      }
      r = k - 1i32;
      if r >= nblock {
        break;
      }
      /*-- now [l, r] bracket current bucket --*/
      if r > l {
        nNotDone += r - l + 1i32;
        fallbackQSort3(fmap, eclass, l, r);
        /*-- scan bucket and generate header bits-- */
        cc = -1i32;
        i = l;
        while i <= r {
          cc1 = *eclass.offset(*fmap.offset(i as isize) as isize) as i32;
          if cc != cc1 {
            let ref mut fresh3 = *bhtab.offset((i >> 5i32) as isize);
            *fresh3 |= (1i32 << (i & 31i32)) as libc::c_uint;
            cc = cc1
          }
          i += 1
        }
      }
    }
    H *= 2i32;
    if H > nblock || nNotDone == 0i32 {
      break;
    }
  }
  /*
   * Reconstruct the original block in
   * eclass8 [0 .. nblock-1], since the
   * previous phase destroyed it.
   */
  j = 0i32;
  i = 0i32;
  while i < nblock {
    while ftabCopy[j as usize] == 0i32 {
      j += 1
    }
    ftabCopy[j as usize] -= 1;
    *(eclass as *mut u8).offset(*fmap.offset(i as isize) as isize) = j as u8;
    i += 1
  }
}
/*---------------------------------------------*/
/*--- The main, O(N^2 log(N)) sorting       ---*/
/*--- algorithm.  Faster for "normal"       ---*/
/*--- non-repetitive blocks.                ---*/
/*---------------------------------------------*/
/*---------------------------------------------*/
#[inline(never)]
unsafe extern "C" fn mainGtU(mut state: *mut EState, mut i1: u32, mut i2: u32) -> libc::c_int {
  let mut k: i32 = 0;
  let mut c1: u8 = 0;
  let mut c2: u8 = 0;
  let mut s1: u16 = 0;
  let mut s2: u16 = 0;
  let block: *mut u8 = (*state).block;
  let quadrant: *mut u16 = (*state).quadrant;
  let nblock: i32 = (*state).nblock;
  /* Loop unrolling here is actually very useful
   * (generated code is much simpler),
   * code size increase is only 270 bytes (i386)
   * but speeds up compression 10% overall
   */
  c1 = *block.offset(i1 as isize);
  c2 = *block.offset(i2 as isize);
  if c1 as libc::c_int != c2 as libc::c_int {
    return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int;
  }
  i1 = i1.wrapping_add(1);
  i2 = i2.wrapping_add(1);
  c1 = *block.offset(i1 as isize);
  c2 = *block.offset(i2 as isize);
  if c1 as libc::c_int != c2 as libc::c_int {
    return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int;
  }
  i1 = i1.wrapping_add(1);
  i2 = i2.wrapping_add(1);
  c1 = *block.offset(i1 as isize);
  c2 = *block.offset(i2 as isize);
  if c1 as libc::c_int != c2 as libc::c_int {
    return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int;
  }
  i1 = i1.wrapping_add(1);
  i2 = i2.wrapping_add(1);
  c1 = *block.offset(i1 as isize);
  c2 = *block.offset(i2 as isize);
  if c1 as libc::c_int != c2 as libc::c_int {
    return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int;
  }
  i1 = i1.wrapping_add(1);
  i2 = i2.wrapping_add(1);
  c1 = *block.offset(i1 as isize);
  c2 = *block.offset(i2 as isize);
  if c1 as libc::c_int != c2 as libc::c_int {
    return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int;
  }
  i1 = i1.wrapping_add(1);
  i2 = i2.wrapping_add(1);
  c1 = *block.offset(i1 as isize);
  c2 = *block.offset(i2 as isize);
  if c1 as libc::c_int != c2 as libc::c_int {
    return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int;
  }
  i1 = i1.wrapping_add(1);
  i2 = i2.wrapping_add(1);
  c1 = *block.offset(i1 as isize);
  c2 = *block.offset(i2 as isize);
  if c1 as libc::c_int != c2 as libc::c_int {
    return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int;
  }
  i1 = i1.wrapping_add(1);
  i2 = i2.wrapping_add(1);
  c1 = *block.offset(i1 as isize);
  c2 = *block.offset(i2 as isize);
  if c1 as libc::c_int != c2 as libc::c_int {
    return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int;
  }
  i1 = i1.wrapping_add(1);
  i2 = i2.wrapping_add(1);
  c1 = *block.offset(i1 as isize);
  c2 = *block.offset(i2 as isize);
  if c1 as libc::c_int != c2 as libc::c_int {
    return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int;
  }
  i1 = i1.wrapping_add(1);
  i2 = i2.wrapping_add(1);
  c1 = *block.offset(i1 as isize);
  c2 = *block.offset(i2 as isize);
  if c1 as libc::c_int != c2 as libc::c_int {
    return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int;
  }
  i1 = i1.wrapping_add(1);
  i2 = i2.wrapping_add(1);
  c1 = *block.offset(i1 as isize);
  c2 = *block.offset(i2 as isize);
  if c1 as libc::c_int != c2 as libc::c_int {
    return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int;
  }
  i1 = i1.wrapping_add(1);
  i2 = i2.wrapping_add(1);
  c1 = *block.offset(i1 as isize);
  c2 = *block.offset(i2 as isize);
  if c1 as libc::c_int != c2 as libc::c_int {
    return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int;
  }
  i1 = i1.wrapping_add(1);
  i2 = i2.wrapping_add(1);
  k = nblock + 8i32;
  loop {
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
      return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int;
    }
    s1 = *quadrant.offset(i1 as isize);
    s2 = *quadrant.offset(i2 as isize);
    if s1 as libc::c_int != s2 as libc::c_int {
      return (s1 as libc::c_int > s2 as libc::c_int) as libc::c_int;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
      return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int;
    }
    s1 = *quadrant.offset(i1 as isize);
    s2 = *quadrant.offset(i2 as isize);
    if s1 as libc::c_int != s2 as libc::c_int {
      return (s1 as libc::c_int > s2 as libc::c_int) as libc::c_int;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
      return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int;
    }
    s1 = *quadrant.offset(i1 as isize);
    s2 = *quadrant.offset(i2 as isize);
    if s1 as libc::c_int != s2 as libc::c_int {
      return (s1 as libc::c_int > s2 as libc::c_int) as libc::c_int;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
      return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int;
    }
    s1 = *quadrant.offset(i1 as isize);
    s2 = *quadrant.offset(i2 as isize);
    if s1 as libc::c_int != s2 as libc::c_int {
      return (s1 as libc::c_int > s2 as libc::c_int) as libc::c_int;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
      return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int;
    }
    s1 = *quadrant.offset(i1 as isize);
    s2 = *quadrant.offset(i2 as isize);
    if s1 as libc::c_int != s2 as libc::c_int {
      return (s1 as libc::c_int > s2 as libc::c_int) as libc::c_int;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
      return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int;
    }
    s1 = *quadrant.offset(i1 as isize);
    s2 = *quadrant.offset(i2 as isize);
    if s1 as libc::c_int != s2 as libc::c_int {
      return (s1 as libc::c_int > s2 as libc::c_int) as libc::c_int;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
      return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int;
    }
    s1 = *quadrant.offset(i1 as isize);
    s2 = *quadrant.offset(i2 as isize);
    if s1 as libc::c_int != s2 as libc::c_int {
      return (s1 as libc::c_int > s2 as libc::c_int) as libc::c_int;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
      return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int;
    }
    s1 = *quadrant.offset(i1 as isize);
    s2 = *quadrant.offset(i2 as isize);
    if s1 as libc::c_int != s2 as libc::c_int {
      return (s1 as libc::c_int > s2 as libc::c_int) as libc::c_int;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    if i1 >= nblock as libc::c_uint {
      i1 = (i1 as libc::c_uint).wrapping_sub(nblock as libc::c_uint) as u32 as u32
    }
    if i2 >= nblock as libc::c_uint {
      i2 = (i2 as libc::c_uint).wrapping_sub(nblock as libc::c_uint) as u32 as u32
    }
    (*state).budget -= 1;
    k -= 8i32;
    if !(k >= 0i32) {
      break;
    }
  }
  return 0i32 as Bool as libc::c_int;
}
/*---------------------------------------------*/
/*
 * Knuth's increments seem to work better
 * than Incerpi-Sedgewick here.  Possibly
 * because the number of elems to sort is
 * usually small, typically <= 20.
 */
static mut incs: [u32; 14] = [
  1i32 as u32,
  4i32 as u32,
  13i32 as u32,
  40i32 as u32,
  121i32 as u32,
  364i32 as u32,
  1093i32 as u32,
  3280i32 as u32,
  9841i32 as u32,
  29524i32 as u32,
  88573i32 as u32,
  265720i32 as u32,
  797161i32 as u32,
  2391484i32 as u32,
];
unsafe extern "C" fn mainSimpleSort(mut state: *mut EState, mut lo: i32, mut hi: i32, mut d: i32) {
  let ptr: *mut u32 = (*state).ptr;
  /* At which increment to start? */
  let mut hp: libc::c_int = 0i32;
  let mut bigN: libc::c_int = hi - lo;
  if bigN <= 0i32 {
    return;
  }
  while incs[hp as usize] <= bigN as libc::c_uint {
    hp += 1
  }
  hp -= 1;
  while hp >= 0i32 {
    let mut i: i32 = 0;
    let mut h: libc::c_uint = 0;
    h = incs[hp as usize];
    i = (lo as libc::c_uint).wrapping_add(h) as i32;
    loop {
      let mut j: libc::c_uint = 0;
      let mut v: libc::c_uint = 0;
      if i > hi {
        break;
      }
      v = *ptr.offset(i as isize);
      j = i as libc::c_uint;
      while mainGtU(
        state,
        (*ptr.offset(j.wrapping_sub(h) as isize)).wrapping_add(d as libc::c_uint),
        v.wrapping_add(d as libc::c_uint),
      ) != 0
      {
        *ptr.offset(j as isize) = *ptr.offset(j.wrapping_sub(h) as isize);
        j = j.wrapping_sub(h);
        if j
          <= (lo as libc::c_uint)
            .wrapping_add(h)
            .wrapping_sub(1i32 as libc::c_uint)
        {
          break;
        }
      }
      *ptr.offset(j as isize) = v;
      i += 1;
      /* 1.5% overall speedup, +290 bytes */
      if (*state).budget < 0i32 {
        return;
      }
    }
    hp -= 1
  }
}
/*---------------------------------------------*/
/*
 * The following is an implementation of
 * an elegant 3-way quicksort for strings,
 * described in a paper "Fast Algorithms for
 * Sorting and Searching Strings", by Robert
 * Sedgewick and Jon L. Bentley.
 */
#[inline(always)]
unsafe extern "C" fn mmed3(mut a: u8, mut b: u8, mut c: u8) -> u8 {
  let mut t: u8 = 0;
  if a as libc::c_int > b as libc::c_int {
    t = a;
    a = b;
    b = t
  }
  /* here b >= a */
  if b as libc::c_int > c as libc::c_int {
    b = c;
    if a as libc::c_int > b as libc::c_int {
      b = a
    }
  }
  return b;
}
#[inline(never)]
unsafe extern "C" fn mainQSort3(mut state: *mut EState, mut loSt: i32, mut hiSt: i32)
/*i32 dSt*/
{
  let mut unLo: i32 = 0;
  let mut unHi: i32 = 0;
  let mut ltLo: i32 = 0;
  let mut gtHi: i32 = 0;
  let mut n: i32 = 0;
  let mut m: i32 = 0;
  let mut med: i32 = 0;
  let mut sp: i32 = 0;
  let mut lo: i32 = 0;
  let mut hi: i32 = 0;
  let mut d: i32 = 0;
  let mut stackLo: [i32; 100] = [0; 100];
  let mut stackHi: [i32; 100] = [0; 100];
  let mut stackD: [i32; 100] = [0; 100];
  let mut nextLo: [i32; 3] = [0; 3];
  let mut nextHi: [i32; 3] = [0; 3];
  let mut nextD: [i32; 3] = [0; 3];
  let ptr: *mut u32 = (*state).ptr;
  let block: *mut u8 = (*state).block;
  sp = 0i32;
  stackLo[sp as usize] = loSt;
  stackHi[sp as usize] = hiSt;
  stackD[sp as usize] = dSt as libc::c_int;
  sp += 1;
  while sp > 0i32 {
    sp -= 1;
    lo = stackLo[sp as usize];
    hi = stackHi[sp as usize];
    d = stackD[sp as usize];
    if hi - lo < 20i32 || d > 2i32 + 12i32 {
      mainSimpleSort(state, lo, hi, d);
      if (*state).budget < 0i32 {
        return;
      }
    } else {
      med = mmed3(
        *block.offset((*ptr.offset(lo as isize)).wrapping_add(d as libc::c_uint) as isize),
        *block.offset((*ptr.offset(hi as isize)).wrapping_add(d as libc::c_uint) as isize),
        *block.offset(
          (*ptr.offset((lo + hi >> 1i32) as isize)).wrapping_add(d as libc::c_uint) as isize,
        ),
      ) as i32;
      ltLo = lo;
      unLo = ltLo;
      gtHi = hi;
      unHi = gtHi;
      loop {
        while !(unLo > unHi) {
          n = *block.offset((*ptr.offset(unLo as isize)).wrapping_add(d as libc::c_uint) as isize)
            as i32
            - med;
          if n == 0i32 {
            let mut zztmp: i32 = *ptr.offset(unLo as isize) as i32;
            *ptr.offset(unLo as isize) = *ptr.offset(ltLo as isize);
            *ptr.offset(ltLo as isize) = zztmp as u32;
            ltLo += 1;
            unLo += 1
          } else {
            if n > 0i32 {
              break;
            }
            unLo += 1
          }
        }
        while !(unLo > unHi) {
          n = *block.offset((*ptr.offset(unHi as isize)).wrapping_add(d as libc::c_uint) as isize)
            as i32
            - med;
          if n == 0i32 {
            let mut zztmp_0: i32 = *ptr.offset(unHi as isize) as i32;
            *ptr.offset(unHi as isize) = *ptr.offset(gtHi as isize);
            *ptr.offset(gtHi as isize) = zztmp_0 as u32;
            gtHi -= 1;
            unHi -= 1
          } else {
            if n < 0i32 {
              break;
            }
            unHi -= 1
          }
        }
        if unLo > unHi {
          break;
        }
        let mut zztmp_1: i32 = *ptr.offset(unLo as isize) as i32;
        *ptr.offset(unLo as isize) = *ptr.offset(unHi as isize);
        *ptr.offset(unHi as isize) = zztmp_1 as u32;
        unLo += 1;
        unHi -= 1
      }
      if gtHi < ltLo {
        stackLo[sp as usize] = lo;
        stackHi[sp as usize] = hi;
        stackD[sp as usize] = d + 1i32;
        sp += 1
      } else {
        n = mmin(ltLo - lo, unLo - ltLo);
        mvswap(ptr, lo, unLo - n, n);
        m = mmin(hi - gtHi, gtHi - unHi);
        mvswap(ptr, unLo, hi - m + 1i32, m);
        n = lo + unLo - ltLo - 1i32;
        m = hi - (gtHi - unHi) + 1i32;
        nextLo[0] = lo;
        nextHi[0] = n;
        nextD[0] = d;
        nextLo[1] = m;
        nextHi[1] = hi;
        nextD[1] = d;
        nextLo[2] = n + 1i32;
        nextHi[2] = m - 1i32;
        nextD[2] = d + 1i32;
        if nextHi[0] - nextLo[0] < nextHi[1] - nextLo[1] {
          let mut tz: i32 = 0;
          tz = nextLo[0];
          nextLo[0] = nextLo[1];
          nextLo[1] = tz;
          tz = nextHi[0];
          nextHi[0] = nextHi[1];
          nextHi[1] = tz;
          tz = nextD[0];
          nextD[0] = nextD[1];
          nextD[1] = tz
        }
        if nextHi[1] - nextLo[1] < nextHi[2] - nextLo[2] {
          let mut tz_0: i32 = 0;
          tz_0 = nextLo[1];
          nextLo[1] = nextLo[2];
          nextLo[2] = tz_0;
          tz_0 = nextHi[1];
          nextHi[1] = nextHi[2];
          nextHi[2] = tz_0;
          tz_0 = nextD[1];
          nextD[1] = nextD[2];
          nextD[2] = tz_0
        }
        if nextHi[0] - nextLo[0] < nextHi[1] - nextLo[1] {
          let mut tz_1: i32 = 0;
          tz_1 = nextLo[0];
          nextLo[0] = nextLo[1];
          nextLo[1] = tz_1;
          tz_1 = nextHi[0];
          nextHi[0] = nextHi[1];
          nextHi[1] = tz_1;
          tz_1 = nextD[0];
          nextD[0] = nextD[1];
          nextD[1] = tz_1
        }
        stackLo[sp as usize] = nextLo[0];
        stackHi[sp as usize] = nextHi[0];
        stackD[sp as usize] = nextD[0];
        sp += 1;
        stackLo[sp as usize] = nextLo[1];
        stackHi[sp as usize] = nextHi[1];
        stackD[sp as usize] = nextD[1];
        sp += 1;
        stackLo[sp as usize] = nextLo[2];
        stackHi[sp as usize] = nextHi[2];
        stackD[sp as usize] = nextD[2];
        sp += 1
      }
    }
  }
}
#[inline(never)]
unsafe extern "C" fn mainSort(mut state: *mut EState) {
  let mut i: i32 = 0;
  let mut j: i32 = 0;
  let mut bigDone: [Bool; 256] = [0; 256];
  let mut runningOrder: [u8; 256] = [0; 256];
  /* bbox: moved to EState to save stack
  i32  copyStart[256];
  i32  copyEnd  [256];
  */
  let ptr: *mut u32 = (*state).ptr;
  let block: *mut u8 = (*state).block;
  let ftab: *mut u32 = (*state).ftab;
  let nblock: i32 = (*state).nblock;
  let quadrant: *mut u16 = (*state).quadrant;
  /*-- set up the 2-byte frequency table --*/
  /* was: for (i = 65536; i >= 0; i--) ftab[i] = 0; */
  memset(
    ftab as *mut libc::c_void,
    0i32,
    (65537i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<u32>() as libc::c_ulong),
  );
  j = (*block.offset(0) as libc::c_int) << 8i32;
  i = nblock - 1i32;
  /* 3%, +300 bytes */
  while i >= 0i32 {
    *quadrant.offset(i as isize) = 0i32 as u16;
    j = ((j >> 8i32) as libc::c_uint | (*block.offset(i as isize) as libc::c_uint) << 8i32) as i32;
    let ref mut fresh4 = *ftab.offset(j as isize);
    *fresh4 = (*fresh4).wrapping_add(1);
    i -= 1
  }
  /*-- (emphasises close relationship of block & quadrant) --*/
  i = 0i32;
  while i < 2i32 + 12i32 + 18i32 + 2i32 {
    *block.offset((nblock + i) as isize) = *block.offset(i as isize);
    *quadrant.offset((nblock + i) as isize) = 0i32 as u16;
    i += 1
  }
  /*-- Complete the initial radix sort --*/
  j = *ftab.offset(0) as i32; /* bbox: optimized */
  i = 1i32;
  while i <= 65536i32 {
    j = (j as libc::c_uint).wrapping_add(*ftab.offset(i as isize)) as i32 as i32;
    *ftab.offset(i as isize) = j as u32;
    i += 1
  }
  let mut s: libc::c_uint = 0;
  s = ((*block.offset(0) as libc::c_int) << 8i32) as libc::c_uint;
  i = nblock - 1i32;
  while i >= 0i32 {
    s = s >> 8i32 | ((*block.offset(i as isize) as libc::c_int) << 8i32) as libc::c_uint;
    j = (*ftab.offset(s as isize)).wrapping_sub(1i32 as libc::c_uint) as i32;
    *ftab.offset(s as isize) = j as u32;
    *ptr.offset(j as isize) = i as u32;
    i -= 1
  }
  /*
   * Now ftab contains the first loc of every small bucket.
   * Calculate the running order, from smallest to largest
   * big bucket.
   */
  i = 0i32;
  while i <= 255i32 {
    bigDone[i as usize] = 0i32 as Bool;
    runningOrder[i as usize] = i as u8;
    i += 1
  }
  /* bbox: was: i32 h = 1; */
  /* do h = 3 * h + 1; while (h <= 256); */
  let mut h: libc::c_uint = 364i32 as libc::c_uint;
  loop {
    /*h = h / 3;*/
    h = h.wrapping_mul(171i32 as libc::c_uint) >> 9i32; /* bbox: fast h/3 */
    i = h as i32; /* uint8[] */
    while i <= 255i32 {
      let mut vv: libc::c_uint = 0;
      let mut jh: libc::c_uint = 0;
      vv = runningOrder[i as usize] as libc::c_uint;
      j = i;
      loop {
        jh = (j as libc::c_uint).wrapping_sub(h);
        if !((*ftab.offset((runningOrder[jh as usize] as libc::c_int + 1i32 << 8i32) as isize))
          .wrapping_sub(
            *ftab.offset(((runningOrder[jh as usize] as libc::c_int) << 8i32) as isize),
          )
          > (*ftab.offset((vv.wrapping_add(1i32 as libc::c_uint) << 8i32) as isize))
            .wrapping_sub(*ftab.offset((vv << 8i32) as isize)))
        {
          break;
        }
        runningOrder[j as usize] = runningOrder[jh as usize];
        j = jh as i32;
        if (j as libc::c_uint) < h {
          break;
        }
      }
      runningOrder[j as usize] = vv as u8;
      i += 1
    }
    if !(h != 1i32 as libc::c_uint) {
      break;
    }
  }
  /*
   * The main sorting loop.
   */
  i = 0i32;
  loop {
    let mut ss: libc::c_uint = 0;
    /*
     * Process big buckets, starting with the least full.
     * Basically this is a 3-step process in which we call
     * mainQSort3 to sort the small buckets [ss, j], but
     * also make a big effort to avoid the calls if we can.
     */
    ss = runningOrder[i as usize] as libc::c_uint;
    /*
     * Step 1:
     * Complete the big bucket [ss] by quicksorting
     * any unsorted small buckets [ss, j], for j != ss.
     * Hopefully previous pointer-scanning phases have already
     * completed many of the small buckets [ss, j], so
     * we don't have to sort them at all.
     */
    j = 0i32;
    while j <= 255i32 {
      if j as libc::c_uint != ss {
        let mut sb: libc::c_uint = 0;
        sb = (ss << 8i32).wrapping_add(j as libc::c_uint);
        if *ftab.offset(sb as isize) & (1i32 << 21i32) as libc::c_uint == 0 {
          let mut lo: i32 = *ftab.offset(sb as isize) as i32;
          let mut hi: i32 = (*ftab.offset(sb.wrapping_add(1i32 as libc::c_uint) as isize)
            & !(1i32 << 21i32) as libc::c_uint)
            .wrapping_sub(1i32 as libc::c_uint) as i32;
          if hi > lo {
            mainQSort3(state, lo, hi);
            if (*state).budget < 0i32 {
              return;
            }
          }
          /*& CLEARMASK (redundant)*/
        }
        let ref mut fresh5 = *ftab.offset(sb as isize);
        *fresh5 |= (1i32 << 21i32) as libc::c_uint
      }
      j += 1
    }
    /*
     * Step 2:
     * Now scan this big bucket [ss] so as to synthesise the
     * sorted order for small buckets [t, ss] for all t,
     * including, magically, the bucket [ss,ss] too.
     * This will avoid doing Real Work in subsequent Step 1's.
     */
    j = 0i32;
    while j <= 255i32 {
      (*state).mainSort__copyStart[j as usize] = (*ftab
        .offset(((j << 8i32) as libc::c_uint).wrapping_add(ss) as isize)
        & !(1i32 << 21i32) as libc::c_uint) as i32;
      (*state).mainSort__copyEnd[j as usize] = (*ftab.offset(
        ((j << 8i32) as libc::c_uint)
          .wrapping_add(ss)
          .wrapping_add(1i32 as libc::c_uint) as isize,
      ) & !(1i32 << 21i32) as libc::c_uint)
        .wrapping_sub(1i32 as libc::c_uint) as i32;
      j += 1
    }
    j = (*ftab.offset((ss << 8i32) as isize) & !(1i32 << 21i32) as libc::c_uint) as i32;
    while j < (*state).mainSort__copyStart[ss as usize] {
      let mut c1: libc::c_uint = 0;
      let mut k: i32 = 0;
      k = (*ptr.offset(j as isize)).wrapping_sub(1i32 as libc::c_uint) as i32;
      if k < 0i32 {
        k += nblock
      }
      c1 = *block.offset(k as isize) as libc::c_uint;
      if bigDone[c1 as usize] == 0 {
        let fresh6 = (*state).mainSort__copyStart[c1 as usize];
        (*state).mainSort__copyStart[c1 as usize] = (*state).mainSort__copyStart[c1 as usize] + 1;
        *ptr.offset(fresh6 as isize) = k as u32
      }
      j += 1
    }
    j = (*ftab.offset((ss.wrapping_add(1i32 as libc::c_uint) << 8i32) as isize)
      & !(1i32 << 21i32) as libc::c_uint)
      .wrapping_sub(1i32 as libc::c_uint) as i32;
    while j > (*state).mainSort__copyEnd[ss as usize] {
      let mut c1_0: libc::c_uint = 0;
      let mut k_0: i32 = 0;
      k_0 = (*ptr.offset(j as isize)).wrapping_sub(1i32 as libc::c_uint) as i32;
      if k_0 < 0i32 {
        k_0 += nblock
      }
      c1_0 = *block.offset(k_0 as isize) as libc::c_uint;
      if bigDone[c1_0 as usize] == 0 {
        let fresh7 = (*state).mainSort__copyEnd[c1_0 as usize];
        (*state).mainSort__copyEnd[c1_0 as usize] = (*state).mainSort__copyEnd[c1_0 as usize] - 1;
        *ptr.offset(fresh7 as isize) = k_0 as u32
      }
      j -= 1
    }
    /* Extremely rare case missing in bzip2-1.0.0 and 1.0.1.
     * Necessity for this case is demonstrated by compressing
     * a sequence of approximately 48.5 million of character
     * 251; 1.0.0/1.0.1 will then die here. */
    j = 0i32;
    while j <= 255i32 {
      let ref mut fresh8 = *ftab.offset(((j << 8i32) as libc::c_uint).wrapping_add(ss) as isize);
      *fresh8 |= (1i32 << 21i32) as libc::c_uint;
      j += 1
    }
    if i == 255i32 {
      break;
    }
    /*
     * Step 3:
     * The [ss] big bucket is now done.  Record this fact,
     * and update the quadrant descriptors.  Remember to
     * update quadrants in the overshoot area too, if
     * necessary.  The "if (i < 255)" test merely skips
     * this updating for the last bucket processed, since
     * updating for the last bucket is pointless.
     *
     * The quadrant array provides a way to incrementally
     * cache sort orderings, as they appear, so as to
     * make subsequent comparisons in fullGtU() complete
     * faster.  For repetitive blocks this makes a big
     * difference (but not big enough to be able to avoid
     * the fallback sorting mechanism, exponential radix sort).
     *
     * The precise meaning is: at all times:
     *
     *	for 0 <= i < nblock and 0 <= j <= nblock
     *
     *	if block[i] != block[j],
     *
     *		then the relative values of quadrant[i] and
     *			  quadrant[j] are meaningless.
     *
     *		else {
     *			if quadrant[i] < quadrant[j]
     *				then the string starting at i lexicographically
     *				precedes the string starting at j
     *
     *			else if quadrant[i] > quadrant[j]
     *				then the string starting at j lexicographically
     *				precedes the string starting at i
     *
     *			else
     *				the relative ordering of the strings starting
     *				at i and j has not yet been determined.
     *		}
     */
    bigDone[ss as usize] = 1i32 as Bool; /* uint32[] */
    let mut bbStart: libc::c_uint =
      *ftab.offset((ss << 8i32) as isize) & !(1i32 << 21i32) as libc::c_uint;
    let mut bbSize: libc::c_uint = (*ftab
      .offset((ss.wrapping_add(1i32 as libc::c_uint) << 8i32) as isize)
      & !(1i32 << 21i32) as libc::c_uint)
      .wrapping_sub(bbStart);
    let mut shifts: libc::c_uint = 0i32 as libc::c_uint;
    while bbSize >> shifts > 65534i32 as libc::c_uint {
      shifts = shifts.wrapping_add(1)
    }
    j = bbSize.wrapping_sub(1i32 as libc::c_uint) as i32;
    while j >= 0i32 {
      let mut a2update: libc::c_uint =
        *ptr.offset(bbStart.wrapping_add(j as libc::c_uint) as isize);
      let mut qVal: u16 = (j >> shifts) as u16;
      *quadrant.offset(a2update as isize) = qVal;
      if a2update < (2i32 + 12i32 + 18i32 + 2i32) as libc::c_uint {
        *quadrant.offset(a2update.wrapping_add(nblock as libc::c_uint) as isize) = qVal
      }
      j -= 1
    }
    i += 1
  }
}
/*-- compression. --*/
/*---------------------------------------------*/
/* Pre:
 *	nblock > 0
 *	arr2 exists for [0 .. nblock-1 +N_OVERSHOOT]
 *	  ((u8*)arr2)[0 .. nblock-1] holds block
 *	arr1 exists for [0 .. nblock-1]
 *
 * Post:
 *	((u8*)arr2) [0 .. nblock-1] holds block
 *	All other areas of block destroyed
 *	ftab[0 .. 65536] destroyed
 *	arr1[0 .. nblock-1] holds sorted order
 */
#[inline(never)]
unsafe extern "C" fn BZ2_blockSort(mut state: *mut EState) -> i32 {
  let mut current_block: u64;
  /* In original bzip2 1.0.4, it's a parameter, but 30
   * (which was the default) should work ok. */
  let mut i: libc::c_uint = 0;
  let mut origPtr: i32 = 0;
  origPtr = origPtr;
  if (*state).nblock >= 10000i32 {
    /* Calculate the location for quadrant, remembering to get
     * the alignment right.  Assumes that &(block[0]) is at least
     * 2-byte aligned -- this should be ok since block is really
     * the first section of arr2.
     */
    i = ((*state).nblock + (2i32 + 12i32 + 18i32 + 2i32)) as libc::c_uint;
    if i & 1i32 as libc::c_uint != 0 {
      i = i.wrapping_add(1)
    }
    (*state).quadrant = &mut *(*state).block.offset(i as isize) as *mut u8 as *mut u16;
    /* (wfact-1) / 3 puts the default-factor-30
     * transition point at very roughly the same place as
     * with v0.1 and v0.9.0.
     * Not that it particularly matters any more, since the
     * resulting compressed stream is now the same regardless
     * of whether or not we use the main sort or fallback sort.
     */
    (*state).budget = (*state).nblock * ((wfact as libc::c_int - 1i32) / 3i32);
    mainSort(state);
    if (*state).budget >= 0i32 {
      current_block = 14622891544831191739;
    } else {
      current_block = 13183875560443969876;
    }
  } else {
    current_block = 13183875560443969876;
  }
  match current_block {
    13183875560443969876 => {
      fallbackSort(state);
    }
    _ => {}
  }
  i = 0i32 as libc::c_uint;
  while i < (*state).nblock as libc::c_uint {
    if *(*state).ptr.offset(i as isize) == 0i32 as libc::c_uint {
      origPtr = i as i32;
      break;
    } else {
      i = i.wrapping_add(1)
    }
  }
  return origPtr;
}
/*
 * bzip2 is written by Julian Seward <jseward@bzip.org>.
 * Adapted for busybox by Denys Vlasenko <vda.linux@googlemail.com>.
 * See README and LICENSE files in this directory for more information.
 */
/*-------------------------------------------------------------*/
/*--- Library top-level functions.                          ---*/
/*---                                               bzlib.c ---*/
/*-------------------------------------------------------------*/
/* ------------------------------------------------------------------
This file is part of bzip2/libbzip2, a program and library for
lossless, block-sorting data compression.

bzip2/libbzip2 version 1.0.4 of 20 December 2006
Copyright (C) 1996-2006 Julian Seward <jseward@bzip.org>

Please read the WARNING, DISCLAIMER and PATENTS sections in the
README file.

This program is released under the terms of the license contained
in the file LICENSE.
------------------------------------------------------------------ */
/* CHANGES
 * 0.9.0    -- original version.
 * 0.9.0a/b -- no changes in this file.
 * 0.9.0c   -- made zero-length BZ_FLUSH work correctly in bzCompress().
 *             fixed bzWrite/bzRead to ignore zero-length requests.
 *             fixed bzread to correctly handle read requests after EOF.
 *             wrong parameter order in call to bzDecompressInit in
 *             bzBuffToBuffDecompress.  Fixed.
 */
/* #include "bzlib_private.h" */
/*---------------------------------------------------*/
/*--- Compression stuff                           ---*/
/*---------------------------------------------------*/
/*---------------------------------------------------*/
/*---------------------------------------------------*/
unsafe extern "C" fn prepare_new_block(mut s: *mut EState) {
  let mut i: libc::c_int = 0;
  (*s).nblock = 0i32;
  //indexes into s->zbits[], initialzation moved to init of s->zbits
  //s->posZ = s->zbits; // was: s->numZ = 0;
  //s->state_out_pos = s->zbits;
  (*s).blockCRC = 0xffffffffi64 as u32;
  /* inlined memset would be nice to have here */
  i = 0i32;
  while i < 256i32 {
    (*s).inUse[i as usize] = 0i32 as Bool;
    i += 1
  }
  (*s).blockNo += 1;
}
/*---------------------------------------------------*/
#[inline(always)]
unsafe extern "C" fn init_RL(mut s: *mut EState) {
  (*s).state_in_ch = 256i32 as u32;
  (*s).state_in_len = 0i32;
}
unsafe extern "C" fn isempty_RL(mut s: *mut EState) -> libc::c_int {
  return ((*s).state_in_ch >= 256i32 as libc::c_uint || (*s).state_in_len <= 0i32) as libc::c_int;
}
/*-- Core (low-level) library functions --*/
/*---------------------------------------------------*/
unsafe extern "C" fn BZ2_bzCompressInit(mut strm: *mut bz_stream, mut blockSize100k: libc::c_int) {
  let mut n: libc::c_uint = 0;
  let mut s: *mut EState = 0 as *mut EState;
  s = xzalloc(::std::mem::size_of::<EState>() as libc::c_ulong) as *mut EState;
  (*s).strm = strm;
  n = (100000i32 * blockSize100k) as libc::c_uint;
  (*s).arr1 =
    xmalloc((n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<u32>() as libc::c_ulong))
      as *mut u32;
  (*s).mtfv = (*s).arr1 as *mut u16;
  (*s).ptr = (*s).arr1;
  (*s).arr2 = xmalloc(
    (n.wrapping_add((2i32 + 12i32 + 18i32 + 2i32) as libc::c_uint) as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<u32>() as libc::c_ulong),
  ) as *mut u32;
  (*s).block = (*s).arr2 as *mut u8;
  (*s).ftab = xmalloc(
    (65537i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<u32>() as libc::c_ulong),
  ) as *mut u32;
  (*s).crc32table = crc32_filltable(0 as *mut u32, 1i32);
  (*s).state = 2i32 as u8;
  (*s).mode = 2i32 as u8;
  (*s).blockSize100k = blockSize100k as u8;
  (*s).nblockMAX = n.wrapping_sub(19i32 as libc::c_uint) as i32;
  (*strm).state = s as *mut libc::c_void;
  /*strm->total_in     = 0;*/
  (*strm).total_out = 0i32 as libc::c_ulonglong;
  init_RL(s);
  prepare_new_block(s);
}
/*---------------------------------------------------*/
unsafe extern "C" fn add_pair_to_block(mut s: *mut EState) {
  let mut i: i32 = 0;
  let mut ch: u8 = (*s).state_in_ch as u8;
  i = 0i32;
  while i < (*s).state_in_len {
    (*s).blockCRC = (*s).blockCRC << 8i32
      ^ *(*s)
        .crc32table
        .offset(((*s).blockCRC >> 24i32 ^ ch as libc::c_uint) as isize);
    i += 1
  }
  (*s).inUse[(*s).state_in_ch as usize] = 1i32 as Bool;
  let mut current_block_23: u64;
  match (*s).state_in_len {
    3 => {
      *(*s).block.offset((*s).nblock as isize) = ch;
      (*s).nblock += 1;
      current_block_23 = 4769713867740228885;
    }
    2 => {
      current_block_23 = 4769713867740228885;
    }
    1 => {
      current_block_23 = 3930333246692246242;
    }
    _ => {
      (*s).inUse[((*s).state_in_len - 4i32) as usize] = 1i32 as Bool;
      *(*s).block.offset((*s).nblock as isize) = ch;
      (*s).nblock += 1;
      *(*s).block.offset((*s).nblock as isize) = ch;
      (*s).nblock += 1;
      *(*s).block.offset((*s).nblock as isize) = ch;
      (*s).nblock += 1;
      *(*s).block.offset((*s).nblock as isize) = ch;
      (*s).nblock += 1;
      *(*s).block.offset((*s).nblock as isize) = ((*s).state_in_len - 4i32) as u8;
      (*s).nblock += 1;
      current_block_23 = 2719512138335094285;
    }
  }
  match current_block_23 {
    4769713867740228885 =>
    /* fall through */
    {
      *(*s).block.offset((*s).nblock as isize) = ch;
      (*s).nblock += 1;
      current_block_23 = 3930333246692246242;
    }
    _ => {}
  }
  match current_block_23 {
    3930333246692246242 =>
    /* fall through */
    {
      *(*s).block.offset((*s).nblock as isize) = ch;
      (*s).nblock += 1
    }
    _ => {}
  };
}
/*---------------------------------------------------*/
unsafe extern "C" fn flush_RL(mut s: *mut EState) {
  if (*s).state_in_ch < 256i32 as libc::c_uint {
    add_pair_to_block(s);
  }
  init_RL(s);
}
/*---------------------------------------------------*/
/*-- fast track the common case --*/
/*-- general, uncommon cases --*/
/*---------------------------------------------------*/
unsafe extern "C" fn copy_input_until_stop(mut s: *mut EState) {
  /*Bool progress_in = False;*/
  /*-- general, uncommon case --*/
  /*-- no input? --*/
  while !((*(*s).strm).avail_in == 0i32 as libc::c_uint) {
    /*-- block full? --*/
    if (*s).nblock >= (*s).nblockMAX {
      break;
    }
    //#	/*-- flush/finish end? --*/
    //#	if (s->avail_in_expect == 0) break;
    /*progress_in = True;*/
    let mut zchh: u32 = *((*(*s).strm).next_in as *mut u8) as u32;
    if zchh != (*s).state_in_ch && (*s).state_in_len == 1i32 {
      let mut ch: u8 = (*s).state_in_ch as u8;
      (*s).blockCRC = (*s).blockCRC << 8i32
        ^ *(*s)
          .crc32table
          .offset(((*s).blockCRC >> 24i32 ^ ch as libc::c_uint) as isize);
      (*s).inUse[(*s).state_in_ch as usize] = 1i32 as Bool;
      *(*s).block.offset((*s).nblock as isize) = ch;
      (*s).nblock += 1;
      (*s).state_in_ch = zchh
    } else if zchh != (*s).state_in_ch || (*s).state_in_len == 255i32 {
      if (*s).state_in_ch < 256i32 as libc::c_uint {
        add_pair_to_block(s);
      }
      (*s).state_in_ch = zchh;
      (*s).state_in_len = 1i32
    } else {
      (*s).state_in_len += 1
    }
    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
    (*(*s).strm).avail_in = (*(*s).strm).avail_in.wrapping_sub(1)
  }
  /*return progress_in;*/
}
/*---------------------------------------------------*/
unsafe extern "C" fn copy_output_until_stop(mut s: *mut EState) {
  /*Bool progress_out = False;*/
  /*-- no output space? --*/
  while !((*(*s).strm).avail_out == 0i32 as libc::c_uint) {
    /*-- block done? --*/
    if (*s).state_out_pos >= (*s).posZ {
      break;
    }
    /*progress_out = True;*/
    let fresh9 = (*s).state_out_pos;
    (*s).state_out_pos = (*s).state_out_pos.offset(1);
    *(*(*s).strm).next_out = *fresh9 as libc::c_char;
    (*(*s).strm).avail_out = (*(*s).strm).avail_out.wrapping_sub(1);
    (*(*s).strm).next_out = (*(*s).strm).next_out.offset(1);
    (*(*s).strm).total_out = (*(*s).strm).total_out.wrapping_add(1)
  }
  /*return progress_out;*/
}
/*---------------------------------------------------*/
unsafe extern "C" fn handle_compress(mut strm: *mut bz_stream) {
  /*Bool progress_in  = False;*/
  /*Bool progress_out = False;*/
  let mut s: *mut EState = (*strm).state as *mut EState;
  loop {
    if (*s).state as libc::c_int == 1i32 {
      /*progress_out |=*/
      copy_output_until_stop(s);
      if (*s).state_out_pos < (*s).posZ {
        break;
      }
      if (*s).mode as libc::c_int == 4i32
        && (*(*s).strm).avail_in == 0i32 as libc::c_uint
        && isempty_RL(s) != 0
      {
        break;
      }
      prepare_new_block(s);
      (*s).state = 2i32 as u8
    }
    if !((*s).state as libc::c_int == 2i32) {
      continue;
    }
    /*progress_in |=*/
    copy_input_until_stop(s);
    //#if (s->mode != BZ_M_RUNNING && s->avail_in_expect == 0) {
    if (*s).mode as libc::c_int != 2i32 && (*(*s).strm).avail_in == 0i32 as libc::c_uint {
      flush_RL(s);
      BZ2_compressBlock(s, ((*s).mode as libc::c_int == 4i32) as libc::c_int);
      (*s).state = 1i32 as u8
    } else if (*s).nblock >= (*s).nblockMAX {
      BZ2_compressBlock(s, 0i32);
      (*s).state = 1i32 as u8
    } else if (*(*s).strm).avail_in == 0i32 as libc::c_uint {
      break;
    }
  }
  /*return progress_in || progress_out;*/
}
/*---------------------------------------------------*/
unsafe extern "C" fn BZ2_bzCompress(
  mut strm: *mut bz_stream,
  mut action: libc::c_int,
) -> libc::c_int {
  /*Bool progress;*/
  let mut s: *mut EState = 0 as *mut EState;
  s = (*strm).state as *mut EState;
  match (*s).mode as libc::c_int {
    2 => {
      if action == 0i32 {
        /*progress =*/
        handle_compress(strm);
        /*return progress ? BZ_RUN_OK : BZ_PARAM_ERROR;*/
        return 1i32;
      } else {
        /*if (action == BZ_FINISH)*/
        //#s->avail_in_expect = strm->avail_in;
        (*s).mode = 4i32 as u8
      }
    }
    _ => {}
  }
  /*case BZ_M_FINISHING:*/
  /*if (s->avail_in_expect != s->strm->avail_in)
  return BZ_SEQUENCE_ERROR;*/
  /*progress =*/
  handle_compress(strm);
  if (*(*s).strm).avail_in > 0i32 as libc::c_uint
    || isempty_RL(s) == 0
    || (*s).state_out_pos < (*s).posZ
  {
    return 3i32;
  }
  return 4i32;
  /*if (!progress) return BZ_SEQUENCE_ERROR;*/
  //#if (s->avail_in_expect > 0 || !isempty_RL(s) || s->state_out_pos < s->posZ)
  //#	return BZ_FINISH_OK;
  /*s->mode = BZ_M_IDLE;*/
  /* return BZ_OK; --not reached--*/
}
/*---------------------------------------------------*/
unsafe extern "C" fn BZ2_bzCompressEnd(mut strm: *mut bz_stream) {
  let mut s: *mut EState = 0 as *mut EState;
  s = (*strm).state as *mut EState;
  free((*s).arr1 as *mut libc::c_void);
  free((*s).arr2 as *mut libc::c_void);
  free((*s).ftab as *mut libc::c_void);
  free((*s).crc32table as *mut libc::c_void);
  free(s as *mut libc::c_void);
}
/*
 * bzip2 is written by Julian Seward <jseward@bzip.org>.
 * Adapted for busybox by Denys Vlasenko <vda.linux@googlemail.com>.
 * See README and LICENSE files in this directory for more information.
 */
/*-------------------------------------------------------------*/
/*--- Compression machinery (not incl block sorting)        ---*/
/*---                                            compress.c ---*/
/*-------------------------------------------------------------*/
/* ------------------------------------------------------------------
This file is part of bzip2/libbzip2, a program and library for
lossless, block-sorting data compression.

bzip2/libbzip2 version 1.0.4 of 20 December 2006
Copyright (C) 1996-2006 Julian Seward <jseward@bzip.org>

Please read the WARNING, DISCLAIMER and PATENTS sections in the
README file.

This program is released under the terms of the license contained
in the file LICENSE.
------------------------------------------------------------------ */
/* CHANGES
 * 0.9.0    -- original version.
 * 0.9.0a/b -- no changes in this file.
 * 0.9.0c   -- changed setting of nGroups in sendMTFValues()
 *             so as to do a bit better on small files
*/
/* #include "bzlib_private.h" */
/*nothing*/
/*---------------------------------------------------*/
/*--- Bit stream I/O                              ---*/
/*---------------------------------------------------*/
/*---------------------------------------------------*/
unsafe extern "C" fn BZ2_bsInitWrite(mut s: *mut EState) {
  (*s).bsLive = 0i32;
  (*s).bsBuff = 0i32 as u32;
}
/*---------------------------------------------------*/
#[inline(never)]
unsafe extern "C" fn bsFinishWrite(mut s: *mut EState) {
  while (*s).bsLive > 0i32 {
    let fresh10 = (*s).posZ;
    (*s).posZ = (*s).posZ.offset(1);
    *fresh10 = ((*s).bsBuff >> 24i32) as u8;
    (*s).bsBuff <<= 8i32;
    (*s).bsLive -= 8i32
  }
}
/*---------------------------------------------------*/
unsafe extern "C" fn bsW(mut s: *mut EState, mut n: i32, mut v: u32) {
  while (*s).bsLive >= 8i32 {
    let fresh11 = (*s).posZ;
    (*s).posZ = (*s).posZ.offset(1);
    *fresh11 = ((*s).bsBuff >> 24i32) as u8;
    (*s).bsBuff <<= 8i32;
    (*s).bsLive -= 8i32
  }
  (*s).bsBuff |= v << 32i32 - (*s).bsLive - n;
  (*s).bsLive += n;
}
/* Same with n == 16: */
unsafe extern "C" fn bsW16(mut s: *mut EState, mut v: u32) {
  while (*s).bsLive >= 8i32 {
    let fresh12 = (*s).posZ;
    (*s).posZ = (*s).posZ.offset(1);
    *fresh12 = ((*s).bsBuff >> 24i32) as u8;
    (*s).bsBuff <<= 8i32;
    (*s).bsLive -= 8i32
  }
  (*s).bsBuff |= v << 16i32 - (*s).bsLive;
  (*s).bsLive += 16i32;
}
/* Same with n == 1: */
#[inline(always)]
unsafe extern "C" fn bsW1_1(mut s: *mut EState) {
  /* need space for only 1 bit, no need for loop freeing > 8 bits */
  if (*s).bsLive >= 8i32 {
    let fresh13 = (*s).posZ;
    (*s).posZ = (*s).posZ.offset(1);
    *fresh13 = ((*s).bsBuff >> 24i32) as u8;
    (*s).bsBuff <<= 8i32;
    (*s).bsLive -= 8i32
  }
  (*s).bsBuff |= (1i32 << 31i32 - (*s).bsLive) as libc::c_uint;
  (*s).bsLive += 1i32;
}
unsafe extern "C" fn bsW1_0(mut s: *mut EState) {
  /* need space for only 1 bit, no need for loop freeing > 8 bits */
  if (*s).bsLive >= 8i32 {
    let fresh14 = (*s).posZ;
    (*s).posZ = (*s).posZ.offset(1);
    *fresh14 = ((*s).bsBuff >> 24i32) as u8;
    (*s).bsBuff <<= 8i32;
    (*s).bsLive -= 8i32
  }
  //s->bsBuff |= (0 << (31 - s->bsLive));
  (*s).bsLive += 1i32;
}
/*---------------------------------------------------*/
#[inline(always)]
unsafe extern "C" fn bsPutU16(mut s: *mut EState, mut u: libc::c_uint) {
  bsW16(s, u);
}
/*---------------------------------------------------*/
unsafe extern "C" fn bsPutU32(mut s: *mut EState, mut u: libc::c_uint) {
  //bsW(s, 32, u); // can't use: may try "uint32 << -n"
  bsW16(s, u >> 16i32 & 0xffffi32 as libc::c_uint);
  bsW16(s, u & 0xffffi32 as libc::c_uint);
}
/*---------------------------------------------------*/
/*--- The back end proper                         ---*/
/*---------------------------------------------------*/
/*---------------------------------------------------*/
unsafe extern "C" fn makeMaps_e(mut s: *mut EState) {
  let mut i: libc::c_int = 0;
  let mut cnt: libc::c_uint = 0i32 as libc::c_uint;
  i = 0i32;
  while i < 256i32 {
    if (*s).inUse[i as usize] != 0 {
      (*s).unseqToSeq[i as usize] = cnt as u8;
      cnt = cnt.wrapping_add(1)
    }
    i += 1
  }
  (*s).nInUse = cnt as i32;
}
/*---------------------------------------------------*/
/*
 * This bit of code is performance-critical.
 * On 32bit x86, gcc-6.3.0 was observed to spill ryy_j to stack,
 * resulting in abysmal performance (x3 slowdown).
 * Forcing it into a separate function alleviates register pressure,
 * and spillage no longer happens.
 * Other versions of gcc do not exhibit this problem, but out-of-line code
 * seems to be helping them too (code is both smaller and faster).
 * Therefore NOINLINE is enabled for the entire 32bit x86 arch for now,
 * without a check for gcc version.
 */
unsafe extern "C" fn inner_loop(mut yy: *mut u8, mut ll_i: u8) -> libc::c_int {
  let mut rtmp: u8 = 0;
  let mut ryy_j: *mut u8 = 0 as *mut u8;
  rtmp = *yy.offset(1);
  *yy.offset(1) = *yy.offset(0);
  ryy_j = &mut *yy.offset(1) as *mut u8;
  while ll_i as libc::c_int != rtmp as libc::c_int {
    let mut rtmp2: u8 = 0;
    ryy_j = ryy_j.offset(1);
    rtmp2 = rtmp;
    rtmp = *ryy_j;
    *ryy_j = rtmp2
  }
  *yy.offset(0) = rtmp;
  return ryy_j.wrapping_offset_from(&mut *yy.offset(0) as *mut u8) as libc::c_long as libc::c_int;
}
#[inline(never)]
unsafe extern "C" fn generateMTFValues(mut s: *mut EState) {
  let mut ll_i: u8 = 0;
  let mut j: i32 = 0;
  let mut current_block: u64;
  let mut yy: [u8; 256] = [0; 256];
  let mut i: libc::c_int = 0;
  let mut zPend: libc::c_int = 0;
  let mut wr: i32 = 0;
  /*
   * After sorting (eg, here),
   * s->arr1[0 .. s->nblock-1] holds sorted order,
   * and
   * ((u8*)s->arr2)[0 .. s->nblock-1]
   * holds the original block data.
   *
   * The first thing to do is generate the MTF values,
   * and put them in ((u16*)s->arr1)[0 .. s->nblock-1].
   *
   * Because there are strictly fewer or equal MTF values
   * than block values, ptr values in this area are overwritten
   * with MTF values only when they are no longer needed.
   *
   * The final compressed bitstream is generated into the
   * area starting at &((u8*)s->arr2)[s->nblock]
   *
   * These storage aliases are set up in bzCompressInit(),
   * except for the last one, which is arranged in
   * compressBlock().
   */
  let mut ptr: *mut u32 = (*s).ptr; /* gcc 4.3.1 thinks it may be used w/o init */
  makeMaps_e(s); /* "process it and come back here" */
  wr = 0i32;
  zPend = 0i32;
  i = 0i32;
  while i <= (*s).nInUse + 1i32 {
    (*s).mtfFreq[i as usize] = 0i32;
    i += 1
  }
  i = 0i32;
  while i < (*s).nInUse {
    yy[i as usize] = i as u8;
    i += 1
  }
  i = 0i32;
  loop {
    if i < (*s).nblock {
      ll_i = ll_i;
      j = 0;
      j = (*ptr.offset(i as isize)).wrapping_sub(1i32 as libc::c_uint) as i32;
      if j < 0i32 {
        j += (*s).nblock
      }
      ll_i = (*s).unseqToSeq[*(*s).block.offset(j as isize) as usize];
      if yy[0] as libc::c_int == ll_i as libc::c_int {
        zPend += 1;
        current_block = 5399440093318478209;
      } else if zPend > 0i32 {
        current_block = 3013028992155267829;
      } else {
        current_block = 8845338526596852646;
      }
    } else {
      i = -1i32;
      if !(zPend > 0i32) {
        break;
      }
      current_block = 3013028992155267829;
    }
    match current_block {
      3013028992155267829 => {
        zPend -= 1;
        loop
        /* same as above, since BZ_RUNA is 0 and BZ_RUNB is 1 */
        {
          let mut run: libc::c_uint = (zPend & 1i32) as libc::c_uint;
          *(*s).mtfv.offset(wr as isize) = run as u16;
          wr += 1;
          (*s).mtfFreq[run as usize] += 1;
          zPend -= 2i32;
          if zPend < 0i32 {
            break;
          }
          zPend = (zPend as libc::c_uint).wrapping_div(2i32 as libc::c_uint) as libc::c_int
        }
        if i < 0i32 {
          break;
        }
        zPend = 0i32;
        current_block = 8845338526596852646;
      }
      _ => {}
    }
    match current_block {
      8845338526596852646 => {
        j = inner_loop(yy.as_mut_ptr(), ll_i);
        *(*s).mtfv.offset(wr as isize) = (j + 1i32) as u16;
        wr += 1;
        (*s).mtfFreq[(j + 1i32) as usize] += 1
      }
      _ => {}
    }
    i += 1
  }
  /* came via "goto process_zPend"? exit */
  *(*s).mtfv.offset(wr as isize) = ((*s).nInUse + 1i32) as u16;
  wr += 1;
  (*s).mtfFreq[((*s).nInUse + 1i32) as usize] += 1;
  (*s).nMTF = wr;
}
#[inline(never)]
unsafe extern "C" fn sendMTFValues(mut s: *mut EState) {
  let mut t: i32 = 0;
  let mut i: i32 = 0;
  let mut iter: libc::c_uint = 0;
  let mut gs: libc::c_uint = 0;
  let mut alphaSize: i32 = 0;
  let mut nSelectors: libc::c_uint = 0;
  let mut selCtr: libc::c_uint = 0;
  let mut nGroups: i32 = 0;
  /*
   * u8 len[BZ_N_GROUPS][BZ_MAX_ALPHA_SIZE];
   * is a global since the decoder also needs it.
   *
   * i32  code[BZ_N_GROUPS][BZ_MAX_ALPHA_SIZE];
   * i32  rfreq[BZ_N_GROUPS][BZ_MAX_ALPHA_SIZE];
   * are also globals only used in this proc.
   * Made global to keep stack frame size small.
   */
  let mut cost: [libc::c_uint; 6] = [0; 6];
  let mut mtfv: *mut u16 = (*s).mtfv;
  alphaSize = (*s).nInUse + 2i32;
  t = 0i32;
  while t < 6i32 {
    let mut v: libc::c_uint = 0;
    v = 0i32 as libc::c_uint;
    while v < alphaSize as libc::c_uint {
      (*s).len[t as usize][v as usize] = 15i32 as u8;
      v = v.wrapping_add(1)
    }
    t += 1
  }
  /*--- Decide how many coding tables to use ---*/
  // 1..199 = 2
  // 200..599 = 3
  // 600..1199 = 4
  // 1200..2399 = 5
  // 2400..99999 = 6
  nGroups = 2i32;
  nGroups += ((*s).nMTF >= 200i32) as libc::c_int;
  nGroups += ((*s).nMTF >= 600i32) as libc::c_int;
  nGroups += ((*s).nMTF >= 1200i32) as libc::c_int;
  nGroups += ((*s).nMTF >= 2400i32) as libc::c_int;
  /*--- Generate an initial set of coding tables ---*/
  let mut nPart: libc::c_uint = 0;
  let mut remF: libc::c_uint = 0;
  nPart = nGroups as libc::c_uint;
  remF = (*s).nMTF as libc::c_uint;
  gs = 0i32 as libc::c_uint;
  while nPart > 0i32 as libc::c_uint {
    let mut v_0: libc::c_uint = 0;
    let mut ge: libc::c_uint = 0;
    let mut tFreq: libc::c_uint = 0;
    let mut aFreq: libc::c_uint = 0;
    tFreq = remF.wrapping_div(nPart);
    ge = gs;
    aFreq = 0i32 as libc::c_uint;
    while aFreq < tFreq && ge < alphaSize as libc::c_uint {
      let fresh15 = ge;
      ge = ge.wrapping_add(1);
      aFreq = aFreq.wrapping_add((*s).mtfFreq[fresh15 as usize] as libc::c_uint)
    }
    ge = ge.wrapping_sub(1);
    if ge > gs
      && nPart != nGroups as libc::c_uint
      && nPart != 1i32 as libc::c_uint
      && (nGroups as libc::c_uint)
        .wrapping_sub(nPart)
        .wrapping_rem(2i32 as libc::c_uint)
        == 1i32 as libc::c_uint
    {
      /* bbox: can this be replaced by x & 1? */
      aFreq = aFreq.wrapping_sub((*s).mtfFreq[ge as usize] as libc::c_uint);
      ge = ge.wrapping_sub(1)
    }
    v_0 = 0i32 as libc::c_uint;
    while v_0 < alphaSize as libc::c_uint {
      if v_0 >= gs && v_0 <= ge {
        (*s).len[nPart.wrapping_sub(1i32 as libc::c_uint) as usize][v_0 as usize] = 0i32 as u8
      } else {
        (*s).len[nPart.wrapping_sub(1i32 as libc::c_uint) as usize][v_0 as usize] = 15i32 as u8
      }
      v_0 = v_0.wrapping_add(1)
    }
    nPart = nPart.wrapping_sub(1);
    gs = ge.wrapping_add(1i32 as libc::c_uint);
    remF = remF.wrapping_sub(aFreq)
  }
  /*
   * Iterate up to BZ_N_ITERS times to improve the tables.
   */
  iter = 0i32 as libc::c_uint;
  while iter < 4i32 as libc::c_uint {
    t = 0i32;
    while t < nGroups {
      let mut v_1: libc::c_uint = 0;
      v_1 = 0i32 as libc::c_uint;
      while v_1 < alphaSize as libc::c_uint {
        (*s).sendMTFValues__rfreq[t as usize][v_1 as usize] = 0i32;
        v_1 = v_1.wrapping_add(1)
      }
      t += 1
    }
    nSelectors = 0i32 as libc::c_uint;
    gs = 0i32 as libc::c_uint;
    loop {
      let mut ge_0: libc::c_uint = 0;
      let mut bt: libc::c_uint = 0;
      let mut bc: libc::c_uint = 0;
      /*--- Set group start & end marks. --*/
      if gs >= (*s).nMTF as libc::c_uint {
        break;
      }
      ge_0 = gs
        .wrapping_add(50i32 as libc::c_uint)
        .wrapping_sub(1i32 as libc::c_uint);
      if ge_0 >= (*s).nMTF as libc::c_uint {
        ge_0 = ((*s).nMTF - 1i32) as libc::c_uint
      }
      /*
       * Calculate the cost of this group as coded
       * by each of the coding tables.
       */
      t = 0i32;
      while t < nGroups {
        cost[t as usize] = 0i32 as libc::c_uint;
        t += 1
      }
      /*--- slow version which correctly handles all situations ---*/
      i = gs as i32;
      while i as libc::c_uint <= ge_0 {
        let mut icv: libc::c_uint = *mtfv.offset(i as isize) as libc::c_uint;
        t = 0i32;
        while t < nGroups {
          cost[t as usize] =
            cost[t as usize].wrapping_add((*s).len[t as usize][icv as usize] as libc::c_uint);
          t += 1
        }
        i += 1
      }
      /*
       * Find the coding table which is best for this group,
       * and record its identity in the selector table.
       */
      /*bc = 999999999;*/
      /*bt = -1;*/
      bc = cost[0];
      bt = 0i32 as libc::c_uint;
      t = 1i32;
      while t < nGroups {
        if cost[t as usize] < bc {
          bc = cost[t as usize];
          bt = t as libc::c_uint
        }
        t += 1
      }
      (*s).selector[nSelectors as usize] = bt as u8;
      nSelectors = nSelectors.wrapping_add(1);
      /*
       * Increment the symbol frequencies for the selected table.
       */
      /* 1% faster compress. +800 bytes */
      /*--- slow version which correctly handles all situations ---*/
      while gs <= ge_0 {
        (*s).sendMTFValues__rfreq[bt as usize][*mtfv.offset(gs as isize) as usize] += 1;
        gs = gs.wrapping_add(1)
      }
    }
    /*
     * Recompute the tables based on the accumulated frequencies.
     */
    /* maxLen was changed from 20 to 17 in bzip2-1.0.3.  See
     * comment in huffman.c for details. */
    t = 0i32;
    while t < nGroups {
      BZ2_hbMakeCodeLengths(
        s,
        &mut *(*(*s).len.as_mut_ptr().offset(t as isize))
          .as_mut_ptr()
          .offset(0),
        &mut *(*(*s).sendMTFValues__rfreq.as_mut_ptr().offset(t as isize))
          .as_mut_ptr()
          .offset(0),
        alphaSize,
        17i32,
      );
      t += 1
    }
    iter = iter.wrapping_add(1)
  }
  /*--- Compute MTF values for the selectors. ---*/
  let mut pos: [u8; 6] = [0; 6];
  let mut ll_i: u8 = 0;
  let mut tmp2: u8 = 0;
  let mut tmp: u8 = 0;
  i = 0i32;
  while i < nGroups {
    pos[i as usize] = i as u8;
    i += 1
  }
  i = 0i32;
  while (i as libc::c_uint) < nSelectors {
    let mut j: libc::c_uint = 0;
    ll_i = (*s).selector[i as usize];
    j = 0i32 as libc::c_uint;
    tmp = pos[j as usize];
    while ll_i as libc::c_int != tmp as libc::c_int {
      j = j.wrapping_add(1);
      tmp2 = tmp;
      tmp = pos[j as usize];
      pos[j as usize] = tmp2
    }
    pos[0] = tmp;
    (*s).selectorMtf[i as usize] = j as u8;
    i += 1
  }
  /*--- Assign actual codes for the tables. --*/
  t = 0i32; //todo: s->len[t][0];
  while t < nGroups {
    let mut minLen: libc::c_uint = 32i32 as libc::c_uint; //todo: s->len[t][0];
    let mut maxLen: libc::c_uint = 0i32 as libc::c_uint;
    i = 0i32;
    while i < alphaSize {
      if (*s).len[t as usize][i as usize] as libc::c_uint > maxLen {
        maxLen = (*s).len[t as usize][i as usize] as libc::c_uint
      }
      if ((*s).len[t as usize][i as usize] as libc::c_uint) < minLen {
        minLen = (*s).len[t as usize][i as usize] as libc::c_uint
      }
      i += 1
    }
    /*20*/
    BZ2_hbAssignCodes(
      &mut *(*(*s).sendMTFValues__code.as_mut_ptr().offset(t as isize))
        .as_mut_ptr()
        .offset(0),
      &mut *(*(*s).len.as_mut_ptr().offset(t as isize))
        .as_mut_ptr()
        .offset(0),
      minLen as i32,
      maxLen as i32,
      alphaSize,
    );
    t += 1
  }
  /*--- Transmit the mapping table. ---*/
  /* bbox: optimized a bit more than in bzip2 */
  let mut inUse16: libc::c_int = 0i32; /* Our CPU can do better */
  i = 0i32; /* move 15th bit into sign bit */
  while i < 16i32 {
    if ::std::mem::size_of::<libc::c_long>() as libc::c_ulong <= 4i32 as libc::c_ulong {
      inUse16 = inUse16 * 2i32
        + (*(&mut *(*s).inUse.as_mut_ptr().offset((i * 16i32 + 0i32) as isize) as *mut Bool
          as *mut bb__aliased_u32)
          | *(&mut *(*s).inUse.as_mut_ptr().offset((i * 16i32 + 4i32) as isize) as *mut Bool
            as *mut bb__aliased_u32)
          | *(&mut *(*s).inUse.as_mut_ptr().offset((i * 16i32 + 8i32) as isize) as *mut Bool
            as *mut bb__aliased_u32)
          | *(&mut *(*s).inUse.as_mut_ptr().offset((i * 16i32 + 12i32) as isize) as *mut Bool
            as *mut bb__aliased_u32)
          != 0i32 as libc::c_uint) as libc::c_int
    } else {
      inUse16 = inUse16 * 2i32
        + (*(&mut *(*s).inUse.as_mut_ptr().offset((i * 16i32 + 0i32) as isize) as *mut Bool
          as *mut bb__aliased_u64)
          | *(&mut *(*s).inUse.as_mut_ptr().offset((i * 16i32 + 8i32) as isize) as *mut Bool
            as *mut bb__aliased_u64)
          != 0i32 as libc::c_ulong) as libc::c_int
    }
    i += 1
  }
  bsW16(s, inUse16 as u32);
  inUse16 <<= (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
    .wrapping_mul(8i32 as libc::c_ulong)
    .wrapping_sub(16i32 as libc::c_ulong);
  i = 0i32;
  while i < 16i32 {
    if inUse16 < 0i32 {
      let mut v16: libc::c_uint = 0i32 as libc::c_uint;
      let mut j_0: libc::c_uint = 0;
      j_0 = 0i32 as libc::c_uint;
      while j_0 < 16i32 as libc::c_uint {
        v16 = v16.wrapping_mul(2i32 as libc::c_uint).wrapping_add(
          (*s).inUse[((i * 16i32) as libc::c_uint).wrapping_add(j_0) as usize] as libc::c_uint,
        );
        j_0 = j_0.wrapping_add(1)
      }
      bsW16(s, v16);
    }
    inUse16 <<= 1i32;
    i += 1
  }
  /*--- Now the selectors. ---*/
  bsW(s, 3i32, nGroups as u32);
  bsW(s, 15i32, nSelectors);
  i = 0i32;
  while (i as libc::c_uint) < nSelectors {
    let mut j_1: libc::c_uint = 0;
    j_1 = 0i32 as libc::c_uint;
    while j_1 < (*s).selectorMtf[i as usize] as libc::c_uint {
      bsW1_1(s);
      j_1 = j_1.wrapping_add(1)
    }
    bsW1_0(s);
    i += 1
  }
  /*--- Now the coding tables. ---*/
  t = 0i32;
  while t < nGroups {
    let mut curr: libc::c_uint = (*s).len[t as usize][0] as libc::c_uint;
    bsW(s, 5i32, curr);
    i = 0i32;
    while i < alphaSize {
      while curr < (*s).len[t as usize][i as usize] as libc::c_uint {
        bsW(s, 2i32, 2i32 as u32);
        curr = curr.wrapping_add(1)
        /* 10 */
      }
      while curr > (*s).len[t as usize][i as usize] as libc::c_uint {
        bsW(s, 2i32, 3i32 as u32);
        curr = curr.wrapping_sub(1)
        /* 11 */
      }
      bsW1_0(s);
      i += 1
    }
    t += 1
  }
  /*--- And finally, the block data proper ---*/
  selCtr = 0i32 as libc::c_uint;
  gs = 0i32 as libc::c_uint;
  loop {
    let mut ge_1: libc::c_uint = 0;
    if gs >= (*s).nMTF as libc::c_uint {
      break;
    }
    ge_1 = gs
      .wrapping_add(50i32 as libc::c_uint)
      .wrapping_sub(1i32 as libc::c_uint);
    if ge_1 >= (*s).nMTF as libc::c_uint {
      ge_1 = ((*s).nMTF - 1i32) as libc::c_uint
    }
    /* Costs 1300 bytes and is _slower_ (on Intel Core 2) */
    /*--- slow version which correctly handles all situations ---*/
    /* code is bit bigger, but moves multiply out of the loop */
    let mut s_len_sel_selCtr: *mut u8 = &mut *(*(*s)
      .len
      .as_mut_ptr()
      .offset(*(*s).selector.as_mut_ptr().offset(selCtr as isize) as isize))
    .as_mut_ptr()
    .offset(0) as *mut u8;
    let mut s_code_sel_selCtr: *mut i32 = &mut *(*(*s)
      .sendMTFValues__code
      .as_mut_ptr()
      .offset(*(*s).selector.as_mut_ptr().offset(selCtr as isize) as isize))
    .as_mut_ptr()
    .offset(0) as *mut i32;
    while gs <= ge_1 {
      bsW(
        s,
        *s_len_sel_selCtr.offset(*mtfv.offset(gs as isize) as isize) as i32,
        *s_code_sel_selCtr.offset(*mtfv.offset(gs as isize) as isize) as u32,
      );
      gs = gs.wrapping_add(1)
    }
    selCtr = selCtr.wrapping_add(1)
  }
}
/*---------------------------------------------------*/
unsafe extern "C" fn BZ2_compressBlock(mut s: *mut EState, mut is_last_block: libc::c_int) {
  let mut origPtr: i32 = 0; // was: s->numZ = 0;
  origPtr = origPtr;
  if (*s).nblock > 0i32 {
    (*s).blockCRC = !(*s).blockCRC;
    (*s).combinedCRC = (*s).combinedCRC << 1i32 | (*s).combinedCRC >> 31i32;
    (*s).combinedCRC ^= (*s).blockCRC;
    if (*s).blockNo > 1i32 {
      (*s).posZ = (*s).zbits
    }
    origPtr = BZ2_blockSort(s)
  }
  (*s).zbits = &mut *((*s).arr2 as *mut u8).offset((*s).nblock as isize) as *mut u8;
  (*s).posZ = (*s).zbits;
  (*s).state_out_pos = (*s).zbits;
  /*-- If this is the first block, create the stream header. --*/
  if (*s).blockNo == 1i32 {
    BZ2_bsInitWrite(s);
    /*bsPutU8(s, BZ_HDR_B);*/
    /*bsPutU8(s, BZ_HDR_Z);*/
    /*bsPutU8(s, BZ_HDR_h);*/
    /*bsPutU8(s, BZ_HDR_0 + s->blockSize100k);*/
    bsPutU32(
      s,
      (0x425a6830i32 + (*s).blockSize100k as libc::c_int) as libc::c_uint,
    );
  }
  if (*s).nblock > 0i32 {
    /*bsPutU8(s, 0x31);*/
    /*bsPutU8(s, 0x41);*/
    /*bsPutU8(s, 0x59);*/
    /*bsPutU8(s, 0x26);*/
    bsPutU32(s, 0x31415926i32 as libc::c_uint);
    /*bsPutU8(s, 0x53);*/
    /*bsPutU8(s, 0x59);*/
    bsPutU16(s, 0x5359i32 as libc::c_uint);
    /*-- Now the block's CRC, so it is in a known place. --*/
    bsPutU32(s, (*s).blockCRC);
    /*
     * Now a single bit indicating (non-)randomisation.
     * As of version 0.9.5, we use a better sorting algorithm
     * which makes randomisation unnecessary.  So always set
     * the randomised bit to 'no'.  Of course, the decoder
     * still needs to be able to handle randomised blocks
     * so as to maintain backwards compatibility with
     * older versions of bzip2.
     */
    bsW1_0(s);
    bsW(s, 24i32, origPtr as u32);
    generateMTFValues(s);
    sendMTFValues(s);
  }
  /*-- If this is the last block, add the stream trailer. --*/
  if is_last_block != 0 {
    /*bsPutU8(s, 0x17);*/
    /*bsPutU8(s, 0x72);*/
    /*bsPutU8(s, 0x45);*/
    /*bsPutU8(s, 0x38);*/
    bsPutU32(s, 0x17724538i32 as libc::c_uint);
    /*bsPutU8(s, 0x50);*/
    /*bsPutU8(s, 0x90);*/
    bsPutU16(s, 0x5090i32 as libc::c_uint);
    bsPutU32(s, (*s).combinedCRC);
    bsFinishWrite(s);
  };
}
/*
 * bzip2 is written by Julian Seward <jseward@bzip.org>.
 * Adapted for busybox by Denys Vlasenko <vda.linux@googlemail.com>.
 * See README and LICENSE files in this directory for more information.
 */
/*-------------------------------------------------------------*/
/*--- Huffman coding low-level stuff                        ---*/
/*---                                             huffman.c ---*/
/*-------------------------------------------------------------*/
/* ------------------------------------------------------------------
This file is part of bzip2/libbzip2, a program and library for
lossless, block-sorting data compression.

bzip2/libbzip2 version 1.0.4 of 20 December 2006
Copyright (C) 1996-2006 Julian Seward <jseward@bzip.org>

Please read the WARNING, DISCLAIMER and PATENTS sections in the
README file.

This program is released under the terms of the license contained
in the file LICENSE.
------------------------------------------------------------------ */
/* #include "bzlib_private.h" */
/*---------------------------------------------------*/
/* 90 bytes, 0.3% of overall compress speed */
/* macro works better than inline (gcc 4.2.1) */
/*---------------------------------------------------*/
unsafe extern "C" fn BZ2_hbMakeCodeLengths(
  mut s: *mut EState,
  mut len: *mut u8,
  mut freq: *mut i32,
  mut alphaSize: i32,
  mut maxLen: i32,
) {
  /*
   * Nodes and heap entries run from 1.  Entry 0
   * for both the heap and nodes is a sentinel.
   */
  let mut nNodes: i32 = 0;
  let mut nHeap: i32 = 0;
  let mut n1: i32 = 0;
  let mut n2: i32 = 0;
  let mut i: i32 = 0;
  let mut j: i32 = 0;
  let mut k: i32 = 0;
  let mut tooLong: Bool = 0;
  /* bbox: moved to EState to save stack
  i32 heap  [BZ_MAX_ALPHA_SIZE + 2];
  i32 weight[BZ_MAX_ALPHA_SIZE * 2];
  i32 parent[BZ_MAX_ALPHA_SIZE * 2];
  */
  i = 0i32;
  while i < alphaSize {
    (*s).BZ2_hbMakeCodeLengths__weight[(i + 1i32) as usize] = (if *freq.offset(i as isize) == 0i32 {
      1i32
    } else {
      *freq.offset(i as isize)
    }) << 8i32;
    i += 1
  }
  loop {
    nNodes = alphaSize;
    nHeap = 0i32;
    (*s).BZ2_hbMakeCodeLengths__heap[0] = 0i32;
    (*s).BZ2_hbMakeCodeLengths__weight[0] = 0i32;
    (*s).BZ2_hbMakeCodeLengths__parent[0] = -2i32;
    i = 1i32;
    while i <= alphaSize {
      (*s).BZ2_hbMakeCodeLengths__parent[i as usize] = -1i32;
      nHeap += 1;
      (*s).BZ2_hbMakeCodeLengths__heap[nHeap as usize] = i;
      let mut zz: i32 = 0;
      let mut tmp: i32 = 0;
      zz = nHeap;
      tmp = (*s).BZ2_hbMakeCodeLengths__heap[zz as usize];
      while (*s).BZ2_hbMakeCodeLengths__weight[tmp as usize]
        < (*s).BZ2_hbMakeCodeLengths__weight
          [(*s).BZ2_hbMakeCodeLengths__heap[(zz >> 1i32) as usize] as usize]
      {
        (*s).BZ2_hbMakeCodeLengths__heap[zz as usize] =
          (*s).BZ2_hbMakeCodeLengths__heap[(zz >> 1i32) as usize];
        zz >>= 1i32
      }
      (*s).BZ2_hbMakeCodeLengths__heap[zz as usize] = tmp;
      i += 1
    }
    while nHeap > 1i32 {
      n1 = (*s).BZ2_hbMakeCodeLengths__heap[1];
      (*s).BZ2_hbMakeCodeLengths__heap[1] = (*s).BZ2_hbMakeCodeLengths__heap[nHeap as usize];
      nHeap -= 1;
      let mut zz_0: i32 = 0;
      let mut yy: i32 = 0;
      let mut tmp_0: i32 = 0;
      zz_0 = 1i32;
      tmp_0 = (*s).BZ2_hbMakeCodeLengths__heap[zz_0 as usize];
      loop {
        yy = zz_0 << 1i32;
        if yy > nHeap {
          break;
        }
        if yy < nHeap
          && (*s).BZ2_hbMakeCodeLengths__weight
            [(*s).BZ2_hbMakeCodeLengths__heap[(yy + 1i32) as usize] as usize]
            < (*s).BZ2_hbMakeCodeLengths__weight
              [(*s).BZ2_hbMakeCodeLengths__heap[yy as usize] as usize]
        {
          yy += 1
        }
        if (*s).BZ2_hbMakeCodeLengths__weight[tmp_0 as usize]
          < (*s).BZ2_hbMakeCodeLengths__weight
            [(*s).BZ2_hbMakeCodeLengths__heap[yy as usize] as usize]
        {
          break;
        }
        (*s).BZ2_hbMakeCodeLengths__heap[zz_0 as usize] =
          (*s).BZ2_hbMakeCodeLengths__heap[yy as usize];
        zz_0 = yy
      }
      (*s).BZ2_hbMakeCodeLengths__heap[zz_0 as usize] = tmp_0;
      n2 = (*s).BZ2_hbMakeCodeLengths__heap[1];
      (*s).BZ2_hbMakeCodeLengths__heap[1] = (*s).BZ2_hbMakeCodeLengths__heap[nHeap as usize];
      nHeap -= 1;
      let mut zz_1: i32 = 0;
      let mut yy_0: i32 = 0;
      let mut tmp_1: i32 = 0;
      zz_1 = 1i32;
      tmp_1 = (*s).BZ2_hbMakeCodeLengths__heap[zz_1 as usize];
      loop {
        yy_0 = zz_1 << 1i32;
        if yy_0 > nHeap {
          break;
        }
        if yy_0 < nHeap
          && (*s).BZ2_hbMakeCodeLengths__weight
            [(*s).BZ2_hbMakeCodeLengths__heap[(yy_0 + 1i32) as usize] as usize]
            < (*s).BZ2_hbMakeCodeLengths__weight
              [(*s).BZ2_hbMakeCodeLengths__heap[yy_0 as usize] as usize]
        {
          yy_0 += 1
        }
        if (*s).BZ2_hbMakeCodeLengths__weight[tmp_1 as usize]
          < (*s).BZ2_hbMakeCodeLengths__weight
            [(*s).BZ2_hbMakeCodeLengths__heap[yy_0 as usize] as usize]
        {
          break;
        }
        (*s).BZ2_hbMakeCodeLengths__heap[zz_1 as usize] =
          (*s).BZ2_hbMakeCodeLengths__heap[yy_0 as usize];
        zz_1 = yy_0
      }
      (*s).BZ2_hbMakeCodeLengths__heap[zz_1 as usize] = tmp_1;
      nNodes += 1;
      (*s).BZ2_hbMakeCodeLengths__parent[n2 as usize] = nNodes;
      (*s).BZ2_hbMakeCodeLengths__parent[n1 as usize] =
        (*s).BZ2_hbMakeCodeLengths__parent[n2 as usize];
      (*s).BZ2_hbMakeCodeLengths__weight[nNodes as usize] =
        (((*s).BZ2_hbMakeCodeLengths__weight[n1 as usize] as libc::c_uint & 0xffffff00u32)
          .wrapping_add(
            (*s).BZ2_hbMakeCodeLengths__weight[n2 as usize] as libc::c_uint & 0xffffff00u32,
          )
          | (1i32
            + (if (*s).BZ2_hbMakeCodeLengths__weight[n1 as usize] & 0xffi32
              > (*s).BZ2_hbMakeCodeLengths__weight[n2 as usize] & 0xffi32
            {
              ((*s).BZ2_hbMakeCodeLengths__weight[n1 as usize]) & 0xffi32
            } else {
              ((*s).BZ2_hbMakeCodeLengths__weight[n2 as usize]) & 0xffi32
            })) as libc::c_uint) as i32;
      (*s).BZ2_hbMakeCodeLengths__parent[nNodes as usize] = -1i32;
      nHeap += 1;
      (*s).BZ2_hbMakeCodeLengths__heap[nHeap as usize] = nNodes;
      let mut zz_2: i32 = 0;
      let mut tmp_2: i32 = 0;
      zz_2 = nHeap;
      tmp_2 = (*s).BZ2_hbMakeCodeLengths__heap[zz_2 as usize];
      while (*s).BZ2_hbMakeCodeLengths__weight[tmp_2 as usize]
        < (*s).BZ2_hbMakeCodeLengths__weight
          [(*s).BZ2_hbMakeCodeLengths__heap[(zz_2 >> 1i32) as usize] as usize]
      {
        (*s).BZ2_hbMakeCodeLengths__heap[zz_2 as usize] =
          (*s).BZ2_hbMakeCodeLengths__heap[(zz_2 >> 1i32) as usize];
        zz_2 >>= 1i32
      }
      (*s).BZ2_hbMakeCodeLengths__heap[zz_2 as usize] = tmp_2
    }
    tooLong = 0i32 as Bool;
    i = 1i32;
    while i <= alphaSize {
      j = 0i32;
      k = i;
      while (*s).BZ2_hbMakeCodeLengths__parent[k as usize] >= 0i32 {
        k = (*s).BZ2_hbMakeCodeLengths__parent[k as usize];
        j += 1
      }
      *len.offset((i - 1i32) as isize) = j as u8;
      if j > maxLen {
        tooLong = 1i32 as Bool
      }
      i += 1
    }
    if tooLong == 0 {
      break;
    }
    /* 17 Oct 04: keep-going condition for the following loop used
    to be 'i < alphaSize', which missed the last element,
    theoretically leading to the possibility of the compressor
    looping.  However, this count-scaling step is only needed if
    one of the generated Huffman code words is longer than
    maxLen, which up to and including version 1.0.2 was 20 bits,
    which is extremely unlikely.  In version 1.0.3 maxLen was
    changed to 17 bits, which has minimal effect on compression
    ratio, but does mean this scaling step is used from time to
    time, enough to verify that it works.

    This means that bzip2-1.0.3 and later will only produce
    Huffman codes with a maximum length of 17 bits.  However, in
    order to preserve backwards compatibility with bitstreams
    produced by versions pre-1.0.3, the decompressor must still
    handle lengths of up to 20. */
    i = 1i32;
    while i <= alphaSize {
      j = (*s).BZ2_hbMakeCodeLengths__weight[i as usize] >> 8i32;
      /* bbox: yes, it is a signed division.
       * don't replace with shift! */
      j = 1i32 + j / 2i32;
      (*s).BZ2_hbMakeCodeLengths__weight[i as usize] = j << 8i32;
      i += 1
    }
  }
}
/*---------------------------------------------------*/
unsafe extern "C" fn BZ2_hbAssignCodes(
  mut code: *mut i32,
  mut length: *mut u8,
  mut minLen: i32,
  mut maxLen: i32,
  mut alphaSize: i32,
) {
  let mut n: i32 = 0;
  let mut vec: i32 = 0;
  let mut i: i32 = 0;
  vec = 0i32;
  n = minLen;
  while n <= maxLen {
    i = 0i32;
    while i < alphaSize {
      if *length.offset(i as isize) as libc::c_int == n {
        *code.offset(i as isize) = vec;
        vec += 1
      }
      i += 1
    }
    vec <<= 1i32;
    n += 1
  }
}
/* NB: compressStream() has to return -1 on errors, not die.
 * bbunpack() will correctly clean up in this case
 * (delete incomplete .bz2 file)
 */
/* Returns:
 * -1 on errors
 * total written bytes so far otherwise
 */
unsafe extern "C" fn bz_write(
  mut strm: *mut bz_stream,
  mut rbuf: *mut libc::c_void,
  mut rlen: ssize_t,
  mut wbuf: *mut libc::c_void,
) -> libc::c_longlong {
  let mut n: libc::c_int = 0;
  let mut n2: libc::c_int = 0;
  let mut ret: libc::c_int = 0;
  (*strm).avail_in = rlen as libc::c_uint;
  (*strm).next_in = rbuf as *mut libc::c_char;
  loop {
    (*strm).avail_out = IOBUF_SIZE as libc::c_int as libc::c_uint;
    (*strm).next_out = wbuf as *mut libc::c_char;
    ret = BZ2_bzCompress(strm, if rlen != 0 { 0i32 } else { 2i32 });
    if ret != 1i32 && ret != 3i32 && ret != 4i32 {
      /* BZ_FINISHed */
      bb_error_msg_and_die(
        b"internal error %d\x00" as *const u8 as *const libc::c_char,
        ret,
      ); /* prevent bogus error message */
    } /* it's small */
    n = (IOBUF_SIZE as libc::c_int as libc::c_uint).wrapping_sub((*strm).avail_out) as libc::c_int;
    if n != 0 {
      n2 = full_write(1i32, wbuf, n as size_t) as libc::c_int;
      if n2 != n {
        if n2 >= 0i32 {
          *bb_errno = 0i32
        }
        bb_simple_perror_msg(if n2 >= 0i32 {
          b"short write\x00" as *const u8 as *const libc::c_char
        } else {
          b"write error\x00" as *const u8 as *const libc::c_char
        });
        return -1i32 as libc::c_longlong;
      }
    }
    if ret == 4i32 {
      break;
    }
    if rlen != 0 && (*strm).avail_in == 0i32 as libc::c_uint {
      break;
    }
  }
  return (0i32 as libc::c_ulonglong).wrapping_add((*strm).total_out) as libc::c_longlong;
}
unsafe extern "C" fn compressStream(mut _xstate: *mut transformer_state_t) -> libc::c_longlong {
  let mut total: libc::c_longlong = 0;
  let mut opt: libc::c_uint = 0;
  let mut level: libc::c_uint = 0;
  let mut count: ssize_t = 0;
  let mut bzs: bz_stream = bz_stream {
    state: 0 as *mut libc::c_void,
    next_in: std::ptr::null_mut::<libc::c_char>(),
    next_out: std::ptr::null_mut::<libc::c_char>(),
    avail_in: 0,
    avail_out: 0,
    total_out: 0,
  };
  let mut iobuf: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  iobuf = xmalloc((2i32 * IOBUF_SIZE as libc::c_int) as size_t) as *mut libc::c_char;
  opt = option_mask32 >> 5i32 + 2i32 + 2i32;
  /* skipped BBUNPK_OPTSTR, "dt" and "zs" bits */
  opt |= 0x100i32 as libc::c_uint; /* if nothing else, assume -9 */
  level = 0i32 as libc::c_uint;
  loop {
    level = level.wrapping_add(1);
    if opt & 1i32 as libc::c_uint != 0 {
      break;
    }
    opt >>= 1i32
  }
  BZ2_bzCompressInit(&mut bzs, level as libc::c_int);
  loop {
    count = full_read(
      0i32,
      iobuf as *mut libc::c_void,
      IOBUF_SIZE as libc::c_int as size_t,
    );
    if count < 0 {
      bb_simple_perror_msg(b"read error\x00" as *const u8 as *const libc::c_char);
      total = -1i32 as libc::c_longlong;
      break;
    } else {
      /* if count == 0, bz_write finalizes compression */
      total = bz_write(
        &mut bzs,
        iobuf as *mut libc::c_void,
        count,
        iobuf.offset(IOBUF_SIZE as libc::c_int as isize) as *mut libc::c_void,
      );
      if count == 0 || total < 0i32 as libc::c_longlong {
        break;
      }
    }
  }
  /* Can't be conditional on ENABLE_FEATURE_CLEAN_UP -
   * we are called repeatedly
   */
  BZ2_bzCompressEnd(&mut bzs);
  free(iobuf as *mut libc::c_void);
  return total;
}
#[no_mangle]
pub unsafe extern "C" fn bzip2_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opt: libc::c_uint = 0;
  /* standard bzip2 flags
   * -d --decompress force decompression
   * -z --compress force compression
   * -k --keep     keep (don't delete) input files
   * -f --force    overwrite existing output files
   * -t --test     test compressed file integrity
   * -c --stdout   output to standard out
   * -q --quiet    suppress noncritical error messages
   * -v --verbose  be verbose (a 2nd -v gives more)
   * -s --small    use less memory (at most 2500k)
   * -1 .. -9      set block size to 100k .. 900k
   * --fast        alias for -1
   * --best        alias for -9
   */
  opt = getopt32(
    argv,
    b"^cfkvqdtzs123456789\x00s2\x00" as *const u8 as *const libc::c_char,
  );
  /* bunzip2_main may not be visible... */
  if opt & (BBUNPK_OPT_DECOMPRESS as libc::c_int | BBUNPK_OPT_TEST as libc::c_int) as libc::c_uint
    != 0
  {
    /* -d and/or -t */
    return bunzip2_main(argc, argv);
  }
  argv = argv.offset(optind as isize);
  return bbunpack(
    argv,
    Some(compressStream as unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong),
    Some(
      append_ext
        as unsafe extern "C" fn(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char,
    ),
    b"bz2\x00" as *const u8 as *const libc::c_char,
  );
}

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



use libc::free;
extern "C" {
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn malloc(_: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  /* Returns $SHELL, getpwuid(getuid())->pw_shell, or DEFAULT_SHELL.
   * Note that getpwuid result might need xstrdup'ing
   * if there is a possibility of intervening getpwxxx() calls.
   */
  /* Structures inside "struct caps" are Linux-specific and libcap-specific: */
  /* setup_environment:
   * if chdir pw->pw_dir: ok: else if to_tmp == 1: goto /tmp else: goto / or die
   * if clear_env = 1: cd(pw->pw_dir), clear environment, then set
   *   TERM=(old value)
   *   USER=pw->pw_name, LOGNAME=pw->pw_name
   *   PATH=bb_default_[root_]path
   *   HOME=pw->pw_dir
   *   SHELL=shell
   * else if change_env = 1:
   *   if not root (if pw->pw_uid != 0):
   *     USER=pw->pw_name, LOGNAME=pw->pw_name
   *   HOME=pw->pw_dir
   *   SHELL=shell
   * else does nothing
   *
   * NB: CHANGEENV and CLEARENV use setenv() - this leaks memory!
   * If setup_environment() is used is vforked child, this leaks memory _in parent too_!
   */
  /* Returns a malloced string */
  /*
   * rnd is additional random input. New one is returned.
   * Useful if you call crypt_make_salt many times in a row:
   * rnd = crypt_make_salt(buf1, 4, 0);
   * rnd = crypt_make_salt(buf2, 4, rnd);
   * rnd = crypt_make_salt(buf3, 4, rnd);
   * (otherwise we risk having same salt generated)
   */
  /*, int rnd*/
  /* "$N$" + sha_salt_16_bytes + NUL */
  /* Returns number of lines changed, or -1 on error */
  /* NB: typically you want to pass fd 0, not 1. Think 'applet | grep something' */
  /* NB: "unsigned request" is crucial! "int request" will break some arches! */
  /* At least glibc has horrendously large inline for this, so wrap it */
  /* "Keycodes" that report an escape sequence.
   * We use something which fits into signed char,
   * yet doesn't represent any valid Unicode character.
   * Also, -1 is reserved for error indication and we don't use it. */
  /* Used only if Alt/Ctrl/Shifted */
  /* Used only if Alted */
  /* ^^^^^ Be sure that last defined value is small enough.
   * Current read_key() code allows going up to -32 (0xfff..fffe0).
   * This gives three upper bits in LSB to play with:
   * KEYCODE_foo values are 0xfff..fffXX, lowest XX bits are: scavvvvv,
   * s=0 if SHIFT, c=0 if CTRL, a=0 if ALT,
   * vvvvv bits are the same for same key regardless of "shift bits".
   */
  //KEYCODE_SHIFT_...   = KEYCODE_...   & ~0x80,
  /* 0xfff..fff00 */
  /* How long is the longest ESC sequence we know?
   * We want it big enough to be able to contain
   * cursor position sequence "ESC [ 9999 ; 9999 R"
   */
  /* Note: fd may be in blocking or non-blocking mode, both make sense.
   * For one, less uses non-blocking mode.
   * Only the first read syscall inside read_key may block indefinitely
   * (unless fd is in non-blocking mode),
   * subsequent reads will time out after a few milliseconds.
   * Return of -1 means EOF or error (errno == 0 on EOF).
   * buffer[0] is used as a counter of buffered chars and must be 0
   * on first call.
   * timeout:
   * -2: do not poll(-1) for input - read() it, return on EAGAIN at once
   * -1: poll(-1) (i.e. block even on NONBLOCKed fd)
   * >=0: poll() for TIMEOUT milliseconds, return -1/EAGAIN on timeout
   */
  /* It's NOT just ENABLEd or disabled. It's a number: */
  /* must never be <= 0 */
  /* meaning of this field depends on FEATURE_EDITING_SAVE_ON_EXIT:
   * if !FEATURE_EDITING_SAVE_ON_EXIT: "how many lines are
   * in on-disk history"
   * if FEATURE_EDITING_SAVE_ON_EXIT: "how many in-memory lines are
   * also in on-disk history (and thus need to be skipped on save)"
   */
  /*
   * maxsize must be >= 2.
   * Returns:
   * -1 on read errors or EOF, or on bare Ctrl-D,
   * 0  on ctrl-C (the line entered is still returned in 'command'),
   * >0 length of input string, including terminating '\n'
   */
  /* synchronize with sizeof(task_struct.comm) in /usr/include/linux/sched.h */
  // For mixed 32/64 userspace, 32-bit pmap still needs
  // 64-bit field here to correctly show 64-bit processes:
  // (strictly speaking, other fields need to be wider too,
  // but they are in kbytes, not bytes, and they hold sizes,
  // not start addresses, sizes tend to be less than 4 terabytes)
  /* Fields are set to 0/NULL if failed to determine (or not requested) */
  /* Everything below must contain no ptrs to malloc'ed data:
   * it is memset(0) for each process in procps_scan() */
  /* we round it to kbytes */
  /* basename of executable in exec(2), read from /proc/N/stat
   * (if executable is symlink or script, it is NOT replaced
   * by link target or interpreter name) */
  /* user/group? - use passwd/group parsing functions */
  /* flag bits for procps_scan(xx, flags) calls */
  /* PSSCAN_CMD      = 1 << 6, - use read_cmdline instead */
  /* NB: used by find_pid_by_name(). Any applet using it
   * needs to be mentioned here. */
  //procps_status_t* alloc_procps_scan(void) FAST_FUNC;
  /* Format cmdline (up to col chars) into char buf[size] */
  /* Puts [comm] if cmdline is empty (-> process is a kernel thread) */
  /* Use strict=1 if you process input from untrusted source:
   * it will return NULL on invalid %xx (bad hex chars)
   * and str + 1 if decoded char is / or NUL.
   * In non-strict mode, it always succeeds (returns str),
   * and also it additionally decoded '+' to space.
   */
  /* Sign-extends to a value which never matches fgetc result: */
  /* always correctly aligned for uint64_t */
  /* must be directly before hash[] */
  /* 4 elements for md5, 5 for sha1, 8 for sha256 */
  /* must be directly before hash[] */
  /* always correctly aligned for uint64_t */
  /* TLS benefits from knowing that sha1 and sha256 share these. Give them "agnostic" names too */
  #[no_mangle]
  static mut global_crc32_table: *mut u32;
  #[no_mangle]
  fn crc32_block_endian0(
    val: u32,
    buf: *const libc::c_void,
    len: libc::c_uint,
    crc_table: *mut u32,
  ) -> u32;
  #[no_mangle]
  fn global_crc32_new_table_le() -> *mut u32;
  #[no_mangle]
  fn xtransformer_write(
    xstate: *mut transformer_state_t,
    buf: *const libc::c_void,
    bufsize: size_t,
  ) -> ssize_t;
}





pub type bb__aliased_u32 = u32;
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

use crate::librb::size_t;

use libc::ssize_t;


use crate::archival::libarchive::bb_archive::transformer_state_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xz_dec {
  pub sequence: C2RustUnnamed_6,
  pub pos: u32,
  pub vli: vli_type,
  pub in_start: size_t,
  pub out_start: size_t,
  pub crc32: u32,
  pub check_type: xz_check,
  pub mode: xz_mode,
  pub allow_buf_error: bool,
  pub block_header: C2RustUnnamed_5,
  pub block: C2RustUnnamed_4,
  pub index: C2RustUnnamed_2,
  pub temp: C2RustUnnamed_1,
  pub lzma2: *mut xz_dec_lzma2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xz_dec_lzma2 {
  pub rc: rc_dec,
  pub dict: dictionary,
  pub lzma2: lzma2_dec,
  pub lzma: lzma_dec,
  pub temp: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
  pub size: u32,
  pub buf: [u8; 63],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_dec {
  pub rep0: u32,
  pub rep1: u32,
  pub rep2: u32,
  pub rep3: u32,
  pub state: lzma_state,
  pub len: u32,
  pub lc: u32,
  pub literal_pos_mask: u32,
  pub pos_mask: u32,
  pub is_match: [[u16; 16]; 12],
  pub is_rep: [u16; 12],
  pub is_rep0: [u16; 12],
  pub is_rep1: [u16; 12],
  pub is_rep2: [u16; 12],
  pub is_rep0_long: [[u16; 16]; 12],
  pub dist_slot: [[u16; 64]; 4],
  pub dist_special: [u16; 114],
  pub dist_align: [u16; 16],
  pub match_len_dec: lzma_len_dec,
  pub rep_len_dec: lzma_len_dec,
  pub literal: [[u16; 768]; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_len_dec {
  pub choice: u16,
  pub choice2: u16,
  pub low: [[u16; 8]; 16],
  pub mid: [[u16; 8]; 16],
  pub high: [u16; 256],
}
/*
 * Maximum number of position states. A position state is the lowest pb
 * number of bits of the current uncompressed offset. In some places there
 * are different sets of probabilities for different position states.
 */
/*
 * This enum is used to track which LZMA symbols have occurred most recently
 * and in which order. This information is used to predict the next symbol.
 *
 * Symbols:
 *  - Literal: One 8-bit byte
 *  - Match: Repeat a chunk of data at some distance
 *  - Long repeat: Multi-byte match at a recently seen distance
 *  - Short repeat: One-byte repeat at a recently seen distance
 *
 * The symbol names are in from STATE_oldest_older_previous. REP means
 * either short or long repeated match, and NONLIT means any non-literal.
 */
pub type lzma_state = libc::c_uint;
pub const STATE_NONLIT_REP: lzma_state = 11;
pub const STATE_NONLIT_MATCH: lzma_state = 10;
pub const STATE_LIT_SHORTREP: lzma_state = 9;
pub const STATE_LIT_LONGREP: lzma_state = 8;
pub const STATE_LIT_MATCH: lzma_state = 7;
pub const STATE_SHORTREP_LIT: lzma_state = 6;
pub const STATE_REP_LIT: lzma_state = 5;
pub const STATE_MATCH_LIT: lzma_state = 4;
pub const STATE_SHORTREP_LIT_LIT: lzma_state = 3;
pub const STATE_REP_LIT_LIT: lzma_state = 2;
pub const STATE_MATCH_LIT_LIT: lzma_state = 1;
pub const STATE_LIT_LIT: lzma_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma2_dec {
  pub sequence: lzma2_seq,
  pub next_sequence: lzma2_seq,
  pub uncompressed: u32,
  pub compressed: u32,
  pub need_dict_reset: bool,
  pub need_props: bool,
}
pub type lzma2_seq = libc::c_uint;
pub const SEQ_COPY: lzma2_seq = 8;
pub const SEQ_LZMA_RUN: lzma2_seq = 7;
pub const SEQ_LZMA_PREPARE: lzma2_seq = 6;
pub const SEQ_PROPERTIES: lzma2_seq = 5;
pub const SEQ_COMPRESSED_1: lzma2_seq = 4;
pub const SEQ_COMPRESSED_0: lzma2_seq = 3;
pub const SEQ_UNCOMPRESSED_2: lzma2_seq = 2;
pub const SEQ_UNCOMPRESSED_1: lzma2_seq = 1;
pub const SEQ_CONTROL: lzma2_seq = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dictionary {
  pub buf: *mut u8,
  pub start: size_t,
  pub pos: size_t,
  pub full: size_t,
  pub limit: size_t,
  pub end: size_t,
  pub size: u32,
  pub size_max: u32,
  pub allocated: u32,
  pub mode: xz_mode,
}
/*
 * XZ decompressor
 *
 * Authors: Lasse Collin <lasse.collin@tukaani.org>
 *          Igor Pavlov <http://7-zip.org/>
 *
 * This file has been put into the public domain.
 * You can do whatever you want with this file.
 */
/* In Linux, this is used to make extern functions static when needed. */
/* In Linux, this is used to mark the functions with __init when needed. */
/* *
 * enum xz_mode - Operation mode
 *
 * @XZ_SINGLE:              Single-call mode. This uses less RAM than
 *                          than multi-call modes, because the LZMA2
 *                          dictionary doesn't need to be allocated as
 *                          part of the decoder state. All required data
 *                          structures are allocated at initialization,
 *                          so xz_dec_run() cannot return XZ_MEM_ERROR.
 * @XZ_PREALLOC:            Multi-call mode with preallocated LZMA2
 *                          dictionary buffer. All data structures are
 *                          allocated at initialization, so xz_dec_run()
 *                          cannot return XZ_MEM_ERROR.
 * @XZ_DYNALLOC:            Multi-call mode. The LZMA2 dictionary is
 *                          allocated once the required size has been
 *                          parsed from the stream headers. If the
 *                          allocation fails, xz_dec_run() will return
 *                          XZ_MEM_ERROR.
 *
 * It is possible to enable support only for a subset of the above
 * modes at compile time by defining XZ_DEC_SINGLE, XZ_DEC_PREALLOC,
 * or XZ_DEC_DYNALLOC. The xz_dec kernel module is always compiled
 * with support for all operation modes, but the preboot code may
 * be built with fewer features to minimize code size.
 */
pub type xz_mode = libc::c_uint;
pub const XZ_DYNALLOC: xz_mode = 2;
pub const XZ_PREALLOC: xz_mode = 1;
pub const XZ_SINGLE: xz_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rc_dec {
  pub range: u32,
  pub code: u32,
  pub init_bytes_left: u32,
  pub in_0: *const u8,
  pub in_pos: size_t,
  pub in_limit: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
  pub pos: size_t,
  pub size: size_t,
  pub buf: [u8; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
  pub sequence: C2RustUnnamed_3,
  pub size: vli_type,
  pub count: vli_type,
  pub hash: xz_dec_hash,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xz_dec_hash {
  pub unpadded: vli_type,
  pub uncompressed: vli_type,
  pub crc32: u32,
}
pub type vli_type = u64;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const SEQ_INDEX_UNCOMPRESSED: C2RustUnnamed_3 = 2;
pub const SEQ_INDEX_UNPADDED: C2RustUnnamed_3 = 1;
pub const SEQ_INDEX_COUNT: C2RustUnnamed_3 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
  pub compressed: vli_type,
  pub uncompressed: vli_type,
  pub count: vli_type,
  pub hash: xz_dec_hash,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
  pub compressed: vli_type,
  pub uncompressed: vli_type,
  pub size: u32,
}
pub type xz_check = libc::c_uint;
pub const XZ_CHECK_SHA256: xz_check = 10;
pub const XZ_CHECK_CRC64: xz_check = 4;
pub const XZ_CHECK_CRC32: xz_check = 1;
pub const XZ_CHECK_NONE: xz_check = 0;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const SEQ_STREAM_FOOTER: C2RustUnnamed_6 = 9;
pub const SEQ_INDEX_CRC32: C2RustUnnamed_6 = 8;
pub const SEQ_INDEX_PADDING: C2RustUnnamed_6 = 7;
pub const SEQ_INDEX: C2RustUnnamed_6 = 6;
pub const SEQ_BLOCK_CHECK: C2RustUnnamed_6 = 5;
pub const SEQ_BLOCK_PADDING: C2RustUnnamed_6 = 4;
pub const SEQ_BLOCK_UNCOMPRESS: C2RustUnnamed_6 = 3;
pub const SEQ_BLOCK_HEADER: C2RustUnnamed_6 = 2;
pub const SEQ_BLOCK_START: C2RustUnnamed_6 = 1;
pub const SEQ_STREAM_HEADER: C2RustUnnamed_6 = 0;
pub const XZ_UNSUPPORTED_CHECK: xz_ret = 2;
pub type xz_ret = libc::c_uint;
pub const XZ_BUF_ERROR: xz_ret = 8;
pub const XZ_DATA_ERROR: xz_ret = 7;
pub const XZ_OPTIONS_ERROR: xz_ret = 6;
pub const XZ_FORMAT_ERROR: xz_ret = 5;
pub const XZ_MEMLIMIT_ERROR: xz_ret = 4;
pub const XZ_MEM_ERROR: xz_ret = 3;
pub const XZ_STREAM_END: xz_ret = 1;
pub const XZ_OK: xz_ret = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xz_buf {
  pub in_0: *const u8,
  pub in_pos: size_t,
  pub in_size: size_t,
  pub out: *mut u8,
  pub out_pos: size_t,
  pub out_size: size_t,
}
/*
 * This file uses XZ Embedded library code which is written
 * by Lasse Collin <lasse.collin@tukaani.org>
 * and Igor Pavlov <http://7-zip.org/>
 *
 * See README file in unxz/ directory for more information.
 *
 * This file is:
 * Copyright (C) 2010 Denys Vlasenko <vda.linux@googlemail.com>
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* Skip check (rather than fail) of unsupported hash functions */
/* We use our own crc32 function */
unsafe extern "C" fn xz_crc32(
  mut buf: *const u8,
  mut size: size_t,
  mut crc: u32,
) -> u32 {
  return !crc32_block_endian0(
    !crc,
    buf as *const libc::c_void,
    size as libc::c_uint,
    global_crc32_table,
  );
}
/* Indicate that the latest symbol was a literal. */
#[inline]
unsafe extern "C" fn lzma_state_literal(mut state: *mut lzma_state) {
  if *state as libc::c_uint <= STATE_SHORTREP_LIT_LIT as libc::c_int as libc::c_uint {
    *state = STATE_LIT_LIT
  } else if *state as libc::c_uint <= STATE_LIT_SHORTREP as libc::c_int as libc::c_uint {
    *state = ::std::mem::transmute::<libc::c_uint, lzma_state>(
      (*state as libc::c_uint).wrapping_sub(3i32 as libc::c_uint),
    ) as lzma_state
  } else {
    *state = ::std::mem::transmute::<libc::c_uint, lzma_state>(
      (*state as libc::c_uint).wrapping_sub(6i32 as libc::c_uint),
    ) as lzma_state
  };
}
/* Indicate that the latest symbol was a match. */
#[inline]
unsafe extern "C" fn lzma_state_match(mut state: *mut lzma_state) {
  *state = if (*state as libc::c_uint) < 7i32 as libc::c_uint {
    STATE_LIT_MATCH as libc::c_int
  } else {
    STATE_NONLIT_MATCH as libc::c_int
  } as lzma_state;
}
/* Indicate that the latest state was a long repeated match. */
#[inline]
unsafe extern "C" fn lzma_state_long_rep(mut state: *mut lzma_state) {
  *state = if (*state as libc::c_uint) < 7i32 as libc::c_uint {
    STATE_LIT_LONGREP as libc::c_int
  } else {
    STATE_NONLIT_REP as libc::c_int
  } as lzma_state;
}
/* Indicate that the latest symbol was a short match. */
#[inline]
unsafe extern "C" fn lzma_state_short_rep(mut state: *mut lzma_state) {
  *state = if (*state as libc::c_uint) < 7i32 as libc::c_uint {
    STATE_LIT_SHORTREP as libc::c_int
  } else {
    STATE_NONLIT_REP as libc::c_int
  } as lzma_state;
}
/* Test if the previous symbol was a literal. */
#[inline]
unsafe extern "C" fn lzma_state_is_literal(mut state: lzma_state) -> bool {
  return (state as libc::c_uint) < 7i32 as libc::c_uint;
}
/*
 * Get the index of the appropriate probability array for decoding
 * the distance slot.
 */
#[inline]
unsafe extern "C" fn lzma_get_dist_state(mut len: u32) -> u32 {
  return if len < (4i32 + 2i32) as libc::c_uint {
    len.wrapping_sub(2i32 as libc::c_uint)
  } else {
    (4i32 - 1i32) as libc::c_uint
  };
}
/* *************
 * Dictionary *
 **************/
/*
 * Reset the dictionary state. When in single-call mode, set up the beginning
 * of the dictionary to point to the actual output buffer.
 */
/* Set dictionary write limit */
unsafe extern "C" fn dict_limit(mut dict: *mut dictionary, mut out_max: size_t) {
  if (*dict).end.wrapping_sub((*dict).pos) <= out_max {
    (*dict).limit = (*dict).end
  } else {
    (*dict).limit = (*dict).pos.wrapping_add(out_max)
  };
}
unsafe extern "C" fn dict_reset(mut dict: *mut dictionary, mut _b: *mut xz_buf) {
  (*dict).start = 0i32 as size_t;
  (*dict).pos = 0i32 as size_t;
  (*dict).limit = 0i32 as size_t;
  (*dict).full = 0i32 as size_t;
}
/* Return true if at least one byte can be written into the dictionary. */
/*
 * Get a byte from the dictionary at the given distance. The distance is
 * assumed to valid, or as a special case, zero when the dictionary is
 * still empty. This special case is needed for single-call decoding to
 * avoid writing a '\0' to the end of the destination buffer.
 */
/*
 * Put one byte into the dictionary. It is assumed that there is space for it.
 */
/*
 * Repeat given number of bytes from the given distance. If the distance is
 * invalid, false is returned. On success, true is returned and *len is
 * updated to indicate how many bytes were left to be repeated.
 */
/* Copy uncompressed data as is from input to dictionary and output buffers. */
/*
 * Flush pending data from dictionary to b->out. It is assumed that there is
 * enough space in b->out. This is guaranteed because caller uses dict_limit()
 * before decoding data into the dictionary.
 */
/* ****************
 * Range decoder *
 *****************/
/* Reset the range decoder. */
/*
 * Read the first five initial bytes into rc->code if they haven't been
 * read already. (Yes, the first byte gets completely ignored.)
 */
/* Return true if there may not be enough input for the next decoding loop. */
/*
 * Return true if it is possible (from point of view of range decoder) that
 * we have reached the end of the LZMA chunk.
 */
/* Read the next input byte if needed. */
/*
 * Decode one bit. In some versions, this function has been split in three
 * functions so that the compiler is supposed to be able to more easily avoid
 * an extra branch. In this particular version of the LZMA decoder, this
 * doesn't seem to be a good idea (tested with GCC 3.3.6, 3.4.6, and 4.3.3
 * on x86). Using a non-split version results in nicer looking code too.
 *
 * NOTE: This must return an int. Do not make it return a bool or the speed
 * of the code generated by GCC 3.x decreases 10-15 %. (GCC 4.3 doesn't care,
 * and it generates 10-20 % faster code than GCC 3.x from this file anyway.)
 */
/* Decode a bittree starting from the most significant bit. */
/* Decode a bittree starting from the least significant bit. */
/* Decode direct bits (fixed fifty-fifty probability) */
/* *******
 * LZMA *
 ********/
/* Get pointer to literal coder probability array. */
unsafe extern "C" fn lzma_literal_probs(mut s: *mut xz_dec_lzma2) -> *mut u16 {
  let mut prev_byte: u32 = dict_get(&mut (*s).dict, 0i32 as u32);
  let mut low: u32 = prev_byte >> (8i32 as libc::c_uint).wrapping_sub((*s).lzma.lc);
  let mut high: u32 =
    (((*s).dict.pos & (*s).lzma.literal_pos_mask as libc::c_ulong) << (*s).lzma.lc) as u32;
  return (*s).lzma.literal[low.wrapping_add(high) as usize].as_mut_ptr();
}
#[inline]
unsafe extern "C" fn rc_is_finished(mut rc: *const rc_dec) -> bool {
  return (*rc).code == 0i32 as libc::c_uint;
}
unsafe extern "C" fn dict_repeat(
  mut dict: *mut dictionary,
  mut len: *mut u32,
  mut dist: u32,
) -> bool {
  let mut back: size_t = 0;
  let mut left: u32 = 0;
  if dist as libc::c_ulong >= (*dict).full || dist >= (*dict).size {
    return 0i32 != 0;
  }
  left = if (*dict).limit.wrapping_sub((*dict).pos) < *len as libc::c_ulong {
    (*dict).limit.wrapping_sub((*dict).pos)
  } else {
    *len as libc::c_ulong
  } as u32;
  *len = (*len as libc::c_uint).wrapping_sub(left) as u32 as u32;
  back = (*dict)
    .pos
    .wrapping_sub(dist as libc::c_ulong)
    .wrapping_sub(1i32 as libc::c_ulong);
  if dist as libc::c_ulong >= (*dict).pos {
    back = (back as libc::c_ulong).wrapping_add((*dict).end) as size_t as size_t
  }
  loop {
    let fresh0 = back;
    back = back.wrapping_add(1);
    let fresh1 = (*dict).pos;
    (*dict).pos = (*dict).pos.wrapping_add(1);
    *(*dict).buf.offset(fresh1 as isize) = *(*dict).buf.offset(fresh0 as isize);
    if back == (*dict).end {
      back = 0i32 as size_t
    }
    left = left.wrapping_sub(1);
    if !(left > 0i32 as libc::c_uint) {
      break;
    }
  }
  if (*dict).full < (*dict).pos {
    (*dict).full = (*dict).pos
  }
  return 1i32 != 0;
}
unsafe extern "C" fn rc_reset(mut rc: *mut rc_dec) {
  (*rc).range = -1i32 as u32;
  (*rc).code = 0i32 as u32;
  (*rc).init_bytes_left = 5i32 as u32;
}
unsafe extern "C" fn dict_uncompressed(
  mut dict: *mut dictionary,
  mut b: *mut xz_buf,
  mut left: *mut u32,
) {
  let mut copy_size: size_t = 0;
  while *left > 0i32 as libc::c_uint && (*b).in_pos < (*b).in_size && (*b).out_pos < (*b).out_size {
    copy_size = if (*b).in_size.wrapping_sub((*b).in_pos) < (*b).out_size.wrapping_sub((*b).out_pos)
    {
      (*b).in_size.wrapping_sub((*b).in_pos)
    } else {
      (*b).out_size.wrapping_sub((*b).out_pos)
    };
    if copy_size > (*dict).end.wrapping_sub((*dict).pos) {
      copy_size = (*dict).end.wrapping_sub((*dict).pos)
    }
    if copy_size > *left as libc::c_ulong {
      copy_size = *left as size_t
    }
    *left = (*left as libc::c_ulong).wrapping_sub(copy_size) as u32 as u32;
    memcpy(
      (*dict).buf.offset((*dict).pos as isize) as *mut libc::c_void,
      (*b).in_0.offset((*b).in_pos as isize) as *const libc::c_void,
      copy_size,
    );
    (*dict).pos = ((*dict).pos as libc::c_ulong).wrapping_add(copy_size) as size_t as size_t;
    if (*dict).full < (*dict).pos {
      (*dict).full = (*dict).pos
    }
    if (*dict).pos == (*dict).end {
      (*dict).pos = 0i32 as size_t
    }
    memcpy(
      (*b).out.offset((*b).out_pos as isize) as *mut libc::c_void,
      (*b).in_0.offset((*b).in_pos as isize) as *const libc::c_void,
      copy_size,
    );
    (*dict).start = (*dict).pos;
    (*b).out_pos = ((*b).out_pos as libc::c_ulong).wrapping_add(copy_size) as size_t as size_t;
    (*b).in_pos = ((*b).in_pos as libc::c_ulong).wrapping_add(copy_size) as size_t as size_t
  }
}
#[inline(always)]
unsafe extern "C" fn rc_bittree_reverse(
  mut rc: *mut rc_dec,
  mut probs: *mut u16,
  mut dest: *mut u32,
  mut limit: u32,
) {
  let mut symbol: u32 = 1i32 as u32;
  let mut i: u32 = 0i32 as u32;
  loop {
    if rc_bit(rc, &mut *probs.offset(symbol as isize)) != 0 {
      symbol = (symbol << 1i32).wrapping_add(1i32 as libc::c_uint);
      *dest =
        (*dest as libc::c_uint).wrapping_add((1i32 << i) as libc::c_uint) as u32 as u32
    } else {
      symbol <<= 1i32
    }
    i = i.wrapping_add(1);
    if !(i < limit) {
      break;
    }
  }
}
#[inline(always)]
unsafe extern "C" fn rc_bit(mut rc: *mut rc_dec, mut prob: *mut u16) -> libc::c_int {
  let mut bound: u32 = 0;
  let mut bit: libc::c_int = 0;
  rc_normalize(rc);
  bound = ((*rc).range >> 11i32).wrapping_mul(*prob as libc::c_uint);
  if (*rc).code < bound {
    (*rc).range = bound;
    *prob = (*prob as libc::c_int + ((1i32 << 11i32) - *prob as libc::c_int >> 5i32)) as u16;
    bit = 0i32
  } else {
    (*rc).range = ((*rc).range as libc::c_uint).wrapping_sub(bound) as u32 as u32;
    (*rc).code = ((*rc).code as libc::c_uint).wrapping_sub(bound) as u32 as u32;
    *prob = (*prob as libc::c_int - (*prob as libc::c_int >> 5i32)) as u16;
    bit = 1i32
  }
  return bit;
}
#[inline]
unsafe extern "C" fn rc_direct(mut rc: *mut rc_dec, mut dest: *mut u32, mut limit: u32) {
  let mut mask: u32 = 0;
  loop {
    rc_normalize(rc);
    (*rc).range >>= 1i32;
    (*rc).code = ((*rc).code as libc::c_uint).wrapping_sub((*rc).range) as u32 as u32;
    mask = (0i32 as u32).wrapping_sub((*rc).code >> 31i32);
    (*rc).code =
      ((*rc).code as libc::c_uint).wrapping_add((*rc).range & mask) as u32 as u32;
    *dest = (*dest << 1i32).wrapping_add(mask.wrapping_add(1i32 as libc::c_uint));
    limit = limit.wrapping_sub(1);
    if !(limit > 0i32 as libc::c_uint) {
      break;
    }
  }
}
unsafe extern "C" fn rc_read_init(mut rc: *mut rc_dec, mut b: *mut xz_buf) -> bool {
  while (*rc).init_bytes_left > 0i32 as libc::c_uint {
    if (*b).in_pos == (*b).in_size {
      return 0i32 != 0;
    }
    let fresh2 = (*b).in_pos;
    (*b).in_pos = (*b).in_pos.wrapping_add(1);
    (*rc).code =
      ((*rc).code << 8i32).wrapping_add(*(*b).in_0.offset(fresh2 as isize) as libc::c_uint);
    (*rc).init_bytes_left = (*rc).init_bytes_left.wrapping_sub(1)
  }
  return 1i32 != 0;
}
#[inline(always)]
unsafe extern "C" fn dict_has_space(mut dict: *const dictionary) -> bool {
  return (*dict).pos < (*dict).limit;
}
#[inline(always)]
unsafe extern "C" fn rc_normalize(mut rc: *mut rc_dec) {
  if (*rc).range < (1i32 << 24i32) as libc::c_uint {
    (*rc).range <<= 8i32;
    let fresh3 = (*rc).in_pos;
    (*rc).in_pos = (*rc).in_pos.wrapping_add(1);
    (*rc).code =
      ((*rc).code << 8i32).wrapping_add(*(*rc).in_0.offset(fresh3 as isize) as libc::c_uint)
  };
}
#[inline]
unsafe extern "C" fn dict_put(mut dict: *mut dictionary, mut byte: u8) {
  let fresh4 = (*dict).pos;
  (*dict).pos = (*dict).pos.wrapping_add(1);
  *(*dict).buf.offset(fresh4 as isize) = byte;
  if (*dict).full < (*dict).pos {
    (*dict).full = (*dict).pos
  };
}
#[inline(always)]
unsafe extern "C" fn dict_get(mut dict: *const dictionary, mut dist: u32) -> u32 {
  let mut offset: size_t = (*dict)
    .pos
    .wrapping_sub(dist as libc::c_ulong)
    .wrapping_sub(1i32 as libc::c_ulong);
  if dist as libc::c_ulong >= (*dict).pos {
    offset = (offset as libc::c_ulong).wrapping_add((*dict).end) as size_t as size_t
  }
  return if (*dict).full > 0i32 as libc::c_ulong {
    *(*dict).buf.offset(offset as isize) as libc::c_int
  } else {
    0i32
  } as u32;
}
#[inline(always)]
unsafe extern "C" fn rc_bittree(
  mut rc: *mut rc_dec,
  mut probs: *mut u16,
  mut limit: u32,
) -> u32 {
  let mut symbol: u32 = 1i32 as u32;
  loop {
    if rc_bit(rc, &mut *probs.offset(symbol as isize)) != 0 {
      symbol = (symbol << 1i32).wrapping_add(1i32 as libc::c_uint)
    } else {
      symbol <<= 1i32
    }
    if !(symbol < limit) {
      break;
    }
  }
  return symbol;
}
unsafe extern "C" fn dict_flush(mut dict: *mut dictionary, mut b: *mut xz_buf) -> u32 {
  let mut copy_size: size_t = (*dict).pos.wrapping_sub((*dict).start);
  if (*dict).pos == (*dict).end {
    (*dict).pos = 0i32 as size_t
  }
  memcpy(
    (*b).out.offset((*b).out_pos as isize) as *mut libc::c_void,
    (*dict).buf.offset((*dict).start as isize) as *const libc::c_void,
    copy_size,
  );
  (*dict).start = (*dict).pos;
  (*b).out_pos = ((*b).out_pos as libc::c_ulong).wrapping_add(copy_size) as size_t as size_t;
  return copy_size as u32;
}
#[inline]
unsafe extern "C" fn rc_limit_exceeded(mut rc: *const rc_dec) -> bool {
  return (*rc).in_pos > (*rc).in_limit;
}
/* Decode a literal (one 8-bit byte) */
unsafe extern "C" fn lzma_literal(mut s: *mut xz_dec_lzma2) {
  let mut probs: *mut u16 = 0 as *mut u16;
  let mut symbol: u32 = 0;
  let mut match_byte: u32 = 0;
  let mut match_bit: u32 = 0;
  let mut offset: u32 = 0;
  let mut i: u32 = 0;
  probs = lzma_literal_probs(s);
  if lzma_state_is_literal((*s).lzma.state) {
    symbol = rc_bittree(&mut (*s).rc, probs, 0x100i32 as u32)
  } else {
    symbol = 1i32 as u32;
    match_byte = dict_get(&mut (*s).dict, (*s).lzma.rep0) << 1i32;
    offset = 0x100i32 as u32;
    loop {
      match_bit = match_byte & offset;
      match_byte <<= 1i32;
      i = offset.wrapping_add(match_bit).wrapping_add(symbol);
      if rc_bit(&mut (*s).rc, &mut *probs.offset(i as isize)) != 0 {
        symbol = (symbol << 1i32).wrapping_add(1i32 as libc::c_uint);
        offset &= match_bit
      } else {
        symbol <<= 1i32;
        offset &= !match_bit
      }
      if !(symbol < 0x100i32 as libc::c_uint) {
        break;
      }
    }
  }
  dict_put(&mut (*s).dict, symbol as u8);
  lzma_state_literal(&mut (*s).lzma.state);
}
/* Decode the length of the match into s->lzma.len. */
unsafe extern "C" fn lzma_len(
  mut s: *mut xz_dec_lzma2,
  mut l: *mut lzma_len_dec,
  mut pos_state: u32,
) {
  let mut probs: *mut u16 = 0 as *mut u16;
  let mut limit: u32 = 0;
  if rc_bit(&mut (*s).rc, &mut (*l).choice) == 0 {
    probs = (*l).low[pos_state as usize].as_mut_ptr();
    limit = (1i32 << 3i32) as u32;
    (*s).lzma.len = 2i32 as u32
  } else if rc_bit(&mut (*s).rc, &mut (*l).choice2) == 0 {
    probs = (*l).mid[pos_state as usize].as_mut_ptr();
    limit = (1i32 << 3i32) as u32;
    (*s).lzma.len = (2i32 + (1i32 << 3i32)) as u32
  } else {
    probs = (*l).high.as_mut_ptr();
    limit = (1i32 << 8i32) as u32;
    (*s).lzma.len = (2i32 + (1i32 << 3i32) + (1i32 << 3i32)) as u32
  }
  (*s).lzma.len = ((*s).lzma.len as libc::c_uint)
    .wrapping_add(rc_bittree(&mut (*s).rc, probs, limit).wrapping_sub(limit))
    as u32 as u32;
}
/* Decode a match. The distance will be stored in s->lzma.rep0. */
unsafe extern "C" fn lzma_match(mut s: *mut xz_dec_lzma2, mut pos_state: u32) {
  let mut probs: *mut u16 = 0 as *mut u16;
  let mut dist_slot: u32 = 0;
  let mut limit: u32 = 0;
  lzma_state_match(&mut (*s).lzma.state);
  (*s).lzma.rep3 = (*s).lzma.rep2;
  (*s).lzma.rep2 = (*s).lzma.rep1;
  (*s).lzma.rep1 = (*s).lzma.rep0;
  lzma_len(s, &mut (*s).lzma.match_len_dec, pos_state);
  probs = (*s).lzma.dist_slot[lzma_get_dist_state((*s).lzma.len) as usize].as_mut_ptr();
  dist_slot = rc_bittree(&mut (*s).rc, probs, (1i32 << 6i32) as u32)
    .wrapping_sub((1i32 << 6i32) as libc::c_uint);
  if dist_slot < 4i32 as libc::c_uint {
    (*s).lzma.rep0 = dist_slot
  } else {
    limit = (dist_slot >> 1i32).wrapping_sub(1i32 as libc::c_uint);
    (*s).lzma.rep0 = (2i32 as libc::c_uint).wrapping_add(dist_slot & 1i32 as libc::c_uint);
    if dist_slot < 14i32 as libc::c_uint {
      (*s).lzma.rep0 <<= limit;
      probs = (*s)
        .lzma
        .dist_special
        .as_mut_ptr()
        .offset((*s).lzma.rep0 as isize)
        .offset(-(dist_slot as isize))
        .offset(-1);
      rc_bittree_reverse(&mut (*s).rc, probs, &mut (*s).lzma.rep0, limit);
    } else {
      rc_direct(
        &mut (*s).rc,
        &mut (*s).lzma.rep0,
        limit.wrapping_sub(4i32 as libc::c_uint),
      );
      (*s).lzma.rep0 <<= 4i32;
      rc_bittree_reverse(
        &mut (*s).rc,
        (*s).lzma.dist_align.as_mut_ptr(),
        &mut (*s).lzma.rep0,
        4i32 as u32,
      );
    }
  };
}
/*
 * Decode a repeated match. The distance is one of the four most recently
 * seen matches. The distance will be stored in s->lzma.rep0.
 */
unsafe extern "C" fn lzma_rep_match(mut s: *mut xz_dec_lzma2, mut pos_state: u32) {
  let mut tmp: u32 = 0;
  if rc_bit(
    &mut (*s).rc,
    &mut *(*s)
      .lzma
      .is_rep0
      .as_mut_ptr()
      .offset((*s).lzma.state as isize),
  ) == 0
  {
    if rc_bit(
      &mut (*s).rc,
      &mut *(*(*s)
        .lzma
        .is_rep0_long
        .as_mut_ptr()
        .offset((*s).lzma.state as isize))
      .as_mut_ptr()
      .offset(pos_state as isize),
    ) == 0
    {
      lzma_state_short_rep(&mut (*s).lzma.state);
      (*s).lzma.len = 1i32 as u32;
      return;
    }
  } else {
    if rc_bit(
      &mut (*s).rc,
      &mut *(*s)
        .lzma
        .is_rep1
        .as_mut_ptr()
        .offset((*s).lzma.state as isize),
    ) == 0
    {
      tmp = (*s).lzma.rep1
    } else {
      if rc_bit(
        &mut (*s).rc,
        &mut *(*s)
          .lzma
          .is_rep2
          .as_mut_ptr()
          .offset((*s).lzma.state as isize),
      ) == 0
      {
        tmp = (*s).lzma.rep2
      } else {
        tmp = (*s).lzma.rep3;
        (*s).lzma.rep3 = (*s).lzma.rep2
      }
      (*s).lzma.rep2 = (*s).lzma.rep1
    }
    (*s).lzma.rep1 = (*s).lzma.rep0;
    (*s).lzma.rep0 = tmp
  }
  lzma_state_long_rep(&mut (*s).lzma.state);
  lzma_len(s, &mut (*s).lzma.rep_len_dec, pos_state);
}
/* LZMA decoder core */
unsafe extern "C" fn lzma_main(mut s: *mut xz_dec_lzma2) -> bool {
  let mut pos_state: u32 = 0;
  /*
   * If the dictionary was reached during the previous call, try to
   * finish the possibly pending repeat in the dictionary.
   */
  if dict_has_space(&mut (*s).dict) as libc::c_int != 0 && (*s).lzma.len > 0i32 as libc::c_uint {
    dict_repeat(&mut (*s).dict, &mut (*s).lzma.len, (*s).lzma.rep0);
  }
  /*
   * Decode more LZMA symbols. One iteration may consume up to
   * LZMA_IN_REQUIRED - 1 bytes.
   */
  while dict_has_space(&mut (*s).dict) as libc::c_int != 0 && !rc_limit_exceeded(&mut (*s).rc) {
    pos_state = ((*s).dict.pos & (*s).lzma.pos_mask as libc::c_ulong) as u32;
    if rc_bit(
      &mut (*s).rc,
      &mut *(*(*s)
        .lzma
        .is_match
        .as_mut_ptr()
        .offset((*s).lzma.state as isize))
      .as_mut_ptr()
      .offset(pos_state as isize),
    ) == 0
    {
      lzma_literal(s);
    } else {
      if rc_bit(
        &mut (*s).rc,
        &mut *(*s)
          .lzma
          .is_rep
          .as_mut_ptr()
          .offset((*s).lzma.state as isize),
      ) != 0
      {
        lzma_rep_match(s, pos_state);
      } else {
        lzma_match(s, pos_state);
      }
      if !dict_repeat(&mut (*s).dict, &mut (*s).lzma.len, (*s).lzma.rep0) {
        return 0i32 != 0;
      }
    }
  }
  /*
   * Having the range decoder always normalized when we are outside
   * this function makes it easier to correctly handle end of the chunk.
   */
  rc_normalize(&mut (*s).rc);
  return 1i32 != 0;
}
/*
 * Reset the LZMA decoder and range decoder state. Dictionary is nore reset
 * here, because LZMA state may be reset without resetting the dictionary.
 */
unsafe extern "C" fn lzma_reset(mut s: *mut xz_dec_lzma2) {
  let mut probs: *mut u16 = 0 as *mut u16;
  let mut i: size_t = 0;
  (*s).lzma.state = STATE_LIT_LIT;
  (*s).lzma.rep0 = 0i32 as u32;
  (*s).lzma.rep1 = 0i32 as u32;
  (*s).lzma.rep2 = 0i32 as u32;
  (*s).lzma.rep3 = 0i32 as u32;
  /*
   * All probabilities are initialized to the same value. This hack
   * makes the code smaller by avoiding a separate loop for each
   * probability array.
   *
   * This could be optimized so that only that part of literal
   * probabilities that are actually required. In the common case
   * we would write 12 KiB less.
   */
  probs = (*s).lzma.is_match[0].as_mut_ptr();
  i = 0i32 as size_t;
  while i < (1846i32 + (1i32 << 4i32) * 0x300i32) as libc::c_ulong {
    *probs.offset(i as isize) = ((1i32 << 11i32) / 2i32) as u16;
    i = i.wrapping_add(1)
  }
  rc_reset(&mut (*s).rc);
}
/*
 * Decode and validate LZMA properties (lc/lp/pb) and calculate the bit masks
 * from the decoded lp and pb values. On success, the LZMA decoder state is
 * reset and true is returned.
 */
unsafe extern "C" fn lzma_props(mut s: *mut xz_dec_lzma2, mut props: u8) -> bool {
  if props as libc::c_int > (4i32 * 5i32 + 4i32) * 9i32 + 8i32 {
    return 0i32 != 0;
  }
  (*s).lzma.pos_mask = 0i32 as u32;
  while props as libc::c_int >= 9i32 * 5i32 {
    props = (props as libc::c_int - 9i32 * 5i32) as u8;
    (*s).lzma.pos_mask = (*s).lzma.pos_mask.wrapping_add(1)
  }
  (*s).lzma.pos_mask = ((1i32 << (*s).lzma.pos_mask) - 1i32) as u32;
  (*s).lzma.literal_pos_mask = 0i32 as u32;
  while props as libc::c_int >= 9i32 {
    props = (props as libc::c_int - 9i32) as u8;
    (*s).lzma.literal_pos_mask = (*s).lzma.literal_pos_mask.wrapping_add(1)
  }
  (*s).lzma.lc = props as u32;
  if (*s).lzma.lc.wrapping_add((*s).lzma.literal_pos_mask) > 4i32 as libc::c_uint {
    return 0i32 != 0;
  }
  (*s).lzma.literal_pos_mask = ((1i32 << (*s).lzma.literal_pos_mask) - 1i32) as u32;
  lzma_reset(s);
  return 1i32 != 0;
}
/* ********
 * LZMA2 *
 *********/
/*
 * The LZMA decoder assumes that if the input limit (s->rc.in_limit) hasn't
 * been exceeded, it is safe to read up to LZMA_IN_REQUIRED bytes. This
 * wrapper function takes care of making the LZMA decoder's assumption safe.
 *
 * As long as there is plenty of input left to be decoded in the current LZMA
 * chunk, we decode directly from the caller-supplied input buffer until
 * there's LZMA_IN_REQUIRED bytes left. Those remaining bytes are copied into
 * s->temp.buf, which (hopefully) gets filled on the next call to this
 * function. We decode a few bytes from the temporary buffer so that we can
 * continue decoding from the caller-supplied input buffer again.
 */
unsafe extern "C" fn lzma2_lzma(mut s: *mut xz_dec_lzma2, mut b: *mut xz_buf) -> bool {
  let mut in_avail: size_t = 0;
  let mut tmp: u32 = 0;
  in_avail = (*b).in_size.wrapping_sub((*b).in_pos);
  if (*s).temp.size > 0i32 as libc::c_uint || (*s).lzma2.compressed == 0i32 as libc::c_uint {
    tmp = ((2i32 * 21i32) as libc::c_uint).wrapping_sub((*s).temp.size);
    if tmp > (*s).lzma2.compressed.wrapping_sub((*s).temp.size) {
      tmp = (*s).lzma2.compressed.wrapping_sub((*s).temp.size)
    }
    if tmp as libc::c_ulong > in_avail {
      tmp = in_avail as u32
    }
    memcpy(
      (*s).temp.buf.as_mut_ptr().offset((*s).temp.size as isize) as *mut libc::c_void,
      (*b).in_0.offset((*b).in_pos as isize) as *const libc::c_void,
      tmp as libc::c_ulong,
    );
    if (*s).temp.size.wrapping_add(tmp) == (*s).lzma2.compressed {
      memset(
        (*s)
          .temp
          .buf
          .as_mut_ptr()
          .offset((*s).temp.size as isize)
          .offset(tmp as isize) as *mut libc::c_void,
        0i32,
        (::std::mem::size_of::<[u8; 63]>() as libc::c_ulong)
          .wrapping_sub((*s).temp.size as libc::c_ulong)
          .wrapping_sub(tmp as libc::c_ulong),
      );
      (*s).rc.in_limit = (*s).temp.size.wrapping_add(tmp) as size_t
    } else if (*s).temp.size.wrapping_add(tmp) < 21i32 as libc::c_uint {
      (*s).temp.size = ((*s).temp.size as libc::c_uint).wrapping_add(tmp) as u32 as u32;
      (*b).in_pos =
        ((*b).in_pos as libc::c_ulong).wrapping_add(tmp as libc::c_ulong) as size_t as size_t;
      return 1i32 != 0;
    } else {
      (*s).rc.in_limit = (*s)
        .temp
        .size
        .wrapping_add(tmp)
        .wrapping_sub(21i32 as libc::c_uint) as size_t
    }
    (*s).rc.in_0 = (*s).temp.buf.as_mut_ptr();
    (*s).rc.in_pos = 0i32 as size_t;
    if !lzma_main(s) || (*s).rc.in_pos > (*s).temp.size.wrapping_add(tmp) as libc::c_ulong {
      return 0i32 != 0;
    }
    (*s).lzma2.compressed =
      ((*s).lzma2.compressed as libc::c_ulong).wrapping_sub((*s).rc.in_pos) as u32 as u32;
    if (*s).rc.in_pos < (*s).temp.size as libc::c_ulong {
      (*s).temp.size =
        ((*s).temp.size as libc::c_ulong).wrapping_sub((*s).rc.in_pos) as u32 as u32;
      memmove(
        (*s).temp.buf.as_mut_ptr() as *mut libc::c_void,
        (*s).temp.buf.as_mut_ptr().offset((*s).rc.in_pos as isize) as *const libc::c_void,
        (*s).temp.size as libc::c_ulong,
      );
      return 1i32 != 0;
    }
    (*b).in_pos = ((*b).in_pos as libc::c_ulong)
      .wrapping_add((*s).rc.in_pos.wrapping_sub((*s).temp.size as libc::c_ulong))
      as size_t as size_t;
    (*s).temp.size = 0i32 as u32
  }
  in_avail = (*b).in_size.wrapping_sub((*b).in_pos);
  if in_avail >= 21i32 as libc::c_ulong {
    (*s).rc.in_0 = (*b).in_0;
    (*s).rc.in_pos = (*b).in_pos;
    if in_avail >= (*s).lzma2.compressed.wrapping_add(21i32 as libc::c_uint) as libc::c_ulong {
      (*s).rc.in_limit = (*b)
        .in_pos
        .wrapping_add((*s).lzma2.compressed as libc::c_ulong)
    } else {
      (*s).rc.in_limit = (*b).in_size.wrapping_sub(21i32 as libc::c_ulong)
    }
    if !lzma_main(s) {
      return 0i32 != 0;
    }
    in_avail = (*s).rc.in_pos.wrapping_sub((*b).in_pos);
    if in_avail > (*s).lzma2.compressed as libc::c_ulong {
      return 0i32 != 0;
    }
    (*s).lzma2.compressed =
      ((*s).lzma2.compressed as libc::c_ulong).wrapping_sub(in_avail) as u32 as u32;
    (*b).in_pos = (*s).rc.in_pos
  }
  in_avail = (*b).in_size.wrapping_sub((*b).in_pos);
  if in_avail < 21i32 as libc::c_ulong {
    if in_avail > (*s).lzma2.compressed as libc::c_ulong {
      in_avail = (*s).lzma2.compressed as size_t
    }
    memcpy(
      (*s).temp.buf.as_mut_ptr() as *mut libc::c_void,
      (*b).in_0.offset((*b).in_pos as isize) as *const libc::c_void,
      in_avail,
    );
    (*s).temp.size = in_avail as u32;
    (*b).in_pos = ((*b).in_pos as libc::c_ulong).wrapping_add(in_avail) as size_t as size_t
  }
  return 1i32 != 0;
}
/*
 * Decode the LZMA2 properties (one byte) and reset the decoder. Return
 * XZ_OK on success, XZ_MEMLIMIT_ERROR if the preallocated dictionary is not
 * big enough, and XZ_OPTIONS_ERROR if props indicates something that this
 * decoder doesn't support.
 */
/* Decode raw LZMA2 stream from b->in to b->out. */
/*
 * Take care of the LZMA2 control layer, and forward the job of actual LZMA
 * decoding or copying of uncompressed chunks to other functions.
 */
#[inline(never)]
unsafe extern "C" fn xz_dec_lzma2_run(mut s: *mut xz_dec_lzma2, mut b: *mut xz_buf) -> xz_ret {
  let mut tmp: u32 = 0;
  while (*b).in_pos < (*b).in_size
    || (*s).lzma2.sequence as libc::c_uint == SEQ_LZMA_RUN as libc::c_int as libc::c_uint
  {
    let mut current_block_64: u64;
    match (*s).lzma2.sequence as libc::c_uint {
      0 => {
        /*
         * LZMA2 control byte
         *
         * Exact values:
         *   0x00   End marker
         *   0x01   Dictionary reset followed by
         *          an uncompressed chunk
         *   0x02   Uncompressed chunk (no dictionary reset)
         *
         * Highest three bits (s->control & 0xE0):
         *   0xE0   Dictionary reset, new properties and state
         *          reset, followed by LZMA compressed chunk
         *   0xC0   New properties and state reset, followed
         *          by LZMA compressed chunk (no dictionary
         *          reset)
         *   0xA0   State reset using old properties,
         *          followed by LZMA compressed chunk (no
         *          dictionary reset)
         *   0x80   LZMA chunk (no dictionary or state reset)
         *
         * For LZMA compressed chunks, the lowest five bits
         * (s->control & 1F) are the highest bits of the
         * uncompressed size (bits 16-20).
         *
         * A new LZMA2 stream must begin with a dictionary
         * reset. The first LZMA chunk must set new
         * properties and reset the LZMA state.
         *
         * Values that don't match anything described above
         * are invalid and we return XZ_DATA_ERROR.
         */
        let fresh5 = (*b).in_pos;
        (*b).in_pos = (*b).in_pos.wrapping_add(1);
        tmp = *(*b).in_0.offset(fresh5 as isize) as u32;
        if tmp == 0i32 as libc::c_uint {
          return XZ_STREAM_END;
        }
        if tmp >= 0xe0i32 as libc::c_uint || tmp == 0x1i32 as libc::c_uint {
          (*s).lzma2.need_props = 1i32 != 0;
          (*s).lzma2.need_dict_reset = 0i32 != 0;
          dict_reset(&mut (*s).dict, b);
        } else if (*s).lzma2.need_dict_reset {
          return XZ_DATA_ERROR;
        }
        if tmp >= 0x80i32 as libc::c_uint {
          (*s).lzma2.uncompressed = (tmp & 0x1fi32 as libc::c_uint) << 16i32;
          (*s).lzma2.sequence = SEQ_UNCOMPRESSED_1;
          if tmp >= 0xc0i32 as libc::c_uint {
            /*
             * When there are new properties,
             * state reset is done at
             * SEQ_PROPERTIES.
             */
            (*s).lzma2.need_props = 0i32 != 0;
            (*s).lzma2.next_sequence = SEQ_PROPERTIES
          } else if (*s).lzma2.need_props {
            return XZ_DATA_ERROR;
          } else {
            (*s).lzma2.next_sequence = SEQ_LZMA_PREPARE;
            if tmp >= 0xa0i32 as libc::c_uint {
              lzma_reset(s);
            }
          }
        } else {
          if tmp > 0x2i32 as libc::c_uint {
            return XZ_DATA_ERROR;
          }
          (*s).lzma2.sequence = SEQ_COMPRESSED_0;
          (*s).lzma2.next_sequence = SEQ_COPY
        }
        current_block_64 = 5807581744382915773;
      }
      1 => {
        let fresh6 = (*b).in_pos;
        (*b).in_pos = (*b).in_pos.wrapping_add(1);
        (*s).lzma2.uncompressed = ((*s).lzma2.uncompressed as libc::c_uint)
          .wrapping_add((*(*b).in_0.offset(fresh6 as isize) as u32) << 8i32)
          as u32 as u32;
        (*s).lzma2.sequence = SEQ_UNCOMPRESSED_2;
        current_block_64 = 5807581744382915773;
      }
      2 => {
        let fresh7 = (*b).in_pos;
        (*b).in_pos = (*b).in_pos.wrapping_add(1);
        (*s).lzma2.uncompressed = ((*s).lzma2.uncompressed as libc::c_uint).wrapping_add(
          (*(*b).in_0.offset(fresh7 as isize) as u32).wrapping_add(1i32 as libc::c_uint),
        ) as u32 as u32;
        (*s).lzma2.sequence = SEQ_COMPRESSED_0;
        current_block_64 = 5807581744382915773;
      }
      3 => {
        let fresh8 = (*b).in_pos;
        (*b).in_pos = (*b).in_pos.wrapping_add(1);
        (*s).lzma2.compressed = (*(*b).in_0.offset(fresh8 as isize) as u32) << 8i32;
        (*s).lzma2.sequence = SEQ_COMPRESSED_1;
        current_block_64 = 5807581744382915773;
      }
      4 => {
        let fresh9 = (*b).in_pos;
        (*b).in_pos = (*b).in_pos.wrapping_add(1);
        (*s).lzma2.compressed = ((*s).lzma2.compressed as libc::c_uint).wrapping_add(
          (*(*b).in_0.offset(fresh9 as isize) as u32).wrapping_add(1i32 as libc::c_uint),
        ) as u32 as u32;
        (*s).lzma2.sequence = (*s).lzma2.next_sequence;
        current_block_64 = 5807581744382915773;
      }
      5 => {
        let fresh10 = (*b).in_pos;
        (*b).in_pos = (*b).in_pos.wrapping_add(1);
        if !lzma_props(s, *(*b).in_0.offset(fresh10 as isize)) {
          return XZ_DATA_ERROR;
        }
        (*s).lzma2.sequence = SEQ_LZMA_PREPARE;
        current_block_64 = 8996628700919479163;
      }
      6 => {
        current_block_64 = 8996628700919479163;
      }
      7 => {
        current_block_64 = 14938516266808412833;
      }
      8 => {
        dict_uncompressed(&mut (*s).dict, b, &mut (*s).lzma2.compressed);
        if (*s).lzma2.compressed > 0i32 as libc::c_uint {
          return XZ_OK;
        }
        (*s).lzma2.sequence = SEQ_CONTROL;
        current_block_64 = 5807581744382915773;
      }
      _ => {
        current_block_64 = 5807581744382915773;
      }
    }
    match current_block_64 {
      8996628700919479163 => {
        if (*s).lzma2.compressed < 5i32 as libc::c_uint {
          return XZ_DATA_ERROR;
        }
        if !rc_read_init(&mut (*s).rc, b) {
          return XZ_OK;
        }
        (*s).lzma2.compressed = ((*s).lzma2.compressed as libc::c_uint)
          .wrapping_sub(5i32 as libc::c_uint) as u32
          as u32;
        (*s).lzma2.sequence = SEQ_LZMA_RUN;
        current_block_64 = 14938516266808412833;
      }
      _ => {}
    }
    match current_block_64 {
      14938516266808412833 => {
        /*
         * Set dictionary limit to indicate how much we want
         * to be encoded at maximum. Decode new data into the
         * dictionary. Flush the new data from dictionary to
         * b->out. Check if we finished decoding this chunk.
         * In case the dictionary got full but we didn't fill
         * the output buffer yet, we may run this loop
         * multiple times without changing s->lzma2.sequence.
         */
        dict_limit(
          &mut (*s).dict,
          if (*b).out_size.wrapping_sub((*b).out_pos) < (*s).lzma2.uncompressed as libc::c_ulong {
            (*b).out_size.wrapping_sub((*b).out_pos)
          } else {
            (*s).lzma2.uncompressed as libc::c_ulong
          },
        );
        if !lzma2_lzma(s, b) {
          return XZ_DATA_ERROR;
        }
        (*s).lzma2.uncompressed = ((*s).lzma2.uncompressed as libc::c_uint)
          .wrapping_sub(dict_flush(&mut (*s).dict, b)) as u32
          as u32;
        if (*s).lzma2.uncompressed == 0i32 as libc::c_uint {
          if (*s).lzma2.compressed > 0i32 as libc::c_uint
            || (*s).lzma.len > 0i32 as libc::c_uint
            || !rc_is_finished(&mut (*s).rc)
          {
            return XZ_DATA_ERROR;
          }
          rc_reset(&mut (*s).rc);
          (*s).lzma2.sequence = SEQ_CONTROL
        } else if (*b).out_pos == (*b).out_size
          || (*b).in_pos == (*b).in_size && (*s).temp.size < (*s).lzma2.compressed
        {
          return XZ_OK;
        }
      }
      _ => {}
    }
  }
  return XZ_OK;
}
/*
 * Private includes and definitions
 *
 * Author: Lasse Collin <lasse.collin@tukaani.org>
 *
 * This file has been put into the public domain.
 * You can do whatever you want with this file.
 */
/*
 * For userspace builds, use a separate header to define the required
 * macros and functions. This makes it easier to adapt the code into
 * different environments and avoids clutter in the Linux kernel tree.
 */
/* If no specific decoding mode is requested, enable support for all modes. */
/*
 * The DEC_IS_foo(mode) macros are used in "if" statements. If only some
 * of the supported modes are enabled, these macros will evaluate to true or
 * false at compile time and thus allow the compiler to omit unneeded code.
 */
/*
 * If any of the BCJ filter decoders are wanted, define XZ_DEC_BCJ.
 * XZ_DEC_BCJ is used to enable generic support for BCJ decoders.
 */
/*
 * Allocate memory for LZMA2 decoder. xz_dec_lzma2_reset() must be used
 * before calling xz_dec_lzma2_run().
 */
unsafe extern "C" fn xz_dec_lzma2_create(
  mut mode: xz_mode,
  mut dict_max: u32,
) -> *mut xz_dec_lzma2 {
  let mut s: *mut xz_dec_lzma2 =
    malloc(::std::mem::size_of::<xz_dec_lzma2>() as libc::c_ulong) as *mut xz_dec_lzma2;
  if s.is_null() {
    return 0 as *mut xz_dec_lzma2;
  }
  (*s).dict.mode = mode;
  (*s).dict.size_max = dict_max;
  if mode as libc::c_uint == XZ_DYNALLOC as libc::c_int as libc::c_uint {
    (*s).dict.buf = 0 as *mut u8;
    (*s).dict.allocated = 0i32 as u32
  }
  return s;
}
unsafe extern "C" fn xz_dec_lzma2_reset(mut s: *mut xz_dec_lzma2, mut props: u8) -> xz_ret {
  /* This limits dictionary size to 3 GiB to keep parsing simpler. */
  if props as libc::c_int > 39i32 {
    return XZ_OPTIONS_ERROR;
  }
  (*s).dict.size = (2i32 + (props as libc::c_int & 1i32)) as u32;
  (*s).dict.size <<= (props as libc::c_int >> 1i32) + 11i32;
  if (*s).dict.size > (*s).dict.size_max {
    return XZ_MEMLIMIT_ERROR;
  }
  (*s).dict.end = (*s).dict.size as size_t;
  if (*s).dict.mode as libc::c_uint == XZ_DYNALLOC as libc::c_int as libc::c_uint {
    if (*s).dict.allocated < (*s).dict.size {
      free((*s).dict.buf as *mut libc::c_void);
      (*s).dict.buf = malloc((*s).dict.size as libc::c_ulong) as *mut u8;
      if (*s).dict.buf.is_null() {
        (*s).dict.allocated = 0i32 as u32;
        return XZ_MEM_ERROR;
      }
    }
  }
  (*s).lzma.len = 0i32 as u32;
  (*s).lzma2.sequence = SEQ_CONTROL;
  (*s).lzma2.need_dict_reset = 1i32 != 0;
  (*s).temp.size = 0i32 as u32;
  return XZ_OK;
}
/* Free the memory allocated for the LZMA2 decoder. */
unsafe extern "C" fn xz_dec_lzma2_end(mut s: *mut xz_dec_lzma2) {
  free((*s).dict.buf as *mut libc::c_void);
  free(s as *mut libc::c_void);
}
/* *
 * xz_dec_run() - Run the XZ decoder
 * @s:          Decoder state allocated using xz_dec_init()
 * @b:          Input and output buffers
 *
 * The possible return values depend on build options and operation mode.
 * See enum xz_ret for details.
 *
 * Note that if an error occurs in single-call mode (return value is not
 * XZ_STREAM_END), b->in_pos and b->out_pos are not modified and the
 * contents of the output buffer from b->out[b->out_pos] onward are
 * undefined. This is true even after XZ_BUF_ERROR, because with some filter
 * chains, there may be a second pass over the output buffer, and this pass
 * cannot be properly done if the output buffer is truncated. Thus, you
 * cannot give the single-call decoder a too small buffer and then expect to
 * get that amount valid data from the beginning of the stream. You must use
 * the multi-call decoder if you don't want to uncompress the whole stream.
 */
/* Sizes of the Check field with different Check IDs */
/*
 * Fill s->temp by copying data starting from b->in[b->in_pos]. Caller
 * must have set s->temp.pos to indicate how much data we are supposed
 * to copy into s->temp.buf. Return true once s->temp.pos has reached
 * s->temp.size.
 */
/* Decode a variable-length integer (little-endian base-128 encoding) */
/* Don't allow non-minimal encodings. */
/*
 * Decode the Compressed Data field from a Block. Update and validate
 * the observed compressed and uncompressed sizes of the Block so that
 * they don't exceed the values possibly stored in the Block Header
 * (validation assumes that no integer overflow occurs, since vli_type
 * is normally uint64_t). Update the CRC32 if presence of the CRC32
 * field was indicated in Stream Header.
 *
 * Once the decoding is finished, validate that the observed sizes match
 * the sizes possibly stored in the Block Header. Update the hash and
 * Block count, which are later used to validate the Index field.
 */
/*
 * There is no need to separately check for VLI_UNKNOWN, since
 * the observed sizes are always smaller than VLI_UNKNOWN.
 */
/* Update the Index size and the CRC32 value. */
/*
 * Decode the Number of Records, Unpadded Size, and Uncompressed Size
 * fields from the Index field. That is, Index Padding and CRC32 are not
 * decoded by this function.
 *
 * This can return XZ_OK (more input needed), XZ_STREAM_END (everything
 * successfully decoded), or XZ_DATA_ERROR (input is corrupt).
 */
/*
 * Validate that the Number of Records field
 * indicates the same number of Records as
 * there were Blocks in the Stream.
 */
/*
 * Validate that the next four input bytes match the value of s->crc32.
 * s->pos must be zero when starting to validate the first byte.
 */
/*
 * Skip over the Check field when the Check ID is not supported.
 * Returns true once the whole Check field has been skipped over.
 */
/* Decode the Stream Header field (the first 12 bytes of the .xz Stream). */
/*
 * Of integrity checks, we support only none (Check ID = 0) and
 * CRC32 (Check ID = 1). However, if XZ_DEC_ANY_CHECK is defined,
 * we will accept other check types too, but then the check won't
 * be verified and a warning (XZ_UNSUPPORTED_CHECK) will be given.
 */
/* Decode the Stream Footer field (the last 12 bytes of the .xz Stream) */
/*
 * Validate Backward Size. Note that we never added the size of the
 * Index CRC32 field to s->index.size, thus we use s->index.size / 4
 * instead of s->index.size / 4 - 1.
 */
/*
 * Use XZ_STREAM_END instead of XZ_OK to be more convenient
 * for the caller.
 */
/* Decode the Block Header and initialize the filter chain. */
/*
 * Validate the CRC32. We know that the temp buffer is at least
 * eight bytes so this is safe.
 */
/*
 * Catch unsupported Block Flags. We support only one or two filters
 * in the chain, so we catch that with the same test.
 */
/* Compressed Size */
/* Uncompressed Size */
/* Valid Filter Flags always take at least two bytes. */
/* Filter ID = LZMA2 */
/* Size of Properties = 1-byte Filter Properties */
/* Filter Properties contains LZMA2 dictionary size. */
/* The rest must be Header Padding. */
/*
 * Store the start position for the case when we are in the middle
 * of the Index field.
 */
/*
 * Stream Header is copied to s->temp, and then
 * decoded from there. This way if the caller
 * gives us only little input at a time, we can
 * still keep the Stream Header decoding code
 * simple. Similar approach is used in many places
 * in this file.
 */
/*
 * If dec_stream_header() returns
 * XZ_UNSUPPORTED_CHECK, it is still possible
 * to continue decoding if working in multi-call
 * mode. Thus, update s->sequence before calling
 * dec_stream_header().
 */
/* We need one byte of input to continue. */
/* See if this is the beginning of the Index field. */
/*
 * Calculate the size of the Block Header and
 * prepare to decode it.
 */
/*
 * Size of Compressed Data + Block Padding
 * must be a multiple of four. We don't need
 * s->block.compressed for anything else
 * anymore, so we use it here to test the size
 * of the Block Padding field.
 */
/* Finish the CRC32 value and Index size. */
/* Compare the hashes to validate the Index field. */
/* Never reached */
/*
 * xz_dec_run() is a wrapper for dec_main() to handle some special cases in
 * multi-call and single-call decoding.
 *
 * In multi-call mode, we must return XZ_BUF_ERROR when it seems clear that we
 * are not going to make any progress anymore. This is to prevent the caller
 * from calling us infinitely when the input file is truncated or otherwise
 * corrupt. Since zlib-style API allows that the caller fills the input buffer
 * only when the decoder doesn't produce any new output, we have to be careful
 * to avoid returning XZ_BUF_ERROR too easily: XZ_BUF_ERROR is returned only
 * after the second consecutive call to xz_dec_run() that makes no progress.
 *
 * In single-call mode, if we couldn't decode everything and no error
 * occurred, either the input is truncated or the output buffer is too small.
 * Since we know that the last input byte never produces any output, we know
 * that if all the input was consumed and decoding wasn't finished, the file
 * must be corrupt. Otherwise the output buffer has to be too small or the
 * file is corrupt in a way that decoding it produces too big output.
 *
 * If single-call decoding fails, we reset b->in_pos and b->out_pos back to
 * their original values. This is because with some filter chains there won't
 * be any valid uncompressed data in the output buffer unless the decoding
 * actually succeeds (that's the price to pay of using the output buffer as
 * the workspace).
 */
unsafe extern "C" fn xz_dec_run(mut s: *mut xz_dec, mut b: *mut xz_buf) -> xz_ret {
  let mut in_start: size_t = 0;
  let mut out_start: size_t = 0;
  let mut ret: xz_ret = XZ_OK;
  in_start = (*b).in_pos;
  out_start = (*b).out_pos;
  ret = dec_main(s, b);
  if ret as libc::c_uint == XZ_OK as libc::c_int as libc::c_uint
    && in_start == (*b).in_pos
    && out_start == (*b).out_pos
  {
    if (*s).allow_buf_error {
      ret = XZ_BUF_ERROR
    }
    (*s).allow_buf_error = 1i32 != 0
  } else {
    (*s).allow_buf_error = 0i32 != 0
  }
  return ret;
}
unsafe extern "C" fn dec_index(mut s: *mut xz_dec, mut b: *mut xz_buf) -> xz_ret {
  let mut ret: xz_ret = XZ_OK;
  loop {
    ret = dec_vli(s, (*b).in_0, &mut (*b).in_pos, (*b).in_size);
    if ret as libc::c_uint != XZ_STREAM_END as libc::c_int as libc::c_uint {
      index_update(s, b);
      return ret;
    }
    match (*s).index.sequence as libc::c_uint {
      0 => {
        (*s).index.count = (*s).vli;
        if (*s).index.count != (*s).block.count {
          return XZ_DATA_ERROR;
        }
        (*s).index.sequence = SEQ_INDEX_UNPADDED
      }
      1 => {
        (*s).index.hash.unpadded = ((*s).index.hash.unpadded as libc::c_ulong)
          .wrapping_add((*s).vli) as vli_type as vli_type;
        (*s).index.sequence = SEQ_INDEX_UNCOMPRESSED
      }
      2 => {
        (*s).index.hash.uncompressed = ((*s).index.hash.uncompressed as libc::c_ulong)
          .wrapping_add((*s).vli) as vli_type as vli_type;
        (*s).index.hash.crc32 = xz_crc32(
          &mut (*s).index.hash as *mut xz_dec_hash as *const u8,
          ::std::mem::size_of::<xz_dec_hash>() as libc::c_ulong,
          (*s).index.hash.crc32,
        );
        (*s).index.count = (*s).index.count.wrapping_sub(1);
        (*s).index.sequence = SEQ_INDEX_UNPADDED
      }
      _ => {}
    }
    if !((*s).index.count > 0i32 as libc::c_ulong) {
      break;
    }
  }
  return XZ_STREAM_END;
}
unsafe extern "C" fn index_update(mut s: *mut xz_dec, mut b: *const xz_buf) {
  let mut in_used: size_t = (*b).in_pos.wrapping_sub((*s).in_start);
  (*s).index.size =
    ((*s).index.size as libc::c_ulong).wrapping_add(in_used) as vli_type as vli_type;
  (*s).crc32 = xz_crc32(
    (*b).in_0.offset((*s).in_start as isize),
    in_used,
    (*s).crc32,
  );
}
unsafe extern "C" fn crc32_validate(mut s: *mut xz_dec, mut b: *mut xz_buf) -> xz_ret {
  loop {
    if (*b).in_pos == (*b).in_size {
      return XZ_OK;
    }
    let fresh11 = (*b).in_pos;
    (*b).in_pos = (*b).in_pos.wrapping_add(1);
    if (*s).crc32 >> (*s).pos & 0xffi32 as libc::c_uint
      != *(*b).in_0.offset(fresh11 as isize) as libc::c_uint
    {
      return XZ_DATA_ERROR;
    }
    (*s).pos =
      ((*s).pos as libc::c_uint).wrapping_add(8i32 as libc::c_uint) as u32 as u32;
    if !((*s).pos < 32i32 as libc::c_uint) {
      break;
    }
  }
  (*s).crc32 = 0i32 as u32;
  (*s).pos = 0i32 as u32;
  return XZ_STREAM_END;
}
unsafe extern "C" fn dec_block_header(mut s: *mut xz_dec) -> xz_ret {
  let mut ret: xz_ret = XZ_OK;
  (*s).temp.size =
    ((*s).temp.size as libc::c_ulong).wrapping_sub(4i32 as libc::c_ulong) as size_t as size_t;
  if xz_crc32((*s).temp.buf.as_mut_ptr(), (*s).temp.size, 0i32 as u32)
    != ({
      let mut v: u32 =
        *((*s).temp.buf.as_mut_ptr().offset((*s).temp.size as isize) as *mut bb__aliased_u32);
      v
    })
  {
    return XZ_DATA_ERROR;
  }
  (*s).temp.pos = 2i32 as size_t;
  if (*s).temp.buf[1] as libc::c_int & 0x3fi32 != 0 {
    return XZ_OPTIONS_ERROR;
  }
  if (*s).temp.buf[1] as libc::c_int & 0x40i32 != 0 {
    if dec_vli(
      s,
      (*s).temp.buf.as_mut_ptr(),
      &mut (*s).temp.pos,
      (*s).temp.size,
    ) as libc::c_uint
      != XZ_STREAM_END as libc::c_int as libc::c_uint
    {
      return XZ_DATA_ERROR;
    }
    (*s).block_header.compressed = (*s).vli
  } else {
    (*s).block_header.compressed = -1i32 as vli_type
  }
  if (*s).temp.buf[1] as libc::c_int & 0x80i32 != 0 {
    if dec_vli(
      s,
      (*s).temp.buf.as_mut_ptr(),
      &mut (*s).temp.pos,
      (*s).temp.size,
    ) as libc::c_uint
      != XZ_STREAM_END as libc::c_int as libc::c_uint
    {
      return XZ_DATA_ERROR;
    }
    (*s).block_header.uncompressed = (*s).vli
  } else {
    (*s).block_header.uncompressed = -1i32 as vli_type
  }
  if (*s).temp.size.wrapping_sub((*s).temp.pos) < 2i32 as libc::c_ulong {
    return XZ_DATA_ERROR;
  }
  let fresh12 = (*s).temp.pos;
  (*s).temp.pos = (*s).temp.pos.wrapping_add(1);
  if (*s).temp.buf[fresh12 as usize] as libc::c_int != 0x21i32 {
    return XZ_OPTIONS_ERROR;
  }
  let fresh13 = (*s).temp.pos;
  (*s).temp.pos = (*s).temp.pos.wrapping_add(1);
  if (*s).temp.buf[fresh13 as usize] as libc::c_int != 0x1i32 {
    return XZ_OPTIONS_ERROR;
  }
  if (*s).temp.size.wrapping_sub((*s).temp.pos) < 1i32 as libc::c_ulong {
    return XZ_DATA_ERROR;
  }
  let fresh14 = (*s).temp.pos;
  (*s).temp.pos = (*s).temp.pos.wrapping_add(1);
  ret = xz_dec_lzma2_reset((*s).lzma2, (*s).temp.buf[fresh14 as usize]);
  if ret as libc::c_uint != XZ_OK as libc::c_int as libc::c_uint {
    return ret;
  }
  while (*s).temp.pos < (*s).temp.size {
    let fresh15 = (*s).temp.pos;
    (*s).temp.pos = (*s).temp.pos.wrapping_add(1);
    if (*s).temp.buf[fresh15 as usize] as libc::c_int != 0i32 {
      return XZ_OPTIONS_ERROR;
    }
  }
  (*s).temp.pos = 0i32 as size_t;
  (*s).block.compressed = 0i32 as vli_type;
  (*s).block.uncompressed = 0i32 as vli_type;
  return XZ_OK;
}
unsafe extern "C" fn dec_stream_header(mut s: *mut xz_dec) -> xz_ret {
  if !(memcmp(
    (*s).temp.buf.as_mut_ptr() as *const libc::c_void,
    b"\xfd7zXZ\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
    6i32 as libc::c_ulong,
  ) == 0i32)
  {
    return XZ_FORMAT_ERROR;
  }
  if xz_crc32(
    (*s).temp.buf.as_mut_ptr().offset(6),
    2i32 as size_t,
    0i32 as u32,
  ) != ({
    let mut v: u32 =
      *((*s).temp.buf.as_mut_ptr().offset(6).offset(2) as *mut bb__aliased_u32);
    v
  }) {
    return XZ_DATA_ERROR;
  }
  if (*s).temp.buf[6] as libc::c_int != 0i32 {
    return XZ_OPTIONS_ERROR;
  }
  (*s).check_type = (*s).temp.buf[(6i32 + 1i32) as usize] as xz_check;
  if (*s).check_type as libc::c_uint > 15i32 as libc::c_uint {
    return XZ_OPTIONS_ERROR;
  }
  if (*s).check_type as libc::c_uint > XZ_CHECK_CRC32 as libc::c_int as libc::c_uint {
    return XZ_UNSUPPORTED_CHECK;
  }
  return XZ_OK;
}
unsafe extern "C" fn fill_temp(mut s: *mut xz_dec, mut b: *mut xz_buf) -> bool {
  let mut copy_size: size_t =
    if (*b).in_size.wrapping_sub((*b).in_pos) < (*s).temp.size.wrapping_sub((*s).temp.pos) {
      (*b).in_size.wrapping_sub((*b).in_pos)
    } else {
      (*s).temp.size.wrapping_sub((*s).temp.pos)
    };
  memcpy(
    (*s).temp.buf.as_mut_ptr().offset((*s).temp.pos as isize) as *mut libc::c_void,
    (*b).in_0.offset((*b).in_pos as isize) as *const libc::c_void,
    copy_size,
  );
  (*b).in_pos = ((*b).in_pos as libc::c_ulong).wrapping_add(copy_size) as size_t as size_t;
  (*s).temp.pos = ((*s).temp.pos as libc::c_ulong).wrapping_add(copy_size) as size_t as size_t;
  if (*s).temp.pos == (*s).temp.size {
    (*s).temp.pos = 0i32 as size_t;
    return 1i32 != 0;
  }
  return 0i32 != 0;
}
unsafe extern "C" fn dec_stream_footer(mut s: *mut xz_dec) -> xz_ret {
  if !(memcmp(
    (*s).temp.buf.as_mut_ptr().offset(10) as *const libc::c_void,
    b"YZ\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
    2i32 as libc::c_ulong,
  ) == 0i32)
  {
    return XZ_DATA_ERROR;
  }
  if xz_crc32(
    (*s).temp.buf.as_mut_ptr().offset(4),
    6i32 as size_t,
    0i32 as u32,
  ) != ({
    let mut v: u32 = *((*s).temp.buf.as_mut_ptr() as *mut bb__aliased_u32);
    v
  }) {
    return XZ_DATA_ERROR;
  }
  if (*s).index.size >> 2i32
    != ({
      let mut v: u32 = *((*s).temp.buf.as_mut_ptr().offset(4) as *mut bb__aliased_u32);
      v
    }) as libc::c_ulong
  {
    return XZ_DATA_ERROR;
  }
  if (*s).temp.buf[8] as libc::c_int != 0i32
    || (*s).temp.buf[9] as libc::c_uint != (*s).check_type as libc::c_uint
  {
    return XZ_DATA_ERROR;
  }
  return XZ_STREAM_END;
}
unsafe extern "C" fn dec_main(mut s: *mut xz_dec, mut b: *mut xz_buf) -> xz_ret {
  let mut ret: xz_ret = XZ_OK;
  (*s).in_start = (*b).in_pos;
  loop {
    let mut current_block_65: u64;
    match (*s).sequence as libc::c_uint {
      0 => {
        if !fill_temp(s, b) {
          return XZ_OK;
        }
        (*s).sequence = SEQ_BLOCK_START;
        ret = dec_stream_header(s);
        if ret as libc::c_uint != XZ_OK as libc::c_int as libc::c_uint {
          return ret;
        }
        current_block_65 = 5837058721879980386;
      }
      1 => {
        current_block_65 = 5837058721879980386;
      }
      2 => {
        current_block_65 = 1459573402483349119;
      }
      3 => {
        current_block_65 = 12462068191749006012;
      }
      4 => {
        current_block_65 = 8206085343828517749;
      }
      5 => {
        current_block_65 = 8853100982098053779;
      }
      6 => {
        ret = dec_index(s, b);
        if ret as libc::c_uint != XZ_STREAM_END as libc::c_int as libc::c_uint {
          return ret;
        }
        (*s).sequence = SEQ_INDEX_PADDING;
        current_block_65 = 17143798186130252483;
      }
      7 => {
        current_block_65 = 17143798186130252483;
      }
      8 => {
        current_block_65 = 3115182076533074907;
      }
      9 => {
        current_block_65 = 1883937180529786320;
      }
      _ => {
        current_block_65 = 15594603006322722090;
      }
    }
    match current_block_65 {
      17143798186130252483 => {
        while (*s)
          .index
          .size
          .wrapping_add((*b).in_pos.wrapping_sub((*s).in_start))
          & 3i32 as libc::c_ulong
          != 0
        {
          if (*b).in_pos == (*b).in_size {
            index_update(s, b);
            return XZ_OK;
          }
          let fresh18 = (*b).in_pos;
          (*b).in_pos = (*b).in_pos.wrapping_add(1);
          if *(*b).in_0.offset(fresh18 as isize) as libc::c_int != 0i32 {
            return XZ_DATA_ERROR;
          }
        }
        index_update(s, b);
        if !(memcmp(
          &mut (*s).block.hash as *mut xz_dec_hash as *const libc::c_void,
          &mut (*s).index.hash as *mut xz_dec_hash as *const libc::c_void,
          ::std::mem::size_of::<xz_dec_hash>() as libc::c_ulong,
        ) == 0i32)
        {
          return XZ_DATA_ERROR;
        }
        (*s).sequence = SEQ_INDEX_CRC32;
        current_block_65 = 3115182076533074907;
      }
      5837058721879980386 => {
        if (*b).in_pos == (*b).in_size {
          return XZ_OK;
        }
        if *(*b).in_0.offset((*b).in_pos as isize) as libc::c_int == 0i32 {
          let fresh16 = (*b).in_pos;
          (*b).in_pos = (*b).in_pos.wrapping_add(1);
          (*s).in_start = fresh16;
          (*s).sequence = SEQ_INDEX;
          current_block_65 = 15594603006322722090;
        } else {
          (*s).block_header.size = (*(*b).in_0.offset((*b).in_pos as isize) as u32)
            .wrapping_add(1i32 as libc::c_uint)
            .wrapping_mul(4i32 as libc::c_uint);
          (*s).temp.size = (*s).block_header.size as size_t;
          (*s).temp.pos = 0i32 as size_t;
          (*s).sequence = SEQ_BLOCK_HEADER;
          current_block_65 = 1459573402483349119;
        }
      }
      _ => {}
    }
    match current_block_65 {
      3115182076533074907 => {
        ret = crc32_validate(s, b);
        if ret as libc::c_uint != XZ_STREAM_END as libc::c_int as libc::c_uint {
          return ret;
        }
        (*s).temp.size = 12i32 as size_t;
        (*s).sequence = SEQ_STREAM_FOOTER;
        current_block_65 = 1883937180529786320;
      }
      1459573402483349119 => {
        if !fill_temp(s, b) {
          return XZ_OK;
        }
        ret = dec_block_header(s);
        if ret as libc::c_uint != XZ_OK as libc::c_int as libc::c_uint {
          return ret;
        }
        (*s).sequence = SEQ_BLOCK_UNCOMPRESS;
        current_block_65 = 12462068191749006012;
      }
      _ => {}
    }
    match current_block_65 {
      1883937180529786320 => {
        if !fill_temp(s, b) {
          return XZ_OK;
        }
        return dec_stream_footer(s);
      }
      12462068191749006012 => {
        ret = dec_block(s, b);
        if ret as libc::c_uint != XZ_STREAM_END as libc::c_int as libc::c_uint {
          return ret;
        }
        (*s).sequence = SEQ_BLOCK_PADDING;
        current_block_65 = 8206085343828517749;
      }
      _ => {}
    }
    match current_block_65 {
      8206085343828517749 => {
        while (*s).block.compressed & 3i32 as libc::c_ulong != 0 {
          if (*b).in_pos == (*b).in_size {
            return XZ_OK;
          }
          let fresh17 = (*b).in_pos;
          (*b).in_pos = (*b).in_pos.wrapping_add(1);
          if *(*b).in_0.offset(fresh17 as isize) as libc::c_int != 0i32 {
            return XZ_DATA_ERROR;
          }
          (*s).block.compressed = (*s).block.compressed.wrapping_add(1)
        }
        (*s).sequence = SEQ_BLOCK_CHECK;
        current_block_65 = 8853100982098053779;
      }
      _ => {}
    }
    match current_block_65 {
      8853100982098053779 => {
        if (*s).check_type as libc::c_uint == XZ_CHECK_CRC32 as libc::c_int as libc::c_uint {
          ret = crc32_validate(s, b);
          if ret as libc::c_uint != XZ_STREAM_END as libc::c_int as libc::c_uint {
            return ret;
          }
        } else if !check_skip(s, b) {
          return XZ_OK;
        }
        (*s).sequence = SEQ_BLOCK_START
      }
      _ => {}
    }
  }
}
unsafe extern "C" fn dec_block(mut s: *mut xz_dec, mut b: *mut xz_buf) -> xz_ret {
  let mut ret: xz_ret = XZ_OK;
  (*s).in_start = (*b).in_pos;
  (*s).out_start = (*b).out_pos;
  ret = xz_dec_lzma2_run((*s).lzma2, b);
  (*s).block.compressed = ((*s).block.compressed as libc::c_ulong)
    .wrapping_add((*b).in_pos.wrapping_sub((*s).in_start)) as vli_type
    as vli_type;
  (*s).block.uncompressed = ((*s).block.uncompressed as libc::c_ulong)
    .wrapping_add((*b).out_pos.wrapping_sub((*s).out_start)) as vli_type
    as vli_type;
  if (*s).block.compressed > (*s).block_header.compressed
    || (*s).block.uncompressed > (*s).block_header.uncompressed
  {
    return XZ_DATA_ERROR;
  }
  if (*s).check_type as libc::c_uint == XZ_CHECK_CRC32 as libc::c_int as libc::c_uint {
    (*s).crc32 = xz_crc32(
      (*b).out.offset((*s).out_start as isize),
      (*b).out_pos.wrapping_sub((*s).out_start),
      (*s).crc32,
    )
  }
  if ret as libc::c_uint == XZ_STREAM_END as libc::c_int as libc::c_uint {
    if (*s).block_header.compressed != -1i32 as vli_type
      && (*s).block_header.compressed != (*s).block.compressed
    {
      return XZ_DATA_ERROR;
    }
    if (*s).block_header.uncompressed != -1i32 as vli_type
      && (*s).block_header.uncompressed != (*s).block.uncompressed
    {
      return XZ_DATA_ERROR;
    }
    (*s).block.hash.unpadded = ((*s).block.hash.unpadded as libc::c_ulong)
      .wrapping_add(((*s).block_header.size as libc::c_ulong).wrapping_add((*s).block.compressed))
      as vli_type as vli_type;
    (*s).block.hash.unpadded = ((*s).block.hash.unpadded as libc::c_ulong)
      .wrapping_add(check_sizes[(*s).check_type as usize] as libc::c_ulong)
      as vli_type as vli_type;
    (*s).block.hash.uncompressed = ((*s).block.hash.uncompressed as libc::c_ulong)
      .wrapping_add((*s).block.uncompressed) as vli_type
      as vli_type;
    (*s).block.hash.crc32 = xz_crc32(
      &mut (*s).block.hash as *mut xz_dec_hash as *const u8,
      ::std::mem::size_of::<xz_dec_hash>() as libc::c_ulong,
      (*s).block.hash.crc32,
    );
    (*s).block.count = (*s).block.count.wrapping_add(1)
  }
  return ret;
}
static mut check_sizes: [u8; 16] = [
  0i32 as u8,
  4i32 as u8,
  4i32 as u8,
  4i32 as u8,
  8i32 as u8,
  8i32 as u8,
  8i32 as u8,
  16i32 as u8,
  16i32 as u8,
  16i32 as u8,
  32i32 as u8,
  32i32 as u8,
  32i32 as u8,
  64i32 as u8,
  64i32 as u8,
  64i32 as u8,
];
unsafe extern "C" fn dec_vli(
  mut s: *mut xz_dec,
  mut in_0: *const u8,
  mut in_pos: *mut size_t,
  mut in_size: size_t,
) -> xz_ret {
  let mut byte: u8 = 0;
  if (*s).pos == 0i32 as libc::c_uint {
    (*s).vli = 0i32 as vli_type
  }
  while *in_pos < in_size {
    byte = *in_0.offset(*in_pos as isize);
    *in_pos = (*in_pos).wrapping_add(1);
    (*s).vli |= ((byte as libc::c_int & 0x7fi32) as vli_type) << (*s).pos;
    if byte as libc::c_int & 0x80i32 == 0i32 {
      if byte as libc::c_int == 0i32 && (*s).pos != 0i32 as libc::c_uint {
        return XZ_DATA_ERROR;
      }
      (*s).pos = 0i32 as u32;
      return XZ_STREAM_END;
    }
    (*s).pos =
      ((*s).pos as libc::c_uint).wrapping_add(7i32 as libc::c_uint) as u32 as u32;
    if (*s).pos as libc::c_ulong
      == (7i32 as libc::c_ulong).wrapping_mul(
        (::std::mem::size_of::<vli_type>() as libc::c_ulong)
          .wrapping_mul(8i32 as libc::c_ulong)
          .wrapping_div(7i32 as libc::c_ulong),
      )
    {
      return XZ_DATA_ERROR;
    }
  }
  return XZ_OK;
}
unsafe extern "C" fn check_skip(mut s: *mut xz_dec, mut b: *mut xz_buf) -> bool {
  while (*s).pos < check_sizes[(*s).check_type as usize] as libc::c_uint {
    if (*b).in_pos == (*b).in_size {
      return 0i32 != 0;
    }
    (*b).in_pos = (*b).in_pos.wrapping_add(1);
    (*s).pos = (*s).pos.wrapping_add(1)
  }
  (*s).pos = 0i32 as u32;
  return 1i32 != 0;
}
/* *
 * enum xz_ret - Return codes
 * @XZ_OK:                  Everything is OK so far. More input or more
 *                          output space is required to continue. This
 *                          return code is possible only in multi-call mode
 *                          (XZ_PREALLOC or XZ_DYNALLOC).
 * @XZ_STREAM_END:          Operation finished successfully.
 * @XZ_UNSUPPORTED_CHECK:   Integrity check type is not supported. Decoding
 *                          is still possible in multi-call mode by simply
 *                          calling xz_dec_run() again.
 *                          Note that this return value is used only if
 *                          XZ_DEC_ANY_CHECK was defined at build time,
 *                          which is not used in the kernel. Unsupported
 *                          check types return XZ_OPTIONS_ERROR if
 *                          XZ_DEC_ANY_CHECK was not defined at build time.
 * @XZ_MEM_ERROR:           Allocating memory failed. This return code is
 *                          possible only if the decoder was initialized
 *                          with XZ_DYNALLOC. The amount of memory that was
 *                          tried to be allocated was no more than the
 *                          dict_max argument given to xz_dec_init().
 * @XZ_MEMLIMIT_ERROR:      A bigger LZMA2 dictionary would be needed than
 *                          allowed by the dict_max argument given to
 *                          xz_dec_init(). This return value is possible
 *                          only in multi-call mode (XZ_PREALLOC or
 *                          XZ_DYNALLOC); the single-call mode (XZ_SINGLE)
 *                          ignores the dict_max argument.
 * @XZ_FORMAT_ERROR:        File format was not recognized (wrong magic
 *                          bytes).
 * @XZ_OPTIONS_ERROR:       This implementation doesn't support the requested
 *                          compression options. In the decoder this means
 *                          that the header CRC32 matches, but the header
 *                          itself specifies something that we don't support.
 * @XZ_DATA_ERROR:          Compressed data is corrupt.
 * @XZ_BUF_ERROR:           Cannot make any progress. Details are slightly
 *                          different between multi-call and single-call
 *                          mode; more information below.
 *
 * In multi-call mode, XZ_BUF_ERROR is returned when two consecutive calls
 * to XZ code cannot consume any input and cannot produce any new output.
 * This happens when there is no new input available, or the output buffer
 * is full while at least one output byte is still pending. Assuming your
 * code is not buggy, you can get this error only when decoding a compressed
 * stream that is truncated or otherwise corrupt.
 *
 * In single-call mode, XZ_BUF_ERROR is returned only when the output buffer
 * is too small or the compressed input is corrupt in a way that makes the
 * decoder produce more output than the caller expected. When it is
 * (relatively) clear that the compressed input is truncated, XZ_DATA_ERROR
 * is used instead of XZ_BUF_ERROR.
 */
/* *
 * struct xz_buf - Passing input and output buffers to XZ code
 * @in:         Beginning of the input buffer. This may be NULL if and only
 *              if in_pos is equal to in_size.
 * @in_pos:     Current position in the input buffer. This must not exceed
 *              in_size.
 * @in_size:    Size of the input buffer
 * @out:        Beginning of the output buffer. This may be NULL if and only
 *              if out_pos is equal to out_size.
 * @out_pos:    Current position in the output buffer. This must not exceed
 *              out_size.
 * @out_size:   Size of the output buffer
 *
 * Only the contents of the output buffer from out[out_pos] onward, and
 * the variables in_pos and out_pos are modified by the XZ code.
 */
/* *
 * struct xz_dec - Opaque type to hold the XZ decoder state
 */
/* *
 * xz_dec_init() - Allocate and initialize a XZ decoder state
 * @mode:       Operation mode
 * @dict_max:   Maximum size of the LZMA2 dictionary (history buffer) for
 *              multi-call decoding. This is ignored in single-call mode
 *              (mode == XZ_SINGLE). LZMA2 dictionary is always 2^n bytes
 *              or 2^n + 2^(n-1) bytes (the latter sizes are less common
 *              in practice), so other values for dict_max don't make sense.
 *              In the kernel, dictionary sizes of 64 KiB, 128 KiB, 256 KiB,
 *              512 KiB, and 1 MiB are probably the only reasonable values,
 *              except for kernel and initramfs images where a bigger
 *              dictionary can be fine and useful.
 *
 * Single-call mode (XZ_SINGLE): xz_dec_run() decodes the whole stream at
 * once. The caller must provide enough output space or the decoding will
 * fail. The output space is used as the dictionary buffer, which is why
 * there is no need to allocate the dictionary as part of the decoder's
 * internal state.
 *
 * Because the output buffer is used as the workspace, streams encoded using
 * a big dictionary are not a problem in single-call mode. It is enough that
 * the output buffer is big enough to hold the actual uncompressed data; it
 * can be smaller than the dictionary size stored in the stream headers.
 *
 * Multi-call mode with preallocated dictionary (XZ_PREALLOC): dict_max bytes
 * of memory is preallocated for the LZMA2 dictionary. This way there is no
 * risk that xz_dec_run() could run out of memory, since xz_dec_run() will
 * never allocate any memory. Instead, if the preallocated dictionary is too
 * small for decoding the given input stream, xz_dec_run() will return
 * XZ_MEMLIMIT_ERROR. Thus, it is important to know what kind of data will be
 * decoded to avoid allocating excessive amount of memory for the dictionary.
 *
 * Multi-call mode with dynamically allocated dictionary (XZ_DYNALLOC):
 * dict_max specifies the maximum allowed dictionary size that xz_dec_run()
 * may allocate once it has parsed the dictionary size from the stream
 * headers. This way excessive allocations can be avoided while still
 * limiting the maximum memory usage to a sane value to prevent running the
 * system out of memory when decompressing streams from untrusted sources.
 *
 * On success, xz_dec_init() returns a pointer to struct xz_dec, which is
 * ready to be used with xz_dec_run(). If memory allocation fails,
 * xz_dec_init() returns NULL.
 */
unsafe extern "C" fn xz_dec_init(mut mode: xz_mode, mut dict_max: u32) -> *mut xz_dec {
  let mut s: *mut xz_dec = malloc(::std::mem::size_of::<xz_dec>() as libc::c_ulong) as *mut xz_dec;
  if s.is_null() {
    return 0 as *mut xz_dec;
  }
  (*s).mode = mode;
  (*s).lzma2 = xz_dec_lzma2_create(mode, dict_max);
  if (*s).lzma2.is_null() {
    free(s as *mut libc::c_void);
    return 0 as *mut xz_dec;
  } else {
    xz_dec_reset(s);
    return s;
  };
}
/* *
 * xz_dec_reset() - Reset an already allocated decoder state
 * @s:          Decoder state allocated using xz_dec_init()
 *
 * This function can be used to reset the multi-call decoder state without
 * freeing and reallocating memory with xz_dec_end() and xz_dec_init().
 *
 * In single-call mode, xz_dec_reset() is always called in the beginning of
 * xz_dec_run(). Thus, explicit call to xz_dec_reset() is useful only in
 * multi-call mode.
 */
unsafe extern "C" fn xz_dec_reset(mut s: *mut xz_dec) {
  (*s).sequence = SEQ_STREAM_HEADER;
  (*s).allow_buf_error = 0i32 != 0;
  (*s).pos = 0i32 as u32;
  (*s).crc32 = 0i32 as u32;
  memset(
    &mut (*s).block as *mut C2RustUnnamed_4 as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong,
  );
  memset(
    &mut (*s).index as *mut C2RustUnnamed_2 as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong,
  );
  (*s).temp.pos = 0i32 as size_t;
  (*s).temp.size = 12i32 as size_t;
}
/* *
 * xz_dec_end() - Free the memory allocated for the decoder state
 * @s:          Decoder state allocated using xz_dec_init(). If s is NULL,
 *              this function does nothing.
 */
unsafe extern "C" fn xz_dec_end(mut s: *mut xz_dec) {
  if !s.is_null() {
    xz_dec_lzma2_end((*s).lzma2);
    free(s as *mut libc::c_void);
  };
}
/* We use arch-optimized unaligned fixed-endian accessors.
 * They have been moved to libbb (proved to be useful elsewhere as well),
 * just check that we have them defined:
 */
#[no_mangle]
pub unsafe extern "C" fn unpack_xz_stream(
  mut xstate: *mut transformer_state_t,
) -> libc::c_longlong {
  let mut xz_result: xz_ret = XZ_OK; /* else: let xz code read & check it */
  let mut iobuf: xz_buf = xz_buf {
    in_0: 0 as *const u8,
    in_pos: 0,
    in_size: 0,
    out: 0 as *mut u8,
    out_pos: 0,
    out_size: 0,
  };
  let mut state: *mut xz_dec = 0 as *mut xz_dec;
  let mut membuf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
  let mut total: libc::c_longlong = 0i32 as libc::c_longlong;
  if global_crc32_table.is_null() {
    global_crc32_new_table_le();
  }
  memset(
    &mut iobuf as *mut xz_buf as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<xz_buf>() as libc::c_ulong,
  );
  membuf = xmalloc((2i32 * 8192i32) as size_t) as *mut libc::c_uchar;
  iobuf.in_0 = membuf;
  iobuf.out = membuf.offset(8192);
  iobuf.out_size = 8192i32 as size_t;
  if xstate.is_null() || (*xstate).signature_skipped as libc::c_int != 0 {
    /* Preload XZ file signature */
    strcpy(
      membuf as *mut libc::c_char,
      b"\xfd7zXZ\x00" as *const u8 as *const libc::c_char,
    );
    iobuf.in_size = 6i32 as size_t
  }
  /* Limit memory usage to about 64 MiB. */
  state = xz_dec_init(XZ_DYNALLOC, (64i32 * 1024i32 * 1024i32) as u32);
  xz_result = XZ_STREAM_END;
  's_75: loop
  /*
   * Can just "break;" here, if not for concatenated
   * .xz streams.
   * Checking for padding may require buffer
   * replenishment. Can't do it here.
   */
  {
    if iobuf.in_pos == iobuf.in_size {
      let mut rd: libc::c_int = safe_read(
        (*xstate).src_fd,
        membuf as *mut libc::c_void,
        8192i32 as size_t,
      ) as libc::c_int;
      if rd < 0i32 {
        bb_simple_error_msg(b"read error\x00" as *const u8 as *const libc::c_char);
        total = -1i32 as libc::c_longlong;
        break;
      } else {
        if rd == 0i32 && xz_result as libc::c_uint == XZ_STREAM_END as libc::c_int as libc::c_uint {
          break;
        }
        iobuf.in_size = rd as size_t;
        iobuf.in_pos = 0i32 as size_t
      }
    }
    if xz_result as libc::c_uint == XZ_STREAM_END as libc::c_int as libc::c_uint {
      loop
      /*
       * Try to start decoding next concatenated stream.
       * Stream padding must always be a multiple of four
       * bytes to preserve four-byte alignment. To keep the
       * code slightly smaller, we aren't as strict here as
       * the .xz spec requires. We just skip all zero-bytes
       * without checking the alignment and thus can accept
       * files that aren't valid, e.g. the XZ utils test
       * files bad-0pad-empty.xz and bad-0catpad-empty.xz.
       */
      {
        if *membuf.offset(iobuf.in_pos as isize) as libc::c_int != 0i32 {
          /* There is more data, but is it XZ data?
           * Example: dpkg-deb -f busybox_1.30.1-4_amd64.deb
           * reads control.tar.xz "control" file
           * inside the ar archive, but tar.xz
           * extraction code reaches end of xz data,
           * reached this code and reads the beginning
           * of data.tar.xz's ar header, which isn't xz data,
           * and prints "corrupted data".
           * The correct solution is to not read
           * past nested archive (to simulate EOF).
           * This is a workaround:
           */
          if *membuf.offset(iobuf.in_pos as isize) as libc::c_int != 0xfdi32 {
            break 's_75;
          }
          xz_dec_reset(state);
          break;
        } else {
          iobuf.in_pos = iobuf.in_pos.wrapping_add(1);
          if !(iobuf.in_pos < iobuf.in_size) {
            break;
          }
        }
      }
    }
    //		bb_error_msg(">in pos:%d size:%d out pos:%d size:%d",
    //				iobuf.in_pos, iobuf.in_size, iobuf.out_pos, iobuf.out_size);
    xz_result = xz_dec_run(state, &mut iobuf);
    //		bb_error_msg("<in pos:%d size:%d out pos:%d size:%d r:%d",
    //				iobuf.in_pos, iobuf.in_size, iobuf.out_pos, iobuf.out_size, xz_result);
    if iobuf.out_pos != 0 {
      xtransformer_write(xstate, iobuf.out as *const libc::c_void, iobuf.out_pos);
      total = (total as libc::c_ulonglong).wrapping_add(iobuf.out_pos as libc::c_ulonglong)
        as libc::c_longlong as libc::c_longlong;
      iobuf.out_pos = 0i32 as size_t
    }
    if xz_result as libc::c_uint == XZ_STREAM_END as libc::c_int as libc::c_uint {
      continue;
    }
    if !(xz_result as libc::c_uint != XZ_OK as libc::c_int as libc::c_uint
      && xz_result as libc::c_uint != XZ_UNSUPPORTED_CHECK as libc::c_int as libc::c_uint)
    {
      continue;
    }
    bb_simple_error_msg(b"corrupted data\x00" as *const u8 as *const libc::c_char);
    total = -1i32 as libc::c_longlong;
    break;
  }
  /* It's definitely not a xz signature
   * (which is 0xfd,"7zXZ",0x00).
   */
  xz_dec_end(state);
  free(membuf as *mut libc::c_void);
  return total;
}

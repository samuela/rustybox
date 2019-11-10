use crate::librb::size_t;
use crate::librb::smallint;
use libc;




































































use libc::ssize_t;

extern "C" {
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static ptr_to_globals: *mut globals;
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
  static mut global_crc32_table: *mut u32;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn xwrite(fd: libc::c_int, buf: *const libc::c_void, count: size_t);
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> u32;
  #[no_mangle]
  fn gunzip_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub l_buf: *mut uch,
  pub d_buf: *mut ush,
  pub outbuf: *mut uch,
  pub window: *mut uch,
  pub prev: *mut ush,
  pub crc: u32,
  pub block_start: lng,
  pub ins_h: libc::c_uint,
  pub prev_length: libc::c_uint,
  pub strstart: libc::c_uint,
  pub match_start: libc::c_uint,
  pub lookahead: libc::c_uint,
  pub isize_0: ulg,
  pub outcnt: libc::c_uint,
  pub eofile: smallint,
  pub bi_buf: libc::c_uint,
  pub bi_valid: libc::c_uint,
}
pub type ulg = u32;
pub type lng = i32;
pub type ush = u16;
pub type uch = u8;
use crate::archival::libarchive::bb_archive::transformer_state_t;

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
pub type Pos = ush;
pub type IPos = libc::c_uint;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const nice_match: C2RustUnnamed_1 = 128;
pub const good_match: C2RustUnnamed_1 = 8;
pub const max_insert_length: C2RustUnnamed_1 = 16;
pub const max_lazy_match: C2RustUnnamed_1 = 16;
pub const max_chain_length: C2RustUnnamed_1 = 128;
pub const comp_level_minus4: C2RustUnnamed_1 = 2;
pub const WINDOW_SIZE: C2RustUnnamed_1 = 65536;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ct_data {
  pub fc: C2RustUnnamed_3,
  pub dl: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
  pub dad: ush,
  pub len: ush,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
  pub freq: ush,
  pub code: ush,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tree_desc {
  pub dyn_tree: *mut ct_data,
  pub static_tree: *mut ct_data,
  pub extra_bits: *const u8,
  pub extra_base: libc::c_int,
  pub elems: libc::c_int,
  pub max_length: libc::c_int,
  pub max_code: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals2 {
  pub heap: [ush; 573],
  pub heap_len: libc::c_int,
  pub heap_max: libc::c_int,
  pub dyn_ltree: [ct_data; 573],
  pub dyn_dtree: [ct_data; 61],
  pub static_ltree: [ct_data; 288],
  pub static_dtree: [ct_data; 30],
  pub bl_tree: [ct_data; 39],
  pub l_desc: tree_desc,
  pub d_desc: tree_desc,
  pub bl_desc: tree_desc,
  pub bl_count: [libc::c_uint; 16],
  pub depth: [uch; 573],
  pub length_code: [uch; 256],
  pub dist_code: [uch; 512],
  pub base_length: [libc::c_int; 29],
  pub base_dist: [libc::c_int; 30],
  pub flag_buf: [uch; 1024],
  pub last_lit: libc::c_uint,
  pub last_dist: libc::c_uint,
  pub last_flags: libc::c_uint,
  pub flags: uch,
  pub flag_bit: uch,
  pub opt_len: ulg,
  pub static_len: ulg,
  /* bit length of current block with static trees */
  //	ulg compressed_len;      /* total bit length of compressed file */
}
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
/* ===========================================================================
 * Write the output buffer outbuf[0..outcnt-1] and update bytes_out.
 * (used for the compressed data only)
 */
unsafe extern "C" fn flush_outbuf() {
  if (*ptr_to_globals.offset(-1)).outcnt == 0i32 as libc::c_uint {
    return;
  }
  xwrite(
    1i32,
    (*ptr_to_globals.offset(-1)).outbuf as *mut libc::c_char as *const libc::c_void,
    (*ptr_to_globals.offset(-1)).outcnt as size_t,
  );
  (*ptr_to_globals.offset(-1)).outcnt = 0i32 as libc::c_uint;
}
/* ===========================================================================
 */
/* put_8bit is used for the compressed output */
/* Output a 16 bit value, lsb first */
unsafe extern "C" fn put_16bit(mut w: ush) {
  /* GCC 4.2.1 won't optimize out redundant loads of G1.outcnt
   * (probably because of fear of aliasing with G1.outbuf[]
   * stores), do it explicitly:
   */
  let mut outcnt: libc::c_uint = (*ptr_to_globals.offset(-1)).outcnt;
  let mut dst: *mut uch =
    &mut *(*ptr_to_globals.offset(-1)).outbuf.offset(outcnt as isize) as *mut uch;
  if outcnt < (8192i32 - 2i32) as libc::c_uint {
    /* Common case */
    let mut dst16: *mut ush = dst as *mut libc::c_void as *mut ush; /* unaligned LSB 16-bit store */
    *dst16 = w;
    (*ptr_to_globals.offset(-1)).outcnt = outcnt.wrapping_add(2i32 as libc::c_uint);
    return;
  }
  *dst = w as uch;
  w = (w as libc::c_int >> 8i32) as ush;
  outcnt = outcnt.wrapping_add(1);
  (*ptr_to_globals.offset(-1)).outcnt = outcnt;
  /* Slowpath: we will need to do flush_outbuf() */
  if outcnt == 8192i32 as libc::c_uint {
    flush_outbuf(); /* here */
  }
  let ref mut fresh0 = (*ptr_to_globals.offset(-1)).outcnt;
  let fresh1 = *fresh0;
  *fresh0 = (*fresh0).wrapping_add(1);
  *(*ptr_to_globals.offset(-1)).outbuf.offset(fresh1 as isize) = w as uch;
  if (*ptr_to_globals.offset(-1)).outcnt == 8192i32 as libc::c_uint {
    flush_outbuf();
  };
  /* or here */
}
unsafe extern "C" fn put_32bit(mut n: ulg) {
  if 0i32 > 0i32 && 1i32 != 0 && 1i32 != 0 {
    let mut outcnt: libc::c_uint = (*ptr_to_globals.offset(-1)).outcnt;
    if outcnt < (8192i32 - 4i32) as libc::c_uint {
      /* Common case */
      let mut dst32: *mut ulg = &mut *(*ptr_to_globals.offset(-1)).outbuf.offset(outcnt as isize)
        as *mut uch as *mut libc::c_void as *mut ulg; /* unaligned LSB 32-bit store */
      *dst32 = n;
      //bb_error_msg("%p", dst32); // store alignment debugging
      (*ptr_to_globals.offset(-1)).outcnt = outcnt.wrapping_add(4i32 as libc::c_uint);
      return;
    }
  }
  put_16bit(n as ush);
  put_16bit((n >> 16i32) as ush);
}
#[inline(always)]
unsafe extern "C" fn flush_outbuf_if_32bit_optimized() {
  /* If put_32bit() performs 32bit stores && it is used in send_bits() */
  if 0i32 > 0i32
    && 1i32 != 0
    && 1i32 != 0
    && (8i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
      as libc::c_int
      > 16i32
  {
    flush_outbuf();
  };
}
/* ===========================================================================
 * Run a set of bytes through the crc shift register.  If s is a NULL
 * pointer, then initialize the crc shift register contents instead.
 * Return the current crc in either case.
 */
unsafe extern "C" fn updcrc(mut s: *mut uch, mut n: libc::c_uint) {
  (*ptr_to_globals.offset(-1)).crc = crc32_block_endian0(
    (*ptr_to_globals.offset(-1)).crc,
    s as *const libc::c_void,
    n,
    global_crc32_table,
  );
}
/* ===========================================================================
 * Read a new buffer from the current input file, perform end-of-line
 * translation, and update the crc and input file size.
 * IN assertion: size >= 2 (for end-of-line translation)
 */
unsafe extern "C" fn file_read(mut buf: *mut libc::c_void, mut size: libc::c_uint) -> libc::c_uint {
  let mut len: libc::c_uint = 0;
  len = safe_read(0i32, buf, size as size_t) as libc::c_uint;
  if len == -1i32 as libc::c_uint || len == 0i32 as libc::c_uint {
    return len;
  }
  updcrc(buf as *mut uch, len);
  let ref mut fresh2 = (*ptr_to_globals.offset(-1)).isize_0;
  *fresh2 = (*fresh2 as libc::c_uint).wrapping_add(len) as ulg as ulg;
  return len;
}
/* ===========================================================================
 * Send a value on a given number of bits.
 * IN assertion: length <= 16 and value fits in length bits.
 */
unsafe extern "C" fn send_bits(mut value: libc::c_uint, mut length: libc::c_uint) {
  let mut new_buf: libc::c_uint = 0;
  new_buf = (*ptr_to_globals.offset(-1)).bi_buf | value << (*ptr_to_globals.offset(-1)).bi_valid;
  /* NB: the above may sometimes do "<< 32" shift (undefined)
   * if check below is changed to "length > BUF_SIZE" instead of >= */
  length = length.wrapping_add((*ptr_to_globals.offset(-1)).bi_valid);
  /* If bi_buf is full */
  if length
    >= (8i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
      as libc::c_int as libc::c_uint
  {
    /* ...use (valid) bits from bi_buf and
     * (BUF_SIZE - bi_valid) bits from value,
     *  leaving (width - (BUF_SIZE-bi_valid)) unused bits in value.
     */
    value >>= ((8i32 as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
      as libc::c_int as libc::c_uint)
      .wrapping_sub((*ptr_to_globals.offset(-1)).bi_valid); /* 16 */
    if (8i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
      as libc::c_int
      == 32i32
    {
      put_32bit(new_buf);
    } else {
      put_16bit(new_buf as ush);
    }
    new_buf = value;
    length = length.wrapping_sub(
      (8i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
        as libc::c_int as libc::c_uint,
    )
  }
  (*ptr_to_globals.offset(-1)).bi_buf = new_buf;
  (*ptr_to_globals.offset(-1)).bi_valid = length;
}
/* ===========================================================================
 * Reverse the first len bits of a code, using straightforward code (a faster
 * method would use a table)
 * IN assertion: 1 <= len <= 15
 */
unsafe extern "C" fn bi_reverse(mut code: libc::c_uint, mut len: libc::c_int) -> libc::c_uint {
  let mut res: libc::c_uint = 0i32 as libc::c_uint;
  loop {
    res |= code & 1i32 as libc::c_uint;
    len -= 1;
    if len <= 0i32 {
      return res;
    }
    code >>= 1i32;
    res <<= 1i32
  }
}
/* ===========================================================================
 * Write out any remaining bits in an incomplete byte.
 */
unsafe extern "C" fn bi_windup() {
  let mut bits: libc::c_uint = (*ptr_to_globals.offset(-1)).bi_buf;
  let mut cnt: libc::c_int = (*ptr_to_globals.offset(-1)).bi_valid as libc::c_int;
  while cnt > 0i32 {
    let ref mut fresh3 = (*ptr_to_globals.offset(-1)).outcnt;
    let fresh4 = *fresh3;
    *fresh3 = (*fresh3).wrapping_add(1);
    *(*ptr_to_globals.offset(-1)).outbuf.offset(fresh4 as isize) = bits as uch;
    if (*ptr_to_globals.offset(-1)).outcnt == 8192i32 as libc::c_uint {
      flush_outbuf();
    }
    bits >>= 8i32;
    cnt -= 8i32
  }
  (*ptr_to_globals.offset(-1)).bi_buf = 0i32 as libc::c_uint;
  (*ptr_to_globals.offset(-1)).bi_valid = 0i32 as libc::c_uint;
}
/* ===========================================================================
 * Copy a stored block to the zip file, storing first the length and its
 * one's complement if requested.
 */
unsafe extern "C" fn copy_block(
  mut buf: *const libc::c_char,
  mut len: libc::c_uint,
  mut header: libc::c_int,
) {
  bi_windup(); /* align on byte boundary */
  if header != 0 {
    let mut v: libc::c_uint = len as u16 as libc::c_uint | !len << 16i32;
    put_32bit(v);
  }
  loop {
    let fresh5 = len;
    len = len.wrapping_sub(1);
    if !(fresh5 != 0) {
      break;
    }
    let fresh6 = buf;
    buf = buf.offset(1);
    let ref mut fresh7 = (*ptr_to_globals.offset(-1)).outcnt;
    let fresh8 = *fresh7;
    *fresh7 = (*fresh7).wrapping_add(1);
    *(*ptr_to_globals.offset(-1)).outbuf.offset(fresh8 as isize) = *fresh6 as uch;
    if (*ptr_to_globals.offset(-1)).outcnt == 8192i32 as libc::c_uint {
      flush_outbuf();
    }
  }
  /* The above can 32-bit misalign outbuf */
  if (*ptr_to_globals.offset(-1)).outcnt & 3i32 as libc::c_uint != 0 {
    /* syscalls are expensive, is it really misaligned? */
    flush_outbuf_if_32bit_optimized();
  };
}
/* ===========================================================================
 * Fill the window when the lookahead becomes insufficient.
 * Updates strstart and lookahead, and sets eofile if end of input file.
 * IN assertion: lookahead < MIN_LOOKAHEAD && strstart + lookahead > 0
 * OUT assertions: at least one byte has been read, or eofile is set;
 *    file reads are performed for at least two bytes (required for the
 *    translate_eol option).
 */
unsafe extern "C" fn fill_window() {
  let mut n: libc::c_uint = 0;
  let mut m: libc::c_uint = 0;
  let mut more: libc::c_uint = (WINDOW_SIZE as libc::c_int as libc::c_uint)
    .wrapping_sub((*ptr_to_globals.offset(-1)).lookahead)
    .wrapping_sub((*ptr_to_globals.offset(-1)).strstart);
  /* Amount of free space at the end of the window. */
  /* If the window is almost full and there is insufficient lookahead,
   * move the upper half to the lower one to make room in the upper half.
   */
  if more == -1i32 as libc::c_uint {
    /* Very unlikely, but possible on 16 bit machine if strstart == 0
     * and lookahead == 1 (input done one byte at time)
     */
    more = more.wrapping_sub(1)
  } else if (*ptr_to_globals.offset(-1)).strstart
    >= (0x8000i32 + (0x8000i32 - (258i32 + 3i32 + 1i32))) as libc::c_uint
  {
    /* By the IN assertion, the window is not empty so we can't confuse
     * more == 0 with more == 64K on a 16 bit machine.
     */
    memcpy(
      (*ptr_to_globals.offset(-1)).window as *mut libc::c_void,
      (*ptr_to_globals.offset(-1))
        .window
        .offset(0x8000i32 as isize) as *const libc::c_void,
      0x8000i32 as libc::c_ulong,
    ); /* we now have strstart >= MAX_DIST: */
    let ref mut fresh9 = (*ptr_to_globals.offset(-1)).match_start;
    *fresh9 = (*fresh9).wrapping_sub(0x8000i32 as libc::c_uint);
    let ref mut fresh10 = (*ptr_to_globals.offset(-1)).strstart;
    *fresh10 = (*fresh10).wrapping_sub(0x8000i32 as libc::c_uint);
    let ref mut fresh11 = (*ptr_to_globals.offset(-1)).block_start;
    *fresh11 -= 0x8000i32;
    n = 0i32 as libc::c_uint;
    while n < (1i32 << 13i32) as libc::c_uint {
      m = *(*ptr_to_globals.offset(-1))
        .prev
        .offset(0x8000i32 as isize)
        .offset(n as isize) as libc::c_uint;
      *(*ptr_to_globals.offset(-1))
        .prev
        .offset(0x8000i32 as isize)
        .offset(n as isize) = if m >= 0x8000i32 as libc::c_uint {
        m.wrapping_sub(0x8000i32 as libc::c_uint)
      } else {
        0i32 as libc::c_uint
      } as Pos;
      n = n.wrapping_add(1)
    }
    n = 0i32 as libc::c_uint;
    while n < 0x8000i32 as libc::c_uint {
      m = *(*ptr_to_globals.offset(-1)).prev.offset(n as isize) as libc::c_uint;
      *(*ptr_to_globals.offset(-1)).prev.offset(n as isize) = if m >= 0x8000i32 as libc::c_uint {
        m.wrapping_sub(0x8000i32 as libc::c_uint)
      } else {
        0i32 as libc::c_uint
      } as Pos;
      n = n.wrapping_add(1)
      /* If n is not on any hash chain, prev[n] is garbage but
       * its value will never be used.
       */
    }
    more = more.wrapping_add(0x8000i32 as libc::c_uint)
  }
  /* At this point, more >= 2 */
  if (*ptr_to_globals.offset(-1)).eofile == 0 {
    n = file_read(
      (*ptr_to_globals.offset(-1))
        .window
        .offset((*ptr_to_globals.offset(-1)).strstart as isize)
        .offset((*ptr_to_globals.offset(-1)).lookahead as isize) as *mut libc::c_void,
      more,
    );
    if n == 0i32 as libc::c_uint || n == -1i32 as libc::c_uint {
      (*ptr_to_globals.offset(-1)).eofile = 1i32 as smallint
    } else {
      let ref mut fresh12 = (*ptr_to_globals.offset(-1)).lookahead;
      *fresh12 = (*fresh12).wrapping_add(n)
    }
  };
}
/* Both users fill window with the same loop: */
unsafe extern "C" fn fill_window_if_needed() {
  while (*ptr_to_globals.offset(-1)).lookahead < (258i32 + 3i32 + 1i32) as libc::c_uint
    && (*ptr_to_globals.offset(-1)).eofile == 0
  {
    fill_window();
  }
}
/* ===========================================================================
 * Set match_start to the longest match starting at the given string and
 * return its length. Matches shorter or equal to prev_length are discarded,
 * in which case the result is equal to prev_length and match_start is
 * garbage.
 * IN assertions: cur_match is the head of the hash chain for the current
 *   string (strstart) and its distance is <= MAX_DIST, and prev_length >= 1
 */
/* For MSDOS, OS/2 and 386 Unix, an optimized version is in match.asm or
 * match.s. The code is functionally equivalent, so you can use the C version
 * if desired.
 */
unsafe extern "C" fn longest_match(mut cur_match: IPos) -> libc::c_int {
  let mut chain_length: libc::c_uint = max_chain_length as libc::c_int as libc::c_uint; /* max hash chain length */
  let mut scan: *mut uch = (*ptr_to_globals.offset(-1))
    .window
    .offset((*ptr_to_globals.offset(-1)).strstart as isize); /* current string */
  let mut match_0: *mut uch = 0 as *mut uch; /* matched string */
  let mut len: libc::c_int = 0; /* length of current match */
  let mut best_len: libc::c_int = (*ptr_to_globals.offset(-1)).prev_length as libc::c_int; /* best match length so far */
  let mut limit: IPos =
    if (*ptr_to_globals.offset(-1)).strstart > (0x8000i32 - (258i32 + 3i32 + 1i32)) as IPos {
      (*ptr_to_globals.offset(-1))
        .strstart
        .wrapping_sub((0x8000i32 - (258i32 + 3i32 + 1i32)) as IPos)
    } else {
      0i32 as libc::c_uint
    };
  /* Stop when cur_match becomes <= limit. To simplify the code,
   * we prevent matches with the string of window index 0.
   */
  /* The code is optimized for HASH_BITS >= 8 and MAX_MATCH-2 multiple of 16.
   * It is easy to get rid of this optimization if necessary.
   */
  let mut strend: *mut uch = (*ptr_to_globals.offset(-1))
    .window
    .offset((*ptr_to_globals.offset(-1)).strstart as isize)
    .offset(258);
  let mut scan_end1: uch = *scan.offset((best_len - 1i32) as isize);
  let mut scan_end: uch = *scan.offset(best_len as isize);
  /* Do not waste too much time if we already have a good match: */
  if (*ptr_to_globals.offset(-1)).prev_length >= good_match as libc::c_int as libc::c_uint {
    chain_length >>= 2i32
  }
  loop {
    match_0 = (*ptr_to_globals.offset(-1))
      .window
      .offset(cur_match as isize);
    /* Skip to next match if the match length cannot increase
     * or if the match length is less than 2:
     */
    if !(*match_0.offset(best_len as isize) as libc::c_int != scan_end as libc::c_int
      || *match_0.offset((best_len - 1i32) as isize) as libc::c_int != scan_end1 as libc::c_int
      || *match_0 as libc::c_int != *scan as libc::c_int
      || {
        match_0 = match_0.offset(1);
        (*match_0 as libc::c_int) != *scan.offset(1) as libc::c_int
      })
    {
      /* The check at best_len-1 can be removed because it will be made
       * again later. (This heuristic is not always a win.)
       * It is not necessary to compare scan[2] and match[2] since they
       * are always equal when the other bytes match, given that
       * the hash keys are equal and that HASH_BITS >= 8.
       */
      scan = scan.offset(2);
      match_0 = match_0.offset(1);
      loop
      /* We check for insufficient lookahead only every 8th comparison;
       * the 256th check will be made at strstart+258.
       */
      {
        scan = scan.offset(1);
        match_0 = match_0.offset(1);
        if !(*scan as libc::c_int == *match_0 as libc::c_int
          && {
            scan = scan.offset(1);
            match_0 = match_0.offset(1);
            (*scan as libc::c_int) == *match_0 as libc::c_int
          }
          && {
            scan = scan.offset(1);
            match_0 = match_0.offset(1);
            (*scan as libc::c_int) == *match_0 as libc::c_int
          }
          && {
            scan = scan.offset(1);
            match_0 = match_0.offset(1);
            (*scan as libc::c_int) == *match_0 as libc::c_int
          }
          && {
            scan = scan.offset(1);
            match_0 = match_0.offset(1);
            (*scan as libc::c_int) == *match_0 as libc::c_int
          }
          && {
            scan = scan.offset(1);
            match_0 = match_0.offset(1);
            (*scan as libc::c_int) == *match_0 as libc::c_int
          }
          && {
            scan = scan.offset(1);
            match_0 = match_0.offset(1);
            (*scan as libc::c_int) == *match_0 as libc::c_int
          }
          && {
            scan = scan.offset(1);
            match_0 = match_0.offset(1);
            (*scan as libc::c_int) == *match_0 as libc::c_int
          }
          && scan < strend)
        {
          break;
        }
      }
      len = 258i32 - strend.wrapping_offset_from(scan) as libc::c_long as libc::c_int;
      scan = strend.offset(-258);
      if len > best_len {
        (*ptr_to_globals.offset(-1)).match_start = cur_match;
        best_len = len;
        if len >= nice_match as libc::c_int {
          break;
        }
        scan_end1 = *scan.offset((best_len - 1i32) as isize);
        scan_end = *scan.offset(best_len as isize)
      }
    }
    cur_match = *(*ptr_to_globals.offset(-1))
      .prev
      .offset((cur_match & (0x8000i32 - 1i32) as libc::c_uint) as isize) as IPos;
    if !(cur_match > limit && {
      chain_length = chain_length.wrapping_sub(1);
      (chain_length) != 0i32 as libc::c_uint
    }) {
      break;
    }
  }
  return best_len;
}
/* number of codes used to transfer the bit lengths */
/* extra bits for each length code */
static mut extra_lbits: [u8; 29] = [
  0i32 as u8, 0i32 as u8, 0i32 as u8, 0i32 as u8, 0i32 as u8, 0i32 as u8, 0i32 as u8, 0i32 as u8,
  1i32 as u8, 1i32 as u8, 1i32 as u8, 1i32 as u8, 2i32 as u8, 2i32 as u8, 2i32 as u8, 2i32 as u8,
  3i32 as u8, 3i32 as u8, 3i32 as u8, 3i32 as u8, 4i32 as u8, 4i32 as u8, 4i32 as u8, 4i32 as u8,
  5i32 as u8, 5i32 as u8, 5i32 as u8, 5i32 as u8, 0i32 as u8,
];
/* extra bits for each distance code */
static mut extra_dbits: [u8; 30] = [
  0i32 as u8,
  0i32 as u8,
  0i32 as u8,
  0i32 as u8,
  1i32 as u8,
  1i32 as u8,
  2i32 as u8,
  2i32 as u8,
  3i32 as u8,
  3i32 as u8,
  4i32 as u8,
  4i32 as u8,
  5i32 as u8,
  5i32 as u8,
  6i32 as u8,
  6i32 as u8,
  7i32 as u8,
  7i32 as u8,
  8i32 as u8,
  8i32 as u8,
  9i32 as u8,
  9i32 as u8,
  10i32 as u8,
  10i32 as u8,
  11i32 as u8,
  11i32 as u8,
  12i32 as u8,
  12i32 as u8,
  13i32 as u8,
  13i32 as u8,
];
/* extra bits for each bit length code */
static mut extra_blbits: [u8; 19] = [
  0i32 as u8, 0i32 as u8, 0i32 as u8, 0i32 as u8, 0i32 as u8, 0i32 as u8, 0i32 as u8, 0i32 as u8,
  0i32 as u8, 0i32 as u8, 0i32 as u8, 0i32 as u8, 0i32 as u8, 0i32 as u8, 0i32 as u8, 0i32 as u8,
  2i32 as u8, 3i32 as u8, 7i32 as u8,
];
/* number of codes at each bit length for an optimal tree */
static mut bl_order: [u8; 19] = [
  16i32 as u8,
  17i32 as u8,
  18i32 as u8,
  0i32 as u8,
  8i32 as u8,
  7i32 as u8,
  9i32 as u8,
  6i32 as u8,
  10i32 as u8,
  5i32 as u8,
  11i32 as u8,
  4i32 as u8,
  12i32 as u8,
  3i32 as u8,
  13i32 as u8,
  2i32 as u8,
  14i32 as u8,
  1i32 as u8,
  15i32 as u8,
];
/* ===========================================================================
 */
/* Send a code of the given tree. c and tree must not have side effects */
/* Mapping from a distance to a distance code. dist is the distance - 1 and
 * must not have side effects. dist_code[256] and dist_code[257] are never
 * used.
 * The arguments must not have side effects.
 */
/* ===========================================================================
 * Initialize a new block.
 */
unsafe extern "C" fn init_block() {
  let mut n: libc::c_int = 0; /* iterates over tree elements */
  /* Initialize the trees. */
  n = 0i32;
  while n < 256i32 + 1i32 + 29i32 {
    (*(ptr_to_globals as *mut globals2)).dyn_ltree[n as usize]
      .fc
      .freq = 0i32 as ush;
    n += 1
  }
  n = 0i32;
  while n < 30i32 {
    (*(ptr_to_globals as *mut globals2)).dyn_dtree[n as usize]
      .fc
      .freq = 0i32 as ush;
    n += 1
  }
  n = 0i32;
  while n < 19i32 {
    (*(ptr_to_globals as *mut globals2)).bl_tree[n as usize]
      .fc
      .freq = 0i32 as ush;
    n += 1
  }
  (*(ptr_to_globals as *mut globals2)).dyn_ltree[256].fc.freq = 1i32 as ush;
  let ref mut fresh13 = (*(ptr_to_globals as *mut globals2)).static_len;
  *fresh13 = 0i32 as ulg;
  (*(ptr_to_globals as *mut globals2)).opt_len = *fresh13;
  let ref mut fresh14 = (*(ptr_to_globals as *mut globals2)).last_flags;
  *fresh14 = 0i32 as libc::c_uint;
  let ref mut fresh15 = (*(ptr_to_globals as *mut globals2)).last_dist;
  *fresh15 = *fresh14;
  (*(ptr_to_globals as *mut globals2)).last_lit = *fresh15;
  (*(ptr_to_globals as *mut globals2)).flags = 0i32 as uch;
  (*(ptr_to_globals as *mut globals2)).flag_bit = 1i32 as uch;
}
/* ===========================================================================
 * Restore the heap property by moving down the tree starting at node k,
 * exchanging a node with the smallest of its two sons if necessary, stopping
 * when the heap property is re-established (each father smaller than its
 * two sons).
 */
/* Compares to subtrees, using the tree depth as tie breaker when
 * the subtrees have equal frequency. This minimizes the worst case length. */
unsafe extern "C" fn pqdownheap(mut tree: *const ct_data, mut k: libc::c_int) {
  let mut v: libc::c_int = (*(ptr_to_globals as *mut globals2)).heap[k as usize] as libc::c_int; /* left son of k */
  let mut j: libc::c_int = k << 1i32;
  while j <= (*(ptr_to_globals as *mut globals2)).heap_len {
    /* Set j to the smallest of the two sons: */
    if j < (*(ptr_to_globals as *mut globals2)).heap_len
      && (((*tree.offset((*(ptr_to_globals as *mut globals2)).heap[(j + 1i32) as usize] as isize))
        .fc
        .freq as libc::c_int)
        < (*tree.offset((*(ptr_to_globals as *mut globals2)).heap[j as usize] as isize))
          .fc
          .freq as libc::c_int
        || (*tree.offset((*(ptr_to_globals as *mut globals2)).heap[(j + 1i32) as usize] as isize))
          .fc
          .freq as libc::c_int
          == (*tree.offset((*(ptr_to_globals as *mut globals2)).heap[j as usize] as isize))
            .fc
            .freq as libc::c_int
          && (*(ptr_to_globals as *mut globals2)).depth
            [(*(ptr_to_globals as *mut globals2)).heap[(j + 1i32) as usize] as usize]
            as libc::c_int
            <= (*(ptr_to_globals as *mut globals2)).depth
              [(*(ptr_to_globals as *mut globals2)).heap[j as usize] as usize]
              as libc::c_int)
    {
      j += 1
    }
    /* Exit if v is smaller than both sons */
    if ((*tree.offset(v as isize)).fc.freq as libc::c_int)
      < (*tree.offset((*(ptr_to_globals as *mut globals2)).heap[j as usize] as isize))
        .fc
        .freq as libc::c_int
      || (*tree.offset(v as isize)).fc.freq as libc::c_int
        == (*tree.offset((*(ptr_to_globals as *mut globals2)).heap[j as usize] as isize))
          .fc
          .freq as libc::c_int
        && (*(ptr_to_globals as *mut globals2)).depth[v as usize] as libc::c_int
          <= (*(ptr_to_globals as *mut globals2)).depth
            [(*(ptr_to_globals as *mut globals2)).heap[j as usize] as usize]
            as libc::c_int
    {
      break;
    }
    /* Exchange v with the smallest son */
    (*(ptr_to_globals as *mut globals2)).heap[k as usize] =
      (*(ptr_to_globals as *mut globals2)).heap[j as usize];
    k = j;
    /* And continue down the tree, setting j to the left son of k */
    j <<= 1i32
  }
  (*(ptr_to_globals as *mut globals2)).heap[k as usize] = v as ush;
}
/* ===========================================================================
 * Compute the optimal bit lengths for a tree and update the total bit length
 * for the current block.
 * IN assertion: the fields freq and dad are set, heap[heap_max] and
 *    above are the tree nodes sorted by increasing frequency.
 * OUT assertions: the field len is set to the optimal bit length, the
 *     array bl_count contains the frequencies for each bit length.
 *     The length opt_len is updated; static_len is also updated if stree is
 *     not null.
 */
unsafe extern "C" fn gen_bitlen(mut desc: *const tree_desc) {
  let mut h: libc::c_int = 0; /* heap index */
  let mut n: libc::c_int = 0; /* iterate over the tree elements */
  let mut m: libc::c_int = 0; /* bit length */
  let mut bits: libc::c_int = 0; /* number of elements with bit length too large */
  let mut overflow: libc::c_int = 0;
  bits = 0i32;
  while (bits as libc::c_uint)
    < (::std::mem::size_of::<[libc::c_uint; 16]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong) as libc::c_uint
  {
    (*(ptr_to_globals as *mut globals2)).bl_count[bits as usize] = 0i32 as libc::c_uint;
    bits += 1
  }
  /* In a first pass, compute the optimal bit lengths (which may
   * overflow in the case of the bit length tree).
   */
  (*(*desc).dyn_tree.offset(
    (*(ptr_to_globals as *mut globals2)).heap
      [(*(ptr_to_globals as *mut globals2)).heap_max as usize] as isize,
  ))
  .dl
  .len = 0i32 as ush; /* root of the heap */
  overflow = 0i32; /* frequency */
  h = (*(ptr_to_globals as *mut globals2)).heap_max + 1i32; /* extra bits */
  while h < 2i32 * (256i32 + 1i32 + 29i32) + 1i32 {
    let mut f: ulg = 0;
    let mut xbits: libc::c_int = 0;
    n = (*(ptr_to_globals as *mut globals2)).heap[h as usize] as libc::c_int;
    bits = (*(*desc)
      .dyn_tree
      .offset((*(*desc).dyn_tree.offset(n as isize)).dl.dad as isize))
    .dl
    .len as libc::c_int
      + 1i32;
    if bits > (*desc).max_length {
      bits = (*desc).max_length;
      overflow += 1
    }
    (*(*desc).dyn_tree.offset(n as isize)).dl.len = bits as ush;
    /* We overwrite tree[n].Dad which is no longer needed */
    if !(n > (*desc).max_code) {
      let ref mut fresh16 = (*(ptr_to_globals as *mut globals2)).bl_count[bits as usize]; /* not a leaf node */
      *fresh16 = (*fresh16).wrapping_add(1);
      xbits = 0i32;
      if n >= (*desc).extra_base {
        xbits = *(*desc).extra_bits.offset((n - (*desc).extra_base) as isize) as libc::c_int
      }
      f = (*(*desc).dyn_tree.offset(n as isize)).fc.freq as ulg;
      let ref mut fresh17 = (*(ptr_to_globals as *mut globals2)).opt_len;
      *fresh17 = (*fresh17 as libc::c_uint)
        .wrapping_add(f.wrapping_mul((bits + xbits) as libc::c_uint)) as ulg
        as ulg;
      if !(*desc).static_tree.is_null() {
        let ref mut fresh18 = (*(ptr_to_globals as *mut globals2)).static_len;
        *fresh18 = (*fresh18 as libc::c_uint).wrapping_add(f.wrapping_mul(
          ((*(*desc).static_tree.offset(n as isize)).dl.len as libc::c_int + xbits) as libc::c_uint,
        )) as ulg as ulg
      }
    }
    h += 1
  }
  if overflow == 0i32 {
    return;
  }
  loop
  /* This happens for example on obj2 and pic of the Calgary corpus */
  /* Find the first bit length which could increase: */
  {
    bits = (*desc).max_length - 1i32; /* move one leaf down the tree */
    while (*(ptr_to_globals as *mut globals2)).bl_count[bits as usize] == 0i32 as libc::c_uint {
      bits -= 1
    } /* move one overflow item as its brother */
    let ref mut fresh19 = (*(ptr_to_globals as *mut globals2)).bl_count[bits as usize];
    *fresh19 = (*fresh19).wrapping_sub(1);
    let ref mut fresh20 = (*(ptr_to_globals as *mut globals2)).bl_count[(bits + 1i32) as usize];
    *fresh20 = (*fresh20).wrapping_add(2i32 as libc::c_uint);
    let ref mut fresh21 =
      (*(ptr_to_globals as *mut globals2)).bl_count[(*desc).max_length as usize];
    *fresh21 = (*fresh21).wrapping_sub(1);
    /* The brother of the overflow item also moves one step up,
     * but this does not affect bl_count[desc->max_length]
     */
    overflow -= 2i32;
    if !(overflow > 0i32) {
      break;
    }
  }
  /* Now recompute all bit lengths, scanning in increasing frequency.
   * h is still equal to HEAP_SIZE. (It is simpler to reconstruct all
   * lengths instead of fixing only the wrong ones. This idea is taken
   * from 'ar' written by Haruhiko Okumura.)
   */
  bits = (*desc).max_length;
  while bits != 0i32 {
    n = (*(ptr_to_globals as *mut globals2)).bl_count[bits as usize] as libc::c_int;
    while n != 0i32 {
      h -= 1;
      m = (*(ptr_to_globals as *mut globals2)).heap[h as usize] as libc::c_int;
      if m > (*desc).max_code {
        continue;
      }
      if (*(*desc).dyn_tree.offset(m as isize)).dl.len as libc::c_uint != bits as libc::c_uint {
        let ref mut fresh22 = (*(ptr_to_globals as *mut globals2)).opt_len;
        *fresh22 = (*fresh22 as libc::c_uint).wrapping_add(
          ((bits - (*(*desc).dyn_tree.offset(m as isize)).dl.len as libc::c_int)
            * (*(*desc).dyn_tree.offset(m as isize)).fc.freq as libc::c_int)
            as libc::c_uint,
        ) as ulg as ulg;
        (*(*desc).dyn_tree.offset(m as isize)).dl.len = bits as ush
      }
      n -= 1
    }
    bits -= 1
  }
}
/* ===========================================================================
 * Generate the codes for a given tree and bit counts (which need not be
 * optimal).
 * IN assertion: the array bl_count contains the bit length statistics for
 * the given tree and the field len is set for all tree elements.
 * OUT assertion: the field code is set for all tree elements of non
 *     zero code length.
 */
unsafe extern "C" fn gen_codes(mut tree: *mut ct_data, mut max_code: libc::c_int) {
  /* next_code[] and code used to be "ush", but "unsigned" results in smaller code */
  let mut next_code: [libc::c_uint; 16] = [0; 16]; /* next code value for each bit length */
  let mut code: libc::c_uint = 0i32 as libc::c_uint; /* running code value */
  let mut bits: libc::c_int = 0; /* bit index */
  let mut n: libc::c_int = 0; /* code index */
  /* The distribution counts are first used to generate the code values
   * without bit reversal.
   */
  bits = 1i32;
  while bits <= 15i32 {
    code = code.wrapping_add((*(ptr_to_globals as *mut globals2)).bl_count[(bits - 1i32) as usize])
      << 1i32;
    next_code[bits as usize] = code;
    bits += 1
  }
  /* Check that the bit counts in bl_count are consistent. The last code
   * must be all ones.
   */
  n = 0i32;
  while n <= max_code {
    let mut len: libc::c_int = (*tree.offset(n as isize)).dl.len as libc::c_int;
    if !(len == 0i32) {
      /* Now reverse the bits */
      let fresh23 = next_code[len as usize];
      next_code[len as usize] = next_code[len as usize].wrapping_add(1);
      (*tree.offset(n as isize)).fc.code = bi_reverse(fresh23, len) as ush
    }
    n += 1
  }
}
/* Index within the heap array of least frequent node in the Huffman tree */
unsafe extern "C" fn build_tree(mut desc: *mut tree_desc) {
  let mut tree: *mut ct_data = (*desc).dyn_tree; /* iterate over heap elements */
  let mut stree: *mut ct_data = (*desc).static_tree; /* largest code with non zero frequency */
  let mut elems: libc::c_int = (*desc).elems; /* next internal node of the tree */
  let mut n: libc::c_int = 0;
  let mut m: libc::c_int = 0;
  let mut max_code: libc::c_int = -1i32;
  let mut node: libc::c_int = elems;
  /* Construct the initial heap, with least frequent element in
   * heap[SMALLEST]. The sons of heap[n] are heap[2*n] and heap[2*n+1].
   * heap[0] is not used.
   */
  (*(ptr_to_globals as *mut globals2)).heap_len = 0i32;
  (*(ptr_to_globals as *mut globals2)).heap_max = 2i32 * (256i32 + 1i32 + 29i32) + 1i32;
  n = 0i32;
  while n < elems {
    if (*tree.offset(n as isize)).fc.freq as libc::c_int != 0i32 {
      max_code = n;
      let ref mut fresh24 = (*(ptr_to_globals as *mut globals2)).heap_len;
      *fresh24 += 1;
      (*(ptr_to_globals as *mut globals2)).heap[*fresh24 as usize] = max_code as ush;
      (*(ptr_to_globals as *mut globals2)).depth[n as usize] = 0i32 as uch
    } else {
      (*tree.offset(n as isize)).dl.len = 0i32 as ush
    }
    n += 1
  }
  /* The pkzip format requires that at least one distance code exists,
   * and that at least one bit should be sent even if there is only one
   * possible code. So to avoid special checks later on we force at least
   * two codes of non zero frequency.
   */
  while (*(ptr_to_globals as *mut globals2)).heap_len < 2i32 {
    let ref mut fresh25 = (*(ptr_to_globals as *mut globals2)).heap_len;
    *fresh25 += 1;
    let ref mut fresh26 = (*(ptr_to_globals as *mut globals2)).heap[*fresh25 as usize];
    *fresh26 = if max_code < 2i32 {
      max_code += 1;
      max_code
    } else {
      0i32
    } as ush;
    let mut new: libc::c_int = *fresh26 as libc::c_int;
    (*tree.offset(new as isize)).fc.freq = 1i32 as ush;
    (*(ptr_to_globals as *mut globals2)).depth[new as usize] = 0i32 as uch;
    let ref mut fresh27 = (*(ptr_to_globals as *mut globals2)).opt_len;
    *fresh27 = (*fresh27).wrapping_sub(1);
    if !stree.is_null() {
      let ref mut fresh28 = (*(ptr_to_globals as *mut globals2)).static_len;
      *fresh28 = (*fresh28 as libc::c_uint)
        .wrapping_sub((*stree.offset(new as isize)).dl.len as libc::c_uint) as ulg
        as ulg
    }
    /* new is 0 or 1 so it does not have extra bits */
  }
  (*desc).max_code = max_code;
  /* The elements heap[heap_len/2+1 .. heap_len] are leaves of the tree,
   * establish sub-heaps of increasing lengths:
   */
  n = (*(ptr_to_globals as *mut globals2)).heap_len / 2i32;
  while n >= 1i32 {
    pqdownheap(tree, n);
    n -= 1
  }
  loop
  /* Construct the Huffman tree by repeatedly combining the least two
   * frequent nodes.
   */
  {
    n = (*(ptr_to_globals as *mut globals2)).heap[1] as libc::c_int; /* n = node of least frequency */
    let ref mut fresh29 = (*(ptr_to_globals as *mut globals2)).heap_len; /* m = node of next least frequency */
    let fresh30 = *fresh29; /* keep the nodes sorted by frequency */
    *fresh29 = *fresh29 - 1;
    (*(ptr_to_globals as *mut globals2)).heap[1] =
      (*(ptr_to_globals as *mut globals2)).heap[fresh30 as usize];
    pqdownheap(tree, 1i32);
    m = (*(ptr_to_globals as *mut globals2)).heap[1] as libc::c_int;
    let ref mut fresh31 = (*(ptr_to_globals as *mut globals2)).heap_max;
    *fresh31 -= 1;
    (*(ptr_to_globals as *mut globals2)).heap[*fresh31 as usize] = n as ush;
    let ref mut fresh32 = (*(ptr_to_globals as *mut globals2)).heap_max;
    *fresh32 -= 1;
    (*(ptr_to_globals as *mut globals2)).heap[*fresh32 as usize] = m as ush;
    /* Create a new node father of n and m */
    (*tree.offset(node as isize)).fc.freq = ((*tree.offset(n as isize)).fc.freq as libc::c_int
      + (*tree.offset(m as isize)).fc.freq as libc::c_int)
      as ush;
    (*(ptr_to_globals as *mut globals2)).depth[node as usize] =
      ((if (*(ptr_to_globals as *mut globals2)).depth[n as usize] as libc::c_int
        > (*(ptr_to_globals as *mut globals2)).depth[m as usize] as libc::c_int
      {
        (*(ptr_to_globals as *mut globals2)).depth[n as usize] as libc::c_int
      } else {
        (*(ptr_to_globals as *mut globals2)).depth[m as usize] as libc::c_int
      }) + 1i32) as uch;
    let ref mut fresh33 = (*tree.offset(m as isize)).dl.dad;
    *fresh33 = node as ush;
    (*tree.offset(n as isize)).dl.dad = *fresh33;
    /* and insert the new node in the heap */
    let fresh34 = node;
    node = node + 1;
    (*(ptr_to_globals as *mut globals2)).heap[1] = fresh34 as ush;
    pqdownheap(tree, 1i32);
    if !((*(ptr_to_globals as *mut globals2)).heap_len >= 2i32) {
      break;
    }
  }
  let ref mut fresh35 = (*(ptr_to_globals as *mut globals2)).heap_max;
  *fresh35 -= 1;
  (*(ptr_to_globals as *mut globals2)).heap[*fresh35 as usize] =
    (*(ptr_to_globals as *mut globals2)).heap[1];
  /* At this point, the fields freq and dad are set. We can now
   * generate the bit lengths.
   */
  gen_bitlen(desc);
  /* The field len is now set, we can generate the bit codes */
  gen_codes(tree, max_code);
}
/* ===========================================================================
 * Scan a literal or distance tree to determine the frequencies of the codes
 * in the bit length tree. Updates opt_len to take into account the repeat
 * counts. (The contribution of the bit length codes will be added later
 * during the construction of bl_tree.)
 */
unsafe extern "C" fn scan_tree(mut tree: *mut ct_data, mut max_code: libc::c_int) {
  let mut n: libc::c_int = 0; /* iterates over all tree elements */
  let mut prevlen: libc::c_int = -1i32; /* last emitted length */
  let mut curlen: libc::c_int = 0; /* length of current code */
  let mut nextlen: libc::c_int = (*tree.offset(0)).dl.len as libc::c_int; /* length of next code */
  let mut count: libc::c_int = 0i32; /* repeat count of the current code */
  let mut max_count: libc::c_int = 7i32; /* max repeat count */
  let mut min_count: libc::c_int = 4i32; /* min repeat count */
  if nextlen == 0i32 {
    max_count = 138i32; /* guard */
    min_count = 3i32
  }
  (*tree.offset((max_code + 1i32) as isize)).dl.len = 0xffffi32 as ush;
  n = 0i32;
  while n <= max_code {
    curlen = nextlen;
    nextlen = (*tree.offset((n + 1i32) as isize)).dl.len as libc::c_int;
    count += 1;
    if !(count < max_count && curlen == nextlen) {
      if count < min_count {
        let ref mut fresh36 = (*(ptr_to_globals as *mut globals2)).bl_tree[curlen as usize]
          .fc
          .freq;
        *fresh36 = (*fresh36 as libc::c_int + count) as ush
      } else if curlen != 0i32 {
        if curlen != prevlen {
          let ref mut fresh37 = (*(ptr_to_globals as *mut globals2)).bl_tree[curlen as usize]
            .fc
            .freq;
          *fresh37 = (*fresh37).wrapping_add(1)
        }
        let ref mut fresh38 = (*(ptr_to_globals as *mut globals2)).bl_tree[16].fc.freq;
        *fresh38 = (*fresh38).wrapping_add(1)
      } else if count <= 10i32 {
        let ref mut fresh39 = (*(ptr_to_globals as *mut globals2)).bl_tree[17].fc.freq;
        *fresh39 = (*fresh39).wrapping_add(1)
      } else {
        let ref mut fresh40 = (*(ptr_to_globals as *mut globals2)).bl_tree[18].fc.freq;
        *fresh40 = (*fresh40).wrapping_add(1)
      }
      count = 0i32;
      prevlen = curlen;
      max_count = 7i32;
      min_count = 4i32;
      if nextlen == 0i32 {
        max_count = 138i32;
        min_count = 3i32
      } else if curlen == nextlen {
        max_count = 6i32;
        min_count = 3i32
      }
    }
    n += 1
  }
}
/* ===========================================================================
 * Send a literal or distance tree in compressed form, using the codes in
 * bl_tree.
 */
unsafe extern "C" fn send_tree(mut tree: *const ct_data, mut max_code: libc::c_int) {
  let mut n: libc::c_int = 0; /* iterates over all tree elements */
  let mut prevlen: libc::c_int = -1i32; /* last emitted length */
  let mut curlen: libc::c_int = 0; /* length of current code */
  let mut nextlen: libc::c_int = (*tree.offset(0)).dl.len as libc::c_int; /* length of next code */
  let mut count: libc::c_int = 0i32; /* repeat count of the current code */
  let mut max_count: libc::c_int = 7i32; /* max repeat count */
  let mut min_count: libc::c_int = 4i32; /* min repeat count */
  /* tree[max_code+1].Len = -1; */
  /* guard already set */
  if nextlen == 0i32 {
    max_count = 138i32;
    min_count = 3i32
  }
  n = 0i32;
  while n <= max_code {
    curlen = nextlen;
    nextlen = (*tree.offset((n + 1i32) as isize)).dl.len as libc::c_int;
    count += 1;
    if !(count < max_count && curlen == nextlen) {
      if count < min_count {
        loop {
          send_bits(
            (*(ptr_to_globals as *mut globals2)).bl_tree[curlen as usize]
              .fc
              .code as libc::c_uint,
            (*(ptr_to_globals as *mut globals2)).bl_tree[curlen as usize]
              .dl
              .len as libc::c_uint,
          );
          count -= 1;
          if !(count != 0) {
            break;
          }
        }
      } else if curlen != 0i32 {
        if curlen != prevlen {
          send_bits(
            (*(ptr_to_globals as *mut globals2)).bl_tree[curlen as usize]
              .fc
              .code as libc::c_uint,
            (*(ptr_to_globals as *mut globals2)).bl_tree[curlen as usize]
              .dl
              .len as libc::c_uint,
          );
          count -= 1
        }
        send_bits(
          (*(ptr_to_globals as *mut globals2)).bl_tree[16].fc.code as libc::c_uint,
          (*(ptr_to_globals as *mut globals2)).bl_tree[16].dl.len as libc::c_uint,
        );
        send_bits((count - 3i32) as libc::c_uint, 2i32 as libc::c_uint);
      } else if count <= 10i32 {
        send_bits(
          (*(ptr_to_globals as *mut globals2)).bl_tree[17].fc.code as libc::c_uint,
          (*(ptr_to_globals as *mut globals2)).bl_tree[17].dl.len as libc::c_uint,
        );
        send_bits((count - 3i32) as libc::c_uint, 3i32 as libc::c_uint);
      } else {
        send_bits(
          (*(ptr_to_globals as *mut globals2)).bl_tree[18].fc.code as libc::c_uint,
          (*(ptr_to_globals as *mut globals2)).bl_tree[18].dl.len as libc::c_uint,
        );
        send_bits((count - 11i32) as libc::c_uint, 7i32 as libc::c_uint);
      }
      count = 0i32;
      prevlen = curlen;
      if nextlen == 0i32 {
        max_count = 138i32;
        min_count = 3i32
      } else if curlen == nextlen {
        max_count = 6i32;
        min_count = 3i32
      } else {
        max_count = 7i32;
        min_count = 4i32
      }
    }
    n += 1
  }
}
/* ===========================================================================
 * Construct the Huffman tree for the bit lengths and return the index in
 * bl_order of the last bit length code to send.
 */
unsafe extern "C" fn build_bl_tree() -> libc::c_int {
  let mut max_blindex: libc::c_int = 0; /* index of last bit length code of non zero freq */
  /* Determine the bit length frequencies for literal and distance trees */
  scan_tree(
    (*(ptr_to_globals as *mut globals2)).dyn_ltree.as_mut_ptr(),
    (*(ptr_to_globals as *mut globals2)).l_desc.max_code,
  );
  scan_tree(
    (*(ptr_to_globals as *mut globals2)).dyn_dtree.as_mut_ptr(),
    (*(ptr_to_globals as *mut globals2)).d_desc.max_code,
  );
  /* Build the bit length tree: */
  build_tree(&mut (*(ptr_to_globals as *mut globals2)).bl_desc);
  /* opt_len now includes the length of the tree representations, except
   * the lengths of the bit lengths codes and the 5+5+4 bits for the counts.
   */
  /* Determine the number of bit length codes to send. The pkzip format
   * requires that at least 4 bit length codes be sent. (appnote.txt says
   * 3 but the actual value used is 4.)
   */
  max_blindex = 19i32 - 1i32;
  while max_blindex >= 3i32 {
    if (*(ptr_to_globals as *mut globals2)).bl_tree[bl_order[max_blindex as usize] as usize]
      .dl
      .len as libc::c_int
      != 0i32
    {
      break;
    }
    max_blindex -= 1
  }
  /* Update opt_len to include the bit length tree and counts */
  let ref mut fresh41 = (*(ptr_to_globals as *mut globals2)).opt_len;
  *fresh41 = (*fresh41 as libc::c_uint)
    .wrapping_add((3i32 * (max_blindex + 1i32) + 5i32 + 5i32 + 4i32) as libc::c_uint)
    as ulg as ulg;
  return max_blindex;
}
/* ===========================================================================
 * Send the header for a block using dynamic Huffman trees: the counts, the
 * lengths of the bit length codes, the literal tree and the distance tree.
 * IN assertion: lcodes >= 257, dcodes >= 1, blcodes >= 4.
 */
unsafe extern "C" fn send_all_trees(
  mut lcodes: libc::c_int,
  mut dcodes: libc::c_int,
  mut blcodes: libc::c_int,
) {
  let mut rank: libc::c_int = 0; /* index in bl_order */
  send_bits((lcodes - 257i32) as libc::c_uint, 5i32 as libc::c_uint); /* not +255 as stated in appnote.txt */
  send_bits((dcodes - 1i32) as libc::c_uint, 5i32 as libc::c_uint); /* not -3 as stated in appnote.txt */
  send_bits((blcodes - 4i32) as libc::c_uint, 4i32 as libc::c_uint); /* send the literal tree */
  rank = 0i32; /* send the distance tree */
  while rank < blcodes {
    send_bits(
      (*(ptr_to_globals as *mut globals2)).bl_tree[bl_order[rank as usize] as usize]
        .dl
        .len as libc::c_uint,
      3i32 as libc::c_uint,
    );
    rank += 1
  }
  send_tree(
    (*(ptr_to_globals as *mut globals2)).dyn_ltree.as_mut_ptr(),
    lcodes - 1i32,
  );
  send_tree(
    (*(ptr_to_globals as *mut globals2)).dyn_dtree.as_mut_ptr(),
    dcodes - 1i32,
  );
}
/* ===========================================================================
 * Save the match info and tally the frequency counts. Return true if
 * the current block must be flushed.
 */
unsafe extern "C" fn ct_tally(mut dist: libc::c_int, mut lc: libc::c_int) -> libc::c_int {
  let ref mut fresh42 = (*(ptr_to_globals as *mut globals2)).last_lit;
  let fresh43 = *fresh42;
  *fresh42 = (*fresh42).wrapping_add(1);
  *(*ptr_to_globals.offset(-1)).l_buf.offset(fresh43 as isize) = lc as uch;
  if dist == 0i32 {
    /* lc is the unmatched char */
    let ref mut fresh44 = (*(ptr_to_globals as *mut globals2)).dyn_ltree[lc as usize]
      .fc
      .freq;
    *fresh44 = (*fresh44).wrapping_add(1)
  } else {
    /* Here, lc is the match length - MIN_MATCH */
    dist -= 1; /* dist = match distance - 1 */
    let ref mut fresh45 = (*(ptr_to_globals as *mut globals2)).dyn_ltree[((*(ptr_to_globals
      as *mut globals2))
      .length_code[lc as usize]
      as libc::c_int
      + 256i32
      + 1i32) as usize]
      .fc
      .freq;
    *fresh45 = (*fresh45).wrapping_add(1);
    let ref mut fresh46 = (*(ptr_to_globals as *mut globals2)).dyn_dtree[if dist < 256i32 {
      (*(ptr_to_globals as *mut globals2)).dist_code[dist as usize] as libc::c_int
    } else {
      (*(ptr_to_globals as *mut globals2)).dist_code[(256i32 + (dist >> 7i32)) as usize]
        as libc::c_int
    } as usize]
      .fc
      .freq;
    *fresh46 = (*fresh46).wrapping_add(1);
    let ref mut fresh47 = (*(ptr_to_globals as *mut globals2)).last_dist;
    let fresh48 = *fresh47;
    *fresh47 = (*fresh47).wrapping_add(1);
    *(*ptr_to_globals.offset(-1)).d_buf.offset(fresh48 as isize) = dist as ush;
    let ref mut fresh49 = (*(ptr_to_globals as *mut globals2)).flags;
    *fresh49 = (*fresh49 as libc::c_int
      | (*(ptr_to_globals as *mut globals2)).flag_bit as libc::c_int) as uch
  }
  let ref mut fresh50 = (*(ptr_to_globals as *mut globals2)).flag_bit;
  *fresh50 = ((*fresh50 as libc::c_int) << 1i32) as uch;
  /* Output the flags if they fill a byte: */
  if (*(ptr_to_globals as *mut globals2)).last_lit & 7i32 as libc::c_uint == 0i32 as libc::c_uint {
    let ref mut fresh51 = (*(ptr_to_globals as *mut globals2)).last_flags;
    let fresh52 = *fresh51;
    *fresh51 = (*fresh51).wrapping_add(1);
    (*(ptr_to_globals as *mut globals2)).flag_buf[fresh52 as usize] =
      (*(ptr_to_globals as *mut globals2)).flags;
    (*(ptr_to_globals as *mut globals2)).flags = 0i32 as uch;
    (*(ptr_to_globals as *mut globals2)).flag_bit = 1i32 as uch
  }
  /* Try to guess if it is profitable to stop the current block here */
  if (*(ptr_to_globals as *mut globals2)).last_lit & 0xfffi32 as libc::c_uint
    == 0i32 as libc::c_uint
  {
    /* Compute an upper bound for the compressed length */
    let mut out_length: ulg =
      ((*(ptr_to_globals as *mut globals2)).last_lit as libc::c_long * 8i64) as ulg;
    let mut in_length: ulg = (*ptr_to_globals.offset(-1))
      .strstart
      .wrapping_sub((*ptr_to_globals.offset(-1)).block_start as libc::c_uint);
    let mut dcode: libc::c_int = 0;
    dcode = 0i32;
    while dcode < 30i32 {
      out_length = (out_length as libc::c_long
        + (*(ptr_to_globals as *mut globals2)).dyn_dtree[dcode as usize]
          .fc
          .freq as libc::c_long
          * (5i64 + extra_dbits[dcode as usize] as libc::c_long)) as ulg;
      dcode += 1
    }
    out_length >>= 3i32;
    if (*(ptr_to_globals as *mut globals2)).last_dist
      < (*(ptr_to_globals as *mut globals2))
        .last_lit
        .wrapping_div(2i32 as libc::c_uint)
      && out_length < in_length.wrapping_div(2i32 as libc::c_uint)
    {
      return 1i32;
    }
  }
  return ((*(ptr_to_globals as *mut globals2)).last_lit == (0x2000i32 - 1i32) as libc::c_uint
    || (*(ptr_to_globals as *mut globals2)).last_dist == 0x2000i32 as libc::c_uint)
    as libc::c_int;
  /* We avoid equality with LIT_BUFSIZE because of wraparound at 64K
   * on 16 bit machines and because stored blocks are restricted to
   * 64K-1 bytes.
   */
}
/* ===========================================================================
 * Send the block data compressed using the given Huffman trees
 */
unsafe extern "C" fn compress_block(mut ltree: *const ct_data, mut dtree: *const ct_data) {
  let mut dist: libc::c_uint = 0; /* distance of matched string */
  let mut lc: libc::c_int = 0; /* match length or unmatched char (if dist == 0) */
  let mut lx: libc::c_uint = 0i32 as libc::c_uint; /* running index in l_buf */
  let mut dx: libc::c_uint = 0i32 as libc::c_uint; /* running index in d_buf */
  let mut fx: libc::c_uint = 0i32 as libc::c_uint; /* running index in flag_buf */
  let mut flag: uch = 0i32 as uch; /* current flags */
  let mut code: libc::c_uint = 0; /* the code to send */
  let mut extra: libc::c_int = 0; /* number of extra bits to send */
  if (*(ptr_to_globals as *mut globals2)).last_lit != 0i32 as libc::c_uint {
    loop {
      if lx & 7i32 as libc::c_uint == 0i32 as libc::c_uint {
        let fresh53 = fx; /* literal or match pair ? */
        fx = fx.wrapping_add(1); /* send a literal byte */
        flag = (*(ptr_to_globals as *mut globals2)).flag_buf[fresh53 as usize]
      }
      let fresh54 = lx;
      lx = lx.wrapping_add(1);
      lc = *(*ptr_to_globals.offset(-1)).l_buf.offset(fresh54 as isize) as libc::c_int;
      if flag as libc::c_int & 1i32 == 0i32 {
        send_bits(
          (*ltree.offset(lc as isize)).fc.code as libc::c_uint,
          (*ltree.offset(lc as isize)).dl.len as libc::c_uint,
        );
      } else {
        /* Here, lc is the match length - MIN_MATCH */
        code = (*(ptr_to_globals as *mut globals2)).length_code[lc as usize] as libc::c_uint; /* send the length code */
        send_bits(
          (*ltree.offset(
            code
              .wrapping_add(256i32 as libc::c_uint)
              .wrapping_add(1i32 as libc::c_uint) as isize,
          ))
          .fc
          .code as libc::c_uint,
          (*ltree.offset(
            code
              .wrapping_add(256i32 as libc::c_uint)
              .wrapping_add(1i32 as libc::c_uint) as isize,
          ))
          .dl
          .len as libc::c_uint,
        );
        extra = extra_lbits[code as usize] as libc::c_int;
        if extra != 0i32 {
          lc -= (*(ptr_to_globals as *mut globals2)).base_length[code as usize];
          send_bits(lc as libc::c_uint, extra as libc::c_uint);
          /* send the extra length bits */
        }
        let fresh55 = dx;
        dx = dx.wrapping_add(1);
        dist = *(*ptr_to_globals.offset(-1)).d_buf.offset(fresh55 as isize) as libc::c_uint;
        /* Here, dist is the match distance - 1 */
        code = if dist < 256i32 as libc::c_uint {
          (*(ptr_to_globals as *mut globals2)).dist_code[dist as usize] as libc::c_int
        } else {
          (*(ptr_to_globals as *mut globals2)).dist_code
            [(256i32 as libc::c_uint).wrapping_add(dist >> 7i32) as usize] as libc::c_int
        } as libc::c_uint; /* send the distance code */
        send_bits(
          (*dtree.offset(code as isize)).fc.code as libc::c_uint,
          (*dtree.offset(code as isize)).dl.len as libc::c_uint,
        );
        extra = extra_dbits[code as usize] as libc::c_int;
        if extra != 0i32 {
          dist = dist.wrapping_sub(
            (*(ptr_to_globals as *mut globals2)).base_dist[code as usize] as libc::c_uint,
          );
          send_bits(dist, extra as libc::c_uint);
          /* send the extra distance bits */
        }
      }
      flag = (flag as libc::c_int >> 1i32) as uch;
      if !(lx < (*(ptr_to_globals as *mut globals2)).last_lit) {
        break;
      }
    }
  }
  send_bits(
    (*ltree.offset(256)).fc.code as libc::c_uint,
    (*ltree.offset(256)).dl.len as libc::c_uint,
  );
}
/* ===========================================================================
 * Determine the best encoding for the current block: dynamic trees, static
 * trees or store, and output the encoded block to the zip file. This function
 * returns the total compressed length for the file so far.
 */
unsafe extern "C" fn flush_block(
  mut buf: *const libc::c_char,
  mut stored_len: ulg,
  mut eof: libc::c_int,
) {
  let mut opt_lenb: ulg = 0; /* opt_len and static_len in bytes */
  let mut static_lenb: ulg = 0; /* index of last bit length code of non zero freq */
  let mut max_blindex: libc::c_int = 0; /* Save the flags for the last 8 items */
  (*(ptr_to_globals as *mut globals2)).flag_buf
    [(*(ptr_to_globals as *mut globals2)).last_flags as usize] =
    (*(ptr_to_globals as *mut globals2)).flags;
  /* Construct the literal and distance trees */
  build_tree(&mut (*(ptr_to_globals as *mut globals2)).l_desc);
  build_tree(&mut (*(ptr_to_globals as *mut globals2)).d_desc);
  /* At this point, opt_len and static_len are the total bit lengths of
   * the compressed block data, excluding the tree representations.
   */
  /* Build the bit length tree for the above two trees, and get the index
   * in bl_order of the last bit length code to send.
   */
  max_blindex = build_bl_tree();
  /* Determine the best encoding. Compute first the block length in bytes */
  opt_lenb = (*(ptr_to_globals as *mut globals2))
    .opt_len
    .wrapping_add(3i32 as libc::c_uint)
    .wrapping_add(7i32 as libc::c_uint)
    >> 3i32;
  static_lenb = (*(ptr_to_globals as *mut globals2))
    .static_len
    .wrapping_add(3i32 as libc::c_uint)
    .wrapping_add(7i32 as libc::c_uint)
    >> 3i32;
  if static_lenb <= opt_lenb {
    opt_lenb = static_lenb
  }
  /* If compression failed and this is the first and last block,
   * and if the zip file can be seeked (to rewrite the local header),
   * the whole file is transformed into a stored file:
   */
  // seekable() is constant FALSE in busybox, and G2.compressed_len is disabled
  // (this was the only user)
  //	if (stored_len <= opt_lenb && eof && G2.compressed_len == 0L && seekable()) {
  //		/* Since LIT_BUFSIZE <= 2*WSIZE, the input data must be there: */
  //		if (buf == NULL)
  //			bb_error_msg("block vanished");
  //
  //		G2.compressed_len = stored_len << 3;
  //		copy_block(buf, (unsigned) stored_len, 0);	/* without header */
  //	} else
  if stored_len.wrapping_add(4i32 as libc::c_uint) <= opt_lenb && !buf.is_null() {
    /* 4: two words for the lengths */
    /* The test buf != NULL is only necessary if LIT_BUFSIZE > WSIZE.
     * Otherwise we can't have processed more than WSIZE input bytes since
     * the last block flush, because compression would have been
     * successful. If LIT_BUFSIZE <= WSIZE, it is never too late to
     * transform a block into a stored block.
     */
    send_bits(((0i32 << 1i32) + eof) as libc::c_uint, 3i32 as libc::c_uint); /* send block type */
    /* with header */
    copy_block(buf, stored_len, 1i32);
  } else if static_lenb == opt_lenb {
    send_bits(((1i32 << 1i32) + eof) as libc::c_uint, 3i32 as libc::c_uint);
    compress_block(
      (*(ptr_to_globals as *mut globals2))
        .static_ltree
        .as_mut_ptr(),
      (*(ptr_to_globals as *mut globals2))
        .static_dtree
        .as_mut_ptr(),
    );
  //		G2.compressed_len = ((G2.compressed_len + 3 + 7) & ~7L)
  //				+ ((stored_len + 4) << 3);
  //		G2.compressed_len += 3 + G2.static_len;
  } else {
    send_bits(((2i32 << 1i32) + eof) as libc::c_uint, 3i32 as libc::c_uint);
    send_all_trees(
      (*(ptr_to_globals as *mut globals2)).l_desc.max_code + 1i32,
      (*(ptr_to_globals as *mut globals2)).d_desc.max_code + 1i32,
      max_blindex + 1i32,
    );
    compress_block(
      (*(ptr_to_globals as *mut globals2)).dyn_ltree.as_mut_ptr(),
      (*(ptr_to_globals as *mut globals2)).dyn_dtree.as_mut_ptr(),
    );
    //		G2.compressed_len += 3 + G2.opt_len;
  }
  //	Assert(G2.compressed_len == G1.bits_sent, "bad compressed size");
  init_block();
  if eof != 0 {
    bi_windup();
    //		G2.compressed_len += 7;	/* align on byte boundary */
  };
  /* was "return G2.compressed_len >> 3;" */
}
/* ===========================================================================
 * Update a hash value with the given input byte
 * IN  assertion: all calls to UPDATE_HASH are made with consecutive
 *    input characters, so that a running hash key can be computed from the
 *    previous key instead of complete recalculation each time.
 */
/* ===========================================================================
 * Same as above, but achieves better compression. We use a lazy
 * evaluation for matches: a match is finally adopted only if there is
 * no better match at the next window position.
 *
 * Processes a new input file and return its compressed length. Sets
 * the compressed length, crc, deflate flags and internal file
 * attributes.
 */
/* Flush the current block, with given end-of-file flag.
 * IN assertion: strstart is set to the end of the current match. */
/* Insert string s in the dictionary and set match_head to the previous head
 * of the hash chain (the most recent string with same hash key). Return
 * the previous length of the hash chain.
 * IN  assertion: all calls to INSERT_STRING are made with consecutive
 *    input characters and the first MIN_MATCH bytes of s are valid
 *    (except for the last MIN_MATCH-1 bytes of the input file). */
#[inline(never)]
unsafe extern "C" fn deflate() {
  let mut hash_head: IPos = 0; /* head of hash chain */
  let mut prev_match: IPos = 0; /* previous match */
  let mut flush: libc::c_int = 0; /* set if current block must be flushed */
  let mut match_available: libc::c_int = 0i32; /* set if previous match exists */
  let mut match_length: libc::c_uint = (3i32 - 1i32) as libc::c_uint; /* length of best match */
  /* Process the input block. */
  while (*ptr_to_globals.offset(-1)).lookahead != 0i32 as libc::c_uint {
    /* Insert the string window[strstart .. strstart+2] in the
     * dictionary, and set hash_head to the head of the hash chain:
     */
    (*ptr_to_globals.offset(-1)).ins_h = ((*ptr_to_globals.offset(-1)).ins_h
      << (13i32 + 3i32 - 1i32) / 3i32
      ^ *(*ptr_to_globals.offset(-1)).window.offset(
        (*ptr_to_globals.offset(-1))
          .strstart
          .wrapping_add(3i32 as libc::c_uint)
          .wrapping_sub(1i32 as libc::c_uint) as isize,
      ) as libc::c_uint)
      & ((1i32 << 13i32) as libc::c_uint).wrapping_sub(1i32 as libc::c_uint);
    hash_head = *(*ptr_to_globals.offset(-1))
      .prev
      .offset(0x8000i32 as isize)
      .offset((*ptr_to_globals.offset(-1)).ins_h as isize) as IPos;
    *(*ptr_to_globals.offset(-1)).prev.offset(
      ((*ptr_to_globals.offset(-1)).strstart & (0x8000i32 - 1i32) as libc::c_uint) as isize,
    ) = hash_head as ush;
    *(*ptr_to_globals.offset(-1))
      .prev
      .offset(0x8000i32 as isize)
      .offset((*ptr_to_globals.offset(-1)).ins_h as isize) =
      (*ptr_to_globals.offset(-1)).strstart as ush;
    /* Find the longest match, discarding those <= prev_length.
     */
    (*ptr_to_globals.offset(-1)).prev_length = match_length;
    prev_match = (*ptr_to_globals.offset(-1)).match_start;
    match_length = (3i32 - 1i32) as libc::c_uint;
    if hash_head != 0i32 as libc::c_uint
      && (*ptr_to_globals.offset(-1)).prev_length < max_lazy_match as libc::c_int as libc::c_uint
      && (*ptr_to_globals.offset(-1))
        .strstart
        .wrapping_sub(hash_head)
        <= (0x8000i32 - (258i32 + 3i32 + 1i32)) as libc::c_uint
    {
      /* To simplify the code, we prevent matches with the string
       * of window index 0 (in particular we have to avoid a match
       * of the string with itself at the start of the input file).
       */
      match_length = longest_match(hash_head) as libc::c_uint;
      /* longest_match() sets match_start */
      if match_length > (*ptr_to_globals.offset(-1)).lookahead {
        match_length = (*ptr_to_globals.offset(-1)).lookahead
      }
      /* Ignore a length 3 match if it is too distant: */
      if match_length == 3i32 as libc::c_uint
        && (*ptr_to_globals.offset(-1))
          .strstart
          .wrapping_sub((*ptr_to_globals.offset(-1)).match_start)
          > 4096i32 as libc::c_uint
      {
        /* If prev_match is also MIN_MATCH, G1.match_start is garbage
         * but we will ignore the current match anyway.
         */
        match_length = match_length.wrapping_sub(1)
      }
    }
    /* If there was a match at the previous step and the current
     * match is not better, output the previous match:
     */
    if (*ptr_to_globals.offset(-1)).prev_length >= 3i32 as libc::c_uint
      && match_length <= (*ptr_to_globals.offset(-1)).prev_length
    {
      flush = ct_tally(
        (*ptr_to_globals.offset(-1))
          .strstart
          .wrapping_sub(1i32 as libc::c_uint)
          .wrapping_sub(prev_match) as libc::c_int,
        (*ptr_to_globals.offset(-1))
          .prev_length
          .wrapping_sub(3i32 as libc::c_uint) as libc::c_int,
      );
      /* Insert in hash table all strings up to the end of the match.
       * strstart-1 and strstart are already inserted.
       */
      let ref mut fresh56 = (*ptr_to_globals.offset(-1)).lookahead;
      *fresh56 = (*fresh56).wrapping_sub(
        (*ptr_to_globals.offset(-1))
          .prev_length
          .wrapping_sub(1i32 as libc::c_uint),
      );
      let ref mut fresh57 = (*ptr_to_globals.offset(-1)).prev_length;
      *fresh57 = (*fresh57).wrapping_sub(2i32 as libc::c_uint);
      loop {
        let ref mut fresh58 = (*ptr_to_globals.offset(-1)).strstart;
        *fresh58 = (*fresh58).wrapping_add(1);
        (*ptr_to_globals.offset(-1)).ins_h = ((*ptr_to_globals.offset(-1)).ins_h
          << (13i32 + 3i32 - 1i32) / 3i32
          ^ *(*ptr_to_globals.offset(-1)).window.offset(
            (*ptr_to_globals.offset(-1))
              .strstart
              .wrapping_add(3i32 as libc::c_uint)
              .wrapping_sub(1i32 as libc::c_uint) as isize,
          ) as libc::c_uint)
          & ((1i32 << 13i32) as libc::c_uint).wrapping_sub(1i32 as libc::c_uint);
        hash_head = *(*ptr_to_globals.offset(-1))
          .prev
          .offset(0x8000i32 as isize)
          .offset((*ptr_to_globals.offset(-1)).ins_h as isize) as IPos;
        *(*ptr_to_globals.offset(-1)).prev.offset(
          ((*ptr_to_globals.offset(-1)).strstart & (0x8000i32 - 1i32) as libc::c_uint) as isize,
        ) = hash_head as ush;
        *(*ptr_to_globals.offset(-1))
          .prev
          .offset(0x8000i32 as isize)
          .offset((*ptr_to_globals.offset(-1)).ins_h as isize) =
          (*ptr_to_globals.offset(-1)).strstart as ush;
        let ref mut fresh59 = (*ptr_to_globals.offset(-1)).prev_length;
        *fresh59 = (*fresh59).wrapping_sub(1);
        if !(*fresh59 != 0i32 as libc::c_uint) {
          break;
        }
        /* strstart never exceeds WSIZE-MAX_MATCH, so there are
         * always MIN_MATCH bytes ahead. If lookahead < MIN_MATCH
         * these bytes are garbage, but it does not matter since the
         * next lookahead bytes will always be emitted as literals.
         */
      }
      match_available = 0i32;
      match_length = (3i32 - 1i32) as libc::c_uint;
      let ref mut fresh60 = (*ptr_to_globals.offset(-1)).strstart;
      *fresh60 = (*fresh60).wrapping_add(1);
      if flush != 0 {
        flush_block(
          if (*ptr_to_globals.offset(-1)).block_start as libc::c_long >= 0i64 {
            &mut *(*ptr_to_globals.offset(-1))
              .window
              .offset((*ptr_to_globals.offset(-1)).block_start as libc::c_uint as isize)
              as *mut uch as *mut libc::c_char
          } else {
            0 as *mut libc::c_void as *mut libc::c_char
          },
          (*ptr_to_globals.offset(-1))
            .strstart
            .wrapping_sub((*ptr_to_globals.offset(-1)).block_start as libc::c_uint),
          0i32,
        );
        (*ptr_to_globals.offset(-1)).block_start = (*ptr_to_globals.offset(-1)).strstart as lng
      }
    } else if match_available != 0 {
      /* If there was no match at the previous position, output a
       * single literal. If there was a match but the current match
       * is longer, truncate the previous match to a single literal.
       */
      if ct_tally(
        0i32,
        *(*ptr_to_globals.offset(-1)).window.offset(
          (*ptr_to_globals.offset(-1))
            .strstart
            .wrapping_sub(1i32 as libc::c_uint) as isize,
        ) as libc::c_int,
      ) != 0
      {
        flush_block(
          if (*ptr_to_globals.offset(-1)).block_start as libc::c_long >= 0i64 {
            &mut *(*ptr_to_globals.offset(-1))
              .window
              .offset((*ptr_to_globals.offset(-1)).block_start as libc::c_uint as isize)
              as *mut uch as *mut libc::c_char
          } else {
            0 as *mut libc::c_void as *mut libc::c_char
          },
          (*ptr_to_globals.offset(-1))
            .strstart
            .wrapping_sub((*ptr_to_globals.offset(-1)).block_start as libc::c_uint),
          0i32,
        );
        (*ptr_to_globals.offset(-1)).block_start = (*ptr_to_globals.offset(-1)).strstart as lng
      }
      let ref mut fresh61 = (*ptr_to_globals.offset(-1)).strstart;
      *fresh61 = (*fresh61).wrapping_add(1);
      let ref mut fresh62 = (*ptr_to_globals.offset(-1)).lookahead;
      *fresh62 = (*fresh62).wrapping_sub(1)
    } else {
      /* There is no previous match to compare with, wait for
       * the next step to decide.
       */
      match_available = 1i32;
      let ref mut fresh63 = (*ptr_to_globals.offset(-1)).strstart;
      *fresh63 = (*fresh63).wrapping_add(1);
      let ref mut fresh64 = (*ptr_to_globals.offset(-1)).lookahead;
      *fresh64 = (*fresh64).wrapping_sub(1)
    }
    /* Make sure that we always have enough lookahead, except
     * at the end of the input file. We need MAX_MATCH bytes
     * for the next match, plus MIN_MATCH bytes to insert the
     * string following the next match.
     */
    fill_window_if_needed();
  }
  if match_available != 0 {
    ct_tally(
      0i32,
      *(*ptr_to_globals.offset(-1)).window.offset(
        (*ptr_to_globals.offset(-1))
          .strstart
          .wrapping_sub(1i32 as libc::c_uint) as isize,
      ) as libc::c_int,
    );
  }
  flush_block(
    if (*ptr_to_globals.offset(-1)).block_start as libc::c_long >= 0i64 {
      &mut *(*ptr_to_globals.offset(-1))
        .window
        .offset((*ptr_to_globals.offset(-1)).block_start as libc::c_uint as isize) as *mut uch
        as *mut libc::c_char
    } else {
      0 as *mut libc::c_void as *mut libc::c_char
    },
    (*ptr_to_globals.offset(-1))
      .strstart
      .wrapping_sub((*ptr_to_globals.offset(-1)).block_start as libc::c_uint),
    1i32,
  );
  /* eof */
}
/* ===========================================================================
 * Initialize the bit string routines.
 */
unsafe extern "C" fn bi_init() {
  //G1.bi_buf = 0; // globals are zeroed in pack_gzip()
  //G1.bi_valid = 0; // globals are zeroed in pack_gzip()
  //DEBUG_bits_sent(= 0L); // globals are zeroed in pack_gzip()
}
/* ===========================================================================
 * Initialize the "longest match" routines for a new file
 */
unsafe extern "C" fn lm_init() {
  let mut j: libc::c_uint = 0;
  /* Initialize the hash table. */
  memset(
    (*ptr_to_globals.offset(-1)).prev.offset(0x8000i32 as isize) as *mut libc::c_void,
    0i32,
    ((1i32 << 13i32) as libc::c_uint as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<ush>() as libc::c_ulong),
  );
  /* prev will be initialized on the fly */
  /* ??? reduce max_chain_length for binary files */
  //G1.strstart = 0; // globals are zeroed in pack_gzip()
  //G1.block_start = 0L; // globals are zeroed in pack_gzip()
  (*ptr_to_globals.offset(-1)).lookahead = file_read(
    (*ptr_to_globals.offset(-1)).window as *mut libc::c_void,
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong <= 2i32 as libc::c_ulong {
      0x8000i32 as libc::c_uint
    } else {
      (2i32 * 0x8000i32) as libc::c_uint
    },
  );
  if (*ptr_to_globals.offset(-1)).lookahead == 0i32 as libc::c_uint
    || (*ptr_to_globals.offset(-1)).lookahead == -1i32 as libc::c_uint
  {
    (*ptr_to_globals.offset(-1)).eofile = 1i32 as smallint;
    (*ptr_to_globals.offset(-1)).lookahead = 0i32 as libc::c_uint;
    return;
  }
  //G1.eofile = 0; // globals are zeroed in pack_gzip()
  /* Make sure that we always have enough lookahead. This is important
   * if input comes from a device such as a tty.
   */
  fill_window_if_needed();
  //G1.ins_h = 0; // globals are zeroed in pack_gzip()
  j = 0i32 as libc::c_uint;
  while j < (3i32 - 1i32) as libc::c_uint {
    (*ptr_to_globals.offset(-1)).ins_h = ((*ptr_to_globals.offset(-1)).ins_h
      << (13i32 + 3i32 - 1i32) / 3i32
      ^ *(*ptr_to_globals.offset(-1)).window.offset(j as isize) as libc::c_uint)
      & ((1i32 << 13i32) as libc::c_uint).wrapping_sub(1i32 as libc::c_uint);
    j = j.wrapping_add(1)
  }
  /* If lookahead < MIN_MATCH, ins_h is garbage, but this is
   * not important since only literal bytes will be emitted.
   */
}
/* ===========================================================================
 * Allocate the match buffer, initialize the various tables and save the
 * location of the internal file attribute (ascii/binary) and method
 * (DEFLATE/STORE).
 * One callsite in zip()
 */
unsafe extern "C" fn ct_init() {
  let mut n: libc::c_int = 0; /* iterates over tree elements */
  let mut length: libc::c_int = 0; /* length value */
  let mut code: libc::c_int = 0; /* code value */
  let mut dist: libc::c_int = 0; /* distance index */
  //	//G2.compressed_len = 0L; // globals are zeroed in pack_gzip()
  /* Initialize the mapping length (0..255) -> length code (0..28) */
  length = 0i32;
  code = 0i32;
  while code < 29i32 - 1i32 {
    (*(ptr_to_globals as *mut globals2)).base_length[code as usize] = length;
    n = 0i32;
    while n < 1i32 << extra_lbits[code as usize] as libc::c_int {
      let fresh65 = length;
      length = length + 1;
      (*(ptr_to_globals as *mut globals2)).length_code[fresh65 as usize] = code as uch;
      n += 1
    }
    code += 1
  }
  /* Note that the length 255 (match length 258) can be represented
   * in two different ways: code 284 + 5 bits or code 285, so we
   * overwrite length_code[255] to use the best encoding:
   */
  (*(ptr_to_globals as *mut globals2)).length_code[(length - 1i32) as usize] = code as uch;
  /* Initialize the mapping dist (0..32K) -> dist code (0..29) */
  dist = 0i32; /* from now on, all distances are divided by 128 */
  code = 0i32;
  while code < 16i32 {
    (*(ptr_to_globals as *mut globals2)).base_dist[code as usize] = dist;
    n = 0i32;
    while n < 1i32 << extra_dbits[code as usize] as libc::c_int {
      let fresh66 = dist;
      dist = dist + 1;
      (*(ptr_to_globals as *mut globals2)).dist_code[fresh66 as usize] = code as uch;
      n += 1
    }
    code += 1
  }
  dist >>= 7i32;
  while code < 30i32 {
    (*(ptr_to_globals as *mut globals2)).base_dist[code as usize] = dist << 7i32;
    n = 0i32;
    while n < 1i32 << extra_dbits[code as usize] as libc::c_int - 7i32 {
      let fresh67 = dist;
      dist = dist + 1;
      (*(ptr_to_globals as *mut globals2)).dist_code[(256i32 + fresh67) as usize] = code as uch;
      n += 1
    }
    code += 1
  }
  /* Construct the codes of the static literal tree */
  //for (n = 0; n <= MAX_BITS; n++) // globals are zeroed in pack_gzip()
  //	G2.bl_count[n] = 0;
  n = 0i32;
  while n <= 143i32 {
    let fresh68 = n;
    n = n + 1;
    (*(ptr_to_globals as *mut globals2)).static_ltree[fresh68 as usize]
      .dl
      .len = 8i32 as ush
    //G2.bl_count[8]++;
  }
  //G2.bl_count[8] = 143 + 1;
  while n <= 255i32 {
    let fresh69 = n;
    n = n + 1;
    (*(ptr_to_globals as *mut globals2)).static_ltree[fresh69 as usize]
      .dl
      .len = 9i32 as ush
    //G2.bl_count[9]++;
  }
  //G2.bl_count[9] = 255 - 143;
  while n <= 279i32 {
    let fresh70 = n;
    n = n + 1;
    (*(ptr_to_globals as *mut globals2)).static_ltree[fresh70 as usize]
      .dl
      .len = 7i32 as ush
    //G2.bl_count[7]++;
  }
  //G2.bl_count[7] = 279 - 255;
  while n <= 287i32 {
    let fresh71 = n;
    n = n + 1;
    (*(ptr_to_globals as *mut globals2)).static_ltree[fresh71 as usize]
      .dl
      .len = 8i32 as ush
    //G2.bl_count[8]++;
  }
  //G2.bl_count[8] += 287 - 279;
  (*(ptr_to_globals as *mut globals2)).bl_count[7] = (279i32 - 255i32) as libc::c_uint;
  (*(ptr_to_globals as *mut globals2)).bl_count[8] =
    (143i32 + 1i32 + (287i32 - 279i32)) as libc::c_uint;
  (*(ptr_to_globals as *mut globals2)).bl_count[9] = (255i32 - 143i32) as libc::c_uint;
  /* Codes 286 and 287 do not exist, but we must include them in the
   * tree construction to get a canonical Huffman tree (longest code
   * all ones)
   */
  gen_codes(
    (*(ptr_to_globals as *mut globals2))
      .static_ltree
      .as_mut_ptr(),
    256i32 + 1i32 + 29i32 + 1i32,
  );
  /* The static distance tree is trivial: */
  n = 0i32;
  while n < 30i32 {
    (*(ptr_to_globals as *mut globals2)).static_dtree[n as usize]
      .dl
      .len = 5i32 as ush;
    (*(ptr_to_globals as *mut globals2)).static_dtree[n as usize]
      .fc
      .code = bi_reverse(n as libc::c_uint, 5i32) as ush;
    n += 1
  }
  /* Initialize the first block of the first file: */
  init_block();
}
/* ===========================================================================
 * Deflate in to out.
 * IN assertions: the input and output buffers are cleared.
 */
unsafe extern "C" fn zip() {
  let mut deflate_flags: libc::c_uint = 0;
  //G1.outcnt = 0; // globals are zeroed in pack_gzip()
  /* Write the header to the gzip file. See algorithm.doc for the format */
  /* magic header for gzip files: 1F 8B */
  /* compression method: 8 (DEFLATED) */
  /* general flags: 0 */
  put_32bit(0x88b1fi32 as ulg); /* Unix timestamp */
  put_32bit(0i32 as ulg);
  /* Write deflated file to zip file */
  (*ptr_to_globals.offset(-1)).crc = !0i32 as u32; /* extra flags. OS id = 3 (Unix) */
  bi_init();
  ct_init();
  lm_init();
  deflate_flags = 0x300i32 as libc::c_uint;
  put_16bit(deflate_flags as ush);
  /* The above 32-bit misaligns outbuf (10 bytes are stored), flush it */
  flush_outbuf_if_32bit_optimized();
  deflate();
  /* Write the crc and uncompressed size */
  put_32bit(!(*ptr_to_globals.offset(-1)).crc);
  put_32bit((*ptr_to_globals.offset(-1)).isize_0);
  flush_outbuf();
}
/* ======================================================================== */
unsafe extern "C" fn pack_gzip(mut _xstate: *mut transformer_state_t) -> libc::c_longlong {
  /* Reinit G1.xxx except pointers to allocated buffers, and entire G2 */
  memset(
    &mut (*ptr_to_globals.offset(-1)).crc as *mut u32 as *mut libc::c_void,
    0i32,
    (::std::mem::size_of::<globals>() as libc::c_ulong)
      .wrapping_sub(40u64)
      .wrapping_add(::std::mem::size_of::<globals2>() as libc::c_ulong),
  );
  /* Clear input and output buffers */
  //G1.outcnt = 0;
  //G1.isize = 0;
  /* Reinit G2.xxx */
  let ref mut fresh72 = (*(ptr_to_globals as *mut globals2)).l_desc.dyn_tree;
  *fresh72 = (*(ptr_to_globals as *mut globals2)).dyn_ltree.as_mut_ptr();
  let ref mut fresh73 = (*(ptr_to_globals as *mut globals2)).l_desc.static_tree;
  *fresh73 = (*(ptr_to_globals as *mut globals2))
    .static_ltree
    .as_mut_ptr();
  let ref mut fresh74 = (*(ptr_to_globals as *mut globals2)).l_desc.extra_bits;
  *fresh74 = extra_lbits.as_ptr();
  (*(ptr_to_globals as *mut globals2)).l_desc.extra_base = 256i32 + 1i32;
  (*(ptr_to_globals as *mut globals2)).l_desc.elems = 256i32 + 1i32 + 29i32;
  (*(ptr_to_globals as *mut globals2)).l_desc.max_length = 15i32;
  //G2.l_desc.max_code     = 0;
  let ref mut fresh75 = (*(ptr_to_globals as *mut globals2)).d_desc.dyn_tree;
  *fresh75 = (*(ptr_to_globals as *mut globals2)).dyn_dtree.as_mut_ptr();
  let ref mut fresh76 = (*(ptr_to_globals as *mut globals2)).d_desc.static_tree;
  *fresh76 = (*(ptr_to_globals as *mut globals2))
    .static_dtree
    .as_mut_ptr();
  let ref mut fresh77 = (*(ptr_to_globals as *mut globals2)).d_desc.extra_bits;
  *fresh77 = extra_dbits.as_ptr();
  //G2.d_desc.extra_base   = 0;
  (*(ptr_to_globals as *mut globals2)).d_desc.elems = 30i32;
  (*(ptr_to_globals as *mut globals2)).d_desc.max_length = 15i32;
  //G2.d_desc.max_code     = 0;
  let ref mut fresh78 = (*(ptr_to_globals as *mut globals2)).bl_desc.dyn_tree;
  *fresh78 = (*(ptr_to_globals as *mut globals2)).bl_tree.as_mut_ptr();
  //G2.bl_desc.static_tree = NULL;
  let ref mut fresh79 = (*(ptr_to_globals as *mut globals2)).bl_desc.extra_bits;
  *fresh79 = extra_blbits.as_ptr();
  (*(ptr_to_globals as *mut globals2)).bl_desc.elems = 19i32;
  (*(ptr_to_globals as *mut globals2)).bl_desc.max_length = 7i32;
  //G2.bl_desc.max_code    = 0;
  zip();
  return 0i32 as libc::c_longlong;
}
static mut gzip_longopts: [libc::c_char; 105] = [
  115, 116, 100, 111, 117, 116, 0, 0, 99, 116, 111, 45, 115, 116, 100, 111, 117, 116, 0, 0, 99,
  102, 111, 114, 99, 101, 0, 0, 102, 118, 101, 114, 98, 111, 115, 101, 0, 0, 118, 100, 101, 99,
  111, 109, 112, 114, 101, 115, 115, 0, 0, 100, 117, 110, 99, 111, 109, 112, 114, 101, 115, 115, 0,
  0, 100, 116, 101, 115, 116, 0, 0, 116, 113, 117, 105, 101, 116, 0, 0, 113, 102, 97, 115, 116, 0,
  0, 49, 98, 101, 115, 116, 0, 0, 57, 110, 111, 45, 110, 97, 109, 101, 0, 0, 110, 0,
];
/*
 * Linux kernel build uses gzip -d -n. We accept and ignore -n.
 * Man page says:
 * -n --no-name
 * gzip: do not save the original file name and time stamp.
 * (The original name is always saved if the name had to be truncated.)
 * gunzip: do not restore the original file name/time even if present
 * (remove only the gzip suffix from the compressed file name).
 * This option is the default when decompressing.
 * -N --name
 * gzip: always save the original file name and time stamp (this is the default)
 * gunzip: restore the original file name and time stamp if present.
 */
#[no_mangle]
pub unsafe extern "C" fn gzip_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opt: libc::c_uint = 0;
  let ref mut fresh80 =
    *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
      as *mut *mut globals);
  *fresh80 = (xzalloc(
    (::std::mem::size_of::<globals>() as libc::c_ulong)
      .wrapping_add(::std::mem::size_of::<globals2>() as libc::c_ulong),
  ) as *mut libc::c_char)
    .offset(::std::mem::size_of::<globals>() as libc::c_ulong as isize)
    as *mut libc::c_void as *mut globals;
  asm!("" : : : "memory" : "volatile");
  /* Must match bbunzip's constants OPT_STDOUT, OPT_FORCE! */
  opt = getopt32long(
    argv,
    b"cfkvqdtn123456789\x00" as *const u8 as *const libc::c_char,
    gzip_longopts.as_ptr(),
  );
  /* gunzip_main may not be visible... */
  if opt & (BBUNPK_OPT_DECOMPRESS as libc::c_int | BBUNPK_OPT_TEST as libc::c_int) as libc::c_uint
    != 0
  {
    /* -d and/or -t */
    return gunzip_main(argc, argv);
  } /* retain only -cfkvq */
  option_mask32 &= ((1i32 << 5i32) - 1i32) as libc::c_uint;
  /* Allocate all global buffers (for DYN_ALLOC option) */
  let ref mut fresh81 = (*ptr_to_globals.offset(-1)).l_buf;
  *fresh81 = xzalloc(
    (((0x2000i32 as libc::c_long + 1i64) / 2i32 as libc::c_long) as size_t)
      .wrapping_mul(2i32 as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<uch>() as libc::c_ulong),
  ) as *mut uch;
  let ref mut fresh82 = (*ptr_to_globals.offset(-1)).outbuf;
  *fresh82 = xzalloc(
    (((8192i32 as libc::c_long + 1i64) / 2i32 as libc::c_long) as size_t)
      .wrapping_mul(2i32 as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<uch>() as libc::c_ulong),
  ) as *mut uch;
  let ref mut fresh83 = (*ptr_to_globals.offset(-1)).d_buf;
  *fresh83 = xzalloc(
    (((0x2000i32 as libc::c_long + 1i64) / 2i32 as libc::c_long) as size_t)
      .wrapping_mul(2i32 as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<ush>() as libc::c_ulong),
  ) as *mut ush;
  let ref mut fresh84 = (*ptr_to_globals.offset(-1)).window;
  *fresh84 = xzalloc(
    (((2i64 * 0x8000i32 as libc::c_long + 1i64) / 2i32 as libc::c_long) as size_t)
      .wrapping_mul(2i32 as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<uch>() as libc::c_ulong),
  ) as *mut uch;
  let ref mut fresh85 = (*ptr_to_globals.offset(-1)).prev;
  *fresh85 = xzalloc(
    ((((1i64 << 16i32) + 1i64) / 2i32 as libc::c_long) as size_t)
      .wrapping_mul(2i32 as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<ush>() as libc::c_ulong),
  ) as *mut ush;
  /* Initialize the CRC32 table */
  global_crc32_new_table_le();
  argv = argv.offset(optind as isize);
  return bbunpack(
    argv,
    Some(pack_gzip as unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong),
    Some(
      append_ext
        as unsafe extern "C" fn(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char,
    ),
    b"gz\x00" as *const u8 as *const libc::c_char,
  );
}

use crate::libbb::xfuncs_printf::xmalloc;
use crate::librb::size_t;
use crate::librb::smallint;
use libc;
use libc::free;
use libc::off_t;
use libc::sigset_t;
use libc::ssize_t;
use libc::time_t;
extern "C" {

  #[no_mangle]
  fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
  #[no_mangle]
  fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
  #[no_mangle]
  fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn malloc_or_warn(size: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn xfunc_die() -> !;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn crc32_filltable(tbl256: *mut u32, endian: libc::c_int) -> *mut u32;
  #[no_mangle]
  fn transformer_write(
    xstate: *mut transformer_state_t,
    buf: *const libc::c_void,
    bufsize: size_t,
  ) -> ssize_t;
  #[no_mangle]
  fn check_signature16(xstate: *mut transformer_state_t, magic16: libc::c_uint) -> libc::c_int;
}

pub type __jmp_buf = [libc::c_long; 8];

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
  pub __jmpbuf: __jmp_buf,
  pub __mask_was_saved: libc::c_int,
  pub __saved_mask: sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];

pub type C2RustUnnamed = libc::c_uint;
pub const XZ_MAGIC2a: C2RustUnnamed = 90;
pub const XZ_MAGIC1a: C2RustUnnamed = 1484404733;
pub const XZ_MAGIC2: C2RustUnnamed = 5920890;
pub const XZ_MAGIC1: C2RustUnnamed = 14333;
pub const BZIP2_MAGIC: C2RustUnnamed = 23106;
pub const GZIP_MAGIC: C2RustUnnamed = 35615;
pub const COMPRESS_MAGIC: C2RustUnnamed = 40223;

/* Structure holding all the housekeeping data, including IO buffers and
 * memory that persists between calls to bunzip
 * Found the most used member:
 *  cat this_file.c | sed -e 's/"/ /g' -e "s/'/ /g" | xargs -n1 \
 *  | grep 'bd->' | sed 's/^.*bd->/bd->/' | sort | $PAGER
 * and moved it (inbufBitCount) to offset 0.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bunzip_data {
  pub inbufBitCount: libc::c_uint,
  pub inbufBits: libc::c_uint,
  pub in_fd: libc::c_int,
  pub out_fd: libc::c_int,
  pub inbufCount: libc::c_int,
  pub inbufPos: libc::c_int,
  pub inbuf: *mut u8,
  pub writeCopies: libc::c_int,
  pub writePos: libc::c_int,
  pub writeRunCountdown: libc::c_int,
  pub writeCount: libc::c_int,
  pub writeCurrent: libc::c_int,
  pub headerCRC: u32,
  pub totalCRC: u32,
  pub writeCRC: u32,
  pub dbuf: *mut u32,
  pub dbufSize: libc::c_uint,
  pub jmpbuf: *mut jmp_buf,
  pub crc32Table: [u32; 256],
  pub selectors: [u8; 32768],
  pub groups: [group_data; 6],
  /* Huffman coding tables */
}
/* This is what we know about each Huffman coding group */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group_data {
  pub limit: [libc::c_int; 21],
  pub base: [libc::c_int; 20],
  pub permute: [libc::c_int; 258],
  pub minLen: libc::c_int,
  pub maxLen: libc::c_int,
}
pub const h0: C2RustUnnamed_0 = 26672;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const BZh0: C2RustUnnamed_0 = 1113221168;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct transformer_state_t {
  pub signature_skipped: smallint,
  pub xformer: Option<unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong>,
  pub src_fd: libc::c_int,
  pub dst_fd: libc::c_int,
  pub mem_output_size_max: size_t,
  pub mem_output_size: size_t,
  pub mem_output_buf: *mut libc::c_char,
  pub bytes_out: off_t,
  pub bytes_in: off_t,
  pub crc32: u32,
  pub mtime: time_t,
  pub magic: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
  pub b: [u8; 8],
  pub b16: [u16; 4],
  pub b32: [u32; 2],
}
/* Return the next nnn bits of input.  All reads from the compressed input
are done through this function.  All reads are big endian */
unsafe extern "C" fn get_bits(
  mut bd: *mut bunzip_data,
  mut bits_wanted: libc::c_int,
) -> libc::c_uint {
  let mut bits: libc::c_uint = 0i32 as libc::c_uint;
  /* Cache bd->inbufBitCount in a CPU register (hopefully): */
  let mut bit_count: libc::c_int = (*bd).inbufBitCount as libc::c_int;
  /* If we need to get more data from the byte buffer, do so.  (Loop getting
  one byte at a time to enforce endianness and avoid unaligned access.) */
  while bit_count < bits_wanted {
    /* If we need to read more data from file into byte buffer, do so */
    if (*bd).inbufPos == (*bd).inbufCount {
      /* if "no input fd" case: in_fd == -1, read fails, we jump */
      (*bd).inbufCount = read(
        (*bd).in_fd,
        (*bd).inbuf as *mut libc::c_void,
        4096i32 as size_t,
      ) as libc::c_int;
      if (*bd).inbufCount <= 0i32 {
        longjmp((*(*bd).jmpbuf).as_mut_ptr(), -3i32);
      }
      (*bd).inbufPos = 0i32
    }
    /* Avoid 32-bit overflow (dump bit buffer to top of output) */
    if bit_count >= 24i32 {
      bits = (*bd).inbufBits & (1u32 << bit_count).wrapping_sub(1i32 as libc::c_uint);
      bits_wanted -= bit_count;
      bits <<= bits_wanted;
      bit_count = 0i32
    }
    /* Grab next 8 bits of input from buffer. */
    let fresh0 = (*bd).inbufPos;
    (*bd).inbufPos = (*bd).inbufPos + 1;
    (*bd).inbufBits =
      (*bd).inbufBits << 8i32 | *(*bd).inbuf.offset(fresh0 as isize) as libc::c_uint;
    bit_count += 8i32
  }
  /* Calculate result */
  bit_count -= bits_wanted;
  (*bd).inbufBitCount = bit_count as libc::c_uint;
  bits |= (*bd).inbufBits >> bit_count & ((1i32 << bits_wanted) - 1i32) as libc::c_uint;
  return bits;
}
//#define get_bits(bd, n) (dbg("%d:get_bits()", __LINE__), get_bits(bd, n))
/* Unpacks the next block and sets up for the inverse Burrows-Wheeler step. */
unsafe extern "C" fn get_next_block(mut bd: *mut bunzip_data) -> libc::c_int {
  let mut groupCount: libc::c_int = 0; /* for compiler */
  let mut selector: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  let mut symCount: libc::c_int = 0;
  let mut symTotal: libc::c_int = 0;
  let mut nSelectors: libc::c_int = 0;
  let mut byteCount: [libc::c_int; 256] = [0; 256];
  let mut uc: u8 = 0;
  let mut symToByte: [u8; 256] = [0; 256];
  let mut mtfSymbol: [u8; 256] = [0; 256];
  let mut selectors: *mut u8 = 0 as *mut u8;
  let mut dbuf: *mut u32 = 0 as *mut u32;
  let mut origPtr: libc::c_uint = 0;
  let mut t: libc::c_uint = 0;
  let mut dbufCount: libc::c_uint = 0;
  let mut runPos: libc::c_uint = 0;
  let mut runCnt: libc::c_uint = 0;
  runCnt = runCnt;
  dbuf = (*bd).dbuf;
  selectors = (*bd).selectors.as_mut_ptr();
  /* In bbox, we are ok with aborting through setjmp which is set up in start_bunzip */
  /* Read in header signature and CRC, then validate signature.
  (last block signature means CRC is for whole file, return now) */
  i = get_bits(bd, 24i32) as libc::c_int;
  j = get_bits(bd, 24i32) as libc::c_int;
  (*bd).headerCRC = get_bits(bd, 32i32);
  if i == 0x177245i32 && j == 0x385090i32 {
    return -1i32;
  }
  if i != 0x314159i32 || j != 0x265359i32 {
    return -2i32;
  }
  /* We can add support for blockRandomised if anybody complains.  There was
  some code for this in busybox 1.0.0-pre3, but nobody ever noticed that
  it didn't actually work. */
  if get_bits(bd, 1i32) != 0 {
    return -7i32;
  }
  origPtr = get_bits(bd, 24i32);
  if origPtr > (*bd).dbufSize {
    return -5i32;
  }
  /* mapping table: if some byte values are never used (encoding things
  like ascii text), the compression code removes the gaps to have fewer
  symbols to deal with, and writes a sparse bitfield indicating which
  values were present.  We make a translation table to convert the symbols
  back to the corresponding bytes. */
  symTotal = 0i32;
  i = 0i32;
  t = get_bits(bd, 16i32);
  loop {
    if t & (1i32 << 15i32) as libc::c_uint != 0 {
      let mut inner_map: libc::c_uint = get_bits(bd, 16i32);
      loop {
        if inner_map & (1i32 << 15i32) as libc::c_uint != 0 {
          let fresh1 = symTotal;
          symTotal = symTotal + 1;
          symToByte[fresh1 as usize] = i as u8
        }
        inner_map <<= 1i32;
        i += 1;
        if !(i & 15i32 != 0) {
          break;
        }
      }
      i -= 16i32
    }
    t <<= 1i32;
    i += 16i32;
    if !(i < 256i32) {
      break;
    }
  }
  /* How many different Huffman coding groups does this block use? */
  groupCount = get_bits(bd, 3i32) as libc::c_int;
  if groupCount < 2i32 || groupCount > 6i32 {
    return -5i32;
  }
  /* nSelectors: Every GROUP_SIZE many symbols we select a new Huffman coding
  group.  Read in the group selector list, which is stored as MTF encoded
  bit runs.  (MTF=Move To Front, as each value is used it's moved to the
  start of the list.) */
  i = 0i32;
  while i < groupCount {
    mtfSymbol[i as usize] = i as u8;
    i += 1
  }
  nSelectors = get_bits(bd, 15i32) as libc::c_int;
  if nSelectors == 0 {
    return -5i32;
  }
  i = 0i32;
  while i < nSelectors {
    let mut tmp_byte: u8 = 0;
    /* Get next value */
    let mut n: libc::c_int = 0i32;
    while get_bits(bd, 1i32) != 0 {
      n += 1;
      if n >= groupCount {
        return -5i32;
      }
    }
    /* Decode MTF to get the next selector */
    tmp_byte = mtfSymbol[n as usize];
    loop {
      n -= 1;
      if !(n >= 0i32) {
        break;
      }
      mtfSymbol[(n + 1i32) as usize] = mtfSymbol[n as usize]
    }
    //We catch it later, in the second loop where we use selectors[i].
    //Maybe this is a better place, though?
    //		if (tmp_byte >= groupCount) {
    //			dbg("%d: selectors[%d]:%d groupCount:%d",
    //					__LINE__, i, tmp_byte, groupCount);
    //			return RETVAL_DATA_ERROR;
    //		}
    let ref mut fresh2 = *selectors.offset(i as isize);
    *fresh2 = tmp_byte;
    mtfSymbol[0] = *fresh2;
    i += 1
  }
  /* Read the Huffman coding tables for each group, which code for symTotal
  literal symbols, plus two run symbols (RUNA, RUNB) */
  symCount = symTotal + 2i32;
  j = 0i32;
  while j < groupCount {
    let mut length: [u8; 258] = [0; 258];
    /* 8 bits is ALMOST enough for temp[], see below */
    let mut temp: [libc::c_uint; 21] = [0; 21];
    let mut hufGroup: *mut group_data = 0 as *mut group_data;
    let mut base: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut limit: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut minLen: libc::c_int = 0;
    let mut maxLen: libc::c_int = 0;
    let mut pp: libc::c_int = 0;
    let mut len_m1: libc::c_int = 0;
    /* Read Huffman code lengths for each symbol.  They're stored in
    a way similar to mtf; record a starting value for the first symbol,
    and an offset from the previous value for every symbol after that.
    (Subtracting 1 before the loop and then adding it back at the end is
    an optimization that makes the test inside the loop simpler: symbol
    length 0 becomes negative, so an unsigned inequality catches it.) */
    len_m1 = get_bits(bd, 5i32).wrapping_sub(1i32 as libc::c_uint) as libc::c_int;
    i = 0i32;
    while i < symCount {
      loop {
        let mut two_bits: libc::c_int = 0;
        if len_m1 as libc::c_uint > (20i32 - 1i32) as libc::c_uint {
          return -5i32;
        }
        /* If first bit is 0, stop.  Else second bit indicates whether
        to increment or decrement the value.  Optimization: grab 2
        bits and unget the second if the first was 0. */
        two_bits = get_bits(bd, 2i32) as libc::c_int;
        if two_bits < 2i32 {
          (*bd).inbufBitCount = (*bd).inbufBitCount.wrapping_add(1);
          break;
        } else {
          /* Add one if second bit 1, else subtract 1.  Avoids if/else */
          len_m1 += (two_bits + 1i32 & 2i32) - 1i32
        }
      }
      /* Correct for the initial -1, to get the final symbol length */
      length[i as usize] = (len_m1 + 1i32) as u8;
      i += 1
    }
    /* Find largest and smallest lengths in this group */
    maxLen = length[0] as libc::c_int;
    minLen = maxLen;
    i = 1i32;
    while i < symCount {
      if length[i as usize] as libc::c_int > maxLen {
        maxLen = length[i as usize] as libc::c_int
      } else if (length[i as usize] as libc::c_int) < minLen {
        minLen = length[i as usize] as libc::c_int
      }
      i += 1
    }
    /* Calculate permute[], base[], and limit[] tables from length[].
     *
     * permute[] is the lookup table for converting Huffman coded symbols
     * into decoded symbols.  base[] is the amount to subtract from the
     * value of a Huffman symbol of a given length when using permute[].
     *
     * limit[] indicates the largest numerical value a symbol with a given
     * number of bits can have.  This is how the Huffman codes can vary in
     * length: each code with a value>limit[length] needs another bit.
     */
    hufGroup = (*bd).groups.as_mut_ptr().offset(j as isize);
    (*hufGroup).minLen = minLen;
    (*hufGroup).maxLen = maxLen;
    /* Note that minLen can't be smaller than 1, so we adjust the base
    and limit array pointers so we're not always wasting the first
    entry.  We do this again when using them (during symbol decoding). */
    base = (*hufGroup).base.as_mut_ptr().offset(-1);
    limit = (*hufGroup).limit.as_mut_ptr().offset(-1);
    /* Calculate permute[].  Concurrently, initialize temp[] and limit[]. */
    pp = 0i32;
    i = minLen;
    while i <= maxLen {
      let mut k: libc::c_int = 0;
      let ref mut fresh3 = *limit.offset(i as isize);
      *fresh3 = 0i32;
      temp[i as usize] = *fresh3 as libc::c_uint;
      k = 0i32;
      while k < symCount {
        if length[k as usize] as libc::c_int == i {
          let fresh4 = pp;
          pp = pp + 1;
          (*hufGroup).permute[fresh4 as usize] = k
        }
        k += 1
      }
      i += 1
    }
    /* Count symbols coded for at each bit length */
    /* NB: in pathological cases, temp[8] can end ip being 256.
     * That's why u8 is too small for temp[]. */
    i = 0i32;
    while i < symCount {
      temp[length[i as usize] as usize] = temp[length[i as usize] as usize].wrapping_add(1);
      i += 1
    }
    /* Calculate limit[] (the largest symbol-coding value at each bit
     * length, which is (previous limit<<1)+symbols at this level), and
     * base[] (number of symbols to ignore at each bit length, which is
     * limit minus the cumulative count of symbols coded for already). */
    t = 0i32 as libc::c_uint;
    pp = t as libc::c_int;
    i = minLen;
    while i < maxLen {
      let mut temp_i: libc::c_uint = temp[i as usize];
      pp = (pp as libc::c_uint).wrapping_add(temp_i) as libc::c_int as libc::c_int;
      /* We read the largest possible symbol size and then unget bits
      after determining how many we need, and those extra bits could
      be set to anything.  (They're noise from future symbols.)  At
      each level we're really only interested in the first few bits,
      so here we set all the trailing to-be-ignored bits to 1 so they
      don't affect the value>limit[length] comparison. */
      *limit.offset(i as isize) = (pp << maxLen - i) - 1i32; /* Sentinel value for reading next sym. */
      pp <<= 1i32;
      t = t.wrapping_add(temp_i);
      i += 1;
      *base.offset(i as isize) = (pp as libc::c_uint).wrapping_sub(t) as libc::c_int
    }
    *limit.offset(maxLen as isize) = (pp as libc::c_uint)
      .wrapping_add(temp[maxLen as usize])
      .wrapping_sub(1i32 as libc::c_uint) as libc::c_int;
    *limit.offset((maxLen + 1i32) as isize) = 2147483647i32;
    *base.offset(minLen as isize) = 0i32;
    j += 1
  }
  /* We've finished reading and digesting the block header.  Now read this
  block's Huffman coded symbols from the file and undo the Huffman coding
  and run length encoding, saving the result into dbuf[dbufCount++] = uc */
  /* Initialize symbol occurrence counters and symbol Move To Front table */
  /*memset(byteCount, 0, sizeof(byteCount)); - smaller, but slower */
  i = 0i32;
  while i < 256i32 {
    byteCount[i as usize] = 0i32;
    mtfSymbol[i as usize] = i as u8;
    i += 1
  }
  /* Loop through compressed symbols. */
  selector = 0i32;
  dbufCount = selector as libc::c_uint;
  runPos = dbufCount;
  's_565: loop {
    let mut hufGroup_0: *mut group_data = 0 as *mut group_data;
    let mut base_0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut limit_0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nextSym: libc::c_int = 0;
    let mut ngrp: u8 = 0;
    /* Fetch next Huffman coding group from list. */
    symCount = 50i32 - 1i32;
    if selector >= nSelectors {
      return -5i32;
    }
    let fresh5 = selector;
    selector = selector + 1;
    ngrp = *selectors.offset(fresh5 as isize);
    if ngrp as libc::c_int >= groupCount {
      return -5i32;
    }
    hufGroup_0 = (*bd)
      .groups
      .as_mut_ptr()
      .offset(ngrp as libc::c_int as isize);
    base_0 = (*hufGroup_0).base.as_mut_ptr().offset(-1);
    limit_0 = (*hufGroup_0).limit.as_mut_ptr().offset(-1);
    loop {
      /* Read next Huffman-coded symbol. */
      /* Note: It is far cheaper to read maxLen bits and back up than it is
        to read minLen bits and then add additional bit at a time, testing
        as we go.  Because there is a trailing last block (with file CRC),
        there is no danger of the overread causing an unexpected EOF for a
        valid compressed file.
      */
      let mut current_block_126: u64; /* unoptimized equivalent */
      let mut new_cnt: libc::c_int = 0;
      loop {
        new_cnt = (*bd)
          .inbufBitCount
          .wrapping_sub((*hufGroup_0).maxLen as libc::c_uint) as libc::c_int;
        if !(new_cnt < 0i32) {
          current_block_126 = 10945915984064580713;
          break;
        }
        /* As a further optimization, we do the read inline
          (falling back to a call to get_bits if the buffer runs dry).
        */
        /* bd->inbufBitCount < hufGroup->maxLen */
        if (*bd).inbufPos == (*bd).inbufCount {
          nextSym = get_bits(bd, (*hufGroup_0).maxLen) as libc::c_int; /* "bd->inbufBitCount -= hufGroup->maxLen;" */
          current_block_126 = 3024573345131975588;
          break;
        } else {
          let fresh6 = (*bd).inbufPos;
          (*bd).inbufPos = (*bd).inbufPos + 1;
          (*bd).inbufBits =
            (*bd).inbufBits << 8i32 | *(*bd).inbuf.offset(fresh6 as isize) as libc::c_uint;
          (*bd).inbufBitCount = (*bd).inbufBitCount.wrapping_add(8i32 as libc::c_uint)
        }
      }
      match current_block_126 {
        10945915984064580713 => {
          (*bd).inbufBitCount = new_cnt as libc::c_uint;
          nextSym = ((*bd).inbufBits >> new_cnt
            & ((1i32 << (*hufGroup_0).maxLen) - 1i32) as libc::c_uint)
            as libc::c_int
        }
        _ => {}
      }
      /* Figure how many bits are in next symbol and unget extras */
      i = (*hufGroup_0).minLen;
      while nextSym > *limit_0.offset(i as isize) {
        i += 1
      }
      j = (*hufGroup_0).maxLen - i;
      if j < 0i32 {
        return -5i32;
      }
      (*bd).inbufBitCount = (*bd).inbufBitCount.wrapping_add(j as libc::c_uint);
      /* Huffman decode value to get nextSym (with bounds checking) */
      nextSym = (nextSym >> j) - *base_0.offset(i as isize);
      if nextSym as libc::c_uint >= 258i32 as libc::c_uint {
        return -5i32;
      }
      nextSym = (*hufGroup_0).permute[nextSym as usize];
      /* We have now decoded the symbol, which indicates either a new literal
      byte, or a repeated run of the most recent literal byte.  First,
      check if nextSym indicates a repeated run, and if so loop collecting
      how many times to repeat the last literal. */
      if nextSym as libc::c_uint <= 1i32 as libc::c_uint {
        /* RUNA or RUNB */
        /* If this is the start of a new run, zero out counter */
        if runPos == 0i32 as libc::c_uint {
          runPos = 1i32 as libc::c_uint;
          runCnt = 0i32 as libc::c_uint
        }
        /* Neat trick that saves 1 symbol: instead of or-ing 0 or 1 at
        each bit position, add 1 or 2 instead.  For example,
        1011 is 1<<0 + 1<<1 + 2<<2.  1010 is 2<<0 + 2<<1 + 1<<2.
        You can make any bit pattern that way using 1 less symbol than
        the basic or 0/1 method (except all bits 0, which would use no
        symbols, but a run of length 0 doesn't mean anything in this
        context).  Thus space is saved. */
        runCnt = runCnt.wrapping_add(runPos << nextSym); /* +runPos if RUNA; +2*runPos if RUNB */
        //The 32-bit overflow of runCnt wasn't yet seen, but probably can happen.
        //This would be the fix (catches too large count way before it can overflow):
        //			if (runCnt > bd->dbufSize) {
        //				dbg("runCnt:%u > dbufSize:%u RETVAL_DATA_ERROR",
        //						runCnt, bd->dbufSize);
        //				return RETVAL_DATA_ERROR;
        //			}
        if runPos < (*bd).dbufSize {
          runPos <<= 1i32
        }
      } else {
        /* When we hit the first non-run symbol after a run, we now know
        how many times to repeat the last literal, so append that many
        copies to our buffer of decoded symbols (dbuf) now.  (The last
        literal used is the one at the head of the mtfSymbol array.) */
        if runPos != 0i32 as libc::c_uint {
          let mut tmp_byte_0: u8 = 0;
          if dbufCount.wrapping_add(runCnt) > (*bd).dbufSize {
            return -5i32;
          }
          tmp_byte_0 = symToByte[mtfSymbol[0] as usize];
          byteCount[tmp_byte_0 as usize] = (byteCount[tmp_byte_0 as usize] as libc::c_uint)
            .wrapping_add(runCnt) as libc::c_int
            as libc::c_int;
          loop {
            runCnt = runCnt.wrapping_sub(1);
            if !(runCnt as libc::c_int >= 0i32) {
              break;
            }
            let fresh7 = dbufCount;
            dbufCount = dbufCount.wrapping_add(1);
            *dbuf.offset(fresh7 as isize) = tmp_byte_0 as u32
          }
          runPos = 0i32 as libc::c_uint
        }
        /* Is this the terminating symbol? */
        if nextSym > symTotal {
          break 's_565;
        }
        /* At this point, nextSym indicates a new literal character.  Subtract
        one to get the position in the MTF array at which this literal is
        currently to be found.  (Note that the result can't be -1 or 0,
        because 0 and 1 are RUNA and RUNB.  But another instance of the
        first symbol in the mtf array, position 0, would have been handled
        as part of a run above.  Therefore 1 unused mtf position minus
        2 non-literal nextSym values equals -1.) */
        if dbufCount >= (*bd).dbufSize {
          return -5i32;
        }
        i = nextSym - 1i32;
        uc = mtfSymbol[i as usize];
        loop
        /* Adjust the MTF array.  Since we typically expect to move only a
         * small number of symbols, and are bound by 256 in any case, using
         * memmove here would typically be bigger and slower due to function
         * call overhead and other assorted setup costs. */
        {
          mtfSymbol[i as usize] = mtfSymbol[(i - 1i32) as usize];
          i -= 1;
          if !(i != 0) {
            break;
          }
        }
        mtfSymbol[0] = uc;
        uc = symToByte[uc as usize];
        /* We have our literal byte.  Save it into dbuf. */
        byteCount[uc as usize] += 1;
        let fresh8 = dbufCount;
        dbufCount = dbufCount.wrapping_add(1);
        *dbuf.offset(fresh8 as isize) = uc as u32
      }
      /* Skip group initialization if we're not done with this group.  Done
       * this way to avoid compiler warning. */
      symCount -= 1;
      if !(symCount >= 0i32) {
        break;
      }
    }
  }
  /* At this point, we've read all the Huffman-coded symbols (and repeated
    runs) for this block from the input stream, and decoded them into the
    intermediate buffer.  There are dbufCount many decoded bytes in dbuf[].
    Now undo the Burrows-Wheeler transform on dbuf.
    See http://dogma.net/markn/articles/bwt/bwt.htm
  */
  /* Turn byteCount into cumulative occurrence counts of 0 to n-1. */
  j = 0i32;
  i = 0i32;
  while i < 256i32 {
    let mut tmp_count: libc::c_int = j + byteCount[i as usize];
    byteCount[i as usize] = j;
    j = tmp_count;
    i += 1
  }
  /* Figure out what order dbuf would be in if we sorted it. */
  i = 0i32;
  while (i as libc::c_uint) < dbufCount {
    let mut tmp_byte_1: u8 = *dbuf.offset(i as isize) as u8;
    let mut tmp_count_0: libc::c_int = byteCount[tmp_byte_1 as usize];
    let ref mut fresh9 = *dbuf.offset(tmp_count_0 as isize);
    *fresh9 |= (i << 8i32) as libc::c_uint;
    byteCount[tmp_byte_1 as usize] = tmp_count_0 + 1i32;
    i += 1
  }
  /* Decode first byte by hand to initialize "previous" byte.  Note that it
  doesn't get output, and if the first three characters are identical
  it doesn't qualify as a run (hence writeRunCountdown=5). */
  if dbufCount != 0 {
    let mut tmp: u32 = 0;
    if origPtr as libc::c_int as libc::c_uint >= dbufCount {
      return -5i32;
    }
    tmp = *dbuf.offset(origPtr as isize);
    (*bd).writeCurrent = tmp as u8 as libc::c_int;
    (*bd).writePos = (tmp >> 8i32) as libc::c_int;
    (*bd).writeRunCountdown = 5i32
  }
  (*bd).writeCount = dbufCount as libc::c_int;
  return 0i32;
}
/* Undo Burrows-Wheeler transform on intermediate buffer to produce output.
   If start_bunzip was initialized with out_fd=-1, then up to len bytes of
   data are written to outbuf.  Return value is number of bytes written or
   error (all errors are negative numbers).  If out_fd!=-1, outbuf and len
   are ignored, data is written to out_fd and return is RETVAL_OK or error.

   NB: read_bunzip returns < 0 on error, or the number of *unfilled* bytes
   in outbuf. IOW: on EOF returns len ("all bytes are not filled"), not 0.
   (Why? This allows to get rid of one local variable)
*/
unsafe extern "C" fn read_bunzip(
  mut bd: *mut bunzip_data,
  mut outbuf: *mut libc::c_char,
  mut len: libc::c_int,
) -> libc::c_int {
  let mut current_block: u64;
  let mut dbuf: *const u32 = 0 as *const u32;
  let mut pos: libc::c_int = 0;
  let mut current: libc::c_int = 0;
  let mut previous: libc::c_int = 0;
  let mut CRC: u32 = 0;
  /* If we already have error/end indicator, return it */
  if (*bd).writeCount < 0i32 {
    return (*bd).writeCount;
  }
  dbuf = (*bd).dbuf;
  /* Register-cached state (hopefully): */
  pos = (*bd).writePos; /* small loss on x86-32 (not enough regs), win on x86-64 */
  current = (*bd).writeCurrent;
  CRC = (*bd).writeCRC;
  /* We will always have pending decoded data to write into the output
  buffer unless this is the very first call (in which case we haven't
  Huffman-decoded a block into the intermediate buffer yet). */
  if (*bd).writeCopies != 0 {
    current_block = 240530425663761810;
  } else {
    current_block = 7333393191927787629;
  }
  'c_10920: loop {
    match current_block {
      7333393191927787629 => {
        /* Refill the intermediate buffer by Huffman-decoding next block of input */
        let mut r: libc::c_int = get_next_block(bd);
        if r != 0 {
          /* error/end */
          (*bd).writeCount = r;
          return if r != -1i32 { r } else { len };
        }
        CRC = !0i32 as u32;
        pos = (*bd).writePos;
        current = (*bd).writeCurrent;
        current_block = 7525104114255620856;
      }
      _ =>
      /* Unlikely branch */
      /*--bd->writeCopies;*/
      /*continue;*/
      /* Same, but (ab)using other existing --writeCopies operation
       * (and this if() compiles into just test+branch pair): */
      /* Inside the loop, writeCopies means extra copies (beyond 1) */
      {
        (*bd).writeCopies -= 1;
        /* Loop outputting bytes */
        current_block = 3640593987805443782; /* for(;;) */
      }
    }
    loop {
      match current_block {
        3640593987805443782 =>
        /* If the output buffer is full, save cached state and return */
        {
          len -= 1;
          if len < 0i32 {
            break 'c_10920;
          }
          /* Write next byte into output buffer, updating CRC */
          let fresh10 = outbuf;
          outbuf = outbuf.offset(1);
          *fresh10 = current as libc::c_char;
          CRC = CRC << 8i32 ^ (*bd).crc32Table[(CRC >> 24i32 ^ current as libc::c_uint) as usize];
          /* Loop now if we're outputting multiple copies of this byte */
          if (*bd).writeCopies != 0 {
            current_block = 240530425663761810; /* input block is fully consumed, need next one */
            continue 'c_10920;
          } else {
            current_block = 7525104114255620856;
          }
        }
        _ => {
          (*bd).writeCount -= 1;
          if (*bd).writeCount < 0i32 {
            break;
          }
          /* Follow sequence vector to undo Burrows-Wheeler transform */
          previous = current;
          pos = *dbuf.offset(pos as isize) as libc::c_int;
          current = pos as u8 as libc::c_int;
          pos >>= 8i32;
          /* After 3 consecutive copies of the same byte, the 4th
           * is a repeat count.  We count down from 4 instead
           * of counting up because testing for non-zero is faster */
          (*bd).writeRunCountdown -= 1;
          if (*bd).writeRunCountdown != 0i32 {
            if current != previous {
              (*bd).writeRunCountdown = 4i32
            }
            current_block = 3640593987805443782;
          } else {
            /* Unlikely branch */
            /* We have a repeated run, this byte indicates the count */
            (*bd).writeCopies = current;
            current = previous;
            (*bd).writeRunCountdown = 5i32;
            /* Sometimes there are just 3 bytes (run length 0) */
            if (*bd).writeCopies == 0 {
              current_block = 7525104114255620856;
              continue;
            }
            /* Subtract the 1 copy we'd output anyway to get extras */
            (*bd).writeCopies -= 1;
            current_block = 3640593987805443782;
          }
        }
      }
    }
    /* Decompression of this input block completed successfully */
    CRC = !CRC;
    (*bd).writeCRC = CRC;
    (*bd).totalCRC = ((*bd).totalCRC << 1i32 | (*bd).totalCRC >> 31i32) ^ CRC;
    /* If this block had a CRC error, force file level CRC error */
    if CRC != (*bd).headerCRC {
      (*bd).totalCRC = (*bd).headerCRC.wrapping_add(1i32 as libc::c_uint);
      return -1i32;
    }
    current_block = 7333393191927787629;
  }
  /* Unlikely branch.
   * Use of "goto" instead of keeping code here
   * helps compiler to realize this. */
  /* Output buffer is full, save cached state and return */
  (*bd).writePos = pos;
  (*bd).writeCurrent = current;
  (*bd).writeCRC = CRC;
  (*bd).writeCopies += 1;
  return 0i32;
}
/* Allocate the structure, read file header.  If in_fd==-1, inbuf must contain
a complete bunzip file (len bytes long).  If in_fd!=-1, inbuf and len are
ignored, and data is read from file handle into temporary buffer. */
/* Because bunzip2 is used for help text unpacking, and because bb_show_usage()
should work for NOFORK applets too, we must be extremely careful to not leak
any allocations! */
unsafe extern "C" fn start_bunzip(
  mut jmpbuf: *mut libc::c_void,
  mut bdp: *mut *mut bunzip_data,
  mut in_fd: libc::c_int,
  mut inbuf: *const libc::c_void,
  mut len: libc::c_int,
) -> libc::c_int {
  let mut bd: *mut bunzip_data = 0 as *mut bunzip_data;
  let mut i: libc::c_uint = 0;
  /* Figure out how much data to allocate */
  i = ::std::mem::size_of::<bunzip_data>() as libc::c_ulong as libc::c_uint;
  if in_fd != -1i32 {
    i = i.wrapping_add(4096i32 as libc::c_uint)
  }
  /* Allocate bunzip_data.  Most fields initialize to zero. */
  *bdp = xzalloc(i as size_t) as *mut bunzip_data;
  bd = *bdp;
  (*bd).jmpbuf = jmpbuf as *mut jmp_buf;
  /* Setup input buffer */
  (*bd).in_fd = in_fd;
  if -1i32 == in_fd {
    /* in this case, bd->inbuf is read-only */
    (*bd).inbuf = inbuf as *mut libc::c_void as *mut u8
  /* cast away const-ness */
  } else {
    (*bd).inbuf = bd.offset(1) as *mut u8;
    memcpy(
      (*bd).inbuf as *mut libc::c_void,
      inbuf,
      len as libc::c_ulong,
    );
  }
  (*bd).inbufCount = len;
  /* Init the CRC32 table (big endian) */
  crc32_filltable((*bd).crc32Table.as_mut_ptr(), 1i32);
  /* Ensure that file starts with "BZh['1'-'9']." */
  /* Update: now caller verifies 1st two bytes, makes .gz/.bz2
   * integration easier */
  /* was: */
  /* i = get_bits(bd, 32); */
  /* if ((unsigned)(i - BZh0 - 1) >= 9) return RETVAL_NOT_BZIP_DATA; */
  i = get_bits(bd, 16i32);
  if i
    .wrapping_sub(h0 as libc::c_int as libc::c_uint)
    .wrapping_sub(1i32 as libc::c_uint)
    >= 9i32 as libc::c_uint
  {
    return -2i32;
  }
  /* Fourth byte (ascii '1'-'9') indicates block size in units of 100k of
  uncompressed data.  Allocate intermediate buffer for block. */
  /* bd->dbufSize = 100000 * (i - BZh0); */
  (*bd).dbufSize =
    (100000i32 as libc::c_uint).wrapping_mul(i.wrapping_sub(h0 as libc::c_int as libc::c_uint));
  /* Cannot use xmalloc - may leak bd in NOFORK case! */
  (*bd).dbuf = malloc_or_warn(
    ((*bd).dbufSize as libc::c_ulong).wrapping_mul(::std::mem::size_of::<u32>() as libc::c_ulong),
  ) as *mut u32;
  if (*bd).dbuf.is_null() {
    free(bd as *mut libc::c_void);
    xfunc_die();
  }
  return 0i32;
}
unsafe extern "C" fn dealloc_bunzip(mut bd: *mut bunzip_data) {
  free((*bd).dbuf as *mut libc::c_void);
  free(bd as *mut libc::c_void);
}
/* Decompress src_fd to dst_fd.  Stops at end of bzip data, not end of file. */
#[no_mangle]
pub unsafe extern "C" fn unpack_bz2_stream(
  mut xstate: *mut transformer_state_t,
) -> libc::c_longlong {
  let mut total_written: libc::c_longlong = 0i32 as libc::c_longlong;
  let mut bd: *mut bunzip_data = 0 as *mut bunzip_data;
  let mut outbuf: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut i: libc::c_int = 0;
  let mut len: libc::c_uint = 0;
  if check_signature16(xstate, BZIP2_MAGIC as libc::c_int as libc::c_uint) != 0 {
    return -1i32 as libc::c_longlong;
  }
  outbuf = xmalloc(4096i32 as size_t) as *mut libc::c_char;
  len = 0i32 as libc::c_uint;
  's_36: loop
  /* "Process one BZ... stream" loop */
  {
    let mut jmpbuf: jmp_buf = [__jmp_buf_tag {
      __jmpbuf: [0; 8],
      __mask_was_saved: 0,
      __saved_mask: std::mem::zeroed(),
    }; 1];
    /* Setup for I/O error handling via longjmp */
    i = _setjmp(jmpbuf.as_mut_ptr());
    if i == 0i32 {
      i = start_bunzip(
        &mut jmpbuf as *mut jmp_buf as *mut libc::c_void,
        &mut bd,
        (*xstate).src_fd,
        outbuf.offset(2) as *const libc::c_void,
        len as libc::c_int,
      )
    }
    if i == 0i32 {
      loop {
        /* "Produce some output bytes" loop */
        i = read_bunzip(bd, outbuf, 4096i32);
        if i < 0i32 {
          /* error? */
          break; /* number of bytes produced */
        } else {
          i = 4096i32 - i;
          if i == 0i32 {
            break;
          }
          if i as isize != transformer_write(xstate, outbuf as *const libc::c_void, i as size_t) {
            i = -4;
            break 's_36;
          } else {
            total_written += i as libc::c_longlong
          }
        }
      }
    }
    /* EOF? */
    if i != -1 && i != 0 {
      bb_error_msg(
        b"bunzip error %d\x00" as *const u8 as *const libc::c_char,
        i,
      );
      break;
    } else if (*bd).headerCRC != (*bd).totalCRC {
      bb_simple_error_msg(b"CRC error\x00" as *const u8 as *const libc::c_char);
      break;
    } else {
      /* Successfully unpacked one BZ stream */
      i = 0;
      /* Do we have "BZ..." after last processed byte?
       * pbzip2 (parallelized bzip2) produces such files.
       */
      len = ((*bd).inbufCount - (*bd).inbufPos) as libc::c_uint;
      memcpy(
        outbuf as *mut libc::c_void,
        &mut *(*bd).inbuf.offset((*bd).inbufPos as isize) as *mut u8 as *const libc::c_void,
        len as libc::c_ulong,
      );
      if len < 2 {
        if safe_read(
          (*xstate).src_fd,
          outbuf.offset(len as isize) as *mut libc::c_void,
          (2 as libc::c_uint).wrapping_sub(len) as size_t,
        ) != (2 as libc::c_uint).wrapping_sub(len) as isize
        {
          break;
        }
        len = 2
      }
      if *(outbuf as *mut u16) as libc::c_int != BZIP2_MAGIC as libc::c_int {
        break;
      }
      dealloc_bunzip(bd);
      len = len.wrapping_sub(2)
    }
  }
  /* "BZ"? */
  dealloc_bunzip(bd);
  free(outbuf as *mut libc::c_void);
  return if i != 0 {
    i as libc::c_longlong
  } else {
    (total_written) + 0i32 as libc::c_longlong
  };
}
/* A bit of bunzip2 internals are exposed for compressed help support: */
#[no_mangle]
pub unsafe extern "C" fn unpack_bz2_data(
  mut packed: *const libc::c_char,
  mut packed_len: libc::c_int,
  mut unpacked_len: libc::c_int,
) -> *mut libc::c_char {
  let mut outbuf: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut bd: *mut bunzip_data = 0 as *mut bunzip_data;
  let mut i: libc::c_int = 0;
  let mut jmpbuf: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: std::mem::zeroed(),
  }; 1];
  /* Setup for I/O error handling via longjmp */
  i = _setjmp(jmpbuf.as_mut_ptr());
  if i == 0i32 {
    i = start_bunzip(
      &mut jmpbuf as *mut jmp_buf as *mut libc::c_void,
      &mut bd,
      -1i32,
      packed as *const libc::c_void,
      packed_len,
    )
  }
  /* read_bunzip can longjmp and end up here with i != 0
   * on read data errors! Not trivial */
  if i == 0i32 {
    /* Cannot use xmalloc: will leak bd in NOFORK case! */
    outbuf = malloc_or_warn(unpacked_len as size_t) as *mut libc::c_char;
    if !outbuf.is_null() {
      read_bunzip(bd, outbuf, unpacked_len);
    }
  }
  dealloc_bunzip(bd);
  return outbuf;
}

use crate::archival::libarchive::bb_archive::transformer_state_t;
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
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn full_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn crc32_new_table_le() -> *mut u32;
  #[no_mangle]
  fn crc32_block_endian0(
    val: u32,
    buf: *const libc::c_void,
    len: libc::c_uint,
    crc_table: *mut u32,
  ) -> u32;
  #[no_mangle]
  fn transformer_write(
    xstate: *mut transformer_state_t,
    buf: *const libc::c_void,
    bufsize: size_t,
  ) -> ssize_t;
  #[no_mangle]
  fn check_signature16(xstate: *mut transformer_state_t, magic16: libc::c_uint) -> libc::c_int;
}

pub type uintptr_t = libc::c_ulong;
pub type bb__aliased_u16 = u16;
pub type bb__aliased_u32 = u32;

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

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
  pub b: [u8; 8],
  pub b16: [u16; 4],
  pub b32: [u32; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_t {
  pub gunzip_bytes_out: off_t,
  pub gunzip_crc: u32,
  pub gunzip_src_fd: libc::c_int,
  pub gunzip_outbuf_count: libc::c_uint,
  pub gunzip_window: *mut libc::c_uchar,
  pub gunzip_crc_table: *mut u32,
  pub gunzip_bb: libc::c_uint,
  pub gunzip_bk: libc::c_uchar,
  pub bytebuffer: *mut libc::c_uchar,
  pub to_read: off_t,
  pub bytebuffer_offset: libc::c_uint,
  pub bytebuffer_size: libc::c_uint,
  pub inflate_codes_ml: libc::c_uint,
  pub inflate_codes_md: libc::c_uint,
  pub inflate_codes_bb: libc::c_uint,
  pub inflate_codes_k: libc::c_uint,
  pub inflate_codes_w: libc::c_uint,
  pub inflate_codes_tl: *mut huft_t,
  pub inflate_codes_td: *mut huft_t,
  pub inflate_codes_bl: libc::c_uint,
  pub inflate_codes_bd: libc::c_uint,
  pub inflate_codes_nn: libc::c_uint,
  pub inflate_codes_dd: libc::c_uint,
  pub resume_copy: smallint,
  pub method: smallint,
  pub need_another_block: smallint,
  pub end_reached: smallint,
  pub inflate_stored_n: libc::c_uint,
  pub inflate_stored_b: libc::c_uint,
  pub inflate_stored_k: libc::c_uint,
  pub inflate_stored_w: libc::c_uint,
  pub error_msg: *const libc::c_char,
  pub error_jmp: jmp_buf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct huft_t {
  pub e: libc::c_uchar,
  pub b: libc::c_uchar,
  pub v: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
  pub n: libc::c_uint,
  pub t: *mut huft_t,
}
/* Put lengths/offsets and extra bits in a struct of arrays
 * to make calls to huft_build() have one fewer parameter.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cp_ext {
  pub cp: [u16; 31],
  pub ext: [u8; 31],
}
/* gunzip_window size--must be a power of two, and
 * at least 32K for zip's deflate method */
pub const GUNZIP_WSIZE: C2RustUnnamed_6 = 32768;
/* If BMAX needs to be larger than 16, then h and x[] should be ulg. */
pub const BMAX: C2RustUnnamed_6 = 16;
pub const dbits: C2RustUnnamed_3 = 6;
pub const lbits: C2RustUnnamed_2 = 9;
pub type C2RustUnnamed_2 = libc::c_uint;
pub type C2RustUnnamed_3 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct C2RustUnnamed_4 {
  pub gz_method: u8,
  pub flags: u8,
  pub mtime: u32,
  pub xtra_flags_UNUSED: u8,
  pub os_flags_UNUSED: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
  pub raw: [libc::c_uchar; 8],
  pub formatted: C2RustUnnamed_4,
}
pub type C2RustUnnamed_6 = libc::c_uint;
/* maximum number of codes in any set */
/* maximum bit length of any code (16 for explode) */
pub const N_MAX: C2RustUnnamed_6 = 288;
/* This is a generic part */
/* Use global data segment */
/* Use malloc space */
static mut mask_bits: [u16; 17] = [
  0i32 as u16,
  0x1i32 as u16,
  0x3i32 as u16,
  0x7i32 as u16,
  0xfi32 as u16,
  0x1fi32 as u16,
  0x3fi32 as u16,
  0x7fi32 as u16,
  0xffi32 as u16,
  0x1ffi32 as u16,
  0x3ffi32 as u16,
  0x7ffi32 as u16,
  0xfffi32 as u16,
  0x1fffi32 as u16,
  0x3fffi32 as u16,
  0x7fffi32 as u16,
  0xffffi32 as u16,
];
/* Copy lengths and extra bits for literal codes 257..285 */
/* note: see note #13 above about the 258 in this list. */
static mut lit: cp_ext = {
  let mut init = cp_ext {
    cp: [
      3i32 as u16,
      4i32 as u16,
      5i32 as u16,
      6i32 as u16,
      7i32 as u16,
      8i32 as u16,
      9i32 as u16,
      10i32 as u16,
      11i32 as u16,
      13i32 as u16,
      15i32 as u16,
      17i32 as u16,
      19i32 as u16,
      23i32 as u16,
      27i32 as u16,
      31i32 as u16,
      35i32 as u16,
      43i32 as u16,
      51i32 as u16,
      59i32 as u16,
      67i32 as u16,
      83i32 as u16,
      99i32 as u16,
      115i32 as u16,
      131i32 as u16,
      163i32 as u16,
      195i32 as u16,
      227i32 as u16,
      258i32 as u16,
      0i32 as u16,
      0i32 as u16,
    ],
    ext: [
      0i32 as u8,
      0i32 as u8,
      0i32 as u8,
      0i32 as u8,
      0i32 as u8,
      0i32 as u8,
      0i32 as u8,
      0i32 as u8,
      1i32 as u8,
      1i32 as u8,
      1i32 as u8,
      1i32 as u8,
      2i32 as u8,
      2i32 as u8,
      2i32 as u8,
      2i32 as u8,
      3i32 as u8,
      3i32 as u8,
      3i32 as u8,
      3i32 as u8,
      4i32 as u8,
      4i32 as u8,
      4i32 as u8,
      4i32 as u8,
      5i32 as u8,
      5i32 as u8,
      5i32 as u8,
      5i32 as u8,
      0i32 as u8,
      99i32 as u8,
      99i32 as u8,
    ],
  };
  init
};
/* Copy offsets and extra bits for distance codes 0..29 */
static mut dist: cp_ext = {
  let mut init = cp_ext {
    cp: [
      1i32 as u16,
      2i32 as u16,
      3i32 as u16,
      4i32 as u16,
      5i32 as u16,
      7i32 as u16,
      9i32 as u16,
      13i32 as u16,
      17i32 as u16,
      25i32 as u16,
      33i32 as u16,
      49i32 as u16,
      65i32 as u16,
      97i32 as u16,
      129i32 as u16,
      193i32 as u16,
      257i32 as u16,
      385i32 as u16,
      513i32 as u16,
      769i32 as u16,
      1025i32 as u16,
      1537i32 as u16,
      2049i32 as u16,
      3073i32 as u16,
      4097i32 as u16,
      6145i32 as u16,
      8193i32 as u16,
      12289i32 as u16,
      16385i32 as u16,
      24577i32 as u16,
      0,
    ],
    ext: [
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
      0,
    ],
  };
  init
};
/* Tables for deflate from PKZIP's appnote.txt. */
/* Order of the bit length code lengths */
static mut border: [u8; 19] = [
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
/*
 * Free the malloc'ed tables built by huft_build(), which makes a linked
 * list of the tables it made, with the links in a dummy first entry of
 * each table.
 * t: table to free
 */
unsafe extern "C" fn huft_free(mut p: *mut huft_t) {
  let mut q: *mut huft_t = 0 as *mut huft_t;
  /* Go through linked list, freeing from the malloced (t[-1]) address. */
  while !p.is_null() {
    p = p.offset(-1);
    q = (*p).v.t;
    free(p as *mut libc::c_void);
    p = q
  }
}
unsafe extern "C" fn huft_free_all(mut state: *mut state_t) {
  huft_free((*state).inflate_codes_tl);
  huft_free((*state).inflate_codes_td);
  (*state).inflate_codes_tl = 0 as *mut huft_t;
  (*state).inflate_codes_td = 0 as *mut huft_t;
}
unsafe extern "C" fn abort_unzip(mut state: *mut state_t) -> ! {
  huft_free_all(state);
  longjmp((*state).error_jmp.as_mut_ptr(), 1i32);
}
unsafe extern "C" fn fill_bitbuffer(
  mut state: *mut state_t,
  mut bitbuffer: libc::c_uint,
  mut current: *mut libc::c_uint,
  required: libc::c_uint,
) -> libc::c_uint {
  while *current < required {
    if (*state).bytebuffer_offset >= (*state).bytebuffer_size {
      let mut sz: libc::c_uint = (0x4000i32 - 4i32) as libc::c_uint;
      if (*state).to_read >= 0 && (*state).to_read < sz as libc::c_long {
        /* unzip only */
        sz = (*state).to_read as libc::c_uint
      }
      /* Leave the first 4 bytes empty so we can always unwind the bitbuffer
       * to the front of the bytebuffer */
      (*state).bytebuffer_size = safe_read(
        (*state).gunzip_src_fd,
        &mut *(*state).bytebuffer.offset(4) as *mut libc::c_uchar as *mut libc::c_void,
        sz as size_t,
      ) as libc::c_uint;
      if ((*state).bytebuffer_size as libc::c_int) < 1i32 {
        (*state).error_msg = b"unexpected end of file\x00" as *const u8 as *const libc::c_char;
        abort_unzip(state);
      }
      if (*state).to_read >= 0 {
        /* unzip only */
        (*state).to_read -= (*state).bytebuffer_size as libc::c_long
      } /* counter for codes of length k */
      (*state).bytebuffer_size = (*state).bytebuffer_size.wrapping_add(4i32 as libc::c_uint); /* bit length count table */
      (*state).bytebuffer_offset = 4i32 as libc::c_uint
    } /* length of end-of-block code (value 256) */
    bitbuffer |= (*(*state)
      .bytebuffer
      .offset((*state).bytebuffer_offset as isize) as libc::c_uint)
      << *current; /* i repeats in table every f entries */
    (*state).bytebuffer_offset = (*state).bytebuffer_offset.wrapping_add(1); /* maximum code length */
    *current = (*current).wrapping_add(8i32 as libc::c_uint)
  } /* table level */
  return bitbuffer; /* counter, current code */
}
unsafe extern "C" fn huft_build(
  mut b: *const libc::c_uint,
  n: libc::c_uint,
  s: libc::c_uint,
  mut cp_ext: *const cp_ext,
  mut m: *mut libc::c_uint,
) -> *mut huft_t {
  let mut a: libc::c_uint = 0; /* counter */
  let mut c: [libc::c_uint; 17] = [0; 17]; /* number of bits in current code */
  let mut eob_len: libc::c_uint = 0; /* pointer into c[], b[], or v[] */
  let mut f: libc::c_uint = 0; /* points to current table */
  let mut g: libc::c_int = 0; /* table entry for structure assignment */
  let mut htl: libc::c_int = 0; /* table stack */
  let mut i: libc::c_uint = 0; /* values in order of bit length. last v[] is never used */
  let mut j: libc::c_uint = 0; /* bits decoded stack */
  let mut k: libc::c_int = 0; /* bits decoded */
  let mut p: *const libc::c_uint = 0 as *const libc::c_uint; /* bit offsets, then code stack */
  let mut q: *mut huft_t = 0 as *mut huft_t; /* pointer into x */
  let mut r: huft_t = huft_t {
    e: 0,
    b: 0,
    v: C2RustUnnamed_1 { n: 0 },
  }; /* number of dummy codes added */
  let mut u: [*mut huft_t; 16] = [0 as *mut huft_t; 16]; /* number of entries in current table */
  let mut v: [libc::c_uint; 289] = [0; 289];
  let mut ws: [libc::c_int; 17] = [0; 17];
  let mut w: libc::c_int = 0;
  let mut x: [libc::c_uint; 17] = [0; 17];
  let mut xp: *mut libc::c_uint = 0 as *mut libc::c_uint;
  let mut y: libc::c_int = 0;
  let mut z: libc::c_uint = 0;
  let mut result: *mut huft_t = 0 as *mut huft_t;
  let mut t: *mut *mut huft_t = 0 as *mut *mut huft_t;
  /* Length of EOB code, if any */
  eob_len = if n > 256i32 as libc::c_uint {
    *b.offset(256)
  } else {
    BMAX as libc::c_int as libc::c_uint
  };
  /* Generate counts for each bit length */
  memset(
    c.as_mut_ptr() as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<[libc::c_uint; 17]>() as libc::c_ulong,
  ); /* assume all entries <= BMAX */
  p = b;
  i = n;
  loop {
    c[*p as usize] = c[*p as usize].wrapping_add(1);
    p = p.offset(1);
    i = i.wrapping_sub(1);
    if !(i != 0) {
      break;
    }
    /* can't combine with above line (Solaris bug) */
  }
  if c[0] == n {
    /* null input - all zero length codes */
    q = xzalloc(
      (3i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<huft_t>() as libc::c_ulong),
    ) as *mut huft_t;
    //q[0].v.t = NULL;
    (*q.offset(1)).e = 99i32 as libc::c_uchar; /* invalid code marker */
    (*q.offset(1)).b = 1i32 as libc::c_uchar; /* invalid code marker */
    (*q.offset(2)).e = 99i32 as libc::c_uchar;
    (*q.offset(2)).b = 1i32 as libc::c_uchar;
    *m = 1i32 as libc::c_uint;
    return q.offset(1);
  }
  /* Find minimum and maximum length, bound *m by those */
  j = 1i32 as libc::c_uint; /* minimum code length */
  while j <= BMAX as libc::c_int as libc::c_uint && c[j as usize] == 0i32 as libc::c_uint {
    j = j.wrapping_add(1)
  } /* maximum code length */
  k = j as libc::c_int;
  i = BMAX as libc::c_int as libc::c_uint;
  while c[i as usize] == 0i32 as libc::c_uint && i != 0 {
    i = i.wrapping_sub(1)
  }
  g = i as libc::c_int;
  *m = if *m < j {
    j
  } else if *m > i {
    i
  } else {
    *m
  };
  /* Adjust last length count to fill out codes, if needed */
  y = 1i32 << j;
  while j < i {
    y = (y as libc::c_uint).wrapping_sub(c[j as usize]) as libc::c_int as libc::c_int;
    if y < 0i32 {
      return 1i32 as uintptr_t as *mut huft_t;
    }
    j = j.wrapping_add(1);
    y <<= 1i32
    /* bad input: more codes than bits */
  }
  y = (y as libc::c_uint).wrapping_sub(c[i as usize]) as libc::c_int as libc::c_int;
  if y < 0i32 {
    return 1i32 as uintptr_t as *mut huft_t;
  }
  c[i as usize] = c[i as usize].wrapping_add(y as libc::c_uint);
  /* Generate starting offsets into the value table for each length */
  j = 0i32 as libc::c_uint;
  x[1] = j;
  p = c.as_mut_ptr().offset(1);
  xp = x.as_mut_ptr().offset(2);
  loop {
    i = i.wrapping_sub(1);
    if !(i != 0) {
      break;
    }
    /* note that i == g from above */
    let fresh0 = p;
    p = p.offset(1);
    j = j.wrapping_add(*fresh0);
    let fresh1 = xp;
    xp = xp.offset(1);
    *fresh1 = j
  }
  /* Make a table of values in order of bit lengths.
   * To detect bad input, unused v[i]'s are set to invalid value UINT_MAX.
   * In particular, last v[i] is never filled and must not be accessed.
   */
  memset(
    v.as_mut_ptr() as *mut libc::c_void,
    0xffi32,
    ::std::mem::size_of::<[libc::c_uint; 289]>() as libc::c_ulong,
  );
  p = b;
  i = 0i32 as libc::c_uint;
  loop {
    let fresh2 = p;
    p = p.offset(1);
    j = *fresh2;
    if j != 0i32 as libc::c_uint {
      let fresh3 = x[j as usize];
      x[j as usize] = x[j as usize].wrapping_add(1);
      v[fresh3 as usize] = i
    }
    i = i.wrapping_add(1);
    if !(i < n) {
      break;
    }
  }
  /* Generate the Huffman codes and for each, make the table entries */
  result = 1i32 as uintptr_t as *mut huft_t; /* first Huffman code is zero */
  t = &mut result; /* grab values in bit order */
  i = 0i32 as libc::c_uint; /* no tables yet--level -1 */
  x[0] = i; /* bits decoded */
  p = v.as_mut_ptr(); /* just to keep compilers happy */
  htl = -1i32; /* ditto */
  ws[0] = 0i32; /* ditto */
  w = ws[0];
  u[0] = 0 as *mut huft_t;
  q = 0 as *mut huft_t;
  z = 0i32 as libc::c_uint;
  /* go through the bit lengths (k already is bits in shortest code) */
  while k <= g {
    a = c[k as usize];
    loop {
      let fresh4 = a;
      a = a.wrapping_sub(1);
      if !(fresh4 != 0) {
        break;
      }
      /* here i is the Huffman code of length k bits for value *p */
      /* make tables up to required level */
      while k > ws[(htl + 1i32) as usize] {
        htl += 1;
        w = ws[htl as usize];
        /* compute minimum size table less than or equal to *m bits */
        z = (g - w) as libc::c_uint; /* upper limit on table size */
        z = if z > *m { *m } else { z };
        j = (k - w) as libc::c_uint;
        f = (1i32 << j) as libc::c_uint;
        if f > a.wrapping_add(1i32 as libc::c_uint) {
          /* try a k-w bit table */
          /* too few codes for k-w bit table */
          f = f.wrapping_sub(a.wrapping_add(1i32 as libc::c_uint)); /* deduct codes from patterns left */
          xp = c.as_mut_ptr().offset(k as isize);
          loop {
            j = j.wrapping_add(1);
            if !(j < z) {
              break;
            }
            /* try smaller tables up to z bits */
            f <<= 1i32;
            xp = xp.offset(1);
            if f <= *xp {
              break;
            }
            f = f.wrapping_sub(*xp)
            /* else deduct codes from patterns */
          }
        } /* make EOB code end at table */
        j = if (w as libc::c_uint).wrapping_add(j) > eob_len && (w as libc::c_uint) < eob_len {
          eob_len.wrapping_sub(w as libc::c_uint)
        } else {
          j
        }; /* table entries for j-bit table */
        z = (1i32 << j) as libc::c_uint; /* set bits decoded in stack */
        ws[(htl + 1i32) as usize] = (w as libc::c_uint).wrapping_add(j) as libc::c_int;
        /* allocate and link in new table */
        q = xzalloc(
          (z.wrapping_add(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<huft_t>() as libc::c_ulong),
        ) as *mut huft_t; /* link to list for huft_free() */
        *t = q.offset(1); /* table starts after link */
        t = &mut (*q).v.t;
        q = q.offset(1);
        u[htl as usize] = q;
        /* connect to last table, if there is one */
        if htl != 0 {
          x[htl as usize] = i; /* save pattern for backing up */
          /* connect to last table */
          r.b = (w - ws[(htl - 1i32) as usize]) as libc::c_uchar; /* bits to dump before this table */
          r.e = (16i32 as libc::c_uint).wrapping_add(j) as libc::c_uchar; /* bits in this table */
          r.v.t = q; /* pointer to this table */
          j = (i & ((1i32 << w) - 1i32) as libc::c_uint) >> ws[(htl - 1i32) as usize];
          *u[(htl - 1i32) as usize].offset(j as isize) = r
        }
      }
      /* set up table entry in r */
      r.b = (k - w) as libc::c_uchar;
      if *p
        == (2147483647i32 as libc::c_uint)
          .wrapping_mul(2u32)
          .wrapping_add(1u32)
      {
        /* do we access uninited v[i]? (see memset(v))*/
        r.e = 99i32 as libc::c_uchar
      /* out of values--invalid code */
      } else if *p < s {
        r.e = if *p < 256i32 as libc::c_uint {
          16i32
        } else {
          15i32
        } as libc::c_uchar;
        let fresh5 = p;
        p = p.offset(1);
        r.v.n = *fresh5 as libc::c_ushort as libc::c_uint /* 256 is EOB code */
      /* simple code is just the value */
      } else {
        r.e = (*cp_ext).ext[(*p).wrapping_sub(s) as usize]; /* non-simple--look up in lists */
        let fresh6 = p;
        p = p.offset(1);
        r.v.n = (*cp_ext).cp[(*fresh6).wrapping_sub(s) as usize] as libc::c_uint
      }
      /* fill code-like entries with r */
      f = (1i32 << k - w) as libc::c_uint;
      j = i >> w;
      while j < z {
        *q.offset(j as isize) = r;
        j = j.wrapping_add(f)
      }
      /* backwards increment the k-bit code i */
      j = (1i32 << k - 1i32) as libc::c_uint;
      while i & j != 0 {
        i ^= j;
        j >>= 1i32
      }
      i ^= j;
      /* backup over finished tables */
      while i & ((1i32 << w) - 1i32) as libc::c_uint != x[htl as usize] {
        htl -= 1;
        w = ws[htl as usize]
      }
    }
    k += 1
  }
  /* return actual size of base table */
  *m = ws[1] as libc::c_uint;
  if y != 0i32 && g != 1i32 {
    /* we were given an incomplete table */
    /* return "result" ORed with 1 */
    return (result as uintptr_t | 1i32 as libc::c_ulong) as *mut libc::c_void as *mut huft_t;
  }
  return result;
}
/*
 * inflate (decompress) the codes in a deflated (compressed) block.
 * Return an error code or zero if it all goes ok.
 *
 * tl, td: literal/length and distance decoder tables
 * bl, bd: number of bits decoded by tl[] and td[]
 */
/* called once from inflate_block */
/* map formerly local static variables to globals */
unsafe extern "C" fn inflate_codes_setup(
  mut state: *mut state_t,
  mut my_bl: libc::c_uint,
  mut my_bd: libc::c_uint,
) {
  (*state).inflate_codes_bl = my_bl;
  (*state).inflate_codes_bd = my_bd;
  /* make local copies of globals */
  (*state).inflate_codes_bb = (*state).gunzip_bb; /* initialize bit buffer */
  (*state).inflate_codes_k = (*state).gunzip_bk as libc::c_uint; /* initialize gunzip_window position */
  (*state).inflate_codes_w = (*state).gunzip_outbuf_count;
  /* inflate the coded data */
  (*state).inflate_codes_ml = mask_bits[(*state).inflate_codes_bl as usize] as libc::c_uint; /* precompute masks for speed */
  (*state).inflate_codes_md = mask_bits[(*state).inflate_codes_bd as usize] as libc::c_uint;
}
/* called once from inflate_get_next_window */
#[inline(never)]
unsafe extern "C" fn inflate_codes(mut state: *mut state_t) -> libc::c_int {
  let mut current_block: u64; /* table entry flag/number of extra bits */
  let mut e: libc::c_uint = 0; /* pointer to table entry */
  let mut t: *mut huft_t = 0 as *mut huft_t;
  if (*state).resume_copy != 0 {
    current_block = 2877038400800049082;
  } else {
    current_block = 12675440807659640239;
  }
  loop {
    match current_block {
      12675440807659640239 => {
        /* do until end of block */
        (*state).inflate_codes_bb = fill_bitbuffer(
          state,
          (*state).inflate_codes_bb,
          &mut (*state).inflate_codes_k,
          (*state).inflate_codes_bl,
        ); /* it's an EOB or a length */
        t = (*state)
          .inflate_codes_tl
          .offset(((*state).inflate_codes_bb & (*state).inflate_codes_ml) as isize);
        e = (*t).e as libc::c_uint;
        if e > 16i32 as libc::c_uint {
          loop {
            if e == 99i32 as libc::c_uint {
              abort_unzip(state);
            }
            (*state).inflate_codes_bb >>= (*t).b as libc::c_int;
            (*state).inflate_codes_k = (*state)
              .inflate_codes_k
              .wrapping_sub((*t).b as libc::c_uint);
            e = e.wrapping_sub(16i32 as libc::c_uint);
            (*state).inflate_codes_bb = fill_bitbuffer(
              state,
              (*state).inflate_codes_bb,
              &mut (*state).inflate_codes_k,
              e,
            );
            t = (*t)
              .v
              .t
              .offset(((*state).inflate_codes_bb & mask_bits[e as usize] as libc::c_uint) as isize);
            e = (*t).e as libc::c_uint;
            if !(e > 16i32 as libc::c_uint) {
              break;
            }
          }
        }
        (*state).inflate_codes_bb >>= (*t).b as libc::c_int;
        (*state).inflate_codes_k = (*state)
          .inflate_codes_k
          .wrapping_sub((*t).b as libc::c_uint);
        if e == 16i32 as libc::c_uint {
          /* then it's a literal */
          let fresh7 = (*state).inflate_codes_w;
          (*state).inflate_codes_w = (*state).inflate_codes_w.wrapping_add(1);
          *(*state).gunzip_window.offset(fresh7 as isize) = (*t).v.n as libc::c_uchar;
          if (*state).inflate_codes_w == GUNZIP_WSIZE as libc::c_int as libc::c_uint {
            (*state).gunzip_outbuf_count = (*state).inflate_codes_w;
            // We have a block to read
            (*state).inflate_codes_w = 0i32 as libc::c_uint;
            return 1i32;
          }
          current_block = 12675440807659640239;
        } else {
          //flush_gunzip_window();
          /* exit if end of block */
          if e == 15i32 as libc::c_uint {
            break;
          }
          /* get length of block to copy */
          (*state).inflate_codes_bb = fill_bitbuffer(
            state,
            (*state).inflate_codes_bb,
            &mut (*state).inflate_codes_k,
            e,
          );
          (*state).inflate_codes_nn = (*t)
            .v
            .n
            .wrapping_add((*state).inflate_codes_bb & mask_bits[e as usize] as libc::c_uint);
          (*state).inflate_codes_bb >>= e;
          (*state).inflate_codes_k = (*state).inflate_codes_k.wrapping_sub(e);
          /* decode distance of block to copy */
          (*state).inflate_codes_bb = fill_bitbuffer(
            state,
            (*state).inflate_codes_bb,
            &mut (*state).inflate_codes_k,
            (*state).inflate_codes_bd,
          );
          t = (*state)
            .inflate_codes_td
            .offset(((*state).inflate_codes_bb & (*state).inflate_codes_md) as isize);
          e = (*t).e as libc::c_uint;
          if e > 16i32 as libc::c_uint {
            loop {
              if e == 99i32 as libc::c_uint {
                abort_unzip(state);
              }
              (*state).inflate_codes_bb >>= (*t).b as libc::c_int;
              (*state).inflate_codes_k = (*state)
                .inflate_codes_k
                .wrapping_sub((*t).b as libc::c_uint);
              e = e.wrapping_sub(16i32 as libc::c_uint);
              (*state).inflate_codes_bb = fill_bitbuffer(
                state,
                (*state).inflate_codes_bb,
                &mut (*state).inflate_codes_k,
                e,
              );
              t = (*t).v.t.offset(
                ((*state).inflate_codes_bb & mask_bits[e as usize] as libc::c_uint) as isize,
              );
              e = (*t).e as libc::c_uint;
              if !(e > 16i32 as libc::c_uint) {
                break;
              }
            }
          }
          (*state).inflate_codes_bb >>= (*t).b as libc::c_int;
          (*state).inflate_codes_k = (*state)
            .inflate_codes_k
            .wrapping_sub((*t).b as libc::c_uint);
          (*state).inflate_codes_bb = fill_bitbuffer(
            state,
            (*state).inflate_codes_bb,
            &mut (*state).inflate_codes_k,
            e,
          );
          (*state).inflate_codes_dd = (*state)
            .inflate_codes_w
            .wrapping_sub((*t).v.n)
            .wrapping_sub((*state).inflate_codes_bb & mask_bits[e as usize] as libc::c_uint);
          (*state).inflate_codes_bb >>= e;
          (*state).inflate_codes_k = (*state).inflate_codes_k.wrapping_sub(e);
          current_block = 2877038400800049082;
        }
      }
      _ =>
      /* do the copy */
      {
        loop {
          /* Was: nn -= (e = (e = GUNZIP_WSIZE - ((dd &= GUNZIP_WSIZE - 1) > w ? dd : w)) > nn ? nn : e); */
          /* Who wrote THAT?? rewritten as: */
          let mut delta: libc::c_uint = 0;
          (*state).inflate_codes_dd &= (GUNZIP_WSIZE as libc::c_int - 1i32) as libc::c_uint;
          e = (GUNZIP_WSIZE as libc::c_int as libc::c_uint).wrapping_sub(
            if (*state).inflate_codes_dd > (*state).inflate_codes_w {
              (*state).inflate_codes_dd
            } else {
              (*state).inflate_codes_w
            },
          );
          delta = if (*state).inflate_codes_w > (*state).inflate_codes_dd {
            (*state)
              .inflate_codes_w
              .wrapping_sub((*state).inflate_codes_dd)
          } else {
            (*state)
              .inflate_codes_dd
              .wrapping_sub((*state).inflate_codes_w)
          };
          if e > (*state).inflate_codes_nn {
            e = (*state).inflate_codes_nn
          }
          (*state).inflate_codes_nn = (*state).inflate_codes_nn.wrapping_sub(e);
          /* copy to new buffer to prevent possible overwrite */
          if delta >= e {
            memcpy(
              (*state)
                .gunzip_window
                .offset((*state).inflate_codes_w as isize) as *mut libc::c_void,
              (*state)
                .gunzip_window
                .offset((*state).inflate_codes_dd as isize) as *const libc::c_void,
              e as libc::c_ulong,
            );
            (*state).inflate_codes_w = (*state).inflate_codes_w.wrapping_add(e);
            (*state).inflate_codes_dd = (*state).inflate_codes_dd.wrapping_add(e)
          } else {
            loop
            /* do it slow to avoid memcpy() overlap */
            /* !NOMEMCPY */
            {
              let fresh8 = (*state).inflate_codes_dd;
              (*state).inflate_codes_dd = (*state).inflate_codes_dd.wrapping_add(1);
              let fresh9 = (*state).inflate_codes_w;
              (*state).inflate_codes_w = (*state).inflate_codes_w.wrapping_add(1);
              *(*state).gunzip_window.offset(fresh9 as isize) =
                *(*state).gunzip_window.offset(fresh8 as isize);
              e = e.wrapping_sub(1);
              if !(e != 0) {
                break;
              }
            }
          }
          if (*state).inflate_codes_w == GUNZIP_WSIZE as libc::c_int as libc::c_uint {
            (*state).gunzip_outbuf_count = (*state).inflate_codes_w;
            (*state).resume_copy =
              ((*state).inflate_codes_nn != 0i32 as libc::c_uint) as libc::c_int as smallint;
            //flush_gunzip_window();
            (*state).inflate_codes_w = 0i32 as libc::c_uint;
            return 1i32;
          }
          if !((*state).inflate_codes_nn != 0) {
            break;
          }
        }
        (*state).resume_copy = 0i32 as smallint;
        current_block = 12675440807659640239;
      }
    }
  }
  /* restore the globals from the locals */
  (*state).gunzip_outbuf_count = (*state).inflate_codes_w; /* restore global gunzip_window pointer */
  (*state).gunzip_bb = (*state).inflate_codes_bb; /* restore global bit buffer */
  (*state).gunzip_bk = (*state).inflate_codes_k as libc::c_uchar;
  /* normally just after call to inflate_codes, but save code by putting it here */
  /* free the decoding tables (tl and td), return */
  huft_free_all(state);
  /* done */
  return 0i32;
}
/* called once from inflate_block */
unsafe extern "C" fn inflate_stored_setup(
  mut state: *mut state_t,
  mut my_n: libc::c_int,
  mut my_b: libc::c_int,
  mut my_k: libc::c_int,
) {
  (*state).inflate_stored_n = my_n as libc::c_uint;
  (*state).inflate_stored_b = my_b as libc::c_uint;
  (*state).inflate_stored_k = my_k as libc::c_uint;
  /* initialize gunzip_window position */
  (*state).inflate_stored_w = (*state).gunzip_outbuf_count;
}
/* called once from inflate_get_next_window */
unsafe extern "C" fn inflate_stored(mut state: *mut state_t) -> libc::c_int {
  loop
  /* read and output the compressed data */
  {
    let fresh10 = (*state).inflate_stored_n;
    (*state).inflate_stored_n = (*state).inflate_stored_n.wrapping_sub(1);
    if !(fresh10 != 0) {
      break;
    }
    (*state).inflate_stored_b = fill_bitbuffer(
      state,
      (*state).inflate_stored_b,
      &mut (*state).inflate_stored_k,
      8i32 as libc::c_uint,
    );
    let fresh11 = (*state).inflate_stored_w;
    (*state).inflate_stored_w = (*state).inflate_stored_w.wrapping_add(1);
    *(*state).gunzip_window.offset(fresh11 as isize) = (*state).inflate_stored_b as libc::c_uchar;
    if (*state).inflate_stored_w == GUNZIP_WSIZE as libc::c_int as libc::c_uint {
      (*state).gunzip_outbuf_count = (*state).inflate_stored_w;
      /* We have a block */
      (*state).inflate_stored_w = 0i32 as libc::c_uint;
      (*state).inflate_stored_b >>= 8i32;
      (*state).inflate_stored_k = (*state).inflate_stored_k.wrapping_sub(8i32 as libc::c_uint);
      return 1i32;
    }
    (*state).inflate_stored_b >>= 8i32;
    (*state).inflate_stored_k = (*state).inflate_stored_k.wrapping_sub(8i32 as libc::c_uint)
  }
  //flush_gunzip_window();
  /* restore the globals from the locals */
  (*state).gunzip_outbuf_count = (*state).inflate_stored_w; /* restore global gunzip_window pointer */
  (*state).gunzip_bb = (*state).inflate_stored_b; /* restore global bit buffer */
  (*state).gunzip_bk = (*state).inflate_stored_k as libc::c_uchar;
  return 0i32;
  /* Finished */
}
/*
 * decompress an inflated block
 * e: last block flag
 *
 * GLOBAL VARIABLES: bb, kk,
 */
/* Return values: -1 = inflate_stored, -2 = inflate_codes */
/* One callsite in inflate_get_next_window */
unsafe extern "C" fn inflate_block(mut state: *mut state_t, mut e: *mut smallint) -> libc::c_int {
  let mut ll: [libc::c_uint; 316] = [0; 316]; /* literal/length and distance code lengths */
  let mut t: libc::c_uint = 0; /* block type */
  let mut b: libc::c_uint = 0; /* bit buffer */
  let mut k: libc::c_uint = 0; /* number of bits in bit buffer */
  /* make local bit buffer */
  b = (*state).gunzip_bb;
  k = (*state).gunzip_bk as libc::c_uint;
  /* read in last block bit */
  b = fill_bitbuffer(state, b, &mut k, 1i32 as libc::c_uint);
  *e = (b & 1i32 as libc::c_uint) as smallint;
  b >>= 1i32;
  k = k.wrapping_sub(1i32 as libc::c_uint);
  /* read in block type */
  b = fill_bitbuffer(state, b, &mut k, 2i32 as libc::c_uint);
  t = b & 3i32 as libc::c_uint;
  b >>= 2i32;
  k = k.wrapping_sub(2i32 as libc::c_uint);
  /* restore the global bit buffer */
  (*state).gunzip_bb = b;
  (*state).gunzip_bk = k as libc::c_uchar;
  /* Do we see block type 1 often? Yes!
   * TODO: fix performance problem (see below) */
  //bb_error_msg("blktype %d", t);
  /* inflate that block type */
  match t {
    0 => {
      /* Inflate stored */
      let mut n: libc::c_uint = 0; /* number of bytes in block */
      let mut b_stored: libc::c_uint = 0; /* bit buffer */
      let mut k_stored: libc::c_uint = 0; /* number of bits in bit buffer */
      b_stored = (*state).gunzip_bb;
      k_stored = (*state).gunzip_bk as libc::c_uint;
      n = k_stored & 7i32 as libc::c_uint;
      b_stored >>= n;
      k_stored = k_stored.wrapping_sub(n);
      b_stored = fill_bitbuffer(state, b_stored, &mut k_stored, 16i32 as libc::c_uint);
      n = b_stored & 0xffffi32 as libc::c_uint;
      b_stored >>= 16i32;
      k_stored = k_stored.wrapping_sub(16i32 as libc::c_uint);
      b_stored = fill_bitbuffer(state, b_stored, &mut k_stored, 16i32 as libc::c_uint);
      if n != !b_stored & 0xffffi32 as libc::c_uint {
        abort_unzip(state);
        /* make local copies of globals */
        /* initialize bit buffer */
        /* go to byte boundary */
        /* get the length and its complement */
        /* error in compressed data */
      }
      b_stored >>= 16i32;
      k_stored = k_stored.wrapping_sub(16i32 as libc::c_uint);
      inflate_stored_setup(
        state,
        n as libc::c_int,
        b_stored as libc::c_int,
        k_stored as libc::c_int,
      );
      return -1i32;
    }
    1 => {
      /* Inflate fixed
       * decompress an inflated type 1 (fixed Huffman codes) block. We should
       * either replace this with a custom decoder, or at least precompute the
       * Huffman tables. TODO */
      let mut i: libc::c_int = 0; /* temporary variable */
      let mut bl: libc::c_uint = 0; /* lookup bits for tl */
      let mut bd: libc::c_uint = 0; /* lookup bits for td */
      i = 0i32;
      while i < 144i32 {
        ll[i as usize] = 8i32 as libc::c_uint;
        i += 1
      }
      while i < 256i32 {
        ll[i as usize] = 9i32 as libc::c_uint;
        i += 1
      }
      while i < 280i32 {
        ll[i as usize] = 7i32 as libc::c_uint;
        i += 1
      }
      while i < 288i32 {
        /* gcc 4.2.1 is too dumb to reuse stackspace. Moved up... */
        //unsigned ll[288];     /* length list for huft_build */
        /* set up literal table */
        /* make a complete, but wrong code set */
        ll[i as usize] = 8i32 as libc::c_uint;
        i += 1
      }
      bl = 7i32 as libc::c_uint;
      (*state).inflate_codes_tl = huft_build(
        ll.as_mut_ptr(),
        288i32 as libc::c_uint,
        257i32 as libc::c_uint,
        &lit,
        &mut bl,
      );
      i = 0i32;
      while i < 30i32 {
        /* ^^^ never returns error here - we use known data */
        /* set up distance table */
        /* make an incomplete code set */
        ll[i as usize] = 5i32 as libc::c_uint;
        i += 1
      }
      bd = 5i32 as libc::c_uint;
      (*state).inflate_codes_td = huft_build(
        ll.as_mut_ptr(),
        30i32 as libc::c_uint,
        0i32 as libc::c_uint,
        &dist,
        &mut bd,
      );
      (*state).inflate_codes_td = ((*state).inflate_codes_td as uintptr_t & !(1i32 as uintptr_t))
        as *mut libc::c_void as *mut huft_t;
      inflate_codes_setup(state, bl, bd);
      return -2i32;
    }
    2 => {
      /* ^^^ does return error here! (lsb bit is set) - we gave it incomplete code set */
      /* clearing error bit: */
      /* set up data for inflate_codes() */
      /* huft_free code moved into inflate_codes */
      /* Inflate dynamic */
      let mut td: *mut huft_t = 0 as *mut huft_t; /* bits in base distance lookup table */
      let mut i_0: libc::c_uint = 0;
      let mut j: libc::c_uint = 0; /* bits in base literal/length lookup table */
      let mut l: libc::c_uint = 0;
      let mut m: libc::c_uint = 0;
      let mut n_0: libc::c_uint = 0;
      let mut bl_0: libc::c_uint = 0;
      let mut bd_0: libc::c_uint = 0;
      let mut nb: libc::c_uint = 0;
      let mut nl: libc::c_uint = 0;
      let mut nd: libc::c_uint = 0;
      let mut b_dynamic: libc::c_uint = 0;
      let mut k_dynamic: libc::c_uint = 0;
      b_dynamic = (*state).gunzip_bb;
      k_dynamic = (*state).gunzip_bk as libc::c_uint; /* distance code table */
      b_dynamic = fill_bitbuffer(state, b_dynamic, &mut k_dynamic, 5i32 as libc::c_uint);
      nl = (257i32 as libc::c_uint).wrapping_add(b_dynamic & 0x1fi32 as libc::c_uint);
      b_dynamic >>= 5i32; /* temporary variables */
      k_dynamic = k_dynamic.wrapping_sub(5i32 as libc::c_uint); /* last length */
      b_dynamic = fill_bitbuffer(state, b_dynamic, &mut k_dynamic, 5i32 as libc::c_uint); /* mask for bit lengths table */
      nd = (1i32 as libc::c_uint).wrapping_add(b_dynamic & 0x1fi32 as libc::c_uint);
      b_dynamic >>= 5i32; /* number of lengths to get */
      k_dynamic = k_dynamic.wrapping_sub(5i32 as libc::c_uint); /* lookup bits for tl */
      b_dynamic = fill_bitbuffer(state, b_dynamic, &mut k_dynamic, 4i32 as libc::c_uint); /* lookup bits for td */
      nb = (4i32 as libc::c_uint).wrapping_add(b_dynamic & 0xfi32 as libc::c_uint);
      b_dynamic >>= 4i32; /* number of bit length codes */
      k_dynamic = k_dynamic.wrapping_sub(4i32 as libc::c_uint); /* number of literal/length codes */
      if nl > 286i32 as libc::c_uint || nd > 30i32 as libc::c_uint {
        abort_unzip(state); /* number of distance codes */
        //unsigned ll[286 + 30];/* literal/length and distance code lengths */
        /* bit buffer */
        /* number of bits in bit buffer */
        /* make local bit buffer */
        /* read in table lengths */
        /* number of literal/length codes */
        /* number of distance codes */
        /* number of bit length codes */
        /* bad lengths */
      }
      j = 0i32 as libc::c_uint;
      while j < nb {
        b_dynamic = fill_bitbuffer(state, b_dynamic, &mut k_dynamic, 3i32 as libc::c_uint);
        ll[border[j as usize] as usize] = b_dynamic & 7i32 as libc::c_uint;
        b_dynamic >>= 3i32;
        k_dynamic = k_dynamic.wrapping_sub(3i32 as libc::c_uint);
        j = j.wrapping_add(1)
      }
      while j < 19i32 as libc::c_uint {
        ll[border[j as usize] as usize] = 0i32 as libc::c_uint;
        j = j.wrapping_add(1)
      }
      bl_0 = 7i32 as libc::c_uint;
      (*state).inflate_codes_tl = huft_build(
        ll.as_mut_ptr(),
        19i32 as libc::c_uint,
        19i32 as libc::c_uint,
        0 as *const cp_ext,
        &mut bl_0,
      );
      if (*state).inflate_codes_tl as uintptr_t & 1i32 as libc::c_ulong != 0 {
        abort_unzip(state);
        /* read in bit-length-code lengths */
        /* build decoding table for trees - single level, 7 bit lookup */
        /* incomplete code set */
      }
      n_0 = nl.wrapping_add(nd);
      m = mask_bits[bl_0 as usize] as libc::c_uint;
      l = 0i32 as libc::c_uint;
      i_0 = l;
      while i_0 < n_0 {
        b_dynamic = fill_bitbuffer(state, b_dynamic, &mut k_dynamic, bl_0);
        td = (*state).inflate_codes_tl.offset((b_dynamic & m) as isize);
        j = (*td).b as libc::c_uint;
        b_dynamic >>= j;
        k_dynamic = k_dynamic.wrapping_sub(j);
        j = (*td).v.n;
        if j < 16i32 as libc::c_uint {
          /* read in literal and distance code lengths */
          /* length of code in bits (0..15) */
          l = j;
          let fresh12 = i_0;
          i_0 = i_0.wrapping_add(1);
          ll[fresh12 as usize] = l
        /* save last length in l */
        } else if j == 16i32 as libc::c_uint {
          /* repeat last length 3 to 6 times */
          b_dynamic = fill_bitbuffer(state, b_dynamic, &mut k_dynamic, 2i32 as libc::c_uint);
          j = (3i32 as libc::c_uint).wrapping_add(b_dynamic & 3i32 as libc::c_uint);
          b_dynamic >>= 2i32;
          k_dynamic = k_dynamic.wrapping_sub(2i32 as libc::c_uint);
          if i_0.wrapping_add(j) > n_0 {
            abort_unzip(state);
            //return 1;
          } /* j == 18: 11 to 138 zero length codes */
          loop {
            let fresh13 = j;
            j = j.wrapping_sub(1);
            if !(fresh13 != 0) {
              break;
            }
            let fresh14 = i_0;
            i_0 = i_0.wrapping_add(1);
            ll[fresh14 as usize] = l
          }
        } else if j == 17i32 as libc::c_uint {
          /* 3 to 10 zero length codes */
          b_dynamic = fill_bitbuffer(state, b_dynamic, &mut k_dynamic, 3i32 as libc::c_uint);
          j = (3i32 as libc::c_uint).wrapping_add(b_dynamic & 7i32 as libc::c_uint);
          b_dynamic >>= 3i32;
          k_dynamic = k_dynamic.wrapping_sub(3i32 as libc::c_uint);
          if i_0.wrapping_add(j) > n_0 {
            abort_unzip(state);
            //return 1;
          }
          loop {
            let fresh15 = j;
            j = j.wrapping_sub(1);
            if !(fresh15 != 0) {
              break;
            }
            let fresh16 = i_0;
            i_0 = i_0.wrapping_add(1);
            ll[fresh16 as usize] = 0i32 as libc::c_uint
          }
          l = 0i32 as libc::c_uint
        } else {
          b_dynamic = fill_bitbuffer(state, b_dynamic, &mut k_dynamic, 7i32 as libc::c_uint);
          j = (11i32 as libc::c_uint).wrapping_add(b_dynamic & 0x7fi32 as libc::c_uint);
          b_dynamic >>= 7i32;
          k_dynamic = k_dynamic.wrapping_sub(7i32 as libc::c_uint);
          if i_0.wrapping_add(j) > n_0 {
            abort_unzip(state);
            //return 1;
          }
          loop {
            let fresh17 = j;
            j = j.wrapping_sub(1);
            if !(fresh17 != 0) {
              break;
            }
            let fresh18 = i_0;
            i_0 = i_0.wrapping_add(1);
            ll[fresh18 as usize] = 0i32 as libc::c_uint
          }
          l = 0i32 as libc::c_uint
        }
      }
      huft_free((*state).inflate_codes_tl);
      (*state).gunzip_bb = b_dynamic;
      (*state).gunzip_bk = k_dynamic as libc::c_uchar;
      bl_0 = lbits as libc::c_int as libc::c_uint;
      (*state).inflate_codes_tl =
        huft_build(ll.as_mut_ptr(), nl, 257i32 as libc::c_uint, &lit, &mut bl_0);
      if (*state).inflate_codes_tl as uintptr_t & 1i32 as libc::c_ulong != 0 {
        abort_unzip(state);
      }
      bd_0 = dbits as libc::c_int as libc::c_uint;
      (*state).inflate_codes_td = huft_build(
        ll.as_mut_ptr().offset(nl as isize),
        nd,
        0i32 as libc::c_uint,
        &dist,
        &mut bd_0,
      );
      if (*state).inflate_codes_td as uintptr_t & 1i32 as libc::c_ulong != 0 {
        abort_unzip(state);
      }
      inflate_codes_setup(state, bl_0, bd_0);
      return -2i32;
    }
    _ => {
      abort_unzip(state);
    }
  };
}
/* free decoding table for trees */
/* restore the global bit buffer */
/* build the decoding tables for literal/length and distance codes */
/* set up data for inflate_codes() */
/* huft_free code moved into inflate_codes */
/* Two callsites, both in inflate_get_next_window */
unsafe extern "C" fn calculate_gunzip_crc(mut state: *mut state_t) {
  (*state).gunzip_crc = crc32_block_endian0(
    (*state).gunzip_crc,
    (*state).gunzip_window as *const libc::c_void,
    (*state).gunzip_outbuf_count,
    (*state).gunzip_crc_table,
  );
  (*state).gunzip_bytes_out += (*state).gunzip_outbuf_count as libc::c_long;
}
/* One callsite in inflate_unzip_internal */
unsafe extern "C" fn inflate_get_next_window(mut state: *mut state_t) -> libc::c_int {
  (*state).gunzip_outbuf_count = 0i32 as libc::c_uint;
  loop {
    let mut ret: libc::c_int = 0;
    if (*state).need_another_block != 0 {
      if (*state).end_reached != 0 {
        calculate_gunzip_crc(state);
        (*state).end_reached = 0i32 as smallint;
        /* end of that block */
        /* Last block */
        return 0i32;
      }
      (*state).method = inflate_block(state, &mut (*state).end_reached) as smallint;
      (*state).need_another_block = 0i32 as smallint
    }
    match (*state).method as libc::c_int {
      -1 => ret = inflate_stored(state),
      -2 => ret = inflate_codes(state),
      _ => {
        /* NB: need_another_block is still set */
        /* cannot happen */
        abort_unzip(state);
      }
    }
    if ret == 1i32 {
      calculate_gunzip_crc(state);
      return 1i32;
      /* more data left */
    }
    (*state).need_another_block = 1i32 as smallint
  }
  /* Doesnt get here */
}
/* Called from unpack_gz_stream() and inflate_unzip() */
unsafe extern "C" fn inflate_unzip_internal(
  mut state: *mut state_t,
  mut xstate: *mut transformer_state_t,
) -> libc::c_longlong {
  let mut current_block: u64;
  let mut n: libc::c_longlong = 0i32 as libc::c_longlong;
  let mut nwrote: ssize_t = 0;
  /* Allocate all global buffers (for DYN_ALLOC option) */
  (*state).gunzip_window = xmalloc(GUNZIP_WSIZE as libc::c_int as size_t) as *mut libc::c_uchar;
  (*state).gunzip_outbuf_count = 0i32 as libc::c_uint;
  (*state).gunzip_bytes_out = 0i32 as off_t;
  (*state).gunzip_src_fd = (*xstate).src_fd;
  /* (re) initialize state */
  (*state).method = -1i32 as smallint;
  (*state).need_another_block = 1i32 as smallint;
  (*state).resume_copy = 0i32 as smallint;
  (*state).gunzip_bk = 0i32 as libc::c_uchar;
  (*state).gunzip_bb = 0i32 as libc::c_uint;
  /* Create the crc table */
  (*state).gunzip_crc_table = crc32_new_table_le();
  (*state).gunzip_crc = !0i32 as u32;
  (*state).error_msg = b"corrupted data\x00" as *const u8 as *const libc::c_char;
  if _setjmp((*state).error_jmp.as_mut_ptr()) != 0 {
    /* Error from deep inside zip machinery */
    bb_simple_error_msg((*state).error_msg);
    n = -1i32 as libc::c_longlong
  } else {
    loop {
      let mut r: libc::c_int = inflate_get_next_window(state);
      nwrote = transformer_write(
        xstate,
        (*state).gunzip_window as *const libc::c_void,
        (*state).gunzip_outbuf_count as size_t,
      );
      if nwrote == -1i32 as ssize_t {
        n = -1i32 as libc::c_longlong;
        current_block = 2557277097681042651;
        break;
      } else {
        n += nwrote as libc::c_longlong;
        if r == 0i32 {
          current_block = 5783071609795492627;
          break;
        }
      }
    }
    match current_block {
      2557277097681042651 => {}
      _ => {
        /* Store unused bytes in a global buffer so calling applets can access it */
        if (*state).gunzip_bk as libc::c_int >= 8i32 {
          /* Undo too much lookahead. The next read will be byte aligned
           * so we can discard unused bits in the last meaningful byte. */
          (*state).bytebuffer_offset = (*state).bytebuffer_offset.wrapping_sub(1);
          *(*state)
            .bytebuffer
            .offset((*state).bytebuffer_offset as isize) =
            ((*state).gunzip_bb & 0xffi32 as libc::c_uint) as libc::c_uchar;
          (*state).gunzip_bb >>= 8i32;
          (*state).gunzip_bk = ((*state).gunzip_bk as libc::c_int - 8i32) as libc::c_uchar
        }
      }
    }
  }
  /* Cleanup */
  free((*state).gunzip_window as *mut libc::c_void);
  free((*state).gunzip_crc_table as *mut libc::c_void);
  return n;
}
/* External entry points */
/* For unzip */
#[no_mangle]
pub unsafe extern "C" fn inflate_unzip(mut xstate: *mut transformer_state_t) -> libc::c_longlong {
  let mut n: libc::c_longlong = 0;
  let mut state: *mut state_t = 0 as *mut state_t;
  state = xzalloc(::std::mem::size_of::<state_t>() as libc::c_ulong) as *mut state_t;
  (*state).to_read = (*xstate).bytes_in;
  //	bytebuffer_max = 0x8000;
  (*state).bytebuffer_offset = 4i32 as libc::c_uint;
  (*state).bytebuffer = xmalloc(0x4000i32 as size_t) as *mut libc::c_uchar;
  n = inflate_unzip_internal(state, xstate);
  free((*state).bytebuffer as *mut libc::c_void);
  (*xstate).crc32 = (*state).gunzip_crc;
  (*xstate).bytes_out = (*state).gunzip_bytes_out;
  free(state as *mut libc::c_void);
  return n;
}
/* For gunzip */
/* helpers first */
/* Top up the input buffer with at least n bytes. */
unsafe extern "C" fn top_up(mut state: *mut state_t, mut n: libc::c_uint) -> libc::c_int {
  let mut count: libc::c_int = (*state)
    .bytebuffer_size
    .wrapping_sub((*state).bytebuffer_offset) as libc::c_int;
  if count < n as libc::c_int {
    memmove(
      (*state).bytebuffer as *mut libc::c_void,
      &mut *(*state)
        .bytebuffer
        .offset((*state).bytebuffer_offset as isize) as *mut libc::c_uchar
        as *const libc::c_void,
      count as libc::c_ulong,
    );
    (*state).bytebuffer_offset = 0i32 as libc::c_uint;
    (*state).bytebuffer_size = full_read(
      (*state).gunzip_src_fd,
      &mut *(*state).bytebuffer.offset(count as isize) as *mut libc::c_uchar as *mut libc::c_void,
      (0x4000i32 - count) as size_t,
    ) as libc::c_uint;
    if ((*state).bytebuffer_size as libc::c_int) < 0i32 {
      bb_simple_error_msg(b"read error\x00" as *const u8 as *const libc::c_char);
      return 0i32;
    }
    (*state).bytebuffer_size = (*state).bytebuffer_size.wrapping_add(count as libc::c_uint);
    if (*state).bytebuffer_size < n {
      return 0i32;
    }
  }
  return 1i32;
}
unsafe extern "C" fn buffer_read_le_u16(mut state: *mut state_t) -> u16 {
  let mut res: u16 = 0;
  res = *(&mut *(*state)
    .bytebuffer
    .offset((*state).bytebuffer_offset as isize) as *mut libc::c_uchar
    as *mut bb__aliased_u16);
  (*state).bytebuffer_offset = (*state)
    .bytebuffer_offset
    .wrapping_add(2i32 as libc::c_uint);
  return res;
}
unsafe extern "C" fn buffer_read_le_u32(mut state: *mut state_t) -> u32 {
  let mut res: u32 = 0;
  res = *(&mut *(*state)
    .bytebuffer
    .offset((*state).bytebuffer_offset as isize) as *mut libc::c_uchar
    as *mut bb__aliased_u32);
  (*state).bytebuffer_offset = (*state)
    .bytebuffer_offset
    .wrapping_add(4i32 as libc::c_uint);
  return res;
}
unsafe extern "C" fn check_header_gzip(
  mut state: *mut state_t,
  mut xstate: *mut transformer_state_t,
) -> libc::c_int {
  let mut header: C2RustUnnamed_5 = C2RustUnnamed_5 { raw: [0; 8] };
  /*
   * Rewind bytebuffer. We use the beginning because the header has 8
   * bytes, leaving enough for unwinding afterwards.
   */
  (*state).bytebuffer_size = (*state)
    .bytebuffer_size
    .wrapping_sub((*state).bytebuffer_offset);
  memmove(
    (*state).bytebuffer as *mut libc::c_void,
    &mut *(*state)
      .bytebuffer
      .offset((*state).bytebuffer_offset as isize) as *mut libc::c_uchar as *const libc::c_void,
    (*state).bytebuffer_size as libc::c_ulong,
  );
  (*state).bytebuffer_offset = 0i32 as libc::c_uint;
  if top_up(state, 8i32 as libc::c_uint) == 0 {
    return 0i32;
  }
  memcpy(
    header.raw.as_mut_ptr() as *mut libc::c_void,
    &mut *(*state)
      .bytebuffer
      .offset((*state).bytebuffer_offset as isize) as *mut libc::c_uchar as *const libc::c_void,
    8i32 as libc::c_ulong,
  );
  (*state).bytebuffer_offset = (*state)
    .bytebuffer_offset
    .wrapping_add(8i32 as libc::c_uint);
  /* Check the compression method */
  if header.formatted.gz_method as libc::c_int != 8i32 {
    return 0i32;
  }
  if header.formatted.flags as libc::c_int & 0x4i32 != 0 {
    /* bit 2 set: extra field present */
    let mut extra_short: libc::c_uint = 0;
    if top_up(state, 2i32 as libc::c_uint) == 0 {
      return 0i32;
    }
    extra_short = buffer_read_le_u16(state) as libc::c_uint;
    if top_up(state, extra_short) == 0 {
      return 0i32;
    }
    /* Ignore extra field */
    (*state).bytebuffer_offset = (*state).bytebuffer_offset.wrapping_add(extra_short)
  }
  /* Discard original name and file comment if any */
  /* bit 3 set: original file name present */
  /* bit 4 set: file comment present */
  if header.formatted.flags as libc::c_int & 0x18i32 != 0 {
    loop {
      loop {
        if top_up(state, 1i32 as libc::c_uint) == 0 {
          return 0i32;
        }
        let fresh19 = (*state).bytebuffer_offset;
        (*state).bytebuffer_offset = (*state).bytebuffer_offset.wrapping_add(1);
        if !(*(*state).bytebuffer.offset(fresh19 as isize) as libc::c_int != 0i32) {
          break;
        }
      }
      if header.formatted.flags as libc::c_int & 0x18i32 != 0x18i32 {
        break;
      }
      header.formatted.flags = (header.formatted.flags as libc::c_int & !0x18i32) as u8
    }
  }
  (*xstate).mtime = header.formatted.mtime as time_t;
  /* Read the header checksum */
  if header.formatted.flags as libc::c_int & 0x2i32 != 0 {
    if top_up(state, 2i32 as libc::c_uint) == 0 {
      return 0i32;
    }
    (*state).bytebuffer_offset = (*state)
      .bytebuffer_offset
      .wrapping_add(2i32 as libc::c_uint)
  }
  return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn unpack_gz_stream(
  mut xstate: *mut transformer_state_t,
) -> libc::c_longlong {
  let mut v32: u32 = 0;
  let mut total: libc::c_longlong = 0;
  let mut n: libc::c_longlong = 0;
  let mut state: *mut state_t = 0 as *mut state_t;
  if check_signature16(xstate, GZIP_MAGIC as libc::c_int as libc::c_uint) != 0 {
    return -1i32 as libc::c_longlong;
  }
  total = 0i32 as libc::c_longlong;
  state = xzalloc(::std::mem::size_of::<state_t>() as libc::c_ulong) as *mut state_t;
  (*state).to_read = -1i32 as off_t;
  //	bytebuffer_max = 0x8000;
  (*state).bytebuffer = xmalloc(0x4000i32 as size_t) as *mut libc::c_uchar;
  (*state).gunzip_src_fd = (*xstate).src_fd;
  loop {
    if check_header_gzip(state, xstate) == 0 {
      bb_simple_error_msg(b"corrupted data\x00" as *const u8 as *const libc::c_char);
      total = -1i32 as libc::c_longlong;
      break;
    } else {
      n = inflate_unzip_internal(state, xstate);
      if n < 0i32 as libc::c_longlong {
        total = -1i32 as libc::c_longlong;
        break;
      } else {
        total += n;
        if top_up(state, 8i32 as libc::c_uint) == 0 {
          bb_simple_error_msg(b"corrupted data\x00" as *const u8 as *const libc::c_char);
          total = -1i32 as libc::c_longlong;
          break;
        } else {
          /* Validate decompression - crc */
          v32 = buffer_read_le_u32(state);
          if !(*state).gunzip_crc != v32 {
            bb_simple_error_msg(b"crc error\x00" as *const u8 as *const libc::c_char);
            total = -1i32 as libc::c_longlong;
            break;
          } else {
            /* Validate decompression - size */
            v32 = buffer_read_le_u32(state); /* EOF */
            if (*state).gunzip_bytes_out as u32 != v32 {
              bb_simple_error_msg(b"incorrect length\x00" as *const u8 as *const libc::c_char);
              total = -1i32 as libc::c_longlong
            }
            if top_up(state, 2i32 as libc::c_uint) == 0 {
              break;
            }
            if !(*(*state)
              .bytebuffer
              .offset((*state).bytebuffer_offset as isize) as libc::c_int
              == 0x1fi32
              && *(*state).bytebuffer.offset(
                (*state)
                  .bytebuffer_offset
                  .wrapping_add(1i32 as libc::c_uint) as isize,
              ) as libc::c_int
                == 0x8bi32)
            {
              break;
            }
            (*state).bytebuffer_offset = (*state)
              .bytebuffer_offset
              .wrapping_add(2i32 as libc::c_uint)
          }
        }
      }
    }
  }
  /* GNU gzip says: */
  /*bb_error_msg("decompression OK, trailing garbage ignored");*/
  free((*state).bytebuffer as *mut libc::c_void);
  free(state as *mut libc::c_void);
  return total;
}

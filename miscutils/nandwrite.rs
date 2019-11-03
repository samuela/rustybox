use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xlseek(fd: libc::c_int, offset: off_t, whence: libc::c_int) -> off_t;
  #[no_mangle]
  fn full_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn xwrite(fd: libc::c_int, buf: *const libc::c_void, count: size_t);
  #[no_mangle]
  fn xstrtou(str: *const libc::c_char, b: libc::c_int) -> libc::c_uint;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> uint32_t;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_xioctl(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __loff_t = __off64_t;
pub type uint32_t = __uint32_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type off_t = __off64_t;
pub type loff_t = __loff_t;
pub type __u8 = libc::c_uchar;
pub type __u32 = libc::c_uint;
pub type __u64 = libc::c_ulonglong;
pub type __kernel_loff_t = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtd_oob_buf {
  pub start: __u32,
  pub length: __u32,
  pub ptr: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtd_info_user {
  pub type_0: __u8,
  pub flags: __u32,
  pub size: __u32,
  pub erasesize: __u32,
  pub writesize: __u32,
  pub oobsize: __u32,
  pub padding: __u64,
}
/* helper for writing out 0xff for bad blocks pad */
unsafe extern "C" fn dump_bad(
  mut meminfo: *mut mtd_info_user,
  mut len: libc::c_uint,
  mut oob: libc::c_int,
) {
  let vla = (*meminfo).writesize as usize;
  let mut buf: Vec<libc::c_uchar> = ::std::vec::from_elem(0, vla);
  let mut count: libc::c_uint = 0;
  /* round len to the next page only if len is not already on a page */
  len = (len.wrapping_sub(1i32 as libc::c_uint)
    | (*meminfo).writesize.wrapping_sub(1i32 as libc::c_uint))
  .wrapping_add(1i32 as libc::c_uint);
  memset(
    buf.as_mut_ptr() as *mut libc::c_void,
    0xffi32,
    (vla * ::std::mem::size_of::<libc::c_uchar>()) as libc::c_ulong,
  );
  count = 0i32 as libc::c_uint;
  while count < len {
    xwrite(
      1i32,
      buf.as_mut_ptr() as *const libc::c_void,
      (*meminfo).writesize as size_t,
    );
    if oob != 0 {
      xwrite(
        1i32,
        buf.as_mut_ptr() as *const libc::c_void,
        (*meminfo).oobsize as size_t,
      );
    }
    count = count.wrapping_add((*meminfo).writesize)
  }
}
unsafe extern "C" fn next_good_eraseblock(
  mut fd: libc::c_int,
  mut meminfo: *mut mtd_info_user,
  mut block_offset: libc::c_uint,
) -> libc::c_uint {
  loop {
    let mut offs: loff_t = 0;
    if block_offset >= (*meminfo).size {
      if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(4) as libc::c_int != 'd' as i32) {
        bb_simple_error_msg_and_die(
          b"not enough space in MTD device\x00" as *const u8 as *const libc::c_char,
        );
      }
      return block_offset;
      /* let the caller exit */
    }
    offs = block_offset as loff_t;
    if bb_xioctl(
      fd,
      ((1u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (('M' as i32) << 0i32 + 8i32) as libc::c_uint
        | (11i32 << 0i32) as libc::c_uint) as libc::c_ulong
        | (::std::mem::size_of::<__kernel_loff_t>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
        as libc::c_uint,
      &mut offs as *mut loff_t as *mut libc::c_void,
      b"MEMGETBADBLOCK\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
      return block_offset;
    }
    /* ioctl returned 1 => "bad block" */
    if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(4) as libc::c_int != 'd' as i32) {
      printf(
        b"Skipping bad block at 0x%08x\n\x00" as *const u8 as *const libc::c_char,
        block_offset,
      );
    }
    block_offset = block_offset.wrapping_add((*meminfo).erasesize)
  }
}
#[no_mangle]
pub unsafe extern "C" fn nandwrite_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  /* Buffer for OOB data */
  let mut oobbuf: *mut libc::c_uchar = 0 as *mut libc::c_uchar; /* nandwrite */
  let mut opts: libc::c_uint = 0;
  let mut bb_method: libc::c_uint = (1i32 << 1i32) as libc::c_uint;
  let mut fd: libc::c_int = 0;
  let mut cnt: ssize_t = 0;
  let mut mtdoffset: libc::c_uint = 0;
  let mut meminfo_writesize: libc::c_uint = 0;
  let mut blockstart: libc::c_uint = 0;
  let mut limit: libc::c_uint = 0;
  let mut end_addr: libc::c_uint = !0i32 as libc::c_uint;
  let mut meminfo: mtd_info_user = mtd_info_user {
    type_0: 0,
    flags: 0,
    size: 0,
    erasesize: 0,
    writesize: 0,
    oobsize: 0,
    padding: 0,
  };
  let mut oob: mtd_oob_buf = mtd_oob_buf {
    start: 0,
    length: 0,
    ptr: 0 as *mut libc::c_uchar,
  };
  let mut filebuf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
  let mut opt_s: *const libc::c_char = b"0\x00" as *const u8 as *const libc::c_char;
  let mut opt_f: *const libc::c_char = b"-\x00" as *const u8 as *const libc::c_char;
  let mut opt_l: *const libc::c_char = 0 as *const libc::c_char;
  let mut opt_bb: *const libc::c_char = 0 as *const libc::c_char;
  if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(4) as libc::c_int == 'd' as i32) {
    opts = getopt32long(
      argv,
      b"^ons:f:l:\x00=1\x00" as *const u8 as *const libc::c_char,
      b"bb\x00\x01\xff\x00" as *const u8 as *const libc::c_char,
      &mut opt_s as *mut *const libc::c_char,
      &mut opt_f as *mut *const libc::c_char,
      &mut opt_l as *mut *const libc::c_char,
      &mut opt_bb as *mut *const libc::c_char,
    )
  } else {
    opts = getopt32(
      argv,
      b"^pns:\x00-1:?2\x00" as *const u8 as *const libc::c_char,
      &mut opt_s as *mut *const libc::c_char,
    )
  }
  argv = argv.offset(optind as isize);
  if 1i32 != 0
    && (1i32 == 0 || *applet_name.offset(4) as libc::c_int != 'd' as i32)
    && !(*argv.offset(1)).is_null()
  {
    opt_f = *argv.offset(1)
  }
  if !(*opt_f.offset(0) as libc::c_int == '-' as i32 && *opt_f.offset(1) == 0) {
    let mut tmp_fd: libc::c_int = xopen(
      opt_f,
      if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(4) as libc::c_int == 'd' as i32) {
        (0o1i32 | 0o1000i32) | 0o100i32
      } else {
        0i32
      },
    );
    xmove_fd(
      tmp_fd,
      if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(4) as libc::c_int == 'd' as i32) {
        1i32
      } else {
        0i32
      },
    );
  }
  fd = xopen(
    *argv.offset(0),
    if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(4) as libc::c_int != 'd' as i32) {
      0o2i32
    } else {
      0i32
    },
  );
  bb_xioctl(
    fd,
    ((2u32 << 0i32 + 8i32 + 8i32 + 14i32
      | (('M' as i32) << 0i32 + 8i32) as libc::c_uint
      | (1i32 << 0i32) as libc::c_uint) as libc::c_ulong
      | (::std::mem::size_of::<mtd_info_user>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
      as libc::c_uint,
    &mut meminfo as *mut mtd_info_user as *mut libc::c_void,
    b"MEMGETINFO\x00" as *const u8 as *const libc::c_char,
  );
  if opts & (1i32 << 1i32) as libc::c_uint != 0 {
    bb_xioctl(
      fd,
      0u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (('M' as i32) << 0i32 + 8i32) as libc::c_uint
        | (19i32 << 0i32) as libc::c_uint
        | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint,
      3i32 as *mut libc::c_void,
      b"MTDFILEMODE\x00" as *const u8 as *const libc::c_char,
    );
  }
  mtdoffset = xstrtou(opt_s, 0i32);
  if 1i32 != 0
    && (1i32 == 0 || *applet_name.offset(4) as libc::c_int == 'd' as i32)
    && opts & (1i32 << 4i32) as libc::c_uint != 0
  {
    let mut length: libc::c_uint = xstrtou(opt_l, 0i32);
    if length < meminfo.size.wrapping_sub(mtdoffset) {
      end_addr = mtdoffset.wrapping_add(length)
    }
  }
  if 1i32 != 0
    && (1i32 == 0 || *applet_name.offset(4) as libc::c_int == 'd' as i32)
    && opts & (1i32 << 5i32) as libc::c_uint != 0
  {
    if strcmp(b"skipbad\x00" as *const u8 as *const libc::c_char, opt_bb) == 0i32 {
      bb_method = (1i32 << 1i32) as libc::c_uint
    } else if strcmp(b"padbad\x00" as *const u8 as *const libc::c_char, opt_bb) == 0i32 {
      bb_method = (1i32 << 0i32) as libc::c_uint
    } else {
      bb_show_usage();
    }
  }
  /* Pull it into a CPU register (hopefully) - smaller code that way */
  meminfo_writesize = meminfo.writesize;
  if mtdoffset & meminfo_writesize.wrapping_sub(1i32 as libc::c_uint) != 0 {
    bb_simple_error_msg_and_die(
      b"start address is not page aligned\x00" as *const u8 as *const libc::c_char,
    );
  }
  filebuf = xmalloc(meminfo_writesize as size_t) as *mut libc::c_uchar;
  oobbuf = xmalloc(meminfo.oobsize as size_t) as *mut libc::c_uchar;
  oob.start = 0i32 as __u32;
  oob.length = meminfo.oobsize;
  oob.ptr = oobbuf;
  blockstart = mtdoffset & !meminfo.erasesize.wrapping_sub(1i32 as libc::c_uint);
  if blockstart != mtdoffset {
    let mut tmp: libc::c_uint = 0;
    /* mtdoffset is in the middle of an erase block, verify that
     * this block is OK. Advance mtdoffset only if this block is
     * bad.
     */
    tmp = next_good_eraseblock(fd, &mut meminfo, blockstart);
    if tmp != blockstart {
      /* bad block(s), advance mtdoffset */
      if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(4) as libc::c_int == 'd' as i32) {
        if bb_method == (1i32 << 0i32) as libc::c_uint {
          let mut bad_len: libc::c_int =
            (if tmp < end_addr { tmp } else { end_addr }).wrapping_sub(mtdoffset) as libc::c_int;
          dump_bad(
            &mut meminfo,
            bad_len as libc::c_uint,
            (opts & (1i32 << 0i32) as libc::c_uint) as libc::c_int,
          );
        }
        /* with option skipbad, increase the total length */
        if bb_method == (1i32 << 1i32) as libc::c_uint {
          end_addr = end_addr.wrapping_add(tmp.wrapping_sub(blockstart))
        }
      }
      mtdoffset = tmp
    }
  }
  cnt = -1i32 as ssize_t;
  limit = if meminfo.size < end_addr {
    meminfo.size
  } else {
    end_addr
  };
  while mtdoffset < limit {
    let mut input_fd: libc::c_int =
      if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(4) as libc::c_int != 'd' as i32) {
        0i32
      } else {
        fd
      };
    let mut output_fd: libc::c_int =
      if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(4) as libc::c_int != 'd' as i32) {
        fd
      } else {
        1i32
      };
    blockstart = mtdoffset & !meminfo.erasesize.wrapping_sub(1i32 as libc::c_uint);
    if blockstart == mtdoffset {
      /* starting a new eraseblock */
      mtdoffset = next_good_eraseblock(fd, &mut meminfo, blockstart);
      if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(4) as libc::c_int != 'd' as i32) {
        printf(
          b"Writing at 0x%08x\n\x00" as *const u8 as *const libc::c_char,
          mtdoffset,
        );
      } else if mtdoffset > blockstart {
        if bb_method == (1i32 << 0i32) as libc::c_uint {
          /* dump FF padded bad block */
          let mut bad_len_0: libc::c_int = (if mtdoffset < limit { mtdoffset } else { limit })
            .wrapping_sub(blockstart) as libc::c_int;
          dump_bad(
            &mut meminfo,
            bad_len_0 as libc::c_uint,
            (opts & (1i32 << 0i32) as libc::c_uint) as libc::c_int,
          );
        } else if bb_method == (1i32 << 1i32) as libc::c_uint {
          /* for skipbad, increase the length */
          if end_addr.wrapping_add(mtdoffset).wrapping_sub(blockstart) > end_addr {
            end_addr = end_addr.wrapping_add(mtdoffset.wrapping_sub(blockstart))
          } else {
            end_addr = !0i32 as libc::c_uint
          }
          limit = if meminfo.size < end_addr {
            meminfo.size
          } else {
            end_addr
          }
        }
      }
      if mtdoffset >= limit {
        break;
      }
    }
    xlseek(fd, mtdoffset as off_t, 0i32);
    /* get some more data from input */
    cnt = full_read(
      input_fd,
      filebuf as *mut libc::c_void,
      meminfo_writesize as size_t,
    );
    if cnt == 0i32 as libc::c_long {
      break;
    }
    if cnt < meminfo_writesize as libc::c_long {
      if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(4) as libc::c_int == 'd' as i32) {
        bb_simple_error_msg_and_die(b"short read\x00" as *const u8 as *const libc::c_char);
      }
      if opts & (1i32 << 0i32) as libc::c_uint == 0 {
        bb_simple_error_msg_and_die(
          b"input size is not rounded up to page size, use -p to zero pad\x00" as *const u8
            as *const libc::c_char,
        );
      }
      /* zero pad to end of write block */
      memset(
        filebuf.offset(cnt as isize) as *mut libc::c_void,
        0i32,
        (meminfo_writesize as libc::c_long - cnt) as libc::c_ulong,
      );
    }
    xwrite(
      output_fd,
      filebuf as *const libc::c_void,
      meminfo_writesize as size_t,
    );
    if 1i32 != 0
      && (1i32 == 0 || *applet_name.offset(4) as libc::c_int == 'd' as i32)
      && opts & (1i32 << 0i32) as libc::c_uint != 0
    {
      /* Dump OOB data */
      oob.start = mtdoffset;
      bb_xioctl(
        fd,
        (((2u32 | 1u32) << 0i32 + 8i32 + 8i32 + 14i32
          | (('M' as i32) << 0i32 + 8i32) as libc::c_uint
          | (4i32 << 0i32) as libc::c_uint) as libc::c_ulong
          | (::std::mem::size_of::<mtd_oob_buf>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
          as libc::c_uint,
        &mut oob as *mut mtd_oob_buf as *mut libc::c_void,
        b"MEMREADOOB\x00" as *const u8 as *const libc::c_char,
      );
      xwrite(
        output_fd,
        oobbuf as *const libc::c_void,
        meminfo.oobsize as size_t,
      );
    }
    mtdoffset = mtdoffset.wrapping_add(meminfo_writesize);
    if cnt < meminfo_writesize as libc::c_long {
      break;
    }
  }
  if 1i32 != 0
    && (1i32 == 0 || *applet_name.offset(4) as libc::c_int != 'd' as i32)
    && cnt != 0i32 as libc::c_long
  {
    /* We filled entire MTD, but did we reach EOF on input? */
    if full_read(
      0i32,
      filebuf as *mut libc::c_void,
      meminfo_writesize as size_t,
    ) != 0i32 as libc::c_long
    {
      /* no */
      bb_simple_error_msg_and_die(
        b"not enough space in MTD device\x00" as *const u8 as *const libc::c_char,
      );
    }
  }
  return 0i32;
}

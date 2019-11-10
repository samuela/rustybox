use crate::librb::md5_ctx_t;
use crate::librb::sha1_ctx_t;
use crate::librb::sha256_ctx_t;
use crate::librb::sha3_ctx_t;
use crate::librb::sha512_ctx_t;
use crate::librb::size_t;
use libc;
use libc::close;
use libc::free;
use libc::printf;
use libc::ssize_t;
use libc::strcmp;
use libc::strstr;
use libc::FILE;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn open_or_warn_stdin(pathname: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;

  #[no_mangle]
  fn xmalloc_fgetline(file: *mut FILE) -> *mut libc::c_char;

  #[no_mangle]
  fn fclose_if_not_stdin(file: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn xfopen_stdin(filename: *const libc::c_char) -> *mut FILE;

  #[no_mangle]
  fn bin2hex(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    count: libc::c_int,
  ) -> *mut libc::c_char;

  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;

  #[no_mangle]
  fn xfunc_die() -> !;

  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);

  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);

  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;

  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);

  #[no_mangle]
  fn md5_begin(ctx: *mut md5_ctx_t);

  #[no_mangle]
  fn md5_hash(ctx: *mut md5_ctx_t, buffer: *const libc::c_void, len: size_t);

  #[no_mangle]
  fn md5_end(ctx: *mut md5_ctx_t, resbuf: *mut libc::c_void) -> libc::c_uint;

  #[no_mangle]
  fn sha1_begin(ctx: *mut sha1_ctx_t);

  #[no_mangle]
  fn sha1_end(ctx: *mut sha1_ctx_t, resbuf: *mut libc::c_void) -> libc::c_uint;

  #[no_mangle]
  fn sha256_begin(ctx: *mut sha256_ctx_t);

  #[no_mangle]
  fn sha512_begin(ctx: *mut sha512_ctx_t);

  #[no_mangle]
  fn sha512_hash(ctx: *mut sha512_ctx_t, buffer: *const libc::c_void, len: size_t);

  #[no_mangle]
  fn sha512_end(ctx: *mut sha512_ctx_t, resbuf: *mut libc::c_void) -> libc::c_uint;

  #[no_mangle]
  fn sha3_begin(ctx: *mut sha3_ctx_t);

  #[no_mangle]
  fn sha3_hash(ctx: *mut sha3_ctx_t, buffer: *const libc::c_void, len: size_t);

  #[no_mangle]
  fn sha3_end(ctx: *mut sha3_ctx_t, resbuf: *mut libc::c_void) -> libc::c_uint;

  #[no_mangle]
  static mut applet_name: *const libc::c_char;
}

pub type C2RustUnnamed = libc::c_uint;
pub const HASH_SHA512: C2RustUnnamed = 53;
pub const HASH_SHA3: C2RustUnnamed = 51;
pub const HASH_SHA256: C2RustUnnamed = 50;
/* "md5>s<um" */
pub const HASH_SHA1: C2RustUnnamed = 49;
/* 4th letter of applet_name is... */
pub const HASH_MD5: C2RustUnnamed = 115;

#[derive(Copy, Clone)]
#[repr(C)]
pub union _ctx_ {
  pub sha3: sha3_ctx_t,
  pub sha512: sha512_ctx_t,
  pub sha256: sha256_ctx_t,
  pub sha1: sha1_ctx_t,
  pub md5: md5_ctx_t,
}

/* This might be useful elsewhere */
unsafe extern "C" fn hash_bin_to_hex(
  mut hash_value: *mut libc::c_uchar,
  mut hash_length: libc::c_uint,
) -> *mut libc::c_uchar {
  /* xzalloc zero-terminates */
  let mut hex_value: *mut libc::c_char = xzalloc(
    hash_length
      .wrapping_mul(2i32 as libc::c_uint)
      .wrapping_add(1i32 as libc::c_uint) as size_t,
  ) as *mut libc::c_char;
  bin2hex(
    hex_value,
    hash_value as *mut libc::c_char,
    hash_length as libc::c_int,
  );
  return hex_value as *mut libc::c_uchar;
}

unsafe extern "C" fn hash_file(
  mut filename: *const libc::c_char,
  mut sha3_width: libc::c_uint,
) -> *mut u8 {
  let mut src_fd: libc::c_int = 0;
  let mut hash_len: libc::c_int = 0;
  let mut count: libc::c_int = 0;
  let mut context: _ctx_ = _ctx_ {
    sha3: sha3_ctx_t {
      state: [0; 25],
      bytes_queued: 0,
      input_block_bytes: 0,
    },
  };
  let mut hash_value: *mut u8 = 0 as *mut u8;
  let mut update: Option<
    unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_void, _: size_t) -> (),
  > = None;
  let mut final_0: Option<
    unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void) -> libc::c_uint,
  > = None;
  let mut hash_algo: libc::c_char = 0;
  src_fd = open_or_warn_stdin(filename);
  if src_fd < 0i32 {
    return 0 as *mut u8;
  }
  hash_algo = *applet_name.offset(3);
  /* figure specific hash algorithms */
  if 1i32 != 0 && hash_algo as libc::c_int == HASH_MD5 as libc::c_int {
    md5_begin(&mut context.md5);
    update = ::std::mem::transmute::<
      *mut libc::c_void,
      Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_void, _: size_t) -> ()>,
    >(::std::mem::transmute::<
      Option<unsafe extern "C" fn(_: *mut md5_ctx_t, _: *const libc::c_void, _: size_t) -> ()>,
      *mut libc::c_void,
    >(Some(
      md5_hash as unsafe extern "C" fn(_: *mut md5_ctx_t, _: *const libc::c_void, _: size_t) -> (),
    )));
    final_0 = ::std::mem::transmute::<
      *mut libc::c_void,
      Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void) -> libc::c_uint>,
    >(::std::mem::transmute::<
      Option<unsafe extern "C" fn(_: *mut md5_ctx_t, _: *mut libc::c_void) -> libc::c_uint>,
      *mut libc::c_void,
    >(Some(
      md5_end as unsafe extern "C" fn(_: *mut md5_ctx_t, _: *mut libc::c_void) -> libc::c_uint,
    )));
    hash_len = 16i32
  } else if 1i32 != 0 && hash_algo as libc::c_int == HASH_SHA1 as libc::c_int {
    sha1_begin(&mut context.sha1);
    update = ::std::mem::transmute::<
      *mut libc::c_void,
      Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_void, _: size_t) -> ()>,
    >(::std::mem::transmute::<
      Option<unsafe extern "C" fn(_: *mut md5_ctx_t, _: *const libc::c_void, _: size_t) -> ()>,
      *mut libc::c_void,
    >(Some(
      md5_hash as unsafe extern "C" fn(_: *mut md5_ctx_t, _: *const libc::c_void, _: size_t) -> (),
    )));
    final_0 = ::std::mem::transmute::<
      *mut libc::c_void,
      Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void) -> libc::c_uint>,
    >(::std::mem::transmute::<
      Option<unsafe extern "C" fn(_: *mut sha1_ctx_t, _: *mut libc::c_void) -> libc::c_uint>,
      *mut libc::c_void,
    >(Some(
      sha1_end as unsafe extern "C" fn(_: *mut sha1_ctx_t, _: *mut libc::c_void) -> libc::c_uint,
    )));
    hash_len = 20i32
  } else if 1i32 != 0 && hash_algo as libc::c_int == HASH_SHA256 as libc::c_int {
    sha256_begin(&mut context.sha256);
    update = ::std::mem::transmute::<
      *mut libc::c_void,
      Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_void, _: size_t) -> ()>,
    >(::std::mem::transmute::<
      Option<unsafe extern "C" fn(_: *mut md5_ctx_t, _: *const libc::c_void, _: size_t) -> ()>,
      *mut libc::c_void,
    >(Some(
      md5_hash as unsafe extern "C" fn(_: *mut md5_ctx_t, _: *const libc::c_void, _: size_t) -> (),
    )));
    final_0 = ::std::mem::transmute::<
      *mut libc::c_void,
      Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void) -> libc::c_uint>,
    >(::std::mem::transmute::<
      Option<unsafe extern "C" fn(_: *mut sha1_ctx_t, _: *mut libc::c_void) -> libc::c_uint>,
      *mut libc::c_void,
    >(Some(
      sha1_end as unsafe extern "C" fn(_: *mut sha1_ctx_t, _: *mut libc::c_void) -> libc::c_uint,
    )));
    hash_len = 32i32
  } else if 1i32 != 0 && hash_algo as libc::c_int == HASH_SHA512 as libc::c_int {
    sha512_begin(&mut context.sha512);
    update = ::std::mem::transmute::<
      *mut libc::c_void,
      Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_void, _: size_t) -> ()>,
    >(::std::mem::transmute::<
      Option<unsafe extern "C" fn(_: *mut sha512_ctx_t, _: *const libc::c_void, _: size_t) -> ()>,
      *mut libc::c_void,
    >(Some(
      sha512_hash
        as unsafe extern "C" fn(_: *mut sha512_ctx_t, _: *const libc::c_void, _: size_t) -> (),
    )));
    final_0 = ::std::mem::transmute::<
      *mut libc::c_void,
      Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void) -> libc::c_uint>,
    >(::std::mem::transmute::<
      Option<unsafe extern "C" fn(_: *mut sha512_ctx_t, _: *mut libc::c_void) -> libc::c_uint>,
      *mut libc::c_void,
    >(Some(
      sha512_end
        as unsafe extern "C" fn(_: *mut sha512_ctx_t, _: *mut libc::c_void) -> libc::c_uint,
    )));
    hash_len = 64i32
  } else if 1i32 != 0 && hash_algo as libc::c_int == HASH_SHA3 as libc::c_int {
    sha3_begin(&mut context.sha3);
    update = ::std::mem::transmute::<
      *mut libc::c_void,
      Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_void, _: size_t) -> ()>,
    >(::std::mem::transmute::<
      Option<unsafe extern "C" fn(_: *mut sha3_ctx_t, _: *const libc::c_void, _: size_t) -> ()>,
      *mut libc::c_void,
    >(Some(
      sha3_hash
        as unsafe extern "C" fn(_: *mut sha3_ctx_t, _: *const libc::c_void, _: size_t) -> (),
    )));
    final_0 = ::std::mem::transmute::<
      *mut libc::c_void,
      Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void) -> libc::c_uint>,
    >(::std::mem::transmute::<
      Option<unsafe extern "C" fn(_: *mut sha3_ctx_t, _: *mut libc::c_void) -> libc::c_uint>,
      *mut libc::c_void,
    >(Some(
      sha3_end as unsafe extern "C" fn(_: *mut sha3_ctx_t, _: *mut libc::c_void) -> libc::c_uint,
    )));
    /*
     * Should support 224, 256, 384, 512.
     * We allow any value which does not blow the algorithm up.
     */
    if sha3_width >= (1600i32 / 2i32) as libc::c_uint
      || sha3_width == 0i32 as libc::c_uint
      || sha3_width & 0x1fi32 as libc::c_uint != 0
    {
      /* should be multiple of 32 */
      /* (because input uses up to 8 byte wide word XORs. 32/4=8) */
      bb_error_msg_and_die(
        b"bad -a%u\x00" as *const u8 as *const libc::c_char,
        sha3_width,
      );
    }
    sha3_width = sha3_width.wrapping_div(4i32 as libc::c_uint);
    context.sha3.input_block_bytes = ((1600i32 / 8i32) as libc::c_uint).wrapping_sub(sha3_width);
    hash_len = sha3_width.wrapping_div(2i32 as libc::c_uint) as libc::c_int
  } else {
    xfunc_die();
    /* can't reach this */
  }
  let mut in_buf: *mut libc::c_uchar = xmalloc(4096i32 as size_t) as *mut libc::c_uchar;
  loop {
    count = safe_read(src_fd, in_buf as *mut libc::c_void, 4096i32 as size_t) as libc::c_int;
    if !(count > 0i32) {
      break;
    }
    update.expect("non-null function pointer")(
      &mut context as *mut _ctx_ as *mut libc::c_void,
      in_buf as *const libc::c_void,
      count as size_t,
    );
  }
  hash_value = 0 as *mut u8;
  if count < 0i32 {
    bb_perror_msg(
      b"can\'t read \'%s\'\x00" as *const u8 as *const libc::c_char,
      filename,
    );
  } else {
    /* count == 0 */
    final_0.expect("non-null function pointer")(
      &mut context as *mut _ctx_ as *mut libc::c_void,
      in_buf as *mut libc::c_void,
    );
    hash_value = hash_bin_to_hex(in_buf, hash_len as libc::c_uint)
  }
  free(in_buf as *mut libc::c_void);
  if src_fd != 0i32 {
    close(src_fd);
  }
  return hash_value;
}

#[no_mangle]
pub unsafe extern "C" fn md5_sha1_sum_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut return_value: libc::c_int = 0i32;
  let mut flags: libc::c_uint = 0;
  let mut sha3_width: libc::c_uint = 224i32 as libc::c_uint;
  /* -b "binary", -t "text" are ignored (shaNNNsum compat) */
  /* -s and -w require -c */
  if *applet_name.offset(3) as libc::c_int == HASH_SHA3 as libc::c_int {
    flags = getopt32(
      argv,
      b"^scwbta:+\x00s?c:w?c\x00" as *const u8 as *const libc::c_char,
      &mut sha3_width as *mut libc::c_uint,
    )
  } else {
    flags = getopt32(
      argv,
      b"^scwbt\x00s?c:w?c\x00" as *const u8 as *const libc::c_char,
    )
  }
  argv = argv.offset(optind as isize);
  //argc -= optind;
  if (*argv).is_null() {
    argv = argv.offset(-1);
    *argv = b"-\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }
  loop {
    if 1i32 != 0 && flags & 2i32 as libc::c_uint != 0 {
      let mut pre_computed_stream: *mut FILE = 0 as *mut FILE;
      let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
      let mut count_total: libc::c_int = 0i32;
      let mut count_failed: libc::c_int = 0i32;
      pre_computed_stream = xfopen_stdin(*argv);
      loop {
        line = xmalloc_fgetline(pre_computed_stream);
        if line.is_null() {
          break;
        }
        let mut hash_value: *mut u8 = 0 as *mut u8;
        let mut filename_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
        count_total += 1;
        filename_ptr = strstr(line, b"  \x00" as *const u8 as *const libc::c_char);
        /* handle format for binary checksums */
        if filename_ptr.is_null() {
          filename_ptr = strstr(line, b" *\x00" as *const u8 as *const libc::c_char)
        }
        if filename_ptr.is_null() {
          if flags & 4i32 as libc::c_uint != 0 {
            bb_simple_error_msg(b"invalid format\x00" as *const u8 as *const libc::c_char);
          }
          count_failed += 1;
          return_value = 1i32;
          free(line as *mut libc::c_void);
        } else {
          *filename_ptr = '\u{0}' as i32 as libc::c_char;
          filename_ptr = filename_ptr.offset(2);
          hash_value = hash_file(filename_ptr, sha3_width);
          if !hash_value.is_null() && strcmp(hash_value as *mut libc::c_char, line) == 0i32 {
            if flags & 1i32 as libc::c_uint == 0 {
              printf(
                b"%s: OK\n\x00" as *const u8 as *const libc::c_char,
                filename_ptr,
              );
            }
          } else {
            if flags & 1i32 as libc::c_uint == 0 {
              printf(
                b"%s: FAILED\n\x00" as *const u8 as *const libc::c_char,
                filename_ptr,
              );
            }
            count_failed += 1;
            return_value = 1i32
          }
          /* possible free(NULL) */
          free(hash_value as *mut libc::c_void);
          free(line as *mut libc::c_void);
        }
      }
      if count_failed != 0 && flags & 1i32 as libc::c_uint == 0 {
        bb_error_msg(
          b"WARNING: %d of %d computed checksums did NOT match\x00" as *const u8
            as *const libc::c_char,
          count_failed,
          count_total,
        );
      }
      if count_total == 0i32 {
        return_value = 1i32;
        /*
         * md5sum from GNU coreutils 8.25 says:
         * md5sum: <FILE>: no properly formatted MD5 checksum lines found
         */
        bb_error_msg(
          b"%s: no checksum lines found\x00" as *const u8 as *const libc::c_char,
          *argv,
        );
      }
      fclose_if_not_stdin(pre_computed_stream);
    } else {
      let mut hash_value_0: *mut u8 = hash_file(*argv, sha3_width);
      if hash_value_0.is_null() {
        return_value = 1i32
      } else {
        printf(
          b"%s  %s\n\x00" as *const u8 as *const libc::c_char,
          hash_value_0,
          *argv,
        );
        free(hash_value_0 as *mut libc::c_void);
      }
    }
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  return return_value;
}

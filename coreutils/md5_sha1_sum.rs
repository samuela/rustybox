use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
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
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
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
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;

use crate::librb::md5_ctx_t;
use crate::librb::FILE;
pub type sha1_ctx_t = md5_ctx_t;
pub type sha256_ctx_t = md5_ctx_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha512_ctx_t {
  pub total64: [uint64_t; 2],
  pub hash: [uint64_t; 8],
  pub wbuffer: [uint8_t; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha3_ctx_t {
  pub state: [uint64_t; 25],
  pub bytes_queued: libc::c_uint,
  pub input_block_bytes: libc::c_uint,
}
/* vi: set sw=4 ts=4: */
/*
 * Copyright (C) 2003 Glenn L. McGrath
 * Copyright (C) 2003-2004 Erik Andersen
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config MD5SUM
//config:	bool "md5sum (6.5 kb)"
//config:	default y
//config:	help
//config:	Compute and check MD5 message digest
//config:
//config:config SHA1SUM
//config:	bool "sha1sum (5.9 kb)"
//config:	default y
//config:	help
//config:	Compute and check SHA1 message digest
//config:
//config:config SHA256SUM
//config:	bool "sha256sum (7 kb)"
//config:	default y
//config:	help
//config:	Compute and check SHA256 message digest
//config:
//config:config SHA512SUM
//config:	bool "sha512sum (7.4 kb)"
//config:	default y
//config:	help
//config:	Compute and check SHA512 message digest
//config:
//config:config SHA3SUM
//config:	bool "sha3sum (6.1 kb)"
//config:	default y
//config:	help
//config:	Compute and check SHA3 message digest
//config:
//config:comment "Common options for md5sum, sha1sum, sha256sum, sha512sum, sha3sum"
//config:	depends on MD5SUM || SHA1SUM || SHA256SUM || SHA512SUM || SHA3SUM
//config:
//config:config FEATURE_MD5_SHA1_SUM_CHECK
//config:	bool "Enable -c, -s and -w options"
//config:	default y
//config:	depends on MD5SUM || SHA1SUM || SHA256SUM || SHA512SUM || SHA3SUM
//config:	help
//config:	Enabling the -c options allows files to be checked
//config:	against pre-calculated hash values.
//config:	-s and -w are useful options when verifying checksums.
//applet:IF_MD5SUM(APPLET_NOEXEC(md5sum, md5_sha1_sum, BB_DIR_USR_BIN, BB_SUID_DROP, md5sum))
//applet:IF_SHA1SUM(APPLET_NOEXEC(sha1sum, md5_sha1_sum, BB_DIR_USR_BIN, BB_SUID_DROP, sha1sum))
//applet:IF_SHA3SUM(APPLET_NOEXEC(sha3sum, md5_sha1_sum, BB_DIR_USR_BIN, BB_SUID_DROP, sha3sum))
//applet:IF_SHA256SUM(APPLET_NOEXEC(sha256sum, md5_sha1_sum, BB_DIR_USR_BIN, BB_SUID_DROP, sha256sum))
//applet:IF_SHA512SUM(APPLET_NOEXEC(sha512sum, md5_sha1_sum, BB_DIR_USR_BIN, BB_SUID_DROP, sha512sum))
//kbuild:lib-$(CONFIG_MD5SUM)    += md5_sha1_sum.o
//kbuild:lib-$(CONFIG_SHA1SUM)   += md5_sha1_sum.o
//kbuild:lib-$(CONFIG_SHA256SUM) += md5_sha1_sum.o
//kbuild:lib-$(CONFIG_SHA512SUM) += md5_sha1_sum.o
//kbuild:lib-$(CONFIG_SHA3SUM)   += md5_sha1_sum.o
//usage:#define md5sum_trivial_usage
//usage:	IF_FEATURE_MD5_SHA1_SUM_CHECK("[-c[sw]] ")"[FILE]..."
//usage:#define md5sum_full_usage "\n\n"
//usage:       "Print" IF_FEATURE_MD5_SHA1_SUM_CHECK(" or check") " MD5 checksums"
//usage:	IF_FEATURE_MD5_SHA1_SUM_CHECK( "\n"
//usage:     "\n	-c	Check sums against list in FILEs"
//usage:     "\n	-s	Don't output anything, status code shows success"
//usage:     "\n	-w	Warn about improperly formatted checksum lines"
//usage:	)
//usage:
//usage:#define md5sum_example_usage
//usage:       "$ md5sum < busybox\n"
//usage:       "6fd11e98b98a58f64ff3398d7b324003\n"
//usage:       "$ md5sum busybox\n"
//usage:       "6fd11e98b98a58f64ff3398d7b324003  busybox\n"
//usage:       "$ md5sum -c -\n"
//usage:       "6fd11e98b98a58f64ff3398d7b324003  busybox\n"
//usage:       "busybox: OK\n"
//usage:       "^D\n"
//usage:
//usage:#define sha1sum_trivial_usage
//usage:	IF_FEATURE_MD5_SHA1_SUM_CHECK("[-c[sw]] ")"[FILE]..."
//usage:#define sha1sum_full_usage "\n\n"
//usage:       "Print" IF_FEATURE_MD5_SHA1_SUM_CHECK(" or check") " SHA1 checksums"
//usage:	IF_FEATURE_MD5_SHA1_SUM_CHECK( "\n"
//usage:     "\n	-c	Check sums against list in FILEs"
//usage:     "\n	-s	Don't output anything, status code shows success"
//usage:     "\n	-w	Warn about improperly formatted checksum lines"
//usage:	)
//usage:
//usage:#define sha256sum_trivial_usage
//usage:	IF_FEATURE_MD5_SHA1_SUM_CHECK("[-c[sw]] ")"[FILE]..."
//usage:#define sha256sum_full_usage "\n\n"
//usage:       "Print" IF_FEATURE_MD5_SHA1_SUM_CHECK(" or check") " SHA256 checksums"
//usage:	IF_FEATURE_MD5_SHA1_SUM_CHECK( "\n"
//usage:     "\n	-c	Check sums against list in FILEs"
//usage:     "\n	-s	Don't output anything, status code shows success"
//usage:     "\n	-w	Warn about improperly formatted checksum lines"
//usage:	)
//usage:
//usage:#define sha512sum_trivial_usage
//usage:	IF_FEATURE_MD5_SHA1_SUM_CHECK("[-c[sw]] ")"[FILE]..."
//usage:#define sha512sum_full_usage "\n\n"
//usage:       "Print" IF_FEATURE_MD5_SHA1_SUM_CHECK(" or check") " SHA512 checksums"
//usage:	IF_FEATURE_MD5_SHA1_SUM_CHECK( "\n"
//usage:     "\n	-c	Check sums against list in FILEs"
//usage:     "\n	-s	Don't output anything, status code shows success"
//usage:     "\n	-w	Warn about improperly formatted checksum lines"
//usage:	)
//usage:
//usage:#define sha3sum_trivial_usage
//usage:	IF_FEATURE_MD5_SHA1_SUM_CHECK("[-c[sw]] ")"[-a BITS] [FILE]..."
//usage:#define sha3sum_full_usage "\n\n"
//usage:       "Print" IF_FEATURE_MD5_SHA1_SUM_CHECK(" or check") " SHA3 checksums"
//usage:	IF_FEATURE_MD5_SHA1_SUM_CHECK( "\n"
//usage:     "\n	-c	Check sums against list in FILEs"
//usage:     "\n	-s	Don't output anything, status code shows success"
//usage:     "\n	-w	Warn about improperly formatted checksum lines"
//usage:     "\n	-a BITS	224 (default), 256, 384, 512"
//usage:	)
//FIXME: GNU coreutils 8.25 has no -s option, it has only these two long opts:
// --quiet   don't print OK for each successfully verified file
// --status  don't output anything, status code shows success
/* This is a NOEXEC applet. Be very careful! */
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
) -> *mut uint8_t {
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
  let mut hash_value: *mut uint8_t = 0 as *mut uint8_t;
  let mut update: Option<
    unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_void, _: size_t) -> (),
  > = None;
  let mut final_0: Option<
    unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void) -> libc::c_uint,
  > = None;
  let mut hash_algo: libc::c_char = 0;
  src_fd = open_or_warn_stdin(filename);
  if src_fd < 0i32 {
    return 0 as *mut uint8_t;
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
  hash_value = 0 as *mut uint8_t;
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
        let mut hash_value: *mut uint8_t = 0 as *mut uint8_t;
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
      let mut hash_value_0: *mut uint8_t = hash_file(*argv, sha3_width);
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

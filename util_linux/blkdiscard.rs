use libc;

extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;

  #[no_mangle]
  static cwbkMG_suffixes: [suffix_mult; 0];

  #[no_mangle]
  fn xatoull_sfx(str: *const libc::c_char, sfx: *const suffix_mult) -> libc::c_ulonglong;

  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;

  #[no_mangle]
  fn ioctl_or_perror_and_die(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    fmt: *const libc::c_char,
    _: ...
  ) -> libc::c_int;

  #[no_mangle]
  fn bb_xioctl(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
}

pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct suffix_mult {
  pub suffix: [libc::c_char; 4],
  pub mult: libc::c_uint,
}

pub type C2RustUnnamed = libc::c_uint;
pub const OPT_SECURE: C2RustUnnamed = 4;
pub const OPT_LENGTH: C2RustUnnamed = 2;
// pub const OPT_OFFSET: C2RustUnnamed = 1;

/*
 * Mini blkdiscard implementation for busybox
 *
 * Copyright (C) 2015 by Ari Sundholm <ari@tuxera.com> and Tuxera Inc.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config BLKDISCARD
//config:	bool "blkdiscard (4.3 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	blkdiscard discards sectors on a given device.
//applet:IF_BLKDISCARD(APPLET_NOEXEC(blkdiscard, blkdiscard, BB_DIR_USR_BIN, BB_SUID_DROP, blkdiscard))
//kbuild:lib-$(CONFIG_BLKDISCARD) += blkdiscard.o
//usage:#define blkdiscard_trivial_usage
//usage:       "[-o OFS] [-l LEN] [-s] DEVICE"
//usage:#define blkdiscard_full_usage "\n\n"
//usage:	"Discard sectors on DEVICE\n"
//usage:	"\n	-o OFS	Byte offset into device"
//usage:	"\n	-l LEN	Number of bytes to discard"
//usage:	"\n	-s	Perform a secure discard"
//usage:
//usage:#define blkdiscard_example_usage
//usage:	"$ blkdiscard -o 0 -l 1G /dev/sdb"
#[no_mangle]
pub unsafe extern "C" fn blkdiscard_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opts: libc::c_uint = 0; /* Leaving these two variables out does not  */
  let mut offset_str: *const libc::c_char = b"0\x00" as *const u8 as *const libc::c_char; /* shrink code size and hampers readability. */
  let mut length_str: *const libc::c_char = 0 as *const libc::c_char;
  let mut offset: uint64_t = 0;
  let mut length: uint64_t = 0;
  let mut range: [uint64_t; 2] = [0; 2];
  let mut fd: libc::c_int = 0;
  opts = getopt32(
    argv,
    b"^o:l:s\x00=1\x00" as *const u8 as *const libc::c_char,
    &mut offset_str as *mut *const libc::c_char,
    &mut length_str as *mut *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  fd = xopen(*argv.offset(0), 0o2i32 | 0o200i32);
  //Why bother, BLK[SEC]DISCARD will fail on non-blockdevs anyway?
  //	xfstat(fd, &st);
  //	if (!S_ISBLK(st.st_mode))
  //		bb_error_msg_and_die("%s: not a block device", argv[0]);
  offset = xatoull_sfx(offset_str, cwbkMG_suffixes.as_ptr().offset(3)) as uint64_t;
  if opts & OPT_LENGTH as libc::c_int as libc::c_uint != 0 {
    length = xatoull_sfx(length_str, cwbkMG_suffixes.as_ptr().offset(3)) as uint64_t
  } else {
    bb_xioctl(
      fd,
      ((2u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (0x12i32 << 0i32 + 8i32) as libc::c_uint
        | (114i32 << 0i32) as libc::c_uint) as libc::c_ulong
        | (::std::mem::size_of::<size_t>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
        as libc::c_uint,
      &mut length as *mut uint64_t as *mut libc::c_void,
      b"BLKGETSIZE64\x00" as *const u8 as *const libc::c_char,
    );
    length = (length as libc::c_ulong).wrapping_sub(offset) as uint64_t as uint64_t
  }
  range[0] = offset;
  range[1] = length;
  ioctl_or_perror_and_die(
    fd,
    if opts & OPT_SECURE as libc::c_int as libc::c_uint != 0 {
      (0u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (0x12i32 << 0i32 + 8i32) as libc::c_uint
        | (125i32 << 0i32) as libc::c_uint)
        | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint
    } else {
      (0u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (0x12i32 << 0i32 + 8i32) as libc::c_uint
        | (119i32 << 0i32) as libc::c_uint)
        | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint
    },
    &mut range as *mut [uint64_t; 2] as *mut libc::c_void,
    b"%s: %s failed\x00" as *const u8 as *const libc::c_char,
    *argv.offset(0),
    if opts & OPT_SECURE as libc::c_int as libc::c_uint != 0 {
      b"BLKSECDISCARD\x00" as *const u8 as *const libc::c_char
    } else {
      b"BLKDISCARD\x00" as *const u8 as *const libc::c_char
    },
  );
  return 0i32;
}

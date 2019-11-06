use libc;
extern "C" {
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xatoi_positive(numstr: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
}

use crate::librb::size_t;

pub type C2RustUnnamed = libc::c_uint;
pub const FL_SCALE512: C2RustUnnamed = 16;
/* argument is provided by user */
pub const FL_NORESULT: C2RustUnnamed = 8;
pub const FL_USRARG: C2RustUnnamed = 4;
pub const ARG_MASK: C2RustUnnamed = 3;
/* Yes, BLKGETSIZE64 takes pointer to u64, not ullong! */
pub const ARG_U64: C2RustUnnamed = 3;
pub const ARG_ULONG: C2RustUnnamed = 2;
pub const ARG_INT: C2RustUnnamed = 1;
pub const ARG_NONE: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
  pub i: libc::c_int,
  pub lu: libc::c_ulong,
  pub u64_0: u64,
}
/*
 * blockdev implementation for busybox
 *
 * Copyright (C) 2010 Sergey Naumov <sknaumov@gmail.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config BLOCKDEV
//config:	bool "blockdev (2.3 kb)"
//config:	default y
//config:	help
//config:	Performs some ioctls with block devices.
//applet:IF_BLOCKDEV(APPLET_NOEXEC(blockdev, blockdev, BB_DIR_SBIN, BB_SUID_DROP, blockdev))
//kbuild:lib-$(CONFIG_BLOCKDEV) += blockdev.o
//usage:#define blockdev_trivial_usage
//usage:	"OPTION BLOCKDEV"
//usage:#define blockdev_full_usage "\n\n"
//usage:       "	--setro		Set ro"
//usage:     "\n	--setrw		Set rw"
//usage:     "\n	--getro		Get ro"
//usage:     "\n	--getss		Get sector size"
//usage:     "\n	--getbsz	Get block size"
//usage:     "\n	--setbsz BYTES	Set block size"
//usage:     "\n	--getsz		Get device size in 512-byte sectors"
/*//usage:     "\n	--getsize	Get device size in sectors (deprecated)"*/
//usage:     "\n	--getsize64	Get device size in bytes"
//usage:     "\n	--flushbufs	Flush buffers"
//usage:     "\n	--rereadpt	Reread partition table"
// util-linux 2.31 also has:
//	--getdiscardzeroes	BLKDISCARDZEROES	Get discard zeroes support status
//	--getpbsz		BLKPBSZGET	Get physical block (sector) size
//	--getiomin		BLKIOMIN	Get minimum I/O size
//	--getioopt		BLKIOOPT	Get optimal I/O size
//	--getalignoff		BLKALIGNOFF	Get alignment offset in bytes
//	--getmaxsect		BLKSECTGET	Get max sectors per request
//	--setra SECTORS		BLKRASET	Set readahead
//	--getra			BLKRAGET	Get readahead
//	--setfra SECTORS	BLKFRASET	Set filesystem readahead
//	--getfra		BLKFRAGET	Get filesystem readahead
/* Takes less space is separate arrays than one array of struct */
static mut bdcmd_names: [libc::c_char; 82] = [
  115, 101, 116, 114, 111, 0, 115, 101, 116, 114, 119, 0, 103, 101, 116, 114, 111, 0, 103, 101,
  116, 115, 115, 0, 103, 101, 116, 98, 115, 122, 0, 115, 101, 116, 98, 115, 122, 0, 103, 101, 116,
  115, 122, 0, 103, 101, 116, 115, 105, 122, 101, 0, 103, 101, 116, 115, 105, 122, 101, 54, 52, 0,
  102, 108, 117, 115, 104, 98, 117, 102, 115, 0, 114, 101, 114, 101, 97, 100, 112, 116, 0, 0,
];
static mut bdcmd_ioctl: [u32; 11] = [
  0u32 << 0i32 + 8i32 + 8i32 + 14i32
    | (0x12i32 << 0i32 + 8i32) as libc::c_uint
    | (93i32 << 0i32) as libc::c_uint
    | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint,
  0u32 << 0i32 + 8i32 + 8i32 + 14i32
    | (0x12i32 << 0i32 + 8i32) as libc::c_uint
    | (93i32 << 0i32) as libc::c_uint
    | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint,
  0u32 << 0i32 + 8i32 + 8i32 + 14i32
    | (0x12i32 << 0i32 + 8i32) as libc::c_uint
    | (94i32 << 0i32) as libc::c_uint
    | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint,
  0u32 << 0i32 + 8i32 + 8i32 + 14i32
    | (0x12i32 << 0i32 + 8i32) as libc::c_uint
    | (104i32 << 0i32) as libc::c_uint
    | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint,
  ((2u32 << 0i32 + 8i32 + 8i32 + 14i32
    | (0x12i32 << 0i32 + 8i32) as libc::c_uint
    | (112i32 << 0i32) as libc::c_uint) as libc::c_ulong
    | (::std::mem::size_of::<size_t>() as libc::c_ulong) << 0i32 + 8i32 + 8i32) as u32,
  ((1u32 << 0i32 + 8i32 + 8i32 + 14i32
    | (0x12i32 << 0i32 + 8i32) as libc::c_uint
    | (113i32 << 0i32) as libc::c_uint) as libc::c_ulong
    | (::std::mem::size_of::<size_t>() as libc::c_ulong) << 0i32 + 8i32 + 8i32) as u32,
  ((2u32 << 0i32 + 8i32 + 8i32 + 14i32
    | (0x12i32 << 0i32 + 8i32) as libc::c_uint
    | (114i32 << 0i32) as libc::c_uint) as libc::c_ulong
    | (::std::mem::size_of::<size_t>() as libc::c_ulong) << 0i32 + 8i32 + 8i32) as u32,
  0u32 << 0i32 + 8i32 + 8i32 + 14i32
    | (0x12i32 << 0i32 + 8i32) as libc::c_uint
    | (96i32 << 0i32) as libc::c_uint
    | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint,
  ((2u32 << 0i32 + 8i32 + 8i32 + 14i32
    | (0x12i32 << 0i32 + 8i32) as libc::c_uint
    | (114i32 << 0i32) as libc::c_uint) as libc::c_ulong
    | (::std::mem::size_of::<size_t>() as libc::c_ulong) << 0i32 + 8i32 + 8i32) as u32,
  0u32 << 0i32 + 8i32 + 8i32 + 14i32
    | (0x12i32 << 0i32 + 8i32) as libc::c_uint
    | (97i32 << 0i32) as libc::c_uint
    | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint,
  0u32 << 0i32 + 8i32 + 8i32 + 14i32
    | (0x12i32 << 0i32 + 8i32) as libc::c_uint
    | (95i32 << 0i32) as libc::c_uint
    | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint,
];
static mut bdcmd_flags: [u8; 11] = [
  (ARG_INT as libc::c_int + FL_NORESULT as libc::c_int) as u8,
  (ARG_INT as libc::c_int + FL_NORESULT as libc::c_int) as u8,
  ARG_INT as libc::c_int as u8,
  ARG_INT as libc::c_int as u8,
  ARG_INT as libc::c_int as u8,
  (ARG_INT as libc::c_int + FL_NORESULT as libc::c_int + FL_USRARG as libc::c_int) as u8,
  (ARG_U64 as libc::c_int + FL_SCALE512 as libc::c_int) as u8,
  ARG_ULONG as libc::c_int as u8,
  ARG_U64 as libc::c_int as u8,
  (ARG_NONE as libc::c_int + FL_NORESULT as libc::c_int) as u8,
  (ARG_NONE as libc::c_int + FL_NORESULT as libc::c_int) as u8,
];
unsafe extern "C" fn find_cmd(mut s: *const libc::c_char) -> libc::c_uint {
  if *s.offset(0) as libc::c_int == '-' as i32 && *s.offset(1) as libc::c_int == '-' as i32 {
    let mut n: libc::c_int = index_in_strings(bdcmd_names.as_ptr(), s.offset(2));
    if n >= 0i32 {
      return n as libc::c_uint;
    }
  }
  bb_show_usage();
}
#[no_mangle]
pub unsafe extern "C" fn blockdev_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut bdcmd: libc::c_uint = 0;
  let mut flags: libc::c_uint = 0;
  let mut fd: libc::c_int = 0;
  let mut u64: u64 = 0;
  let mut ioctl_val_on_stack: C2RustUnnamed_0 = C2RustUnnamed_0 { i: 0 };
  argv = argv.offset(1);
  if (*argv.offset(0)).is_null() || (*argv.offset(1)).is_null() {
    /* must have at least 2 args */
    bb_show_usage();
  }
  bdcmd = find_cmd(*argv);
  /* setrw translates to BLKROSET(0), most other ioctls don't care... */
  /* ...setro will do BLKROSET(1) */
  u64 = (bdcmd == 0i32 as libc::c_uint) as libc::c_int as u64;
  if bdcmd == 5i32 as libc::c_uint {
    /* ...setbsz is BLKBSZSET(bytes) */
    argv = argv.offset(1);
    u64 = xatoi_positive(*argv) as u64
  }
  argv = argv.offset(1);
  if (*argv.offset(0)).is_null() || !(*argv.offset(1)).is_null() {
    bb_show_usage();
  }
  fd = xopen(*argv.offset(0), 0i32);
  ioctl_val_on_stack.u64_0 = u64;
  flags = bdcmd_flags[bdcmd as usize] as libc::c_uint;
  if ioctl(
    fd,
    bdcmd_ioctl[bdcmd as usize] as libc::c_ulong,
    &mut ioctl_val_on_stack.u64_0 as *mut u64,
  ) == -1i32
  {
    bb_simple_perror_msg_and_die(*argv);
  }
  /* Fetch it into register(s) */
  u64 = ioctl_val_on_stack.u64_0;
  if flags & FL_SCALE512 as libc::c_int as libc::c_uint != 0 {
    u64 >>= 9i32
  }
  let mut current_block_21: u64;
  /* Zero- or one-extend the value if needed, then print */
  match flags & (ARG_MASK as libc::c_int + FL_NORESULT as libc::c_int) as libc::c_uint {
    1 => {
      /* Smaller code when we use long long
       * (gcc tail-merges printf call)
       */
      printf(
        b"%lld\n\x00" as *const u8 as *const libc::c_char,
        u64 as libc::c_int as libc::c_longlong,
      );
      current_block_21 = 10043043949733653460;
    }
    2 => {
      u64 = u64;
      current_block_21 = 388174793330894917;
    }
    3 => {
      current_block_21 = 388174793330894917;
    }
    _ => {
      current_block_21 = 10043043949733653460;
    }
  }
  match current_block_21 {
    388174793330894917 =>
    /* FALLTHROUGH */
    {
      printf(
        b"%llu\n\x00" as *const u8 as *const libc::c_char,
        u64 as libc::c_ulonglong,
      );
    }
    _ => {}
  }
  return 0i32;
}

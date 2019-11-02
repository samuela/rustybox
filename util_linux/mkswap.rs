use libc;
extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xlseek(fd: libc::c_int, offset: off_t, whence: libc::c_int) -> off_t;
  #[no_mangle]
  fn get_volume_size_in_bytes(
    fd: libc::c_int,
    override_0: *const libc::c_char,
    override_units: libc::c_uint,
    extend: libc::c_int,
  ) -> uoff_t;
  #[no_mangle]
  fn safe_strncpy(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    size: size_t,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn xwrite(fd: libc::c_int, buf: *const libc::c_void, count: size_t);
  #[no_mangle]
  fn bin2hex(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    count: libc::c_int,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn generate_uuid(buf: *mut uint8_t);
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn fsync(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn getpagesize() -> libc::c_int;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __off64_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type off_t = __off64_t;
pub type uoff_t = libc::c_ulong;
/* vi: set sw=4 ts=4: */
/*
 * mkswap.c - format swap device (Linux v1 only)
 *
 * Copyright 2006 Rob Landley <rob@landley.net>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config MKSWAP
//config:	bool "mkswap (6.3 kb)"
//config:	default y
//config:	help
//config:	The mkswap utility is used to configure a file or disk partition as
//config:	Linux swap space. This allows Linux to use the entire file or
//config:	partition as if it were additional RAM, which can greatly increase
//config:	the capability of low-memory machines. This additional memory is
//config:	much slower than real RAM, but can be very helpful at preventing your
//config:	applications being killed by the Linux out of memory (OOM) killer.
//config:	Once you have created swap space using 'mkswap' you need to enable
//config:	the swap space using the 'swapon' utility.
//config:
//config:config FEATURE_MKSWAP_UUID
//config:	bool "UUID support"
//config:	default y
//config:	depends on MKSWAP
//config:	help
//config:	Generate swap spaces with universally unique identifiers.
//applet:IF_MKSWAP(APPLET(mkswap, BB_DIR_SBIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_MKSWAP) += mkswap.o
//usage:#define mkswap_trivial_usage
//usage:       "[-L LBL] BLOCKDEV [KBYTES]"
//usage:#define mkswap_full_usage "\n\n"
//usage:       "Prepare BLOCKDEV to be used as swap partition\n"
//usage:     "\n	-L LBL	Label"
/* from Linux 2.6.23 */
/*
 * Magic header for a swap area. ... Note that the first
 * kilobyte is reserved for boot loader or disk label stuff.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swap_header_v1 {
  pub version: uint32_t,
  pub last_page: uint32_t,
  pub nr_badpages: uint32_t,
  pub sws_uuid: [libc::c_char; 16],
  pub sws_volume: [libc::c_char; 16],
  pub padding: [uint32_t; 117],
  pub badpages: [uint32_t; 1],
  /* 128 */
  /* total 129 32-bit words in 2nd kilobyte */
}
/* Stored without terminating NUL */
static mut SWAPSPACE2: [libc::c_char; 10] = [83, 87, 65, 80, 83, 80, 65, 67, 69, 50];
#[no_mangle]
pub unsafe extern "C" fn mkswap_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut fd: libc::c_int = 0;
  let mut pagesize: libc::c_uint = 0;
  let mut len: off_t = 0;
  let mut label: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
  /* TODO: -p PAGESZ, -U UUID */
  getopt32(
    argv,
    b"^L:\x00-1\x00" as *const u8 as *const libc::c_char,
    &mut label as *mut *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  fd = xopen(*argv.offset(0), 0o1i32);
  /* Figure out how big the device is */
  len = get_volume_size_in_bytes(fd, *argv.offset(1), 1024i32 as libc::c_uint, 1i32) as off_t;
  pagesize = getpagesize() as libc::c_uint;
  len -= pagesize as libc::c_long;
  /* Announce our intentions */
  printf(
    b"Setting up swapspace version 1, size = %lu bytes\n\x00" as *const u8 as *const libc::c_char,
    len,
  );
  /* hdr is zero-filled so far. Clear the first kbyte, or else
   * mkswap-ing former FAT partition does NOT erase its signature.
   *
   * util-linux-ng 2.17.2 claims to erase it only if it does not see
   * a partition table and is not run on whole disk. -f forces it.
   */
  xwrite(
    fd,
    bb_common_bufsiz1.as_mut_ptr() as *mut swap_header_v1 as *const libc::c_void,
    1024i32 as size_t,
  );
  /* Fill the header. */
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut swap_header_v1)).version = 1i32 as uint32_t;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut swap_header_v1)).last_page =
    (len as uoff_t).wrapping_div(pagesize as libc::c_ulong) as uint32_t;
  let mut uuid_string: [libc::c_char; 32] = [0; 32];
  generate_uuid(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut swap_header_v1))
      .sws_uuid
      .as_mut_ptr() as *mut libc::c_void as *mut uint8_t,
  );
  bin2hex(
    uuid_string.as_mut_ptr(),
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut swap_header_v1))
      .sws_uuid
      .as_mut_ptr(),
    16i32,
  );
  /* f.e. UUID=dfd9c173-be52-4d27-99a5-c34c6c2ff55f */
  printf(
    b"UUID=%.8s-%.4s-%.4s-%.4s-%.12s\n\x00" as *const u8 as *const libc::c_char,
    uuid_string.as_mut_ptr(),
    uuid_string.as_mut_ptr().offset(8),
    uuid_string.as_mut_ptr().offset(8).offset(4),
    uuid_string.as_mut_ptr().offset(8).offset(4).offset(4),
    uuid_string
      .as_mut_ptr()
      .offset(8)
      .offset(4)
      .offset(4)
      .offset(4),
  );
  safe_strncpy(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut swap_header_v1))
      .sws_volume
      .as_mut_ptr(),
    label,
    16i32 as size_t,
  );
  /* Write the header.  Sync to disk because some kernel versions check
   * signature on disk (not in cache) during swapon. */
  xwrite(
    fd,
    bb_common_bufsiz1.as_mut_ptr() as *mut swap_header_v1 as *const libc::c_void,
    (129i32 * 4i32) as size_t,
  );
  xlseek(
    fd,
    pagesize.wrapping_sub(10i32 as libc::c_uint) as off_t,
    0i32,
  );
  xwrite(
    fd,
    SWAPSPACE2.as_ptr() as *const libc::c_void,
    10i32 as size_t,
  );
  fsync(fd);
  return 0i32;
}

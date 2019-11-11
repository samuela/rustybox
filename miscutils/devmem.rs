use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::appletlib::applet_name;
use libc;
use libc::printf;
extern "C" {
  #[no_mangle]
  fn getpagesize() -> libc::c_int;

  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn mmap(
    __addr: *mut libc::c_void,
    __len: size_t,
    __prot: libc::c_int,
    __flags: libc::c_int,
    __fd: libc::c_int,
    __offset: off64_t,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;

  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xatou(str: *const libc::c_char) -> libc::c_uint;
  /* Non-aborting kind of convertors: bb_strto[u][l]l */
  /* On exit: errno = 0 only if there was non-empty, '\0' terminated value
   * errno = EINVAL if value was not '\0' terminated, but otherwise ok
   *    Return value is still valid, caller should just check whether end[0]
   *    is a valid terminating char for particular case. OTOH, if caller
   *    requires '\0' terminated input, [s]he can just check errno == 0.
   * errno = ERANGE if value had alphanumeric terminating char ("1234abcg").
   * errno = ERANGE if value is out of range, missing, etc.
   * errno = ERANGE if value had minus sign for strtouXX (even "-0" is not ok )
   *    return value is all-ones in this case.
   */
  #[no_mangle]
  fn bb_strtoull(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_ulonglong;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
}

use crate::librb::size_t;
use libc::off64_t;
use libc::off_t;
// Initialized in run_static_initializers
static mut sizes: [u8; 5] = [0; 5];
/*
 * Copyright (C) 2000, Jan-Derk Bakker (J.D.Bakker@its.tudelft.nl)
 * Copyright (C) 2008, BusyBox Team. -solar 4/26/08
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config DEVMEM
//config:	bool "devmem (2.5 kb)"
//config:	default y
//config:	help
//config:	devmem is a small program that reads and writes from physical
//config:	memory using /dev/mem.
//applet:IF_DEVMEM(APPLET(devmem, BB_DIR_SBIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_DEVMEM) += devmem.o
//usage:#define devmem_trivial_usage
//usage:	"ADDRESS [WIDTH [VALUE]]"
//usage:#define devmem_full_usage "\n\n"
//usage:       "Read/write from physical address\n"
//usage:     "\n	ADDRESS	Address to act upon"
//usage:     "\n	WIDTH	Width (8/16/...)"
//usage:     "\n	VALUE	Data to be written"
#[no_mangle]
pub unsafe extern "C" fn devmem_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut map_base: *mut libc::c_void = 0 as *mut libc::c_void; /* for compiler */
  let mut virt_addr: *mut libc::c_void = 0 as *mut libc::c_void;
  let mut read_result: u64 = 0;
  let mut writeval: u64 = 0;
  writeval = writeval;
  let mut target: off_t = 0;
  let mut page_size: libc::c_uint = 0;
  let mut mapped_size: libc::c_uint = 0;
  let mut offset_in_page: libc::c_uint = 0;
  let mut fd: libc::c_int = 0;
  let mut width: libc::c_uint = (8i32 as libc::c_ulong)
    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
    as libc::c_uint;
  /* devmem ADDRESS [WIDTH [VALUE]] */
  // TODO: options?
  // -r: read and output only the value in hex, with 0x prefix
  // -w: write only, no reads before or after, and no output
  // or make this behavior default?
  // Let's try this and see how users react.
  /* ADDRESS */
  if (*argv.offset(1)).is_null() {
    bb_show_usage(); /* allows hex, oct etc */
  }
  *bb_errno = 0i32;
  target = bb_strtoull(*argv.offset(1), 0 as *mut *mut libc::c_char, 0i32) as off_t;
  /* WIDTH */
  if !(*argv.offset(2)).is_null() {
    if (*(*argv.offset(2)).offset(0) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int
      <= 9i32
      || *(*argv.offset(2)).offset(1) as libc::c_int != 0
    {
      width = xatou(*argv.offset(2))
    } else {
      static mut bhwl: [libc::c_char; 5] = [98, 104, 119, 108, 0]; /* argv[2] == NULL */
      width = strchrnul(
        bhwl.as_ptr(),
        *(*argv.offset(2)).offset(0) as libc::c_int | 0x20i32,
      )
      .wrapping_offset_from(bhwl.as_ptr()) as libc::c_long as libc::c_uint;
      width = sizes[width as usize] as libc::c_uint
    }
    /* VALUE */
    if !(*argv.offset(3)).is_null() {
      writeval = bb_strtoull(*argv.offset(3), 0 as *mut *mut libc::c_char, 0i32) as u64
    }
  } else {
    /* make argv[3] to be a valid thing to fetch */
    argv = argv.offset(-1)
  } /* one of bb_strtouXX failed */
  if *bb_errno != 0 {
    bb_show_usage();
  }
  fd = xopen(
    b"/dev/mem\x00" as *const u8 as *const libc::c_char,
    if !(*argv.offset(3)).is_null() {
      (0o2i32) | 0o4010000i32
    } else {
      (0i32) | 0o4010000i32
    },
  );
  page_size = getpagesize() as libc::c_uint;
  mapped_size = page_size;
  offset_in_page = target as libc::c_uint & page_size.wrapping_sub(1i32 as libc::c_uint);
  if offset_in_page.wrapping_add(width) > page_size {
    /* This access spans pages.
     * Must map two pages to make it possible: */
    mapped_size = mapped_size.wrapping_mul(2i32 as libc::c_uint)
  }
  map_base = mmap(
    0 as *mut libc::c_void,
    mapped_size as size_t,
    if !(*argv.offset(3)).is_null() {
      (0x1i32) | 0x2i32
    } else {
      0x1i32
    },
    0x1i32,
    fd,
    target & !(page_size.wrapping_sub(1i32 as libc::c_uint) as off_t),
  );
  if map_base == -1i32 as *mut libc::c_void {
    bb_simple_perror_msg_and_die(b"mmap\x00" as *const u8 as *const libc::c_char);
  }
  //	printf("Memory mapped at address %p.\n", map_base);
  virt_addr = (map_base as *mut libc::c_char).offset(offset_in_page as isize) as *mut libc::c_void;
  if (*argv.offset(3)).is_null() {
    match width {
      8 => read_result = *(virt_addr as *mut u8) as u64,
      16 => read_result = *(virt_addr as *mut u16) as u64,
      32 => read_result = *(virt_addr as *mut u32) as u64,
      64 => read_result = *(virt_addr as *mut u64),
      _ => {
        bb_simple_error_msg_and_die(b"bad width\x00" as *const u8 as *const libc::c_char);
      }
    }
    //		printf("Value at address 0x%"OFF_FMT"X (%p): 0x%llX\n",
    //			target, virt_addr,
    //			(unsigned long long)read_result);
    /* Zero-padded output shows the width of access just done */
    printf(
      b"0x%0*llX\n\x00" as *const u8 as *const libc::c_char,
      width >> 2i32,
      read_result as libc::c_ulonglong,
    );
  } else {
    match width {
      8 => ::std::ptr::write_volatile(virt_addr as *mut u8, writeval as u8),
      16 => ::std::ptr::write_volatile(virt_addr as *mut u16, writeval as u16),
      32 => ::std::ptr::write_volatile(virt_addr as *mut u32, writeval as u32),
      64 => ::std::ptr::write_volatile(virt_addr as *mut u64, writeval),
      _ => {
        bb_simple_error_msg_and_die(b"bad width\x00" as *const u8 as *const libc::c_char);
      }
    }
    //		printf("Written 0x%llX; readback 0x%llX\n",
    //				(unsigned long long)writeval,
    //				(unsigned long long)read_result);
  }
  return 0i32;
}
unsafe extern "C" fn run_static_initializers() {
  sizes = [
    (8i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
      as u8,
    (8i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_short>() as libc::c_ulong)
      as u8,
    (8i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
      as u8,
    (8i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
      as u8,
    0i32 as u8,
  ]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

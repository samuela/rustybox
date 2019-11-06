use libc;
extern "C" {
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_xioctl(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
}

use libc::uint32_t;

/*
 * Display or change file attributes on a fat file system
 *
 * Copyright 2005 H. Peter Anvin
 * Busybox'ed (2014) by Pascal Bellard <pascal.bellard@ads-lu.com>
 *
 * This file can be redistributed under the terms of the GNU General
 * Public License
 */
//config:config FATATTR
//config:	bool "fatattr (1.9 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	fatattr lists or changes the file attributes on a fat file system.
//applet:IF_FATATTR(APPLET_NOEXEC(fatattr, fatattr, BB_DIR_BIN, BB_SUID_DROP, fatattr))
//kbuild:lib-$(CONFIG_FATATTR) += fatattr.o
//usage:#define fatattr_trivial_usage
//usage:       "[-+rhsvda] FILE..."
//usage:#define fatattr_full_usage "\n\n"
//usage:       "Change file attributes on FAT filesystem\n"
//usage:     "\n	-	Clear attributes"
//usage:     "\n	+	Set attributes"
//usage:     "\n	r	Read only"
//usage:     "\n	h	Hidden"
//usage:     "\n	s	System"
//usage:     "\n	v	Volume label"
//usage:     "\n	d	Directory"
//usage:     "\n	a	Archive"
/* linux/msdos_fs.h says: */
/* Currently supports only the FAT flags, not the NTFS ones.
 * Extra space at the end is a hack to print space separator in file listing.
 * Let's hope no one ever passes space as an option char :)
 */
static mut bit_to_char: [libc::c_char; 10] = [114, 104, 115, 118, 100, 97, 54, 55, 32, 0];
#[inline]
unsafe extern "C" fn get_flag(mut c: libc::c_char) -> libc::c_ulong {
  let mut fp: *const libc::c_char = strchr(bit_to_char.as_ptr(), c as libc::c_int);
  if fp.is_null() {
    bb_error_msg_and_die(
      b"invalid character \'%c\'\x00" as *const u8 as *const libc::c_char,
      c as libc::c_int,
    );
  }
  return (1i32 << fp.wrapping_offset_from(bit_to_char.as_ptr()) as libc::c_long) as libc::c_ulong;
}
unsafe extern "C" fn decode_arg(mut arg: *const libc::c_char) -> libc::c_uint {
  let mut fl: libc::c_uint = 0i32 as libc::c_uint;
  loop {
    arg = arg.offset(1);
    if !(*arg != 0) {
      break;
    }
    fl = (fl as libc::c_ulong | get_flag(*arg)) as libc::c_uint
  }
  return fl;
}
#[no_mangle]
pub unsafe extern "C" fn fatattr_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut set_mask: libc::c_uint = 0i32 as libc::c_uint;
  let mut clear_mask: libc::c_uint = 0i32 as libc::c_uint;
  loop {
    let mut fl: libc::c_uint = 0;
    argv = argv.offset(1);
    let mut arg: *mut libc::c_char = *argv;
    if arg.is_null() {
      bb_show_usage();
    }
    if *arg.offset(0) as libc::c_int != '-' as i32 && *arg.offset(0) as libc::c_int != '+' as i32 {
      break;
    }
    fl = decode_arg(arg);
    if *arg.offset(0) as libc::c_int == '+' as i32 {
      set_mask |= fl
    } else {
      clear_mask |= fl
    }
  }
  loop {
    let mut fd: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut attr: uint32_t = 0;
    fd = xopen(*argv, 0i32);
    bb_xioctl(
      fd,
      ((2u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (('r' as i32) << 0i32 + 8i32) as libc::c_uint
        | (0x10i32 << 0i32) as libc::c_uint) as libc::c_ulong
        | (::std::mem::size_of::<uint32_t>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
        as libc::c_uint,
      &mut attr as *mut uint32_t as *mut libc::c_void,
      b"FAT_IOCTL_GET_ATTRIBUTES\x00" as *const u8 as *const libc::c_char,
    );
    attr = (attr | set_mask) & !clear_mask;
    if set_mask | clear_mask != 0 {
      bb_xioctl(
        fd,
        ((1u32 << 0i32 + 8i32 + 8i32 + 14i32
          | (('r' as i32) << 0i32 + 8i32) as libc::c_uint
          | (0x11i32 << 0i32) as libc::c_uint) as libc::c_ulong
          | (::std::mem::size_of::<uint32_t>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
          as libc::c_uint,
        &mut attr as *mut uint32_t as *mut libc::c_void,
        b"FAT_IOCTL_SET_ATTRIBUTES\x00" as *const u8 as *const libc::c_char,
      );
    } else {
      i = 0i32;
      while bit_to_char[i as usize] != 0 {
        bb_putchar(if attr & 1i32 as libc::c_uint != 0 {
          bit_to_char[i as usize] as libc::c_int
        } else {
          ' ' as i32
        });
        attr >>= 1i32;
        i += 1
      }
      puts(*argv);
    }
    close(fd);
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  return 0i32;
}

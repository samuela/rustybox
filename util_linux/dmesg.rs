use crate::libbb::xfuncs_printf::xmalloc;
use crate::librb::size_t;
use libc;
use libc::putchar_unlocked;
extern "C" {
  #[no_mangle]
  fn klogctl(__type: libc::c_int, __bufp: *mut libc::c_char, __len: libc::c_int) -> libc::c_int;

}

pub const OPT_r: C2RustUnnamed = 8;
pub const OPT_c: C2RustUnnamed = 1;
pub const OPT_s: C2RustUnnamed = 2;
pub const OPT_n: C2RustUnnamed = 4;
pub type C2RustUnnamed = libc::c_uint;

/*
 *
 * dmesg - display/control kernel ring buffer.
 *
 * Copyright 2006 Rob Landley <rob@landley.net>
 * Copyright 2006 Bernhard Reutner-Fischer <rep.nop@aon.at>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config DMESG
//config:	bool "dmesg (3.7 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	dmesg is used to examine or control the kernel ring buffer. When the
//config:	Linux kernel prints messages to the system log, they are stored in
//config:	the kernel ring buffer. You can use dmesg to print the kernel's ring
//config:	buffer, clear the kernel ring buffer, change the size of the kernel
//config:	ring buffer, and change the priority level at which kernel messages
//config:	are also logged to the system console. Enable this option if you
//config:	wish to enable the 'dmesg' utility.
//config:
//config:config FEATURE_DMESG_PRETTY
//config:	bool "Pretty output"
//config:	default y
//config:	depends on DMESG
//config:	help
//config:	If you wish to scrub the syslog level from the output, say 'Y' here.
//config:	The syslog level is a string prefixed to every line with the form
//config:	"<#>".
//config:
//config:	With this option you will see:
//config:		# dmesg
//config:		Linux version 2.6.17.4 .....
//config:		BIOS-provided physical RAM map:
//config:		 BIOS-e820: 0000000000000000 - 000000000009f000 (usable)
//config:
//config:	Without this option you will see:
//config:		# dmesg
//config:		<5>Linux version 2.6.17.4 .....
//config:		<6>BIOS-provided physical RAM map:
//config:		<6> BIOS-e820: 0000000000000000 - 000000000009f000 (usable)
//applet:IF_DMESG(APPLET(dmesg, BB_DIR_BIN, SUID_DROP))
//kbuild:lib-$(CONFIG_DMESG) += dmesg.o
//usage:#define dmesg_trivial_usage
//usage:       "[-c] [-n LEVEL] [-s SIZE]"
//usage:#define dmesg_full_usage "\n\n"
//usage:       "Print or control the kernel ring buffer\n"
//usage:     "\n	-c		Clear ring buffer after printing"
//usage:     "\n	-n LEVEL	Set console logging level"
//usage:     "\n	-s SIZE		Buffer size"
//usage:     "\n	-r		Print raw message buffer"
#[no_mangle]
pub unsafe extern "C" fn dmesg_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut len: libc::c_int = 0; /* read ring buffer size */
  let mut level: libc::c_int = 0; /* read ring buffer */
  let mut buf: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut opts: libc::c_uint = 0;
  opts = crate::libbb::getopt32::getopt32(
    argv,
    b"cs:+n:+r\x00" as *const u8 as *const libc::c_char,
    &mut len as *mut libc::c_int,
    &mut level as *mut libc::c_int,
  );
  if opts & OPT_n as libc::c_int as libc::c_uint != 0 {
    if klogctl(
      8i32,
      std::ptr::null_mut::<libc::c_char>(),
      level as libc::c_long as libc::c_int,
    ) != 0
    {
      crate::libbb::perror_msg::bb_simple_perror_msg_and_die(
        b"klogctl\x00" as *const u8 as *const libc::c_char,
      );
    }
    return 0i32;
  }
  if opts & OPT_s as libc::c_int as libc::c_uint == 0 {
    len = klogctl(10i32, std::ptr::null_mut::<libc::c_char>(), 0i32)
  }
  if len < 16i32 * 1024i32 {
    len = 16i32 * 1024i32
  }
  if len > 16i32 * 1024i32 * 1024i32 {
    len = 16i32 * 1024i32 * 1024i32
  }
  buf = xmalloc(len as size_t) as *mut libc::c_char;
  len = klogctl(
    (3i32 as libc::c_uint).wrapping_add(opts & OPT_c as libc::c_int as libc::c_uint) as libc::c_int,
    buf,
    len,
  );
  if len < 0i32 {
    crate::libbb::perror_msg::bb_simple_perror_msg_and_die(
      b"klogctl\x00" as *const u8 as *const libc::c_char,
    );
  }
  if len == 0i32 {
    return 0i32;
  }
  if 1i32 != 0 && opts & OPT_r as libc::c_int as libc::c_uint == 0 {
    let mut last: libc::c_int = '\n' as i32;
    let mut in_0: libc::c_int = 0i32;
    loop
    /* Skip <[0-9]+> at the start of lines */
    {
      if last == '\n' as i32 && *buf.offset(in_0 as isize) as libc::c_int == '<' as i32 {
        loop {
          let fresh0 = in_0;
          in_0 = in_0 + 1;
          if !(*buf.offset(fresh0 as isize) as libc::c_int != '>' as i32 && in_0 < len) {
            break;
          }
        }
      } else {
        let fresh1 = in_0;
        in_0 = in_0 + 1;
        last = *buf.offset(fresh1 as isize) as libc::c_int;
        putchar_unlocked(last);
      }
      if in_0 >= len {
        break;
      }
    }
    /* Make sure we end with a newline */
    if last != '\n' as i32 {
      crate::libbb::xfuncs_printf::bb_putchar('\n' as i32);
    }
  } else {
    crate::libbb::full_write::full_write(1i32, buf as *const libc::c_void, len as size_t);
    if *buf.offset((len - 1i32) as isize) as libc::c_int != '\n' as i32 {
      crate::libbb::xfuncs_printf::bb_putchar('\n' as i32);
    }
  }
  return 0i32;
}

use libc;
use libc::close;
use libc::dup2;
use libc::ioctl;
use libc::open;
use libc::puts;
use libc::sprintf;
use libc::strcpy;
use libc::strrchr;
extern "C" {

  #[no_mangle]
  fn open_or_warn(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn overlapping_strcpy(dst: *mut libc::c_char, src: *const libc::c_char);
  #[no_mangle]
  fn open_read_close(
    filename: *const libc::c_char,
    buf: *mut libc::c_void,
    maxsz: size_t,
  ) -> ssize_t;
  #[no_mangle]
  fn BB_EXECVP_or_die(argv: *mut *mut libc::c_char) -> !;
}

use crate::librb::size_t;
use libc::ssize_t;
/*
 * Copyright (c) 2007 Denys Vlasenko <vda.linux@googlemail.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config CTTYHACK
//config:	bool "cttyhack (2.4 kb)"
//config:	default y
//config:	help
//config:	One common problem reported on the mailing list is the "can't
//config:	access tty; job control turned off" error message, which typically
//config:	appears when one tries to use a shell with stdin/stdout on
//config:	/dev/console.
//config:	This device is special - it cannot be a controlling tty.
//config:
//config:	The proper solution is to use the correct device instead of
//config:	/dev/console.
//config:
//config:	cttyhack provides a "quick and dirty" solution to this problem.
//config:	It analyzes stdin with various ioctls, trying to determine whether
//config:	it is a /dev/ttyN or /dev/ttySN (virtual terminal or serial line).
//config:	On Linux it also checks sysfs for a pointer to the active console.
//config:	If cttyhack is able to find the real console device, it closes
//config:	stdin/out/err and reopens that device.
//config:	Then it executes the given program. Opening the device will make
//config:	that device a controlling tty. This may require cttyhack
//config:	to be a session leader.
//config:
//config:	Example for /etc/inittab (for busybox init):
//config:
//config:	::respawn:/bin/cttyhack /bin/sh
//config:
//config:	Starting an interactive shell from boot shell script:
//config:
//config:	setsid cttyhack sh
//config:
//config:	Giving controlling tty to shell running with PID 1:
//config:
//config:	# exec cttyhack sh
//config:
//config:	Without cttyhack, you need to know exact tty name,
//config:	and do something like this:
//config:
//config:	# exec setsid sh -c 'exec sh </dev/tty1 >/dev/tty1 2>&1'
//config:
//config:	Starting getty on a controlling tty from a shell script:
//config:
//config:	# getty 115200 $(cttyhack)
//applet:IF_CTTYHACK(APPLET_NOEXEC(cttyhack, cttyhack, BB_DIR_BIN, SUID_DROP, cttyhack))
//kbuild:lib-$(CONFIG_CTTYHACK) += cttyhack.o
//usage:#define cttyhack_trivial_usage
//usage:       "[PROG ARGS]"
//usage:#define cttyhack_full_usage "\n\n"
//usage:       "Give PROG a controlling tty if possible."
//usage:     "\nExample for /etc/inittab (for busybox init):"
//usage:     "\n	::respawn:/bin/cttyhack /bin/sh"
//usage:     "\nGiving controlling tty to shell running with PID 1:"
//usage:     "\n	$ exec cttyhack sh"
//usage:     "\nStarting interactive shell from boot shell script:"
//usage:     "\n	setsid cttyhack sh"
/* From <linux/vt.h> */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vt_stat {
  pub v_active: libc::c_ushort,
  pub v_signal: libc::c_ushort,
  pub v_state: libc::c_ushort,
  /* vt bitmask */
}
pub type C2RustUnnamed = libc::c_uint;
pub const VT_GETSTATE: C2RustUnnamed = 22019;
/* get global vt state info */
/* From <linux/serial.h> */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct serial_struct {
  pub type_0: libc::c_int,
  pub line: libc::c_int,
  pub port: libc::c_uint,
  pub irq: libc::c_int,
  pub flags: libc::c_int,
  pub xmit_fifo_size: libc::c_int,
  pub custom_divisor: libc::c_int,
  pub baud_base: libc::c_int,
  pub close_delay: libc::c_ushort,
  pub io_type: libc::c_char,
  pub reserved_char: [libc::c_char; 1],
  pub hub6: libc::c_int,
  pub closing_wait: libc::c_ushort,
  pub closing_wait2: libc::c_ushort,
  pub iomem_base: *mut libc::c_uchar,
  pub iomem_reg_shift: libc::c_ushort,
  pub port_high: libc::c_uint,
  pub iomap_base: libc::c_ulong,
  pub reserved: [libc::c_int; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
  pub vt: vt_stat,
  pub sr: serial_struct,
  pub paranoia: [libc::c_char; 240],
}
#[no_mangle]
pub unsafe extern "C" fn cttyhack_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut fd: libc::c_int = 0;
  let mut console: [libc::c_char; 28] = [0; 28];
  let mut u: C2RustUnnamed_0 = C2RustUnnamed_0 {
    vt: vt_stat {
      v_active: 0,
      v_signal: 0,
      v_state: 0,
    },
  };
  strcpy(
    console.as_mut_ptr(),
    b"/dev/tty\x00" as *const u8 as *const libc::c_char,
  );
  fd = open(console.as_mut_ptr(), 0o2i32);
  if fd < 0i32 {
    /* We don't have ctty (or don't have "/dev/tty" node...) */
    /* Note that this method does not use _stdin_.
     * Thus, "cttyhack </dev/something" can't be used.
     * However, this method is more reliable than
     * TIOCGSERIAL check, which assumes that all
     * serial lines follow /dev/ttySn convention -
     * which is not always the case.
     * Therefore, we use this method first:
     */
    let mut s: libc::c_int = open_read_close(
      b"/sys/class/tty/console/active\x00" as *const u8 as *const libc::c_char,
      console.as_mut_ptr().offset(5) as *mut libc::c_void,
      (::std::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong)
        .wrapping_sub(5i32 as libc::c_ulong),
    ) as libc::c_int;
    if s > 0i32 {
      let mut last: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
      /* Found active console via sysfs (Linux 2.6.38+).
       * It looks like "[tty0 ]ttyS0\n" so zap the newline:
       */
      console[(4i32 + s) as usize] = '\u{0}' as i32 as libc::c_char;
      /* If there are multiple consoles,
       * take the last one:
       */
      last = strrchr(console.as_mut_ptr().offset(5), ' ' as i32);
      if !last.is_null() {
        overlapping_strcpy(console.as_mut_ptr().offset(5), last.offset(1));
      }
    } else if ioctl(
      0i32,
      VT_GETSTATE as libc::c_int as libc::c_ulong,
      &mut u.vt as *mut vt_stat,
    ) == 0i32
    {
      /* this is linux virtual tty */
      sprintf(
        console.as_mut_ptr().offset(8),
        (b"S%u\x00" as *const u8 as *const libc::c_char).offset(1),
        u.vt.v_active as libc::c_int,
      );
    } else if ioctl(
      0i32,
      0x541ei32 as libc::c_ulong,
      &mut u.sr as *mut serial_struct,
    ) == 0i32
    {
      /* this is a serial console; assuming it is named /dev/ttySn */
      sprintf(
        console.as_mut_ptr().offset(8),
        b"S%u\x00" as *const u8 as *const libc::c_char,
        u.sr.line,
      );
    } else {
      /* nope, could not find it */
      console[0] = '\u{0}' as i32 as libc::c_char
    }
  }
  argv = argv.offset(1);
  if (*argv.offset(0)).is_null() {
    if console[0] == 0 {
      return 1i32;
    }
    puts(console.as_mut_ptr());
    return 0i32;
  }
  if fd < 0i32 {
    fd = open_or_warn(console.as_mut_ptr(), 0o2i32);
    if fd < 0i32 {
      current_block = 5769750084105306570;
    } else {
      current_block = 14359455889292382949;
    }
  } else {
    current_block = 14359455889292382949;
  }
  match current_block {
    14359455889292382949 => {
      //bb_error_msg("switching to '%s'", console);
      dup2(fd, 0i32);
      dup2(fd, 1i32);
      dup2(fd, 2i32);
      while fd > 2i32 {
        let fresh0 = fd;
        fd = fd - 1;
        close(fresh0);
      }
      /* Some other session may have it as ctty,
       * try to steal it from them:
       */
      ioctl(0i32, 0x540ei32 as libc::c_ulong, 1i32);
    }
    _ => {}
  }
  BB_EXECVP_or_die(argv);
}

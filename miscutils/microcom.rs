use crate::libbb::ptr_to_globals::bb_errno;
use crate::librb::size_t;
use crate::librb::smallint;

use libc;
use libc::close;
use libc::getpid;
use libc::isatty;
use libc::open;
use libc::pollfd;
use libc::speed_t;
use libc::ssize_t;
use libc::termios;
use libc::unlink;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn dprintf(__fd: libc::c_int, __fmt: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn cfsetspeed(__termios_p: *mut termios, __speed: speed_t) -> libc::c_int;
  #[no_mangle]
  fn tcsetattr(
    __fd: libc::c_int,
    __optional_actions: libc::c_int,
    __termios_p: *const termios,
  ) -> libc::c_int;
  #[no_mangle]
  fn tcsendbreak(__fd: libc::c_int, __duration: libc::c_int) -> libc::c_int;

  #[no_mangle]
  static mut bb_got_signal: smallint;

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

pub type nfds_t = libc::c_ulong;

pub type C2RustUnnamed = libc::c_uint;
pub const COMMON_BUFSIZE: C2RustUnnamed = 1024;
pub const OPT_X: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
// timeout, ms
// wait for device response, ms
pub const OPT_t: C2RustUnnamed_0 = 8;
// baudrate
pub const OPT_d: C2RustUnnamed_0 = 4;
// do not respect Ctrl-X, Ctrl-@
pub const OPT_s: C2RustUnnamed_0 = 2;

/*
 * bare bones 'talk to modem' program - similar to 'cu -l $device'
 * inspired by mgetty's microcom
 *
 * Copyright (C) 2008 by Vladimir Dronnikov <dronnikov@gmail.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config MICROCOM
//config:	bool "microcom (5.7 kb)"
//config:	default y
//config:	help
//config:	The poor man's minicom utility for chatting with serial port devices.
//applet:IF_MICROCOM(APPLET(microcom, BB_DIR_USR_BIN, SUID_DROP))
//kbuild:lib-$(CONFIG_MICROCOM) += microcom.o
//usage:#define microcom_trivial_usage
//usage:       "[-d DELAY] [-t TIMEOUT] [-s SPEED] [-X] TTY"
//usage:#define microcom_full_usage "\n\n"
//usage:       "Copy bytes for stdin to TTY and from TTY to stdout\n"
//usage:     "\n	-d	Wait up to DELAY ms for TTY output before sending every"
//usage:     "\n		next byte to it"
//usage:     "\n	-t	Exit if both stdin and TTY are silent for TIMEOUT ms"
//usage:     "\n	-s	Set serial line to SPEED"
//usage:     "\n	-X	Disable special meaning of NUL and Ctrl-X from stdin"
// set raw tty mode
unsafe extern "C" fn xget1(mut fd: libc::c_int, mut t: *mut termios, mut oldt: *mut termios) {
  crate::libbb::xfuncs::get_termios_and_make_raw(
    fd,
    t,
    oldt,
    0 | 1i32 << 0 | 1i32 << 3i32 | (1i32 << 1i32 | 1i32 << 2i32),
  );
}
unsafe extern "C" fn xset1(
  mut fd: libc::c_int,
  mut tio: *mut termios,
  mut device: *const libc::c_char,
) -> libc::c_int {
  let mut ret: libc::c_int = tcsetattr(fd, 2i32, tio);
  if ret != 0 {
    crate::libbb::perror_msg::bb_perror_msg(
      b"can\'t tcsetattr for %s\x00" as *const u8 as *const libc::c_char,
      device,
    );
  }
  return ret;
}
#[no_mangle]
pub unsafe extern "C" fn microcom_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut sfd: libc::c_int = 0;
  let mut nfd: libc::c_int = 0;
  let mut pfd: [pollfd; 2] = [pollfd {
    fd: 0,
    events: 0,
    revents: 0,
  }; 2];
  let mut tio0: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
  };
  let mut tiosfd: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
  };
  let mut tio: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
  };
  let mut device_lock_file: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut speed: speed_t = 9600i32 as speed_t;
  let mut delay: libc::c_int = -1i32;
  let mut timeout: libc::c_int = -1i32;
  let mut opts: libc::c_uint = 0;
  // fetch options
  opts = crate::libbb::getopt32::getopt32(
    argv,
    b"^Xs:+d:+t:+\x00=1\x00" as *const u8 as *const libc::c_char,
    &mut speed as *mut speed_t,
    &mut delay as *mut libc::c_int,
    &mut timeout as *mut libc::c_int,
  );
  //	argc -= optind;
  argv = argv.offset(optind as isize);
  // try to create lock file in /var/lock
  device_lock_file =
    crate::libbb::get_last_path_component::bb_basename(*argv.offset(0)) as *mut libc::c_char;
  device_lock_file = crate::libbb::xfuncs_printf::xasprintf(
    b"/var/lock/LCK..%s\x00" as *const u8 as *const libc::c_char,
    device_lock_file,
  );
  sfd = open(
    device_lock_file,
    0o100i32 | 0o1i32 | 0o1000i32 | 0o200i32,
    0o644i32,
  );
  if sfd < 0 {
    // device already locked -> bail out
    if *bb_errno == 17i32 {
      crate::libbb::perror_msg::bb_perror_msg_and_die(
        b"can\'t create \'%s\'\x00" as *const u8 as *const libc::c_char,
        device_lock_file,
      );
    }
    // can't create lock -> don't care
    device_lock_file = std::ptr::null_mut::<libc::c_char>()
  } else {
    // %4d to make concurrent mgetty (if any) happy.
    // Mgetty treats 4-bytes lock files as binary,
    // not text, PID. Making 5+ char file. Brrr...
    dprintf(
      sfd,
      b"%4d\n\x00" as *const u8 as *const libc::c_char,
      getpid(),
    );
    close(sfd);
  }
  // setup signals
  crate::libbb::signals::bb_signals(
    0 + (1i32 << 1i32) + (1i32 << 2i32) + (1i32 << 15i32) + (1i32 << 13i32),
    Some(crate::libbb::signals::record_signo as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  // error exit code if we fail to open the device
  bb_got_signal = 1i32 as smallint;
  // open device
  sfd = crate::libbb::xfuncs_printf::open_or_warn(*argv.offset(0), 0o2i32 | 0o400i32 | 0o4000i32);
  if !(sfd < 0) {
    libc::fcntl(sfd, 4i32, 0o2i32);
    // put device to "raw mode"
    xget1(sfd, &mut tio, &mut tiosfd);
    // set device speed
    cfsetspeed(
      &mut tio,
      crate::libbb::speed_table::tty_value_to_baud(speed),
    );
    if !(xset1(sfd, &mut tio, *argv.offset(0)) != 0) {
      // put stdin to "raw mode" (if stdin is a TTY),
      // handle one character at a time
      if isatty(0i32) != 0 {
        xget1(0i32, &mut tio, &mut tio0);
        if xset1(
          0,
          &mut tio,
          b"stdin\x00" as *const u8 as *const libc::c_char,
        ) != 0
        {
          current_block = 12164430063138770559;
        } else {
          current_block = 4761528863920922185;
        }
      } else {
        current_block = 4761528863920922185;
      }
      match current_block {
        12164430063138770559 => {}
        _ => {
          // main loop: check with poll(), then read/write bytes across
          pfd[0].fd = sfd;
          pfd[0].events = 0x1i32 as libc::c_short;
          pfd[1].fd = 0;
          pfd[1].events = 0x1i32 as libc::c_short;
          bb_got_signal = 0 as smallint;
          nfd = 2i32;
          // Not safe_poll: we want to exit on signal
          while bb_got_signal == 0 && poll(pfd.as_mut_ptr(), nfd as nfds_t, timeout) > 0 {
            if nfd > 1i32 && pfd[1].revents as libc::c_int != 0 {
              let mut c: libc::c_char = 0;
              // read from stdin -> write to device
              if crate::libbb::read::safe_read(
                0,
                &mut c as *mut libc::c_char as *mut libc::c_void,
                1i32 as size_t,
              ) < 1
              {
                // don't poll stdin anymore if we got EOF/error
                nfd -= 1
              } else {
                // do we need special processing?
                if opts & OPT_X as libc::c_int as libc::c_uint == 0 {
                  // ^@ sends Break
                  if 0 == c as libc::c_int {
                    tcsendbreak(sfd, 0);
                    current_block = 13460095289871124136;
                  } else {
                    // ^X exits
                    if 24i32 == c as libc::c_int {
                      break;
                    }
                    current_block = 1622411330066726685;
                  }
                } else {
                  current_block = 1622411330066726685;
                }
                match current_block {
                  13460095289871124136 => {}
                  _ => {
                    libc::write(sfd, &mut c as *mut libc::c_char as *const libc::c_void, 1);
                    if delay >= 0 {
                      crate::libbb::safe_poll::safe_poll(pfd.as_mut_ptr(), 1i32 as nfds_t, delay);
                    }
                  }
                }
              }
            }
            if !(pfd[0].revents != 0) {
              continue;
            }
            let mut len: ssize_t = 0;
            // read from device -> write to stdout
            len = crate::libbb::read::safe_read(
              sfd,
              bb_common_bufsiz1.as_mut_ptr() as *mut libc::c_void,
              COMMON_BUFSIZE as libc::c_int as size_t,
            );
            if len > 0 {
              crate::libbb::full_write::full_write(
                1i32,
                bb_common_bufsiz1.as_mut_ptr() as *const libc::c_void,
                len as size_t,
              );
            } else {
              // EOF/error -> bail out
              bb_got_signal = 1i32 as smallint;
              break;
            }
          }
          // restore device mode
          tcsetattr(sfd, 2i32, &mut tiosfd);
          if isatty(0i32) != 0 {
            tcsetattr(0i32, 2i32, &mut tio0);
          }
        }
      }
    }
  }
  if !device_lock_file.is_null() {
    unlink(device_lock_file);
  }
  return bb_got_signal as libc::c_int;
}

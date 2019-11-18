use crate::librb::signal::__sighandler_t;
use crate::librb::size_t;
use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;

use libc;
use libc::alarm;
use libc::printf;
use libc::ssize_t;
use libc::time;
use libc::time_t;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;

  #[no_mangle]
  fn ctime(__timer: *const time_t) -> *mut libc::c_char;
  #[no_mangle]
  fn stime(__when: *const time_t) -> libc::c_int;
  /* Create client TCP socket connected to peer:port. Peer cannot be NULL.
   * Peer can be numeric IP ("N.N.N.N"), numeric IPv6 address or hostname,
   * and can have ":PORT" suffix (for IPv6 use "[X:X:...:X]:PORT").
   * If there is no suffix, port argument is used */
  #[no_mangle]
  fn create_and_connect_stream_or_die(peer: *const libc::c_char, port: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
}

/*
 * The Rdate command will ask a time server for the RFC 868 time
 * and optionally set the system time.
 *
 * by Sterling Huxley <sterling@europa.com>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config RDATE
//config:	bool "rdate (5.6 kb)"
//config:	default y
//config:	help
//config:	The rdate utility allows you to synchronize the date and time of your
//config:	system clock with the date and time of a remote networked system using
//config:	the RFC868 protocol, which is built into the inetd daemon on most
//config:	systems.
//applet:IF_RDATE(APPLET(rdate, BB_DIR_USR_SBIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_RDATE) += rdate.o
//usage:#define rdate_trivial_usage
//usage:       "[-s/-p] HOST"
//usage:#define rdate_full_usage "\n\n"
//usage:       "Set and print time from HOST using RFC 868\n"
//usage:     "\n	-s	Only set system time"
//usage:     "\n	-p	Only print time"
pub type C2RustUnnamed = libc::c_uint;
pub const RFC_868_BIAS: C2RustUnnamed = 2208988800;
unsafe extern "C" fn socket_timeout(mut _sig: libc::c_int) {
  bb_simple_error_msg_and_die(
    b"timeout connecting to time server\x00" as *const u8 as *const libc::c_char,
  );
}
unsafe extern "C" fn askremotedate(mut host: *const libc::c_char) -> time_t {
  let mut nett: u32 = 0;
  let mut fd: libc::c_int = 0;
  /* Timeout for dead or inaccessible servers */
  alarm(10i32 as libc::c_uint);
  signal(
    14i32,
    Some(socket_timeout as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  fd = create_and_connect_stream_or_die(host, 37i32);
  if safe_read(
    fd,
    &mut nett as *mut u32 as *mut libc::c_void,
    4i32 as size_t,
  ) != 4
  {
    /* read time from server */
    bb_error_msg_and_die(
      b"%s: %s\x00" as *const u8 as *const libc::c_char,
      host,
      b"short read\x00" as *const u8 as *const libc::c_char,
    );
  }
  /* Convert from network byte order to local byte order.
   * RFC 868 time is seconds since 1900-01-01 00:00 GMT.
   * RFC 868 time 2,208,988,800 corresponds to 1970-01-01 00:00 GMT.
   * Subtract the RFC 868 time to get Linux epoch.
   */
  nett = ({
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = nett;
    if false {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh0 = &mut __v;
      let fresh1;
      let fresh2 = __x;
      asm!("bswap $0" : "=r" (fresh1) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
    }
    __v
  })
  .wrapping_sub(RFC_868_BIAS as libc::c_uint);
  if ::std::mem::size_of::<time_t>() as libc::c_ulong > 4i32 as libc::c_ulong {
    /* Now we have 32-bit lsb of a wider time_t
     * Imagine that  nett =   0x00000001,
     * current time  cur = 0x123ffffffff.
     * Assuming our time is not some 40 years off,
     * remote time must be 0x12400000001.
     * Need to adjust our time by (i32)(nett - cur).
     */
    let mut cur: time_t = time(0 as *mut time_t);
    let mut adjust: i32 = nett.wrapping_sub(cur as u32) as i32;
    return cur + adjust as libc::c_long;
  }
  /* This is not going to work, but what can we do */
  return nett as time_t;
}
#[no_mangle]
pub unsafe extern "C" fn rdate_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut remote_time: time_t = 0;
  let mut flags: libc::c_uint = 0;
  flags = getopt32(argv, b"^sp\x00-1\x00" as *const u8 as *const libc::c_char);
  remote_time = askremotedate(*argv.offset(optind as isize));
  /* Manpages of various Unixes are confusing. What happens is:
   * (no opts) set and print time
   * -s: set time ("do not print the time")
   * -p: print time ("do not set, just print the remote time")
   * -sp: print time (that's what we do, not sure this is right)
   */
  if flags & 2i32 as libc::c_uint == 0 {
    /* no -p (-s may be present) */
    if time(0 as *mut time_t) == remote_time {
      bb_simple_error_msg(
        b"current time matches remote time\x00" as *const u8 as *const libc::c_char,
      );
    } else if stime(&mut remote_time) < 0i32 {
      bb_simple_perror_msg_and_die(
        b"can\'t set time of day\x00" as *const u8 as *const libc::c_char,
      );
    }
  }
  if flags != 1i32 as libc::c_uint {
    /* not lone -s */
    printf(
      b"%s\x00" as *const u8 as *const libc::c_char,
      ctime(&mut remote_time),
    );
  }
  return 0i32;
}

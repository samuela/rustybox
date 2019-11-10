use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;





























use libc::printf;











use libc::close;

extern "C" {
  pub type sockaddr_x25;
  pub type sockaddr_un;
  pub type sockaddr_ns;
  pub type sockaddr_iso;
  pub type sockaddr_ipx;
  pub type sockaddr_inarp;
  pub type sockaddr_eon;
  pub type sockaddr_dl;
  pub type sockaddr_ax25;
  pub type sockaddr_at;

  #[no_mangle]
  fn usleep(__useconds: useconds_t) -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn connect(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG, __len: socklen_t) -> libc::c_int;
  #[no_mangle]
  fn getservbyport(__port: libc::c_int, __proto: *const libc::c_char) -> *mut servent;

  #[no_mangle]
  fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;

  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn monotonic_us() -> libc::c_ulonglong;
  #[no_mangle]
  fn ndelay_on(fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xsocket(domain: libc::c_int, type_0: libc::c_int, protocol: libc::c_int) -> libc::c_int;
  /* Version which dies on error */
  #[no_mangle]
  fn xhost2sockaddr(host: *const libc::c_char, port: libc::c_int) -> *mut len_and_sockaddr;
  /* Assign sin[6]_port member if the socket is an AF_INET[6] one,
   * otherwise no-op. Useful for ftp.
   * NB: does NOT do htons() internally, just direct assignment. */
  #[no_mangle]
  fn set_nport(sa: *mut sockaddr, port: libc::c_uint);
  #[no_mangle]
  fn xatou_range(str: *const libc::c_char, l: libc::c_uint, u: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_perror_nomsg_and_die() -> !;
}

use libc::useconds_t;

pub type __socklen_t = libc::c_uint;
use crate::librb::size_t;
use libc::ssize_t;

pub type socklen_t = __socklen_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
use libc::sa_family_t;
use libc::sockaddr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
  pub sin6_family: sa_family_t,
  pub sin6_port: in_port_t,
  pub sin6_flowinfo: u32,
  pub sin6_addr: in6_addr,
  pub sin6_scope_id: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
  pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub __u6_addr8: [u8; 16],
  pub __u6_addr16: [u16; 8],
  pub __u6_addr32: [u32; 4],
}
pub type in_port_t = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
  pub sin_family: sa_family_t,
  pub sin_port: in_port_t,
  pub sin_addr: in_addr,
  pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
  pub s_addr: in_addr_t,
}
pub type in_addr_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
  pub __sockaddr__: *const sockaddr,
  pub __sockaddr_at__: *const sockaddr_at,
  pub __sockaddr_ax25__: *const sockaddr_ax25,
  pub __sockaddr_dl__: *const sockaddr_dl,
  pub __sockaddr_eon__: *const sockaddr_eon,
  pub __sockaddr_in__: *const sockaddr_in,
  pub __sockaddr_in6__: *const sockaddr_in6,
  pub __sockaddr_inarp__: *const sockaddr_inarp,
  pub __sockaddr_ipx__: *const sockaddr_ipx,
  pub __sockaddr_iso__: *const sockaddr_iso,
  pub __sockaddr_ns__: *const sockaddr_ns,
  pub __sockaddr_un__: *const sockaddr_un,
  pub __sockaddr_x25__: *const sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct servent {
  pub s_name: *mut libc::c_char,
  pub s_aliases: *mut *mut libc::c_char,
  pub s_port: libc::c_int,
  pub s_proto: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct len_and_sockaddr {
  pub len: socklen_t,
  pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
  pub sa: sockaddr,
  pub sin: sockaddr_in,
  pub sin6: sockaddr_in6,
}
/*
 * Pscan is a mini port scanner implementation for busybox
 *
 * Copyright 2007 Tito Ragusa <farmatito@tiscali.it>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config PSCAN
//config:	bool "pscan (6 kb)"
//config:	default y
//config:	help
//config:	Simple network port scanner.
//applet:IF_PSCAN(APPLET(pscan, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_PSCAN) += pscan.o
//usage:#define pscan_trivial_usage
//usage:       "[-cb] [-p MIN_PORT] [-P MAX_PORT] [-t TIMEOUT] [-T MIN_RTT] HOST"
//usage:#define pscan_full_usage "\n\n"
//usage:       "Scan a host, print all open ports\n"
//usage:     "\n	-c	Show closed ports too"
//usage:     "\n	-b	Show blocked ports too"
//usage:     "\n	-p	Scan from this port (default 1)"
//usage:     "\n	-P	Scan up to this port (default 1024)"
//usage:     "\n	-t	Timeout (default 5000 ms)"
//usage:     "\n	-T	Minimum rtt (default 5 ms, increase for congested hosts)"
/* debugging */
unsafe extern "C" fn port_name(mut port: libc::c_uint) -> *const libc::c_char {
  let mut server: *mut servent = 0 as *mut servent;
  server = getservbyport(
    ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = port as libc::c_ushort;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh0 = &mut __v;
        let fresh1;
        let fresh2 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh1) : "0"
                                    (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2))
                                    : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
      }
      __v
    }) as libc::c_int,
    0 as *const libc::c_char,
  );
  if !server.is_null() {
    return (*server).s_name;
  }
  return b"unknown\x00" as *const u8 as *const libc::c_char;
}
/* We don't expect to see 1000+ seconds delay, unsigned is enough */
#[no_mangle]
pub unsafe extern "C" fn pscan_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opt_max_port: *const libc::c_char = b"1024\x00" as *const u8 as *const libc::c_char; /* -P: default max port */
  let mut opt_min_port: *const libc::c_char = b"1\x00" as *const u8 as *const libc::c_char; /* -p: default min port */
  let mut opt_timeout: *const libc::c_char = b"5000\x00" as *const u8 as *const libc::c_char; /* -t: default timeout in msec */
  /* We estimate rtt and wait rtt*4 before concluding that port is
   * totally blocked. min rtt of 5 ms may be too low if you are
   * scanning an Internet host behind saturated/traffic shaped link.
   * Rule of thumb: with min_rtt of N msec, scanning 1000 ports
   * will take N seconds at absolute minimum */
  let mut opt_min_rtt: *const libc::c_char = b"5\x00" as *const u8 as *const libc::c_char; /* -T: default min rtt in msec */
  let mut result_str: *const libc::c_char = 0 as *const libc::c_char;
  let mut lsap: *mut len_and_sockaddr = 0 as *mut len_and_sockaddr;
  let mut s: libc::c_int = 0;
  let mut opt: libc::c_uint = 0;
  let mut port: libc::c_uint = 0;
  let mut max_port: libc::c_uint = 0;
  let mut nports: libc::c_uint = 0;
  let mut closed_ports: libc::c_uint = 0i32 as libc::c_uint;
  let mut open_ports: libc::c_uint = 0i32 as libc::c_uint;
  /* all in usec */
  let mut timeout: libc::c_uint = 0;
  let mut min_rtt: libc::c_uint = 0;
  let mut rtt_4: libc::c_uint = 0;
  let mut start: libc::c_uint = 0;
  let mut diff: libc::c_uint = 0;
  opt = getopt32(
    argv,
    b"^cbp:P:t:T:\x00=1\x00" as *const u8 as *const libc::c_char,
    &mut opt_min_port as *mut *const libc::c_char,
    &mut opt_max_port as *mut *const libc::c_char,
    &mut opt_timeout as *mut *const libc::c_char,
    &mut opt_min_rtt as *mut *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  max_port = xatou_range(opt_max_port, 1i32 as libc::c_uint, 65535i32 as libc::c_uint);
  port = xatou_range(opt_min_port, 1i32 as libc::c_uint, max_port);
  nports = max_port
    .wrapping_sub(port)
    .wrapping_add(1i32 as libc::c_uint);
  min_rtt = xatou_range(
    opt_min_rtt,
    1i32 as libc::c_uint,
    (2147483647i32 / 1000i32 / 4i32) as libc::c_uint,
  )
  .wrapping_mul(1000i32 as libc::c_uint);
  timeout = xatou_range(
    opt_timeout,
    1i32 as libc::c_uint,
    (2147483647i32 / 1000i32 / 4i32) as libc::c_uint,
  )
  .wrapping_mul(1000i32 as libc::c_uint);
  /* Initial rtt is BIG: */
  rtt_4 = timeout;
  lsap = xhost2sockaddr(*argv, port as libc::c_int);
  printf(
    b"Scanning %s ports %u to %u\n Port\tProto\tState\tService\n\x00" as *const u8
      as *const libc::c_char,
    *argv,
    port,
    max_port,
  );
  while port <= max_port {
    let mut current_block_42: u64;
    /* The SOCK_STREAM socket type is implemented on the TCP/IP protocol. */
    set_nport(
      &mut (*lsap).u.sa,
      ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = port as libc::c_ushort;
        if 0 != 0 {
          __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
            as libc::c_ushort
        } else {
          let fresh3 = &mut __v;
          let fresh4;
          let fresh5 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh4) : "0"
                                (c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5))
                                : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
        }
        __v
      }) as libc::c_uint,
    );
    s = xsocket(
      (*lsap).u.sa.sa_family as libc::c_int,
      SOCK_STREAM as libc::c_int,
      0i32,
    );
    /* We need unblocking socket so we don't need to wait for ETIMEOUT. */
    /* Nonblocking connect typically "fails" with errno == EINPROGRESS */
    ndelay_on(s);
    result_str = 0 as *const libc::c_char;
    start = monotonic_us() as libc::c_uint;
    if connect(
      s,
      __CONST_SOCKADDR_ARG {
        __sockaddr__: &mut (*lsap).u.sa,
      },
      (*lsap).len,
    ) == 0i32
    {
      current_block_42 = 3964065496216564342;
    } else {
      /* Check for untypical errors... */
      if *bb_errno != 11i32 && *bb_errno != 115i32 && *bb_errno != 111i32 {
        bb_perror_nomsg_and_die();
      }
      diff = 0i32 as libc::c_uint;
      current_block_42 = 17281240262373992796;
    }
    loop {
      match current_block_42 {
        17281240262373992796 => {
          if *bb_errno == 111i32 {
            if opt & 1i32 as libc::c_uint != 0 {
              /* -c: show closed too */
              result_str = b"closed\x00" as *const u8 as *const libc::c_char
            }
            closed_ports = closed_ports.wrapping_add(1);
            break;
          } else if diff > rtt_4 {
            if opt & 2i32 as libc::c_uint != 0 {
              /* -b: show blocked too */
              result_str = b"blocked\x00" as *const u8 as *const libc::c_char
            }
            break;
          } else {
            /* Can sleep (much) longer than specified delay.
             * We check rtt BEFORE we usleep, otherwise
             * on localhost we'll have no writes done (!)
             * before we exceed (rather small) rtt */
            usleep(rtt_4.wrapping_div(8i32 as libc::c_uint));
            current_block_42 = 3964065496216564342;
          }
        }
        _ =>
        /* Unlikely, for me even localhost fails :) */
        {
          diff = (monotonic_us() as libc::c_uint).wrapping_sub(start);
          if !(write(
            s,
            b" \x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            1i32 as size_t,
          ) >= 0)
          {
            current_block_42 = 17281240262373992796;
            continue;
          }
          /* We were able to write to the socket */
          open_ports = open_ports.wrapping_add(1);
          result_str = b"open\x00" as *const u8 as *const libc::c_char;
          break;
        }
      }
    }
    if !result_str.is_null() {
      printf(
        b"%5u\ttcp\t%s\t%s\n\x00" as *const u8 as *const libc::c_char,
        port,
        result_str,
        port_name(port),
      );
    }
    /* Estimate new rtt - we don't want to wait entire timeout
     * for each port. *4 allows for rise in net delay.
     * We increase rtt quickly (rtt_4*4), decrease slowly
     * (diff is at least rtt_4/8, *4 == rtt_4/2)
     * because we don't want to accidentally miss ports. */
    rtt_4 = diff.wrapping_mul(4i32 as libc::c_uint);
    if rtt_4 < min_rtt {
      rtt_4 = min_rtt
    }
    if rtt_4 > timeout {
      rtt_4 = timeout
    }
    /* Clean up */
    close(s);
    port = port.wrapping_add(1)
  }
  printf(
    b"%u closed, %u open, %u timed out (or blocked) ports\n\x00" as *const u8
      as *const libc::c_char,
    closed_ports,
    open_ports,
    nports.wrapping_sub(closed_ports.wrapping_add(open_ports)),
  );
  return 0i32;
}

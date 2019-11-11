use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use crate::libbb::ptr_to_globals::bb_errno;
use libc;
use libc::openlog;
use libc::putenv;
extern "C" {
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn rand() -> libc::c_int;
  #[no_mangle]
  fn srand(__seed: libc::c_uint);

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
  #[no_mangle]
  fn inet_aton(__cp: *const libc::c_char, __inp: *mut in_addr) -> libc::c_int;

  #[no_mangle]
  fn monotonic_us() -> libc::c_ulonglong;
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn xsetenv(key: *const libc::c_char, value: *const libc::c_char);
  #[no_mangle]
  fn bb_unsetenv_and_free(key: *mut libc::c_char);
  #[no_mangle]
  fn xsocket(domain: libc::c_int, type_0: libc::c_int, protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xbind(sockfd: libc::c_int, my_addr: *mut sockaddr, addrlen: socklen_t);
  #[no_mangle]
  fn xsendto(
    s: libc::c_int,
    buf: *const libc::c_void,
    len: size_t,
    to: *const sockaddr,
    tolen: socklen_t,
  ) -> ssize_t;
  #[no_mangle]
  fn safe_strncpy(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    size: size_t,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn strncpy_IFNAMSIZ(dst: *mut libc::c_char, src: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn safe_poll(ufds: *mut pollfd, nfds: nfds_t, timeout_ms: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn spawn_and_wait(argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn bb_daemonize_or_rexec(flags: libc::c_int);
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  static mut logmode: smallint;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_logenv_override();
  #[no_mangle]
  fn bb_info_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_xioctl(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  static const_int_0: libc::c_int;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];

}

pub type __caddr_t = *mut libc::c_char;
pub type __socklen_t = libc::c_uint;

pub type bb__aliased_u32 = u32;
/* NB: unaligned parameter should be a pointer, aligned one -
 * a lvalue. This makes it more likely to not swap them by mistake
 */
/* #elif ... - add your favorite arch today! */
/* Unaligned, fixed-endian accessors */
/* unxz needs an aligned fixed-endian accessor.
 * (however, the compiler does not realize it's aligned, the cast is still necessary)
 */
/* ---- Size-saving "small" ints (arch-dependent) ----------- */
/* add other arches which benefit from this... */
use crate::librb::size_t;
use crate::librb::smallint;
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

use libc::sockaddr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
  pub s_addr: in_addr_t,
}
pub type in_addr_t = u32;
pub type nfds_t = libc::c_ulong;
use libc::pollfd;
pub type C2RustUnnamed = libc::c_uint;
pub const LOGMODE_BOTH: C2RustUnnamed = 3;
pub const LOGMODE_SYSLOG: C2RustUnnamed = 2;
pub const LOGMODE_STDIO: C2RustUnnamed = 1;
pub const LOGMODE_NONE: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub iface_sockaddr: sockaddr,
  pub our_ethaddr: ether_addr,
  pub localnet_ip: u32,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ether_addr {
  pub ether_addr_octet: [u8; 6],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ether_header {
  pub ether_dhost: [u8; 6],
  pub ether_shost: [u8; 6],
  pub ether_type: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arphdr {
  pub ar_hrd: libc::c_ushort,
  pub ar_pro: libc::c_ushort,
  pub ar_hln: libc::c_uchar,
  pub ar_pln: libc::c_uchar,
  pub ar_op: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ether_arp {
  pub ea_hdr: arphdr,
  pub arp_sha: [u8; 6],
  pub arp_spa: [u8; 4],
  pub arp_tha: [u8; 6],
  pub arp_tpa: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifmap {
  pub mem_start: libc::c_ulong,
  pub mem_end: libc::c_ulong,
  pub base_addr: libc::c_ushort,
  pub irq: libc::c_uchar,
  pub dma: libc::c_uchar,
  pub port: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifreq {
  pub ifr_ifrn: C2RustUnnamed_1,
  pub ifr_ifru: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
  pub ifru_addr: sockaddr,
  pub ifru_dstaddr: sockaddr,
  pub ifru_broadaddr: sockaddr,
  pub ifru_netmask: sockaddr,
  pub ifru_hwaddr: sockaddr,
  pub ifru_flags: libc::c_short,
  pub ifru_ivalue: libc::c_int,
  pub ifru_mtu: libc::c_int,
  pub ifru_map: ifmap,
  pub ifru_slave: [libc::c_char; 16],
  pub ifru_newname: [libc::c_char; 16],
  pub ifru_data: __caddr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
  pub ifrn_name: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct arp_packet {
  pub eth: ether_header,
  pub arp: ether_arp,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const DEFEND_INTERVAL: C2RustUnnamed_2 = 10;
pub const CONFLICT_MULTIPLIER: C2RustUnnamed_2 = 2;
pub const ANNOUNCE_NUM: C2RustUnnamed_2 = 3;
pub const ANNOUNCE_INTERVAL: C2RustUnnamed_2 = 2;
pub const PROBE_NUM: C2RustUnnamed_2 = 3;
pub const PROBE_MAX: C2RustUnnamed_2 = 2;
pub const PROBE_MIN: C2RustUnnamed_2 = 1;
pub const PROBE_WAIT: C2RustUnnamed_2 = 1;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const DEFEND: C2RustUnnamed_3 = 3;
pub const MONITOR: C2RustUnnamed_3 = 2;
pub const ANNOUNCE: C2RustUnnamed_3 = 1;
pub const PROBE: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const sock_fd: C2RustUnnamed_4 = 3;
pub const op: C2RustUnnamed_5 = 1;
pub type C2RustUnnamed_5 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
  pub null_ethaddr: ether_addr,
  pub ifr: ifreq,
  pub chosen_nip: u32,
  pub conflicts: libc::c_int,
  pub timeout_ms: libc::c_int,
  pub verbose: libc::c_int,
}
/* *
 * Pick a random link local IP address on 169.254/16, except that
 * the first and last 256 addresses are reserved.
 */
unsafe extern "C" fn pick_nip() -> u32 {
  let mut tmp: libc::c_uint = 0;
  loop {
    tmp = rand() as libc::c_uint & (0xffffffffu32 & !0xffff0000u32);
    if !(tmp > (0xffffffffu32 & !0xffff0000u32).wrapping_sub(0x200i32 as libc::c_uint)) {
      break;
    }
  }
  return {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .localnet_ip
      .wrapping_add(0x100i32 as libc::c_uint)
      .wrapping_add(tmp);
    if 0 != 0 {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh0 = &mut __v;
      let fresh1;
      let fresh2 = __x;
      asm!("bswap $0" : "=r" (fresh1) : "0"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2))
                         :);
      c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
    }
    __v
  };
}
unsafe extern "C" fn nip_to_a(mut nip: u32) -> *const libc::c_char {
  let mut in_0: in_addr = in_addr { s_addr: 0 };
  in_0.s_addr = nip;
  return inet_ntoa(in_0);
}
/* *
 * Broadcast an ARP packet.
 */
unsafe extern "C" fn send_arp_request(
  mut source_nip: u32,
  mut target_eth: *const ether_addr,
  mut target_nip: u32,
) {
  let mut p: arp_packet = arp_packet {
    eth: ether_header {
      ether_dhost: [0; 6],
      ether_shost: [0; 6],
      ether_type: 0,
    },
    arp: ether_arp {
      ea_hdr: arphdr {
        ar_hrd: 0,
        ar_pro: 0,
        ar_hln: 0,
        ar_pln: 0,
        ar_op: 0,
      },
      arp_sha: [0; 6],
      arp_spa: [0; 4],
      arp_tha: [0; 6],
      arp_tpa: [0; 4],
    },
  };
  memset(
    &mut p as *mut arp_packet as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<arp_packet>() as libc::c_ulong,
  );
  // ether header
  p.eth.ether_type = {
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = 0x806i32 as libc::c_ushort;
    if 0 != 0 {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh3 = &mut __v;
      let fresh4;
      let fresh5 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh4) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5)) :
                      "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
    }
    __v
  };
  memcpy(
    p.eth.ether_shost.as_mut_ptr() as *mut libc::c_void,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).our_ethaddr as *mut ether_addr
      as *const libc::c_void,
    6i32 as libc::c_ulong,
  );
  memset(
    p.eth.ether_dhost.as_mut_ptr() as *mut libc::c_void,
    0xffi32,
    6i32 as libc::c_ulong,
  );
  // arp request
  p.arp.ea_hdr.ar_hrd = {
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = 1i32 as libc::c_ushort;
    if 0 != 0 {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh6 = &mut __v;
      let fresh7;
      let fresh8 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh7) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh6, fresh8)) :
                      "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
    }
    __v
  };
  p.arp.ea_hdr.ar_pro = {
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = 0x800i32 as libc::c_ushort;
    if 0 != 0 {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh9 = &mut __v;
      let fresh10;
      let fresh11 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh10) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh9, fresh11)) :
                      "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
    }
    __v
  };
  p.arp.ea_hdr.ar_hln = 6i32 as libc::c_uchar;
  p.arp.ea_hdr.ar_pln = 4i32 as libc::c_uchar;
  p.arp.ea_hdr.ar_op = {
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = op as libc::c_int as libc::c_ushort;
    if 0 != 0 {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh12 = &mut __v;
      let fresh13;
      let fresh14 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh13) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh12, fresh14)) :
                      "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh12, fresh14, fresh13);
    }
    __v
  };
  memcpy(
    &mut p.arp.arp_sha as *mut [u8; 6] as *mut libc::c_void,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).our_ethaddr as *mut ether_addr
      as *const libc::c_void,
    6i32 as libc::c_ulong,
  );
  memcpy(
    &mut p.arp.arp_spa as *mut [u8; 4] as *mut libc::c_void,
    &mut source_nip as *mut u32 as *const libc::c_void,
    4i32 as libc::c_ulong,
  );
  memcpy(
    &mut p.arp.arp_tha as *mut [u8; 6] as *mut libc::c_void,
    target_eth as *const libc::c_void,
    6i32 as libc::c_ulong,
  );
  memcpy(
    &mut p.arp.arp_tpa as *mut [u8; 4] as *mut libc::c_void,
    &mut target_nip as *mut u32 as *const libc::c_void,
    4i32 as libc::c_ulong,
  );
  // send it
  // Even though sock_fd is already bound to G.iface_sockaddr, just send()
  // won't work, because "socket is not connected"
  // (and connect() won't fix that, "operation not supported").
  // Thus we sendto() to G.iface_sockaddr. I wonder which sockaddr
  // (from bind() or from sendto()?) kernel actually uses
  // to determine iface to emit the packet from...
  xsendto(
    sock_fd as libc::c_int,
    &mut p as *mut arp_packet as *const libc::c_void,
    ::std::mem::size_of::<arp_packet>() as libc::c_ulong,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).iface_sockaddr,
    ::std::mem::size_of::<sockaddr>() as libc::c_ulong as socklen_t,
  );
}
/* *
 * Run a script.
 * argv[0]:intf argv[1]:script_name argv[2]:junk argv[3]:NULL
 */
unsafe extern "C" fn run(
  mut argv: *mut *mut libc::c_char,
  mut param: *const libc::c_char,
  mut nip: u32,
) -> libc::c_int {
  let mut status: libc::c_int = 0; /* for gcc */
  let mut addr: *const libc::c_char = 0 as *const libc::c_char;
  addr = addr;
  let mut fmt: *const libc::c_char =
    (b"%s %s %s\x00" as *const u8 as *const libc::c_char).offset(3);
  let mut env_ip: *mut libc::c_char = 0 as *mut libc::c_char;
  env_ip = env_ip;
  let ref mut fresh15 = *argv.offset(2);
  *fresh15 = param as *mut libc::c_char;
  if nip != 0i32 as libc::c_uint {
    addr = nip_to_a(nip);
    /* Must not use setenv() repeatedly, it leaks memory. Use putenv() */
    env_ip = xasprintf(b"ip=%s\x00" as *const u8 as *const libc::c_char, addr);
    putenv(env_ip);
    fmt = fmt.offset(-3)
  }
  bb_info_msg(fmt, *argv.offset(2), *argv.offset(0), addr);
  status = spawn_and_wait(argv.offset(1));
  if nip != 0i32 as libc::c_uint {
    bb_unsetenv_and_free(env_ip);
  }
  if status < 0i32 {
    bb_perror_msg(
      (b"%s %s %s\x00" as *const u8 as *const libc::c_char).offset(3),
      *argv.offset(2),
      *argv.offset(0),
    );
    return -*bb_errno;
  }
  if status != 0i32 {
    bb_error_msg(
      b"script %s %s failed, exitcode=%d\x00" as *const u8 as *const libc::c_char,
      *argv.offset(1),
      *argv.offset(2),
      status & 0xffi32,
    );
  }
  return status;
}
/* *
 * Return milliseconds of random delay, up to "secs" seconds.
 */
#[inline(always)]
unsafe extern "C" fn random_delay_ms(mut secs: libc::c_uint) -> libc::c_uint {
  return (rand() as libc::c_uint).wrapping_rem(secs.wrapping_mul(1000i32 as libc::c_uint));
}
/* *
 * main program
 */
#[no_mangle]
pub unsafe extern "C" fn zcip_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut r_opt: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut l_opt: *const libc::c_char = b"169.254.0.0\x00" as *const u8 as *const libc::c_char;
  let mut state: libc::c_int = 0;
  let mut nsent: libc::c_int = 0;
  let mut opts: libc::c_uint = 0;
  // Ugly trick, but I want these zeroed in one go
  let mut L: C2RustUnnamed_6 = C2RustUnnamed_6 {
    null_ethaddr: ether_addr {
      ether_addr_octet: [0; 6],
    },
    ifr: ifreq {
      ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
      ifr_ifru: C2RustUnnamed_0 {
        ifru_addr: sockaddr {
          sa_family: 0,
          sa_data: [0; 14],
        },
      },
    },
    chosen_nip: 0,
    conflicts: 0,
    timeout_ms: 0,
    verbose: 0,
  };
  memset(
    &mut L as *mut C2RustUnnamed_6 as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong,
  );
  // Parse commandline: prog [options] ifname script
  // exactly 2 args; -v accumulates and implies -f
  opts = getopt32(
    argv,
    b"^fqr:l:v\x00=2:vv:vf\x00" as *const u8 as *const libc::c_char,
    &mut r_opt as *mut *mut libc::c_char,
    &mut l_opt as *mut *const libc::c_char,
    &mut L.verbose as *mut libc::c_int,
  );
  // Open an ARP socket
  // (need to do it before openlog to prevent openlog from taking
  // fd 3 (sock_fd==3))
  xmove_fd(
    xsocket(
      17i32,
      SOCK_PACKET as libc::c_int,
      ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = 0x806i32 as libc::c_ushort;
        if 0 != 0 {
          __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
            as libc::c_ushort
        } else {
          let fresh16 = &mut __v;
          let fresh17;
          let fresh18 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh17) : "0"
                                   (c2rust_asm_casts::AsmCast::cast_in(fresh16, fresh18))
                                   : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh16, fresh18, fresh17);
        }
        __v
      }) as libc::c_int,
    ),
    sock_fd as libc::c_int,
  );
  if opts & 1i32 as libc::c_uint == 0 {
    // do it before all bb_xx_msg calls
    openlog(applet_name, 0i32, 3i32 << 3i32);
    logmode = (logmode as libc::c_int | LOGMODE_SYSLOG as libc::c_int) as smallint
  }
  bb_logenv_override();
  // -l n.n.n.n
  let mut net: in_addr = in_addr { s_addr: 0 };
  if inet_aton(l_opt, &mut net) == 0i32
    || net.s_addr
      & ({
        let mut __v: libc::c_uint = 0;
        let mut __x: libc::c_uint = 0xffff0000u32;
        if 0 != 0 {
          __v = (__x & 0xff000000u32) >> 24i32
            | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
            | (__x & 0xff00i32 as libc::c_uint) << 8i32
            | (__x & 0xffi32 as libc::c_uint) << 24i32
        } else {
          let fresh19 = &mut __v;
          let fresh20;
          let fresh21 = __x;
          asm!("bswap $0" : "=r" (fresh20) : "0"
                             (c2rust_asm_casts::AsmCast::cast_in(fresh19, fresh21))
                             :);
          c2rust_asm_casts::AsmCast::cast_out(fresh19, fresh21, fresh20);
        }
        __v
      })
      != net.s_addr
  {
    bb_simple_error_msg_and_die(b"invalid network address\x00" as *const u8 as *const libc::c_char);
  }
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).localnet_ip = {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = net.s_addr;
    if 0 != 0 {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh22 = &mut __v;
      let fresh23;
      let fresh24 = __x;
      asm!("bswap $0" : "=r" (fresh23) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh22, fresh24))
                      :);
      c2rust_asm_casts::AsmCast::cast_out(fresh22, fresh24, fresh23);
    }
    __v
  };
  if opts & 4i32 as libc::c_uint != 0 {
    // -r n.n.n.n
    let mut ip: in_addr = in_addr { s_addr: 0 };
    if inet_aton(r_opt, &mut ip) == 0i32
      || ({
        let mut __v: libc::c_uint = 0;
        let mut __x: libc::c_uint = ip.s_addr;
        if 0 != 0 {
          __v = (__x & 0xff000000u32) >> 24i32
            | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
            | (__x & 0xff00i32 as libc::c_uint) << 8i32
            | (__x & 0xffi32 as libc::c_uint) << 24i32
        } else {
          let fresh25 = &mut __v;
          let fresh26;
          let fresh27 = __x;
          asm!("bswap $0" : "=r" (fresh26) : "0"
                             (c2rust_asm_casts::AsmCast::cast_in(fresh25, fresh27))
                             :);
          c2rust_asm_casts::AsmCast::cast_out(fresh25, fresh27, fresh26);
        }
        __v
      }) & 0xffff0000u32
        != (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).localnet_ip
    {
      bb_simple_error_msg_and_die(b"invalid link address\x00" as *const u8 as *const libc::c_char);
    }
    L.chosen_nip = ip.s_addr
  }
  argv = argv.offset((optind - 1i32) as isize);
  /* Now: argv[0]:junk argv[1]:intf argv[2]:script argv[3]:NULL */
  /* We need to make space for script argument: */
  let ref mut fresh28 = *argv.offset(0);
  *fresh28 = *argv.offset(1);
  let ref mut fresh29 = *argv.offset(1);
  *fresh29 = *argv.offset(2);
  /* Now: argv[0]:intf argv[1]:script argv[2]:junk argv[3]:NULL */
  xsetenv(
    b"interface\x00" as *const u8 as *const libc::c_char,
    *argv.offset(0),
  );
  // Initialize the interface (modprobe, ifup, etc)
  if run(
    argv,
    b"init\x00" as *const u8 as *const libc::c_char,
    0i32 as u32,
  ) != 0
  {
    return 1i32;
  }
  // Initialize G.iface_sockaddr
  // G.iface_sockaddr is: { u16 sa_family; u8 sa_data[14]; }
  //memset(&G.iface_sockaddr, 0, sizeof(G.iface_sockaddr));
  //TODO: are we leaving sa_family == 0 (AF_UNSPEC)?!
  safe_strncpy(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .iface_sockaddr
      .sa_data
      .as_mut_ptr(),
    *argv.offset(0),
    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong,
  );
  // Bind to the interface's ARP socket
  xbind(
    sock_fd as libc::c_int,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).iface_sockaddr,
    ::std::mem::size_of::<sockaddr>() as libc::c_ulong as socklen_t,
  );
  // Get the interface's ethernet address
  //memset(&ifr, 0, sizeof(ifr));
  strncpy_IFNAMSIZ(L.ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), *argv.offset(0));
  bb_xioctl(
    sock_fd as libc::c_int,
    0x8927i32 as libc::c_uint,
    &mut L.ifr as *mut ifreq as *mut libc::c_void,
    b"SIOCGIFHWADDR\x00" as *const u8 as *const libc::c_char,
  );
  memcpy(
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).our_ethaddr as *mut ether_addr
      as *mut libc::c_void,
    &mut L.ifr.ifr_ifru.ifru_hwaddr.sa_data as *mut [libc::c_char; 14] as *const libc::c_void,
    6i32 as libc::c_ulong,
  );
  // Start with some stable ip address, either a function of
  // the hardware address or else the last address we used.
  // we are taking low-order four bytes, as top-order ones
  // aren't random enough.
  // NOTE: the sequence of addresses we try changes only
  // depending on when we detect conflicts.
  let mut t: u32 = 0;
  t = *((&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).our_ethaddr as *mut ether_addr
    as *mut libc::c_char)
    .offset(2) as *mut bb__aliased_u32);
  srand(t);
  // FIXME cases to handle:
  //  - zcip already running!
  //  - link already has local address... just defend/update
  // Daemonize now; don't delay system startup
  if opts & 1i32 as libc::c_uint == 0 {
    bb_daemonize_or_rexec(0i32); /*was: DAEMON_CHDIR_ROOT*/
    bb_info_msg(
      b"start, interface %s\x00" as *const u8 as *const libc::c_char,
      *argv.offset(0),
    );
  }
  // Run the dynamic address negotiation protocol,
  // restarting after address conflicts:
  //  - start with some address we want to try
  //  - short random delay
  //  - arp probes to see if another host uses it
  //    00:04:e2:64:23:c2 > ff:ff:ff:ff:ff:ff arp who-has 169.254.194.171 tell 0.0.0.0
  //  - arp announcements that we're claiming it
  //    00:04:e2:64:23:c2 > ff:ff:ff:ff:ff:ff arp who-has 169.254.194.171 (00:04:e2:64:23:c2) tell 169.254.194.171
  //  - use it
  //  - defend it, within limits
  // exit if:
  // - address is successfully obtained and -q was given:
  //   run "<script> config", then exit with exitcode 0
  // - poll error (when does this happen?)
  // - read error (when does this happen?)
  // - sendto error (in send_arp_request()) (when does this happen?)
  // - revents & POLLERR (link down). run "<script> deconfig" first
  if L.chosen_nip == 0i32 as libc::c_uint {
    current_block = 11627159856682951070;
  } else {
    current_block = 5330834795799507926;
  }
  loop {
    match current_block {
      11627159856682951070 => {
        L.chosen_nip = pick_nip();
        current_block = 5330834795799507926;
      }
      _ => {
        nsent = 0i32;
        state = PROBE as libc::c_int;
        loop {
          let mut fds: [pollfd; 1] = [pollfd {
            fd: 0,
            events: 0,
            revents: 0,
          }; 1];
          let mut deadline_us: libc::c_uint = 0;
          deadline_us = deadline_us;
          let mut p: arp_packet = arp_packet {
            eth: ether_header {
              ether_dhost: [0; 6],
              ether_shost: [0; 6],
              ether_type: 0,
            },
            arp: ether_arp {
              ea_hdr: arphdr {
                ar_hrd: 0,
                ar_pro: 0,
                ar_hln: 0,
                ar_pln: 0,
                ar_op: 0,
              },
              arp_sha: [0; 6],
              arp_spa: [0; 4],
              arp_tha: [0; 6],
              arp_tpa: [0; 4],
            },
          };
          let mut ip_conflict: libc::c_int = 0;
          let mut n: libc::c_int = 0;
          fds[0].fd = sock_fd as libc::c_int;
          fds[0].events = 0x1i32 as libc::c_short;
          fds[0].revents = 0i32 as libc::c_short;
          // Note: if we only have a target IP conflict here (ip_conflict & 2),
          // IOW: if we just saw this sort of ARP packet:
          //  aa:bb:cc:dd:ee:ff > xx:xx:xx:xx:xx:xx arp who-has <chosen_nip> tell 0.0.0.0
          // we expect _kernel_ to respond to that, because <chosen_nip>
          // is (expected to be) configured on this iface.
          // Poll, being ready to adjust current timeout
          if L.timeout_ms == 0 {
            L.timeout_ms = random_delay_ms(PROBE_WAIT as libc::c_int as libc::c_uint) as libc::c_int
            // FIXME setsockopt(sock_fd, SO_ATTACH_FILTER, ...) to
            // make the kernel filter out all packets except
            // ones we'd care about.
          }
          if L.timeout_ms >= 0i32 {
            // Set deadline_us to the point in time when we timeout
            deadline_us = (monotonic_us() as libc::c_uint)
              .wrapping_add((L.timeout_ms * 1000i32) as libc::c_uint)
          }
          n = safe_poll(fds.as_mut_ptr(), 1i32 as nfds_t, L.timeout_ms);
          if n < 0i32 {
            //bb_perror_msg("poll"); - done in safe_poll
            return 1i32;
          }
          if n == 0i32 {
            // timed out?
            match state {
              0 => {
                // No conflicting ARP packets were seen:
                // we can progress through the states
                if nsent < PROBE_NUM as libc::c_int {
                  nsent += 1;
                  L.timeout_ms = PROBE_MIN as libc::c_int * 1000i32;
                  L.timeout_ms = (L.timeout_ms as libc::c_uint).wrapping_add(random_delay_ms(
                    (PROBE_MAX as libc::c_int - PROBE_MIN as libc::c_int) as libc::c_uint,
                  )) as libc::c_int as libc::c_int;
                  send_arp_request(0i32 as u32, &L.null_ethaddr, L.chosen_nip);
                  continue;
                } else {
                  // Switch to announce state
                  nsent = 0i32;
                  state = ANNOUNCE as libc::c_int
                }
                current_block = 7214547043153522063;
              }
              1 => {
                // No conflicting ARP packets were seen:
                // we can progress through the states
                if nsent < ANNOUNCE_NUM as libc::c_int {
                  current_block = 7214547043153522063;
                } else {
                  // Switch to monitor state
                  // FIXME update filters
                  run(
                    argv,
                    b"config\x00" as *const u8 as *const libc::c_char,
                    L.chosen_nip,
                  );
                  // NOTE: all other exit paths should deconfig...
                  if opts & 2i32 as libc::c_uint != 0 {
                    return 0i32;
                  }
                  current_block = 14823871904523042870;
                }
              }
              _ => {
                current_block = 14823871904523042870;
              }
            }
            match current_block {
              14823871904523042870 =>
              // fall through: switch to MONITOR
              {
                L.timeout_ms = -1i32; // never timeout in monitor state
                state = MONITOR as libc::c_int
                // case DEFEND:
                // case MONITOR: (shouldn't happen, MONITOR timeout is infinite)
                // Defend period ended with no ARP replies - we won
              }
              _ => {
                nsent += 1;
                L.timeout_ms = ANNOUNCE_INTERVAL as libc::c_int * 1000i32;
                send_arp_request(
                  L.chosen_nip,
                  &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).our_ethaddr,
                  L.chosen_nip,
                );
              }
            }
          } else {
            // Packet arrived, or link went down.
            // We need to adjust the timeout in case we didn't receive
            // a conflicting packet.
            if L.timeout_ms > 0i32 {
              let mut diff: libc::c_uint = deadline_us.wrapping_sub(monotonic_us() as libc::c_uint);
              if (diff as libc::c_int) < 0i32 {
                // never 0
                // Current time is greater than the expected timeout time.
                diff = 0i32 as libc::c_uint
              }
              L.timeout_ms =
                (diff.wrapping_div(1000i32 as libc::c_uint) | 1i32 as libc::c_uint) as libc::c_int
            }
            if fds[0].revents as libc::c_int & 0x1i32 == 0i32 {
              if fds[0].revents as libc::c_int & 0x8i32 != 0 {
                // FIXME: links routinely go down;
                // this shouldn't necessarily exit.
                bb_error_msg(
                  b"iface %s is down\x00" as *const u8 as *const libc::c_char,
                  *argv.offset(0),
                );
                if state >= MONITOR as libc::c_int {
                  // Only if we are in MONITOR or DEFEND
                  run(
                    argv,
                    b"deconfig\x00" as *const u8 as *const libc::c_char,
                    L.chosen_nip,
                  );
                }
                return 1i32;
              }
            } else {
              // Read ARP packet
              if safe_read(
                sock_fd as libc::c_int,
                &mut p as *mut arp_packet as *mut libc::c_void,
                ::std::mem::size_of::<arp_packet>() as libc::c_ulong,
              ) < 0
              {
                bb_simple_perror_msg_and_die(b"read error\x00" as *const u8 as *const libc::c_char);
              }
              if p.eth.ether_type as libc::c_int
                != ({
                  let mut __v: libc::c_ushort = 0;
                  let mut __x: libc::c_ushort = 0x806i32 as libc::c_ushort;
                  if 0 != 0 {
                    __v = (__x as libc::c_int >> 8i32 & 0xffi32
                      | (__x as libc::c_int & 0xffi32) << 8i32)
                      as libc::c_ushort
                  } else {
                    let fresh30 = &mut __v;
                    let fresh31;
                    let fresh32 = __x;
                    asm!("rorw $$8, ${0:w}" : "=r"
                                                 (fresh31) : "0"
                                                 (c2rust_asm_casts::AsmCast::cast_in(fresh30, fresh32))
                                                 : "cc");
                    c2rust_asm_casts::AsmCast::cast_out(fresh30, fresh32, fresh31);
                  }
                  __v
                }) as libc::c_int
              {
                continue;
              }
              if p.arp.ea_hdr.ar_op as libc::c_int
                != ({
                  let mut __v: libc::c_ushort = 0;
                  let mut __x: libc::c_ushort = 1i32 as libc::c_ushort;
                  if 0 != 0 {
                    __v = (__x as libc::c_int >> 8i32 & 0xffi32
                      | (__x as libc::c_int & 0xffi32) << 8i32)
                      as libc::c_ushort
                  } else {
                    let fresh33 = &mut __v;
                    let fresh34;
                    let fresh35 = __x;
                    asm!("rorw $$8, ${0:w}" : "=r"
                                                 (fresh34) : "0"
                                                 (c2rust_asm_casts::AsmCast::cast_in(fresh33, fresh35))
                                                 : "cc");
                    c2rust_asm_casts::AsmCast::cast_out(fresh33, fresh35, fresh34);
                  }
                  __v
                }) as libc::c_int
                && p.arp.ea_hdr.ar_op as libc::c_int
                  != ({
                    let mut __v: libc::c_ushort = 0;
                    let mut __x: libc::c_ushort = 2i32 as libc::c_ushort;
                    if 0 != 0 {
                      __v = (__x as libc::c_int >> 8i32 & 0xffi32
                        | (__x as libc::c_int & 0xffi32) << 8i32)
                        as libc::c_ushort
                    } else {
                      let fresh36 = &mut __v;
                      let fresh37;
                      let fresh38 = __x;
                      asm!("rorw $$8, ${0:w}" : "=r"
                                                     (fresh37) : "0"
                                                     (c2rust_asm_casts::AsmCast::cast_in(fresh36, fresh38))
                                                     : "cc");
                      c2rust_asm_casts::AsmCast::cast_out(fresh36, fresh38, fresh37);
                    }
                    __v
                  }) as libc::c_int
              {
                continue;
              }
              ip_conflict = 0i32;
              if memcmp(
                &mut p.arp.arp_sha as *mut [u8; 6] as *const libc::c_void,
                &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).our_ethaddr
                  as *mut ether_addr as *const libc::c_void,
                6i32 as libc::c_ulong,
              ) != 0i32
              {
                if memcmp(
                  p.arp.arp_spa.as_mut_ptr() as *const libc::c_void,
                  &mut L.chosen_nip as *mut u32 as *const libc::c_void,
                  4i32 as libc::c_ulong,
                ) == 0i32
                {
                  // A probe or reply with source_ip == chosen ip
                  ip_conflict = 1i32
                }
                if p.arp.ea_hdr.ar_op as libc::c_int
                  == ({
                    let mut __v: libc::c_ushort = 0;
                    let mut __x: libc::c_ushort = 1i32 as libc::c_ushort;
                    if 0 != 0 {
                      __v = (__x as libc::c_int >> 8i32 & 0xffi32
                        | (__x as libc::c_int & 0xffi32) << 8i32)
                        as libc::c_ushort
                    } else {
                      let fresh39 = &mut __v;
                      let fresh40;
                      let fresh41 = __x;
                      asm!("rorw $$8, ${0:w}" : "=r"
                                                     (fresh40) : "0"
                                                     (c2rust_asm_casts::AsmCast::cast_in(fresh39, fresh41))
                                                     : "cc");
                      c2rust_asm_casts::AsmCast::cast_out(fresh39, fresh41, fresh40);
                    }
                    __v
                  }) as libc::c_int
                  && memcmp(
                    p.arp.arp_spa.as_mut_ptr() as *const libc::c_void,
                    &const_int_0 as *const libc::c_int as *const libc::c_void,
                    4i32 as libc::c_ulong,
                  ) == 0i32
                  && memcmp(
                    p.arp.arp_tpa.as_mut_ptr() as *const libc::c_void,
                    &mut L.chosen_nip as *mut u32 as *const libc::c_void,
                    4i32 as libc::c_ulong,
                  ) == 0i32
                {
                  // A probe with source_ip == 0.0.0.0, target_ip == chosen ip:
                  // another host trying to claim this ip!
                  ip_conflict |= 2i32
                }
              }
              if ip_conflict == 0 {
                continue;
              }
              // Either src or target IP conflict exists
              if state <= ANNOUNCE as libc::c_int {
                // PROBE or ANNOUNCE
                L.conflicts += 1;
                L.timeout_ms = ((PROBE_MIN as libc::c_int * 1000i32) as libc::c_uint).wrapping_add(
                  (CONFLICT_MULTIPLIER as libc::c_int as libc::c_uint)
                    .wrapping_mul(random_delay_ms(L.conflicts as libc::c_uint)),
                ) as libc::c_int;
                current_block = 11627159856682951070;
                break;
              } else {
                // MONITOR or DEFEND: only src IP conflict is a problem
                if !(ip_conflict & 1i32 != 0) {
                  continue;
                }
                if state == MONITOR as libc::c_int {
                  // Src IP conflict, defend with a single ARP probe
                  L.timeout_ms = DEFEND_INTERVAL as libc::c_int * 1000i32;
                  state = DEFEND as libc::c_int;
                  send_arp_request(
                    L.chosen_nip,
                    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).our_ethaddr,
                    L.chosen_nip,
                  );
                } else {
                  // state == DEFEND
                  // Another src IP conflict, start over
                  run(
                    argv,
                    b"deconfig\x00" as *const u8 as *const libc::c_char,
                    L.chosen_nip,
                  );
                  L.conflicts = 0i32;
                  L.timeout_ms = 0i32;
                  current_block = 11627159856682951070;
                  break;
                }
              }
            }
          }
        }
      }
    }
  }
  // while (1)
}

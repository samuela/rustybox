use crate::librb::size_t;
use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
use libc::alarm;
use libc::close;
use libc::printf;
use libc::puts;
use libc::sigaddset;
use libc::sigprocmask;
use libc::sigset_t;
use libc::ssize_t;
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
  fn sendto(
    __fd: libc::c_int,
    __buf: *const libc::c_void,
    __n: size_t,
    __flags: libc::c_int,
    __addr: __CONST_SOCKADDR_ARG,
    __addr_len: socklen_t,
  ) -> ssize_t;
  #[no_mangle]
  fn recvfrom(
    __fd: libc::c_int,
    __buf: *mut libc::c_void,
    __n: size_t,
    __flags: libc::c_int,
    __addr: __SOCKADDR_ARG,
    __addr_len: *mut socklen_t,
  ) -> ssize_t;
  #[no_mangle]
  fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
  #[no_mangle]
  fn inet_aton(__cp: *const libc::c_char, __inp: *mut in_addr) -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn exit(_: libc::c_int) -> !;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn mempcpy(
    __dest: *mut libc::c_void,
    __src: *const libc::c_void,
    __n: size_t,
  ) -> *mut libc::c_void;

  #[no_mangle]
  fn monotonic_us() -> libc::c_ulonglong;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn signal_SA_RESTART_empty_mask(
    sig: libc::c_int,
    handler: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
  );
  #[no_mangle]
  fn xsocket(domain: libc::c_int, type_0: libc::c_int, protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xbind(sockfd: libc::c_int, my_addr: *mut sockaddr, addrlen: socklen_t);
  #[no_mangle]
  fn xconnect(s: libc::c_int, s_addr: *const sockaddr, addrlen: socklen_t);
  #[no_mangle]
  fn setsockopt_SOL_SOCKET_1(fd: libc::c_int, optname: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn setsockopt_bindtodevice(fd: libc::c_int, iface: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn bb_getsockname(
    sockfd: libc::c_int,
    addr: *mut libc::c_void,
    addrlen: socklen_t,
  ) -> libc::c_int;
  #[no_mangle]
  fn xhost_and_af2sockaddr(
    host: *const libc::c_char,
    port: libc::c_int,
    af: sa_family_t,
  ) -> *mut len_and_sockaddr;
  #[no_mangle]
  fn strncpy_IFNAMSIZ(dst: *mut libc::c_char, src: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn xatou_range(str: *const libc::c_char, l: libc::c_uint, u: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  static mut xfunc_error_retval: u8;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn ioctl_or_perror_and_die(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    fmt: *const libc::c_char,
    _: ...
  ) -> libc::c_int;
  #[no_mangle]
  fn bb_xioctl(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  static ptr_to_globals: *mut globals;
}

pub type __caddr_t = *mut libc::c_char;
pub type __socklen_t = libc::c_uint;

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
pub union __SOCKADDR_ARG {
  pub __sockaddr__: *mut sockaddr,
  pub __sockaddr_at__: *mut sockaddr_at,
  pub __sockaddr_ax25__: *mut sockaddr_ax25,
  pub __sockaddr_dl__: *mut sockaddr_dl,
  pub __sockaddr_eon__: *mut sockaddr_eon,
  pub __sockaddr_in__: *mut sockaddr_in,
  pub __sockaddr_in6__: *mut sockaddr_in6,
  pub __sockaddr_inarp__: *mut sockaddr_inarp,
  pub __sockaddr_ipx__: *mut sockaddr_ipx,
  pub __sockaddr_iso__: *mut sockaddr_iso,
  pub __sockaddr_ns__: *mut sockaddr_ns,
  pub __sockaddr_un__: *mut sockaddr_un,
  pub __sockaddr_x25__: *mut sockaddr_x25,
}
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IFF_DYNAMIC: C2RustUnnamed_0 = 32768;
pub const IFF_AUTOMEDIA: C2RustUnnamed_0 = 16384;
pub const IFF_PORTSEL: C2RustUnnamed_0 = 8192;
pub const IFF_MULTICAST: C2RustUnnamed_0 = 4096;
pub const IFF_SLAVE: C2RustUnnamed_0 = 2048;
pub const IFF_MASTER: C2RustUnnamed_0 = 1024;
pub const IFF_ALLMULTI: C2RustUnnamed_0 = 512;
pub const IFF_PROMISC: C2RustUnnamed_0 = 256;
pub const IFF_NOARP: C2RustUnnamed_0 = 128;
pub const IFF_RUNNING: C2RustUnnamed_0 = 64;
pub const IFF_NOTRAILERS: C2RustUnnamed_0 = 32;
pub const IFF_POINTOPOINT: C2RustUnnamed_0 = 16;
pub const IFF_LOOPBACK: C2RustUnnamed_0 = 8;
pub const IFF_DEBUG: C2RustUnnamed_0 = 4;
pub const IFF_BROADCAST: C2RustUnnamed_0 = 2;
pub const IFF_UP: C2RustUnnamed_0 = 1;
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
  pub ifr_ifrn: C2RustUnnamed_2,
  pub ifr_ifru: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
pub union C2RustUnnamed_2 {
  pub ifrn_name: [libc::c_char; 16],
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
pub struct sockaddr_ll {
  pub sll_family: libc::c_ushort,
  pub sll_protocol: libc::c_ushort,
  pub sll_ifindex: libc::c_int,
  pub sll_hatype: libc::c_ushort,
  pub sll_pkttype: libc::c_uchar,
  pub sll_halen: libc::c_uchar,
  pub sll_addr: [libc::c_uchar; 8],
}
pub type bb__aliased_u32 = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct len_and_sockaddr {
  pub len: socklen_t,
  pub u: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
  pub sa: sockaddr,
  pub sin: sockaddr_in,
  pub sin6: sockaddr_in6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub src: in_addr,
  pub dst: in_addr,
  pub me: sockaddr_ll,
  pub he: sockaddr_ll,
  pub count: libc::c_int,
  pub last: libc::c_uint,
  pub timeout_us: libc::c_uint,
  pub start: libc::c_uint,
  pub sent: libc::c_uint,
  pub brd_sent: libc::c_uint,
  pub received: libc::c_uint,
  pub brd_recv: libc::c_uint,
  pub req_recv: libc::c_uint,
  pub ifr: ifreq,
  pub probe_saddr: sockaddr_in,
  pub sset: sigset_t,
  pub packet: [libc::c_uchar; 4096],
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const TIMEOUT: C2RustUnnamed_4 = 128;
pub const UNICASTING: C2RustUnnamed_4 = 64;
pub const BCAST_ONLY: C2RustUnnamed_4 = 32;
pub const QUIT_ON_REPLY: C2RustUnnamed_4 = 16;
pub const QUIET: C2RustUnnamed_4 = 8;
pub const ADVERT: C2RustUnnamed_4 = 4;
pub const DAD: C2RustUnnamed_4 = 2;
pub const UNSOLICITED: C2RustUnnamed_4 = 1;
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
unsafe extern "C" fn send_pack(
  mut src_addr: *mut in_addr,
  mut dst_addr: *mut in_addr,
  mut ME: *mut sockaddr_ll,
  mut HE: *mut sockaddr_ll,
) -> libc::c_int {
  let mut err: libc::c_int = 0;
  let mut buf: [libc::c_uchar; 256] = [0; 256];
  let mut ah: *mut arphdr = buf.as_mut_ptr() as *mut arphdr;
  let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
  (*ah).ar_hrd = {
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = 1i32 as libc::c_ushort;
    if 0 != 0 {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh0 = &mut __v;
      let fresh1;
      let fresh2 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh1) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2)) :
                      "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
    }
    __v
  };
  (*ah).ar_pro = {
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = 0x800i32 as libc::c_ushort;
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
  (*ah).ar_hln = (*ME).sll_halen;
  (*ah).ar_pln = 4i32 as libc::c_uchar;
  (*ah).ar_op = if option_mask32 & ADVERT as libc::c_int as libc::c_uint != 0 {
    ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = 2i32 as libc::c_ushort;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh6 = &mut __v;
        let fresh7;
        let fresh8 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh7) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh6, fresh8))
                          : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
      }
      __v
    }) as libc::c_int
  } else {
    ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = 1i32 as libc::c_ushort;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh9 = &mut __v;
        let fresh10;
        let fresh11 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh10) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh9, fresh11))
                          : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
      }
      __v
    }) as libc::c_int
  } as libc::c_ushort;
  p = ah.offset(1) as *mut libc::c_uchar;
  p = mempcpy(
    p as *mut libc::c_void,
    &mut (*ME).sll_addr as *mut [libc::c_uchar; 8] as *const libc::c_void,
    (*ah).ar_hln as size_t,
  ) as *mut libc::c_uchar;
  p = mempcpy(
    p as *mut libc::c_void,
    src_addr as *const libc::c_void,
    4i32 as size_t,
  ) as *mut libc::c_uchar;
  if option_mask32 & ADVERT as libc::c_int as libc::c_uint != 0 {
    p = mempcpy(
      p as *mut libc::c_void,
      &mut (*ME).sll_addr as *mut [libc::c_uchar; 8] as *const libc::c_void,
      (*ah).ar_hln as size_t,
    ) as *mut libc::c_uchar
  } else {
    p = mempcpy(
      p as *mut libc::c_void,
      &mut (*HE).sll_addr as *mut [libc::c_uchar; 8] as *const libc::c_void,
      (*ah).ar_hln as size_t,
    ) as *mut libc::c_uchar
  }
  p = mempcpy(
    p as *mut libc::c_void,
    dst_addr as *const libc::c_void,
    4i32 as size_t,
  ) as *mut libc::c_uchar;
  err = sendto(
    3i32,
    buf.as_mut_ptr() as *const libc::c_void,
    p.wrapping_offset_from(buf.as_mut_ptr()) as libc::c_long as size_t,
    0i32,
    __CONST_SOCKADDR_ARG {
      __sockaddr__: HE as *mut sockaddr,
    },
    ::std::mem::size_of::<sockaddr_ll>() as libc::c_ulong as socklen_t,
  ) as libc::c_int;
  if err as libc::c_long == p.wrapping_offset_from(buf.as_mut_ptr()) as libc::c_long {
    (*ptr_to_globals).last = monotonic_us() as libc::c_uint;
    (*ptr_to_globals).sent = (*ptr_to_globals).sent.wrapping_add(1);
    if option_mask32 & UNICASTING as libc::c_int as libc::c_uint == 0 {
      (*ptr_to_globals).brd_sent = (*ptr_to_globals).brd_sent.wrapping_add(1)
    }
  }
  return err;
}
unsafe extern "C" fn finish() -> ! {
  if option_mask32 & QUIET as libc::c_int as libc::c_uint == 0 {
    printf(b"Sent %u probe(s) (%u broadcast(s))\nReceived %u response(s) (%u request(s), %u broadcast(s))\n\x00"
                   as *const u8 as *const libc::c_char,
               (*ptr_to_globals).sent, (*ptr_to_globals).brd_sent,
               (*ptr_to_globals).received, (*ptr_to_globals).req_recv,
               (*ptr_to_globals).brd_recv);
  }
  if option_mask32 & DAD as libc::c_int as libc::c_uint != 0 {
    exit(((*ptr_to_globals).received != 0) as libc::c_int);
  }
  if option_mask32 & UNSOLICITED as libc::c_int as libc::c_uint != 0 {
    exit(0i32);
  }
  exit(((*ptr_to_globals).received == 0) as libc::c_int);
}
unsafe extern "C" fn catcher() {
  let mut now: libc::c_uint = 0;
  now = monotonic_us() as libc::c_uint;
  if (*ptr_to_globals).start == 0i32 as libc::c_uint {
    (*ptr_to_globals).start = now
  }
  if (*ptr_to_globals).count == 0i32
    || (*ptr_to_globals).timeout_us != 0
      && now.wrapping_sub((*ptr_to_globals).start) > (*ptr_to_globals).timeout_us
  {
    finish();
  }
  /* count < 0 means "infinite count" */
  if (*ptr_to_globals).count > 0i32 {
    (*ptr_to_globals).count -= 1
  }
  if (*ptr_to_globals).last == 0i32 as libc::c_uint
    || now.wrapping_sub((*ptr_to_globals).last) > 500000i32 as libc::c_uint
  {
    send_pack(
      &mut (*ptr_to_globals).src,
      &mut (*ptr_to_globals).dst,
      &mut (*ptr_to_globals).me,
      &mut (*ptr_to_globals).he,
    );
    if (*ptr_to_globals).count == 0i32
      && option_mask32 & UNSOLICITED as libc::c_int as libc::c_uint != 0
    {
      finish();
    }
  }
  alarm(1i32 as libc::c_uint);
}
unsafe extern "C" fn recv_pack(
  mut buf: *mut libc::c_uchar,
  mut len: libc::c_int,
  mut FROM: *mut sockaddr_ll,
) {
  let mut ah: *mut arphdr = buf as *mut arphdr;
  let mut p: *mut libc::c_uchar = ah.offset(1) as *mut libc::c_uchar;
  let mut src_ip: in_addr = in_addr { s_addr: 0 };
  let mut dst_ip: in_addr = in_addr { s_addr: 0 };
  /* moves below assume in_addr is 4 bytes big, ensure that */
  /* Filter out wild packets */
  if (*FROM).sll_pkttype as libc::c_int != 0i32
    && (*FROM).sll_pkttype as libc::c_int != 1i32
    && (*FROM).sll_pkttype as libc::c_int != 2i32
  {
    return;
  }
  /* Only these types are recognized */
  if (*ah).ar_op as libc::c_int
    != ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = 1i32 as libc::c_ushort;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh12 = &mut __v;
        let fresh13;
        let fresh14 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh13) : "0"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh12, fresh14))
                         : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh12, fresh14, fresh13);
      }
      __v
    }) as libc::c_int
    && (*ah).ar_op as libc::c_int
      != ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = 2i32 as libc::c_ushort;
        if 0 != 0 {
          __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
            as libc::c_ushort
        } else {
          let fresh15 = &mut __v;
          let fresh16;
          let fresh17 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh16) : "0"
                             (c2rust_asm_casts::AsmCast::cast_in(fresh15, fresh17))
                             : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh15, fresh17, fresh16);
        }
        __v
      }) as libc::c_int
  {
    return;
  }
  /* ARPHRD check and this darned FDDI hack here :-( */
  if (*ah).ar_hrd as libc::c_int
    != ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = (*FROM).sll_hatype;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh18 = &mut __v;
        let fresh19;
        let fresh20 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh19) : "0"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh18, fresh20))
                         : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh18, fresh20, fresh19);
      }
      __v
    }) as libc::c_int
    && ((*FROM).sll_hatype as libc::c_int != 774i32
      || (*ah).ar_hrd as libc::c_int
        != ({
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = 1i32 as libc::c_ushort;
          if 0 != 0 {
            __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
              as libc::c_ushort
          } else {
            let fresh21 = &mut __v;
            let fresh22;
            let fresh23 = __x;
            asm!("rorw $$8, ${0:w}" : "=r" (fresh22) : "0"
                                  (c2rust_asm_casts::AsmCast::cast_in(fresh21, fresh23))
                                  : "cc");
            c2rust_asm_casts::AsmCast::cast_out(fresh21, fresh23, fresh22);
          }
          __v
        }) as libc::c_int)
  {
    return;
  }
  /* Protocol must be IP. */
  if (*ah).ar_pro as libc::c_int
    != ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = 0x800i32 as libc::c_ushort;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh24 = &mut __v;
        let fresh25;
        let fresh26 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh25) : "0"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh24, fresh26))
                         : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh24, fresh26, fresh25);
      }
      __v
    }) as libc::c_int
    || (*ah).ar_pln as libc::c_int != 4i32
    || (*ah).ar_hln as libc::c_int != (*ptr_to_globals).me.sll_halen as libc::c_int
    || len
      < (::std::mem::size_of::<arphdr>() as libc::c_ulong)
        .wrapping_add((2i32 * (4i32 + (*ah).ar_hln as libc::c_int)) as libc::c_ulong)
        as libc::c_int
  {
    return;
  }
  src_ip.s_addr = *(p.offset((*ah).ar_hln as libc::c_int as isize) as *mut bb__aliased_u32);
  dst_ip.s_addr = *(p
    .offset((*ah).ar_hln as libc::c_int as isize)
    .offset(4)
    .offset((*ah).ar_hln as libc::c_int as isize) as *mut bb__aliased_u32);
  if (*ptr_to_globals).dst.s_addr != src_ip.s_addr {
    return;
  }
  if option_mask32 & DAD as libc::c_int as libc::c_uint == 0 {
    if (*ptr_to_globals).src.s_addr != dst_ip.s_addr
      || memcmp(
        p.offset((*ah).ar_hln as libc::c_int as isize).offset(4) as *const libc::c_void,
        &mut (*ptr_to_globals).me.sll_addr as *mut [libc::c_uchar; 8] as *const libc::c_void,
        (*ah).ar_hln as libc::c_ulong,
      ) != 0
    {
      return;
    }
  } else if memcmp(
    p as *const libc::c_void,
    &mut (*ptr_to_globals).me.sll_addr as *mut [libc::c_uchar; 8] as *const libc::c_void,
    (*ptr_to_globals).me.sll_halen as libc::c_ulong,
  ) == 0i32
    || (*ptr_to_globals).src.s_addr != 0 && (*ptr_to_globals).src.s_addr != dst_ip.s_addr
  {
    return;
  }
  if option_mask32 & QUIET as libc::c_int as libc::c_uint == 0 {
    let mut s_printed: libc::c_int = 0i32;
    /* DAD packet was:
      src_ip = 0 (or some src)
      src_hw = ME
      dst_ip = tested address
      dst_hw = <unspec>

      We fail, if receive request/reply with:
      src_ip = tested_address
      src_hw != ME
      if src_ip in request was not zero, check
      also that it matches to dst_ip, otherwise
      dst_ip/dst_hw do not matter.
    */
    //TODO: arping from iputils-s20160308 print upprcase hex in MAC, follow them?
    printf(
      b"%scast re%s from %s [%02x:%02x:%02x:%02x:%02x:%02x]\x00" as *const u8
        as *const libc::c_char,
      if (*FROM).sll_pkttype as libc::c_int == 0i32 {
        b"Uni\x00" as *const u8 as *const libc::c_char
      } else {
        b"Broad\x00" as *const u8 as *const libc::c_char
      },
      if (*ah).ar_op as libc::c_int
        == ({
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = 2i32 as libc::c_ushort;
          if 0 != 0 {
            __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
              as libc::c_ushort
          } else {
            let fresh27 = &mut __v;
            let fresh28;
            let fresh29 = __x;
            asm!("rorw $$8, ${0:w}" : "=r" (fresh28) : "0"
                                    (c2rust_asm_casts::AsmCast::cast_in(fresh27, fresh29))
                                    : "cc");
            c2rust_asm_casts::AsmCast::cast_out(fresh27, fresh29, fresh28);
          }
          __v
        }) as libc::c_int
      {
        b"ply\x00" as *const u8 as *const libc::c_char
      } else {
        b"quest\x00" as *const u8 as *const libc::c_char
      },
      inet_ntoa(src_ip),
      *p.offset(0) as libc::c_int,
      *p.offset(1) as libc::c_int,
      *p.offset(2) as libc::c_int,
      *p.offset(3) as libc::c_int,
      *p.offset(4) as libc::c_int,
      *p.offset(5) as libc::c_int,
    );
    if dst_ip.s_addr != (*ptr_to_globals).src.s_addr {
      printf(
        b"for %s\x00" as *const u8 as *const libc::c_char,
        inet_ntoa(dst_ip),
      );
      s_printed = 1i32
    }
    if memcmp(
      p.offset((*ah).ar_hln as libc::c_int as isize).offset(4) as *const libc::c_void,
      (*ptr_to_globals).me.sll_addr.as_mut_ptr() as *const libc::c_void,
      (*ah).ar_hln as libc::c_ulong,
    ) != 0
    {
      let mut pp: *mut libc::c_uchar = p.offset((*ah).ar_hln as libc::c_int as isize).offset(4);
      if s_printed == 0 {
        printf(b" for\x00" as *const u8 as *const libc::c_char);
      }
      printf(
        b" [%02x:%02x:%02x:%02x:%02x:%02x]\x00" as *const u8 as *const libc::c_char,
        *pp.offset(0) as libc::c_int,
        *pp.offset(1) as libc::c_int,
        *pp.offset(2) as libc::c_int,
        *pp.offset(3) as libc::c_int,
        *pp.offset(4) as libc::c_int,
        *pp.offset(5) as libc::c_int,
      );
    }
    if (*ptr_to_globals).last != 0 {
      let mut diff: libc::c_uint =
        (monotonic_us() as libc::c_uint).wrapping_sub((*ptr_to_globals).last);
      printf(
        b" %u.%03ums\n\x00" as *const u8 as *const libc::c_char,
        diff.wrapping_div(1000i32 as libc::c_uint),
        diff.wrapping_rem(1000i32 as libc::c_uint),
      );
    } else {
      puts(b" UNSOLICITED?\x00" as *const u8 as *const libc::c_char);
    }
    fflush_all();
  }
  (*ptr_to_globals).received = (*ptr_to_globals).received.wrapping_add(1);
  if (*FROM).sll_pkttype as libc::c_int != 0i32 {
    (*ptr_to_globals).brd_recv = (*ptr_to_globals).brd_recv.wrapping_add(1)
  }
  if (*ah).ar_op as libc::c_int
    == ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = 1i32 as libc::c_ushort;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh30 = &mut __v;
        let fresh31;
        let fresh32 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh31) : "0"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh30, fresh32))
                         : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh30, fresh32, fresh31);
      }
      __v
    }) as libc::c_int
  {
    (*ptr_to_globals).req_recv = (*ptr_to_globals).req_recv.wrapping_add(1)
  }
  if option_mask32 & QUIT_ON_REPLY as libc::c_int as libc::c_uint != 0 {
    finish();
  }
  if option_mask32 & BCAST_ONLY as libc::c_int as libc::c_uint == 0 {
    memcpy(
      (*ptr_to_globals).he.sll_addr.as_mut_ptr() as *mut libc::c_void,
      p as *const libc::c_void,
      (*ptr_to_globals).me.sll_halen as libc::c_ulong,
    );
    option_mask32 |= UNICASTING as libc::c_int as libc::c_uint
  };
}
#[no_mangle]
pub unsafe extern "C" fn arping_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut device: *const libc::c_char = b"eth0\x00" as *const u8 as *const libc::c_char;
  let mut source: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut target: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut err_str: *mut libc::c_char = 0 as *mut libc::c_char;
  let ref mut fresh33 =
    *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
      as *mut *mut globals);
  *fresh33 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  (*ptr_to_globals).count = -1i32;
  xmove_fd(xsocket(17i32, SOCK_DGRAM as libc::c_int, 0i32), 3i32);
  // If you ever change BB_SUID_DROP to BB_SUID_REQUIRE,
  // drop suid root privileges here:
  //xsetuid(getuid());
  let mut opt: libc::c_uint = 0;
  let mut str_timeout: *mut libc::c_char = 0 as *mut libc::c_char;
  opt = getopt32(
    argv,
    b"^UDAqfbc:+w:I:s:\x00=1:Df:AU\x00" as *const u8 as *const libc::c_char,
    &mut (*ptr_to_globals).count as *mut libc::c_int,
    &mut str_timeout as *mut *mut libc::c_char,
    &mut device as *mut *const libc::c_char,
    &mut source as *mut *mut libc::c_char,
  );
  if opt & TIMEOUT as libc::c_int as libc::c_uint != 0 {
    (*ptr_to_globals).timeout_us = xatou_range(
      str_timeout,
      0i32 as libc::c_uint,
      (2147483647i32 / 2000000i32) as libc::c_uint,
    )
    .wrapping_mul(1000000i32 as libc::c_uint)
    .wrapping_add(500000i32 as libc::c_uint)
  }
  target = *argv.offset(optind as isize);
  err_str = xasprintf(
    b"interface %s %%s\x00" as *const u8 as *const libc::c_char,
    device,
  );
  xfunc_error_retval = 2i32 as u8;
  /*memset(&G.ifr, 0, sizeof(G.ifr)); - zeroed by INIT_G */
  strncpy_IFNAMSIZ(
    (*ptr_to_globals).ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
    device,
  );
  ioctl_or_perror_and_die(
    3i32,
    0x8933i32 as libc::c_uint,
    &mut (*ptr_to_globals).ifr as *mut ifreq as *mut libc::c_void,
    err_str,
    b"not found\x00" as *const u8 as *const libc::c_char,
  );
  (*ptr_to_globals).me.sll_ifindex = (*ptr_to_globals).ifr.ifr_ifru.ifru_ivalue;
  bb_xioctl(
    3i32,
    0x8913i32 as libc::c_uint,
    &mut (*ptr_to_globals).ifr as *mut ifreq as *mut libc::c_char as *mut libc::c_void,
    b"SIOCGIFFLAGS\x00" as *const u8 as *const libc::c_char,
  );
  if (*ptr_to_globals).ifr.ifr_ifru.ifru_flags as libc::c_int & IFF_UP as libc::c_int == 0 {
    bb_error_msg_and_die(err_str, b"is down\x00" as *const u8 as *const libc::c_char);
  }
  if (*ptr_to_globals).ifr.ifr_ifru.ifru_flags as libc::c_int
    & (IFF_NOARP as libc::c_int | IFF_LOOPBACK as libc::c_int)
    != 0
  {
    bb_error_msg(
      err_str,
      b"is not ARPable\x00" as *const u8 as *const libc::c_char,
    );
    /* exit 0 if DAD, else exit 2 */
    return (!option_mask32 & DAD as libc::c_int as libc::c_uint) as libc::c_int;
  }
  /* if (!inet_aton(target, &dst)) - not needed */
  let mut lsa: *mut len_and_sockaddr = 0 as *mut len_and_sockaddr;
  lsa = xhost_and_af2sockaddr(target, 0i32, 2i32 as sa_family_t);
  (*ptr_to_globals).dst = (*lsa).u.sin.sin_addr;
  if !source.is_null() && inet_aton(source, &mut (*ptr_to_globals).src) == 0 {
    bb_error_msg_and_die(
      b"invalid source address %s\x00" as *const u8 as *const libc::c_char,
      source,
    );
  }
  if option_mask32 & (DAD as libc::c_int | UNSOLICITED as libc::c_int) as libc::c_uint
    == UNSOLICITED as libc::c_int as libc::c_uint
    && (*ptr_to_globals).src.s_addr == 0i32 as libc::c_uint
  {
    (*ptr_to_globals).src = (*ptr_to_globals).dst
  }
  if option_mask32 & DAD as libc::c_int as libc::c_uint == 0 || (*ptr_to_globals).src.s_addr != 0 {
    /*struct sockaddr_in probe_saddr;*/
    let mut probe_fd: libc::c_int = xsocket(2i32, SOCK_DGRAM as libc::c_int, 0i32);
    setsockopt_bindtodevice(probe_fd, device);
    /*memset(&G.probe_saddr, 0, sizeof(G.probe_saddr)); - zeroed by INIT_G */
    (*ptr_to_globals).probe_saddr.sin_family = 2i32 as sa_family_t; /* !(option_mask32 & DAD) case */
    if (*ptr_to_globals).src.s_addr != 0 {
      /* Check that this is indeed our IP */
      (*ptr_to_globals).probe_saddr.sin_addr = (*ptr_to_globals).src;
      xbind(
        probe_fd,
        &mut (*ptr_to_globals).probe_saddr as *mut sockaddr_in as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
      );
    } else {
      /* Find IP address on this iface */
      (*ptr_to_globals).probe_saddr.sin_port = {
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = 1025i32 as libc::c_ushort;
        if 0 != 0 {
          __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
            as libc::c_ushort
        } else {
          let fresh34 = &mut __v;
          let fresh35;
          let fresh36 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh35) : "0"
                              (c2rust_asm_casts::AsmCast::cast_in(fresh34, fresh36))
                              : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh34, fresh36, fresh35);
        }
        __v
      };
      (*ptr_to_globals).probe_saddr.sin_addr = (*ptr_to_globals).dst;
      if setsockopt_SOL_SOCKET_1(probe_fd, 5i32) != 0i32 {
        bb_perror_msg(
          b"setsockopt(%s)\x00" as *const u8 as *const libc::c_char,
          b"SO_DONTROUTE\x00" as *const u8 as *const libc::c_char,
        );
      }
      xconnect(
        probe_fd,
        &mut (*ptr_to_globals).probe_saddr as *mut sockaddr_in as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
      );
      bb_getsockname(
        probe_fd,
        &mut (*ptr_to_globals).probe_saddr as *mut sockaddr_in as *mut sockaddr
          as *mut libc::c_void,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
      );
      if (*ptr_to_globals).probe_saddr.sin_family as libc::c_int != 2i32 {
        bb_simple_error_msg_and_die(
          b"no IP address configured\x00" as *const u8 as *const libc::c_char,
        );
      }
      (*ptr_to_globals).src = (*ptr_to_globals).probe_saddr.sin_addr
    }
    close(probe_fd);
  }
  (*ptr_to_globals).me.sll_family = 17i32 as libc::c_ushort;
  //me.sll_ifindex = ifindex; - done before
  (*ptr_to_globals).me.sll_protocol = {
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = 0x806i32 as libc::c_ushort;
    if 0 != 0 {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh37 = &mut __v;
      let fresh38;
      let fresh39 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh38) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh37, fresh39)) :
                      "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh37, fresh39, fresh38);
    }
    __v
  };
  xbind(
    3i32,
    &mut (*ptr_to_globals).me as *mut sockaddr_ll as *mut sockaddr,
    ::std::mem::size_of::<sockaddr_ll>() as libc::c_ulong as socklen_t,
  );
  bb_getsockname(
    3i32,
    &mut (*ptr_to_globals).me as *mut sockaddr_ll as *mut sockaddr as *mut libc::c_void,
    ::std::mem::size_of::<sockaddr_ll>() as libc::c_ulong as socklen_t,
  );
  //never happens:
  //if (getsockname(sock_fd, (struct sockaddr *) &me, &alen) == -1)
  //	bb_perror_msg_and_die("getsockname");
  if (*ptr_to_globals).me.sll_halen as libc::c_int == 0i32 {
    bb_error_msg(
      err_str,
      b"is not ARPable (no ll address)\x00" as *const u8 as *const libc::c_char,
    );
    /* exit 0 if DAD, else exit 2 */
    return (!option_mask32 & DAD as libc::c_int as libc::c_uint) as libc::c_int;
  }
  (*ptr_to_globals).he = (*ptr_to_globals).me;
  memset(
    (*ptr_to_globals).he.sll_addr.as_mut_ptr() as *mut libc::c_void,
    -1i32,
    (*ptr_to_globals).he.sll_halen as libc::c_ulong,
  );
  if option_mask32 & QUIET as libc::c_int as libc::c_uint == 0 {
    /* inet_ntoa uses static storage, can't use in same printf */
    printf(
      b"ARPING %s\x00" as *const u8 as *const libc::c_char,
      inet_ntoa((*ptr_to_globals).dst),
    );
    printf(
      b" from %s %s\n\x00" as *const u8 as *const libc::c_char,
      inet_ntoa((*ptr_to_globals).src),
      device,
    );
  }
  /*sigemptyset(&G.sset); - zeroed by INIT_G */
  sigaddset(&mut (*ptr_to_globals).sset, 14i32);
  sigaddset(&mut (*ptr_to_globals).sset, 2i32);
  signal_SA_RESTART_empty_mask(
    2i32,
    ::std::mem::transmute::<
      Option<unsafe extern "C" fn() -> !>,
      Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    >(Some(finish as unsafe extern "C" fn() -> !)),
  );
  signal_SA_RESTART_empty_mask(
    14i32,
    ::std::mem::transmute::<
      Option<unsafe extern "C" fn() -> ()>,
      Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    >(Some(catcher as unsafe extern "C" fn() -> ())),
  );
  /* Send the first packet, arm ALRM */
  catcher();
  loop {
    let mut from: sockaddr_ll = sockaddr_ll {
      sll_family: 0,
      sll_protocol: 0,
      sll_ifindex: 0,
      sll_hatype: 0,
      sll_pkttype: 0,
      sll_halen: 0,
      sll_addr: [0; 8],
    };
    let mut alen: socklen_t = ::std::mem::size_of::<sockaddr_ll>() as libc::c_ulong as socklen_t;
    let mut cc: libc::c_int = 0;
    /* Unblock SIGALRM so that the previously called alarm()
     * can prevent recvfrom from blocking forever in case the
     * inherited procmask is blocking SIGALRM.
     */
    sigprocmask(1i32, &mut (*ptr_to_globals).sset, 0 as *mut sigset_t);
    cc = recvfrom(
      3i32,
      (*ptr_to_globals).packet.as_mut_ptr() as *mut libc::c_void,
      ::std::mem::size_of::<[libc::c_uchar; 4096]>() as libc::c_ulong,
      0i32,
      __SOCKADDR_ARG {
        __sockaddr__: &mut from as *mut sockaddr_ll as *mut sockaddr,
      },
      &mut alen,
    ) as libc::c_int;
    /* Don't allow SIGALRMs while we process the reply */
    sigprocmask(0i32, &mut (*ptr_to_globals).sset, 0 as *mut sigset_t);
    if cc < 0i32 {
      bb_simple_perror_msg(b"recvfrom\x00" as *const u8 as *const libc::c_char);
    } else {
      recv_pack((*ptr_to_globals).packet.as_mut_ptr(), cc, &mut from);
    }
  }
}

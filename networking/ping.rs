use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
use libc;

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
  fn __cmsg_nxthdr(__mhdr: *mut msghdr, __cmsg: *mut cmsghdr) -> *mut cmsghdr;
  #[no_mangle]
  fn socket(__domain: libc::c_int, __type: libc::c_int, __protocol: libc::c_int) -> libc::c_int;
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
  fn recvmsg(__fd: libc::c_int, __message: *mut msghdr, __flags: libc::c_int) -> ssize_t;
  #[no_mangle]
  fn setsockopt(
    __fd: libc::c_int,
    __level: libc::c_int,
    __optname: libc::c_int,
    __optval: *const libc::c_void,
    __optlen: socklen_t,
  ) -> libc::c_int;
  #[no_mangle]
  fn if_nametoindex(__ifname: *const libc::c_char) -> libc::c_uint;
  #[no_mangle]
  fn exit(_: libc::c_int) -> !;
  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn getpid() -> __pid_t;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn setitimer(
    __which: __itimer_which_t,
    __new: *const itimerval,
    __old: *mut itimerval,
  ) -> libc::c_int;
  #[no_mangle]
  fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
  #[no_mangle]
  fn inet_ntop(
    __af: libc::c_int,
    __cp: *const libc::c_void,
    __buf: *mut libc::c_char,
    __len: socklen_t,
  ) -> *const libc::c_char;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn monotonic_us() -> libc::c_ulonglong;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
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
  fn setsockopt_int(
    fd: libc::c_int,
    level: libc::c_int,
    optname: libc::c_int,
    optval: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn setsockopt_1(fd: libc::c_int, level: libc::c_int, optname: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn setsockopt_SOL_SOCKET_int(
    fd: libc::c_int,
    optname: libc::c_int,
    optval: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn setsockopt_broadcast(fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn setsockopt_bindtodevice(fd: libc::c_int, iface: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xdotted2sockaddr(host: *const libc::c_char, port: libc::c_int) -> *mut len_and_sockaddr;
  #[no_mangle]
  fn xhost_and_af2sockaddr(
    host: *const libc::c_char,
    port: libc::c_int,
    af: sa_family_t,
  ) -> *mut len_and_sockaddr;
  #[no_mangle]
  fn xmalloc_sockaddr2dotted_noport(sa: *const sockaddr) -> *mut libc::c_char;
  #[no_mangle]
  fn inet_cksum(addr: *mut u16, len: libc::c_int) -> u16;
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn xstrtou_range(
    str: *const libc::c_char,
    b: libc::c_int,
    l: libc::c_uint,
    u: libc::c_uint,
  ) -> libc::c_uint;
  #[no_mangle]
  fn xatou16(numstr: *const libc::c_char) -> u16;
  #[no_mangle]
  fn parse_duration_str(str: *mut libc::c_char) -> duration_t;
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  static bb_msg_can_not_create_raw_socket: [libc::c_char; 0];
  #[no_mangle]
  static bb_msg_perm_denied_are_you_root: [libc::c_char; 0];
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

use crate::librb::__pid_t;
use crate::librb::__suseconds_t;
use crate::librb::__time_t;

pub type __socklen_t = libc::c_uint;
use crate::librb::int32_t;
use crate::librb::size_t;
use crate::librb::ssize_t;
use libc::timeval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
  pub iov_base: *mut libc::c_void,
  pub iov_len: size_t,
}
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
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
  pub sa_family: sa_family_t,
  pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msghdr {
  pub msg_name: *mut libc::c_void,
  pub msg_namelen: socklen_t,
  pub msg_iov: *mut iovec,
  pub msg_iovlen: size_t,
  pub msg_control: *mut libc::c_void,
  pub msg_controllen: size_t,
  pub msg_flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmsghdr {
  pub cmsg_len: size_t,
  pub cmsg_level: libc::c_int,
  pub cmsg_type: libc::c_int,
  pub __cmsg_data: [libc::c_uchar; 0],
}
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_0 = 256;
pub const IPPROTO_RAW: C2RustUnnamed_0 = 255;
pub const IPPROTO_MPLS: C2RustUnnamed_0 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_0 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_0 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_0 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_0 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_0 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_0 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_0 = 92;
pub const IPPROTO_AH: C2RustUnnamed_0 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_0 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_0 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_0 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_0 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_0 = 33;
pub const IPPROTO_TP: C2RustUnnamed_0 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_0 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_0 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_0 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_0 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_0 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_0 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_0 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_0 = 1;
pub const IPPROTO_IP: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IPPROTO_MH: C2RustUnnamed_1 = 135;
pub const IPPROTO_DSTOPTS: C2RustUnnamed_1 = 60;
pub const IPPROTO_NONE: C2RustUnnamed_1 = 59;
pub const IPPROTO_ICMPV6: C2RustUnnamed_1 = 58;
pub const IPPROTO_FRAGMENT: C2RustUnnamed_1 = 44;
pub const IPPROTO_ROUTING: C2RustUnnamed_1 = 43;
pub const IPPROTO_HOPOPTS: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct iphdr {
  #[bitfield(name = "ihl", ty = "libc::c_uint", bits = "0..=3")]
  #[bitfield(name = "version", ty = "libc::c_uint", bits = "4..=7")]
  pub ihl_version: [u8; 1],
  pub tos: u8,
  pub tot_len: u16,
  pub id: u16,
  pub frag_off: u16,
  pub ttl: u8,
  pub protocol: u8,
  pub check: u16,
  pub saddr: u32,
  pub daddr: u32,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct ip {
  #[bitfield(name = "ip_hl", ty = "libc::c_uint", bits = "0..=3")]
  #[bitfield(name = "ip_v", ty = "libc::c_uint", bits = "4..=7")]
  pub ip_hl_ip_v: [u8; 1],
  pub ip_tos: u8,
  pub ip_len: libc::c_ushort,
  pub ip_id: libc::c_ushort,
  pub ip_off: libc::c_ushort,
  pub ip_ttl: u8,
  pub ip_p: u8,
  pub ip_sum: libc::c_ushort,
  pub ip_src: in_addr,
  pub ip_dst: in_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp_ra_addr {
  pub ira_addr: u32,
  pub ira_preference: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp {
  pub icmp_type: u8,
  pub icmp_code: u8,
  pub icmp_cksum: u16,
  pub icmp_hun: C2RustUnnamed_5,
  pub icmp_dun: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
  pub id_ts: C2RustUnnamed_4,
  pub id_ip: C2RustUnnamed_3,
  pub id_radv: icmp_ra_addr,
  pub id_mask: u32,
  pub id_data: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
  pub idi_ip: ip,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
  pub its_otime: u32,
  pub its_rtime: u32,
  pub its_ttime: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
  pub ih_pptr: libc::c_uchar,
  pub ih_gwaddr: in_addr,
  pub ih_idseq: ih_idseq,
  pub ih_void: u32,
  pub ih_pmtu: ih_pmtu,
  pub ih_rtradv: ih_rtradv,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ih_rtradv {
  pub irt_num_addrs: u8,
  pub irt_wpa: u8,
  pub irt_lifetime: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ih_pmtu {
  pub ipm_void: u16,
  pub ipm_nextmtu: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ih_idseq {
  pub icd_id: u16,
  pub icd_seq: u16,
}
pub type bb__aliased_int = libc::c_int;
pub type bb__aliased_u32 = u32;
use crate::librb::signal::__sighandler_t;
pub type __itimer_which = libc::c_uint;
pub const ITIMER_PROF: __itimer_which = 2;
pub const ITIMER_VIRTUAL: __itimer_which = 1;
pub const ITIMER_REAL: __itimer_which = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct itimerval {
  pub it_interval: timeval,
  pub it_value: timeval,
}
pub type __itimer_which_t = __itimer_which;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct len_and_sockaddr {
  pub len: socklen_t,
  pub u: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
  pub sa: sockaddr,
  pub sin: sockaddr_in,
  pub sin6: sockaddr_in6,
}
pub type duration_t = libc::c_double;
//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub if_index: libc::c_int,
  pub str_I: *mut libc::c_char,
  pub source_lsa: *mut len_and_sockaddr,
  pub datalen: libc::c_uint,
  pub pingcount: libc::c_uint,
  pub opt_ttl: libc::c_uint,
  pub ntransmitted: libc::c_ulong,
  pub nreceived: libc::c_ulong,
  pub nrepeats: libc::c_ulong,
  pub myid: u16,
  pub pattern: u8,
  pub tmin: libc::c_uint,
  pub tmax: libc::c_uint,
  pub tsum: libc::c_ulonglong,
  pub cur_us: libc::c_uint,
  pub deadline_us: libc::c_uint,
  pub interval_us: libc::c_uint,
  pub timeout: libc::c_uint,
  pub sizeof_rcv_packet: libc::c_uint,
  pub rcv_packet: *mut libc::c_char,
  pub snd_packet: *mut libc::c_void,
  pub hostname: *const libc::c_char,
  pub dotted: *const libc::c_char,
  pub pingaddr: C2RustUnnamed_7,
  pub rcvd_tbl: [libc::c_uchar; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
  pub sa: sockaddr,
  pub sin: sockaddr_in,
  pub sin6: sockaddr_in6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp6_filter {
  pub icmp6_filt: [u32; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp6_hdr {
  pub icmp6_type: u8,
  pub icmp6_code: u8,
  pub icmp6_cksum: u16,
  pub icmp6_dataun: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
  pub icmp6_un_data32: [u32; 1],
  pub icmp6_un_data16: [u16; 2],
  pub icmp6_un_data8: [u8; 4],
}
pub type C2RustUnnamed_9 = libc::c_uint;
pub const pingsock: C2RustUnnamed_9 = 0;
pub const PINGINTERVAL: C2RustUnnamed_9 = 1;
pub const MAXWAIT: C2RustUnnamed_9 = 10;
pub const MAX_DUP_CHK: C2RustUnnamed_9 = 1024;
pub const MAXICMPLEN: C2RustUnnamed_9 = 76;
pub const MAXIPLEN: C2RustUnnamed_9 = 60;
pub const DEFDATALEN: C2RustUnnamed_9 = 56;
pub type C2RustUnnamed_10 = libc::c_uint;
pub const OPT_IPV6: C2RustUnnamed_10 = 8192;
pub const OPT_IPV4: C2RustUnnamed_10 = 4096;
pub const OPT_i: C2RustUnnamed_10 = 2048;
pub const OPT_p: C2RustUnnamed_10 = 1024;
pub const OPT_I: C2RustUnnamed_10 = 256;
pub const OPT_W: C2RustUnnamed_10 = 128;
pub const OPT_w: C2RustUnnamed_10 = 64;
pub const OPT_t: C2RustUnnamed_10 = 32;
pub const OPT_s: C2RustUnnamed_10 = 16;
pub const OPT_c: C2RustUnnamed_10 = 8;
pub const OPT_A: C2RustUnnamed_10 = 4;
pub const OPT_VERBOSE: C2RustUnnamed_10 = 2;
pub const OPT_QUIET: C2RustUnnamed_10 = 1;
unsafe extern "C" fn create_icmp_socket(mut lsa: *mut len_and_sockaddr) {
  let mut sock: libc::c_int = 0;
  if (*lsa).u.sa.sa_family as libc::c_int == 10i32 {
    sock = socket(
      10i32,
      SOCK_RAW as libc::c_int,
      IPPROTO_ICMPV6 as libc::c_int,
    )
  } else {
    sock = socket(2i32, SOCK_RAW as libc::c_int, 1i32)
  }
  if sock < 0i32 {
    if *bb_errno == 1i32 {
      bb_simple_error_msg_and_die(bb_msg_perm_denied_are_you_root.as_ptr());
    }
    bb_simple_perror_msg_and_die(bb_msg_can_not_create_raw_socket.as_ptr());
  }
  xmove_fd(sock, pingsock as libc::c_int);
}
unsafe extern "C" fn print_stats_and_exit(mut _junk: libc::c_int) -> ! {
  let mut ul: libc::c_ulong = 0;
  let mut nrecv: libc::c_ulong = 0;
  signal(
    2i32,
    ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
  );
  nrecv = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nreceived;
  printf(
    b"\n--- %s ping statistics ---\n%lu packets transmitted, %lu packets received, \x00"
      as *const u8 as *const libc::c_char,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hostname,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ntransmitted,
    nrecv,
  );
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nrepeats != 0 {
    printf(
      b"%lu duplicates, \x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nrepeats,
    );
  }
  ul = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ntransmitted;
  if ul != 0i32 as libc::c_ulong {
    ul = ul
      .wrapping_sub(nrecv)
      .wrapping_mul(100i32 as libc::c_ulong)
      .wrapping_div(ul)
  }
  printf(
    b"%lu%% packet loss\n\x00" as *const u8 as *const libc::c_char,
    ul,
  );
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tmin
    != (2147483647i32 as libc::c_uint)
      .wrapping_mul(2u32)
      .wrapping_add(1u32)
  {
    let mut tavg: libc::c_uint = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .tsum
      .wrapping_div(
        nrecv.wrapping_add((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nrepeats)
          as libc::c_ulonglong,
      ) as libc::c_uint;
    printf(
      b"round-trip min/avg/max = %u.%03u/%u.%03u/%u.%03u ms\n\x00" as *const u8
        as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .tmin
        .wrapping_div(1000i32 as libc::c_uint),
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .tmin
        .wrapping_rem(1000i32 as libc::c_uint),
      tavg.wrapping_div(1000i32 as libc::c_uint),
      tavg.wrapping_rem(1000i32 as libc::c_uint),
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .tmax
        .wrapping_div(1000i32 as libc::c_uint),
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .tmax
        .wrapping_rem(1000i32 as libc::c_uint),
    );
  }
  /* if condition is true, exit with 1 -- 'failure' */
  exit(
    (nrecv == 0i32 as libc::c_ulong
      || (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).deadline_us != 0
        && nrecv < (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pingcount as libc::c_ulong)
      as libc::c_int,
  );
}
unsafe extern "C" fn sendping_tail(
  mut sp: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
  mut size_pkt: libc::c_int,
) {
  let mut sz: libc::c_int = 0;
  let ref mut fresh0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rcvd_tbl
    [((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ntransmitted as u16 as libc::c_int
      % MAX_DUP_CHK as libc::c_int
      >> 3i32) as usize];
  *fresh0 = (*fresh0 as libc::c_int
    & !(1i32
      << ((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ntransmitted as u16 as libc::c_int
        % MAX_DUP_CHK as libc::c_int
        & 7i32))) as libc::c_uchar;
  let ref mut fresh1 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ntransmitted;
  *fresh1 = (*fresh1).wrapping_add(1);
  size_pkt = (size_pkt as libc::c_uint)
    .wrapping_add((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).datalen)
    as libc::c_int as libc::c_int;
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).deadline_us != 0 {
    let mut n: libc::c_uint = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .cur_us
      .wrapping_sub((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).deadline_us);
    if n as libc::c_int >= 0i32 {
      print_stats_and_exit(0i32);
    }
  }
  /* sizeof(pingaddr) can be larger than real sa size, but I think
   * it doesn't matter */
  sz = xsendto(
    pingsock as libc::c_int,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).snd_packet,
    size_pkt as size_t,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .pingaddr
      .sa,
    ::std::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong as socklen_t,
  ) as libc::c_int; /* -c NN, and all NN are sent */
  if sz != size_pkt {
    bb_simple_error_msg_and_die(b"write error\x00" as *const u8 as *const libc::c_char);
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pingcount == 0i32 as libc::c_uint
    || (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ntransmitted
      < (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pingcount as libc::c_ulong
  {
    let mut i: itimerval = itimerval {
      it_interval: timeval {
        tv_sec: 0,
        tv_usec: 0,
      },
      it_value: timeval {
        tv_sec: 0,
        tv_usec: 0,
      },
    };
    signal(14i32, sp);
    /* Didn't send all pings yet - schedule next in -i SEC interval */
    i.it_interval.tv_sec = 0i32 as __time_t;
    i.it_interval.tv_usec = 0i32 as __suseconds_t;
    i.it_value.tv_sec = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .interval_us
      .wrapping_div(1000000i32 as libc::c_uint) as __time_t;
    i.it_value.tv_usec = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .interval_us
      .wrapping_rem(1000000i32 as libc::c_uint) as __suseconds_t;
    setitimer(ITIMER_REAL, &mut i, 0 as *mut itimerval);
  } else {
    /*ualarm(G.interval_us, 0); - does not work for >=1sec on some libc */
    /* Wait for the last ping to come back.
     * -W timeout: wait for a response in seconds.
     * Affects only timeout in absence of any responses,
     * otherwise ping waits for two RTTs. */
    let mut expire: libc::c_uint = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).timeout;
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nreceived != 0 {
      /* approx. 2*tmax, in seconds (2 RTT) */
      expire = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .tmax
        .wrapping_div((512i32 * 1024i32) as libc::c_uint);
      if expire == 0i32 as libc::c_uint {
        expire = 1i32 as libc::c_uint
      }
    }
    signal(
      14i32,
      ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_int) -> !>, __sighandler_t>(
        Some(print_stats_and_exit as unsafe extern "C" fn(_: libc::c_int) -> !),
      ),
    );
    alarm(expire);
  };
}
unsafe extern "C" fn sendping4(mut _junk: libc::c_int) {
  let mut pkt: *mut icmp =
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).snd_packet as *mut icmp;
  memset(
    pkt as *mut libc::c_void,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pattern as libc::c_int,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .datalen
      .wrapping_add(8i32 as libc::c_uint)
      .wrapping_add(4i32 as libc::c_uint) as libc::c_ulong,
  );
  (*pkt).icmp_type = 8i32 as u8;
  /*pkt->icmp_code = 0;*/
  (*pkt).icmp_cksum = 0i32 as u16; /* cksum is calculated with this field set to 0 */
  (*pkt).icmp_hun.ih_idseq.icd_seq = {
    let mut __v: libc::c_ushort = 0; /* don't ++ here, it can be a macro */
    let mut __x: libc::c_ushort =
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ntransmitted as libc::c_ushort;
    if 0 != 0 {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh2 = &mut __v;
      let fresh3;
      let fresh4 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh3) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh2, fresh4)) :
                      "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh2, fresh4, fresh3);
    }
    __v
  };
  (*pkt).icmp_hun.ih_idseq.icd_id = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).myid;
  /* If datalen < 4, we store timestamp _past_ the packet,
   * but it's ok - we allocated 4 extra bytes in xzalloc() just in case.
   */
  /*if (datalen >= 4)*/
  /* No hton: we'll read it back on the same machine */
  let ref mut fresh5 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cur_us;
  *fresh5 = monotonic_us() as libc::c_uint;
  *(&mut (*pkt).icmp_dun as *mut C2RustUnnamed_2 as *mut u32) = *fresh5;
  (*pkt).icmp_cksum = inet_cksum(
    pkt as *mut u16,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .datalen
      .wrapping_add(8i32 as libc::c_uint) as libc::c_int,
  );
  sendping_tail(
    Some(sendping4 as unsafe extern "C" fn(_: libc::c_int) -> ()),
    8i32,
  );
}
unsafe extern "C" fn sendping6(mut _junk: libc::c_int) {
  let mut pkt: *mut icmp6_hdr =
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).snd_packet as *mut icmp6_hdr;
  memset(
    pkt as *mut libc::c_void,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pattern as libc::c_int,
    ((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).datalen as libc::c_ulong)
      .wrapping_add(::std::mem::size_of::<icmp6_hdr>() as libc::c_ulong)
      .wrapping_add(4i32 as libc::c_ulong),
  );
  (*pkt).icmp6_type = 128i32 as u8;
  /*pkt->icmp6_code = 0;*/
  /*pkt->icmp6_cksum = 0;*/
  (*pkt).icmp6_dataun.icmp6_un_data16[1] = {
    let mut __v: libc::c_ushort = 0; /* don't ++ here, it can be a macro */
    let mut __x: libc::c_ushort =
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ntransmitted as libc::c_ushort;
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
  (*pkt).icmp6_dataun.icmp6_un_data16[0] = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).myid;
  /*if (datalen >= 4)*/
  let ref mut fresh9 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cur_us;
  *fresh9 = monotonic_us() as libc::c_uint;
  *(&mut *(*pkt).icmp6_dataun.icmp6_un_data8.as_mut_ptr().offset(4) as *mut u8
    as *mut bb__aliased_u32) = *fresh9;
  //TODO? pkt->icmp_cksum = inet_cksum(...);
  sendping_tail(
    Some(sendping6 as unsafe extern "C" fn(_: libc::c_int) -> ()),
    ::std::mem::size_of::<icmp6_hdr>() as libc::c_ulong as libc::c_int,
  );
}
unsafe extern "C" fn icmp_type_name(mut id: libc::c_int) -> *const libc::c_char {
  match id {
    0 => return b"Echo Reply\x00" as *const u8 as *const libc::c_char,
    3 => return b"Destination Unreachable\x00" as *const u8 as *const libc::c_char,
    4 => return b"Source Quench\x00" as *const u8 as *const libc::c_char,
    5 => return b"Redirect (change route)\x00" as *const u8 as *const libc::c_char,
    8 => return b"Echo Request\x00" as *const u8 as *const libc::c_char,
    11 => return b"Time Exceeded\x00" as *const u8 as *const libc::c_char,
    12 => return b"Parameter Problem\x00" as *const u8 as *const libc::c_char,
    13 => return b"Timestamp Request\x00" as *const u8 as *const libc::c_char,
    14 => return b"Timestamp Reply\x00" as *const u8 as *const libc::c_char,
    15 => return b"Information Request\x00" as *const u8 as *const libc::c_char,
    16 => return b"Information Reply\x00" as *const u8 as *const libc::c_char,
    17 => return b"Address Mask Request\x00" as *const u8 as *const libc::c_char,
    18 => return b"Address Mask Reply\x00" as *const u8 as *const libc::c_char,
    _ => return b"unknown ICMP type\x00" as *const u8 as *const libc::c_char,
  };
}
/* RFC3542 changed some definitions from RFC2292 for no good reason, whee!
 * the newer 3542 uses a MLD_ prefix where as 2292 uses ICMP6_ prefix */
unsafe extern "C" fn icmp6_type_name(mut id: libc::c_int) -> *const libc::c_char {
  match id {
    1 => return b"Destination Unreachable\x00" as *const u8 as *const libc::c_char,
    2 => return b"Packet too big\x00" as *const u8 as *const libc::c_char,
    3 => return b"Time Exceeded\x00" as *const u8 as *const libc::c_char,
    4 => return b"Parameter Problem\x00" as *const u8 as *const libc::c_char,
    129 => return b"Echo Reply\x00" as *const u8 as *const libc::c_char,
    128 => return b"Echo Request\x00" as *const u8 as *const libc::c_char,
    130 => return b"Listener Query\x00" as *const u8 as *const libc::c_char,
    131 => return b"Listener Report\x00" as *const u8 as *const libc::c_char,
    132 => return b"Listener Reduction\x00" as *const u8 as *const libc::c_char,
    _ => return b"unknown ICMP type\x00" as *const u8 as *const libc::c_char,
  }; /* for gcc */
}
unsafe extern "C" fn unpack_tail(
  mut sz: libc::c_int,
  mut tp: *mut u32,
  mut from_str: *const libc::c_char,
  mut recv_seq: u16,
  mut ttl: libc::c_int,
) {
  let mut b: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
  let mut m: libc::c_uchar = 0;
  let mut dupmsg: *const libc::c_char = b" (DUP!)\x00" as *const u8 as *const libc::c_char;
  let mut triptime: libc::c_uint = 0;
  triptime = triptime;
  if !tp.is_null() {
    /* (int32_t) cast is for hypothetical 64-bit unsigned */
    /* (doesn't hurt 32-bit real-world anyway) */
    triptime = (monotonic_us() as u32).wrapping_sub(*tp) as int32_t as libc::c_uint;
    let ref mut fresh10 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tsum;
    *fresh10 = (*fresh10).wrapping_add(triptime as libc::c_ulonglong);
    if triptime < (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tmin {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tmin = triptime
    }
    if triptime > (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tmax {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tmax = triptime
    }
  }
  b = &mut *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .rcvd_tbl
    .as_mut_ptr()
    .offset((recv_seq as libc::c_int % MAX_DUP_CHK as libc::c_int >> 3i32) as isize)
    as *mut libc::c_uchar;
  m = (1i32 << (recv_seq as libc::c_int % MAX_DUP_CHK as libc::c_int & 7i32)) as libc::c_uchar;
  /*if TST(recv_seq % MAX_DUP_CHK):*/
  if *b as libc::c_int & m as libc::c_int != 0 {
    let ref mut fresh11 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nrepeats;
    *fresh11 = (*fresh11).wrapping_add(1)
  } else {
    /*SET(recv_seq % MAX_DUP_CHK):*/
    *b = (*b as libc::c_int | m as libc::c_int) as libc::c_uchar;
    let ref mut fresh12 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nreceived;
    *fresh12 = (*fresh12).wrapping_add(1);
    dupmsg = dupmsg.offset(7)
  }
  if option_mask32 & OPT_QUIET as libc::c_int as libc::c_uint != 0 {
    return;
  }
  printf(
    b"%d bytes from %s: seq=%u ttl=%d\x00" as *const u8 as *const libc::c_char,
    sz,
    from_str,
    recv_seq as libc::c_int,
    ttl,
  );
  if !tp.is_null() {
    printf(
      b" time=%u.%03u ms\x00" as *const u8 as *const libc::c_char,
      triptime.wrapping_div(1000i32 as libc::c_uint),
      triptime.wrapping_rem(1000i32 as libc::c_uint),
    );
  }
  puts(dupmsg);
  fflush_all();
}
unsafe extern "C" fn unpack4(
  mut buf: *mut libc::c_char,
  mut sz: libc::c_int,
  mut from: *mut sockaddr_in,
) -> libc::c_int {
  let mut icmppkt: *mut icmp = 0 as *mut icmp;
  let mut iphdr: *mut iphdr = 0 as *mut iphdr;
  let mut hlen: libc::c_int = 0;
  /* discard if too short */
  if (sz as libc::c_uint)
    < (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .datalen
      .wrapping_add(8i32 as libc::c_uint)
  {
    return 0i32;
  }
  /* check IP header */
  iphdr = buf as *mut iphdr; /* not our ping */
  hlen = ((*iphdr).ihl() as libc::c_int) << 2i32;
  sz -= hlen;
  icmppkt = buf.offset(hlen as isize) as *mut icmp;
  if (*icmppkt).icmp_hun.ih_idseq.icd_id as libc::c_int
    != (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).myid as libc::c_int
  {
    return 0i32;
  }
  if (*icmppkt).icmp_type as libc::c_int == 0i32 {
    let mut recv_seq: u16 = {
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = (*icmppkt).icmp_hun.ih_idseq.icd_seq;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh13 = &mut __v;
        let fresh14;
        let fresh15 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh14) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh13, fresh15))
                          : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh13, fresh15, fresh14);
      }
      __v
    };
    let mut tp: *mut u32 = 0 as *mut u32;
    if sz as libc::c_ulong
      >= (8i32 as libc::c_ulong).wrapping_add(::std::mem::size_of::<u32>() as libc::c_ulong)
    {
      tp = (*icmppkt).icmp_dun.id_data.as_mut_ptr() as *mut u32
    }
    unpack_tail(
      sz,
      tp,
      inet_ntoa(*(&mut (*from).sin_addr.s_addr as *mut in_addr_t as *mut in_addr)),
      recv_seq,
      (*iphdr).ttl as libc::c_int,
    );
    return 1i32;
  }
  if (*icmppkt).icmp_type as libc::c_int != 8i32 {
    bb_error_msg(
      b"warning: got ICMP %d (%s)\x00" as *const u8 as *const libc::c_char,
      (*icmppkt).icmp_type as libc::c_int,
      icmp_type_name((*icmppkt).icmp_type as libc::c_int),
    );
  }
  return 0i32;
}
unsafe extern "C" fn unpack6(
  mut packet: *mut libc::c_char,
  mut sz: libc::c_int,
  mut from: *mut sockaddr_in6,
  mut hoplimit: libc::c_int,
) -> libc::c_int {
  let mut icmppkt: *mut icmp6_hdr = 0 as *mut icmp6_hdr;
  let mut buf: [libc::c_char; 46] = [0; 46];
  /* discard if too short */
  if (sz as libc::c_ulong)
    < ((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).datalen as libc::c_ulong)
      .wrapping_add(::std::mem::size_of::<icmp6_hdr>() as libc::c_ulong)
  {
    return 0i32;
  } /* not our ping */
  icmppkt = packet as *mut icmp6_hdr;
  if (*icmppkt).icmp6_dataun.icmp6_un_data16[0] as libc::c_int
    != (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).myid as libc::c_int
  {
    return 0i32;
  }
  if (*icmppkt).icmp6_type as libc::c_int == 129i32 {
    let mut recv_seq: u16 = {
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = (*icmppkt).icmp6_dataun.icmp6_un_data16[1];
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
    };
    let mut tp: *mut u32 = 0 as *mut u32;
    if sz as libc::c_ulong
      >= (::std::mem::size_of::<icmp6_hdr>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<u32>() as libc::c_ulong)
    {
      tp = &mut *(*icmppkt)
        .icmp6_dataun
        .icmp6_un_data8
        .as_mut_ptr()
        .offset(4) as *mut u8 as *mut u32
    }
    unpack_tail(
      sz,
      tp,
      inet_ntop(
        10i32,
        &mut (*from).sin6_addr as *mut in6_addr as *const libc::c_void,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong as socklen_t,
      ),
      recv_seq,
      hoplimit,
    );
    return 1i32;
  }
  if (*icmppkt).icmp6_type as libc::c_int != 128i32 {
    bb_error_msg(
      b"warning: got ICMP %d (%s)\x00" as *const u8 as *const libc::c_char,
      (*icmppkt).icmp6_type as libc::c_int,
      icmp6_type_name((*icmppkt).icmp6_type as libc::c_int),
    );
  }
  return 0i32;
}
unsafe extern "C" fn ping4(mut lsa: *mut len_and_sockaddr) {
  let mut sockopt: libc::c_int = 0;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .pingaddr
    .sin = (*lsa).u.sin;
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .source_lsa
    .is_null()
  {
    if setsockopt(
      pingsock as libc::c_int,
      IPPROTO_IP as libc::c_int,
      32i32,
      &mut (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).source_lsa)
        .u
        .sa as *mut sockaddr as *const libc::c_void,
      (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).source_lsa).len,
    ) != 0
    {
      bb_simple_error_msg_and_die(
        b"can\'t set multicast source interface\x00" as *const u8 as *const libc::c_char,
      );
    }
    xbind(
      pingsock as libc::c_int,
      &mut (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).source_lsa)
        .u
        .sa,
      (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).source_lsa).len,
    );
  }
  /* enable broadcast pings */
  setsockopt_broadcast(pingsock as libc::c_int);
  /* set recv buf (needed if we can get lots of responses: flood ping,
   * broadcast ping etc) */
  sockopt = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .datalen
    .wrapping_mul(2i32 as libc::c_uint)
    .wrapping_add((7i32 * 1024i32) as libc::c_uint) as libc::c_int; /* giving it a bit of extra room */
  setsockopt_SOL_SOCKET_int(pingsock as libc::c_int, 8i32, sockopt);
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).opt_ttl != 0i32 as libc::c_uint {
    setsockopt_int(
      pingsock as libc::c_int,
      IPPROTO_IP as libc::c_int,
      2i32,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).opt_ttl as libc::c_int,
    );
    /* above doesn't affect packets sent to bcast IP, so... */
    setsockopt_int(
      pingsock as libc::c_int,
      IPPROTO_IP as libc::c_int,
      33i32,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).opt_ttl as libc::c_int,
    );
  }
  signal(
    2i32,
    ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_int) -> !>, __sighandler_t>(
      Some(print_stats_and_exit as unsafe extern "C" fn(_: libc::c_int) -> !),
    ),
  );
  'c_10978: loop
  /* start the ping's going ... */
  {
    sendping4(0i32);
    loop
    /* listen for replies */
    {
      let mut from: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
      };
      let mut fromlen: socklen_t =
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
      let mut c: libc::c_int = 0;
      c = recvfrom(
        pingsock as libc::c_int,
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rcv_packet as *mut libc::c_void,
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).sizeof_rcv_packet as size_t,
        0i32,
        __SOCKADDR_ARG {
          __sockaddr__: &mut from as *mut sockaddr_in as *mut sockaddr,
        },
        &mut fromlen,
      ) as libc::c_int;
      if c < 0i32 {
        if *bb_errno != 4i32 {
          bb_simple_perror_msg(b"recvfrom\x00" as *const u8 as *const libc::c_char);
        }
      } else {
        c = unpack4(
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rcv_packet,
          c,
          &mut from,
        );
        if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pingcount != 0
          && (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nreceived
            >= (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pingcount as libc::c_ulong
        {
          break 'c_10978;
        }
        if c != 0 && option_mask32 & OPT_A as libc::c_int as libc::c_uint != 0 {
          break;
        }
      }
    }
  }
}
unsafe extern "C" fn ping6(mut lsa: *mut len_and_sockaddr) {
  let mut sockopt: libc::c_int = 0;
  let mut msg: msghdr = msghdr {
    msg_name: 0 as *mut libc::c_void,
    msg_namelen: 0,
    msg_iov: 0 as *mut iovec,
    msg_iovlen: 0,
    msg_control: 0 as *mut libc::c_void,
    msg_controllen: 0,
    msg_flags: 0,
  };
  let mut from: sockaddr_in6 = sockaddr_in6 {
    sin6_family: 0,
    sin6_port: 0,
    sin6_flowinfo: 0,
    sin6_addr: in6_addr {
      __in6_u: C2RustUnnamed {
        __u6_addr8: [0; 16],
      },
    },
    sin6_scope_id: 0,
  };
  let mut iov: iovec = iovec {
    iov_base: 0 as *mut libc::c_void,
    iov_len: 0,
  };
  let mut control_buf: [libc::c_char; 56] = [0; 56];
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .pingaddr
    .sin6 = (*lsa).u.sin6;
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .source_lsa
    .is_null()
  {
    xbind(
      pingsock as libc::c_int,
      &mut (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).source_lsa)
        .u
        .sa,
      (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).source_lsa).len,
    );
  }
  let mut filt: icmp6_filter = icmp6_filter { icmp6_filt: [0; 8] };
  if option_mask32 & OPT_VERBOSE as libc::c_int as libc::c_uint == 0 {
    memset(
      &mut filt as *mut icmp6_filter as *mut libc::c_void,
      0xffi32,
      ::std::mem::size_of::<icmp6_filter>() as libc::c_ulong,
    );
    filt.icmp6_filt[(129i32 >> 5i32) as usize] &= !(1i32 << (129i32 & 31i32)) as libc::c_uint
  } else {
    memset(
      &mut filt as *mut icmp6_filter as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<icmp6_filter>() as libc::c_ulong,
    );
  }
  if setsockopt(
    pingsock as libc::c_int,
    IPPROTO_ICMPV6 as libc::c_int,
    1i32,
    &mut filt as *mut icmp6_filter as *const libc::c_void,
    ::std::mem::size_of::<icmp6_filter>() as libc::c_ulong as socklen_t,
  ) < 0i32
  {
    bb_error_msg_and_die(
      b"setsockopt(%s)\x00" as *const u8 as *const libc::c_char,
      b"ICMP6_FILTER\x00" as *const u8 as *const libc::c_char,
    );
  }
  /*ICMP6_FILTER*/
  /* enable broadcast pings */
  setsockopt_broadcast(pingsock as libc::c_int);
  /* set recv buf (needed if we can get lots of responses: flood ping,
   * broadcast ping etc) */
  sockopt = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .datalen
    .wrapping_mul(2i32 as libc::c_uint)
    .wrapping_add((7i32 * 1024i32) as libc::c_uint) as libc::c_int; /* giving it a bit of extra room */
  setsockopt_SOL_SOCKET_int(pingsock as libc::c_int, 8i32, sockopt);
  sockopt = 2u64 as libc::c_int;
  setsockopt_int(pingsock as libc::c_int, 255i32, 7i32, sockopt);
  /* request ttl info to be returned in ancillary data */
  setsockopt_1(pingsock as libc::c_int, 41i32, 8i32);
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).if_index != 0 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .pingaddr
      .sin6
      .sin6_scope_id = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).if_index as u32
  }
  signal(
    2i32,
    ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_int) -> !>, __sighandler_t>(
      Some(print_stats_and_exit as unsafe extern "C" fn(_: libc::c_int) -> !),
    ),
  );
  msg.msg_name = &mut from as *mut sockaddr_in6 as *mut libc::c_void;
  msg.msg_namelen = ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong as socklen_t;
  msg.msg_iov = &mut iov;
  msg.msg_iovlen = 1i32 as size_t;
  msg.msg_control = control_buf.as_mut_ptr() as *mut libc::c_void;
  iov.iov_base =
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rcv_packet as *mut libc::c_void;
  iov.iov_len = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).sizeof_rcv_packet as size_t;
  'c_11318: loop
  /* start the ping's going ... */
  {
    sendping6(0i32);
    loop
    /* listen for replies */
    {
      let mut c: libc::c_int = 0;
      let mut mp: *mut cmsghdr = 0 as *mut cmsghdr;
      let mut hoplimit: libc::c_int = -1i32;
      msg.msg_controllen = ::std::mem::size_of::<[libc::c_char; 56]>() as libc::c_ulong;
      c = recvmsg(pingsock as libc::c_int, &mut msg, 0i32) as libc::c_int;
      if c < 0i32 {
        if *bb_errno != 4i32 {
          bb_simple_perror_msg(b"recvfrom\x00" as *const u8 as *const libc::c_char);
        }
      } else {
        mp = if msg.msg_controllen >= ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
          msg.msg_control as *mut cmsghdr
        } else {
          0 as *mut cmsghdr
        };
        while !mp.is_null() {
          if (*mp).cmsg_level == 41i32 && (*mp).cmsg_type == 8i32 {
            /* don't check len - we trust the kernel: */
            /* && mp->cmsg_len >= CMSG_LEN(sizeof(int)) */
            /*hoplimit = *(int*)CMSG_DATA(mp); - unaligned access */
            hoplimit = *((*mp).__cmsg_data.as_mut_ptr() as *mut bb__aliased_int)
          }
          mp = __cmsg_nxthdr(&mut msg, mp)
        }
        c = unpack6(
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rcv_packet,
          c,
          &mut from,
          hoplimit,
        );
        if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pingcount != 0
          && (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).nreceived
            >= (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pingcount as libc::c_ulong
        {
          break 'c_11318;
        }
        if c != 0 && option_mask32 & OPT_A as libc::c_int as libc::c_uint != 0 {
          break;
        }
      }
    }
  }
}
unsafe extern "C" fn ping(mut lsa: *mut len_and_sockaddr) {
  printf(
    b"PING %s (%s)\x00" as *const u8 as *const libc::c_char,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hostname,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).dotted,
  );
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .source_lsa
    .is_null()
  {
    printf(
      b" from %s\x00" as *const u8 as *const libc::c_char,
      xmalloc_sockaddr2dotted_noport(
        &mut (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).source_lsa)
          .u
          .sa,
      ),
    );
  }
  printf(
    b": %d data bytes\n\x00" as *const u8 as *const libc::c_char,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).datalen,
  );
  create_icmp_socket(lsa);
  /* untested whether "-I addr" really works for IPv6: */
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .str_I
    .is_null()
  {
    setsockopt_bindtodevice(
      pingsock as libc::c_int,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).str_I,
    );
  }
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).sizeof_rcv_packet =
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .datalen
      .wrapping_add(MAXIPLEN as libc::c_int as libc::c_uint)
      .wrapping_add(MAXICMPLEN as libc::c_int as libc::c_uint);
  let ref mut fresh19 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).rcv_packet;
  *fresh19 =
    xzalloc((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).sizeof_rcv_packet as size_t)
      as *mut libc::c_char;
  if (*lsa).u.sa.sa_family as libc::c_int == 10i32 {
    /* +4 reserves a place for timestamp, which may end up sitting
     * _after_ packet. Saves one if() - see sendping4/6() */
    let ref mut fresh20 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).snd_packet; // -s
    *fresh20 = xzalloc(
      ((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).datalen as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<icmp6_hdr>() as libc::c_ulong)
        .wrapping_add(4i32 as libc::c_ulong),
    );
    ping6(lsa);
  } else {
    let ref mut fresh21 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).snd_packet;
    *fresh21 = xzalloc(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .datalen
        .wrapping_add(8i32 as libc::c_uint)
        .wrapping_add(4i32 as libc::c_uint) as size_t,
    );
    ping4(lsa);
  };
}
unsafe extern "C" fn common_ping_main(
  mut opt: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut lsa: *mut len_and_sockaddr = 0 as *mut len_and_sockaddr;
  let mut str_s: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut str_p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut str_i: *mut libc::c_char =
    b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
  let mut interval: duration_t = 0.;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).datalen =
    DEFDATALEN as libc::c_int as libc::c_uint;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).timeout =
    MAXWAIT as libc::c_int as libc::c_uint;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tmin = (2147483647i32 as libc::c_uint)
    .wrapping_mul(2u32)
    .wrapping_add(1u32);
  opt = (opt as libc::c_uint
    | getopt32(
      argv,
      b"^qvAc:+s:t:+w:+W:+I:np:i:46\x00=1:q--v:v--q\x00" as *const u8 as *const libc::c_char,
      &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pingcount as *mut libc::c_uint,
      &mut str_s as *mut *mut libc::c_char,
      &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).opt_ttl as *mut libc::c_uint,
      &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).deadline_us as *mut libc::c_uint,
      &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).timeout as *mut libc::c_uint,
      &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).str_I as *mut *mut libc::c_char,
      &mut str_p as *mut *mut libc::c_char,
      &mut str_i as *mut *mut libc::c_char,
    )) as libc::c_int;
  if opt & OPT_s as libc::c_int != 0 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).datalen = xatou16(str_s) as libc::c_uint
  }
  if opt & OPT_I as libc::c_int != 0 {
    // -I
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).if_index =
      if_nametoindex((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).str_I) as libc::c_int;
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).if_index == 0 {
      /* TODO: I'm not sure it takes IPv6 unless in [XX:XX..] format */
      let ref mut fresh22 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).source_lsa;
      *fresh22 = xdotted2sockaddr(
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).str_I,
        0i32,
      );
      let ref mut fresh23 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).str_I;
      *fresh23 = 0 as *mut libc::c_char
      /* don't try to bind to device later */
    }
  }
  if opt & OPT_p as libc::c_int != 0 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pattern =
      xstrtou_range(str_p, 16i32, 0i32 as libc::c_uint, 255i32 as libc::c_uint) as u8
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).deadline_us != 0 {
    let mut d: libc::c_uint = if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).deadline_us
      < (2147483647i32 / 1000000i32) as libc::c_uint
    {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).deadline_us
    } else {
      (2147483647i32 / 1000000i32) as libc::c_uint
    };
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).deadline_us = (1i32 as libc::c_ulonglong
      | (d.wrapping_mul(1000000i32 as libc::c_uint) as libc::c_ulonglong)
        .wrapping_add(monotonic_us()))
      as libc::c_uint
  }
  interval = parse_duration_str(str_i);
  if interval > (2147483647i32 / 1000000i32) as libc::c_double {
    interval = (2147483647i32 / 1000000i32) as duration_t
  }
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).interval_us =
    (interval * 1000000i32 as libc::c_double) as libc::c_uint;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).myid = getpid() as u16;
  let ref mut fresh24 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hostname;
  *fresh24 = *argv.offset(optind as isize);
  let mut af: sa_family_t = 0i32 as sa_family_t;
  if opt & OPT_IPV4 as libc::c_int != 0 {
    af = 2i32 as sa_family_t
  }
  if opt & OPT_IPV6 as libc::c_int != 0 {
    af = 10i32 as sa_family_t
  }
  lsa = xhost_and_af2sockaddr(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).hostname,
    0i32,
    af,
  );
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .source_lsa
    .is_null()
    && (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).source_lsa)
      .u
      .sa
      .sa_family as libc::c_int
      != (*lsa).u.sa.sa_family as libc::c_int
  {
    /* leaking it here... */
    let ref mut fresh25 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).source_lsa;
    *fresh25 = 0 as *mut len_and_sockaddr
  }
  let ref mut fresh26 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).dotted;
  *fresh26 = xmalloc_sockaddr2dotted_noport(&mut (*lsa).u.sa);
  ping(lsa);
  print_stats_and_exit(0i32);
  /*return EXIT_SUCCESS;*/
}
/* FEATURE_FANCY_PING */
#[no_mangle]
pub unsafe extern "C" fn ping_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  return common_ping_main(0i32, argv);
}
#[no_mangle]
pub unsafe extern "C" fn ping6_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  return common_ping_main(OPT_IPV6 as libc::c_int, argv);
}
/* from ping6.c:
 * Copyright (c) 1989 The Regents of the University of California.
 * All rights reserved.
 *
 * This code is derived from software contributed to Berkeley by
 * Mike Muuss.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * 3. <BSD Advertising Clause omitted per the July 22, 1999 licensing change
 *		ftp://ftp.cs.berkeley.edu/pub/4bsd/README.Impt.License.Change>
 *
 * 4. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ''AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

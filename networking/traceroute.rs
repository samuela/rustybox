use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
use libc;
extern "C" {
  #[no_mangle]
  fn setsockopt(
    __fd: libc::c_int,
    __level: libc::c_int,
    __optname: libc::c_int,
    __optval: *const libc::c_void,
    __optlen: socklen_t,
  ) -> libc::c_int;
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
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn getgid() -> __gid_t;
  #[no_mangle]
  fn getuid() -> __uid_t;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn getpid() -> __pid_t;
  #[no_mangle]
  fn monotonic_us() -> libc::c_ulonglong;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xmemdup(s: *const libc::c_void, n: libc::c_int) -> *mut libc::c_void;
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  /* not FAST_FUNC! */
  #[no_mangle]
  fn xsetgid(gid: gid_t);
  #[no_mangle]
  fn xsetuid(uid: uid_t);
  #[no_mangle]
  fn xsocket(domain: libc::c_int, type_0: libc::c_int, protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xbind(sockfd: libc::c_int, my_addr: *mut sockaddr, addrlen: socklen_t);
  #[no_mangle]
  fn xconnect(s: libc::c_int, s_addr: *const sockaddr, addrlen: socklen_t);
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
  fn setsockopt_SOL_SOCKET_1(fd: libc::c_int, optname: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn setsockopt_bindtodevice(fd: libc::c_int, iface: *const libc::c_char) -> libc::c_int;
  /* Get local address of bound or accepted socket */
  #[no_mangle]
  fn get_sock_lsa(fd: libc::c_int) -> *mut len_and_sockaddr;
  #[no_mangle]
  fn xhost_and_af2sockaddr(
    host: *const libc::c_char,
    port: libc::c_int,
    af: sa_family_t,
  ) -> *mut len_and_sockaddr;
  /* Assign sin[6]_port member if the socket is an AF_INET[6] one,
   * otherwise no-op. Useful for ftp.
   * NB: does NOT do htons() internally, just direct assignment. */
  #[no_mangle]
  fn set_nport(sa: *mut sockaddr, port: libc::c_uint);
  /* This one doesn't append :PORTNUM */
  #[no_mangle]
  fn xmalloc_sockaddr2host_noport(sa: *const sockaddr) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc_sockaddr2dotted_noport(sa: *const sockaddr) -> *mut libc::c_char;
  #[no_mangle]
  fn recv_from_to(
    fd: libc::c_int,
    buf: *mut libc::c_void,
    len: size_t,
    flags: libc::c_int,
    from: *mut sockaddr,
    to: *mut sockaddr,
    sa_size: socklen_t,
  ) -> ssize_t;
  #[no_mangle]
  fn inet_cksum(addr: *mut uint16_t, len: libc::c_int) -> uint16_t;
  /* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
   * But potentially slow, don't use in one-billion-times loops */
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  /* Wrapper which restarts poll on EINTR or ENOMEM.
   * On other errors complains [perror("poll")] and returns.
   * Warning! May take (much) longer than timeout_ms to return!
   * If this is a problem, use bare poll and open-code EINTR/ENOMEM handling */
  #[no_mangle]
  fn safe_poll(ufds: *mut pollfd, nfds: nfds_t, timeout_ms: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xatoull_range(
    str: *const libc::c_char,
    l: libc::c_ulonglong,
    u: libc::c_ulonglong,
  ) -> libc::c_ulonglong;
  #[no_mangle]
  fn xatou_range(str: *const libc::c_char, l: libc::c_uint, u: libc::c_uint) -> libc::c_uint;
  /* Useful for reading port numbers */
  #[no_mangle]
  fn xatou16(numstr: *const libc::c_char) -> uint16_t;
  #[no_mangle]
  fn bb_sanitize_stdio();
  /* { "-", NULL } */
  #[no_mangle]
  static mut option_mask32: uint32_t;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  static bb_msg_you_must_be_root: [libc::c_char; 0];
  /* '*const' ptr makes gcc optimize code much better.
   * Magic prevents ptr_to_globals from going into rodata.
   * If you want to assign a value, use SET_PTR_TO_GLOBALS(x) */
  #[no_mangle]
  static ptr_to_globals: *mut globals;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn usleep(__useconds: __useconds_t) -> libc::c_int;
}

pub type __uint16_t = libc::c_ushort;

pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
use crate::librb::__pid_t;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;

pub type __socklen_t = libc::c_uint;
use crate::librb::gid_t;
use crate::librb::uid_t;
use crate::librb::ssize_t;
use crate::librb::size_t;
use crate::librb::timeval;
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
pub type C2RustUnnamed = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed = 67108864;
pub const MSG_BATCH: C2RustUnnamed = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed = 65536;
pub const MSG_MORE: C2RustUnnamed = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed = 8192;
pub const MSG_RST: C2RustUnnamed = 4096;
pub const MSG_CONFIRM: C2RustUnnamed = 2048;
pub const MSG_SYN: C2RustUnnamed = 1024;
pub const MSG_FIN: C2RustUnnamed = 512;
pub const MSG_WAITALL: C2RustUnnamed = 256;
pub const MSG_EOR: C2RustUnnamed = 128;
pub const MSG_DONTWAIT: C2RustUnnamed = 64;
pub const MSG_TRUNC: C2RustUnnamed = 32;
pub const MSG_PROXY: C2RustUnnamed = 16;
pub const MSG_CTRUNC: C2RustUnnamed = 8;
pub const MSG_TRYHARD: C2RustUnnamed = 4;
pub const MSG_DONTROUTE: C2RustUnnamed = 4;
pub const MSG_PEEK: C2RustUnnamed = 2;
pub const MSG_OOB: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
  pub sin6_family: sa_family_t,
  pub sin6_port: in_port_t,
  pub sin6_flowinfo: uint32_t,
  pub sin6_addr: in6_addr,
  pub sin6_scope_id: uint32_t,
}
use crate::librb::uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
  pub __in6_u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
  pub __u6_addr8: [uint8_t; 16],
  pub __u6_addr16: [uint16_t; 8],
  pub __u6_addr32: [uint32_t; 4],
}
pub type uint16_t = __uint16_t;
use crate::librb::uint8_t;
pub type in_port_t = uint16_t;
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
pub type in_addr_t = uint32_t;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_1 = 256;
pub const IPPROTO_RAW: C2RustUnnamed_1 = 255;
pub const IPPROTO_MPLS: C2RustUnnamed_1 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_1 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_1 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_1 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_1 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_1 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_1 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_1 = 92;
pub const IPPROTO_AH: C2RustUnnamed_1 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_1 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_1 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_1 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_1 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_1 = 33;
pub const IPPROTO_TP: C2RustUnnamed_1 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_1 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_1 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_1 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_1 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_1 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_1 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_1 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_1 = 1;
pub const IPPROTO_IP: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const IPPROTO_MH: C2RustUnnamed_2 = 135;
pub const IPPROTO_DSTOPTS: C2RustUnnamed_2 = 60;
pub const IPPROTO_NONE: C2RustUnnamed_2 = 59;
pub const IPPROTO_ICMPV6: C2RustUnnamed_2 = 58;
pub const IPPROTO_FRAGMENT: C2RustUnnamed_2 = 44;
pub const IPPROTO_ROUTING: C2RustUnnamed_2 = 43;
pub const IPPROTO_HOPOPTS: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct udphdr {
  pub c2rust_unnamed: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
  pub c2rust_unnamed: C2RustUnnamed_5,
  pub c2rust_unnamed_0: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
  pub source: uint16_t,
  pub dest: uint16_t,
  pub len: uint16_t,
  pub check: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
  pub uh_sport: uint16_t,
  pub uh_dport: uint16_t,
  pub uh_ulen: uint16_t,
  pub uh_sum: uint16_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct ip {
  #[bitfield(name = "ip_hl", ty = "libc::c_uint", bits = "0..=3")]
  #[bitfield(name = "ip_v", ty = "libc::c_uint", bits = "4..=7")]
  pub ip_hl_ip_v: [u8; 1],
  pub ip_tos: uint8_t,
  pub ip_len: libc::c_ushort,
  pub ip_id: libc::c_ushort,
  pub ip_off: libc::c_ushort,
  pub ip_ttl: uint8_t,
  pub ip_p: uint8_t,
  pub ip_sum: libc::c_ushort,
  pub ip_src: in_addr,
  pub ip_dst: in_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp_ra_addr {
  pub ira_addr: uint32_t,
  pub ira_preference: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp {
  pub icmp_type: uint8_t,
  pub icmp_code: uint8_t,
  pub icmp_cksum: uint16_t,
  pub icmp_hun: C2RustUnnamed_9,
  pub icmp_dun: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
  pub id_ts: C2RustUnnamed_8,
  pub id_ip: C2RustUnnamed_7,
  pub id_radv: icmp_ra_addr,
  pub id_mask: uint32_t,
  pub id_data: [uint8_t; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
  pub idi_ip: ip,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
  pub its_otime: uint32_t,
  pub its_rtime: uint32_t,
  pub its_ttime: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
  pub ih_pptr: libc::c_uchar,
  pub ih_gwaddr: in_addr,
  pub ih_idseq: ih_idseq,
  pub ih_void: uint32_t,
  pub ih_pmtu: ih_pmtu,
  pub ih_rtradv: ih_rtradv,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ih_rtradv {
  pub irt_num_addrs: uint8_t,
  pub irt_wpa: uint8_t,
  pub irt_lifetime: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ih_pmtu {
  pub ipm_void: uint16_t,
  pub ipm_nextmtu: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ih_idseq {
  pub icd_id: uint16_t,
  pub icd_seq: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip6_hdr {
  pub ip6_ctlun: C2RustUnnamed_10,
  pub ip6_src: in6_addr,
  pub ip6_dst: in6_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
  pub ip6_un1: ip6_hdrctl,
  pub ip6_un2_vfc: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip6_hdrctl {
  pub ip6_un1_flow: uint32_t,
  pub ip6_un1_plen: uint16_t,
  pub ip6_un1_nxt: uint8_t,
  pub ip6_un1_hlim: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp6_hdr {
  pub icmp6_type: uint8_t,
  pub icmp6_code: uint8_t,
  pub icmp6_cksum: uint16_t,
  pub icmp6_dataun: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
  pub icmp6_un_data32: [uint32_t; 1],
  pub icmp6_un_data16: [uint16_t; 2],
  pub icmp6_un_data8: [uint8_t; 4],
}
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
  pub fd: libc::c_int,
  pub events: libc::c_short,
  pub revents: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct len_and_sockaddr {
  pub len: socklen_t,
  pub u: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
  pub sa: sockaddr,
  pub sin: sockaddr_in,
  pub sin6: sockaddr_in6,
}
pub type C2RustUnnamed_13 = libc::c_uint;
pub const LSA_SIZEOF_SA: C2RustUnnamed_13 = 28;
pub const LSA_LEN_SIZE: C2RustUnnamed_13 = 4;
//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub outip: *mut ip,
  pub outdata: *mut outdata_t,
  pub dest_lsa: *mut len_and_sockaddr,
  pub packlen: libc::c_int,
  pub pmtu: libc::c_int,
  pub ident: uint32_t,
  pub port: uint16_t,
  pub waittime: libc::c_int,
  pub recv_pkt: [libc::c_uchar; 512],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct outdata_t {
  pub seq: libc::c_uchar,
  pub ttl: libc::c_uchar,
  pub tv_UNUSED: timeval,
}
pub type C2RustUnnamed_14 = libc::c_uint;
pub const OPT_IPV6: C2RustUnnamed_14 = 262144;
pub const OPT_IPV4: C2RustUnnamed_14 = 131072;
pub const OPT_FIRST_TTL: C2RustUnnamed_14 = 65536;
pub const OPT_PAUSE_MS: C2RustUnnamed_14 = 32768;
pub const OPT_WAITTIME: C2RustUnnamed_14 = 16384;
pub const OPT_SOURCE: C2RustUnnamed_14 = 8192;
pub const OPT_NPROBES: C2RustUnnamed_14 = 4096;
pub const OPT_PORT: C2RustUnnamed_14 = 2048;
pub const OPT_MAX_TTL: C2RustUnnamed_14 = 1024;
pub const OPT_DEVICE: C2RustUnnamed_14 = 512;
pub const OPT_TOS: C2RustUnnamed_14 = 256;
pub const OPT_IP_CHKSUM: C2RustUnnamed_14 = 128;
pub const OPT_VERBOSE: C2RustUnnamed_14 = 64;
pub const OPT_DEBUG: C2RustUnnamed_14 = 32;
pub const OPT_BYPASS_ROUTE: C2RustUnnamed_14 = 16;
pub const OPT_ADDR_NUM: C2RustUnnamed_14 = 8;
pub const OPT_TTL_FLAG: C2RustUnnamed_14 = 4;
pub const OPT_USE_ICMP: C2RustUnnamed_14 = 2;
pub const OPT_DONT_FRAGMNT: C2RustUnnamed_14 = 1;
pub type C2RustUnnamed_15 = libc::c_uint;
pub const sndsock: C2RustUnnamed_15 = 4;
pub const rcvsock: C2RustUnnamed_15 = 3;
pub const SIZEOF_ICMP_HDR: C2RustUnnamed_15 = 8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct outdata6_t {
  pub ident6: uint32_t,
  pub seq6: uint32_t,
  pub tv_UNUSED: timeval,
}
#[inline(always)]
unsafe extern "C" fn xatoul_range(
  mut str: *const libc::c_char,
  mut l: libc::c_ulong,
  mut u: libc::c_ulong,
) -> libc::c_ulong {
  return xatoull_range(str, l as libc::c_ulonglong, u as libc::c_ulonglong) as libc::c_ulong;
}
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
/* last inbound (icmp) packet */
unsafe extern "C" fn wait_for_reply(
  mut from_lsa: *mut len_and_sockaddr,
  mut to: *mut sockaddr,
  mut timestamp_us: *mut libc::c_uint,
  mut left_ms: *mut libc::c_int,
) -> libc::c_int {
  let mut pfd: [pollfd; 1] = [pollfd {
    fd: 0,
    events: 0,
    revents: 0,
  }; 1];
  let mut read_len: libc::c_int = 0i32;
  pfd[0].fd = rcvsock as libc::c_int;
  pfd[0].events = 0x1i32 as libc::c_short;
  if *left_ms >= 0i32 && safe_poll(pfd.as_mut_ptr(), 1i32 as nfds_t, *left_ms) > 0i32 {
    let mut t: libc::c_uint = 0;
    read_len = recv_from_to(
      rcvsock as libc::c_int,
      (*ptr_to_globals).recv_pkt.as_mut_ptr() as *mut libc::c_void,
      ::std::mem::size_of::<[libc::c_uchar; 512]>() as libc::c_ulong,
      MSG_DONTWAIT as libc::c_int,
      &mut (*from_lsa).u.sa,
      to,
      (*from_lsa).len,
    ) as libc::c_int;
    t = monotonic_us() as libc::c_uint;
    *left_ms = (*left_ms as libc::c_uint).wrapping_sub(
      t.wrapping_sub(*timestamp_us)
        .wrapping_div(1000i32 as libc::c_uint),
    ) as libc::c_int as libc::c_int;
    *timestamp_us = t
  }
  return read_len;
}
unsafe extern "C" fn send_probe(mut seq: libc::c_int, mut ttl: libc::c_int) {
  let mut len: libc::c_int = 0;
  let mut res: libc::c_int = 0;
  let mut out: *mut libc::c_void = 0 as *mut libc::c_void;
  /* Payload */
  if (*(*ptr_to_globals).dest_lsa).u.sa.sa_family as libc::c_int == 10i32 {
    let mut pkt: *mut outdata6_t = (*ptr_to_globals).outdata as *mut outdata6_t;
    (*pkt).ident6 = {
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = (*ptr_to_globals).ident;
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
    (*pkt).seq6 = {
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = seq as libc::c_uint;
      if 0 != 0 {
        __v = (__x & 0xff000000u32) >> 24i32
          | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
          | (__x & 0xff00i32 as libc::c_uint) << 8i32
          | (__x & 0xffi32 as libc::c_uint) << 24i32
      } else {
        let fresh3 = &mut __v;
        let fresh4;
        let fresh5 = __x;
        asm!("bswap $0" : "=r" (fresh4) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5))
                          :);
        c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
      }
      __v
    }
  /*gettimeofday(&pkt->tv, &tz);*/
  } else {
    (*(*ptr_to_globals).outdata).seq = seq as libc::c_uchar;
    (*(*ptr_to_globals).outdata).ttl = ttl as libc::c_uchar;
    // UNUSED: was storing gettimeofday's result there, but never ever checked it
    /*memcpy(&outdata->tv, tp, sizeof(outdata->tv));*/
    if option_mask32 & OPT_USE_ICMP as libc::c_int as libc::c_uint != 0 {
      (*((*ptr_to_globals).outip.offset(1) as *mut icmp))
        .icmp_hun
        .ih_idseq
        .icd_seq = {
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = seq as libc::c_ushort;
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
      };
      /* Always calculate checksum for icmp packets */
      (*((*ptr_to_globals).outip.offset(1) as *mut icmp)).icmp_cksum = 0i32 as uint16_t;
      (*((*ptr_to_globals).outip.offset(1) as *mut icmp)).icmp_cksum = inet_cksum(
        (*ptr_to_globals).outip.offset(1) as *mut icmp as *mut uint16_t,
        ((*ptr_to_globals).outip as *mut libc::c_char)
          .offset((*ptr_to_globals).packlen as isize)
          .wrapping_offset_from((*ptr_to_globals).outip.offset(1) as *mut icmp as *mut libc::c_char)
          as libc::c_long as libc::c_int,
      );
      if (*((*ptr_to_globals).outip.offset(1) as *mut icmp)).icmp_cksum as libc::c_int == 0i32 {
        (*((*ptr_to_globals).outip.offset(1) as *mut icmp)).icmp_cksum = 0xffffi32 as uint16_t
      }
    }
  }
  //BUG! verbose is (x & OPT_VERBOSE), not a counter!
  //ENABLE_FEATURE_TRACEROUTE_VERBOSE
  out = (*ptr_to_globals).outdata as *mut libc::c_void;
  if (*(*ptr_to_globals).dest_lsa).u.sa.sa_family as libc::c_int == 10i32 {
    res = setsockopt_int(sndsock as libc::c_int, 41i32, 16i32, ttl);
    if res != 0i32 {
      bb_perror_msg_and_die(
        b"setsockopt(%s) %d\x00" as *const u8 as *const libc::c_char,
        b"UNICAST_HOPS\x00" as *const u8 as *const libc::c_char,
        ttl,
      );
    }
  } else {
    res = setsockopt_int(sndsock as libc::c_int, IPPROTO_IP as libc::c_int, 2i32, ttl);
    if res != 0i32 {
      bb_perror_msg_and_die(
        b"setsockopt(%s) %d\x00" as *const u8 as *const libc::c_char,
        b"TTL\x00" as *const u8 as *const libc::c_char,
        ttl,
      );
    }
    if option_mask32 & OPT_USE_ICMP as libc::c_int as libc::c_uint != 0 {
      out = (*ptr_to_globals).outip.offset(1) as *mut icmp as *mut libc::c_void
    }
  }
  if option_mask32 & OPT_USE_ICMP as libc::c_int as libc::c_uint == 0 {
    set_nport(
      &mut (*(*ptr_to_globals).dest_lsa).u.sa,
      ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort =
          ((*ptr_to_globals).port as libc::c_int + seq) as libc::c_ushort;
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
      }) as libc::c_uint,
    );
  }
  len = ((*ptr_to_globals).outip as *mut libc::c_char)
    .offset((*ptr_to_globals).packlen as isize)
    .wrapping_offset_from(out as *mut libc::c_char) as libc::c_long as libc::c_int;
  res = xsendto(
    sndsock as libc::c_int,
    out,
    len as size_t,
    &mut (*(*ptr_to_globals).dest_lsa).u.sa,
    (*(*ptr_to_globals).dest_lsa).len,
  ) as libc::c_int;
  if res != len {
    bb_error_msg(
      b"sent %d octets, ret=%d\x00" as *const u8 as *const libc::c_char,
      len,
      res,
    );
  };
}
/*
 * Convert an ICMP "type" field to a printable string.
 */
unsafe extern "C" fn pr_type(mut t: libc::c_uchar) -> *const libc::c_char {
  static mut ttab: [*const libc::c_char; 19] = [
    b"Echo Reply\x00" as *const u8 as *const libc::c_char,
    b"ICMP 1\x00" as *const u8 as *const libc::c_char,
    b"ICMP 2\x00" as *const u8 as *const libc::c_char,
    b"Dest Unreachable\x00" as *const u8 as *const libc::c_char,
    b"Source Quench\x00" as *const u8 as *const libc::c_char,
    b"Redirect\x00" as *const u8 as *const libc::c_char,
    b"ICMP 6\x00" as *const u8 as *const libc::c_char,
    b"ICMP 7\x00" as *const u8 as *const libc::c_char,
    b"Echo\x00" as *const u8 as *const libc::c_char,
    b"Router Advert\x00" as *const u8 as *const libc::c_char,
    b"Router Solicit\x00" as *const u8 as *const libc::c_char,
    b"Time Exceeded\x00" as *const u8 as *const libc::c_char,
    b"Param Problem\x00" as *const u8 as *const libc::c_char,
    b"Timestamp\x00" as *const u8 as *const libc::c_char,
    b"Timestamp Reply\x00" as *const u8 as *const libc::c_char,
    b"Info Request\x00" as *const u8 as *const libc::c_char,
    b"Info Reply\x00" as *const u8 as *const libc::c_char,
    b"Mask Request\x00" as *const u8 as *const libc::c_char,
    b"Mask Reply\x00" as *const u8 as *const libc::c_char,
  ];
  static mut ttab6: [*const libc::c_char; 18] = [
    b"Error\x00" as *const u8 as *const libc::c_char,
    b"Dest Unreachable\x00" as *const u8 as *const libc::c_char,
    b"Packet Too Big\x00" as *const u8 as *const libc::c_char,
    b"Time Exceeded\x00" as *const u8 as *const libc::c_char,
    b"Param Problem\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    b"Echo Request\x00" as *const u8 as *const libc::c_char,
    b"Echo Reply\x00" as *const u8 as *const libc::c_char,
    b"Membership Query\x00" as *const u8 as *const libc::c_char,
    b"Membership Report\x00" as *const u8 as *const libc::c_char,
    b"Membership Reduction\x00" as *const u8 as *const libc::c_char,
    b"Router Solicit\x00" as *const u8 as *const libc::c_char,
    b"Router Advert\x00" as *const u8 as *const libc::c_char,
    b"Neighbor Solicit\x00" as *const u8 as *const libc::c_char,
    b"Neighbor Advert\x00" as *const u8 as *const libc::c_char,
    b"Redirect\x00" as *const u8 as *const libc::c_char,
  ];
  if (*(*ptr_to_globals).dest_lsa).u.sa.sa_family as libc::c_int == 10i32 {
    if (t as libc::c_int) < 5i32 {
      return ttab6[t as usize];
    }
    if (t as libc::c_int) < 128i32 || t as libc::c_int > 137i32 {
      return b"OUT-OF-RANGE\x00" as *const u8 as *const libc::c_char;
    }
    return ttab6[((t as libc::c_int & 63i32) + 8i32) as usize];
  }
  if t as libc::c_uint
    >= (::std::mem::size_of::<[*const libc::c_char; 19]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
      as libc::c_uint
  {
    return b"OUT-OF-RANGE\x00" as *const u8 as *const libc::c_char;
  }
  return ttab[t as usize];
}
unsafe extern "C" fn packet4_ok(
  mut read_len: libc::c_int,
  mut from: *const sockaddr_in,
  mut seq: libc::c_int,
) -> libc::c_int {
  let mut icp: *const icmp = 0 as *const icmp;
  let mut type_0: libc::c_uchar = 0;
  let mut code: libc::c_uchar = 0;
  let mut hlen: libc::c_int = 0;
  let mut ip: *const ip = 0 as *const ip;
  ip = (*ptr_to_globals).recv_pkt.as_mut_ptr() as *mut ip;
  hlen = ((*ip).ip_hl() as libc::c_int) << 2i32;
  if read_len < hlen + 8i32 {
    if option_mask32 & OPT_VERBOSE as libc::c_int as libc::c_uint != 0 {
      printf(
        b"packet too short (%d bytes) from %s\n\x00" as *const u8 as *const libc::c_char,
        read_len,
        inet_ntoa((*from).sin_addr),
      );
    }
    return 0i32;
  }
  read_len -= hlen;
  icp = (*ptr_to_globals)
    .recv_pkt
    .as_mut_ptr()
    .offset(hlen as isize) as *mut icmp;
  type_0 = (*icp).icmp_type;
  code = (*icp).icmp_code;
  /* Path MTU Discovery (RFC1191) */
  (*ptr_to_globals).pmtu = 0i32;
  if code as libc::c_int == 4i32 {
    (*ptr_to_globals).pmtu = ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = (*icp).icmp_hun.ih_pmtu.ipm_nextmtu;
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
  }
  if type_0 as libc::c_int == 11i32 && code as libc::c_int == 0i32
    || type_0 as libc::c_int == 3i32
    || type_0 as libc::c_int == 0i32
  {
    let mut hip: *const ip = 0 as *const ip;
    let mut up: *const udphdr = 0 as *const udphdr;
    hip = &(*icp).icmp_dun.id_ip.idi_ip;
    hlen = ((*hip).ip_hl() as libc::c_int) << 2i32;
    if option_mask32 & OPT_USE_ICMP as libc::c_int as libc::c_uint != 0 {
      let mut hicmp: *mut icmp = 0 as *mut icmp;
      /* XXX */
      if type_0 as libc::c_int == 0i32
        && (*icp).icmp_hun.ih_idseq.icd_id as libc::c_int
          == ({
            let mut __v: libc::c_ushort = 0;
            let mut __x: libc::c_ushort = (*ptr_to_globals).ident as libc::c_ushort;
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
        && (*icp).icmp_hun.ih_idseq.icd_seq as libc::c_int
          == ({
            let mut __v: libc::c_ushort = 0;
            let mut __x: libc::c_ushort = seq as libc::c_ushort;
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
      {
        return 3i32 + 1i32;
      }
      hicmp = (hip as *mut libc::c_uchar).offset(hlen as isize) as *mut icmp;
      if hlen + SIZEOF_ICMP_HDR as libc::c_int <= read_len
        && (*hip).ip_p as libc::c_int == IPPROTO_ICMP as libc::c_int
        && (*hicmp).icmp_hun.ih_idseq.icd_id as libc::c_int
          == ({
            let mut __v: libc::c_ushort = 0;
            let mut __x: libc::c_ushort = (*ptr_to_globals).ident as libc::c_ushort;
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
          }) as libc::c_int
        && (*hicmp).icmp_hun.ih_idseq.icd_seq as libc::c_int
          == ({
            let mut __v: libc::c_ushort = 0;
            let mut __x: libc::c_ushort = seq as libc::c_ushort;
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
      {
        return if type_0 as libc::c_int == 11i32 {
          -1i32
        } else {
          (code as libc::c_int) + 1i32
        };
      }
    } else {
      up = (hip as *mut libc::c_char).offset(hlen as isize) as *mut udphdr;
      if hlen + 12i32 <= read_len
        && (*hip).ip_p as libc::c_int == IPPROTO_UDP as libc::c_int
        && (*up).c2rust_unnamed.c2rust_unnamed_0.dest as libc::c_int
          == ({
            let mut __v: libc::c_ushort = 0;
            let mut __x: libc::c_ushort =
              ((*ptr_to_globals).port as libc::c_int + seq) as libc::c_ushort;
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
        return if type_0 as libc::c_int == 11i32 {
          -1i32
        } else {
          (code as libc::c_int) + 1i32
        };
      }
    }
  }
  if option_mask32 & OPT_VERBOSE as libc::c_int as libc::c_uint != 0 {
    let mut i: libc::c_int = 0;
    let mut lp: *mut uint32_t = &(*icp).icmp_dun.id_ip.idi_ip as *const ip as *mut uint32_t;
    printf(
      b"\n%d bytes from %s to %s: icmp type %d (%s) code %d\n\x00" as *const u8
        as *const libc::c_char,
      read_len,
      inet_ntoa((*from).sin_addr),
      inet_ntoa((*ip).ip_dst),
      type_0 as libc::c_int,
      pr_type(type_0),
      (*icp).icmp_code as libc::c_int,
    );
    i = 4i32;
    while i < read_len {
      let fresh30 = lp;
      lp = lp.offset(1);
      printf(
        b"%2d: x%8.8x\n\x00" as *const u8 as *const libc::c_char,
        i,
        *fresh30,
      );
      i = (i as libc::c_ulong).wrapping_add(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
        as libc::c_int as libc::c_int
    }
  }
  return 0i32;
}
unsafe extern "C" fn packet_ok(
  mut read_len: libc::c_int,
  mut from_lsa: *mut len_and_sockaddr,
  mut to: *mut sockaddr,
  mut seq: libc::c_int,
) -> libc::c_int {
  let mut icp: *const icmp6_hdr = 0 as *const icmp6_hdr;
  let mut type_0: libc::c_uchar = 0;
  let mut code: libc::c_uchar = 0;
  if (*from_lsa).u.sa.sa_family as libc::c_int == 2i32 {
    return packet4_ok(read_len, &mut (*from_lsa).u.sin, seq);
  }
  icp = (*ptr_to_globals).recv_pkt.as_mut_ptr() as *mut icmp6_hdr;
  type_0 = (*icp).icmp6_type;
  code = (*icp).icmp6_code;
  if type_0 as libc::c_int == 3i32 && code as libc::c_int == 0i32 || type_0 as libc::c_int == 1i32 {
    let mut hip: *mut ip6_hdr = 0 as *mut ip6_hdr;
    let mut up: *mut udphdr = 0 as *mut udphdr;
    let mut nexthdr: libc::c_int = 0;
    hip = icp.offset(1) as *mut ip6_hdr;
    up = hip.offset(1) as *mut udphdr;
    nexthdr = (*hip).ip6_ctlun.ip6_un1.ip6_un1_nxt as libc::c_int;
    if nexthdr == IPPROTO_FRAGMENT as libc::c_int {
      nexthdr = *(up as *mut libc::c_uchar) as libc::c_int;
      up = up.offset(1)
    }
    if nexthdr == IPPROTO_UDP as libc::c_int {
      let mut pkt: *mut outdata6_t = 0 as *mut outdata6_t;
      pkt = up.offset(1) as *mut outdata6_t;
      if ({
        let mut __v: libc::c_uint = 0;
        let mut __x: libc::c_uint = (*pkt).ident6;
        if 0 != 0 {
          __v = (__x & 0xff000000u32) >> 24i32
            | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
            | (__x & 0xff00i32 as libc::c_uint) << 8i32
            | (__x & 0xffi32 as libc::c_uint) << 24i32
        } else {
          let fresh31 = &mut __v;
          let fresh32;
          let fresh33 = __x;
          asm!("bswap $0" : "=r" (fresh32) : "0"
                             (c2rust_asm_casts::AsmCast::cast_in(fresh31, fresh33))
                             :);
          c2rust_asm_casts::AsmCast::cast_out(fresh31, fresh33, fresh32);
        }
        __v
      }) == (*ptr_to_globals).ident
        && ({
          let mut __v: libc::c_uint = 0;
          let mut __x: libc::c_uint = (*pkt).seq6;
          if 0 != 0 {
            __v = (__x & 0xff000000u32) >> 24i32
              | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
              | (__x & 0xff00i32 as libc::c_uint) << 8i32
              | (__x & 0xffi32 as libc::c_uint) << 24i32
          } else {
            let fresh34 = &mut __v;
            let fresh35;
            let fresh36 = __x;
            asm!("bswap $0" : "=r" (fresh35) : "0"
                                 (c2rust_asm_casts::AsmCast::cast_in(fresh34, fresh36))
                                 :);
            c2rust_asm_casts::AsmCast::cast_out(fresh34, fresh36, fresh35);
          }
          __v
        }) == seq as libc::c_uint
      {
        return if type_0 as libc::c_int == 3i32 {
          -1i32
        } else {
          ((code as libc::c_int) << 8i32) + 1i32
        };
      }
    }
  }
  if option_mask32 & OPT_VERBOSE as libc::c_int as libc::c_uint != 0 {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut pa1: [libc::c_char; 64] = [0; 64];
    let mut pa2: [libc::c_char; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    p = icp.offset(1) as *mut libc::c_uchar;
    printf(
      b"\n%d bytes from %s to %s: icmp type %d (%s) code %d\n\x00" as *const u8
        as *const libc::c_char,
      read_len,
      inet_ntop(
        10i32,
        &mut (*from_lsa).u.sin6.sin6_addr as *mut in6_addr as *const libc::c_void,
        pa1.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as socklen_t,
      ),
      inet_ntop(
        10i32,
        &mut (*(to as *mut sockaddr_in6)).sin6_addr as *mut in6_addr as *const libc::c_void,
        pa2.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as socklen_t,
      ),
      type_0 as libc::c_int,
      pr_type(type_0),
      (*icp).icmp6_code as libc::c_int,
    );
    read_len = (read_len as libc::c_ulong)
      .wrapping_sub(::std::mem::size_of::<icmp6_hdr>() as libc::c_ulong)
      as libc::c_int as libc::c_int;
    i = 0i32;
    while i < read_len {
      if i % 16i32 == 0i32 {
        printf(b"%04x:\x00" as *const u8 as *const libc::c_char, i);
      }
      if i % 4i32 == 0i32 {
        bb_putchar(' ' as i32);
      }
      printf(
        b"%02x\x00" as *const u8 as *const libc::c_char,
        *p.offset(i as isize) as libc::c_int,
      );
      if i % 16i32 == 15i32 && i + 1i32 < read_len {
        bb_putchar('\n' as i32);
      }
      i += 1
    }
    bb_putchar('\n' as i32);
  }
  return 0i32;
}
/* !ENABLE_TRACEROUTE6 */
/*
 * Construct an Internet address representation.
 * If the -n flag has been supplied, give
 * numeric value, otherwise try for symbolic name.
 */
unsafe extern "C" fn print_inetname(mut from: *const sockaddr) {
  let mut ina: *mut libc::c_char = xmalloc_sockaddr2dotted_noport(from);
  if option_mask32 & OPT_ADDR_NUM as libc::c_int as libc::c_uint != 0 {
    printf(b"  %s\x00" as *const u8 as *const libc::c_char, ina);
  } else {
    let mut n: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*from).sa_family as libc::c_int != 2i32
      || (*(from as *mut sockaddr_in)).sin_addr.s_addr != 0i32 as in_addr_t
    {
      /* Try to reverse resolve if it is not 0.0.0.0 */
      n = xmalloc_sockaddr2host_noport(from as *mut sockaddr)
    }
    printf(
      b"  %s (%s)\x00" as *const u8 as *const libc::c_char,
      if !n.is_null() { n } else { ina },
      ina,
    );
    free(n as *mut libc::c_void);
  }
  free(ina as *mut libc::c_void);
}
unsafe extern "C" fn print(
  mut read_len: libc::c_int,
  mut from: *const sockaddr,
  mut to: *const sockaddr,
) {
  print_inetname(from);
  if option_mask32 & OPT_VERBOSE as libc::c_int as libc::c_uint != 0 {
    let mut ina: *mut libc::c_char = xmalloc_sockaddr2dotted_noport(to);
    if (*to).sa_family as libc::c_int == 10i32 {
      read_len = (read_len as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<ip6_hdr>() as libc::c_ulong)
        as libc::c_int as libc::c_int
    } else {
      let mut ip4packet: *mut ip = (*ptr_to_globals).recv_pkt.as_mut_ptr() as *mut ip;
      read_len -= ((*ip4packet).ip_hl() as libc::c_int) << 2i32
    }
    printf(
      b" %d bytes to %s\x00" as *const u8 as *const libc::c_char,
      read_len,
      ina,
    );
    free(ina as *mut libc::c_void);
  };
}
unsafe extern "C" fn print_delta_ms(mut t1p: libc::c_uint, mut t2p: libc::c_uint) {
  let mut tt: libc::c_uint = t2p.wrapping_sub(t1p);
  printf(
    b"  %u.%03u ms\x00" as *const u8 as *const libc::c_char,
    tt.wrapping_div(1000i32 as libc::c_uint),
    tt.wrapping_rem(1000i32 as libc::c_uint),
  );
}
/*
 * Usage: [-dFIlnrvx] [-g gateway] [-i iface] [-f first_ttl]
 * [-m max_ttl] [ -p port] [-q nqueries] [-s src_addr] [-t tos]
 * [-w waittime] [-z pausemsecs] host [packetlen]"
 */
unsafe extern "C" fn common_traceroute_main(
  mut op: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut minpacket: libc::c_int = 0;
  let mut tos: libc::c_int = 0i32;
  let mut max_ttl: libc::c_int = 30i32;
  let mut nprobes: libc::c_int = 3i32;
  let mut first_ttl: libc::c_int = 1i32;
  let mut pausemsecs: libc::c_uint = 0i32 as libc::c_uint;
  let mut source: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut device: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut tos_str: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut max_ttl_str: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut port_str: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut nprobes_str: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut waittime_str: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut pausemsecs_str: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut first_ttl_str: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut dest_str: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut af: sa_family_t = 0;
  let mut ttl: libc::c_int = 0;
  let mut seq: libc::c_int = 0;
  let mut from_lsa: *mut len_and_sockaddr = 0 as *mut len_and_sockaddr;
  let mut lastaddr: *mut sockaddr = 0 as *mut sockaddr;
  let mut to: *mut sockaddr = 0 as *mut sockaddr;
  let ref mut fresh37 =
    *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
      as *mut *mut globals);
  *fresh37 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  (*ptr_to_globals).port = 33434i32 as uint16_t;
  (*ptr_to_globals).waittime = 5i32;
  op = (op as libc::c_uint
    | getopt32(
      argv,
      b"^FIlnrdvxt:i:m:p:q:s:w:z:f:46\x00-1:x-x\x00" as *const u8 as *const libc::c_char,
      &mut tos_str as *mut *mut libc::c_char,
      &mut device as *mut *mut libc::c_char,
      &mut max_ttl_str as *mut *mut libc::c_char,
      &mut port_str as *mut *mut libc::c_char,
      &mut nprobes_str as *mut *mut libc::c_char,
      &mut source as *mut *mut libc::c_char,
      &mut waittime_str as *mut *mut libc::c_char,
      &mut pausemsecs_str as *mut *mut libc::c_char,
      &mut first_ttl_str as *mut *mut libc::c_char,
    )) as libc::c_int;
  argv = argv.offset(optind as isize);
  /* IGNORED */
  if op & OPT_TOS as libc::c_int != 0 {
    tos = xatou_range(tos_str, 0i32 as libc::c_uint, 255i32 as libc::c_uint) as libc::c_int
  }
  if op & OPT_MAX_TTL as libc::c_int != 0 {
    max_ttl = xatou_range(max_ttl_str, 1i32 as libc::c_uint, 255i32 as libc::c_uint) as libc::c_int
  }
  if op & OPT_PORT as libc::c_int != 0 {
    (*ptr_to_globals).port = xatou16(port_str)
  }
  if op & OPT_NPROBES as libc::c_int != 0 {
    nprobes = xatou_range(
      nprobes_str,
      1i32 as libc::c_uint,
      2147483647i32 as libc::c_uint,
    ) as libc::c_int
  }
  if op & OPT_SOURCE as libc::c_int != 0 {
    /*
     * set the ip source address of the outbound
     * probe (e.g., on a multi-homed host).
     */
    if getuid() != 0i32 as libc::c_uint {
      bb_simple_error_msg_and_die(bb_msg_you_must_be_root.as_ptr());
    }
  }
  if op & OPT_WAITTIME as libc::c_int != 0 {
    (*ptr_to_globals).waittime = xatou_range(
      waittime_str,
      1i32 as libc::c_uint,
      (24i32 * 60i32 * 60i32) as libc::c_uint,
    ) as libc::c_int
  }
  if op & OPT_PAUSE_MS as libc::c_int != 0 {
    pausemsecs = xatou_range(
      pausemsecs_str,
      0i32 as libc::c_uint,
      (60i32 * 60i32 * 1000i32) as libc::c_uint,
    )
  }
  if op & OPT_FIRST_TTL as libc::c_int != 0 {
    first_ttl =
      xatou_range(first_ttl_str, 1i32 as libc::c_uint, max_ttl as libc::c_uint) as libc::c_int
  }
  /* Process destination and optional packet size */
  minpacket = (::std::mem::size_of::<ip>() as libc::c_ulong)
    .wrapping_add(SIZEOF_ICMP_HDR as libc::c_int as libc::c_ulong)
    .wrapping_add(::std::mem::size_of::<outdata_t>() as libc::c_ulong) as libc::c_int;
  if op & OPT_USE_ICMP as libc::c_int == 0 {
    minpacket = (::std::mem::size_of::<ip>() as libc::c_ulong)
      .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
      .wrapping_add(::std::mem::size_of::<outdata_t>() as libc::c_ulong)
      as libc::c_int
  }
  af = 0i32 as sa_family_t;
  if op & OPT_IPV4 as libc::c_int != 0 {
    af = 2i32 as sa_family_t
  }
  if op & OPT_IPV6 as libc::c_int != 0 {
    af = 10i32 as sa_family_t
  }
  (*ptr_to_globals).dest_lsa =
    xhost_and_af2sockaddr(*argv.offset(0), (*ptr_to_globals).port as libc::c_int, af);
  af = (*(*ptr_to_globals).dest_lsa).u.sa.sa_family;
  if af as libc::c_int == 10i32 {
    minpacket = (::std::mem::size_of::<ip6_hdr>() as libc::c_ulong)
      .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
      .wrapping_add(::std::mem::size_of::<outdata6_t>() as libc::c_ulong)
      as libc::c_int
  }
  (*ptr_to_globals).packlen = minpacket;
  if !(*argv.offset(1)).is_null() {
    (*ptr_to_globals).packlen = xatoul_range(
      *argv.offset(1),
      minpacket as libc::c_ulong,
      (32i32 * 1024i32) as libc::c_ulong,
    ) as libc::c_int
  }
  /* Ensure the socket fds won't be 0, 1 or 2 */
  bb_sanitize_stdio();
  if af as libc::c_int == 10i32 {
    xmove_fd(
      xsocket(
        10i32,
        SOCK_RAW as libc::c_int,
        IPPROTO_ICMPV6 as libc::c_int,
      ),
      rcvsock as libc::c_int,
    );
    setsockopt_1(rcvsock as libc::c_int, 41i32, 49i32);
  } else {
    xmove_fd(
      xsocket(2i32, SOCK_RAW as libc::c_int, IPPROTO_ICMP as libc::c_int),
      rcvsock as libc::c_int,
    );
  }
  if op & OPT_BYPASS_ROUTE as libc::c_int != 0 {
    setsockopt_SOL_SOCKET_1(rcvsock as libc::c_int, 5i32);
  }
  if af as libc::c_int == 10i32 {
    if setsockopt_int(rcvsock as libc::c_int, 255i32, 7i32, 2i32) != 0i32 {
      bb_perror_msg_and_die(
        b"setsockopt(%s)\x00" as *const u8 as *const libc::c_char,
        b"IPV6_CHECKSUM\x00" as *const u8 as *const libc::c_char,
      );
    }
    xmove_fd(
      xsocket(af as libc::c_int, SOCK_DGRAM as libc::c_int, 0i32),
      sndsock as libc::c_int,
    );
  } else if op & OPT_USE_ICMP as libc::c_int != 0 {
    xmove_fd(
      xsocket(2i32, SOCK_RAW as libc::c_int, IPPROTO_ICMP as libc::c_int),
      sndsock as libc::c_int,
    );
  } else {
    xmove_fd(
      xsocket(2i32, SOCK_DGRAM as libc::c_int, 0i32),
      sndsock as libc::c_int,
    );
  }
  if setsockopt_SOL_SOCKET_int(sndsock as libc::c_int, 7i32, (*ptr_to_globals).packlen) != 0i32 {
    bb_perror_msg_and_die(
      b"setsockopt(%s)\x00" as *const u8 as *const libc::c_char,
      b"SO_SNDBUF\x00" as *const u8 as *const libc::c_char,
    );
  }
  if op & OPT_TOS as libc::c_int != 0
    && setsockopt_int(sndsock as libc::c_int, IPPROTO_IP as libc::c_int, 1i32, tos) != 0i32
  {
    bb_perror_msg_and_die(
      b"setsockopt(%s) %d\x00" as *const u8 as *const libc::c_char,
      b"TOS\x00" as *const u8 as *const libc::c_char,
      tos,
    );
  }
  if op & OPT_BYPASS_ROUTE as libc::c_int != 0 {
    setsockopt_SOL_SOCKET_1(sndsock as libc::c_int, 5i32);
  }
  (*ptr_to_globals).outip = xzalloc((*ptr_to_globals).packlen as size_t) as *mut ip;
  (*ptr_to_globals).ident = getpid() as uint32_t;
  if 1i32 == 0 || af as libc::c_int == 2i32 {
    if op & OPT_USE_ICMP as libc::c_int != 0 {
      (*ptr_to_globals).ident |= 0x8000i32 as libc::c_uint;
      (*((*ptr_to_globals).outip.offset(1) as *mut icmp)).icmp_type = 8i32 as uint8_t;
      (*((*ptr_to_globals).outip.offset(1) as *mut icmp))
        .icmp_hun
        .ih_idseq
        .icd_id = {
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = (*ptr_to_globals).ident as libc::c_ushort;
        if 0 != 0 {
          __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
            as libc::c_ushort
        } else {
          let fresh38 = &mut __v;
          let fresh39;
          let fresh40 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh39) : "0"
                              (c2rust_asm_casts::AsmCast::cast_in(fresh38, fresh40))
                              : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh38, fresh40, fresh39);
        }
        __v
      };
      (*ptr_to_globals).outdata =
        ((*ptr_to_globals).outip.offset(1) as *mut icmp as *mut libc::c_char)
          .offset(SIZEOF_ICMP_HDR as libc::c_int as isize) as *mut outdata_t
    } else {
      (*ptr_to_globals).outdata =
        ((*ptr_to_globals).outip.offset(1) as *mut udphdr).offset(1) as *mut outdata_t
    }
  }
  if af as libc::c_int == 10i32 {
    (*ptr_to_globals).outdata = ((*ptr_to_globals).outip as *mut libc::c_char)
      .offset(::std::mem::size_of::<ip6_hdr>() as libc::c_ulong as isize)
      .offset(::std::mem::size_of::<udphdr>() as libc::c_ulong as isize)
      as *mut libc::c_void as *mut outdata_t
  }
  if op & OPT_DEVICE as libc::c_int != 0 {
    /* hmm, do we need error check? */
    setsockopt_bindtodevice(sndsock as libc::c_int, device);
  }
  if op & OPT_SOURCE as libc::c_int != 0 {
    // TODO: need xdotted_and_af2sockaddr?
    let mut source_lsa: *mut len_and_sockaddr = xhost_and_af2sockaddr(source, 0i32, af);
    /* Ping4 does this (why?) */
    if af as libc::c_int == 2i32 {
      if setsockopt(
        sndsock as libc::c_int,
        IPPROTO_IP as libc::c_int,
        32i32,
        &mut (*source_lsa).u.sa as *mut sockaddr as *const libc::c_void,
        (*source_lsa).len,
      ) != 0
      {
        bb_simple_error_msg_and_die(
          b"can\'t set multicast source interface\x00" as *const u8 as *const libc::c_char,
        );
      }
    }
    //TODO: we can query source port we bound to,
    // and check it in replies... if we care enough
    xbind(
      sndsock as libc::c_int,
      &mut (*source_lsa).u.sa,
      (*source_lsa).len,
    );
    free(source_lsa as *mut libc::c_void);
  } else if af as libc::c_int == 10i32 {
    //TODO: why we don't do it for IPv4?
    let mut source_lsa_0: *mut len_and_sockaddr = 0 as *mut len_and_sockaddr;
    let mut probe_fd: libc::c_int = xsocket(af as libc::c_int, SOCK_DGRAM as libc::c_int, 0i32);
    if op & OPT_DEVICE as libc::c_int != 0 {
      setsockopt_bindtodevice(probe_fd, device);
    }
    set_nport(
      &mut (*(*ptr_to_globals).dest_lsa).u.sa,
      ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = 1025i32 as libc::c_ushort;
        if 0 != 0 {
          __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
            as libc::c_ushort
        } else {
          let fresh41 = &mut __v;
          let fresh42;
          let fresh43 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh42) : "0"
                                (c2rust_asm_casts::AsmCast::cast_in(fresh41, fresh43))
                                : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh41, fresh43, fresh42);
        }
        __v
      }) as libc::c_uint,
    );
    /* dummy connect. makes kernel pick source IP (and port) */
    xconnect(
      probe_fd,
      &mut (*(*ptr_to_globals).dest_lsa).u.sa,
      (*(*ptr_to_globals).dest_lsa).len,
    );
    set_nport(
      &mut (*(*ptr_to_globals).dest_lsa).u.sa,
      ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = (*ptr_to_globals).port;
        if 0 != 0 {
          __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
            as libc::c_ushort
        } else {
          let fresh44 = &mut __v;
          let fresh45;
          let fresh46 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh45) : "0"
                                (c2rust_asm_casts::AsmCast::cast_in(fresh44, fresh46))
                                : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh44, fresh46, fresh45);
        }
        __v
      }) as libc::c_uint,
    );
    /* read IP and port */
    source_lsa_0 = get_sock_lsa(probe_fd);
    if source_lsa_0.is_null() {
      bb_simple_error_msg_and_die(b"can\'t get probe addr\x00" as *const u8 as *const libc::c_char);
    }
    close(probe_fd);
    /* bind our sockets to this IP (but not port) */
    set_nport(&mut (*source_lsa_0).u.sa, 0i32 as libc::c_uint);
    xbind(
      sndsock as libc::c_int,
      &mut (*source_lsa_0).u.sa,
      (*source_lsa_0).len,
    );
    xbind(
      rcvsock as libc::c_int,
      &mut (*source_lsa_0).u.sa,
      (*source_lsa_0).len,
    );
    free(source_lsa_0 as *mut libc::c_void);
  }
  /* Revert to non-privileged user after opening sockets */
  xsetgid(getgid()); /* counter */
  xsetuid(getuid()); /* flags */
  dest_str = xmalloc_sockaddr2dotted_noport(&mut (*(*ptr_to_globals).dest_lsa).u.sa); /* for (nprobes) */
  printf(
    b"traceroute to %s (%s)\x00" as *const u8 as *const libc::c_char,
    *argv.offset(0),
    dest_str,
  ); /* while (wait and read a packet) */
  if op & OPT_SOURCE as libc::c_int != 0 {
    printf(b" from %s\x00" as *const u8 as *const libc::c_char, source);
  }
  printf(
    b", %d hops max, %d byte packets\n\x00" as *const u8 as *const libc::c_char,
    max_ttl,
    (*ptr_to_globals).packlen,
  );
  from_lsa = xmemdup(
    (*ptr_to_globals).dest_lsa as *const libc::c_void,
    (LSA_LEN_SIZE as libc::c_int as libc::c_uint).wrapping_add((*(*ptr_to_globals).dest_lsa).len)
      as libc::c_int,
  ) as *mut len_and_sockaddr;
  lastaddr = xzalloc((*(*ptr_to_globals).dest_lsa).len as size_t) as *mut sockaddr;
  to = xzalloc((*(*ptr_to_globals).dest_lsa).len as size_t) as *mut sockaddr;
  seq = 0i32;
  ttl = first_ttl;
  while ttl <= max_ttl {
    let mut probe: libc::c_int = 0;
    let mut unreachable: libc::c_int = 0i32;
    let mut gotlastaddr: libc::c_int = 0i32;
    let mut got_there: libc::c_int = 0i32;
    printf(b"%2d\x00" as *const u8 as *const libc::c_char, ttl);
    probe = 0i32;
    while probe < nprobes {
      let mut read_len: libc::c_int = 0;
      let mut t1: libc::c_uint = 0;
      let mut t2: libc::c_uint = 0;
      let mut left_ms: libc::c_int = 0;
      let mut ip: *mut ip = 0 as *mut ip;
      fflush_all();
      if probe != 0i32 && pausemsecs > 0i32 as libc::c_uint {
        usleep(pausemsecs.wrapping_mul(1000i32 as libc::c_uint));
      }
      seq += 1;
      send_probe(seq, ttl);
      t1 = monotonic_us() as libc::c_uint;
      t2 = t1;
      left_ms = (*ptr_to_globals).waittime * 1000i32;
      loop {
        read_len = wait_for_reply(from_lsa, to, &mut t2, &mut left_ms);
        if !(read_len != 0i32) {
          break;
        }
        let mut icmp_code: libc::c_int = 0;
        /* Recv'ed a packet, or read error */
        /* t2 = monotonic_us() - set by wait_for_reply */
        if read_len < 0i32 {
          continue;
        }
        icmp_code = packet_ok(read_len, from_lsa, to, seq);
        /* Skip short packet */
        if icmp_code == 0i32 {
          continue;
        }
        if gotlastaddr == 0
          || memcmp(
            lastaddr as *const libc::c_void,
            &mut (*from_lsa).u.sa as *mut sockaddr as *const libc::c_void,
            (*from_lsa).len as libc::c_ulong,
          ) != 0i32
        {
          print(read_len, &mut (*from_lsa).u.sa, to);
          memcpy(
            lastaddr as *mut libc::c_void,
            &mut (*from_lsa).u.sa as *mut sockaddr as *const libc::c_void,
            (*from_lsa).len as libc::c_ulong,
          );
          gotlastaddr = 1i32
        }
        print_delta_ms(t1, t2);
        ip = (*ptr_to_globals).recv_pkt.as_mut_ptr() as *mut ip;
        if (*from_lsa).u.sa.sa_family as libc::c_int == 2i32 {
          if op & OPT_TTL_FLAG as libc::c_int != 0 {
            printf(
              b" (%d)\x00" as *const u8 as *const libc::c_char,
              (*ip).ip_ttl as libc::c_int,
            );
          }
        }
        /* time exceeded in transit */
        if icmp_code == -1i32 {
          break;
        }
        icmp_code -= 1;
        match icmp_code {
          1024 => got_there = 1i32,
          3 => {
            if (*ip).ip_ttl as libc::c_int <= 1i32 {
              printf(b" !\x00" as *const u8 as *const libc::c_char);
            }
            got_there = 1i32
          }
          0 => {
            printf(b" !N\x00" as *const u8 as *const libc::c_char);
            unreachable += 1
          }
          1 | 768 => {
            printf(b" !H\x00" as *const u8 as *const libc::c_char);
            unreachable += 1
          }
          2 => {
            printf(b" !P\x00" as *const u8 as *const libc::c_char);
            got_there = 1i32
          }
          4 => {
            printf(
              b" !F-%d\x00" as *const u8 as *const libc::c_char,
              (*ptr_to_globals).pmtu,
            );
            unreachable += 1
          }
          5 | 256 => {
            printf(b" !S\x00" as *const u8 as *const libc::c_char);
            unreachable += 1
          }
          13 | 9 => {
            /* misuse */
            printf(b" !A\x00" as *const u8 as *const libc::c_char);
            unreachable += 1
          }
          10 => {
            printf(b" !C\x00" as *const u8 as *const libc::c_char);
            unreachable += 1
          }
          14 => {
            printf(b" !V\x00" as *const u8 as *const libc::c_char);
            unreachable += 1
          }
          15 => {
            printf(b" !C\x00" as *const u8 as *const libc::c_char);
            unreachable += 1
          }
          6 | 7 => {
            printf(b" !U\x00" as *const u8 as *const libc::c_char);
            unreachable += 1
          }
          8 => {
            printf(b" !I\x00" as *const u8 as *const libc::c_char);
            unreachable += 1
          }
          11 | 12 => {
            printf(b" !T\x00" as *const u8 as *const libc::c_char);
            unreachable += 1
          }
          _ => {
            printf(b" !<%d>\x00" as *const u8 as *const libc::c_char, icmp_code);
            unreachable += 1
          }
        }
        break;
      }
      /* there was no packet at all? */
      if read_len == 0i32 {
        printf(b"  *\x00" as *const u8 as *const libc::c_char);
      }
      probe += 1
    }
    bb_putchar('\n' as i32);
    if got_there != 0 || unreachable > 0i32 && unreachable >= nprobes - 1i32 {
      break;
    }
    ttl += 1
  }
  return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn traceroute_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  return common_traceroute_main(0i32, argv);
}
#[no_mangle]
pub unsafe extern "C" fn traceroute6_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  return common_traceroute_main(OPT_IPV6 as libc::c_int, argv);
}

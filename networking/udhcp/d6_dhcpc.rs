use crate::libbb::appletlib::applet_name;
use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::xfuncs_printf::xmalloc;
use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
use libc::close;
use libc::free;
use libc::openlog;
use libc::putenv;
use libc::sleep;
use libc::strcmp;
use libc::unlink;
extern "C" {

  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn mempcpy(
    __dest: *mut libc::c_void,
    __src: *const libc::c_void,
    __n: size_t,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  /* client -> server */
  /* client -> server */
  /* client <- server */
  /* client <- server */
  /* client -> server */
  /* client -> server */
  /* Same as above + ensures that option length is 4 bytes
   * (returns NULL if size is different)
   */
  // RFC 2131  Table 5: Fields and options used by DHCP clients
  //
  // Fields 'hops', 'yiaddr', 'siaddr', 'giaddr' are always zero, 'chaddr' is always client's MAC
  //
  // Field      DHCPDISCOVER          DHCPINFORM            DHCPREQUEST           DHCPDECLINE         DHCPRELEASE
  // -----      ------------          ------------          -----------           -----------         -----------
  // 'xid'      selected by client    selected by client    'xid' from server     selected by client  selected by client
  //                                                        DHCPOFFER message
  // 'secs'     0 or seconds since    0 or seconds since    0 or seconds since    0                   0
  //            DHCP process started  DHCP process started  DHCP process started
  // 'flags'    Set 'BROADCAST'       Set 'BROADCAST'       Set 'BROADCAST'       0                   0
  //            flag if client needs  flag if client needs  flag if client needs
  //            broadcast reply       broadcast reply       broadcast reply
  // 'ciaddr'   0                     client's IP           0 or client's IP      0                   client's IP
  //                                                        (BOUND/RENEW/REBIND)
  // 'sname'    options or sname      options or sname      options or sname      (unused)            (unused)
  // 'file'     options or file       options or file       options or file       (unused)            (unused)
  // 'options'  options               options               options               message type opt    message type opt
  //
  // Option                     DHCPDISCOVER  DHCPINFORM  DHCPREQUEST             DHCPDECLINE  DHCPRELEASE
  // ------                     ------------  ----------  -----------             -----------  -----------
  // Requested IP address       MAY           MUST NOT    MUST (in SELECTING      MUST         MUST NOT
  //                                                      or INIT-REBOOT)
  //                                                      MUST NOT (in BOUND
  //                                                      or RENEWING)
  // IP address lease time      MAY           MUST NOT    MAY                     MUST NOT     MUST NOT
  // Use 'file'/'sname' fields  MAY           MAY         MAY                     MAY          MAY
  // Client identifier          MAY           MAY         MAY                     MAY          MAY
  // Vendor class identifier    MAY           MAY         MAY                     MUST NOT     MUST NOT
  // Server identifier          MUST NOT      MUST NOT    MUST (after SELECTING)  MUST         MUST
  //                                                      MUST NOT (after
  //                                                      INIT-REBOOT, BOUND,
  //                                                      RENEWING or REBINDING)
  // Parameter request list     MAY           MAY         MAY                     MUST NOT     MUST NOT
  // Maximum message size       MAY           MAY         MAY                     MUST NOT     MUST NOT
  // Message                    SHOULD NOT    SHOULD NOT  SHOULD NOT              SHOULD       SHOULD
  // Site-specific              MAY           MAY         MAY                     MUST NOT     MUST NOT
  // All others                 MAY           MAY         MAY                     MUST NOT     MUST NOT
  /* ** Logging ***/
  /* ** Other shared functions ***/
  /* 2nd param is "u32*" */
  /* 2nd param is "struct option_set**" */
  /*u32 ip,*/
  /* Returns 1 if no reply received */
  /* note: ip is a pointer to an IPv6 in network order, possibly misaliged */
  #[no_mangle]
  fn sprint_nip6(dest: *mut libc::c_char, ip: *const u8) -> libc::c_int;
  #[no_mangle]
  fn udhcp_listen_socket(port: libc::c_int, inf: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn udhcp_sp_read() -> libc::c_int;
  #[no_mangle]
  fn udhcp_sp_fd_set(pfds: *mut pollfd, extra_fd: libc::c_int);
  #[no_mangle]
  fn udhcp_sp_setup();
  #[no_mangle]
  fn rand() -> libc::c_int;
  #[no_mangle]
  fn srand(__seed: libc::c_uint);

  #[no_mangle]
  fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn inet_pton(
    __af: libc::c_int,
    __cp: *const libc::c_char,
    __buf: *mut libc::c_void,
  ) -> libc::c_int;

  #[no_mangle]
  fn monotonic_us() -> libc::c_ulonglong;
  #[no_mangle]
  fn monotonic_sec() -> libc::c_uint;

  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc_vector_helper(
    vector: *mut libc::c_void,
    sizeof_and_shift: libc::c_uint,
    idx: libc::c_int,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xmemdup(s: *const libc::c_void, n: libc::c_int) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_unsetenv_and_free(key: *mut libc::c_char);
  #[no_mangle]
  fn xsocket(domain: libc::c_int, type_0: libc::c_int, protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xbind(sockfd: libc::c_int, my_addr: *mut sockaddr, addrlen: socklen_t);
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  #[no_mangle]
  fn spawn_and_wait(argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn bb_daemonize_or_rexec(flags: libc::c_int);
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> u32;
  #[no_mangle]
  fn llist_pop(elm: *mut *mut llist_t) -> *mut libc::c_void;
  #[no_mangle]
  static mut wrote_pidfile: smallint;
  #[no_mangle]
  fn write_pidfile(path: *const libc::c_char);
  #[no_mangle]
  static mut logmode: smallint;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_info_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_info_msg(s: *const libc::c_char);

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
  #[no_mangle]
  static MAC_BCAST_ADDR: [u8; 6];
  #[no_mangle]
  fn udhcp_option_idx(
    name: *const libc::c_char,
    option_strings: *const libc::c_char,
  ) -> libc::c_uint;
  #[no_mangle]
  fn dname_dec(cstr: *const u8, clen: libc::c_int, pre: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  static mut dhcp_verbose: libc::c_uint;
  #[no_mangle]
  fn udhcp_str2optset(
    str: *const libc::c_char,
    arg: *mut libc::c_void,
    optflags: *const dhcp_optflag,
    option_strings: *const libc::c_char,
    dhcpv6: bool,
  ) -> libc::c_int;
  #[no_mangle]
  fn d6_read_interface(
    interface: *const libc::c_char,
    ifindex: *mut libc::c_int,
    nip6: *mut in6_addr,
    mac: *mut u8,
  ) -> libc::c_int;
  #[no_mangle]
  fn d6_recv_kernel_packet(
    peer_ipv6: *mut in6_addr,
    packet: *mut d6_packet,
    fd: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn d6_send_raw_packet(
    d6_pkt: *mut d6_packet,
    d6_pkt_size: libc::c_uint,
    src_ipv6: *mut in6_addr,
    source_port: libc::c_int,
    dst_ipv6: *mut in6_addr,
    dest_port: libc::c_int,
    dest_arp: *const u8,
    ifindex: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn d6_send_kernel_packet(
    d6_pkt: *mut d6_packet,
    d6_pkt_size: libc::c_uint,
    src_ipv6: *mut in6_addr,
    source_port: libc::c_int,
    dst_ipv6: *mut in6_addr,
    dest_port: libc::c_int,
    ifindex: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn d6_dump_packet(packet: *mut d6_packet);
}

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
pub type nfds_t = libc::c_ulong;
use crate::libbb::llist::llist_t;
use libc::pollfd;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const LOGMODE_BOTH: C2RustUnnamed_1 = 3;
pub const LOGMODE_SYSLOG: C2RustUnnamed_1 = 2;
pub const LOGMODE_STDIO: C2RustUnnamed_1 = 1;
pub const LOGMODE_NONE: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const COMMON_BUFSIZE: C2RustUnnamed_2 = 1024;
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
  pub source: u16,
  pub dest: u16,
  pub len: u16,
  pub check: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
  pub uh_sport: u16,
  pub uh_dport: u16,
  pub uh_ulen: u16,
  pub uh_sum: u16,
}
pub type C2RustUnnamed_6 = libc::c_uint;
pub const OPTION_LIST: C2RustUnnamed_6 = 32;
pub const OPTION_REQ: C2RustUnnamed_6 = 16;
pub const OPTION_TYPE_MASK: C2RustUnnamed_6 = 15;
pub const OPTION_SIP_SERVERS: C2RustUnnamed_6 = 13;
pub const OPTION_DNS_STRING: C2RustUnnamed_6 = 12;
pub const OPTION_6RD: C2RustUnnamed_6 = 11;
pub const OPTION_STATIC_ROUTES: C2RustUnnamed_6 = 10;
pub const OPTION_BIN: C2RustUnnamed_6 = 9;
pub const OPTION_S32: C2RustUnnamed_6 = 8;
pub const OPTION_U32: C2RustUnnamed_6 = 7;
pub const OPTION_U16: C2RustUnnamed_6 = 6;
pub const OPTION_U8: C2RustUnnamed_6 = 5;
pub const OPTION_STRING_HOST: C2RustUnnamed_6 = 4;
pub const OPTION_STRING: C2RustUnnamed_6 = 3;
pub const OPTION_IP_PAIR: C2RustUnnamed_6 = 2;
pub const OPTION_IP: C2RustUnnamed_6 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_optflag {
  pub flags: u8,
  pub code: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option_set {
  pub data: *mut u8,
  pub next: *mut option_set,
}

/*
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_data_t {
  pub client_mac: [u8; 6],
  pub ifindex: libc::c_int,
  pub opt_mask: [u8; 32],
  pub interface: *const libc::c_char,
  pub pidfile: *mut libc::c_char,
  pub script: *const libc::c_char,
  pub options: *mut option_set,
  pub clientid: *mut u8,
  pub vendorclass: *mut u8,
  pub hostname: *mut u8,
  pub fqdn: *mut u8,
  pub first_secs: libc::c_uint,
  pub last_secs: libc::c_uint,
  pub sockfd: libc::c_int,
  pub listen_mode: smallint,
  pub state: smallint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip6_hdr {
  pub ip6_ctlun: C2RustUnnamed_7,
  pub ip6_src: in6_addr,
  pub ip6_dst: in6_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
  pub ip6_un1: ip6_hdrctl,
  pub ip6_un2_vfc: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip6_hdrctl {
  pub ip6_un1_flow: u32,
  pub ip6_un1_plen: u16,
  pub ip6_un1_nxt: u8,
  pub ip6_un1_hlim: u8,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct d6_packet {
  pub d6_u: C2RustUnnamed_8,
  pub d6_options: [u8; 604],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
  pub d6_msg_type: u8,
  pub d6_xid32: u32,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ip6_udp_d6_packet {
  pub ip6: ip6_hdr,
  pub udp: udphdr,
  pub data: d6_packet,
}
/* ** Options ***/
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct d6_option {
  pub code_hi: u8,
  pub code: u8,
  pub len_hi: u8,
  pub len: u8,
  pub data: [u8; 1],
}
/* ** Other shared functions ***/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client6_data_t {
  pub server_id: *mut d6_option,
  pub ia_na: *mut d6_option,
  pub ia_pd: *mut d6_option,
  pub env_ptr: *mut *mut libc::c_char,
  pub env_idx: libc::c_uint,
  pub ll_ip6: in6_addr,
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
// /	IF_FEATURE_UDHCPC_ARPING("arping\0"	No_argument       "a")
/* Must match getopt32 option string order */
pub type C2RustUnnamed_9 = libc::c_uint;
// /IF_FEATURE_UDHCPC_ARPING(OPT_a = 1 << OPTBIT_a,)
// /IF_FEATURE_UDHCPC_ARPING(OPTBIT_a,)
pub const OPT_b: C2RustUnnamed_9 = 131072;
pub const OPTBIT_b: C2RustUnnamed_9 = 17;
/* The rest has variable bit positions, need to be clever */
pub const OPTBIT_d: C2RustUnnamed_9 = 16;
pub const OPT_d: C2RustUnnamed_9 = 65536;
pub const OPT_l: C2RustUnnamed_9 = 32768;
pub const OPT_f: C2RustUnnamed_9 = 16384;
pub const OPT_x: C2RustUnnamed_9 = 8192;
pub const OPT_o: C2RustUnnamed_9 = 4096;
pub const OPT_O: C2RustUnnamed_9 = 2048;
pub const OPT_A: C2RustUnnamed_9 = 1024;
pub const OPT_S: C2RustUnnamed_9 = 512;
pub const OPT_t: C2RustUnnamed_9 = 256;
pub const OPT_T: C2RustUnnamed_9 = 128;
pub const OPT_s: C2RustUnnamed_9 = 64;
pub const OPT_r: C2RustUnnamed_9 = 32;
pub const OPT_R: C2RustUnnamed_9 = 16;
pub const OPT_q: C2RustUnnamed_9 = 8;
pub const OPT_p: C2RustUnnamed_9 = 4;
pub const OPT_n: C2RustUnnamed_9 = 2;
pub const OPT_i: C2RustUnnamed_9 = 1;

/*
 * DHCPv6 client.
 *
 * Copyright (C) 2011-2017 Denys Vlasenko.
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config UDHCPC6
//config:	bool "udhcpc6 (21 kb)"
//config:	default y
//config:	depends on FEATURE_IPV6
//config:	help
//config:	udhcpc6 is a DHCPv6 client
//config:
//config:config FEATURE_UDHCPC6_RFC3646
//config:	bool "Support RFC 3646 (DNS server and search list)"
//config:	default y
//config:	depends on UDHCPC6
//config:	help
//config:	List of DNS servers and domain search list can be requested with
//config:	"-O dns" and "-O search". If server gives these values,
//config:	they will be set in environment variables "dns" and "search".
//config:
//config:config FEATURE_UDHCPC6_RFC4704
//config:	bool "Support RFC 4704 (Client FQDN)"
//config:	default y
//config:	depends on UDHCPC6
//config:	help
//config:	You can request FQDN to be given by server using "-O fqdn".
//config:
//config:config FEATURE_UDHCPC6_RFC4833
//config:	bool "Support RFC 4833 (Timezones)"
//config:	default y
//config:	depends on UDHCPC6
//config:	help
//config:	You can request POSIX timezone with "-O tz" and timezone name
//config:	with "-O timezone".
//config:
//config:config FEATURE_UDHCPC6_RFC5970
//config:	bool "Support RFC 5970 (Network Boot)"
//config:	default y
//config:	depends on UDHCPC6
//config:	help
//config:	You can request bootfile-url with "-O bootfile_url" and
//config:	bootfile-params with "-O bootfile_params".
//applet:IF_UDHCPC6(APPLET(udhcpc6, BB_DIR_USR_BIN, SUID_DROP))
//kbuild:lib-$(CONFIG_UDHCPC6) += d6_dhcpc.o d6_packet.o d6_socket.o common.o socket.o signalpipe.o
//kbuild:lib-$(CONFIG_FEATURE_UDHCPC6_RFC3646) += domain_codec.o
//kbuild:lib-$(CONFIG_FEATURE_UDHCPC6_RFC4704) += domain_codec.o
/* Override ENABLE_FEATURE_PIDFILE - ifupdown needs our pidfile to always exist */
/* "struct client_data_t client_data" is in bb_common_bufsiz1 */
static mut d6_optflags: [dhcp_optflag; 10] = [
  {
    let mut init = dhcp_optflag {
      flags: (OPTION_6RD as libc::c_int | OPTION_LIST as libc::c_int | OPTION_REQ as libc::c_int)
        as u8,
      code: 23i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: (OPTION_DNS_STRING as libc::c_int
        | OPTION_LIST as libc::c_int
        | OPTION_REQ as libc::c_int) as u8,
      code: 24i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_DNS_STRING as libc::c_int as u8,
      code: 39i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_STRING as libc::c_int as u8,
      code: 41i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_STRING as libc::c_int as u8,
      code: 42i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_STRING as libc::c_int as u8,
      code: 59i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_STRING as libc::c_int as u8,
      code: 60i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_STRING as libc::c_int as u8,
      code: 0xd1i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: OPTION_STRING as libc::c_int as u8,
      code: 0xd2i32 as u8,
    };
    init
  },
  {
    let mut init = dhcp_optflag {
      flags: 0 as u8,
      code: 0 as u8,
    };
    init
  },
];
/* Must match d6_optflags[] order */
static mut d6_option_strings: [libc::c_char; 84] = [
  100, 110, 115, 0, 115, 101, 97, 114, 99, 104, 0, 102, 113, 100, 110, 0, 116, 122, 0, 116, 105,
  109, 101, 122, 111, 110, 101, 0, 98, 111, 111, 116, 102, 105, 108, 101, 95, 117, 114, 108, 0, 98,
  111, 111, 116, 102, 105, 108, 101, 95, 112, 97, 114, 97, 109, 0, 112, 120, 101, 99, 111, 110,
  102, 102, 105, 108, 101, 0, 112, 120, 101, 112, 97, 116, 104, 112, 114, 101, 102, 105, 120, 0, 0,
  0,
];
static mut udhcpc6_longopts: [libc::c_char; 197] = [
  105, 110, 116, 101, 114, 102, 97, 99, 101, 0, 1, 105, 110, 111, 119, 0, 0, 110, 112, 105, 100,
  102, 105, 108, 101, 0, 1, 112, 113, 117, 105, 116, 0, 0, 113, 114, 101, 108, 101, 97, 115, 101,
  0, 0, 82, 114, 101, 113, 117, 101, 115, 116, 0, 1, 114, 114, 101, 113, 117, 101, 115, 116, 112,
  114, 101, 102, 105, 120, 0, 0, 100, 115, 99, 114, 105, 112, 116, 0, 1, 115, 116, 105, 109, 101,
  111, 117, 116, 0, 1, 84, 114, 101, 116, 114, 105, 101, 115, 0, 1, 116, 116, 114, 121, 97, 103,
  97, 105, 110, 0, 1, 65, 115, 121, 115, 108, 111, 103, 0, 0, 83, 114, 101, 113, 117, 101, 115,
  116, 45, 111, 112, 116, 105, 111, 110, 0, 1, 79, 110, 111, 45, 100, 101, 102, 97, 117, 108, 116,
  45, 111, 112, 116, 105, 111, 110, 115, 0, 0, 111, 102, 111, 114, 101, 103, 114, 111, 117, 110,
  100, 0, 0, 102, 115, 116, 97, 116, 101, 108, 101, 115, 115, 0, 0, 108, 98, 97, 99, 107, 103, 114,
  111, 117, 110, 100, 0, 0, 98, 0,
];
static mut opt_fqdn_req: [libc::c_char; 6] = [
  (39i32 >> 8i32) as libc::c_char,
  (39i32 & 0xffi32) as libc::c_char,
  0 as libc::c_char,
  2i32 as libc::c_char,
  0 as libc::c_char,
  0 as libc::c_char,
];
/* ** Utility functions ***/
unsafe extern "C" fn d6_find_option(
  mut option: *mut u8,
  mut option_end: *mut u8,
  mut code: libc::c_uint,
) -> *mut libc::c_void {
  /* "length minus 4" */
  let mut len_m4: libc::c_int =
    (option_end.wrapping_offset_from(option) as libc::c_long - 4i32 as libc::c_long) as libc::c_int;
  while len_m4 >= 0 {
    /* Next option's len is too big? */
    if *option.offset(3) as libc::c_int > len_m4 {
      return 0 as *mut libc::c_void;
    } /* yes. bogus packet! */
    /* So far we treat any opts with code >255
     * or len >255 as bogus, and stop at once.
     * This simplifies big-endian handling.
     */
    if *option.offset(0) as libc::c_int != 0 || *option.offset(2) as libc::c_int != 0 {
      return 0 as *mut libc::c_void;
    }
    /* Option seems to be valid */
    /* Does its code match? */
    if *option.offset(1) as libc::c_uint == code {
      return option as *mut libc::c_void;
    } /* yes! */
    len_m4 -= *option.offset(3) as libc::c_int + 4i32;
    option = option.offset((*option.offset(3) as libc::c_int + 4i32) as isize)
  }
  return 0 as *mut libc::c_void;
}
unsafe extern "C" fn d6_copy_option(
  mut option: *mut u8,
  mut option_end: *mut u8,
  mut code: libc::c_uint,
) -> *mut libc::c_void {
  let mut opt: *mut u8 = d6_find_option(option, option_end, code) as *mut u8;
  if opt.is_null() {
    return opt as *mut libc::c_void;
  }
  return xmemdup(
    opt as *const libc::c_void,
    *opt.offset(3) as libc::c_int + 4i32,
  );
}
/* ** Script execution code ***/
unsafe extern "C" fn new_env() -> *mut *mut libc::c_char {
  let ref mut fresh0 = (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
    (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
      .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
  ) as *mut libc::c_char as *mut client6_data_t))
    .env_ptr;
  *fresh0 = xrealloc_vector_helper(
    (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
    ) as *mut libc::c_char as *mut client6_data_t))
      .env_ptr as *mut libc::c_void,
    ((::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) << 8i32)
      .wrapping_add(3i32 as libc::c_ulong) as libc::c_uint,
    (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
    ) as *mut libc::c_char as *mut client6_data_t))
      .env_idx as libc::c_int,
  ) as *mut *mut libc::c_char;
  let ref mut fresh1 = (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
    (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
      .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
  ) as *mut libc::c_char as *mut client6_data_t))
    .env_idx;
  let fresh2 = *fresh1;
  *fresh1 = (*fresh1).wrapping_add(1);
  return &mut *(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
    (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
      .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
  ) as *mut libc::c_char as *mut client6_data_t))
    .env_ptr
    .offset(fresh2 as isize) as *mut *mut libc::c_char;
}
unsafe extern "C" fn string_option_to_env(
  mut option: *const u8,
  mut option_end: *const u8,
) -> *mut libc::c_char {
  let mut current_block: u64;
  let mut ptr: *const libc::c_char = std::ptr::null();
  let mut name: *const libc::c_char = std::ptr::null();
  let mut val_len: libc::c_uint = 0;
  let mut i: libc::c_int = 0;
  ptr = d6_option_strings.as_ptr();
  i = 0;
  loop {
    if !(*ptr != 0) {
      current_block = 3276175668257526147;
      break;
    }
    if d6_optflags[i as usize].code as libc::c_int == *option.offset(1) as libc::c_int {
      name = ptr;
      current_block = 9732342458255933827;
      break;
    } else {
      ptr = ptr.offset(strlen(ptr).wrapping_add(1i32 as libc::c_ulong) as isize);
      i += 1
    }
  }
  match current_block {
    9732342458255933827 => {
      val_len = ((*option.offset(2) as libc::c_int) << 8i32 | *option.offset(3) as libc::c_int)
        as libc::c_uint;
      if (&*option.offset(4) as *const u8).offset(val_len as isize) > option_end {
        bb_simple_error_msg(
          b"option data exceeds option length\x00" as *const u8 as *const libc::c_char,
        );
        return std::ptr::null_mut::<libc::c_char>();
      }
      return xasprintf(
        b"%s=%.*s\x00" as *const u8 as *const libc::c_char,
        name,
        val_len,
        (option as *mut libc::c_char).offset(4),
      );
    }
    _ => {
      bb_error_msg(
        b"can\'t find option name for 0x%x, skipping\x00" as *const u8 as *const libc::c_char,
        *option.offset(1) as libc::c_int,
      );
      return std::ptr::null_mut::<libc::c_char>();
    }
  };
}
/* put all the parameters into the environment */
unsafe extern "C" fn option_to_env(mut option: *const u8, mut option_end: *const u8) {
  let mut addrs: libc::c_int = 0;
  let mut option_offset: libc::c_int = 0;
  /* "length minus 4" */
  let mut len_m4: libc::c_int =
    (option_end.wrapping_offset_from(option) as libc::c_long - 4i32 as libc::c_long) as libc::c_int;
  while len_m4 >= 0 {
    let mut v32: u32 = 0;
    let mut ipv6str: [libc::c_char; 40] = [0; 40];
    if *option.offset(0) as libc::c_int != 0 || *option.offset(2) as libc::c_int != 0 {
      break;
    }
    /* Check if option-length exceeds size of option */
    if *option.offset(3) as libc::c_int > len_m4 {
      break;
    }
    let mut current_block_28: u64;
    match *option.offset(1) as libc::c_int {
      3 | 25 => {
        //case D6_OPT_CLIENTID:
        //case D6_OPT_SERVERID:
        option_to_env(
          option.offset(16),
          option
            .offset(4)
            .offset(*option.offset(3) as libc::c_int as isize),
        );
        current_block_28 = 5141539773904409130;
      }
      5 => {
        /*  0                   1                   2                   3
         *  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
         * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
         * |          OPTION_IAADDR        |          option-len           |
         * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
         * |                                                               |
         * |                         IPv6 address                          |
         * |                                                               |
         * |                                                               |
         * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
         * |                      preferred-lifetime                       |
         * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
         * |                        valid-lifetime                         |
         * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
         */
        /* Make sure payload contains an address */
        if (*option.offset(3) as libc::c_int) < 24i32 {
          current_block_28 = 5141539773904409130;
        } else {
          sprint_nip6(ipv6str.as_mut_ptr(), option.offset(4));
          let ref mut fresh3 = *new_env();
          *fresh3 = xasprintf(
            b"ipv6=%s\x00" as *const u8 as *const libc::c_char,
            ipv6str.as_mut_ptr(),
          );
          v32 = *(option.offset(4).offset(16).offset(4) as *mut bb__aliased_u32);
          let ref mut fresh4 = *new_env();
          *fresh4 = xasprintf(b"lease=%u\x00" as *const u8 as *const libc::c_char, v32);
          current_block_28 = 5141539773904409130;
        }
      }
      26 => {
        //case D6_OPT_ORO:
        //case D6_OPT_PREFERENCE:
        //case D6_OPT_ELAPSED_TIME:
        //case D6_OPT_RELAY_MSG:
        //case D6_OPT_AUTH:
        //case D6_OPT_UNICAST:
        //case D6_OPT_STATUS_CODE:
        //case D6_OPT_RAPID_COMMIT:
        //case D6_OPT_USER_CLASS:
        //case D6_OPT_VENDOR_CLASS:
        //case D6_OPT_VENDOR_OPTS:
        //case D6_OPT_INTERFACE_ID:
        //case D6_OPT_RECONF_MSG:
        //case D6_OPT_RECONF_ACCEPT:
        /*  0                   1                   2                   3
         *  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
         * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
         * |        OPTION_IAPREFIX        |         option-length         |
         * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
         * |                      preferred-lifetime                       |
         * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
         * |                        valid-lifetime                         |
         * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
         * | prefix-length |                                               |
         * +-+-+-+-+-+-+-+-+          IPv6 prefix                          |
         * |                           (16 octets)                         |
         * |                                                               |
         * |               +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
         * |               |
         * +-+-+-+-+-+-+-+-+
         */
        v32 = *(option.offset(4).offset(4) as *mut bb__aliased_u32);
        let ref mut fresh5 = *new_env();
        *fresh5 = xasprintf(
          b"ipv6prefix_lease=%u\x00" as *const u8 as *const libc::c_char,
          v32,
        );
        sprint_nip6(
          ipv6str.as_mut_ptr(),
          option.offset(4).offset(4).offset(4).offset(1),
        );
        let ref mut fresh6 = *new_env();
        *fresh6 = xasprintf(
          b"ipv6prefix=%s/%u\x00" as *const u8 as *const libc::c_char,
          ipv6str.as_mut_ptr(),
          *option.offset((4i32 + 4i32 + 4i32) as isize) as libc::c_uint,
        );
        current_block_28 = 5141539773904409130;
      }
      23 => {
        let mut dlist: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
        /* Make sure payload-size is a multiple of 16 */
        if *option.offset(3) as libc::c_int & 0xfi32 != 0 {
          current_block_28 = 5141539773904409130;
        } else {
          /* Get the number of addresses on the option */
          addrs = *option.offset(3) as libc::c_int >> 4i32;
          /* Setup environment variable */
          dlist = xmalloc((4i32 + addrs * 40i32 - 1i32) as size_t) as *mut libc::c_char;
          let ref mut fresh7 = *new_env();
          *fresh7 = dlist;
          dlist = stpcpy(dlist, b"dns=\x00" as *const u8 as *const libc::c_char);
          option_offset = 0;
          loop {
            let fresh8 = addrs;
            addrs = addrs - 1;
            if !(fresh8 != 0) {
              break;
            }
            sprint_nip6(dlist, option.offset(4).offset(option_offset as isize));
            dlist = dlist.offset(39);
            option_offset += 16i32;
            if addrs != 0 {
              let fresh9 = dlist;
              dlist = dlist.offset(1);
              *fresh9 = ' ' as i32 as libc::c_char
            }
          }
          current_block_28 = 5141539773904409130;
        }
      }
      24 => {
        let mut dlist_0: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
        dlist_0 = dname_dec(
          option.offset(4),
          (*option.offset(2) as libc::c_int) << 8i32 | *option.offset(3) as libc::c_int,
          b"search=\x00" as *const u8 as *const libc::c_char,
        );
        if dlist_0.is_null() {
          current_block_28 = 5141539773904409130;
        } else {
          let ref mut fresh10 = *new_env();
          *fresh10 = dlist_0;
          current_block_28 = 5141539773904409130;
        }
      }
      39 => {
        let mut dlist_1: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
        if *option.offset(3) as libc::c_int == 0 {
          current_block_28 = 5141539773904409130;
        } else {
          /* Work around broken ISC DHCPD6.
           * ISC DHCPD6 does not implement RFC 4704 correctly: It says the first
           * byte of option-payload should contain flags where the bits 7-3 are
           * reserved for future use and MUST be zero. Instead ISC DHCPD6 just
           * writes the entire FQDN as string to option-payload. We assume a
           * broken server here if any of the reserved bits are set.
           */
          if *option.offset(4) as libc::c_int & 0xf8i32 != 0 {
            let ref mut fresh11 = *new_env();
            *fresh11 = xasprintf(
              b"fqdn=%.*s\x00" as *const u8 as *const libc::c_char,
              *option.offset(3) as libc::c_int,
              (option as *mut libc::c_char).offset(4),
            )
          } else {
            dlist_1 = dname_dec(
              option.offset(5),
              *option.offset(3) as libc::c_int - 1i32,
              b"fqdn=\x00" as *const u8 as *const libc::c_char,
            );
            if !dlist_1.is_null() {
              let ref mut fresh12 = *new_env();
              *fresh12 = dlist_1
            }
          }
          current_block_28 = 5141539773904409130;
        }
      }
      41 => {
        /* RFC 4833 Timezones */
        let ref mut fresh13 = *new_env();
        *fresh13 = xasprintf(
          b"tz=%.*s\x00" as *const u8 as *const libc::c_char,
          *option.offset(3) as libc::c_int,
          (option as *mut libc::c_char).offset(4),
        );
        current_block_28 = 5141539773904409130;
      }
      42 => {
        let ref mut fresh14 = *new_env();
        *fresh14 = xasprintf(
          b"tz_name=%.*s\x00" as *const u8 as *const libc::c_char,
          *option.offset(3) as libc::c_int,
          (option as *mut libc::c_char).offset(4),
        );
        current_block_28 = 5141539773904409130;
      }
      59 | 60 | 209 => {
        /* DHCP_PXE_CONF_FILE */
        current_block_28 = 3160140712158701372;
      }
      210 => {
        current_block_28 = 3160140712158701372;
      }
      _ => {
        current_block_28 = 5141539773904409130;
      }
    }
    match current_block_28 {
      3160140712158701372 =>
      /* DHCP_PXE_PATH_PREFIX */
      {
        let mut tmp: *mut libc::c_char = string_option_to_env(option, option_end);
        if !tmp.is_null() {
          let ref mut fresh15 = *new_env();
          *fresh15 = tmp
        }
      }
      _ => {}
    }
    len_m4 -= 4i32 + *option.offset(3) as libc::c_int;
    option = option.offset((4i32 + *option.offset(3) as libc::c_int) as isize)
  }
}
unsafe extern "C" fn fill_envp(
  mut option: *const u8,
  mut option_end: *const u8,
) -> *mut *mut libc::c_char {
  let mut envp: *mut *mut libc::c_char = std::ptr::null_mut();
  let mut curr: *mut *mut libc::c_char = std::ptr::null_mut();
  let ref mut fresh16 = (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
    (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
      .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
  ) as *mut libc::c_char as *mut client6_data_t))
    .env_ptr;
  *fresh16 = std::ptr::null_mut();
  (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
    (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
      .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
  ) as *mut libc::c_char as *mut client6_data_t))
    .env_idx = 0 as libc::c_uint;
  let ref mut fresh17 = *new_env();
  *fresh17 = xasprintf(
    b"interface=%s\x00" as *const u8 as *const libc::c_char,
    (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .interface,
  );
  if !option.is_null() {
    option_to_env(option, option_end);
  }
  curr = (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
    (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
      .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
  ) as *mut libc::c_char as *mut client6_data_t))
    .env_ptr;
  envp = curr;
  while !(*curr).is_null() {
    let fresh18 = curr;
    curr = curr.offset(1);
    putenv(*fresh18);
  }
  return envp;
}
/* Call a script with a par file and env vars */
unsafe extern "C" fn d6_run_script(
  mut option: *const u8,
  mut option_end: *const u8,
  mut name: *const libc::c_char,
) {
  let mut envp: *mut *mut libc::c_char = std::ptr::null_mut();
  let mut curr: *mut *mut libc::c_char = std::ptr::null_mut();
  let mut argv: [*mut libc::c_char; 3] = [0 as *mut libc::c_char; 3];
  envp = fill_envp(option, option_end);
  /* call script */
  if dhcp_verbose >= 1i32 as libc::c_uint {
    bb_info_msg(
      b"executing %s %s\x00" as *const u8 as *const libc::c_char,
      (*(&mut *bb_common_bufsiz1
        .as_mut_ptr()
        .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
        as *mut client_data_t))
        .script,
      name,
    );
  }
  argv[0] = (*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
    as *mut client_data_t))
    .script as *mut libc::c_char;
  argv[1] = name as *mut libc::c_char;
  argv[2] = std::ptr::null_mut::<libc::c_char>();
  spawn_and_wait(argv.as_mut_ptr());
  curr = envp;
  while !(*curr).is_null() {
    if dhcp_verbose >= 2i32 as libc::c_uint {
      bb_info_msg(b" %s\x00" as *const u8 as *const libc::c_char, *curr);
    }
    bb_unsetenv_and_free(*curr);
    curr = curr.offset(1)
  }
  free(envp as *mut libc::c_void);
}
/* Call a script with a par file and no env var */
unsafe extern "C" fn d6_run_script_no_option(mut name: *const libc::c_char) {
  d6_run_script(0 as *const u8, 0 as *const u8, name);
}
/* ** Sending/receiving packets ***/
#[inline(always)]
unsafe extern "C" fn random_xid() -> u32 {
  let mut t: u32 = rand() as libc::c_uint
    & ({
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = 0xffffffi32 as libc::c_uint;
      if false {
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
    });
  return t;
}
/* Initialize the packet with the proper defaults */
unsafe extern "C" fn init_d6_packet(
  mut packet: *mut d6_packet,
  mut type_0: libc::c_char,
  mut xid: u32,
) -> *mut u8 {
  let mut ptr: *mut u8 = std::ptr::null_mut();
  let mut clientid: *mut d6_option = std::ptr::null_mut();
  let mut secs: libc::c_uint = 0;
  memset(
    packet as *mut libc::c_void,
    0,
    ::std::mem::size_of::<d6_packet>() as libc::c_ulong,
  );
  (*packet).d6_u.d6_xid32 = xid;
  (*packet).d6_u.d6_msg_type = type_0 as u8;
  /* ELAPSED_TIME option is required to be present by the RFC,
   * and some servers do check for its presense. [which?]
   */
  ptr = (*packet).d6_options.as_mut_ptr(); /* NB: it is 32-bit aligned */
  *(ptr as *mut u32) = {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = ((8i32 << 16i32) + 2i32) as libc::c_uint;
    if false {
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
  ptr = ptr.offset(4);
  (*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
    as *mut client_data_t))
    .last_secs = monotonic_sec();
  if (*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
    as *mut client_data_t))
    .first_secs
    == 0 as libc::c_uint
  {
    (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .first_secs = (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
      as *mut libc::c_char as *mut client_data_t))
      .last_secs
  }
  secs = (*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
    as *mut client_data_t))
    .last_secs
    .wrapping_sub(
      (*(&mut *bb_common_bufsiz1
        .as_mut_ptr()
        .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
        as *mut client_data_t))
        .first_secs,
    );
  *(ptr as *mut u16) = if secs < 0xffffi32 as libc::c_uint {
    ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = secs as libc::c_ushort;
      if false {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh25 = &mut __v;
        let fresh26;
        let fresh27 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh26) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh25, fresh27))
                          : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh25, fresh27, fresh26);
      }
      __v
    }) as libc::c_int
  } else {
    0xffffi32
  } as u16;
  ptr = ptr.offset(2);
  /* add CLIENTID option */
  clientid = (*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
    as *mut client_data_t))
    .clientid as *mut libc::c_void as *mut d6_option;
  return mempcpy(
    ptr as *mut libc::c_void,
    clientid as *const libc::c_void,
    ((*clientid).len as libc::c_int + 2i32 + 2i32) as size_t,
  ) as *mut u8;
}
unsafe extern "C" fn add_d6_client_options(mut ptr: *mut u8) -> *mut u8 {
  let mut curr: *mut option_set = std::ptr::null_mut();
  let mut start: *mut u8 = ptr;
  let mut option: libc::c_uint = 0;
  let mut len: u16 = 0;
  ptr = ptr.offset(4);
  option = 1i32 as libc::c_uint;
  while option < 256i32 as libc::c_uint {
    if (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .opt_mask[(option >> 3i32) as usize] as libc::c_int
      & 1i32 << (option & 7i32 as libc::c_uint)
      != 0
    {
      *ptr.offset(0) = (option >> 8i32) as u8;
      *ptr.offset(1) = option as u8;
      ptr = ptr.offset(2)
    }
    option = option.wrapping_add(1)
  }
  if ptr.wrapping_offset_from(start) as libc::c_long - 4i32 as libc::c_long != 0 {
    *start.offset(0) = (6i32 >> 8i32) as u8;
    *start.offset(1) = 6i32 as u8;
    *start.offset(2) =
      (ptr.wrapping_offset_from(start) as libc::c_long - 4i32 as libc::c_long >> 8i32) as u8;
    *start.offset(3) =
      (ptr.wrapping_offset_from(start) as libc::c_long - 4i32 as libc::c_long) as u8
  } else {
    ptr = start
  }
  ptr = mempcpy(
    ptr as *mut libc::c_void,
    &opt_fqdn_req as *const [libc::c_char; 6] as *const libc::c_void,
    ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
  ) as *mut u8;
  /* Add -x options if any */
  curr = (*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
    as *mut client_data_t))
    .options;
  while !curr.is_null() {
    len = ((*(*curr).data.offset(2) as libc::c_int) << 8i32
      | *(*curr).data.offset((2i32 + 1i32) as isize) as libc::c_int) as u16;
    ptr = mempcpy(
      ptr as *mut libc::c_void,
      (*curr).data as *const libc::c_void,
      (4i32 + len as libc::c_int) as size_t,
    ) as *mut u8;
    curr = (*curr).next
  }
  return ptr;
}
unsafe extern "C" fn d6_mcast_from_client_data_ifindex(
  mut packet: *mut d6_packet,
  mut end: *mut u8,
) -> libc::c_int {
  /* FF02::1:2 is "All_DHCP_Relay_Agents_and_Servers" address */
  static mut FF02__1_2: [u8; 16] = [
    0xffi32 as u8,
    0x2i32 as u8,
    0 as u8,
    0 as u8,
    0 as u8,
    0 as u8,
    0 as u8,
    0 as u8,
    0 as u8,
    0 as u8,
    0 as u8,
    0 as u8,
    0 as u8,
    0x1i32 as u8,
    0 as u8,
    0x2i32 as u8,
  ];
  return d6_send_raw_packet(
    packet,
    end.wrapping_offset_from(packet as *mut u8) as libc::c_long as libc::c_uint,
    &mut (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
    ) as *mut libc::c_char as *mut client6_data_t))
      .ll_ip6,
    546i32,
    FF02__1_2.as_ptr() as *mut in6_addr,
    547i32,
    MAC_BCAST_ADDR.as_ptr(),
    (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .ifindex,
  );
}
/* RFC 3315 18.1.5. Creation and Transmission of Information-request Messages
 *
 * The client uses an Information-request message to obtain
 * configuration information without having addresses assigned to it.
 *
 * The client sets the "msg-type" field to INFORMATION-REQUEST.  The
 * client generates a transaction ID and inserts this value in the
 * "transaction-id" field.
 *
 * The client SHOULD include a Client Identifier option to identify
 * itself to the server.  If the client does not include a Client
 * Identifier option, the server will not be able to return any client-
 * specific options to the client, or the server may choose not to
 * respond to the message at all.  The client MUST include a Client
 * Identifier option if the Information-Request message will be
 * authenticated.
 *
 * The client MUST include an Option Request option (see section 22.7)
 * to indicate the options the client is interested in receiving.  The
 * client MAY include options with data values as hints to the server
 * about parameter values the client would like to have returned.
 */
/* NOINLINE: limit stack usage in caller */
#[inline(never)]
unsafe extern "C" fn send_d6_info_request(mut xid: u32) -> libc::c_int {
  let mut packet: d6_packet = d6_packet {
    d6_u: C2RustUnnamed_8 { d6_msg_type: 0 },
    d6_options: [0; 604],
  };
  let mut opt_ptr: *mut u8 = std::ptr::null_mut();
  /* Fill in: msg type, client id */
  opt_ptr = init_d6_packet(&mut packet, 11i32 as libc::c_char, xid);
  /* Add options:
   * "param req" option according to -O, options specified with -x
   */
  opt_ptr = add_d6_client_options(opt_ptr);
  bb_error_msg(
    b"sending %s\x00" as *const u8 as *const libc::c_char,
    b"info request\x00" as *const u8 as *const libc::c_char,
  );
  return d6_mcast_from_client_data_ifindex(&mut packet, opt_ptr);
}
/* Milticast a DHCPv6 Solicit packet to the network, with an optionally requested IP.
*
* RFC 3315 17.1.1. Creation of Solicit Messages
*
* The client MUST include a Client Identifier option to identify itself
* to the server.  The client includes IA options for any IAs to which
* it wants the server to assign addresses.  The client MAY include
* addresses in the IAs as a hint to the server about addresses for
* which the client has a preference. ...
*
* The client uses IA_NA options to request the assignment of non-
* temporary addresses and uses IA_TA options to request the assignment
* of temporary addresses.  Either IA_NA or IA_TA options, or a
* combination of both, can be included in DHCP messages.
*
* The client SHOULD include an Option Request option (see section 22.7)
* to indicate the options the client is interested in receiving.  The
* client MAY additionally include instances of those options that are
* identified in the Option Request option, with data values as hints to
* the server about parameter values the client would like to have
* returned.
*
* The client includes a Reconfigure Accept option (see section 22.20)
* if the client is willing to accept Reconfigure messages from the
* server.
     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |        OPTION_CLIENTID        |          option-len           |
     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     .                                                               .
     .                              DUID                             .
     .                        (variable length)                      .
     .                                                               .
     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+


     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |          OPTION_IA_NA         |          option-len           |
     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |                        IAID (4 octets)                        |
     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |                              T1                               |
     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |                              T2                               |
     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |                                                               |
     .                         IA_NA-options                         .
     .                                                               .
     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+


     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |          OPTION_IAADDR        |          option-len           |
     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |                                                               |
     |                         IPv6 address                          |
     |                                                               |
     |                                                               |
     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |                      preferred-lifetime                       |
     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |                        valid-lifetime                         |
     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     .                                                               .
     .                        IAaddr-options                         .
     .                                                               .
     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+


     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |           OPTION_ORO          |           option-len          |
     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |    requested-option-code-1    |    requested-option-code-2    |
     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |                              ...                              |
     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+


     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |     OPTION_RECONF_ACCEPT      |               0               |
     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
*/
/* NOINLINE: limit stack usage in caller */
#[inline(never)]
unsafe extern "C" fn send_d6_discover(
  mut xid: u32,
  mut requested_ipv6: *mut in6_addr,
) -> libc::c_int {
  let mut packet: d6_packet = d6_packet {
    d6_u: C2RustUnnamed_8 { d6_msg_type: 0 },
    d6_options: [0; 604],
  };
  let mut opt_ptr: *mut u8 = std::ptr::null_mut();
  let mut len: libc::c_uint = 0;
  /* Fill in: msg type, client id */
  opt_ptr = init_d6_packet(&mut packet, 1i32 as libc::c_char, xid);
  /* Create new IA_NA, optionally with included IAADDR with requested IP */
  free(
    (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
    ) as *mut libc::c_char as *mut client6_data_t))
      .ia_na as *mut libc::c_void,
  ); /* IAID */
  let ref mut fresh28 = (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
    (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
      .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
  ) as *mut libc::c_char as *mut client6_data_t))
    .ia_na;
  *fresh28 = std::ptr::null_mut();
  if option_mask32 & OPT_r as libc::c_int as libc::c_uint != 0 {
    len = if !requested_ipv6.is_null() {
      (2i32 + 2i32 + 4i32 + 4i32 + 4i32 + 2i32 + 2i32 + 16i32 + 4i32) + 4i32
    } else {
      (2i32 + 2i32 + 4i32 + 4i32) + 4i32
    } as libc::c_uint;
    let ref mut fresh29 = (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
    ) as *mut libc::c_char as *mut client6_data_t))
      .ia_na;
    *fresh29 = xzalloc(len as size_t) as *mut d6_option;
    (*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
    ) as *mut libc::c_char as *mut client6_data_t))
      .ia_na)
      .code = 3i32 as u8;
    (*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
    ) as *mut libc::c_char as *mut client6_data_t))
      .ia_na)
      .len = len.wrapping_sub(4i32 as libc::c_uint) as u8;
    *((*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
    ) as *mut libc::c_char as *mut client6_data_t))
      .ia_na)
      .data
      .as_mut_ptr() as *mut bb__aliased_u32) = rand() as bb__aliased_u32;
    if !requested_ipv6.is_null() {
      let mut iaaddr: *mut d6_option = (*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
        (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
          .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
      ) as *mut libc::c_char as *mut client6_data_t))
        .ia_na)
        .data
        .as_mut_ptr()
        .offset(4)
        .offset(4)
        .offset(4) as *mut libc::c_void as *mut d6_option;
      (*iaaddr).code = 5i32 as u8;
      (*iaaddr).len = (16i32 + 4i32 + 4i32) as u8;
      memcpy(
        (*iaaddr).data.as_mut_ptr() as *mut libc::c_void,
        requested_ipv6 as *const libc::c_void,
        16i32 as libc::c_ulong,
      );
    }
    opt_ptr = mempcpy(
      opt_ptr as *mut libc::c_void,
      (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
        (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
          .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
      ) as *mut libc::c_char as *mut client6_data_t))
        .ia_na as *const libc::c_void,
      len as size_t,
    ) as *mut u8
  }
  /* IA_PD */
  free(
    (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
    ) as *mut libc::c_char as *mut client6_data_t))
      .ia_pd as *mut libc::c_void,
  ); /* IAID */
  let ref mut fresh30 = (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
    (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
      .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
  ) as *mut libc::c_char as *mut client6_data_t))
    .ia_pd;
  *fresh30 = std::ptr::null_mut();
  if option_mask32 & OPT_d as libc::c_int as libc::c_uint != 0 {
    len = (2i32 + 2i32 + 4i32 + 4i32 + 4i32) as libc::c_uint;
    let ref mut fresh31 = (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
    ) as *mut libc::c_char as *mut client6_data_t))
      .ia_pd;
    *fresh31 = xzalloc(len as size_t) as *mut d6_option;
    (*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
    ) as *mut libc::c_char as *mut client6_data_t))
      .ia_pd)
      .code = 25i32 as u8;
    (*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
    ) as *mut libc::c_char as *mut client6_data_t))
      .ia_pd)
      .len = len.wrapping_sub(4i32 as libc::c_uint) as u8;
    *((*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
    ) as *mut libc::c_char as *mut client6_data_t))
      .ia_pd)
      .data
      .as_mut_ptr() as *mut bb__aliased_u32) = rand() as bb__aliased_u32;
    opt_ptr = mempcpy(
      opt_ptr as *mut libc::c_void,
      (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
        (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
          .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
      ) as *mut libc::c_char as *mut client6_data_t))
        .ia_pd as *const libc::c_void,
      len as size_t,
    ) as *mut u8
  }
  /* Add options:
   * "param req" option according to -O, options specified with -x
   */
  opt_ptr = add_d6_client_options(opt_ptr);
  bb_info_msg(
    b"sending %s\x00" as *const u8 as *const libc::c_char,
    b"discover\x00" as *const u8 as *const libc::c_char,
  );
  return d6_mcast_from_client_data_ifindex(&mut packet, opt_ptr);
}
/* Multicast a DHCPv6 request message
 *
 * RFC 3315 18.1.1. Creation and Transmission of Request Messages
 *
 * The client uses a Request message to populate IAs with addresses and
 * obtain other configuration information.  The client includes one or
 * more IA options in the Request message.  The server then returns
 * addresses and other information about the IAs to the client in IA
 * options in a Reply message.
 *
 * The client generates a transaction ID and inserts this value in the
 * "transaction-id" field.
 *
 * The client places the identifier of the destination server in a
 * Server Identifier option.
 *
 * The client MUST include a Client Identifier option to identify itself
 * to the server.  The client adds any other appropriate options,
 * including one or more IA options (if the client is requesting that
 * the server assign it some network addresses).
 *
 * The client MUST include an Option Request option (see section 22.7)
 * to indicate the options the client is interested in receiving.  The
 * client MAY include options with data values as hints to the server
 * about parameter values the client would like to have returned.
 *
 * The client includes a Reconfigure Accept option (see section 22.20)
 * indicating whether or not the client is willing to accept Reconfigure
 * messages from the server.
 */
/* NOINLINE: limit stack usage in caller */
#[inline(never)]
unsafe extern "C" fn send_d6_select(mut xid: u32) -> libc::c_int {
  let mut packet: d6_packet = d6_packet {
    d6_u: C2RustUnnamed_8 { d6_msg_type: 0 },
    d6_options: [0; 604],
  };
  let mut opt_ptr: *mut u8 = std::ptr::null_mut();
  /* Fill in: msg type, client id */
  opt_ptr = init_d6_packet(&mut packet, 3i32 as libc::c_char, xid);
  /* server id */
  opt_ptr = mempcpy(
    opt_ptr as *mut libc::c_void,
    (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
    ) as *mut libc::c_char as *mut client6_data_t))
      .server_id as *const libc::c_void,
    ((*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
    ) as *mut libc::c_char as *mut client6_data_t))
      .server_id)
      .len as libc::c_int
      + 2i32
      + 2i32) as size_t,
  ) as *mut u8;
  /* IA NA (contains requested IP) */
  if !(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
    (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
      .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
  ) as *mut libc::c_char as *mut client6_data_t))
    .ia_na
    .is_null()
  {
    opt_ptr = mempcpy(
      opt_ptr as *mut libc::c_void,
      (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
        (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
          .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
      ) as *mut libc::c_char as *mut client6_data_t))
        .ia_na as *const libc::c_void,
      ((*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
        (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
          .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
      ) as *mut libc::c_char as *mut client6_data_t))
        .ia_na)
        .len as libc::c_int
        + 2i32
        + 2i32) as size_t,
    ) as *mut u8
  }
  /* IA PD */
  if !(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
    (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
      .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
  ) as *mut libc::c_char as *mut client6_data_t))
    .ia_pd
    .is_null()
  {
    opt_ptr = mempcpy(
      opt_ptr as *mut libc::c_void,
      (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
        (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
          .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
      ) as *mut libc::c_char as *mut client6_data_t))
        .ia_pd as *const libc::c_void,
      ((*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
        (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
          .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
      ) as *mut libc::c_char as *mut client6_data_t))
        .ia_pd)
        .len as libc::c_int
        + 2i32
        + 2i32) as size_t,
    ) as *mut u8
  }
  /* Add options:
   * "param req" option according to -O, options specified with -x
   */
  opt_ptr = add_d6_client_options(opt_ptr);
  bb_info_msg(
    b"sending %s\x00" as *const u8 as *const libc::c_char,
    b"select\x00" as *const u8 as *const libc::c_char,
  );
  return d6_mcast_from_client_data_ifindex(&mut packet, opt_ptr);
}
/* Unicast or broadcast a DHCP renew message
 *
 * RFC 3315 18.1.3. Creation and Transmission of Renew Messages
 *
 * To extend the valid and preferred lifetimes for the addresses
 * associated with an IA, the client sends a Renew message to the server
 * from which the client obtained the addresses in the IA containing an
 * IA option for the IA.  The client includes IA Address options in the
 * IA option for the addresses associated with the IA.  The server
 * determines new lifetimes for the addresses in the IA according to the
 * administrative configuration of the server.  The server may also add
 * new addresses to the IA.  The server may remove addresses from the IA
 * by setting the preferred and valid lifetimes of those addresses to
 * zero.
 *
 * The server controls the time at which the client contacts the server
 * to extend the lifetimes on assigned addresses through the T1 and T2
 * parameters assigned to an IA.
 *
 * At time T1 for an IA, the client initiates a Renew/Reply message
 * exchange to extend the lifetimes on any addresses in the IA.  The
 * client includes an IA option with all addresses currently assigned to
 * the IA in its Renew message.
 *
 * If T1 or T2 is set to 0 by the server (for an IA_NA) or there are no
 * T1 or T2 times (for an IA_TA), the client may send a Renew or Rebind
 * message, respectively, at the client's discretion.
 *
 * The client sets the "msg-type" field to RENEW.  The client generates
 * a transaction ID and inserts this value in the "transaction-id"
 * field.
 *
 * The client places the identifier of the destination server in a
 * Server Identifier option.
 *
 * The client MUST include a Client Identifier option to identify itself
 * to the server.  The client adds any appropriate options, including
 * one or more IA options.  The client MUST include the list of
 * addresses the client currently has associated with the IAs in the
 * Renew message.
 *
 * The client MUST include an Option Request option (see section 22.7)
 * to indicate the options the client is interested in receiving.  The
 * client MAY include options with data values as hints to the server
 * about parameter values the client would like to have returned.
 */
/* NOINLINE: limit stack usage in caller */
#[inline(never)]
unsafe extern "C" fn send_d6_renew(
  mut xid: u32,
  mut server_ipv6: *mut in6_addr,
  mut our_cur_ipv6: *mut in6_addr,
) -> libc::c_int {
  let mut packet: d6_packet = d6_packet {
    d6_u: C2RustUnnamed_8 { d6_msg_type: 0 },
    d6_options: [0; 604],
  };
  let mut opt_ptr: *mut u8 = std::ptr::null_mut();
  /* Fill in: msg type, client id */
  opt_ptr = init_d6_packet(&mut packet, 3i32 as libc::c_char, xid);
  /* server id */
  opt_ptr = mempcpy(
    opt_ptr as *mut libc::c_void,
    (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
    ) as *mut libc::c_char as *mut client6_data_t))
      .server_id as *const libc::c_void,
    ((*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
    ) as *mut libc::c_char as *mut client6_data_t))
      .server_id)
      .len as libc::c_int
      + 2i32
      + 2i32) as size_t,
  ) as *mut u8;
  /* IA NA (contains requested IP) */
  if !(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
    (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
      .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
  ) as *mut libc::c_char as *mut client6_data_t))
    .ia_na
    .is_null()
  {
    opt_ptr = mempcpy(
      opt_ptr as *mut libc::c_void,
      (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
        (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
          .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
      ) as *mut libc::c_char as *mut client6_data_t))
        .ia_na as *const libc::c_void,
      ((*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
        (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
          .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
      ) as *mut libc::c_char as *mut client6_data_t))
        .ia_na)
        .len as libc::c_int
        + 2i32
        + 2i32) as size_t,
    ) as *mut u8
  }
  /* IA PD */
  if !(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
    (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
      .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
  ) as *mut libc::c_char as *mut client6_data_t))
    .ia_pd
    .is_null()
  {
    opt_ptr = mempcpy(
      opt_ptr as *mut libc::c_void,
      (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
        (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
          .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
      ) as *mut libc::c_char as *mut client6_data_t))
        .ia_pd as *const libc::c_void,
      ((*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
        (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
          .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
      ) as *mut libc::c_char as *mut client6_data_t))
        .ia_pd)
        .len as libc::c_int
        + 2i32
        + 2i32) as size_t,
    ) as *mut u8
  }
  /* Add options:
   * "param req" option according to -O, options specified with -x
   */
  opt_ptr = add_d6_client_options(opt_ptr);
  bb_info_msg(
    b"sending %s\x00" as *const u8 as *const libc::c_char,
    b"renew\x00" as *const u8 as *const libc::c_char,
  );
  if !server_ipv6.is_null() {
    return d6_send_kernel_packet(
      &mut packet,
      opt_ptr.wrapping_offset_from(&mut packet as *mut d6_packet as *mut u8) as libc::c_long
        as libc::c_uint,
      our_cur_ipv6,
      546i32,
      server_ipv6,
      547i32,
      (*(&mut *bb_common_bufsiz1
        .as_mut_ptr()
        .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
        as *mut client_data_t))
        .ifindex,
    );
  }
  return d6_mcast_from_client_data_ifindex(&mut packet, opt_ptr);
}
/* Unicast a DHCP release message */
#[inline(always)]
unsafe extern "C" fn send_d6_release(
  mut server_ipv6: *mut in6_addr,
  mut our_cur_ipv6: *mut in6_addr,
) -> libc::c_int {
  let mut packet: d6_packet = d6_packet {
    d6_u: C2RustUnnamed_8 { d6_msg_type: 0 },
    d6_options: [0; 604],
  };
  let mut opt_ptr: *mut u8 = std::ptr::null_mut();
  /* Fill in: msg type, client id */
  opt_ptr = init_d6_packet(&mut packet, 8i32 as libc::c_char, random_xid());
  /* server id */
  opt_ptr = mempcpy(
    opt_ptr as *mut libc::c_void,
    (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
    ) as *mut libc::c_char as *mut client6_data_t))
      .server_id as *const libc::c_void,
    ((*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
    ) as *mut libc::c_char as *mut client6_data_t))
      .server_id)
      .len as libc::c_int
      + 2i32
      + 2i32) as size_t,
  ) as *mut u8;
  /* IA NA (contains our current IP) */
  if !(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
    (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
      .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
  ) as *mut libc::c_char as *mut client6_data_t))
    .ia_na
    .is_null()
  {
    opt_ptr = mempcpy(
      opt_ptr as *mut libc::c_void,
      (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
        (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
          .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
      ) as *mut libc::c_char as *mut client6_data_t))
        .ia_na as *const libc::c_void,
      ((*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
        (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
          .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
      ) as *mut libc::c_char as *mut client6_data_t))
        .ia_na)
        .len as libc::c_int
        + 2i32
        + 2i32) as size_t,
    ) as *mut u8
  }
  /* IA PD */
  if !(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
    (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
      .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
  ) as *mut libc::c_char as *mut client6_data_t))
    .ia_pd
    .is_null()
  {
    opt_ptr = mempcpy(
      opt_ptr as *mut libc::c_void,
      (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
        (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
          .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
      ) as *mut libc::c_char as *mut client6_data_t))
        .ia_pd as *const libc::c_void,
      ((*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
        (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
          .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
      ) as *mut libc::c_char as *mut client6_data_t))
        .ia_pd)
        .len as libc::c_int
        + 2i32
        + 2i32) as size_t,
    ) as *mut u8
  }
  bb_info_msg(
    b"sending %s\x00" as *const u8 as *const libc::c_char,
    b"release\x00" as *const u8 as *const libc::c_char,
  );
  return d6_send_kernel_packet(
    &mut packet,
    opt_ptr.wrapping_offset_from(&mut packet as *mut d6_packet as *mut u8) as libc::c_long
      as libc::c_uint,
    our_cur_ipv6,
    546i32,
    server_ipv6,
    547i32,
    (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .ifindex,
  );
}
/* Returns -1 on errors that are fatal for the socket, -2 for those that aren't */
/* NOINLINE: limit stack usage in caller */
#[inline(never)]
unsafe extern "C" fn d6_recv_raw_packet(
  mut peer_ipv6: *mut in6_addr,
  mut d6_pkt: *mut d6_packet,
  mut fd: libc::c_int,
) -> libc::c_int {
  let mut bytes: libc::c_int = 0;
  let mut packet: ip6_udp_d6_packet = ip6_udp_d6_packet {
    ip6: ip6_hdr {
      ip6_ctlun: C2RustUnnamed_7 {
        ip6_un1: ip6_hdrctl {
          ip6_un1_flow: 0,
          ip6_un1_plen: 0,
          ip6_un1_nxt: 0,
          ip6_un1_hlim: 0,
        },
      },
      ip6_src: in6_addr {
        __in6_u: C2RustUnnamed {
          __u6_addr8: [0; 16],
        },
      },
      ip6_dst: in6_addr {
        __in6_u: C2RustUnnamed {
          __u6_addr8: [0; 16],
        },
      },
    },
    udp: udphdr {
      c2rust_unnamed: C2RustUnnamed_3 {
        c2rust_unnamed: C2RustUnnamed_5 {
          uh_sport: 0,
          uh_dport: 0,
          uh_ulen: 0,
          uh_sum: 0,
        },
      },
    },
    data: d6_packet {
      d6_u: C2RustUnnamed_8 { d6_msg_type: 0 },
      d6_options: [0; 604],
    },
  };
  bytes = safe_read(
    fd,
    &mut packet as *mut ip6_udp_d6_packet as *mut libc::c_void,
    ::std::mem::size_of::<ip6_udp_d6_packet>() as libc::c_ulong,
  ) as libc::c_int;
  if bytes < 0 {
    if dhcp_verbose >= 1i32 as libc::c_uint {
      bb_simple_info_msg(b"packet read error, ignoring\x00" as *const u8 as *const libc::c_char);
    }
    /* returns -1 */
    return bytes;
  }
  if bytes
    < (::std::mem::size_of::<ip6_hdr>() as libc::c_ulong)
      .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong) as libc::c_int
  {
    if dhcp_verbose >= 1i32 as libc::c_uint {
      bb_simple_info_msg(b"packet is too short, ignoring\x00" as *const u8 as *const libc::c_char);
    }
    return -2i32;
  }
  if (bytes as libc::c_ulong) <
           (::std::mem::size_of::<ip6_hdr>() as
                libc::c_ulong).wrapping_add(({
                                                 let mut __v: libc::c_ushort =
                                                     0;
                                                 let mut __x: libc::c_ushort =
                                                     packet.ip6.ip6_ctlun.ip6_un1.ip6_un1_plen;
                                                 if false {
                                                     __v =
                                                         (__x as libc::c_int
                                                              >> 8i32 &
                                                              0xffi32 |
                                                              (__x as
                                                                   libc::c_int
                                                                   & 0xffi32)
                                                                  << 8i32) as
                                                             libc::c_ushort
                                                 } else {
                                                     let fresh32 = &mut __v;
                                                     let fresh33;
                                                     let fresh34 = __x;
                                                     asm!("rorw $$8, ${0:w}" :
                                                          "=r" (fresh33) : "0"
                                                          (c2rust_asm_casts::AsmCast::cast_in(fresh32, fresh34))
                                                          : "cc");
                                                     c2rust_asm_casts::AsmCast::cast_out(fresh32,
                                                                                         fresh34,
                                                                                         fresh33);
                                                 }
                                                 __v
                                             }) as libc::c_ulong) {
        /* NB: possible down interface, etc. Caller should pause. */
        /* packet is bigger than sizeof(packet), we did partial read */
        if dhcp_verbose >= 1i32 as libc::c_uint {
            bb_simple_info_msg(b"oversized packet, ignoring\x00" as *const u8
                                   as *const libc::c_char);
        }
        return -2i32
    }
  /* ignore any extra garbage bytes */
  bytes =
        (::std::mem::size_of::<ip6_hdr>() as
             libc::c_ulong).wrapping_add(({
                                              let mut __v: libc::c_ushort = 0;
                                              let mut __x: libc::c_ushort =
                                                  packet.ip6.ip6_ctlun.ip6_un1.ip6_un1_plen;
                                              if false {
                                                  __v =
                                                      (__x as libc::c_int >>
                                                           8i32 & 0xffi32 |
                                                           (__x as libc::c_int
                                                                & 0xffi32) <<
                                                               8i32) as
                                                          libc::c_ushort
                                              } else {
                                                  let fresh35 = &mut __v;
                                                  let fresh36;
                                                  let fresh37 = __x;
                                                  asm!("rorw $$8, ${0:w}" :
                                                       "=r" (fresh36) : "0"
                                                       (c2rust_asm_casts::AsmCast::cast_in(fresh35, fresh37))
                                                       : "cc");
                                                  c2rust_asm_casts::AsmCast::cast_out(fresh35,
                                                                                      fresh37,
                                                                                      fresh36);
                                              }
                                              __v
                                          }) as libc::c_ulong) as libc::c_int;
  /* make sure its the right packet for us, and that it passes sanity checks */
  if packet.ip6.ip6_ctlun.ip6_un1.ip6_un1_nxt as libc::c_int != IPPROTO_UDP as libc::c_int
    || packet.ip6.ip6_ctlun.ip6_un2_vfc as libc::c_int >> 4i32 != 6i32
    || packet.udp.c2rust_unnamed.c2rust_unnamed_0.dest as libc::c_int
      != ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = 546i32 as libc::c_ushort;
        if false {
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
      }) as libc::c_int
    || packet.udp.c2rust_unnamed.c2rust_unnamed_0.len as libc::c_int
      != packet.ip6.ip6_ctlun.ip6_un1.ip6_un1_plen as libc::c_int
  {
    if dhcp_verbose >= 1i32 as libc::c_uint {
      bb_simple_info_msg(
        b"unrelated/bogus packet, ignoring\x00" as *const u8 as *const libc::c_char,
      );
    }
    return -2i32;
  }
  //How to do this for ipv6?
  //	/* verify UDP checksum. IP header has to be modified for this */
  //	memset(&packet.ip, 0, offsetof(struct iphdr, protocol));
  //	/* ip.xx fields which are not memset: protocol, check, saddr, daddr */
  //	packet.ip.tot_len = packet.udp.len; /* yes, this is needed */
  //	check = packet.udp.check;
  //	packet.udp.check = 0;
  //	if (check && check != inet_cksum((u16 *)&packet, bytes)) {
  //		log1("packet with bad UDP checksum received, ignoring");
  //		return -2;
  //	}
  if !peer_ipv6.is_null() {
    *peer_ipv6 = packet.ip6.ip6_src
  } /* struct copy */
  if dhcp_verbose >= 1i32 as libc::c_uint {
    bb_info_msg(
      b"received %s\x00" as *const u8 as *const libc::c_char,
      b"a packet\x00" as *const u8 as *const libc::c_char,
    );
  }
  d6_dump_packet(&mut packet.data);
  bytes = (bytes as libc::c_ulong).wrapping_sub(
    (::std::mem::size_of::<ip6_hdr>() as libc::c_ulong)
      .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong),
  ) as libc::c_int as libc::c_int;
  memcpy(
    d6_pkt as *mut libc::c_void,
    &mut packet.data as *mut d6_packet as *const libc::c_void,
    bytes as libc::c_ulong,
  );
  return bytes;
}
unsafe extern "C" fn d6_raw_socket(mut ifindex: libc::c_int) -> libc::c_int {
  let mut fd: libc::c_int = 0;
  let mut sock: sockaddr_ll = sockaddr_ll {
    sll_family: 0,
    sll_protocol: 0,
    sll_ifindex: 0,
    sll_hatype: 0,
    sll_pkttype: 0,
    sll_halen: 0,
    sll_addr: [0; 8],
  };
  /*
   * Comment:
   *
   *	I've selected not to see LL header, so BPF doesn't see it, too.
   *	The filter may also pass non-IP and non-ARP packets, but we do
   *	a more complete check when receiving the message in userspace.
   *
   * and filter shamelessly stolen from:
   *
   *	http://www.flamewarmaster.de/software/dhcpclient/
   *
   * There are a few other interesting ideas on that page (look under
   * "Motivation").  Use of netlink events is most interesting.  Think
   * of various network servers listening for events and reconfiguring.
   * That would obsolete sending HUP signals and/or make use of restarts.
   *
   * Copyright: 2006, 2007 Stefan Rompf <sux@loplof.de>.
   * License: GPL v2.
   *
   * TODO: make conditional?
   */
  if dhcp_verbose >= 2i32 as libc::c_uint {
    bb_info_msg(
      b"opening raw socket on ifindex %d\x00" as *const u8 as *const libc::c_char,
      ifindex,
    ); /* let's be deterministic */
  }
  fd = xsocket(
    17i32,
    SOCK_DGRAM as libc::c_int,
    ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = 0x86ddi32 as libc::c_ushort;
      if false {
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
    }) as libc::c_int,
  );
  memset(
    &mut sock as *mut sockaddr_ll as *mut libc::c_void,
    0,
    ::std::mem::size_of::<sockaddr_ll>() as libc::c_ulong,
  );
  sock.sll_family = 17i32 as libc::c_ushort;
  sock.sll_protocol = {
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = 0x86ddi32 as libc::c_ushort;
    if false {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh44 = &mut __v;
      let fresh45;
      let fresh46 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh45) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh44, fresh46)) :
                      "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh44, fresh46, fresh45);
    }
    __v
  };
  sock.sll_ifindex = ifindex;
  /*sock.sll_hatype = ARPHRD_???;*/
  /*sock.sll_pkttype = PACKET_???;*/
  /*sock.sll_halen = ???;*/
  /*sock.sll_addr[8] = ???;*/
  xbind(
    fd,
    &mut sock as *mut sockaddr_ll as *mut sockaddr,
    ::std::mem::size_of::<sockaddr_ll>() as libc::c_ulong as socklen_t,
  );
  if dhcp_verbose >= 1i32 as libc::c_uint {
    bb_simple_info_msg(b"created raw socket\x00" as *const u8 as *const libc::c_char);
  }
  return fd;
}
unsafe extern "C" fn change_listen_mode(mut new_mode: libc::c_int) {
  if dhcp_verbose >= 1i32 as libc::c_uint {
    bb_info_msg(
      b"entering listen mode: %s\x00" as *const u8 as *const libc::c_char,
      if new_mode != 0 {
        if new_mode == 1i32 {
          b"kernel\x00" as *const u8 as *const libc::c_char
        } else {
          b"raw\x00" as *const u8 as *const libc::c_char
        }
      } else {
        b"none\x00" as *const u8 as *const libc::c_char
      },
    );
  }
  (*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
    as *mut client_data_t))
    .listen_mode = new_mode as smallint;
  if (*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
    as *mut client_data_t))
    .sockfd
    >= 0
  {
    close(
      (*(&mut *bb_common_bufsiz1
        .as_mut_ptr()
        .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
        as *mut client_data_t))
        .sockfd,
    );
    (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .sockfd = -1i32
  }
  if new_mode == 1i32 {
    (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .sockfd = udhcp_listen_socket(
      546i32,
      (*(&mut *bb_common_bufsiz1
        .as_mut_ptr()
        .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
        as *mut client_data_t))
        .interface,
    )
  } else if new_mode != 0 {
    (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .sockfd = d6_raw_socket(
      (*(&mut *bb_common_bufsiz1
        .as_mut_ptr()
        .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
        as *mut client_data_t))
        .ifindex,
    )
  };
  /* else LISTEN_NONE: client_data.sockfd stays closed */
}
/* Called only on SIGUSR1 */
unsafe extern "C" fn perform_renew() {
  bb_simple_info_msg(b"performing DHCP renew\x00" as *const u8 as *const libc::c_char);
  let mut current_block_5: u64;
  match (*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
    as *mut client_data_t))
    .state as libc::c_int
  {
    2 => {
      change_listen_mode(1i32);
      current_block_5 = 10596831121517990951;
    }
    3 | 4 => {
      current_block_5 = 10596831121517990951;
    }
    5 => {
      /* impatient are we? fine, square 1 */
      d6_run_script_no_option(b"deconfig\x00" as *const u8 as *const libc::c_char);
      current_block_5 = 17879264213467329600;
    }
    1 | 6 => {
      current_block_5 = 17879264213467329600;
    }
    0 | _ => {
      current_block_5 = 1394248824506584008;
    }
  }
  match current_block_5 {
    17879264213467329600 => {
      change_listen_mode(2i32);
      (*(&mut *bb_common_bufsiz1
        .as_mut_ptr()
        .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
        as *mut client_data_t))
        .state = 0 as smallint
    }
    10596831121517990951 => {
      (*(&mut *bb_common_bufsiz1
        .as_mut_ptr()
        .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
        as *mut client_data_t))
        .state = 5i32 as smallint
    }
    _ => {}
  };
}
unsafe extern "C" fn perform_d6_release(
  mut server_ipv6: *mut in6_addr,
  mut our_cur_ipv6: *mut in6_addr,
) {
  /* send release packet */
  if (*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
    as *mut client_data_t))
    .state as libc::c_int
    == 2i32
    || (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .state as libc::c_int
      == 3i32
    || (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .state as libc::c_int
      == 4i32
    || (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .state as libc::c_int
      == 5i32
  {
    bb_simple_info_msg(b"unicasting a release\x00" as *const u8 as *const libc::c_char);
    send_d6_release(server_ipv6, our_cur_ipv6);
    /* unicast */
  }
  bb_simple_info_msg(b"entering released state\x00" as *const u8 as *const libc::c_char);
  /*
   * We can be here on: SIGUSR2,
   * or on exit (SIGTERM) and -R "release on quit" is specified.
   * Users requested to be notified in all cases, even if not in one
   * of the states above.
   */
  d6_run_script_no_option(b"deconfig\x00" as *const u8 as *const libc::c_char);
  change_listen_mode(0i32);
  (*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
    as *mut client_data_t))
    .state = 6i32 as smallint;
}
// /static u8* alloc_dhcp_option(int code, const char *str, int extra)
// /{
// /	u8 *storage;
// /	int len = strnlen(str, 255);
// /	storage = xzalloc(len + extra + OPT_DATA);
// /	storage[OPT_CODE] = code;
// /	storage[OPT_LEN] = len + extra;
// /	memcpy(storage + extra + OPT_DATA, str, len);
// /	return storage;
// /}
unsafe extern "C" fn client_background() {
  bb_daemonize_or_rexec(0i32);
  logmode = (logmode as libc::c_int & !(LOGMODE_STDIO as libc::c_int)) as smallint;
  /* rewrite pidfile, as our pid is different now */
  write_pidfile(
    (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .pidfile,
  );
}
//usage:#if defined CONFIG_UDHCP_DEBUG && CONFIG_UDHCP_DEBUG >= 1
//usage:# define IF_UDHCP_VERBOSE(...) __VA_ARGS__
//usage:#else
//usage:# define IF_UDHCP_VERBOSE(...)
//usage:#endif
//usage:#define udhcpc6_trivial_usage
//usage:       "[-fbnq"IF_UDHCP_VERBOSE("v")"odR] [-i IFACE] [-r IPv6] [-s PROG] [-p PIDFILE]\n"
//usage:       "	[-x OPT:VAL]... [-O OPT]..." IF_FEATURE_UDHCP_PORT(" [-P N]")
//usage:#define udhcpc6_full_usage "\n"
//usage:     "\n	-i IFACE	Interface to use (default eth0)"
//usage:     "\n	-p FILE		Create pidfile"
//usage:     "\n	-s PROG		Run PROG at DHCP events (default "CONFIG_UDHCPC_DEFAULT_SCRIPT")"
//usage:     "\n	-B		Request broadcast replies"
//usage:     "\n	-t N		Send up to N discover packets"
//usage:     "\n	-T N		Pause between packets (default 3 seconds)"
//usage:     "\n	-A N		Wait N seconds (default 20) after failure"
//usage:     "\n	-f		Run in foreground"
//usage:	USE_FOR_MMU(
//usage:     "\n	-b		Background if lease is not obtained"
//usage:	)
//usage:     "\n	-n		Exit if lease is not obtained"
//usage:     "\n	-q		Exit after obtaining lease"
//usage:     "\n	-R		Release IP on exit"
//usage:     "\n	-S		Log to syslog too"
//usage:	IF_FEATURE_UDHCP_PORT(
//usage:     "\n	-P N		Use port N (default 546)"
//usage:	)
// //usage:	IF_FEATURE_UDHCPC_ARPING(
// //usage:     "\n	-a		Use arping to validate offered address"
// //usage:	)
//usage:     "\n	-O OPT		Request option OPT from server (cumulative)"
//usage:     "\n	-o		Don't request any options (unless -O is given)"
//usage:     "\n	-r IPv6		Request this address ('no' to not request any IP)"
//usage:     "\n	-d		Request prefix"
//usage:     "\n	-l		Send 'information request' instead of 'solicit'"
//usage:     "\n			(used for servers which do not assign IPv6 addresses)"
//usage:     "\n	-x OPT:VAL	Include option OPT in sent packets (cumulative)"
//usage:     "\n			Examples of string, numeric, and hex byte opts:"
//usage:     "\n			-x hostname:bbox - option 12"
//usage:     "\n			-x lease:3600 - option 51 (lease time)"
//usage:     "\n			-x 0x3d:0100BEEFC0FFEE - option 61 (client id)"
//usage:     "\n			-x 14:'\"dumpfile\"' - option 14 (shell-quoted)"
//usage:	IF_UDHCP_VERBOSE(
//usage:     "\n	-v		Verbose"
//usage:	)
//usage:     "\nSignals:"
//usage:     "\n	USR1	Renew lease"
//usage:     "\n	USR2	Release lease"
#[no_mangle]
pub unsafe extern "C" fn udhcpc6_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut lease_seconds: u32 = 0; /* must be signed */
  let mut option: *mut d6_option = std::ptr::null_mut();
  let mut address_timeout: libc::c_uint = 0;
  let mut prefix_timeout: libc::c_uint = 0;
  let mut current_block: u64;
  let mut str_r: *const libc::c_char = std::ptr::null();
  let mut clientid_mac_ptr: *mut libc::c_void = std::ptr::null_mut();
  let mut list_O: *mut llist_t = std::ptr::null_mut();
  let mut list_x: *mut llist_t = std::ptr::null_mut();
  let mut tryagain_timeout: libc::c_int = 20i32;
  let mut discover_timeout: libc::c_int = 3i32;
  let mut discover_retries: libc::c_int = 3i32;
  let mut srv6_buf: in6_addr = in6_addr {
    __in6_u: C2RustUnnamed {
      __u6_addr8: [0; 16],
    },
  };
  let mut ipv6_buf: in6_addr = in6_addr {
    __in6_u: C2RustUnnamed {
      __u6_addr8: [0; 16],
    },
  };
  let mut requested_ipv6: *mut in6_addr = std::ptr::null_mut();
  let mut xid: u32 = 0 as u32;
  let mut packet_num: libc::c_int = 0;
  let mut timeout: libc::c_int = 0;
  let mut already_waited_sec: libc::c_uint = 0;
  let mut opt: libc::c_uint = 0;
  let mut retval: libc::c_int = 0;
  /* We want random_xid to be random */
  srand(monotonic_us() as libc::c_uint);
  /* Default options */
  let ref mut fresh47 = (*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
    as *mut libc::c_char as *mut client_data_t))
    .interface;
  *fresh47 = b"eth0\x00" as *const u8 as *const libc::c_char;
  let ref mut fresh48 = (*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
    as *mut libc::c_char as *mut client_data_t))
    .script;
  *fresh48 = b"/usr/share/udhcpc/default.script\x00" as *const u8 as *const libc::c_char;
  (*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
    as *mut client_data_t))
    .sockfd = -1i32;
  /* Make sure fd 0,1,2 are open */
  /* Set up the signal pipe on fds 3,4 - must be before openlog() */
  udhcp_sp_setup();
  /* Parse command line */
  opt = getopt32long(
    argv,
    b"^i:np:qRr:s:T:+t:+SA:+O:*ox:*fldbv\x00vv\x00" as *const u8 as *const libc::c_char,
    udhcpc6_longopts.as_ptr(),
    &mut (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .interface as *mut *const libc::c_char,
    &mut (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .pidfile as *mut *mut libc::c_char,
    &mut str_r as *mut *const libc::c_char,
    &mut (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .script as *mut *const libc::c_char,
    &mut discover_timeout as *mut libc::c_int,
    &mut discover_retries as *mut libc::c_int,
    &mut tryagain_timeout as *mut libc::c_int,
    &mut list_O as *mut *mut llist_t,
    &mut list_x as *mut *mut llist_t,
    &mut dhcp_verbose as *mut libc::c_uint,
  );
  requested_ipv6 = std::ptr::null_mut();
  option_mask32 |= OPT_r as libc::c_int as libc::c_uint;
  if opt & OPT_l as libc::c_int as libc::c_uint != 0 {
    /* for -l, do not require IPv6 assignment from server */
    option_mask32 &= !(OPT_r as libc::c_int) as libc::c_uint
  } else if opt & OPT_r as libc::c_int as libc::c_uint != 0 {
    /* explicit "-r ARG" given */
    if strcmp(str_r, b"no\x00" as *const u8 as *const libc::c_char) == 0 {
      option_mask32 &= !(OPT_r as libc::c_int) as libc::c_uint
    } else {
      if inet_pton(
        10i32,
        str_r,
        &mut ipv6_buf as *mut in6_addr as *mut libc::c_void,
      ) <= 0
      {
        bb_error_msg_and_die(
          b"bad IPv6 address \'%s\'\x00" as *const u8 as *const libc::c_char,
          str_r,
        );
      }
      requested_ipv6 = &mut ipv6_buf
    }
  }
  while !list_O.is_null() {
    let mut optstr: *mut libc::c_char = llist_pop(&mut list_O) as *mut libc::c_char;
    let mut n: libc::c_uint = bb_strtou(optstr, 0 as *mut *mut libc::c_char, 0);
    if *bb_errno != 0 || n > 254i32 as libc::c_uint {
      n = udhcp_option_idx(optstr, d6_option_strings.as_ptr());
      n = d6_optflags[n as usize].code as libc::c_uint
    }
    let ref mut fresh49 = (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
      as *mut libc::c_char as *mut client_data_t))
      .opt_mask[(n >> 3i32) as usize];
    *fresh49 = (*fresh49 as libc::c_int | 1i32 << (n & 7i32 as libc::c_uint)) as u8
  }
  if opt & OPT_o as libc::c_int as libc::c_uint == 0 {
    let mut i: libc::c_uint = 0;
    let mut n_0: libc::c_uint = 0;
    i = 0 as libc::c_uint;
    loop {
      n_0 = d6_optflags[i as usize].code as libc::c_uint;
      if !(n_0 != 0 as libc::c_uint) {
        break;
      }
      if d6_optflags[i as usize].flags as libc::c_int & OPTION_REQ as libc::c_int != 0 {
        let ref mut fresh50 = (*(&mut *bb_common_bufsiz1
          .as_mut_ptr()
          .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
          as *mut libc::c_char as *mut client_data_t))
          .opt_mask[(n_0 >> 3i32) as usize];
        *fresh50 = (*fresh50 as libc::c_int | 1i32 << (n_0 & 7i32 as libc::c_uint)) as u8
      }
      i = i.wrapping_add(1)
    }
  }
  while !list_x.is_null() {
    let mut optstr_0: *mut libc::c_char = xstrdup(llist_pop(&mut list_x) as *const libc::c_char);
    udhcp_str2optset(
      optstr_0,
      &mut (*(&mut *bb_common_bufsiz1
        .as_mut_ptr()
        .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
        as *mut client_data_t))
        .options as *mut *mut option_set as *mut libc::c_void,
      d6_optflags.as_ptr(),
      d6_option_strings.as_ptr(),
      1i32 != 0,
    );
    free(optstr_0 as *mut libc::c_void);
  }
  if d6_read_interface(
    (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .interface,
    &mut (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .ifindex,
    &mut (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong) as isize,
    ) as *mut libc::c_char as *mut client6_data_t))
      .ll_ip6,
    (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .client_mac
      .as_mut_ptr(),
  ) != 0
  {
    return 1i32;
  }
  /* Create client ID based on mac, set clientid_mac_ptr */
  let mut clientid: *mut d6_option = std::ptr::null_mut(); /* DUID-LL */
  clientid = xzalloc((2i32 + 2i32 + 2i32 + 2i32 + 6i32) as size_t) as *mut d6_option; /* ethernet */
  (*clientid).code = 1i32 as u8;
  (*clientid).len = (2i32 + 2i32 + 6i32) as u8;
  *(*clientid).data.as_mut_ptr().offset(1) = 3i32 as u8;
  *(*clientid).data.as_mut_ptr().offset(3) = 1i32 as u8;
  clientid_mac_ptr = (*clientid).data.as_mut_ptr().offset(2).offset(2) as *mut libc::c_void;
  memcpy(
    clientid_mac_ptr,
    (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .client_mac
      .as_mut_ptr() as *const libc::c_void,
    6i32 as libc::c_ulong,
  );
  let ref mut fresh51 = (*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
    as *mut libc::c_char as *mut client_data_t))
    .clientid;
  *fresh51 = clientid as *mut libc::c_void as *mut u8;
  if opt & OPT_S as libc::c_int as libc::c_uint != 0 {
    openlog(applet_name, 0x1i32, 3i32 << 3i32);
    logmode = (logmode as libc::c_int | LOGMODE_SYSLOG as libc::c_int) as smallint
  }
  /* Create pidfile */
  write_pidfile(
    (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .pidfile,
  );
  /* Goes to stdout (unless NOMMU) and possibly syslog */
  bb_simple_info_msg(b"started, v1.32.0.git\x00" as *const u8 as *const libc::c_char);
  (*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
    as *mut client_data_t))
    .state = 0 as smallint;
  d6_run_script_no_option(b"deconfig\x00" as *const u8 as *const libc::c_char);
  change_listen_mode(2i32);
  packet_num = 0;
  timeout = 0;
  already_waited_sec = 0 as libc::c_uint;
  /* Main event loop. select() waits on signal pipe and possibly
   * on sockfd.
   * "continue" statements in code below jump to the top of the loop.
   */
  loop {
    let mut tv: libc::c_int = 0;
    let mut pfds: [pollfd; 2] = [pollfd {
      fd: 0,
      events: 0,
      revents: 0,
    }; 2]; /* for (;;) - main loop ends */
    let mut packet: d6_packet = d6_packet {
      d6_u: C2RustUnnamed_8 { d6_msg_type: 0 },
      d6_options: [0; 604],
    };
    let mut packet_end: *mut u8 = std::ptr::null_mut();
    /* back to main loop */
    /* silence "uninitialized!" warning */
    let mut timestamp_before_wait: libc::c_uint = 0;
    timestamp_before_wait = timestamp_before_wait;
    //bb_error_msg("sockfd:%d, listen_mode:%d", client_data.sockfd, client_data.listen_mode);
    /* Was opening raw or udp socket here
     * if (client_data.listen_mode != LISTEN_NONE && client_data.sockfd < 0),
     * but on fast network renew responses return faster
     * than we open sockets. Thus this code is moved
     * to change_listen_mode(). Thus we open listen socket
     * BEFORE we send renew request (see "case BOUND:"). */
    udhcp_sp_fd_set(
      pfds.as_mut_ptr(),
      (*(&mut *bb_common_bufsiz1
        .as_mut_ptr()
        .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
        as *mut client_data_t))
        .sockfd,
    );
    tv = (timeout as libc::c_uint).wrapping_sub(already_waited_sec) as libc::c_int;
    retval = 0;
    /* If we already timed out, fall through with retval = 0, else... */
    if tv > 0 {
      if dhcp_verbose >= 1i32 as libc::c_uint {
        bb_info_msg(
          b"waiting %u seconds\x00" as *const u8 as *const libc::c_char,
          tv,
        );
      }
      timestamp_before_wait = monotonic_sec();
      retval = poll(
        pfds.as_mut_ptr(),
        2i32 as nfds_t,
        if tv < 2147483647i32 / 1000i32 {
          (tv) * 1000i32
        } else {
          2147483647i32
        },
      );
      if retval < 0 {
        /* EINTR? A signal was caught, don't panic */
        if *bb_errno == 4i32 {
          already_waited_sec =
            already_waited_sec.wrapping_add(monotonic_sec().wrapping_sub(timestamp_before_wait));
          continue;
        } else {
          /* Else: an error occured, panic! */
          bb_simple_perror_msg_and_die(b"poll\x00" as *const u8 as *const libc::c_char);
        }
      }
    }
    /* If timeout dropped to zero, time to become active:
     * resend discover/renew/whatever
     */
    if retval == 0 {
      /* if poll timed out */
      /* When running on a bridge, the ifindex may have changed
       * (e.g. if member interfaces were added/removed
       * or if the status of the bridge changed).
       * Refresh ifindex and client_mac:
       */
      if d6_read_interface(
        (*(&mut *bb_common_bufsiz1
          .as_mut_ptr()
          .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
          as *mut client_data_t))
          .interface,
        &mut (*(&mut *bb_common_bufsiz1
          .as_mut_ptr()
          .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
          as *mut libc::c_char as *mut client_data_t))
          .ifindex,
        &mut (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
          (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong)
            as isize,
        ) as *mut libc::c_char as *mut client6_data_t))
          .ll_ip6,
        (*(&mut *bb_common_bufsiz1
          .as_mut_ptr()
          .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
          as *mut client_data_t))
          .client_mac
          .as_mut_ptr(),
      ) != 0
      {
        current_block = 4436526174799533647;
        break;
      /* iface is gone? */
      } else {
        memcpy(
          clientid_mac_ptr,
          (*(&mut *bb_common_bufsiz1
            .as_mut_ptr()
            .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
            as *mut libc::c_char as *mut client_data_t))
            .client_mac
            .as_mut_ptr() as *const libc::c_void,
          6i32 as libc::c_ulong,
        );
        /* We will restart the wait in any case */
        already_waited_sec = 0 as libc::c_uint;
        match (*(&mut *bb_common_bufsiz1
          .as_mut_ptr()
          .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
          as *mut libc::c_char as *mut client_data_t))
          .state as libc::c_int
        {
          0 => {
            current_block = 1265706858151309666;
            match current_block {
              5250576585193495047 => {
                /* yah, I know, *you* say it would never happen */
                timeout = 2147483647i32;
                continue;
              }
              8823050030433040401 => {
                if discover_retries == 0 || packet_num < discover_retries {
                  /* send multicast select packet */
                  send_d6_select(xid);
                  timeout = discover_timeout;
                  packet_num += 1;
                  continue;
                } else {
                  /* Timed out, go back to init state.
                   * "discover...select...discover..." loops
                   * were seen in the wild. Treat them similarly
                   * to "no response to discover" case */
                  change_listen_mode(2i32);
                  (*(&mut *bb_common_bufsiz1
                    .as_mut_ptr()
                    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                    as *mut libc::c_char as *mut client_data_t))
                    .state = 0 as smallint
                }
                current_block = 1431398736652930548;
              }
              1265706858151309666 => {
                if discover_retries == 0 || packet_num < discover_retries {
                  if packet_num == 0 {
                    xid = random_xid()
                  }
                  /* multicast */
                  if opt & OPT_l as libc::c_int as libc::c_uint != 0 {
                    send_d6_info_request(xid);
                  } else {
                    send_d6_discover(xid, requested_ipv6);
                  }
                  timeout = discover_timeout;
                  packet_num += 1;
                  continue;
                } else {
                  current_block = 1431398736652930548;
                }
                /* case RELEASED: */
              }
              _ => {
                /* 1/2 lease passed, enter renewing state */
                (*(&mut *bb_common_bufsiz1
                  .as_mut_ptr()
                  .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                  as *mut libc::c_char as *mut client_data_t))
                  .state = 3i32 as smallint; /* make secs field count from 0 */
                (*(&mut *bb_common_bufsiz1
                  .as_mut_ptr()
                  .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                  as *mut libc::c_char as *mut client_data_t))
                  .first_secs = 0 as libc::c_uint;
                change_listen_mode(1i32);
                if dhcp_verbose >= 1i32 as libc::c_uint {
                  bb_simple_info_msg(
                    b"entering renew state\x00" as *const u8 as *const libc::c_char,
                  );
                }
                /* fall right through */
                current_block = 9539714035271414315;
              }
            }
            match current_block {
              9539714035271414315 => {}
              _ => {
                d6_run_script_no_option(b"leasefail\x00" as *const u8 as *const libc::c_char);
                /* -b is not supported on NOMMU */
                if opt & OPT_b as libc::c_int as libc::c_uint != 0 {
                  /* background if no lease */
                  bb_simple_info_msg(
                    b"no lease, forking to background\x00" as *const u8 as *const libc::c_char,
                  );
                  client_background();
                  /* ^^^ also disables -n (-b takes priority over -n):
                   * ifup's default udhcpc options are -R -n,
                   * and users want to be able to add -b
                   * (in a config file) to make it background
                   * _and not exit_.
                   */
                  opt = opt & !(OPT_b as libc::c_int | OPT_n as libc::c_int) as libc::c_uint
                    | OPT_f as libc::c_int as libc::c_uint
                } else if opt & OPT_n as libc::c_int as libc::c_uint != 0 {
                  /* do not background again! */
                  /* abort if no lease */
                  bb_simple_info_msg(b"no lease, failing\x00" as *const u8 as *const libc::c_char);
                  retval = 1i32;
                  current_block = 2938061015511697447;
                  break;
                }
                /* wait before trying again */
                timeout = tryagain_timeout;
                packet_num = 0;
                continue;
              }
            }
          }
          1 => {
            current_block = 8823050030433040401;
            match current_block {
              5250576585193495047 => {
                timeout = 2147483647i32;
                continue;
              }
              8823050030433040401 => {
                if discover_retries == 0 || packet_num < discover_retries {
                  send_d6_select(xid);
                  timeout = discover_timeout;
                  packet_num += 1;
                  continue;
                } else {
                  change_listen_mode(2i32);
                  (*(&mut *bb_common_bufsiz1
                    .as_mut_ptr()
                    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                    as *mut libc::c_char as *mut client_data_t))
                    .state = 0 as smallint
                }
                current_block = 1431398736652930548;
              }
              1265706858151309666 => {
                if discover_retries == 0 || packet_num < discover_retries {
                  if packet_num == 0 {
                    xid = random_xid()
                  }
                  if opt & OPT_l as libc::c_int as libc::c_uint != 0 {
                    send_d6_info_request(xid);
                  } else {
                    send_d6_discover(xid, requested_ipv6);
                  }
                  timeout = discover_timeout;
                  packet_num += 1;
                  continue;
                } else {
                  current_block = 1431398736652930548;
                }
              }
              _ => {
                (*(&mut *bb_common_bufsiz1
                  .as_mut_ptr()
                  .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                  as *mut libc::c_char as *mut client_data_t))
                  .state = 3i32 as smallint;
                (*(&mut *bb_common_bufsiz1
                  .as_mut_ptr()
                  .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                  as *mut libc::c_char as *mut client_data_t))
                  .first_secs = 0 as libc::c_uint;
                change_listen_mode(1i32);
                if dhcp_verbose >= 1i32 as libc::c_uint {
                  bb_simple_info_msg(
                    b"entering renew state\x00" as *const u8 as *const libc::c_char,
                  );
                }
                current_block = 9539714035271414315;
              }
            }
            match current_block {
              9539714035271414315 => {}
              _ => {
                d6_run_script_no_option(b"leasefail\x00" as *const u8 as *const libc::c_char);
                if opt & OPT_b as libc::c_int as libc::c_uint != 0 {
                  bb_simple_info_msg(
                    b"no lease, forking to background\x00" as *const u8 as *const libc::c_char,
                  );
                  client_background();
                  opt = opt & !(OPT_b as libc::c_int | OPT_n as libc::c_int) as libc::c_uint
                    | OPT_f as libc::c_int as libc::c_uint
                } else if opt & OPT_n as libc::c_int as libc::c_uint != 0 {
                  bb_simple_info_msg(b"no lease, failing\x00" as *const u8 as *const libc::c_char);
                  retval = 1i32;
                  current_block = 2938061015511697447;
                  break;
                }
                timeout = tryagain_timeout;
                packet_num = 0;
                continue;
              }
            }
          }
          2 => {
            current_block = 4931485695408934665;
            match current_block {
              5250576585193495047 => {
                timeout = 2147483647i32;
                continue;
              }
              8823050030433040401 => {
                if discover_retries == 0 || packet_num < discover_retries {
                  send_d6_select(xid);
                  timeout = discover_timeout;
                  packet_num += 1;
                  continue;
                } else {
                  change_listen_mode(2i32);
                  (*(&mut *bb_common_bufsiz1
                    .as_mut_ptr()
                    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                    as *mut libc::c_char as *mut client_data_t))
                    .state = 0 as smallint
                }
                current_block = 1431398736652930548;
              }
              1265706858151309666 => {
                if discover_retries == 0 || packet_num < discover_retries {
                  if packet_num == 0 {
                    xid = random_xid()
                  }
                  if opt & OPT_l as libc::c_int as libc::c_uint != 0 {
                    send_d6_info_request(xid);
                  } else {
                    send_d6_discover(xid, requested_ipv6);
                  }
                  timeout = discover_timeout;
                  packet_num += 1;
                  continue;
                } else {
                  current_block = 1431398736652930548;
                }
              }
              _ => {
                (*(&mut *bb_common_bufsiz1
                  .as_mut_ptr()
                  .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                  as *mut libc::c_char as *mut client_data_t))
                  .state = 3i32 as smallint;
                (*(&mut *bb_common_bufsiz1
                  .as_mut_ptr()
                  .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                  as *mut libc::c_char as *mut client_data_t))
                  .first_secs = 0 as libc::c_uint;
                change_listen_mode(1i32);
                if dhcp_verbose >= 1i32 as libc::c_uint {
                  bb_simple_info_msg(
                    b"entering renew state\x00" as *const u8 as *const libc::c_char,
                  );
                }
                current_block = 9539714035271414315;
              }
            }
            match current_block {
              9539714035271414315 => {}
              _ => {
                d6_run_script_no_option(b"leasefail\x00" as *const u8 as *const libc::c_char);
                if opt & OPT_b as libc::c_int as libc::c_uint != 0 {
                  bb_simple_info_msg(
                    b"no lease, forking to background\x00" as *const u8 as *const libc::c_char,
                  );
                  client_background();
                  opt = opt & !(OPT_b as libc::c_int | OPT_n as libc::c_int) as libc::c_uint
                    | OPT_f as libc::c_int as libc::c_uint
                } else if opt & OPT_n as libc::c_int as libc::c_uint != 0 {
                  bb_simple_info_msg(b"no lease, failing\x00" as *const u8 as *const libc::c_char);
                  retval = 1i32;
                  current_block = 2938061015511697447;
                  break;
                }
                timeout = tryagain_timeout;
                packet_num = 0;
                continue;
              }
            }
          }
          5 | 3 => {
            current_block = 9539714035271414315;
          }
          4 => {
            current_block = 11699506720926175598;
          }
          _ => {
            current_block = 5250576585193495047;
            match current_block {
              5250576585193495047 => {
                timeout = 2147483647i32;
                continue;
              }
              8823050030433040401 => {
                if discover_retries == 0 || packet_num < discover_retries {
                  send_d6_select(xid);
                  timeout = discover_timeout;
                  packet_num += 1;
                  continue;
                } else {
                  change_listen_mode(2i32);
                  (*(&mut *bb_common_bufsiz1
                    .as_mut_ptr()
                    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                    as *mut libc::c_char as *mut client_data_t))
                    .state = 0 as smallint
                }
                current_block = 1431398736652930548;
              }
              1265706858151309666 => {
                if discover_retries == 0 || packet_num < discover_retries {
                  if packet_num == 0 {
                    xid = random_xid()
                  }
                  if opt & OPT_l as libc::c_int as libc::c_uint != 0 {
                    send_d6_info_request(xid);
                  } else {
                    send_d6_discover(xid, requested_ipv6);
                  }
                  timeout = discover_timeout;
                  packet_num += 1;
                  continue;
                } else {
                  current_block = 1431398736652930548;
                }
              }
              _ => {
                (*(&mut *bb_common_bufsiz1
                  .as_mut_ptr()
                  .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                  as *mut libc::c_char as *mut client_data_t))
                  .state = 3i32 as smallint;
                (*(&mut *bb_common_bufsiz1
                  .as_mut_ptr()
                  .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                  as *mut libc::c_char as *mut client_data_t))
                  .first_secs = 0 as libc::c_uint;
                change_listen_mode(1i32);
                if dhcp_verbose >= 1i32 as libc::c_uint {
                  bb_simple_info_msg(
                    b"entering renew state\x00" as *const u8 as *const libc::c_char,
                  );
                }
                current_block = 9539714035271414315;
              }
            }
            match current_block {
              9539714035271414315 => {}
              _ => {
                d6_run_script_no_option(b"leasefail\x00" as *const u8 as *const libc::c_char);
                if opt & OPT_b as libc::c_int as libc::c_uint != 0 {
                  bb_simple_info_msg(
                    b"no lease, forking to background\x00" as *const u8 as *const libc::c_char,
                  );
                  client_background();
                  opt = opt & !(OPT_b as libc::c_int | OPT_n as libc::c_int) as libc::c_uint
                    | OPT_f as libc::c_int as libc::c_uint
                } else if opt & OPT_n as libc::c_int as libc::c_uint != 0 {
                  bb_simple_info_msg(b"no lease, failing\x00" as *const u8 as *const libc::c_char);
                  retval = 1i32;
                  current_block = 2938061015511697447;
                  break;
                }
                timeout = tryagain_timeout;
                packet_num = 0;
                continue;
              }
            }
          }
        }
      }
    /* back to main loop */
    } else {
      /* poll() didn't timeout, something happened */
      /* Is it a signal? */
      match udhcp_sp_read() {
        10 => {
          (*(&mut *bb_common_bufsiz1
            .as_mut_ptr()
            .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
            as *mut libc::c_char as *mut client_data_t))
            .first_secs = 0 as libc::c_uint; /* make secs field count from 0 */
          already_waited_sec = 0 as libc::c_uint;
          perform_renew();
          if (*(&mut *bb_common_bufsiz1
            .as_mut_ptr()
            .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
            as *mut libc::c_char as *mut client_data_t))
            .state as libc::c_int
            == 5i32
          {
            /* We might be either on the same network
             * (in which case renew might work),
             * or we might be on a completely different one
             * (in which case renew won't ever succeed).
             * For the second case, must make sure timeout
             * is not too big, or else we can send
             * futile renew requests for hours.
             */
            if timeout > 60i32 {
              timeout = 60i32
            }
          } else {
            /* Start things over */
            packet_num = 0;
            /* Kill any timeouts, user wants this to hurry along */
            timeout = 0;
            continue;
          }
        }
        12 => {
          perform_d6_release(&mut srv6_buf, requested_ipv6);
          timeout = 2147483647i32;
          continue;
        }
        15 => {
          bb_info_msg(
            b"received %s\x00" as *const u8 as *const libc::c_char,
            b"SIGTERM\x00" as *const u8 as *const libc::c_char,
          );
          current_block = 4436526174799533647;
          break;
        }
        _ => {
          /* Is it a packet? */
          if pfds[1].revents == 0 {
            continue; /* no */
          }
          let mut len: libc::c_int = 0;
          /* A packet is ready, read it */
          if (*(&mut *bb_common_bufsiz1
            .as_mut_ptr()
            .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
            as *mut libc::c_char as *mut client_data_t))
            .listen_mode as libc::c_int
            == 1i32
          {
            len = d6_recv_kernel_packet(
              &mut srv6_buf,
              &mut packet,
              (*(&mut *bb_common_bufsiz1
                .as_mut_ptr()
                .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                as *mut libc::c_char as *mut client_data_t))
                .sockfd,
            )
          } else {
            len = d6_recv_raw_packet(
              &mut srv6_buf,
              &mut packet,
              (*(&mut *bb_common_bufsiz1
                .as_mut_ptr()
                .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                as *mut libc::c_char as *mut client_data_t))
                .sockfd,
            )
          }
          if len == -1i32 {
            /* Error is severe, reopen socket */
            bb_error_msg(
              b"read error: %m, reopening socket\x00" as *const u8 as *const libc::c_char,
            );
            /* just close and reopen */
            sleep(discover_timeout as libc::c_uint); /* 3 seconds by default */
            change_listen_mode(
              (*(&mut *bb_common_bufsiz1
                .as_mut_ptr()
                .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                as *mut libc::c_char as *mut client_data_t))
                .listen_mode as libc::c_int,
            );
          }
          /* If this packet will turn out to be unrelated/bogus,
           * we will go back and wait for next one.
           * Be sure timeout is properly decreased. */
          already_waited_sec =
            already_waited_sec.wrapping_add(monotonic_sec().wrapping_sub(timestamp_before_wait));
          if len < 0 {
            continue;
          }
          packet_end = (&mut packet as *mut d6_packet as *mut u8).offset(len as isize);
          if packet.d6_u.d6_xid32
            & ({
              let mut __v: libc::c_uint = 0;
              let mut __x: libc::c_uint = 0xffffffi32 as libc::c_uint;
              if false {
                __v = (__x & 0xff000000u32) >> 24i32
                  | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
                  | (__x & 0xff00i32 as libc::c_uint) << 8i32
                  | (__x & 0xffi32 as libc::c_uint) << 24i32
              } else {
                let fresh52 = &mut __v;
                let fresh53;
                let fresh54 = __x;
                asm!("bswap $0" : "=r" (fresh53) : "0"
                                         (c2rust_asm_casts::AsmCast::cast_in(fresh52, fresh54))
                                         :);
                c2rust_asm_casts::AsmCast::cast_out(fresh52, fresh54, fresh53);
              }
              __v
            })
            != xid
          {
            if dhcp_verbose >= 1i32 as libc::c_uint {
              bb_info_msg(
                b"xid %x (our is %x), ignoring packet\x00" as *const u8 as *const libc::c_char,
                packet.d6_u.d6_xid32
                  & ({
                    let mut __v: libc::c_uint = 0;
                    let mut __x: libc::c_uint = 0xffffffi32 as libc::c_uint;
                    if false {
                      __v = (__x & 0xff000000u32) >> 24i32
                        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
                        | (__x & 0xff00i32 as libc::c_uint) << 8i32
                        | (__x & 0xffi32 as libc::c_uint) << 24i32
                    } else {
                      let fresh55 = &mut __v;
                      let fresh56;
                      let fresh57 = __x;
                      asm!("bswap $0" : "=r"
                                                          (fresh56) : "0"
                                                          (c2rust_asm_casts::AsmCast::cast_in(fresh55, fresh57))
                                                          :);
                      c2rust_asm_casts::AsmCast::cast_out(fresh55, fresh57, fresh56);
                    }
                    __v
                  }),
                xid,
              );
            }
            continue;
          } else {
            match (*(&mut *bb_common_bufsiz1
              .as_mut_ptr()
              .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
              as *mut libc::c_char as *mut client_data_t))
              .state as libc::c_int
            {
              0 => {
                if packet.d6_u.d6_msg_type as libc::c_int == 2i32 {
                  current_block = 18191924429319573516;
                } else {
                  current_block = 7608327953539086561;
                }
                /* case BOUND: - ignore all packets */
                /* case RELEASED: - ignore all packets */
              }
              1 | 3 | 5 | 4 => {
                current_block = 7608327953539086561;
              }
              _ => {
                continue;
              }
            }
            match current_block {
              7608327953539086561 =>
              /* DHCPv6 has "Rapid Commit", when instead of Advertise,
               * server sends Reply right away.
               * Fall through to check for this case.
               */
              {
                if !(packet.d6_u.d6_msg_type as libc::c_int == 7i32) {
                  continue;
                }
                lease_seconds = 0;
                option = std::ptr::null_mut();
                address_timeout = 0;
                prefix_timeout = 0;
                /* back to main loop */
              }
              _ => {}
            }
            address_timeout = 0 as libc::c_uint;
            prefix_timeout = 0 as libc::c_uint;
            option = d6_find_option(
              packet.d6_options.as_mut_ptr(),
              packet_end,
              13i32 as libc::c_uint,
            ) as *mut d6_option;
            if !option.is_null()
              && *(*option).data.as_mut_ptr().offset(0) as libc::c_int
                | *(*option).data.as_mut_ptr().offset(1) as libc::c_int
                != 0
            {
              /* return to init state */
              bb_info_msg(
                b"received DHCP NAK (%u)\x00" as *const u8 as *const libc::c_char,
                *(*option).data.as_mut_ptr().offset(4) as libc::c_int,
              ); /* avoid excessive network traffic */
              d6_run_script(
                packet.d6_options.as_mut_ptr(),
                packet_end,
                b"nak\x00" as *const u8 as *const libc::c_char,
              ); /* make secs field count from 0 */
              if (*(&mut *bb_common_bufsiz1
                .as_mut_ptr()
                .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                as *mut libc::c_char as *mut client_data_t))
                .state as libc::c_int
                != 1i32
              {
                d6_run_script_no_option(b"deconfig\x00" as *const u8 as *const libc::c_char);
              }
              change_listen_mode(2i32);
              sleep(3i32 as libc::c_uint);
              (*(&mut *bb_common_bufsiz1
                .as_mut_ptr()
                .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                as *mut libc::c_char as *mut client_data_t))
                .state = 0 as smallint;
              (*(&mut *bb_common_bufsiz1
                .as_mut_ptr()
                .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                as *mut libc::c_char as *mut client_data_t))
                .first_secs = 0 as libc::c_uint;
              requested_ipv6 = std::ptr::null_mut();
              timeout = 0;
              packet_num = 0;
              already_waited_sec = 0 as libc::c_uint;
              continue;
            } else {
              option = d6_copy_option(
                packet.d6_options.as_mut_ptr(),
                packet_end,
                2i32 as libc::c_uint,
              ) as *mut d6_option;
              if option.is_null() {
                bb_simple_info_msg(
                  b"no server ID, ignoring packet\x00" as *const u8 as *const libc::c_char,
                );
                continue;
              /* still selecting - this server looks bad */
              } else {
                //Note: we do not bother comparing server IDs in Advertise and Reply msgs.
                //server_id variable is used solely for creation of proper server_id option
                //in outgoing packets. (why DHCPv6 even introduced it is a mystery).
                free(
                  (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
                    (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
                      .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong)
                      as isize,
                  ) as *mut libc::c_char as *mut client6_data_t))
                    .server_id as *mut libc::c_void,
                );
                let ref mut fresh58 = (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
                  (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
                    .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong)
                    as isize,
                ) as *mut libc::c_char
                  as *mut client6_data_t))
                  .server_id;
                *fresh58 = option;
                if packet.d6_u.d6_msg_type as libc::c_int == 2i32 {
                  /* enter requesting state */
                  (*(&mut *bb_common_bufsiz1
                    .as_mut_ptr()
                    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                    as *mut libc::c_char as *mut client_data_t))
                    .state = 1i32 as smallint;
                  timeout = 0;
                  packet_num = 0;
                  already_waited_sec = 0 as libc::c_uint;
                  continue;
                } else {
                  /* It's a D6_MSG_REPLY */
                  /*
                   * RFC 3315 18.1.8. Receipt of Reply Messages
                   *
                   * Upon the receipt of a valid Reply message in response to a Solicit
                   * (with a Rapid Commit option), Request, Confirm, Renew, Rebind or
                   * Information-request message, the client extracts the configuration
                   * information contained in the Reply.  The client MAY choose to report
                   * any status code or message from the status code option in the Reply
                   * message.
                   *
                   * The client SHOULD perform duplicate address detection [17] on each of
                   * the addresses in any IAs it receives in the Reply message before
                   * using that address for traffic.  If any of the addresses are found to
                   * be in use on the link, the client sends a Decline message to the
                   * server as described in section 18.1.7.
                   *
                   * If the Reply was received in response to a Solicit (with a Rapid
                   * Commit option), Request, Renew or Rebind message, the client updates
                   * the information it has recorded about IAs from the IA options
                   * contained in the Reply message:
                   *
                   * -  Record T1 and T2 times.
                   *
                   * -  Add any new addresses in the IA option to the IA as recorded by
                   *    the client.
                   *
                   * -  Update lifetimes for any addresses in the IA option that the
                   *    client already has recorded in the IA.
                   *
                   * -  Discard any addresses from the IA, as recorded by the client, that
                   *    have a valid lifetime of 0 in the IA Address option.
                   *
                   * -  Leave unchanged any information about addresses the client has
                   *    recorded in the IA but that were not included in the IA from the
                   *    server.
                   *
                   * Management of the specific configuration information is detailed in
                   * the definition of each option in section 22.
                   *
                   * If the client receives a Reply message with a Status Code containing
                   * UnspecFail, the server is indicating that it was unable to process
                   * the message due to an unspecified failure condition.  If the client
                   * retransmits the original message to the same server to retry the
                   * desired operation, the client MUST limit the rate at which it
                   * retransmits the message and limit the duration of the time during
                   * which it retransmits the message.
                   *
                   * When the client receives a Reply message with a Status Code option
                   * with the value UseMulticast, the client records the receipt of the
                   * message and sends subsequent messages to the server through the
                   * interface on which the message was received using multicast.  The
                   * client resends the original message using multicast.
                   *
                   * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
                   * |          OPTION_IA_NA         |          option-len           |
                   * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
                   * |                        IAID (4 octets)                        |
                   * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
                   * |                              T1                               |
                   * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
                   * |                              T2                               |
                   * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
                   * |                                                               |
                   * .                         IA_NA-options                         .
                   * .                                                               .
                   * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
                   *
                   *
                   * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
                   * |          OPTION_IAADDR        |          option-len           |
                   * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
                   * |                                                               |
                   * |                         IPv6 address                          |
                   * |                                                               |
                   * |                                                               |
                   * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
                   * |                      preferred-lifetime                       |
                   * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
                   * |                        valid-lifetime                         |
                   * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
                   * .                                                               .
                   * .                        IAaddr-options                         .
                   * .                                                               .
                   * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
                   */
                  if option_mask32 & OPT_r as libc::c_int as libc::c_uint != 0 {
                    let mut iaaddr: *mut d6_option = std::ptr::null_mut();
                    free(
                      (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
                        (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
                          .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong)
                          as isize,
                      ) as *mut libc::c_char as *mut client6_data_t))
                        .ia_na as *mut libc::c_void,
                    );
                    let ref mut fresh59 = (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
                      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
                        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong)
                        as isize,
                    ) as *mut libc::c_char
                      as *mut client6_data_t))
                      .ia_na;
                    *fresh59 = d6_copy_option(
                      packet.d6_options.as_mut_ptr(),
                      packet_end,
                      3i32 as libc::c_uint,
                    ) as *mut d6_option;
                    if (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
                      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
                        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong)
                        as isize,
                    ) as *mut libc::c_char as *mut client6_data_t))
                      .ia_na
                      .is_null()
                    {
                      bb_info_msg(
                        b"no %s option, ignoring packet\x00" as *const u8 as *const libc::c_char,
                        b"IA_NA\x00" as *const u8 as *const libc::c_char,
                      );
                      continue;
                    } else if ((*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
                      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
                        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong)
                        as isize,
                    ) as *mut libc::c_char
                      as *mut client6_data_t))
                      .ia_na)
                      .len as libc::c_int)
                      < 4i32 + 4i32 + 4i32 + (2i32 + 2i32 + 16i32 + 4i32 + 4i32)
                    {
                      bb_info_msg(
                        b"%s option is too short:%d bytes\x00" as *const u8 as *const libc::c_char,
                        b"IA_NA\x00" as *const u8 as *const libc::c_char,
                        (*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
                          (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
                            .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong)
                            as isize,
                        ) as *mut libc::c_char
                          as *mut client6_data_t))
                          .ia_na)
                          .len as libc::c_int,
                      );
                      continue;
                    } else {
                      iaaddr = d6_find_option(
                        (*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
                          (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
                            .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong)
                            as isize,
                        ) as *mut libc::c_char
                          as *mut client6_data_t))
                          .ia_na)
                          .data
                          .as_mut_ptr()
                          .offset(4)
                          .offset(4)
                          .offset(4),
                        (*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
                          (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
                            .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong)
                            as isize,
                        ) as *mut libc::c_char
                          as *mut client6_data_t))
                          .ia_na)
                          .data
                          .as_mut_ptr()
                          .offset(
                            (*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
                              (COMMON_BUFSIZE as libc::c_int as libc::c_ulong).wrapping_sub(
                                ::std::mem::size_of::<client6_data_t>() as libc::c_ulong,
                              ) as isize,
                            ) as *mut libc::c_char
                              as *mut client6_data_t))
                              .ia_na)
                              .len as libc::c_int as isize,
                          ),
                        5i32 as libc::c_uint,
                      ) as *mut d6_option;
                      if iaaddr.is_null() {
                        bb_info_msg(
                          b"no %s option, ignoring packet\x00" as *const u8 as *const libc::c_char,
                          b"IAADDR\x00" as *const u8 as *const libc::c_char,
                        );
                        continue;
                      } else if ((*iaaddr).len as libc::c_int) < 16i32 + 4i32 + 4i32 {
                        bb_info_msg(
                          b"%s option is too short:%d bytes\x00" as *const u8
                            as *const libc::c_char,
                          b"IAADDR\x00" as *const u8 as *const libc::c_char,
                          (*iaaddr).len as libc::c_int,
                        );
                        continue;
                      } else {
                        /* Note: the address is sufficiently aligned for cast:
                         * we _copied_ IA-NA, and copy is always well-aligned.
                         */
                        requested_ipv6 = (*iaaddr).data.as_mut_ptr() as *mut in6_addr;
                        lease_seconds = *((*iaaddr).data.as_mut_ptr().offset(16).offset(4)
                          as *mut bb__aliased_u32);
                        lease_seconds = {
                          let mut __v: libc::c_uint = 0;
                          let mut __x: libc::c_uint = lease_seconds;
                          if false {
                            __v = (__x & 0xff000000u32) >> 24i32
                              | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
                              | (__x & 0xff00i32 as libc::c_uint) << 8i32
                              | (__x & 0xffi32 as libc::c_uint) << 24i32
                          } else {
                            let fresh60 = &mut __v;
                            let fresh61;
                            let fresh62 = __x;
                            asm!("bswap $0" :
                                                                  "=r"
                                                                  (fresh61) :
                                                                  "0"
                                                                  (c2rust_asm_casts::AsmCast::cast_in(fresh60, fresh62))
                                                                  :);
                            c2rust_asm_casts::AsmCast::cast_out(fresh60, fresh62, fresh61);
                          }
                          __v
                        };
                        // / TODO: check for 0 lease time?
                        bb_info_msg(
                          b"%s obtained, lease time %u\x00" as *const u8 as *const libc::c_char,
                          b"IPv6\x00" as *const u8 as *const libc::c_char,
                          lease_seconds,
                        );
                        address_timeout = lease_seconds
                      }
                    }
                  }
                  if option_mask32 & OPT_d as libc::c_int as libc::c_uint != 0 {
                    let mut iaprefix: *mut d6_option = std::ptr::null_mut();
                    free(
                      (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
                        (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
                          .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong)
                          as isize,
                      ) as *mut libc::c_char as *mut client6_data_t))
                        .ia_pd as *mut libc::c_void,
                    );
                    let ref mut fresh63 = (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
                      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
                        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong)
                        as isize,
                    ) as *mut libc::c_char
                      as *mut client6_data_t))
                      .ia_pd;
                    *fresh63 = d6_copy_option(
                      packet.d6_options.as_mut_ptr(),
                      packet_end,
                      25i32 as libc::c_uint,
                    ) as *mut d6_option;
                    if (*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
                      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
                        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong)
                        as isize,
                    ) as *mut libc::c_char as *mut client6_data_t))
                      .ia_pd
                      .is_null()
                    {
                      bb_info_msg(
                        b"no %s option, ignoring packet\x00" as *const u8 as *const libc::c_char,
                        b"IA_PD\x00" as *const u8 as *const libc::c_char,
                      );
                      continue;
                    } else if ((*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
                      (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
                        .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong)
                        as isize,
                    ) as *mut libc::c_char
                      as *mut client6_data_t))
                      .ia_pd)
                      .len as libc::c_int)
                      < 4i32 + 4i32 + 4i32 + (2i32 + 2i32 + 4i32 + 4i32 + 1i32 + 16i32)
                    {
                      bb_info_msg(
                        b"%s option is too short:%d bytes\x00" as *const u8 as *const libc::c_char,
                        b"IA_PD\x00" as *const u8 as *const libc::c_char,
                        (*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
                          (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
                            .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong)
                            as isize,
                        ) as *mut libc::c_char
                          as *mut client6_data_t))
                          .ia_pd)
                          .len as libc::c_int,
                      );
                      continue;
                    } else {
                      iaprefix = d6_find_option(
                        (*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
                          (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
                            .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong)
                            as isize,
                        ) as *mut libc::c_char
                          as *mut client6_data_t))
                          .ia_pd)
                          .data
                          .as_mut_ptr()
                          .offset(4)
                          .offset(4)
                          .offset(4),
                        (*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
                          (COMMON_BUFSIZE as libc::c_int as libc::c_ulong)
                            .wrapping_sub(::std::mem::size_of::<client6_data_t>() as libc::c_ulong)
                            as isize,
                        ) as *mut libc::c_char
                          as *mut client6_data_t))
                          .ia_pd)
                          .data
                          .as_mut_ptr()
                          .offset(
                            (*(*(&mut *bb_common_bufsiz1.as_mut_ptr().offset(
                              (COMMON_BUFSIZE as libc::c_int as libc::c_ulong).wrapping_sub(
                                ::std::mem::size_of::<client6_data_t>() as libc::c_ulong,
                              ) as isize,
                            ) as *mut libc::c_char
                              as *mut client6_data_t))
                              .ia_pd)
                              .len as libc::c_int as isize,
                          ),
                        26i32 as libc::c_uint,
                      ) as *mut d6_option;
                      if iaprefix.is_null() {
                        bb_info_msg(
                          b"no %s option, ignoring packet\x00" as *const u8 as *const libc::c_char,
                          b"IAPREFIX\x00" as *const u8 as *const libc::c_char,
                        );
                        continue;
                      } else if ((*iaprefix).len as libc::c_int) < 4i32 + 4i32 + 1i32 + 16i32 {
                        bb_info_msg(
                          b"%s option is too short:%d bytes\x00" as *const u8
                            as *const libc::c_char,
                          b"IAPREFIX\x00" as *const u8 as *const libc::c_char,
                          (*iaprefix).len as libc::c_int,
                        );
                        continue;
                      } else {
                        lease_seconds =
                          *((*iaprefix).data.as_mut_ptr().offset(4) as *mut bb__aliased_u32);
                        lease_seconds = {
                          let mut __v: libc::c_uint = 0;
                          let mut __x: libc::c_uint = lease_seconds;
                          if false {
                            __v = (__x & 0xff000000u32) >> 24i32
                              | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
                              | (__x & 0xff00i32 as libc::c_uint) << 8i32
                              | (__x & 0xffi32 as libc::c_uint) << 24i32
                          } else {
                            let fresh64 = &mut __v;
                            let fresh65;
                            let fresh66 = __x;
                            asm!("bswap $0" :
                                                                  "=r"
                                                                  (fresh65) :
                                                                  "0"
                                                                  (c2rust_asm_casts::AsmCast::cast_in(fresh64, fresh66))
                                                                  :);
                            c2rust_asm_casts::AsmCast::cast_out(fresh64, fresh66, fresh65);
                          }
                          __v
                        };
                        bb_info_msg(
                          b"%s obtained, lease time %u\x00" as *const u8 as *const libc::c_char,
                          b"prefix\x00" as *const u8 as *const libc::c_char,
                          lease_seconds,
                        );
                        prefix_timeout = lease_seconds
                      }
                    }
                  }
                  if address_timeout == 0 {
                    address_timeout = prefix_timeout
                  }
                  if prefix_timeout == 0 {
                    prefix_timeout = address_timeout
                  }
                  /* note: "int timeout" will not overflow even with 0xffffffff inputs here: */
                  timeout = (if prefix_timeout < address_timeout {
                    prefix_timeout
                  } else {
                    address_timeout
                  })
                  .wrapping_div(2i32 as libc::c_uint) as libc::c_int;
                  if opt & OPT_l as libc::c_int as libc::c_uint != 0 {
                    /* TODO: request OPTION_INFORMATION_REFRESH_TIME (32)
                     * and use its value instead of the default 1 day.
                     */
                    timeout = 24i32 * 60i32 * 60i32
                  }
                  /* paranoia: must not be too small */
                  /* timeout > 60 - ensures at least one unicast renew attempt */
                  if timeout < 61i32 {
                    timeout = 61i32
                  }
                  /* enter bound state */
                  d6_run_script(
                    packet.d6_options.as_mut_ptr(),
                    packet_end,
                    if (*(&mut *bb_common_bufsiz1
                      .as_mut_ptr()
                      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                      as *mut libc::c_char as *mut client_data_t))
                      .state as libc::c_int
                      == 1i32
                    {
                      b"bound\x00" as *const u8 as *const libc::c_char
                    } else {
                      b"renew\x00" as *const u8 as *const libc::c_char
                    },
                  );
                  (*(&mut *bb_common_bufsiz1
                    .as_mut_ptr()
                    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                    as *mut libc::c_char as *mut client_data_t))
                    .state = 2i32 as smallint;
                  change_listen_mode(0i32);
                  if opt & OPT_q as libc::c_int as libc::c_uint != 0 {
                    current_block = 4436526174799533647;
                    break;
                  }
                  /* future renew failures should not exit (JM) */
                  opt &= !(OPT_n as libc::c_int) as libc::c_uint;
                  /* NOMMU case backgrounded earlier */
                  if opt & OPT_f as libc::c_int as libc::c_uint == 0 {
                    client_background();
                    /* do not background again! */
                    opt = opt & !(OPT_b as libc::c_int) as libc::c_uint
                      | OPT_f as libc::c_int as libc::c_uint
                  }
                  already_waited_sec = 0 as libc::c_uint;
                  continue;
                }
              }
            }
          }
        }
      }
      current_block = 9539714035271414315;
    }
    match current_block {
      9539714035271414315 =>
      /* manual (SIGUSR1) renew */
      {
        if timeout >= 60i32 {
          /* send an unicast renew request */
          /* Sometimes observed to fail (EADDRNOTAVAIL) to bind
           * a new UDP socket for sending inside send_renew.
           * I hazard to guess existing listening socket
           * is somehow conflicting with it, but why is it
           * not deterministic then?! Strange.
           * Anyway, it does recover by eventually failing through
           * into INIT_SELECTING state.
           */
          if opt & OPT_l as libc::c_int as libc::c_uint != 0 {
            send_d6_info_request(xid);
          } else {
            send_d6_renew(xid, &mut srv6_buf, requested_ipv6);
          }
          timeout >>= 1i32;
          continue;
        } else {
          /* Timed out, enter rebinding state */
          if dhcp_verbose >= 1i32 as libc::c_uint {
            bb_simple_info_msg(b"entering rebinding state\x00" as *const u8 as *const libc::c_char);
          }
          (*(&mut *bb_common_bufsiz1
            .as_mut_ptr()
            .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
            as *mut libc::c_char as *mut client_data_t))
            .state = 4i32 as smallint
        }
      }
      _ => {}
    }
    /* fall right through */
    /* Switch to bcast receive */
    change_listen_mode(2i32);
    if timeout > 0 {
      if opt & OPT_l as libc::c_int as libc::c_uint != 0 {
        send_d6_info_request(xid);
      } else {
        /* Lease is *really* about to run out,
         * try to find DHCP server using broadcast */
        /* send a broadcast renew request */
        send_d6_renew(xid, 0 as *mut in6_addr, requested_ipv6);
      }
      timeout >>= 1i32
    } else {
      /* Timed out, enter init state */
      bb_simple_info_msg(
        b"lease lost, entering init state\x00" as *const u8 as *const libc::c_char,
      ); /* make secs field count from 0 */
      d6_run_script_no_option(b"deconfig\x00" as *const u8 as *const libc::c_char);
      (*(&mut *bb_common_bufsiz1
        .as_mut_ptr()
        .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
        as *mut client_data_t))
        .state = 0 as smallint;
      (*(&mut *bb_common_bufsiz1
        .as_mut_ptr()
        .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
        as *mut client_data_t))
        .first_secs = 0 as libc::c_uint;
      /*timeout = 0; - already is */
      packet_num = 0
    }
  }
  match current_block {
    4436526174799533647 =>
    /* quit after lease */
    {
      if opt & OPT_R as libc::c_int as libc::c_uint != 0 {
        /* release on quit */
        perform_d6_release(&mut srv6_buf, requested_ipv6);
      }
      retval = 0
    }
    _ => {}
  }
  /*if (client_data.pidfile) - remove_pidfile has its own check */
  if wrote_pidfile != 0 {
    unlink(
      (*(&mut *bb_common_bufsiz1
        .as_mut_ptr()
        .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
        as *mut client_data_t))
        .pidfile,
    );
  }
  return retval;
}

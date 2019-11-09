use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
use libc;
use libc::unlink;
use libc::close;
use libc::free;
extern "C" {
  #[no_mangle]
  fn openlog(__ident: *const libc::c_char, __option: libc::c_int, __facility: libc::c_int);

  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;

  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;



  #[no_mangle]
  fn sleep(__seconds: libc::c_uint) -> libc::c_uint;



  #[no_mangle]
  fn __cmsg_nxthdr(__mhdr: *mut msghdr, __cmsg: *mut cmsghdr) -> *mut cmsghdr;

  #[no_mangle]
  fn recvmsg(__fd: libc::c_int, __message: *mut msghdr, __flags: libc::c_int) -> ssize_t;

  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn rand() -> libc::c_int;

  #[no_mangle]
  fn srand(__seed: libc::c_uint);



  #[no_mangle]
  fn putenv(__string: *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn strnlen(__string: *const libc::c_char, __maxlen: size_t) -> size_t;

  #[no_mangle]
  fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;

  #[no_mangle]
  fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
  /* Some useful definitions */
  /* Macros for min/max.  */
  /* buffer allocation schemes */
  /* glibc uses __errno_location() to get a ptr to errno */
  /* We can just memorize it once - no multithreading in busybox :) */
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn monotonic_us() -> libc::c_ulonglong;
  #[no_mangle]
  fn monotonic_sec() -> libc::c_uint;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_unsetenv_and_free(key: *mut libc::c_char);
  #[no_mangle]
  fn xsocket(domain: libc::c_int, type_0: libc::c_int, protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xbind(sockfd: libc::c_int, my_addr: *mut sockaddr, addrlen: socklen_t);
  #[no_mangle]
  fn setsockopt_1(fd: libc::c_int, level: libc::c_int, optname: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn inet_cksum(addr: *mut u16, len: libc::c_int) -> u16;
  #[no_mangle]
  fn safe_strncpy(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    size: size_t,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  /* Put a string of hex bytes ("1b2e66fe"...), return advanced pointer */
  #[no_mangle]
  fn bin2hex(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    count: libc::c_int,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn xatou(str: *const libc::c_char) -> libc::c_uint;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  /* ***********************************************************************/
  /* spawn_and_wait/run_nofork_applet/run_applet_no_and_exit need to work */
  /* carefully together to reinit some global state while not disturbing  */
  /* other. Be careful if you change them. Consult docs/nofork_noexec.txt */
  /* ***********************************************************************/
  /* Same as wait4pid(spawn(argv)), but with NOFORK/NOEXEC if configured: */
  #[no_mangle]
  fn spawn_and_wait(argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn bb_daemonize_or_rexec(flags: libc::c_int);
  /* { "-", NULL } */
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
  /* BTW, surprisingly, changing API to
   *   llist_t *llist_add_to(llist_t *old_head, void *data)
   * etc does not result in smaller code... */
  /* start_stop_daemon and udhcpc are special - they want
   * to create pidfiles regardless of FEATURE_PIDFILE */
  /* True only if we created pidfile which is *file*, not /dev/null etc */
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
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_info_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_info_msg(s: *const libc::c_char);
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
  #[no_mangle]
  static MAC_BCAST_ADDR: [u8; 6];
  #[no_mangle]
  static dhcp_optflags: [dhcp_optflag; 0];
  #[no_mangle]
  static dhcp_option_strings: [libc::c_char; 0];
  #[no_mangle]
  static dhcp_option_lengths: [u8; 0];
  #[no_mangle]
  fn udhcp_option_idx(
    name: *const libc::c_char,
    option_strings: *const libc::c_char,
  ) -> libc::c_uint;
  #[no_mangle]
  fn udhcp_get_option(packet: *mut dhcp_packet, code: libc::c_int) -> *mut u8;
  /* Same as above + ensures that option length is 4 bytes
   * (returns NULL if size is different)
   */
  #[no_mangle]
  fn udhcp_get_option32(packet: *mut dhcp_packet, code: libc::c_int) -> *mut u8;
  #[no_mangle]
  fn udhcp_end_option(optionptr: *mut u8) -> libc::c_int;
  #[no_mangle]
  fn udhcp_add_binary_option(packet: *mut dhcp_packet, addopt: *mut u8);
  #[no_mangle]
  fn udhcp_add_simple_option(packet: *mut dhcp_packet, code: u8, data: u32);
  #[no_mangle]
  fn dname_dec(
    cstr: *const u8,
    clen: libc::c_int,
    pre: *const libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn udhcp_find_option(opt_list: *mut option_set, code: u8) -> *mut option_set;
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
  #[no_mangle]
  static mut dhcp_verbose: libc::c_uint;
  #[no_mangle]
  fn udhcp_dump_packet(packet: *mut dhcp_packet);
  /* 2nd param is "struct option_set**" */
  #[no_mangle]
  fn udhcp_str2optset(
    str: *const libc::c_char,
    arg: *mut libc::c_void,
    optflags: *const dhcp_optflag,
    option_strings: *const libc::c_char,
    dhcpv6: bool,
  ) -> libc::c_int;
  #[no_mangle]
  fn udhcp_init_header(packet: *mut dhcp_packet, type_0: libc::c_char);
  #[no_mangle]
  fn udhcp_recv_kernel_packet(packet: *mut dhcp_packet, fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn udhcp_send_raw_packet(
    dhcp_pkt: *mut dhcp_packet,
    source_nip: u32,
    source_port: libc::c_int,
    dest_nip: u32,
    dest_port: libc::c_int,
    dest_arp: *const u8,
    ifindex: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn udhcp_send_kernel_packet(
    dhcp_pkt: *mut dhcp_packet,
    source_nip: u32,
    source_port: libc::c_int,
    dest_nip: u32,
    dest_port: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn udhcp_sp_setup();
  #[no_mangle]
  fn udhcp_sp_fd_set(pfds: *mut pollfd, extra_fd: libc::c_int);
  #[no_mangle]
  fn udhcp_sp_read() -> libc::c_int;
  #[no_mangle]
  fn udhcp_read_interface(
    interface: *const libc::c_char,
    ifindex: *mut libc::c_int,
    nip: *mut u32,
    mac: *mut u8,
  ) -> libc::c_int;
  #[no_mangle]
  fn udhcp_listen_socket(port: libc::c_int, inf: *const libc::c_char) -> libc::c_int;
  /* Returns 1 if no reply received */
  #[no_mangle]
  fn arpping(
    test_nip: u32,
    safe_mac: *const u8,
    from_ip: u32,
    from_mac: *mut u8,
    interface: *const libc::c_char,
    timeo: libc::c_uint,
  ) -> libc::c_int;
  /* note: ip is a pointer to an IPv6 in network order, possibly misaliged */
  #[no_mangle]
  fn sprint_nip6(dest: *mut libc::c_char, ip: *const u8) -> libc::c_int;
}

pub type __socklen_t = libc::c_uint;



pub type bb__aliased_u16 = u16;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
  pub iov_base: *mut libc::c_void,
  pub iov_len: size_t,
}
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
pub struct in_addr {
  pub s_addr: in_addr_t,
}
pub type in_addr_t = u32;
pub type C2RustUnnamed = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed = 256;
pub const IPPROTO_RAW: C2RustUnnamed = 255;
pub const IPPROTO_MPLS: C2RustUnnamed = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed = 136;
pub const IPPROTO_SCTP: C2RustUnnamed = 132;
pub const IPPROTO_COMP: C2RustUnnamed = 108;
pub const IPPROTO_PIM: C2RustUnnamed = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed = 94;
pub const IPPROTO_MTP: C2RustUnnamed = 92;
pub const IPPROTO_AH: C2RustUnnamed = 51;
pub const IPPROTO_ESP: C2RustUnnamed = 50;
pub const IPPROTO_GRE: C2RustUnnamed = 47;
pub const IPPROTO_RSVP: C2RustUnnamed = 46;
pub const IPPROTO_IPV6: C2RustUnnamed = 41;
pub const IPPROTO_DCCP: C2RustUnnamed = 33;
pub const IPPROTO_TP: C2RustUnnamed = 29;
pub const IPPROTO_IDP: C2RustUnnamed = 22;
pub const IPPROTO_UDP: C2RustUnnamed = 17;
pub const IPPROTO_PUP: C2RustUnnamed = 12;
pub const IPPROTO_EGP: C2RustUnnamed = 8;
pub const IPPROTO_TCP: C2RustUnnamed = 6;
pub const IPPROTO_IPIP: C2RustUnnamed = 4;
pub const IPPROTO_IGMP: C2RustUnnamed = 2;
pub const IPPROTO_ICMP: C2RustUnnamed = 1;
pub const IPPROTO_IP: C2RustUnnamed = 0;
pub type nfds_t = libc::c_ulong;
use libc::pollfd;
/* BSD-derived getopt() functions require that optind be set to 1 in
 * order to reset getopt() state.  This used to be generally accepted
 * way of resetting getopt().  However, glibc's getopt()
 * has additional getopt() state beyond optind (specifically, glibc
 * extensions such as '+' and '-' at the start of the string), and requires
 * that optind be set to zero to reset its state.  BSD-derived versions
 * of getopt() misbehaved if optind is set to 0 in order to reset getopt(),
 * and glibc's getopt() used to coredump if optind is set 1 in order
 * to reset getopt().
 * Then BSD introduced additional variable "optreset" which should be
 * set to 1 in order to reset getopt().  Sigh.  Standards, anyone?
 *
 * By ~2008, OpenBSD 3.4 was changed to survive glibc-like optind = 0
 * (to interpret it as if optreset was set).
 */
/*def __GLIBC__*/
/* BSD style */
/* Having next pointer as a first member allows easy creation
 * of "llist-compatible" structs, and using llist_FOO functions
 * on them.
 */
use crate::libbb::llist::llist_t;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const LOGMODE_BOTH: C2RustUnnamed_0 = 3;
pub const LOGMODE_SYSLOG: C2RustUnnamed_0 = 2;
pub const LOGMODE_STDIO: C2RustUnnamed_0 = 1;
pub const LOGMODE_NONE: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const COMMON_BUFSIZE: C2RustUnnamed_1 = 1024;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct udphdr {
  pub c2rust_unnamed: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
  pub c2rust_unnamed: C2RustUnnamed_4,
  pub c2rust_unnamed_0: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
  pub source: u16,
  pub dest: u16,
  pub len: u16,
  pub check: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
  pub uh_sport: u16,
  pub uh_dport: u16,
  pub uh_ulen: u16,
  pub uh_sum: u16,
}
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
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct dhcp_packet {
  pub op: u8,
  pub htype: u8,
  pub hlen: u8,
  pub hops: u8,
  pub xid: u32,
  pub secs: u16,
  pub flags: u16,
  pub ciaddr: u32,
  pub yiaddr: u32,
  pub siaddr_nip: u32,
  pub gateway_nip: u32,
  pub chaddr: [u8; 16],
  pub sname: [u8; 64],
  pub file: [u8; 128],
  pub cookie: u32,
  pub options: [u8; 388],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ip_udp_dhcp_packet {
  pub ip: iphdr,
  pub udp: udphdr,
  pub data: dhcp_packet,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const DHCP_SIZE: C2RustUnnamed_5 = 548;
pub const UDP_DHCP_SIZE: C2RustUnnamed_5 = 556;
pub const IP_UDP_DHCP_SIZE: C2RustUnnamed_5 = 576;
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
/* client -> server */
/* client -> server */
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
pub type __u16 = libc::c_ushort;
pub type u32 = libc::c_uint;
pub type __be16 = __u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_ll {
  pub sll_family: libc::c_ushort,
  pub sll_protocol: __be16,
  pub sll_ifindex: libc::c_int,
  pub sll_hatype: libc::c_ushort,
  pub sll_pkttype: libc::c_uchar,
  pub sll_halen: libc::c_uchar,
  pub sll_addr: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tpacket_auxdata {
  pub tp_status: u32,
  pub tp_len: u32,
  pub tp_snaplen: u32,
  pub tp_mac: __u16,
  pub tp_net: __u16,
  pub tp_vlan_tci: __u16,
  pub tp_vlan_tpid: __u16,
}
/* Must match getopt32 option string order */
pub type C2RustUnnamed_7 = libc::c_uint;
pub const OPT_a: C2RustUnnamed_7 = 4194304;
pub const OPT_b: C2RustUnnamed_7 = 2097152;
pub const OPTBIT_a: C2RustUnnamed_7 = 22;
pub const OPTBIT_b: C2RustUnnamed_7 = 21;
/* The rest has variable bit positions, need to be clever */
pub const OPTBIT_B: C2RustUnnamed_7 = 20;
pub const OPT_B: C2RustUnnamed_7 = 1048576;
pub const OPT_f: C2RustUnnamed_7 = 524288;
pub const OPT_x: C2RustUnnamed_7 = 262144;
pub const OPT_o: C2RustUnnamed_7 = 131072;
pub const OPT_O: C2RustUnnamed_7 = 65536;
pub const OPT_A: C2RustUnnamed_7 = 32768;
pub const OPT_S: C2RustUnnamed_7 = 16384;
pub const OPT_t: C2RustUnnamed_7 = 8192;
pub const OPT_T: C2RustUnnamed_7 = 4096;
pub const OPT_s: C2RustUnnamed_7 = 2048;
pub const OPT_r: C2RustUnnamed_7 = 1024;
pub const OPT_R: C2RustUnnamed_7 = 512;
pub const OPT_q: C2RustUnnamed_7 = 256;
pub const OPT_p: C2RustUnnamed_7 = 128;
pub const OPT_n: C2RustUnnamed_7 = 64;
pub const OPT_i: C2RustUnnamed_7 = 32;
pub const OPT_F: C2RustUnnamed_7 = 16;
pub const OPT_h: C2RustUnnamed_7 = 8;
pub const OPT_H: C2RustUnnamed_7 = 4;
pub const OPT_V: C2RustUnnamed_7 = 2;
pub const OPT_C: C2RustUnnamed_7 = 1;

/*
 * udhcp client
 * Russ Dill <Russ.Dill@asu.edu> July 2001
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
 */
//applet:IF_UDHCPC(APPLET(udhcpc, BB_DIR_SBIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_UDHCPC) += common.o packet.o signalpipe.o socket.o
//kbuild:lib-$(CONFIG_UDHCPC) += dhcpc.o
//kbuild:lib-$(CONFIG_FEATURE_UDHCPC_ARPING) += arpping.o
//kbuild:lib-$(CONFIG_FEATURE_UDHCP_RFC3397) += domain_codec.o
/* Override ENABLE_FEATURE_PIDFILE - ifupdown needs our pidfile to always exist */
/* "struct client_data_t client_data" is in bb_common_bufsiz1 */
static mut udhcpc_longopts: [libc::c_char; 238] = [
  99, 108, 105, 101, 110, 116, 105, 100, 45, 110, 111, 110, 101, 0, 0, 67, 118, 101, 110, 100, 111,
  114, 99, 108, 97, 115, 115, 0, 1, 86, 104, 111, 115, 116, 110, 97, 109, 101, 0, 1, 72, 102, 113,
  100, 110, 0, 1, 70, 105, 110, 116, 101, 114, 102, 97, 99, 101, 0, 1, 105, 110, 111, 119, 0, 0,
  110, 112, 105, 100, 102, 105, 108, 101, 0, 1, 112, 113, 117, 105, 116, 0, 0, 113, 114, 101, 108,
  101, 97, 115, 101, 0, 0, 82, 114, 101, 113, 117, 101, 115, 116, 0, 1, 114, 115, 99, 114, 105,
  112, 116, 0, 1, 115, 116, 105, 109, 101, 111, 117, 116, 0, 1, 84, 114, 101, 116, 114, 105, 101,
  115, 0, 1, 116, 116, 114, 121, 97, 103, 97, 105, 110, 0, 1, 65, 115, 121, 115, 108, 111, 103, 0,
  0, 83, 114, 101, 113, 117, 101, 115, 116, 45, 111, 112, 116, 105, 111, 110, 0, 1, 79, 110, 111,
  45, 100, 101, 102, 97, 117, 108, 116, 45, 111, 112, 116, 105, 111, 110, 115, 0, 0, 111, 102, 111,
  114, 101, 103, 114, 111, 117, 110, 100, 0, 0, 102, 98, 97, 99, 107, 103, 114, 111, 117, 110, 100,
  0, 0, 98, 98, 114, 111, 97, 100, 99, 97, 115, 116, 0, 0, 66, 97, 114, 112, 105, 110, 103, 0, 2,
  97, 0,
];
/* ** Script execution code ***/
/* get a rough idea of how long an option will be (rounding up...) */
// Initialized in run_static_initializers
static mut len_of_option_as_string: [u8; 14] = [0; 14];
/* note: ip is a pointer to an IP in network order, possibly misaliged */
unsafe extern "C" fn sprint_nip(
  mut dest: *mut libc::c_char,
  mut pre: *const libc::c_char,
  mut ip: *const u8,
) -> libc::c_int {
  return sprintf(
    dest,
    b"%s%u.%u.%u.%u\x00" as *const u8 as *const libc::c_char,
    pre,
    *ip.offset(0) as libc::c_int,
    *ip.offset(1) as libc::c_int,
    *ip.offset(2) as libc::c_int,
    *ip.offset(3) as libc::c_int,
  );
}
/* really simple implementation, just count the bits */
unsafe extern "C" fn mton(mut mask: u32) -> libc::c_int {
  let mut i: libc::c_int = 0i32; /* 111110000-like bit pattern */
  mask = {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = mask;
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
                      (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
    }
    __v
  };
  while mask != 0 {
    i += 1;
    mask <<= 1i32
  }
  return i;
}
/* Check if a given label represents a valid DNS label
 * Return pointer to the first character after the label
 * (NUL or dot) upon success, NULL otherwise.
 * See RFC1035, 2.3.1
 */
/* We don't need to be particularly anal. For example, allowing _, hyphen
 * at the end, or leading and trailing dots would be ok, since it
 * can't be used for attacks. (Leading hyphen can be, if someone uses
 * cmd "$hostname"
 * in the script: then hostname may be treated as an option)
 */
unsafe extern "C" fn valid_domain_label(mut label: *const libc::c_char) -> *const libc::c_char {
  let mut ch: libc::c_uchar = 0;
  //unsigned pos = 0;
  if *label.offset(0) as libc::c_int == '-' as i32 {
    return 0 as *const libc::c_char;
  }
  loop {
    ch = *label as libc::c_uchar;
    if ch as libc::c_int | 0x20i32 < 'a' as i32 || ch as libc::c_int | 0x20i32 > 'z' as i32 {
      if (ch as libc::c_int) < '0' as i32 || ch as libc::c_int > '9' as i32 {
        if ch as libc::c_int == '\u{0}' as i32 || ch as libc::c_int == '.' as i32 {
          return label;
        }
        //pos++;
        //Do we want this?
        //if (pos > 63) /* NS_MAXLABEL; labels must be 63 chars or less */
        //	return NULL;
        /* DNS allows only '-', but we are more permissive */
        if ch as libc::c_int != '-' as i32 && ch as libc::c_int != '_' as i32 {
          return 0 as *const libc::c_char;
        }
      }
    }
    label = label.offset(1)
  }
}
/* Check if a given name represents a valid DNS name */
/* See RFC1035, 2.3.1 */
unsafe extern "C" fn good_hostname(mut name: *const libc::c_char) -> libc::c_int {
  loop
  //const char *start = name;
  {
    name = valid_domain_label(name);
    if name.is_null() {
      return 0i32;
    }
    if *name.offset(0) == 0 {
      return 1i32;
    }
    // We allow trailing dot too
    name = name.offset(1);
    if *name as libc::c_int == '\u{0}' as i32 {
      return 1i32;
    }
  }
}
//Do we want this?
//return ((name - start) < 1025); /* NS_MAXDNAME */
/* Create "opt_name=opt_value" string */
#[inline(never)]
unsafe extern "C" fn xmalloc_optname_optval(
  mut option: *mut u8,
  mut optflag: *const dhcp_optflag,
  mut opt_name: *const libc::c_char,
) -> *mut libc::c_char {
  let mut upper_length: libc::c_uint = 0;
  let mut len: libc::c_int = 0;
  let mut type_0: libc::c_int = 0;
  let mut optlen: libc::c_int = 0;
  let mut dest: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
  /* option points to OPT_DATA, need to go back to get OPT_LEN */
  len = *option.offset((-2i32 + 1i32) as isize) as libc::c_int; /* while */
  type_0 = (*optflag).flags as libc::c_int & OPTION_TYPE_MASK as libc::c_int; /* switch */
  optlen = *dhcp_option_lengths.as_ptr().offset(type_0 as isize) as libc::c_int;
  upper_length = (len_of_option_as_string[type_0 as usize] as libc::c_uint)
    .wrapping_mul(((len + optlen) as libc::c_uint).wrapping_div(optlen as libc::c_uint));
  ret = xmalloc(
    (upper_length as libc::c_ulong)
      .wrapping_add(strlen(opt_name))
      .wrapping_add(2i32 as libc::c_ulong),
  ) as *mut libc::c_char;
  dest = ret;
  dest = dest.offset(sprintf(
    ret,
    b"%s=\x00" as *const u8 as *const libc::c_char,
    opt_name,
  ) as isize);
  while len >= optlen {
    match type_0 {
      1 | 2 => {
        dest = dest
          .offset(sprint_nip(dest, b"\x00" as *const u8 as *const libc::c_char, option) as isize);
        if !(type_0 == OPTION_IP as libc::c_int) {
          dest = dest.offset(sprint_nip(
            dest,
            b"/\x00" as *const u8 as *const libc::c_char,
            option.offset(4),
          ) as isize)
        }
      }
      5 => {
        //		case OPTION_BOOLEAN:
        //			dest += sprintf(dest, *option ? "yes" : "no");
        //			break;
        dest = dest.offset(sprintf(
          dest,
          b"%u\x00" as *const u8 as *const libc::c_char,
          *option as libc::c_int,
        ) as isize)
      }
      6 => {
        //		case OPTION_S16:
        let mut val_u16: u16 = 0;
        val_u16 = *(option as *mut bb__aliased_u16);
        dest =
                    dest.offset(sprintf(dest,
                                        b"%u\x00" as *const u8 as
                                            *const libc::c_char,
                                        ({
                                             let mut __v: libc::c_ushort = 0;
                                             let mut __x: libc::c_ushort =
                                                 val_u16;
                                             if 0 != 0 {
                                                 __v =
                                                     (__x as libc::c_int >>
                                                          8i32 & 0xffi32 |
                                                          (__x as libc::c_int
                                                               & 0xffi32) <<
                                                              8i32) as
                                                         libc::c_ushort
                                             } else {
                                                 let fresh3 = &mut __v;
                                                 let fresh4;
                                                 let fresh5 = __x;
                                                 asm!("rorw $$8, ${0:w}" :
                                                      "=r" (fresh4) : "0"
                                                      (c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5))
                                                      : "cc");
                                                 c2rust_asm_casts::AsmCast::cast_out(fresh3,
                                                                                     fresh5,
                                                                                     fresh4);
                                             }
                                             __v
                                         }) as libc::c_int) as isize)
      }
      8 | 7 => {
        let mut val_u32: u32 = 0;
        val_u32 = *(option as *mut bb__aliased_u32);
        dest =
                    dest.offset(sprintf(dest,
                                        if type_0 == OPTION_U32 as libc::c_int
                                           {
                                            b"%lu\x00" as *const u8 as
                                                *const libc::c_char
                                        } else {
                                            b"%ld\x00" as *const u8 as
                                                *const libc::c_char
                                        },
                                        ({
                                             let mut __v: libc::c_uint = 0;
                                             let mut __x: libc::c_uint =
                                                 val_u32;
                                             if 0 != 0 {
                                                 __v =
                                                     (__x & 0xff000000u32) >>
                                                         24i32 |
                                                         (__x &
                                                              0xff0000i32 as
                                                                  libc::c_uint)
                                                             >> 8i32 |
                                                         (__x &
                                                              0xff00i32 as
                                                                  libc::c_uint)
                                                             << 8i32 |
                                                         (__x &
                                                              0xffi32 as
                                                                  libc::c_uint)
                                                             << 24i32
                                             } else {
                                                 let fresh6 = &mut __v;
                                                 let fresh7;
                                                 let fresh8 = __x;
                                                 asm!("bswap $0" : "=r"
                                                      (fresh7) : "0"
                                                      (c2rust_asm_casts::AsmCast::cast_in(fresh6, fresh8))
                                                      :);
                                                 c2rust_asm_casts::AsmCast::cast_out(fresh6,
                                                                                     fresh8,
                                                                                     fresh7);
                                             }
                                             __v
                                         }) as libc::c_ulong) as isize)
      }
      3 | 4 => {
        /* Note: options which use 'return' instead of 'break'
         * (for example, OPTION_STRING) skip the code which handles
         * the case of list of options.
         */
        memcpy(
          dest as *mut libc::c_void,
          option as *const libc::c_void,
          len as libc::c_ulong,
        );
        *dest.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
        //TODO: it appears option 15 DHCP_DOMAIN_NAME is often abused
        //by DHCP admins to contain a space-separated list of domains,
        //not one domain name (presumably, to work as list of search domains,
        //instead of using proper option 119 DHCP_DOMAIN_SEARCH).
        //Currently, good_hostname() balks on strings containing spaces.
        //Do we need to allow it? Only for DHCP_DOMAIN_NAME option?
        if type_0 == OPTION_STRING_HOST as libc::c_int && good_hostname(dest) == 0 {
          safe_strncpy(
            dest,
            b"bad\x00" as *const u8 as *const libc::c_char,
            len as size_t,
          );
        }
        return ret;
      }
      10 => {
        /* Option binary format:
         * mask [one byte, 0..32]
         * ip [big endian, 0..4 bytes depending on mask]
         * router [big endian, 4 bytes]
         * may be repeated
         *
         * We convert it to a string "IP/MASK ROUTER IP2/MASK2 ROUTER2"
         */
        let mut pfx: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
        while len >= 1i32 + 4i32 {
          /* mask + 0-byte ip + router */
          let mut nip: u32 = 0; /* 0 -> 0, 1..8 -> 1, 9..16 -> 2 etc */
          let mut p: *mut u8 = 0 as *mut u8;
          let mut mask: libc::c_uint = 0;
          let mut bytes: libc::c_int = 0;
          let fresh9 = option;
          option = option.offset(1);
          mask = *fresh9 as libc::c_uint;
          if mask > 32i32 as libc::c_uint {
            break;
          }
          len -= 1;
          nip = 0i32 as u32;
          p = &mut nip as *mut u32 as *mut libc::c_void as *mut u8;
          bytes = mask
            .wrapping_add(7i32 as libc::c_uint)
            .wrapping_div(8i32 as libc::c_uint) as libc::c_int;
          loop {
            bytes -= 1;
            if !(bytes >= 0i32) {
              break;
            }
            let fresh10 = option;
            option = option.offset(1);
            let fresh11 = p;
            p = p.offset(1);
            *fresh11 = *fresh10;
            len -= 1
          }
          if len < 4i32 {
            break;
          }
          /* print ip/mask */
          dest = dest.offset(sprint_nip(
            dest,
            pfx,
            &mut nip as *mut u32 as *mut libc::c_void as *const u8,
          ) as isize);
          pfx = b" \x00" as *const u8 as *const libc::c_char;
          dest = dest
            .offset(sprintf(dest, b"/%u \x00" as *const u8 as *const libc::c_char, mask) as isize);
          /* print router */
          dest = dest
            .offset(sprint_nip(dest, b"\x00" as *const u8 as *const libc::c_char, option) as isize);
          option = option.offset(4);
          len -= 4i32
        }
        return ret;
      }
      11 => {
        /* Option binary format (see RFC 5969):
         *  0                   1                   2                   3
         *  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
         * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
         * |  OPTION_6RD   | option-length |  IPv4MaskLen  |  6rdPrefixLen |
         * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
         * |                           6rdPrefix                           |
         * ...                        (16 octets)                        ...
         * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
         * ...                   6rdBRIPv4Address(es)                    ...
         * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
         * We convert it to a string
         * "IPv4MaskLen 6rdPrefixLen 6rdPrefix 6rdBRIPv4Address..."
         *
         * Sanity check: ensure that our length is at least 22 bytes, that
         * IPv4MaskLen <= 32,
         * 6rdPrefixLen <= 128,
         * 6rdPrefixLen + (32 - IPv4MaskLen) <= 128
         * (2nd condition need no check - it follows from 1st and 3rd).
         * Else, return envvar with empty value ("optname=")
         */
        if len >= 1i32 + 1i32 + 16i32 + 4i32
          && *option.offset(0) as libc::c_int <= 32i32
          && *option.offset(1) as libc::c_int + 32i32 - *option.offset(0) as libc::c_int <= 128i32
        {
          /* IPv4MaskLen */
          let fresh12 = option;
          option = option.offset(1);
          dest = dest.offset(sprintf(
            dest,
            b"%u \x00" as *const u8 as *const libc::c_char,
            *fresh12 as libc::c_int,
          ) as isize);
          /* 6rdPrefixLen */
          let fresh13 = option;
          option = option.offset(1);
          dest = dest.offset(sprintf(
            dest,
            b"%u \x00" as *const u8 as *const libc::c_char,
            *fresh13 as libc::c_int,
          ) as isize);
          /* 6rdPrefix */
          dest = dest.offset(sprint_nip6(dest, option) as isize);
          option = option.offset(16);
          len -= 1i32 + 1i32 + 16i32 + 4i32;
          loop
          /* "+ 4" above corresponds to the length of IPv4 addr
           * we consume in the loop below */
          /* 6rdBRIPv4Address(es) */
          {
            dest =
              dest.offset(
                sprint_nip(dest, b" \x00" as *const u8 as *const libc::c_char, option) as isize,
              ); /* do we have yet another 4+ bytes? */
            option = option.offset(4);
            len -= 4i32;
            if len < 0i32 {
              break;
            }
          }
        }
        return ret;
      }
      12 => {
        /* unpack option into dest; use ret for prefix (i.e., "optname=") */
        dest = dname_dec(option, len, ret);
        if !dest.is_null() {
          free(ret as *mut libc::c_void);
          return dest;
        }
        /* error. return "optname=" string */
        return ret;
      }
      13 => {
        /* Option binary format:
         * type: byte
         * type=0: domain names, dns-compressed
         * type=1: IP addrs
         */
        option = option.offset(1);
        len -= 1;
        if *option.offset(-1i32 as isize) as libc::c_int == 0i32 {
          dest = dname_dec(option, len, ret);
          if !dest.is_null() {
            free(ret as *mut libc::c_void);
            return dest;
          }
        } else if *option.offset(-1i32 as isize) as libc::c_int == 1i32 {
          let mut pfx_0: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
          loop {
            len -= 4i32;
            if len < 0i32 {
              break;
            }
            dest = dest.offset(sprint_nip(dest, pfx_0, option) as isize);
            pfx_0 = b" \x00" as *const u8 as *const libc::c_char;
            option = option.offset(4)
          }
        }
        return ret;
      }
      _ => {}
    }
    /* If we are here, try to format any remaining data
     * in the option as another, similarly-formatted option
     */
    option = option.offset(optlen as isize);
    len -= optlen;
    // TODO: it can be a list only if (optflag->flags & OPTION_LIST).
    // Should we bail out/warn if we see multi-ip option which is
    // not allowed to be such (for example, DHCP_BROADCAST)? -
    if len < optlen {
      break;
    }
    let fresh14 = dest;
    dest = dest.offset(1);
    *fresh14 = ' ' as i32 as libc::c_char;
    *dest = '\u{0}' as i32 as libc::c_char
  }
  return ret;
}
/* put all the parameters into the environment */
unsafe extern "C" fn fill_envp(mut packet: *mut dhcp_packet) -> *mut *mut libc::c_char {
  let mut envc: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  let mut envp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut curr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut opt_name: *const libc::c_char = 0 as *const libc::c_char;
  let mut temp: *mut u8 = 0 as *mut u8;
  let mut overload: u8 = 0i32 as u8;
  let mut found_opts: [libc::c_uint; 8] = [0; 8];
  memset(
    found_opts.as_mut_ptr() as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<[libc::c_uint; 8]>() as libc::c_ulong,
  );
  /* We need 7 elements for:
   * "interface=IFACE"
   * "ip=N.N.N.N" from packet->yiaddr
   * "giaddr=IP" from packet->gateway_nip (unless 0)
   * "siaddr=IP" from packet->siaddr_nip (unless 0)
   * "boot_file=FILE" from packet->file (unless overloaded)
   * "sname=SERVER_HOSTNAME" from packet->sname (unless overloaded)
   * terminating NULL
   */
  envc = 7i32;
  /* +1 element for each option, +2 for subnet option: */
  if !packet.is_null() {
    /* note: do not search for "pad" (0) and "end" (255) options */
    //TODO: change logic to scan packet _once_
    i = 1i32; /* for $mask */
    while i < 255i32 {
      temp = udhcp_get_option(packet, i);
      if !temp.is_null() {
        if i == 0x34i32 {
          overload = (overload as libc::c_int | *temp as libc::c_int) as u8
        } else if i == 0x1i32 {
          envc += 1
        }
        envc += 1;
        /*if (i != DHCP_MESSAGE_TYPE)*/
        found_opts[(i as libc::c_uint as libc::c_ulong).wrapping_div(
          (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8i32 as libc::c_ulong),
        ) as usize] |= (1i32
          << (i as libc::c_ulong
            & (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
              .wrapping_mul(8i32 as libc::c_ulong)
              .wrapping_sub(1i32 as libc::c_ulong))) as libc::c_uint
      }
      i += 1
    }
  }
  envp = xzalloc(
    (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
      .wrapping_mul(envc as libc::c_ulong),
  ) as *mut *mut libc::c_char;
  curr = envp;
  *curr = xasprintf(
    b"interface=%s\x00" as *const u8 as *const libc::c_char,
    (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .interface,
  );
  let fresh15 = curr;
  curr = curr.offset(1);
  putenv(*fresh15);
  if packet.is_null() {
    return envp;
  }
  /* Export BOOTP fields. Fields we don't (yet?) export:
   * u8 op;      // always BOOTREPLY
   * u8 htype;   // hardware address type. 1 = 10mb ethernet
   * u8 hlen;    // hardware address length
   * u8 hops;    // used by relay agents only
   * u32 xid;
   * u16 secs;   // elapsed since client began acquisition/renewal
   * u16 flags;  // only one flag so far: bcast. Never set by server
   * u32 ciaddr; // client IP (usually == yiaddr. can it be different
   *                  // if during renew server wants to give us different IP?)
   * u8 chaddr[16]; // link-layer client hardware address (MAC)
   */
  /* Most important one: yiaddr as $ip */
  *curr =
    xmalloc(::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong) as *mut libc::c_char;
  sprint_nip(
    *curr,
    b"ip=\x00" as *const u8 as *const libc::c_char,
    &mut (*packet).yiaddr as *mut u32 as *mut u8,
  );
  let fresh16 = curr;
  curr = curr.offset(1);
  putenv(*fresh16);
  if (*packet).siaddr_nip != 0 {
    /* IP address of next server to use in bootstrap */
    *curr =
      xmalloc(::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong) as *mut libc::c_char;
    sprint_nip(
      *curr,
      b"siaddr=\x00" as *const u8 as *const libc::c_char,
      &mut (*packet).siaddr_nip as *mut u32 as *mut u8,
    );
    let fresh17 = curr;
    curr = curr.offset(1);
    putenv(*fresh17);
  }
  if (*packet).gateway_nip != 0 {
    /* IP address of DHCP relay agent */
    *curr =
      xmalloc(::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong) as *mut libc::c_char;
    sprint_nip(
      *curr,
      b"giaddr=\x00" as *const u8 as *const libc::c_char,
      &mut (*packet).gateway_nip as *mut u32 as *mut u8,
    );
    let fresh18 = curr;
    curr = curr.offset(1);
    putenv(*fresh18);
  }
  if overload as libc::c_int & 1i32 == 0 && (*packet).file[0] as libc::c_int != 0 {
    /* watch out for invalid packets */
    *curr = xasprintf(
      b"boot_file=%.128s\x00" as *const u8 as *const libc::c_char,
      (*packet).file.as_mut_ptr(),
    );
    let fresh19 = curr;
    curr = curr.offset(1);
    putenv(*fresh19);
  }
  if overload as libc::c_int & 2i32 == 0 && (*packet).sname[0] as libc::c_int != 0 {
    /* watch out for invalid packets */
    *curr = xasprintf(
      b"sname=%.64s\x00" as *const u8 as *const libc::c_char,
      (*packet).sname.as_mut_ptr(),
    );
    let fresh20 = curr;
    curr = curr.offset(1);
    putenv(*fresh20);
  }
  /* Export known DHCP options */
  opt_name = dhcp_option_strings.as_ptr(); /* leave only unknown options */
  i = 0i32;
  while *opt_name != 0 {
    let mut code: u8 = (*dhcp_optflags.as_ptr().offset(i as isize)).code;
    let mut found_ptr: *mut libc::c_uint = &mut *found_opts.as_mut_ptr().offset(
      (code as libc::c_uint as libc::c_ulong).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
          .wrapping_mul(8i32 as libc::c_ulong),
      ) as isize,
    ) as *mut libc::c_uint;
    let mut found_mask: libc::c_uint = (1i32
      << (code as libc::c_ulong
        & (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
          .wrapping_mul(8i32 as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong)))
      as libc::c_uint;
    if !(*found_ptr & found_mask == 0) {
      *found_ptr &= !found_mask;
      temp = udhcp_get_option(packet, code as libc::c_int);
      *curr = xmalloc_optname_optval(temp, &*dhcp_optflags.as_ptr().offset(i as isize), opt_name);
      let fresh21 = curr;
      curr = curr.offset(1);
      putenv(*fresh21);
      if code as libc::c_int == 0x1i32
        && *temp.offset((-2i32 + 1i32) as isize) as libc::c_int == 4i32
      {
        /* Subnet option: make things like "$ip/$mask" possible */
        let mut subnet: u32 = 0;
        subnet = *(temp as *mut bb__aliased_u32);
        *curr = xasprintf(
          b"mask=%u\x00" as *const u8 as *const libc::c_char,
          mton(subnet),
        );
        let fresh22 = curr;
        curr = curr.offset(1);
        putenv(*fresh22);
      }
    }
    opt_name = opt_name.offset(strlen(opt_name).wrapping_add(1i32 as libc::c_ulong) as isize);
    i += 1
  }
  /* Export unknown options */
  i = 0i32;
  while i < 256i32 {
    let mut bitmap: libc::c_uint = found_opts[(i as libc::c_uint as libc::c_ulong).wrapping_div(
      (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong),
    ) as usize];
    if bitmap == 0 {
      i = (i as libc::c_ulong).wrapping_add(
        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
          .wrapping_mul(8i32 as libc::c_ulong),
      ) as libc::c_int as libc::c_int
    } else {
      if bitmap
        & (1i32
          << (i as libc::c_ulong
            & (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
              .wrapping_mul(8i32 as libc::c_ulong)
              .wrapping_sub(1i32 as libc::c_ulong))) as libc::c_uint
        != 0
      {
        let mut len: libc::c_uint = 0;
        let mut ofs: libc::c_uint = 0;
        temp = udhcp_get_option(packet, i);
        /* udhcp_get_option returns ptr to data portion,
         * need to go back to get len
         */
        len = *temp.offset((-2i32 + 1i32) as isize) as libc::c_uint;
        *curr = xmalloc(
          (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_add(1i32 as libc::c_ulong)
            .wrapping_add(len.wrapping_mul(2i32 as libc::c_uint) as libc::c_ulong),
        ) as *mut libc::c_char;
        ofs = sprintf(*curr, b"opt%u=\x00" as *const u8 as *const libc::c_char, i) as libc::c_uint;
        *bin2hex(
          (*curr).offset(ofs as isize),
          temp as *mut libc::c_void as *const libc::c_char,
          len as libc::c_int,
        ) = '\u{0}' as i32 as libc::c_char;
        let fresh23 = curr;
        curr = curr.offset(1);
        putenv(*fresh23);
      }
      i += 1
    }
  }
  return envp;
}
/* Call a script with a par file and env vars */
unsafe extern "C" fn udhcp_run_script(mut packet: *mut dhcp_packet, mut name: *const libc::c_char) {
  let mut envp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut curr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut argv: [*mut libc::c_char; 3] = [0 as *mut libc::c_char; 3];
  envp = fill_envp(packet);
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
  argv[2] = 0 as *mut libc::c_char;
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
/* ** Sending/receiving packets ***/
#[inline(always)]
unsafe extern "C" fn random_xid() -> u32 {
  return rand() as u32;
}
/* Initialize the packet with the proper defaults */
unsafe extern "C" fn init_packet(mut packet: *mut dhcp_packet, mut type_0: libc::c_char) {
  let mut secs: libc::c_uint = 0;
  /* Fill in: op, htype, hlen, cookie fields; message type option: */
  udhcp_init_header(packet, type_0);
  (*packet).xid = random_xid();
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
    == 0i32 as libc::c_uint
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
  (*packet).secs = if secs < 0xffffi32 as libc::c_uint {
    ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = secs as libc::c_ushort;
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
  } else {
    0xffffi32
  } as u16;
  memcpy(
    (*packet).chaddr.as_mut_ptr() as *mut libc::c_void,
    (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .client_mac
      .as_mut_ptr() as *const libc::c_void,
    6i32 as libc::c_ulong,
  );
  if !(*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
    as *mut client_data_t))
    .clientid
    .is_null()
  {
    udhcp_add_binary_option(
      packet,
      (*(&mut *bb_common_bufsiz1
        .as_mut_ptr()
        .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
        as *mut client_data_t))
        .clientid,
    );
  };
}
unsafe extern "C" fn add_client_options(mut packet: *mut dhcp_packet) {
  let mut i: libc::c_int = 0;
  let mut end: libc::c_int = 0;
  let mut len: libc::c_int = 0;
  udhcp_add_simple_option(
    packet,
    0x39i32 as u8,
    ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = IP_UDP_DHCP_SIZE as libc::c_int as libc::c_ushort;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh27 = &mut __v;
        let fresh28;
        let fresh29 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh28)
                                          : "0"
                                          (c2rust_asm_casts::AsmCast::cast_in(fresh27, fresh29))
                                          : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh27, fresh29, fresh28);
      }
      __v
    }) as u32,
  );
  /* Add a "param req" option with the list of options we'd like to have
   * from stubborn DHCP servers. Pull the data from the struct in common.c.
   * No bounds checking because it goes towards the head of the packet. */
  end = udhcp_end_option((*packet).options.as_mut_ptr());
  len = 0i32;
  i = 1i32;
  while i < 0xffi32 {
    if (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .opt_mask[(i >> 3i32) as usize] as libc::c_int
      & 1i32 << (i & 7i32)
      != 0
    {
      (*packet).options[(end + 2i32 + len) as usize] = i as u8;
      len += 1
    }
    i += 1
  }
  if len != 0 {
    (*packet).options[(end + 0i32) as usize] = 0x37i32 as u8;
    (*packet).options[(end + 1i32) as usize] = len as u8;
    (*packet).options[(end + 2i32 + len) as usize] = 0xffi32 as u8
  }
  if !(*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
    as *mut client_data_t))
    .vendorclass
    .is_null()
  {
    udhcp_add_binary_option(
      packet,
      (*(&mut *bb_common_bufsiz1
        .as_mut_ptr()
        .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
        as *mut client_data_t))
        .vendorclass,
    );
  }
  if !(*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
    as *mut client_data_t))
    .hostname
    .is_null()
  {
    udhcp_add_binary_option(
      packet,
      (*(&mut *bb_common_bufsiz1
        .as_mut_ptr()
        .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
        as *mut client_data_t))
        .hostname,
    );
  }
  if !(*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
    as *mut client_data_t))
    .fqdn
    .is_null()
  {
    udhcp_add_binary_option(
      packet,
      (*(&mut *bb_common_bufsiz1
        .as_mut_ptr()
        .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
        as *mut client_data_t))
        .fqdn,
    );
  }
  /* Request broadcast replies if we have no IP addr */
  if option_mask32 & OPT_B as libc::c_int as libc::c_uint != 0
    && (*packet).ciaddr == 0i32 as libc::c_uint
  {
    (*packet).flags = ((*packet).flags as libc::c_int
      | ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = 0x8000i32 as libc::c_ushort;
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
      }) as libc::c_int) as u16
  }
  /* Add -x options if any */
  let mut curr: *mut option_set = (*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
    as *mut libc::c_char as *mut client_data_t))
    .options;
  while !curr.is_null() {
    udhcp_add_binary_option(packet, (*curr).data);
    curr = (*curr).next
  }
  //		if (client_data.sname)
  //			strncpy((char*)packet->sname, client_data.sname, sizeof(packet->sname) - 1);
  //		if (client_data.boot_file)
  //			strncpy((char*)packet->file, client_data.boot_file, sizeof(packet->file) - 1);
  // This will be needed if we remove -V VENDOR_STR in favor of
  // -x vendor:VENDOR_STR
  //if (!udhcp_find_option(packet.options, DHCP_VENDOR))
  //	/* not set, set the default vendor ID */
  //	...add (DHCP_VENDOR, "udhcp "BB_VER) opt...
}
/* RFC 2131
 * 4.4.4 Use of broadcast and unicast
 *
 * The DHCP client broadcasts DHCPDISCOVER, DHCPREQUEST and DHCPINFORM
 * messages, unless the client knows the address of a DHCP server.
 * The client unicasts DHCPRELEASE messages to the server. Because
 * the client is declining the use of the IP address supplied by the server,
 * the client broadcasts DHCPDECLINE messages.
 *
 * When the DHCP client knows the address of a DHCP server, in either
 * INIT or REBOOTING state, the client may use that address
 * in the DHCPDISCOVER or DHCPREQUEST rather than the IP broadcast address.
 * The client may also use unicast to send DHCPINFORM messages
 * to a known DHCP server. If the client receives no response to DHCP
 * messages sent to the IP address of a known DHCP server, the DHCP
 * client reverts to using the IP broadcast address.
 */
unsafe extern "C" fn raw_bcast_from_client_data_ifindex(
  mut packet: *mut dhcp_packet,
  mut src_nip: u32,
) -> libc::c_int {
  return udhcp_send_raw_packet(
    packet,
    src_nip,
    68i32,
    0xffffffffu32,
    67i32,
    MAC_BCAST_ADDR.as_ptr(),
    (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .ifindex,
  );
}
unsafe extern "C" fn bcast_or_ucast(
  mut packet: *mut dhcp_packet,
  mut ciaddr: u32,
  mut server: u32,
) -> libc::c_int {
  if server != 0 {
    return udhcp_send_kernel_packet(packet, ciaddr, 68i32, server, 67i32);
  }
  return raw_bcast_from_client_data_ifindex(packet, ciaddr);
}
/* Broadcast a DHCP discover packet to the network, with an optionally requested IP */
/* NOINLINE: limit stack usage in caller */
#[inline(never)]
unsafe extern "C" fn send_discover(mut xid: u32, mut requested: u32) -> libc::c_int {
  let mut packet: dhcp_packet = dhcp_packet {
    op: 0,
    htype: 0,
    hlen: 0,
    hops: 0,
    xid: 0,
    secs: 0,
    flags: 0,
    ciaddr: 0,
    yiaddr: 0,
    siaddr_nip: 0,
    gateway_nip: 0,
    chaddr: [0; 16],
    sname: [0; 64],
    file: [0; 128],
    cookie: 0,
    options: [0; 388],
  };
  /* Fill in: op, htype, hlen, cookie, chaddr fields,
   * random xid field (we override it below),
   * client-id option (unless -C), message type option:
   */
  init_packet(&mut packet, 1i32 as libc::c_char);
  packet.xid = xid;
  if requested != 0 {
    udhcp_add_simple_option(&mut packet, 0x32i32 as u8, requested);
  }
  /* Add options: maxsize,
   * optionally: hostname, fqdn, vendorclass,
   * "param req" option according to -O, options specified with -x
   */
  add_client_options(&mut packet);
  bb_info_msg(
    b"sending %s\x00" as *const u8 as *const libc::c_char,
    b"discover\x00" as *const u8 as *const libc::c_char,
  );
  return raw_bcast_from_client_data_ifindex(&mut packet, 0i32 as in_addr_t);
}
/* Broadcast a DHCP request message */
/* RFC 2131 3.1 paragraph 3:
 * "The client _broadcasts_ a DHCPREQUEST message..."
 */
/* NOINLINE: limit stack usage in caller */
#[inline(never)]
unsafe extern "C" fn send_select(
  mut xid: u32,
  mut server: u32,
  mut requested: u32,
) -> libc::c_int {
  let mut packet: dhcp_packet = dhcp_packet {
    op: 0,
    htype: 0,
    hlen: 0,
    hops: 0,
    xid: 0,
    secs: 0,
    flags: 0,
    ciaddr: 0,
    yiaddr: 0,
    siaddr_nip: 0,
    gateway_nip: 0,
    chaddr: [0; 16],
    sname: [0; 64],
    file: [0; 128],
    cookie: 0,
    options: [0; 388],
  };
  let mut temp_addr: in_addr = in_addr { s_addr: 0 };
  /*
   * RFC 2131 4.3.2 DHCPREQUEST message
   * ...
   * If the DHCPREQUEST message contains a 'server identifier'
   * option, the message is in response to a DHCPOFFER message.
   * Otherwise, the message is a request to verify or extend an
   * existing lease. If the client uses a 'client identifier'
   * in a DHCPREQUEST message, it MUST use that same 'client identifier'
   * in all subsequent messages. If the client included a list
   * of requested parameters in a DHCPDISCOVER message, it MUST
   * include that list in all subsequent messages.
   */
  /* Fill in: op, htype, hlen, cookie, chaddr fields,
   * random xid field (we override it below),
   * client-id option (unless -C), message type option:
   */
  init_packet(&mut packet, 3i32 as libc::c_char);
  packet.xid = xid;
  udhcp_add_simple_option(&mut packet, 0x32i32 as u8, requested);
  udhcp_add_simple_option(&mut packet, 0x36i32 as u8, server);
  /* Add options: maxsize,
   * optionally: hostname, fqdn, vendorclass,
   * "param req" option according to -O, and options specified with -x
   */
  add_client_options(&mut packet);
  temp_addr.s_addr = requested;
  bb_info_msg(
    b"sending select for %s\x00" as *const u8 as *const libc::c_char,
    inet_ntoa(temp_addr),
  );
  return raw_bcast_from_client_data_ifindex(&mut packet, 0i32 as in_addr_t);
}
/* Unicast or broadcast a DHCP renew message */
/* NOINLINE: limit stack usage in caller */
#[inline(never)]
unsafe extern "C" fn send_renew(
  mut xid: u32,
  mut server: u32,
  mut ciaddr: u32,
) -> libc::c_int {
  let mut packet: dhcp_packet = dhcp_packet {
    op: 0,
    htype: 0,
    hlen: 0,
    hops: 0,
    xid: 0,
    secs: 0,
    flags: 0,
    ciaddr: 0,
    yiaddr: 0,
    siaddr_nip: 0,
    gateway_nip: 0,
    chaddr: [0; 16],
    sname: [0; 64],
    file: [0; 128],
    cookie: 0,
    options: [0; 388],
  };
  let mut temp_addr: in_addr = in_addr { s_addr: 0 };
  /*
   * RFC 2131 4.3.2 DHCPREQUEST message
   * ...
   * DHCPREQUEST generated during RENEWING state:
   *
   * 'server identifier' MUST NOT be filled in, 'requested IP address'
   * option MUST NOT be filled in, 'ciaddr' MUST be filled in with
   * client's IP address. In this situation, the client is completely
   * configured, and is trying to extend its lease. This message will
   * be unicast, so no relay agents will be involved in its
   * transmission.  Because 'giaddr' is therefore not filled in, the
   * DHCP server will trust the value in 'ciaddr', and use it when
   * replying to the client.
   */
  /* Fill in: op, htype, hlen, cookie, chaddr fields,
   * random xid field (we override it below),
   * client-id option (unless -C), message type option:
   */
  init_packet(&mut packet, 3i32 as libc::c_char);
  packet.xid = xid;
  packet.ciaddr = ciaddr;
  /* Add options: maxsize,
   * optionally: hostname, fqdn, vendorclass,
   * "param req" option according to -O, and options specified with -x
   */
  add_client_options(&mut packet);
  temp_addr.s_addr = server;
  bb_info_msg(
    b"sending renew to %s\x00" as *const u8 as *const libc::c_char,
    inet_ntoa(temp_addr),
  );
  return bcast_or_ucast(&mut packet, ciaddr, server);
}
/* Broadcast a DHCP decline message */
/* NOINLINE: limit stack usage in caller */
#[inline(never)]
unsafe extern "C" fn send_decline(mut server: u32, mut requested: u32) -> libc::c_int {
  let mut packet: dhcp_packet = dhcp_packet {
    op: 0,
    htype: 0,
    hlen: 0,
    hops: 0,
    xid: 0,
    secs: 0,
    flags: 0,
    ciaddr: 0,
    yiaddr: 0,
    siaddr_nip: 0,
    gateway_nip: 0,
    chaddr: [0; 16],
    sname: [0; 64],
    file: [0; 128],
    cookie: 0,
    options: [0; 388],
  };
  /* Fill in: op, htype, hlen, cookie, chaddr, random xid fields,
   * client-id option (unless -C), message type option:
   */
  init_packet(&mut packet, 4i32 as libc::c_char);
  /* DHCPDECLINE uses "requested ip", not ciaddr, to store offered IP */
  udhcp_add_simple_option(&mut packet, 0x32i32 as u8, requested);
  udhcp_add_simple_option(&mut packet, 0x36i32 as u8, server);
  bb_info_msg(
    b"sending %s\x00" as *const u8 as *const libc::c_char,
    b"decline\x00" as *const u8 as *const libc::c_char,
  );
  return raw_bcast_from_client_data_ifindex(&mut packet, 0i32 as in_addr_t);
}
/* Unicast a DHCP release message */
#[inline(always)]
unsafe extern "C" fn send_release(mut server: u32, mut ciaddr: u32) -> libc::c_int {
  let mut packet: dhcp_packet = dhcp_packet {
    op: 0,
    htype: 0,
    hlen: 0,
    hops: 0,
    xid: 0,
    secs: 0,
    flags: 0,
    ciaddr: 0,
    yiaddr: 0,
    siaddr_nip: 0,
    gateway_nip: 0,
    chaddr: [0; 16],
    sname: [0; 64],
    file: [0; 128],
    cookie: 0,
    options: [0; 388],
  };
  /* Fill in: op, htype, hlen, cookie, chaddr, random xid fields,
   * client-id option (unless -C), message type option:
   */
  init_packet(&mut packet, 7i32 as libc::c_char);
  /* DHCPRELEASE uses ciaddr, not "requested ip", to store IP being released */
  packet.ciaddr = ciaddr;
  udhcp_add_simple_option(&mut packet, 0x36i32 as u8, server);
  bb_info_msg(
    b"sending %s\x00" as *const u8 as *const libc::c_char,
    b"release\x00" as *const u8 as *const libc::c_char,
  );
  /* Note: normally we unicast here since "server" is not zero.
   * However, there _are_ people who run "address-less" DHCP servers,
   * and reportedly ISC dhcp client and Windows allow that.
   */
  return bcast_or_ucast(&mut packet, ciaddr, server);
}
/* Returns -1 on errors that are fatal for the socket, -2 for those that aren't */
/* NOINLINE: limit stack usage in caller */
#[inline(never)]
unsafe extern "C" fn udhcp_recv_raw_packet(
  mut dhcp_pkt: *mut dhcp_packet,
  mut fd: libc::c_int,
) -> libc::c_int {
  let mut current_block: u64;
  let mut bytes: libc::c_int = 0;
  let mut packet: ip_udp_dhcp_packet = ip_udp_dhcp_packet {
    ip: iphdr {
      ihl_version: [0; 1],
      tos: 0,
      tot_len: 0,
      id: 0,
      frag_off: 0,
      ttl: 0,
      protocol: 0,
      check: 0,
      saddr: 0,
      daddr: 0,
    },
    udp: udphdr {
      c2rust_unnamed: C2RustUnnamed_2 {
        c2rust_unnamed: C2RustUnnamed_4 {
          uh_sport: 0,
          uh_dport: 0,
          uh_ulen: 0,
          uh_sum: 0,
        },
      },
    },
    data: dhcp_packet {
      op: 0,
      htype: 0,
      hlen: 0,
      hops: 0,
      xid: 0,
      secs: 0,
      flags: 0,
      ciaddr: 0,
      yiaddr: 0,
      siaddr_nip: 0,
      gateway_nip: 0,
      chaddr: [0; 16],
      sname: [0; 64],
      file: [0; 128],
      cookie: 0,
      options: [0; 388],
    },
  };
  let mut check: u16 = 0;
  let mut cmsgbuf: [libc::c_uchar; 36] = [0; 36];
  let mut iov: iovec = iovec {
    iov_base: 0 as *mut libc::c_void,
    iov_len: 0,
  };
  let mut msg: msghdr = msghdr {
    msg_name: 0 as *mut libc::c_void,
    msg_namelen: 0,
    msg_iov: 0 as *mut iovec,
    msg_iovlen: 0,
    msg_control: 0 as *mut libc::c_void,
    msg_controllen: 0,
    msg_flags: 0,
  };
  let mut cmsg: *mut cmsghdr = 0 as *mut cmsghdr;
  /* used to use just safe_read(fd, &packet, sizeof(packet))
   * but we need to check for TP_STATUS_CSUMNOTREADY :(
   */
  iov.iov_base = &mut packet as *mut ip_udp_dhcp_packet as *mut libc::c_void;
  iov.iov_len = ::std::mem::size_of::<ip_udp_dhcp_packet>() as libc::c_ulong;
  memset(
    &mut msg as *mut msghdr as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<msghdr>() as libc::c_ulong,
  );
  msg.msg_iov = &mut iov;
  msg.msg_iovlen = 1i32 as size_t;
  msg.msg_control = cmsgbuf.as_mut_ptr() as *mut libc::c_void;
  msg.msg_controllen = ::std::mem::size_of::<[libc::c_uchar; 36]>() as libc::c_ulong;
  loop {
    bytes = recvmsg(fd, &mut msg, 0i32) as libc::c_int;
    if !(bytes < 0i32) {
      break;
    }
    if *bb_errno == 4i32 {
      continue;
      /* returns -1 */
    }
    if dhcp_verbose >= 1i32 as libc::c_uint {
      bb_simple_info_msg(b"packet read error, ignoring\x00" as *const u8 as *const libc::c_char);
    }
    return bytes;
  }
  if bytes
    < (::std::mem::size_of::<iphdr>() as libc::c_ulong)
      .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong) as libc::c_int
  {
    if dhcp_verbose >= 1i32 as libc::c_uint {
      bb_simple_info_msg(b"packet is too short, ignoring\x00" as *const u8 as *const libc::c_char);
    }
    return -2i32;
  }
  if bytes
    < ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = packet.ip.tot_len;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh33 = &mut __v;
        let fresh34;
        let fresh35 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh34) : "0"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh33, fresh35))
                         : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh33, fresh35, fresh34);
      }
      __v
    }) as libc::c_int
  {
    /* NB: possible down interface, etc. Caller should pause. */
    /* packet is bigger than sizeof(packet), we did partial read */
    if dhcp_verbose >= 1i32 as libc::c_uint {
      bb_simple_info_msg(b"oversized packet, ignoring\x00" as *const u8 as *const libc::c_char);
    }
    return -2i32;
  }
  /* ignore any extra garbage bytes */
  bytes = ({
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = packet.ip.tot_len;
    if 0 != 0 {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh36 = &mut __v;
      let fresh37;
      let fresh38 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh37) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh36, fresh38)) :
                      "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh36, fresh38, fresh37);
    }
    __v
  }) as libc::c_int;
  /* make sure its the right packet for us, and that it passes sanity checks */
  if packet.ip.protocol as libc::c_int != IPPROTO_UDP as libc::c_int
    || packet.ip.version() as libc::c_int != 4i32
    || packet.ip.ihl() as libc::c_ulong != ::std::mem::size_of::<iphdr>() as libc::c_ulong >> 2i32
    || packet.udp.c2rust_unnamed.c2rust_unnamed_0.dest as libc::c_int
      != ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = 68i32 as libc::c_ushort;
        if 0 != 0 {
          __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
            as libc::c_ushort
        } else {
          let fresh39 = &mut __v;
          let fresh40;
          let fresh41 = __x;
          asm!("rorw $$8, ${0:w}" : "=r" (fresh40) : "0"
                             (c2rust_asm_casts::AsmCast::cast_in(fresh39, fresh41))
                             : "cc");
          c2rust_asm_casts::AsmCast::cast_out(fresh39, fresh41, fresh40);
        }
        __v
      }) as libc::c_int
    || ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = packet.udp.c2rust_unnamed.c2rust_unnamed_0.len;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh42 = &mut __v;
        let fresh43;
        let fresh44 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh43) : "0"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh42, fresh44))
                         : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh42, fresh44, fresh43);
      }
      __v
    }) as libc::c_int
      != (bytes as libc::c_ulong).wrapping_sub(::std::mem::size_of::<iphdr>() as libc::c_ulong)
        as u16 as libc::c_int
  {
    if dhcp_verbose >= 1i32 as libc::c_uint {
      bb_simple_info_msg(
        b"unrelated/bogus packet, ignoring\x00" as *const u8 as *const libc::c_char,
      );
    }
    return -2i32;
  }
  /* verify IP checksum */
  check = packet.ip.check;
  packet.ip.check = 0i32 as u16;
  if check as libc::c_int
    != inet_cksum(
      &mut packet.ip as *mut iphdr as *mut u16,
      ::std::mem::size_of::<iphdr>() as libc::c_ulong as libc::c_int,
    ) as libc::c_int
  {
    if dhcp_verbose >= 1i32 as libc::c_uint {
      bb_simple_info_msg(
        b"bad IP header checksum, ignoring\x00" as *const u8 as *const libc::c_char,
      );
    }
    return -2i32;
  }
  cmsg = if msg.msg_controllen >= ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
    msg.msg_control as *mut cmsghdr
  } else {
    0 as *mut cmsghdr
  };
  loop {
    if cmsg.is_null() {
      current_block = 13763002826403452995;
      break;
    }
    if (*cmsg).cmsg_level == 263i32 && (*cmsg).cmsg_type == 8i32 {
      /* some VMs don't checksum UDP and TCP data
       * they send to the same physical machine,
       * here we detect this case:
       */
      let mut aux: *mut tpacket_auxdata =
        (*cmsg).__cmsg_data.as_mut_ptr() as *mut libc::c_void as *mut tpacket_auxdata;
      if (*aux).tp_status & (1i32 << 3i32) as libc::c_uint != 0 {
        current_block = 15152947382816299763;
        break;
      }
    }
    cmsg = __cmsg_nxthdr(&mut msg, cmsg)
  }
  match current_block {
    13763002826403452995 => {
      /* verify UDP checksum. IP header has to be modified for this */
      memset(
        &mut packet.ip as *mut iphdr as *mut libc::c_void,
        0i32,
        9u64,
      );
      /* ip.xx fields which are not memset: protocol, check, saddr, daddr */
      packet.ip.tot_len = packet.udp.c2rust_unnamed.c2rust_unnamed_0.len; /* yes, this is needed */
      check = packet.udp.c2rust_unnamed.c2rust_unnamed_0.check;
      packet.udp.c2rust_unnamed.c2rust_unnamed_0.check = 0i32 as u16;
      if check as libc::c_int != 0
        && check as libc::c_int
          != inet_cksum(
            &mut packet as *mut ip_udp_dhcp_packet as *mut u16,
            bytes,
          ) as libc::c_int
      {
        if dhcp_verbose >= 1i32 as libc::c_uint {
          bb_simple_info_msg(
            b"packet with bad UDP checksum received, ignoring\x00" as *const u8
              as *const libc::c_char,
          );
        }
        return -2i32;
      }
    }
    _ => {}
  }
  if packet.data.cookie
    != ({
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = 0x63825363i32 as libc::c_uint;
      if 0 != 0 {
        __v = (__x & 0xff000000u32) >> 24i32
          | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
          | (__x & 0xff00i32 as libc::c_uint) << 8i32
          | (__x & 0xffi32 as libc::c_uint) << 24i32
      } else {
        let fresh45 = &mut __v;
        let fresh46;
        let fresh47 = __x;
        asm!("bswap $0" : "=r" (fresh46) : "0"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh45, fresh47))
                         :);
        c2rust_asm_casts::AsmCast::cast_out(fresh45, fresh47, fresh46);
      }
      __v
    })
  {
    bb_simple_info_msg(b"packet with bad magic, ignoring\x00" as *const u8 as *const libc::c_char);
    return -2i32;
  }
  if dhcp_verbose >= 1i32 as libc::c_uint {
    bb_info_msg(
      b"received %s\x00" as *const u8 as *const libc::c_char,
      b"a packet\x00" as *const u8 as *const libc::c_char,
    );
  }
  udhcp_dump_packet(&mut packet.data);
  bytes = (bytes as libc::c_ulong).wrapping_sub(
    (::std::mem::size_of::<iphdr>() as libc::c_ulong)
      .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong),
  ) as libc::c_int as libc::c_int;
  memcpy(
    dhcp_pkt as *mut libc::c_void,
    &mut packet.data as *mut dhcp_packet as *const libc::c_void,
    bytes as libc::c_ulong,
  );
  return bytes;
}
unsafe extern "C" fn udhcp_raw_socket(mut ifindex: libc::c_int) -> libc::c_int {
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
  if dhcp_verbose >= 2i32 as libc::c_uint {
    bb_info_msg(
      b"opening raw socket on ifindex %d\x00" as *const u8 as *const libc::c_char,
      ifindex,
    );
  }
  fd = xsocket(
    17i32,
    SOCK_DGRAM as libc::c_int,
    ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = 0x800i32 as libc::c_ushort;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh48 = &mut __v;
        let fresh49;
        let fresh50 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh49) : "0"
                              (c2rust_asm_casts::AsmCast::cast_in(fresh48, fresh50))
                              : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh48, fresh50, fresh49);
      }
      __v
    }) as libc::c_int,
  );
  /* ^^^^^
   * SOCK_DGRAM: remove link-layer headers on input (SOCK_RAW keeps them)
   * ETH_P_IP: want to receive only packets with IPv4 eth type
   */
  memset(
    &mut sock as *mut sockaddr_ll as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<sockaddr_ll>() as libc::c_ulong,
  ); /* let's be deterministic */
  sock.sll_family = 17i32 as libc::c_ushort;
  sock.sll_protocol = {
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = 0x800i32 as libc::c_ushort;
    if 0 != 0 {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh51 = &mut __v;
      let fresh52;
      let fresh53 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh52) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh51, fresh53)) :
                      "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh51, fresh53, fresh52);
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
  /* Several users reported breakage when BPF filter is used */
  if setsockopt_1(fd, 263i32, 8i32) != 0i32 {
    if *bb_errno != 92i32 {
      if dhcp_verbose >= 1i32 as libc::c_uint {
        bb_simple_info_msg(
          b"can\'t set PACKET_AUXDATA on raw socket\x00" as *const u8 as *const libc::c_char,
        );
      }
    }
  }
  if dhcp_verbose >= 1i32 as libc::c_uint {
    bb_simple_info_msg(b"created raw socket\x00" as *const u8 as *const libc::c_char);
  }
  return fd;
}
unsafe extern "C" fn change_listen_mode(mut new_mode: libc::c_int) {
  if dhcp_verbose >= 1i32 as libc::c_uint {
    bb_info_msg(
      b"entering listen mode: %s\x00" as *const u8 as *const libc::c_char,
      if new_mode != 0i32 {
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
    >= 0i32
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
      68i32,
      (*(&mut *bb_common_bufsiz1
        .as_mut_ptr()
        .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
        as *mut client_data_t))
        .interface,
    )
  } else if new_mode != 0i32 {
    (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .sockfd = udhcp_raw_socket(
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
      current_block_5 = 16734918824224422158;
    }
    3 | 4 => {
      current_block_5 = 16734918824224422158;
    }
    5 => {
      /* impatient are we? fine, square 1 */
      udhcp_run_script(
        0 as *mut dhcp_packet,
        b"deconfig\x00" as *const u8 as *const libc::c_char,
      );
      current_block_5 = 4193636798346030980;
    }
    1 | 6 => {
      current_block_5 = 4193636798346030980;
    }
    0 | _ => {
      current_block_5 = 1394248824506584008;
    }
  }
  match current_block_5 {
    4193636798346030980 => {
      change_listen_mode(2i32);
      (*(&mut *bb_common_bufsiz1
        .as_mut_ptr()
        .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
        as *mut client_data_t))
        .state = 0i32 as smallint
    }
    16734918824224422158 => {
      (*(&mut *bb_common_bufsiz1
        .as_mut_ptr()
        .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
        as *mut client_data_t))
        .state = 5i32 as smallint
    }
    _ => {}
  };
}
unsafe extern "C" fn perform_release(mut server_addr: u32, mut requested_ip: u32) {
  let mut buffer: [libc::c_char; 16] = [0; 16];
  let mut temp_addr: in_addr = in_addr { s_addr: 0 };
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
    temp_addr.s_addr = server_addr;
    strcpy(buffer.as_mut_ptr(), inet_ntoa(temp_addr));
    temp_addr.s_addr = requested_ip;
    bb_info_msg(
      b"unicasting a release of %s to %s\x00" as *const u8 as *const libc::c_char,
      inet_ntoa(temp_addr),
      buffer.as_mut_ptr(),
    );
    send_release(server_addr, requested_ip);
    /* unicast */
  }
  bb_simple_info_msg(b"entering released state\x00" as *const u8 as *const libc::c_char);
  /*
   * We can be here on: SIGUSR2,
   * or on exit (SIGTERM) and -R "release on quit" is specified.
   * Users requested to be notified in all cases, even if not in one
   * of the states above.
   */
  udhcp_run_script(
    0 as *mut dhcp_packet,
    b"deconfig\x00" as *const u8 as *const libc::c_char,
  );
  change_listen_mode(0i32);
  (*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
    as *mut client_data_t))
    .state = 6i32 as smallint;
}
unsafe extern "C" fn alloc_dhcp_option(
  mut code: libc::c_int,
  mut str: *const libc::c_char,
  mut extra: libc::c_int,
) -> *mut u8 {
  let mut storage: *mut u8 = 0 as *mut u8;
  let mut len: libc::c_int = strnlen(str, 255i32 as size_t) as libc::c_int;
  storage = xzalloc((len + extra + 2i32) as size_t) as *mut u8;
  *storage.offset(0) = code as u8;
  *storage.offset(1) = (len + extra) as u8;
  memcpy(
    storage.offset(extra as isize).offset(2) as *mut libc::c_void,
    str as *const libc::c_void,
    len as libc::c_ulong,
  );
  return storage;
}
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
//usage:#define udhcpc_trivial_usage
//usage:       "[-fbq"IF_UDHCP_VERBOSE("v")"RB]"IF_FEATURE_UDHCPC_ARPING(" [-a[MSEC]]")" [-t N] [-T SEC] [-A SEC/-n]\n"
//usage:       "	[-i IFACE]"IF_FEATURE_UDHCP_PORT(" [-P PORT]")" [-s PROG] [-p PIDFILE]\n"
//usage:       "	[-oC] [-r IP] [-V VENDOR] [-F NAME] [-x OPT:VAL]... [-O OPT]..."
//usage:#define udhcpc_full_usage "\n"
//usage:     "\n	-i IFACE	Interface to use (default eth0)"
//usage:	IF_FEATURE_UDHCP_PORT(
//usage:     "\n	-P PORT		Use PORT (default 68)"
//usage:	)
//usage:     "\n	-s PROG		Run PROG at DHCP events (default "CONFIG_UDHCPC_DEFAULT_SCRIPT")"
//usage:     "\n	-p FILE		Create pidfile"
//usage:     "\n	-B		Request broadcast replies"
//usage:     "\n	-t N		Send up to N discover packets (default 3)"
//usage:     "\n	-T SEC		Pause between packets (default 3)"
//usage:     "\n	-A SEC		Wait if lease is not obtained (default 20)"
//usage:	USE_FOR_MMU(
//usage:     "\n	-b		Background if lease is not obtained"
//usage:	)
//usage:     "\n	-n		Exit if lease is not obtained"
//usage:     "\n	-q		Exit after obtaining lease"
//usage:     "\n	-R		Release IP on exit"
//usage:     "\n	-f		Run in foreground"
//usage:     "\n	-S		Log to syslog too"
//usage:	IF_FEATURE_UDHCPC_ARPING(
//usage:     "\n	-a[MSEC]	Validate offered address with ARP ping"
//usage:	)
//usage:     "\n	-r IP		Request this IP address"
//usage:     "\n	-o		Don't request any options (unless -O is given)"
//usage:     "\n	-O OPT		Request option OPT from server (cumulative)"
//usage:     "\n	-x OPT:VAL	Include option OPT in sent packets (cumulative)"
//usage:     "\n			Examples of string, numeric, and hex byte opts:"
//usage:     "\n			-x hostname:bbox - option 12"
//usage:     "\n			-x lease:3600 - option 51 (lease time)"
//usage:     "\n			-x 0x3d:0100BEEFC0FFEE - option 61 (client id)"
//usage:     "\n			-x 14:'\"dumpfile\"' - option 14 (shell-quoted)"
//usage:     "\n	-F NAME		Ask server to update DNS mapping for NAME"
//usage:     "\n	-V VENDOR	Vendor identifier (default 'udhcp VERSION')"
//usage:     "\n	-C		Don't send MAC as client identifier"
//usage:	IF_UDHCP_VERBOSE(
//usage:     "\n	-v		Verbose"
//usage:	)
//usage:     "\nSignals:"
//usage:     "\n	USR1	Renew lease"
//usage:     "\n	USR2	Release lease"
#[no_mangle]
pub unsafe extern "C" fn udhcpc_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64; /* for compiler */
  let mut message: *mut u8 = 0 as *mut u8; /* for compiler */
  let mut str_V: *const libc::c_char = 0 as *const libc::c_char; /* must be signed */
  let mut str_h: *const libc::c_char = 0 as *const libc::c_char;
  let mut str_F: *const libc::c_char = 0 as *const libc::c_char;
  let mut str_r: *const libc::c_char = 0 as *const libc::c_char;
  let mut str_a: *const libc::c_char = b"2000\x00" as *const u8 as *const libc::c_char;
  let mut clientid_mac_ptr: *mut libc::c_void = 0 as *mut libc::c_void;
  let mut list_O: *mut llist_t = 0 as *mut llist_t;
  let mut list_x: *mut llist_t = 0 as *mut llist_t;
  let mut tryagain_timeout: libc::c_int = 20i32;
  let mut discover_timeout: libc::c_int = 3i32;
  let mut discover_retries: libc::c_int = 3i32;
  let mut server_addr: u32 = 0;
  server_addr = server_addr;
  let mut requested_ip: u32 = 0i32 as u32;
  let mut xid: u32 = 0;
  xid = xid;
  let mut packet_num: libc::c_int = 0;
  let mut timeout: libc::c_int = 0;
  let mut already_waited_sec: libc::c_uint = 0;
  let mut opt: libc::c_uint = 0;
  let mut arpping_ms: libc::c_uint = 0;
  let mut retval: libc::c_int = 0;
  /* Default options */
  let ref mut fresh54 = (*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
    as *mut libc::c_char as *mut client_data_t))
    .interface;
  *fresh54 = b"eth0\x00" as *const u8 as *const libc::c_char;
  let ref mut fresh55 = (*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
    as *mut libc::c_char as *mut client_data_t))
    .script;
  *fresh55 = b"/usr/share/udhcpc/default.script\x00" as *const u8 as *const libc::c_char;
  (*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
    as *mut client_data_t))
    .sockfd = -1i32;
  str_V = b"udhcp 1.32.0.git\x00" as *const u8 as *const libc::c_char;
  /* Make sure fd 0,1,2 are open */
  /* Set up the signal pipe on fds 3,4 - must be before openlog() */
  udhcp_sp_setup();
  /* Parse command line */
  opt = getopt32long(
    argv,
    b"^CV:H:h:F:i:np:qRr:s:T:+t:+SA:+O:*ox:*fBba::v\x00vv\x00" as *const u8 as *const libc::c_char,
    udhcpc_longopts.as_ptr(),
    &mut str_V as *mut *const libc::c_char,
    &mut str_h as *mut *const libc::c_char,
    &mut str_h as *mut *const libc::c_char,
    &mut str_F as *mut *const libc::c_char,
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
    &mut str_a as *mut *const libc::c_char,
    &mut dhcp_verbose as *mut libc::c_uint,
  );
  if opt & (OPT_h as libc::c_int | OPT_H as libc::c_int) as libc::c_uint != 0 {
    //msg added 2011-11
    bb_simple_error_msg(
      b"option -h NAME is deprecated, use -x hostname:NAME\x00" as *const u8 as *const libc::c_char,
    );
    let ref mut fresh56 = (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
      as *mut libc::c_char as *mut client_data_t))
      .hostname;
    *fresh56 = alloc_dhcp_option(0xci32, str_h, 0i32)
  }
  if opt & OPT_F as libc::c_int as libc::c_uint != 0 {
    /* FQDN option format: [0x51][len][flags][0][0]<fqdn> */
    let ref mut fresh57 = (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
      as *mut libc::c_char as *mut client_data_t))
      .fqdn;
    *fresh57 = alloc_dhcp_option(0x51i32, str_F, 3i32);
    /*client_data.fqdn[OPT_DATA + 1] = 0; - xzalloc did it */
    /*client_data.fqdn[OPT_DATA + 2] = 0; */
    *(*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .fqdn
      .offset((2i32 + 0i32) as isize) = 0x1i32 as u8
  }
  if opt & OPT_r as libc::c_int as libc::c_uint != 0 {
    requested_ip = inet_addr(str_r)
  }
  arpping_ms = xatou(str_a);
  while !list_O.is_null() {
    let mut optstr: *mut libc::c_char = llist_pop(&mut list_O) as *mut libc::c_char;
    let mut n: libc::c_uint = bb_strtou(optstr, 0 as *mut *mut libc::c_char, 0i32);
    if *bb_errno != 0 || n > 254i32 as libc::c_uint {
      n = udhcp_option_idx(optstr, dhcp_option_strings.as_ptr());
      n = (*dhcp_optflags.as_ptr().offset(n as isize)).code as libc::c_uint
    }
    let ref mut fresh58 = (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
      as *mut libc::c_char as *mut client_data_t))
      .opt_mask[(n >> 3i32) as usize];
    *fresh58 = (*fresh58 as libc::c_int | 1i32 << (n & 7i32 as libc::c_uint)) as u8
  }
  if opt & OPT_o as libc::c_int as libc::c_uint == 0 {
    let mut i: libc::c_uint = 0;
    let mut n_0: libc::c_uint = 0;
    i = 0i32 as libc::c_uint;
    loop {
      n_0 = (*dhcp_optflags.as_ptr().offset(i as isize)).code as libc::c_uint;
      if !(n_0 != 0i32 as libc::c_uint) {
        break;
      }
      if (*dhcp_optflags.as_ptr().offset(i as isize)).flags as libc::c_int
        & OPTION_REQ as libc::c_int
        != 0
      {
        let ref mut fresh59 = (*(&mut *bb_common_bufsiz1
          .as_mut_ptr()
          .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
          as *mut libc::c_char as *mut client_data_t))
          .opt_mask[(n_0 >> 3i32) as usize];
        *fresh59 = (*fresh59 as libc::c_int | 1i32 << (n_0 & 7i32 as libc::c_uint)) as u8
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
      dhcp_optflags.as_ptr(),
      dhcp_option_strings.as_ptr(),
      0i32 != 0,
    );
    free(optstr_0 as *mut libc::c_void);
  }
  if udhcp_read_interface(
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
    0 as *mut u32,
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
  clientid_mac_ptr = 0 as *mut libc::c_void;
  if opt & OPT_C as libc::c_int as libc::c_uint == 0
    && udhcp_find_option(
      (*(&mut *bb_common_bufsiz1
        .as_mut_ptr()
        .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
        as *mut client_data_t))
        .options,
      0x3di32 as u8,
    )
    .is_null()
  {
    /* Flag bits: 0000NEOS
     * S: 1 = Client requests server to update A RR in DNS as well as PTR
     * O: 1 = Server indicates to client that DNS has been updated regardless
     * E: 1 = Name is in DNS format, i.e. <4>host<6>domain<3>com<0>,
     *    not "host.domain.com". Format 0 is obsolete.
     * N: 1 = Client requests server to not update DNS (S must be 0 then)
     * Two [0] bytes which follow are deprecated and must be 0.
     */
    /* not suppressed and not set, set the default client ID */
    let ref mut fresh60 = (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
      as *mut libc::c_char as *mut client_data_t))
      .clientid; /* type: ethernet */
    *fresh60 = alloc_dhcp_option(0x3di32, b"\x00" as *const u8 as *const libc::c_char, 7i32);
    *(*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
      as *mut client_data_t))
      .clientid
      .offset(2) = 1i32 as u8;
    clientid_mac_ptr = (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
      as *mut libc::c_char as *mut client_data_t))
      .clientid
      .offset(2)
      .offset(1) as *mut libc::c_void;
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
  }
  if *str_V.offset(0) as libc::c_int != '\u{0}' as i32 {
    // can drop -V, str_V, client_data.vendorclass,
    // but need to add "vendor" to the list of recognized
    // string opts for this to work;
    // and need to tweak add_client_options() too...
    // ...so the question is, should we?
    //bb_error_msg("option -V VENDOR is deprecated, use -x vendor:VENDOR");
    let ref mut fresh61 = (*(&mut *bb_common_bufsiz1
      .as_mut_ptr()
      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
      as *mut libc::c_char as *mut client_data_t))
      .vendorclass;
    *fresh61 = alloc_dhcp_option(0x3ci32, str_V, 0i32)
  }
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
  /* We want random_xid to be random... */
  srand(monotonic_us() as libc::c_uint);
  (*(&mut *bb_common_bufsiz1
    .as_mut_ptr()
    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
    as *mut client_data_t))
    .state = 0i32 as smallint;
  udhcp_run_script(
    0 as *mut dhcp_packet,
    b"deconfig\x00" as *const u8 as *const libc::c_char,
  );
  change_listen_mode(2i32);
  packet_num = 0i32;
  timeout = 0i32;
  already_waited_sec = 0i32 as libc::c_uint;
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
    let mut packet: dhcp_packet = dhcp_packet {
      op: 0,
      htype: 0,
      hlen: 0,
      hops: 0,
      xid: 0,
      secs: 0,
      flags: 0,
      ciaddr: 0,
      yiaddr: 0,
      siaddr_nip: 0,
      gateway_nip: 0,
      chaddr: [0; 16],
      sname: [0; 64],
      file: [0; 128],
      cookie: 0,
      options: [0; 388],
    };
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
    retval = 0i32;
    /* If we already timed out, fall through with retval = 0, else... */
    if tv > 0i32 {
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
      if retval < 0i32 {
        /* EINTR? A signal was caught, don't panic */
        if *bb_errno == 4i32 {
          already_waited_sec =
            already_waited_sec.wrapping_add(monotonic_sec().wrapping_sub(timestamp_before_wait));
          continue;
        } else {
          /* Else: an error occurred, panic! */
          bb_simple_perror_msg_and_die(b"poll\x00" as *const u8 as *const libc::c_char);
        }
      }
    }
    /* If timeout dropped to zero, time to become active:
     * resend discover/renew/whatever
     */
    if retval == 0i32 {
      /* if poll timed out */
      /* When running on a bridge, the ifindex may have changed
       * (e.g. if member interfaces were added/removed
       * or if the status of the bridge changed).
       * Refresh ifindex and client_mac:
       */
      if udhcp_read_interface(
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
        0 as *mut u32,
        (*(&mut *bb_common_bufsiz1
          .as_mut_ptr()
          .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
          as *mut client_data_t))
          .client_mac
          .as_mut_ptr(),
      ) != 0
      {
        current_block = 5606174660785430415;
        break;
      /* iface is gone? */
      } else {
        if !clientid_mac_ptr.is_null() {
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
        }
        /* We will restart the wait in any case */
        already_waited_sec = 0i32 as libc::c_uint;
        match (*(&mut *bb_common_bufsiz1
          .as_mut_ptr()
          .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
          as *mut libc::c_char as *mut client_data_t))
          .state as libc::c_int
        {
          0 => {
            current_block = 4490583144771985778;
            match current_block {
              10570719081292997246 => {
                /* yah, I know, *you* say it would never happen */
                timeout = 2147483647i32;
                continue;
              }
              10109487716296584538 => {
                if packet_num < 3i32 {
                  /* send broadcast select packet */
                  send_select(xid, server_addr, requested_ip);
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
                    .state = 0i32 as smallint
                }
                current_block = 2086477769549425743;
              }
              4490583144771985778 => {
                if discover_retries == 0 || packet_num < discover_retries {
                  if packet_num == 0i32 {
                    xid = random_xid()
                  }
                  /* broadcast */
                  send_discover(xid, requested_ip);
                  timeout = discover_timeout;
                  packet_num += 1;
                  continue;
                } else {
                  current_block = 2086477769549425743;
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
                  .first_secs = 0i32 as libc::c_uint;
                change_listen_mode(1i32);
                if dhcp_verbose >= 1i32 as libc::c_uint {
                  bb_simple_info_msg(
                    b"entering renew state\x00" as *const u8 as *const libc::c_char,
                  );
                }
                /* fall right through */
                current_block = 18276703784498340658;
              }
            }
            match current_block {
              18276703784498340658 => {}
              _ => {
                udhcp_run_script(
                  0 as *mut dhcp_packet,
                  b"leasefail\x00" as *const u8 as *const libc::c_char,
                );
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
                  current_block = 795874283904168307;
                  break;
                }
                /* wait before trying again */
                timeout = tryagain_timeout;
                packet_num = 0i32;
                continue;
              }
            }
          }
          1 => {
            current_block = 10109487716296584538;
            match current_block {
              10570719081292997246 => {
                timeout = 2147483647i32;
                continue;
              }
              10109487716296584538 => {
                if packet_num < 3i32 {
                  send_select(xid, server_addr, requested_ip);
                  timeout = discover_timeout;
                  packet_num += 1;
                  continue;
                } else {
                  change_listen_mode(2i32);
                  (*(&mut *bb_common_bufsiz1
                    .as_mut_ptr()
                    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                    as *mut libc::c_char as *mut client_data_t))
                    .state = 0i32 as smallint
                }
                current_block = 2086477769549425743;
              }
              4490583144771985778 => {
                if discover_retries == 0 || packet_num < discover_retries {
                  if packet_num == 0i32 {
                    xid = random_xid()
                  }
                  send_discover(xid, requested_ip);
                  timeout = discover_timeout;
                  packet_num += 1;
                  continue;
                } else {
                  current_block = 2086477769549425743;
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
                  .first_secs = 0i32 as libc::c_uint;
                change_listen_mode(1i32);
                if dhcp_verbose >= 1i32 as libc::c_uint {
                  bb_simple_info_msg(
                    b"entering renew state\x00" as *const u8 as *const libc::c_char,
                  );
                }
                current_block = 18276703784498340658;
              }
            }
            match current_block {
              18276703784498340658 => {}
              _ => {
                udhcp_run_script(
                  0 as *mut dhcp_packet,
                  b"leasefail\x00" as *const u8 as *const libc::c_char,
                );
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
                  current_block = 795874283904168307;
                  break;
                }
                timeout = tryagain_timeout;
                packet_num = 0i32;
                continue;
              }
            }
          }
          2 => {
            current_block = 1216349088684621954;
            match current_block {
              10570719081292997246 => {
                timeout = 2147483647i32;
                continue;
              }
              10109487716296584538 => {
                if packet_num < 3i32 {
                  send_select(xid, server_addr, requested_ip);
                  timeout = discover_timeout;
                  packet_num += 1;
                  continue;
                } else {
                  change_listen_mode(2i32);
                  (*(&mut *bb_common_bufsiz1
                    .as_mut_ptr()
                    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                    as *mut libc::c_char as *mut client_data_t))
                    .state = 0i32 as smallint
                }
                current_block = 2086477769549425743;
              }
              4490583144771985778 => {
                if discover_retries == 0 || packet_num < discover_retries {
                  if packet_num == 0i32 {
                    xid = random_xid()
                  }
                  send_discover(xid, requested_ip);
                  timeout = discover_timeout;
                  packet_num += 1;
                  continue;
                } else {
                  current_block = 2086477769549425743;
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
                  .first_secs = 0i32 as libc::c_uint;
                change_listen_mode(1i32);
                if dhcp_verbose >= 1i32 as libc::c_uint {
                  bb_simple_info_msg(
                    b"entering renew state\x00" as *const u8 as *const libc::c_char,
                  );
                }
                current_block = 18276703784498340658;
              }
            }
            match current_block {
              18276703784498340658 => {}
              _ => {
                udhcp_run_script(
                  0 as *mut dhcp_packet,
                  b"leasefail\x00" as *const u8 as *const libc::c_char,
                );
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
                  current_block = 795874283904168307;
                  break;
                }
                timeout = tryagain_timeout;
                packet_num = 0i32;
                continue;
              }
            }
          }
          5 | 3 => {
            current_block = 18276703784498340658;
          }
          4 => {
            current_block = 8680220904978397701;
          }
          _ => {
            current_block = 10570719081292997246;
            match current_block {
              10570719081292997246 => {
                timeout = 2147483647i32;
                continue;
              }
              10109487716296584538 => {
                if packet_num < 3i32 {
                  send_select(xid, server_addr, requested_ip);
                  timeout = discover_timeout;
                  packet_num += 1;
                  continue;
                } else {
                  change_listen_mode(2i32);
                  (*(&mut *bb_common_bufsiz1
                    .as_mut_ptr()
                    .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                    as *mut libc::c_char as *mut client_data_t))
                    .state = 0i32 as smallint
                }
                current_block = 2086477769549425743;
              }
              4490583144771985778 => {
                if discover_retries == 0 || packet_num < discover_retries {
                  if packet_num == 0i32 {
                    xid = random_xid()
                  }
                  send_discover(xid, requested_ip);
                  timeout = discover_timeout;
                  packet_num += 1;
                  continue;
                } else {
                  current_block = 2086477769549425743;
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
                  .first_secs = 0i32 as libc::c_uint;
                change_listen_mode(1i32);
                if dhcp_verbose >= 1i32 as libc::c_uint {
                  bb_simple_info_msg(
                    b"entering renew state\x00" as *const u8 as *const libc::c_char,
                  );
                }
                current_block = 18276703784498340658;
              }
            }
            match current_block {
              18276703784498340658 => {}
              _ => {
                udhcp_run_script(
                  0 as *mut dhcp_packet,
                  b"leasefail\x00" as *const u8 as *const libc::c_char,
                );
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
                  current_block = 795874283904168307;
                  break;
                }
                timeout = tryagain_timeout;
                packet_num = 0i32;
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
            .first_secs = 0i32 as libc::c_uint; /* make secs field count from 0 */
          already_waited_sec = 0i32 as libc::c_uint;
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
            packet_num = 0i32;
            /* Kill any timeouts, user wants this to hurry along */
            timeout = 0i32;
            continue;
          }
        }
        12 => {
          perform_release(server_addr, requested_ip);
          timeout = 2147483647i32;
          continue;
        }
        15 => {
          bb_info_msg(
            b"received %s\x00" as *const u8 as *const libc::c_char,
            b"SIGTERM\x00" as *const u8 as *const libc::c_char,
          );
          current_block = 5606174660785430415;
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
            len = udhcp_recv_kernel_packet(
              &mut packet,
              (*(&mut *bb_common_bufsiz1
                .as_mut_ptr()
                .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                as *mut libc::c_char as *mut client_data_t))
                .sockfd,
            )
          } else {
            len = udhcp_recv_raw_packet(
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
          if len < 0i32 {
            continue;
          }
          if packet.xid != xid {
            if dhcp_verbose >= 1i32 as libc::c_uint {
              bb_info_msg(
                b"xid %x (our is %x), ignoring packet\x00" as *const u8 as *const libc::c_char,
                packet.xid,
                xid,
              );
            }
            continue;
          } else if packet.hlen as libc::c_int != 6i32
            || memcmp(
              packet.chaddr.as_mut_ptr() as *const libc::c_void,
              (*(&mut *bb_common_bufsiz1
                .as_mut_ptr()
                .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                as *mut libc::c_char as *mut client_data_t))
                .client_mac
                .as_mut_ptr() as *const libc::c_void,
              6i32 as libc::c_ulong,
            ) != 0i32
          {
            /* Ignore packets that aren't for us */
            //FIXME: need to also check that last 10 bytes are zero
            if dhcp_verbose >= 1i32 as libc::c_uint {
              bb_info_msg(
                b"chaddr does not match%s\x00" as *const u8 as *const libc::c_char,
                b", ignoring packet\x00" as *const u8 as *const libc::c_char,
              ); // log2?
            }
            continue;
          } else {
            message = udhcp_get_option(&mut packet, 0x35i32);
            if message.is_null() {
              bb_info_msg(
                b"no message type option%s\x00" as *const u8 as *const libc::c_char,
                b", ignoring packet\x00" as *const u8 as *const libc::c_char,
              );
              continue;
            } else {
              match (*(&mut *bb_common_bufsiz1
                .as_mut_ptr()
                .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                as *mut libc::c_char as *mut client_data_t))
                .state as libc::c_int
              {
                0 => {
                  /* Must be a DHCPOFFER */
                  if *message as libc::c_int == 2i32 {
                    let mut temp: *mut u8 = 0 as *mut u8;
                    /* What exactly is server's IP? There are several values.
                     * Example DHCP offer captured with tchdump:
                     *
                     * 10.34.25.254:67 > 10.34.25.202:68 // IP header's src
                     * BOOTP fields:
                     * Your-IP 10.34.25.202
                     * Server-IP 10.34.32.125   // "next server" IP
                     * Gateway-IP 10.34.25.254  // relay's address (if DHCP relays are in use)
                     * DHCP options:
                     * DHCP-Message Option 53, length 1: Offer
                     * Server-ID Option 54, length 4: 10.34.255.7       // "server ID"
                     * Default-Gateway Option 3, length 4: 10.34.25.254 // router
                     *
                     * We think that real server IP (one to use in renew/release)
                     * is one in Server-ID option. But I am not 100% sure.
                     * IP header's src and Gateway-IP (same in this example)
                     * might work too.
                     * "Next server" and router are definitely wrong ones to use, though...
                     */
                    /* We used to ignore packets without DHCP_SERVER_ID.
                     * I've got user reports from people who run "address-less" servers.
                     * They either supply DHCP_SERVER_ID of 0.0.0.0 or don't supply it at all.
                     * They say ISC DHCP client supports this case.
                     */
                    server_addr = 0i32 as u32;
                    temp = udhcp_get_option32(&mut packet, 0x36i32);
                    if temp.is_null() {
                      bb_simple_info_msg(
                        b"no server ID, using 0.0.0.0\x00" as *const u8 as *const libc::c_char,
                      );
                    } else {
                      /* it IS unaligned sometimes, don't "optimize" */
                      server_addr = *(temp as *mut bb__aliased_u32)
                    }
                    /*xid = packet.xid; - already is */
                    requested_ip = packet.yiaddr;
                    /* enter requesting state */
                    (*(&mut *bb_common_bufsiz1
                      .as_mut_ptr()
                      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                      as *mut libc::c_char as *mut client_data_t))
                      .state = 1i32 as smallint;
                    timeout = 0i32;
                    packet_num = 0i32;
                    already_waited_sec = 0i32 as libc::c_uint
                  }
                  continue;
                  /* case BOUND: - ignore all packets */
                  /* case RELEASED: - ignore all packets */
                }
                1 | 3 | 5 | 4 => {
                  if *message as libc::c_int == 5i32 {
                    let mut start: libc::c_uint = 0;
                    let mut lease_seconds: u32 = 0;
                    let mut temp_addr: in_addr = in_addr { s_addr: 0 };
                    let mut temp_0: *mut u8 = 0 as *mut u8;
                    temp_0 = udhcp_get_option32(&mut packet, 0x33i32);
                    if temp_0.is_null() {
                      bb_simple_info_msg(
                        b"no lease time with ACK, using 1 hour lease\x00" as *const u8
                          as *const libc::c_char,
                      );
                      lease_seconds = (60i32 * 60i32) as u32
                    } else {
                      /* back to main loop */
                      /* it IS unaligned sometimes, don't "optimize" */
                      lease_seconds = *(temp_0 as *mut bb__aliased_u32);
                      lease_seconds = {
                        let mut __v: libc::c_uint = 0;
                        let mut __x: libc::c_uint = lease_seconds;
                        if 0 != 0 {
                          __v = (__x & 0xff000000u32) >> 24i32
                            | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
                            | (__x & 0xff00i32 as libc::c_uint) << 8i32
                            | (__x & 0xffi32 as libc::c_uint) << 24i32
                        } else {
                          let fresh62 = &mut __v;
                          let fresh63;
                          let fresh64 = __x;
                          asm!("bswap $0" :
                                                              "=r" (fresh63) :
                                                              "0"
                                                              (c2rust_asm_casts::AsmCast::cast_in(fresh62, fresh64))
                                                              :);
                          c2rust_asm_casts::AsmCast::cast_out(fresh62, fresh64, fresh63);
                        }
                        __v
                      };
                      //if (lease_seconds > 0x7fffffff)
                      //	lease_seconds = 0x7fffffff;
                      //^^^not necessary since "timeout = lease_seconds / 2"
                      //does not overflow even for 0xffffffff.
                      if lease_seconds < (2i32 * 61i32) as libc::c_uint {
                        lease_seconds = (2i32 * 61i32) as u32
                      }
                    }
                    if opt & OPT_a as libc::c_int as libc::c_uint != 0 {
                      /* paranoia: must not be too small and not prone to overflows */
                      /* timeout > 60 - ensures at least one unicast renew attempt */
                      /* RFC 2131 3.1 paragraph 5:
                       * "The client receives the DHCPACK message with configuration
                       * parameters. The client SHOULD perform a final check on the
                       * parameters (e.g., ARP for allocated network address), and notes
                       * the duration of the lease specified in the DHCPACK message. At this
                       * point, the client is configured. If the client detects that the
                       * address is already in use (e.g., through the use of ARP),
                       * the client MUST send a DHCPDECLINE message to the server and restarts
                       * the configuration process..." */
                      if arpping(
                        packet.yiaddr,
                        0 as *const u8,
                        0i32 as u32,
                        (*(&mut *bb_common_bufsiz1
                          .as_mut_ptr()
                          .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                          as *mut libc::c_char as *mut client_data_t))
                          .client_mac
                          .as_mut_ptr(),
                        (*(&mut *bb_common_bufsiz1
                          .as_mut_ptr()
                          .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                          as *mut libc::c_char as *mut client_data_t))
                          .interface,
                        arpping_ms,
                      ) == 0
                      {
                        bb_simple_info_msg(
                          b"offered address is in use (got ARP reply), declining\x00" as *const u8
                            as *const libc::c_char,
                        );
                        send_decline(server_addr, packet.yiaddr);
                        if (*(&mut *bb_common_bufsiz1
                          .as_mut_ptr()
                          .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                          as *mut libc::c_char as *mut client_data_t))
                          .state as libc::c_int
                          != 1i32
                        {
                          udhcp_run_script(
                            0 as *mut dhcp_packet,
                            b"deconfig\x00" as *const u8 as *const libc::c_char,
                          );
                        }
                        change_listen_mode(2i32);
                        (*(&mut *bb_common_bufsiz1
                          .as_mut_ptr()
                          .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                          as *mut libc::c_char as *mut client_data_t))
                          .state = 0i32 as smallint;
                        /* back to main loop */
                        (*(&mut *bb_common_bufsiz1
                          .as_mut_ptr()
                          .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                          as *mut libc::c_char as *mut client_data_t))
                          .first_secs = 0i32 as libc::c_uint; /* make secs field count from 0 */
                        requested_ip = 0i32 as u32;
                        timeout = tryagain_timeout;
                        packet_num = 0i32;
                        already_waited_sec = 0i32 as libc::c_uint;
                        continue;
                      }
                    }
                    /* enter bound state */
                    temp_addr.s_addr = packet.yiaddr;
                    bb_info_msg(
                      b"lease of %s obtained, lease time %u\x00" as *const u8
                        as *const libc::c_char,
                      inet_ntoa(temp_addr),
                      lease_seconds,
                    );
                    requested_ip = packet.yiaddr;
                    start = monotonic_sec();
                    udhcp_run_script(
                      &mut packet,
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
                    already_waited_sec = monotonic_sec().wrapping_sub(start);
                    timeout = lease_seconds.wrapping_div(2i32 as libc::c_uint) as libc::c_int;
                    if (timeout as libc::c_uint) < already_waited_sec {
                      /* Something went wrong. Back to discover state */
                      already_waited_sec = 0i32 as libc::c_uint;
                      timeout = already_waited_sec as libc::c_int
                    }
                    (*(&mut *bb_common_bufsiz1
                      .as_mut_ptr()
                      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                      as *mut libc::c_char as *mut client_data_t))
                      .state = 2i32 as smallint;
                    change_listen_mode(0i32);
                    if opt & OPT_q as libc::c_int as libc::c_uint != 0 {
                      current_block = 5606174660785430415;
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
                    continue;
                  } else {
                    if !(*message as libc::c_int == 6i32) {
                      continue;
                    }
                    /* make future renew packets use different xid */
                    /* xid = random_xid(); ...but why bother? */
                    /* If network has more than one DHCP server,
                     * "wrong" server can reply first, with a NAK.
                     * Do not interpret it as a NAK from "our" server.
                     */
                    if server_addr != 0i32 as libc::c_uint {
                      let mut svid: u32 = 0;
                      let mut temp_1: *mut u8 = 0 as *mut u8;
                      temp_1 = udhcp_get_option32(&mut packet, 0x36i32);
                      if temp_1.is_null() {
                        current_block = 13335948699411770265;
                      } else {
                        svid = *(temp_1 as *mut bb__aliased_u32);
                        if svid != server_addr {
                          current_block = 13335948699411770265;
                        } else {
                          current_block = 2493083811365744214;
                        }
                      }
                      match current_block {
                        2493083811365744214 => {}
                        _ => {
                          if dhcp_verbose >= 1i32 as libc::c_uint {
                            bb_info_msg(
                              b"received DHCP NAK with wrong server ID%s\x00" as *const u8
                                as *const libc::c_char,
                              b", ignoring packet\x00" as *const u8 as *const libc::c_char,
                            );
                          }
                          continue;
                        }
                      }
                    }
                    /* return to init state */
                    bb_info_msg(
                      b"received %s\x00" as *const u8 as *const libc::c_char,
                      b"DHCP NAK\x00" as *const u8 as *const libc::c_char,
                    ); /* avoid excessive network traffic */
                    udhcp_run_script(&mut packet, b"nak\x00" as *const u8 as *const libc::c_char); /* make secs field count from 0 */
                    if (*(&mut *bb_common_bufsiz1
                      .as_mut_ptr()
                      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                      as *mut libc::c_char as *mut client_data_t))
                      .state as libc::c_int
                      != 1i32
                    {
                      udhcp_run_script(
                        0 as *mut dhcp_packet,
                        b"deconfig\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    change_listen_mode(2i32);
                    sleep(3i32 as libc::c_uint);
                    (*(&mut *bb_common_bufsiz1
                      .as_mut_ptr()
                      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                      as *mut libc::c_char as *mut client_data_t))
                      .state = 0i32 as smallint;
                    (*(&mut *bb_common_bufsiz1
                      .as_mut_ptr()
                      .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize)
                      as *mut libc::c_char as *mut client_data_t))
                      .first_secs = 0i32 as libc::c_uint;
                    requested_ip = 0i32 as u32;
                    timeout = 0i32;
                    packet_num = 0i32;
                    already_waited_sec = 0i32 as libc::c_uint;
                    continue;
                  }
                }
                _ => {
                  continue;
                }
              }
            }
          }
        }
      }
      current_block = 18276703784498340658;
    }
    match current_block {
      18276703784498340658 =>
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
          if send_renew(xid, server_addr, requested_ip) >= 0i32 {
            timeout >>= 1i32;
            //TODO: the timeout to receive an answer for our renew should not be selected
            //with "timeout = lease_seconds / 2; ...; timeout = timeout / 2": it is often huge.
            //Waiting e.g. 4*3600 seconds for a reply does not make sense
            //(if reply isn't coming, we keep an open socket for hours),
            //it should be something like 10 seconds.
            //Also, it's probably best to try sending renew in kernel mode a few (3-5) times
            //and fall back to raw mode if it does not work.
            continue;
          }
          /* else: error sending.
           * example: ENETUNREACH seen with server
           * which gave us bogus server ID 1.1.1.1
           * which wasn't reachable (and probably did not exist).
           */
        }
        /* Timed out or error, enter rebinding state */
        if dhcp_verbose >= 1i32 as libc::c_uint {
          bb_simple_info_msg(b"entering rebinding state\x00" as *const u8 as *const libc::c_char);
        }
        (*(&mut *bb_common_bufsiz1
          .as_mut_ptr()
          .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
          as *mut client_data_t))
          .state = 4i32 as smallint
      }
      _ => {}
    }
    /* fall right through */
    /* Switch to bcast receive */
    change_listen_mode(2i32);
    if timeout > 0i32 {
      /* Lease is *really* about to run out,
       * try to find DHCP server using broadcast */
      /* send a broadcast renew request */
      send_renew(xid, 0i32 as u32, requested_ip);
      timeout >>= 1i32
    } else {
      /* Timed out, enter init state */
      bb_simple_info_msg(
        b"lease lost, entering init state\x00" as *const u8 as *const libc::c_char,
      ); /* make secs field count from 0 */
      udhcp_run_script(
        0 as *mut dhcp_packet,
        b"deconfig\x00" as *const u8 as *const libc::c_char,
      );
      (*(&mut *bb_common_bufsiz1
        .as_mut_ptr()
        .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
        as *mut client_data_t))
        .state = 0i32 as smallint;
      (*(&mut *bb_common_bufsiz1
        .as_mut_ptr()
        .offset((COMMON_BUFSIZE as libc::c_int / 2i32) as isize) as *mut libc::c_char
        as *mut client_data_t))
        .first_secs = 0i32 as libc::c_uint;
      /*timeout = 0; - already is */
      packet_num = 0i32
    }
  }
  match current_block {
    5606174660785430415 =>
    /* quit after lease */
    {
      if opt & OPT_R as libc::c_int as libc::c_uint != 0 {
        /* release on quit */
        perform_release(server_addr, requested_ip);
      }
      retval = 0i32
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
unsafe extern "C" fn run_static_initializers() {
  len_of_option_as_string = [
    0,
    ::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as u8,
    (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
      .wrapping_mul(2i32 as libc::c_ulong) as u8,
    1i32 as u8,
    1i32 as u8,
    ::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as u8,
    ::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as u8,
    ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as u8,
    ::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as u8,
    0,
    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as u8,
    ::std::mem::size_of::<[libc::c_char; 65]>() as libc::c_ulong as u8,
    1i32 as u8,
    ::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as u8,
  ]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

use c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
use libc;
extern "C" {
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut FILE,
  ) -> *mut libc::c_char;

  // These come from libresolv...
  #[no_mangle]
  fn ns_get16(_: *const libc::c_uchar) -> libc::c_uint;
  #[no_mangle]
  fn ns_get32(_: *const libc::c_uchar) -> libc::c_ulong;
  #[no_mangle]
  fn ns_initparse(_: *const libc::c_uchar, _: libc::c_int, _: *mut ns_msg) -> libc::c_int;
  #[no_mangle]
  fn ns_parserr(_: *mut ns_msg, _: ns_sect, _: libc::c_int, _: *mut ns_rr) -> libc::c_int;
  #[no_mangle]
  fn ns_name_uncompress(
    _: *const libc::c_uchar,
    _: *const libc::c_uchar,
    _: *const libc::c_uchar,
    _: *mut libc::c_char,
    _: size_t,
  ) -> libc::c_int;
  #[no_mangle]
  fn __res_mkquery(
    _: libc::c_int,
    _: *const libc::c_char,
    _: libc::c_int,
    _: libc::c_int,
    _: *const libc::c_uchar,
    _: libc::c_int,
    _: *const libc::c_uchar,
    _: *mut libc::c_uchar,
    _: libc::c_int,
  ) -> libc::c_int;
  // ...end resolv externs

  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn inet_pton(
    __af: libc::c_int,
    __cp: *const libc::c_char,
    __buf: *mut libc::c_void,
  ) -> libc::c_int;
  #[no_mangle]
  fn inet_ntop(
    __af: libc::c_int,
    __cp: *const libc::c_void,
    __buf: *mut libc::c_char,
    __len: socklen_t,
  ) -> *const libc::c_char;
  #[no_mangle]
  fn monotonic_ms() -> libc::c_ulonglong;
  #[no_mangle]
  fn skip_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn skip_non_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  /* After v = xrealloc_vector(v, SHIFT, idx) it's ok to use
   * at least v[idx] and v[idx+1], for all idx values.
   * SHIFT specifies how many new elements are added (1:2, 2:4, ..., 8:256...)
   * when all elements are used up. New elements are zeroed out.
   * xrealloc_vector(v, SHIFT, idx) *MUST* be called with consecutive IDXs -
   * skipping an index is a bad bug - it may miss a realloc!
   */
  #[no_mangle]
  fn xrealloc_vector_helper(
    vector: *mut libc::c_void,
    sizeof_and_shift: libc::c_uint,
    idx: libc::c_int,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn ndelay_on(fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xbind(sockfd: libc::c_int, my_addr: *mut sockaddr, addrlen: socklen_t);
  #[no_mangle]
  fn xconnect(s: libc::c_int, s_addr: *const sockaddr, addrlen: socklen_t);
  /* Create stream socket, and allocate suitable lsa.
   * (lsa of correct size and lsa->sa.sa_family (AF_INET/AF_INET6))
   * af == AF_UNSPEC will result in trying to create IPv6 socket,
   * and if kernel doesn't support it, fall back to IPv4.
   * This is useful if you plan to bind to resulting local lsa.
   */
  #[no_mangle]
  fn xsocket_type(
    lsap: *mut *mut len_and_sockaddr,
    af: libc::c_int,
    sock_type: libc::c_int,
  ) -> libc::c_int;
  /* Version which dies on error */
  #[no_mangle]
  fn xhost2sockaddr(host: *const libc::c_char, port: libc::c_int) -> *mut len_and_sockaddr;
  /* inet_[ap]ton on steroids */
  #[no_mangle]
  fn xmalloc_sockaddr2dotted(sa: *const sockaddr) -> *mut libc::c_char;
  /* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
   * But potentially slow, don't use in one-billion-times loops */
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn auto_string(str: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn safe_gethostname() -> *mut libc::c_char;
  #[no_mangle]
  fn xatou_range(str: *const libc::c_char, l: libc::c_uint, u: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
  #[no_mangle]
  fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
  /* { "-", NULL } */
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn index_in_substrings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  /* NB: (bb_hexdigits_upcase[i] | 0x20) -> lowercase hex digit */
  #[no_mangle]
  static bb_hexdigits_upcase: [libc::c_char; 0];
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

pub type __socklen_t = libc::c_uint;
use crate::librb::size_t;
use libc::ssize_t;




use libc::FILE;
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
pub type __ns_sect = libc::c_uint;
pub const ns_s_max: __ns_sect = 4;
pub const ns_s_ar: __ns_sect = 3;
pub const ns_s_ud: __ns_sect = 2;
pub const ns_s_ns: __ns_sect = 2;
pub const ns_s_pr: __ns_sect = 1;
pub const ns_s_an: __ns_sect = 1;
pub const ns_s_zn: __ns_sect = 0;
pub const ns_s_qd: __ns_sect = 0;
pub type ns_sect = __ns_sect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __ns_msg {
  pub _msg: *const libc::c_uchar,
  pub _eom: *const libc::c_uchar,
  pub _id: u16,
  pub _flags: u16,
  pub _counts: [u16; 4],
  pub _sections: [*const libc::c_uchar; 4],
  pub _sect: ns_sect,
  pub _rrnum: libc::c_int,
  pub _msg_ptr: *const libc::c_uchar,
}
pub type ns_msg = __ns_msg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __ns_rr {
  pub name: [libc::c_char; 1025],
  pub type_0: u16,
  pub rr_class: u16,
  pub ttl: u32,
  pub rdlength: u16,
  pub rdata: *const libc::c_uchar,
}
pub type ns_rr = __ns_rr;
pub type __ns_opcode = libc::c_uint;
pub const ns_o_max: __ns_opcode = 6;
pub const ns_o_update: __ns_opcode = 5;
pub const ns_o_notify: __ns_opcode = 4;
pub const ns_o_status: __ns_opcode = 2;
pub const ns_o_iquery: __ns_opcode = 1;
pub const ns_o_query: __ns_opcode = 0;
pub type __ns_type = libc::c_uint;
pub const ns_t_max: __ns_type = 65536;
pub const ns_t_dlv: __ns_type = 32769;
pub const ns_t_ta: __ns_type = 32768;
pub const ns_t_avc: __ns_type = 258;
pub const ns_t_caa: __ns_type = 257;
pub const ns_t_uri: __ns_type = 256;
pub const ns_t_any: __ns_type = 255;
pub const ns_t_maila: __ns_type = 254;
pub const ns_t_mailb: __ns_type = 253;
pub const ns_t_axfr: __ns_type = 252;
pub const ns_t_ixfr: __ns_type = 251;
pub const ns_t_tsig: __ns_type = 250;
pub const ns_t_tkey: __ns_type = 249;
pub const ns_t_eui64: __ns_type = 109;
pub const ns_t_eui48: __ns_type = 108;
pub const ns_t_lp: __ns_type = 107;
pub const ns_t_l64: __ns_type = 106;
pub const ns_t_l32: __ns_type = 105;
pub const ns_t_nid: __ns_type = 104;
pub const ns_t_unspec: __ns_type = 103;
pub const ns_t_gid: __ns_type = 102;
pub const ns_t_uid: __ns_type = 101;
pub const ns_t_uinfo: __ns_type = 100;
pub const ns_t_spf: __ns_type = 99;
pub const ns_t_csync: __ns_type = 62;
pub const ns_t_openpgpkey: __ns_type = 61;
pub const ns_t_cdnskey: __ns_type = 60;
pub const ns_t_cds: __ns_type = 59;
pub const ns_t_talink: __ns_type = 58;
pub const ns_t_rkey: __ns_type = 57;
pub const ns_t_ninfo: __ns_type = 56;
pub const ns_t_hip: __ns_type = 55;
pub const ns_t_smimea: __ns_type = 53;
pub const ns_t_tlsa: __ns_type = 52;
pub const ns_t_nsec3param: __ns_type = 51;
pub const ns_t_nsec3: __ns_type = 50;
pub const ns_t_dhcid: __ns_type = 49;
pub const ns_t_dnskey: __ns_type = 48;
pub const ns_t_nsec: __ns_type = 47;
pub const ns_t_rrsig: __ns_type = 46;
pub const ns_t_ipseckey: __ns_type = 45;
pub const ns_t_sshfp: __ns_type = 44;
pub const ns_t_ds: __ns_type = 43;
pub const ns_t_apl: __ns_type = 42;
pub const ns_t_opt: __ns_type = 41;
pub const ns_t_sink: __ns_type = 40;
pub const ns_t_dname: __ns_type = 39;
pub const ns_t_a6: __ns_type = 38;
pub const ns_t_cert: __ns_type = 37;
pub const ns_t_kx: __ns_type = 36;
pub const ns_t_naptr: __ns_type = 35;
pub const ns_t_atma: __ns_type = 34;
pub const ns_t_srv: __ns_type = 33;
pub const ns_t_nimloc: __ns_type = 32;
pub const ns_t_eid: __ns_type = 31;
pub const ns_t_nxt: __ns_type = 30;
pub const ns_t_loc: __ns_type = 29;
pub const ns_t_aaaa: __ns_type = 28;
pub const ns_t_gpos: __ns_type = 27;
pub const ns_t_px: __ns_type = 26;
pub const ns_t_key: __ns_type = 25;
pub const ns_t_sig: __ns_type = 24;
pub const ns_t_nsap_ptr: __ns_type = 23;
pub const ns_t_nsap: __ns_type = 22;
pub const ns_t_rt: __ns_type = 21;
pub const ns_t_isdn: __ns_type = 20;
pub const ns_t_x25: __ns_type = 19;
pub const ns_t_afsdb: __ns_type = 18;
pub const ns_t_rp: __ns_type = 17;
pub const ns_t_txt: __ns_type = 16;
pub const ns_t_mx: __ns_type = 15;
pub const ns_t_minfo: __ns_type = 14;
pub const ns_t_hinfo: __ns_type = 13;
pub const ns_t_ptr: __ns_type = 12;
pub const ns_t_wks: __ns_type = 11;
pub const ns_t_null: __ns_type = 10;
pub const ns_t_mr: __ns_type = 9;
pub const ns_t_mg: __ns_type = 8;
pub const ns_t_mb: __ns_type = 7;
pub const ns_t_soa: __ns_type = 6;
pub const ns_t_cname: __ns_type = 5;
pub const ns_t_mf: __ns_type = 4;
pub const ns_t_md: __ns_type = 3;
pub const ns_t_ns: __ns_type = 2;
pub const ns_t_a: __ns_type = 1;
pub const ns_t_invalid: __ns_type = 0;
pub type ns_type = __ns_type;
pub type __ns_class = libc::c_uint;
pub const ns_c_max: __ns_class = 65536;
pub const ns_c_any: __ns_class = 255;
pub const ns_c_none: __ns_class = 254;
pub const ns_c_hs: __ns_class = 4;
pub const ns_c_chaos: __ns_class = 3;
pub const ns_c_2: __ns_class = 2;
pub const ns_c_in: __ns_class = 1;
pub const ns_c_invalid: __ns_class = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct HEADER {
  #[bitfield(name = "id", ty = "libc::c_uint", bits = "0..=15")]
  #[bitfield(name = "rd", ty = "libc::c_uint", bits = "16..=16")]
  #[bitfield(name = "tc", ty = "libc::c_uint", bits = "17..=17")]
  #[bitfield(name = "aa", ty = "libc::c_uint", bits = "18..=18")]
  #[bitfield(name = "opcode", ty = "libc::c_uint", bits = "19..=22")]
  #[bitfield(name = "qr", ty = "libc::c_uint", bits = "23..=23")]
  #[bitfield(name = "rcode", ty = "libc::c_uint", bits = "24..=27")]
  #[bitfield(name = "cd", ty = "libc::c_uint", bits = "28..=28")]
  #[bitfield(name = "ad", ty = "libc::c_uint", bits = "29..=29")]
  #[bitfield(name = "unused", ty = "libc::c_uint", bits = "30..=30")]
  #[bitfield(name = "ra", ty = "libc::c_uint", bits = "31..=31")]
  #[bitfield(name = "qdcount", ty = "libc::c_uint", bits = "32..=47")]
  #[bitfield(name = "ancount", ty = "libc::c_uint", bits = "48..=63")]
  #[bitfield(name = "nscount", ty = "libc::c_uint", bits = "64..=79")]
  #[bitfield(name = "arcount", ty = "libc::c_uint", bits = "80..=95")]
  pub id_rd_tc_aa_opcode_qr_rcode_cd_ad_unused_ra_qdcount_ancount_nscount_arcount: [u8; 12],
}
pub type smalluint = libc::c_uchar;
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
  pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
  pub sa: sockaddr,
  pub sin: sockaddr_in,
  pub sin6: sockaddr_in6,
}
//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub default_port: libc::c_uint,
  pub default_retry: libc::c_uint,
  pub default_timeout: libc::c_uint,
  pub query_count: libc::c_uint,
  pub serv_count: libc::c_uint,
  pub server: *mut ns,
  pub query: *mut query,
  pub search: *mut libc::c_char,
  pub have_search_directive: smalluint,
  pub exitcode: smalluint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct query {
  pub name: *const libc::c_char,
  pub qlen: libc::c_uint,
  pub query: [libc::c_uchar; 512],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ns {
  pub name: *const libc::c_char,
  pub lsa: *mut len_and_sockaddr,
  pub replies: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
  pub type_0: libc::c_uchar,
  pub name: [libc::c_char; 7],
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const OPT_debug: C2RustUnnamed_2 = 1;
static mut qtypes: [C2RustUnnamed_1; 10] = [
  {
    let mut init = C2RustUnnamed_1 {
      type_0: ns_t_soa as libc::c_int as libc::c_uchar,
      name: [83, 79, 65, 0, 0, 0, 0],
    };
    init
  },
  {
    let mut init = C2RustUnnamed_1 {
      type_0: ns_t_ns as libc::c_int as libc::c_uchar,
      name: [78, 83, 0, 0, 0, 0, 0],
    };
    init
  },
  {
    let mut init = C2RustUnnamed_1 {
      type_0: ns_t_a as libc::c_int as libc::c_uchar,
      name: [65, 0, 0, 0, 0, 0, 0],
    };
    init
  },
  {
    let mut init = C2RustUnnamed_1 {
      type_0: ns_t_aaaa as libc::c_int as libc::c_uchar,
      name: [65, 65, 65, 65, 0, 0, 0],
    };
    init
  },
  {
    let mut init = C2RustUnnamed_1 {
      type_0: ns_t_cname as libc::c_int as libc::c_uchar,
      name: [67, 78, 65, 77, 69, 0, 0],
    };
    init
  },
  {
    let mut init = C2RustUnnamed_1 {
      type_0: ns_t_mx as libc::c_int as libc::c_uchar,
      name: [77, 88, 0, 0, 0, 0, 0],
    };
    init
  },
  {
    let mut init = C2RustUnnamed_1 {
      type_0: ns_t_txt as libc::c_int as libc::c_uchar,
      name: [84, 88, 84, 0, 0, 0, 0],
    };
    init
  },
  {
    let mut init = C2RustUnnamed_1 {
      type_0: ns_t_srv as libc::c_int as libc::c_uchar,
      name: [83, 82, 86, 0, 0, 0, 0],
    };
    init
  },
  {
    let mut init = C2RustUnnamed_1 {
      type_0: ns_t_ptr as libc::c_int as libc::c_uchar,
      name: [80, 84, 82, 0, 0, 0, 0],
    };
    init
  },
  {
    let mut init = C2RustUnnamed_1 {
      type_0: ns_t_any as libc::c_int as libc::c_uchar,
      name: [65, 78, 89, 0, 0, 0, 0],
    };
    init
  },
];
static mut rcodes: [*const libc::c_char; 16] = [
  b"NOERROR\x00" as *const u8 as *const libc::c_char,
  b"FORMERR\x00" as *const u8 as *const libc::c_char,
  b"SERVFAIL\x00" as *const u8 as *const libc::c_char,
  b"NXDOMAIN\x00" as *const u8 as *const libc::c_char,
  b"NOTIMP\x00" as *const u8 as *const libc::c_char,
  b"REFUSED\x00" as *const u8 as *const libc::c_char,
  b"YXDOMAIN\x00" as *const u8 as *const libc::c_char,
  b"YXRRSET\x00" as *const u8 as *const libc::c_char,
  b"NXRRSET\x00" as *const u8 as *const libc::c_char,
  b"NOTAUTH\x00" as *const u8 as *const libc::c_char,
  b"NOTZONE\x00" as *const u8 as *const libc::c_char,
  b"11\x00" as *const u8 as *const libc::c_char,
  b"12\x00" as *const u8 as *const libc::c_char,
  b"13\x00" as *const u8 as *const libc::c_char,
  b"14\x00" as *const u8 as *const libc::c_char,
  b"15\x00" as *const u8 as *const libc::c_char,
];
static mut v4_mapped: [libc::c_char; 12] = [
  0i32 as libc::c_char,
  0i32 as libc::c_char,
  0i32 as libc::c_char,
  0i32 as libc::c_char,
  0i32 as libc::c_char,
  0i32 as libc::c_char,
  0i32 as libc::c_char,
  0i32 as libc::c_char,
  0i32 as libc::c_char,
  0i32 as libc::c_char,
  0xffi32 as libc::c_char,
  0xffi32 as libc::c_char,
];
unsafe extern "C" fn parse_reply(mut msg: *const libc::c_uchar, mut len: size_t) -> libc::c_int {
  let mut header: *mut HEADER = 0 as *mut HEADER;
  let mut handle: ns_msg = ns_msg {
    _msg: 0 as *const libc::c_uchar,
    _eom: 0 as *const libc::c_uchar,
    _id: 0,
    _flags: 0,
    _counts: [0; 4],
    _sections: [0 as *const libc::c_uchar; 4],
    _sect: ns_s_qd,
    _rrnum: 0,
    _msg_ptr: 0 as *const libc::c_uchar,
  };
  let mut rr: ns_rr = ns_rr {
    name: [0; 1025],
    type_0: 0,
    rr_class: 0,
    ttl: 0,
    rdlength: 0,
    rdata: 0 as *const libc::c_uchar,
  };
  let mut i: libc::c_int = 0;
  let mut n: libc::c_int = 0;
  let mut rdlen: libc::c_int = 0;
  let mut format: *const libc::c_char = 0 as *const libc::c_char;
  let mut astr: [libc::c_char; 46] = [0; 46];
  let mut dname: [libc::c_char; 1025] = [0; 1025];
  let mut cp: *const libc::c_uchar = 0 as *const libc::c_uchar;
  header = msg as *mut HEADER;
  if (*header).aa() == 0 {
    printf(b"Non-authoritative answer:\n\x00" as *const u8 as *const libc::c_char);
  }
  if ns_initparse(msg, len as libc::c_int, &mut handle) != 0i32 {
    //printf("Unable to parse reply: %s\n", strerror(errno));
    return -1i32;
  }
  i = 0i32;
  while i < handle._counts[ns_s_an as libc::c_int as usize] as libc::c_int + 0i32 {
    if ns_parserr(&mut handle, ns_s_an, i, &mut rr) != 0i32 {
      //printf("Unable to parse resource record: %s\n", strerror(errno));
      return -1i32;
    }
    rdlen = rr.rdlength as libc::c_int + 0i32;
    let mut current_block_86: u64;
    match (rr.type_0 as libc::c_int + 0i32) as ns_type as libc::c_uint {
      1 => {
        if rdlen != 4i32 {
          return -1i32;
        }
        inet_ntop(
          2i32,
          rr.rdata.offset(0) as *const libc::c_void,
          astr.as_mut_ptr(),
          ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong as socklen_t,
        );
        printf(
          b"Name:\t%s\nAddress: %s\n\x00" as *const u8 as *const libc::c_char,
          if rr.name[0] as libc::c_int != '\u{0}' as i32 {
            rr.name.as_mut_ptr()
          } else {
            b".\x00" as *const u8 as *const libc::c_char
          },
          astr.as_mut_ptr(),
        );
        current_block_86 = 14294131666767243020;
      }
      28 => {
        if rdlen != 16i32 {
          return -1i32;
        }
        inet_ntop(
          10i32,
          rr.rdata.offset(0) as *const libc::c_void,
          astr.as_mut_ptr(),
          ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong as socklen_t,
        );
        /* bind-utils-9.11.3 uses the same format for A and AAAA answers */
        printf(
          b"Name:\t%s\nAddress: %s\n\x00" as *const u8 as *const libc::c_char,
          if rr.name[0] as libc::c_int != '\u{0}' as i32 {
            rr.name.as_mut_ptr()
          } else {
            b".\x00" as *const u8 as *const libc::c_char
          },
          astr.as_mut_ptr(),
        );
        current_block_86 = 14294131666767243020;
      }
      2 => {
        if format.is_null() {
          format = b"%s\tnameserver = %s\n\x00" as *const u8 as *const libc::c_char
        }
        current_block_86 = 6791211198087082855;
      }
      5 => {
        current_block_86 = 6791211198087082855;
      }
      12 => {
        current_block_86 = 12559006430393540496;
      }
      15 => {
        if rdlen < 2i32 {
          printf(b"MX record too short\n\x00" as *const u8 as *const libc::c_char);
          return -1i32;
        }
        n = ns_get16(rr.rdata.offset(0)) as libc::c_int;
        if ns_name_uncompress(
          handle._msg.offset(0),
          handle._eom.offset(0),
          rr.rdata.offset(0).offset(2),
          dname.as_mut_ptr(),
          ::std::mem::size_of::<[libc::c_char; 1025]>() as libc::c_ulong,
        ) < 0i32
        {
          //printf("Cannot uncompress MX domain: %s\n", strerror(errno));
          return -1i32;
        }
        printf(
          b"%s\tmail exchanger = %d %s\n\x00" as *const u8 as *const libc::c_char,
          if rr.name[0] as libc::c_int != '\u{0}' as i32 {
            rr.name.as_mut_ptr()
          } else {
            b".\x00" as *const u8 as *const libc::c_char
          },
          n,
          dname.as_mut_ptr(),
        );
        current_block_86 = 14294131666767243020;
      }
      16 => {
        if rdlen < 1i32 {
          //printf("TXT record too short\n");
          return -1i32;
        }
        n = *(rr.rdata.offset(0) as *mut libc::c_uchar) as libc::c_int;
        if n > 0i32 {
          memset(
            dname.as_mut_ptr() as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<[libc::c_char; 1025]>() as libc::c_ulong,
          );
          memcpy(
            dname.as_mut_ptr() as *mut libc::c_void,
            rr.rdata.offset(0).offset(1) as *const libc::c_void,
            n as libc::c_ulong,
          );
          printf(
            b"%s\ttext = \"%s\"\n\x00" as *const u8 as *const libc::c_char,
            if rr.name[0] as libc::c_int != '\u{0}' as i32 {
              rr.name.as_mut_ptr()
            } else {
              b".\x00" as *const u8 as *const libc::c_char
            },
            dname.as_mut_ptr(),
          );
        }
        current_block_86 = 14294131666767243020;
      }
      33 => {
        if rdlen < 6i32 {
          //printf("SRV record too short\n");
          return -1i32;
        }
        cp = rr.rdata.offset(0);
        n = ns_name_uncompress(
          handle._msg.offset(0),
          handle._eom.offset(0),
          cp.offset(6),
          dname.as_mut_ptr(),
          ::std::mem::size_of::<[libc::c_char; 1025]>() as libc::c_ulong,
        );
        if n < 0i32 {
          //printf("Unable to uncompress domain: %s\n", strerror(errno));
          return -1i32;
        }
        printf(
          b"%s\tservice = %u %u %u %s\n\x00" as *const u8 as *const libc::c_char,
          if rr.name[0] as libc::c_int != '\u{0}' as i32 {
            rr.name.as_mut_ptr()
          } else {
            b".\x00" as *const u8 as *const libc::c_char
          },
          ns_get16(cp),
          ns_get16(cp.offset(2)),
          ns_get16(cp.offset(4)),
          dname.as_mut_ptr(),
        );
        current_block_86 = 14294131666767243020;
      }
      6 => {
        if rdlen < 20i32 {
          return -1i32;
        }
        printf(
          b"%s\n\x00" as *const u8 as *const libc::c_char,
          if rr.name[0] as libc::c_int != '\u{0}' as i32 {
            rr.name.as_mut_ptr()
          } else {
            b".\x00" as *const u8 as *const libc::c_char
          },
        );
        cp = rr.rdata.offset(0);
        n = ns_name_uncompress(
          handle._msg.offset(0),
          handle._eom.offset(0),
          cp,
          dname.as_mut_ptr(),
          ::std::mem::size_of::<[libc::c_char; 1025]>() as libc::c_ulong,
        );
        if n < 0i32 {
          //printf("Unable to uncompress domain: %s\n", strerror(errno));
          return -1i32;
        }
        printf(
          b"\torigin = %s\n\x00" as *const u8 as *const libc::c_char,
          dname.as_mut_ptr(),
        );
        cp = cp.offset(n as isize);
        n = ns_name_uncompress(
          handle._msg.offset(0),
          handle._eom.offset(0),
          cp,
          dname.as_mut_ptr(),
          ::std::mem::size_of::<[libc::c_char; 1025]>() as libc::c_ulong,
        );
        if n < 0i32 {
          //printf("Unable to uncompress domain: %s\n", strerror(errno));
          return -1i32;
        }
        printf(
          b"\tmail addr = %s\n\x00" as *const u8 as *const libc::c_char,
          dname.as_mut_ptr(),
        );
        cp = cp.offset(n as isize);
        printf(
          b"\tserial = %lu\n\x00" as *const u8 as *const libc::c_char,
          ns_get32(cp),
        );
        cp = cp.offset(4);
        printf(
          b"\trefresh = %lu\n\x00" as *const u8 as *const libc::c_char,
          ns_get32(cp),
        );
        cp = cp.offset(4);
        printf(
          b"\tretry = %lu\n\x00" as *const u8 as *const libc::c_char,
          ns_get32(cp),
        );
        cp = cp.offset(4);
        printf(
          b"\texpire = %lu\n\x00" as *const u8 as *const libc::c_char,
          ns_get32(cp),
        );
        cp = cp.offset(4);
        printf(
          b"\tminimum = %lu\n\x00" as *const u8 as *const libc::c_char,
          ns_get32(cp),
        );
        current_block_86 = 14294131666767243020;
      }
      _ => {
        current_block_86 = 14294131666767243020;
      }
    }
    match current_block_86 {
      6791211198087082855 =>
      /* fall through */
      {
        if format.is_null() {
          format = b"%s\tcanonical name = %s\n\x00" as *const u8 as *const libc::c_char
        }
        current_block_86 = 12559006430393540496;
      }
      _ => {}
    }
    match current_block_86 {
      12559006430393540496 =>
      /* fall through */
      {
        if format.is_null() {
          format = b"%s\tname = %s\n\x00" as *const u8 as *const libc::c_char
        }
        if ns_name_uncompress(
          handle._msg.offset(0),
          handle._eom.offset(0),
          rr.rdata.offset(0),
          dname.as_mut_ptr(),
          ::std::mem::size_of::<[libc::c_char; 1025]>() as libc::c_ulong,
        ) < 0i32
        {
          //printf("Unable to uncompress domain: %s\n", strerror(errno));
          return -1i32;
        }
        printf(
          format,
          if rr.name[0] as libc::c_int != '\u{0}' as i32 {
            rr.name.as_mut_ptr()
          } else {
            b".\x00" as *const u8 as *const libc::c_char
          },
          dname.as_mut_ptr(),
        );
      }
      _ => {}
    }
    i += 1
  }
  return i;
}
/*
 * Function logic borrowed & modified from musl libc, res_msend.c
 * G.query_count is always > 0.
 */
unsafe extern "C" fn send_queries(mut ns: *mut ns) -> libc::c_int {
  let mut qn: libc::c_int = 0;
  let mut recvlen: libc::c_int = 0;
  let mut current_block: u64;
  let mut reply: [libc::c_uchar; 512] = [0; 512];
  let mut rcode: u8 = 0;
  let mut local_lsa: *mut len_and_sockaddr = 0 as *mut len_and_sockaddr;
  let mut pfd: pollfd = pollfd {
    fd: 0,
    events: 0,
    revents: 0,
  };
  let mut servfail_retry: libc::c_int = 0i32;
  let mut n_replies: libc::c_int = 0i32;
  //	int save_idx = 0;
  let mut retry_interval: libc::c_uint = 0;
  let mut timeout: libc::c_uint = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .default_timeout
    .wrapping_mul(1000i32 as libc::c_uint);
  let mut tstart: libc::c_uint = 0;
  let mut tsent: libc::c_uint = 0;
  let mut tcur: libc::c_uint = 0;
  pfd.events = 0x1i32 as libc::c_short;
  pfd.fd = xsocket_type(
    &mut local_lsa,
    (*(*ns).lsa).u.sa.sa_family as libc::c_int,
    SOCK_DGRAM as libc::c_int,
  );
  /*
   * local_lsa has "null" address and port 0 now.
   * bind() ensures we have a *particular port* selected by kernel
   * and remembered in fd, thus later recv(fd)
   * receives only packets sent to this port.
   */
  xbind(pfd.fd, &mut (*local_lsa).u.sa, (*local_lsa).len);
  free(local_lsa as *mut libc::c_void);
  /* Make read/writes know the destination */
  xconnect(pfd.fd, &mut (*(*ns).lsa).u.sa, (*(*ns).lsa).len); /* this one was replied already */
  ndelay_on(pfd.fd); /* "no go, try next server" */
  retry_interval =
    timeout.wrapping_div((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).default_retry);
  tcur = monotonic_ms() as libc::c_uint;
  tstart = tcur;
  's_78: loop {
    qn = 0i32;
    while (qn as libc::c_uint) < (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).query_count {
      if !((*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .query
        .offset(qn as isize))
      .qlen
        == 0i32 as libc::c_uint)
      {
        if write(
          pfd.fd,
          (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .query
            .offset(qn as isize))
          .query
          .as_mut_ptr() as *const libc::c_void,
          (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .query
            .offset(qn as isize))
          .qlen as size_t,
        ) < 0         {
          bb_perror_msg(
            b"write to \'%s\'\x00" as *const u8 as *const libc::c_char,
            (*ns).name,
          );
          n_replies = -1i32;
          break 's_78;
        }
      }
      qn += 1
    }
    tsent = tcur;
    servfail_retry = (2i32 as libc::c_uint)
      .wrapping_mul((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).query_count)
      as libc::c_int;
    loop
    /* Wait for a response, or until time to retry */
    {
      if poll(
        &mut pfd,
        1i32 as nfds_t,
        retry_interval.wrapping_sub(tcur.wrapping_sub(tsent)) as libc::c_int,
      ) <= 0i32
      {
        current_block = 6236198777448170658;
      } else {
        recvlen = read(
          pfd.fd,
          reply.as_mut_ptr() as *mut libc::c_void,
          ::std::mem::size_of::<[libc::c_uchar; 512]>() as libc::c_ulong,
        ) as libc::c_int;
        if recvlen < 0i32 {
          bb_simple_perror_msg(b"read\x00" as *const u8 as *const libc::c_char);
          current_block = 6236198777448170658;
        } else {
          let fresh0 = (*ns).replies;
          (*ns).replies = (*ns).replies + 1;
          if fresh0 == 0i32 {
            printf(
              b"Server:\t\t%s\n\x00" as *const u8 as *const libc::c_char,
              (*ns).name,
            );
            printf(
              b"Address:\t%s\n\n\x00" as *const u8 as *const libc::c_char,
              auto_string(xmalloc_sockaddr2dotted(&mut (*(*ns).lsa).u.sa)),
            );
            /* In "Address", bind-utils-9.11.3 show port after a hash: "1.2.3.4#53" */
            /* Should we do the same? */
          }
          /* Non-identifiable packet */
          if recvlen < 4i32 {
            current_block = 6236198777448170658;
          } else {
            /* Find which query this answer goes with, if any */
            //		qn = save_idx;
            qn = 0i32;
            loop {
              if memcmp(
                reply.as_mut_ptr() as *const libc::c_void,
                (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                  .query
                  .offset(qn as isize))
                .query
                .as_mut_ptr() as *const libc::c_void,
                2i32 as libc::c_ulong,
              ) == 0i32
              {
                current_block = 2122094917359643297;
                break;
              }
              qn += 1;
              if qn as libc::c_uint
                >= (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).query_count
              {
                current_block = 6236198777448170658;
                break;
              }
            }
            match current_block {
              6236198777448170658 => {}
              _ => {
                if (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                  .query
                  .offset(qn as isize))
                .qlen
                  == 0i32 as libc::c_uint
                {
                  current_block = 6236198777448170658;
                } else {
                  rcode = (reply[3] as libc::c_int & 0xfi32) as u8;
                  /* Retry immediately on SERVFAIL */
                  if rcode as libc::c_int == 2i32 {
                    //UNUSED: ns->failures++;
                    if servfail_retry != 0 {
                      servfail_retry -= 1;
                      write(
                        pfd.fd,
                        (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                          .query
                          .offset(qn as isize))
                        .query
                        .as_mut_ptr() as *const libc::c_void,
                        (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                          .query
                          .offset(qn as isize))
                        .qlen as size_t,
                      );
                      current_block = 6236198777448170658;
                    } else {
                      current_block = 8347882395825654554;
                    }
                  } else {
                    current_block = 8347882395825654554;
                  }
                  match current_block {
                    6236198777448170658 => {}
                    _ => {
                      /* Process reply */
                      (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                        .query
                        .offset(qn as isize))
                      .qlen = 0i32 as libc::c_uint; /* flag: "reply received" */
                      tcur = monotonic_ms() as libc::c_uint; /* while() */
                      if option_mask32 & OPT_debug as libc::c_int as libc::c_uint != 0 {
                        printf(
                          b"Query #%d completed in %ums:\n\x00" as *const u8 as *const libc::c_char,
                          qn,
                          tcur.wrapping_sub(tstart),
                        );
                      }
                      if rcode as libc::c_int != 0i32 {
                        printf(
                          b"** server can\'t find %s: %s\n\x00" as *const u8 as *const libc::c_char,
                          (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                            .query
                            .offset(qn as isize))
                          .name,
                          rcodes[rcode as usize],
                        );
                        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).exitcode =
                          1i32 as smalluint
                      } else {
                        match parse_reply(reply.as_mut_ptr(), recvlen as size_t) {
                          -1 => {
                            printf(
                              b"*** Can\'t find %s: Parse error\n\x00" as *const u8
                                as *const libc::c_char,
                              (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                                .query
                                .offset(qn as isize))
                              .name,
                            );
                            (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).exitcode =
                              1i32 as smalluint
                          }
                          0 => {
                            printf(
                              b"*** Can\'t find %s: No answer\n\x00" as *const u8
                                as *const libc::c_char,
                              (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                                .query
                                .offset(qn as isize))
                              .name,
                            );
                          }
                          _ => {}
                        }
                      }
                      bb_putchar('\n' as i32);
                      n_replies += 1;
                      if n_replies as libc::c_uint
                        >= (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).query_count
                      {
                        break 's_78;
                      }
                      current_block = 1841672684692190573;
                    }
                  }
                }
              }
            }
          }
        }
      }
      match current_block {
        6236198777448170658 => tcur = monotonic_ms() as libc::c_uint,
        _ => {}
      }
      if !(tcur.wrapping_sub(tstart) < timeout) {
        break 's_78;
      }
      qn = 0;
      recvlen = 0;
      if tcur.wrapping_sub(tsent) >= retry_interval {
        break;
      }
    }
  }
  close(pfd.fd);
  return n_replies;
}
unsafe extern "C" fn add_ns(mut addr: *const libc::c_char) {
  let mut ns: *mut ns = 0 as *mut ns;
  let mut count: libc::c_uint = 0;
  let ref mut fresh1 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).serv_count;
  let fresh2 = *fresh1;
  *fresh1 = (*fresh1).wrapping_add(1);
  count = fresh2;
  let ref mut fresh3 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).server;
  *fresh3 = xrealloc_vector_helper(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).server as *mut libc::c_void,
    ((::std::mem::size_of::<ns>() as libc::c_ulong) << 8i32).wrapping_add(3i32 as libc::c_ulong)
      as libc::c_uint,
    count as libc::c_int,
  ) as *mut ns;
  ns = &mut *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .server
    .offset(count as isize) as *mut ns;
  (*ns).name = addr;
  (*ns).lsa = xhost2sockaddr(
    addr,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).default_port as libc::c_int,
  );
  /*ns->replies = 0; - already is */
  /*ns->failures = 0; - already is */
}
unsafe extern "C" fn parse_resolvconf() {
  let mut resolv: *mut FILE = 0 as *mut FILE; /* "search" is defined to be up to 256 chars */
  resolv = fopen(
    b"/etc/resolv.conf\x00" as *const u8 as *const libc::c_char,
    b"r\x00" as *const u8 as *const libc::c_char,
  );
  if !resolv.is_null() {
    let mut line: [libc::c_char; 512] = [0; 512];
    while !fgets_unlocked(
      line.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as libc::c_int,
      resolv,
    )
    .is_null()
    {
      let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
      let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
      p = strtok(
        line.as_mut_ptr(),
        b" \t\n\x00" as *const u8 as *const libc::c_char,
      );
      if p.is_null() {
        continue;
      }
      arg = strtok(
        0 as *mut libc::c_char,
        b"\n\x00" as *const u8 as *const libc::c_char,
      );
      if arg.is_null() {
        continue;
      }
      if strcmp(p, b"domain\x00" as *const u8 as *const libc::c_char) == 0i32 {
        /* domain DOM */
        if !((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).have_search_directive == 0) {
          continue;
        }
      } else if strcmp(p, b"search\x00" as *const u8 as *const libc::c_char) == 0i32 {
        /* search DOM1 DOM2... */
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).have_search_directive =
          1i32 as smalluint
      } else {
        if strcmp(p, b"nameserver\x00" as *const u8 as *const libc::c_char) != 0i32 {
          continue;
        }
        /* nameserver DNS */
        add_ns(xstrdup(arg));
        continue;
      }
      free((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).search as *mut libc::c_void);
      let ref mut fresh4 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).search;
      *fresh4 = xstrdup(arg)
    }
    fclose(resolv);
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .search
    .is_null()
  {
    /* default search domain is domain part of hostname */
    let mut h: *mut libc::c_char = safe_gethostname();
    let mut d: *mut libc::c_char = strchr(h, '.' as i32);
    if !d.is_null() {
      let ref mut fresh5 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).search;
      *fresh5 = d.offset(1)
    }
  }
  /* Cater for case of "domain ." in resolv.conf */
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .search
    .is_null()
    && (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .search
      .offset(0) as libc::c_int
      == '.' as i32
      && *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .search
        .offset(1)
        == 0)
  {
    let ref mut fresh6 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).search;
    *fresh6 = 0 as *mut libc::c_char
  };
}
unsafe extern "C" fn add_query(mut type_0: libc::c_int, mut dname: *const libc::c_char) {
  let mut new_q: *mut query = 0 as *mut query;
  let mut count: libc::c_uint = 0;
  let mut qlen: ssize_t = 0;
  let ref mut fresh7 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).query_count;
  let fresh8 = *fresh7;
  *fresh7 = (*fresh7).wrapping_add(1);
  count = fresh8;
  let ref mut fresh9 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).query;
  *fresh9 = xrealloc_vector_helper(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).query as *mut libc::c_void,
    ((::std::mem::size_of::<query>() as libc::c_ulong) << 8i32).wrapping_add(2i32 as libc::c_ulong)
      as libc::c_uint,
    count as libc::c_int,
  ) as *mut query;
  new_q = &mut *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .query
    .offset(count as isize) as *mut query;
  (*new_q).name = dname;
  qlen = __res_mkquery(
    ns_o_query as libc::c_int,
    dname,
    ns_c_in as libc::c_int,
    type_0,
    0 as *const libc::c_uchar,
    0i32,
    0 as *const libc::c_uchar,
    (*new_q).query.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_uchar; 512]>() as libc::c_ulong as libc::c_int,
  ) as ssize_t;
  (*new_q).qlen = qlen as libc::c_uint;
}
unsafe extern "C" fn add_query_with_search(
  mut type_0: libc::c_int,
  mut dname: *const libc::c_char,
) {
  let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
  if type_0 == ns_t_ptr as libc::c_int
    || (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .search
      .is_null()
    || !strchr(dname, '.' as i32).is_null()
  {
    add_query(type_0, dname);
    return;
  }
  s = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).search;
  loop {
    let mut fullname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    e = skip_non_whitespace(s);
    fullname = xasprintf(
      b"%s.%.*s\x00" as *const u8 as *const libc::c_char,
      dname,
      e.wrapping_offset_from(s) as libc::c_long as libc::c_int,
      s,
    );
    add_query(type_0, fullname);
    s = skip_whitespace(e);
    if *s == 0 {
      break;
    }
  }
}
unsafe extern "C" fn make_ptr(mut addrstr: *const libc::c_char) -> *mut libc::c_char {
  let mut addr: [libc::c_uchar; 16] = [0; 16];
  if inet_pton(10i32, addrstr, addr.as_mut_ptr() as *mut libc::c_void) != 0 {
    if memcmp(
      addr.as_mut_ptr() as *const libc::c_void,
      v4_mapped.as_ptr() as *const libc::c_void,
      12i32 as libc::c_ulong,
    ) != 0i32
    {
      let mut i: libc::c_int = 0;
      let mut resbuf: [libc::c_char; 80] = [0; 80];
      let mut ptr: *mut libc::c_char = resbuf.as_mut_ptr();
      i = 0i32;
      while i < 16i32 {
        let fresh10 = ptr;
        ptr = ptr.offset(1);
        *fresh10 = (0x20i32
          | *bb_hexdigits_upcase
            .as_ptr()
            .offset((addr[(15i32 - i) as usize] as libc::c_int & 0xfi32) as isize)
            as libc::c_int) as libc::c_char;
        let fresh11 = ptr;
        ptr = ptr.offset(1);
        *fresh11 = '.' as i32 as libc::c_char;
        let fresh12 = ptr;
        ptr = ptr.offset(1);
        *fresh12 = (0x20i32
          | *bb_hexdigits_upcase
            .as_ptr()
            .offset((addr[(15i32 - i) as usize] as libc::c_int >> 4i32) as isize)
            as libc::c_int) as libc::c_char;
        let fresh13 = ptr;
        ptr = ptr.offset(1);
        *fresh13 = '.' as i32 as libc::c_char;
        i += 1
      }
      strcpy(ptr, b"ip6.arpa\x00" as *const u8 as *const libc::c_char);
      return xstrdup(resbuf.as_mut_ptr());
    }
    return xasprintf(
      b"%u.%u.%u.%u.in-addr.arpa\x00" as *const u8 as *const libc::c_char,
      addr[15] as libc::c_int,
      addr[14] as libc::c_int,
      addr[13] as libc::c_int,
      addr[12] as libc::c_int,
    );
  }
  if inet_pton(2i32, addrstr, addr.as_mut_ptr() as *mut libc::c_void) != 0 {
    return xasprintf(
      b"%u.%u.%u.%u.in-addr.arpa\x00" as *const u8 as *const libc::c_char,
      addr[3] as libc::c_int,
      addr[2] as libc::c_int,
      addr[1] as libc::c_int,
      addr[0] as libc::c_int,
    );
  }
  return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn nslookup_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut types: libc::c_uint = 0;
  let mut rc: libc::c_int = 0;
  let mut err: libc::c_int = 0;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).default_port = 53i32 as libc::c_uint;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).default_retry = 2i32 as libc::c_uint;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).default_timeout = 5i32 as libc::c_uint;
  /* manpage: "Options can also be specified on the command line
   * if they precede the arguments and are prefixed with a hyphen."
   */
  types = 0i32 as libc::c_uint;
  argv = argv.offset(1);
  loop {
    let mut options: *const libc::c_char =
      b"type\x00querytype\x00port\x00retry\x00debug\x00t\x00timeout\x00\x00" as *const u8
        as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*argv).is_null() {
      bb_show_usage();
    }
    if *(*argv.offset(0)).offset(0) as libc::c_int != '-' as i32 {
      break;
    }
    /* Separate out "=val" part */
    let fresh14 = argv;
    argv = argv.offset(1);
    arg = (*fresh14).offset(1);
    val = strchrnul(arg, '=' as i32);
    if *val != 0 {
      let fresh15 = val;
      val = val.offset(1);
      *fresh15 = '\u{0}' as i32 as libc::c_char
    }
    i = index_in_substrings(options, arg);
    //bb_error_msg("i:%d arg:'%s' val:'%s'", i, arg, val);
    if i < 0i32 {
      bb_show_usage();
    }
    if i <= 1i32 {
      i = 0i32;
      loop {
        if i as libc::c_uint
          == (::std::mem::size_of::<[C2RustUnnamed_1; 10]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong)
            as libc::c_uint
        {
          bb_error_msg_and_die(
            b"invalid query type \"%s\"\x00" as *const u8 as *const libc::c_char,
            val,
          );
        }
        if strcasecmp(qtypes[i as usize].name.as_ptr(), val) == 0i32 {
          break;
        }
        i += 1
      }
      types |= (1i32 << i) as libc::c_uint
    } else {
      if i == 2i32 {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).default_port =
          xatou_range(val, 1i32 as libc::c_uint, 0xffffi32 as libc::c_uint)
      }
      if i == 3i32 {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).default_retry =
          xatou_range(val, 1i32 as libc::c_uint, 2147483647i32 as libc::c_uint)
      }
      if i == 4i32 {
        option_mask32 |= OPT_debug as libc::c_int as libc::c_uint
      }
      if i > 4i32 {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).default_timeout = xatou_range(
          val,
          1i32 as libc::c_uint,
          (2147483647i32 / 1000i32) as libc::c_uint,
        )
      }
    }
  }
  /* Use given DNS server if present */
  if !(*argv.offset(1)).is_null() {
    if !(*argv.offset(2)).is_null() {
      bb_show_usage();
    }
    add_ns(*argv.offset(1));
  } else {
    parse_resolvconf();
    /* Fall back to localhost if we could not find NS in resolv.conf */
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).serv_count == 0i32 as libc::c_uint {
      add_ns(b"127.0.0.1\x00" as *const u8 as *const libc::c_char);
    }
  }
  if types == 0i32 as libc::c_uint {
    /* No explicit type given, guess query type.
     * If we can convert the domain argument into a ptr (means that
     * inet_pton() could read it) we assume a PTR request, else
     * we issue A+AAAA queries and switch to an output format
     * mimicking the one of the traditional nslookup applet.
     */
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    ptr = make_ptr(*argv.offset(0));
    if !ptr.is_null() {
      add_query(ns_t_ptr as libc::c_int, ptr);
    } else {
      add_query_with_search(ns_t_a as libc::c_int, *argv.offset(0));
      add_query_with_search(ns_t_aaaa as libc::c_int, *argv.offset(0));
    }
  } else {
    let mut c: libc::c_int = 0;
    c = 0i32;
    while (c as libc::c_uint)
      < (::std::mem::size_of::<[C2RustUnnamed_1; 10]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong)
        as libc::c_uint
    {
      if types & (1i32 << c) as libc::c_uint != 0 {
        add_query_with_search(qtypes[c as usize].type_0 as libc::c_int, *argv.offset(0));
      }
      c += 1
    }
  }
  rc = 0i32;
  while (rc as libc::c_uint) < (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).serv_count {
    let mut c_0: libc::c_int = 0;
    c_0 = send_queries(
      &mut *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .server
        .offset(rc as isize),
    );
    if c_0 > 0i32 {
      break;
    //FIXME: we "break" even though some queries may still be not answered, and other servers may know them?
    } else {
      /* c = 0: timed out waiting for replies */
      /* c < 0: error (message already printed) */
      rc += 1;
      if rc as libc::c_uint >= (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).serv_count {
        //
        // NB: bind-utils-9.11.3 behavior (all to stdout, not stderr):
        //
        // $ nslookup gmail.com 8.8.8.8
        // ;; connection timed out; no servers could be reached
        //
        // Using TCP mode:
        // $ nslookup -vc gmail.com 8.8.8.8; echo EXITCODE:$?
        //     <~10 sec>
        // ;; Connection to 8.8.8.8#53(8.8.8.8) for gmail.com failed: timed out.
        //     <~10 sec>
        // ;; Connection to 8.8.8.8#53(8.8.8.8) for gmail.com failed: timed out.
        //     <~10 sec>
        // ;; connection timed out; no servers could be reached
        // ;; Connection to 8.8.8.8#53(8.8.8.8) for gmail.com failed: timed out.
        //     <empty line>
        // EXITCODE:1
        // $ _
        printf(
          b";; connection timed out; no servers could be reached\n\n\x00" as *const u8
            as *const libc::c_char,
        );
        return 1i32;
      }
    }
  }
  err = 0i32;
  rc = 0i32;
  while (rc as libc::c_uint) < (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).query_count {
    if (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .query
      .offset(rc as isize))
    .qlen
      != 0
    {
      printf(
        b"*** Can\'t find %s: No answer\n\x00" as *const u8 as *const libc::c_char,
        (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .query
          .offset(rc as isize))
        .name,
      );
      err = 1i32
    }
    rc += 1
  }
  if err != 0 {
    /* should this affect exicode too? */
    bb_putchar('\n' as i32);
  }
  return (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).exitcode as libc::c_int;
}

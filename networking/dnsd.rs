use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;

use crate::libbb::appletlib::applet_name;
use libc;
use libc::free;
use libc::openlog;
use libc::sprintf;
use libc::strcasecmp;
use libc::strcpy;
extern "C" {

  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  fn inet_aton(__cp: *const libc::c_char, __inp: *mut in_addr) -> libc::c_int;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xsocket(domain: libc::c_int, type_0: libc::c_int, protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xbind(sockfd: libc::c_int, my_addr: *mut sockaddr, addrlen: socklen_t);
  #[no_mangle]
  fn xdotted2sockaddr(host: *const libc::c_char, port: libc::c_int) -> *mut len_and_sockaddr;
  #[no_mangle]
  fn xmalloc_sockaddr2dotted(sa: *const sockaddr) -> *mut libc::c_char;
  #[no_mangle]
  fn socket_want_pktinfo(fd: libc::c_int);
  #[no_mangle]
  fn send_to_from(
    fd: libc::c_int,
    buf: *mut libc::c_void,
    len: size_t,
    flags: libc::c_int,
    to: *const sockaddr,
    from: *const sockaddr,
    tolen: socklen_t,
  ) -> ssize_t;
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
  fn xatou_range(str: *const libc::c_char, l: libc::c_uint, u: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn bb_daemonize_or_rexec(flags: libc::c_int);
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  static mut logmode: smallint;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_info_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_info_msg(s: *const libc::c_char);
  #[no_mangle]
  fn config_open(filename: *const libc::c_char) -> *mut parser_t;
  #[no_mangle]
  fn config_read(
    parser: *mut parser_t,
    tokens: *mut *mut libc::c_char,
    flags: libc::c_uint,
    delims: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  fn config_close(parser: *mut parser_t);


}

pub type __socklen_t = libc::c_uint;

pub type bb__aliased_u16 = u16;
pub type bb__aliased_u32 = u32;
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

use libc::FILE;
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const LSA_SIZEOF_SA: C2RustUnnamed_1 = 28;
pub const LSA_LEN_SIZE: C2RustUnnamed_1 = 4;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const DAEMON_ONLY_SANITIZE: C2RustUnnamed_2 = 8;
pub const DAEMON_CLOSE_EXTRA_FDS: C2RustUnnamed_2 = 4;
pub const DAEMON_DEVNULL_STDIO: C2RustUnnamed_2 = 2;
pub const DAEMON_CHDIR_ROOT: C2RustUnnamed_2 = 1;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const LOGMODE_BOTH: C2RustUnnamed_3 = 3;
pub const LOGMODE_SYSLOG: C2RustUnnamed_3 = 2;
pub const LOGMODE_STDIO: C2RustUnnamed_3 = 1;
pub const LOGMODE_NONE: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const PARSE_NORMAL: C2RustUnnamed_4 = 4653056;
pub const PARSE_WS_COMMENTS: C2RustUnnamed_4 = 16777216;
pub const PARSE_ALT_COMMENTS: C2RustUnnamed_4 = 8388608;
pub const PARSE_EOL_COMMENTS: C2RustUnnamed_4 = 4194304;
pub const PARSE_KEEP_COPY: C2RustUnnamed_4 = 2097152;
pub const PARSE_MIN_DIE: C2RustUnnamed_4 = 1048576;
pub const PARSE_GREEDY: C2RustUnnamed_4 = 262144;
pub const PARSE_TRIM: C2RustUnnamed_4 = 131072;
pub const PARSE_COLLAPSE: C2RustUnnamed_4 = 65536;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser_t {
  pub fp: *mut FILE,
  pub data: *mut libc::c_char,
  pub line: *mut libc::c_char,
  pub nline: *mut libc::c_char,
  pub line_alloc: size_t,
  pub nline_alloc: size_t,
  pub lineno: libc::c_int,
}

/*
 * Mini DNS server implementation for busybox
 *
 * Copyright (C) 2005 Roberto A. Foglietta (me@roberto.foglietta.name)
 * Copyright (C) 2005 Odd Arild Olsen (oao at fibula dot no)
 * Copyright (C) 2003 Paul Sheer
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 *
 * Odd Arild Olsen started out with the sheerdns [1] of Paul Sheer and rewrote
 * it into a shape which I believe is both easier to understand and maintain.
 * I also reused the input buffer for output and removed services he did not
 * need.  [1] http://threading.2038bug.com/sheerdns/
 *
 * Some bugfix and minor changes was applied by Roberto A. Foglietta who made
 * the first porting of oao' scdns to busybox also.
 */
//config:config DNSD
//config:	bool "dnsd (9.8 kb)"
//config:	default y
//config:	help
//config:	Small and static DNS server daemon.
//applet:IF_DNSD(APPLET(dnsd, BB_DIR_USR_SBIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_DNSD) += dnsd.o
//usage:#define dnsd_trivial_usage
//usage:       "[-dvs] [-c CONFFILE] [-t TTL_SEC] [-p PORT] [-i ADDR]"
//usage:#define dnsd_full_usage "\n\n"
//usage:       "Small static DNS server daemon\n"
//usage:     "\n	-c FILE	Config file"
//usage:     "\n	-t SEC	TTL"
//usage:     "\n	-p PORT	Listen on PORT"
//usage:     "\n	-i ADDR	Listen on ADDR"
//usage:     "\n	-d	Daemonize"
//usage:     "\n	-v	Verbose"
//usage:     "\n	-s	Send successful replies only. Use this if you want"
//usage:     "\n		to use /etc/resolv.conf with two nameserver lines:"
//usage:     "\n			nameserver DNSD_SERVER"
//usage:     "\n			nameserver NORMAL_DNS_SERVER"
//#define DEBUG 1
pub type C2RustUnnamed_5 = libc::c_uint;
pub const REQ_PTR: C2RustUnnamed_5 = 12;
pub const REQ_A: C2RustUnnamed_5 = 1;
pub const MAX_NAME_LEN: C2RustUnnamed_5 = 30;
pub const IP_STRING_LEN: C2RustUnnamed_5 = 17;
/* cannot get bigger packets than 512 per RFC1035. */
pub const MAX_PACK_LEN: C2RustUnnamed_5 = 512;
/* can tweak this */
pub const DEFAULT_TTL: C2RustUnnamed_5 = 120;
/* the message from client and first part of response msg */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_head {
  pub id: u16,
  pub flags: u16,
  pub nquer: u16,
  pub nansw: u16,
  pub nauth: u16,
  pub nadd: u16,
}
/* Structure used to access type and class fields.
 * They are totally unaligned, but gcc 4.3.4 thinks that pointer of type u16*
 * is 16-bit aligned and replaces 16-bit memcpy (in move_from_unaligned16 macro)
 * with aligned halfword access on arm920t!
 * Oh well. Slapping PACKED everywhere seems to help: */
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct type_and_class {
  pub type_0: u16,
  pub class: u16,
}
/* element of known name, ip address and reversed ip address */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_entry {
  pub next: *mut dns_entry,
  pub ip: u32,
  pub rip: [libc::c_char; 17],
  pub name: [libc::c_char; 1],
}
/*
 * Insert length of substrings instead of dots
 */
unsafe extern "C" fn undot(mut rip: *mut libc::c_char) {
  let mut i: libc::c_int = 0i32;
  let mut s: libc::c_int = 0i32;
  while *rip.offset(i as isize) != 0 {
    i += 1
  }
  i -= 1;
  while i >= 0i32 {
    if *rip.offset(i as isize) as libc::c_int == '.' as i32 {
      *rip.offset(i as isize) = s as libc::c_char;
      s = 0i32
    } else {
      s += 1
    }
    i -= 1
  }
}
/*
 * Read hostname/IP records from file
 */
unsafe extern "C" fn parse_conf_file(mut fileconf: *const libc::c_char) -> *mut dns_entry {
  let mut token: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
  let mut parser: *mut parser_t = 0 as *mut parser_t;
  let mut m: *mut dns_entry = 0 as *mut dns_entry;
  let mut conf_data: *mut dns_entry = 0 as *mut dns_entry;
  let mut nextp: *mut *mut dns_entry = 0 as *mut *mut dns_entry;
  conf_data = 0 as *mut dns_entry;
  nextp = &mut conf_data;
  parser = config_open(fileconf);
  while config_read(
    parser,
    token.as_mut_ptr(),
    (PARSE_NORMAL as libc::c_int | (2i32 & 0xffi32) << 8i32 | 2i32 & 0xffi32) as libc::c_uint,
    b"# \t\x00" as *const u8 as *const libc::c_char,
  ) != 0
  {
    let mut ip: in_addr = in_addr { s_addr: 0 };
    let mut v32: u32 = 0;
    if inet_aton(token[1], &mut ip) == 0i32 {
      bb_error_msg(
        b"error at line %u, skipping\x00" as *const u8 as *const libc::c_char,
        (*parser).lineno,
      );
    } else {
      if option_mask32 & 1i32 as libc::c_uint != 0 {
        bb_info_msg(
          b"name:%s, ip:%s\x00" as *const u8 as *const libc::c_char,
          token[0],
          token[1],
        );
      }
      /* sizeof(*m) includes 1 byte for m->name[0] */
      m = xzalloc(
        (::std::mem::size_of::<dns_entry>() as libc::c_ulong)
          .wrapping_add(strlen(token[0]))
          .wrapping_add(1i32 as libc::c_ulong),
      ) as *mut dns_entry;
      /*m->next = NULL;*/
      *nextp = m; /* in network order */
      nextp = &mut (*m).next;
      *(*m).name.as_mut_ptr().offset(0) = '.' as i32 as libc::c_char;
      strcpy((*m).name.as_mut_ptr().offset(1), token[0]);
      undot((*m).name.as_mut_ptr());
      (*m).ip = ip.s_addr;
      v32 = {
        let mut __v: libc::c_uint = 0;
        let mut __x: libc::c_uint = (*m).ip;
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
      /* inverted order */
      sprintf(
        (*m).rip.as_mut_ptr(),
        b".%u.%u.%u.%u\x00" as *const u8 as *const libc::c_char,
        v32 as u8 as libc::c_int,
        (v32 >> 8i32) as u8 as libc::c_int,
        (v32 >> 16i32) as u8 as libc::c_int,
        v32 >> 24i32,
      );
      undot((*m).rip.as_mut_ptr());
    }
  }
  config_close(parser);
  return conf_data;
}
/*
 * Look query up in dns records and return answer if found.
 */
unsafe extern "C" fn table_lookup(
  mut d: *mut dns_entry,
  mut type_0: u16,
  mut query_string: *mut libc::c_char,
) -> *mut libc::c_char {
  while !d.is_null() {
    let mut current_block_5: u64;
    let mut len: libc::c_uint = *(*d).name.as_mut_ptr().offset(0) as libc::c_uint;
    /* d->name[len] is the last (non NUL) char */
    if type_0 as libc::c_int
      == ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = REQ_A as libc::c_int as libc::c_ushort;
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
      }) as libc::c_int
    {
      /* search by host name */
      if len != 1i32 as libc::c_uint
        || *(*d).name.as_mut_ptr().offset(1) as libc::c_int != '*' as i32
      {
        /* we are lax, hope no name component is ever >64 so that length
         * (which will be represented as 'A','B'...) matches a lowercase letter.
         * Actually, I think false matches are hard to construct.
         * Example.
         * [31] len is represented as '1', [65] as 'A', [65+32] as 'a'.
         * [65]   <65 same chars>[31]<31 same chars>NUL
         * [65+32]<65 same chars>1   <31 same chars>NUL
         * This example seems to be the minimal case when false match occurs.
         */
        if strcasecmp((*d).name.as_mut_ptr(), query_string) != 0i32 {
          current_block_5 = 137226434401907431;
        } else {
          current_block_5 = 820271813250567934;
        }
      } else {
        current_block_5 = 820271813250567934;
      }
      match current_block_5 {
        137226434401907431 => {}
        _ => return &mut (*d).ip as *mut u32 as *mut libc::c_char,
      }
    } else if (len != 1i32 as libc::c_uint
      || *(*d).name.as_mut_ptr().offset(1) as libc::c_int != '*' as i32)
      && !is_prefixed_with(query_string, (*d).rip.as_mut_ptr()).is_null()
    {
      return (*d).name.as_mut_ptr();
    }
    d = (*d).next
  }
  return 0 as *mut libc::c_char;
}
/* search by IP-address */
/*
 * Decode message and generate answer
 */
/* RFC 1035
...
Whenever an octet represents a numeric quantity, the left most bit
in the diagram is the high order or most significant bit.
That is, the bit labeled 0 is the most significant bit.
...

4.1.1. Header section format
      0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                      ID                       |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |QR|   OPCODE  |AA|TC|RD|RA| 0  0  0|   RCODE   |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                    QDCOUNT                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                    ANCOUNT                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                    NSCOUNT                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                    ARCOUNT                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
ID      16 bit random identifier assigned by querying peer.
        Used to match query/response.
QR      message is a query (0), or a response (1).
OPCODE  0   standard query (QUERY)
        1   inverse query (IQUERY)
        2   server status request (STATUS)
AA      Authoritative Answer - this bit is valid in responses.
        Responding name server is an authority for the domain name
        in question section. Answer section may have multiple owner names
        because of aliases.  The AA bit corresponds to the name which matches
        the query name, or the first owner name in the answer section.
TC      TrunCation - this message was truncated.
RD      Recursion Desired - this bit may be set in a query and
        is copied into the response.  If RD is set, it directs
        the name server to pursue the query recursively.
        Recursive query support is optional.
RA      Recursion Available - this be is set or cleared in a
        response, and denotes whether recursive query support is
        available in the name server.
RCODE   Response code.
        0   No error condition
        1   Format error
        2   Server failure - server was unable to process the query
            due to a problem with the name server.
        3   Name Error - meaningful only for responses from
            an authoritative name server. The referenced domain name
            does not exist.
        4   Not Implemented.
        5   Refused.
QDCOUNT number of entries in the question section.
ANCOUNT number of records in the answer section.
NSCOUNT number of records in the authority records section.
ARCOUNT number of records in the additional records section.

4.1.2. Question section format

The section contains QDCOUNT (usually 1) entries, each of this format:
      0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /                     QNAME                     /
    /                                               /
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                     QTYPE                     |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                     QCLASS                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
QNAME   a domain name represented as a sequence of labels, where
        each label consists of a length octet followed by that
        number of octets. The domain name terminates with the
        zero length octet for the null label of the root. Note
        that this field may be an odd number of octets; no
        padding is used.
QTYPE   a two octet type of the query.
          1 a host address [REQ_A const]
          2 an authoritative name server
          3 a mail destination (Obsolete - use MX)
          4 a mail forwarder (Obsolete - use MX)
          5 the canonical name for an alias
          6 marks the start of a zone of authority
          7 a mailbox domain name (EXPERIMENTAL)
          8 a mail group member (EXPERIMENTAL)
          9 a mail rename domain name (EXPERIMENTAL)
         10 a null RR (EXPERIMENTAL)
         11 a well known service description
         12 a domain name pointer [REQ_PTR const]
         13 host information
         14 mailbox or mail list information
         15 mail exchange
         16 text strings
       0x1c IPv6?
        252 a request for a transfer of an entire zone
        253 a request for mailbox-related records (MB, MG or MR)
        254 a request for mail agent RRs (Obsolete - see MX)
        255 a request for all records
QCLASS  a two octet code that specifies the class of the query.
          1 the Internet
        (others are historic only)
        255 any class

4.1.3. Resource Record format

The answer, authority, and additional sections all share the same format:
a variable number of resource records, where the number of records
is specified in the corresponding count field in the header.
Each resource record has this format:
      0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /                                               /
    /                      NAME                     /
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                      TYPE                     |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                     CLASS                     |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                      TTL                      |
    |                                               |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                   RDLENGTH                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--|
    /                     RDATA                     /
    /                                               /
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
NAME    a domain name to which this resource record pertains.
TYPE    two octets containing one of the RR type codes.  This
        field specifies the meaning of the data in the RDATA field.
CLASS   two octets which specify the class of the data in the RDATA field.
TTL     a 32 bit unsigned integer that specifies the time interval
        (in seconds) that the record may be cached.
RDLENGTH a 16 bit integer, length in octets of the RDATA field.
RDATA   a variable length string of octets that describes the resource.
        The format of this information varies according to the TYPE
        and CLASS of the resource record.
        If the TYPE is A and the CLASS is IN, it's a 4 octet IP address.

4.1.4. Message compression

In order to reduce the size of messages, domain names coan be compressed.
An entire domain name or a list of labels at the end of a domain name
is replaced with a pointer to a prior occurrence of the same name.

The pointer takes the form of a two octet sequence:
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    | 1  1|                OFFSET                   |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
The first two bits are ones.  This allows a pointer to be distinguished
from a label, since the label must begin with two zero bits because
labels are restricted to 63 octets or less.  The OFFSET field specifies
an offset from the start of the message (i.e., the first octet
of the ID field in the domain header).
A zero offset specifies the first byte of the ID field, etc.
Domain name in a message can be represented as either:
   - a sequence of labels ending in a zero octet
   - a pointer
   - a sequence of labels ending with a pointer
 */
unsafe extern "C" fn process_packet(
  mut conf_data: *mut dns_entry,
  mut conf_ttl: u32,
  mut buf: *mut u8,
) -> libc::c_int {
  let mut head: *mut dns_head = 0 as *mut dns_head;
  let mut unaligned_type_class: *mut type_and_class = 0 as *mut type_and_class;
  let mut err_msg: *const libc::c_char = 0 as *const libc::c_char;
  let mut query_string: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut answstr: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut answb: *mut u8 = 0 as *mut u8;
  let mut outr_rlen: u16 = 0;
  let mut outr_flags: u16 = 0;
  let mut type_0: u16 = 0;
  let mut class: u16 = 0;
  let mut query_len: libc::c_int = 0;
  head = buf as *mut dns_head;
  if (*head).nquer as libc::c_int == 0i32 {
    bb_simple_error_msg(b"packet has 0 queries, ignored\x00" as *const u8 as *const libc::c_char);
    return 0i32;
    /* don't reply */
  }
  if (*head).flags as libc::c_int
    & ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = 0x8000i32 as libc::c_ushort;
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
    != 0
  {
    /* QR bit */
    bb_simple_error_msg(b"response packet, ignored\x00" as *const u8 as *const libc::c_char);
    return 0i32;
    /* don't reply */
  }
  /* QR = 1 "response", RCODE = 4 "Not Implemented" */
  outr_flags = {
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = (0x8000i32 | 4i32) as libc::c_ushort;
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
  err_msg = 0 as *const libc::c_char;
  /* start of query string */
  query_string = head.offset(1) as *mut libc::c_void as *mut libc::c_char;
  /* caller guarantees strlen is <= MAX_PACK_LEN */
  query_len = strlen(query_string).wrapping_add(1i32 as libc::c_ulong) as libc::c_int;
  /* may be unaligned! */
  unaligned_type_class =
    query_string.offset(query_len as isize) as *mut libc::c_void as *mut type_and_class;
  query_len = (query_len as libc::c_ulong)
    .wrapping_add(::std::mem::size_of::<type_and_class>() as libc::c_ulong)
    as libc::c_int as libc::c_int;
  /* where to append answer block */
  answb = unaligned_type_class.offset(1) as *mut libc::c_void as *mut u8;
  /* OPCODE != 0 "standard query"? */
  if (*head).flags as libc::c_int
    & ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = 0x7800i32 as libc::c_ushort;
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
    != 0i32
  {
    err_msg = b"opcode != 0\x00" as *const u8 as *const libc::c_char
  } else {
    class = *(&mut (*unaligned_type_class).class as *mut u16 as *mut bb__aliased_u16);
    if class as libc::c_int
      != ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = 1i32 as libc::c_ushort;
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
      /* not class INET? */
      err_msg = b"class != 1\x00" as *const u8 as *const libc::c_char
    } else {
      type_0 = *(&mut (*unaligned_type_class).type_0 as *mut u16 as *mut bb__aliased_u16);
      if type_0 as libc::c_int
        != ({
          let mut __v: libc::c_ushort = 0;
          let mut __x: libc::c_ushort = REQ_A as libc::c_int as libc::c_ushort;
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
        && type_0 as libc::c_int
          != ({
            let mut __v: libc::c_ushort = 0;
            let mut __x: libc::c_ushort = REQ_PTR as libc::c_int as libc::c_ushort;
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
      {
        /* we can't handle this query type */
        //TODO: happens all the time with REQ_AAAA (0x1c) requests - implement those?
        err_msg = b"type is !REQ_A and !REQ_PTR\x00" as *const u8 as *const libc::c_char
      } else {
        /* look up the name */
        answstr = table_lookup(conf_data, type_0, query_string);
        outr_rlen = 4i32 as u16;
        if !answstr.is_null()
          && type_0 as libc::c_int
            == ({
              let mut __v: libc::c_ushort = 0;
              let mut __x: libc::c_ushort = REQ_PTR as libc::c_int as libc::c_ushort;
              if 0 != 0 {
                __v = (__x as libc::c_int >> 8i32 & 0xffi32
                  | (__x as libc::c_int & 0xffi32) << 8i32) as libc::c_ushort
              } else {
                let fresh24 = &mut __v;
                let fresh25;
                let fresh26 = __x;
                asm!("rorw $$8, ${0:w}" : "=r" (fresh25) :
                                         "0"
                                         (c2rust_asm_casts::AsmCast::cast_in(fresh24, fresh26))
                                         : "cc");
                c2rust_asm_casts::AsmCast::cast_out(fresh24, fresh26, fresh25);
              }
              __v
            }) as libc::c_int
        {
          /* returning a host name */
          outr_rlen = strlen(answstr).wrapping_add(1i32 as libc::c_ulong) as u16
        }
        if answstr.is_null()
          || (answb.wrapping_offset_from(buf) as libc::c_long as libc::c_uint)
            .wrapping_add(query_len as libc::c_uint)
            .wrapping_add(4i32 as libc::c_uint)
            .wrapping_add(2i32 as libc::c_uint)
            .wrapping_add(outr_rlen as libc::c_uint)
            > MAX_PACK_LEN as libc::c_int as libc::c_uint
        {
          /* QR = 1 "response"
           * AA = 1 "Authoritative Answer"
           * RCODE = 3 "Name Error" */
          err_msg = b"name is not found\x00" as *const u8 as *const libc::c_char;
          outr_flags = {
            let mut __v: libc::c_ushort = 0;
            let mut __x: libc::c_ushort = (0x8000i32 | 0x400i32 | 3i32) as libc::c_ushort;
            if 0 != 0 {
              __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
                as libc::c_ushort
            } else {
              let fresh27 = &mut __v;
              let fresh28;
              let fresh29 = __x;
              asm!("rorw $$8, ${0:w}" : "=r" (fresh28) :
                                      "0"
                                      (c2rust_asm_casts::AsmCast::cast_in(fresh27, fresh29))
                                      : "cc");
              c2rust_asm_casts::AsmCast::cast_out(fresh27, fresh29, fresh28);
            }
            __v
          }
        } else {
          /* Append answer Resource Record */
          memcpy(
            answb as *mut libc::c_void,
            query_string as *const libc::c_void,
            query_len as libc::c_ulong,
          ); /* name, type, class */
          answb = answb.offset(query_len as isize);
          *(answb as *mut u32 as *mut bb__aliased_u32) = {
            let mut __v: libc::c_uint = 0;
            let mut __x: libc::c_uint = conf_ttl;
            if 0 != 0 {
              __v = (__x & 0xff000000u32) >> 24i32
                | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
                | (__x & 0xff00i32 as libc::c_uint) << 8i32
                | (__x & 0xffi32 as libc::c_uint) << 24i32
            } else {
              let fresh30 = &mut __v;
              let fresh31;
              let fresh32 = __x;
              asm!("bswap $0" : "=r" (fresh31) : "0"
                                      (c2rust_asm_casts::AsmCast::cast_in(fresh30, fresh32))
                                      :);
              c2rust_asm_casts::AsmCast::cast_out(fresh30, fresh32, fresh31);
            }
            __v
          };
          answb = answb.offset(4);
          *(answb as *mut u16 as *mut bb__aliased_u16) = {
            let mut __v: libc::c_ushort = 0;
            let mut __x: libc::c_ushort = outr_rlen;
            if 0 != 0 {
              __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
                as libc::c_ushort
            } else {
              let fresh33 = &mut __v;
              let fresh34;
              let fresh35 = __x;
              asm!("rorw $$8, ${0:w}" : "=r" (fresh34) :
                                      "0"
                                      (c2rust_asm_casts::AsmCast::cast_in(fresh33, fresh35))
                                      : "cc");
              c2rust_asm_casts::AsmCast::cast_out(fresh33, fresh35, fresh34);
            }
            __v
          };
          answb = answb.offset(2);
          memcpy(
            answb as *mut libc::c_void,
            answstr as *const libc::c_void,
            outr_rlen as libc::c_ulong,
          );
          answb = answb.offset(outr_rlen as libc::c_int as isize);
          /* QR = 1 "response",
           * AA = 1 "Authoritative Answer",
           * TODO: need to set RA bit 0x80? One user says nslookup complains
           * "Got recursion not available from SERVER, trying next server"
           * "** server can't find HOSTNAME"
           * RCODE = 0 "success"
           */
          if option_mask32 & 1i32 as libc::c_uint != 0 {
            bb_simple_info_msg(b"returning positive reply\x00" as *const u8 as *const libc::c_char);
          }
          outr_flags = {
            let mut __v: libc::c_ushort = 0;
            let mut __x: libc::c_ushort = (0x8000i32 | 0x400i32 | 0i32) as libc::c_ushort;
            if 0 != 0 {
              __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
                as libc::c_ushort
            } else {
              let fresh36 = &mut __v;
              let fresh37;
              let fresh38 = __x;
              asm!("rorw $$8, ${0:w}" : "=r" (fresh37) :
                                      "0"
                                      (c2rust_asm_casts::AsmCast::cast_in(fresh36, fresh38))
                                      : "cc");
              c2rust_asm_casts::AsmCast::cast_out(fresh36, fresh38, fresh37);
            }
            __v
          };
          /* we have one answer */
          (*head).nansw = {
            let mut __v: libc::c_ushort = 0;
            let mut __x: libc::c_ushort = 1i32 as libc::c_ushort;
            if 0 != 0 {
              __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
                as libc::c_ushort
            } else {
              let fresh39 = &mut __v;
              let fresh40;
              let fresh41 = __x;
              asm!("rorw $$8, ${0:w}" : "=r" (fresh40) :
                                      "0"
                                      (c2rust_asm_casts::AsmCast::cast_in(fresh39, fresh41))
                                      : "cc");
              c2rust_asm_casts::AsmCast::cast_out(fresh39, fresh41, fresh40);
            }
            __v
          }
        }
      }
    }
  }
  if outr_flags as libc::c_int
    & ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = 0xfi32 as libc::c_ushort;
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
    != 0i32
  {
    /* not a positive response */
    if option_mask32 & 1i32 as libc::c_uint != 0 {
      bb_error_msg(
        b"%s, %s\x00" as *const u8 as *const libc::c_char,
        err_msg,
        if option_mask32 & 2i32 as libc::c_uint != 0 {
          b"dropping query\x00" as *const u8 as *const libc::c_char
        } else {
          b"sending error reply\x00" as *const u8 as *const libc::c_char
        },
      ); // why???
    }
    if option_mask32 & 2i32 as libc::c_uint != 0 {
      return 0i32;
    }
  }
  (*head).flags = ((*head).flags as libc::c_int | outr_flags as libc::c_int) as u16;
  (*head).nadd = 0i32 as u16;
  (*head).nauth = (*head).nadd;
  (*head).nquer = {
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = 1i32 as libc::c_ushort;
    if 0 != 0 {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh45 = &mut __v;
      let fresh46;
      let fresh47 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh46) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh45, fresh47)) :
                      "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh45, fresh47, fresh46);
    }
    __v
  };
  return answb.wrapping_offset_from(buf) as libc::c_long as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dnsd_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut listen_interface: *const libc::c_char =
    b"0.0.0.0\x00" as *const u8 as *const libc::c_char;
  let mut fileconf: *const libc::c_char = b"/etc/dnsd.conf\x00" as *const u8 as *const libc::c_char;
  let mut conf_data: *mut dns_entry = 0 as *mut dns_entry;
  let mut conf_ttl: u32 = DEFAULT_TTL as libc::c_int as u32;
  let mut sttl: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut sport: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut lsa: *mut len_and_sockaddr = 0 as *mut len_and_sockaddr;
  let mut from: *mut len_and_sockaddr = 0 as *mut len_and_sockaddr;
  let mut to: *mut len_and_sockaddr = 0 as *mut len_and_sockaddr;
  let mut lsa_size: libc::c_uint = 0;
  let mut udps: libc::c_int = 0;
  let mut opts: libc::c_int = 0;
  let mut port: u16 = 53i32 as u16;
  /* Ensure buf is 32bit aligned (we need 16bit, but 32bit can't hurt) */
  let mut buf: [u8; 513] = [0; 513];
  opts = getopt32(
    argv,
    b"vsi:c:t:p:d\x00" as *const u8 as *const libc::c_char,
    &mut listen_interface as *mut *const libc::c_char,
    &mut fileconf as *mut *const libc::c_char,
    &mut sttl as *mut *mut libc::c_char,
    &mut sport as *mut *mut libc::c_char,
  ) as libc::c_int;
  //if (opts & (1 << 0)) // -v
  //if (opts & (1 << 1)) // -s
  //if (opts & (1 << 2)) // -i
  //if (opts & (1 << 3)) // -c
  if opts & 1i32 << 4i32 != 0 {
    // -t
    conf_ttl = xatou_range(sttl, 1i32 as libc::c_uint, 0xffffffffu32)
  }
  if opts & 1i32 << 5i32 != 0 {
    // -p
    port = xatou_range(sport, 1i32 as libc::c_uint, 0xffffi32 as libc::c_uint) as u16
  }
  if opts & 1i32 << 6i32 != 0 {
    // -d
    bb_daemonize_or_rexec(DAEMON_CLOSE_EXTRA_FDS as libc::c_int); /* needed for recv_from_to to work */
    openlog(applet_name, 0x1i32, 3i32 << 3i32);
    logmode = LOGMODE_SYSLOG as libc::c_int as smallint
  }
  conf_data = parse_conf_file(fileconf);
  lsa = xdotted2sockaddr(listen_interface, port as libc::c_int);
  udps = xsocket(
    (*lsa).u.sa.sa_family as libc::c_int,
    SOCK_DGRAM as libc::c_int,
    0i32,
  );
  xbind(udps, &mut (*lsa).u.sa, (*lsa).len);
  socket_want_pktinfo(udps);
  lsa_size = (LSA_LEN_SIZE as libc::c_int as libc::c_uint).wrapping_add((*lsa).len);
  from = xzalloc(lsa_size as size_t) as *mut len_and_sockaddr;
  to = xzalloc(lsa_size as size_t) as *mut len_and_sockaddr;
  let mut p: *mut libc::c_char = xmalloc_sockaddr2dotted(&mut (*lsa).u.sa);
  bb_info_msg(
    b"accepting UDP packets on %s\x00" as *const u8 as *const libc::c_char,
    p,
  );
  free(p as *mut libc::c_void);
  loop {
    let mut r: libc::c_int = 0;
    /* Try to get *DEST* address (to which of our addresses
     * this query was directed), and reply from the same address.
     * Or else we can exhibit usual UDP ugliness:
     * [ip1.multihomed.ip2] <=  query to ip1  <= peer
     * [ip1.multihomed.ip2] => reply from ip2 => peer (confused) */
    memcpy(
      to as *mut libc::c_void,
      lsa as *const libc::c_void,
      lsa_size as libc::c_ulong,
    ); /* paranoia */
    r = recv_from_to(
      udps,
      buf.as_mut_ptr() as *mut libc::c_void,
      (MAX_PACK_LEN as libc::c_int + 1i32) as size_t,
      0i32,
      &mut (*from).u.sa,
      &mut (*to).u.sa,
      (*lsa).len,
    ) as libc::c_int;
    if r < 12i32 || r > MAX_PACK_LEN as libc::c_int {
      bb_error_msg(
        b"packet size %d, ignored\x00" as *const u8 as *const libc::c_char,
        r,
      );
    } else {
      if option_mask32 & 1i32 as libc::c_uint != 0 {
        bb_simple_info_msg(b"got UDP packet\x00" as *const u8 as *const libc::c_char);
      }
      buf[r as usize] = '\u{0}' as i32 as u8;
      r = process_packet(conf_data, conf_ttl, buf.as_mut_ptr());
      if r <= 0i32 {
        continue;
      }
      send_to_from(
        udps,
        buf.as_mut_ptr() as *mut libc::c_void,
        r as size_t,
        0i32,
        &mut (*from).u.sa,
        &mut (*to).u.sa,
        (*lsa).len,
      );
    }
  }
}

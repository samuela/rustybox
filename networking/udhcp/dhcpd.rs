use crate::libbb::appletlib::applet_name;
use crate::libbb::ptr_to_globals::bb_errno;
use crate::networking::udhcp::common::BUG_bb_strtou32_unimplemented;
use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
use libc::close;
use libc::free;
use libc::openlog;
use libc::strcasecmp;
use libc::time;
use libc::unlink;
extern "C" {
  //extern const int const_int_1;
  /* This struct is deliberately not defined. */
  /* See docs/keep_data_small.txt */
  pub type globals;
  #[no_mangle]
  fn ether_aton_r(__asc: *const libc::c_char, __addr: *mut ether_addr) -> *mut ether_addr;

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
  #[no_mangle]
  fn strtok_r(
    __s: *mut libc::c_char,
    __delim: *const libc::c_char,
    __save_ptr: *mut *mut libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn strnlen(__string: *const libc::c_char, __maxlen: size_t) -> size_t;

  #[no_mangle]
  fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
  /* Some useful definitions */
  /* Macros for min/max.  */
  /* buffer allocation schemes */
  /* glibc uses __errno_location() to get a ptr to errno */
  /* We can just memorize it once - no multithreading in busybox :) */

  #[no_mangle]
  fn monotonic_sec() -> libc::c_uint;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn open_or_warn(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xhost_and_af2sockaddr(
    host: *const libc::c_char,
    port: libc::c_int,
    af: sa_family_t,
  ) -> *mut len_and_sockaddr;
  #[no_mangle]
  fn safe_strncpy(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    size: size_t,
  ) -> *mut libc::c_char;
  // NB: will return short read on error, not -1,
  // if some data was read before error occurred
  #[no_mangle]
  fn full_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  // NB: will return short write on error, not -1,
  // if some data was written before error occurred
  #[no_mangle]
  fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn xatou(str: *const libc::c_char) -> libc::c_uint;
  /* Non-aborting kind of convertors: bb_strto[u][l]l */
  /* On exit: errno = 0 only if there was non-empty, '\0' terminated value
   * errno = EINVAL if value was not '\0' terminated, but otherwise ok
   *    Return value is still valid, caller should just check whether end[0]
   *    is a valid terminating char for particular case. OTOH, if caller
   *    requires '\0' terminated input, [s]he can just check errno == 0.
   * errno = ERANGE if value had alphanumeric terminating char ("1234abcg").
   * errno = ERANGE if value is out of range, missing, etc.
   * errno = ERANGE if value had minus sign for strtouXX (even "-0" is not ok )
   *    return value is all-ones in this case.
   */
  #[no_mangle]
  fn bb_strtoull(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_ulonglong;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  // #[no_mangle]
  // fn BUG_bb_strtou32_unimplemented() -> u32;
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
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
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
  fn config_open(filename: *const libc::c_char) -> *mut parser_t;
  /* delims[0] is a comment char (use '\0' to disable), the rest are token delimiters */
  #[no_mangle]
  fn config_read(
    parser: *mut parser_t,
    tokens: *mut *mut libc::c_char,
    flags: libc::c_uint,
    delims: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  fn config_close(parser: *mut parser_t);

  /* '*const' ptr makes gcc optimize code much better.
   * Magic prevents ptr_to_globals from going into rodata.
   * If you want to assign a value, use SET_PTR_TO_GLOBALS(x) */
  #[no_mangle]
  static ptr_to_globals: *mut globals;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];

  /*
   * Russ Dill <Russ.Dill@asu.edu> September 2001
   * Rewritten by Vladimir Oleynik <dzo@simtreas.ru> (C) 2003
   *
   * Licensed under GPLv2 or later, see file LICENSE in this source tree.
   */
  #[no_mangle]
  static MAC_BCAST_ADDR: [u8; 6];
  #[no_mangle]
  static dhcp_optflags: [dhcp_optflag; 0];
  #[no_mangle]
  static dhcp_option_strings: [libc::c_char; 0];
  #[no_mangle]
  fn udhcp_get_option(packet: *mut dhcp_packet, code: libc::c_int) -> *mut u8;
  /* Same as above + ensures that option length is 4 bytes
   * (returns NULL if size is different)
   */
  #[no_mangle]
  fn udhcp_get_option32(packet: *mut dhcp_packet, code: libc::c_int) -> *mut u8;
  #[no_mangle]
  fn udhcp_add_binary_option(packet: *mut dhcp_packet, addopt: *mut u8);
  #[no_mangle]
  fn udhcp_add_simple_option(packet: *mut dhcp_packet, code: u8, data: u32);
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
  /* ** Other shared functions ***/
  /* 2nd param is "u32*" */
  #[no_mangle]
  fn udhcp_str2nip(str: *const libc::c_char, arg: *mut libc::c_void) -> libc::c_int;
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
}

pub type __int64_t = libc::c_long;

pub type __socklen_t = libc::c_uint;

use crate::librb::size_t;
use libc::ssize_t;
use libc::time_t;
pub type int64_t = __int64_t;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ether_addr {
  pub ether_addr_octet: [u8; 6],
}
pub type socklen_t = __socklen_t;
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
pub type bb__aliased_u32 = u32;

/*
 * Copyright 2006, Bernhard Reutner-Fischer
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Convenience macros to test the version of gcc. */
/* __restrict is known in EGCS 1.2 and above. */
/* "The malloc attribute is used to tell the compiler that a function
 * may be treated as if any non-NULL pointer it returns cannot alias
 * any other pointer valid when the function returns. This will often
 * improve optimization. Standard functions with this property include
 * malloc and calloc. realloc-like functions have this property as long
 * as the old pointer is never referred to (including comparing it
 * to the new pointer) after the function returns a non-NULL value."
 */
/* __NO_INLINE__: some gcc's do not honor inlining! :( */
/* I've seen a toolchain where I needed __noinline__ instead of noinline */
/* used by unit test machinery to run registration functions before calling main() */
/* -fwhole-program makes all symbols local. The attribute externally_visible
 * forces a symbol global.  */
//__attribute__ ((__externally_visible__))
/* At 4.4 gcc become much more anal about this, need to use "aliased" types */
/* We use __extension__ in some places to suppress -pedantic warnings
 * about GCC extensions.  This feature didn't work properly before
 * gcc 2.8.  */
/* FAST_FUNC is a qualifier which (possibly) makes function call faster
 * and/or smaller by using modified ABI. It is usually only needed
 * on non-static, busybox internal functions. Recent versions of gcc
 * optimize statics automatically. FAST_FUNC on static is required
 * only if you need to match a function pointer's type.
 * FAST_FUNC may not work well with -flto so allow user to disable this.
 * (-DFAST_FUNC= )
 */
/* Make all declarations hidden (-fvisibility flag only affects definitions) */
/* (don't include system headers after this until corresponding pop!) */
/* gcc-2.95 had no va_copy but only __va_copy. */
/* ---- Endian Detection ------------------------------------ */
/* SWAP_LEnn means "convert CPU<->little_endian by swapping bytes" */
/* ---- Unaligned access ------------------------------------ */
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
use crate::librb::smallint;
use libc::FILE;
pub type nfds_t = libc::c_ulong;
use libc::pollfd;
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
pub const LOGMODE_BOTH: C2RustUnnamed_1 = 3;
pub const LOGMODE_SYSLOG: C2RustUnnamed_1 = 2;
pub const LOGMODE_STDIO: C2RustUnnamed_1 = 1;
pub const LOGMODE_NONE: C2RustUnnamed_1 = 0;
/*
 * Config file parser
 */
pub type C2RustUnnamed_2 = libc::c_uint;
// comments are recognized even if there is whitespace before
// ("line start><space><tab><space>#comment" is also comment, not only "line start>#comment")
// NORMAL is:
// * remove leading and trailing delimiters and collapse
//   multiple delimiters into one
// * warn and continue if less than mintokens delimiters found
// * grab everything into last token
// * comments are recognized even if they aren't the first char
pub const PARSE_NORMAL: C2RustUnnamed_2 = 4653056;
// delim[0] and delim[1] are two different allowed comment chars
// (so far, delim[0] will only work as comment char for full-line comment)
// (IOW: it works as if PARSE_EOL_COMMENTS is not set. sysctl applet is okay with this)
pub const PARSE_WS_COMMENTS: C2RustUnnamed_2 = 16777216;
// comments are recognized even if they aren't the first char
pub const PARSE_ALT_COMMENTS: C2RustUnnamed_2 = 8388608;
pub const PARSE_EOL_COMMENTS: C2RustUnnamed_2 = 4194304;
// die if < min tokens found
// keep a copy of current line
pub const PARSE_KEEP_COPY: C2RustUnnamed_2 = 2097152;
// last token takes entire remainder of the line
pub const PARSE_MIN_DIE: C2RustUnnamed_2 = 1048576;
// trim leading and trailing delimiters
// TODO: COLLAPSE and TRIM seem to always go in pair
pub const PARSE_GREEDY: C2RustUnnamed_2 = 262144;
// treat consecutive delimiters as one
pub const PARSE_TRIM: C2RustUnnamed_2 = 131072;
pub const PARSE_COLLAPSE: C2RustUnnamed_2 = 65536;
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
/* six all-ones */
/* ** DHCP packet ***/
/* DHCP protocol. See RFC 2131 */
//TODO: rename ciaddr/yiaddr/chaddr
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
/* DHCP option codes (partial list). See RFC 2132 and
 * http://www.iana.org/assignments/bootp-dhcp-parameters/
 * Commented out options are handled by common option machinery,
 * uncommented ones have special cases (grep for them to see).
 */
//#define DHCP_TIME_OFFSET      0x02 /* (localtime - UTC_time) in seconds. signed */
//#define DHCP_ROUTER           0x03
//#define DHCP_TIME_SERVER      0x04 /* RFC 868 time server (32-bit, 0 = 1.1.1900) */
//#define DHCP_NAME_SERVER      0x05 /* IEN 116 _really_ ancient kind of NS */
//#define DHCP_DNS_SERVER       0x06
//#define DHCP_LOG_SERVER       0x07 /* port 704 UDP log (not syslog) */
//#define DHCP_COOKIE_SERVER    0x08 /* "quote of the day" server */
//#define DHCP_LPR_SERVER       0x09
/* 12: either client informs server or server gives name to client */
//#define DHCP_BOOT_SIZE        0x0d
//#define DHCP_DOMAIN_NAME      0x0f /* 15: server gives domain suffix */
//#define DHCP_SWAP_SERVER      0x10
//#define DHCP_ROOT_PATH        0x11
//#define DHCP_IP_TTL           0x17
//#define DHCP_MTU              0x1a
//#define DHCP_BROADCAST        0x1c
//#define DHCP_ROUTES           0x21
//#define DHCP_NIS_DOMAIN       0x28
//#define DHCP_NIS_SERVER       0x29
//#define DHCP_NTP_SERVER       0x2a
//#define DHCP_WINS_SERVER      0x2c
/* 50: sent by client if specific IP is wanted */
/* 51: */
/* 52: */
/* 53: */
/* 54: server's IP */
/* 55: list of options client wants */
//#define DHCP_ERR_MESSAGE      0x38 /* 56: error message when sending NAK etc */
/* 57: */
/* 60: client's vendor (a string) */
/* 61: by default client's MAC addr, but may be arbitrarily long */
//#define DHCP_TFTP_SERVER_NAME 0x42 /* 66: same as 'sname' field */
//#define DHCP_BOOT_FILE        0x43 /* 67: same as 'file' field */
//#define DHCP_USER_CLASS       0x4d /* 77: RFC 3004. set of LASCII strings. "I am a printer" etc */
/* 81: client asks to update DNS to map its FQDN to its new IP */
//#define DHCP_PCODE            0x64 /* 100: RFC 4833. IEEE 1003.1 TZ string */
//#define DHCP_TCODE            0x65 /* 101: RFC 4833. Reference to the TZ database string */
//#define DHCP_DOMAIN_SEARCH    0x77 /* 119: RFC 3397. set of ASCIZ string, DNS-style compressed */
//#define DHCP_SIP_SERVERS      0x78 /* 120: RFC 3361. flag byte, then: 0: domain names, 1: IP addrs */
//#define DHCP_STATIC_ROUTES    0x79 /* 121: RFC 3442. (mask,ip,router) tuples */
//#define DHCP_VLAN_ID          0x84 /* 132: 802.1P VLAN ID */
//#define DHCP_VLAN_PRIORITY    0x85 /* 133: 802.1Q VLAN priority */
//#define DHCP_PXE_CONF_FILE    0xd1 /* 209: RFC 5071 Configuration file */
//#define DHCP_PXE_PATH_PREFIX  0xd2 /* 210: RFC 5071 Path prefix */
//#define DHCP_REBOOT_TIME      0xd3 /* 211: RFC 5071 Reboot time */
//#define DHCP_MS_STATIC_ROUTES 0xf9 /* 249: Microsoft's pre-RFC 3442 code for 0x79? */
//#define DHCP_WPAD             0xfc /* 252: MSIE's Web Proxy Autodiscovery Protocol */
/* 255: */
/* Offsets in option byte sequence */
/* Offsets in option byte sequence for DHCPv6 */
/* Bits in "overload" option */
/* DHCP_MESSAGE_TYPE values */
/* client -> server */
/* client <- server */
/* client -> server */
/* client -> server */
/* client <- server */
/* client <- server */
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct static_lease {
  pub next: *mut static_lease,
  pub nip: u32,
  pub mac: [u8; 6],
  pub opt: [u8; 1],
  /* interface to use */
  //TODO: ifindex, server_nip, server_mac
  // are obtained from interface name.
  // Instead of querying them *once*, create update_server_network_data_cache()
  // and call it before any usage of these fields.
  // update_server_network_data_cache() must re-query data
  // if more than N seconds have passed after last use.
  /* our MAC address (used only for ARP probing) */
  /* list of DHCP options loaded from the config file */
  /* start,end are in host order: we need to compare start <= ip <= end */
  /* start address of leases, in host order */
  /* end of leases, in host order */
  /* maximum lease time (host order) */
  /* minimum lease time a client can request */
  /* maximum number of leases (including reserved addresses) */
  /* how long should udhcpd wait before writing a config file.
   * if this is zero, it will only write one on SIGUSR1 */
  /* how long an address is reserved if a client returns a
   * decline message */
  /* how long an arp conflict offender is leased for */
  /* how long an offered address is reserved */
  /* "next server" bootp option */
  /* what to run whenever leases are written */
  /* bootp server name */
  /* bootp boot file option */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server_data_t {
  pub interface: *mut libc::c_char,
  pub ifindex: libc::c_int,
  pub server_nip: u32,
  pub server_mac: [u8; 6],
  pub options: *mut option_set,
  pub start_ip: u32,
  pub end_ip: u32,
  pub max_lease_sec: u32,
  pub min_lease_sec: u32,
  pub max_leases: u32,
  pub auto_time: u32,
  pub decline_time: u32,
  pub conflict_time: u32,
  pub offer_time: u32,
  pub siaddr_nip: u32,
  pub lease_file: *mut libc::c_char,
  pub pidfile: *mut libc::c_char,
  pub notify_file: *mut libc::c_char,
  pub sname: *mut libc::c_char,
  pub boot_file: *mut libc::c_char,
  pub static_leases: *mut static_lease,
  /* List of ip/mac pairs to assign static leases */
}
pub type leasetime_t = u32;
pub type signed_leasetime_t = i32;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct dyn_lease {
  pub expires: leasetime_t,
  pub lease_nip: u32,
  pub lease_mac: [u8; 6],
  pub hostname: [libc::c_char; 20],
  pub pad: [u8; 2],
  /* total size is a multiply of 4 */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_keyword {
  pub keyword: *const libc::c_char,
  pub handler:
    Option<unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_void) -> libc::c_int>,
  pub ofs: libc::c_uint,
  pub def: *const libc::c_char,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const KWS_WITH_DEFAULTS: C2RustUnnamed_3 = 12;
#[inline(always)]
unsafe extern "C" fn bb_strtoul(
  mut arg: *const libc::c_char,
  mut endp: *mut *mut libc::c_char,
  mut base: libc::c_int,
) -> libc::c_ulong {
  return bb_strtoull(arg, endp, base) as libc::c_ulong;
}
#[inline(always)]
unsafe extern "C" fn bb_strtou32(
  mut arg: *const libc::c_char,
  mut endp: *mut *mut libc::c_char,
  mut base: libc::c_int,
) -> u32 {
  if ::std::mem::size_of::<u32>() as libc::c_ulong
    == ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong
  {
    return bb_strtou(arg, endp, base);
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong
    == ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong
  {
    return bb_strtoul(arg, endp, base) as u32;
  }
  return BUG_bb_strtou32_unimplemented();
}
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
/* We redefine ctype macros. Unicode-correct handling of char types
 * can't be done with such byte-oriented operations anyway,
 * we don't lose anything.
 */
/* We save ~500 bytes on isdigit alone.
 * BTW, x86 likes (unsigned char) cast more than (unsigned). */
/* These work the same for ASCII and Unicode,
 * assuming no one asks "is this a *Unicode* letter?" using isalpha(letter) */
/* In POSIX/C locale isspace is only these chars: "\t\n\v\f\r" and space.
 * "\t\n\v\f\r" happen to have ASCII codes 9,10,11,12,13.
 */
// Unsafe wrt NUL: #define ispunct(a) (strchr("!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~", (a)) != NULL)
// Bigger code: #define isalnum(a) ({ unsigned char bb__isalnum = (a) - '0'; bb__isalnum <= 9 || ((bb__isalnum - ('A' - '0')) & 0xdf) <= 25; })
#[inline(always)]
unsafe extern "C" fn bb_ascii_isalnum(mut a: libc::c_uchar) -> libc::c_int {
  let mut b: libc::c_uchar = (a as libc::c_int - '0' as i32) as libc::c_uchar;
  if b as libc::c_int <= 9i32 {
    return (b as libc::c_int <= 9i32) as libc::c_int;
  }
  b = ((a as libc::c_int | 0x20i32) - 'a' as i32) as libc::c_uchar;
  return (b as libc::c_int <= 'z' as i32 - 'a' as i32) as libc::c_int;
}
/* Takes the address of the pointer to the static_leases linked list,
 * address to a 6 byte mac address,
 * 4 byte IP address */
unsafe extern "C" fn add_static_lease(
  mut st_lease_pp: *mut *mut static_lease,
  mut mac: *mut u8,
  mut nip: u32,
  mut opts: *const libc::c_char,
) {
  let mut st_lease: *mut static_lease = 0 as *mut static_lease;
  let mut optlen: libc::c_uint = 0;
  optlen = if !opts.is_null() {
    ((1i32 + 1i32) as libc::c_ulong).wrapping_add(strnlen(opts, 120i32 as size_t))
  } else {
    0i32 as libc::c_ulong
  } as libc::c_uint;
  loop
  /* Find the tail of the list */
  {
    st_lease = *st_lease_pp;
    if st_lease.is_null() {
      break;
    }
    st_lease_pp = &mut (*st_lease).next
  }
  /* Add new node */
  st_lease = xzalloc(
    (::std::mem::size_of::<static_lease>() as libc::c_ulong).wrapping_add(optlen as libc::c_ulong),
  ) as *mut static_lease;
  *st_lease_pp = st_lease;
  memcpy(
    (*st_lease).mac.as_mut_ptr() as *mut libc::c_void,
    mac as *const libc::c_void,
    6i32 as libc::c_ulong,
  );
  (*st_lease).nip = nip;
  /*st_lease->next = NULL;*/
  if optlen != 0 {
    *(*st_lease).opt.as_mut_ptr().offset(0) = 0xci32 as u8;
    optlen = optlen.wrapping_sub(2i32 as libc::c_uint);
    *(*st_lease).opt.as_mut_ptr().offset(1) = optlen as u8;
    memcpy(
      &mut *(*st_lease).opt.as_mut_ptr().offset(2) as *mut u8 as *mut libc::c_void,
      opts as *const libc::c_void,
      optlen as libc::c_ulong,
    );
  }
  /* Print out static leases just to check what's going on */
  if dhcp_verbose >= 2i32 as libc::c_uint {
    bb_info_msg(
      b"static lease: mac:%02x:%02x:%02x:%02x:%02x:%02x nip:%x\x00" as *const u8
        as *const libc::c_char,
      (*st_lease).mac[0] as libc::c_int,
      (*st_lease).mac[1] as libc::c_int,
      (*st_lease).mac[2] as libc::c_int,
      (*st_lease).mac[3] as libc::c_int,
      (*st_lease).mac[4] as libc::c_int,
      (*st_lease).mac[5] as libc::c_int,
      (*st_lease).nip,
    );
  };
}
/* Find static lease IP by mac */
unsafe extern "C" fn get_static_nip_by_mac(mut mac: *mut libc::c_void) -> u32 {
  let mut st_lease: *mut static_lease =
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).static_leases;
  while !st_lease.is_null() {
    if memcmp(
      (*st_lease).mac.as_mut_ptr() as *const libc::c_void,
      mac,
      6i32 as libc::c_ulong,
    ) == 0i32
    {
      return (*st_lease).nip;
    }
    st_lease = (*st_lease).next
  }
  return 0i32 as u32;
}
unsafe extern "C" fn is_nip_reserved_as_static(mut nip: u32) -> libc::c_int {
  let mut st_lease: *mut static_lease =
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).static_leases;
  while !st_lease.is_null() {
    if (*st_lease).nip == nip {
      return 1i32;
    }
    st_lease = (*st_lease).next
  }
  return 0i32;
}
/* Find the oldest expired lease, NULL if there are no expired leases */
unsafe extern "C" fn oldest_expired_lease() -> *mut dyn_lease {
  let mut oldest_lease: *mut dyn_lease = 0 as *mut dyn_lease;
  let mut oldest_time: leasetime_t = time(0 as *mut time_t) as leasetime_t;
  let mut i: libc::c_uint = 0;
  /* Unexpired leases have g_leases[i].expires >= current time
   * and therefore can't ever match */
  i = 0i32 as libc::c_uint;
  while i < (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).max_leases {
    if (*(ptr_to_globals as *mut dyn_lease).offset(i as isize)).expires == 0i32 as libc::c_uint
      || (*(ptr_to_globals as *mut dyn_lease).offset(i as isize)).expires < oldest_time
    {
      oldest_time = (*(ptr_to_globals as *mut dyn_lease).offset(i as isize)).expires;
      oldest_lease = &mut *(ptr_to_globals as *mut dyn_lease).offset(i as isize) as *mut dyn_lease
    }
    i = i.wrapping_add(1)
  }
  return oldest_lease;
}
/* Clear out all leases with matching nonzero chaddr OR yiaddr.
 * If chaddr == NULL, this is a conflict lease.
 */
unsafe extern "C" fn clear_leases(mut chaddr: *const u8, mut yiaddr: u32) {
  let mut i: libc::c_uint = 0;
  i = 0i32 as libc::c_uint;
  while i < (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).max_leases {
    if !chaddr.is_null()
      && memcmp(
        (*(ptr_to_globals as *mut dyn_lease).offset(i as isize))
          .lease_mac
          .as_mut_ptr() as *const libc::c_void,
        chaddr as *const libc::c_void,
        6i32 as libc::c_ulong,
      ) == 0i32
      || yiaddr != 0 && (*(ptr_to_globals as *mut dyn_lease).offset(i as isize)).lease_nip == yiaddr
    {
      memset(
        &mut *(ptr_to_globals as *mut dyn_lease).offset(i as isize) as *mut dyn_lease
          as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<dyn_lease>() as libc::c_ulong,
      );
    }
    i = i.wrapping_add(1)
  }
}
/* Add a lease into the table, clearing out any old ones.
 * If chaddr == NULL, this is a conflict lease.
 */
unsafe extern "C" fn add_lease(
  mut chaddr: *const u8,
  mut yiaddr: u32,
  mut leasetime: leasetime_t,
  mut hostname: *const libc::c_char,
  mut hostname_len: libc::c_int,
) -> *mut dyn_lease {
  let mut oldest: *mut dyn_lease = 0 as *mut dyn_lease;
  /* clean out any old ones */
  clear_leases(chaddr, yiaddr); /* include NUL */
  oldest = oldest_expired_lease();
  if !oldest.is_null() {
    memset(
      oldest as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<dyn_lease>() as libc::c_ulong,
    );
    if !hostname.is_null() {
      let mut p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
      hostname_len += 1;
      if hostname_len as libc::c_ulong
        > ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong
      {
        hostname_len = ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong as libc::c_int
      }
      p = safe_strncpy(
        (*oldest).hostname.as_mut_ptr(),
        hostname,
        hostname_len as size_t,
      );
      /*
       * Sanitization (s/bad_char/./g).
       * The intent is not to allow only "DNS-valid" hostnames,
       * but merely make dumpleases output safe for shells to use.
       * We accept "0-9A-Za-z._-", all other chars turn to dots.
       */
      while *p != 0 {
        if bb_ascii_isalnum(*p as libc::c_uchar) == 0
          && *p as libc::c_int != '-' as i32
          && *p as libc::c_int != '_' as i32
        {
          *p = '.' as i32 as libc::c_char
        }
        p = p.offset(1)
      }
    }
    if !chaddr.is_null() {
      memcpy(
        (*oldest).lease_mac.as_mut_ptr() as *mut libc::c_void,
        chaddr as *const libc::c_void,
        6i32 as libc::c_ulong,
      );
    }
    (*oldest).lease_nip = yiaddr;
    (*oldest).expires = (time(0 as *mut time_t) + leasetime as libc::c_long) as leasetime_t
  }
  return oldest;
}
/* True if a lease has expired */
unsafe extern "C" fn is_expired_lease(mut lease: *mut dyn_lease) -> libc::c_int {
  return ((*lease).expires < time(0 as *mut time_t) as leasetime_t) as libc::c_int;
}
/* Find the first lease that matches MAC, NULL if no match */
unsafe extern "C" fn find_lease_by_mac(mut mac: *const u8) -> *mut dyn_lease {
  let mut i: libc::c_uint = 0;
  i = 0i32 as libc::c_uint;
  while i < (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).max_leases {
    if memcmp(
      (*(ptr_to_globals as *mut dyn_lease).offset(i as isize))
        .lease_mac
        .as_mut_ptr() as *const libc::c_void,
      mac as *const libc::c_void,
      6i32 as libc::c_ulong,
    ) == 0i32
    {
      return &mut *(ptr_to_globals as *mut dyn_lease).offset(i as isize) as *mut dyn_lease;
    }
    i = i.wrapping_add(1)
  }
  return 0 as *mut dyn_lease;
}
/* Find the first lease that matches IP, NULL is no match */
unsafe extern "C" fn find_lease_by_nip(mut nip: u32) -> *mut dyn_lease {
  let mut i: libc::c_uint = 0;
  i = 0i32 as libc::c_uint;
  while i < (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).max_leases {
    if (*(ptr_to_globals as *mut dyn_lease).offset(i as isize)).lease_nip == nip {
      return &mut *(ptr_to_globals as *mut dyn_lease).offset(i as isize) as *mut dyn_lease;
    }
    i = i.wrapping_add(1)
  }
  return 0 as *mut dyn_lease;
}
/* Check if the IP is taken; if it is, add it to the lease table */
unsafe extern "C" fn nobody_responds_to_arp(
  mut nip: u32,
  mut safe_mac: *const u8,
  mut arpping_ms: libc::c_uint,
) -> libc::c_int {
  let mut temp: in_addr = in_addr { s_addr: 0 };
  let mut r: libc::c_int = 0;
  r = arpping(
    nip,
    safe_mac,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).server_nip,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t))
      .server_mac
      .as_mut_ptr(),
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).interface,
    arpping_ms,
  );
  if r != 0 {
    return r;
  }
  temp.s_addr = nip;
  bb_info_msg(
    b"%s belongs to someone, reserving it for %u seconds\x00" as *const u8 as *const libc::c_char,
    inet_ntoa(temp),
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).conflict_time,
  );
  add_lease(
    0 as *const u8,
    nip,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).conflict_time,
    0 as *const libc::c_char,
    0i32,
  );
  return 0i32;
}
/* Find a new usable (we think) address */
unsafe extern "C" fn find_free_or_expired_nip(
  mut safe_mac: *const u8,
  mut arpping_ms: libc::c_uint,
) -> u32 {
  let mut addr: u32 = 0;
  let mut oldest_lease: *mut dyn_lease = 0 as *mut dyn_lease;
  addr = (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).start_ip;
  loop {
    let mut nip: u32 = 0;
    let mut lease: *mut dyn_lease = 0 as *mut dyn_lease;
    /* ie, 192.168.55.0 */
    if !(addr & 0xffi32 as libc::c_uint == 0i32 as libc::c_uint) {
      /* ie, 192.168.55.255 */
      if !(addr & 0xffi32 as libc::c_uint == 0xffi32 as libc::c_uint) {
        nip = {
          let mut __v: libc::c_uint = 0;
          let mut __x: libc::c_uint = addr;
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
                                  (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2))
                                  :);
            c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
          }
          __v
        };
        /* skip our own address */
        if !(nip == (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).server_nip) {
          /* is this a static lease addr? */
          if !(is_nip_reserved_as_static(nip) != 0) {
            lease = find_lease_by_nip(nip);
            if lease.is_null() {
              //TODO: DHCP servers do not always sit on the same subnet as clients: should *ping*, not arp-ping!
              if nobody_responds_to_arp(nip, safe_mac, arpping_ms) != 0 {
                return nip;
              }
            } else if oldest_lease.is_null() || (*lease).expires < (*oldest_lease).expires {
              oldest_lease = lease
            }
          }
        }
      }
    }
    addr = addr.wrapping_add(1);
    if !(addr
      != (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t))
        .end_ip
        .wrapping_add(1i32 as libc::c_uint))
    {
      break;
    }
  }
  if !oldest_lease.is_null()
    && is_expired_lease(oldest_lease) != 0
    && nobody_responds_to_arp((*oldest_lease).lease_nip, safe_mac, arpping_ms) != 0
  {
    return (*oldest_lease).lease_nip;
  }
  return 0i32 as u32;
}
/* On these functions, make sure your datatype matches */
unsafe extern "C" fn read_str(
  mut line: *const libc::c_char,
  mut arg: *mut libc::c_void,
) -> libc::c_int {
  let mut dest: *mut *mut libc::c_char = arg as *mut *mut libc::c_char; /* it's "struct { u8 mac[6]; }" */
  free(*dest as *mut libc::c_void);
  *dest = xstrdup(line);
  return 1i32;
}
unsafe extern "C" fn read_u32(
  mut line: *const libc::c_char,
  mut arg: *mut libc::c_void,
) -> libc::c_int {
  *(arg as *mut u32) = bb_strtou32(line, 0 as *mut *mut libc::c_char, 10i32);
  return (*bb_errno == 0i32) as libc::c_int;
}
unsafe extern "C" fn read_staticlease(
  mut const_line: *const libc::c_char,
  mut arg: *mut libc::c_void,
) -> libc::c_int {
  let mut line: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut mac_string: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut ip_string: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut opts: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut mac_bytes: ether_addr = ether_addr {
    ether_addr_octet: [0; 6],
  };
  let mut nip: u32 = 0;
  /* Read mac */
  line = const_line as *mut libc::c_char;
  mac_string = strtok_r(
    line,
    b" \t\x00" as *const u8 as *const libc::c_char,
    &mut line,
  );
  if mac_string.is_null() || ether_aton_r(mac_string, &mut mac_bytes).is_null() {
    return 0i32;
  }
  /* Read ip */
  ip_string = strtok_r(
    std::ptr::null_mut::<libc::c_char>(),
    b" \t\x00" as *const u8 as *const libc::c_char,
    &mut line,
  );
  if ip_string.is_null() || udhcp_str2nip(ip_string, &mut nip as *mut u32 as *mut libc::c_void) == 0
  {
    return 0i32;
  }
  opts = strtok_r(
    std::ptr::null_mut::<libc::c_char>(),
    b" \t\x00" as *const u8 as *const libc::c_char,
    &mut line,
  );
  /* opts might be NULL, that's not an error */
  add_static_lease(
    arg as *mut *mut static_lease,
    &mut mac_bytes as *mut ether_addr as *mut u8,
    nip,
    opts,
  );
  return 1i32;
}
unsafe extern "C" fn read_optset(
  mut line: *const libc::c_char,
  mut arg: *mut libc::c_void,
) -> libc::c_int {
  return udhcp_str2optset(
    line,
    arg,
    dhcp_optflags.as_ptr(),
    dhcp_option_strings.as_ptr(),
    0i32 != 0,
  );
}

static mut keywords: [config_keyword; 18] = [
  {
    let mut init = config_keyword {
      keyword: b"start\x00" as *const u8 as *const libc::c_char,
      handler: Some(
        udhcp_str2nip
          as unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_void) -> libc::c_int,
      ),
      ofs: 32u64 as libc::c_uint,
      def: b"192.168.0.20\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = config_keyword {
      keyword: b"end\x00" as *const u8 as *const libc::c_char,
      handler: Some(
        udhcp_str2nip
          as unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_void) -> libc::c_int,
      ),
      ofs: 36u64 as libc::c_uint,
      def: b"192.168.0.254\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = config_keyword {
      keyword: b"interface\x00" as *const u8 as *const libc::c_char,
      handler: Some(
        read_str
          as unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_void) -> libc::c_int,
      ),
      ofs: 0u64 as libc::c_uint,
      def: b"eth0\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = config_keyword {
      keyword: b"max_leases\x00" as *const u8 as *const libc::c_char,
      handler: Some(
        read_u32
          as unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_void) -> libc::c_int,
      ),
      ofs: 48u64 as libc::c_uint,
      def: b"235\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = config_keyword {
      keyword: b"auto_time\x00" as *const u8 as *const libc::c_char,
      handler: Some(
        read_u32
          as unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_void) -> libc::c_int,
      ),
      ofs: 52u64 as libc::c_uint,
      def: b"7200\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = config_keyword {
      keyword: b"decline_time\x00" as *const u8 as *const libc::c_char,
      handler: Some(
        read_u32
          as unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_void) -> libc::c_int,
      ),
      ofs: 56u64 as libc::c_uint,
      def: b"3600\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = config_keyword {
      keyword: b"conflict_time\x00" as *const u8 as *const libc::c_char,
      handler: Some(
        read_u32
          as unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_void) -> libc::c_int,
      ),
      ofs: 60u64 as libc::c_uint,
      def: b"3600\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = config_keyword {
      keyword: b"offer_time\x00" as *const u8 as *const libc::c_char,
      handler: Some(
        read_u32
          as unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_void) -> libc::c_int,
      ),
      ofs: 64u64 as libc::c_uint,
      def: b"60\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = config_keyword {
      keyword: b"min_lease\x00" as *const u8 as *const libc::c_char,
      handler: Some(
        read_u32
          as unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_void) -> libc::c_int,
      ),
      ofs: 44u64 as libc::c_uint,
      def: b"60\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = config_keyword {
      keyword: b"lease_file\x00" as *const u8 as *const libc::c_char,
      handler: Some(
        read_str
          as unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_void) -> libc::c_int,
      ),
      ofs: 72u64 as libc::c_uint,
      def: b"/var/lib/misc/udhcpd.leases\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = config_keyword {
      keyword: b"pidfile\x00" as *const u8 as *const libc::c_char,
      handler: Some(
        read_str
          as unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_void) -> libc::c_int,
      ),
      ofs: 80u64 as libc::c_uint,
      def: b"/var/run/udhcpd.pid\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = config_keyword {
      keyword: b"siaddr\x00" as *const u8 as *const libc::c_char,
      handler: Some(
        udhcp_str2nip
          as unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_void) -> libc::c_int,
      ),
      ofs: 68u64 as libc::c_uint,
      def: b"0.0.0.0\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = config_keyword {
      keyword: b"option\x00" as *const u8 as *const libc::c_char,
      handler: Some(
        read_optset
          as unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_void) -> libc::c_int,
      ),
      ofs: 24u64 as libc::c_uint,
      def: b"\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = config_keyword {
      keyword: b"opt\x00" as *const u8 as *const libc::c_char,
      handler: Some(
        read_optset
          as unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_void) -> libc::c_int,
      ),
      ofs: 24u64 as libc::c_uint,
      def: b"\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = config_keyword {
      keyword: b"notify_file\x00" as *const u8 as *const libc::c_char,
      handler: Some(
        read_str
          as unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_void) -> libc::c_int,
      ),
      ofs: 88u64 as libc::c_uint,
      def: 0 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = config_keyword {
      keyword: b"sname\x00" as *const u8 as *const libc::c_char,
      handler: Some(
        read_str
          as unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_void) -> libc::c_int,
      ),
      ofs: 96u64 as libc::c_uint,
      def: 0 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = config_keyword {
      keyword: b"boot_file\x00" as *const u8 as *const libc::c_char,
      handler: Some(
        read_str
          as unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_void) -> libc::c_int,
      ),
      ofs: 104u64 as libc::c_uint,
      def: 0 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = config_keyword {
      keyword: b"static_lease\x00" as *const u8 as *const libc::c_char,
      handler: Some(
        read_staticlease
          as unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_void) -> libc::c_int,
      ),
      ofs: 112u64 as libc::c_uint,
      def: b"\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
];

#[inline(never)]
unsafe extern "C" fn read_config(mut file: *const libc::c_char) {
  let mut parser: *mut parser_t = 0 as *mut parser_t;
  let mut k: *const config_keyword = 0 as *const config_keyword;
  let mut i: libc::c_uint = 0;
  let mut token: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
  i = 0i32 as libc::c_uint;
  while i < KWS_WITH_DEFAULTS as libc::c_int as libc::c_uint {
    keywords[i as usize]
      .handler
      .expect("non-null function pointer")(
      keywords[i as usize].def,
      (bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t as *mut libc::c_char)
        .offset(keywords[i as usize].ofs as isize) as *mut libc::c_void,
    );
    i = i.wrapping_add(1)
  }
  parser = config_open(file);
  while config_read(
    parser,
    token.as_mut_ptr(),
    (PARSE_NORMAL as libc::c_int | (2i32 & 0xffi32) << 8i32 | 2i32 & 0xffi32) as libc::c_uint,
    b"# \t\x00" as *const u8 as *const libc::c_char,
  ) != 0
  {
    k = keywords.as_ptr();
    i = 0i32 as libc::c_uint;
    while i
      < (::std::mem::size_of::<[config_keyword; 18]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<config_keyword>() as libc::c_ulong)
        as libc::c_uint
    {
      if strcasecmp(token[0], (*k).keyword) == 0i32 {
        if (*k).handler.expect("non-null function pointer")(
          token[1],
          (bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t as *mut libc::c_char)
            .offset((*k).ofs as isize) as *mut libc::c_void,
        ) == 0
        {
          bb_error_msg(
            b"can\'t parse line %u in %s\x00" as *const u8 as *const libc::c_char,
            (*parser).lineno,
            file,
          );
          /* reset back to the default value */
          (*k).handler.expect("non-null function pointer")(
            (*k).def,
            (bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t as *mut libc::c_char)
              .offset((*k).ofs as isize) as *mut libc::c_void,
          );
        }
        break;
      } else {
        k = k.offset(1);
        i = i.wrapping_add(1)
      }
    }
  }
  config_close(parser);
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).start_ip = {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).start_ip;
    if false {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh3 = &mut __v;
      let fresh4;
      let fresh5 = __x;
      asm!("bswap $0" : "=r" (fresh4) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
    }
    __v
  };
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).end_ip = {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).end_ip;
    if false {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh6 = &mut __v;
      let fresh7;
      let fresh8 = __x;
      asm!("bswap $0" : "=r" (fresh7) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh6, fresh8)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
    }
    __v
  };
}
unsafe extern "C" fn write_leases() {
  let mut fd: libc::c_int = 0;
  let mut i: libc::c_uint = 0;
  let mut curr: leasetime_t = 0;
  let mut written_at: int64_t = 0;
  fd = open_or_warn(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).lease_file,
    0o1i32 | 0o100i32 | 0o1000i32,
  );
  if fd < 0i32 {
    return;
  }
  written_at = time(0 as *mut time_t);
  curr = written_at as leasetime_t;
  written_at = ({
    let mut __v: u64 = 0;
    let mut __x: u64 = written_at as u64;
    if false {
      __v = ((__x as libc::c_ulonglong & 0xff00000000000000u64) >> 56i32
        | (__x as libc::c_ulonglong & 0xff000000000000u64) >> 40i32
        | (__x as libc::c_ulonglong & 0xff0000000000u64) >> 24i32
        | (__x as libc::c_ulonglong & 0xff00000000u64) >> 8i32
        | (__x as libc::c_ulonglong & 0xff000000u64) << 8i32
        | (__x as libc::c_ulonglong & 0xff0000u64) << 24i32
        | (__x as libc::c_ulonglong & 0xff00u64) << 40i32
        | (__x as libc::c_ulonglong & 0xffu64) << 56i32) as u64
    } else {
      let fresh9 = &mut __v;
      let fresh10;
      let fresh11 = __x;
      asm!("bswap ${0:q}" : "=r" (fresh10) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh9, fresh11))
                      :);
      c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
    }
    __v
  }) as int64_t;
  full_write(
    fd,
    &mut written_at as *mut int64_t as *const libc::c_void,
    ::std::mem::size_of::<int64_t>() as libc::c_ulong,
  );
  i = 0i32 as libc::c_uint;
  while i < (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).max_leases {
    let mut tmp_time: leasetime_t = 0;
    if !((*(ptr_to_globals as *mut dyn_lease).offset(i as isize)).lease_nip == 0i32 as libc::c_uint)
    {
      /* Screw with the time in the struct, for easier writing */
      tmp_time = (*(ptr_to_globals as *mut dyn_lease).offset(i as isize)).expires;
      let ref mut fresh12 = (*(ptr_to_globals as *mut dyn_lease).offset(i as isize)).expires;
      *fresh12 = (*fresh12 as libc::c_uint).wrapping_sub(curr) as leasetime_t as leasetime_t;
      if ((*(ptr_to_globals as *mut dyn_lease).offset(i as isize)).expires as signed_leasetime_t)
        < 0i32
      {
        (*(ptr_to_globals as *mut dyn_lease).offset(i as isize)).expires = 0i32 as leasetime_t
      }
      (*(ptr_to_globals as *mut dyn_lease).offset(i as isize)).expires = {
        let mut __v: libc::c_uint = 0;
        let mut __x: libc::c_uint =
          (*(ptr_to_globals as *mut dyn_lease).offset(i as isize)).expires;
        if false {
          __v = (__x & 0xff000000u32) >> 24i32
            | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
            | (__x & 0xff00i32 as libc::c_uint) << 8i32
            | (__x & 0xffi32 as libc::c_uint) << 24i32
        } else {
          let fresh13 = &mut __v;
          let fresh14;
          let fresh15 = __x;
          asm!("bswap $0" : "=r" (fresh14) : "0"
                              (c2rust_asm_casts::AsmCast::cast_in(fresh13, fresh15))
                              :);
          c2rust_asm_casts::AsmCast::cast_out(fresh13, fresh15, fresh14);
        }
        __v
      };
      /* No error check. If the file gets truncated,
       * we lose some leases on restart. Oh well. */
      full_write(
        fd,
        &mut *(ptr_to_globals as *mut dyn_lease).offset(i as isize) as *mut dyn_lease
          as *const libc::c_void,
        ::std::mem::size_of::<dyn_lease>() as libc::c_ulong,
      );
      /* Then restore it when done */
      (*(ptr_to_globals as *mut dyn_lease).offset(i as isize)).expires = tmp_time
    }
    i = i.wrapping_add(1)
  }
  close(fd);
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t))
    .notify_file
    .is_null()
  {
    let mut argv: [*mut libc::c_char; 3] = [0 as *mut libc::c_char; 3];
    argv[0] = (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).notify_file;
    argv[1] = (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).lease_file;
    argv[2] = std::ptr::null_mut::<libc::c_char>();
    spawn_and_wait(argv.as_mut_ptr());
  };
}
#[inline(never)]
unsafe extern "C" fn read_leases(mut file: *const libc::c_char) {
  let mut lease: dyn_lease = dyn_lease {
    expires: 0,
    lease_nip: 0,
    lease_mac: [0; 6],
    hostname: [0; 20],
    pad: [0; 2],
  };
  let mut written_at: int64_t = 0;
  let mut time_passed: int64_t = 0;
  let mut fd: libc::c_int = 0;
  let mut i: libc::c_uint = 0i32 as libc::c_uint;
  fd = open_or_warn(file, 0i32);
  if fd < 0i32 {
    return;
  }
  if !(full_read(
    fd,
    &mut written_at as *mut int64_t as *mut libc::c_void,
    ::std::mem::size_of::<int64_t>() as libc::c_ulong,
  ) as libc::c_ulong
    != ::std::mem::size_of::<int64_t>() as libc::c_ulong)
  {
    written_at = ({
      let mut __v: u64 = 0;
      let mut __x: u64 = written_at as u64;
      if false {
        __v = ((__x as libc::c_ulonglong & 0xff00000000000000u64) >> 56i32
          | (__x as libc::c_ulonglong & 0xff000000000000u64) >> 40i32
          | (__x as libc::c_ulonglong & 0xff0000000000u64) >> 24i32
          | (__x as libc::c_ulonglong & 0xff00000000u64) >> 8i32
          | (__x as libc::c_ulonglong & 0xff000000u64) << 8i32
          | (__x as libc::c_ulonglong & 0xff0000u64) << 24i32
          | (__x as libc::c_ulonglong & 0xff00u64) << 40i32
          | (__x as libc::c_ulonglong & 0xffu64) << 56i32) as u64
      } else {
        let fresh16 = &mut __v;
        let fresh17;
        let fresh18 = __x;
        asm!("bswap ${0:q}" : "=r" (fresh17) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh16, fresh18))
                          :);
        c2rust_asm_casts::AsmCast::cast_out(fresh16, fresh18, fresh17);
      }
      __v
    }) as int64_t;
    time_passed = time(0 as *mut time_t) - written_at;
    /* Strange written_at, or lease file from old version of udhcpd
     * which had no "written_at" field? */
    if !(time_passed as u64 > (12i32 * 60i32 * 60i32) as libc::c_ulong) {
      /* NB: we do not add lease even if static_nip == lease.lease_nip.
       */
      while full_read(
        fd,
        &mut lease as *mut dyn_lease as *mut libc::c_void,
        ::std::mem::size_of::<dyn_lease>() as libc::c_ulong,
      ) as libc::c_ulong
        == ::std::mem::size_of::<dyn_lease>() as libc::c_ulong
      {
        let mut y: u32 = {
          let mut __v: libc::c_uint = 0;
          let mut __x: libc::c_uint = lease.lease_nip;
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
        };
        if !(y >= (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).start_ip
          && y <= (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).end_ip)
        {
          continue;
        }
        let mut expires: signed_leasetime_t = ({
          let mut __v: libc::c_uint = 0;
          let mut __x: libc::c_uint = lease.expires;
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
        })
        .wrapping_sub(time_passed as signed_leasetime_t as libc::c_uint)
          as signed_leasetime_t;
        let mut static_nip: u32 = 0;
        if expires <= 0i32 {
          /* We keep expired leases: add_lease() will add
           * a lease with 0 seconds remaining.
           * Fewer IP address changes this way for mass reboot scenario.
           */
          expires = 0i32
        }
        /* Check if there is a different static lease for this IP or MAC */
        static_nip = get_static_nip_by_mac(lease.lease_mac.as_mut_ptr() as *mut libc::c_void);
        if static_nip != 0 {
          continue;
        }
        if is_nip_reserved_as_static(lease.lease_nip) != 0 {
          continue;
        }
        /* NB: add_lease takes "relative time", IOW,
         * lease duration, not lease deadline. */
        if add_lease(
          lease.lease_mac.as_mut_ptr(),
          lease.lease_nip,
          expires as leasetime_t,
          lease.hostname.as_mut_ptr(),
          ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong as libc::c_int,
        )
        .is_null()
        {
          bb_error_msg(
            b"too many leases while loading %s\x00" as *const u8 as *const libc::c_char,
            file,
          );
          break;
        } else {
          i = i.wrapping_add(1)
        }
      }
      if dhcp_verbose >= 1i32 as libc::c_uint {
        bb_info_msg(b"read %d leases\x00" as *const u8 as *const libc::c_char, i);
      }
    }
  }
  close(fd);
}
/* Send a packet to a specific mac address and ip address by creating our own ip packet */
unsafe extern "C" fn send_packet_to_client(
  mut dhcp_pkt: *mut dhcp_packet,
  mut force_broadcast: libc::c_int,
) {
  let mut chaddr: *const u8 = 0 as *const u8;
  let mut ciaddr: u32 = 0;
  // Was:
  //if (force_broadcast) { /* broadcast */ }
  //else if (dhcp_pkt->ciaddr) { /* unicast to dhcp_pkt->ciaddr */ }
  //else if (dhcp_pkt->flags & htons(BROADCAST_FLAG)) { /* broadcast */ }
  //else { /* unicast to dhcp_pkt->yiaddr */ }
  // But this is wrong: yiaddr is _our_ idea what client's IP is
  // (for example, from lease file). Client may not know that,
  // and may not have UDP socket listening on that IP!
  // We should never unicast to dhcp_pkt->yiaddr!
  // dhcp_pkt->ciaddr, OTOH, comes from client's request packet,
  // and can be used.
  if force_broadcast != 0
    || (*dhcp_pkt).flags as libc::c_int
      & ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = 0x8000i32 as libc::c_ushort;
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
      != 0
    || (*dhcp_pkt).ciaddr == 0i32 as libc::c_uint
  {
    if dhcp_verbose >= 1i32 as libc::c_uint {
      bb_simple_info_msg(b"broadcasting packet to client\x00" as *const u8 as *const libc::c_char);
    }
    ciaddr = 0xffffffffu32;
    chaddr = MAC_BCAST_ADDR.as_ptr()
  } else {
    if dhcp_verbose >= 1i32 as libc::c_uint {
      bb_simple_info_msg(
        b"unicasting packet to client ciaddr\x00" as *const u8 as *const libc::c_char,
      );
    }
    ciaddr = (*dhcp_pkt).ciaddr;
    chaddr = (*dhcp_pkt).chaddr.as_mut_ptr()
  }
  udhcp_send_raw_packet(
    dhcp_pkt,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).server_nip,
    67i32,
    ciaddr,
    68i32,
    chaddr,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).ifindex,
  );
}
/* Send a packet to gateway_nip using the kernel ip stack */
unsafe extern "C" fn send_packet_to_relay(mut dhcp_pkt: *mut dhcp_packet) {
  if dhcp_verbose >= 1i32 as libc::c_uint {
    bb_simple_info_msg(b"forwarding packet to relay\x00" as *const u8 as *const libc::c_char);
  }
  udhcp_send_kernel_packet(
    dhcp_pkt,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).server_nip,
    67i32,
    (*dhcp_pkt).gateway_nip,
    67i32,
  );
}
unsafe extern "C" fn send_packet(mut dhcp_pkt: *mut dhcp_packet, mut force_broadcast: libc::c_int) {
  if (*dhcp_pkt).gateway_nip != 0 {
    send_packet_to_relay(dhcp_pkt);
  } else {
    send_packet_to_client(dhcp_pkt, force_broadcast);
  };
}
unsafe extern "C" fn send_packet_verbose(
  mut dhcp_pkt: *mut dhcp_packet,
  mut fmt: *const libc::c_char,
) {
  let mut addr: in_addr = in_addr { s_addr: 0 };
  addr.s_addr = (*dhcp_pkt).yiaddr;
  bb_info_msg(fmt, inet_ntoa(addr));
  /* send_packet emits error message itself if it detects failure */
  send_packet(dhcp_pkt, 0i32);
}
unsafe extern "C" fn init_packet(
  mut packet: *mut dhcp_packet,
  mut oldpacket: *mut dhcp_packet,
  mut type_0: libc::c_char,
) {
  /* Sets op, htype, hlen, cookie fields
   * and adds DHCP_MESSAGE_TYPE option */
  udhcp_init_header(packet, type_0);
  (*packet).xid = (*oldpacket).xid;
  memcpy(
    (*packet).chaddr.as_mut_ptr() as *mut libc::c_void,
    (*oldpacket).chaddr.as_mut_ptr() as *const libc::c_void,
    ::std::mem::size_of::<[u8; 16]>() as libc::c_ulong,
  );
  (*packet).flags = (*oldpacket).flags;
  (*packet).gateway_nip = (*oldpacket).gateway_nip;
  (*packet).ciaddr = (*oldpacket).ciaddr;
  udhcp_add_simple_option(
    packet,
    0x36i32 as u8,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).server_nip,
  );
}
/* Fill options field, siaddr_nip, and sname and boot_file fields.
 * TODO: teach this code to use overload option.
 */
unsafe extern "C" fn add_server_options(mut packet: *mut dhcp_packet) {
  let mut config_opts: *mut option_set = 0 as *mut option_set;
  let mut client_hostname_opt: *mut u8 = 0 as *mut u8;
  client_hostname_opt = 0 as *mut u8;
  if (*packet).yiaddr != 0 {
    /* if we aren't from send_inform()... */
    let mut st_lease: *mut static_lease =
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).static_leases;
    while !st_lease.is_null() {
      if (*st_lease).nip == (*packet).yiaddr {
        if *(*st_lease).opt.as_mut_ptr().offset(0) as libc::c_int != 0i32 {
          client_hostname_opt = (*st_lease).opt.as_mut_ptr()
        }
        break;
      } else {
        st_lease = (*st_lease).next
      }
    }
  }
  config_opts = (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).options;
  while !config_opts.is_null() {
    if *(*config_opts).data.offset(0) as libc::c_int != 0x33i32 {
      /* ^^^^
       * DHCP_LEASE_TIME is already filled, or in case of
       * send_inform(), should not be filled at all.
       */
      if *(*config_opts).data.offset(0) as libc::c_int != 0xci32 || client_hostname_opt.is_null() {
        /* Why "!client_hostname_opt":
         * add hostname only if client has no hostname
         * on its static lease line.
         * (Not that "opt hostname HOST"
         * makes much sense in udhcpd.conf,
         * that'd give all clients the same hostname,
         * but it's a valid configuration).
         */
        udhcp_add_binary_option(packet, (*config_opts).data);
      }
    }
    config_opts = (*config_opts).next
  }
  if !client_hostname_opt.is_null() {
    udhcp_add_binary_option(packet, client_hostname_opt);
  }
  (*packet).siaddr_nip = (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).siaddr_nip;
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t))
    .sname
    .is_null()
  {
    strncpy(
      (*packet).sname.as_mut_ptr() as *mut libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).sname,
      (::std::mem::size_of::<[u8; 64]>() as libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong),
    );
  }
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t))
    .boot_file
    .is_null()
  {
    strncpy(
      (*packet).file.as_mut_ptr() as *mut libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).boot_file,
      (::std::mem::size_of::<[u8; 128]>() as libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong),
    );
  };
}
unsafe extern "C" fn select_lease_time(mut packet: *mut dhcp_packet) -> u32 {
  let mut lease_time_sec: u32 =
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).max_lease_sec;
  let mut lease_time_opt: *mut u8 = udhcp_get_option32(packet, 0x33i32);
  if !lease_time_opt.is_null() {
    lease_time_sec = *(lease_time_opt as *mut bb__aliased_u32);
    lease_time_sec = {
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = lease_time_sec;
      if false {
        __v = (__x & 0xff000000u32) >> 24i32
          | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
          | (__x & 0xff00i32 as libc::c_uint) << 8i32
          | (__x & 0xffi32 as libc::c_uint) << 24i32
      } else {
        let fresh28 = &mut __v;
        let fresh29;
        let fresh30 = __x;
        asm!("bswap $0" : "=r" (fresh29) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh28, fresh30))
                          :);
        c2rust_asm_casts::AsmCast::cast_out(fresh28, fresh30, fresh29);
      }
      __v
    };
    if lease_time_sec > (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).max_lease_sec {
      lease_time_sec = (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).max_lease_sec
    }
    if lease_time_sec < (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).min_lease_sec {
      lease_time_sec = (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).min_lease_sec
    }
  }
  return lease_time_sec;
}
/* We got a DHCP DISCOVER. Send an OFFER. */
/* NOINLINE: limit stack usage in caller */
#[inline(never)]
unsafe extern "C" fn send_offer(
  mut oldpacket: *mut dhcp_packet,
  mut static_lease_nip: u32,
  mut lease: *mut dyn_lease,
  mut requested_nip: u32,
  mut arpping_ms: libc::c_uint,
) {
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
  let mut lease_time_sec: u32 = 0;
  init_packet(&mut packet, oldpacket, 2i32 as libc::c_char);
  /* If it is a static lease, use its IP */
  packet.yiaddr = static_lease_nip;
  /* Else: */
  if static_lease_nip == 0 {
    /* We have no static lease for client's chaddr */
    let mut p_host_name: *const libc::c_char = 0 as *const libc::c_char;
    if !lease.is_null() {
      /* We have a dynamic lease for client's chaddr.
       * Reuse its IP (even if lease is expired).
       * Note that we ignore requested IP in this case.
       */
      packet.yiaddr = (*lease).lease_nip
    } else if requested_nip != 0i32 as libc::c_uint
      && ({
        let mut __v: libc::c_uint = 0;
        let mut __x: libc::c_uint = requested_nip;
        if false {
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
      }) >= (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).start_ip
      && ({
        let mut __v: libc::c_uint = 0;
        let mut __x: libc::c_uint = requested_nip;
        if false {
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
      }) <= (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).end_ip
      && {
        lease = find_lease_by_nip(requested_nip);
        (lease.is_null()) || is_expired_lease(lease) != 0
      }
    {
      packet.yiaddr = requested_nip
    } else {
      /* Or: if client has requested an IP */
      /* Otherwise, find a free IP */
      packet.yiaddr = find_free_or_expired_nip((*oldpacket).chaddr.as_mut_ptr(), arpping_ms)
    }
    if packet.yiaddr == 0 {
      bb_simple_error_msg(
        b"no free IP addresses. OFFER abandoned\x00" as *const u8 as *const libc::c_char,
      );
      return;
    }
    /* Reserve the IP for a short time hoping to get DHCPREQUEST soon */
    p_host_name = udhcp_get_option(oldpacket, 0xci32) as *const libc::c_char;
    lease = add_lease(
      packet.chaddr.as_mut_ptr(),
      packet.yiaddr,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).offer_time,
      p_host_name,
      if !p_host_name.is_null() {
        *p_host_name.offset((1i32 - 2i32) as isize) as libc::c_uchar as libc::c_int
      } else {
        0i32
      },
    );
    if lease.is_null() {
      bb_simple_error_msg(
        b"no free IP addresses. OFFER abandoned\x00" as *const u8 as *const libc::c_char,
      );
      return;
    }
  }
  lease_time_sec = select_lease_time(oldpacket);
  udhcp_add_simple_option(&mut packet, 0x33i32 as u8, {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = lease_time_sec;
    if false {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh37 = &mut __v;
      let fresh38;
      let fresh39 = __x;
      asm!("bswap $0" : "=r" (fresh38) : "0"
                                          (c2rust_asm_casts::AsmCast::cast_in(fresh37, fresh39))
                                          :);
      c2rust_asm_casts::AsmCast::cast_out(fresh37, fresh39, fresh38);
    }
    __v
  });
  add_server_options(&mut packet);
  /* send_packet emits error message itself if it detects failure */
  send_packet_verbose(
    &mut packet,
    b"sending OFFER to %s\x00" as *const u8 as *const libc::c_char,
  );
}
/* NOINLINE: limit stack usage in caller */
#[inline(never)]
unsafe extern "C" fn send_NAK(mut oldpacket: *mut dhcp_packet) {
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
  init_packet(&mut packet, oldpacket, 6i32 as libc::c_char);
  if dhcp_verbose >= 1i32 as libc::c_uint {
    bb_info_msg(
      b"sending %s\x00" as *const u8 as *const libc::c_char,
      b"NAK\x00" as *const u8 as *const libc::c_char,
    );
  }
  send_packet(&mut packet, 1i32);
}
/* NOINLINE: limit stack usage in caller */
#[inline(never)]
unsafe extern "C" fn send_ACK(mut oldpacket: *mut dhcp_packet, mut yiaddr: u32) {
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
  let mut lease_time_sec: u32 = 0;
  let mut p_host_name: *const libc::c_char = 0 as *const libc::c_char;
  init_packet(&mut packet, oldpacket, 5i32 as libc::c_char);
  packet.yiaddr = yiaddr;
  lease_time_sec = select_lease_time(oldpacket);
  udhcp_add_simple_option(&mut packet, 0x33i32 as u8, {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = lease_time_sec;
    if false {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh40 = &mut __v;
      let fresh41;
      let fresh42 = __x;
      asm!("bswap $0" : "=r" (fresh41) : "0"
                                          (c2rust_asm_casts::AsmCast::cast_in(fresh40, fresh42))
                                          :);
      c2rust_asm_casts::AsmCast::cast_out(fresh40, fresh42, fresh41);
    }
    __v
  });
  add_server_options(&mut packet);
  send_packet_verbose(
    &mut packet,
    b"sending ACK to %s\x00" as *const u8 as *const libc::c_char,
  );
  p_host_name = udhcp_get_option(oldpacket, 0xci32) as *const libc::c_char;
  add_lease(
    packet.chaddr.as_mut_ptr(),
    packet.yiaddr,
    lease_time_sec,
    p_host_name,
    if !p_host_name.is_null() {
      *p_host_name.offset((1i32 - 2i32) as isize) as libc::c_uchar as libc::c_int
    } else {
      0i32
    },
  );
  /* rewrite the file with leases at every new acceptance */
  write_leases();
}
/* NOINLINE: limit stack usage in caller */
#[inline(never)]
unsafe extern "C" fn send_inform(mut oldpacket: *mut dhcp_packet) {
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
  /* "If a client has obtained a network address through some other means
   * (e.g., manual configuration), it may use a DHCPINFORM request message
   * to obtain other local configuration parameters.  Servers receiving a
   * DHCPINFORM message construct a DHCPACK message with any local
   * configuration parameters appropriate for the client without:
   * allocating a new address, checking for an existing binding, filling
   * in 'yiaddr' or including lease time parameters.  The servers SHOULD
   * unicast the DHCPACK reply to the address given in the 'ciaddr' field
   * of the DHCPINFORM message.
   * ...
   * The server responds to a DHCPINFORM message by sending a DHCPACK
   * message directly to the address given in the 'ciaddr' field
   * of the DHCPINFORM message.  The server MUST NOT send a lease
   * expiration time to the client and SHOULD NOT fill in 'yiaddr'."
   */
  //TODO: do a few sanity checks: is ciaddr set?
  //Better yet: is ciaddr == IP source addr?
  init_packet(&mut packet, oldpacket, 5i32 as libc::c_char);
  add_server_options(&mut packet);
  send_packet(&mut packet, 0i32);
  // or maybe? send_packet_verbose(&packet, "sending ACK to %s");
}
#[no_mangle]
pub unsafe extern "C" fn udhcpd_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut server_socket: libc::c_int = -1i32;
  let mut retval: libc::c_int = 0;
  let mut state: *mut u8 = 0 as *mut u8;
  let mut timeout_end: libc::c_uint = 0;
  let mut num_ips: libc::c_uint = 0;
  let mut opt: libc::c_uint = 0;
  let mut option: *mut option_set = 0 as *mut option_set;
  let mut str_I: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  str_I = str_I;
  let mut str_a: *const libc::c_char = b"2000\x00" as *const u8 as *const libc::c_char;
  let mut arpping_ms: libc::c_uint = 0;
  /* Make sure fd 0,1,2 are open */
  /* Setup the signal pipe on fds 3,4 - must be before openlog() */
  udhcp_sp_setup();
  opt = getopt32(
    argv,
    b"^fSI:va:\x00vv\x00" as *const u8 as *const libc::c_char,
    &mut str_I as *mut *mut libc::c_char,
    &mut str_a as *mut *const libc::c_char,
    &mut dhcp_verbose as *mut libc::c_uint,
  );
  if opt & 1i32 as libc::c_uint == 0 {
    /* no -f */
    bb_daemonize_or_rexec(0i32);
    logmode = LOGMODE_NONE as libc::c_int as smallint
  }
  /* update argv after the possible vfork+exec in daemonize */
  argv = argv.offset(optind as isize);
  if opt & 2i32 as libc::c_uint != 0 {
    /* -S */
    openlog(applet_name, 0x1i32, 3i32 << 3i32);
    logmode = (logmode as libc::c_int | LOGMODE_SYSLOG as libc::c_int) as smallint
  }
  if opt & 4i32 as libc::c_uint != 0 {
    /* -I */
    let mut lsa: *mut len_and_sockaddr = xhost_and_af2sockaddr(str_I, 0i32, 2i32 as sa_family_t);
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).server_nip =
      (*lsa).u.sin.sin_addr.s_addr;
    free(lsa as *mut libc::c_void);
  }
  arpping_ms = xatou(str_a);
  /* Would rather not do read_config before daemonization -
   * otherwise NOMMU machines will parse config twice */
  read_config(if !(*argv.offset(0)).is_null() {
    *argv.offset(0)
  } else {
    b"/etc/udhcpd.conf\x00" as *const u8 as *const libc::c_char
  });
  /* prevent poll timeout overflow */
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).auto_time
    > (2147483647i32 / 1000i32) as libc::c_uint
  {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).auto_time =
      (2147483647i32 / 1000i32) as u32
  }
  /* Create pidfile */
  write_pidfile((*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).pidfile);
  /* if (!..) bb_perror_msg("can't create pidfile %s", pidfile); */
  bb_simple_info_msg(b"started, v1.32.0.git\x00" as *const u8 as *const libc::c_char);
  option = udhcp_find_option(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).options,
    0x33i32 as u8,
  );
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).max_lease_sec =
    (60i32 * 60i32 * 24i32 * 10i32) as u32;
  if !option.is_null() {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).max_lease_sec =
      *((*option).data.offset(2) as *mut bb__aliased_u32);
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).max_lease_sec = {
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint =
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).max_lease_sec;
      if false {
        __v = (__x & 0xff000000u32) >> 24i32
          | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
          | (__x & 0xff00i32 as libc::c_uint) << 8i32
          | (__x & 0xffi32 as libc::c_uint) << 24i32
      } else {
        let fresh43 = &mut __v;
        let fresh44;
        let fresh45 = __x;
        asm!("bswap $0" : "=r" (fresh44) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh43, fresh45))
                          :);
        c2rust_asm_casts::AsmCast::cast_out(fresh43, fresh45, fresh44);
      }
      __v
    }
  }
  /* Sanity check */
  num_ips = (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t))
    .end_ip
    .wrapping_sub((*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).start_ip)
    .wrapping_add(1i32 as libc::c_uint);
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).max_leases > num_ips {
    bb_error_msg(
      b"max_leases=%u is too big, setting to %u\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).max_leases,
      num_ips,
    );
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).max_leases = num_ips
  }
  /* this sets g_leases */
  let ref mut fresh46 =
    *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
      as *mut *mut globals);
  *fresh46 = xzalloc(
    ((*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).max_leases as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<dyn_lease>() as libc::c_ulong),
  ) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  read_leases((*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).lease_file);
  if udhcp_read_interface(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).interface,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).ifindex,
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).server_nip == 0i32 as libc::c_uint
    {
      &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).server_nip
    } else {
      0 as *mut u32
    },
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t))
      .server_mac
      .as_mut_ptr(),
  ) != 0
  {
    retval = 1i32
  } else {
    'c_13480: loop
    /* why not just reset the timeout, eh */
    {
      timeout_end = monotonic_sec()
        .wrapping_add((*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).auto_time);
      loop
      /* loop until universe collapses */
      {
        let mut pfds: [pollfd; 2] = [pollfd {
          fd: 0,
          events: 0,
          revents: 0,
        }; 2];
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
        let mut bytes: libc::c_int = 0;
        let mut tv: libc::c_int = 0;
        let mut server_id_opt: *mut u8 = 0 as *mut u8;
        let mut requested_ip_opt: *mut u8 = 0 as *mut u8;
        let mut requested_nip: u32 = 0;
        let mut static_lease_nip: u32 = 0;
        let mut lease: *mut dyn_lease = 0 as *mut dyn_lease;
        let mut fake_lease: dyn_lease = dyn_lease {
          expires: 0,
          lease_nip: 0,
          lease_mac: [0; 6],
          hostname: [0; 20],
          pad: [0; 2],
        };
        if server_socket < 0i32 {
          server_socket = udhcp_listen_socket(
            67i32,
            (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).interface,
          )
        }
        udhcp_sp_fd_set(pfds.as_mut_ptr(), server_socket);
        loop {
          tv = -1i32;
          if (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).auto_time != 0 {
            tv = timeout_end.wrapping_sub(monotonic_sec()) as libc::c_int;
            if tv <= 0i32 {
              current_block = 16181713839514877146;
              break;
            }
            tv *= 1000i32
          }
          /* Block here waiting for either signal or packet */
          retval = poll(pfds.as_mut_ptr(), 2i32 as nfds_t, tv);
          if retval <= 0i32 {
            if retval == 0i32 {
              current_block = 16181713839514877146;
              break;
            }
            if *bb_errno == 4i32 {
              continue;
            }
            /* < 0 and not EINTR: should not happen */
            bb_simple_perror_msg_and_die(b"poll\x00" as *const u8 as *const libc::c_char);
          } else if pfds[0].revents != 0 {
            current_block = 7494008139977416618;
            break;
          } else {
            current_block = 12961834331865314435;
            break;
          }
        }
        match current_block {
          7494008139977416618 => match udhcp_sp_read() {
            10 => {
              current_block = 15734667288518933029;
              match current_block {
                14059375994042912985 => {
                  bb_info_msg(
                    b"received %s\x00" as *const u8 as *const libc::c_char,
                    b"SIGTERM\x00" as *const u8 as *const libc::c_char,
                  );
                  write_leases();
                  break;
                }
                _ => {
                  bb_info_msg(
                    b"received %s\x00" as *const u8 as *const libc::c_char,
                    b"SIGUSR1\x00" as *const u8 as *const libc::c_char,
                  );
                  write_leases();
                  continue 'c_13480;
                }
              }
            }
            15 => {
              current_block = 14059375994042912985;
              match current_block {
                14059375994042912985 => {
                  bb_info_msg(
                    b"received %s\x00" as *const u8 as *const libc::c_char,
                    b"SIGTERM\x00" as *const u8 as *const libc::c_char,
                  );
                  write_leases();
                  break;
                }
                _ => {
                  bb_info_msg(
                    b"received %s\x00" as *const u8 as *const libc::c_char,
                    b"SIGUSR1\x00" as *const u8 as *const libc::c_char,
                  );
                  write_leases();
                  continue 'c_13480;
                }
              }
            }
            _ => {}
          },
          16181713839514877146 => {
            write_leases();
            continue 'c_13480;
          }
          _ => {}
        }
        /* Is it a packet? */
        if pfds[1].revents == 0 {
          continue; /* no */
        }
        /* Note: we do not block here, we block on poll() instead.
         * Blocking here would prevent SIGTERM from working:
         * socket read inside this call is restarted on caught signals.
         */
        bytes = udhcp_recv_kernel_packet(&mut packet, server_socket);
        if bytes < 0i32 {
          /* bytes can also be -2 ("bad packet data") */
          if bytes == -1i32 && *bb_errno != 4i32 {
            if dhcp_verbose >= 1i32 as libc::c_uint {
              bb_info_msg(
                b"read error: %m, reopening socket\x00" as *const u8 as *const libc::c_char,
              );
            }
            close(server_socket);
            server_socket = -1i32
          }
        } else if packet.hlen as libc::c_int != 6i32 {
          bb_info_msg(
            b"MAC length != 6%s\x00" as *const u8 as *const libc::c_char,
            b", ignoring packet\x00" as *const u8 as *const libc::c_char,
          );
        } else if packet.op as libc::c_int != 1i32 {
          bb_info_msg(
            b"not a REQUEST%s\x00" as *const u8 as *const libc::c_char,
            b", ignoring packet\x00" as *const u8 as *const libc::c_char,
          );
        } else {
          state = udhcp_get_option(&mut packet, 0x35i32);
          if state.is_null()
            || (*state.offset(0) as libc::c_int) < 1i32
            || *state.offset(0) as libc::c_int > 8i32
          {
            bb_info_msg(
              b"no or bad message type option%s\x00" as *const u8 as *const libc::c_char,
              b", ignoring packet\x00" as *const u8 as *const libc::c_char,
            );
          } else {
            /* Get SERVER_ID if present */
            server_id_opt = udhcp_get_option32(&mut packet, 0x36i32);
            if !server_id_opt.is_null() {
              let mut server_id_network_order: u32 = 0;
              server_id_network_order = *(server_id_opt as *mut bb__aliased_u32);
              if server_id_network_order
                != (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).server_nip
              {
                /* client talks to somebody else */
                if dhcp_verbose >= 1i32 as libc::c_uint {
                  bb_info_msg(
                    b"server ID doesn\'t match%s\x00" as *const u8 as *const libc::c_char,
                    b", ignoring\x00" as *const u8 as *const libc::c_char,
                  );
                }
                continue;
              }
            }
            /* Look for a static/dynamic lease */
            static_lease_nip =
              get_static_nip_by_mac(&mut packet.chaddr as *mut [u8; 16] as *mut libc::c_void);
            if static_lease_nip != 0 {
              bb_info_msg(
                b"found static lease: %x\x00" as *const u8 as *const libc::c_char,
                static_lease_nip,
              );
              memcpy(
                &mut fake_lease.lease_mac as *mut [u8; 6] as *mut libc::c_void,
                &mut packet.chaddr as *mut [u8; 16] as *const libc::c_void,
                6i32 as libc::c_ulong,
              );
              fake_lease.lease_nip = static_lease_nip;
              fake_lease.expires = 0i32 as leasetime_t;
              lease = &mut fake_lease
            } else {
              lease = find_lease_by_mac(packet.chaddr.as_mut_ptr())
            }
            /* Get REQUESTED_IP if present */
            requested_nip = 0i32 as u32;
            requested_ip_opt = udhcp_get_option32(&mut packet, 0x32i32);
            if !requested_ip_opt.is_null() {
              requested_nip = *(requested_ip_opt as *mut bb__aliased_u32)
            }
            let mut current_block_125: u64;
            match *state.offset(0) as libc::c_int {
              1 => {
                if dhcp_verbose >= 1i32 as libc::c_uint {
                  bb_info_msg(
                    b"received %s\x00" as *const u8 as *const libc::c_char,
                    b"DISCOVER\x00" as *const u8 as *const libc::c_char,
                  );
                }
                send_offer(
                  &mut packet,
                  static_lease_nip,
                  lease,
                  requested_nip,
                  arpping_ms,
                );
              }
              3 => {
                if dhcp_verbose >= 1i32 as libc::c_uint {
                  bb_info_msg(
                    b"received %s\x00" as *const u8 as *const libc::c_char,
                    b"REQUEST\x00" as *const u8 as *const libc::c_char,
                  );
                }
                /* RFC 2131:

                o DHCPREQUEST generated during SELECTING state:

                   Client inserts the address of the selected server in 'server
                   identifier', 'ciaddr' MUST be zero, 'requested IP address' MUST be
                   filled in with the yiaddr value from the chosen DHCPOFFER.

                   Note that the client may choose to collect several DHCPOFFER
                   messages and select the "best" offer.  The client indicates its
                   selection by identifying the offering server in the DHCPREQUEST
                   message.  If the client receives no acceptable offers, the client
                   may choose to try another DHCPDISCOVER message.  Therefore, the
                   servers may not receive a specific DHCPREQUEST from which they can
                   decide whether or not the client has accepted the offer.

                o DHCPREQUEST generated during INIT-REBOOT state:

                   'server identifier' MUST NOT be filled in, 'requested IP address'
                   option MUST be filled in with client's notion of its previously
                   assigned address. 'ciaddr' MUST be zero. The client is seeking to
                   verify a previously allocated, cached configuration. Server SHOULD
                   send a DHCPNAK message to the client if the 'requested IP address'
                   is incorrect, or is on the wrong network.

                   Determining whether a client in the INIT-REBOOT state is on the
                   correct network is done by examining the contents of 'giaddr', the
                   'requested IP address' option, and a database lookup. If the DHCP
                   server detects that the client is on the wrong net (i.e., the
                   result of applying the local subnet mask or remote subnet mask (if
                   'giaddr' is not zero) to 'requested IP address' option value
                   doesn't match reality), then the server SHOULD send a DHCPNAK
                   message to the client.

                   If the network is correct, then the DHCP server should check if
                   the client's notion of its IP address is correct. If not, then the
                   server SHOULD send a DHCPNAK message to the client. If the DHCP
                   server has no record of this client, then it MUST remain silent,
                   and MAY output a warning to the network administrator. This
                   behavior is necessary for peaceful coexistence of non-
                   communicating DHCP servers on the same wire.

                   If 'giaddr' is 0x0 in the DHCPREQUEST message, the client is on
                   the same subnet as the server.  The server MUST broadcast the
                   DHCPNAK message to the 0xffffffff broadcast address because the
                   client may not have a correct network address or subnet mask, and
                   the client may not be answering ARP requests.

                   If 'giaddr' is set in the DHCPREQUEST message, the client is on a
                   different subnet.  The server MUST set the broadcast bit in the
                   DHCPNAK, so that the relay agent will broadcast the DHCPNAK to the
                   client, because the client may not have a correct network address
                   or subnet mask, and the client may not be answering ARP requests.

                o DHCPREQUEST generated during RENEWING state:

                   'server identifier' MUST NOT be filled in, 'requested IP address'
                   option MUST NOT be filled in, 'ciaddr' MUST be filled in with
                   client's IP address. In this situation, the client is completely
                   configured, and is trying to extend its lease. This message will
                   be unicast, so no relay agents will be involved in its
                   transmission.  Because 'giaddr' is therefore not filled in, the
                   DHCP server will trust the value in 'ciaddr', and use it when
                   replying to the client.

                   A client MAY choose to renew or extend its lease prior to T1.  The
                   server may choose not to extend the lease (as a policy decision by
                   the network administrator), but should return a DHCPACK message
                   regardless.

                o DHCPREQUEST generated during REBINDING state:

                   'server identifier' MUST NOT be filled in, 'requested IP address'
                   option MUST NOT be filled in, 'ciaddr' MUST be filled in with
                   client's IP address. In this situation, the client is completely
                   configured, and is trying to extend its lease. This message MUST
                   be broadcast to the 0xffffffff IP broadcast address.  The DHCP
                   server SHOULD check 'ciaddr' for correctness before replying to
                   the DHCPREQUEST.

                   The DHCPREQUEST from a REBINDING client is intended to accommodate
                   sites that have multiple DHCP servers and a mechanism for
                   maintaining consistency among leases managed by multiple servers.
                   A DHCP server MAY extend a client's lease only if it has local
                   administrative authority to do so.
                */
                if requested_ip_opt.is_null() {
                  requested_nip = packet.ciaddr;
                  if requested_nip == 0i32 as libc::c_uint {
                    if dhcp_verbose >= 1i32 as libc::c_uint {
                      bb_info_msg(
                        b"no requested IP and no ciaddr%s\x00" as *const u8 as *const libc::c_char,
                        b", ignoring\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    current_block_125 = 18316056106135622027;
                  } else {
                    current_block_125 = 7923086311623215889;
                  }
                } else {
                  current_block_125 = 7923086311623215889;
                }
                match current_block_125 {
                  18316056106135622027 => {}
                  _ => {
                    if !lease.is_null() && requested_nip == (*lease).lease_nip {
                      /* client requested or configured IP matches the lease.
                       * ACK it, and bump lease expiration time. */
                      send_ACK(&mut packet, (*lease).lease_nip);
                    } else if !server_id_opt.is_null() || !requested_ip_opt.is_null() {
                      /* No lease for this MAC, or lease IP != requested IP */
                      /* else: client is in RENEWING or REBINDING, do not answer */
                      /* client is in INIT-REBOOT state */
                      /* "No, we don't have this IP for you" */
                      send_NAK(&mut packet);
                    }
                  }
                }
              }
              4 => {
                /* RFC 2131:
                 * "If the server receives a DHCPDECLINE message,
                 * the client has discovered through some other means
                 * that the suggested network address is already
                 * in use. The server MUST mark the network address
                 * as not available and SHOULD notify the local
                 * sysadmin of a possible configuration problem."
                 *
                 * SERVER_ID must be present,
                 * REQUESTED_IP must be present,
                 * chaddr must be filled in,
                 * ciaddr must be 0 (we do not check this)
                 */
                if dhcp_verbose >= 1i32 as libc::c_uint {
                  bb_info_msg(
                    b"received %s\x00" as *const u8 as *const libc::c_char,
                    b"DECLINE\x00" as *const u8 as *const libc::c_char,
                  );
                }
                if !server_id_opt.is_null()
                  && !requested_ip_opt.is_null()
                  && !lease.is_null()
                  && requested_nip == (*lease).lease_nip
                {
                  memset(
                    (*lease).lease_mac.as_mut_ptr() as *mut libc::c_void,
                    0i32,
                    ::std::mem::size_of::<[u8; 6]>() as libc::c_ulong,
                  );
                  (*lease).expires = (time(0 as *mut time_t)
                    + (*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).decline_time
                      as libc::c_long) as leasetime_t
                }
              }
              7 => {
                /* "Upon receipt of a DHCPRELEASE message, the server
                 * marks the network address as not allocated."
                 *
                 * SERVER_ID must be present,
                 * REQUESTED_IP must not be present (we do not check this),
                 * chaddr must be filled in,
                 * ciaddr must be filled in
                 */
                if dhcp_verbose >= 1i32 as libc::c_uint {
                  bb_info_msg(
                    b"received %s\x00" as *const u8 as *const libc::c_char,
                    b"RELEASE\x00" as *const u8 as *const libc::c_char,
                  );
                }
                if !server_id_opt.is_null()
                  && !lease.is_null()
                  && packet.ciaddr == (*lease).lease_nip
                {
                  (*lease).expires = time(0 as *mut time_t) as leasetime_t
                }
              }
              8 => {
                if dhcp_verbose >= 1i32 as libc::c_uint {
                  bb_info_msg(
                    b"received %s\x00" as *const u8 as *const libc::c_char,
                    b"INFORM\x00" as *const u8 as *const libc::c_char,
                  );
                }
                send_inform(&mut packet);
              }
              _ => {}
            }
          }
        }
      }
      retval = 0i32;
      break;
    }
  }
  /*if (server_data.pidfile) - server_data.pidfile is never NULL */
  if wrote_pidfile != 0 {
    unlink((*(bb_common_bufsiz1.as_mut_ptr() as *mut server_data_t)).pidfile);
  }
  return retval;
}

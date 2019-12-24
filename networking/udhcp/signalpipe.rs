use crate::libbb::ptr_to_globals::bb_errno;

use libc;
extern "C" {
  #[no_mangle]
  fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;

}

use crate::librb::fd_pair;
use crate::librb::size_t;
use libc::pollfd;
use libc::ssize_t;
unsafe extern "C" fn signal_handler(mut sig: libc::c_int) {
  let mut sv: libc::c_int = *bb_errno; /* use char, avoid dealing with partial writes */
  let mut ch: libc::c_uchar = sig as libc::c_uchar;
  if write(
    4i32,
    &mut ch as *mut libc::c_uchar as *const libc::c_void,
    1i32 as size_t,
  ) != 1
  {
    crate::libbb::perror_msg::bb_simple_perror_msg(
      b"can\'t send signal\x00" as *const u8 as *const libc::c_char,
    );
  }
  *bb_errno = sv;
}
/* Call this before doing anything else. Sets up the socket pair
 * and installs the signal handler */
#[no_mangle]
pub unsafe extern "C" fn udhcp_sp_setup() {
  let mut signal_pipe: fd_pair = fd_pair { rd: 0, wr: 0 };
  /* All callers also want this, so... */
  crate::libbb::vfork_daemon_rexec::bb_sanitize_stdio();
  /* was socketpair, but it needs AF_UNIX in kernel */
  crate::libbb::xfuncs_printf::xpipe(&mut signal_pipe.rd);
  /* usually we get fds 3 and 4, but if we get higher ones... */
  if signal_pipe.rd != 3i32 {
    crate::libbb::xfuncs_printf::xmove_fd(signal_pipe.rd, 3i32);
  }
  if signal_pipe.wr != 4i32 {
    crate::libbb::xfuncs_printf::xmove_fd(signal_pipe.wr, 4i32);
  }
  crate::libbb::xfuncs::close_on_exec_on(3i32);
  crate::libbb::xfuncs::close_on_exec_on(4i32);
  crate::libbb::xfuncs::ndelay_on(3i32);
  crate::libbb::xfuncs::ndelay_on(4i32);
  crate::libbb::signals::bb_signals(
    0 + (1i32 << 10i32) + (1i32 << 12i32) + (1i32 << 15i32),
    Some(signal_handler as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
}
/* Quick little function to setup the pfds.
 * Limited in that you can only pass one extra fd.
 */
#[no_mangle]
pub unsafe extern "C" fn udhcp_sp_fd_set(mut pfds: *mut pollfd, mut extra_fd: libc::c_int) {
  (*pfds.offset(0)).fd = 3i32;
  (*pfds.offset(0)).events = 0x1i32 as libc::c_short;
  (*pfds.offset(1)).fd = -1i32;
  if extra_fd >= 0 {
    crate::libbb::xfuncs::close_on_exec_on(extra_fd);
    (*pfds.offset(1)).fd = extra_fd;
    (*pfds.offset(1)).events = 0x1i32 as libc::c_short
  }
  /* this simplifies "is extra_fd ready?" tests elsewhere: */
  (*pfds.offset(1)).revents = 0 as libc::c_short;
}

/*
 * Russ Dill <Russ.Dill@asu.edu> September 2001
 * Rewritten by Vladimir Oleynik <dzo@simtreas.ru> (C) 2003
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* six all-ones */
/* ** DHCP packet ***/
/* DHCP protocol. See RFC 2131 */
//TODO: rename ciaddr/yiaddr/chaddr
/* BOOTREQUEST or BOOTREPLY */
/* hardware address type. 1 = 10mb ethernet */
/* hardware address length */
/* used by relay agents only */
/* unique id */
/* elapsed since client began acquisition/renewal */
/* only one flag so far: */
/* "I need broadcast replies" */
/* client IP (if client is in BOUND, RENEW or REBINDING state) */
/* 'your' (client) IP address */
/* IP address of next server to use in bootstrap, returned in DHCPOFFER, DHCPACK by server */
/* aka 'giaddr': relay agent IP address */
/* link-layer client hardware address (MAC) */
/* server host name (ASCIZ) */
/* boot file name (ASCIZ) */
/* fixed first four option bytes (99,130,83,99 dec) */
/* Let's see whether compiler understood us right */
/* ** Options ***/
/* Opts of STRING_HOST type will be sanitized before they are passed
 * to udhcpc script's environment: */
//	OPTION_BOOLEAN,
//	OPTION_S16,
/* RFC1035 compressed domain name list */
/* Client requests this option by default */
/* There can be a list of 1 or more of these */
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
/* Read a signal from the signal pipe. Returns 0 if there is
 * no signal, -1 on error (and sets errno appropriately), and
 * your signal on success */
#[no_mangle]
pub unsafe extern "C" fn udhcp_sp_read() -> libc::c_int {
  let mut sig: libc::c_uchar = 0;
  /* Can't block here, fd is in nonblocking mode */
  if crate::libbb::read::safe_read(
    3i32,
    &mut sig as *mut libc::c_uchar as *mut libc::c_void,
    1i32 as size_t,
  ) != 1
  {
    return 0;
  }
  return sig as libc::c_int;
}

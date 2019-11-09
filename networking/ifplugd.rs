use crate::librb::size_t;
use crate::librb::smallint;
use libc;
use libc::unlink;

use libc::free;
use libc::pid_t;
use libc::ssize_t;

extern "C" {

  #[no_mangle]
  fn recv(
    __fd: libc::c_int,
    __buf: *mut libc::c_void,
    __n: size_t,
    __flags: libc::c_int,
  ) -> ssize_t;
  #[no_mangle]
  fn kill(__pid: pid_t, __sig: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn putenv(__string: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn setenv(
    __name: *const libc::c_char,
    __value: *const libc::c_char,
    __replace: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn monotonic_sec() -> libc::c_uint;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn bb_signals(sigs: libc::c_int, f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>);
  #[no_mangle]
  static mut bb_got_signal: smallint;
  #[no_mangle]
  fn record_signo(signo: libc::c_int);
  #[no_mangle]
  fn bb_unsetenv_and_free(key: *mut libc::c_char);
  #[no_mangle]
  fn xsocket(domain: libc::c_int, type_0: libc::c_int, protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn create_and_bind_to_netlink(
    proto: libc::c_int,
    grp: libc::c_int,
    rcvbuf: libc::c_uint,
  ) -> libc::c_int;
  #[no_mangle]
  fn strncpy_IFNAMSIZ(dst: *mut libc::c_char, src: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn open_read_close(
    filename: *const libc::c_char,
    buf: *mut libc::c_void,
    maxsz: size_t,
  ) -> ssize_t;
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
  fn spawn_and_wait(argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn bb_daemonize_or_rexec(flags: libc::c_int);
  #[no_mangle]
  static mut wrote_pidfile: smallint;
  #[no_mangle]
  fn write_pidfile(path: *const libc::c_char);
  #[no_mangle]
  static mut logmode: smallint;
  #[no_mangle]
  fn xfunc_die() -> !;
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_info_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_info_msg(s: *const libc::c_char);
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  static bb_banner: [libc::c_char; 0];
  #[no_mangle]
  static ptr_to_globals: *mut globals;
  #[no_mangle]
  fn openlog(__ident: *const libc::c_char, __option: libc::c_int, __facility: libc::c_int);
}

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
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
  pub fd: libc::c_int,
  pub events: libc::c_short,
  pub revents: libc::c_short,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const DAEMON_ONLY_SANITIZE: C2RustUnnamed_0 = 8;
pub const DAEMON_CLOSE_EXTRA_FDS: C2RustUnnamed_0 = 4;
pub const DAEMON_DEVNULL_STDIO: C2RustUnnamed_0 = 2;
pub const DAEMON_CHDIR_ROOT: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const LOGMODE_BOTH: C2RustUnnamed_1 = 3;
pub const LOGMODE_SYSLOG: C2RustUnnamed_1 = 2;
pub const LOGMODE_STDIO: C2RustUnnamed_1 = 1;
pub const LOGMODE_NONE: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub iface_last_status: smallint,
  pub iface_prev_status: smallint,
  pub iface_exists: smallint,
  pub api_method_num: smallint,
  pub poll_time: libc::c_uint,
  pub delay_up: libc::c_uint,
  pub delay_down: libc::c_uint,
  pub iface: *const libc::c_char,
  pub api_mode: *const libc::c_char,
  pub script_name: *const libc::c_char,
  pub extra_arg: *const libc::c_char,
}
pub type __u8 = libc::c_uchar;
pub type __s16 = libc::c_short;
pub type __u16 = libc::c_ushort;
pub type __s32 = libc::c_int;
pub type u32 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sync_serial_settings {
  pub clock_rate: libc::c_uint,
  pub clock_type: libc::c_uint,
  pub loopback: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct te1_settings {
  pub clock_rate: libc::c_uint,
  pub clock_type: libc::c_uint,
  pub loopback: libc::c_ushort,
  pub slot_map: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct raw_hdlc_proto {
  pub encoding: libc::c_ushort,
  pub parity: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fr_proto {
  pub t391: libc::c_uint,
  pub t392: libc::c_uint,
  pub n391: libc::c_uint,
  pub n392: libc::c_uint,
  pub n393: libc::c_uint,
  pub lmi: libc::c_ushort,
  pub dce: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fr_proto_pvc {
  pub dlci: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fr_proto_pvc_info {
  pub dlci: libc::c_uint,
  pub master: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cisco_proto {
  pub interval: libc::c_uint,
  pub timeout: libc::c_uint,
}
pub type net_device_flags = libc::c_uint;
pub const IFF_ECHO: net_device_flags = 262144;
pub const IFF_DORMANT: net_device_flags = 131072;
pub const IFF_LOWER_UP: net_device_flags = 65536;
pub const IFF_DYNAMIC: net_device_flags = 32768;
pub const IFF_AUTOMEDIA: net_device_flags = 16384;
pub const IFF_PORTSEL: net_device_flags = 8192;
pub const IFF_MULTICAST: net_device_flags = 4096;
pub const IFF_SLAVE: net_device_flags = 2048;
pub const IFF_MASTER: net_device_flags = 1024;
pub const IFF_ALLMULTI: net_device_flags = 512;
pub const IFF_PROMISC: net_device_flags = 256;
pub const IFF_NOARP: net_device_flags = 128;
pub const IFF_RUNNING: net_device_flags = 64;
pub const IFF_NOTRAILERS: net_device_flags = 32;
pub const IFF_POINTOPOINT: net_device_flags = 16;
pub const IFF_LOOPBACK: net_device_flags = 8;
pub const IFF_DEBUG: net_device_flags = 4;
pub const IFF_BROADCAST: net_device_flags = 2;
pub const IFF_UP: net_device_flags = 1;
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
pub struct if_settings {
  pub type_0: libc::c_uint,
  pub size: libc::c_uint,
  pub ifs_ifsu: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
  pub raw_hdlc: *mut raw_hdlc_proto,
  pub cisco: *mut cisco_proto,
  pub fr: *mut fr_proto,
  pub fr_pvc: *mut fr_proto_pvc,
  pub fr_pvc_info: *mut fr_proto_pvc_info,
  pub sync: *mut sync_serial_settings,
  pub te1: *mut te1_settings,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifreq {
  pub ifr_ifrn: C2RustUnnamed_4,
  pub ifr_ifru: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
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
  pub ifru_data: *mut libc::c_void,
  pub ifru_settings: if_settings,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
  pub ifrn_name: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ethtool_value {
  pub cmd: u32,
  pub data: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mii_ioctl_data {
  pub phy_id: __u16,
  pub reg_num: __u16,
  pub val_in: __u16,
  pub val_out: __u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nlmsghdr {
  pub nlmsg_len: u32,
  pub nlmsg_type: __u16,
  pub nlmsg_flags: __u16,
  pub nlmsg_seq: u32,
  pub nlmsg_pid: u32,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const __IFLA_MAX: C2RustUnnamed_5 = 50;
pub const IFLA_NEW_IFINDEX: C2RustUnnamed_5 = 49;
pub const IFLA_CARRIER_DOWN_COUNT: C2RustUnnamed_5 = 48;
pub const IFLA_CARRIER_UP_COUNT: C2RustUnnamed_5 = 47;
pub const IFLA_IF_NETNSID: C2RustUnnamed_5 = 46;
pub const IFLA_NEW_NETNSID: C2RustUnnamed_5 = 45;
pub const IFLA_EVENT: C2RustUnnamed_5 = 44;
pub const IFLA_XDP: C2RustUnnamed_5 = 43;
pub const IFLA_PAD: C2RustUnnamed_5 = 42;
pub const IFLA_GSO_MAX_SIZE: C2RustUnnamed_5 = 41;
pub const IFLA_GSO_MAX_SEGS: C2RustUnnamed_5 = 40;
pub const IFLA_PROTO_DOWN: C2RustUnnamed_5 = 39;
pub const IFLA_PHYS_PORT_NAME: C2RustUnnamed_5 = 38;
pub const IFLA_LINK_NETNSID: C2RustUnnamed_5 = 37;
pub const IFLA_PHYS_SWITCH_ID: C2RustUnnamed_5 = 36;
pub const IFLA_CARRIER_CHANGES: C2RustUnnamed_5 = 35;
pub const IFLA_PHYS_PORT_ID: C2RustUnnamed_5 = 34;
pub const IFLA_CARRIER: C2RustUnnamed_5 = 33;
pub const IFLA_NUM_RX_QUEUES: C2RustUnnamed_5 = 32;
pub const IFLA_NUM_TX_QUEUES: C2RustUnnamed_5 = 31;
pub const IFLA_PROMISCUITY: C2RustUnnamed_5 = 30;
pub const IFLA_EXT_MASK: C2RustUnnamed_5 = 29;
pub const IFLA_NET_NS_FD: C2RustUnnamed_5 = 28;
pub const IFLA_GROUP: C2RustUnnamed_5 = 27;
pub const IFLA_AF_SPEC: C2RustUnnamed_5 = 26;
pub const IFLA_PORT_SELF: C2RustUnnamed_5 = 25;
pub const IFLA_VF_PORTS: C2RustUnnamed_5 = 24;
pub const IFLA_STATS64: C2RustUnnamed_5 = 23;
pub const IFLA_VFINFO_LIST: C2RustUnnamed_5 = 22;
pub const IFLA_NUM_VF: C2RustUnnamed_5 = 21;
pub const IFLA_IFALIAS: C2RustUnnamed_5 = 20;
pub const IFLA_NET_NS_PID: C2RustUnnamed_5 = 19;
pub const IFLA_LINKINFO: C2RustUnnamed_5 = 18;
pub const IFLA_LINKMODE: C2RustUnnamed_5 = 17;
pub const IFLA_OPERSTATE: C2RustUnnamed_5 = 16;
pub const IFLA_WEIGHT: C2RustUnnamed_5 = 15;
pub const IFLA_MAP: C2RustUnnamed_5 = 14;
pub const IFLA_TXQLEN: C2RustUnnamed_5 = 13;
pub const IFLA_PROTINFO: C2RustUnnamed_5 = 12;
pub const IFLA_WIRELESS: C2RustUnnamed_5 = 11;
pub const IFLA_MASTER: C2RustUnnamed_5 = 10;
pub const IFLA_PRIORITY: C2RustUnnamed_5 = 9;
pub const IFLA_COST: C2RustUnnamed_5 = 8;
pub const IFLA_STATS: C2RustUnnamed_5 = 7;
pub const IFLA_QDISC: C2RustUnnamed_5 = 6;
pub const IFLA_LINK: C2RustUnnamed_5 = 5;
pub const IFLA_MTU: C2RustUnnamed_5 = 4;
pub const IFLA_IFNAME: C2RustUnnamed_5 = 3;
pub const IFLA_BROADCAST: C2RustUnnamed_5 = 2;
pub const IFLA_ADDRESS: C2RustUnnamed_5 = 1;
pub const IFLA_UNSPEC: C2RustUnnamed_5 = 0;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const __RTM_MAX: C2RustUnnamed_6 = 97;
pub const RTM_NEWCACHEREPORT: C2RustUnnamed_6 = 96;
pub const RTM_GETSTATS: C2RustUnnamed_6 = 94;
pub const RTM_NEWSTATS: C2RustUnnamed_6 = 92;
pub const RTM_GETNSID: C2RustUnnamed_6 = 90;
pub const RTM_DELNSID: C2RustUnnamed_6 = 89;
pub const RTM_NEWNSID: C2RustUnnamed_6 = 88;
pub const RTM_GETMDB: C2RustUnnamed_6 = 86;
pub const RTM_DELMDB: C2RustUnnamed_6 = 85;
pub const RTM_NEWMDB: C2RustUnnamed_6 = 84;
pub const RTM_GETNETCONF: C2RustUnnamed_6 = 82;
pub const RTM_DELNETCONF: C2RustUnnamed_6 = 81;
pub const RTM_NEWNETCONF: C2RustUnnamed_6 = 80;
pub const RTM_SETDCB: C2RustUnnamed_6 = 79;
pub const RTM_GETDCB: C2RustUnnamed_6 = 78;
pub const RTM_GETADDRLABEL: C2RustUnnamed_6 = 74;
pub const RTM_DELADDRLABEL: C2RustUnnamed_6 = 73;
pub const RTM_NEWADDRLABEL: C2RustUnnamed_6 = 72;
pub const RTM_NEWNDUSEROPT: C2RustUnnamed_6 = 68;
pub const RTM_SETNEIGHTBL: C2RustUnnamed_6 = 67;
pub const RTM_GETNEIGHTBL: C2RustUnnamed_6 = 66;
pub const RTM_NEWNEIGHTBL: C2RustUnnamed_6 = 64;
pub const RTM_GETANYCAST: C2RustUnnamed_6 = 62;
pub const RTM_GETMULTICAST: C2RustUnnamed_6 = 58;
pub const RTM_NEWPREFIX: C2RustUnnamed_6 = 52;
pub const RTM_GETACTION: C2RustUnnamed_6 = 50;
pub const RTM_DELACTION: C2RustUnnamed_6 = 49;
pub const RTM_NEWACTION: C2RustUnnamed_6 = 48;
pub const RTM_GETTFILTER: C2RustUnnamed_6 = 46;
pub const RTM_DELTFILTER: C2RustUnnamed_6 = 45;
pub const RTM_NEWTFILTER: C2RustUnnamed_6 = 44;
pub const RTM_GETTCLASS: C2RustUnnamed_6 = 42;
pub const RTM_DELTCLASS: C2RustUnnamed_6 = 41;
pub const RTM_NEWTCLASS: C2RustUnnamed_6 = 40;
pub const RTM_GETQDISC: C2RustUnnamed_6 = 38;
pub const RTM_DELQDISC: C2RustUnnamed_6 = 37;
pub const RTM_NEWQDISC: C2RustUnnamed_6 = 36;
pub const RTM_GETRULE: C2RustUnnamed_6 = 34;
pub const RTM_DELRULE: C2RustUnnamed_6 = 33;
pub const RTM_NEWRULE: C2RustUnnamed_6 = 32;
pub const RTM_GETNEIGH: C2RustUnnamed_6 = 30;
pub const RTM_DELNEIGH: C2RustUnnamed_6 = 29;
pub const RTM_NEWNEIGH: C2RustUnnamed_6 = 28;
pub const RTM_GETROUTE: C2RustUnnamed_6 = 26;
pub const RTM_DELROUTE: C2RustUnnamed_6 = 25;
pub const RTM_NEWROUTE: C2RustUnnamed_6 = 24;
pub const RTM_GETADDR: C2RustUnnamed_6 = 22;
pub const RTM_DELADDR: C2RustUnnamed_6 = 21;
pub const RTM_NEWADDR: C2RustUnnamed_6 = 20;
pub const RTM_SETLINK: C2RustUnnamed_6 = 19;
pub const RTM_GETLINK: C2RustUnnamed_6 = 18;
pub const RTM_DELLINK: C2RustUnnamed_6 = 17;
pub const RTM_NEWLINK: C2RustUnnamed_6 = 16;
pub const RTM_BASE: C2RustUnnamed_6 = 16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rtattr {
  pub rta_len: libc::c_ushort,
  pub rta_type: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifinfomsg {
  pub ifi_family: libc::c_uchar,
  pub __ifi_pad: libc::c_uchar,
  pub ifi_type: libc::c_ushort,
  pub ifi_index: libc::c_int,
  pub ifi_flags: libc::c_uint,
  pub ifi_change: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iw_param {
  pub value: __s32,
  pub fixed: __u8,
  pub disabled: __u8,
  pub flags: __u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iw_point {
  pub pointer: *mut libc::c_void,
  pub length: __u16,
  pub flags: __u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iw_freq {
  pub m: __s32,
  pub e: __s16,
  pub i: __u8,
  pub flags: __u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iw_quality {
  pub qual: __u8,
  pub level: __u8,
  pub noise: __u8,
  pub updated: __u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union iwreq_data {
  pub name: [libc::c_char; 16],
  pub essid: iw_point,
  pub nwid: iw_param,
  pub freq: iw_freq,
  pub sens: iw_param,
  pub bitrate: iw_param,
  pub txpower: iw_param,
  pub rts: iw_param,
  pub frag: iw_param,
  pub mode: u32,
  pub retry: iw_param,
  pub encoding: iw_point,
  pub power: iw_param,
  pub qual: iw_quality,
  pub ap_addr: sockaddr,
  pub addr: sockaddr,
  pub param: iw_param,
  pub data: iw_point,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iwreq {
  pub ifr_ifrn: C2RustUnnamed_7,
  pub u: iwreq_data,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
  pub ifrn_name: [libc::c_char; 16],
}
pub type C2RustUnnamed_8 = libc::c_uint;
pub const FLAG_KILL: C2RustUnnamed_8 = 131072;
pub const FLAG_MONITOR: C2RustUnnamed_8 = 65536;
pub const FLAG_EXTRA_ARG: C2RustUnnamed_8 = 32768;
pub const FLAG_INITIAL_DOWN: C2RustUnnamed_8 = 16384;
pub const FLAG_NO_SHUTDOWN: C2RustUnnamed_8 = 8192;
pub const FLAG_NO_STARTUP: C2RustUnnamed_8 = 4096;
pub const FLAG_API_MODE: C2RustUnnamed_8 = 2048;
pub const FLAG_DELAY_DOWN: C2RustUnnamed_8 = 1024;
pub const FLAG_DELAY_UP: C2RustUnnamed_8 = 512;
pub const FLAG_POLL_TIME: C2RustUnnamed_8 = 256;
pub const FLAG_IGNORE_RETVAL: C2RustUnnamed_8 = 128;
pub const FLAG_RUN: C2RustUnnamed_8 = 64;
pub const FLAG_IFACE: C2RustUnnamed_8 = 32;
pub const FLAG_IGNORE_FAIL_POSITIVE: C2RustUnnamed_8 = 16;
pub const FLAG_IGNORE_FAIL: C2RustUnnamed_8 = 8;
pub const FLAG_NO_SYSLOG: C2RustUnnamed_8 = 4;
pub const FLAG_NO_DAEMON: C2RustUnnamed_8 = 2;
pub const FLAG_NO_AUTO: C2RustUnnamed_8 = 1;
pub type C2RustUnnamed_9 = libc::c_int;
pub const IFSTATUS_UP: C2RustUnnamed_9 = 1;
pub const IFSTATUS_DOWN: C2RustUnnamed_9 = 0;
pub const IFSTATUS_ERR: C2RustUnnamed_9 = -1;
pub type C2RustUnnamed_10 = libc::c_uint;
pub const netlink_fd: C2RustUnnamed_10 = 4;
pub const ioctl_fd: C2RustUnnamed_10 = 3;
pub type C2RustUnnamed_11 = libc::c_uint;
// 'a'
// 'i'
pub const API_AUTO: C2RustUnnamed_11 = 5;
// 'w'
pub const API_IFF: C2RustUnnamed_11 = 4;
// 'p'
pub const API_WLAN: C2RustUnnamed_11 = 3;
// 'm'
pub const API_PRIVATE: C2RustUnnamed_11 = 2;
// 'e'
pub const API_MII: C2RustUnnamed_11 = 1;
// api mode
pub const API_ETHTOOL: C2RustUnnamed_11 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
  pub name: *const libc::c_char,
  pub func: Option<unsafe extern "C" fn() -> smallint>,
}
pub const BUF_SIZE: C2RustUnnamed_13 = 8192;
pub type C2RustUnnamed_13 = libc::c_uint;
#[inline(always)]
unsafe extern "C" fn bb_strtoul(
  mut arg: *const libc::c_char,
  mut endp: *mut *mut libc::c_char,
  mut base: libc::c_int,
) -> libc::c_ulong {
  return bb_strtoull(arg, endp, base) as libc::c_ulong;
}
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
/* Utility routines */
unsafe extern "C" fn set_ifreq_to_ifname(mut ifreq: *mut ifreq) {
  memset(
    ifreq as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<ifreq>() as libc::c_ulong,
  );
  strncpy_IFNAMSIZ(
    (*ifreq).ifr_ifrn.ifrn_name.as_mut_ptr(),
    (*ptr_to_globals).iface,
  );
}
unsafe extern "C" fn network_ioctl(
  mut request: libc::c_int,
  mut data: *mut libc::c_void,
  mut errmsg: *const libc::c_char,
) -> libc::c_int {
  let mut r: libc::c_int = ioctl(ioctl_fd as libc::c_int, request as libc::c_ulong, data);
  if r < 0i32 && !errmsg.is_null() {
    bb_perror_msg(b"%s failed\x00" as *const u8 as *const libc::c_char, errmsg);
  }
  return r;
}
/* Link detection routines and table */
unsafe extern "C" fn detect_link_mii() -> smallint {
  /* char buffer instead of bona-fide struct avoids aliasing warning */
  let mut buf: [libc::c_char; 40] = [0; 40];
  let ifreq: *mut ifreq = buf.as_mut_ptr() as *mut libc::c_void as *mut ifreq;
  let mut mii: *mut mii_ioctl_data = &mut (*ifreq).ifr_ifru.ifru_data as *mut *mut libc::c_void
    as *mut libc::c_void as *mut mii_ioctl_data;
  set_ifreq_to_ifname(ifreq);
  if network_ioctl(
    0x8947i32,
    ifreq as *mut libc::c_void,
    b"SIOCGMIIPHY\x00" as *const u8 as *const libc::c_char,
  ) < 0i32
  {
    return IFSTATUS_ERR as libc::c_int as smallint;
  }
  (*mii).reg_num = 1i32 as __u16;
  if network_ioctl(
    0x8948i32,
    ifreq as *mut libc::c_void,
    b"SIOCGMIIREG\x00" as *const u8 as *const libc::c_char,
  ) < 0i32
  {
    return IFSTATUS_ERR as libc::c_int as smallint;
  }
  return if (*mii).val_out as libc::c_int & 0x4i32 != 0 {
    IFSTATUS_UP as libc::c_int
  } else {
    IFSTATUS_DOWN as libc::c_int
  } as smallint;
}
unsafe extern "C" fn detect_link_priv() -> smallint {
  /* char buffer instead of bona-fide struct avoids aliasing warning */
  let mut buf: [libc::c_char; 40] = [0; 40];
  let ifreq: *mut ifreq = buf.as_mut_ptr() as *mut libc::c_void as *mut ifreq;
  let mut mii: *mut mii_ioctl_data = &mut (*ifreq).ifr_ifru.ifru_data as *mut *mut libc::c_void
    as *mut libc::c_void as *mut mii_ioctl_data;
  set_ifreq_to_ifname(ifreq);
  if network_ioctl(
    0x89f0i32,
    ifreq as *mut libc::c_void,
    b"SIOCDEVPRIVATE\x00" as *const u8 as *const libc::c_char,
  ) < 0i32
  {
    return IFSTATUS_ERR as libc::c_int as smallint;
  }
  (*mii).reg_num = 1i32 as __u16;
  if network_ioctl(
    0x89f0i32 + 1i32,
    ifreq as *mut libc::c_void,
    b"SIOCDEVPRIVATE+1\x00" as *const u8 as *const libc::c_char,
  ) < 0i32
  {
    return IFSTATUS_ERR as libc::c_int as smallint;
  }
  return if (*mii).val_out as libc::c_int & 0x4i32 != 0 {
    IFSTATUS_UP as libc::c_int
  } else {
    IFSTATUS_DOWN as libc::c_int
  } as smallint;
}
unsafe extern "C" fn detect_link_ethtool() -> smallint {
  let mut ifreq: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_4 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed_3 {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  let mut edata: ethtool_value = ethtool_value { cmd: 0, data: 0 };
  set_ifreq_to_ifname(&mut ifreq);
  edata.cmd = 0xai32 as u32;
  ifreq.ifr_ifru.ifru_data = &mut edata as *mut ethtool_value as *mut libc::c_void;
  if network_ioctl(
    0x8946i32,
    &mut ifreq as *mut ifreq as *mut libc::c_void,
    b"ETHTOOL_GLINK\x00" as *const u8 as *const libc::c_char,
  ) < 0i32
  {
    return IFSTATUS_ERR as libc::c_int as smallint;
  }
  return if edata.data != 0 {
    IFSTATUS_UP as libc::c_int
  } else {
    IFSTATUS_DOWN as libc::c_int
  } as smallint;
}
unsafe extern "C" fn detect_link_iff() -> smallint {
  let mut ifreq: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_4 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed_3 {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  set_ifreq_to_ifname(&mut ifreq);
  if network_ioctl(
    0x8913i32,
    &mut ifreq as *mut ifreq as *mut libc::c_void,
    b"SIOCGIFFLAGS\x00" as *const u8 as *const libc::c_char,
  ) < 0i32
  {
    return IFSTATUS_ERR as libc::c_int as smallint;
  }
  /* If IFF_UP is not set (interface is down), IFF_RUNNING is never set
   * regardless of link status. Simply continue to report last status -
   * no point in reporting spurious link downs if interface is disabled
   * by admin. When/if it will be brought up,
   * we'll report real link status.
   */
  if ifreq.ifr_ifru.ifru_flags as libc::c_int & IFF_UP as libc::c_int == 0
    && (*ptr_to_globals).iface_last_status as libc::c_int != IFSTATUS_ERR as libc::c_int
  {
    return (*ptr_to_globals).iface_last_status;
  }
  return if ifreq.ifr_ifru.ifru_flags as libc::c_int & IFF_RUNNING as libc::c_int != 0 {
    IFSTATUS_UP as libc::c_int
  } else {
    IFSTATUS_DOWN as libc::c_int
  } as smallint;
}
unsafe extern "C" fn detect_link_wlan() -> smallint {
  let mut i: libc::c_int = 0;
  let mut iwrequest: iwreq = iwreq {
    ifr_ifrn: C2RustUnnamed_7 { ifrn_name: [0; 16] },
    u: iwreq_data { name: [0; 16] },
  };
  let mut mac: [u8; 6] = [0; 6];
  memset(
    &mut iwrequest as *mut iwreq as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<iwreq>() as libc::c_ulong,
  );
  strncpy_IFNAMSIZ(
    iwrequest.ifr_ifrn.ifrn_name.as_mut_ptr(),
    (*ptr_to_globals).iface,
  );
  if network_ioctl(
    0x8b15i32,
    &mut iwrequest as *mut iwreq as *mut libc::c_void,
    b"SIOCGIWAP\x00" as *const u8 as *const libc::c_char,
  ) < 0i32
  {
    return IFSTATUS_ERR as libc::c_int as smallint;
  }
  memcpy(
    mac.as_mut_ptr() as *mut libc::c_void,
    &mut iwrequest.u.ap_addr.sa_data as *mut [libc::c_char; 14] as *const libc::c_void,
    6i32 as libc::c_ulong,
  );
  if mac[0] as libc::c_int == 0xffi32
    || mac[0] as libc::c_int == 0x44i32
    || mac[0] as libc::c_int == 0i32
  {
    i = 1i32;
    while i < 6i32 {
      if mac[i as usize] as libc::c_int != mac[0] as libc::c_int {
        return IFSTATUS_UP as libc::c_int as smallint;
      }
      i += 1
    }
    return IFSTATUS_DOWN as libc::c_int as smallint;
  }
  return IFSTATUS_UP as libc::c_int as smallint;
}
static mut api_modes: [libc::c_char; 7] = [101, 109, 112, 119, 105, 97, 0];
static mut method_table: [C2RustUnnamed_12; 5] = [
  {
    let mut init = C2RustUnnamed_12 {
      name: b"SIOCETHTOOL\x00" as *const u8 as *const libc::c_char,
      func: Some(detect_link_ethtool as unsafe extern "C" fn() -> smallint),
    };
    init
  },
  {
    let mut init = C2RustUnnamed_12 {
      name: b"SIOCGMIIPHY\x00" as *const u8 as *const libc::c_char,
      func: Some(detect_link_mii as unsafe extern "C" fn() -> smallint),
    };
    init
  },
  {
    let mut init = C2RustUnnamed_12 {
      name: b"SIOCDEVPRIVATE\x00" as *const u8 as *const libc::c_char,
      func: Some(detect_link_priv as unsafe extern "C" fn() -> smallint),
    };
    init
  },
  {
    let mut init = C2RustUnnamed_12 {
      name: b"wireless extension\x00" as *const u8 as *const libc::c_char,
      func: Some(detect_link_wlan as unsafe extern "C" fn() -> smallint),
    };
    init
  },
  {
    let mut init = C2RustUnnamed_12 {
      name: b"IFF_RUNNING\x00" as *const u8 as *const libc::c_char,
      func: Some(detect_link_iff as unsafe extern "C" fn() -> smallint),
    };
    init
  },
];
unsafe extern "C" fn strstatus(mut status: libc::c_int) -> *const libc::c_char {
  if status == IFSTATUS_ERR as libc::c_int {
    return b"error\x00" as *const u8 as *const libc::c_char;
  }
  return (b"down\x00up\x00" as *const u8 as *const libc::c_char).offset((status * 5i32) as isize);
}
unsafe extern "C" fn run_script(mut action: *const libc::c_char) -> libc::c_int {
  let mut env_PREVIOUS: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut env_CURRENT: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut argv: [*mut libc::c_char; 5] = [0 as *mut libc::c_char; 5];
  let mut r: libc::c_int = 0;
  bb_info_msg(
    b"executing \'%s %s %s\'\x00" as *const u8 as *const libc::c_char,
    (*ptr_to_globals).script_name,
    (*ptr_to_globals).iface,
    action,
  );
  argv[0] = (*ptr_to_globals).script_name as *mut libc::c_char;
  argv[1] = (*ptr_to_globals).iface as *mut libc::c_char;
  argv[2] = action as *mut libc::c_char;
  argv[3] = (*ptr_to_globals).extra_arg as *mut libc::c_char;
  argv[4] = 0 as *mut libc::c_char;
  env_PREVIOUS = xasprintf(
    b"%s=%s\x00" as *const u8 as *const libc::c_char,
    b"IFPLUGD_PREVIOUS\x00" as *const u8 as *const libc::c_char,
    strstatus((*ptr_to_globals).iface_prev_status as libc::c_int),
  );
  putenv(env_PREVIOUS);
  env_CURRENT = xasprintf(
    b"%s=%s\x00" as *const u8 as *const libc::c_char,
    b"IFPLUGD_CURRENT\x00" as *const u8 as *const libc::c_char,
    strstatus((*ptr_to_globals).iface_last_status as libc::c_int),
  );
  putenv(env_CURRENT);
  /* r < 0 - can't exec, 0 <= r < 0x180 - exited, >=0x180 - killed by sig (r-0x180) */
  r = spawn_and_wait(argv.as_mut_ptr());
  bb_unsetenv_and_free(env_PREVIOUS);
  bb_unsetenv_and_free(env_CURRENT);
  bb_info_msg(
    b"exit code: %d\x00" as *const u8 as *const libc::c_char,
    r & 0xffi32,
  );
  return if option_mask32 & FLAG_IGNORE_RETVAL as libc::c_int as libc::c_uint != 0 {
    0i32
  } else {
    r
  };
}
unsafe extern "C" fn up_iface() {
  let mut ifrequest: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_4 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed_3 {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  if (*ptr_to_globals).iface_exists == 0 {
    return;
  }
  set_ifreq_to_ifname(&mut ifrequest);
  if network_ioctl(
    0x8913i32,
    &mut ifrequest as *mut ifreq as *mut libc::c_void,
    b"getting interface flags\x00" as *const u8 as *const libc::c_char,
  ) < 0i32
  {
    (*ptr_to_globals).iface_exists = 0i32 as smallint;
    return;
  }
  if ifrequest.ifr_ifru.ifru_flags as libc::c_int & IFF_UP as libc::c_int == 0 {
    ifrequest.ifr_ifru.ifru_flags =
      (ifrequest.ifr_ifru.ifru_flags as libc::c_int | IFF_UP as libc::c_int) as libc::c_short;
    /* Let user know we mess up with interface */
    bb_simple_info_msg(b"upping interface\x00" as *const u8 as *const libc::c_char);
    if network_ioctl(
      0x8914i32,
      &mut ifrequest as *mut ifreq as *mut libc::c_void,
      b"setting interface flags\x00" as *const u8 as *const libc::c_char,
    ) < 0i32
    {
      if *bb_errno != 19i32 && *bb_errno != 99i32 {
        xfunc_die();
      }
      (*ptr_to_globals).iface_exists = 0i32 as smallint;
      return;
    }
  };
  /* why do we mess with IP addr? It's not our business */
}
unsafe extern "C" fn maybe_up_new_iface() {
  if option_mask32 & FLAG_NO_AUTO as libc::c_int as libc::c_uint == 0 {
    up_iface();
  }
  /* bloat */
  if *(*ptr_to_globals).api_mode.offset(0) as libc::c_int == 'a' as i32 {
    (*ptr_to_globals).api_method_num = API_AUTO as libc::c_int as smallint
  };
}
unsafe extern "C" fn detect_link() -> smallint {
  let mut status: smallint = 0;
  if (*ptr_to_globals).iface_exists == 0 {
    return if option_mask32 & FLAG_MONITOR as libc::c_int as libc::c_uint != 0 {
      IFSTATUS_DOWN as libc::c_int
    } else {
      IFSTATUS_ERR as libc::c_int
    } as smallint;
  }
  /* Some drivers can't detect link status when the interface is down.
   * I imagine detect_link_iff() is the most vulnerable.
   * That's why -a "noauto" in an option, not a hardwired behavior.
   */
  if option_mask32 & FLAG_NO_AUTO as libc::c_int as libc::c_uint == 0 {
    up_iface();
  }
  if (*ptr_to_globals).api_method_num as libc::c_int == API_AUTO as libc::c_int {
    let mut i: libc::c_int = 0;
    let mut sv_logmode: smallint = 0;
    sv_logmode = logmode;
    i = 0i32;
    while (i as libc::c_uint)
      < (::std::mem::size_of::<[C2RustUnnamed_12; 5]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<C2RustUnnamed_12>() as libc::c_ulong)
        as libc::c_uint
    {
      logmode = LOGMODE_NONE as libc::c_int as smallint;
      status = method_table[i as usize]
        .func
        .expect("non-null function pointer")();
      logmode = sv_logmode;
      if status as libc::c_int != IFSTATUS_ERR as libc::c_int {
        (*ptr_to_globals).api_method_num = i as smallint;
        bb_info_msg(
          b"using %s detection mode\x00" as *const u8 as *const libc::c_char,
          method_table[i as usize].name,
        );
        break;
      } else {
        i += 1
      }
    }
  } else {
    status = method_table[(*ptr_to_globals).api_method_num as usize]
      .func
      .expect("non-null function pointer")()
  }
  if status as libc::c_int == IFSTATUS_ERR as libc::c_int {
    if option_mask32 & FLAG_IGNORE_FAIL as libc::c_int as libc::c_uint != 0 {
      status = IFSTATUS_DOWN as libc::c_int as smallint
    } else if option_mask32 & FLAG_IGNORE_FAIL_POSITIVE as libc::c_int as libc::c_uint != 0 {
      status = IFSTATUS_UP as libc::c_int as smallint
    } else if *(*ptr_to_globals).api_mode.offset(0) as libc::c_int == 'a' as i32 {
      bb_simple_error_msg(b"can\'t detect link status\x00" as *const u8 as *const libc::c_char);
    }
  }
  if status as libc::c_int != (*ptr_to_globals).iface_last_status as libc::c_int {
    (*ptr_to_globals).iface_prev_status = (*ptr_to_globals).iface_last_status;
    (*ptr_to_globals).iface_last_status = status
  }
  return status;
}
#[inline(never)]
unsafe extern "C" fn check_existence_through_netlink() -> libc::c_int {
  let mut iface_len: libc::c_int = 0;
  /* Buffer was 1K, but on linux-3.9.9 it was reported to be too small.
   * netlink.h: "limit to 8K to avoid MSG_TRUNC when PAGE_SIZE is very large".
   * Note: on error returns (-1) we exit, no need to free replybuf.
   */
  let mut replybuf: *mut libc::c_char =
    xmalloc(BUF_SIZE as libc::c_int as size_t) as *mut libc::c_char;
  iface_len = strlen((*ptr_to_globals).iface) as libc::c_int;
  loop {
    let mut mhdr: *mut nlmsghdr = 0 as *mut nlmsghdr;
    let mut bytes: ssize_t = 0;
    bytes = recv(
      netlink_fd as libc::c_int,
      replybuf as *mut libc::c_void,
      BUF_SIZE as libc::c_int as size_t,
      MSG_DONTWAIT as libc::c_int,
    );
    if bytes < 0 {
      if *bb_errno == 11i32 {
        break;
      }
      if *bb_errno == 4i32 {
        continue;
      }
      bb_simple_perror_msg(b"netlink: recv\x00" as *const u8 as *const libc::c_char);
      return -1i32;
    } else {
      mhdr = replybuf as *mut nlmsghdr;
      while bytes > 0 {
        if !(bytes >= ::std::mem::size_of::<nlmsghdr>() as isize
          && (*mhdr).nlmsg_len as libc::c_ulong
            >= ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong
          && (*mhdr).nlmsg_len <= bytes as u32)
        {
          bb_simple_error_msg(
            b"netlink packet too small or truncated\x00" as *const u8 as *const libc::c_char,
          );
          return -1i32;
        }
        if (*mhdr).nlmsg_type as libc::c_int == RTM_NEWLINK as libc::c_int
          || (*mhdr).nlmsg_type as libc::c_int == RTM_DELLINK as libc::c_int
        {
          let mut attr: *mut rtattr = 0 as *mut rtattr;
          let mut attr_len: libc::c_int = 0;
          if ((*mhdr).nlmsg_len as libc::c_ulong)
            < (::std::mem::size_of::<ifinfomsg>() as libc::c_ulong).wrapping_add(
              ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
                .wrapping_add(4u32 as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong)
                & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
                as libc::c_int as libc::c_ulong,
            )
          {
            bb_simple_error_msg(
              b"netlink packet too small or truncated\x00" as *const u8 as *const libc::c_char,
            );
            return -1i32;
          }
          attr = ((mhdr as *mut libc::c_char).offset(
            (0i32
              + ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
                .wrapping_add(4u32 as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong)
                & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
                as libc::c_int) as isize,
          ) as *mut libc::c_void as *mut libc::c_char)
            .offset(
              ((::std::mem::size_of::<ifinfomsg>() as libc::c_ulong)
                .wrapping_add(4u32 as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong)
                & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as isize,
            ) as *mut rtattr;
          attr_len = ((*mhdr).nlmsg_len as libc::c_ulong).wrapping_sub(
            (::std::mem::size_of::<ifinfomsg>() as libc::c_ulong)
              .wrapping_add(
                ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
                  .wrapping_add(4u32 as libc::c_ulong)
                  .wrapping_sub(1i32 as libc::c_ulong)
                  & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
                  as libc::c_int as libc::c_ulong,
              )
              .wrapping_add(4u32 as libc::c_ulong)
              .wrapping_sub(1i32 as libc::c_ulong)
              & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong,
          ) as libc::c_int;
          while attr_len >= ::std::mem::size_of::<rtattr>() as libc::c_ulong as libc::c_int
            && (*attr).rta_len as libc::c_ulong >= ::std::mem::size_of::<rtattr>() as libc::c_ulong
            && (*attr).rta_len as libc::c_int <= attr_len
          {
            if (*attr).rta_type as libc::c_int == IFLA_IFNAME as libc::c_int {
              let mut len: libc::c_int = ((*attr).rta_len as libc::c_int as libc::c_ulong)
                .wrapping_sub(
                  ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
                    .wrapping_add(4u32 as libc::c_ulong)
                    .wrapping_sub(1i32 as libc::c_ulong)
                    & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
                    .wrapping_add(0i32 as libc::c_ulong),
                ) as libc::c_int;
              if len > 16i32 {
                len = 16i32
              }
              if iface_len <= len
                && strncmp(
                  (*ptr_to_globals).iface,
                  (attr as *mut libc::c_char).offset(
                    ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
                      .wrapping_add(4u32 as libc::c_ulong)
                      .wrapping_sub(1i32 as libc::c_ulong)
                      & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
                      .wrapping_add(0i32 as libc::c_ulong) as isize,
                  ) as *mut libc::c_void as *const libc::c_char,
                  len as libc::c_ulong,
                ) == 0i32
              {
                (*ptr_to_globals).iface_exists = ((*mhdr).nlmsg_type as libc::c_int
                  == RTM_NEWLINK as libc::c_int)
                  as libc::c_int as smallint
              }
            }
            attr_len = (attr_len as libc::c_uint).wrapping_sub(
              ((*attr).rta_len as libc::c_uint)
                .wrapping_add(4u32)
                .wrapping_sub(1i32 as libc::c_uint)
                & !4u32.wrapping_sub(1i32 as libc::c_uint),
            ) as libc::c_int as libc::c_int;
            attr = (attr as *mut libc::c_char).offset(
              (((*attr).rta_len as libc::c_uint)
                .wrapping_add(4u32)
                .wrapping_sub(1i32 as libc::c_uint)
                & !4u32.wrapping_sub(1i32 as libc::c_uint)) as isize,
            ) as *mut rtattr
          }
        }
        bytes -= ((*mhdr)
          .nlmsg_len
          .wrapping_add(4u32)
          .wrapping_sub(1i32 as libc::c_uint)
          & !4u32.wrapping_sub(1i32 as libc::c_uint)) as isize;
        mhdr = (mhdr as *mut libc::c_char).offset(
          ((*mhdr)
            .nlmsg_len
            .wrapping_add(4u32)
            .wrapping_sub(1i32 as libc::c_uint)
            & !4u32.wrapping_sub(1i32 as libc::c_uint)) as isize,
        ) as *mut nlmsghdr
      }
    }
  }
  free(replybuf as *mut libc::c_void);
  return (*ptr_to_globals).iface_exists as libc::c_int;
}
#[inline(never)]
unsafe extern "C" fn read_pid(mut filename: *const libc::c_char) -> pid_t {
  let mut len: libc::c_int = 0;
  let mut buf: [libc::c_char; 128] = [0; 128];
  len = open_read_close(
    filename,
    buf.as_mut_ptr() as *mut libc::c_void,
    127i32 as size_t,
  ) as libc::c_int;
  if len > 0i32 {
    buf[len as usize] = '\u{0}' as i32 as libc::c_char;
    /* returns ULONG_MAX on error => -1 */
    return bb_strtoul(buf.as_mut_ptr(), 0 as *mut *mut libc::c_char, 10i32) as pid_t;
  }
  return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn ifplugd_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut iface_status: libc::c_int = 0;
  let mut delay_time: libc::c_int = 0;
  let mut iface_status_str: *const libc::c_char = 0 as *const libc::c_char;
  let mut netlink_pollfd: [pollfd; 1] = [pollfd {
    fd: 0,
    events: 0,
    revents: 0,
  }; 1];
  let mut opts: libc::c_uint = 0;
  let mut api_mode_found: *const libc::c_char = 0 as *const libc::c_char;
  let mut pidfile_name: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut pid_from_pidfile: pid_t = 0;
  let ref mut fresh0 = *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
    as *mut *mut globals);
  *fresh0 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  (*ptr_to_globals).iface_last_status = -1i32 as smallint;
  (*ptr_to_globals).iface_exists = 1i32 as smallint;
  (*ptr_to_globals).poll_time = 1i32 as libc::c_uint;
  (*ptr_to_globals).delay_down = 5i32 as libc::c_uint;
  (*ptr_to_globals).iface = b"eth0\x00" as *const u8 as *const libc::c_char;
  (*ptr_to_globals).api_mode = b"a\x00" as *const u8 as *const libc::c_char;
  (*ptr_to_globals).script_name =
    b"/etc/ifplugd/ifplugd.action\x00" as *const u8 as *const libc::c_char;
  opts = getopt32(
    argv,
    b"+ansfFi:r:It:+u:+d:+m:pqlx:Mk\x00" as *const u8 as *const libc::c_char,
    &mut (*ptr_to_globals).iface as *mut *const libc::c_char,
    &mut (*ptr_to_globals).script_name as *mut *const libc::c_char,
    &mut (*ptr_to_globals).poll_time as *mut libc::c_uint,
    &mut (*ptr_to_globals).delay_up as *mut libc::c_uint,
    &mut (*ptr_to_globals).delay_down as *mut libc::c_uint,
    &mut (*ptr_to_globals).api_mode as *mut *const libc::c_char,
    &mut (*ptr_to_globals).extra_arg as *mut *const libc::c_char,
  );
  (*ptr_to_globals).poll_time = (*ptr_to_globals)
    .poll_time
    .wrapping_mul(1000i32 as libc::c_uint);
  applet_name = xasprintf(
    b"ifplugd(%s)\x00" as *const u8 as *const libc::c_char,
    (*ptr_to_globals).iface,
  );
  pidfile_name = xasprintf(
    b"/var/run/ifplugd.%s.pid\x00" as *const u8 as *const libc::c_char,
    (*ptr_to_globals).iface,
  );
  pid_from_pidfile = read_pid(pidfile_name);
  if opts & FLAG_KILL as libc::c_int as libc::c_uint != 0 {
    if pid_from_pidfile > 0i32 {
      /* Upstream tool use SIGINT for -k */
      kill(pid_from_pidfile, 2i32);
    }
    return 0i32;
  }
  if pid_from_pidfile > 0i32 && kill(pid_from_pidfile, 0i32) == 0i32 {
    bb_simple_error_msg_and_die(b"daemon already running\x00" as *const u8 as *const libc::c_char);
  }
  api_mode_found = strchr(
    api_modes.as_ptr(),
    *(*ptr_to_globals).api_mode.offset(0) as libc::c_int,
  );
  if api_mode_found.is_null() {
    bb_error_msg_and_die(
      b"unknown API mode \'%s\'\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).api_mode,
    );
  }
  (*ptr_to_globals).api_method_num =
    api_mode_found.wrapping_offset_from(api_modes.as_ptr()) as libc::c_long as smallint;
  if opts & FLAG_NO_DAEMON as libc::c_int as libc::c_uint == 0 {
    bb_daemonize_or_rexec(DAEMON_CHDIR_ROOT as libc::c_int);
  }
  xmove_fd(
    xsocket(2i32, SOCK_DGRAM as libc::c_int, 0i32),
    ioctl_fd as libc::c_int,
  );
  if opts & FLAG_MONITOR as libc::c_int as libc::c_uint != 0 {
    let mut fd: libc::c_int = create_and_bind_to_netlink(0i32, 1i32, 0i32 as libc::c_uint);
    xmove_fd(fd, netlink_fd as libc::c_int);
  }
  write_pidfile(pidfile_name);
  /* this can't be moved before socket creation */
  if opts & FLAG_NO_SYSLOG as libc::c_int as libc::c_uint == 0 {
    openlog(applet_name, 0i32, 3i32 << 3i32);
    logmode = (logmode as libc::c_int | LOGMODE_SYSLOG as libc::c_int) as smallint
  }
  bb_signals(
    0i32 | 1i32 << 2i32 | 1i32 << 15i32 | 1i32 << 3i32 | 1i32 << 1i32,
    Some(record_signo as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  bb_info_msg(
    b"started: %s\x00" as *const u8 as *const libc::c_char,
    bb_banner.as_ptr(),
  );
  if opts & FLAG_MONITOR as libc::c_int as libc::c_uint != 0 {
    let mut ifrequest: ifreq = ifreq {
      ifr_ifrn: C2RustUnnamed_4 { ifrn_name: [0; 16] },
      ifr_ifru: C2RustUnnamed_3 {
        ifru_addr: sockaddr {
          sa_family: 0,
          sa_data: [0; 14],
        },
      },
    };
    set_ifreq_to_ifname(&mut ifrequest);
    (*ptr_to_globals).iface_exists = (network_ioctl(
      0x8933i32,
      &mut ifrequest as *mut ifreq as *mut libc::c_void,
      0 as *const libc::c_char,
    ) == 0i32) as libc::c_int as smallint
  }
  if (*ptr_to_globals).iface_exists != 0 {
    maybe_up_new_iface();
  }
  iface_status = detect_link() as libc::c_int;
  if !(iface_status == IFSTATUS_ERR as libc::c_int) {
    iface_status_str = strstatus(iface_status);
    if opts & FLAG_MONITOR as libc::c_int as libc::c_uint != 0 {
      bb_info_msg(
        b"interface %s\x00" as *const u8 as *const libc::c_char,
        if (*ptr_to_globals).iface_exists as libc::c_int != 0 {
          b"exists\x00" as *const u8 as *const libc::c_char
        } else {
          b"doesn\'t exist, waiting\x00" as *const u8 as *const libc::c_char
        },
      );
    }
    /* else we assume it always exists, but don't mislead user
     * by potentially lying that it really exists */
    if (*ptr_to_globals).iface_exists != 0 {
      bb_info_msg(
        b"link is %s\x00" as *const u8 as *const libc::c_char,
        iface_status_str,
      );
    }
    if opts & FLAG_NO_STARTUP as libc::c_int as libc::c_uint == 0
      && iface_status == IFSTATUS_UP as libc::c_int
      || opts & FLAG_INITIAL_DOWN as libc::c_int as libc::c_uint != 0
    {
      if run_script(iface_status_str) != 0i32 {
        current_block = 8614729751013307988;
      } else {
        current_block = 6560072651652764009;
      }
    } else {
      current_block = 6560072651652764009;
    }
    match current_block {
      8614729751013307988 => {}
      _ => {
        /* Main loop */
        netlink_pollfd[0].fd = netlink_fd as libc::c_int; /* while (1) */
        netlink_pollfd[0].events = 0x1i32 as libc::c_short;
        delay_time = 0i32;
        loop {
          let mut iface_status_old: libc::c_int = 0;
          match bb_got_signal as libc::c_int {
            2 | 15 => {
              bb_got_signal = 0i32 as smallint;
              current_block = 538028991732400653;
              break;
            }
            3 => {
              bb_got_signal = 0i32 as smallint;
              current_block = 8614729751013307988;
              break;
            }
            0 => {}
            _ => bb_got_signal = 0i32 as smallint,
          }
          /* do not clear bb_got_signal if already 0, this can lose signals */
          if poll(
            netlink_pollfd.as_mut_ptr(),
            (if opts & FLAG_MONITOR as libc::c_int as libc::c_uint != 0 {
              1i32
            } else {
              0i32
            }) as nfds_t,
            (*ptr_to_globals).poll_time as libc::c_int,
          ) < 0i32
          {
            if *bb_errno == 4i32 {
              continue;
            }
            bb_simple_perror_msg(b"poll\x00" as *const u8 as *const libc::c_char);
            current_block = 8614729751013307988;
            break;
          } else {
            if opts & FLAG_MONITOR as libc::c_int as libc::c_uint != 0
              && netlink_pollfd[0].revents as libc::c_int & 0x1i32 != 0
            {
              let mut iface_exists_old: libc::c_int = 0;
              iface_exists_old = (*ptr_to_globals).iface_exists as libc::c_int;
              (*ptr_to_globals).iface_exists = check_existence_through_netlink() as smallint;
              if ((*ptr_to_globals).iface_exists as libc::c_int) < 0i32 {
                current_block = 8614729751013307988;
                break;
              }
              if iface_exists_old != (*ptr_to_globals).iface_exists as libc::c_int {
                bb_info_msg(
                  b"interface %sappeared\x00" as *const u8 as *const libc::c_char,
                  if (*ptr_to_globals).iface_exists as libc::c_int != 0 {
                    b"\x00" as *const u8 as *const libc::c_char
                  } else {
                    b"dis\x00" as *const u8 as *const libc::c_char
                  },
                );
                if (*ptr_to_globals).iface_exists != 0 {
                  maybe_up_new_iface();
                }
              }
            }
            /* note: if !G.iface_exists, returns DOWN */
            iface_status_old = iface_status;
            iface_status = detect_link() as libc::c_int;
            if iface_status == IFSTATUS_ERR as libc::c_int {
              if opts & FLAG_MONITOR as libc::c_int as libc::c_uint == 0 {
                current_block = 8614729751013307988;
                break;
              }
              iface_status = IFSTATUS_DOWN as libc::c_int
            }
            iface_status_str = strstatus(iface_status);
            if iface_status_old != iface_status {
              bb_info_msg(
                b"link is %s\x00" as *const u8 as *const libc::c_char,
                iface_status_str,
              );
              if delay_time != 0 {
                /* link restored its old status before
                 * we ran script. don't run the script: */
                delay_time = 0i32
              } else {
                delay_time = monotonic_sec() as libc::c_int;
                if iface_status == IFSTATUS_UP as libc::c_int {
                  delay_time = (delay_time as libc::c_uint).wrapping_add((*ptr_to_globals).delay_up)
                    as libc::c_int as libc::c_int
                }
                if iface_status == IFSTATUS_DOWN as libc::c_int {
                  delay_time = (delay_time as libc::c_uint)
                    .wrapping_add((*ptr_to_globals).delay_down)
                    as libc::c_int as libc::c_int
                }
                /* if you are back in 1970... */
              }
            }
            if !(delay_time != 0
              && monotonic_sec().wrapping_sub(delay_time as libc::c_uint) as libc::c_int >= 0i32)
            {
              continue;
            }
            if run_script(iface_status_str) != 0i32 {
              current_block = 8614729751013307988;
              break;
            }
            delay_time = 0i32
          }
        }
        match current_block {
          8614729751013307988 => {}
          _ => {
            if opts & FLAG_NO_SHUTDOWN as libc::c_int as libc::c_uint == 0
              && (iface_status == IFSTATUS_UP as libc::c_int
                || iface_status == IFSTATUS_DOWN as libc::c_int && delay_time != 0)
            {
              setenv(
                b"IFPLUGD_PREVIOUS\x00" as *const u8 as *const libc::c_char,
                strstatus(iface_status),
                1i32,
              );
              setenv(
                b"IFPLUGD_CURRENT\x00" as *const u8 as *const libc::c_char,
                strstatus(-1i32),
                1i32,
              );
              run_script(b"down\x00up\x00" as *const u8 as *const libc::c_char);
              /* reusing string */
            }
          }
        }
      }
    }
  }
  /* error */
  if wrote_pidfile != 0 {
    unlink(pidfile_name);
  }
  bb_simple_error_msg_and_die(b"exiting\x00" as *const u8 as *const libc::c_char);
}

use libc;



extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
  #[no_mangle]
  fn skip_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn skip_non_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xsocket(domain: libc::c_int, type_0: libc::c_int, protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn strncpy_IFNAMSIZ(dst: *mut libc::c_char, src: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xfopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn xatoi_positive(numstr: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  static mut logmode: smallint;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn config_open(filename: *const libc::c_char) -> *mut parser_t;
  #[no_mangle]
  fn config_open2(
    filename: *const libc::c_char,
    fopen_func: Option<unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE>,
  ) -> *mut parser_t;
  #[no_mangle]
  fn config_read(
    parser: *mut parser_t,
    tokens: *mut *mut libc::c_char,
    flags: libc::c_uint,
    delims: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  fn config_close(parser: *mut parser_t);
  #[no_mangle]
  fn ioctl_or_perror_and_die(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    fmt: *const libc::c_char,
    _: ...
  ) -> libc::c_int;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  fn openlog(__ident: *const libc::c_char, __option: libc::c_int, __facility: libc::c_int);
  #[no_mangle]
  fn ether_aton_r(__asc: *const libc::c_char, __addr: *mut ether_addr) -> *mut ether_addr;
  #[no_mangle]
  fn delete_eth_table(ch: *mut ethtable_t);
}

pub type __caddr_t = *mut libc::c_char;




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
pub type caddr_t = __caddr_t;
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

use libc::FILE;
pub type C2RustUnnamed = libc::c_uint;
pub const LOGMODE_BOTH: C2RustUnnamed = 3;
pub const LOGMODE_SYSLOG: C2RustUnnamed = 2;
pub const LOGMODE_STDIO: C2RustUnnamed = 1;
pub const LOGMODE_NONE: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PARSE_NORMAL: C2RustUnnamed_0 = 4653056;
pub const PARSE_WS_COMMENTS: C2RustUnnamed_0 = 16777216;
pub const PARSE_ALT_COMMENTS: C2RustUnnamed_0 = 8388608;
pub const PARSE_EOL_COMMENTS: C2RustUnnamed_0 = 4194304;
pub const PARSE_KEEP_COPY: C2RustUnnamed_0 = 2097152;
pub const PARSE_MIN_DIE: C2RustUnnamed_0 = 1048576;
pub const PARSE_GREEDY: C2RustUnnamed_0 = 262144;
pub const PARSE_TRIM: C2RustUnnamed_0 = 131072;
pub const PARSE_COLLAPSE: C2RustUnnamed_0 = 65536;
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
#[repr(C, packed)]
pub struct ether_addr {
  pub ether_addr_octet: [u8; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ethtable_s {
  pub next: *mut ethtable_s,
  pub prev: *mut ethtable_s,
  pub ifname: *mut libc::c_char,
  pub mac: *mut ether_addr,
  pub bus_info: *mut libc::c_char,
  pub driver: *mut libc::c_char,
  pub phy_address: i32,
}
pub type ethtable_t = ethtable_s;
/* Cut'n'paste from ethtool.h */
/* these strings are set to whatever the driver author decides... */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ethtool_drvinfo {
  pub cmd: u32,
  pub driver: [libc::c_char; 32],
  pub version: [libc::c_char; 32],
  pub fw_version: [libc::c_char; 32],
  pub bus_info: [libc::c_char; 32],
  pub reserved1: [libc::c_char; 32],
  pub reserved2: [libc::c_char; 16],
  pub n_stats: u32,
  pub testinfo_len: u32,
  pub eedump_len: u32,
  pub regdump_len: u32,
  /* Size of data from ETHTOOL_GREGS (bytes) */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ethtool_cmd {
  pub cmd: u32,
  pub supported: u32,
  pub advertising: u32,
  pub speed: u16,
  pub duplex: u8,
  pub port: u8,
  pub phy_address: u8,
  pub transceiver: u8,
  pub autoneg: u8,
  pub maxtxpkt: u32,
  pub maxrxpkt: u32,
  pub speed_hi: u16,
  pub reserved2: u16,
  pub reserved: [u32; 3],
}
/* Get driver info. */
unsafe extern "C" fn nameif_parse_selector(
  mut ch: *mut ethtable_t,
  mut selector: *mut libc::c_char,
) {
  let mut lmac: *mut ether_addr = 0 as *mut ether_addr;
  let mut found_selector: libc::c_int = 0i32;
  while *selector != 0 {
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    selector = skip_whitespace(selector);
    (*ch).phy_address = -1i32;
    if *selector as libc::c_int == '\u{0}' as i32 {
      break;
    }
    /* Search for the end .... */
    next = skip_non_whitespace(selector);
    if *next != 0 {
      let fresh0 = next;
      next = next.offset(1);
      *fresh0 = '\u{0}' as i32 as libc::c_char
    }
    /* Check for selectors, mac= is assumed */
    if !is_prefixed_with(selector, b"bus=\x00" as *const u8 as *const libc::c_char).is_null() {
      (*ch).bus_info = xstrdup(selector.offset(4));
      found_selector += 1
    } else if !is_prefixed_with(selector, b"driver=\x00" as *const u8 as *const libc::c_char)
      .is_null()
    {
      (*ch).driver = xstrdup(selector.offset(7));
      found_selector += 1
    } else if !is_prefixed_with(
      selector,
      b"phyaddr=\x00" as *const u8 as *const libc::c_char,
    )
    .is_null()
    {
      (*ch).phy_address = xatoi_positive(selector.offset(8));
      found_selector += 1
    } else {
      lmac = xmalloc(6i32 as size_t) as *mut ether_addr;
      (*ch).mac = ether_aton_r(
        selector.offset(
          (if !is_prefixed_with(selector, b"mac=\x00" as *const u8 as *const libc::c_char).is_null()
          {
            4i32
          } else {
            0i32
          }) as isize,
        ),
        lmac,
      );
      if (*ch).mac.is_null() {
        bb_error_msg_and_die(
          b"can\'t parse %s\x00" as *const u8 as *const libc::c_char,
          selector,
        );
      }
      found_selector += 1
    }
    selector = next
  }
  if found_selector == 0i32 {
    bb_error_msg_and_die(
      b"no selectors found for %s\x00" as *const u8 as *const libc::c_char,
      (*ch).ifname,
    );
  };
}
unsafe extern "C" fn prepend_new_eth_table(
  mut clist: *mut *mut ethtable_t,
  mut ifname: *mut libc::c_char,
  mut selector: *mut libc::c_char,
) {
  let mut ch: *mut ethtable_t = 0 as *mut ethtable_t;
  if strlen(ifname) >= 16i32 as libc::c_ulong {
    bb_error_msg_and_die(
      b"interface name \'%s\' too long\x00" as *const u8 as *const libc::c_char,
      ifname,
    );
  }
  ch = xzalloc(::std::mem::size_of::<ethtable_t>() as libc::c_ulong) as *mut ethtable_t;
  (*ch).ifname = xstrdup(ifname);
  nameif_parse_selector(ch, selector);
  (*ch).next = *clist;
  if !(*clist).is_null() {
    (**clist).prev = ch
  }
  *clist = ch;
}
#[no_mangle]
pub unsafe extern "C" fn nameif_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut clist: *mut ethtable_t = 0 as *mut ethtable_t;
  let mut fname: *const libc::c_char = b"/etc/mactab\x00" as *const u8 as *const libc::c_char;
  let mut ctl_sk: libc::c_int = 0;
  let mut ch: *mut ethtable_t = 0 as *mut ethtable_t;
  let mut parser: *mut parser_t = 0 as *mut parser_t;
  let mut token: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
  if 1i32 as libc::c_uint
    & getopt32(
      argv,
      b"sc:\x00" as *const u8 as *const libc::c_char,
      &mut fname as *mut *const libc::c_char,
    )
    != 0
  {
    openlog(applet_name, 0i32, 16i32 << 3i32);
    /* Why not just "="? I assume logging to stderr
     * can't hurt. 2>/dev/null if you don't like it: */
    logmode = (logmode as libc::c_int | LOGMODE_SYSLOG as libc::c_int) as smallint
  } /* while */
  argv = argv.offset(optind as isize); /* Skip the first two lines */
  if !(*argv.offset(0)).is_null() {
    loop {
      if (*argv.offset(1)).is_null() {
        bb_show_usage();
      }
      prepend_new_eth_table(&mut clist, *argv.offset(0), *argv.offset(1));
      argv = argv.offset(2);
      if (*argv).is_null() {
        break;
      }
    }
  } else {
    parser = config_open(fname);
    while config_read(
      parser,
      token.as_mut_ptr(),
      (PARSE_NORMAL as libc::c_int | (2i32 & 0xffi32) << 8i32 | 2i32 & 0xffi32) as libc::c_uint,
      b"# \t\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
      prepend_new_eth_table(&mut clist, token[0], token[1]);
    }
    config_close(parser);
  }
  ctl_sk = xsocket(2i32, SOCK_DGRAM as libc::c_int, 0i32);
  parser = config_open2(
    b"/proc/net/dev\x00" as *const u8 as *const libc::c_char,
    Some(xfopen_for_read as unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE),
  );
  let mut current_block_42: u64;
  while !clist.is_null()
    && config_read(
      parser,
      token.as_mut_ptr(),
      (PARSE_NORMAL as libc::c_int | (2i32 & 0xffi32) << 8i32 | 2i32 & 0xffi32) as libc::c_uint,
      b"\x00: \t\x00" as *const u8 as *const libc::c_char,
    ) != 0
  {
    let mut ifr: ifreq = ifreq {
      ifr_ifrn: C2RustUnnamed_2 { ifrn_name: [0; 16] },
      ifr_ifru: C2RustUnnamed_1 {
        ifru_addr: sockaddr {
          sa_family: 0,
          sa_data: [0; 14],
        },
      },
    };
    let mut drvinfo: ethtool_drvinfo = ethtool_drvinfo {
      cmd: 0,
      driver: [0; 32],
      version: [0; 32],
      fw_version: [0; 32],
      bus_info: [0; 32],
      reserved1: [0; 32],
      reserved2: [0; 16],
      n_stats: 0,
      testinfo_len: 0,
      eedump_len: 0,
      regdump_len: 0,
    };
    let mut eth_settings: ethtool_cmd = ethtool_cmd {
      cmd: 0,
      supported: 0,
      advertising: 0,
      speed: 0,
      duplex: 0,
      port: 0,
      phy_address: 0,
      transceiver: 0,
      autoneg: 0,
      maxtxpkt: 0,
      maxrxpkt: 0,
      speed_hi: 0,
      reserved2: 0,
      reserved: [0; 3],
    };
    if (*parser).lineno <= 2i32 {
      continue;
    }
    /* Find the current interface name and copy it to ifr.ifr_name */
    memset(
      &mut ifr as *mut ifreq as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<ifreq>() as libc::c_ulong,
    );
    strncpy_IFNAMSIZ(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), token[0]);
    /* Check for phy address */
    memset(
      &mut eth_settings as *mut ethtool_cmd as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<ethtool_cmd>() as libc::c_ulong,
    );
    eth_settings.cmd = 0x1i32 as u32;
    ifr.ifr_ifru.ifru_data = &mut eth_settings as *mut ethtool_cmd as caddr_t;
    ioctl(ctl_sk, 0x8946i32 as libc::c_ulong, &mut ifr as *mut ifreq);
    /* Check for driver etc. */
    memset(
      &mut drvinfo as *mut ethtool_drvinfo as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<ethtool_drvinfo>() as libc::c_ulong,
    );
    drvinfo.cmd = 0x3i32 as u32;
    ifr.ifr_ifru.ifru_data = &mut drvinfo as *mut ethtool_drvinfo as caddr_t;
    /* Get driver and businfo first, so we have it in drvinfo */
    ioctl(ctl_sk, 0x8946i32 as libc::c_ulong, &mut ifr as *mut ifreq);
    ioctl(ctl_sk, 0x8927i32 as libc::c_ulong, &mut ifr as *mut ifreq);
    /* Search the list for a matching device */
    ch = clist;
    loop {
      if ch.is_null() {
        current_block_42 = 4567019141635105728;
        break;
      }
      if !(!(*ch).bus_info.is_null()
        && strcmp((*ch).bus_info, drvinfo.bus_info.as_mut_ptr()) != 0i32)
      {
        if !(!(*ch).driver.is_null() && strcmp((*ch).driver, drvinfo.driver.as_mut_ptr()) != 0i32) {
          if !((*ch).phy_address != -1i32
            && (*ch).phy_address != eth_settings.phy_address as libc::c_int)
          {
            if !(!(*ch).mac.is_null()
              && memcmp(
                (*ch).mac as *const libc::c_void,
                ifr.ifr_ifru.ifru_hwaddr.sa_data.as_mut_ptr() as *const libc::c_void,
                6i32 as libc::c_ulong,
              ) != 0i32)
            {
              current_block_42 = 7506631333841133343;
              break;
            }
          }
        }
      }
      ch = (*ch).next
    }
    match current_block_42 {
      4567019141635105728 =>
        /* Nothing found for current interface */
        {}
      _ =>
      /* if we came here, all selectors have matched */
      {
        if strcmp(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), (*ch).ifname) != 0i32 {
          strcpy(ifr.ifr_ifru.ifru_newname.as_mut_ptr(), (*ch).ifname);
          ioctl_or_perror_and_die(
            ctl_sk,
            0x8923i32 as libc::c_uint,
            &mut ifr as *mut ifreq as *mut libc::c_void,
            b"can\'t change ifname %s to %s\x00" as *const u8 as *const libc::c_char,
            ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
            (*ch).ifname,
          );
        }
        /* Remove list entry of renamed interface */
        if !(*ch).prev.is_null() {
          (*(*ch).prev).next = (*ch).next
        } else {
          clist = (*ch).next
        }
        if !(*ch).next.is_null() {
          (*(*ch).next).prev = (*ch).prev
        }
      }
    }
  }
  return 0i32;
}

use crate::libbb::ptr_to_globals::bb_errno;
use crate::librb::smallint;
use libc;
use libc::ioctl;
use libc::sa_family_t;
use libc::sockaddr;
use libc::strcpy;
extern "C" {
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  static ptr_to_globals: *mut globals;
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

//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct globals {
  pub abi_ver: libc::c_uint,
  pub hwaddr_set: smallint,
  pub master: dev_data,
  pub slave: dev_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_data {
  pub mtu: ifreq,
  pub flags: ifreq,
  pub hwaddr: ifreq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifreq {
  pub ifr_ifrn: C2RustUnnamed_1,
  pub ifr_ifru: C2RustUnnamed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed {
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct if_settings {
  pub type_0: libc::c_uint,
  pub size: libc::c_uint,
  pub ifs_ifsu: C2RustUnnamed_0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_0 {
  pub raw_hdlc: *mut raw_hdlc_proto,
  pub cisco: *mut cisco_proto,
  pub fr: *mut fr_proto,
  pub fr_pvc: *mut fr_proto_pvc,
  pub fr_pvc_info: *mut fr_proto_pvc_info,
  pub sync: *mut sync_serial_settings,
  pub te1: *mut te1_settings,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct te1_settings {
  pub clock_rate: libc::c_uint,
  pub clock_type: libc::c_uint,
  pub loopback: libc::c_ushort,
  pub slot_map: libc::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sync_serial_settings {
  pub clock_rate: libc::c_uint,
  pub clock_type: libc::c_uint,
  pub loopback: libc::c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fr_proto_pvc_info {
  pub dlci: libc::c_uint,
  pub master: [libc::c_char; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fr_proto_pvc {
  pub dlci: libc::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fr_proto {
  pub t391: libc::c_uint,
  pub t392: libc::c_uint,
  pub n391: libc::c_uint,
  pub n392: libc::c_uint,
  pub n393: libc::c_uint,
  pub lmi: libc::c_ushort,
  pub dce: libc::c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cisco_proto {
  pub interval: libc::c_uint,
  pub timeout: libc::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_hdlc_proto {
  pub encoding: libc::c_ushort,
  pub parity: libc::c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifmap {
  pub mem_start: libc::c_ulong,
  pub mem_end: libc::c_ulong,
  pub base_addr: libc::c_ushort,
  pub irq: libc::c_uchar,
  pub dma: libc::c_uchar,
  pub port: libc::c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_1 {
  pub ifrn_name: [libc::c_char; 16],
}
pub type u32 = libc::c_uint;
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_drvinfo {
  pub cmd: u32,
  pub driver: [libc::c_char; 32],
  pub version: [libc::c_char; 32],
  pub fw_version: [libc::c_char; 32],
  pub bus_info: [libc::c_char; 32],
  pub erom_version: [libc::c_char; 32],
  pub reserved2: [libc::c_char; 12],
  pub n_priv_flags: u32,
  pub n_stats: u32,
  pub testinfo_len: u32,
  pub eedump_len: u32,
  pub regdump_len: u32,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const skfd: C2RustUnnamed_2 = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
  pub g_ioctl: u16,
  pub s_ioctl: u16,
}
pub const OPT_d: C2RustUnnamed_4 = 2;
pub const OPT_c: C2RustUnnamed_4 = 1;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const OPT_f: C2RustUnnamed_4 = 4;
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
/* NOINLINEs are placed where it results in smaller code (gcc 4.3.1) */
unsafe extern "C" fn ioctl_on_skfd(mut request: libc::c_uint, mut ifr: *mut ifreq) -> libc::c_int {
  return ioctl(skfd as libc::c_int, request as libc::c_ulong, ifr);
}
unsafe extern "C" fn set_ifrname_and_do_ioctl(
  mut request: libc::c_uint,
  mut ifr: *mut ifreq,
  mut ifname: *const libc::c_char,
) -> libc::c_int {
  crate::libbb::xfuncs::strncpy_IFNAMSIZ((*ifr).ifr_ifrn.ifrn_name.as_mut_ptr(), ifname);
  return ioctl_on_skfd(request, ifr);
}
unsafe extern "C" fn get_if_settings(
  mut ifname: *mut libc::c_char,
  mut dd: *mut dev_data,
) -> libc::c_int {
  let mut res: libc::c_int = 0;
  res = set_ifrname_and_do_ioctl(0x8921i32 as libc::c_uint, &mut (*dd).mtu, ifname);
  res |= set_ifrname_and_do_ioctl(0x8913i32 as libc::c_uint, &mut (*dd).flags, ifname);
  res |= set_ifrname_and_do_ioctl(0x8927i32 as libc::c_uint, &mut (*dd).hwaddr, ifname);
  return res;
}
unsafe extern "C" fn get_slave_flags(mut slave_ifname: *mut libc::c_char) -> libc::c_int {
  return set_ifrname_and_do_ioctl(
    0x8913i32 as libc::c_uint,
    &mut (*ptr_to_globals).slave.flags,
    slave_ifname,
  );
}
unsafe extern "C" fn set_hwaddr(
  mut ifname: *mut libc::c_char,
  mut hwaddr: *mut sockaddr,
) -> libc::c_int {
  let mut ifr: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  memcpy(
    &mut ifr.ifr_ifru.ifru_hwaddr as *mut sockaddr as *mut libc::c_void,
    hwaddr as *const libc::c_void,
    ::std::mem::size_of::<sockaddr>() as libc::c_ulong,
  );
  return set_ifrname_and_do_ioctl(0x8924i32 as libc::c_uint, &mut ifr, ifname);
}
unsafe extern "C" fn set_mtu(mut ifname: *mut libc::c_char, mut mtu: libc::c_int) -> libc::c_int {
  let mut ifr: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  ifr.ifr_ifru.ifru_mtu = mtu;
  return set_ifrname_and_do_ioctl(0x8922i32 as libc::c_uint, &mut ifr, ifname);
}
unsafe extern "C" fn set_if_flags(
  mut ifname: *mut libc::c_char,
  mut flags: libc::c_int,
) -> libc::c_int {
  let mut ifr: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  ifr.ifr_ifru.ifru_flags = flags as libc::c_short;
  return set_ifrname_and_do_ioctl(0x8914i32 as libc::c_uint, &mut ifr, ifname);
}
unsafe extern "C" fn set_if_up(
  mut ifname: *mut libc::c_char,
  mut flags: libc::c_int,
) -> libc::c_int {
  let mut res: libc::c_int = set_if_flags(ifname, flags | IFF_UP as libc::c_int);
  if res != 0 {
    crate::libbb::perror_msg::bb_perror_msg(
      b"%s: can\'t up\x00" as *const u8 as *const libc::c_char,
      ifname,
    );
  }
  return res;
}
unsafe extern "C" fn set_if_down(
  mut ifname: *mut libc::c_char,
  mut flags: libc::c_int,
) -> libc::c_int {
  let mut res: libc::c_int = set_if_flags(ifname, flags & !(IFF_UP as libc::c_int));
  if res != 0 {
    crate::libbb::perror_msg::bb_perror_msg(
      b"%s: can\'t down\x00" as *const u8 as *const libc::c_char,
      ifname,
    );
  }
  return res;
}
unsafe extern "C" fn clear_if_addr(mut ifname: *mut libc::c_char) -> libc::c_int {
  let mut ifr: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  ifr.ifr_ifru.ifru_addr.sa_family = 2i32 as sa_family_t;
  memset(
    ifr.ifr_ifru.ifru_addr.sa_data.as_mut_ptr() as *mut libc::c_void,
    0,
    ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong,
  );
  return set_ifrname_and_do_ioctl(0x8916i32 as libc::c_uint, &mut ifr, ifname);
}
unsafe extern "C" fn set_if_addr(
  mut master_ifname: *mut libc::c_char,
  mut slave_ifname: *mut libc::c_char,
) -> libc::c_int {
  static mut ifra: [C2RustUnnamed_3; 4] = [
    {
      let mut init = C2RustUnnamed_3 {
        g_ioctl: 0x8915i32 as u16,
        s_ioctl: 0x8916i32 as u16,
      };
      init
    },
    {
      let mut init = C2RustUnnamed_3 {
        g_ioctl: 0x8917i32 as u16,
        s_ioctl: 0x8918i32 as u16,
      };
      init
    },
    {
      let mut init = C2RustUnnamed_3 {
        g_ioctl: 0x8919i32 as u16,
        s_ioctl: 0x891ai32 as u16,
      };
      init
    },
    {
      let mut init = C2RustUnnamed_3 {
        g_ioctl: 0x891bi32 as u16,
        s_ioctl: 0x891ci32 as u16,
      };
      init
    },
  ];
  let mut ifr: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  let mut res: libc::c_int = 0;
  let mut i: libc::c_uint = 0;
  i = 0 as libc::c_uint;
  while i
    < (::std::mem::size_of::<[C2RustUnnamed_3; 4]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong) as libc::c_uint
  {
    res = set_ifrname_and_do_ioctl(
      ifra[i as usize].g_ioctl as libc::c_uint,
      &mut ifr,
      master_ifname,
    );
    if res < 0 {
      ifr.ifr_ifru.ifru_addr.sa_family = 2i32 as sa_family_t;
      memset(
        ifr.ifr_ifru.ifru_addr.sa_data.as_mut_ptr() as *mut libc::c_void,
        0,
        ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong,
      );
    }
    res = set_ifrname_and_do_ioctl(
      ifra[i as usize].s_ioctl as libc::c_uint,
      &mut ifr,
      slave_ifname,
    );
    if res < 0 {
      return res;
    }
    i = i.wrapping_add(1)
  }
  return 0;
}
unsafe extern "C" fn change_active(
  mut master_ifname: *mut libc::c_char,
  mut slave_ifname: *mut libc::c_char,
) {
  let mut ifr: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  if (*ptr_to_globals).slave.flags.ifr_ifru.ifru_flags as libc::c_int & IFF_SLAVE as libc::c_int
    == 0
  {
    crate::libbb::verror_msg::bb_error_msg_and_die(
      b"%s is not a slave\x00" as *const u8 as *const libc::c_char,
      slave_ifname,
    );
  }
  crate::libbb::xfuncs::strncpy_IFNAMSIZ(ifr.ifr_ifru.ifru_slave.as_mut_ptr(), slave_ifname);
  if set_ifrname_and_do_ioctl(0x8995i32 as libc::c_uint, &mut ifr, master_ifname) != 0
    && ioctl_on_skfd((0x89f0i32 + 13i32) as libc::c_uint, &mut ifr) != 0
  {
    crate::libbb::perror_msg::bb_perror_msg_and_die(
      b"master %s, slave %s: can\'t change active\x00" as *const u8 as *const libc::c_char,
      master_ifname,
      slave_ifname,
    );
  };
}
#[inline(never)]
unsafe extern "C" fn enslave(
  mut master_ifname: *mut libc::c_char,
  mut slave_ifname: *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut ifr: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  let mut res: libc::c_int = 0;
  if (*ptr_to_globals).slave.flags.ifr_ifru.ifru_flags as libc::c_int & IFF_SLAVE as libc::c_int
    != 0
  {
    crate::libbb::verror_msg::bb_error_msg(
      b"%s is already a slave\x00" as *const u8 as *const libc::c_char,
      slave_ifname,
    );
    return 1i32;
  }
  res = set_if_down(
    slave_ifname,
    (*ptr_to_globals).slave.flags.ifr_ifru.ifru_flags as libc::c_int,
  );
  if res != 0 {
    return res;
  }
  if (*ptr_to_globals).abi_ver < 2i32 as libc::c_uint {
    /* Older bonding versions would panic if the slave has no IP
     * address, so get the IP setting from the master.
     */
    res = set_if_addr(master_ifname, slave_ifname);
    if res != 0 {
      crate::libbb::perror_msg::bb_perror_msg(
        b"%s: can\'t set address\x00" as *const u8 as *const libc::c_char,
        slave_ifname,
      );
      return res;
    }
  } else {
    res = clear_if_addr(slave_ifname);
    if res != 0 {
      crate::libbb::perror_msg::bb_perror_msg(
        b"%s: can\'t clear address\x00" as *const u8 as *const libc::c_char,
        slave_ifname,
      );
      return res;
    }
  }
  if (*ptr_to_globals).master.mtu.ifr_ifru.ifru_mtu != (*ptr_to_globals).slave.mtu.ifr_ifru.ifru_mtu
  {
    res = set_mtu(slave_ifname, (*ptr_to_globals).master.mtu.ifr_ifru.ifru_mtu);
    if res != 0 {
      crate::libbb::perror_msg::bb_perror_msg(
        b"%s: can\'t set MTU\x00" as *const u8 as *const libc::c_char,
        slave_ifname,
      );
      return res;
    }
  }
  if (*ptr_to_globals).hwaddr_set != 0 {
    /* Master already has an hwaddr
     * so set it's hwaddr to the slave
     */
    if (*ptr_to_globals).abi_ver < 1i32 as libc::c_uint {
      /* The driver is using an old ABI, so
       * the application sets the slave's
       * hwaddr
       */
      if set_hwaddr(
        slave_ifname,
        &mut (*ptr_to_globals).master.hwaddr.ifr_ifru.ifru_hwaddr,
      ) != 0
      {
        crate::libbb::perror_msg::bb_perror_msg(
          b"%s: can\'t set hw address\x00" as *const u8 as *const libc::c_char,
          slave_ifname,
        );
        current_block = 18257456917340031726;
      } else if set_if_up(
        slave_ifname,
        (*ptr_to_globals).slave.flags.ifr_ifru.ifru_flags as libc::c_int,
      ) != 0
      {
        set_hwaddr(
          slave_ifname,
          &mut (*ptr_to_globals).slave.hwaddr.ifr_ifru.ifru_hwaddr,
        );
        current_block = 18257456917340031726;
      } else {
        current_block = 7226443171521532240;
      }
    } else {
      current_block = 7226443171521532240;
    }
  /* For old ABI the application needs to bring the
   * slave back up
   */
  /* The driver is using a new ABI,
   * so the driver takes care of setting
   * the slave's hwaddr and bringing
   * it up again
   */
  } else {
    /* No hwaddr for master yet, so
     * set the slave's hwaddr to it
     */
    if (*ptr_to_globals).abi_ver < 1i32 as libc::c_uint {
      /* For old ABI, the master needs to be
       * down before setting it's hwaddr
       */
      if set_if_down(
        master_ifname,
        (*ptr_to_globals).master.flags.ifr_ifru.ifru_flags as libc::c_int,
      ) != 0
      {
        current_block = 18257456917340031726;
      } else {
        current_block = 8704759739624374314;
      }
    } else {
      current_block = 8704759739624374314;
    }
    match current_block {
      18257456917340031726 => {}
      _ => {
        if set_hwaddr(
          master_ifname,
          &mut (*ptr_to_globals).slave.hwaddr.ifr_ifru.ifru_hwaddr,
        ) != 0
        {
          crate::libbb::verror_msg::bb_error_msg(
            b"%s: can\'t set hw address\x00" as *const u8 as *const libc::c_char,
            master_ifname,
          );
          current_block = 18257456917340031726;
        } else {
          if (*ptr_to_globals).abi_ver < 1i32 as libc::c_uint {
            /* For old ABI, bring the master
             * back up
             */
            if set_if_up(
              master_ifname,
              (*ptr_to_globals).master.flags.ifr_ifru.ifru_flags as libc::c_int,
            ) != 0
            {
              current_block = 14714681040628071429;
            } else {
              current_block = 7746103178988627676;
            }
          } else {
            current_block = 7746103178988627676;
          }
          match current_block {
            14714681040628071429 => {}
            _ => {
              (*ptr_to_globals).hwaddr_set = 1i32 as smallint;
              current_block = 7226443171521532240;
            }
          }
        }
      }
    }
  }
  match current_block {
    7226443171521532240 => {
      /* Do the real thing */
      crate::libbb::xfuncs::strncpy_IFNAMSIZ(ifr.ifr_ifru.ifru_slave.as_mut_ptr(), slave_ifname);
      if set_ifrname_and_do_ioctl(0x8990i32 as libc::c_uint, &mut ifr, master_ifname) != 0
        && ioctl_on_skfd(0x89f0i32 as libc::c_uint, &mut ifr) != 0
      {
        current_block = 14714681040628071429;
      } else {
        return 0;
      }
    }
    _ => {}
  }
  match current_block {
    14714681040628071429 =>
    /* rollback (best effort) */
    {
      set_hwaddr(
        master_ifname,
        &mut (*ptr_to_globals).master.hwaddr.ifr_ifru.ifru_hwaddr,
      );
      (*ptr_to_globals).hwaddr_set = 0 as smallint
    }
    _ => {}
  }
  set_mtu(slave_ifname, (*ptr_to_globals).slave.mtu.ifr_ifru.ifru_mtu);
  return 1i32;
}
unsafe extern "C" fn release(
  mut master_ifname: *mut libc::c_char,
  mut slave_ifname: *mut libc::c_char,
) -> libc::c_int {
  let mut ifr: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  let mut res: libc::c_int = 0;
  if (*ptr_to_globals).slave.flags.ifr_ifru.ifru_flags as libc::c_int & IFF_SLAVE as libc::c_int
    == 0
  {
    crate::libbb::verror_msg::bb_error_msg(
      b"%s is not a slave\x00" as *const u8 as *const libc::c_char,
      slave_ifname,
    );
    return 1i32;
  }
  crate::libbb::xfuncs::strncpy_IFNAMSIZ(ifr.ifr_ifru.ifru_slave.as_mut_ptr(), slave_ifname);
  if set_ifrname_and_do_ioctl(0x8991i32 as libc::c_uint, &mut ifr, master_ifname) < 0
    && ioctl_on_skfd((0x89f0i32 + 1i32) as libc::c_uint, &mut ifr) < 0
  {
    return 1i32;
  }
  if (*ptr_to_globals).abi_ver < 1i32 as libc::c_uint {
    /* The driver is using an old ABI, so we'll set the interface
     * down to avoid any conflicts due to same MAC/IP
     */
    res = set_if_down(
      slave_ifname,
      (*ptr_to_globals).slave.flags.ifr_ifru.ifru_flags as libc::c_int,
    )
  }
  /* set to default mtu */
  set_mtu(slave_ifname, 1500i32);
  return res;
}
#[inline(never)]
unsafe extern "C" fn get_drv_info(mut master_ifname: *mut libc::c_char) {
  let mut ifr: ifreq = ifreq {
    ifr_ifrn: C2RustUnnamed_1 { ifrn_name: [0; 16] },
    ifr_ifru: C2RustUnnamed {
      ifru_addr: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  let mut info: ethtool_drvinfo = ethtool_drvinfo {
    cmd: 0,
    driver: [0; 32],
    version: [0; 32],
    fw_version: [0; 32],
    bus_info: [0; 32],
    erom_version: [0; 32],
    reserved2: [0; 12],
    n_priv_flags: 0,
    n_stats: 0,
    testinfo_len: 0,
    eedump_len: 0,
    regdump_len: 0,
  };
  memset(
    &mut ifr as *mut ifreq as *mut libc::c_void,
    0,
    ::std::mem::size_of::<ifreq>() as libc::c_ulong,
  );
  ifr.ifr_ifru.ifru_data = &mut info as *mut ethtool_drvinfo as caddr_t as *mut libc::c_void;
  info.cmd = 0x3i32 as u32;
  /* both fields are 32 bytes long (long enough) */
  strcpy(
    info.driver.as_mut_ptr(),
    b"ifenslave\x00" as *const u8 as *const libc::c_char,
  );
  strcpy(
    info.fw_version.as_mut_ptr(),
    crate::libbb::xfuncs::utoa(2i32 as libc::c_uint),
  );
  if set_ifrname_and_do_ioctl(0x8946i32 as libc::c_uint, &mut ifr, master_ifname) < 0 {
    if *bb_errno == 95i32 {
      return;
    }
    crate::libbb::perror_msg::bb_perror_msg_and_die(
      b"%s: SIOCETHTOOL error\x00" as *const u8 as *const libc::c_char,
      master_ifname,
    );
  }
  (*ptr_to_globals).abi_ver = crate::libbb::bb_strtonum::bb_strtou(
    info.fw_version.as_mut_ptr(),
    0 as *mut *mut libc::c_char,
    0,
  );
  if *bb_errno != 0 {
    crate::libbb::verror_msg::bb_error_msg_and_die(
      b"%s: SIOCETHTOOL error\x00" as *const u8 as *const libc::c_char,
      master_ifname,
    );
  };
}
pub unsafe fn ifenslave_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut master_ifname: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut slave_ifname: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut rv: libc::c_int = 0;
  let mut res: libc::c_int = 0;
  let mut opt: libc::c_uint = 0;
  let ref mut fresh0 = *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
    as *mut *mut globals);
  *fresh0 = crate::libbb::xfuncs_printf::xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong)
    as *mut globals;
  llvm_asm!("" : : : "memory" : "volatile");
  opt = crate::libbb::getopt32::getopt32long(
    argv,
    b"cdfa\x00" as *const u8 as *const libc::c_char,
    b"change-active\x00\x00cdetach\x00\x00dforce\x00\x00f\x00" as *const u8 as *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  if opt & opt.wrapping_sub(1i32 as libc::c_uint) != 0 {
    /* Only one option can be given */
    crate::libbb::appletlib::bb_show_usage();
  }
  let fresh1 = argv;
  argv = argv.offset(1);
  master_ifname = *fresh1;
  /* No interface names - show all interfaces. */
  if master_ifname.is_null() {
    crate::networking::interface::display_interfaces(0 as *mut libc::c_char);
    return 0;
  }
  /* Open a basic socket */
  crate::libbb::xfuncs_printf::xmove_fd(
    crate::libbb::xfuncs_printf::xsocket(2i32, SOCK_DGRAM as libc::c_int, 0),
    skfd as libc::c_int,
  );
  /* Exchange abi version with bonding module */
  get_drv_info(master_ifname);
  let fresh2 = argv;
  argv = argv.offset(1);
  slave_ifname = *fresh2;
  if slave_ifname.is_null() {
    if opt & (OPT_d as libc::c_int | OPT_c as libc::c_int) as libc::c_uint != 0 {
      /* --change or --detach, and no slaves given -
       * show all interfaces. */
      crate::networking::interface::display_interfaces(slave_ifname);
      return 2i32;
      /* why 2? */
    }
    /* A single arg means show the
     * configuration for this interface
     */
    crate::networking::interface::display_interfaces(master_ifname);
    return 0;
  }
  if get_if_settings(master_ifname, &mut (*ptr_to_globals).master) != 0 {
    /* Probably a good reason not to go on */
    crate::libbb::perror_msg::bb_perror_msg_and_die(
      b"%s: can\'t get settings\x00" as *const u8 as *const libc::c_char,
      master_ifname,
    );
  }
  /* Check if master is indeed a master;
   * if not then fail any operation
   */
  if (*ptr_to_globals).master.flags.ifr_ifru.ifru_flags as libc::c_int & IFF_MASTER as libc::c_int
    == 0
  {
    crate::libbb::verror_msg::bb_error_msg_and_die(
      b"%s is not a master\x00" as *const u8 as *const libc::c_char,
      master_ifname,
    );
  }
  /* Check if master is up; if not then fail any operation */
  if (*ptr_to_globals).master.flags.ifr_ifru.ifru_flags as libc::c_int & IFF_UP as libc::c_int == 0
  {
    crate::libbb::verror_msg::bb_error_msg_and_die(
      b"%s is not up\x00" as *const u8 as *const libc::c_char,
      master_ifname,
    );
  }
  /* Accepts only one slave */
  if opt & OPT_c as libc::c_int as libc::c_uint != 0 {
    /* Change active slave */
    if get_slave_flags(slave_ifname) != 0 {
      crate::libbb::perror_msg::bb_perror_msg_and_die(
        b"%s: can\'t get flags\x00" as *const u8 as *const libc::c_char,
        slave_ifname,
      );
    }
    change_active(master_ifname, slave_ifname);
    return 0;
  }
  /* Accepts multiple slaves */
  res = 0;
  loop {
    if opt & OPT_d as libc::c_int as libc::c_uint != 0 {
      /* Detach a slave interface from the master */
      rv = get_slave_flags(slave_ifname);
      if rv != 0 {
        /* Can't work with this slave, */
        /* remember the error and skip it */
        crate::libbb::perror_msg::bb_perror_msg(
          b"skipping %s: can\'t get %s\x00" as *const u8 as *const libc::c_char,
          slave_ifname,
          b"flags\x00" as *const u8 as *const libc::c_char,
        );
        res = rv
      } else {
        rv = release(master_ifname, slave_ifname);
        if rv != 0 {
          crate::libbb::perror_msg::bb_perror_msg(
            b"can\'t release %s from %s\x00" as *const u8 as *const libc::c_char,
            slave_ifname,
            master_ifname,
          );
          res = rv
        }
      }
    } else {
      /* Attach a slave interface to the master */
      rv = get_if_settings(slave_ifname, &mut (*ptr_to_globals).slave);
      if rv != 0 {
        /* Can't work with this slave, */
        /* remember the error and skip it */
        crate::libbb::perror_msg::bb_perror_msg(
          b"skipping %s: can\'t get %s\x00" as *const u8 as *const libc::c_char,
          slave_ifname,
          b"settings\x00" as *const u8 as *const libc::c_char,
        );
        res = rv
      } else {
        rv = enslave(master_ifname, slave_ifname);
        if rv != 0 {
          crate::libbb::perror_msg::bb_perror_msg(
            b"can\'t enslave %s to %s\x00" as *const u8 as *const libc::c_char,
            slave_ifname,
            master_ifname,
          );
          res = rv
        }
      }
    }
    let fresh3 = argv;
    argv = argv.offset(1);
    slave_ifname = *fresh3;
    if slave_ifname.is_null() {
      break;
    }
  }
  return res;
}

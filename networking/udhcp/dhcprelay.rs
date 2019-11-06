use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
extern "C" {
  pub type sockaddr_x25;
  pub type sockaddr_un;
  pub type sockaddr_ns;
  pub type sockaddr_iso;
  pub type sockaddr_ipx;
  pub type sockaddr_inarp;
  pub type sockaddr_eon;
  pub type sockaddr_dl;
  pub type sockaddr_ax25;
  pub type sockaddr_at;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn select(
    __nfds: libc::c_int,
    __readfds: *mut fd_set,
    __writefds: *mut fd_set,
    __exceptfds: *mut fd_set,
    __timeout: *mut timeval,
  ) -> libc::c_int;
  #[no_mangle]
  fn sendto(
    __fd: libc::c_int,
    __buf: *const libc::c_void,
    __n: size_t,
    __flags: libc::c_int,
    __addr: __CONST_SOCKADDR_ARG,
    __addr_len: socklen_t,
  ) -> ssize_t;
  #[no_mangle]
  fn recvfrom(
    __fd: libc::c_int,
    __buf: *mut libc::c_void,
    __n: size_t,
    __flags: libc::c_int,
    __addr: __SOCKADDR_ARG,
    __addr_len: *mut socklen_t,
  ) -> ssize_t;
  #[no_mangle]
  fn udhcp_listen_socket(port: libc::c_int, inf: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn udhcp_read_interface(
    interface: *const libc::c_char,
    ifindex: *mut libc::c_int,
    nip: *mut uint32_t,
    mac: *mut uint8_t,
  ) -> libc::c_int;
  #[no_mangle]
  fn udhcp_recv_kernel_packet(packet: *mut dhcp_packet, fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn udhcp_get_option(packet: *mut dhcp_packet, code: libc::c_int) -> *mut uint8_t;
  #[no_mangle]
  fn inet_aton(__cp: *const libc::c_char, __inp: *mut in_addr) -> libc::c_int;
  /* Some useful definitions */
  /* Macros for min/max.  */
  /* buffer allocation schemes */
  /* glibc uses __errno_location() to get a ptr to errno */
  /* We can just memorize it once - no multithreading in busybox :) */
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn monotonic_sec() -> libc::c_uint;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

use crate::librb::__suseconds_t;
use crate::librb::__time_t;

pub type __socklen_t = libc::c_uint;
use crate::librb::size_t;
use crate::librb::ssize_t;
use libc::uint16_t;
use libc::uint32_t;
 use libc::uint8_t;
pub type socklen_t = __socklen_t;
 use libc::timeval;
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
  pub fds_bits: [__fd_mask; 16],
}
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
  pub sa_family: sa_family_t,
  pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
  pub __sockaddr__: *mut sockaddr,
  pub __sockaddr_at__: *mut sockaddr_at,
  pub __sockaddr_ax25__: *mut sockaddr_ax25,
  pub __sockaddr_dl__: *mut sockaddr_dl,
  pub __sockaddr_eon__: *mut sockaddr_eon,
  pub __sockaddr_in__: *mut sockaddr_in,
  pub __sockaddr_in6__: *mut sockaddr_in6,
  pub __sockaddr_inarp__: *mut sockaddr_inarp,
  pub __sockaddr_ipx__: *mut sockaddr_ipx,
  pub __sockaddr_iso__: *mut sockaddr_iso,
  pub __sockaddr_ns__: *mut sockaddr_ns,
  pub __sockaddr_un__: *mut sockaddr_un,
  pub __sockaddr_x25__: *mut sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
  pub sin6_family: sa_family_t,
  pub sin6_port: in_port_t,
  pub sin6_flowinfo: uint32_t,
  pub sin6_addr: in6_addr,
  pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
  pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub __u6_addr8: [uint8_t; 16],
  pub __u6_addr16: [uint16_t; 8],
  pub __u6_addr32: [uint32_t; 4],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
  pub __sockaddr__: *const sockaddr,
  pub __sockaddr_at__: *const sockaddr_at,
  pub __sockaddr_ax25__: *const sockaddr_ax25,
  pub __sockaddr_dl__: *const sockaddr_dl,
  pub __sockaddr_eon__: *const sockaddr_eon,
  pub __sockaddr_in__: *const sockaddr_in,
  pub __sockaddr_in6__: *const sockaddr_in6,
  pub __sockaddr_inarp__: *const sockaddr_inarp,
  pub __sockaddr_ipx__: *const sockaddr_ipx,
  pub __sockaddr_iso__: *const sockaddr_iso,
  pub __sockaddr_ns__: *const sockaddr_ns,
  pub __sockaddr_un__: *const sockaddr_un,
  pub __sockaddr_x25__: *const sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct dhcp_packet {
  pub op: uint8_t,
  pub htype: uint8_t,
  pub hlen: uint8_t,
  pub hops: uint8_t,
  pub xid: uint32_t,
  pub secs: uint16_t,
  pub flags: uint16_t,
  pub ciaddr: uint32_t,
  pub yiaddr: uint32_t,
  pub siaddr_nip: uint32_t,
  pub gateway_nip: uint32_t,
  pub chaddr: [uint8_t; 16],
  pub sname: [uint8_t; 64],
  pub file: [uint8_t; 128],
  pub cookie: uint32_t,
  pub options: [uint8_t; 388],
}
/* This list holds information about clients. The xid_* functions manipulate this list. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xid_item {
  pub timestamp: libc::c_uint,
  pub client: libc::c_int,
  pub xid: uint32_t,
  pub ip: sockaddr_in,
  pub next: *mut xid_item,
}
unsafe extern "C" fn xid_add(
  mut xid: uint32_t,
  mut ip: *mut sockaddr_in,
  mut client: libc::c_int,
) -> *mut xid_item {
  let mut item: *mut xid_item = 0 as *mut xid_item;
  /* create new xid entry */
  item = xmalloc(::std::mem::size_of::<xid_item>() as libc::c_ulong) as *mut xid_item;
  /* add xid entry */
  (*item).ip = *ip;
  (*item).xid = xid;
  (*item).client = client;
  (*item).timestamp = monotonic_sec();
  (*item).next = (*(bb_common_bufsiz1.as_mut_ptr() as *mut xid_item)).next;
  let ref mut fresh0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut xid_item)).next;
  *fresh0 = item;
  return item;
}
unsafe extern "C" fn xid_expire() {
  let mut item: *mut xid_item = (*(bb_common_bufsiz1.as_mut_ptr() as *mut xid_item)).next;
  let mut last: *mut xid_item = bb_common_bufsiz1.as_mut_ptr() as *mut xid_item;
  let mut current_time: libc::c_uint = monotonic_sec();
  while !item.is_null() {
    if current_time.wrapping_sub((*item).timestamp) > (2i32 * 60i32) as libc::c_uint {
      (*last).next = (*item).next;
      free(item as *mut libc::c_void);
      item = (*last).next
    } else {
      last = item;
      item = (*item).next
    }
  }
}
unsafe extern "C" fn xid_find(mut xid: uint32_t) -> *mut xid_item {
  let mut item: *mut xid_item = (*(bb_common_bufsiz1.as_mut_ptr() as *mut xid_item)).next;
  while !item.is_null() {
    if (*item).xid == xid {
      break;
    }
    item = (*item).next
  }
  return item;
}
unsafe extern "C" fn xid_del(mut xid: uint32_t) {
  let mut item: *mut xid_item = (*(bb_common_bufsiz1.as_mut_ptr() as *mut xid_item)).next;
  let mut last: *mut xid_item = bb_common_bufsiz1.as_mut_ptr() as *mut xid_item;
  while !item.is_null() {
    if (*item).xid == xid {
      (*last).next = (*item).next;
      free(item as *mut libc::c_void);
      item = (*last).next
    } else {
      last = item;
      item = (*item).next
    }
  }
}
/* *
 * get_dhcp_packet_type - gets the message type of a dhcp packet
 * p - pointer to the dhcp packet
 * returns the message type on success, -1 otherwise
 */
unsafe extern "C" fn get_dhcp_packet_type(mut p: *mut dhcp_packet) -> libc::c_int {
  let mut op: *mut uint8_t = 0 as *mut uint8_t;
  /* it must be either a BOOTREQUEST or a BOOTREPLY */
  if (*p).op as libc::c_int != 1i32 && (*p).op as libc::c_int != 2i32 {
    return -1i32;
  }
  /* get message type option */
  op = udhcp_get_option(p, 0x35i32);
  if !op.is_null() {
    return *op.offset(0) as libc::c_int;
  }
  return -1i32;
}
/* *
 * make_iface_list - parses client/server interface names
 * returns array
 */
unsafe extern "C" fn make_iface_list(
  mut client_and_server_ifaces: *mut *mut libc::c_char,
  mut client_number: *mut libc::c_int,
) -> *mut *mut libc::c_char {
  let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut iface_list: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut i: libc::c_int = 0;
  let mut cn: libc::c_int = 0;
  /* get number of items */
  cn = 2i32; /* 1 server iface + at least 1 client one */
  s = *client_and_server_ifaces.offset(0); /* list of client ifaces */
  while *s != 0 {
    if *s as libc::c_int == ',' as i32 {
      cn += 1
    }
    s = s.offset(1)
  }
  *client_number = cn;
  /* create vector of pointers */
  iface_list = xzalloc(
    (cn as libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
  ) as *mut *mut libc::c_char; /* server iface */
  let ref mut fresh1 = *iface_list.offset(0); /* list of client ifaces */
  *fresh1 = *client_and_server_ifaces.offset(1);
  i = 1i32;
  s = xstrdup(*client_and_server_ifaces.offset(0));
  'c_9337: loop {
    let fresh3 = i;
    i = i + 1;
    let ref mut fresh4 = *iface_list.offset(fresh3 as isize);
    *fresh4 = s;
    loop {
      if !(i < cn) {
        break 'c_9337;
      }
      let fresh2 = s;
      s = s.offset(1);
      if *fresh2 as libc::c_int == ',' as i32 {
        break;
      }
    }
    *s.offset(-1i32 as isize) = '\u{0}' as i32 as libc::c_char
  }
  return iface_list;
}
/* Creates listen sockets (in fds) bound to client and server ifaces,
 * and returns numerically max fd.
 */
unsafe extern "C" fn init_sockets(
  mut iface_list: *mut *mut libc::c_char,
  mut num_clients: libc::c_int,
  mut fds: *mut libc::c_int,
) -> libc::c_int {
  let mut i: libc::c_int = 0;
  let mut n: libc::c_int = 0;
  n = 0i32;
  i = 0i32;
  while i < num_clients {
    *fds.offset(i as isize) = udhcp_listen_socket(67i32, *iface_list.offset(i as isize));
    if n < *fds.offset(i as isize) {
      n = *fds.offset(i as isize)
    }
    i += 1
  }
  return n;
}
unsafe extern "C" fn sendto_ip4(
  mut sock: libc::c_int,
  mut msg: *const libc::c_void,
  mut msg_len: libc::c_int,
  mut to: *mut sockaddr_in,
) -> libc::c_int {
  let mut err: libc::c_int = 0;
  *bb_errno = 0i32;
  err = sendto(
    sock,
    msg,
    msg_len as size_t,
    0i32,
    __CONST_SOCKADDR_ARG {
      __sockaddr__: to as *mut sockaddr,
    },
    ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
  ) as libc::c_int;
  err -= msg_len;
  if err != 0 {
    bb_simple_perror_msg(b"sendto\x00" as *const u8 as *const libc::c_char);
  }
  return err;
}
/* *
 * pass_to_server() - forwards dhcp packets from client to server
 * p - packet to send
 * client - number of the client
 */
unsafe extern "C" fn pass_to_server(
  mut p: *mut dhcp_packet,
  mut packet_len: libc::c_int,
  mut client: libc::c_int,
  mut fds: *mut libc::c_int,
  mut client_addr: *mut sockaddr_in,
  mut server_addr: *mut sockaddr_in,
) {
  let mut type_0: libc::c_int = 0;
  /* check packet_type */
  type_0 = get_dhcp_packet_type(p);
  if type_0 != 1i32 && type_0 != 3i32 && type_0 != 4i32 && type_0 != 7i32 && type_0 != 8i32 {
    return;
  }
  /* create new xid entry */
  xid_add((*p).xid, client_addr, client);
  /* forward request to server */
  /* note that we send from fds[0] which is bound to SERVER_PORT (67).
   * IOW: we send _from_ SERVER_PORT! Although this may look strange,
   * RFC 1542 not only allows, but prescribes this for BOOTP relays.
   */
  sendto_ip4(
    *fds.offset(0),
    p as *const libc::c_void,
    packet_len,
    server_addr,
  );
}
/* *
 * pass_to_client() - forwards dhcp packets from server to client
 * p - packet to send
 */
unsafe extern "C" fn pass_to_client(
  mut p: *mut dhcp_packet,
  mut packet_len: libc::c_int,
  mut fds: *mut libc::c_int,
) {
  let mut type_0: libc::c_int = 0;
  let mut item: *mut xid_item = 0 as *mut xid_item;
  /* check xid */
  item = xid_find((*p).xid);
  if item.is_null() {
    return;
  }
  /* check packet type */
  type_0 = get_dhcp_packet_type(p);
  if type_0 != 2i32 && type_0 != 5i32 && type_0 != 6i32 {
    return;
  }
  //TODO: also do it if (p->flags & htons(BROADCAST_FLAG)) is set!
  if (*item).ip.sin_addr.s_addr
    == ({
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = 0i32 as in_addr_t;
      if 0 != 0 {
        __v = (__x & 0xff000000u32) >> 24i32
          | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
          | (__x & 0xff00i32 as libc::c_uint) << 8i32
          | (__x & 0xffi32 as libc::c_uint) << 24i32
      } else {
        let fresh5 = &mut __v;
        let fresh6;
        let fresh7 = __x;
        asm!("bswap $0" : "=r" (fresh6) : "0"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh5, fresh7))
                         :);
        c2rust_asm_casts::AsmCast::cast_out(fresh5, fresh7, fresh6);
      }
      __v
    })
  {
    (*item).ip.sin_addr.s_addr = {
      let mut __v: libc::c_uint = 0;
      let mut __x: libc::c_uint = 0xffffffffu32;
      if 0 != 0 {
        __v = (__x & 0xff000000u32) >> 24i32
          | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
          | (__x & 0xff00i32 as libc::c_uint) << 8i32
          | (__x & 0xffi32 as libc::c_uint) << 24i32
      } else {
        let fresh8 = &mut __v;
        let fresh9;
        let fresh10 = __x;
        asm!("bswap $0" : "=r" (fresh9) : "0"
                          (c2rust_asm_casts::AsmCast::cast_in(fresh8, fresh10))
                          :);
        c2rust_asm_casts::AsmCast::cast_out(fresh8, fresh10, fresh9);
      }
      __v
    }
  }
  if sendto_ip4(
    *fds.offset((*item).client as isize),
    p as *const libc::c_void,
    packet_len,
    &mut (*item).ip,
  ) != 0i32
  {
    return;
    /* send error occurred */
  }
  /* remove xid entry */
  xid_del((*p).xid);
}
#[no_mangle]
pub unsafe extern "C" fn dhcprelay_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut server_addr: sockaddr_in = sockaddr_in {
    sin_family: 0,
    sin_port: 0,
    sin_addr: in_addr { s_addr: 0 },
    sin_zero: [0; 8],
  };
  let mut iface_list: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut fds: *mut libc::c_int = 0 as *mut libc::c_int;
  let mut num_sockets: libc::c_int = 0;
  let mut max_socket: libc::c_int = 0;
  let mut our_nip: uint32_t = 0;
  server_addr.sin_family = 2i32 as sa_family_t;
  server_addr.sin_addr.s_addr = {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = 0xffffffffu32;
    if 0 != 0 {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh11 = &mut __v;
      let fresh12;
      let fresh13 = __x;
      asm!("bswap $0" : "=r" (fresh12) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh11, fresh13))
                      :);
      c2rust_asm_casts::AsmCast::cast_out(fresh11, fresh13, fresh12);
    }
    __v
  };
  server_addr.sin_port = {
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = 67i32 as libc::c_ushort;
    if 0 != 0 {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh14 = &mut __v;
      let fresh15;
      let fresh16 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh15) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh14, fresh16)) :
                      "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh14, fresh16, fresh15);
    }
    __v
  };
  /* dhcprelay CLIENT_IFACE1[,CLIENT_IFACE2...] SERVER_IFACE [SERVER_IP] */
  if (*argv.offset(1)).is_null() || (*argv.offset(2)).is_null() {
    bb_show_usage();
  }
  if !(*argv.offset(3)).is_null() {
    if inet_aton(*argv.offset(3), &mut server_addr.sin_addr) == 0 {
      bb_simple_perror_msg_and_die(b"bad server IP\x00" as *const u8 as *const libc::c_char);
    }
  }
  iface_list = make_iface_list(argv.offset(1), &mut num_sockets);
  fds = xmalloc(
    (num_sockets as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
  ) as *mut libc::c_int;
  /* Create sockets and bind one to every iface */
  max_socket = init_sockets(iface_list, num_sockets, fds);
  /* Get our IP on server_iface */
  if udhcp_read_interface(
    *argv.offset(2),
    0 as *mut libc::c_int,
    &mut our_nip,
    0 as *mut uint8_t,
  ) != 0
  {
    return 1i32;
  }
  loop
  /* Main loop */
  // reinit stuff from time to time? go back to make_iface_list
  // every N minutes?
  {
    let mut rfds: fd_set = fd_set { fds_bits: [0; 16] };
    let mut tv: timeval = timeval {
      tv_sec: 0,
      tv_usec: 0,
    };
    let mut i: libc::c_int = 0;
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh17 = &mut __d0;
    let fresh18;
    let fresh19 = &mut __d1;
    let fresh20;
    let fresh21 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh22 = &mut *rfds.fds_bits.as_mut_ptr().offset(0) as *mut __fd_mask;
    asm!("cld; rep; stosq" : "={cx}" (fresh18), "={di}" (fresh20) : "{ax}"
             (0i32), "0"
             (c2rust_asm_casts::AsmCast::cast_in(fresh17, fresh21)), "1"
             (c2rust_asm_casts::AsmCast::cast_in(fresh19, fresh22)) : "memory"
             : "volatile");
    c2rust_asm_casts::AsmCast::cast_out(fresh17, fresh21, fresh18);
    c2rust_asm_casts::AsmCast::cast_out(fresh19, fresh22, fresh20);
    i = 0i32;
    while i < num_sockets {
      rfds.fds_bits[(*fds.offset(i as isize)
        / (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize] |= (1u64
        << *fds.offset(i as isize)
          % (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as __fd_mask;
      i += 1
    }
    tv.tv_sec = (2i32 * 60i32 / 8i32) as __time_t;
    tv.tv_usec = 0i32 as __suseconds_t;
    if select(
      max_socket + 1i32,
      &mut rfds,
      0 as *mut fd_set,
      0 as *mut fd_set,
      &mut tv,
    ) > 0i32
    {
      let mut packlen: libc::c_int = 0;
      let mut dhcp_msg: dhcp_packet = dhcp_packet {
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
      /* server */
      if rfds.fds_bits[(*fds.offset(0)
        / (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        & (1u64
          << *fds.offset(0)
            % (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
          as __fd_mask
        != 0i32 as libc::c_long
      {
        packlen = udhcp_recv_kernel_packet(&mut dhcp_msg, *fds.offset(0));
        if packlen > 0i32 {
          pass_to_client(&mut dhcp_msg, packlen, fds);
        }
      }
      /* clients */
      i = 1i32;
      while i < num_sockets {
        let mut client_addr: sockaddr_in = sockaddr_in {
          sin_family: 0,
          sin_port: 0,
          sin_addr: in_addr { s_addr: 0 },
          sin_zero: [0; 8],
        };
        let mut addr_size: socklen_t = 0;
        if rfds.fds_bits[(*fds.offset(i as isize)
          / (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
          as usize]
          & (1u64
            << *fds.offset(i as isize)
              % (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as __fd_mask
          != 0i32 as libc::c_long
        {
          addr_size = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
          packlen = recvfrom(
            *fds.offset(i as isize),
            &mut dhcp_msg as *mut dhcp_packet as *mut libc::c_void,
            ::std::mem::size_of::<dhcp_packet>() as libc::c_ulong,
            0i32,
            __SOCKADDR_ARG {
              __sockaddr__: &mut client_addr as *mut sockaddr_in as *mut sockaddr,
            },
            &mut addr_size,
          ) as libc::c_int;
          if !(packlen <= 0i32) {
            /* Get our IP on corresponding client_iface */
            // RFC 1542
            // 4.1 General BOOTP Processing for Relay Agents
            // 4.1.1 BOOTREQUEST Messages
            //   If the relay agent does decide to relay the request, it MUST examine
            //   the 'giaddr' ("gateway" IP address) field.  If this field is zero,
            //   the relay agent MUST fill this field with the IP address of the
            //   interface on which the request was received.  If the interface has
            //   more than one IP address logically associated with it, the relay
            //   agent SHOULD choose one IP address associated with that interface and
            //   use it consistently for all BOOTP messages it relays.  If the
            //   'giaddr' field contains some non-zero value, the 'giaddr' field MUST
            //   NOT be modified.  The relay agent MUST NOT, under any circumstances,
            //   fill the 'giaddr' field with a broadcast address as is suggested in
            //   [1] (Section 8, sixth paragraph).
            // but why? what if server can't route such IP? Client ifaces may be, say, NATed!
            // 4.1.2 BOOTREPLY Messages
            //   BOOTP relay agents relay BOOTREPLY messages only to BOOTP clients.
            //   It is the responsibility of BOOTP servers to send BOOTREPLY messages
            //   directly to the relay agent identified in the 'giaddr' field.
            // (yeah right, unless it is impossible... see comment above)
            //   Therefore, a relay agent may assume that all BOOTREPLY messages it
            //   receives are intended for BOOTP clients on its directly-connected
            //   networks.
            //
            //   When a relay agent receives a BOOTREPLY message, it should examine
            //   the BOOTP 'giaddr', 'yiaddr', 'chaddr', 'htype', and 'hlen' fields.
            //   These fields should provide adequate information for the relay agent
            //   to deliver the BOOTREPLY message to the client.
            //
            //   The 'giaddr' field can be used to identify the logical interface from
            //   which the reply must be sent (i.e., the host or router interface
            //   connected to the same network as the BOOTP client).  If the content
            //   of the 'giaddr' field does not match one of the relay agent's
            //   directly-connected logical interfaces, the BOOTREPLY message MUST be
            //   silently discarded.
            if udhcp_read_interface(
              *iface_list.offset(i as isize),
              0 as *mut libc::c_int,
              &mut dhcp_msg.gateway_nip,
              0 as *mut uint8_t,
            ) != 0
            {
              /* Fall back to our IP on server iface */
              // this makes more sense!
              dhcp_msg.gateway_nip = our_nip
            }
            // maybe dhcp_msg.hops++? drop packets with too many hops (RFC 1542 says 4 or 16)?
            pass_to_server(
              &mut dhcp_msg,
              packlen,
              i,
              fds,
              &mut client_addr,
              &mut server_addr,
            );
          }
        }
        i += 1
      }
    }
    xid_expire();
  }
  /* while (1) */
  /* return 0; - not reached */
}

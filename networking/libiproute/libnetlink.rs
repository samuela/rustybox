use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::xfuncs_printf::xmalloc;
use crate::librb::rtattr;
use crate::librb::size_t;
use crate::librb::socklen_t;
use libc;
use libc::free;
use libc::nlmsghdr;
use libc::sockaddr;
use libc::sockaddr_nl;
use libc::ssize_t;
use libc::time;
use libc::time_t;
extern "C" {
  #[no_mangle]
  fn recvmsg(__fd: libc::c_int, __message: *mut msghdr, __flags: libc::c_int) -> ssize_t;
  #[no_mangle]
  fn sendmsg(__fd: libc::c_int, __message: *const msghdr, __flags: libc::c_int) -> ssize_t;
  #[no_mangle]
  fn recv(
    __fd: libc::c_int,
    __buf: *mut libc::c_void,
    __n: size_t,
    __flags: libc::c_int,
  ) -> ssize_t;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iovec {
  pub iov_base: *mut libc::c_void,
  pub iov_len: size_t,
}

pub type __socklen_t = libc::c_uint;
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msghdr {
  pub msg_name: *mut libc::c_void,
  pub msg_namelen: socklen_t,
  pub msg_iov: *mut iovec,
  pub msg_iovlen: size_t,
  pub msg_control: *mut libc::c_void,
  pub msg_controllen: size_t,
  pub msg_flags: libc::c_int,
}

pub type bb__aliased_u32 = u32;
pub type __u16 = libc::c_ushort;
pub type u32 = libc::c_uint;
pub type __kernel_sa_family_t = libc::c_ushort;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nlmsgerr {
  pub error: libc::c_int,
  pub msg: nlmsghdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtgenmsg {
  pub rtgen_family: libc::c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtnl_handle {
  pub fd: libc::c_int,
  pub local: sockaddr_nl,
  pub peer: sockaddr_nl,
  pub seq: u32,
  pub dump: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_0 {
  pub nlh: nlmsghdr,
  pub g: rtgenmsg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
  pub nlh: nlmsghdr,
  pub msg: msghdr,
  pub nladdr: sockaddr_nl,
}
#[inline(always)]
unsafe extern "C" fn rtnl_send(
  mut rth: *mut rtnl_handle,
  mut buf: *const libc::c_void,
  mut len: libc::c_int,
) {
  crate::libbb::xfuncs_printf::xwrite((*rth).fd, buf, len as size_t);
}

/*
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version
 * 2 of the License, or (at your option) any later version.
 *
 * Authors: Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
 */
pub unsafe fn xrtnl_open(mut rth: *mut rtnl_handle)
/*, unsigned subscriptions*/
{
  memset(
    rth as *mut libc::c_void,
    0,
    ::std::mem::size_of::<rtnl_handle>() as libc::c_ulong,
  );
  (*rth).fd = crate::libbb::xfuncs_printf::xsocket(16i32, SOCK_RAW as libc::c_int, 0);
  (*rth).local.nl_family = 16i32 as __kernel_sa_family_t;
  /*rth->local.nl_groups = subscriptions;*/
  crate::libbb::xfuncs_printf::xbind(
    (*rth).fd,
    &mut (*rth).local as *mut sockaddr_nl as *mut sockaddr,
    ::std::mem::size_of::<sockaddr_nl>() as libc::c_ulong as socklen_t,
  );
  crate::libbb::bb_getsockname::bb_getsockname(
    (*rth).fd,
    &mut (*rth).local as *mut sockaddr_nl as *mut sockaddr as *mut libc::c_void,
    ::std::mem::size_of::<sockaddr_nl>() as libc::c_ulong as socklen_t,
  );
  /* too much paranoia
    if (getsockname(rth->fd, (struct sockaddr*)&rth->local, &addr_len) < 0)
      bb_perror_msg_and_die("getsockname");
    if (addr_len != sizeof(rth->local))
      bb_error_msg_and_die("wrong address length %d", addr_len);
    if (rth->local.nl_family != AF_NETLINK)
      bb_error_msg_and_die("wrong address family %d", rth->local.nl_family);
  */
  (*rth).seq = time(0 as *mut time_t) as u32;
}
pub unsafe fn xrtnl_wilddump_request(
  mut rth: *mut rtnl_handle,
  mut family: libc::c_int,
  mut type_0: libc::c_int,
) {
  let mut req: C2RustUnnamed_0 = std::mem::zeroed();
  req.nlh.nlmsg_len = ::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong as u32;
  req.nlh.nlmsg_type = type_0 as __u16;
  req.nlh.nlmsg_flags = (0x100i32 | 0x200i32 | 0x1i32) as __u16;
  req.nlh.nlmsg_pid = 0 as u32;
  (*rth).seq = (*rth).seq.wrapping_add(1);
  (*rth).dump = (*rth).seq;
  req.nlh.nlmsg_seq = (*rth).dump;
  req.g.rtgen_family = family as libc::c_uchar;
  rtnl_send(
    rth,
    &mut req as *mut C2RustUnnamed_0 as *mut libc::c_void,
    ::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong as libc::c_int,
  );
}
/* A version which checks for e.g. EPERM errors.
 * Try: setuidgid 1:1 ip addr flush dev eth0
 */
pub unsafe fn rtnl_send_check(
  mut rth: *mut rtnl_handle,
  mut buf: *const libc::c_void,
  mut len: libc::c_int,
) -> libc::c_int {
  let mut h: *mut nlmsghdr = std::ptr::null_mut();
  let mut status: libc::c_int = 0;
  let mut resp: [libc::c_char; 1024] = [0; 1024];
  status = libc::write((*rth).fd, buf, len as usize) as libc::c_int;
  if status < 0 {
    return status;
  }
  /* Check for immediate errors */
  status = recv(
    (*rth).fd,
    resp.as_mut_ptr() as *mut libc::c_void,
    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    MSG_DONTWAIT as libc::c_int | MSG_PEEK as libc::c_int,
  ) as libc::c_int;
  if status < 0 {
    if *bb_errno == 11i32 {
      /* if no error, this happens */
      return 0;
    }
    return -1i32;
  }
  h = resp.as_mut_ptr() as *mut nlmsghdr;
  while status >= ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong as libc::c_int
    && (*h).nlmsg_len as libc::c_ulong >= ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong
    && (*h).nlmsg_len <= status as libc::c_uint
  {
    if (*h).nlmsg_type as libc::c_int == 0x2i32 {
      let mut err: *mut nlmsgerr = (h as *mut libc::c_char).offset(
        (0i32
          + ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int)
          as isize,
      ) as *mut libc::c_void as *mut nlmsgerr;
      if ((*h).nlmsg_len as libc::c_ulong)
        < (::std::mem::size_of::<nlmsgerr>() as libc::c_ulong).wrapping_add(
          ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
            .wrapping_add(4u32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
            & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int
            as libc::c_ulong,
        )
      {
        crate::libbb::verror_msg::bb_simple_error_msg(
          b"ERROR truncated\x00" as *const u8 as *const libc::c_char,
        );
      } else {
        *bb_errno = -(*err).error
      }
      return -1i32;
    }
    status = (status as libc::c_uint).wrapping_sub(
      (*h)
        .nlmsg_len
        .wrapping_add(4u32)
        .wrapping_sub(1i32 as libc::c_uint)
        & !4u32.wrapping_sub(1i32 as libc::c_uint),
    ) as libc::c_int as libc::c_int;
    h = (h as *mut libc::c_char).offset(
      ((*h)
        .nlmsg_len
        .wrapping_add(4u32)
        .wrapping_sub(1i32 as libc::c_uint)
        & !4u32.wrapping_sub(1i32 as libc::c_uint)) as isize,
    ) as *mut nlmsghdr
  }
  return 0;
}
pub unsafe fn rtnl_dump_request(
  mut rth: *mut rtnl_handle,
  mut type_0: libc::c_int,
  mut req: *mut libc::c_void,
  mut len: libc::c_int,
) -> libc::c_int {
  let mut s: C2RustUnnamed_1 = std::mem::zeroed();
  let mut iov: [iovec; 2] = [
    {
      let mut init = iovec {
        iov_base: &mut s.nlh as *mut nlmsghdr as *mut libc::c_void,
        iov_len: ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong,
      };
      init
    },
    {
      let mut init = iovec {
        iov_base: req,
        iov_len: len as size_t,
      };
      init
    },
  ];
  memset(
    &mut s as *mut C2RustUnnamed_1 as *mut libc::c_void,
    0,
    ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong,
  );
  s.msg.msg_name = &mut s.nladdr as *mut sockaddr_nl as *mut libc::c_void;
  s.msg.msg_namelen = ::std::mem::size_of::<sockaddr_nl>() as libc::c_ulong as socklen_t;
  s.msg.msg_iov = iov.as_mut_ptr();
  s.msg.msg_iovlen = 2i32 as size_t;
  /*s.msg.msg_control = NULL; - already is */
  /*s.msg.msg_controllen = 0; - already is */
  /*s.msg.msg_flags = 0; - already is */
  s.nladdr.nl_family = 16i32 as __kernel_sa_family_t;
  s.nlh.nlmsg_len = (len
    + ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
      .wrapping_add(4u32 as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong)
      & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong) as libc::c_int)
    as u32;
  s.nlh.nlmsg_type = type_0 as __u16;
  s.nlh.nlmsg_flags = (0x100i32 | 0x200i32 | 0x1i32) as __u16;
  /*s.nlh.nlmsg_pid = 0; - already is */
  (*rth).seq = (*rth).seq.wrapping_add(1);
  (*rth).dump = (*rth).seq;
  s.nlh.nlmsg_seq = (*rth).dump;
  return sendmsg((*rth).fd, &mut s.msg, 0) as libc::c_int;
}
unsafe extern "C" fn rtnl_dump_filter(
  mut rth: *mut rtnl_handle,
  mut filter: Option<
    unsafe fn(_: *const sockaddr_nl, _: *mut nlmsghdr, _: *mut libc::c_void) -> libc::c_int,
  >,
  mut arg1: *mut libc::c_void,
) -> libc::c_int
/*,
		int (*junk)(struct sockaddr_nl *, struct nlmsghdr *n, void *),
		void *arg2*/ {
  let mut current_block: u64; /* avoid big stack buffer */
  let mut retval: libc::c_int = -1i32; /* while (1) */
  let mut buf: *mut libc::c_char = xmalloc((8i32 * 1024i32) as size_t) as *mut libc::c_char;
  let mut nladdr: sockaddr_nl = std::mem::zeroed();
  let mut iov: iovec = {
    let mut init = iovec {
      iov_base: buf as *mut libc::c_void,
      iov_len: (8i32 * 1024i32) as size_t,
    };
    init
  };
  's_17: loop {
    let mut status: libc::c_int = 0;
    let mut h: *mut nlmsghdr = std::ptr::null_mut();
    /* Use designated initializers, struct layout is non-portable */
    let mut msg: msghdr = {
      let mut init = msghdr {
        msg_name: &mut nladdr as *mut sockaddr_nl as *mut libc::c_void,
        msg_namelen: ::std::mem::size_of::<sockaddr_nl>() as libc::c_ulong as socklen_t,
        msg_iov: &mut iov,
        msg_iovlen: 1i32 as size_t,
        msg_control: std::ptr::null_mut(),
        msg_controllen: 0 as size_t,
        msg_flags: 0,
      };
      init
    };
    status = recvmsg((*rth).fd, &mut msg, 0) as libc::c_int;
    if status < 0 {
      if *bb_errno == 4i32 {
        continue;
      }
      crate::libbb::perror_msg::bb_simple_perror_msg(
        b"OVERRUN\x00" as *const u8 as *const libc::c_char,
      );
    } else if status == 0 {
      crate::libbb::verror_msg::bb_simple_error_msg(
        b"EOF on netlink\x00" as *const u8 as *const libc::c_char,
      );
      current_block = 2982084649171948612;
      break;
    } else {
      if msg.msg_namelen as libc::c_ulong != ::std::mem::size_of::<sockaddr_nl>() as libc::c_ulong {
        crate::libbb::verror_msg::bb_error_msg_and_die(
          b"sender address length == %d\x00" as *const u8 as *const libc::c_char,
          msg.msg_namelen,
        );
      }
      h = buf as *mut nlmsghdr;
      while status >= ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong as libc::c_int
        && (*h).nlmsg_len as libc::c_ulong >= ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong
        && (*h).nlmsg_len <= status as libc::c_uint
      {
        let mut err: libc::c_int = 0;
        if !(nladdr.nl_pid != 0 as libc::c_uint
          || (*h).nlmsg_pid != (*rth).local.nl_pid
          || (*h).nlmsg_seq != (*rth).dump)
        {
          if (*h).nlmsg_type as libc::c_int == 0x3i32 {
            current_block = 11190378652498320402;
            break 's_17;
          }
          if (*h).nlmsg_type as libc::c_int == 0x2i32 {
            let mut l_err: *mut nlmsgerr = (h as *mut libc::c_char).offset(
              (0i32
                + ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
                  .wrapping_add(4u32 as libc::c_ulong)
                  .wrapping_sub(1i32 as libc::c_ulong)
                  & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
                  as libc::c_int) as isize,
            ) as *mut libc::c_void as *mut nlmsgerr;
            if ((*h).nlmsg_len as libc::c_ulong)
              < (::std::mem::size_of::<nlmsgerr>() as libc::c_ulong).wrapping_add(
                ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
                  .wrapping_add(4u32 as libc::c_ulong)
                  .wrapping_sub(1i32 as libc::c_ulong)
                  & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
                  as libc::c_int as libc::c_ulong,
              )
            {
              crate::libbb::verror_msg::bb_simple_error_msg(
                b"ERROR truncated\x00" as *const u8 as *const libc::c_char,
              );
            } else {
              *bb_errno = -(*l_err).error;
              crate::libbb::perror_msg::bb_simple_perror_msg(
                b"RTNETLINK answers\x00" as *const u8 as *const libc::c_char,
              );
            }
            current_block = 2982084649171948612;
            break 's_17;
          } else {
            err = filter.expect("non-null function pointer")(&mut nladdr, h, arg1);
            if err < 0 {
              retval = err;
              current_block = 2982084649171948612;
              break 's_17;
            }
          }
        }
        //				if (junk) {
        //					err = junk(&nladdr, h, arg2);
        //					if (err < 0) {
        //						retval = err;
        //						goto ret;
        //					}
        //				}
        status = (status as libc::c_uint).wrapping_sub(
          (*h)
            .nlmsg_len
            .wrapping_add(4u32)
            .wrapping_sub(1i32 as libc::c_uint)
            & !4u32.wrapping_sub(1i32 as libc::c_uint),
        ) as libc::c_int as libc::c_int; /* = 0 */
        h = (h as *mut libc::c_char).offset(
          ((*h)
            .nlmsg_len
            .wrapping_add(4u32)
            .wrapping_sub(1i32 as libc::c_uint)
            & !4u32.wrapping_sub(1i32 as libc::c_uint)) as isize,
        ) as *mut nlmsghdr
      }
      if msg.msg_flags & MSG_TRUNC as libc::c_int != 0 {
        crate::libbb::verror_msg::bb_simple_error_msg(
          b"message truncated\x00" as *const u8 as *const libc::c_char,
        );
      } else if status != 0 {
        crate::libbb::verror_msg::bb_error_msg_and_die(
          b"remnant of size %d!\x00" as *const u8 as *const libc::c_char,
          status,
        );
      }
    }
  }
  match current_block {
    11190378652498320402 => retval += 1,
    _ => {}
  }
  free(buf as *mut libc::c_void);
  return retval;
}
pub unsafe fn xrtnl_dump_filter(
  mut rth: *mut rtnl_handle,
  mut filter: Option<
    unsafe fn(_: *const sockaddr_nl, _: *mut nlmsghdr, _: *mut libc::c_void) -> libc::c_int,
  >,
  mut arg1: *mut libc::c_void,
) -> libc::c_int {
  let mut ret: libc::c_int = rtnl_dump_filter(rth, filter, arg1);
  if ret < 0 {
    crate::libbb::verror_msg::bb_simple_error_msg_and_die(
      b"dump terminated\x00" as *const u8 as *const libc::c_char,
    );
  }
  return ret;
}
pub unsafe fn rtnl_talk(
  mut rtnl: *mut rtnl_handle,
  mut n: *mut nlmsghdr,
  mut answer: *mut nlmsghdr,
) -> libc::c_int {
  let mut current_block: u64;
  /* bbox doesn't use parameters no. 3, 4, 6, 7, they are stubbed out */
  let mut retval: libc::c_int = -1i32; /* avoid big stack buffer */
  let mut status: libc::c_int = 0;
  let mut seq: libc::c_uint = 0;
  let mut h: *mut nlmsghdr = std::ptr::null_mut();
  let mut nladdr: sockaddr_nl = std::mem::zeroed();
  let mut iov: iovec = {
    let mut init = iovec {
      iov_base: n as *mut libc::c_void,
      iov_len: (*n).nlmsg_len as size_t,
    };
    init
  };
  let mut buf: *mut libc::c_char = xmalloc((8i32 * 1024i32) as size_t) as *mut libc::c_char;
  /* Use designated initializers, struct layout is non-portable */
  let mut msg: msghdr = {
    let mut init = msghdr {
      msg_name: &mut nladdr as *mut sockaddr_nl as *mut libc::c_void,
      msg_namelen: ::std::mem::size_of::<sockaddr_nl>() as libc::c_ulong as socklen_t,
      msg_iov: &mut iov,
      msg_iovlen: 1i32 as size_t,
      msg_control: std::ptr::null_mut(),
      msg_controllen: 0 as size_t,
      msg_flags: 0,
    };
    init
  };
  memset(
    &mut nladdr as *mut sockaddr_nl as *mut libc::c_void,
    0,
    ::std::mem::size_of::<sockaddr_nl>() as libc::c_ulong,
  );
  nladdr.nl_family = 16i32 as __kernel_sa_family_t;
  //	nladdr.nl_pid = peer;
  //	nladdr.nl_groups = groups;
  (*rtnl).seq = (*rtnl).seq.wrapping_add(1); /* while (1) */
  seq = (*rtnl).seq;
  (*n).nlmsg_seq = seq;
  if answer.is_null() {
    (*n).nlmsg_flags = ((*n).nlmsg_flags as libc::c_int | 0x4i32) as __u16
  }
  status = sendmsg((*rtnl).fd, &mut msg, 0) as libc::c_int;
  if status < 0 {
    crate::libbb::perror_msg::bb_simple_perror_msg(
      b"can\'t talk to rtnetlink\x00" as *const u8 as *const libc::c_char,
    );
  } else {
    iov.iov_base = buf as *mut libc::c_void;
    's_76: loop {
      iov.iov_len = (8i32 * 1024i32) as size_t;
      status = recvmsg((*rtnl).fd, &mut msg, 0) as libc::c_int;
      if status < 0 {
        if *bb_errno == 4i32 {
          continue;
        }
        crate::libbb::perror_msg::bb_simple_perror_msg(
          b"OVERRUN\x00" as *const u8 as *const libc::c_char,
        );
      } else if status == 0 {
        crate::libbb::verror_msg::bb_simple_error_msg(
          b"EOF on netlink\x00" as *const u8 as *const libc::c_char,
        );
        current_block = 14567512515169274304;
        break;
      } else {
        if msg.msg_namelen as libc::c_ulong != ::std::mem::size_of::<sockaddr_nl>() as libc::c_ulong
        {
          crate::libbb::verror_msg::bb_error_msg_and_die(
            b"sender address length == %d\x00" as *const u8 as *const libc::c_char,
            msg.msg_namelen,
          );
        }
        h = buf as *mut nlmsghdr;
        //				if (junk) {
        //					l_err = junk(&nladdr, h, jarg);
        //					if (l_err < 0) {
        //						retval = l_err;
        //						goto ret;
        //					}
        //				}
        while status >= ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong as libc::c_int {
          //			int l_err;
          let mut len: libc::c_int = (*h).nlmsg_len as libc::c_int; /* = 0 */
          let mut l: libc::c_int = (len as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
            as libc::c_int;
          if l < 0 || len > status {
            if msg.msg_flags & MSG_TRUNC as libc::c_int != 0 {
              crate::libbb::verror_msg::bb_simple_error_msg(
                b"truncated message\x00" as *const u8 as *const libc::c_char,
              );
              current_block = 14567512515169274304;
              break 's_76;
            } else {
              crate::libbb::verror_msg::bb_error_msg_and_die(
                b"malformed message: len=%d!\x00" as *const u8 as *const libc::c_char,
                len,
              );
            }
          } else {
            if nladdr.nl_pid != 0 as libc::c_uint
              || (*h).nlmsg_pid != (*rtnl).local.nl_pid
              || (*h).nlmsg_seq != seq
            {
              continue;
            }
            if (*h).nlmsg_type as libc::c_int == 0x2i32 {
              let mut err: *mut nlmsgerr = (h as *mut libc::c_char).offset(
                (0i32
                  + ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
                    .wrapping_add(4u32 as libc::c_ulong)
                    .wrapping_sub(1i32 as libc::c_ulong)
                    & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
                    as libc::c_int) as isize,
              ) as *mut libc::c_void as *mut nlmsgerr;
              if l < ::std::mem::size_of::<nlmsgerr>() as libc::c_ulong as libc::c_int {
                crate::libbb::verror_msg::bb_simple_error_msg(
                  b"ERROR truncated\x00" as *const u8 as *const libc::c_char,
                );
                current_block = 14567512515169274304;
                break 's_76;
              } else {
                *bb_errno = -(*err).error;
                if *bb_errno == 0 {
                  if !answer.is_null() {
                    memcpy(
                      answer as *mut libc::c_void,
                      h as *const libc::c_void,
                      (*h).nlmsg_len as libc::c_ulong,
                    );
                  }
                  current_block = 15591632685197371519;
                  break 's_76;
                } else {
                  crate::libbb::perror_msg::bb_simple_perror_msg(
                    b"RTNETLINK answers\x00" as *const u8 as *const libc::c_char,
                  );
                  current_block = 14567512515169274304;
                  break 's_76;
                }
              }
            } else if !answer.is_null() {
              memcpy(
                answer as *mut libc::c_void,
                h as *const libc::c_void,
                (*h).nlmsg_len as libc::c_ulong,
              );
              current_block = 15591632685197371519;
              break 's_76;
            } else {
              crate::libbb::verror_msg::bb_simple_error_msg(
                b"unexpected reply!\x00" as *const u8 as *const libc::c_char,
              );
              status = (status as libc::c_uint).wrapping_sub(
                (len as libc::c_uint)
                  .wrapping_add(4u32)
                  .wrapping_sub(1i32 as libc::c_uint)
                  & !4u32.wrapping_sub(1i32 as libc::c_uint),
              ) as libc::c_int as libc::c_int;
              h = (h as *mut libc::c_char).offset(
                ((len as libc::c_uint)
                  .wrapping_add(4u32)
                  .wrapping_sub(1i32 as libc::c_uint)
                  & !4u32.wrapping_sub(1i32 as libc::c_uint)) as isize,
              ) as *mut nlmsghdr
            }
          }
        }
        if msg.msg_flags & MSG_TRUNC as libc::c_int != 0 {
          crate::libbb::verror_msg::bb_simple_error_msg(
            b"message truncated\x00" as *const u8 as *const libc::c_char,
          );
        } else if status != 0 {
          crate::libbb::verror_msg::bb_error_msg_and_die(
            b"remnant of size %d!\x00" as *const u8 as *const libc::c_char,
            status,
          );
        }
      }
    }
    match current_block {
      14567512515169274304 => {}
      _ => retval += 1,
    }
  }
  free(buf as *mut libc::c_void);
  return retval;
}
pub unsafe fn addattr32(
  mut n: *mut nlmsghdr,
  mut maxlen: libc::c_int,
  mut type_0: libc::c_int,
  mut data: u32,
) -> libc::c_int {
  let mut len: libc::c_int = ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
    .wrapping_add(4u32 as libc::c_ulong)
    .wrapping_sub(1i32 as libc::c_ulong)
    & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
    .wrapping_add(4i32 as libc::c_ulong) as libc::c_int;
  let mut rta: *mut rtattr = std::ptr::null_mut();
  if ((*n)
    .nlmsg_len
    .wrapping_add(len as libc::c_uint)
    .wrapping_add(4u32)
    .wrapping_sub(1i32 as libc::c_uint)
    & !4u32.wrapping_sub(1i32 as libc::c_uint)) as libc::c_int
    > maxlen
  {
    return -1i32;
  }
  rta = (n as *mut libc::c_char).offset(
    ((*n)
      .nlmsg_len
      .wrapping_add(4u32)
      .wrapping_sub(1i32 as libc::c_uint)
      & !4u32.wrapping_sub(1i32 as libc::c_uint)) as isize,
  ) as *mut rtattr;
  (*rta).rta_type = type_0 as libc::c_ushort;
  (*rta).rta_len = len as libc::c_ushort;
  *((rta as *mut libc::c_char).offset(
    ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
      .wrapping_add(4u32 as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong)
      & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
      .wrapping_add(0i32 as libc::c_ulong) as isize,
  ) as *mut libc::c_void as *mut bb__aliased_u32) = data;
  (*n).nlmsg_len = (*n)
    .nlmsg_len
    .wrapping_add(len as libc::c_uint)
    .wrapping_add(4u32)
    .wrapping_sub(1i32 as libc::c_uint)
    & !4u32.wrapping_sub(1i32 as libc::c_uint);
  return 0;
}
pub unsafe fn addattr_l(
  mut n: *mut nlmsghdr,
  mut maxlen: libc::c_int,
  mut type_0: libc::c_int,
  mut data: *mut libc::c_void,
  mut alen: libc::c_int,
) -> libc::c_int {
  let mut len: libc::c_int = ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
    .wrapping_add(4u32 as libc::c_ulong)
    .wrapping_sub(1i32 as libc::c_ulong)
    & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
    .wrapping_add(alen as libc::c_ulong) as libc::c_int;
  let mut rta: *mut rtattr = std::ptr::null_mut();
  if ((*n)
    .nlmsg_len
    .wrapping_add(len as libc::c_uint)
    .wrapping_add(4u32)
    .wrapping_sub(1i32 as libc::c_uint)
    & !4u32.wrapping_sub(1i32 as libc::c_uint)) as libc::c_int
    > maxlen
  {
    return -1i32;
  }
  rta = (n as *mut libc::c_char).offset(
    ((*n)
      .nlmsg_len
      .wrapping_add(4u32)
      .wrapping_sub(1i32 as libc::c_uint)
      & !4u32.wrapping_sub(1i32 as libc::c_uint)) as isize,
  ) as *mut rtattr;
  (*rta).rta_type = type_0 as libc::c_ushort;
  (*rta).rta_len = len as libc::c_ushort;
  memcpy(
    (rta as *mut libc::c_char).offset(
      ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
        .wrapping_add(0i32 as libc::c_ulong) as isize,
    ) as *mut libc::c_void,
    data,
    alen as libc::c_ulong,
  );
  (*n).nlmsg_len = (*n)
    .nlmsg_len
    .wrapping_add(len as libc::c_uint)
    .wrapping_add(4u32)
    .wrapping_sub(1i32 as libc::c_uint)
    & !4u32.wrapping_sub(1i32 as libc::c_uint);
  return 0;
}
pub unsafe fn rta_addattr32(
  mut rta: *mut rtattr,
  mut maxlen: libc::c_int,
  mut type_0: libc::c_int,
  mut data: u32,
) -> libc::c_int {
  let mut len: libc::c_int = ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
    .wrapping_add(4u32 as libc::c_ulong)
    .wrapping_sub(1i32 as libc::c_ulong)
    & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
    .wrapping_add(4i32 as libc::c_ulong) as libc::c_int;
  let mut subrta: *mut rtattr = std::ptr::null_mut();
  if (((*rta).rta_len as libc::c_int + len) as libc::c_uint)
    .wrapping_add(4u32)
    .wrapping_sub(1i32 as libc::c_uint)
    & !4u32.wrapping_sub(1i32 as libc::c_uint)
    > maxlen as libc::c_uint
  {
    return -1i32;
  }
  subrta = (rta as *mut libc::c_char).offset(
    (((*rta).rta_len as libc::c_uint)
      .wrapping_add(4u32)
      .wrapping_sub(1i32 as libc::c_uint)
      & !4u32.wrapping_sub(1i32 as libc::c_uint)) as isize,
  ) as *mut rtattr;
  (*subrta).rta_type = type_0 as libc::c_ushort;
  (*subrta).rta_len = len as libc::c_ushort;
  *((subrta as *mut libc::c_char).offset(
    ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
      .wrapping_add(4u32 as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong)
      & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
      .wrapping_add(0i32 as libc::c_ulong) as isize,
  ) as *mut libc::c_void as *mut bb__aliased_u32) = data;
  (*rta).rta_len = ((((*rta).rta_len as libc::c_int + len) as libc::c_uint)
    .wrapping_add(4u32)
    .wrapping_sub(1i32 as libc::c_uint)
    & !4u32.wrapping_sub(1i32 as libc::c_uint)) as libc::c_ushort;
  return 0;
}
pub unsafe fn rta_addattr_l(
  mut rta: *mut rtattr,
  mut maxlen: libc::c_int,
  mut type_0: libc::c_int,
  mut data: *mut libc::c_void,
  mut alen: libc::c_int,
) -> libc::c_int {
  let mut subrta: *mut rtattr = std::ptr::null_mut();
  let mut len: libc::c_int = ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
    .wrapping_add(4u32 as libc::c_ulong)
    .wrapping_sub(1i32 as libc::c_ulong)
    & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
    .wrapping_add(alen as libc::c_ulong) as libc::c_int;
  if (((*rta).rta_len as libc::c_int + len) as libc::c_uint)
    .wrapping_add(4u32)
    .wrapping_sub(1i32 as libc::c_uint)
    & !4u32.wrapping_sub(1i32 as libc::c_uint)
    > maxlen as libc::c_uint
  {
    return -1i32;
  }
  subrta = (rta as *mut libc::c_char).offset(
    (((*rta).rta_len as libc::c_uint)
      .wrapping_add(4u32)
      .wrapping_sub(1i32 as libc::c_uint)
      & !4u32.wrapping_sub(1i32 as libc::c_uint)) as isize,
  ) as *mut rtattr;
  (*subrta).rta_type = type_0 as libc::c_ushort;
  (*subrta).rta_len = len as libc::c_ushort;
  memcpy(
    (subrta as *mut libc::c_char).offset(
      ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
        .wrapping_add(4u32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        & !4u32.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong)
        .wrapping_add(0i32 as libc::c_ulong) as isize,
    ) as *mut libc::c_void,
    data,
    alen as libc::c_ulong,
  );
  (*rta).rta_len = ((((*rta).rta_len as libc::c_int + len) as libc::c_uint)
    .wrapping_add(4u32)
    .wrapping_sub(1i32 as libc::c_uint)
    & !4u32.wrapping_sub(1i32 as libc::c_uint)) as libc::c_ushort;
  return 0;
}

/* We need linux/types.h because older kernels use u32 etc
 * in linux/[rt]netlink.h. 2.6.19 seems to be ok, though */
/* bbox doesn't use parameters no. 3, 4, 6, 7, stub them out */
//TODO: pass rth->fd instead of full rth?
// Used to be:
//struct sockaddr_nl nladdr;
//memset(&nladdr, 0, sizeof(nladdr));
//nladdr.nl_family = AF_NETLINK;
//return xsendto(rth->fd, buf, len, (struct sockaddr*)&nladdr, sizeof(nladdr));
// iproute2-4.2.0 simplified the above to:
//return send(rth->fd, buf, len, 0);
// We are using even shorter:
// and convert to void, inline.
pub unsafe fn parse_rtattr(
  mut tb: *mut *mut rtattr,
  mut max: libc::c_int,
  mut rta: *mut rtattr,
  mut len: libc::c_int,
) {
  memset(
    tb as *mut libc::c_void,
    0,
    ((max + 1i32) as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<*mut rtattr>() as libc::c_ulong),
  );
  while len >= ::std::mem::size_of::<rtattr>() as libc::c_ulong as libc::c_int
    && (*rta).rta_len as libc::c_ulong >= ::std::mem::size_of::<rtattr>() as libc::c_ulong
    && (*rta).rta_len as libc::c_int <= len
  {
    if (*rta).rta_type as libc::c_int <= max {
      let ref mut fresh0 = *tb.offset((*rta).rta_type as isize);
      *fresh0 = rta
    }
    len = (len as libc::c_uint).wrapping_sub(
      ((*rta).rta_len as libc::c_uint)
        .wrapping_add(4u32)
        .wrapping_sub(1i32 as libc::c_uint)
        & !4u32.wrapping_sub(1i32 as libc::c_uint),
    ) as libc::c_int as libc::c_int;
    rta = (rta as *mut libc::c_char).offset(
      (((*rta).rta_len as libc::c_uint)
        .wrapping_add(4u32)
        .wrapping_sub(1i32 as libc::c_uint)
        & !4u32.wrapping_sub(1i32 as libc::c_uint)) as isize,
    ) as *mut rtattr
  }
  if len != 0 {
    crate::libbb::verror_msg::bb_error_msg(
      b"deficit %d, rta_len=%d!\x00" as *const u8 as *const libc::c_char,
      len,
      (*rta).rta_len as libc::c_int,
    );
  };
}

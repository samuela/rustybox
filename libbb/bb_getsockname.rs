use crate::librb::socklen_t;
use libc;
use libc::sockaddr;
use libc::sockaddr_in;
use libc::sockaddr_in6;
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
  fn getsockname(__fd: libc::c_int, __addr: __SOCKADDR_ARG, __len: *mut socklen_t) -> libc::c_int;
}

pub type __socklen_t = libc::c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed {
  pub __u6_addr8: [u8; 16],
  pub __u6_addr16: [u16; 8],
  pub __u6_addr32: [u32; 4],
}

pub type in_port_t = u16;
pub type in_addr_t = u32;

pub unsafe fn bb_getsockname(
  mut sockfd: libc::c_int,
  mut addr: *mut libc::c_void,
  mut addrlen: socklen_t,
) -> libc::c_int {
  /* The usefullness of this function is that for getsockname(),
   * addrlen must go on stack (to _have_ an address to be passed),
   * but many callers do not need its modified value.
   * By using this shim, they can avoid unnecessary stack spillage.
   */
  return getsockname(
    sockfd,
    __SOCKADDR_ARG {
      __sockaddr__: addr as *mut sockaddr,
    },
    &mut addrlen,
  );
}

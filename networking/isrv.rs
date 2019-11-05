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
  fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn select(
    __nfds: libc::c_int,
    __readfds: *mut fd_set,
    __writefds: *mut fd_set,
    __exceptfds: *mut fd_set,
    __timeout: *mut timeval,
  ) -> libc::c_int;
  #[no_mangle]
  fn accept(__fd: libc::c_int, __addr: __SOCKADDR_ARG, __addr_len: *mut socklen_t) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn monotonic_sec() -> libc::c_uint;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
use crate::librb::uint8_t;
pub type uint16_t = __uint16_t;
use crate::librb::uint32_t;
pub type size_t = libc::c_ulong;
pub type socklen_t = __socklen_t;
use crate::librb::time_t;
use crate::librb::timeval;
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
pub type ptrdiff_t = libc::c_long;
/* vi: set sw=4 ts=4: */
/*
 * Generic non-forking server infrastructure.
 * Intended to make writing telnetd-type servers easier.
 *
 * Copyright (C) 2007 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* opaque structure */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct isrv_state_t {
  pub fd2peer: *mut libc::c_short,
  pub param_tbl: *mut *mut libc::c_void,
  pub timeo_tbl: *mut time_t,
  pub new_peer: Option<unsafe extern "C" fn(_: *mut isrv_state_t, _: libc::c_int) -> libc::c_int>,
  pub curtime: time_t,
  pub timeout: libc::c_int,
  pub fd_count: libc::c_int,
  pub peer_count: libc::c_int,
  pub wr_count: libc::c_int,
  pub rd: fd_set,
  pub wr: fd_set,
  /* callbacks */
  /* Driver:
   *
   * Select on listen_fd for <linger_timeout> (or forever if 0).
   *
   * If we time out and we have no peers, exit.
   * If we have peers, call do_timeout(peer_param),
   * if it returns !0, peer is removed.
   *
   * If listen_fd is active, accept new connection ("peer"),
   * call new_peer() on it, and if it returns 0,
   * add it to fds to select on.
   * Now, select will wait for <timeout>, not <linger_timeout>
   * (as long as we have more than zero peers).
   *
   * If a peer's fd is active, we call do_rd() on it if read
   * bit was set, and then do_wr() if write bit was also set.
   * If either returns !0, peer is removed.
   * Reaching this place also resets timeout counter for this peer.
   *
   * Note that peer must indicate that he wants to be selected
   * for read and/or write using isrv_want_rd()/isrv_want_wr()
   * [can be called in new_peer() or in do_rd()/do_wr()].
   * If it never wants to be selected for write, do_wr()
   * will never be called (can be NULL).
   */
}
pub const LONG_CNT: C2RustUnnamed_0 = 16;
pub type C2RustUnnamed_0 = libc::c_uint;
/* callback */
#[no_mangle]
pub unsafe extern "C" fn isrv_want_rd(mut state: *mut isrv_state_t, mut fd: libc::c_int) {
  (*state).rd.fds_bits[(fd
    / (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
    as usize] |= (1u64
    << fd % (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
    as __fd_mask;
}
/* callback */
#[no_mangle]
pub unsafe extern "C" fn isrv_want_wr(mut state: *mut isrv_state_t, mut fd: libc::c_int) {
  if !((*state).wr.fds_bits
    [(fd / (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int)) as usize]
    & (1u64 << fd % (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
      as __fd_mask
    != 0i32 as libc::c_long)
  {
    (*state).wr_count += 1;
    (*state).wr.fds_bits[(fd
      / (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
      as usize] |= (1u64
      << fd % (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
      as __fd_mask
  };
}
/* callback */
#[no_mangle]
pub unsafe extern "C" fn isrv_dont_want_rd(mut state: *mut isrv_state_t, mut fd: libc::c_int) {
  (*state).rd.fds_bits[(fd
    / (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
    as usize] &= !((1u64
    << fd % (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
    as __fd_mask);
}
/* callback */
#[no_mangle]
pub unsafe extern "C" fn isrv_dont_want_wr(mut state: *mut isrv_state_t, mut fd: libc::c_int) {
  if (*state).wr.fds_bits
    [(fd / (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int)) as usize]
    & (1u64 << fd % (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
      as __fd_mask
    != 0i32 as libc::c_long
  {
    (*state).wr_count -= 1;
    (*state).wr.fds_bits[(fd
      / (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
      as usize] &= !((1u64
      << fd % (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
      as __fd_mask)
  };
}
/* callback */
#[no_mangle]
pub unsafe extern "C" fn isrv_register_fd(
  mut state: *mut isrv_state_t,
  mut peer: libc::c_int,
  mut fd: libc::c_int,
) -> libc::c_int {
  let mut n: libc::c_int = 0;
  if (*state).fd_count >= 1024i32 {
    return -1i32;
  }
  if (*state).fd_count <= fd {
    n = (*state).fd_count;
    (*state).fd_count = fd + 1i32;
    (*state).fd2peer = xrealloc(
      (*state).fd2peer as *mut libc::c_void,
      ((*state).fd_count as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_short>() as libc::c_ulong),
    ) as *mut libc::c_short;
    while n < fd {
      let fresh0 = n;
      n = n + 1;
      *(*state).fd2peer.offset(fresh0 as isize) = -1i32 as libc::c_short
    }
  }
  *(*state).fd2peer.offset(fd as isize) = peer as libc::c_short;
  return 0i32;
}
/* callback */
#[no_mangle]
pub unsafe extern "C" fn isrv_close_fd(mut state: *mut isrv_state_t, mut fd: libc::c_int) {
  close(fd);
  isrv_dont_want_rd(state, fd);
  if (*state).wr_count != 0 {
    isrv_dont_want_wr(state, fd);
  }
  *(*state).fd2peer.offset(fd as isize) = -1i32 as libc::c_short;
  if fd == (*state).fd_count - 1i32 {
    loop {
      fd -= 1;
      if !(fd >= 0i32 && *(*state).fd2peer.offset(fd as isize) as libc::c_int == -1i32) {
        break;
      }
    }
    (*state).fd_count = fd + 1i32;
    (*state).fd2peer = xrealloc(
      (*state).fd2peer as *mut libc::c_void,
      ((*state).fd_count as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_short>() as libc::c_ulong),
    ) as *mut libc::c_short
  };
}
/* callback */
#[no_mangle]
pub unsafe extern "C" fn isrv_register_peer(
  mut state: *mut isrv_state_t,
  mut param: *mut libc::c_void,
) -> libc::c_int {
  let mut n: libc::c_int = 0;
  if (*state).peer_count >= 1024i32 {
    return -1i32;
  }
  let fresh1 = (*state).peer_count;
  (*state).peer_count = (*state).peer_count + 1;
  n = fresh1;
  (*state).param_tbl = xrealloc(
    (*state).param_tbl as *mut libc::c_void,
    ((*state).peer_count as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
  ) as *mut *mut libc::c_void;
  let ref mut fresh2 = *(*state).param_tbl.offset(n as isize);
  *fresh2 = param;
  if (*state).timeout != 0 {
    (*state).timeo_tbl = xrealloc(
      (*state).timeo_tbl as *mut libc::c_void,
      ((*state).peer_count as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<time_t>() as libc::c_ulong),
    ) as *mut time_t;
    *(*state).timeo_tbl.offset(n as isize) = (*state).curtime
  }
  return n;
}
unsafe extern "C" fn remove_peer(mut state: *mut isrv_state_t, mut peer: libc::c_int) {
  let mut movesize: libc::c_int = 0;
  let mut fd: libc::c_int = 0;
  fd = (*state).fd_count - 1i32;
  while fd >= 0i32 {
    if *(*state).fd2peer.offset(fd as isize) as libc::c_int == peer {
      isrv_close_fd(state, fd);
      fd -= 1
    } else {
      if *(*state).fd2peer.offset(fd as isize) as libc::c_int > peer {
        let ref mut fresh3 = *(*state).fd2peer.offset(fd as isize);
        *fresh3 -= 1
      }
      fd -= 1
    }
  }
  (*state).peer_count -= 1;
  movesize = (((*state).peer_count - peer) as libc::c_ulong)
    .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
    as libc::c_int;
  if movesize > 0i32 {
    memcpy(
      &mut *(*state).param_tbl.offset(peer as isize) as *mut *mut libc::c_void as *mut libc::c_void,
      &mut *(*state).param_tbl.offset((peer + 1i32) as isize) as *mut *mut libc::c_void
        as *const libc::c_void,
      movesize as libc::c_ulong,
    );
    if (*state).timeout != 0 {
      memcpy(
        &mut *(*state).timeo_tbl.offset(peer as isize) as *mut time_t as *mut libc::c_void,
        &mut *(*state).timeo_tbl.offset((peer + 1i32) as isize) as *mut time_t
          as *const libc::c_void,
        movesize as libc::c_ulong,
      );
    }
  }
  (*state).param_tbl = xrealloc(
    (*state).param_tbl as *mut libc::c_void,
    ((*state).peer_count as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
  ) as *mut *mut libc::c_void;
  if (*state).timeout != 0 {
    (*state).timeo_tbl = xrealloc(
      (*state).timeo_tbl as *mut libc::c_void,
      ((*state).peer_count as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<time_t>() as libc::c_ulong),
    ) as *mut time_t
  };
}
unsafe extern "C" fn handle_accept(mut state: *mut isrv_state_t, mut fd: libc::c_int) {
  let mut n: libc::c_int = 0;
  let mut newfd: libc::c_int = 0;
  /* suppress gcc warning "cast from ptr to int of different size" */
  fcntl(
    fd,
    4i32,
    *(*state).param_tbl.offset(0) as ptrdiff_t as libc::c_int | 0o4000i32,
  );
  newfd = accept(
    fd,
    __SOCKADDR_ARG {
      __sockaddr__: 0 as *mut libc::c_void as *mut sockaddr,
    },
    0 as *mut socklen_t,
  );
  fcntl(
    fd,
    4i32,
    *(*state).param_tbl.offset(0) as ptrdiff_t as libc::c_int,
  );
  if newfd < 0i32 {
    if *bb_errno == 11i32 {
      return;
    }
    /* Most probably someone gave us wrong fd type
     * (for example, non-socket). Don't want
     * to loop forever. */
    bb_simple_perror_msg_and_die(b"accept\x00" as *const u8 as *const libc::c_char);
  }
  n = (*state).new_peer.expect("non-null function pointer")(state, newfd);
  if n != 0 {
    remove_peer(state, n);
  };
  /* unsuccessful peer start */
}
unsafe extern "C" fn handle_fd_set(
  mut state: *mut isrv_state_t,
  mut fds: *mut fd_set,
  mut h: Option<unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_void) -> libc::c_int>,
) {
  let mut fds_pos: libc::c_int = 0;
  let mut fd: libc::c_int = 0;
  let mut peer: libc::c_int = 0;
  /* need to know value at _the beginning_ of this routine */
  let mut fd_cnt: libc::c_int = (*state).fd_count;
  fds_pos = 0i32;
  loop
  /* Find next nonzero bit */
  {
    if fds_pos < LONG_CNT as libc::c_int {
      if *(fds as *mut libc::c_long).offset(fds_pos as isize) == 0i32 as libc::c_long {
        fds_pos += 1
      } else {
        /* Found non-zero word */
        fd = (fds_pos as libc::c_ulong)
          .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
          .wrapping_mul(8i32 as libc::c_ulong) as libc::c_int; /* word# -> bit# */
        loop {
          if (*fds).fds_bits[(fd
            / (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            & (1u64
              << fd % (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
              as __fd_mask
            != 0i32 as libc::c_long
          {
            (*fds).fds_bits[(fd
              / (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
              as usize] &= !((1u64
              << fd % (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
              as __fd_mask); /* peer is already gone */
            break;
          } else {
            fd += 1
          }
        }
        if fd >= fd_cnt {
          break;
        }
        peer = *(*state).fd2peer.offset(fd as isize) as libc::c_int;
        if peer < 0i32 {
          continue;
        }
        if peer == 0i32 {
          handle_accept(state, fd);
        } else if h.expect("non-null function pointer")(
          fd,
          &mut *(*state).param_tbl.offset(peer as isize),
        ) != 0
        {
          /* this peer is gone */
          remove_peer(state, peer); /* all words are zero */
        } else if (*state).timeout != 0 {
          *(*state).timeo_tbl.offset(peer as isize) = monotonic_sec() as time_t
        }
      }
    } else {
      break;
    }
  }
}
unsafe extern "C" fn handle_timeout(
  mut state: *mut isrv_state_t,
  mut do_timeout: Option<unsafe extern "C" fn(_: *mut *mut libc::c_void) -> libc::c_int>,
) {
  let mut n: libc::c_int = 0;
  let mut peer: libc::c_int = 0;
  peer = (*state).peer_count - 1i32;
  /* peer 0 is not checked */
  while peer > 0i32 {
    if (*state).curtime - *(*state).timeo_tbl.offset(peer as isize)
      >= (*state).timeout as libc::c_long
    {
      n = do_timeout.expect("non-null function pointer")(
        &mut *(*state).param_tbl.offset(peer as isize),
      );
      if n != 0 {
        remove_peer(state, peer);
      }
    }
    peer -= 1
  }
}
/* Driver */
#[no_mangle]
pub unsafe extern "C" fn isrv_run(
  mut listen_fd: libc::c_int,
  mut new_peer: Option<unsafe extern "C" fn(_: *mut isrv_state_t, _: libc::c_int) -> libc::c_int>,
  mut do_rd: Option<unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_void) -> libc::c_int>,
  mut do_wr: Option<unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_void) -> libc::c_int>,
  mut do_timeout: Option<unsafe extern "C" fn(_: *mut *mut libc::c_void) -> libc::c_int>,
  mut timeout: libc::c_int,
  mut linger_timeout: libc::c_int,
) {
  let mut state: *mut isrv_state_t =
    xzalloc(::std::mem::size_of::<isrv_state_t>() as libc::c_ulong) as *mut isrv_state_t;
  (*state).new_peer = new_peer;
  (*state).timeout = timeout;
  /* register "peer" #0 - it will accept new connections */
  isrv_register_peer(state, 0 as *mut libc::c_void);
  isrv_register_fd(state, 0i32, listen_fd);
  isrv_want_rd(state, listen_fd);
  /* remember flags to make blocking<->nonblocking switch faster */
  /* (suppress gcc warning "cast from ptr to int of different size") */
  let ref mut fresh4 = *(*state).param_tbl.offset(0);
  *fresh4 = fcntl(listen_fd, 3i32) as ptrdiff_t as *mut libc::c_void;
  loop {
    let mut tv: timeval = timeval {
      tv_sec: 0,
      tv_usec: 0,
    };
    let mut rd: fd_set = fd_set { fds_bits: [0; 16] };
    let mut wr: fd_set = fd_set { fds_bits: [0; 16] };
    let mut wrp: *mut fd_set = 0 as *mut fd_set;
    let mut n: libc::c_int = 0;
    tv.tv_sec = timeout as __time_t;
    if (*state).peer_count <= 1i32 {
      tv.tv_sec = linger_timeout as __time_t
    }
    tv.tv_usec = 0i32 as __suseconds_t;
    rd = (*state).rd;
    if (*state).wr_count != 0 {
      wr = (*state).wr;
      wrp = &mut wr
    }
    n = select(
      (*state).fd_count,
      &mut rd,
      wrp,
      0 as *mut fd_set,
      if tv.tv_sec != 0 {
        &mut tv
      } else {
        0 as *mut timeval
      },
    );
    if n < 0i32 {
      if *bb_errno != 4i32 {
        bb_simple_perror_msg(b"select\x00" as *const u8 as *const libc::c_char);
      }
    } else {
      if n == 0i32 && linger_timeout != 0 && (*state).peer_count <= 1i32 {
        break;
      }
      if timeout != 0 {
        let mut t: time_t = monotonic_sec() as time_t;
        if t != (*state).curtime {
          (*state).curtime = t;
          handle_timeout(state, do_timeout);
        }
      }
      if n > 0i32 {
        handle_fd_set(state, &mut rd, do_rd);
        if !wrp.is_null() {
          handle_fd_set(state, wrp, do_wr);
        }
      }
    }
  }
  /* NB: accept socket is not closed. Caller is to decide what to do */
}

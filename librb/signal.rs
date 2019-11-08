// Things from signal.h and friends
// See
//  * http://man7.org/linux/man-pages/man7/sigevent.7.html
//  * https://pubs.opengroup.org/onlinepubs/009695399/basedefs/signal.h.html

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub _pad: [libc::c_int; 28],
  pub _kill: C2RustUnnamed_8,
  pub _timer: C2RustUnnamed_7,
  pub _rt: C2RustUnnamed_6,
  pub _sigchld: C2RustUnnamed_5,
  pub _sigfault: C2RustUnnamed_2,
  pub _sigpoll: C2RustUnnamed_1,
  pub _sigsys: C2RustUnnamed_0,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
  pub _call_addr: *mut libc::c_void,
  pub _syscall: libc::c_int,
  pub _arch: libc::c_uint,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
  pub si_band: libc::c_long,
  pub si_fd: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
  pub si_addr: *mut libc::c_void,
  pub si_addr_lsb: libc::c_short,
  pub _bounds: C2RustUnnamed_3,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
  pub _addr_bnd: C2RustUnnamed_4,
  pub _pkey: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
  pub _lower: *mut libc::c_void,
  pub _upper: *mut libc::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
  pub si_pid: libc::pid_t,
  pub si_uid: libc::uid_t,
  pub si_status: libc::c_int,
  pub si_utime: libc::clock_t,
  pub si_stime: libc::clock_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
  pub si_pid: libc::pid_t,
  pub si_uid: libc::uid_t,
  pub si_sigval: libc::sigval,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
  pub si_tid: libc::c_int,
  pub si_overrun: libc::c_int,
  pub si_sigval: libc::sigval,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
  pub si_pid: libc::pid_t,
  pub si_uid: libc::uid_t,
}

// libc::sighandler_t is size_t which doesn't really make much sense.
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;

// This is meaningfully different from libc::sigaction.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
  pub __sigaction_handler: SigactionHandler,
  pub sa_mask: libc::sigset_t,
  pub sa_flags: libc::c_int,
  pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union SigactionHandler {
  /* Used if SA_SIGINFO is not set. */
  pub sa_handler: __sighandler_t,

  /* Used if SA_SIGINFO is set. */
  pub sa_sigaction: Option<
    unsafe extern "C" fn(_: libc::c_int, _: *mut libc::siginfo_t, _: *mut libc::c_void) -> (),
  >,
}

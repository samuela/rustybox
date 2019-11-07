// Things from signal.h and friends
// See
//  * http://man7.org/linux/man-pages/man7/sigevent.7.html
//  * https://pubs.opengroup.org/onlinepubs/009695399/basedefs/signal.h.html

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
  pub __val: [libc::c_ulong; 16],
}

pub type sigset_t = __sigset_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
  pub sival_int: libc::c_int,
  pub sival_ptr: *mut libc::c_void,
}

pub type __sigval_t = sigval;

// See http://man7.org/linux/man-pages/man2/sigaction.2.html.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
  pub si_signo: libc::c_int,
  pub si_errno: libc::c_int,
  pub si_code: libc::c_int,
  pub __pad0: libc::c_int,
  pub _sifields: C2RustUnnamed,
}

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
  pub si_sigval: __sigval_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
  pub si_tid: libc::c_int,
  pub si_overrun: libc::c_int,
  pub si_sigval: __sigval_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
  pub si_pid: libc::pid_t,
  pub si_uid: libc::uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
  pub __sigaction_handler: C2RustUnnamed_9,
  pub sa_mask: __sigset_t,
  pub sa_flags: libc::c_int,
  pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
  pub sa_handler: __sighandler_t,
  pub sa_sigaction:
    Option<unsafe extern "C" fn(_: libc::c_int, _: *mut siginfo_t, _: *mut libc::c_void) -> ()>,
}

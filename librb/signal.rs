// Things from signal.h and friends
// See
//  * http://man7.org/linux/man-pages/man7/sigevent.7.html
//  * https://pubs.opengroup.org/onlinepubs/009695399/basedefs/signal.h.html

// libc::sighandler_t is size_t which doesn't really make much sense.
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;

// This is meaningfully different from libc::sigaction.

extern "C" {
  #[no_mangle]
  fn sigaction(__sig: libc::c_int, __act: *const sigaction, __oact: *mut sigaction) -> libc::c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigaction {
  pub __sigaction_handler: SigactionHandler,
  pub sa_mask: libc::sigset_t,
  pub sa_flags: libc::c_int,
  pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union SigactionHandler {
  /* Used if SA_SIGINFO is not set. */
  pub sa_handler: __sighandler_t,

  /* Used if SA_SIGINFO is set. */
  pub sa_sigaction: Option<
    unsafe extern "C" fn(_: libc::c_int, _: *mut libc::siginfo_t, _: *mut libc::c_void) -> (),
  >,
}

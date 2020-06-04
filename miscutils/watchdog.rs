use crate::librb::suffix_mult;
use libc;
use libc::close;
use libc::useconds_t;
extern "C" {
  #[no_mangle]
  fn _exit(_: libc::c_int) -> !;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn usleep(__useconds: useconds_t) -> libc::c_int;

}

pub type C2RustUnnamed = libc::c_uint;
pub const BB_FATAL_SIGS: C2RustUnnamed = 117503054;

pub type C2RustUnnamed_0 = libc::c_uint;
pub const DAEMON_ONLY_SANITIZE: C2RustUnnamed_0 = 8;
pub const DAEMON_CLOSE_EXTRA_FDS: C2RustUnnamed_0 = 4;
pub const DAEMON_DEVNULL_STDIO: C2RustUnnamed_0 = 2;
pub const DAEMON_CHDIR_ROOT: C2RustUnnamed_0 = 1;
unsafe extern "C" fn shutdown_watchdog() {
  static mut V: libc::c_char = 'V' as i32 as libc::c_char; /* Magic, see watchdog-api.txt in kernel */
  libc::write(3, &V as *const libc::c_char as *const libc::c_void, 1);
  close(3i32);
}
unsafe extern "C" fn shutdown_on_signal(mut _sig: libc::c_int) {
  crate::libbb::pidfile::remove_pidfile_std_path_and_ext(
    b"watchdog\x00" as *const u8 as *const libc::c_char,
  );
  shutdown_watchdog();
  _exit(0i32);
}
unsafe extern "C" fn watchdog_open(mut device: *const libc::c_char) {
  /* Use known fd # - avoid needing global 'int fd' */
  crate::libbb::xfuncs_printf::xmove_fd(crate::libbb::xfuncs_printf::xopen(device, 0o1i32), 3i32);
  /* If the watchdog driver can do something other than cause a reboot
   * on a timeout, then it's possible this program may be starting from
   * a state when the watchdog hadn't been previously stopped with
   * the magic write followed by a close.  In this case the driver may
   * not start properly, so always do the proper stop first just in case.
   */
  shutdown_watchdog(); /* how often to restart */
  crate::libbb::xfuncs_printf::xmove_fd(crate::libbb::xfuncs_printf::xopen(device, 0o1i32), 3i32);
  /* reboots after N ms if not restarted */
}
pub unsafe fn watchdog_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  static mut enable: libc::c_int = 0x2i32;
  static mut suffixes: [suffix_mult; 3] = [
    {
      let mut init = suffix_mult {
        suffix: [109, 115, 0, 0],
        mult: 1i32 as libc::c_uint,
      };
      init
    },
    {
      let mut init = suffix_mult {
        suffix: [0, 0, 0, 0],
        mult: 1000i32 as libc::c_uint,
      };
      init
    },
    {
      let mut init = suffix_mult {
        suffix: [0, 0, 0, 0],
        mult: 0 as libc::c_uint,
      };
      init
    },
  ];
  let mut opts: libc::c_uint = 0;
  let mut stimer_duration: libc::c_uint = 0;
  let mut htimer_duration: libc::c_uint = 60000i32 as libc::c_uint;
  let mut st_arg: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut ht_arg: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  opts = crate::libbb::getopt32::getopt32(
    argv,
    b"^Ft:T:\x00=1\x00" as *const u8 as *const libc::c_char,
    &mut st_arg as *mut *mut libc::c_char,
    &mut ht_arg as *mut *mut libc::c_char,
  );
  /* We need to daemonize *before* opening the watchdog as many drivers
   * will only allow one process at a time to do so.  Since daemonizing
   * is not perfect (child may run before parent finishes exiting), we
   * can't rely on parent exiting before us (let alone *cleanly* releasing
   * the watchdog fd -- something else that may not even be allowed).
   */
  if opts & (1i32 << 0) as libc::c_uint == 0 {
    crate::libbb::vfork_daemon_rexec::bb_daemonize_or_rexec(DAEMON_CHDIR_ROOT as libc::c_int);
  }
  /* maybe bb_logenv_override(); here for LOGGING=syslog to work? */
  if opts & (1i32 << 2i32) as libc::c_uint != 0 {
    htimer_duration = crate::libbb::xatonum::xatou_sfx(ht_arg, suffixes.as_ptr())
  }
  stimer_duration = htimer_duration.wrapping_div(2i32 as libc::c_uint);
  if opts & (1i32 << 1i32) as libc::c_uint != 0 {
    stimer_duration = crate::libbb::xatonum::xatou_sfx(st_arg, suffixes.as_ptr())
  }
  crate::libbb::signals::bb_signals(BB_FATAL_SIGS as libc::c_int, Some(shutdown_on_signal));
  watchdog_open(*argv.offset(optind as isize));
  /* WDIOC_SETTIMEOUT takes seconds, not milliseconds */
  htimer_duration = htimer_duration.wrapping_div(1000i32 as libc::c_uint);
  crate::libbb::xfuncs_printf::bb_ioctl_or_warn(
    3i32,
    ((2u32 << 0 + 8i32 + 8i32 + 14i32
      | (('W' as i32) << 0 + 8i32) as libc::c_uint
      | (4i32 << 0) as libc::c_uint) as libc::c_ulong
      | (::std::mem::size_of::<libc::c_int>() as libc::c_ulong) << 0 + 8i32 + 8i32)
      as libc::c_uint,
    &enable as *const libc::c_int as *mut libc::c_void,
    b"WDIOC_SETOPTIONS\x00" as *const u8 as *const libc::c_char,
  );
  crate::libbb::xfuncs_printf::bb_ioctl_or_warn(
    3i32,
    (((2u32 | 1u32) << 0 + 8i32 + 8i32 + 14i32
      | (('W' as i32) << 0 + 8i32) as libc::c_uint
      | (6i32 << 0) as libc::c_uint) as libc::c_ulong
      | (::std::mem::size_of::<libc::c_int>() as libc::c_ulong) << 0 + 8i32 + 8i32)
      as libc::c_uint,
    &mut htimer_duration as *mut libc::c_uint as *mut libc::c_void,
    b"WDIOC_SETTIMEOUT\x00" as *const u8 as *const libc::c_char,
  );
  crate::libbb::pidfile::write_pidfile_std_path_and_ext(
    b"watchdog\x00" as *const u8 as *const libc::c_char,
  );
  loop {
    /*
     * Make sure we clear the counter before sleeping,
     * as the counter value is undefined at this point -- PFM
     */
    libc::write(
      3,
      b"\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
      1,
    ); /* write zero byte */
    usleep((stimer_duration as i64 * 1000i64) as useconds_t);
  }
  /* - not reached, but gcc 4.2.1 is too dumb! */
}

use libc::termios;
use libc;
use libc::putenv;
use libc::umask;
use libc::mknod;
use libc::fchmod;
use libc::fscanf;
use libc::alarm;
use libc::sync;
use libc::setsid;
use libc::ioctl;
use libc::statfs;
use libc::mount;
use libc::prctl;
use libc::opendir;
use libc::closedir;
use libc::readdir;
use libc::strtok;
use libc::putchar_unlocked;
use libc::endmntent;
use libc::setmntent;
use libc::umount2;
use libc::getegid;
use libc::getuid;
use libc::getgid;
use libc::setutxent;
use libc::endutxent;
















use libc::sleep;





















use libc::system;
use libc::open;


use libc::free;
use libc::cc_t;
use libc::speed_t;
use libc::tcflag_t;

extern "C" {


  #[no_mangle]
  static mut optind: libc::c_int;


  #[no_mangle]
  fn exit(_: libc::c_int) -> !;

  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn cfgetospeed(__termios_p: *const termios) -> speed_t;
  #[no_mangle]
  fn cfgetispeed(__termios_p: *const termios) -> speed_t;
  #[no_mangle]
  fn cfsetospeed(__termios_p: *mut termios, __speed: speed_t) -> libc::c_int;
  #[no_mangle]
  fn cfsetispeed(__termios_p: *mut termios, __speed: speed_t) -> libc::c_int;
  #[no_mangle]
  fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
  #[no_mangle]
  fn tcsetattr(
    __fd: libc::c_int,
    __optional_actions: libc::c_int,
    __termios_p: *const termios,
  ) -> libc::c_int;
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn bb_signals(sigs: libc::c_int, f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>);
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn tty_value_to_baud(value: libc::c_uint) -> speed_t;
  #[no_mangle]
  fn concat_path_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn bb_ioctl_or_warn(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  fn bb_xioctl(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  fn xatoi(str: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
  #[no_mangle]
  fn invarg_1_to_2(_: *const libc::c_char, _: *const libc::c_char) -> !;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub saved_disc: libc::c_int,
  pub saved_state: termios,
}
pub const OPT_c_extcmd: C2RustUnnamed = 4;
pub const OPT_h_watch: C2RustUnnamed = 16;
pub const OPT_e_quit: C2RustUnnamed = 8;
pub const OPT_s_baud: C2RustUnnamed = 2;
pub const OPT_F_noflow: C2RustUnnamed = 128;
pub const OPT_L_local: C2RustUnnamed = 64;
pub const OPT_m_nonraw: C2RustUnnamed = 32;
pub type C2RustUnnamed = libc::c_uint;
pub const OPT_p_proto: C2RustUnnamed = 1;
unsafe extern "C" fn tcsetattr_serial_or_warn(mut state: *mut termios) -> libc::c_int {
  let mut ret: libc::c_int = 0;
  ret = tcsetattr(3i32, 0i32, state);
  if ret != 0i32 {
    bb_simple_perror_msg(b"tcsetattr\x00" as *const u8 as *const libc::c_char);
    return 1i32;
    /* used as exitcode */
  }
  return ret;
  /* 0 */
}
unsafe extern "C" fn restore_state_and_exit(mut exitcode: libc::c_int) -> ! {
  let mut state: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
  };
  /* Restore line discipline */
  if bb_ioctl_or_warn(
    3i32,
    0x5423i32 as libc::c_uint,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).saved_disc as *mut libc::c_int
      as *mut libc::c_void,
    b"TIOCSETD\x00" as *const u8 as *const libc::c_char,
  ) != 0
  {
    exitcode = 1i32
  }
  /* Hangup */
  memcpy(
    &mut state as *mut termios as *mut libc::c_void,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).saved_state as *mut termios
      as *const libc::c_void,
    ::std::mem::size_of::<termios>() as libc::c_ulong,
  );
  cfsetispeed(&mut state, 0i32 as speed_t);
  cfsetospeed(&mut state, 0i32 as speed_t);
  exitcode |= tcsetattr_serial_or_warn(&mut state);
  sleep(1i32 as libc::c_uint);
  /* Restore line status */
  if tcsetattr_serial_or_warn(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).saved_state)
    != 0
  {
    exit(1i32);
  }
  exit(exitcode);
}
unsafe extern "C" fn sig_handler(mut _signo: libc::c_int) {
  restore_state_and_exit(0i32);
}
#[no_mangle]
pub unsafe extern "C" fn slattach_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  /* Line discipline code table */
  static mut proto_names: [libc::c_char; 34] = [
    115, 108, 105, 112, 0, 99, 115, 108, 105, 112, 0, 115, 108, 105, 112, 54, 0, 99, 115, 108, 105,
    112, 54, 0, 97, 100, 97, 112, 116, 105, 118, 101, 0, 0,
  ];
  static mut int_N_SLIP: libc::c_int = 1i32;
  let mut encap: libc::c_int = 0;
  let mut opt: libc::c_int = 0;
  let mut fd: libc::c_int = 0;
  let mut state: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
  };
  let mut proto: *const libc::c_char = b"cslip\x00" as *const u8 as *const libc::c_char;
  /* 8 */
  let mut extcmd: *const libc::c_char = 0 as *const libc::c_char; /* Command to execute after hangup */
  let mut baud_str: *const libc::c_char = 0 as *const libc::c_char; /* for compiler */
  let mut baud_code: libc::c_int = 0;
  baud_code = baud_code;
  /* Parse command line options */
  opt = getopt32(
    argv,
    b"^p:s:c:ehmLF\x00=1\x00" as *const u8 as *const libc::c_char,
    &mut proto as *mut *const libc::c_char,
    &mut baud_str as *mut *const libc::c_char,
    &mut extcmd as *mut *const libc::c_char,
  ) as libc::c_int;
  /*argc -= optind;*/
  argv = argv.offset(optind as isize);
  encap = index_in_strings(proto_names.as_ptr(), proto);
  if encap < 0i32 {
    invarg_1_to_2(proto, b"protocol\x00" as *const u8 as *const libc::c_char);
  }
  if encap > 3i32 {
    encap = 8i32
  }
  /* We want to know if the baud rate is valid before we start touching the ttys */
  if opt & OPT_s_baud as libc::c_int != 0 {
    baud_code = tty_value_to_baud(xatoi(baud_str) as libc::c_uint) as libc::c_int;
    if baud_code < 0i32 {
      invarg_1_to_2(
        baud_str,
        b"baud rate\x00" as *const u8 as *const libc::c_char,
      );
    }
  }
  /* Open tty */
  fd = open(*argv, 0o2i32 | 0o4000i32);
  if fd < 0i32 {
    let mut buf: *mut libc::c_char =
      concat_path_file(b"/dev\x00" as *const u8 as *const libc::c_char, *argv);
    fd = xopen(buf, 0o2i32 | 0o4000i32);
    /* maybe if (ENABLE_FEATURE_CLEAN_UP) ?? */
    free(buf as *mut libc::c_void);
  }
  xmove_fd(fd, 3i32);
  /* Save current tty state */
  if tcgetattr(
    3i32,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).saved_state,
  ) != 0i32
  {
    bb_simple_perror_msg_and_die(b"tcgetattr\x00" as *const u8 as *const libc::c_char);
  }
  /* Save line discipline */
  bb_xioctl(
    3i32,
    0x5424i32 as libc::c_uint,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).saved_disc as *mut libc::c_int
      as *mut libc::c_void,
    b"TIOCGETD\x00" as *const u8 as *const libc::c_char,
  );
  /* Trap signals in order to restore tty states upon exit */
  if opt & OPT_e_quit as libc::c_int == 0 {
    bb_signals(
      0i32 + (1i32 << 1i32) + (1i32 << 2i32) + (1i32 << 3i32) + (1i32 << 15i32),
      Some(sig_handler as unsafe extern "C" fn(_: libc::c_int) -> ()),
    );
  }
  /* Configure tty */
  memcpy(
    &mut state as *mut termios as *mut libc::c_void,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).saved_state as *mut termios
      as *const libc::c_void,
    ::std::mem::size_of::<termios>() as libc::c_ulong,
  );
  if opt & OPT_m_nonraw as libc::c_int == 0 {
    /* raw not suppressed */
    memset(
      &mut state.c_cc as *mut [cc_t; 32] as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<[cc_t; 32]>() as libc::c_ulong,
    );
    state.c_cc[6] = 1i32 as cc_t;
    state.c_iflag = (0o1i32 | 0o4i32) as tcflag_t;
    /*state.c_oflag = 0;*/
    /*state.c_lflag = 0;*/
    state.c_cflag = (0o60i32
      | 0o2000i32
      | 0o200i32
      | (if opt & OPT_L_local as libc::c_int != 0 {
        0o4000i32
      } else {
        0i32
      })) as libc::c_uint
      | (if opt & OPT_F_noflow as libc::c_int != 0 {
        0i32 as libc::c_uint
      } else {
        0o20000000000u32
      });
    cfsetispeed(
      &mut state,
      cfgetispeed(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).saved_state),
    );
    cfsetospeed(
      &mut state,
      cfgetospeed(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).saved_state),
    );
  }
  if opt & OPT_s_baud as libc::c_int != 0 {
    cfsetispeed(&mut state, baud_code as speed_t);
    cfsetospeed(&mut state, baud_code as speed_t);
  }
  /* Set line status */
  if !(tcsetattr_serial_or_warn(&mut state) != 0) {
    /* Set line disclipline (N_SLIP always) */
    if !(bb_ioctl_or_warn(
      3i32,
      0x5423i32 as libc::c_uint,
      &int_N_SLIP as *const libc::c_int as *mut libc::c_void,
      b"TIOCSETD\x00" as *const u8 as *const libc::c_char,
    ) != 0)
    {
      /* Set encapsulation (SLIP, CSLIP, etc) */
      if !(bb_ioctl_or_warn(
        3i32,
        0x8926i32 as libc::c_uint,
        &mut encap as *mut libc::c_int as *mut libc::c_void,
        b"SIOCSIFENCAP\x00" as *const u8 as *const libc::c_char,
      ) != 0)
      {
        /* Exit now if option -e was passed */
        if opt & OPT_e_quit as libc::c_int != 0 {
          return 0i32;
        }
        /* If we're not requested to watch, just keep descriptor open
         * until we are killed */
        if opt & OPT_h_watch as libc::c_int == 0 {
          loop {
            sleep((24i32 * 60i32 * 60i32) as libc::c_uint);
          }
        }
        loop
        /* Watch line for hangup */
        {
          let mut modem_stat: libc::c_int = 0;
          if ioctl(
            3i32,
            0x5415i32 as libc::c_ulong,
            &mut modem_stat as *mut libc::c_int,
          ) != 0
          {
            break;
          }
          if modem_stat & 0x40i32 == 0 {
            break;
          }
          sleep(15i32 as libc::c_uint);
        }
        /* Execute command on hangup */
        if opt & OPT_c_extcmd as libc::c_int != 0 {
          system(extcmd);
        }
        /* Restore states and exit */
        restore_state_and_exit(0i32);
      }
    }
  }
  restore_state_and_exit(1i32);
}

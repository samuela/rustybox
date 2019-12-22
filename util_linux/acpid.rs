use crate::libbb::appletlib::applet_name;
use crate::libbb::parse_config::parser_t;
use crate::librb::signal::__sighandler_t;
use crate::librb::size_t;
use crate::librb::smallint;
use libc;
use libc::close;
use libc::free;
use libc::open;
use libc::openlog;
use libc::pollfd;
use libc::stat;
use libc::strstr;
use libc::timeval;
use libc::unlink;
use libc::FILE;
extern "C" {

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  /* After v = xrealloc_vector(v, SHIFT, idx) it's ok to use
   * at least v[idx] and v[idx+1], for all idx values.
   * SHIFT specifies how many new elements are added (1:2, 2:4, ..., 8:256...)
   * when all elements are used up. New elements are zeroed out.
   * xrealloc_vector(v, SHIFT, idx) *MUST* be called with consecutive IDXs -
   * skipping an index is a bad bug - it may miss a realloc!
   */

  // NB: will return short read on error, not -1,
  // if some data was read before error occurred

  // Reads one line a-la fgets (but doesn't save terminating '\n').
  // Reads byte-by-byte. Useful when it is important to not read ahead.
  // Bytes are appended to pfx (which must be malloced, or NULL).

  /* Wrapper which restarts poll on EINTR or ENOMEM.
   * On other errors complains [perror("poll")] and returns.
   * Warning! May take (much) longer than timeout_ms to return!
   * If this is a problem, use bare poll and open-code EINTR/ENOMEM handling */

  /* Specialized: */
  /* Using xatoi() instead of naive atoi() is not always convenient -
   * in many places people want *non-negative* values, but store them
   * in signed int. Therefore we need this one:
   * dies if input is not in [0, INT_MAX] range. Also will reject '-0' etc.
   * It should really be named xatoi_nonnegative (since it allows 0),
   * but that would be too long.
   */

  /* Useful for reading port numbers */

  /* NOMMU friendy fork+exec: */

  /* { "-", NULL } */
  #[no_mangle]
  static mut option_mask32: u32;

  /* BTW, surprisingly, changing API to
   *   llist_t *llist_add_to(llist_t *old_head, void *data)
   * etc does not result in smaller code... */
  /* start_stop_daemon and udhcpc are special - they want
   * to create pidfiles regardless of FEATURE_PIDFILE */
  /* True only if we created pidfile which is *file*, not /dev/null etc */
  #[no_mangle]
  static mut wrote_pidfile: smallint;

  #[no_mangle]
  static mut logmode: smallint;

  /* delims[0] is a comment char (use '\0' to disable), the rest are token delimiters */

  /* '*const' ptr makes gcc optimize code much better.
   * Magic prevents ptr_to_globals from going into rodata.
   * If you want to assign a value, use SET_PTR_TO_GLOBALS(x) */
  #[no_mangle]
  static ptr_to_globals: *mut globals;

}

pub type nfds_t = libc::c_ulong;

/* Helpers for daemonization.
 *
 * bb_daemonize(flags) = daemonize, does not compile on NOMMU
 *
 * bb_daemonize_or_rexec(flags, argv) = daemonizes on MMU (and ignores argv),
 *      rexec's itself on NOMMU with argv passed as command line.
 * Thus bb_daemonize_or_rexec may cause your <applet>_main() to be re-executed
 * from the start. (It will detect it and not reexec again second time).
 * You have to audit carefully that you don't do something twice as a result
 * (opening files/sockets, parsing config files etc...)!
 *
 * Both of the above will redirect fd 0,1,2 to /dev/null and drop ctty
 * (will do setsid()).
 *
 * fork_or_rexec(argv) = bare-bones fork on MMU,
 *      "vfork + re-exec ourself" on NOMMU. No fd redirection, no setsid().
 *      On MMU ignores argv.
 *
 * Helper for network daemons in foreground mode:
 *
 * bb_sanitize_stdio() = make sure that fd 0,1,2 are opened by opening them
 * to /dev/null if they are not.
 */

pub type C2RustUnnamed = libc::c_uint;
// pub const DAEMON_ONLY_SANITIZE: C2RustUnnamed = 8;
pub const DAEMON_CLOSE_EXTRA_FDS: C2RustUnnamed = 4;
// pub const DAEMON_DEVNULL_STDIO: C2RustUnnamed = 2;
// pub const DAEMON_CHDIR_ROOT: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
// pub const LOGMODE_BOTH: C2RustUnnamed_0 = 3;
pub const LOGMODE_SYSLOG: C2RustUnnamed_0 = 2;
pub const LOGMODE_STDIO: C2RustUnnamed_0 = 1;
// pub const LOGMODE_NONE: C2RustUnnamed_0 = 0;

/* internal use */
//DAEMON_DOUBLE_FORK     = 1 << 4, /* double fork to avoid controlling tty */
/*
 * Config file parser
 */
// comments are recognized even if there is whitespace before
// ("line start><space><tab><space>#comment" is also comment, not only "line start>#comment")
// NORMAL is:
// * remove leading and trailing delimiters and collapse
//   multiple delimiters into one
// * warn and continue if less than mintokens delimiters found
// * grab everything into last token
// * comments are recognized even if they aren't the first char

pub type C2RustUnnamed_1 = libc::c_uint;
pub const PARSE_NORMAL: C2RustUnnamed_1 = 4653056;
// delim[0] and delim[1] are two different allowed comment chars
// (so far, delim[0] will only work as comment char for full-line comment)
// (IOW: it works as if PARSE_EOL_COMMENTS is not set. sysctl applet is okay with this)
// pub const PARSE_WS_COMMENTS: C2RustUnnamed_1 = 16777216;
// comments are recognized even if they aren't the first char
// pub const PARSE_ALT_COMMENTS: C2RustUnnamed_1 = 8388608;
// pub const PARSE_EOL_COMMENTS: C2RustUnnamed_1 = 4194304;
// die if < min tokens found
// keep a copy of current line
// pub const PARSE_KEEP_COPY: C2RustUnnamed_1 = 2097152;
// last token takes entire remainder of the line
// pub const PARSE_MIN_DIE: C2RustUnnamed_1 = 1048576;
// trim leading and trailing delimiters
// TODO: COLLAPSE and TRIM seem to always go in pair
// pub const PARSE_GREEDY: C2RustUnnamed_1 = 262144;
// treat consecutive delimiters as one
// pub const PARSE_TRIM: C2RustUnnamed_1 = 131072;
// pub const PARSE_COLLAPSE: C2RustUnnamed_1 = 65536;

//extern const int const_int_1;

/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct globals {
  pub act_tab: *mut acpi_action,
  pub n_act: libc::c_int,
  pub evt_tab: *mut acpi_event,
  pub n_evt: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_event {
  pub s_type: *const libc::c_char,
  pub n_type: u16,
  pub s_code: *const libc::c_char,
  pub n_code: u16,
  pub value: u32,
  pub desc: *const libc::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_action {
  pub key: *const libc::c_char,
  pub action: *const libc::c_char,
}

pub type __u16 = libc::c_ushort;
pub type __s32 = libc::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct input_event {
  pub time: timeval,
  pub type_0: __u16,
  pub code: __u16,
  pub value: __s32,
}

pub type C2RustUnnamed_2 = libc::c_uint;
// pub const OPT_p: C2RustUnnamed_2 = 128;
// pub const OPT_M: C2RustUnnamed_2 = 64;
// pub const OPT_a: C2RustUnnamed_2 = 32;
// pub const OPT_l: C2RustUnnamed_2 = 16;
pub const OPT_f: C2RustUnnamed_2 = 8;
pub const OPT_e: C2RustUnnamed_2 = 4;
pub const OPT_d: C2RustUnnamed_2 = 2;
// pub const OPT_c: C2RustUnnamed_2 = 1;

#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
static mut f_evt_tab: [acpi_event; 3] = [
  {
    let mut init = acpi_event {
      s_type: b"EV_KEY\x00" as *const u8 as *const libc::c_char,
      n_type: 0x1i32 as u16,
      s_code: b"KEY_POWER\x00" as *const u8 as *const libc::c_char,
      n_code: 116i32 as u16,
      value: 1i32 as u32,
      desc: b"button/power PWRF 00000080\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = acpi_event {
      s_type: b"EV_KEY\x00" as *const u8 as *const libc::c_char,
      n_type: 0x1i32 as u16,
      s_code: b"KEY_POWER\x00" as *const u8 as *const libc::c_char,
      n_code: 116i32 as u16,
      value: 1i32 as u32,
      desc: b"button/power PWRB 00000080\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = acpi_event {
      s_type: b"EV_SW\x00" as *const u8 as *const libc::c_char,
      n_type: 0x5i32 as u16,
      s_code: b"SW_LID\x00" as *const u8 as *const libc::c_char,
      n_code: 0i32 as u16,
      value: 1i32 as u32,
      desc: b"button/lid LID0 00000080\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
];
static mut f_act_tab: [acpi_action; 2] = [
  {
    let mut init = acpi_action {
      key: b"PWRF\x00" as *const u8 as *const libc::c_char,
      action: b"PWRF/00000080\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
  {
    let mut init = acpi_action {
      key: b"LID0\x00" as *const u8 as *const libc::c_char,
      action: b"LID/00000080\x00" as *const u8 as *const libc::c_char,
    };
    init
  },
];
/*
 * acpid listens to ACPI events coming either in textual form
 * from /proc/acpi/event (though it is marked deprecated,
 * it is still widely used and _is_ a standard) or in binary form
 * from specified evdevs (just use /dev/input/event*).
 * It parses the event to retrieve ACTION and a possible PARAMETER.
 * It then spawns /etc/acpi/<ACTION>[/<PARAMETER>] either via run-parts
 * (if the resulting path is a directory) or directly.
 * If the resulting path does not exist it logs it via perror
 * and continues listening.
 */
unsafe extern "C" fn process_event(mut event: *const libc::c_char) {
  let mut st: stat = std::mem::zeroed();
  let mut handler: *mut libc::c_char =
    crate::libbb::xfuncs_printf::xasprintf(b"./%s\x00" as *const u8 as *const libc::c_char, event);
  let mut args: [*const libc::c_char; 3] = [
    b"run-parts\x00" as *const u8 as *const libc::c_char,
    handler as *const libc::c_char,
    0 as *const libc::c_char,
  ];
  // log the event
  crate::libbb::verror_msg::bb_simple_error_msg(event);
  // spawn handler
  // N.B. run-parts would require scripts to have #!/bin/sh
  // handler is directory? -> use run-parts
  // handler is file? -> run it directly
  if 0i32 == stat(event, &mut st) {
    crate::libbb::vfork_daemon_rexec::spawn((args.as_mut_ptr() as *mut *mut libc::c_char).offset(
      (0i32 as libc::c_uint == st.st_mode & 0o40000i32 as libc::c_uint) as libc::c_int as isize,
    ));
  } else {
    crate::libbb::perror_msg::bb_simple_perror_msg(event);
  }
  free(handler as *mut libc::c_void);
}
unsafe extern "C" fn find_action(
  mut ev: *mut input_event,
  mut buf: *const libc::c_char,
) -> *const libc::c_char {
  let mut action: *const libc::c_char = 0 as *const libc::c_char;
  let mut i: libc::c_int = 0;
  // map event
  i = 0i32;
  while i < (*ptr_to_globals).n_evt {
    if !ev.is_null() {
      if (*ev).type_0 as libc::c_int
        == (*(*ptr_to_globals).evt_tab.offset(i as isize)).n_type as libc::c_int
        && (*ev).code as libc::c_int
          == (*(*ptr_to_globals).evt_tab.offset(i as isize)).n_code as libc::c_int
        && (*ev).value as libc::c_uint == (*(*ptr_to_globals).evt_tab.offset(i as isize)).value
      {
        action = (*(*ptr_to_globals).evt_tab.offset(i as isize)).desc;
        break;
      }
    }
    if !buf.is_null() {
      if !crate::libbb::compare_string_array::is_prefixed_with(
        (*(*ptr_to_globals).evt_tab.offset(i as isize)).desc,
        buf,
      )
      .is_null()
      {
        action = (*(*ptr_to_globals).evt_tab.offset(i as isize)).desc;
        break;
      }
    }
    i += 1
  }
  // get action
  if !action.is_null() {
    i = 0i32;
    while i < (*ptr_to_globals).n_act {
      if !strstr(action, (*(*ptr_to_globals).act_tab.offset(i as isize)).key).is_null() {
        action = (*(*ptr_to_globals).act_tab.offset(i as isize)).action;
        break;
      } else {
        i += 1
      }
    }
  }
  return action;
}
unsafe extern "C" fn parse_conf_file(mut filename: *const libc::c_char) {
  let mut parser: *mut parser_t = std::ptr::null_mut();
  let mut tokens: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
  parser = crate::libbb::parse_config::config_open2(
    filename,
    Some(
      crate::libbb::wfopen::fopen_for_read
        as unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE,
    ),
  );
  if !parser.is_null() {
    while crate::libbb::parse_config::config_read(
      parser,
      tokens.as_mut_ptr(),
      (PARSE_NORMAL as libc::c_int | (2i32 & 0xffi32) << 8i32 | 2i32 & 0xffi32) as libc::c_uint,
      b"# \t\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
      (*ptr_to_globals).act_tab = crate::libbb::xrealloc_vector::xrealloc_vector_helper(
        (*ptr_to_globals).act_tab as *mut libc::c_void,
        ((::std::mem::size_of::<acpi_action>() as libc::c_ulong) << 8i32)
          .wrapping_add(1i32 as libc::c_ulong) as libc::c_uint,
        (*ptr_to_globals).n_act,
      ) as *mut acpi_action;
      let ref mut fresh0 = (*(*ptr_to_globals)
        .act_tab
        .offset((*ptr_to_globals).n_act as isize))
      .key;
      *fresh0 = crate::libbb::xfuncs_printf::xstrdup(tokens[0]);
      let ref mut fresh1 = (*(*ptr_to_globals)
        .act_tab
        .offset((*ptr_to_globals).n_act as isize))
      .action;
      *fresh1 = crate::libbb::xfuncs_printf::xstrdup(tokens[1]);
      (*ptr_to_globals).n_act += 1
    }
    crate::libbb::parse_config::config_close(parser);
  } else {
    (*ptr_to_globals).act_tab = f_act_tab.as_ptr() as *mut libc::c_void as *mut acpi_action;
    (*ptr_to_globals).n_act = (::std::mem::size_of::<[acpi_action; 2]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<acpi_action>() as libc::c_ulong)
      as libc::c_uint as libc::c_int
  };
}
unsafe extern "C" fn parse_map_file(mut filename: *const libc::c_char) {
  let mut parser: *mut parser_t = std::ptr::null_mut();
  let mut tokens: [*mut libc::c_char; 6] = [0 as *mut libc::c_char; 6];
  parser = crate::libbb::parse_config::config_open2(
    filename,
    Some(
      crate::libbb::wfopen::fopen_for_read
        as unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE,
    ),
  );
  if !parser.is_null() {
    while crate::libbb::parse_config::config_read(
      parser,
      tokens.as_mut_ptr(),
      (PARSE_NORMAL as libc::c_int | (6i32 & 0xffi32) << 8i32 | 6i32 & 0xffi32) as libc::c_uint,
      b"# \t\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
      (*ptr_to_globals).evt_tab = crate::libbb::xrealloc_vector::xrealloc_vector_helper(
        (*ptr_to_globals).evt_tab as *mut libc::c_void,
        ((::std::mem::size_of::<acpi_event>() as libc::c_ulong) << 8i32)
          .wrapping_add(1i32 as libc::c_ulong) as libc::c_uint,
        (*ptr_to_globals).n_evt,
      ) as *mut acpi_event;
      let ref mut fresh2 = (*(*ptr_to_globals)
        .evt_tab
        .offset((*ptr_to_globals).n_evt as isize))
      .s_type;
      *fresh2 = crate::libbb::xfuncs_printf::xstrdup(tokens[0]);
      (*(*ptr_to_globals)
        .evt_tab
        .offset((*ptr_to_globals).n_evt as isize))
      .n_type = crate::libbb::xatonum::xstrtou(tokens[1], 16i32) as u16;
      let ref mut fresh3 = (*(*ptr_to_globals)
        .evt_tab
        .offset((*ptr_to_globals).n_evt as isize))
      .s_code;
      *fresh3 = crate::libbb::xfuncs_printf::xstrdup(tokens[2]);
      (*(*ptr_to_globals)
        .evt_tab
        .offset((*ptr_to_globals).n_evt as isize))
      .n_code = crate::libbb::xatonum::xatou16(tokens[3]);
      (*(*ptr_to_globals)
        .evt_tab
        .offset((*ptr_to_globals).n_evt as isize))
      .value = crate::libbb::xatonum::xatoi_positive(tokens[4]) as u32;
      let ref mut fresh4 = (*(*ptr_to_globals)
        .evt_tab
        .offset((*ptr_to_globals).n_evt as isize))
      .desc;
      *fresh4 = crate::libbb::xfuncs_printf::xstrdup(tokens[5]);
      (*ptr_to_globals).n_evt += 1
    }
    crate::libbb::parse_config::config_close(parser);
  } else {
    (*ptr_to_globals).evt_tab = f_evt_tab.as_ptr() as *mut libc::c_void as *mut acpi_event;
    (*ptr_to_globals).n_evt = (::std::mem::size_of::<[acpi_event; 3]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<acpi_event>() as libc::c_ulong)
      as libc::c_uint as libc::c_int
  };
}
/*
 * acpid [-c conf_dir] [-r conf_file ] [-a map_file ] [-l log_file] [-e proc_event_file]
 */
#[no_mangle]
pub unsafe extern "C" fn acpid_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut nfd: libc::c_int = 0;
  let mut opts: libc::c_int = 0;
  let mut pfd: *mut pollfd = std::ptr::null_mut();
  let mut opt_dir: *const libc::c_char = b"/etc/acpi\x00" as *const u8 as *const libc::c_char;
  let mut opt_input: *const libc::c_char =
    b"/dev/input/event\x00" as *const u8 as *const libc::c_char;
  let mut opt_logfile: *const libc::c_char =
    b"/var/log/acpid.log\x00" as *const u8 as *const libc::c_char;
  let mut opt_action: *const libc::c_char =
    b"/etc/acpid.conf\x00" as *const u8 as *const libc::c_char;
  let mut opt_map: *const libc::c_char = b"/etc/acpi.map\x00" as *const u8 as *const libc::c_char;
  let mut opt_pidfile: *const libc::c_char =
    b"/var/run/acpid.pid\x00" as *const u8 as *const libc::c_char;
  let ref mut fresh5 = *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
    as *mut *mut globals);
  *fresh5 = crate::libbb::xfuncs_printf::xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong)
    as *mut globals;
  asm!("" : : : "memory" : "volatile");
  opts = crate::libbb::getopt32::getopt32(
    argv,
    b"^c:de:fl:a:M:p:g:m:s:S:v\x00df:e--e\x00" as *const u8 as *const libc::c_char,
    &mut opt_dir as *mut *const libc::c_char,
    &mut opt_input as *mut *const libc::c_char,
    &mut opt_logfile as *mut *const libc::c_char,
    &mut opt_action as *mut *const libc::c_char,
    &mut opt_map as *mut *const libc::c_char,
    &mut opt_pidfile as *mut *const libc::c_char,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
  ) as libc::c_int;
  if opts & OPT_f as libc::c_int == 0 {
    /* No -f "Foreground", we go to background */
    crate::libbb::vfork_daemon_rexec::bb_daemonize_or_rexec(DAEMON_CLOSE_EXTRA_FDS as libc::c_int);
  }
  if opts & OPT_d as libc::c_int == 0 {
    /* No -d "Debug", we log to log file.
     * This includes any output from children.
     */
    crate::libbb::xfuncs_printf::xmove_fd(
      crate::libbb::xfuncs_printf::xopen(opt_logfile, 0o1i32 | 0o100i32 | 0o2000i32),
      1i32,
    );
    crate::libbb::xfuncs_printf::xdup2(1i32, 2i32);
    /* Also, acpid's messages (but not children) will go to syslog too */
    openlog(applet_name, 0x1i32, 3i32 << 3i32);
    logmode = (LOGMODE_SYSLOG as libc::c_int | LOGMODE_STDIO as libc::c_int) as smallint
  }
  /* else: -d "Debug", log is not redirected */
  parse_conf_file(opt_action);
  parse_map_file(opt_map);
  crate::libbb::xfuncs_printf::xchdir(opt_dir);
  /* We spawn children but don't wait for them. Prevent zombies: */
  crate::libbb::signals::bb_signals(
    1i32 << 17i32,
    ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
  );
  // If you enable this, (1) explain why, (2)
  // make sure while(poll) loop below is still interruptible
  // by SIGTERM et al:
  //bb_signals(BB_FATAL_SIGS, record_signo);
  pfd = std::ptr::null_mut(); /* this fd has nothing */
  nfd = 0i32;
  loop {
    let mut fd: libc::c_int = 0;
    let mut dev_event: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    dev_event = crate::libbb::xfuncs_printf::xasprintf(
      if opts & OPT_e as libc::c_int != 0 {
        b"%s\x00" as *const u8 as *const libc::c_char
      } else {
        b"%s%u\x00" as *const u8 as *const libc::c_char
      },
      opt_input,
      nfd,
    );
    fd = open(dev_event, 0i32 | 0o4000i32);
    if fd < 0i32 {
      if nfd == 0i32 {
        crate::libbb::perror_msg::bb_simple_perror_msg_and_die(dev_event);
      }
      break;
    } else {
      free(dev_event as *mut libc::c_void);
      pfd = crate::libbb::xrealloc_vector::xrealloc_vector_helper(
        pfd as *mut libc::c_void,
        ((::std::mem::size_of::<pollfd>() as libc::c_ulong) << 8i32)
          .wrapping_add(1i32 as libc::c_ulong) as libc::c_uint,
        nfd,
      ) as *mut pollfd;
      (*pfd.offset(nfd as isize)).fd = fd;
      (*pfd.offset(nfd as isize)).events = 0x1i32 as libc::c_short;
      nfd += 1
    }
  }
  crate::libbb::pidfile::write_pidfile(opt_pidfile);
  while crate::libbb::safe_poll::safe_poll(pfd, nfd as nfds_t, -1i32) > 0i32 {
    let mut i: libc::c_int = 0;
    let mut current_block_47: u64;
    i = 0i32;
    while i < nfd {
      let mut event: *const libc::c_char = 0 as *const libc::c_char;
      if (*pfd.offset(i as isize)).revents as libc::c_int & 0x1i32 == 0 {
        if !((*pfd.offset(i as isize)).revents as libc::c_int == 0i32) {
          /* Likely POLLERR, POLLHUP, POLLNVAL.
           * Do not listen on this fd anymore.
           */
          close((*pfd.offset(i as isize)).fd);
          nfd -= 1;
          while i < nfd {
            (*pfd.offset(i as isize)).fd = (*pfd.offset((i + 1i32) as isize)).fd;
            i += 1
          }
          break;
        }
      /* do poll() again */
      } else {
        event = 0 as *const libc::c_char;
        if option_mask32 & OPT_e as libc::c_int as libc::c_uint != 0 {
          let mut buf: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
          let mut len: libc::c_int = 0;
          buf = crate::libbb::read_printf::xmalloc_reads(
            (*pfd.offset(i as isize)).fd,
            std::ptr::null_mut::<size_t>(),
          );
          /* buf = "button/power PWRB 00000080 00000000" */
          len = strlen(buf).wrapping_sub(9i32 as libc::c_ulong) as libc::c_int;
          if len >= 0i32 {
            *buf.offset(len as isize) = '\u{0}' as i32 as libc::c_char
          }
          event = find_action(0 as *mut input_event, buf);
          free(buf as *mut libc::c_void);
          current_block_47 = 17075014677070940716;
        } else {
          let mut ev: input_event = input_event {
            time: timeval {
              tv_sec: 0,
              tv_usec: 0,
            },
            type_0: 0,
            code: 0,
            value: 0,
          };
          if ::std::mem::size_of::<input_event>() as libc::c_ulong
            != crate::libbb::read::full_read(
              (*pfd.offset(i as isize)).fd,
              &mut ev as *mut input_event as *mut libc::c_void,
              ::std::mem::size_of::<input_event>() as libc::c_ulong,
            ) as libc::c_ulong
          {
            current_block_47 = 7226443171521532240;
          } else if ev.value != 1i32 && ev.value != 0i32 {
            current_block_47 = 7226443171521532240;
          } else {
            event = find_action(&mut ev, 0 as *const libc::c_char);
            current_block_47 = 17075014677070940716;
          }
        }
        match current_block_47 {
          7226443171521532240 => {}
          _ => {
            if !event.is_null() {
              /* spawn event handler */
              process_event(event);
            }
          }
        }
      }
      i += 1
    }
  }
  if wrote_pidfile != 0 {
    unlink(opt_pidfile);
  }
  return 0i32;
}

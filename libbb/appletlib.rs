use crate::applets::applet_tables::applets;
use crate::applets::applet_tables::Entrypoint;
use crate::applets::applet_tables::InstallLoc;
use crate::applets::applet_tables::SUID;
use crate::libbb::llist::llist_t;
use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::skip_whitespace::skip_whitespace;
use crate::libpwdgrp::pwd_grp::bb_internal_getgrgid;
use crate::libpwdgrp::pwd_grp::bb_internal_getpwnam;
use crate::librb::bb_uidgid_t;
use crate::librb::smallint;
use libc;
use libc::gid_t;
use libc::group;
use libc::mode_t;
use libc::passwd;
use libc::setresgid;
use libc::setresuid;
use libc::strcasecmp;
use libc::uid_t;
use std::ffi::CStr;
use std::ffi::CString;
use std::path::Path;
extern "C" {

  #[no_mangle]
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut libc::FILE,
  ) -> *mut libc::c_char;

  #[no_mangle]
  fn feof_unlocked(__stream: *mut libc::FILE) -> libc::c_int;

  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  static bb_busybox_exec_path: [libc::c_char; 0];

  #[no_mangle]
  static bb_msg_requires_arg: [libc::c_char; 0];

  #[no_mangle]
  static mut xfunc_error_retval: u8;
}

// This struct is actually defined in appletlib.c.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct suid_config_t {
  /* next ptr must be first: this struct needs to be llist-compatible */
  pub m_next: *mut suid_config_t,
  pub m_ugid: bb_uidgid_t,
  pub m_applet: libc::c_int,
  pub m_mode: mode_t,
}
static mut suid_config: *mut suid_config_t = std::ptr::null_mut();
static mut suid_cfg_readable: bool = false;

pub unsafe fn string_array_len(argv: *mut *mut libc::c_char) -> libc::c_uint {
  let mut start: *mut *mut libc::c_char = argv;
  let mut current: *mut *mut libc::c_char = argv;
  while !(*current).is_null() {
    current = current.offset(1)
  }
  return current.wrapping_offset_from(start) as libc::c_long as libc::c_uint;
}

pub unsafe fn bb_show_usage() -> ! {
  let aname = ptr_to_str(applet_name);
  let usage_msg = usage(&aname).expect("Applet usage failed.");
  println!("Usage: {} {}", aname, usage_msg);
  crate::libbb::xfunc_die::xfunc_die();
}

/* The code below can well be in applets/applets.c, as it is used only
 * for busybox binary, not "individual" binaries.
 * However, keeping it here and linking it into libbusybox.so
 * (together with remaining tiny applets/applets.o)
 * makes it possible to avoid --whole-archive at link time.
 * This makes (shared busybox) + libbusybox smaller.
 * (--gc-sections would be even better....)
 */
pub static mut applet_name: *const libc::c_char = std::ptr::null();

static mut ruid: uid_t = 0; /* real uid */

/* libbb candidate */
unsafe fn get_trimmed_slice(
  mut s: *mut libc::c_char,
  mut e: *mut libc::c_char,
) -> *mut libc::c_char {
  /* First, consider the value at e to be nul and back up until we
   * reach a non-space char.  Set the char after that (possibly at
   * the original e) to nul. */
  loop {
    let fresh2 = e;
    e = e.offset(-1);
    if !(fresh2 > s) {
      break;
    }
    if ({
      let mut bb__isspace: libc::c_uchar = (*e as libc::c_int - 9i32) as libc::c_uchar;
      (bb__isspace as libc::c_int == ' ' as i32 - 9i32
        || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
    }) == 0
    {
      break;
    }
  }
  *e.offset(1) = '\u{0}' as i32 as libc::c_char;

  /* Next, advance past all leading space and return a ptr to the
   * first non-space char; possibly the terminating nul. */
  return skip_whitespace(s);
}

unsafe fn parse_config_file() {
  /* Don't depend on the tools to combine strings. */
  let config_file = "/etc/rustybox.conf";

  let mut sct_head: *mut suid_config_t = std::ptr::null_mut();
  let mut applet_no: libc::c_int = 0;
  let mut f: *mut libc::FILE = std::ptr::null_mut();
  let mut errmsg: *const libc::c_char = std::ptr::null();
  let mut lc: libc::c_uint = 0;
  let mut section: smallint = 0;
  let mut st: libc::stat = std::mem::zeroed();

  ruid = libc::getuid();
  if ruid == 0 {
    /* run by root - don't need to even read config file */
    return;
  }

  if libc::stat(str_to_ptr(config_file), &mut st) != 0      /* No config file? */
    || !(st.st_mode & 0o170000 == 0o100000)                 /* Not a regular file? */
    || st.st_uid != 0                                       /* Not owned by root? */
    || st.st_mode & (0o200 >> 3 | 0o200 >> 3 >> 3) != 0     /* Writable by non-root? */
    || {
      /* Cannot open? */
      f = crate::libbb::wfopen::fopen_for_read(str_to_ptr(config_file));
      f.is_null()
    }
  {
    return;
  }

  suid_cfg_readable = true;
  sct_head = std::ptr::null_mut();
  lc = 0;
  section = 0;

  's_65: loop {
    let mut buffer: [libc::c_char; 256] = [0; 256];
    let mut s: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    if fgets_unlocked(
      buffer.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
      f,
    )
    .is_null()
    {
      /* Are we done? */
      // Looks like bloat
      //if (ferror(f)) {   /* Make sure it wasn't a read error. */
      //	errmsg = "reading";
      //	goto pe_label;
      //}
      libc::fclose(f);
      suid_config = sct_head; /* Success, so set the pointer. */
      return;
    }

    s = buffer.as_mut_ptr();
    lc = lc.wrapping_add(1); /* Got a (partial) line. */

    /* If a line is too long for our buffer, we consider it an error.
     * The following test does mistreat one corner case though.
     * If the final line of the file does not end with a newline and
     * yet exactly fills the buffer, it will be treated as too long
     * even though there isn't really a problem.  But it isn't really
     * worth adding code to deal with such an unlikely situation, and
     * we do err on the side of caution.  Besides, the line would be
     * too long if it did end with a newline. */
    if libc::strchr(s, '\n' as i32).is_null() && feof_unlocked(f) == 0 {
      errmsg = b"line too long\x00" as *const u8 as *const libc::c_char;
      break;
    } else {
      /* Trim leading and trailing whitespace, ignoring comments, and
       * check if the resulting string is empty. */
      s = get_trimmed_slice(s, strchrnul(s, '#' as i32));
      if *s == 0 {
        continue;
      }
      /* Check for a section header. */
      if *s as libc::c_int == '[' as i32 {
        /* Unlike the old code, we ignore leading and trailing
         * whitespace for the section name.  We also require that
         * there are no stray characters after the closing bracket. */
        let mut e: *mut libc::c_char = libc::strchr(s, ']' as i32);
        if e.is_null() || *e.offset(1) as libc::c_int != 0 || {
          s = get_trimmed_slice(s.offset(1), e);
          (*s) == 0
        } {
          /* Missing name? */
          errmsg = b"section header\x00" as *const u8 as *const libc::c_char;
          break;
        } else if strcasecmp(s, b"SUID\x00" as *const u8 as *const libc::c_char) == 0 {
          section = 1
        } else {
          /* Right now we only have one section so just check it.
           * If more sections are added in the future, please don't
           * resort to cascading ifs with multiple strcasecmp calls.
           * That kind of bloated code is all too common.  A loop
           * and a string table would be a better choice unless the
           * number of sections is very small. */
          section = -1
        }
      } else if section as libc::c_int == 1i32 {
        /* Unknown section so set to skip. */
        /* Process sections. */
        /* SUID */
        /* Since we trimmed leading and trailing space above, we're
         * now looking for strings of the form
         *    <key>[::space::]*=[::space::]*<value>
         * where both key and value could contain inner whitespace. */
        /* First get the key (an applet name in our case). */
        let mut e_0: *mut libc::c_char = libc::strchr(s, '=' as i32);
        if !e_0.is_null() {
          s = get_trimmed_slice(s, e_0)
        }
        if e_0.is_null() || *s == 0 {
          /* Missing '=' or empty key. */
          errmsg = b"keyword\x00" as *const u8 as *const libc::c_char;
          break;
        } else {
          /* Ok, we have an applet name.  Process the rhs if this
           * applet is currently built in and ignore it otherwise.
           * Note: this can hide config file bugs which only pop
           * up when the busybox configuration is changed. */
          match find_applet_by_name(&ptr_to_str(s)) {
            None => continue,
            Some(n) => applet_no = n as i32,
          };

          let mut i: libc::c_uint = 0;
          let mut sct: *mut suid_config_t = std::ptr::null_mut();
          /* Note: We currently don't check for duplicates!
           * The last config line for each applet will be the
           * one used since we insert at the head of the list.
           * I suppose this could be considered a feature. */
          sct = crate::libbb::xfuncs_printf::xzalloc(::std::mem::size_of::<suid_config_t>() as u64)
            as *mut suid_config_t;
          (*sct).m_applet = applet_no;
          /*sct->m_mode = 0;*/
          (*sct).m_next = sct_head;
          sct_head = sct;
          /* Get the specified mode. */
          e_0 = skip_whitespace(e_0.offset(1));
          i = 0 as libc::c_uint;
          while i < 3i32 as libc::c_uint {
            /* There are 4 chars for each of user/group/other.
             * "x-xx" instead of "x-" are to make
             * "idx > 3" check catch invalid chars.
             */
            static mut mode_chars: [libc::c_char; 13] =
              [83, 115, 120, 45, 83, 115, 120, 45, 120, 45, 120, 120, 0];
            static mut mode_mask: [libc::c_ushort; 10] = [
              0o4000i32 as libc::c_ushort,
              (0o4000i32 | 0o100i32) as libc::c_ushort,
              0o100i32 as libc::c_ushort,
              0 as libc::c_ushort,
              0o2000i32 as libc::c_ushort,
              (0o2000i32 | 0o100i32 >> 3i32) as libc::c_ushort,
              (0o100i32 >> 3i32) as libc::c_ushort,
              0 as libc::c_ushort,
              (0o100i32 >> 3i32 >> 3i32) as libc::c_ushort,
              0 as libc::c_ushort,
            ];
            let mut q: *const libc::c_char = strchrnul(
              mode_chars
                .as_ptr()
                .offset((4i32 as libc::c_uint).wrapping_mul(i) as isize),
              *e_0 as libc::c_int,
            );
            let mut idx: libc::c_uint = q.wrapping_offset_from(
              mode_chars
                .as_ptr()
                .offset((4i32 as libc::c_uint).wrapping_mul(i) as isize),
            ) as libc::c_long as libc::c_uint;
            if idx > 3i32 as libc::c_uint {
              errmsg = b"mode\x00" as *const u8 as *const libc::c_char;
              break 's_65;
            } else {
              (*sct).m_mode |= mode_mask
                [q.wrapping_offset_from(mode_chars.as_ptr()) as libc::c_long as usize]
                as libc::c_uint;
              e_0 = e_0.offset(1);
              i = i.wrapping_add(1)
            }
          }
          /* Now get the user/group info. */
          s = skip_whitespace(e_0);
          /* Default is 0.0, else parse USER.GROUP: */
          if !(*s != 0) {
            continue;
          }
          /* We require whitespace between mode and USER.GROUP */
          if s == e_0 || {
            e_0 = libc::strchr(s, '.' as i32); /* get_uidgid needs USER:GROUP syntax */
            e_0.is_null()
          } {
            errmsg = b"uid.gid\x00" as *const u8 as *const libc::c_char;
            break;
          } else {
            *e_0 = ':' as i32 as libc::c_char;
            if !(crate::libpwdgrp::uidgid_get::get_uidgid(&mut (*sct).m_ugid, s) == 0) {
              continue;
            }
            errmsg = b"unknown user/group\x00" as *const u8 as *const libc::c_char;
            break;
          }
        }
      } else {
        /* Unknown sections are ignored. */
        /* Encountering configuration lines prior to seeing a
         * section header is treated as an error.  This is how
         * the old code worked, but it may not be desirable.
         * We may want to simply ignore such lines in case they
         * are used in some future version of busybox. */
        if !(section == 0) {
          continue;
        }
        errmsg = b"keyword outside section\x00" as *const u8 as *const libc::c_char;
        break;
      }
    }
  }
  libc::fclose(f);
  crate::libbb::verror_msg::bb_error_msg(
    b"parse error in %s, line %u: %s\x00" as *const u8 as *const libc::c_char,
    config_file.as_ptr(),
    lc,
    errmsg,
  );

  /* Release any allocated memory before returning. */
  crate::libbb::llist::llist_free(sct_head as *mut llist_t);
}

/* check if u is member of group g */
unsafe fn ingroup(mut u: uid_t, mut g: gid_t) -> bool {
  let mut grp: *mut group = bb_internal_getgrgid(g); /* real gid */
  if !grp.is_null() {
    let mut mem: *mut *mut libc::c_char = std::ptr::null_mut(); /* run by root - no need to check more */
    mem = (*grp).gr_mem;
    while !(*mem).is_null() {
      let mut pwd: *mut passwd = bb_internal_getpwnam(*mem);
      if !pwd.is_null() && (*pwd).pw_uid == u {
        return true;
      }
      mem = mem.offset(1)
    }
  }
  return false;
}

unsafe fn check_suid(applet_no: usize) {
  let mut current_block: u64;

  let mut rgid: gid_t = 0; /* real gid */

  /* ruid set by parse_config_file() */
  if ruid == 0 {
    /* run by root - no need to check more */
    return;
  }
  rgid = libc::getgid();

  if suid_cfg_readable {
    let mut uid: uid_t = 0;
    let mut sct: *mut suid_config_t = std::ptr::null_mut();
    let mut m: mode_t = 0;
    sct = suid_config;
    loop {
      if sct.is_null() {
        current_block = 7187160828046810477;
        break;
      }
      if (*sct).m_applet == applet_no as i32 {
        current_block = 14059243314339256598;
        break;
      }
      sct = (*sct).m_next
    }
    match current_block {
      7187160828046810477 => {}
      _ => {
        /* Is this user allowed to run this applet? */
        m = (*sct).m_mode;
        if (*sct).m_ugid.uid == ruid {
          /* same uid */
          m >>= 6i32
        } else if (*sct).m_ugid.gid == rgid || ingroup(ruid, (*sct).m_ugid.gid) {
          /* same group / in group */
          m >>= 3i32
        }
        if m & (0o100i32 >> 3i32 >> 3i32) as libc::c_uint == 0 {
          /* is x bit not set? */
          crate::libbb::verror_msg::bb_simple_error_msg_and_die(
            b"you have no permission to run this applet\x00" as *const u8 as *const libc::c_char,
          );
        }
        /* We set effective AND saved ids. If saved-id is not set
         * like we do below, seteuid(0) can still later succeed! */
        /* Are we directed to change gid
         * (APPLET = *s* USER.GROUP or APPLET = *S* USER.GROUP)?
         */
        if (*sct).m_mode & 0o2000i32 as libc::c_uint != 0 {
          rgid = (*sct).m_ugid.gid
        }
        /* else: we will set egid = rgid, thus dropping sgid effect */
        if setresgid(-1i32 as gid_t, rgid, rgid) != 0 {
          crate::libbb::perror_msg::bb_simple_perror_msg_and_die(
            b"setresgid\x00" as *const u8 as *const libc::c_char,
          );
        }
        /* Are we directed to change uid
         * (APPLET = s** USER.GROUP or APPLET = S** USER.GROUP)?
         */
        uid = ruid;
        if (*sct).m_mode & 0o4000i32 as libc::c_uint != 0 {
          uid = (*sct).m_ugid.uid
        }
        /* else: we will set euid = ruid, thus dropping suid effect */
        if setresuid(-1i32 as uid_t, uid, uid) != 0 {
          crate::libbb::perror_msg::bb_simple_perror_msg_and_die(
            b"setresuid\x00" as *const u8 as *const libc::c_char,
          );
        }
        current_block = 14136749492126903395;
      }
    }
  } else {
    current_block = 7187160828046810477;
  }
  match current_block {
    7187160828046810477 => {
      if applets[applet_no].need_suid == SUID::SUID_REQUIRE {
        /* Real uid is not 0. If euid isn't 0 too, suid bit
         * is most probably not set on our executable */
        if libc::geteuid() != 0 {
          crate::libbb::verror_msg::bb_simple_error_msg_and_die(
            b"must be suid to work properly\x00" as *const u8 as *const libc::c_char,
          );
        }
      } else if applets[applet_no].need_suid == SUID::SUID_DROP {
        /*
         * Drop all privileges.
         *
         * Don't check for errors: in normal use, they are impossible,
         * and in special cases, exiting is harmful. Example:
         * 'unshare --user' when user's shell is also from busybox.
         *
         * 'unshare --user' creates a new user namespace without any
         * uid mappings. Thus, busybox binary is setuid nobody:nogroup
         * within the namespace, as that is the only user. However,
         * since no uids are mapped, calls to setgid/setuid
         * fail (even though they would do nothing).
         */
        libc::setgid(rgid);
        libc::setuid(ruid);
      }
    }
    _ => {}
  }
  crate::libbb::llist::llist_free(suid_config as *mut llist_t);
}

/* create (sym)links for each applet */
fn install_links(
  rustybox_path: &Path,
  use_symbolic_links: bool,
  custom_install_dir: Option<&Path>,
) {
  let lf = if use_symbolic_links {
    std::os::unix::fs::symlink
  } else {
    std::fs::hard_link
  };

  for app in applets.iter() {
    let default_dst_str = install_loc_to_string(app.install_loc);
    let dst = custom_install_dir
      .unwrap_or_else(|| Path::new(&default_dst_str))
      .join(app.name);

    eprintln!(
      "linky linkin {} -> {}",
      rustybox_path.display(),
      dst.display()
    );
    lf(rustybox_path, dst).expect("Failed to link!");
  }
}

unsafe fn print_rustybox_help() {
  /* -1 prevent last comma to be in the very last pos */
  let output_width = crate::libbb::xfuncs::get_terminal_width(2) - 1;

  // TODO: add version info to this banner.
  eprintln!(
    "\
█▀▀█ █  █ █▀▀ ▀▀█▀▀ █  █ █▀▀▄ █▀▀█ █ █
█▄▄▀ █  █ ▀▀█   █   █▄▄█ █▀▀▄ █  █ ▄▀▄
▀ ▀▀  ▀▀▀ ▀▀▀   ▀   ▄▄▄█ ▀▀▀  ▀▀▀▀ ▀ ▀

RustyBox is a BusyBox fork written entirely in Rust. Kudos go to
the developers of the BusyBox and c2rust projects!

Usage: rustybox [function [arguments]...]
   or: rustybox --list[-full]
   or: rustybox --install [-s] [DIR]
   or: function [arguments]...

\tRustyBox is a multi-call binary that combines many common Unix
\tutilities into a single executable.  Most people will create a
\tlink to rustybox for each function they wish to use and RustyBox
\twill act like whatever it was invoked as.

Functions:"
  );

  let mut col: libc::c_int = 0;
  for app in applets.iter() {
    let len2 = app.name.len() + 2;
    if col >= output_width - len2 as i32 {
      eprintln!(",");
      col = 0
    }
    if col == 0 {
      col = 6;
      eprint!("\t")
    } else {
      eprint!(", ")
    }
    eprint!("{}", app.name);
    col += len2 as i32;
  }
  eprintln!();
}

unsafe fn rustybox_main(argv: &[&str]) -> i32 {
  if argv.len() == 1 {
    /* Called without arguments */
    print_rustybox_help();
    return 0;
  } else {
    if argv[1] == "--list" {
      for applet in applets.iter() {
        println!("{}", applet.name);
      }
      return 0;
    }
    if argv[1] == "--list-full" {
      for applet in applets.iter() {
        println!(
          "{}{}",
          &install_loc_to_string(applet.install_loc)[1..],
          applet.name
        );
      }
      return 0;
    }

    if argv[1] == "--install" {
      // According to the docs using this can have negative security
      // implications in some scenarios. In the future we should ask the user to
      // confirm this value.
      let current_exe = std::env::current_exe()
        .expect("Unable to get std::env::current_exe()")
        .canonicalize()
        .expect("Could not get absolute path of the rustybox executable.");

      /* busybox --install [-s] [DIR]:
       * -s: make symlinks
       * DIR: directory to install links to
       */
      let use_symbolic_links = (argv.len() > 2) && (argv[2] == "-s");
      let custom_dir = argv
        .get(if use_symbolic_links { 3 } else { 2 })
        .map(Path::new);
      install_links(&current_exe, use_symbolic_links, custom_dir);
      return 0;
    }

    if argv[1] == "--help" {
      /* "busybox --help [<applet>]" */
      if argv.len() < 3 {
        // Missing the applet to ask for help with.
        print_rustybox_help();
        return 0;
      } else {
        /* convert to "<applet> --help" */
        run_applet_and_exit(&argv[2], &[argv[2].clone(), "--help".into()]);
      }
    }

    /* "busybox <applet> arg1 arg2 ..." */
    /* We support "busybox /a/path/to/applet args..." too. Allows for
     * "#!/bin/busybox"-style wrappers */
    applet_name = crate::libbb::get_last_path_component::bb_get_last_path_component_nostrip(
      str_to_ptr(&argv[1]),
    );
    run_applet_and_exit(&ptr_to_str(applet_name), &argv[1..]);
  }
}

unsafe fn run_applet_no_and_exit(applet_no: usize, name: &str, argv: &[&str]) -> ! {
  let argc = argv.len() as i32;

  /* We do not use argv[0]: do not want to repeat massaging of
   * "-/sbin/halt" -> "halt", for example. */
  applet_name = str_to_ptr(name);

  // Special case. POSIX says "test --help" should be no different from e.g.
  // "test --foo". Thus for "test", we skip --help check. "true" and "false" are
  // also special.
  let main_name = applets[applet_no].main;
  if main_name != "test" && main_name != "true" && main_name != "false" {
    if argc == 2 && argv[1] == "--help" {
      /* Make "foo --help" exit with 0: */
      xfunc_error_retval = 0;
      bb_show_usage();
    }
  }

  check_suid(applet_no);

  match applets[applet_no].entrypoint {
    Entrypoint::CStyle(f) => {
      xfunc_error_retval = f(argc, str_vec_to_ptrs(argv)) as u8;

      /* Note: applet_main() may also not return (die on a xfunc or such) */
      crate::libbb::xfunc_die::xfunc_die();
    }
    Entrypoint::SafeStyle(f) => f(argv),
  }
}

unsafe fn run_applet_and_exit(name: &str, argv: &[&str]) -> ! {
  if name == "rustybox" {
    ::std::process::exit(rustybox_main(argv));
  } else {
    /* find_applet_by_name() search is more expensive, so goes second */
    match find_applet_by_name(name) {
      None => {
        eprintln!("{}: applet not found", ptr_to_str(applet_name));

        /* POSIX: "If a command is not found, the exit status shall be 127" */
        ::std::process::exit(127);
      }
      Some(applet_no) => run_applet_no_and_exit(applet_no, name, argv),
    }
  }
}

pub unsafe fn main() {
  // This is absolutely essential to fix bb_errno which is really the same as
  // errno. In the future we should come up with a more elegant approach to
  // interfacing with errno.
  bb_errno = libc::__errno_location();
  *bb_errno = 0;

  let argv: Vec<String> = ::std::env::args().collect();
  applet_name =
    crate::libbb::get_last_path_component::bb_basename(str_to_ptr(argv[0].trim_start_matches('-')));
  parse_config_file(); /* ...maybe, if FEATURE_SUID_CONFIG */
  run_applet_and_exit(
    &ptr_to_str(applet_name),
    &argv.iter().map(String::as_ref).collect::<Vec<&str>>(),
  )
}

unsafe fn ptr_to_str(strptr: *const libc::c_char) -> String {
  CStr::from_ptr(strptr).to_string_lossy().into_owned()
}

fn str_to_ptr(string: &str) -> *mut libc::c_char {
  CString::new(string.as_bytes())
    .expect("CString::new failed.")
    .into_raw()
}

fn str_vec_to_ptrs(strings: &[&str]) -> *mut *mut libc::c_char {
  let mut ret: Vec<*mut libc::c_char> = Vec::new();
  for arg in strings {
    ret.push(str_to_ptr(arg));
  }
  ret.push(::std::ptr::null_mut());

  // This is necessary because otherwise `ret` is dropped prematurely. We need
  // the pointer to remain valid when calling into C code. Probably introduces a
  // small memory leak, but we'll live with it for now.
  let mut nodrop = ::std::mem::ManuallyDrop::new(ret);
  nodrop.as_mut_ptr()
}

unsafe fn usage(app_name: &str) -> Option<&'static str> {
  // This isn't necessarily the fastest way to do this since it involves a
  // linear search over usage_array but sort would require another alloc and
  // this whole setup will hopefully change soon anyways.
  applets.iter().find(|x| x.name == app_name).map(|x| x.usage)
}
unsafe fn find_applet_by_name(name: &str) -> Option<usize> {
  // TODO: we could do a binary search here, but we should add a test that
  // applets is correctly sorted first.
  applets.iter().position(|a| a.name == name)
}
fn install_loc_to_string(install_loc: InstallLoc) -> String {
  String::from(match install_loc {
    InstallLoc::DIR_USR_SBIN => "/usr/sbin/",
    InstallLoc::DIR_USR_BIN => "/usr/bin/",
    InstallLoc::DIR_SBIN => "/sbin/",
    InstallLoc::DIR_BIN => "/bin/",
    InstallLoc::DIR_ROOT => "/",
  })
}

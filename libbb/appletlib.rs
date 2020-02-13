use crate::applets::applet_tables::applets;
use crate::applets::applet_tables::Entrypoint;
use crate::applets::applet_tables::InstallLoc;
use crate::libbb::ptr_to_globals::bb_errno;
use libc;
use std::ffi::CStr;
use std::ffi::CString;
use std::path::Path;

extern "C" {
  #[no_mangle]
  static mut xfunc_error_retval: u8;
}

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

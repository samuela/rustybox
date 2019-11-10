use libc;






use libc::getenv;












use libc::strcpy;










use libc::puts;



use libc::strchr;







use libc::free;
extern "C" {


  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static bb_PATH_root_path: [libc::c_char; 0];



  #[no_mangle]
  fn file_is_executable(name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn find_executable(
    filename: *const libc::c_char,
    PATHp: *mut *mut libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
}



/*
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 * Copyright (C) 2006 Gabriel Somlo <somlo at cmu.edu>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config WHICH
//config:	bool "which (3.8 kb)"
//config:	default y
//config:	help
//config:	which is used to find programs in your PATH and
//config:	print out their pathnames.
//applet:IF_WHICH(APPLET_NOFORK(which, which, BB_DIR_USR_BIN, BB_SUID_DROP, which))
//kbuild:lib-$(CONFIG_WHICH) += which.o
//usage:#define which_trivial_usage
//usage:       "[COMMAND]..."
//usage:#define which_full_usage "\n\n"
//usage:       "Locate a COMMAND"
//usage:
//usage:#define which_example_usage
//usage:       "$ which login\n"
//usage:       "/bin/login\n"
#[no_mangle]
pub unsafe extern "C" fn which_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut env_path: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut status: libc::c_int = 0i32;
  /* This sizeof(): bb_default_root_path is shorter than BB_PATH_ROOT_PATH */
  let mut buf: [libc::c_char; 35] = [0; 35];
  env_path = getenv(b"PATH\x00" as *const u8 as *const libc::c_char);
  if env_path.is_null() {
    /* env_path must be writable, and must not alloc, so... */
    env_path = strcpy(
      buf.as_mut_ptr(),
      bb_PATH_root_path
        .as_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as isize),
    )
  }
  getopt32(argv, b"^a\x00-1\x00" as *const u8 as *const libc::c_char);
  argv = argv.offset(optind as isize);
  loop {
    let mut missing: libc::c_int = 1i32;
    /* If file contains a slash don't use PATH */
    if !strchr(*argv, '/' as i32).is_null() {
      if file_is_executable(*argv) != 0 {
        missing = 0i32;
        puts(*argv);
      }
    } else {
      let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
      let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
      path = env_path;
      loop
      /* NOFORK NB: xmalloc inside find_executable(), must have no allocs above! */
      {
        p = find_executable(*argv, &mut path);
        if p.is_null() {
          break;
        }
        missing = 0i32;
        puts(p);
        free(p as *mut libc::c_void);
        if option_mask32 == 0 {
          break;
        }
      }
    }
    status |= missing;
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  return status;
}

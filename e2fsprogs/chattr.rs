
use crate::libbb::appletlib::applet_name;
use libc;
use libc::free;
use libc::lstat;
use libc::strchr;
extern "C" {

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn xatoull(str: *const libc::c_char) -> libc::c_ulonglong;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn concat_subpath_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;

  /*
   * See README for additional information
   *
   * This file can be redistributed under the terms of the GNU Library General
   * Public License
   */
  /* Constants and structures */
  /* Iterate a function on each entry of a directory */
  #[no_mangle]
  fn iterate_on_dir(
    dir_name: *const libc::c_char,
    func: Option<
      unsafe extern "C" fn(
        _: *const libc::c_char,
        _: *mut dirent,
        _: *mut libc::c_void,
      ) -> libc::c_int,
    >,
    private: *mut libc::c_void,
  ) -> libc::c_int;
  /* Get/set a file version on an ext2 file system */
  #[no_mangle]
  fn fgetsetversion(
    name: *const libc::c_char,
    get_version: *mut libc::c_ulong,
    set_version: libc::c_ulong,
  ) -> libc::c_int;
  /* Get/set a file flags on an ext2 file system */
  #[no_mangle]
  fn fgetsetflags(
    name: *const libc::c_char,
    get_flags: *mut libc::c_ulong,
    set_flags: libc::c_ulong,
  ) -> libc::c_int;
  #[no_mangle]
  static e2attr_flags_value: [u32; 0];
  #[no_mangle]
  static e2attr_flags_sname: [libc::c_char; 0];
}

use crate::librb::smallint;
use libc::dirent;
use libc::stat;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub version: libc::c_ulong,
  pub af: libc::c_ulong,
  pub rf: libc::c_ulong,
  pub flags: libc::c_int,
  pub recursive: smallint,
}
#[inline(always)]
unsafe extern "C" fn xatoul(mut str: *const libc::c_char) -> libc::c_ulong {
  return xatoull(str) as libc::c_ulong; /* if (opt == '=') */
}
unsafe extern "C" fn get_flag(mut c: libc::c_char) -> libc::c_ulong {
  let mut fp: *const libc::c_char =
    strchr(&*e2attr_flags_sname.as_ptr().offset(1), c as libc::c_int);
  if !fp.is_null() {
    return *(&*e2attr_flags_value.as_ptr().offset(1) as *const u32).offset(
      fp.wrapping_offset_from(&*e2attr_flags_sname.as_ptr().offset(1) as *const libc::c_char)
        as libc::c_long as isize,
    ) as libc::c_ulong;
  }
  bb_show_usage();
}
unsafe extern "C" fn decode_arg(
  mut argv: *mut *mut libc::c_char,
  mut gp: *mut globals,
) -> *mut *mut libc::c_char {
  let mut fl: *mut libc::c_ulong = 0 as *mut libc::c_ulong;
  let mut arg: *const libc::c_char = *argv;
  let mut opt: libc::c_char = *arg;
  fl = &mut (*gp).af;
  if opt as libc::c_int == '-' as i32 {
    (*gp).flags |= 2i32;
    fl = &mut (*gp).rf
  } else if opt as libc::c_int == '+' as i32 {
    (*gp).flags |= 1i32
  } else {
    (*gp).flags |= 4i32
  }
  loop
  /*"suppress most error messages" (nop) */
  {
    arg = arg.offset(1);
    if !(*arg != 0) {
      break;
    }
    if opt as libc::c_int == '-' as i32 {
      //e2fsprogs-1.43.1 accepts:
      // "-RRR", "-RRRv VER" and even "-ARRRva VER" and "-vvv V1 V2 V3"
      // but not "-vVER".
      // IOW: options are parsed as part of "remove attrs" strings,
      // if "v" is seen, next argv[] is VER, even if more opts/attrs follow in this argv[]!
      if *arg as libc::c_int == 'R' as i32 {
        (*gp).recursive = 1i32 as smallint;
        continue;
      } else if *arg as libc::c_int == 'V' as i32 {
        /*"verbose and print program version" (nop for now) */
        continue;
      } else {
        if *arg as libc::c_int == 'f' as i32 {
          continue;
        }
        if *arg as libc::c_int == 'v' as i32 {
          argv = argv.offset(1);
          if (*argv).is_null() {
            bb_show_usage();
          }
          (*gp).version = xatoul(*argv);
          (*gp).flags |= 8i32;
          continue;
        }
      }
      //TODO: "-p PROJECT_NUM" ?
      /* not a known option, try as an attribute */
    }
    *fl |= get_flag(*arg)
  }
  return argv;
}
unsafe extern "C" fn chattr_dir_proc(
  mut dir_name: *const libc::c_char,
  mut de: *mut dirent,
  mut gp: *mut libc::c_void,
) -> libc::c_int {
  let mut path: *mut libc::c_char = concat_subpath_file(dir_name, (*de).d_name.as_mut_ptr());
  /* path is NULL if de->d_name is "." or "..", else... */
  if !path.is_null() {
    change_attributes(path, gp as *mut globals);
    free(path as *mut libc::c_void);
  }
  return 0i32;
}
unsafe extern "C" fn change_attributes(mut name: *const libc::c_char, mut gp: *mut globals) {
  let mut current_block: u64;
  let mut fsflags: libc::c_ulong = 0;
  let mut st: stat = std::mem::zeroed();
  if lstat(name, &mut st) != 0i32 {
    bb_perror_msg(b"stat %s\x00" as *const u8 as *const libc::c_char, name);
    return;
  }
  if st.st_mode & 0o170000i32 as libc::c_uint == 0o120000i32 as libc::c_uint
    && (*gp).recursive as libc::c_int != 0
  {
    return;
  }
  /* Don't try to open device files, fifos etc.  We probably
   * ought to display an error if the file was explicitly given
   * on the command line (whether or not recursive was
   * requested).  */
  if !(st.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint)
    && !(st.st_mode & 0o170000i32 as libc::c_uint == 0o120000i32 as libc::c_uint)
    && !(st.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint)
  {
    return;
  }
  if (*gp).flags & 8i32 != 0 {
    if fgetsetversion(name, 0 as *mut libc::c_ulong, (*gp).version) != 0i32 {
      bb_perror_msg(
        b"setting version on %s\x00" as *const u8 as *const libc::c_char,
        name,
      );
    }
  }
  if (*gp).flags & 4i32 != 0 {
    fsflags = (*gp).af;
    current_block = 12124785117276362961;
  } else if fgetsetflags(name, &mut fsflags, 0i32 as libc::c_ulong) != 0i32 {
    bb_perror_msg(
      b"reading flags on %s\x00" as *const u8 as *const libc::c_char,
      name,
    );
    current_block = 6233515183305828174;
  } else {
    /*if (gp->flags & OPT_REM) - not needed, rf is zero otherwise */
    fsflags &= !(*gp).rf;
    /*if (gp->flags & OPT_ADD) - not needed, af is zero otherwise */
    fsflags |= (*gp).af;
    // What is this? And why it's not done for SET case?
    if !(st.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint) {
      fsflags &= !0x10000i32 as libc::c_ulong
    }
    current_block = 12124785117276362961;
  }
  match current_block {
    12124785117276362961 => {
      if fgetsetflags(name, 0 as *mut libc::c_ulong, fsflags) != 0i32 {
        bb_perror_msg(
          b"setting flags on %s\x00" as *const u8 as *const libc::c_char,
          name,
        );
      }
    }
    _ => {}
  }
  if (*gp).recursive as libc::c_int != 0
    && st.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint
  {
    iterate_on_dir(
      name,
      Some(
        chattr_dir_proc
          as unsafe extern "C" fn(
            _: *const libc::c_char,
            _: *mut dirent,
            _: *mut libc::c_void,
          ) -> libc::c_int,
      ),
      gp as *mut libc::c_void,
    );
  };
}
#[no_mangle]
pub unsafe extern "C" fn chattr_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut g: globals = globals {
    version: 0,
    af: 0,
    rf: 0,
    flags: 0,
    recursive: 0,
  };
  memset(
    &mut g as *mut globals as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<globals>() as libc::c_ulong,
  );
  loop
  /* parse the args */
  {
    argv = argv.offset(1);
    let mut arg: *mut libc::c_char = *argv;
    if arg.is_null() {
      bb_show_usage();
    }
    if *arg.offset(0) as libc::c_int != '-' as i32
      && *arg.offset(0) as libc::c_int != '+' as i32
      && *arg.offset(0) as libc::c_int != '=' as i32
    {
      break;
    }
    argv = decode_arg(argv, &mut g)
  }
  /* note: on loop exit, remaining argv[] is never empty */
  /* run sanity checks on all the arguments given us */
  if g.flags & 4i32 != 0 && g.flags & (1i32 | 2i32) != 0 {
    bb_simple_error_msg_and_die(
      b"= is incompatible with - and +\x00" as *const u8 as *const libc::c_char,
    );
  }
  if g.rf & g.af != 0 {
    bb_simple_error_msg_and_die(
      b"can\'t set and unset a flag\x00" as *const u8 as *const libc::c_char,
    );
  }
  if g.flags == 0 {
    bb_simple_error_msg_and_die(
      b"must use \'-v\', =, - or +\x00" as *const u8 as *const libc::c_char,
    );
  }
  loop
  /* now run chattr on all the files passed to us */
  {
    change_attributes(*argv, &mut g);
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  return 0i32;
}

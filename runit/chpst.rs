use crate::librb::size_t;
use crate::libbb::ptr_to_globals::bb_errno;
use libc;
use libc::close;
use libc::closedir;
use libc::gid_t;
use libc::open;
use libc::readdir;
use libc::setsid;
use libc::ssize_t;
use libc::strchr;
use libc::uid_t;
extern "C" {

  #[no_mangle]
  fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn nice(__inc: libc::c_int) -> libc::c_int;

  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;

  #[no_mangle]
  fn getrlimit(__resource: __rlimit_resource_t, __rlimits: *mut rlimit) -> libc::c_int;
  #[no_mangle]
  fn setrlimit(__resource: __rlimit_resource_t, __rlimits: *const rlimit) -> libc::c_int;
  #[no_mangle]
  fn setgroups(__n: size_t, __groups: *const gid_t) -> libc::c_int;

  #[no_mangle]
  fn xopendir(path: *const libc::c_char) -> *mut DIR;
  #[no_mangle]
  fn xsetgid(gid: gid_t);
  #[no_mangle]
  fn xsetuid(uid: uid_t);
  #[no_mangle]
  fn xchdir(path: *const libc::c_char);
  #[no_mangle]
  fn xfchdir(fd: libc::c_int);
  #[no_mangle]
  fn xchroot(path: *const libc::c_char);
  #[no_mangle]
  fn xsetenv(key: *const libc::c_char, value: *const libc::c_char);
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn full_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn utoa(n: libc::c_uint) -> *mut libc::c_char;
  #[no_mangle]
  fn xatoi(str: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xget_uidgid(_: *mut bb_uidgid_t, _: *const libc::c_char);
  #[no_mangle]
  fn BB_EXECVP_or_die(argv: *mut *mut libc::c_char) -> !;
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
}

pub type __rlim64_t = libc::c_ulong;

use libc::dirent;
use libc::DIR;
pub type __rlimit_resource = libc::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
pub type rlim_t = __rlim64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
  pub rlim_cur: rlim_t,
  pub rlim_max: rlim_t,
}
pub type __rlimit_resource_t = __rlimit_resource;
use crate::librb::bb_uidgid_t;
/*
Copyright (c) 2001-2006, Gerrit Pape
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

   1. Redistributions of source code must retain the above copyright notice,
      this list of conditions and the following disclaimer.
   2. Redistributions in binary form must reproduce the above copyright
      notice, this list of conditions and the following disclaimer in the
      documentation and/or other materials provided with the distribution.
   3. The name of the author may not be used to endorse or promote products
      derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE AUTHOR ''AS IS'' AND ANY EXPRESS OR IMPLIED
WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO
EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS;
OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF
ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
/* Busyboxed by Denys Vlasenko <vda.linux@googlemail.com> */
//config:config CHPST
//config:	bool "chpst (9 kb)"
//config:	default y
//config:	help
//config:	chpst changes the process state according to the given options, and
//config:	execs specified program.
//config:
//config:config SETUIDGID
//config:	bool "setuidgid (4 kb)"
//config:	default y
//config:	help
//config:	Sets soft resource limits as specified by options
//config:
//config:config ENVUIDGID
//config:	bool "envuidgid (3.9 kb)"
//config:	default y
//config:	help
//config:	Sets $UID to account's uid and $GID to account's gid
//config:
//config:config ENVDIR
//config:	bool "envdir (2.5 kb)"
//config:	default y
//config:	help
//config:	Sets various environment variables as specified by files
//config:	in the given directory
//config:
//config:config SOFTLIMIT
//config:	bool "softlimit (4.5 kb)"
//config:	default y
//config:	help
//config:	Sets soft resource limits as specified by options
//applet:IF_CHPST(    APPLET_NOEXEC(chpst,     chpst, BB_DIR_USR_BIN, BB_SUID_DROP, chpst))
//                    APPLET_NOEXEC:name       main   location        suid_type     help
//applet:IF_ENVDIR(   APPLET_NOEXEC(envdir,    chpst, BB_DIR_USR_BIN, BB_SUID_DROP, envdir))
//applet:IF_ENVUIDGID(APPLET_NOEXEC(envuidgid, chpst, BB_DIR_USR_BIN, BB_SUID_DROP, envuidgid))
//applet:IF_SETUIDGID(APPLET_NOEXEC(setuidgid, chpst, BB_DIR_USR_BIN, BB_SUID_DROP, setuidgid))
//applet:IF_SOFTLIMIT(APPLET_NOEXEC(softlimit, chpst, BB_DIR_USR_BIN, BB_SUID_DROP, softlimit))
//kbuild:lib-$(CONFIG_CHPST) += chpst.o
//kbuild:lib-$(CONFIG_ENVDIR) += chpst.o
//kbuild:lib-$(CONFIG_ENVUIDGID) += chpst.o
//kbuild:lib-$(CONFIG_SETUIDGID) += chpst.o
//kbuild:lib-$(CONFIG_SOFTLIMIT) += chpst.o
//usage:#define chpst_trivial_usage
//usage:       "[-vP012] [-u USER[:GRP]] [-U USER[:GRP]] [-e DIR]\n"
//usage:       "	[-/ DIR] [-n NICE] [-m BYTES] [-d BYTES] [-o N]\n"
//usage:       "	[-p N] [-f BYTES] [-c BYTES] PROG ARGS"
//usage:#define chpst_full_usage "\n\n"
//usage:       "Change the process state, run PROG\n"
//usage:     "\n	-u USER[:GRP]	Set uid and gid"
//usage:     "\n	-U USER[:GRP]	Set $UID and $GID in environment"
//usage:     "\n	-e DIR		Set environment variables as specified by files"
//usage:     "\n			in DIR: file=1st_line_of_file"
//usage:     "\n	-/ DIR		Chroot to DIR"
//usage:     "\n	-n NICE		Add NICE to nice value"
//usage:     "\n	-m BYTES	Same as -d BYTES -s BYTES -l BYTES"
//usage:     "\n	-d BYTES	Limit data segment"
//usage:     "\n	-o N		Limit number of open files per process"
//usage:     "\n	-p N		Limit number of processes per uid"
//usage:     "\n	-f BYTES	Limit output file sizes"
//usage:     "\n	-c BYTES	Limit core file size"
//usage:     "\n	-v		Verbose"
//usage:     "\n	-P		Create new process group"
//usage:     "\n	-0		Close stdin"
//usage:     "\n	-1		Close stdout"
//usage:     "\n	-2		Close stderr"
//usage:
//usage:#define envdir_trivial_usage
//usage:       "DIR PROG ARGS"
//usage:#define envdir_full_usage "\n\n"
//usage:       "Set various environment variables as specified by files\n"
//usage:       "in the directory DIR, run PROG"
//usage:
//usage:#define envuidgid_trivial_usage
//usage:       "USER PROG ARGS"
//usage:#define envuidgid_full_usage "\n\n"
//usage:       "Set $UID to USER's uid and $GID to USER's gid, run PROG"
//usage:
//usage:#define setuidgid_trivial_usage
//usage:       "USER PROG ARGS"
//usage:#define setuidgid_full_usage "\n\n"
//usage:       "Set uid and gid to USER's uid and gid, drop supplementary group ids,\n"
//usage:       "run PROG"
//usage:
//usage:#define softlimit_trivial_usage
//usage:       "[-a BYTES] [-m BYTES] [-d BYTES] [-s BYTES] [-l BYTES]\n"
//usage:       "	[-f BYTES] [-c BYTES] [-r BYTES] [-o N] [-p N] [-t N]\n"
//usage:       "	PROG ARGS"
//usage:#define softlimit_full_usage "\n\n"
//usage:       "Set soft resource limits, then run PROG\n"
//usage:     "\n	-a BYTES	Limit total size of all segments"
//usage:     "\n	-m BYTES	Same as -d BYTES -s BYTES -l BYTES -a BYTES"
//usage:     "\n	-d BYTES	Limit data segment"
//usage:     "\n	-s BYTES	Limit stack segment"
//usage:     "\n	-l BYTES	Limit locked memory size"
//usage:     "\n	-o N		Limit number of open files per process"
//usage:     "\n	-p N		Limit number of processes per uid"
//usage:     "\nOptions controlling file sizes:"
//usage:     "\n	-f BYTES	Limit output file sizes"
//usage:     "\n	-c BYTES	Limit core file size"
//usage:     "\nEfficiency opts:"
//usage:     "\n	-r BYTES	Limit resident set size"
//usage:     "\n	-t N		Limit CPU time, process receives"
//usage:     "\n			a SIGXCPU after N seconds"
/*
Five applets here: chpst, envdir, envuidgid, setuidgid, softlimit.

Only softlimit and chpst are taking options:

# common
-o N            Limit number of open files per process
-p N            Limit number of processes per uid
-m BYTES        Same as -d BYTES -s BYTES -l BYTES [-a BYTES]
-d BYTES        Limit data segment
-f BYTES        Limit output file sizes
-c BYTES        Limit core file size
# softlimit
-a BYTES        Limit total size of all segments
-s BYTES        Limit stack segment
-l BYTES        Limit locked memory size
-r BYTES        Limit resident set size
-t N            Limit CPU time
# chpst
-u USER[:GRP]   Set uid and gid
-U USER[:GRP]   Set $UID and $GID in environment
-e DIR          Set environment variables as specified by files in DIR
-/ DIR          Chroot to DIR
-n NICE         Add NICE to nice value
-v              Verbose
-P              Create new process group
-0 -1 -2        Close fd 0,1,2

Even though we accept all these options for both softlimit and chpst,
they are not to be advertised on their help texts.
We have enough problems with feature creep in other people's
software, don't want to add our own.

envdir, envuidgid, setuidgid take no options, but they reuse code which
handles -e, -U and -u.
*/
pub type C2RustUnnamed = libc::c_uint;
pub const OPT_2: C2RustUnnamed = 1048576;
pub const OPT_1: C2RustUnnamed = 524288;
pub const OPT_0: C2RustUnnamed = 262144;
pub const OPT_P: C2RustUnnamed = 131072;
pub const OPT_v: C2RustUnnamed = 65536;
pub const OPT_n: C2RustUnnamed = 32768;
pub const OPT_root: C2RustUnnamed = 16384;
pub const OPT_e: C2RustUnnamed = 8192;
pub const OPT_U: C2RustUnnamed = 4096;
pub const OPT_u: C2RustUnnamed = 2048;
pub const OPT_t: C2RustUnnamed = 1024;
pub const OPT_s: C2RustUnnamed = 512;
pub const OPT_r: C2RustUnnamed = 256;
pub const OPT_p: C2RustUnnamed = 128;
pub const OPT_o: C2RustUnnamed = 64;
pub const OPT_m: C2RustUnnamed = 32;
pub const OPT_l: C2RustUnnamed = 16;
pub const OPT_f: C2RustUnnamed = 8;
pub const OPT_d: C2RustUnnamed = 4;
pub const OPT_c: C2RustUnnamed = 2;
pub const OPT_a: C2RustUnnamed = 1;
/* TODO: use recursive_action? */
#[inline(never)]
unsafe extern "C" fn edir(mut directory_name: *const libc::c_char) {
  let mut wdir: libc::c_int = 0;
  let mut dir: *mut DIR = 0 as *mut DIR;
  let mut d: *mut dirent = 0 as *mut dirent;
  let mut fd: libc::c_int = 0;
  wdir = xopen(
    b".\x00" as *const u8 as *const libc::c_char,
    0i32 | 0o4000i32,
  );
  xchdir(directory_name);
  dir = xopendir(b".\x00" as *const u8 as *const libc::c_char);
  loop {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut tail: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: libc::c_int = 0;
    *bb_errno = 0i32;
    d = readdir(dir);
    if d.is_null() {
      if *bb_errno != 0 {
        bb_perror_msg_and_die(
          b"readdir %s\x00" as *const u8 as *const libc::c_char,
          directory_name,
        );
      }
      break;
    } else {
      if (*d).d_name[0] as libc::c_int == '.' as i32 {
        continue;
      }
      fd = open((*d).d_name.as_mut_ptr(), 0i32 | 0o4000i32);
      if fd < 0i32 {
        if *bb_errno == 21i32 && !directory_name.is_null() {
          if option_mask32 & OPT_v as libc::c_int as libc::c_uint != 0 {
            bb_perror_msg(
              b"warning: %s/%s is a directory\x00" as *const u8 as *const libc::c_char,
              directory_name,
              (*d).d_name.as_mut_ptr(),
            );
          }
        } else {
          bb_perror_msg_and_die(
            b"open %s/%s\x00" as *const u8 as *const libc::c_char,
            directory_name,
            (*d).d_name.as_mut_ptr(),
          );
        }
      } else {
        size = full_read(
          fd,
          buf.as_mut_ptr() as *mut libc::c_void,
          (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong),
        ) as libc::c_int;
        close(fd);
        if size < 0i32 {
          bb_perror_msg_and_die(
            b"read %s/%s\x00" as *const u8 as *const libc::c_char,
            directory_name,
            (*d).d_name.as_mut_ptr(),
          );
        }
        if size == 0i32 {
          unsetenv((*d).d_name.as_mut_ptr());
        } else {
          buf[size as usize] = '\n' as i32 as libc::c_char;
          tail = strchr(buf.as_mut_ptr(), '\n' as i32);
          loop
          /* skip trailing whitespace */
          {
            *tail = '\u{0}' as i32 as libc::c_char;
            tail = tail.offset(-1);
            if tail < buf.as_mut_ptr()
              || ({
                let mut bb__isspace: libc::c_uchar = (*tail as libc::c_int - 9i32) as libc::c_uchar;
                (bb__isspace as libc::c_int == ' ' as i32 - 9i32
                  || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
              }) == 0
            {
              break;
            }
          }
          xsetenv((*d).d_name.as_mut_ptr(), buf.as_mut_ptr());
        }
      }
    }
  }
  closedir(dir);
  xfchdir(wdir);
  close(wdir);
}
unsafe extern "C" fn limit(mut what: libc::c_int, mut l: libc::c_long) {
  let mut r: rlimit = rlimit {
    rlim_cur: 0,
    rlim_max: 0,
  };
  /* Never fails under Linux (except if you pass it bad arguments) */
  getrlimit(what as __rlimit_resource_t, &mut r); /* for compiler */
  if l < 0 || l as libc::c_ulong > r.rlim_max {
    r.rlim_cur = r.rlim_max
  } else {
    r.rlim_cur = l as rlim_t
  }
  if setrlimit(what as __rlimit_resource_t, &mut r) == -1i32 {
    bb_simple_perror_msg_and_die(b"setrlimit\x00" as *const u8 as *const libc::c_char);
  };
}
#[no_mangle]
pub unsafe extern "C" fn chpst_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut ugid: bb_uidgid_t = bb_uidgid_t { uid: 0, gid: 0 };
  let mut set_user: *mut libc::c_char = 0 as *mut libc::c_char;
  set_user = set_user;
  let mut env_dir: *mut libc::c_char = 0 as *mut libc::c_char;
  env_dir = env_dir;
  let mut root: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut nicestr: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut limita: libc::c_uint = 0;
  let mut limitc: libc::c_uint = 0;
  let mut limitd: libc::c_uint = 0;
  let mut limitf: libc::c_uint = 0;
  let mut limitl: libc::c_uint = 0;
  let mut limitm: libc::c_uint = 0;
  let mut limito: libc::c_uint = 0;
  let mut limitp: libc::c_uint = 0;
  let mut limitr: libc::c_uint = 0;
  let mut limits: libc::c_uint = 0;
  let mut limitt: libc::c_uint = 0;
  let mut opt: libc::c_uint = 0;
  if 1i32 != 0 && *applet_name.offset(0) as libc::c_int == 'c' as i32
    || 1i32 != 0 && *applet_name.offset(1) as libc::c_int == 'o' as i32
  {
    // FIXME: can we live with int-sized limits?
    // can we live with 40000 days?
    // if yes -> getopt converts strings to numbers for us
    opt = getopt32(
      argv,
      b"^+a:+c:+d:+f:+l:+m:+o:+p:+r:+s:+t:+u:U:e:/:n:vP012\x00-1\x00" as *const u8
        as *const libc::c_char,
      &mut limita as *mut libc::c_uint,
      &mut limitc as *mut libc::c_uint,
      &mut limitd as *mut libc::c_uint,
      &mut limitf as *mut libc::c_uint,
      &mut limitl as *mut libc::c_uint,
      &mut limitm as *mut libc::c_uint,
      &mut limito as *mut libc::c_uint,
      &mut limitp as *mut libc::c_uint,
      &mut limitr as *mut libc::c_uint,
      &mut limits as *mut libc::c_uint,
      &mut limitt as *mut libc::c_uint,
      &mut set_user as *mut *mut libc::c_char,
      &mut set_user as *mut *mut libc::c_char,
      &mut env_dir as *mut *mut libc::c_char,
      &mut root as *mut *mut libc::c_char,
      &mut nicestr as *mut *mut libc::c_char,
    );
    argv = argv.offset(optind as isize);
    if opt & OPT_m as libc::c_int as libc::c_uint != 0 {
      // -m means -asld
      limitd = limitm;
      limitl = limitd;
      limits = limitl;
      limita = limits;
      opt |=
        (OPT_s as libc::c_int | OPT_l as libc::c_int | OPT_a as libc::c_int | OPT_d as libc::c_int)
          as libc::c_uint
    }
  } else {
    opt = 0i32 as libc::c_uint;
    option_mask32 = opt;
    argv = argv.offset(1);
    if (*argv).is_null() {
      bb_show_usage();
    }
  }
  // envdir?
  if 1i32 != 0 && *applet_name.offset(3) as libc::c_int == 'd' as i32 {
    let fresh0 = argv;
    argv = argv.offset(1);
    env_dir = *fresh0;
    opt |= OPT_e as libc::c_int as libc::c_uint
  }
  // setuidgid?
  if 1i32 != 0 && *applet_name.offset(1) as libc::c_int == 'e' as i32 {
    let fresh1 = argv;
    argv = argv.offset(1);
    set_user = *fresh1;
    opt |= OPT_u as libc::c_int as libc::c_uint
  }
  // envuidgid?
  if 1i32 != 0
    && *applet_name.offset(0) as libc::c_int == 'e' as i32
    && *applet_name.offset(3) as libc::c_int == 'u' as i32
  {
    let fresh2 = argv;
    argv = argv.offset(1);
    set_user = *fresh2;
    opt |= OPT_U as libc::c_int as libc::c_uint
  }
  // we must have PROG [ARGS]
  if (*argv).is_null() {
    bb_show_usage();
  }
  // set limits
  if opt & OPT_d as libc::c_int as libc::c_uint != 0 {
    limit(RLIMIT_DATA as libc::c_int, limitd as libc::c_long);
  }
  if opt & OPT_s as libc::c_int as libc::c_uint != 0 {
    limit(RLIMIT_STACK as libc::c_int, limits as libc::c_long);
  }
  if opt & OPT_l as libc::c_int as libc::c_uint != 0 {
    limit(__RLIMIT_MEMLOCK as libc::c_int, limitl as libc::c_long);
  }
  if opt & OPT_a as libc::c_int as libc::c_uint != 0 {
    limit(RLIMIT_AS as libc::c_int, limita as libc::c_long);
  }
  if opt & OPT_o as libc::c_int as libc::c_uint != 0 {
    limit(RLIMIT_NOFILE as libc::c_int, limito as libc::c_long);
  }
  if opt & OPT_p as libc::c_int as libc::c_uint != 0 {
    limit(__RLIMIT_NPROC as libc::c_int, limitp as libc::c_long);
  }
  if opt & OPT_f as libc::c_int as libc::c_uint != 0 {
    limit(RLIMIT_FSIZE as libc::c_int, limitf as libc::c_long);
  }
  if opt & OPT_c as libc::c_int as libc::c_uint != 0 {
    limit(RLIMIT_CORE as libc::c_int, limitc as libc::c_long);
  }
  if opt & OPT_r as libc::c_int as libc::c_uint != 0 {
    limit(__RLIMIT_RSS as libc::c_int, limitr as libc::c_long);
  }
  if opt & OPT_t as libc::c_int as libc::c_uint != 0 {
    limit(RLIMIT_CPU as libc::c_int, limitt as libc::c_long);
  }
  if opt & OPT_P as libc::c_int as libc::c_uint != 0 {
    setsid();
  }
  if opt & OPT_e as libc::c_int as libc::c_uint != 0 {
    edir(env_dir);
  }
  if opt & (OPT_u as libc::c_int | OPT_U as libc::c_int) as libc::c_uint != 0 {
    xget_uidgid(&mut ugid, set_user);
  }
  // chrooted jail must have /etc/passwd if we move this after chroot.
  // OTOH chroot fails for non-roots.
  // Solution: cache uid/gid before chroot, apply uid/gid after.
  if opt & OPT_U as libc::c_int as libc::c_uint != 0 {
    xsetenv(
      b"GID\x00" as *const u8 as *const libc::c_char,
      utoa(ugid.gid),
    );
    xsetenv(
      b"UID\x00" as *const u8 as *const libc::c_char,
      utoa(ugid.uid),
    );
  }
  if opt & OPT_root as libc::c_int as libc::c_uint != 0 {
    xchroot(root);
  }
  /* nice should be done before xsetuid */
  if opt & OPT_n as libc::c_int as libc::c_uint != 0 {
    *bb_errno = 0i32;
    if nice(xatoi(nicestr)) == -1i32 {
      bb_simple_perror_msg_and_die(b"nice\x00" as *const u8 as *const libc::c_char);
    }
  }
  if opt & OPT_u as libc::c_int as libc::c_uint != 0 {
    if setgroups(1i32 as size_t, &mut ugid.gid) == -1i32 {
      bb_simple_perror_msg_and_die(b"setgroups\x00" as *const u8 as *const libc::c_char);
    }
    xsetgid(ugid.gid);
    xsetuid(ugid.uid);
  }
  if opt & OPT_0 as libc::c_int as libc::c_uint != 0 {
    close(0i32);
  }
  if opt & OPT_1 as libc::c_int as libc::c_uint != 0 {
    close(1i32);
  }
  if opt & OPT_2 as libc::c_int as libc::c_uint != 0 {
    close(2i32);
  }
  BB_EXECVP_or_die(argv);
}

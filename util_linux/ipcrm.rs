use crate::librb::size_t;
use libc;
use libc::getopt;
use libc::gid_t;
use libc::pid_t;
use libc::puts;
use libc::time_t;
use libc::uid_t;
extern "C" {
  #[no_mangle]
  static mut optarg: *mut libc::c_char;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
  #[no_mangle]
  fn xstrtoull(str: *const libc::c_char, b: libc::c_int) -> libc::c_ulonglong;
  #[no_mangle]
  fn xatoull(str: *const libc::c_char) -> libc::c_ulonglong;
  /* Non-aborting kind of convertors: bb_strto[u][l]l */
  /* On exit: errno = 0 only if there was non-empty, '\0' terminated value
   * errno = EINVAL if value was not '\0' terminated, but otherwise ok
   *    Return value is still valid, caller should just check whether end[0]
   *    is a valid terminating char for particular case. OTOH, if caller
   *    requires '\0' terminated input, [s]he can just check errno == 0.
   * errno = ERANGE if value had alphanumeric terminating char ("1234abcg").
   * errno = ERANGE if value is out of range, missing, etc.
   * errno = ERANGE if value had minus sign for strtouXX (even "-0" is not ok )
   *    return value is all-ones in this case.
   */
  #[no_mangle]
  fn bb_strtoull(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_ulonglong;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn shmctl(__shmid: libc::c_int, __cmd: libc::c_int, __buf: *mut shmid_ds) -> libc::c_int;
  #[no_mangle]
  fn shmget(__key: key_t, __size: size_t, __shmflg: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn msgctl(__msqid: libc::c_int, __cmd: libc::c_int, __buf: *mut msqid_ds) -> libc::c_int;
  #[no_mangle]
  fn msgget(__key: key_t, __msgflg: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn semctl(__semid: libc::c_int, __semnum: libc::c_int, __cmd: libc::c_int, _: ...)
    -> libc::c_int;
  #[no_mangle]
  fn semget(__key: key_t, __nsems: libc::c_int, __semflg: libc::c_int) -> libc::c_int;
}

pub type __key_t = libc::c_int;
pub type __syscall_ulong_t = libc::c_ulong;
pub type key_t = __key_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_perm {
  pub __key: __key_t,
  pub uid: uid_t,
  pub gid: gid_t,
  pub cuid: uid_t,
  pub cgid: gid_t,
  pub mode: libc::c_ushort,
  pub __pad1: libc::c_ushort,
  pub __seq: libc::c_ushort,
  pub __pad2: libc::c_ushort,
  pub __glibc_reserved1: __syscall_ulong_t,
  pub __glibc_reserved2: __syscall_ulong_t,
}
pub type shmatt_t = __syscall_ulong_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shmid_ds {
  pub shm_perm: ipc_perm,
  pub shm_segsz: size_t,
  pub shm_atime: time_t,
  pub shm_dtime: time_t,
  pub shm_ctime: time_t,
  pub shm_cpid: pid_t,
  pub shm_lpid: pid_t,
  pub shm_nattch: shmatt_t,
  pub __glibc_reserved4: __syscall_ulong_t,
  pub __glibc_reserved5: __syscall_ulong_t,
}
pub type msgqnum_t = __syscall_ulong_t;
pub type msglen_t = __syscall_ulong_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msqid_ds {
  pub msg_perm: ipc_perm,
  pub msg_stime: time_t,
  pub msg_rtime: time_t,
  pub msg_ctime: time_t,
  pub __msg_cbytes: __syscall_ulong_t,
  pub msg_qnum: msgqnum_t,
  pub msg_qbytes: msglen_t,
  pub msg_lspid: pid_t,
  pub msg_lrpid: pid_t,
  pub __glibc_reserved4: __syscall_ulong_t,
  pub __glibc_reserved5: __syscall_ulong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct semid_ds {
  pub sem_perm: ipc_perm,
  pub sem_otime: time_t,
  pub __glibc_reserved1: __syscall_ulong_t,
  pub sem_ctime: time_t,
  pub __glibc_reserved2: __syscall_ulong_t,
  pub sem_nsems: __syscall_ulong_t,
  pub __glibc_reserved3: __syscall_ulong_t,
  pub __glibc_reserved4: __syscall_ulong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seminfo {
  pub semmap: libc::c_int,
  pub semmni: libc::c_int,
  pub semmns: libc::c_int,
  pub semmnu: libc::c_int,
  pub semmsl: libc::c_int,
  pub semopm: libc::c_int,
  pub semume: libc::c_int,
  pub semusz: libc::c_int,
  pub semvmx: libc::c_int,
  pub semaem: libc::c_int,
}

/*
 * ipcrm.c - utility to allow removal of IPC objects and data structures.
 *
 * 01 Sept 2004 - Rodney Radford <rradford@mindspring.com>
 * Adapted for busybox from util-linux-2.12a.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config IPCRM
//config:	bool "ipcrm (3.2 kb)"
//config:	default y
//config:	help
//config:	The ipcrm utility allows the removal of System V interprocess
//config:	communication (IPC) objects and the associated data structures
//config:	from the system.
//applet:IF_IPCRM(APPLET_NOEXEC(ipcrm, ipcrm, BB_DIR_USR_BIN, BB_SUID_DROP, ipcrm))
//kbuild:lib-$(CONFIG_IPCRM) += ipcrm.o
/* X/OPEN tells us to use <sys/{types,ipc,sem}.h> for semctl() */
/* X/OPEN tells us to use <sys/{types,ipc,msg}.h> for msgctl() */
/* according to X/OPEN we have to define it ourselves */
#[derive(Copy, Clone)]
#[repr(C)]
pub union semun {
  pub val: libc::c_int,
  pub buf: *mut semid_ds,
  pub array: *mut libc::c_ushort,
  pub __buf: *mut seminfo,
}
pub type type_id = libc::c_uint;
pub const MSG: type_id = 2;
pub const SEM: type_id = 1;
pub const SHM: type_id = 0;
#[inline(always)]
unsafe extern "C" fn xstrtoul(mut str: *const libc::c_char, mut b: libc::c_int) -> libc::c_ulong {
  return xstrtoull(str, b) as libc::c_ulong;
}
#[inline(always)]
unsafe extern "C" fn xatoul(mut str: *const libc::c_char) -> libc::c_ulong {
  return xatoull(str) as libc::c_ulong;
}
#[inline(always)]
unsafe extern "C" fn bb_strtoul(
  mut arg: *const libc::c_char,
  mut endp: *mut *mut libc::c_char,
  mut base: libc::c_int,
) -> libc::c_ulong {
  return bb_strtoull(arg, endp, base) as libc::c_ulong;
}
unsafe extern "C" fn remove_ids(
  mut type_0: type_id,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut id: libc::c_ulong = 0;
  let mut nb_errors: libc::c_int = 0i32;
  let mut arg: semun = semun { val: 0 };
  arg.val = 0i32;
  while !(*argv.offset(0)).is_null() {
    id = bb_strtoul(*argv.offset(0), 0 as *mut *mut libc::c_char, 10i32);
    if *bb_errno != 0 || id > 2147483647i32 as libc::c_ulong {
      bb_error_msg(
        b"invalid id: %s\x00" as *const u8 as *const libc::c_char,
        *argv.offset(0),
      );
      nb_errors += 1
    } else {
      let mut ret: libc::c_int = 0i32;
      if type_0 as libc::c_uint == SEM as libc::c_int as libc::c_uint {
        ret = semctl(id as libc::c_int, 0i32, 0i32, arg)
      } else if type_0 as libc::c_uint == MSG as libc::c_int as libc::c_uint {
        ret = msgctl(id as libc::c_int, 0i32, 0 as *mut msqid_ds)
      } else if type_0 as libc::c_uint == SHM as libc::c_int as libc::c_uint {
        ret = shmctl(id as libc::c_int, 0i32, 0 as *mut shmid_ds)
      }
      if ret != 0 {
        bb_perror_msg(
          b"can\'t remove id %s\x00" as *const u8 as *const libc::c_char,
          *argv.offset(0),
        );
        nb_errors += 1
      }
    }
    argv = argv.offset(1)
  }
  return nb_errors;
}
/* IPCRM_LEGACY */
//usage:#define ipcrm_trivial_usage
//usage:       "[-MQS key] [-mqs id]"
//usage:#define ipcrm_full_usage "\n\n"
//usage:       "Upper-case options MQS remove an object by shmkey value.\n"
//usage:       "Lower-case options remove an object by shmid value.\n"
//usage:     "\n	-mM	Remove memory segment after last detach"
//usage:     "\n	-qQ	Remove message queue"
//usage:     "\n	-sS	Remove semaphore"
#[no_mangle]
pub unsafe extern "C" fn ipcrm_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut c: libc::c_int = 0;
  let mut error: libc::c_int = 0i32;
  /* if the command is executed without parameters, do nothing */
  if argc == 1i32 {
    return 0i32;
  }
  /* check to see if the command is being invoked in the old way if so
  then run the old code. Valid commands are msg, shm, sem. */
  let mut what: type_id = SHM; /* silence gcc */
  let mut w: libc::c_char = 0;
  w = *(*argv.offset(1)).offset(0);
  if (w as libc::c_int == 'm' as i32
    && *(*argv.offset(1)).offset(1) as libc::c_int == 's' as i32
    && *(*argv.offset(1)).offset(2) as libc::c_int == 'g' as i32
    || *(*argv.offset(1)).offset(0) as libc::c_int == 's' as i32
      && {
        w = *(*argv.offset(1)).offset(1);
        (w as libc::c_int == 'h' as i32) || w as libc::c_int == 'e' as i32
      }
      && *(*argv.offset(1)).offset(2) as libc::c_int == 'm' as i32)
    && *(*argv.offset(1)).offset(3) as libc::c_int == '\u{0}' as i32
  {
    if argc < 3i32 {
      bb_show_usage();
    }
    if w as libc::c_int == 'h' as i32 {
      what = SHM
    } else if w as libc::c_int == 'm' as i32 {
      what = MSG
    } else if w as libc::c_int == 'e' as i32 {
      what = SEM
    }
    if remove_ids(what, &mut *argv.offset(2)) != 0 {
      fflush_stdout_and_exit(1i32);
    }
    puts(b"resource(s) deleted\x00" as *const u8 as *const libc::c_char);
    return 0i32;
  }
  loop
  /* IPCRM_LEGACY */
  /* process new syntax to conform with SYSV ipcrm */
  {
    c = getopt(
      argc,
      argv,
      b"q:m:s:Q:M:S:\x00" as *const u8 as *const libc::c_char,
    );
    if !(c != -1i32) {
      break;
    }
    let mut result: libc::c_int = 0;
    let mut id: libc::c_int = 0;
    let mut iskey: libc::c_int = 0;
    /* needed to delete semaphores */
    let mut arg: semun = semun { val: 0 };
    if c == '?' as i32 {
      /* option not in the string */
      bb_show_usage(); /* uppercase? */
    }
    id = 0i32;
    arg.val = 0i32;
    iskey = (c & 0x20i32 == 0) as libc::c_int;
    if iskey != 0 {
      /* keys are in hex or decimal */
      let mut key: key_t = xstrtoul(optarg, 0i32) as key_t; /* lowercase. c is 'q', 'm' or 's' now */
      if key == 0i32 {
        error += 1;
        bb_error_msg(
          b"illegal key (%s)\x00" as *const u8 as *const libc::c_char,
          optarg,
        );
        continue;
      } else {
        c |= 0x20i32;
        /* convert key to id */
        id = if c == 'q' as i32 {
          msgget(key, 0i32)
        } else if c == 'm' as i32 {
          shmget(key, 0i32 as size_t, 0i32)
        } else {
          semget(key, 0i32, 0i32)
        };
        if id < 0i32 {
          let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
          error += 1;
          match *bb_errno {
            13 => errmsg = b"permission denied for\x00" as *const u8 as *const libc::c_char,
            43 => errmsg = b"already removed\x00" as *const u8 as *const libc::c_char,
            2 => errmsg = b"invalid\x00" as *const u8 as *const libc::c_char,
            _ => errmsg = b"unknown error in\x00" as *const u8 as *const libc::c_char,
          }
          bb_error_msg(
            b"%s %s (%s)\x00" as *const u8 as *const libc::c_char,
            errmsg,
            b"key\x00" as *const u8 as *const libc::c_char,
            optarg,
          );
          continue;
        }
      }
    } else {
      /* ids are in decimal */
      id = xatoul(optarg) as libc::c_int
    }
    result = if c == 'q' as i32 {
      msgctl(id, 0i32, 0 as *mut msqid_ds)
    } else if c == 'm' as i32 {
      shmctl(id, 0i32, 0 as *mut shmid_ds)
    } else {
      semctl(id, 0i32, 0i32, arg)
    };
    if !(result != 0) {
      continue;
    }
    let mut errmsg_0: *const libc::c_char = 0 as *const libc::c_char;
    let what_0: *const libc::c_char = if iskey != 0 {
      b"key\x00" as *const u8 as *const libc::c_char
    } else {
      b"id\x00" as *const u8 as *const libc::c_char
    };
    error += 1;
    match *bb_errno {
      13 | 1 => errmsg_0 = b"permission denied for\x00" as *const u8 as *const libc::c_char,
      22 => errmsg_0 = b"invalid\x00" as *const u8 as *const libc::c_char,
      43 => errmsg_0 = b"already removed\x00" as *const u8 as *const libc::c_char,
      _ => errmsg_0 = b"unknown error in\x00" as *const u8 as *const libc::c_char,
    }
    bb_error_msg(
      b"%s %s (%s)\x00" as *const u8 as *const libc::c_char,
      errmsg_0,
      what_0,
      optarg,
    );
  }
  /* print usage if we still have some arguments left over */
  if optind != argc {
    bb_show_usage();
  }
  return error;
}
/* exit value reflects the number of errors encountered */

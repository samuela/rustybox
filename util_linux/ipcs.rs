use crate::libpwdgrp::pwd_grp::bb_internal_getgrgid;
use crate::librb::size_t;
use libc;
use libc::gid_t;
use libc::pid_t;
use libc::printf;
use libc::time_t;
use libc::uid_t;
extern "C" {
  #[no_mangle]
  fn semctl(__semid: libc::c_int, __semnum: libc::c_int, __cmd: libc::c_int, _: ...)
    -> libc::c_int;
  #[no_mangle]
  fn msgctl(__msqid: libc::c_int, __cmd: libc::c_int, __buf: *mut msqid_ds) -> libc::c_int;
  #[no_mangle]
  fn shmctl(__shmid: libc::c_int, __cmd: libc::c_int, __buf: *mut shmid_ds) -> libc::c_int;

  #[no_mangle]
  fn ctime(__timer: *const time_t) -> *mut libc::c_char;
/* Search for an entry with a matching user ID.  */

/* Search for an entry with a matching group ID.  */

/* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
 * But potentially slow, don't use in one-billion-times loops */

}

pub type __key_t = libc::c_int;
pub type __syscall_ulong_t = libc::c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
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
pub type msgqnum_t = __syscall_ulong_t;
pub type msglen_t = __syscall_ulong_t;

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msginfo {
  pub msgpool: libc::c_int,
  pub msgmap: libc::c_int,
  pub msgmax: libc::c_int,
  pub msgmnb: libc::c_int,
  pub msgmni: libc::c_int,
  pub msgssz: libc::c_int,
  pub msgtql: libc::c_int,
  pub msgseg: libc::c_ushort,
}
pub type shmatt_t = __syscall_ulong_t;

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shminfo {
  pub shmmax: __syscall_ulong_t,
  pub shmmin: __syscall_ulong_t,
  pub shmmni: __syscall_ulong_t,
  pub shmseg: __syscall_ulong_t,
  pub shmall: __syscall_ulong_t,
  pub __glibc_reserved1: __syscall_ulong_t,
  pub __glibc_reserved2: __syscall_ulong_t,
  pub __glibc_reserved3: __syscall_ulong_t,
  pub __glibc_reserved4: __syscall_ulong_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_info {
  pub used_ids: libc::c_int,
  pub shm_tot: __syscall_ulong_t,
  pub shm_rss: __syscall_ulong_t,
  pub shm_swp: __syscall_ulong_t,
  pub swap_attempts: __syscall_ulong_t,
  pub swap_successes: __syscall_ulong_t,
}
use libc::group;
use libc::passwd;
/*
 * ipcs.c -- provides information on allocated ipc resources.
 *
 * 01 Sept 2004 - Rodney Radford <rradford@mindspring.com>
 * Adapted for busybox from util-linux-2.12a.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config IPCS
//config:	bool "ipcs (11 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	The ipcs utility is used to provide information on the currently
//config:	allocated System V interprocess (IPC) objects in the system.
//applet:IF_IPCS(APPLET_NOEXEC(ipcs, ipcs, BB_DIR_USR_BIN, SUID_DROP, ipcs))
//kbuild:lib-$(CONFIG_IPCS) += ipcs.o
/* X/OPEN tells us to use <sys/{types,ipc,sem}.h> for semctl() */
/* X/OPEN tells us to use <sys/{types,ipc,msg}.h> for msgctl() */
/* X/OPEN tells us to use <sys/{types,ipc,shm}.h> for shmctl() */
/*-------------------------------------------------------------------*/
/* SHM_DEST and SHM_LOCKED are defined in kernel headers,
but inside #ifdef __KERNEL__ ... #endif */
/* For older kernels the same holds for the defines below */
/* Some versions of libc only define IPC_INFO when __USE_GNU is defined. */
/*-------------------------------------------------------------------*/
/* The last arg of semctl is a union semun, but where is it defined?
X/OPEN tells us to define it ourselves, but until recently
Linux include files would also define it. */
/* according to X/OPEN we have to define it ourselves */

#[repr(C)]
#[derive(Copy, Clone)]
pub union semun {
  pub val: libc::c_int,
  pub buf: *mut semid_ds,
  pub array: *mut libc::c_ushort,
  pub __buf: *mut seminfo,
}
unsafe fn print_perms(mut id: libc::c_int, mut ipcp: *mut ipc_perm) {
  let mut pw: *mut passwd = std::ptr::null_mut();
  let mut gr: *mut group = std::ptr::null_mut();
  printf(
    b"%-10d %-10o\x00" as *const u8 as *const libc::c_char,
    id,
    (*ipcp).mode as libc::c_int & 0o777i32,
  );
  pw = crate::libpwdgrp::pwd_grp::bb_internal_getpwuid((*ipcp).cuid);
  if !pw.is_null() {
    printf(
      b" %-10s\x00" as *const u8 as *const libc::c_char,
      (*pw).pw_name,
    );
  } else {
    printf(
      b" %-10d\x00" as *const u8 as *const libc::c_char,
      (*ipcp).cuid,
    );
  }
  gr = bb_internal_getgrgid((*ipcp).cgid);
  if !gr.is_null() {
    printf(
      b" %-10s\x00" as *const u8 as *const libc::c_char,
      (*gr).gr_name,
    );
  } else {
    printf(
      b" %-10d\x00" as *const u8 as *const libc::c_char,
      (*ipcp).cgid,
    );
  }
  pw = crate::libpwdgrp::pwd_grp::bb_internal_getpwuid((*ipcp).uid);
  if !pw.is_null() {
    printf(
      b" %-10s\x00" as *const u8 as *const libc::c_char,
      (*pw).pw_name,
    );
  } else {
    printf(
      b" %-10d\x00" as *const u8 as *const libc::c_char,
      (*ipcp).uid,
    );
  }
  gr = bb_internal_getgrgid((*ipcp).gid);
  if !gr.is_null() {
    printf(
      b" %-10s\n\x00" as *const u8 as *const libc::c_char,
      (*gr).gr_name,
    );
  } else {
    printf(
      b" %-10d\n\x00" as *const u8 as *const libc::c_char,
      (*ipcp).gid,
    );
  };
}
#[inline(never)]
unsafe fn do_shm(mut format: libc::c_int) {
  let mut maxid: libc::c_int = 0;
  let mut shmid: libc::c_int = 0;
  let mut id: libc::c_int = 0;
  let mut shmseg: shmid_ds = shmid_ds {
    shm_perm: ipc_perm {
      __key: 0,
      uid: 0,
      gid: 0,
      cuid: 0,
      cgid: 0,
      mode: 0,
      __pad1: 0,
      __seq: 0,
      __pad2: 0,
      __glibc_reserved1: 0,
      __glibc_reserved2: 0,
    },
    shm_segsz: 0,
    shm_atime: 0,
    shm_dtime: 0,
    shm_ctime: 0,
    shm_cpid: 0,
    shm_lpid: 0,
    shm_nattch: 0,
    __glibc_reserved4: 0,
    __glibc_reserved5: 0,
  };
  let mut shm_info: shm_info = shm_info {
    used_ids: 0,
    shm_tot: 0,
    shm_rss: 0,
    shm_swp: 0,
    swap_attempts: 0,
    swap_successes: 0,
  };
  let mut shminfo: shminfo = shminfo {
    shmmax: 0,
    shmmin: 0,
    shmmni: 0,
    shmseg: 0,
    shmall: 0,
    __glibc_reserved1: 0,
    __glibc_reserved2: 0,
    __glibc_reserved3: 0,
    __glibc_reserved4: 0,
  };
  let mut ipcp: *mut ipc_perm = &mut shmseg.shm_perm;
  let mut pw: *mut passwd = std::ptr::null_mut();
  maxid = shmctl(
    0,
    14i32,
    &mut shm_info as *mut shm_info as *mut libc::c_void as *mut shmid_ds,
  );
  if maxid < 0 {
    printf(
      b"kernel not configured for %s\n\x00" as *const u8 as *const libc::c_char,
      b"shared memory\x00" as *const u8 as *const libc::c_char,
    );
    return;
  }
  match format {
    1 => {
      printf(
        b"------ Shared Memory %s --------\n\x00" as *const u8 as *const libc::c_char,
        b"Limits\x00" as *const u8 as *const libc::c_char,
      );
      if shmctl(
        0,
        3i32,
        &mut shminfo as *mut shminfo as *mut libc::c_void as *mut shmid_ds,
      ) < 0
      {
        return;
      }
      /* glibc 2.1.3 and all earlier libc's have ints as fields
       * of struct shminfo; glibc 2.1.91 has unsigned long; ach */
      printf(b"max number of segments = %lu\nmax seg size (kbytes) = %lu\nmax total shared memory (pages) = %lu\nmin seg size (bytes) = %lu\n\x00"
                       as *const u8 as *const libc::c_char, shminfo.shmmni,
                   shminfo.shmmax >> 10i32, shminfo.shmall, shminfo.shmmin);
      return;
    }
    2 => {
      printf(
        b"------ Shared Memory %s --------\n\x00" as *const u8 as *const libc::c_char,
        b"Status\x00" as *const u8 as *const libc::c_char,
      );
      printf(b"segments allocated %d\npages allocated %lu\npages resident  %lu\npages swapped   %lu\nSwap performance: %lu attempts\t%lu successes\n\x00"
                       as *const u8 as *const libc::c_char, shm_info.used_ids,
                   shm_info.shm_tot, shm_info.shm_rss, shm_info.shm_swp,
                   shm_info.swap_attempts, shm_info.swap_successes);
      return;
    }
    3 => {
      printf(
        b"------ Shared Memory %s --------\n\x00" as *const u8 as *const libc::c_char,
        b"Segment Creators/Owners\x00" as *const u8 as *const libc::c_char,
      );
      printf(
        b"%-10s %-10s %-10s %-10s %-10s %-10s\n\x00" as *const u8 as *const libc::c_char,
        b"shmid\x00" as *const u8 as *const libc::c_char,
        b"perms\x00" as *const u8 as *const libc::c_char,
        b"cuid\x00" as *const u8 as *const libc::c_char,
        b"cgid\x00" as *const u8 as *const libc::c_char,
        b"uid\x00" as *const u8 as *const libc::c_char,
        b"gid\x00" as *const u8 as *const libc::c_char,
      );
    }
    4 => {
      printf(
        b"------ Shared Memory %s --------\n\x00" as *const u8 as *const libc::c_char,
        b"Attach/Detach/Change Times\x00" as *const u8 as *const libc::c_char,
      );
      printf(
        b"%-10s %-10s %-20s %-20s %-20s\n\x00" as *const u8 as *const libc::c_char,
        b"shmid\x00" as *const u8 as *const libc::c_char,
        b"owner\x00" as *const u8 as *const libc::c_char,
        b"attached\x00" as *const u8 as *const libc::c_char,
        b"detached\x00" as *const u8 as *const libc::c_char,
        b"changed\x00" as *const u8 as *const libc::c_char,
      );
    }
    5 => {
      printf(
        b"------ Shared Memory %s --------\n\x00" as *const u8 as *const libc::c_char,
        b"Creator/Last-op\x00" as *const u8 as *const libc::c_char,
      );
      printf(
        b"%-10s %-10s %-10s %-10s\n\x00" as *const u8 as *const libc::c_char,
        b"shmid\x00" as *const u8 as *const libc::c_char,
        b"owner\x00" as *const u8 as *const libc::c_char,
        b"cpid\x00" as *const u8 as *const libc::c_char,
        b"lpid\x00" as *const u8 as *const libc::c_char,
      );
    }
    _ => {
      printf(
        b"------ Shared Memory %s --------\n\x00" as *const u8 as *const libc::c_char,
        b"Segments\x00" as *const u8 as *const libc::c_char,
      );
      printf(
        b"%-10s %-10s %-10s %-10s %-10s %-10s %-12s\n\x00" as *const u8 as *const libc::c_char,
        b"key\x00" as *const u8 as *const libc::c_char,
        b"shmid\x00" as *const u8 as *const libc::c_char,
        b"owner\x00" as *const u8 as *const libc::c_char,
        b"perms\x00" as *const u8 as *const libc::c_char,
        b"bytes\x00" as *const u8 as *const libc::c_char,
        b"nattch\x00" as *const u8 as *const libc::c_char,
        b"status\x00" as *const u8 as *const libc::c_char,
      );
    }
  }
  id = 0;
  while id <= maxid {
    shmid = shmctl(id, 13i32, &mut shmseg);
    if !(shmid < 0) {
      if format == 3i32 {
        print_perms(shmid, ipcp);
      } else {
        pw = crate::libpwdgrp::pwd_grp::bb_internal_getpwuid((*ipcp).uid);
        match format {
          4 => {
            if !pw.is_null() {
              printf(
                b"%-10d %-10.10s\x00" as *const u8 as *const libc::c_char,
                shmid,
                (*pw).pw_name,
              );
            } else {
              printf(
                b"%-10d %-10d\x00" as *const u8 as *const libc::c_char,
                shmid,
                (*ipcp).uid,
              );
            }
            /* ctime uses static buffer: use separate calls */
            printf(
              b" %-20.16s\x00" as *const u8 as *const libc::c_char,
              if shmseg.shm_atime != 0 {
                ctime(&mut shmseg.shm_atime).offset(4)
              } else {
                b"Not set\x00" as *const u8 as *const libc::c_char
              },
            ); /* damn union */
            printf(
              b" %-20.16s\x00" as *const u8 as *const libc::c_char,
              if shmseg.shm_dtime != 0 {
                ctime(&mut shmseg.shm_dtime).offset(4)
              } else {
                b"Not set\x00" as *const u8 as *const libc::c_char
              },
            );
            printf(
              b" %-20.16s\n\x00" as *const u8 as *const libc::c_char,
              if shmseg.shm_ctime != 0 {
                ctime(&mut shmseg.shm_ctime).offset(4)
              } else {
                b"Not set\x00" as *const u8 as *const libc::c_char
              },
            );
          }
          5 => {
            if !pw.is_null() {
              printf(
                b"%-10d %-10.10s\x00" as *const u8 as *const libc::c_char,
                shmid,
                (*pw).pw_name,
              );
            } else {
              printf(
                b"%-10d %-10d\x00" as *const u8 as *const libc::c_char,
                shmid,
                (*ipcp).uid,
              );
            }
            printf(
              b" %-10d %-10d\n\x00" as *const u8 as *const libc::c_char,
              shmseg.shm_cpid,
              shmseg.shm_lpid,
            );
          }
          _ => {
            printf(
              b"0x%08x \x00" as *const u8 as *const libc::c_char,
              (*ipcp).__key,
            );
            if !pw.is_null() {
              printf(
                b"%-10d %-10.10s\x00" as *const u8 as *const libc::c_char,
                shmid,
                (*pw).pw_name,
              );
            } else {
              printf(
                b"%-10d %-10d\x00" as *const u8 as *const libc::c_char,
                shmid,
                (*ipcp).uid,
              );
            }
            printf(
              b" %-10o %-10lu %-10ld %-6s %-6s\n\x00" as *const u8 as *const libc::c_char,
              (*ipcp).mode as libc::c_int & 0o777i32,
              shmseg.shm_segsz,
              shmseg.shm_nattch as libc::c_long,
              if (*ipcp).mode as libc::c_int & 0o1000i32 != 0 {
                b"dest\x00" as *const u8 as *const libc::c_char
              } else {
                b" \x00" as *const u8 as *const libc::c_char
              },
              if (*ipcp).mode as libc::c_int & 0o2000i32 != 0 {
                b"locked\x00" as *const u8 as *const libc::c_char
              } else {
                b" \x00" as *const u8 as *const libc::c_char
              },
            );
          }
        }
      }
    }
    id += 1
  }
}
#[inline(never)]
unsafe fn do_sem(mut format: libc::c_int) {
  let mut maxid: libc::c_int = 0;
  let mut semid: libc::c_int = 0;
  let mut id: libc::c_int = 0;
  let mut semary: semid_ds = semid_ds {
    sem_perm: ipc_perm {
      __key: 0,
      uid: 0,
      gid: 0,
      cuid: 0,
      cgid: 0,
      mode: 0,
      __pad1: 0,
      __seq: 0,
      __pad2: 0,
      __glibc_reserved1: 0,
      __glibc_reserved2: 0,
    },
    sem_otime: 0,
    __glibc_reserved1: 0,
    sem_ctime: 0,
    __glibc_reserved2: 0,
    sem_nsems: 0,
    __glibc_reserved3: 0,
    __glibc_reserved4: 0,
  };
  let mut seminfo: seminfo = seminfo {
    semmap: 0,
    semmni: 0,
    semmns: 0,
    semmnu: 0,
    semmsl: 0,
    semopm: 0,
    semume: 0,
    semusz: 0,
    semvmx: 0,
    semaem: 0,
  };
  let mut ipcp: *mut ipc_perm = &mut semary.sem_perm;
  let mut pw: *mut passwd = std::ptr::null_mut();
  let mut arg: semun = semun { val: 0 };
  arg.array = &mut seminfo as *mut seminfo as *mut libc::c_void as *mut libc::c_ushort;
  maxid = semctl(0i32, 0, 19i32, arg);
  if maxid < 0 {
    printf(
      b"kernel not configured for %s\n\x00" as *const u8 as *const libc::c_char,
      b"semaphores\x00" as *const u8 as *const libc::c_char,
    );
    return;
  }
  match format {
    1 => {
      printf(
        b"------ Semaphore %s --------\n\x00" as *const u8 as *const libc::c_char,
        b"Limits\x00" as *const u8 as *const libc::c_char,
      );
      arg.array = &mut seminfo as *mut seminfo as *mut libc::c_void as *mut libc::c_ushort;
      if semctl(0i32, 0, 3i32, arg) < 0 {
        return;
      }
      printf(b"max number of arrays = %d\nmax semaphores per array = %d\nmax semaphores system wide = %d\nmax ops per semop call = %d\nsemaphore max value = %d\n\x00"
                       as *const u8 as *const libc::c_char, seminfo.semmni,
                   seminfo.semmsl, seminfo.semmns, seminfo.semopm,
                   seminfo.semvmx);
      return;
    }
    2 => {
      printf(
        b"------ Semaphore %s --------\n\x00" as *const u8 as *const libc::c_char,
        b"Status\x00" as *const u8 as *const libc::c_char,
      );
      printf(
        b"used arrays = %d\nallocated semaphores = %d\n\x00" as *const u8 as *const libc::c_char,
        seminfo.semusz,
        seminfo.semaem,
      );
      return;
    }
    3 => {
      printf(
        b"------ Semaphore %s --------\n\x00" as *const u8 as *const libc::c_char,
        b"Arrays Creators/Owners\x00" as *const u8 as *const libc::c_char,
      );
      printf(
        b"%-10s %-10s %-10s %-10s %-10s %-10s\n\x00" as *const u8 as *const libc::c_char,
        b"semid\x00" as *const u8 as *const libc::c_char,
        b"perms\x00" as *const u8 as *const libc::c_char,
        b"cuid\x00" as *const u8 as *const libc::c_char,
        b"cgid\x00" as *const u8 as *const libc::c_char,
        b"uid\x00" as *const u8 as *const libc::c_char,
        b"gid\x00" as *const u8 as *const libc::c_char,
      );
    }
    4 => {
      printf(
        b"------ Shared Memory %s --------\n\x00" as *const u8 as *const libc::c_char,
        b"Operation/Change Times\x00" as *const u8 as *const libc::c_char,
      );
      printf(
        b"%-8s %-10s %-26.24s %-26.24s\n\x00" as *const u8 as *const libc::c_char,
        b"shmid\x00" as *const u8 as *const libc::c_char,
        b"owner\x00" as *const u8 as *const libc::c_char,
        b"last-op\x00" as *const u8 as *const libc::c_char,
        b"last-changed\x00" as *const u8 as *const libc::c_char,
      );
    }
    5 => {}
    _ => {
      printf(
        b"------ Semaphore %s --------\n\x00" as *const u8 as *const libc::c_char,
        b"Arrays\x00" as *const u8 as *const libc::c_char,
      );
      printf(
        b"%-10s %-10s %-10s %-10s %-10s\n\x00" as *const u8 as *const libc::c_char,
        b"key\x00" as *const u8 as *const libc::c_char,
        b"semid\x00" as *const u8 as *const libc::c_char,
        b"owner\x00" as *const u8 as *const libc::c_char,
        b"perms\x00" as *const u8 as *const libc::c_char,
        b"nsems\x00" as *const u8 as *const libc::c_char,
      );
    }
  }
  id = 0;
  while id <= maxid {
    arg.buf = &mut semary as *mut semid_ds;
    semid = semctl(id, 0, 18i32, arg);
    if !(semid < 0) {
      if format == 3i32 {
        print_perms(semid, ipcp);
      } else {
        pw = crate::libpwdgrp::pwd_grp::bb_internal_getpwuid((*ipcp).uid);
        match format {
          4 => {
            if !pw.is_null() {
              printf(
                b"%-8d %-10.10s\x00" as *const u8 as *const libc::c_char,
                semid,
                (*pw).pw_name,
              );
            } else {
              printf(
                b"%-8d %-10d\x00" as *const u8 as *const libc::c_char,
                semid,
                (*ipcp).uid,
              );
            }
            /* ctime uses static buffer: use separate calls */
            printf(
              b"  %-26.24s\x00" as *const u8 as *const libc::c_char,
              if semary.sem_otime != 0 {
                ctime(&mut semary.sem_otime)
              } else {
                b"Not set\x00" as *const u8 as *const libc::c_char
              },
            );
            printf(
              b" %-26.24s\n\x00" as *const u8 as *const libc::c_char,
              if semary.sem_ctime != 0 {
                ctime(&mut semary.sem_ctime)
              } else {
                b"Not set\x00" as *const u8 as *const libc::c_char
              },
            );
          }
          5 => {}
          _ => {
            printf(
              b"0x%08x \x00" as *const u8 as *const libc::c_char,
              (*ipcp).__key,
            );
            if !pw.is_null() {
              printf(
                b"%-10d %-10.9s\x00" as *const u8 as *const libc::c_char,
                semid,
                (*pw).pw_name,
              );
            } else {
              printf(
                b"%-10d %-9d\x00" as *const u8 as *const libc::c_char,
                semid,
                (*ipcp).uid,
              );
            }
            printf(
              b" %-10o %-10ld\n\x00" as *const u8 as *const libc::c_char,
              (*ipcp).mode as libc::c_int & 0o777i32,
              semary.sem_nsems as libc::c_long,
            );
          }
        }
      }
    }
    id += 1
  }
}
#[inline(never)]
unsafe fn do_msg(mut format: libc::c_int) {
  let mut maxid: libc::c_int = 0;
  let mut msqid: libc::c_int = 0;
  let mut id: libc::c_int = 0;
  let mut msgque: msqid_ds = msqid_ds {
    msg_perm: ipc_perm {
      __key: 0,
      uid: 0,
      gid: 0,
      cuid: 0,
      cgid: 0,
      mode: 0,
      __pad1: 0,
      __seq: 0,
      __pad2: 0,
      __glibc_reserved1: 0,
      __glibc_reserved2: 0,
    },
    msg_stime: 0,
    msg_rtime: 0,
    msg_ctime: 0,
    __msg_cbytes: 0,
    msg_qnum: 0,
    msg_qbytes: 0,
    msg_lspid: 0,
    msg_lrpid: 0,
    __glibc_reserved4: 0,
    __glibc_reserved5: 0,
  };
  let mut msginfo: msginfo = msginfo {
    msgpool: 0,
    msgmap: 0,
    msgmax: 0,
    msgmnb: 0,
    msgmni: 0,
    msgssz: 0,
    msgtql: 0,
    msgseg: 0,
  };
  let mut ipcp: *mut ipc_perm = &mut msgque.msg_perm;
  let mut pw: *mut passwd = std::ptr::null_mut();
  maxid = msgctl(
    0,
    12i32,
    &mut msginfo as *mut msginfo as *mut libc::c_void as *mut msqid_ds,
  );
  if maxid < 0 {
    printf(
      b"kernel not configured for %s\n\x00" as *const u8 as *const libc::c_char,
      b"message queues\x00" as *const u8 as *const libc::c_char,
    );
    return;
  }
  match format {
    1 => {
      if msgctl(
        0,
        3i32,
        &mut msginfo as *mut msginfo as *mut libc::c_void as *mut msqid_ds,
      ) < 0
      {
        return;
      }
      printf(
        b"------ Message%s --------\n\x00" as *const u8 as *const libc::c_char,
        b"s: Limits\x00" as *const u8 as *const libc::c_char,
      );
      printf(b"max queues system wide = %d\nmax size of message (bytes) = %d\ndefault max size of queue (bytes) = %d\n\x00"
                       as *const u8 as *const libc::c_char, msginfo.msgmni,
                   msginfo.msgmax, msginfo.msgmnb);
      return;
    }
    2 => {
      printf(
        b"------ Message%s --------\n\x00" as *const u8 as *const libc::c_char,
        b"s: Status\x00" as *const u8 as *const libc::c_char,
      );
      printf(
        b"allocated queues = %d\nused headers = %d\nused space = %d bytes\n\x00" as *const u8
          as *const libc::c_char,
        msginfo.msgpool,
        msginfo.msgmap,
        msginfo.msgtql,
      );
      return;
    }
    3 => {
      printf(
        b"------ Message%s --------\n\x00" as *const u8 as *const libc::c_char,
        b" Queues: Creators/Owners\x00" as *const u8 as *const libc::c_char,
      );
      printf(
        b"%-10s %-10s %-10s %-10s %-10s %-10s\n\x00" as *const u8 as *const libc::c_char,
        b"msqid\x00" as *const u8 as *const libc::c_char,
        b"perms\x00" as *const u8 as *const libc::c_char,
        b"cuid\x00" as *const u8 as *const libc::c_char,
        b"cgid\x00" as *const u8 as *const libc::c_char,
        b"uid\x00" as *const u8 as *const libc::c_char,
        b"gid\x00" as *const u8 as *const libc::c_char,
      );
    }
    4 => {
      printf(
        b"------ Message%s --------\n\x00" as *const u8 as *const libc::c_char,
        b" Queues Send/Recv/Change Times\x00" as *const u8 as *const libc::c_char,
      );
      printf(
        b"%-8s %-10s %-20s %-20s %-20s\n\x00" as *const u8 as *const libc::c_char,
        b"msqid\x00" as *const u8 as *const libc::c_char,
        b"owner\x00" as *const u8 as *const libc::c_char,
        b"send\x00" as *const u8 as *const libc::c_char,
        b"recv\x00" as *const u8 as *const libc::c_char,
        b"change\x00" as *const u8 as *const libc::c_char,
      );
    }
    5 => {
      printf(
        b"------ Message%s --------\n\x00" as *const u8 as *const libc::c_char,
        b" Queues PIDs\x00" as *const u8 as *const libc::c_char,
      );
      printf(
        b"%-10s %-10s %-10s %-10s\n\x00" as *const u8 as *const libc::c_char,
        b"msqid\x00" as *const u8 as *const libc::c_char,
        b"owner\x00" as *const u8 as *const libc::c_char,
        b"lspid\x00" as *const u8 as *const libc::c_char,
        b"lrpid\x00" as *const u8 as *const libc::c_char,
      );
    }
    _ => {
      printf(
        b"------ Message%s --------\n\x00" as *const u8 as *const libc::c_char,
        b" Queues\x00" as *const u8 as *const libc::c_char,
      );
      printf(
        b"%-10s %-10s %-10s %-10s %-12s %-12s\n\x00" as *const u8 as *const libc::c_char,
        b"key\x00" as *const u8 as *const libc::c_char,
        b"msqid\x00" as *const u8 as *const libc::c_char,
        b"owner\x00" as *const u8 as *const libc::c_char,
        b"perms\x00" as *const u8 as *const libc::c_char,
        b"used-bytes\x00" as *const u8 as *const libc::c_char,
        b"messages\x00" as *const u8 as *const libc::c_char,
      );
    }
  }
  id = 0;
  while id <= maxid {
    msqid = msgctl(id, 11i32, &mut msgque);
    if !(msqid < 0) {
      if format == 3i32 {
        print_perms(msqid, ipcp);
      } else {
        pw = crate::libpwdgrp::pwd_grp::bb_internal_getpwuid((*ipcp).uid);
        match format {
          4 => {
            if !pw.is_null() {
              printf(
                b"%-8d %-10.10s\x00" as *const u8 as *const libc::c_char,
                msqid,
                (*pw).pw_name,
              );
            } else {
              printf(
                b"%-8d %-10d\x00" as *const u8 as *const libc::c_char,
                msqid,
                (*ipcp).uid,
              );
            }
            printf(
              b" %-20.16s\x00" as *const u8 as *const libc::c_char,
              if msgque.msg_stime != 0 {
                ctime(&mut msgque.msg_stime).offset(4)
              } else {
                b"Not set\x00" as *const u8 as *const libc::c_char
              },
            );
            printf(
              b" %-20.16s\x00" as *const u8 as *const libc::c_char,
              if msgque.msg_rtime != 0 {
                ctime(&mut msgque.msg_rtime).offset(4)
              } else {
                b"Not set\x00" as *const u8 as *const libc::c_char
              },
            );
            printf(
              b" %-20.16s\n\x00" as *const u8 as *const libc::c_char,
              if msgque.msg_ctime != 0 {
                ctime(&mut msgque.msg_ctime).offset(4)
              } else {
                b"Not set\x00" as *const u8 as *const libc::c_char
              },
            );
          }
          5 => {
            if !pw.is_null() {
              printf(
                b"%-8d %-10.10s\x00" as *const u8 as *const libc::c_char,
                msqid,
                (*pw).pw_name,
              );
            } else {
              printf(
                b"%-8d %-10d\x00" as *const u8 as *const libc::c_char,
                msqid,
                (*ipcp).uid,
              );
            }
            printf(
              b"  %5d     %5d\n\x00" as *const u8 as *const libc::c_char,
              msgque.msg_lspid,
              msgque.msg_lrpid,
            );
          }
          _ => {
            printf(
              b"0x%08x \x00" as *const u8 as *const libc::c_char,
              (*ipcp).__key,
            );
            if !pw.is_null() {
              printf(
                b"%-10d %-10.10s\x00" as *const u8 as *const libc::c_char,
                msqid,
                (*pw).pw_name,
              );
            } else {
              printf(
                b"%-10d %-10d\x00" as *const u8 as *const libc::c_char,
                msqid,
                (*ipcp).uid,
              );
            }
            printf(
              b" %-10o %-12ld %-12ld\n\x00" as *const u8 as *const libc::c_char,
              (*ipcp).mode as libc::c_int & 0o777i32,
              msgque.__msg_cbytes as libc::c_long,
              msgque.msg_qnum as libc::c_long,
            );
          }
        }
      }
    }
    id += 1
  }
}
unsafe fn print_shm(mut shmid: libc::c_int) {
  let mut shmds: shmid_ds = shmid_ds {
    shm_perm: ipc_perm {
      __key: 0,
      uid: 0,
      gid: 0,
      cuid: 0,
      cgid: 0,
      mode: 0,
      __pad1: 0,
      __seq: 0,
      __pad2: 0,
      __glibc_reserved1: 0,
      __glibc_reserved2: 0,
    },
    shm_segsz: 0,
    shm_atime: 0,
    shm_dtime: 0,
    shm_ctime: 0,
    shm_cpid: 0,
    shm_lpid: 0,
    shm_nattch: 0,
    __glibc_reserved4: 0,
    __glibc_reserved5: 0,
  };
  let mut ipcp: *mut ipc_perm = &mut shmds.shm_perm;
  if shmctl(shmid, 2i32, &mut shmds) == -1i32 {
    crate::libbb::perror_msg::bb_simple_perror_msg(
      b"shmctl\x00" as *const u8 as *const libc::c_char,
    );
    return;
  }
  printf(b"\nShared memory Segment shmid=%d\nuid=%d\tgid=%d\tcuid=%d\tcgid=%d\nmode=%#o\taccess_perms=%#o\nbytes=%ld\tlpid=%d\tcpid=%d\tnattch=%ld\n\x00"
               as *const u8 as *const libc::c_char, shmid, (*ipcp).uid,
           (*ipcp).gid, (*ipcp).cuid, (*ipcp).cgid,
           (*ipcp).mode as libc::c_int,
           (*ipcp).mode as libc::c_int & 0o777i32,
           shmds.shm_segsz as libc::c_long, shmds.shm_lpid, shmds.shm_cpid,
           shmds.shm_nattch as libc::c_long);
  printf(
    b"att_time=%-26.24s\n\x00" as *const u8 as *const libc::c_char,
    if shmds.shm_atime != 0 {
      ctime(&mut shmds.shm_atime)
    } else {
      b"Not set\x00" as *const u8 as *const libc::c_char
    },
  );
  printf(
    b"det_time=%-26.24s\n\x00" as *const u8 as *const libc::c_char,
    if shmds.shm_dtime != 0 {
      ctime(&mut shmds.shm_dtime)
    } else {
      b"Not set\x00" as *const u8 as *const libc::c_char
    },
  );
  printf(
    b"change_time=%-26.24s\n\n\x00" as *const u8 as *const libc::c_char,
    ctime(&mut shmds.shm_ctime),
  );
}
unsafe fn print_msg(mut msqid: libc::c_int) {
  let mut buf: msqid_ds = msqid_ds {
    msg_perm: ipc_perm {
      __key: 0,
      uid: 0,
      gid: 0,
      cuid: 0,
      cgid: 0,
      mode: 0,
      __pad1: 0,
      __seq: 0,
      __pad2: 0,
      __glibc_reserved1: 0,
      __glibc_reserved2: 0,
    },
    msg_stime: 0,
    msg_rtime: 0,
    msg_ctime: 0,
    __msg_cbytes: 0,
    msg_qnum: 0,
    msg_qbytes: 0,
    msg_lspid: 0,
    msg_lrpid: 0,
    __glibc_reserved4: 0,
    __glibc_reserved5: 0,
  };
  let mut ipcp: *mut ipc_perm = &mut buf.msg_perm;
  if msgctl(msqid, 2i32, &mut buf) == -1i32 {
    crate::libbb::perror_msg::bb_simple_perror_msg(
      b"msgctl\x00" as *const u8 as *const libc::c_char,
    );
    return;
  }
  printf(b"\nMessage Queue msqid=%d\nuid=%d\tgid=%d\tcuid=%d\tcgid=%d\tmode=%#o\ncbytes=%ld\tqbytes=%ld\tqnum=%ld\tlspid=%d\tlrpid=%d\n\x00"
               as *const u8 as *const libc::c_char, msqid, (*ipcp).uid,
           (*ipcp).gid, (*ipcp).cuid, (*ipcp).cgid,
           (*ipcp).mode as libc::c_int, buf.__msg_cbytes as libc::c_long,
           buf.msg_qbytes as libc::c_long, buf.msg_qnum as libc::c_long,
           buf.msg_lspid, buf.msg_lrpid);
  printf(
    b"send_time=%-26.24s\n\x00" as *const u8 as *const libc::c_char,
    if buf.msg_stime != 0 {
      ctime(&mut buf.msg_stime)
    } else {
      b"Not set\x00" as *const u8 as *const libc::c_char
    },
  );
  printf(
    b"rcv_time=%-26.24s\n\x00" as *const u8 as *const libc::c_char,
    if buf.msg_rtime != 0 {
      ctime(&mut buf.msg_rtime)
    } else {
      b"Not set\x00" as *const u8 as *const libc::c_char
    },
  );
  printf(
    b"change_time=%-26.24s\n\n\x00" as *const u8 as *const libc::c_char,
    if buf.msg_ctime != 0 {
      ctime(&mut buf.msg_ctime)
    } else {
      b"Not set\x00" as *const u8 as *const libc::c_char
    },
  );
}
unsafe fn print_sem(mut semid: libc::c_int) {
  let mut semds: semid_ds = semid_ds {
    sem_perm: ipc_perm {
      __key: 0,
      uid: 0,
      gid: 0,
      cuid: 0,
      cgid: 0,
      mode: 0,
      __pad1: 0,
      __seq: 0,
      __pad2: 0,
      __glibc_reserved1: 0,
      __glibc_reserved2: 0,
    },
    sem_otime: 0,
    __glibc_reserved1: 0,
    sem_ctime: 0,
    __glibc_reserved2: 0,
    sem_nsems: 0,
    __glibc_reserved3: 0,
    __glibc_reserved4: 0,
  };
  let mut ipcp: *mut ipc_perm = &mut semds.sem_perm;
  let mut arg: semun = semun { val: 0 };
  let mut i: libc::c_uint = 0;
  arg.buf = &mut semds;
  if semctl(semid, 0, 2i32, arg) != 0 {
    crate::libbb::perror_msg::bb_simple_perror_msg(
      b"semctl\x00" as *const u8 as *const libc::c_char,
    );
    return;
  }
  printf(b"\nSemaphore Array semid=%d\nuid=%d\t gid=%d\t cuid=%d\t cgid=%d\nmode=%#o, access_perms=%#o\nnsems = %ld\notime = %-26.24s\n\x00"
               as *const u8 as *const libc::c_char, semid, (*ipcp).uid,
           (*ipcp).gid, (*ipcp).cuid, (*ipcp).cgid,
           (*ipcp).mode as libc::c_int,
           (*ipcp).mode as libc::c_int & 0o777i32,
           semds.sem_nsems as libc::c_long,
           if semds.sem_otime != 0 {
               ctime(&mut semds.sem_otime)
           } else { b"Not set\x00" as *const u8 as *const libc::c_char });
  printf(
    b"ctime = %-26.24s\n%-10s %-10s %-10s %-10s %-10s\n\x00" as *const u8 as *const libc::c_char,
    ctime(&mut semds.sem_ctime),
    b"semnum\x00" as *const u8 as *const libc::c_char,
    b"value\x00" as *const u8 as *const libc::c_char,
    b"ncount\x00" as *const u8 as *const libc::c_char,
    b"zcount\x00" as *const u8 as *const libc::c_char,
    b"pid\x00" as *const u8 as *const libc::c_char,
  );
  arg.val = 0;
  i = 0 as libc::c_uint;
  while (i as libc::c_ulong) < semds.sem_nsems {
    let mut val: libc::c_int = 0;
    let mut ncnt: libc::c_int = 0;
    let mut zcnt: libc::c_int = 0;
    let mut pid: libc::c_int = 0;
    val = semctl(semid, i as libc::c_int, 12i32, arg);
    ncnt = semctl(semid, i as libc::c_int, 14i32, arg);
    zcnt = semctl(semid, i as libc::c_int, 15i32, arg);
    pid = semctl(semid, i as libc::c_int, 11i32, arg);
    if val < 0 || ncnt < 0 || zcnt < 0 || pid < 0 {
      crate::libbb::perror_msg::bb_simple_perror_msg_and_die(
        b"semctl\x00" as *const u8 as *const libc::c_char,
      );
    }
    printf(
      b"%-10u %-10d %-10d %-10d %-10d\n\x00" as *const u8 as *const libc::c_char,
      i,
      val,
      ncnt,
      zcnt,
      pid,
    );
    i = i.wrapping_add(1)
  }
  crate::libbb::xfuncs_printf::bb_putchar('\n' as i32);
}
//usage:#define ipcs_trivial_usage
//usage:       "[[-smq] -i SHMID] | [[-asmq] [-tcplu]]"
//usage:#define ipcs_full_usage "\n\n"
//usage:       "	-i ID	Show specific resource"
//usage:     "\nResource specification:"
//usage:     "\n	-m	Shared memory segments"
//usage:     "\n	-q	Message queues"
//usage:     "\n	-s	Semaphore arrays"
//usage:     "\n	-a	All (default)"
//usage:     "\nOutput format:"
//usage:     "\n	-t	Time"
//usage:     "\n	-c	Creator"
//usage:     "\n	-p	Pid"
//usage:     "\n	-l	Limits"
//usage:     "\n	-u	Summary"
pub unsafe fn ipcs_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut format: libc::c_int = 0; // -t
  let mut opt: libc::c_uint = 0; // -c
  let mut opt_i: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>(); // -p
  opt = crate::libbb::getopt32::getopt32(
    argv,
    b"i:aqsmtcplu\x00" as *const u8 as *const libc::c_char,
    &mut opt_i as *mut *mut libc::c_char,
  ); // -l
  if opt & (1i32 << 5i32) as libc::c_uint != 0 {
    format = 4i32
  } // -u
  if opt & (1i32 << 6i32) as libc::c_uint != 0 {
    format = 3i32
  }
  if opt & (1i32 << 7i32) as libc::c_uint != 0 {
    format = 5i32
  }
  if opt & (1i32 << 8i32) as libc::c_uint != 0 {
    format = 1i32
  }
  if opt & (1i32 << 9i32) as libc::c_uint != 0 {
    format = 2i32
  }
  if opt & (1i32 << 0) as libc::c_uint != 0 {
    // -i
    let mut id: libc::c_int = 0;
    id = crate::libbb::xatonum::xatoi(opt_i);
    if opt & (1i32 << 4i32) as libc::c_uint != 0 {
      print_shm(id);
      crate::libbb::fflush_stdout_and_exit::fflush_stdout_and_exit(0i32);
    }
    if opt & (1i32 << 3i32) as libc::c_uint != 0 {
      print_sem(id);
      crate::libbb::fflush_stdout_and_exit::fflush_stdout_and_exit(0i32);
    }
    if opt & (1i32 << 2i32) as libc::c_uint != 0 {
      print_msg(id);
      crate::libbb::fflush_stdout_and_exit::fflush_stdout_and_exit(0i32);
    }
    crate::libbb::appletlib::bb_show_usage();
  }
  if opt & (1i32 << 1i32) as libc::c_uint != 0
    || opt & (1i32 << 2i32 | 1i32 << 3i32 | 1i32 << 4i32) as libc::c_uint == 0
  {
    // none of -q,-s,-m == all
    opt |= (1i32 << 2i32 | 1i32 << 3i32 | 1i32 << 4i32) as libc::c_uint
  }
  crate::libbb::xfuncs_printf::bb_putchar('\n' as i32);
  if opt & (1i32 << 2i32) as libc::c_uint != 0 {
    do_msg(format);
    crate::libbb::xfuncs_printf::bb_putchar('\n' as i32);
  }
  if opt & (1i32 << 4i32) as libc::c_uint != 0 {
    do_shm(format);
    crate::libbb::xfuncs_printf::bb_putchar('\n' as i32);
  }
  if opt & (1i32 << 3i32) as libc::c_uint != 0 {
    do_sem(format);
    crate::libbb::xfuncs_printf::bb_putchar('\n' as i32);
  }
  crate::libbb::fflush_stdout_and_exit::fflush_stdout_and_exit(0i32);
}

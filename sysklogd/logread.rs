use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  static mut stdout: *mut FILE;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn strnlen(__string: *const libc::c_char, __maxlen: size_t) -> size_t;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_signals(sigs: libc::c_int, f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>);
  #[no_mangle]
  fn kill_myself_with_sig(sig: libc::c_int) -> !;
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
  #[no_mangle]
  fn semget(__key: key_t, __nsems: libc::c_int, __semflg: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn semop(__semid: libc::c_int, __sops: *mut sembuf, __nsops: size_t) -> libc::c_int;
  #[no_mangle]
  fn shmget(__key: key_t, __size: size_t, __shmflg: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn shmat(
    __shmid: libc::c_int,
    __shmaddr: *const libc::c_void,
    __shmflg: libc::c_int,
  ) -> *mut libc::c_void;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __key_t = libc::c_int;
pub type int32_t = __int32_t;
use crate::librb::uint32_t;
use crate::librb::size_t;
pub type key_t = __key_t;



use crate::librb::FILE;
pub type C2RustUnnamed = libc::c_uint;
pub const BB_FATAL_SIGS: C2RustUnnamed = 117503054;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub SMrup: [sembuf; 1],
  pub SMrdn: [sembuf; 2],
  pub shbuf: *mut shbuf_ds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shbuf_ds {
  pub size: int32_t,
  pub tail: int32_t,
  pub data: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sembuf {
  pub sem_num: libc::c_ushort,
  pub sem_op: libc::c_short,
  pub sem_flg: libc::c_short,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const KEY_ID: C2RustUnnamed_0 = 1095648583;
static mut init_sem: [sembuf; 3] = [
  {
    let mut init = sembuf {
      sem_num: 0i32 as libc::c_ushort,
      sem_op: -1i32 as libc::c_short,
      sem_flg: (0o4000i32 | 0x1000i32) as libc::c_short,
    };
    init
  },
  {
    let mut init = sembuf {
      sem_num: 1i32 as libc::c_ushort,
      sem_op: 0i32 as libc::c_short,
      sem_flg: 0,
    };
    init
  },
  {
    let mut init = sembuf {
      sem_num: 0i32 as libc::c_ushort,
      sem_op: 1i32 as libc::c_short,
      sem_flg: 0x1000i32 as libc::c_short,
    };
    init
  },
];
/* On Linux, shmdt is not mandatory on exit */
/*
 * sem_up - up()'s a semaphore.
 */
unsafe extern "C" fn sem_up(mut semid: libc::c_int) {
  if semop(
    semid,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .SMrup
      .as_mut_ptr(),
    1i32 as size_t,
  ) == -1i32
  {
    bb_simple_perror_msg_and_die(b"semop[SMrup]\x00" as *const u8 as *const libc::c_char);
  };
}
unsafe extern "C" fn interrupted(mut sig: libc::c_int) {
  /* shmdt(shbuf); - on Linux, shmdt is not mandatory on exit */
  kill_myself_with_sig(sig); /* ipc semaphore id */
}
#[no_mangle]
pub unsafe extern "C" fn logread_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut cur: libc::c_uint = 0; /* ipc shared memory id */
  let mut log_semid: libc::c_int = 0;
  let mut log_shmid: libc::c_int = 0;
  let mut follow: libc::c_int =
    getopt32(argv, b"fF\x00" as *const u8 as *const libc::c_char) as libc::c_int;
  memcpy(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .SMrup
      .as_mut_ptr() as *mut libc::c_void,
    init_sem.as_ptr() as *const libc::c_void,
    ::std::mem::size_of::<[sembuf; 3]>() as libc::c_ulong,
  );
  log_shmid = shmget(KEY_ID as libc::c_int, 0i32 as size_t, 0i32);
  if log_shmid == -1i32 {
    bb_perror_msg_and_die(
      b"can\'t %s syslogd buffer\x00" as *const u8 as *const libc::c_char,
      b"find\x00" as *const u8 as *const libc::c_char,
    );
  }
  /* Attach shared memory to our char* */
  let ref mut fresh0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).shbuf;
  *fresh0 = shmat(log_shmid, 0 as *const libc::c_void, 0o10000i32) as *mut shbuf_ds;
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .shbuf
    .is_null()
  {
    bb_perror_msg_and_die(
      b"can\'t %s syslogd buffer\x00" as *const u8 as *const libc::c_char,
      b"access\x00" as *const u8 as *const libc::c_char,
    );
  }
  log_semid = semget(KEY_ID as libc::c_int, 0i32, 0i32);
  if log_semid == -1i32 {
    bb_simple_perror_msg_and_die(
      b"can\'t get access to semaphores for syslogd buffer\x00" as *const u8 as *const libc::c_char,
    );
  }
  bb_signals(
    BB_FATAL_SIGS as libc::c_int,
    Some(interrupted as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  /* Suppose atomic memory read */
  /* Max possible value for tail is shbuf->size - 1 */
  cur = (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).shbuf).tail as libc::c_uint;
  let mut current_block_49: u64;
  loop
  /* Loop for -f or -F, one pass otherwise */
  {
    let mut shbuf_size: libc::c_uint = 0; /* for gcc */
    let mut shbuf_tail: libc::c_uint = 0; /* for gcc */
    let mut shbuf_data: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut len_first_part: libc::c_int = 0;
    let mut len_total: libc::c_int = 0;
    len_total = len_total;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    copy = copy;
    if semop(
      log_semid,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .SMrdn
        .as_mut_ptr(),
      2i32 as size_t,
    ) == -1i32
    {
      bb_simple_perror_msg_and_die(b"semop[SMrdn]\x00" as *const u8 as *const libc::c_char);
    }
    /* Copy the info, helps gcc to realize that it doesn't change */
    shbuf_size = (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).shbuf).size as libc::c_uint; /* pointer! */
    shbuf_tail = (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).shbuf).tail as libc::c_uint; /* -f */
    shbuf_data = (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).shbuf)
      .data
      .as_mut_ptr();
    if follow & 1i32 == 0 {
      /* not -f */
      /* if -F, "convert" it to -f, so that we don't
       * dump the entire buffer on each iteration
       */
      follow >>= 1i32;
      /* advance to oldest complete message */
      /* find NUL */
      cur = (cur as libc::c_ulong).wrapping_add(strlen(shbuf_data.offset(cur as isize)))
        as libc::c_uint as libc::c_uint;
      if cur >= shbuf_size {
        /* last byte in buffer? */
        cur = strnlen(shbuf_data, shbuf_tail as size_t) as libc::c_uint;
        if cur == shbuf_tail {
          current_block_49 = 15956509640765329438;
        } else {
          current_block_49 = 2891135413264362348;
        }
      /* no complete messages */
      } else {
        current_block_49 = 2891135413264362348;
      }
      match current_block_49 {
        15956509640765329438 => {}
        _ => {
          /* advance to first byte of the message */
          cur = cur.wrapping_add(1);
          if cur >= shbuf_size {
            /* last byte in buffer? */
            cur = 0i32 as libc::c_uint
          } /* TODO: replace me with a sleep_on */
          current_block_49 = 6450597802325118133;
        }
      }
    } else if cur == shbuf_tail {
      sem_up(log_semid);
      fflush_all();
      sleep(1i32 as libc::c_uint);
      current_block_49 = 9606288038608642794;
    } else {
      current_block_49 = 6450597802325118133;
    }
    match current_block_49 {
      6450597802325118133 => {
        /* Read from cur to tail */
        len_total = shbuf_tail.wrapping_sub(cur) as libc::c_int;
        len_first_part = len_total;
        if len_total < 0i32 {
          /* message wraps: */
          /* [SECOND PART.........FIRST PART] */
          /*  ^data      ^tail    ^cur      ^size */
          len_total =
            (len_total as libc::c_uint).wrapping_add(shbuf_size) as libc::c_int as libc::c_int
        }
        copy = xmalloc((len_total + 1i32) as size_t) as *mut libc::c_char;
        if len_first_part < 0i32 {
          /* message wraps (see above) */
          len_first_part = shbuf_size.wrapping_sub(cur) as libc::c_int;
          memcpy(
            copy.offset(len_first_part as isize) as *mut libc::c_void,
            shbuf_data as *const libc::c_void,
            shbuf_tail as libc::c_ulong,
          );
        }
        memcpy(
          copy as *mut libc::c_void,
          shbuf_data.offset(cur as isize) as *const libc::c_void,
          len_first_part as libc::c_ulong,
        );
        *copy.offset(len_total as isize) = '\u{0}' as i32 as libc::c_char;
        cur = shbuf_tail;
        current_block_49 = 15956509640765329438;
      }
      _ => {}
    }
    match current_block_49 {
      15956509640765329438 => {
        /* release the lock on the log chain */
        sem_up(log_semid);
        i = 0i32;
        while i < len_total {
          fputs_unlocked(copy.offset(i as isize), stdout);
          i = (i as libc::c_ulong)
            .wrapping_add(strlen(copy.offset(i as isize)).wrapping_add(1i32 as libc::c_ulong))
            as libc::c_int as libc::c_int
        }
        free(copy as *mut libc::c_void);
        fflush_all();
      }
      _ => {}
    }
    if !(follow != 0) {
      break;
    }
  }
  /* shmdt(shbuf); - on Linux, shmdt is not mandatory on exit */
  fflush_stdout_and_exit(0i32);
}

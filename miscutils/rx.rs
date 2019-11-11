use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::appletlib::applet_name;
use libc;
use libc::alarm;
extern "C" {

  #[no_mangle]
  fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;

  #[no_mangle]
  fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
  #[no_mangle]
  fn tcsetattr(
    __fd: libc::c_int,
    __optional_actions: libc::c_int,
    __termios_p: *const termios,
  ) -> libc::c_int;
  #[no_mangle]
  fn cfmakeraw(__termios_p: *mut termios);
  #[no_mangle]
  fn tcflush(__fd: libc::c_int, __queue_selector: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn signal_no_SA_RESTART_empty_mask(
    sig: libc::c_int,
    handler: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
  );
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn safe_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
  #[no_mangle]
  fn single_argv(argv: *mut *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
}

use crate::librb::size_t;
use libc::ssize_t;
use libc::termios;
unsafe extern "C" fn read_byte(mut timeout: libc::c_uint) -> libc::c_int {
  let mut buf: libc::c_uchar = 0;
  let mut n: libc::c_int = 0;
  alarm(timeout);
  /* NOT safe_read! We want ALRM to interrupt us */
  n = read(
    0i32,
    &mut buf as *mut libc::c_uchar as *mut libc::c_void,
    1i32 as size_t,
  ) as libc::c_int;
  alarm(0i32 as libc::c_uint);
  if n == 1i32 {
    return buf as libc::c_int;
  }
  return -1i32;
}
unsafe extern "C" fn receive(mut file_fd: libc::c_int) -> libc::c_int {
  let mut current_block: u64;
  let mut blockBuf: [libc::c_uchar; 1024] = [0; 1024];
  let mut blockLength: libc::c_uint = 0i32 as libc::c_uint;
  let mut errors: libc::c_uint = 0i32 as libc::c_uint;
  let mut wantBlockNo: libc::c_uint = 1i32 as libc::c_uint;
  let mut length: libc::c_uint = 0i32 as libc::c_uint;
  let mut do_crc: libc::c_int = 1i32;
  let mut reply_char: libc::c_char = 0;
  let mut timeout: libc::c_uint = 10i32 as libc::c_uint;
  /* Flush pending input */
  tcflush(0i32, 0i32);
  /* Ask for CRC; if we get errors, we will go with checksum */
  reply_char = 'C' as i32 as libc::c_char;
  full_write(
    1i32,
    &mut reply_char as *mut libc::c_char as *const libc::c_void,
    1i32 as size_t,
  );
  loop {
    let mut blockBegin: libc::c_int = 0;
    let mut blockNo: libc::c_int = 0;
    let mut blockNoOnesCompl: libc::c_int = 0;
    let mut cksum_or_crc: libc::c_int = 0;
    let mut expected: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    blockBegin = read_byte(timeout);
    if blockBegin < 0i32 {
      current_block = 17524159567010234572;
    } else {
      /* If last block, remove padding */
      if blockBegin == 0x4i32 {
        /* Data blocks can be padded with ^Z characters */
        /* This code tries to detect and remove them */
        if blockLength >= 3i32 as libc::c_uint
          && blockBuf[blockLength.wrapping_sub(1i32 as libc::c_uint) as usize] as libc::c_int
            == 0x1ai32
          && blockBuf[blockLength.wrapping_sub(2i32 as libc::c_uint) as usize] as libc::c_int
            == 0x1ai32
          && blockBuf[blockLength.wrapping_sub(3i32 as libc::c_uint) as usize] as libc::c_int
            == 0x1ai32
        {
          while blockLength != 0
            && blockBuf[blockLength.wrapping_sub(1i32 as libc::c_uint) as usize] as libc::c_int
              == 0x1ai32
          {
            blockLength = blockLength.wrapping_sub(1)
          }
        }
      }
      /* Write previously received block */
      *bb_errno = 0i32;
      if full_write(
        file_fd,
        blockBuf.as_mut_ptr() as *const libc::c_void,
        blockLength as size_t,
      ) != blockLength as isize
      {
        bb_simple_perror_msg(b"write error\x00" as *const u8 as *const libc::c_char);
        current_block = 2830166982076203563;
      } else {
        timeout = 1i32 as libc::c_uint;
        reply_char = 0x15i32 as libc::c_char;
        match blockBegin {
          1 | 2 => {
            current_block = 11459959175219260272;
            match current_block {
              3021757436737313741 => {
                reply_char = 0x6i32 as libc::c_char;
                full_write(
                  1i32,
                  &mut reply_char as *mut libc::c_char as *const libc::c_void,
                  1i32 as size_t,
                );
                return length as libc::c_int;
              }
              _ => {
                /* Block no */
                blockNo = read_byte(1i32 as libc::c_uint);
                if !(blockNo < 0i32) {
                  /* Block no, in one's complement form */
                  blockNoOnesCompl = read_byte(1i32 as libc::c_uint);
                  if !(blockNoOnesCompl < 0i32) {
                    if blockNo != 255i32 - blockNoOnesCompl {
                      bb_simple_error_msg(
                        b"bad block ones compl\x00" as *const u8 as *const libc::c_char,
                      );
                    } else {
                      blockLength = if blockBegin == 0x1i32 {
                        128i32
                      } else {
                        1024i32
                      } as libc::c_uint;
                      i = 0i32;
                      loop {
                        if !((i as libc::c_uint) < blockLength) {
                          current_block = 790185930182612747;
                          break;
                        }
                        let mut cc: libc::c_int = read_byte(1i32 as libc::c_uint);
                        if cc < 0i32 {
                          current_block = 17524159567010234572;
                          break;
                        }
                        blockBuf[i as usize] = cc as libc::c_uchar;
                        i += 1
                      }
                      match current_block {
                        17524159567010234572 => {}
                        _ => {
                          cksum_or_crc = read_byte(1i32 as libc::c_uint);
                          if !(cksum_or_crc < 0i32) {
                            if do_crc != 0 {
                              cksum_or_crc = cksum_or_crc << 8i32 | read_byte(1i32 as libc::c_uint);
                              if cksum_or_crc < 0i32 {
                                current_block = 17524159567010234572;
                              } else {
                                current_block = 13460095289871124136;
                              }
                            } else {
                              current_block = 13460095289871124136;
                            }
                            match current_block {
                              17524159567010234572 => {}
                              _ => {
                                if blockNo as libc::c_uint
                                  == wantBlockNo.wrapping_sub(1i32 as libc::c_uint)
                                    & 0xffi32 as libc::c_uint
                                {
                                  /* a repeat of the last block is ok, just ignore it. */
                                  /* this also ignores the initial block 0 which is */
                                  /* meta data. */
                                  blockLength = 0i32 as libc::c_uint;
                                  current_block = 1735099875076272758;
                                } else if blockNo as libc::c_uint
                                  != wantBlockNo & 0xffi32 as libc::c_uint
                                {
                                  bb_error_msg(
                                    b"unexpected block no, 0x%08x, expecting 0x%08x\x00"
                                      as *const u8
                                      as *const libc::c_char,
                                    blockNo,
                                    wantBlockNo,
                                  );
                                  current_block = 17524159567010234572;
                                } else {
                                  expected = 0i32 as libc::c_uint;
                                  if do_crc != 0 {
                                    i = 0i32;
                                    while (i as libc::c_uint) < blockLength {
                                      expected = expected
                                        ^ ((blockBuf[i as usize] as libc::c_int) << 8i32)
                                          as libc::c_uint;
                                      j = 0i32;
                                      while j < 8i32 {
                                        if expected & 0x8000i32 as libc::c_uint != 0 {
                                          expected = expected << 1i32 ^ 0x1021i32 as libc::c_uint
                                        } else {
                                          expected = expected << 1i32
                                        }
                                        j += 1
                                      }
                                      i += 1
                                    }
                                    expected &= 0xffffi32 as libc::c_uint
                                  } else {
                                    i = 0i32;
                                    while (i as libc::c_uint) < blockLength {
                                      expected =
                                        expected.wrapping_add(blockBuf[i as usize] as libc::c_uint);
                                      i += 1
                                    }
                                    expected &= 0xffi32 as libc::c_uint
                                  }
                                  if cksum_or_crc as libc::c_uint != expected {
                                    bb_error_msg(
                                      if do_crc != 0 {
                                        b"crc error, expected 0x%04x, got 0x%04x\x00" as *const u8
                                          as *const libc::c_char
                                      } else {
                                        b"checksum error, expected 0x%02x, got 0x%02x\x00"
                                          as *const u8
                                          as *const libc::c_char
                                      },
                                      expected,
                                      cksum_or_crc,
                                    );
                                    current_block = 17524159567010234572;
                                  } else {
                                    wantBlockNo = wantBlockNo.wrapping_add(1);
                                    length = length.wrapping_add(blockLength);
                                    current_block = 1735099875076272758;
                                  }
                                }
                                match current_block {
                                  17524159567010234572 => {}
                                  _ => {
                                    errors = 0i32 as libc::c_uint;
                                    reply_char = 0x6i32 as libc::c_char;
                                    full_write(
                                      1i32,
                                      &mut reply_char as *mut libc::c_char as *const libc::c_void,
                                      1i32 as size_t,
                                    );
                                    continue;
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
            current_block = 17524159567010234572;
          }
          4 => {
            current_block = 3021757436737313741;
            match current_block {
              3021757436737313741 => {
                reply_char = 0x6i32 as libc::c_char;
                full_write(
                  1i32,
                  &mut reply_char as *mut libc::c_char as *const libc::c_void,
                  1i32 as size_t,
                );
                return length as libc::c_int;
              }
              _ => {
                blockNo = read_byte(1i32 as libc::c_uint);
                if !(blockNo < 0i32) {
                  blockNoOnesCompl = read_byte(1i32 as libc::c_uint);
                  if !(blockNoOnesCompl < 0i32) {
                    if blockNo != 255i32 - blockNoOnesCompl {
                      bb_simple_error_msg(
                        b"bad block ones compl\x00" as *const u8 as *const libc::c_char,
                      );
                    } else {
                      blockLength = if blockBegin == 0x1i32 {
                        128i32
                      } else {
                        1024i32
                      } as libc::c_uint;
                      i = 0i32;
                      loop {
                        if !((i as libc::c_uint) < blockLength) {
                          current_block = 790185930182612747;
                          break;
                        }
                        let mut cc: libc::c_int = read_byte(1i32 as libc::c_uint);
                        if cc < 0i32 {
                          current_block = 17524159567010234572;
                          break;
                        }
                        blockBuf[i as usize] = cc as libc::c_uchar;
                        i += 1
                      }
                      match current_block {
                        17524159567010234572 => {}
                        _ => {
                          cksum_or_crc = read_byte(1i32 as libc::c_uint);
                          if !(cksum_or_crc < 0i32) {
                            if do_crc != 0 {
                              cksum_or_crc = cksum_or_crc << 8i32 | read_byte(1i32 as libc::c_uint);
                              if cksum_or_crc < 0i32 {
                                current_block = 17524159567010234572;
                              } else {
                                current_block = 13460095289871124136;
                              }
                            } else {
                              current_block = 13460095289871124136;
                            }
                            match current_block {
                              17524159567010234572 => {}
                              _ => {
                                if blockNo as libc::c_uint
                                  == wantBlockNo.wrapping_sub(1i32 as libc::c_uint)
                                    & 0xffi32 as libc::c_uint
                                {
                                  blockLength = 0i32 as libc::c_uint;
                                  current_block = 1735099875076272758;
                                } else if blockNo as libc::c_uint
                                  != wantBlockNo & 0xffi32 as libc::c_uint
                                {
                                  bb_error_msg(
                                    b"unexpected block no, 0x%08x, expecting 0x%08x\x00"
                                      as *const u8
                                      as *const libc::c_char,
                                    blockNo,
                                    wantBlockNo,
                                  );
                                  current_block = 17524159567010234572;
                                } else {
                                  expected = 0i32 as libc::c_uint;
                                  if do_crc != 0 {
                                    i = 0i32;
                                    while (i as libc::c_uint) < blockLength {
                                      expected = expected
                                        ^ ((blockBuf[i as usize] as libc::c_int) << 8i32)
                                          as libc::c_uint;
                                      j = 0i32;
                                      while j < 8i32 {
                                        if expected & 0x8000i32 as libc::c_uint != 0 {
                                          expected = expected << 1i32 ^ 0x1021i32 as libc::c_uint
                                        } else {
                                          expected = expected << 1i32
                                        }
                                        j += 1
                                      }
                                      i += 1
                                    }
                                    expected &= 0xffffi32 as libc::c_uint
                                  } else {
                                    i = 0i32;
                                    while (i as libc::c_uint) < blockLength {
                                      expected =
                                        expected.wrapping_add(blockBuf[i as usize] as libc::c_uint);
                                      i += 1
                                    }
                                    expected &= 0xffi32 as libc::c_uint
                                  }
                                  if cksum_or_crc as libc::c_uint != expected {
                                    bb_error_msg(
                                      if do_crc != 0 {
                                        b"crc error, expected 0x%04x, got 0x%04x\x00" as *const u8
                                          as *const libc::c_char
                                      } else {
                                        b"checksum error, expected 0x%02x, got 0x%02x\x00"
                                          as *const u8
                                          as *const libc::c_char
                                      },
                                      expected,
                                      cksum_or_crc,
                                    );
                                    current_block = 17524159567010234572;
                                  } else {
                                    wantBlockNo = wantBlockNo.wrapping_add(1);
                                    length = length.wrapping_add(blockLength);
                                    current_block = 1735099875076272758;
                                  }
                                }
                                match current_block {
                                  17524159567010234572 => {}
                                  _ => {
                                    errors = 0i32 as libc::c_uint;
                                    reply_char = 0x6i32 as libc::c_char;
                                    full_write(
                                      1i32,
                                      &mut reply_char as *mut libc::c_char as *const libc::c_void,
                                      1i32 as size_t,
                                    );
                                    continue;
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
            current_block = 17524159567010234572;
          }
          _ => {
            current_block = 17524159567010234572;
          }
        }
      }
    }
    loop {
      match current_block {
        2830166982076203563 => {
          /* 5 CAN followed by 5 BS. Don't try too hard... */
          safe_write(
            1i32,
            b"\x18\x18\x18\x18\x18\x08\x08\x08\x08\x08\x00" as *const u8 as *const libc::c_char
              as *const libc::c_void,
            10i32 as size_t,
          );
          return -1i32;
        }
        _ => {
          blockLength = 0i32 as libc::c_uint;
          errors = errors.wrapping_add(1);
          if errors == 10i32 as libc::c_uint {
            /* Abort */
            /* If were asking for crc, try again w/o crc */
            if reply_char as libc::c_int == 'C' as i32 {
              reply_char = 0x15i32 as libc::c_char;
              errors = 0i32 as libc::c_uint;
              do_crc = 0i32;
              current_block = 17524159567010234572;
            } else {
              bb_simple_error_msg(
                b"too many errors; giving up\x00" as *const u8 as *const libc::c_char,
              );
              current_block = 2830166982076203563;
            }
          } else {
            /* Flush pending input */
            tcflush(0i32, 0i32);
            full_write(
              1i32,
              &mut reply_char as *mut libc::c_char as *const libc::c_void,
              1i32 as size_t,
            );
            break;
          }
        }
      }
    }
  }
  /* for (;;) */
}
unsafe extern "C" fn sigalrm_handler(mut _signum: libc::c_int) {}
#[no_mangle]
pub unsafe extern "C" fn rx_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut tty: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
  };
  let mut orig_tty: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
  };
  let mut termios_err: libc::c_int = 0;
  let mut file_fd: libc::c_int = 0;
  let mut n: libc::c_int = 0;
  /* Disabled by vda:
   * why we can't receive from stdin? Why we *require*
   * controlling tty?? */
  /*read_fd = xopen(CURRENT_TTY, O_RDWR);*/
  file_fd = xopen(single_argv(argv), 0o2i32 | 0o100i32 | 0o1000i32);
  termios_err = tcgetattr(0i32, &mut tty);
  if termios_err == 0i32 {
    //TODO: use set_termios_to_raw()
    orig_tty = tty;
    cfmakeraw(&mut tty);
    tcsetattr(0i32, 2i32, &mut tty);
  }
  /* No SA_RESTART: we want ALRM to interrupt read() */
  signal_no_SA_RESTART_empty_mask(
    14i32,
    Some(sigalrm_handler as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  n = receive(file_fd);
  if termios_err == 0i32 {
    tcsetattr(0i32, 2i32, &mut orig_tty);
  }
  fflush_stdout_and_exit((n >= 0i32) as libc::c_int);
}

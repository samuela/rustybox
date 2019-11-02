use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  static mut stdin: *mut _IO_FILE;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn isqrt(N: libc::c_ulonglong) -> libc::c_ulong;
  #[no_mangle]
  fn skip_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn skip_non_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  /* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
   * But potentially slow, don't use in one-billion-times loops */
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  /* Chops off '\n' from the end, unlike fgets: */
  #[no_mangle]
  fn xmalloc_fgetline(file: *mut FILE) -> *mut libc::c_char;
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
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
  pub _flags: libc::c_int,
  pub _IO_read_ptr: *mut libc::c_char,
  pub _IO_read_end: *mut libc::c_char,
  pub _IO_read_base: *mut libc::c_char,
  pub _IO_write_base: *mut libc::c_char,
  pub _IO_write_ptr: *mut libc::c_char,
  pub _IO_write_end: *mut libc::c_char,
  pub _IO_buf_base: *mut libc::c_char,
  pub _IO_buf_end: *mut libc::c_char,
  pub _IO_save_base: *mut libc::c_char,
  pub _IO_backup_base: *mut libc::c_char,
  pub _IO_save_end: *mut libc::c_char,
  pub _markers: *mut _IO_marker,
  pub _chain: *mut _IO_FILE,
  pub _fileno: libc::c_int,
  pub _flags2: libc::c_int,
  pub _old_offset: __off_t,
  pub _cur_column: libc::c_ushort,
  pub _vtable_offset: libc::c_schar,
  pub _shortbuf: [libc::c_char; 1],
  pub _lock: *mut libc::c_void,
  pub _offset: __off64_t,
  pub __pad1: *mut libc::c_void,
  pub __pad2: *mut libc::c_void,
  pub __pad3: *mut libc::c_void,
  pub __pad4: *mut libc::c_void,
  pub __pad5: size_t,
  pub _mode: libc::c_int,
  pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
  pub _next: *mut _IO_marker,
  pub _sbuf: *mut _IO_FILE,
  pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
/*
 * Copyright (C) 2017 Denys Vlasenko <vda.linux@googlemail.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config FACTOR
//config:	bool "factor (2.7 kb)"
//config:	default y
//config:	help
//config:	factor factorizes integers
//applet:IF_FACTOR(APPLET(factor, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_FACTOR) += factor.o
//usage:#define factor_trivial_usage
//usage:       "[NUMBER]..."
//usage:#define factor_full_usage "\n\n"
//usage:       "Print prime factors"
pub type wide_t = libc::c_ulonglong;
/* "unsigned" is half as wide as ullong */
pub type half_t = libc::c_uint;
pub const SHIFT_7: C2RustUnnamed = 128;
pub const MULTIPLE_OF_7: C2RustUnnamed = 2048;
pub const SHIFT_5: C2RustUnnamed = 8;
pub const MULTIPLE_OF_5: C2RustUnnamed = 64;
pub const SHIFT_3: C2RustUnnamed = 1;
pub const MULTIPLE_OF_3: C2RustUnnamed = 4;
pub const MULTIPLE_DETECTED: C2RustUnnamed = 2116;
pub const INCREMENT_EACH: C2RustUnnamed = 137;
pub type C2RustUnnamed = libc::c_uint;
unsafe extern "C" fn isqrt_odd(mut N: wide_t) -> half_t {
  let mut s: half_t = isqrt(N) as half_t;
  /* Subtract 1 from even s, odd s won't change: */
  /* (doesnt work for zero, but we know that s != 0 here) */
  s = s.wrapping_sub(1i32 as libc::c_uint) | 1i32 as libc::c_uint;
  return s;
}
#[inline(never)]
unsafe extern "C" fn factorize(mut N: wide_t) {
  let mut factor: half_t = 0;
  let mut max_factor: half_t = 0;
  // unsigned count3;
  // unsigned count5;
  // unsigned count7;
  // ^^^^^^^^^^^^^^^ commented-out simple sieving code (easier to grasp).
  // Faster sieving, using one word for potentially up to 6 counters:
  // count upwards in each mask, counter "triggers" when it sets its mask to "100[0]..."
  // 10987654321098765432109876543210 - bits 31-0 in 32-bit word
  //    17777713333311111777775555333 - bit masks for counters for primes 3,5,7,11,13,17
  //         100000100001000010001001 - value for adding 1 to each mask
  //    10000010000010000100001000100 - value for checking that any mask reached msb
  let mut sieve_word: libc::c_uint = 0;
  if !(N < 4i32 as libc::c_ulonglong) {
    while N & 1i32 as libc::c_ulonglong == 0 {
      printf(b" 2\x00" as *const u8 as *const libc::c_char);
      N >>= 1i32
    }
    /* The code needs to be optimized for the case where
     * there are large prime factors. For example,
     * this is not hard:
     * 8262075252869367027 = 3 7 17 23 47 101 113 127 131 137 823
     * (the largest factor to test is only ~sqrt(823) = 28)
     * but this is:
     * 18446744073709551601 = 53 348051774975651917
     * the last factor requires testing up to
     * 589959129 - about 100 million iterations.
     * The slowest case (largest prime) for N < 2^64 is
     * factor 18446744073709551557 (0xffffffffffffffc5).
     */
    max_factor = isqrt_odd(N);
    // count3 = 3;
    // count5 = 6;
    // count7 = 9;
    sieve_word = (0i32
      + (MULTIPLE_OF_3 as libc::c_int - 3i32 * SHIFT_3 as libc::c_int)
      + (MULTIPLE_OF_5 as libc::c_int - 6i32 * SHIFT_5 as libc::c_int)
      + (MULTIPLE_OF_7 as libc::c_int - 9i32 * SHIFT_7 as libc::c_int))
      as libc::c_uint;
    //+ (MULTIPLE_OF_11 - 15 * SHIFT_11)
    //+ (MULTIPLE_OF_13 - 18 * SHIFT_13)
    //+ (MULTIPLE_OF_17 - 24 * SHIFT_17)
    factor = 3i32 as half_t;
    's_56: loop {
      /* The division is the most costly part of the loop.
       * On 64bit CPUs, takes at best 12 cycles, often ~20.
       */
      while N.wrapping_rem(factor as libc::c_ulonglong) == 0i32 as libc::c_ulonglong {
        /* not likely */
        N = N.wrapping_div(factor as libc::c_ulonglong);
        printf(b" %u\x00" as *const u8 as *const libc::c_char, factor);
        max_factor = isqrt_odd(N)
      }
      loop {
        if factor >= max_factor {
          break 's_56;
        }
        factor = (factor as libc::c_uint).wrapping_add(2i32 as libc::c_uint) as half_t as half_t;
        /* Rudimentary wheel sieving: skip multiples of 3, 5 and 7:
         * Every third odd number is divisible by three and thus isn't a prime:
         * 5 7 9 11 13 15 17 19 21 23 25 27 29 31 33 35 37 39 41 43 45 47...
         * ^ ^   ^  ^     ^  ^     ^  _     ^  ^     _  ^     ^  ^     ^
         * (^ = primes, _ = would-be-primes-if-not-divisible-by-5)
         * The numbers with space under them are excluded by sieve 3.
         */
        // count7--;
        // count5--;
        // count3--;
        // if (count3 && count5 && count7)
        // 	continue;
        sieve_word = sieve_word.wrapping_add(INCREMENT_EACH as libc::c_int as libc::c_uint);
        if sieve_word & MULTIPLE_DETECTED as libc::c_int as libc::c_uint == 0 {
          break;
        }
        /*
         * "factor" is multiple of 3 33% of the time (count3 reached 0),
         * else, multiple of 5 13% of the time,
         * else, multiple of 7 7.6% of the time.
         * Cumulatively, with 3,5,7 sieving we are here 54.3% of the time.
         */
        // if (count3 == 0)
        // 	count3 = 3;
        if sieve_word & MULTIPLE_OF_3 as libc::c_int as libc::c_uint != 0 {
          sieve_word = sieve_word.wrapping_sub((SHIFT_3 as libc::c_int * 3i32) as libc::c_uint)
        }
        // if (count5 == 0)
        // 	count5 = 5;
        if sieve_word & MULTIPLE_OF_5 as libc::c_int as libc::c_uint != 0 {
          sieve_word = sieve_word.wrapping_sub((SHIFT_5 as libc::c_int * 5i32) as libc::c_uint)
        }
        // if (count7 == 0)
        // 	count7 = 7;
        if sieve_word & MULTIPLE_OF_7 as libc::c_int as libc::c_uint != 0 {
          sieve_word = sieve_word.wrapping_sub((SHIFT_7 as libc::c_int * 7i32) as libc::c_uint)
        }
      }
    }
  }
  if N > 1i32 as libc::c_ulonglong {
    printf(b" %llu\x00" as *const u8 as *const libc::c_char, N);
  }
  bb_putchar('\n' as i32);
}
unsafe extern "C" fn factorize_numstr(mut numstr: *const libc::c_char) {
  let mut N: wide_t = 0;
  /* Leading + is ok (coreutils compat) */
  if *numstr as libc::c_int == '+' as i32 {
    numstr = numstr.offset(1)
  }
  N = bb_strtoull(numstr, 0 as *mut *mut libc::c_char, 10i32);
  if *bb_errno != 0 {
    bb_show_usage();
  }
  printf(b"%llu:\x00" as *const u8 as *const libc::c_char, N);
  factorize(N);
}
#[no_mangle]
pub unsafe extern "C" fn factor_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  // // coreutils has undocumented option ---debug (three dashes)
  //getopt32(argv, "");
  //argv += optind;
  argv = argv.offset(1);
  if (*argv).is_null() {
    loop
    /* Read from stdin, several numbers per line are accepted */
    {
      let mut numstr: *mut libc::c_char = 0 as *mut libc::c_char;
      let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
      line = xmalloc_fgetline(stdin);
      if line.is_null() {
        return 0i32;
      }
      numstr = line;
      loop {
        let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
        numstr = skip_whitespace(numstr);
        if *numstr.offset(0) == 0 {
          break;
        }
        end = skip_non_whitespace(numstr);
        if *end as libc::c_int != '\u{0}' as i32 {
          let fresh0 = end;
          end = end.offset(1);
          *fresh0 = '\u{0}' as i32 as libc::c_char
        }
        factorize_numstr(numstr);
        numstr = end
      }
      free(line as *mut libc::c_void);
    }
  }
  loop {
    /* Leading spaces are ok (coreutils compat) */
    factorize_numstr(skip_whitespace(*argv));
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  return 0i32;
}

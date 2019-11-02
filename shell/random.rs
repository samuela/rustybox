use libc;
extern "C" {
  #[no_mangle]
  fn getpid() -> __pid_t;
  #[no_mangle]
  fn monotonic_us() -> libc::c_ulonglong;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct random_t {
  pub galois_LFSR: int32_t,
  pub LCG: uint32_t,
  pub xs64_x: uint32_t,
  pub xs64_y: uint32_t,
}
pub const a: C2RustUnnamed = 2;
pub const b: C2RustUnnamed = 7;
pub const c: C2RustUnnamed = 3;
pub const MASK: C2RustUnnamed_0 = 2147483659;
pub type C2RustUnnamed = libc::c_uint;
pub type C2RustUnnamed_0 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn next_random(mut rnd: *mut random_t) -> uint32_t {
  /* Galois LFSR parameter:
   * Taps at 32 31 29 1:
   */
  /* Another example - taps at 32 31 30 10: */
  /* enum { MASK = 0x00400007 }; */
  /* Xorshift parameters:
   * Choices for a,b,c: 10,13,10; 8,9,22; 2,7,3; 23,3,24
   * (given by algorithm author)
   */
  let mut t: uint32_t = 0;
  if (*rnd).galois_LFSR == 0i32 {
    /* Can use monotonic_ns() for better randomness but for now
     * it is not used anywhere else in busybox... so avoid bloat
     */
    (*rnd).xs64_x = getpid() as uint32_t;
    (*rnd).galois_LFSR = (*rnd).xs64_x as int32_t;
    (*rnd).xs64_y = monotonic_us() as uint32_t;
    (*rnd).LCG = (*rnd).xs64_y
  }
  /* LCG: period of 2^32, but quite weak:
   * bit 0 alternates beetween 0 and 1 (pattern of length 2)
   * bit 1 has a repeating pattern of length 4
   * bit 2 has a repeating pattern of length 8
   * etc...
   */
  (*rnd).LCG = (1664525i32 as libc::c_uint)
    .wrapping_mul((*rnd).LCG)
    .wrapping_add(1013904223i32 as libc::c_uint);
  /* Galois LFSR:
   * period of 2^32-1 = 3 * 5 * 17 * 257 * 65537.
   * Successive values are right-shifted one bit
   * and possibly xored with a sparse constant.
   */
  t = ((*rnd).galois_LFSR << 1i32) as uint32_t;
  if (*rnd).galois_LFSR < 0i32 {
    /* if we just shifted 1 out of msb... */
    t ^= MASK as libc::c_uint
  }
  (*rnd).galois_LFSR = t as int32_t;
  loop
  /* http://en.wikipedia.org/wiki/Xorshift
   * Moderately good statistical properties:
   * fails the following "dieharder -g 200 -a" tests:
   *       diehard_operm5|   0
   *         diehard_oqso|   0
   * diehard_count_1s_byt|   0
   *     diehard_3dsphere|   3
   *      diehard_squeeze|   0
   *         diehard_runs|   0
   *         diehard_runs|   0
   *        diehard_craps|   0
   *        diehard_craps|   0
   * rgb_minimum_distance|   3
   * rgb_minimum_distance|   4
   * rgb_minimum_distance|   5
   *     rgb_permutations|   3
   *     rgb_permutations|   4
   *     rgb_permutations|   5
   *         dab_filltree|  32
   *         dab_filltree|  32
   *         dab_monobit2|  12
   */
  {
    t = (*rnd).xs64_x ^ (*rnd).xs64_x << a as libc::c_int;
    (*rnd).xs64_x = (*rnd).xs64_y;
    (*rnd).xs64_y = (*rnd).xs64_y ^ (*rnd).xs64_y >> c as libc::c_int ^ t ^ t >> b as libc::c_int;
    /*
     * Period 2^64-1 = 2^32+1 * 2^32-1 has a common divisor with Galois LFSR.
     * By skipping two possible states (0x1 and 0x2) we reduce period to
     * 2^64-3 = 13 * 3889 * 364870227143809 which has no common divisors:
     */
    if !((*rnd).xs64_y == 0i32 as libc::c_uint && (*rnd).xs64_x <= 2i32 as libc::c_uint) {
      break;
    }
  }
  /* Combined LCG + Galois LFSR rng has 2^32 * 2^32-1 period.
   * Strength:
   * individually, both are extremely weak cryptographycally;
   * when combined, they fail the following "dieharder -g 200 -a" tests:
   *     diehard_rank_6x8|   0
   *         diehard_oqso|   0
   *          diehard_dna|   0
   * diehard_count_1s_byt|   0
   *          rgb_bitdist|   2
   *         dab_monobit2|  12
   *
   * Combining them with xorshift-64 increases period to
   * 2^32 * 2^32-1 * 2^64-3
   * which is about 2^128, or in base 10 ~3.40*10^38.
   * Strength of the combination:
   * passes all "dieharder -g 200 -a" tests.
   *
   * Combining with subtraction and addition is just for fun.
   * It does not add meaningful strength, could use xor operation instead.
   */
  t = ((*rnd).galois_LFSR as libc::c_uint)
    .wrapping_sub((*rnd).LCG)
    .wrapping_add((*rnd).xs64_y);
  /* bash compat $RANDOM range: */
  return t & 0x7fffi32 as libc::c_uint;
}

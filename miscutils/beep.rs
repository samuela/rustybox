use libc;
use libc::getopt;
use libc::ioctl;
extern "C" {
  #[no_mangle]
  static mut optarg: *mut libc::c_char;

  #[no_mangle]
  fn usleep(__useconds: useconds_t) -> libc::c_int;

}
use libc::useconds_t;
pub type uintptr_t = libc::c_ulong;
pub unsafe fn beep_main(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut speaker: libc::c_int = crate::libbb::get_console::get_console_fd_or_die(); /* for compiler */
  let mut tickrate_div_freq: libc::c_uint = 0;
  tickrate_div_freq = tickrate_div_freq;
  let mut length: libc::c_uint = 0;
  length = length;
  let mut delay: libc::c_uint = 0;
  delay = delay;
  let mut rep: libc::c_uint = 0;
  rep = rep;
  let mut c: libc::c_int = 0;
  c = 'n' as i32;
  while c != -1i32 {
    if c == 'n' as i32 {
      tickrate_div_freq = (1193180i32 / 4000i32) as libc::c_uint;
      length = 30i32 as libc::c_uint;
      delay = 0 as libc::c_uint;
      rep = 1i32 as libc::c_uint
    }
    c = getopt(
      argc,
      argv,
      b"f:l:d:r:n\x00" as *const u8 as *const libc::c_char,
    );
    /* TODO: -s, -c:
     * pipe stdin to stdout, but also beep after each line (-s) or char (-c)
     */
    match c {
      102 => {
        /* TODO: what "-f 0" should do? */
        tickrate_div_freq =
          (1193180i32 as libc::c_uint).wrapping_div(crate::libbb::xatonum::xatou(optarg))
      }
      108 => length = crate::libbb::xatonum::xatou(optarg),
      100 => {
        /* TODO:
         * -d N, -D N
         * specify a delay of N milliseconds between repetitions.
         * -d specifies that this delay should only occur between beeps,
         * that is, it should not occur after the last repetition.
         * -D indicates that the delay should occur after every repetition
         */
        delay = crate::libbb::xatonum::xatou(optarg)
      }
      114 => rep = crate::libbb::xatonum::xatou(optarg),
      110 | -1 => {
        while rep != 0 {
          //bb_error_msg("rep[%d] freq=%d, length=%d, delay=%d", rep, freq, length, delay);
          crate::libbb::xfuncs_printf::bb_xioctl(
            speaker,
            0x4b2fi32 as libc::c_uint,
            tickrate_div_freq as uintptr_t as *mut libc::c_void,
            b"KIOCSOUND\x00" as *const u8 as *const libc::c_char,
          );
          usleep((1000i32 as libc::c_uint).wrapping_mul(length));
          ioctl(speaker, 0x4b2fi32 as libc::c_ulong, 0 as *mut libc::c_void);
          rep = rep.wrapping_sub(1);
          if rep != 0 {
            usleep((1000i32 as libc::c_uint).wrapping_mul(delay));
          }
        }
      }
      _ => {
        crate::libbb::appletlib::bb_show_usage();
      }
    }
  }
  return 0;
}
/*
 * so, e.g. Beethoven's 9th symphony "Ode an die Freude" would be
 * something like:
a=$((220*3))
b=$((247*3))
c=$((262*3))
d=$((294*3))
e=$((329*3))
f=$((349*3))
g=$((392*3))
#./beep -f$d -l200 -r2 -n -f$e -l100 -d 10 -n -f$c -l400 -f$g -l200
./beep -f$e -l200 -r2 \
        -n -d 100 -f$f -l200 \
        -n -f$g -l200 -r2 \
        -n -f$f -l200 \
        -n -f$e -l200 \
        -n -f$d -l200 \
        -n -f$c -l200 -r2 \
        -n -f$d -l200 \
        -n -f$e -l200 \
        -n -f$e -l400 \
        -n -f$d -l100 \
        -n -f$d -l200 \
*/

use libc;
use libc::free;

use libc::time_t;

extern "C" {
  #[no_mangle]
  fn sleep(__seconds: libc::c_uint) -> libc::c_uint;

  #[no_mangle]
  fn time(__timer: *mut time_t) -> time_t;
}

#[no_mangle]
pub unsafe extern "C" fn bb_do_delay(mut seconds: libc::c_int) {
  let mut start: time_t = 0;
  let mut now: time_t = 0;
  start = time(0 as *mut time_t);
  loop {
    sleep(seconds as libc::c_uint);
    now = time(0 as *mut time_t);
    if !(now - start < seconds as libc::c_long) {
      break;
    }
  }
}

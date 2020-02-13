use crate::libbb::default_error_retval::xfunc_error_retval;
use libc;

/* Keeping it separate allows to NOT pull in stdio for VERY small applets.
 * Try building busybox with only "true" enabled... */

pub static mut die_func: Option<unsafe extern "C" fn() -> ()> = None;

pub unsafe fn xfunc_die() -> ! {
  match die_func {
    Some(df) => df(),
    None => (),
  }
  ::std::process::exit(xfunc_error_retval as libc::c_int);
}

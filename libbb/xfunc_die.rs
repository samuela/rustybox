use libc;
use libc::unlink;



extern "C" {
  #[no_mangle]
  static mut xfunc_error_retval: u8;
}



/* Keeping it separate allows to NOT pull in stdio for VERY small applets.
 * Try building busybox with only "true" enabled... */

#[no_mangle]
pub static mut die_func: Option<unsafe extern "C" fn() -> ()> = None;

#[no_mangle]
pub unsafe extern "C" fn xfunc_die() -> ! {
  if die_func.is_some() {
    die_func.expect("non-null function pointer")();
  }
  ::std::process::exit(xfunc_error_retval as libc::c_int);
}

extern "C" {
  pub type test_statics;
}

/* We cheat here. It is declared as const ptr in libbb.h,
 * but here we make it live in R/W memory */
#[no_mangle]
pub static mut test_ptr_to_statics: *mut test_statics =
  0 as *const test_statics as *mut test_statics;

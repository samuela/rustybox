use libc;
extern "C" {
  pub type globals;
}

/* We cheat here. It is declared as const ptr in libbb.h,
 * but here we make it live in R/W memory */
#[no_mangle]
pub static mut ptr_to_globals: *mut globals = std::ptr::null();

pub static mut bb_errno: *mut libc::c_int = std::ptr::null();

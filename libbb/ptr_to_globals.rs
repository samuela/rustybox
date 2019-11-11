use libc;
extern "C" {
  pub type globals;
}

/* We cheat here. It is declared as const ptr in libbb.h,
 * but here we make it live in R/W memory */
#[no_mangle]
pub static mut ptr_to_globals: *mut globals = 0 as *const globals as *mut globals;

pub static mut bb_errno: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;

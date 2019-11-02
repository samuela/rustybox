extern "C" {
  pub type globals_misc;
  pub type globals_memstack;
  pub type globals_var;
}
// We cheat here. They are declared as const ptr in ash.c, but here we make them
// live in R/W memory.
#[no_mangle]
pub static mut ash_ptr_to_globals_misc: *mut globals_misc =
  0 as *const globals_misc as *mut globals_misc;

#[no_mangle]
pub static mut ash_ptr_to_globals_memstack: *mut globals_memstack =
  0 as *const globals_memstack as *mut globals_memstack;

#[no_mangle]
pub static mut ash_ptr_to_globals_var: *mut globals_var =
  0 as *const globals_var as *mut globals_var;

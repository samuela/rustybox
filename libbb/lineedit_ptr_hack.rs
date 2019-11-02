use libc;
extern "C" {
  /* vi: set sw=4 ts=4: */
  /*
   * Copyright (C) 2008 by Denys Vlasenko <vda.linux@googlemail.com>
   *
   * Licensed under GPLv2, see file LICENSE in this source tree.
   */
  pub type lineedit_statics;
}
/* We cheat here. It is declared as const ptr in libbb.h,
 * but here we make it live in R/W memory */
#[no_mangle]
pub static mut lineedit_ptr_to_statics: *mut lineedit_statics =
  0 as *const lineedit_statics as *mut lineedit_statics;

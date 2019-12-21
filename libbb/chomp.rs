use libc;
extern "C" {
  /* dmalloc will redefine these to it's own implementation. It is safe
   * to have the prototypes here unconditionally.  */
  /* After v = xrealloc_vector(v, SHIFT, idx) it's ok to use
   * at least v[idx] and v[idx+1], for all idx values.
   * SHIFT specifies how many new elements are added (1:2, 2:4, ..., 8:256...)
   * when all elements are used up. New elements are zeroed out.
   * xrealloc_vector(v, SHIFT, idx) *MUST* be called with consecutive IDXs -
   * skipping an index is a bad bug - it may miss a realloc!
   */
  //TODO: supply a pointer to char[11] buffer (avoid statics)?
  /* cp.c, mv.c, install.c depend on these values. CAREFUL when changing them! */
  /* -p */
  /* !-d */
  /* -R */
  /* -f */
  /* -i */
  /* -l */
  /* -s */
  /* -L */
  /* -H */
  /* -a = -pdR (mapped in cp.c) */
  /* -r = -dR  (mapped in cp.c) */
  /* -P = -d   (mapped in cp.c) */
  /* -v */
  /* -u */
  /* -T */
  /* --remove-destination */
  /* bit 17 skipped for "cp --parents" */
  /* cp --reflink=auto */
  /* cp --reflink[=always] */
  /*
   * Hole. cp may have some bits set here,
   * they should not affect remove_file()/copy_file()
   */
  /* NB: without FILEUTILS_RECUR in flags, it will basically "cat"
   * the source, not copy (unless "source" is a directory).
   * This makes "cp /dev/null file" and "install /dev/null file" (!!!)
   * work coreutils-compatibly. */
  /*ACTION_REVERSE      = (1 << 4), - unused */
  /* more than enough for "/dev/ttyXXX" */
  /* bb_copyfd_XX print read/write errors and return -1 if they occur */
  /* "short" copy can be detected by return value < size */
  /* this helper yells "short read!" if param is not -1 */
  /* xxxx_strip version can modify its parameter:
   * "/"        -> "/"
   * "abc"      -> "abc"
   * "abc/def"  -> "def"
   * "abc/def/" -> "def" !!
   */
  /* "abc/def/" -> "" and it never modifies 'path' */
  /* Simpler version: does not special case "/" string */
  /* NB: can violate const-ness (similarly to strchr) */

}

/*
 * Utility routines.
 *
 * Copyright (C) many different people.
 * If you wrote this, please acknowledge your work.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn chomp(mut s: *mut libc::c_char) {
  let mut lc: *mut libc::c_char = crate::libbb::last_char_is::last_char_is(s, '\n' as i32);
  if !lc.is_null() {
    *lc = '\u{0}' as i32 as libc::c_char
  };
}

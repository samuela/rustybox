use crate::librb::__compar_fn_t;
use crate::librb::size_t;
use libc;
use libc::strcmp;
extern "C" {
  #[no_mangle]
  fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
}
unsafe extern "C" fn bb_pstrcmp(a: *const libc::c_void, b: *const libc::c_void) -> libc::c_int {
  return strcmp(
    *(a as *mut *mut libc::c_char),
    *(b as *mut *mut libc::c_char),
  );
}

pub unsafe fn qsort_string_vector(mut sv: *mut *mut libc::c_char, mut count: libc::c_uint) {
  qsort(
    sv as *mut libc::c_void,
    count as size_t,
    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    Some(bb_pstrcmp),
  );
}

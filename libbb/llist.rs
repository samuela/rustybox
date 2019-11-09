use libc;
use libc::free;

use crate::librb::size_t;

extern "C" {


  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct llist_t {
  pub link: *mut llist_t,
  pub data: *mut libc::c_char,
}

/* Add data to the start of the linked list.  */
#[no_mangle]
pub unsafe extern "C" fn llist_add_to(
  mut old_head: *mut *mut llist_t,
  mut data: *mut libc::c_void,
) {
  let mut new_head: *mut llist_t =
    xmalloc(::std::mem::size_of::<llist_t>() as libc::c_ulong) as *mut llist_t;
  (*new_head).data = data as *mut libc::c_char;
  (*new_head).link = *old_head;
  *old_head = new_head;
}

/* Add data to the end of the linked list.  */
#[no_mangle]
pub unsafe extern "C" fn llist_add_to_end(
  mut list_head: *mut *mut llist_t,
  mut data: *mut libc::c_void,
) {
  while !(*list_head).is_null() {
    list_head = &mut (**list_head).link
  }
  *list_head = xzalloc(::std::mem::size_of::<llist_t>() as libc::c_ulong) as *mut llist_t;
  (**list_head).data = data as *mut libc::c_char;
  /*(*list_head)->link = NULL;*/
}

/* Remove first element from the list and return it */
#[no_mangle]
pub unsafe extern "C" fn llist_pop(mut head: *mut *mut llist_t) -> *mut libc::c_void {
  let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
  let mut temp: *mut llist_t = *head;
  if !temp.is_null() {
    data = (*temp).data as *mut libc::c_void;
    *head = (*temp).link;
    free(temp as *mut libc::c_void);
  }
  return data;
}

/* Unlink arbitrary given element from the list */
#[no_mangle]
pub unsafe extern "C" fn llist_unlink(mut head: *mut *mut llist_t, mut elm: *mut llist_t) {
  if elm.is_null() {
    return;
  }
  while !(*head).is_null() {
    if *head == elm {
      *head = (**head).link;
      break;
    } else {
      head = &mut (**head).link
    }
  }
}

/* Recursively free all elements in the linked list.  If freeit != NULL
 * call it on each datum in the list */
#[no_mangle]
pub unsafe extern "C" fn llist_free(
  mut elm: *mut llist_t,
  mut freeit: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
) {
  while !elm.is_null() {
    let mut data: *mut libc::c_void = llist_pop(&mut elm);
    if freeit.is_some() {
      freeit.expect("non-null function pointer")(data);
    }
  }
}

/* Reverse list order. */
#[no_mangle]
pub unsafe extern "C" fn llist_rev(mut list: *mut llist_t) -> *mut llist_t {
  let mut rev: *mut llist_t = 0 as *mut llist_t;
  while !list.is_null() {
    let mut next: *mut llist_t = (*list).link;
    (*list).link = rev;
    rev = list;
    list = next
  }
  return rev;
}

#[no_mangle]
pub unsafe extern "C" fn llist_find_str(
  mut list: *mut llist_t,
  mut str: *const libc::c_char,
) -> *mut llist_t {
  while !list.is_null() {
    if strcmp((*list).data, str) == 0i32 {
      break;
    }
    list = (*list).link
  }
  return list;
}

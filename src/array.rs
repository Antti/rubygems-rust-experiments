use ruby::*;

//
// pub fn rb_ary_new() -> VALUE;
// pub fn rb_ary_new_capa(capa: ::libc::c_long) -> VALUE;
// pub fn rb_ary_new_from_args(n: ::libc::c_long, ...) -> VALUE;
// pub fn rb_ary_new_from_values(n: ::libc::c_long, elts: *const VALUE)
//  -> VALUE;
// pub fn rb_ary_tmp_new(arg1: ::libc::c_long) -> VALUE;
// pub fn rb_ary_free(arg1: VALUE) -> ();
// pub fn rb_ary_modify(arg1: VALUE) -> ();
// pub fn rb_ary_freeze(arg1: VALUE) -> VALUE;
// pub fn rb_ary_shared_with_p(arg1: VALUE, arg2: VALUE) -> VALUE;
// pub fn rb_ary_aref(arg1: ::libc::c_int, arg2: *const VALUE, arg3: VALUE)
//  -> VALUE;
// pub fn rb_ary_subseq(arg1: VALUE, arg2: ::libc::c_long,
//                      arg3: ::libc::c_long) -> VALUE;
// pub fn rb_ary_store(arg1: VALUE, arg2: ::libc::c_long, arg3: VALUE) -> ();
// pub fn rb_ary_dup(arg1: VALUE) -> VALUE;
// pub fn rb_ary_resurrect(ary: VALUE) -> VALUE;
// pub fn rb_ary_to_ary(arg1: VALUE) -> VALUE;
// pub fn rb_ary_to_s(arg1: VALUE) -> VALUE;
// pub fn rb_ary_cat(arg1: VALUE, arg2: *const VALUE, arg3: ::libc::c_long)
//  -> VALUE;
// pub fn rb_ary_push(arg1: VALUE, arg2: VALUE) -> VALUE;
// pub fn rb_ary_pop(arg1: VALUE) -> VALUE;
// pub fn rb_ary_shift(arg1: VALUE) -> VALUE;
// pub fn rb_ary_unshift(arg1: VALUE, arg2: VALUE) -> VALUE;
// pub fn rb_ary_entry(arg1: VALUE, arg2: ::libc::c_long) -> VALUE;
// pub fn rb_ary_each(arg1: VALUE) -> VALUE;
// pub fn rb_ary_join(arg1: VALUE, arg2: VALUE) -> VALUE;
// pub fn rb_ary_reverse(arg1: VALUE) -> VALUE;
// pub fn rb_ary_rotate(arg1: VALUE, arg2: ::libc::c_long) -> VALUE;
// pub fn rb_ary_sort(arg1: VALUE) -> VALUE;
// pub fn rb_ary_sort_bang(arg1: VALUE) -> VALUE;
// pub fn rb_ary_delete(arg1: VALUE, arg2: VALUE) -> VALUE;
// pub fn rb_ary_delete_at(arg1: VALUE, arg2: ::libc::c_long) -> VALUE;
// pub fn rb_ary_clear(arg1: VALUE) -> VALUE;
// pub fn rb_ary_plus(arg1: VALUE, arg2: VALUE) -> VALUE;
// pub fn rb_ary_concat(arg1: VALUE, arg2: VALUE) -> VALUE;
// pub fn rb_ary_assoc(arg1: VALUE, arg2: VALUE) -> VALUE;
// pub fn rb_ary_rassoc(arg1: VALUE, arg2: VALUE) -> VALUE;
// pub fn rb_ary_includes(arg1: VALUE, arg2: VALUE) -> VALUE;
// pub fn rb_ary_cmp(arg1: VALUE, arg2: VALUE) -> VALUE;
// pub fn rb_ary_replace(copy: VALUE, orig: VALUE) -> VALUE;
// pub fn rb_get_values_at(arg1: VALUE, arg2: ::libc::c_long,
//                         arg3: ::libc::c_int, arg4: *const VALUE,
//                         arg5:
//                             ::std::option::Option<extern "C" fn(arg1:
//                                                                     VALUE,
//                                                                 arg2:
//                                                                     ::libc::c_long)
//                                                       -> VALUE>) -> VALUE;
// pub fn rb_ary_resize(ary: VALUE, len: ::libc::c_long) -> VALUE;


#[derive(Debug)]
pub struct Array {
    val: VALUE
}

impl Array {
    pub fn new() -> Self {
        Array { val: unsafe { rb_ary_new() } }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Array { val: unsafe { rb_ary_new_capa(capacity as i64) } }
    }

    pub fn to_value(&self) -> VALUE {
        self.val
    }

    pub fn push(&mut self, value: VALUE) -> VALUE {
        unsafe { rb_ary_push(self.val, value) }
    }

    pub fn pop(&mut self) -> VALUE {
        unsafe { rb_ary_pop(self.val) }
    }

    pub fn shift(&mut self, value: VALUE) -> VALUE {
        unsafe { rb_ary_shift(self.val) }
    }

    pub fn unshift(&mut self, value: VALUE) -> VALUE {
        unsafe { rb_ary_unshift(self.val, value) }
    }
}

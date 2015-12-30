use ruby::*;
use std::ops;
use super::{RubyValue, cast_str};

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

pub struct ArrayIterator {
    arr: Array,
    initial_size: usize,
    current_idx: usize
}

impl Array {
    pub fn new() -> Self {
        Array { val: unsafe { rb_ary_new() } }
    }

    pub fn from_value(value: VALUE) -> Self {
        Array { val: value }
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

    pub fn len(&self) -> usize {
        let value = RubyValue::from_value(unsafe { rb_funcall(self.val, rb_intern(cast_str("size\x00")), 0) } );
        match value {
            RubyValue::Fixnum(len) => len as usize,
            _ => panic!("Unexpected result of array.size")
        }
    }

    pub fn entry(&self, index: usize) -> VALUE {
        unsafe { rb_ary_entry(self.to_value(), index as i64) }
    }
}

impl ArrayIterator {
    fn new(arr: Array) -> Self {
        let len = arr.len();
        ArrayIterator { arr: arr, initial_size: len, current_idx: 0 }
    }
}

impl Iterator for ArrayIterator {
    type Item = VALUE;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx < self.initial_size {
            let val = self.arr.entry(self.current_idx);
            self.current_idx += 1;
            Some(val)
        } else {
            None
        }
    }
}

impl IntoIterator for Array {
    type Item = VALUE;
    type IntoIter = ArrayIterator;
    fn into_iter(self) -> Self::IntoIter {
        ArrayIterator::new(self)
    }
}

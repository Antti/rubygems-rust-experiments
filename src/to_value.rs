use super::ruby::{self, VALUE};
use super::Nil;
use super::macros::*;
use std::ffi::CString;

pub trait ToValue {
    fn to_value(&self) -> VALUE;
}

impl ToValue for VALUE {
    fn to_value(&self) -> VALUE {
        *self
    }
}

impl ToValue for bool {
    fn to_value(&self) -> VALUE {
        match *self {
            true => ruby::RUBY_Qtrue as VALUE,
            false => ruby::RUBY_Qfalse as VALUE
        }
    }
}

impl ToValue for i64 {
    fn to_value(&self) -> VALUE {
        INT2FIX(*self)
    }
}

impl ToValue for i32 {
    fn to_value(&self) -> VALUE {
        INT2FIX(*self as i64)
    }
}

impl ToValue for f64 {
    fn to_value(&self) -> VALUE {
        unsafe { ruby::rb_float_new(*self) }
    }
}

impl ToValue for String {
    fn to_value(&self) -> VALUE {
        unsafe { ruby::rb_str_new_cstr(CString::new(self.clone()).unwrap().as_ptr() as *const i8) }
    }
}

impl ToValue for Nil {
    fn to_value(&self) -> VALUE {
        ruby::RUBY_Qnil as VALUE
    }
}

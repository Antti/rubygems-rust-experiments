use super::RubyType;
use super::ruby::{self, VALUE};
use super::Nil;

pub trait FromValue: Sized {
    // TODO: Use associated const when available and support access from the trait itself, so we can check type
    fn from_value(value: VALUE) -> Option<Self>;
    fn from_value_unchecked(value: VALUE) -> Self;
}

impl FromValue for VALUE {
    fn from_value(val: VALUE) -> Option<Self> {
        Some(val)
    }

    fn from_value_unchecked(val: VALUE) -> Self {
        val
    }
}

// Core type from value
impl FromValue for bool {
    fn from_value(value: VALUE) -> Option<Self> {
        match RubyType::from_value(value)  {
            RubyType::False | RubyType::True => Some(FromValue::from_value_unchecked(value)),
            _ => None
        }
    }
    fn from_value_unchecked(value: VALUE) -> Self {
        match RubyType::from_value(value)  {
            RubyType::False => false,
            RubyType::True => true,
            _ => panic!("Unhandled value")
        }
    }
}


impl FromValue for i64 {
    fn from_value_unchecked(value: VALUE) -> Self {
        unsafe { ruby::rb_fix2int(value) }
    }
    fn from_value(value: VALUE) -> Option<Self> {
        match RubyType::from_value(value)  {
            RubyType::Fixnum => Some(FromValue::from_value_unchecked(value)),
            _ => None
        }
    }
}

impl FromValue for i32 {
    fn from_value_unchecked(value: VALUE) -> Self {
        unsafe { ruby::rb_num2short(value) as i32 }
    }
    fn from_value(value: VALUE) -> Option<Self> {
        match RubyType::from_value(value)  {
            RubyType::Fixnum => Some(FromValue::from_value_unchecked(value)),
            _ => None
        }
    }
}

impl FromValue for f64 {
    fn from_value_unchecked(value: VALUE) -> Self {
        unsafe { ruby::rb_float_value(value) }
    }
    fn from_value(value: VALUE) -> Option<Self> {
        match RubyType::from_value(value)  {
            RubyType::Float => Some(FromValue::from_value_unchecked(value)),
            _ => None
        }
    }
}

impl FromValue for String {
    fn from_value(value: VALUE) -> Option<Self> {
        match RubyType::from_value(value)  {
            RubyType::String => Some(FromValue::from_value_unchecked(value)),
            _ => None
        }
    }
    fn from_value_unchecked(mut value: VALUE) -> Self {
        use std::slice;
        unsafe {
            let strlen = ruby::rb_str_strlen(value) as usize;
            let ptr = ruby::rb_string_value_ptr(&mut value) as *const u8;
            let buf = slice::from_raw_parts(ptr, strlen);
            String::from_utf8_lossy(buf).into_owned()
        }
    }
}

impl FromValue for Nil {
    fn from_value_unchecked(_value: VALUE) -> Self {
        Nil
    }
    fn from_value(value: VALUE) -> Option<Self> {
        match RubyType::from_value(value)  {
            RubyType::Nil => Some(FromValue::from_value_unchecked(value)),
            _ => None
        }
    }
}

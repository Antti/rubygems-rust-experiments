#![feature(libc)]
extern crate libc;

#[allow(dead_code, non_upper_case_globals, non_camel_case_types, non_snake_case)]
mod ruby;
mod array;
mod hash;
#[allow(dead_code, non_upper_case_globals, non_camel_case_types, non_snake_case)]
mod macros;
mod from_value;
mod to_value;
mod ruby_type;

use ruby::*;
use macros::*;
pub use array::Array;
pub use hash::Hash;
pub use from_value::FromValue;
pub use to_value::ToValue;
pub use ruby_type::RubyType;

use std::ffi::{CString, CStr};
use std::fmt;

pub struct InspectValue(VALUE);

impl fmt::Debug for InspectValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ruby_type = RubyType::from_value(self.0);
        match ruby_type {
            RubyType::Nil => write!(f, "Nil"),
            RubyType::Fixnum => write!(f, "Fixnum({})", i64::from_value_unchecked(self.0)),
            RubyType::Float => write!(f, "Float({})", f64::from_value_unchecked(self.0)),
            RubyType::True | RubyType::False => write!(f, "Bool({})", bool::from_value_unchecked(self.0)),
            RubyType::String => write!(f, "String({})", String::from_value_unchecked(self.0)),
            // RubyType::Symbol => write!(f, "Symbol(_not_implemented_yet_)"),
            RubyType::Array => write!(f, "{:?}", Array::from_value_unchecked(self.0)),
            RubyType::Hash => write!(f, "{:?}", Hash::from_value_unchecked(self.0)),
            _ => write!(f, "Object({})", String::from_value_unchecked(unsafe { rb_inspect(self.0) }) )
        }
    }
}


#[derive(Debug)]
pub struct Nil;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Init_test_rust() {
  let my_mod = rb_define_module(cast_str("TestRust\x00"));
  ruby_define_singleton_method(my_mod, "foo", foo, 1);
}

#[no_mangle]
pub extern "C" fn foo(_this: VALUE, arg: VALUE) -> VALUE { //argc: usize, argv: *const VALUE, this: VALUE
    ruby_eval_string("puts 'hello world from rust'");
    let mut ary = Array::new();
    ary.push(Nil);
    ary.push(true);
    ary.push(25);
    ary.push(rb_type(arg) as i64);
    println!("Type of arg: {:?}", RubyType::from_value(arg));
    // println!("Arg value: {:?}", RubyValue::from_value(arg));
    if let Some(arr) = Array::from_value(arg) {
        for val in arr {
            println!("Array item: {:?}", InspectValue(val))
        }
    }

    if let Some(hash) = Hash::from_value(arg) {
        println!("Hash len: {:?}", i32::from_value(hash.len()));
        println!("Hash keys: {:?}", hash.keys().into_iter().map(|itm| InspectValue(itm)).collect::<Vec<_>>() );
    }

    println!("Arg class name: {:?}", unsafe { CStr::from_ptr(rb_obj_classname(arg)) } );
    println!("Arg class name manual: {:?}", unsafe { String::from_value(rb_class_name(rb_funcall(arg, rb_intern(cast_str("class\x00")), 0))) } );
    ary.to_value()
}

fn ruby_define_singleton_method(module: VALUE, name: &str, func: extern "C" fn(VALUE, VALUE) -> VALUE, argc: i32) {
    use std::mem::transmute;
    let buf = CString::new(name).unwrap();
    unsafe { rb_define_singleton_method(module, buf.as_ptr() as *const i8, Some(transmute(func)), argc); }
}

fn ruby_eval_string(program: &str) {
    let buf = CString::new(program).unwrap();
    unsafe { rb_eval_string(buf.as_ptr() as *const i8) };
}

#[inline(always)]
fn cast_str(string: &'static str) -> *const i8 {
    string.as_ptr() as *const i8
}

#[inline(always)]
fn rb_type(obj: VALUE) -> u64
{
    if RB_IMMEDIATE_P(obj) {
    	if RB_FIXNUM_P(obj) { return ruby::RUBY_T_FIXNUM as u64; }
        if RB_FLONUM_P(obj) { return ruby::RUBY_T_FLOAT as u64; }
        if obj == ruby::RUBY_Qtrue as u64 { return ruby::RUBY_T_TRUE as u64; }
    	if RB_STATIC_SYM_P(obj) { return ruby::RUBY_T_SYMBOL as u64; }
    	if obj == ruby::RUBY_Qundef as u64 { return ruby::RUBY_T_UNDEF as u64; }
    } else if !RTEST(obj) {
    	if obj == RUBY_Qnil as u64  { return ruby::RUBY_T_NIL as u64; }
    	if obj == RUBY_Qfalse as u64 { return ruby::RUBY_T_FALSE as u64; }
    }
    return RB_BUILTIN_TYPE(obj);
}

#![feature(libc)]

extern crate libc;

#[allow(dead_code, non_upper_case_globals, non_camel_case_types)]
mod ruby;
mod array;
mod macros;

use ruby::*;
use macros::*;
use array::Array;
use std::mem::transmute;
use std::ffi::{CString, CStr};

#[derive(Debug)]
enum RubyType {
    Nil, // T_NIL
    Object, // T_OBJECT
    Class, // T_CLASS
    Module, // T_MODULE
    Float, //T_FLOAT
    String, // T_STRING
    Regexp, // T_REGEXP
    Array, // T_ARRAY
    Hash, // T_HASH
    Struct, // T_STRUCT
    Bignum, // T_BIGNUM
    Fixnum, // T_FIXNUM
    Complex, // T_COMPLEX
    Rational, // T_RATIONAL
    File, // T_FILE
    True, // T_TRUE
    False, // T_FALSE
    Data, // T_DATA
    Symbol //T_SYMBOL
}

// Basic types:
// ruby::RUBY_T_FALSE => RubyType::False,
// ruby::RUBY_T_FIXNUM => RubyType::Fixnum,
// ruby::RUBY_T_FLOAT => RubyType::Float,
// ruby::RUBY_T_NIL => RubyType::Nil,
// ruby::RUBY_T_SYMBOL => RubyType::Symbol,
// ruby::RUBY_T_TRUE => RubyType::True,
// ruby::RUBY_T_UNDEF => RubyType::Undef,


impl RubyType {
    fn from_value(obj: VALUE) -> Self {
        match rb_type(obj) as u32 {
            ruby::RUBY_T_ARRAY => RubyType::Array,
            ruby::RUBY_T_BIGNUM => RubyType::Bignum,
            ruby::RUBY_T_CLASS => RubyType::Class,
            ruby::RUBY_T_COMPLEX => RubyType::Complex,
            ruby::RUBY_T_DATA => RubyType::Data,
            ruby::RUBY_T_FALSE => RubyType::False,
            ruby::RUBY_T_FILE => RubyType::File,
            ruby::RUBY_T_FIXNUM => RubyType::Fixnum,
            ruby::RUBY_T_FLOAT => RubyType::Float,
            ruby::RUBY_T_HASH => RubyType::Hash,
            ruby::RUBY_T_MODULE => RubyType::Module,
            ruby::RUBY_T_NIL => RubyType::Nil,
            ruby::RUBY_T_RATIONAL => RubyType::Rational,
            ruby::RUBY_T_REGEXP => RubyType::Regexp,
            ruby::RUBY_T_STRING => RubyType::String,
            ruby::RUBY_T_STRUCT => RubyType::Struct,
            ruby::RUBY_T_SYMBOL => RubyType::Symbol,
            ruby::RUBY_T_TRUE => RubyType::True,
            _ => RubyType::Object,
        }
    }
}

#[derive(Debug)]
enum RubyValue {
    Nil, // T_NIL
    Float(f64), //T_FLOAT
    Bool(bool), // T_TRUE, T_FALSE
    String(String), // T_STRING
    Array(Array), // T_ARRAY
    Hash, // T_HASH
    Struct, // T_STRUCT
    Bignum, // T_BIGNUM
    Fixnum(i64), // T_FIXNUM
    Complex, // T_COMPLEX
    Rational, // T_RATIONAL
    File, // T_FILE
    Symbol, //T_SYMBOL
    Object(VALUE)
}

impl RubyValue {
    fn from_value(mut value: VALUE) -> Self {
        use std::slice;
        match RubyType::from_value(value) {
            RubyType::False => RubyValue::Bool(false),
            RubyType::True => RubyValue::Bool(true),
            RubyType::String => {
                unsafe {
                    let strlen = ruby::rb_str_strlen(value) as usize;
                    let ptr = ruby::rb_string_value_ptr(&mut value) as *const u8;
                    let buf = slice::from_raw_parts(ptr, strlen);
                    RubyValue::String(String::from_utf8_lossy(buf).into_owned())
                }
            },
            RubyType::Fixnum => RubyValue::Fixnum(unsafe { ruby::rb_fix2int(value) }),
            RubyType::Float => RubyValue::Float(unsafe { ruby::rb_float_value(value) }),
            RubyType::Nil => RubyValue::Nil,
            RubyType::Array => RubyValue::Array(Array::from_value(value)),
            _ => RubyValue::Object(value)
        }
    }

    fn to_value(&self) -> VALUE {
        match *self {
            _ => unimplemented!()
        }
    }
}


#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Init_test_rust() {
  let my_mod = rb_define_module(cast_str("TestRust\x00"));
  ruby_define_singleton_method(my_mod, "foo", foo, 1);
}

#[no_mangle]
pub extern "C" fn foo(this: VALUE, arg: VALUE) -> VALUE { //this: VALUE, argc: usize, argv: *const VALUE
    ruby_eval_string("puts 'hello world from rust'");
    let mut ary = Array::new();
    ary.push(RUBY_Qnil as VALUE);
    ary.push(RUBY_Qtrue as VALUE);
    ary.push(INT2FIX(25));
    ary.push(INT2FIX(rb_type(arg) as u64));
    println!("Type of arg: {:?}", RubyType::from_value(arg));
    println!("Arg value: {:?}", RubyValue::from_value(arg));
    if let RubyValue::Array(arr) = RubyValue::from_value(arg) {
        for val in arr {
            println!("Array item: {:?}", RubyValue::from_value(val))
        }
    }
    println!("Arg class name: {:?}", unsafe { CStr::from_ptr(rb_obj_classname(arg)) } );
    println!("Arg class name manual: {:?}", unsafe { RubyValue::from_value(rb_class_name(rb_funcall(arg, rb_intern(cast_str("class\x00")), 0))) } );
    ary.to_value()
}

fn ruby_define_singleton_method(module: VALUE, name: &str, func: extern "C" fn(VALUE, VALUE) -> VALUE , argc: i32) {
    let buf = CString::new(name).unwrap();
    unsafe { rb_define_singleton_method(module, buf.as_ptr() as *const i8, Some(transmute(func)), argc); }
}

fn ruby_eval_string(program: &str) {
    let buf = CString::new(program).unwrap();
    unsafe { rb_eval_string(buf.as_ptr() as *const i8) };
}

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

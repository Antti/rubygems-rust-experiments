#![feature(libc)]
#![feature(associated_consts)]

extern crate libc;

#[allow(dead_code, non_upper_case_globals, non_camel_case_types)]
mod ruby;
mod array;
mod hash;
#[allow(dead_code, non_upper_case_globals, non_camel_case_types)]
mod macros;

use ruby::*;
use macros::*;
use array::Array;
use hash::Hash;
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
    Fixnum(i64), // T_FIXNUM
    Float(f64), //T_FLOAT
    Bool(bool), // T_TRUE, T_FALSE
    String(String), // T_STRING
    Array(Array), // T_ARRAY
    Hash(Hash), // T_HASH
    // Struct, // T_STRUCT
    // Bignum, // T_BIGNUM
    // Complex, // T_COMPLEX
    // Rational, // T_RATIONAL
    // File, // T_FILE
    Symbol(VALUE), //T_SYMBOL
    Object(VALUE)
}

pub trait ToValue {
    fn to_value(&self) -> VALUE;
}

pub trait FromValue: Sized {
    const RUBY_TYPE: RubyType = RubyType::Object;
    fn from_value(value: VALUE) -> Option<Self> {
        match RubyType::from_value(value) {
            Self::RUBY_TYPE => Some(FromValue::from_value_unchecked(value)),
            _ => None
        }
    }
    fn from_value_unchecked(value: VALUE) -> Self;
}
//
// impl FromValue for RubyValue {
//     fn from_value_unchecked(value: VALUE) -> Self {
//         use std::slice;
//         match RubyType::from_value(value) {
//             RubyType::False => RubyValue::Bool(false),
//             RubyType::True => RubyValue::Bool(true),
//             RubyType::Nil => RubyValue::Nil,
//             RubyType::Fixnum => RubyValue::Fixnum(unsafe { ruby::rb_fix2int(value) }),
//             RubyType::Float => RubyValue::Float(unsafe { ruby::rb_float_value(value) }),
//             RubyType::Symbol => RubyValue::Symbol(value),
//             RubyType::Array => RubyValue::Array(Array::from_value(value)),
//             RubyType::Hash => RubyValue::Hash(Hash::from_value(value)),
//             RubyType::String => {
//                 unsafe {
//                     let strlen = ruby::rb_str_strlen(value) as usize;
//                     let ptr = ruby::rb_string_value_ptr(&mut value) as *const u8;
//                     let buf = slice::from_raw_parts(ptr, strlen);
//                     RubyValue::String(String::from_utf8_lossy(buf).into_owned())
//                 }
//             },
//             _ => RubyValue::Object(value)
//         }
//     }
//     fn from_value(mut value: VALUE) -> Option<Self> {
//         Some(FromValue::from_value_unchecked(value))
//     }
// }

// impl ToValue for RubyValue {
//     fn to_value(&self) -> VALUE {
//         match *self {
//             RubyValue::Bool(true) => RUBY_Qtrue as VALUE,
//             RubyValue::Bool(false) => RUBY_Qfalse as VALUE,
//             RubyValue::Nil => RUBY_Qnil as VALUE,
//             RubyValue::Fixnum(num) => INT2FIX(num),
//             RubyValue::Float(num) => unsafe { rb_float_new(num) },
//             RubyValue::Symbol(val) => val,
//             RubyValue::Array(ref arr) => arr.to_value(),
//             RubyValue::Hash(ref hash) => hash.to_value(),
//             RubyValue::String(ref s) => unsafe { rb_str_new_cstr(CString::new(s.clone()).unwrap().as_ptr() as *const i8) },
//             RubyValue::Object(val) => val,
//         }
//     }
// }

impl FromValue for VALUE {
    fn from_value(val: VALUE) -> Option<Self> {
        Some(val)
    }

    fn from_value_unchecked(val: VALUE) -> Self {
        val
    }
}

impl ToValue for VALUE {
    fn to_value(&self) -> VALUE {
        *self
    }
}


// Core type from value

impl FromValue for bool {
    fn from_value(value: VALUE) -> Option<Self> {
        match RubyType::from_value(value)  {
            RubyType::False | RubyType::False => Some(FromValue::from_value_unchecked(value)),
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
    const RUBY_TYPE: RubyType = RubyType::Fixnum;
    fn from_value_unchecked(value: VALUE) -> Self {
        unsafe { ruby::rb_fix2int(value) }
    }
}

impl FromValue for f64 {
    const RUBY_TYPE: RubyType = RubyType::Float;
    fn from_value_unchecked(value: VALUE) -> Self {
        unsafe { ruby::rb_float_value(value) }
    }
}

impl FromValue for String {
    const RUBY_TYPE: RubyType = RubyType::String;
    fn from_value_unchecked(value: VALUE) -> Self {
        use std::slice;
        unsafe {
            let strlen = ruby::rb_str_strlen(value) as usize;
            let ptr = ruby::rb_string_value_ptr(&mut value) as *const u8;
            let buf = slice::from_raw_parts(ptr, strlen);
            String::from_utf8_lossy(buf).into_owned()
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
pub extern "C" fn foo(_this: VALUE, arg: VALUE) -> VALUE { //argc: usize, argv: *const VALUE, this: VALUE
    ruby_eval_string("puts 'hello world from rust'");
    let mut ary = Array::new();
    ary.push(RubyValue::Nil);
    ary.push(RubyValue::Bool(true));
    ary.push(INT2FIX(25));
    ary.push(INT2FIX(rb_type(arg) as i64));
    println!("Type of arg: {:?}", RubyType::from_value(arg));
    println!("Arg value: {:?}", RubyValue::from_value(arg));
    if let Some(RubyValue::Array(arr)) = RubyValue::from_value(arg) {
        for val in arr {
            println!("Array item: {:?}", RubyValue::from_value(val))
        }
    }

    if let Some(RubyValue::Hash(hash)) = RubyValue::from_value(arg) {
        println!("Hash len: {:?}", RubyValue::from_value(hash.len()));
        println!("Hash keys: {:?}", hash.keys().into_iter().map(|itm| RubyValue::from_value(itm)).collect::<Vec<_>>() );
    }

    println!("Arg class name: {:?}", unsafe { CStr::from_ptr(rb_obj_classname(arg)) } );
    println!("Arg class name manual: {:?}", unsafe { RubyValue::from_value(rb_class_name(rb_funcall(arg, rb_intern(cast_str("class\x00")), 0))) } );
    ary.to_value()
}

fn ruby_define_singleton_method(module: VALUE, name: &str, func: extern "C" fn(VALUE, VALUE) -> VALUE, argc: i32) {
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

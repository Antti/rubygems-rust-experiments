#![feature(libc)]
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
use std::fmt;

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

struct WrapValue(VALUE);

impl fmt::Debug for WrapValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ruby_type = RubyType::from_value(self.0);
        match ruby_type {
            RubyType::Nil => write!(f, "Nil"),
            RubyType::Fixnum => write!(f, "Fixnum({})", i64::from_value_unchecked(self.0)),
            RubyType::Float => write!(f, "Float({})", f64::from_value_unchecked(self.0)),
            RubyType::True | RubyType::False => write!(f, "Bool({})", bool::from_value_unchecked(self.0)),
            RubyType::String => write!(f, "String({})", String::from_value_unchecked(self.0)),
            RubyType::Symbol => write!(f, "Symbol(_not_implemented_yet_)"),
            RubyType::Array => write!(f, "Array"),
            RubyType::Hash => write!(f, "Hash"),
            _ => write!(f, "Object")
        }
    }
}


#[derive(Debug)]
pub struct Nil;

pub trait ToValue {
    fn to_value(&self) -> VALUE;
}

// TODO: Use associated const when available and support access from the trait itself.
pub trait FromValue: Sized {
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

impl ToValue for VALUE {
    fn to_value(&self) -> VALUE {
        *self
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

impl ToValue for bool {
    fn to_value(&self) -> VALUE {
        match *self {
            true => RUBY_Qtrue as VALUE,
            false => RUBY_Qfalse as VALUE
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
        unsafe { rb_float_new(*self) }
    }
}

impl ToValue for String {
    fn to_value(&self) -> VALUE {
        unsafe { rb_str_new_cstr(CString::new(self.clone()).unwrap().as_ptr() as *const i8) }
    }
}

impl ToValue for Nil {
    fn to_value(&self) -> VALUE {
        RUBY_Qnil as VALUE
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
    ary.push(Nil);
    ary.push(true);
    ary.push(25);
    ary.push(rb_type(arg) as i64);
    println!("Type of arg: {:?}", RubyType::from_value(arg));
    // println!("Arg value: {:?}", RubyValue::from_value(arg));
    if let Some(arr) = Array::from_value(arg) {
        for val in arr {
            println!("Array item: {:?}", WrapValue(val))
        }
    }

    if let Some(hash) = Hash::from_value(arg) {
        println!("Hash len: {:?}", i32::from_value(hash.len()));
        println!("Hash keys: {:?}", hash.keys().into_iter().map(|itm| WrapValue(itm)).collect::<Vec<_>>() );
    }

    println!("Arg class name: {:?}", unsafe { CStr::from_ptr(rb_obj_classname(arg)) } );
    println!("Arg class name manual: {:?}", unsafe { String::from_value(rb_class_name(rb_funcall(arg, rb_intern(cast_str("class\x00")), 0))) } );
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

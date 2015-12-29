#![feature(libc)]

extern crate libc;

#[allow(dead_code, non_upper_case_globals, non_camel_case_types)]
mod ruby;
mod array;

use ruby::*;
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

#[derive(Debug)]
struct Hash;

#[derive(Debug)]
enum RubyValue {
    Nil, // T_NIL
    Float(f64), //T_FLOAT
    Bool(bool), // T_TRUE, T_FALSE
    String(String), // T_STRING
    Array(Array), // T_ARRAY
    Hash(Hash), // T_HASH
    Struct, // T_STRUCT
    Bignum, // T_BIGNUM
    Fixnum(i64), // T_FIXNUM
    Complex, // T_COMPLEX
    Rational, // T_RATIONAL
    File, // T_FILE
    Symbol, //T_SYMBOL
    Object
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Init_test_rust() {
  let my_mod = rb_define_module(str_to_ruby_str("TestRust\x00"));
  // let bar = rb_define_module_under(my_mod, str_to_ruby_str("Bar\x00"));
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
    println!("Type of arg: {:?}", ruby_type(arg));
    println!("Arg value: {:?}", ruby_value(arg));
    println!("Arg class name: {:?}", unsafe { CStr::from_ptr(rb_obj_classname(arg)) } );
    println!("Arg class name manual: {:?}", unsafe { ruby_value(rb_class_name(rb_funcall(arg, rb_intern(str_to_ruby_str("class\x00")), 0))) } );
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

fn str_to_ruby_str(string: &'static str) -> *const i8 {
    string.as_ptr() as *const i8
}

#[inline(always)]
fn INT2FIX(i: u64) -> VALUE {
    (i << 1 | RUBY_FIXNUM_FLAG as u64) as VALUE
}

fn ruby_value(mut value: VALUE) -> RubyValue {
    use std::slice;
    match ruby_type(value) {
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
        _ => RubyValue::Object
    }
}


// Basic types:
// ruby::RUBY_T_FALSE => RubyType::False,
// ruby::RUBY_T_FIXNUM => RubyType::Fixnum,
// ruby::RUBY_T_FLOAT => RubyType::Float,
// ruby::RUBY_T_NIL => RubyType::Nil,
// ruby::RUBY_T_SYMBOL => RubyType::Symbol,
// ruby::RUBY_T_TRUE => RubyType::True,
// ruby::RUBY_T_UNDEF => RubyType::Undef,

fn ruby_type(obj: VALUE) -> RubyType {
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

// RB_IMMEDIATE_P(x) ((VALUE)(x) & RUBY_IMMEDIATE_MASK)
fn RB_IMMEDIATE_P(x: VALUE) -> bool {
    (x as u32 & ruby::RUBY_IMMEDIATE_MASK) > 0
}

// RB_FIXNUM_P(f) (((int)(SIGNED_VALUE)(f))&RUBY_FIXNUM_FLAG)
fn RB_FIXNUM_P(f: VALUE) -> bool {
    (f as u32 & ruby::RUBY_FIXNUM_FLAG) > 0
}

// RB_FLONUM_P(x) ((((int)(SIGNED_VALUE)(x))&RUBY_FLONUM_MASK) == RUBY_FLONUM_FLAG)
fn RB_FLONUM_P(x: VALUE) -> bool {
    (x as u32 & ruby::RUBY_FLONUM_MASK == ruby::RUBY_FLONUM_FLAG)
}

// RB_STATIC_SYM_P(x) (((VALUE)(x)&~((~(VALUE)0)<<RUBY_SPECIAL_SHIFT)) == RUBY_SYMBOL_FLAG)
fn RB_STATIC_SYM_P(x: VALUE) -> bool {
    ((x as u32 & (!((!0)<<RUBY_SPECIAL_SHIFT))) == RUBY_SYMBOL_FLAG)
}

// RTEST(v) !(((VALUE)(v) & ~Qnil) == 0)
fn RTEST(v: VALUE) -> bool {
    !(v as u32 & (!ruby::RUBY_Qnil) == 0)
}

// NIL_P(v) !((VALUE)(v) != Qnil)
fn NIL_P(v: VALUE) -> bool {
    !(v as u32 != ruby::RUBY_Qnil)
}

// RB_BUILTIN_TYPE(x) (int)(((struct RBasic*)(x))->flags & RUBY_T_MASK)
fn RB_BUILTIN_TYPE(x: VALUE) -> u64 {
    unsafe { (*(x as *const ruby::Struct_RBasic)).flags as u64 & ruby::RUBY_T_MASK as u64 }
}

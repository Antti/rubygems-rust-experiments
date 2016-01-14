use super::ruby::{self, VALUE};
use super::rb_type;

#[derive(Debug)]
pub enum RubyType {
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
    pub fn from_value(obj: VALUE) -> Self {
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

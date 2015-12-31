use ruby::*;

#[inline(always)]
pub fn INT2FIX(i: i64) -> VALUE {
    (i << 1 | RUBY_FIXNUM_FLAG as i64) as VALUE
}

// RB_IMMEDIATE_P(x) ((VALUE)(x) & RUBY_IMMEDIATE_MASK)
pub fn RB_IMMEDIATE_P(x: VALUE) -> bool {
    (x as u32 & RUBY_IMMEDIATE_MASK) > 0
}

// RB_FIXNUM_P(f) (((int)(SIGNED_VALUE)(f))&RUBY_FIXNUM_FLAG)
pub fn RB_FIXNUM_P(f: VALUE) -> bool {
    (f as u32 & RUBY_FIXNUM_FLAG) > 0
}

// RB_FLONUM_P(x) ((((int)(SIGNED_VALUE)(x))&RUBY_FLONUM_MASK) == RUBY_FLONUM_FLAG)
pub fn RB_FLONUM_P(x: VALUE) -> bool {
    (x as u32 & RUBY_FLONUM_MASK == RUBY_FLONUM_FLAG)
}

// RB_STATIC_SYM_P(x) (((VALUE)(x)&~((~(VALUE)0)<<RUBY_SPECIAL_SHIFT)) == RUBY_SYMBOL_FLAG)
pub fn RB_STATIC_SYM_P(x: VALUE) -> bool {
    ((x as u32 & (!((!0)<<RUBY_SPECIAL_SHIFT))) == RUBY_SYMBOL_FLAG)
}

// RTEST(v) !(((VALUE)(v) & ~Qnil) == 0)
pub fn RTEST(v: VALUE) -> bool {
    !(v as u32 & (!RUBY_Qnil) == 0)
}

// NIL_P(v) !((VALUE)(v) != Qnil)
pub fn NIL_P(v: VALUE) -> bool {
    !(v as u32 != RUBY_Qnil)
}

// RB_BUILTIN_TYPE(x) (int)(((struct RBasic*)(x))->flags & RUBY_T_MASK)
pub fn RB_BUILTIN_TYPE(x: VALUE) -> u64 {
    unsafe { (*(x as *const Struct_RBasic)).flags as u64 & RUBY_T_MASK as u64 }
}

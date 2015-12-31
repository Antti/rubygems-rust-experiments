use ruby::*;
use array::Array;
use super::{RubyValue, cast_str, FromValue, ToValue};

// pub fn rb_hash_foreach(arg1: VALUE,
//                        arg2:
//                            ::std::option::Option<extern "C" fn()
//                                                      -> ::libc::c_int>,
//                        arg3: VALUE) -> ();
// pub fn rb_hash(arg1: VALUE) -> VALUE;
// pub fn rb_hash_new() -> VALUE;
// pub fn rb_hash_dup(arg1: VALUE) -> VALUE;
// pub fn rb_hash_freeze(arg1: VALUE) -> VALUE;
// pub fn rb_hash_aref(arg1: VALUE, arg2: VALUE) -> VALUE;
// pub fn rb_hash_lookup(arg1: VALUE, arg2: VALUE) -> VALUE;
// pub fn rb_hash_lookup2(arg1: VALUE, arg2: VALUE, arg3: VALUE) -> VALUE;
// pub fn rb_hash_fetch(arg1: VALUE, arg2: VALUE) -> VALUE;
// pub fn rb_hash_aset(arg1: VALUE, arg2: VALUE, arg3: VALUE) -> VALUE;
// pub fn rb_hash_clear(arg1: VALUE) -> VALUE;
// pub fn rb_hash_delete_if(arg1: VALUE) -> VALUE;
// pub fn rb_hash_delete(arg1: VALUE, arg2: VALUE) -> VALUE;
// pub fn rb_hash_set_ifnone(hash: VALUE, ifnone: VALUE) -> VALUE;
// pub fn rb_hash_update_by(hash1: VALUE, hash2: VALUE,
//                          func:
//                              *mut ::std::option::Option<extern "C" fn()
//                                                             -> VALUE>)
//  -> VALUE;
// pub fn rb_hash_tbl(arg1: VALUE) -> *mut Struct_st_table;
// pub fn rb_path_check(arg1: *const ::libc::c_char) -> ::libc::c_int;
// pub fn rb_env_path_tainted() -> ::libc::c_int;
// pub fn rb_env_clear() -> VALUE;
// pub fn rb_hash_size(arg1: VALUE) -> VALUE;
// pub fn rb_hash_iter_lev(arg1: VALUE) -> ::libc::c_int;
// pub fn rb_hash_ifnone(arg1: VALUE) -> VALUE;

#[derive(Debug)]
pub struct Hash {
    val: VALUE
}

impl Hash {
    pub fn new() -> Self {
        Hash { val: unsafe { rb_hash_new() } }
    }

    pub fn from_value(val: VALUE) -> Self {
        Hash { val: val }
    }

    pub fn to_value(&self) -> VALUE {
        self.val
    }

    pub fn aref(&self, key: VALUE) -> VALUE {
        unsafe { rb_hash_aref(self.val, key) }
    }

    pub fn len(&self) -> VALUE {
        unsafe { rb_hash_size(self.val) }
    }

    pub fn keys(&self) -> Array {
        let value = RubyValue::from_value(unsafe { rb_funcall(self.val, rb_intern(cast_str("keys\x00")), 0) } );
        match value {
            Some(RubyValue::Array(arr)) => arr,
            _ => panic!("Unexpected result of hash.keys")
        }
    }
}

impl ToValue for Hash {
    fn to_value(&self) -> VALUE {
        self.val
    }
}

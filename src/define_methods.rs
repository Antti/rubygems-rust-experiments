
// Typesafe method definition, example on rb_define_method(VALUE klass, const char *name, VALUE (*func)(), int argc)
// Goal: be able to define method in type safe maner.
// Type variance: method arguments.
// Cases:
//     * Positive number of arguments, function definition looks like:
//         pub extern "C" fn func(this: VALUE, arg1: VALUE, arg2: VALUE, arg3: VALUE) -> VALUE
//     * -1 number of arguments:
//         pub extern "C" fn func(argc: usize, argv: *const VALUE, this: VALUE) -> VALUE
//     * -2 arguments:
//         pub extern "C" fn func(this: VALUE, args: VALUE) -> VALUE
// Approach: Define a trait, variant over function type (arguments) and implement for each number of arguments (up to 17)


trait RubyCallable: Sized {
    // const number_of_args: i32;
    fn number_of_args(&self) -> i32;

    fn as_function_ptr(&self) -> extern "C" fn() -> VALUE;

    fn define_singleton_method(&self, object: VALUE, name: &str) {
        let c_name = CString::new(name).unwrap();
        unsafe { rb_define_singleton_method(object, c_name.as_ptr(), Some(self.as_function_ptr()), self.number_of_args() - 1); }
    }
    fn define_method(&self, class: VALUE, name: &str) {
        let c_name = CString::new(name).unwrap();
        unsafe { rb_define_method(class, c_name.as_ptr(), Some(self.as_function_ptr()), self.number_of_args() -1 ); }
    }
    fn define_private_method(&self, class: VALUE, name: &str) {
        let c_name = CString::new(name).unwrap();
        unsafe { rb_define_private_method(class, c_name.as_ptr(), Some(self.as_function_ptr()), self.number_of_args() - 1); }
    }
    fn define_protected_method(&self, class: VALUE, name: &str) {
        let c_name = CString::new(name).unwrap();
        unsafe { rb_define_protected_method(class, c_name.as_ptr(), Some(self.as_function_ptr()), self.number_of_args() - 1); }
    }
    fn define_module_function(&self, module: VALUE, name: &str) {
        let c_name = CString::new(name).unwrap();
        unsafe { rb_define_module_function(module, c_name.as_ptr(), Some(self.as_function_ptr()), self.number_of_args() - 1); }
    }
    fn define_global_function(&self, name: &str) {
        let c_name = CString::new(name).unwrap();
        unsafe { rb_define_global_function(c_name.as_ptr(), Some(self.as_function_ptr()), self.number_of_args()); }
    }
}

macro_rules! ruby_callable_def {
    ($ty:ty, $num:expr) => {
        impl RubyCallable for $ty {
            fn as_function_ptr(&self) -> extern "C" fn() -> VALUE {
                unsafe { transmute(*self) }
            }

            fn number_of_args(&self) -> i32 {
                $num
            }
        }
    }
}

ruby_callable_def!(extern "C" fn () -> VALUE, 0);
ruby_callable_def!(extern "C" fn (VALUE) -> VALUE, 1);
ruby_callable_def!(extern "C" fn (VALUE, VALUE) -> VALUE, 2);
ruby_callable_def!(extern "C" fn (VALUE, VALUE, VALUE) -> VALUE, 3);
ruby_callable_def!(extern "C" fn (VALUE, VALUE, VALUE, VALUE) -> VALUE, 4);
ruby_callable_def!(extern "C" fn (VALUE, VALUE, VALUE, VALUE, VALUE) -> VALUE, 5);
ruby_callable_def!(extern "C" fn (VALUE, VALUE, VALUE, VALUE, VALUE, VALUE) -> VALUE, 6);
ruby_callable_def!(extern "C" fn (VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE) -> VALUE, 7);
ruby_callable_def!(extern "C" fn (VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE) -> VALUE, 8);
ruby_callable_def!(extern "C" fn (VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE) -> VALUE, 9);
ruby_callable_def!(extern "C" fn (VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE) -> VALUE, 10);
ruby_callable_def!(extern "C" fn (VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE) -> VALUE, 11);
ruby_callable_def!(extern "C" fn (VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE) -> VALUE, 12);
ruby_callable_def!(extern "C" fn (VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE) -> VALUE, 13);
ruby_callable_def!(extern "C" fn (VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE) -> VALUE, 14);
ruby_callable_def!(extern "C" fn (VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE) -> VALUE, 15);
ruby_callable_def!(extern "C" fn (VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE) -> VALUE, 16);
ruby_callable_def!(extern "C" fn (VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE, VALUE) -> VALUE, 17);
ruby_callable_def!(extern "C" fn (usize, *const VALUE, VALUE) -> VALUE, -1);


// Since Rust doesn't allow to automatically cast functions to their prototypes this looks ugly
fn usage {
    (foo as extern "C" fn(VALUE, VALUE) -> VALUE).define_singleton_method(my_mod, "foo")
}

pub extern "C" foo(this: VALUE, arg: VALUE) -> VALUE {
    unimplemented!()
}

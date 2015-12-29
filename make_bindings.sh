#! /bin/sh

./target/debug/bindgen -builtins -l ruby -I /usr/local/opt/rbenv/sources/2.2.3/ruby-2.2.3/include/  /usr/local/opt/rbenv/sources/2.2.3/ruby-2.2.3/include/ruby.h > ruby.rs

use trait_ffi::*;

#[def_extern_trait]
pub trait DemeIf {
    fn say_hello<'a>(a: usize) -> i32;
}

pub fn if_say_hello(a: usize) -> i32 {
    println!("Hello from DemeIf with value: {a}");
    deme_if::say_hello(a)
}

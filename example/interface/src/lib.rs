use extern_trait::*;

#[def_extern_trait]
pub trait DemeIf {
    fn say_hello(a: usize) -> i32;
}

pub fn if_say_hello(a: usize) -> i32 {
    println!("Hello from DemeIf with value: {}", a);
    demeif::say_hello(a)
}

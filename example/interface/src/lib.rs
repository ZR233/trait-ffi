pub mod aaa;

pub fn if_say_hello<'a>(a: usize) -> &'a str {
    println!("Hello from DemeIf with value: {a}");
    aaa::deme_if::say_hello(a)
}

pub fn test_trait_function(x: i32) -> i32 {
    println!("Calling test function with value: {x}");
    aaa::test_trait_without_impl_macro::test_function(x)
}

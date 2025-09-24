use interface_demo::{
    aaa::{DemeIf, TestTraitWithoutImplMacro},
    impl_trait,
};

pub struct MyImpl;

impl_trait! {
    impl DemeIf for MyImpl {
        unsafe fn say_hello<'a>(a: usize) -> &'a str {
            println!("Hello from MyImpl with value: {a}");
            "Hello from MyImpl"
        }

        #[cfg(feature = "all")]
        fn ft_all() -> &'static str {
            "Hello from MyImpl's implementation of ft_all"
        }
    }
}

pub struct TestImpl;

impl_trait! {
    impl TestTraitWithoutImplMacro for TestImpl {
        fn test_function(x: i32) -> i32 {
            println!("TestImpl test_function called with value: {x}");
            x * 2
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_say_hello() {
        let result = interface_demo::if_say_hello(3);
        assert_eq!(result, "Hello from MyImpl");
    }
}

use interface_demo::{aaa::DemeIf, impl_trait};

pub struct MyImpl;

impl_trait! {
    impl DemeIf for MyImpl {
        unsafe fn say_hello<'a>(a: usize) -> &'a str {
            println!("Hello from MyImpl with value: {a}");
            "Hello from MyImpl"
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

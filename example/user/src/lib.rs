use interface::{DemeIf, impl_trait};

pub struct MyImpl;

impl_trait! {
    impl DemeIf for MyImpl {
        fn say_hello(a: usize) -> i32 {
            println!("Hello from MyImpl with value: {}", a);
            (a * 2) as i32
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_say_hello() {
        let result = interface::if_say_hello(3);
        assert_eq!(result, 6);
    }
}

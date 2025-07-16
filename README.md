# extern-trait

A Rust procedural macro library for creating and implementing extern fn with Trait.

## Features

This library provides procedural macros to define and implement cross-crate extern func, supporting:

1. **Define external traits** (`#[def_extern_trait]`) - Define a trait that can be implemented across crates
2. **Generated implementation macros** (`impl_trait!`) - Auto-generated macros for simplified trait implementation
3. **Multiple ABI support** - Support for both C and Rust ABIs

## Usage

### Define external traits

Define an external trait in the interface crate:

```rust
use extern_trait::*;

#[def_extern_trait]
pub trait DemeIf {
    fn say_hello(a: usize) -> i32;
}

pub fn if_say_hello(a: usize) -> i32 {
    println!("Hello from DemeIf with value: {}", a);
    demeif::say_hello(a)
}
```

### Implement external traits

Implement the external trait in the implementation crate:

```rust
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
```

### Specify ABI

You can specify the ABI for external traits (default is "rust"):

```rust
#[def_extern_trait(abi = "c")]
pub trait MyTrait {
    fn my_function() -> i32;
}
```

Supported ABI types:

- `"rust"` - Rust ABI (default)
- `"c"` - C ABI

## How it works

This library uses Rust's procedural macro system to generate cross-crate external function calls:

1. **Definition stage**: The `#[def_extern_trait]` macro generates corresponding external function declarations and call wrappers for each function in the trait
2. **Implementation stage**: The `#[impl_extern_trait]` macro generates `#[no_mangle]` external functions for the implemented functions
3. **Linking stage**: Through function name prefix conventions, ensures that the interface crate can correctly call functions in the implementation crate

Generated function names use the crate name as a prefix, e.g., `__mycrate_function_name`, to avoid symbol conflicts.

## Key Features

- **Type safety**: Maintains complete Rust type system safety
- **Zero runtime overhead**: All processing is done at compile time
- **Multiple ABI support**: Supports both C and Rust ABIs
- **Automatic symbol management**: Automatically handles function name prefixes to avoid symbol conflicts
- **Easy to use**: Provides clean macro interfaces

## Limitations

- Only supports function-type trait items
- Requires linking implementation crates at compile time
- Function parameters and return values must be FFI-safe types (when using C ABI)

## License

This project is licensed under the MIT License.

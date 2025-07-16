# extern-trait

一个用于获取使用过程宏的 crate 名称的 Rust 过程宏库。

## 功能

这个库提供了多种方式来获取调用者的 crate 名称：

1. **函数宏** (`extern_crate_name!()`) - 直接返回 crate 名称
2. **属性宏** (`#[get_extern_crate_name]`) - 为结构体添加获取 crate 名称的方法
3. **Derive 宏** (`#[derive(ExternCrateName)]`) - 通过 derive 为结构体添加方法
4. **信息模块宏** (`create_crate_info!()`) - 创建包含完整 crate 信息的模块

## 使用方法

### 1. 函数宏

```rust
use extern_trait::extern_crate_name;

fn main() {
    let crate_name = extern_crate_name!();
    println!("当前 crate 名称: {}", crate_name);
}
```

### 2. 属性宏

```rust
use extern_trait::get_extern_crate_name;

#[get_extern_crate_name]
struct MyService;

fn main() {
    let crate_name = MyService::get_crate_name();
    println!("Crate 名称: {}", crate_name);
}
```

### 3. Derive 宏

```rust
use extern_trait::ExternCrateName;

#[derive(ExternCrateName)]
struct MyStruct;

fn main() {
    let crate_name = MyStruct::get_crate_name();
    println!("Crate 名称: {}", crate_name);
}
```

### 4. 信息模块宏

```rust
use extern_trait::create_crate_info;

create_crate_info!();

fn main() {
    println!("Crate 名称: {}", crate_info::NAME);
    println!("Crate 版本: {}", crate_info::VERSION);
    println!("Crate 作者: {}", crate_info::AUTHORS);
    println!("Crate 描述: {}", crate_info::DESCRIPTION);
    
    let info = crate_info::get_info();
    println!("完整信息: {:?}", info);
}
```

## 工作原理

这个库利用 Rust 的编译时环境变量来获取 crate 信息：

- `CARGO_PKG_NAME` - crate 名称
- `CARGO_PKG_VERSION` - crate 版本
- `CARGO_PKG_AUTHORS` - crate 作者
- `CARGO_PKG_DESCRIPTION` - crate 描述

这些信息在编译时被嵌入到生成的代码中，因此不会有运行时开销。

## 示例

运行示例：

```bash
cd example/interface
cargo run --bin demo
```

运行测试：

```bash
cd example/interface
cargo test
```

## 依赖

- `proc-macro2` - 用于处理过程宏的 token
- `quote` - 用于生成 Rust 代码
- `syn` - 用于解析 Rust 语法

## 许可证

本项目采用 MIT 许可证。

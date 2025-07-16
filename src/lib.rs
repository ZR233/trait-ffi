use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{ItemImpl, ItemTrait, parse_macro_input, spanned::Spanned};

macro_rules! bail {
    ($i:expr, $msg:expr) => {
        return syn::parse::Error::new($i, $msg).to_compile_error().into();
    };
}

fn get_crate_name() -> String {
    std::env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "unknown".to_string())
}

fn parse_def_extern_trait_args(args: TokenStream) -> Result<String, String> {
    if args.is_empty() {
        return Ok("rust".to_string()); // 默认使用 Rust ABI
    }

    let args_str = args.to_string();
    let mut abi = None;

    // 简单解析 abi="value" 形式
    let parts: Vec<&str> = args_str.split(',').collect();

    for part in parts {
        let part = part.trim();
        if part.starts_with("abi") {
            if let Some(start) = part.find('"') {
                if let Some(end) = part.rfind('"') {
                    if start < end {
                        abi = Some(part[start + 1..end].to_string());
                    }
                }
            }
        }
    }

    let abi = abi.unwrap_or_else(|| "rust".to_string());

    if abi != "c" && abi != "rust" {
        return Err("Invalid abi parameter. Supported values: \"c\", \"rust\"".to_string());
    }

    Ok(abi)
}

#[proc_macro_attribute]
pub fn def_extern_trait(args: TokenStream, input: TokenStream) -> TokenStream {
    let abi = match parse_def_extern_trait_args(args) {
        Ok(abi) => abi,
        Err(error_msg) => {
            bail!(Span::call_site(), error_msg);
        }
    };

    let input = parse_macro_input!(input as ItemTrait);
    let vis = input.vis.clone();
    let mod_name = format_ident!("{}", input.ident.to_string().to_lowercase());
    let crate_name_str = get_crate_name();
    let prefix = make_prefix(&crate_name_str);

    let mut fn_list = vec![];

    for item in &input.items {
        if let syn::TraitItem::Fn(func) = item {
            let fn_name = func.sig.ident.clone();
            let extern_fn_name = format_ident!("{}{}", prefix, func.sig.ident);
            let attrs = &func.attrs;
            let inputs = &func.sig.inputs;
            let output = &func.sig.output;

            // 生成参数名和类型
            let mut param_names = vec![];
            let mut param_types = vec![];

            for input in inputs {
                if let syn::FnArg::Typed(pat_type) = input {
                    param_names.push(&pat_type.pat);
                    param_types.push(&pat_type.ty);
                }
            }

            let extern_abi = if abi == "rust" { "Rust" } else { "C" };

            fn_list.push(quote! {
                #(#attrs)*
                pub fn #fn_name(#inputs) #output {
                    unsafe extern #extern_abi {
                        fn #extern_fn_name(#inputs) #output;
                    }
                    unsafe{ #extern_fn_name(#(#param_names),*) }
                }
            });
        } else {
            bail!(
                item.span(),
                "Only function items are allowed in extern traits"
            );
        }
    }

    let crate_name = format_ident!("{}", crate_name_str.replace("-", "_"));

    let warn_fn_name = format_ident!(
        "Trait_{}_in_crate_{}_need_impl",
        input.ident,
        crate_name_str.replace("-", "_")
    );

    let generated_macro = quote! {
        #[macro_export]
        macro_rules! impl_trait {
            (impl $trait:ident for $type:ty { $($body:tt)* }) => {
                #[#crate_name::impl_extern_trait(name = #crate_name_str, abi = #abi)]
                impl $trait for $type {
                    $($body)*
                }

                #[allow(snake_case)]
                #[unsafe(no_mangle)]
                extern "C" fn #warn_fn_name() { }
            };
        }
    };

    quote! {
        pub use trait_ffi::impl_extern_trait;

        #input

        #vis mod #mod_name {
            use super::*;
            pub fn ____checker_do_not_use(){
                unsafe extern "C" {
                    fn #warn_fn_name();
                }
                unsafe { #warn_fn_name() };
            }
            #(#fn_list)*
        }

        #generated_macro
    }
    .into()
}

fn make_prefix(name: &str) -> String {
    format!("__{}_", name.to_lowercase().replace("-", "_"))
}

fn parse_extern_trait_args(args: TokenStream) -> Result<(String, String), String> {
    if args.is_empty() {
        return Err(
            "Missing parameters. Usage: #[impl_extern_trait(name=\"crate_name\", abi=\"c\")]"
                .to_string(),
        );
    }

    let args_str = args.to_string();
    let mut name = None;
    let mut abi = None;

    // 简单解析 name="value", abi="value" 形式
    let parts: Vec<&str> = args_str.split(',').collect();

    for part in parts {
        let part = part.trim();
        if part.starts_with("name") {
            if let Some(start) = part.find('"') {
                if let Some(end) = part.rfind('"') {
                    if start < end {
                        name = Some(part[start + 1..end].to_string());
                    }
                }
            }
        } else if part.starts_with("abi") {
            if let Some(start) = part.find('"') {
                if let Some(end) = part.rfind('"') {
                    if start < end {
                        abi = Some(part[start + 1..end].to_string());
                    }
                }
            }
        }
    }

    let name = name.ok_or_else(|| {
        "Missing name parameter. Usage: #[impl_extern_trait(name=\"crate_name\", abi=\"c\")]"
            .to_string()
    })?;
    let abi = abi.unwrap_or_else(|| "c".to_string());

    if abi != "c" && abi != "rust" {
        return Err("Invalid abi parameter. Supported values: \"c\", \"rust\"".to_string());
    }

    Ok((name, abi))
}

#[proc_macro_attribute]
pub fn impl_extern_trait(args: TokenStream, input: TokenStream) -> TokenStream {
    let (crate_name_str, abi) = match parse_extern_trait_args(args) {
        Ok((name, abi)) => (name, abi),
        Err(error_msg) => {
            bail!(Span::call_site(), error_msg);
        }
    };
    let input = parse_macro_input!(input as ItemImpl);
    let mut extern_fn_list = vec![];

    let prefix = make_prefix(&crate_name_str);

    let struct_name = input.self_ty.clone();
    let trait_name = input.clone().trait_.unwrap().1;

    for item in &input.items {
        if let syn::ImplItem::Fn(func) = item {
            let fn_name_raw = &func.sig.ident;
            let fn_name = format_ident!("{prefix}{fn_name_raw}");
            let inputs = &func.sig.inputs;
            let output = &func.sig.output;

            let extern_abi = if abi == "rust" { "Rust" } else { "C" };

            let mut param_names = vec![];
            let mut param_types = vec![];

            for input in inputs {
                if let syn::FnArg::Typed(pat_type) = input {
                    param_names.push(&pat_type.pat);
                    param_types.push(&pat_type.ty);
                }
            }

            extern_fn_list.push(quote! {
                #[unsafe(no_mangle)]
                pub extern #extern_abi fn #fn_name(#inputs) #output {
                    <#struct_name as #trait_name>::#fn_name_raw(#(#param_names),*)
                }
            });
        }
    }

    quote! {
        #input
        #(#extern_fn_list)*
    }
    .into()
}

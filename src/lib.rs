use proc_macro::TokenStream;
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
#[proc_macro_attribute]
pub fn def_extern_trait(_args: TokenStream, input: TokenStream) -> TokenStream {
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

            fn_list.push(quote! {
                #(#attrs)*
                pub fn #fn_name(#inputs) #output {
                    unsafe extern "C" {
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
    let crate_name = format_ident!("{crate_name_str}");

    let generated_macro = quote! {
        #[macro_export]
        macro_rules! impl_trait {
            (impl $trait:ident for $type:ty { $($body:tt)* }) => {
                #[#crate_name::impl_extern_trait(#crate_name_str)]
                impl $trait for $type {
                    $($body)*
                }
            };
        }
    };
    quote! {
        pub use extern_trait::impl_extern_trait;

        #input

        #vis mod #mod_name {
            use super::*;

            #(#fn_list)*
        }

        #generated_macro
    }
    .into()
}

fn make_prefix(name: &str) -> String {
    format!("__{}_", name.to_lowercase().replace("-", "_"))
}

fn parse_crate_name(args: TokenStream) -> Result<String, String> {
    if args.is_empty() {
        return Err(
            "Missing crate_name parameter. Usage: #[impl_extern_trait(\"crate_name\")]".to_string(),
        );
    }

    let args_str = args.to_string();
    // 简单解析 crate_name = "value" 形式
    if let Some(start) = args_str.find('"') {
        if let Some(end) = args_str.rfind('"') {
            if start < end {
                return Ok(args_str[start + 1..end].to_string());
            }
        }
    }

    Err(
        "Invalid crate_name parameter format. Usage: #[impl_extern_trait(\"crate_name\")]"
            .to_string(),
    )
}

#[proc_macro_attribute]
pub fn impl_extern_trait(args: TokenStream, input: TokenStream) -> TokenStream {
    let crate_name_str = match parse_crate_name(args) {
        Ok(name) => name,
        Err(error_msg) => {
            return quote! {
                compile_error!(#error_msg);
            }
            .into();
        }
    };
    let input = parse_macro_input!(input as ItemImpl);
    let mut extern_fn_list = vec![];

    let prefix = make_prefix(&crate_name_str);

    for item in &input.items {
        if let syn::ImplItem::Fn(func) = item {
            let fn_name = format_ident!("{prefix}{}", func.sig.ident);
            let inputs = &func.sig.inputs;
            let output = &func.sig.output;
            let block = &func.block;

            extern_fn_list.push(quote! {
                #[unsafe(no_mangle)]
                pub extern "C" fn #fn_name(#inputs) #output #block
            });
        }
    }

    quote! {
        #input
        #(#extern_fn_list)*
    }
    .into()
}

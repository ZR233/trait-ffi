use trait_ffi::*;

#[def_extern_trait(mod_path = "aaa")]
pub trait DemeIf {
    /// # Safety
    ///
    unsafe fn say_hello<'a>(a: usize) -> &'a str;

    #[cfg(feature = "all")]
    fn ft_all() -> &'static str;
}

// 测试新的 not_def_impl 参数
#[def_extern_trait(not_def_impl)]
pub trait TestTraitWithoutImplMacro {
    fn test_function(x: i32) -> i32;
}
